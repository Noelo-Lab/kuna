//! (kuna pushan) VPC-sensitive flat-CFG recovery.
//!
//! The paper's stage 1.  A node is a native address plus the virtual program
//! counter the interpreter holds when it gets there (and the register holding
//! it, when it is one), so every bytecode instruction's handler copy is its
//! own block and the recovered graph is the protected program's control flow
//! laid flat through the interpreter.  Each
//! node is emulated once per round ([`Emulator`]): a guard that folds to a
//! constant prunes the other side, a symbolic guard keeps both, and a computed
//! target is enumerated by case-splitting its boolean atoms.  When a later path
//! reaches an existing node with a different constant in some location, that
//! location is recorded; the next round starts over with those locations
//! symbolic at the node's entry (the paper's symbolization), until no new
//! location appears.
//!
//! The VPC is found as the paper describes: the register a block dereferences
//! with a small displacement into the interpreter's own section is the
//! bytecode pointer, kept while it keeps being used and replaced when the
//! interpreter moves it.  Outside the interpreter the context sticks to the
//! last VPC, salted with the return address pushed into a dispatch call so two
//! re-entry stubs never share the entry handler's node.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use super::kuna_pushanemu::{merge_ite, split_over_symbols, symbol_pin, Emulator, Exit, InsnTrace, LiftedInsn, PushanTarget, SymValues, VpcSource, FETCH_WINDOW};
use super::kuna_pushanexpr::{ExprArena, ExprId, VarId, VarKind};
use super::kuna_pushanstate::{Diff, Loc, PushanImage, State, SymLoc};

pub type NodeId = u32;

/// A memory cell: its symbolic base (None for a concrete address) and offset.
pub type Cell = (Option<VarId>, i128);

/// Locations holding small constants (an interleaved interpreter's selector).
pub type SmallLocs = Vec<(Loc, u128)>;

/// Where an interleaved interpreter's selector is pinned: the address the
/// run that loads the VPC cell starts at, and each location with its value.
pub type Pin = (u64, SmallLocs);

/// One dispatch the probe saw: the computed jump, the start of the run that
/// loaded the cell fetched through last, the small constants there, and that cell.
type Turn = (u64, u64, SmallLocs, Vec<Cell>);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NodeKey {
    pub addr: u64,
    pub ctx: Option<u64>,
    /// The stack pointer's offset from its entry value on entry (None when
    /// it is not relative to the entry stack pointer).
    pub sp: Option<i128>,
    /// The register the path holds the virtual PC in, when it is one
    /// (`VmCtx::vpc_reg` with no memory cell).
    pub holder: Option<u64>,
}

impl NodeKey {
    pub fn at(addr: u64, ctx: Option<u64>) -> Self {
        NodeKey { addr, ctx, sp: None, holder: None }
    }
}

/// Per-path interpreter context carried along edges (not part of the state).
#[derive(Clone, Copy, Debug, Default)]
pub struct VmCtx {
    pub vpc_reg: Option<u64>,
    /// A memory cell holding the VPC, as a base (None: a concrete address)
    /// and an offset: Themida's VM context, or an in-function interpreter's
    /// local; it takes precedence over `vpc_reg`.
    pub vpc_cell: Option<Cell>,
    pub ctx: Option<u64>,
    pub in_vm: bool,
    /// A far value the VPC register took without a fetch, followed once it
    /// survives a second block.
    pub pending_vpc: Option<u64>,
    /// The path left the interpreter for the function's own code; a jump
    /// back into the protector is a re-entry.
    pub exited: bool,
}

/// How a node's control flow was resolved.
#[derive(Clone, Debug)]
pub enum NodeExit {
    /// One successor (direct branch, resolved indirect, dispatch call, ret-as-jump).
    Branch(NodeId),
    /// The native conditional branch(es) kept: arms in program order, then the fall-through.
    CondBranch { arms: Vec<(ExprId, NodeId)>, fall: NodeId },
    /// A computed target enumerated into several successors; each carries the
    /// boolean assignments that select it.  `by_atoms` is false when the
    /// enumeration ran over a symbolized location's recorded constants.
    Select { atoms: Vec<ExprId>, targets: Vec<(Vec<Vec<(ExprId, bool)>>, NodeId)>, by_atoms: bool },
    /// A call that leaves the interpreter; emulation continued at the return address.
    Call { target: Option<u64>, ret: NodeId, slot: Option<u64> },
    /// The protected function returns to its caller.
    Return,
    /// A jump to another function's entry.
    TailCall(u64),
    /// A jump through an import or GOT slot.
    TailCallSlot(u64),
    /// A call that never returns (Windows `__fastfail`); the original op is kept.
    NoReturn,
    /// A target the emulation could not enumerate; the original op is kept.
    Unresolved(String),
    Halt(String),
}

#[derive(Clone, Debug)]
pub struct Node {
    /// For an external call: the bytes the callee pops beyond its return address.
    pub call_pop: Option<u32>,
    /// For an external call: the scratch registers its summary overwrote
    /// (every one the callee was not shown to keep; the return register is
    /// not among them).
    pub call_clobbers: Option<Vec<(u64, u8)>>,
    /// Where each variable of a select's deciding atoms lives when the node
    /// exits, as the constant it is offset by there, with the stack pointer's
    /// offset from its entry value there.
    pub exit_locs: Option<(Vec<(VarId, Loc, i128)>, Option<i128>)>,
    /// For a select that forked the virtual PC: where the VPC is held at the
    /// node's exit (a register, or a concrete memory cell), and the VPC each
    /// target continues with, in target order.
    pub vpc_fork: Option<(Loc, Vec<u64>)>,
    pub key: NodeKey,
    pub entry: State,
    pub entry_ctx: VmCtx,
    pub insns: Vec<Rc<LiftedInsn>>,
    pub traces: Vec<InsnTrace>,
    pub exit: NodeExit,
    pub preds: Vec<NodeId>,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub rounds: u32,
    pub nodes: usize,
    pub insns: usize,
    pub symbolized: usize,
    pub unresolved: usize,
    pub halts: usize,
    pub vm_nodes: usize,
    /// Paths joined to a node at another stack depth without merging.
    pub unmerged: usize,
    /// Stores to an address the emulation could not place, ignored.
    pub dropped_stores: usize,
    /// Stack-pointer writes no identity placed ([`Emulator::lost_sp`]).
    pub lost_sp: usize,
    /// The last round recorded nothing new (the symbolization fixpoint).
    pub converged: bool,
    /// The exploration stopped at the node limit or the time budget.
    pub cut_short: bool,
    /// An in-function interpreter's fetch read bytecode from writable
    /// memory as the file holds it.
    pub writable_bytecode: bool,
    /// Computed jumps of an in-function interpreter the exploration decided.
    pub data_dispatches: usize,
    /// Jumps to another function the exploration kept as calls it could not
    /// follow: the stack pointer was not at its entry value there.
    pub misplaced_tail_calls: usize,
    /// Identities that held only for a stack pointer within the platform's
    /// user-mode range ([`ExprArena::sp_bound`]).
    pub user_stack_assumed: usize,
    /// Interleaved bytecode programs found; only the first is followed.
    pub woven: usize,
    /// `rep movs` copies modelled ([`Explorer::rep_copy`]).
    pub rep_copies: usize,
    /// The last round read a copy's destination other than through a
    /// folded VPC fetch.
    pub copy_read_unfolded: bool,
}

pub struct Config {
    pub max_nodes: usize,
    pub max_block_insns: usize,
    pub max_rounds: u32,
    pub max_select_targets: usize,
    /// Wall-clock budget for the whole exploration (seconds).
    pub max_seconds: u64,
    /// Streaming diagnostics sink (one line per emulated node).
    pub trace: Option<std::rc::Rc<std::cell::RefCell<std::fs::File>>>,
}

impl Default for Config {
    fn default() -> Self {
        let trace = std::env::var("KUNA_PUSHAN_TRACE")
            .ok()
            .and_then(|p| std::fs::File::create(p).ok())
            .map(|f| std::rc::Rc::new(std::cell::RefCell::new(f)));
        let max_seconds = std::env::var("KUNA_PUSHAN_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
        let max_nodes = std::env::var("KUNA_PUSHAN_MAX_NODES").ok().and_then(|v| v.parse().ok()).unwrap_or(200_000);
        Config { max_nodes, max_block_insns: 4096, max_rounds: 40, max_select_targets: 16, max_seconds, trace }
    }
}

/// ABI facts the exploration needs from the architecture.
#[derive(Clone)]
pub struct Abi {
    pub ptr_size: u8,
    pub sp: u64,
    pub return_reg: u64,
    /// Registers (offset, size) an external call may clobber, besides the return register.
    pub clobbers: Vec<(u64, u8)>,
    pub flag_regs: (u64, u64),
    /// Registers the VPC heuristic considers (general purpose).
    pub gprs: Vec<u64>,
    pub big_endian: bool,
    /// Direction flag register, held clear at entry (string ops).
    pub df: Option<u64>,
    /// Stack slots above the return address an external call may read as arguments.
    pub stack_args: u8,
    /// The outgoing argument area of an external call, in words above the
    /// return address: the callee's home space, then its stack arguments.
    pub call_home_words: u8,
    pub call_arg_words: u8,
    /// The program counter register (written by call/return emulation only).
    pub pc: Option<(u64, u8)>,
    /// System flags with their normal user-mode values at entry (TF=0, IF=1,
    /// NT/RF/VM/AC/VIF/VIP/ID=0): the interpreter's anti-debugging checks fold.
    pub fixed_flags: Vec<(u64, u8)>,
    /// Registers preserved across calls (plus the stack pointer).
    pub callee_saved: Vec<(u64, u8)>,
    /// Registers with a fixed value at entry (offset, size, value): the
    /// segment base of the thread block on Windows.
    pub fixed_regs: Vec<(u64, u8, u64)>,
    /// Registers an external call may read.
    pub args: Vec<(u64, u8)>,
}

pub struct FlatCfg {
    pub nodes: Vec<Node>,
    pub index: BTreeMap<NodeKey, NodeId>,
    pub entry: NodeId,
    /// Entry-stack offsets whose address was handed to an external call.
    pub escapes: Vec<i128>,
    /// The values each merge symbol stood for on the paths into its node in
    /// the last round (`None` when there were more than [`MAX_SYM_SOURCES`]).
    pub sym_sources: BTreeMap<VarId, Option<BTreeSet<ExprId>>>,
}

pub struct Explorer<'a> {
    pub target: &'a dyn PushanTarget,
    pub abi: Abi,
    pub config: Config,
    pub arena: ExprArena,
    pub entry_addr: u64,
    /// Locations symbolized at a node, with every constant seen there.
    pub to_symbolize: BTreeMap<NodeKey, BTreeMap<SymLoc, BTreeSet<u128>>>,
    /// The constants recorded for each symbol created by [`State::symbolize`].
    sym_values: SymValues,
    /// Bits every value seen at a symbolized location agreed on.
    sym_known: BTreeMap<(NodeKey, SymLoc), (u128, u128)>,
    /// How often a location's agreed bits narrowed.
    sym_shrinks: BTreeMap<(NodeKey, SymLoc), u8>,
    /// The values each merge symbol stood for this round.
    sym_sources: BTreeMap<VarId, Option<BTreeSet<ExprId>>>,
    /// Pointer values the emulator read from image slots (value -> slot).
    image_ptrs: BTreeMap<u64, u64>,
    /// Entry-stack offsets passed to external calls, over every round.
    escapes: BTreeSet<i128>,
    /// Offsets that escaped for the first time this round.
    new_escapes: usize,
    /// Locations recorded by the last select built, for its node.
    pending_locs: Option<(Vec<(VarId, Loc, i128)>, Option<i128>)>,
    /// The VPC holder and per-target VPCs of the last virtual-PC fork.
    pending_vpc_fork: Option<(Loc, Vec<u64>)>,
    /// Callees in the image already read.
    callees: BTreeMap<u64, Option<CalleeScan>>,
    /// The native addresses nodes start at this round (handler entries).
    node_addrs: BTreeSet<u64>,
    /// The stack depths seen this round at each native address and context.
    stack_depths: BTreeMap<(u64, Option<u64>), Vec<Option<i128>>>,
    /// Addresses in the function's own code some path reached outside the
    /// interpreter, over every round: a native node's run ends there.
    native_starts: BTreeSet<u64>,
    /// Addresses inside the runs of native nodes emulated this round.
    native_covered: BTreeSet<u64>,
    /// Native starts found this round inside an earlier native node's run.
    new_splits: usize,
    /// The stack pointer's offset from its entry value where native code
    /// first jumped into the protector this round.
    vm_entry_depth: Option<Option<i128>>,
    /// The node being resolved left by a direct jump (a constant in the instruction).
    direct_exit: bool,
    /// The context each arm of the conditional exit being resolved continues in.
    arm_ctxs: Vec<VmCtx>,
    /// Nodes that paths reached with different contents behind a base that is
    /// a fresh variable every round: such memory starts unknown there.
    forget_fresh: BTreeSet<NodeKey>,
    /// Nodes that paths reached after different external calls.
    fresh_epoch: BTreeSet<NodeKey>,
    /// Nodes that paths reached with different copies of image memory.
    forget_copies: BTreeSet<NodeKey>,
    /// Symbolized locations some path reached with a value that is not one
    /// of a few constants: their recorded constants are not all they hold.
    sym_open: BTreeSet<(NodeKey, SymLoc)>,
    /// The location each merged symbol was made for.
    own_syms: BTreeMap<VarId, (NodeKey, SymLoc)>,
    /// External calls emulated so far, numbering each one's memory effect.
    call_epochs: u64,
    /// What the last call summarized pops beyond its return address.
    last_pop: Option<u32>,
    last_clobbers: Option<Vec<(u64, u8)>>,
    pub stats: Stats,
    pub log: Vec<String>,
    retaddr: Option<ExprId>,
    sp0: Option<u32>,
    new_syms: usize,
    started: std::time::Instant,
    pub timed_out: bool,
    /// The memory cell an in-function interpreter keeps its virtual PC in
    /// ([`Explorer::probe_interpreter`]).
    pub data_vpc: Option<Cell>,
    /// Probe for an in-function interpreter before the first round.
    pub probe: bool,
    /// The selector of interleaved bytecode programs, pinned to the first
    /// program's index where the run that loads the VPC cell starts.
    pub pin: Option<Pin>,
    /// Model a `rep movs` of file data as a copy ([`Explorer::rep_copy`]).
    pub rep_copies: bool,
    /// A pointer into a copy's destination reached a call this round.
    copy_escaped: bool,
}

/// What [`Explorer::probe_interpreter`] found.
#[derive(Default, Debug)]
pub struct Interpreter {
    /// The VPC cell of each interleaved program, the one dispatched first leading.
    pub cells: Vec<Cell>,
    /// The selector to pin, when there are several programs.
    pub pin: Option<Pin>,
}

const VPC_MAX_DISP: i128 = 16;
/// Farthest a new VPC register's value may sit from the previous context.
const VPC_NEAR: u64 = 0x100;
/// Most constants remembered for one symbolized location.
const MAX_SYM_VALUES: usize = 32;
/// Most incoming values remembered for one merge symbol.
const MAX_SYM_SOURCES: usize = 64;
/// Distinct virtual-PC values a node may fork into.
const MAX_VPC_FORK: usize = 4;
/// Most instructions [`Explorer::handler_fragment`] follows.
const FRAGMENT_INSNS: usize = 128;
/// Most instructions [`Explorer::probe_stack_effect`] emulates.
const PROBE_INSNS: usize = 4096;
/// Bytes behind each pointer argument an external call may write.
const CALL_POINTEE_BYTES: i128 = 16;
/// Most instructions read from a callee in the image when summarizing it.
const MAX_CALLEE_SCAN: usize = 512;
/// Most stack words a call's pushed arguments are counted over.
const MAX_PUSHED_WORDS: i128 = 16;
/// Most stack depths kept apart at one native address and context.
const MAX_STACK_DEPTHS: usize = 4;
/// Salt of the context a re-entered interpreter region starts with.
const REGION_TAG: u64 = 0x7e00_0000_0000_0000;
/// Salt of the key of an in-function interpreter's VPC that points into a
/// copy of its bytecode on the stack.
const STACK_VPC_TAG: u64 = 0x7d00_0000_0000_0000;
/// Most instructions [`Explorer::probe_interpreter`] runs.
const PROBE_RUN_INSNS: usize = 4096;
/// Most times the probe runs one instruction.
const PROBE_VISITS: u32 = 64;
/// Instructions the probe runs without loading a bytecode pointer from a cell
/// before it gives up.
const PROBE_IDLE_INSNS: usize = 512;
/// Fetches at distinct places a computed jump needs to be a dispatch.
const PROBE_DISPATCHES: usize = 4;

/// A call leaving the interpreter: where it goes and where it comes back.
#[derive(Clone, Copy, Debug)]
struct Callee {
    target: Option<u64>,
    slot: Option<u64>,
    ret: Option<u64>,
}

/// A callee in the image as its code reads.
#[derive(Clone, Debug)]
struct CalleeScan {
    /// Register ranges some instruction writes.
    writes: Vec<(u64, u8)>,
    /// The bytes every `ret` pops beyond the return address.
    pop: Option<u32>,
    /// Every path was followed to a return without calling out or jumping
    /// indirectly, so `writes` is complete.
    closed: bool,
}

fn classify_space_of(vn: &kuna_num::pcoderaw::VarnodeData) -> Option<super::kuna_pushanemu::Space> {
    vn.space.as_ref().map(super::kuna_pushanemu::classify_space)
}

impl<'a> Explorer<'a> {
    pub fn new(target: &'a dyn PushanTarget, abi: Abi, config: Config, entry_addr: u64) -> Self {
        Explorer {
            target,
            abi,
            config,
            arena: ExprArena::new(),
            entry_addr,
            to_symbolize: BTreeMap::new(),
            sym_values: Default::default(),
            sym_known: BTreeMap::new(),
            sym_shrinks: BTreeMap::new(),
            sym_sources: BTreeMap::new(),
            image_ptrs: BTreeMap::new(),
            escapes: BTreeSet::new(),
            new_escapes: 0,
            pending_locs: None,
            pending_vpc_fork: None,
            callees: BTreeMap::new(),
            node_addrs: BTreeSet::new(),
            stack_depths: BTreeMap::new(),
            native_starts: BTreeSet::new(),
            native_covered: BTreeSet::new(),
            new_splits: 0,
            vm_entry_depth: None,
            direct_exit: false,
            arm_ctxs: Vec::new(),
            forget_fresh: BTreeSet::new(),
            fresh_epoch: BTreeSet::new(),
            forget_copies: BTreeSet::new(),
            sym_open: BTreeSet::new(),
            own_syms: BTreeMap::new(),
            call_epochs: 0,
            last_pop: None,
            last_clobbers: None,
            stats: Stats::default(),
            log: Vec::new(),
            retaddr: None,
            sp0: None,
            new_syms: 0,
            started: std::time::Instant::now(),
            timed_out: false,
            data_vpc: None,
            probe: true,
            pin: None,
            rep_copies: true,
            copy_escaped: false,
        }
    }

    fn initial_state(&mut self) -> State {
        let mut st = State::new(self.abi.big_endian, self.abi.flag_regs);
        let ps = self.abi.ptr_size;
        // The entry stack pointer's low bits follow the ABI (x64: a return
        // address was just pushed onto a 16-byte aligned stack), so alignment
        // masks on it fold away instead of breaking stack addressing.
        let low = if ps == 8 { 0xFFF8u128 } else { 0xFFFCu128 };
        self.arena.var_of_known(VarKind::InitReg { offset: self.abi.sp }, ps, false, (0xFFFF & !low, low));
        for &r in &self.abi.gprs {
            st.read_reg(&mut self.arena, r, ps);
        }
        let sp = st.read_reg(&mut self.arena, self.abi.sp, ps);
        self.arena.mark_pointer(sp);
        let sp_var = self.arena.as_var(sp).expect("sp is a variable");
        self.sp0 = Some(sp_var);
        let clear = self.target.user_stack_clear_bits() & super::kuna_pushanexpr::mask(ps);
        self.arena.sp_bound = (clear != 0).then_some((sp_var, clear));
        let ret = st.read_mem(&mut self.arena, self.target, Some(sp_var), 0, ps);
        self.retaddr = Some(ret);
        if let Some(df) = self.abi.df {
            let z = self.arena.cst(1, 0);
            st.write_reg(df, z, 1);
        }
        for &(off, v) in &self.abi.fixed_flags {
            let c = self.arena.cst(1, v as u128);
            st.write_reg(off, c, 1);
        }
        for &(off, size, v) in &self.abi.fixed_regs {
            let c = self.arena.cst(size, v as u128);
            st.write_reg(off, c, size);
        }
        st
    }

    /// Look for an interpreter that is the function's own body (no protector
    /// section): a short concrete run from the entry, with every byte of file
    /// data read as the file holds it and every undecided branch taken on its
    /// fall-through side, that reaches one computed jump at least
    /// [`PROBE_DISPATCHES`] times, each right after a fetch within
    /// [`FETCH_WINDOW`] bytes of a pointer loaded from one memory cell (a
    /// stack slot or a concrete address), the pointer at a different place
    /// in file data, or in a copy of file data on the stack, each time, and
    /// the jump going to at least two places. That cell is the virtual PC.
    /// The run stops at the first computed jump not preceded by such a fetch
    /// (an ordinary `switch`), at a return, after [`PROBE_RUN_INSNS`]
    /// instructions, after [`PROBE_IDLE_INSNS`] without a bytecode pointer
    /// loaded from a cell, or when one instruction ran [`PROBE_VISITS`] times.
    ///
    /// Several cells fetched through at one dispatch, each a fixed stride
    /// from the next, are the VPCs of interleaved bytecode programs (Tigress
    /// runs bogus functions round-robin with the real one, `pc[select]`).
    /// The selector is then every register or entry-stack slot that, where
    /// the straight run that loads the fetched cell starts, holds the index of
    /// that cell, on every turn: [`Interpreter::pin`].
    pub fn probe_interpreter(&mut self) -> Interpreter {
        let ps = self.abi.ptr_size;
        let saved = (self.escapes.clone(), self.call_epochs);
        let mut state = self.initial_state();
        let mut emu = Emulator::new(self.target);
        emu.track_sp = Some((self.abi.sp, ps));
        emu.trust_file = true;
        let mut addr = self.entry_addr;
        type Seen = (BTreeSet<i128>, BTreeSet<u64>, usize);
        let mut loads: Vec<(Cell, Option<VarId>, i128, Pin)> = Vec::new();
        let mut fetched: Vec<(Cell, i128)> = Vec::new();
        let mut last_head: Option<Pin> = None;
        let mut seen: BTreeMap<(u64, Cell), Seen> = BTreeMap::new();
        let mut visits: BTreeMap<u64, u32> = BTreeMap::new();
        let mut turns: Vec<Turn> = Vec::new();
        let mut run = (addr, Vec::new());
        let mut order = 0usize;
        let mut found: Option<u64> = None;
        let mut promising = false;
        for step in 0..PROBE_RUN_INSNS {
            let n = visits.entry(addr).or_insert(0);
            *n += 1;
            if *n > PROBE_VISITS || (step == PROBE_IDLE_INSNS && !promising) {
                break;
            }
            let Ok(insn) = self.target.lift(addr) else { break };
            if self.rep_copy(&mut state, &insn, true) {
                addr = insn.addr.wrapping_add(insn.len as u64);
                continue;
            }
            let mut trace = InsnTrace::default();
            let exit = emu.exec_insn(&mut state, &mut self.arena, &insn, &mut trace);
            if !trace.forked {
                for a in trace.accesses.iter().filter(|a| !a.is_store) {
                    let Some(at) = self.arena.affine_probe(a.addr_expr) else { continue };
                    if at.0.is_some() && at.0 != self.sp0 {
                        continue;
                    }
                    if let Some((cell, _, p, head)) = loads.iter().rev().find(|(c, b, p, _)| *c != at && *b == at.0 && at.1 >= *p && at.1 < *p + FETCH_WINDOW) {
                        if !fetched.contains(&(*cell, *p)) {
                            fetched.push((*cell, *p));
                        }
                        last_head = Some(head.clone());
                    }
                    if a.size != ps {
                        continue;
                    }
                    let Some(v) = trace.op_out.get(a.op_index).copied().flatten() else { continue };
                    if let Some((vb, p)) = self.arena.affine(v) {
                        if self.bytecode_pointer(&state, vb, p) {
                            promising = true;
                            loads.retain(|(c, _, _, _)| *c != at);
                            loads.push((at, vb, p, run.clone()));
                        }
                    }
                }
            }
            let computed = flow_op(&insn).is_some_and(|op| op.opc == OpCode::CPUI_BRANCHIND);
            addr = match exit {
                Exit::Fallthrough(n) => n,
                Exit::CondJump { fallthrough, .. } => fallthrough,
                Exit::Jump(t) => {
                    let Some(c) = self.arena.is_const(t).map(|c| c as u64) else { break };
                    if computed {
                        if fetched.is_empty() || !self.target.is_executable(c) {
                            break;
                        }
                        for &(cell, p) in &fetched {
                            let e = seen.entry((insn.addr, cell)).or_insert_with(|| (BTreeSet::new(), BTreeSet::new(), order));
                            order += 1;
                            e.0.insert(p);
                            e.1.insert(c);
                        }
                        let last: Vec<Cell> = fetched.last().map(|f| f.0).into_iter().collect();
                        let (h, vals) = last_head.take().unwrap_or_default();
                        turns.push((insn.addr, h, vals, last));
                        fetched.clear();
                        loads.clear();
                        if found.is_none() {
                            if let Some(((d, _), _)) = seen.iter().find(|(_, (p, t, _))| p.len() >= PROBE_DISPATCHES && t.len() >= 2) {
                                found = Some(*d);
                            }
                        }
                        if found.is_some() && turns.len() >= 3 * PROBE_DISPATCHES {
                            break;
                        }
                    }
                    run = (c, self.small_locations(&mut state));
                    c
                }
                Exit::Call { target, fallthrough } => {
                    let tc = self.arena.is_const(target).map(|c| c as u64);
                    self.apply_call_summary(&mut state, insn.addr, Callee { target: tc, slot: None, ret: Some(fallthrough) });
                    fallthrough
                }
                _ => break,
            };
        }
        (self.escapes, self.call_epochs) = saved;
        self.last_pop = None;
        self.last_clobbers = None;
        let Some(d) = found else { return Interpreter::default() };
        let mut cells: Vec<(usize, Cell)> = seen.iter().filter(|((a, _), (p, _, _))| *a == d && p.len() >= 2).map(|((_, c), (_, _, o))| (*o, *c)).collect();
        cells.sort();
        let cells: Vec<Cell> = cells.into_iter().map(|(_, c)| c).collect();
        let pin = if cells.len() > 1 { self.infer_selector(d, &cells, &turns) } else { None };
        self.log.push(format!("in-function interpreter: dispatch at {d:#x}, virtual PC cells {:x?}, selector {:x?}", cells.iter().map(|c| c.1).collect::<Vec<_>>(), pin));
        Interpreter { cells, pin }
    }

    /// The registers and 4-byte entry-stack slots holding a small constant
    /// (an index below 16).
    fn small_locations(&mut self, state: &mut State) -> SmallLocs {
        let mut out = Vec::new();
        for r in self.abi.gprs.clone() {
            if r == self.abi.sp || !state.regs.has_any(r as i128, 4) {
                continue;
            }
            let v = state.read_reg(&mut self.arena, r, 4);
            if let Some(c) = self.arena.is_const(v).filter(|c| *c < 16) {
                out.push((Loc::Reg { off: r }, c));
            }
        }
        let mut offs: Vec<i128> = state.mem_offsets(self.sp0).into_iter().map(|o| o.div_euclid(4) * 4).collect();
        offs.dedup();
        for o in offs {
            let v = state.read_mem(&mut self.arena, self.target, self.sp0, o, 4);
            if let Some(c) = self.arena.is_const(v).filter(|c| *c < 16) {
                out.push((Loc::Mem { base: self.sp0, off: o }, c));
            }
        }
        out
    }

    /// The selector of interleaved programs dispatched at `d` through
    /// `cells` (one per program, a fixed stride apart): the start of the run
    /// that loads the cell on most turns, and every location that holds the
    /// fetched cell's index there on each such turn, to be pinned to the
    /// index of the program dispatched first.
    fn infer_selector(&self, d: u64, cells: &[Cell], turns: &[Turn]) -> Option<Pin> {
        let mut sorted: Vec<Cell> = cells.to_vec();
        sorted.sort_by_key(|c| c.1);
        let stride = sorted[1].1 - sorted[0].1;
        if stride <= 0 || sorted.iter().any(|c| c.0 != sorted[0].0) || sorted.windows(2).any(|w| w[1].1 - w[0].1 != stride) {
            return None;
        }
        let index = |c: &Cell| ((c.1 - sorted[0].1) / stride) as u128;
        let mut heads: BTreeMap<u64, usize> = BTreeMap::new();
        for t in turns.iter().filter(|t| t.0 == d) {
            *heads.entry(t.1).or_insert(0) += 1;
        }
        let (&h, _) = heads.iter().max_by_key(|(_, n)| **n)?;
        let at_h: Vec<&Turn> = turns.iter().filter(|t| t.0 == d && t.1 == h && t.3.len() == 1).collect();
        if at_h.len() < 2 * cells.len() || cells.iter().any(|c| !at_h.iter().any(|t| t.3[0] == *c)) {
            return None;
        }
        let locs: SmallLocs = at_h[0]
            .2
            .iter()
            .map(|(l, _)| *l)
            .filter(|l| at_h.iter().all(|t| t.2.iter().any(|(m, v)| m == l && *v == index(&t.3[0]))))
            .map(|l| (l, index(&cells[0])))
            .collect();
        (!locs.is_empty()).then_some((h, locs))
    }

    /// `base + p` points into file data, or into a copy of file data on the
    /// entry stack: where an in-function interpreter's bytecode lives.
    fn bytecode_pointer(&self, state: &State, base: Option<VarId>, p: i128) -> bool {
        match base {
            None => u64::try_from(p).is_ok_and(|a| self.target.file_data(a)),
            Some(_) => base == self.sp0 && state.copy_source(base, p).is_some_and(|a| self.target.file_data(a)),
        }
    }

    /// The key of an in-function interpreter's VPC value: a pointer into file
    /// data is its address, one into a stack copy its tagged stack offset,
    /// and a select over a few constants in file data their hash.
    fn data_vpc_key(&mut self, state: &State, v: ExprId) -> Option<u64> {
        if let Some((b, p)) = self.arena.affine(v) {
            if !self.bytecode_pointer(state, b, p) {
                return None;
            }
            return Some(if b.is_none() { p as u64 } else { STACK_VPC_TAG ^ p as u64 });
        }
        let vals: Vec<u64> = self.arena.possible_values(v).filter(|vs| vs.len() <= MAX_VPC_FORK)?.into_iter().map(|c| c as u64).collect();
        vals.iter().all(|c| self.bytecode_pointer(state, None, *c as i128)).then(|| vpc_key(&vals))
    }

    /// Run exploration rounds to the symbolization fixpoint.
    pub fn run(&mut self) -> FlatCfg {
        self.arena.deadline = Some(self.started + std::time::Duration::from_secs(self.config.max_seconds));
        if self.probe && self.data_vpc.is_none() {
            let found = self.probe_interpreter();
            self.data_vpc = found.cells.first().copied();
            if let Some(pin) = found.pin {
                self.stats.woven = found.cells.len();
                self.pin = Some(pin);
            } else if found.cells.len() > 1 {
                self.log.push("interleaved bytecode programs without a selector to pin: other programs' dispatches stay unresolved".into());
            }
        }
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0, escapes: Vec::new(), sym_sources: BTreeMap::new() };
        let mut prev_nodes = usize::MAX;
        for round in 0..self.config.max_rounds {
            self.stats.rounds = round + 1;
            if round > 0 && round + 1 == self.config.max_rounds && self.new_syms > 0 {
                self.widen_all();
            }
            self.new_syms = 0;
            self.new_splits = 0;
            self.new_escapes = 0;
            if self.timed_out {
                break;
            }
            cfg = self.explore_round();
            cfg.escapes = self.escapes.iter().copied().collect();
            cfg.sym_sources = std::mem::take(&mut self.sym_sources);
            let n = cfg.nodes.len();
            self.log.push(format!(
                "round {}: {} nodes, {} new symbolized locations, {} unresolved, {} halts, {:.1}s",
                round + 1,
                n,
                self.new_syms,
                self.stats.unresolved,
                self.stats.halts,
                self.started.elapsed().as_secs_f64()
            ));
            if self.new_splits > 0 {
                self.log.push(format!("round {}: {} native blocks split", round + 1, self.new_splits));
            }
            if self.new_escapes > 0 {
                self.log.push(format!("round {}: {} frame offsets newly escaped", round + 1, self.new_escapes));
            }
            if self.new_syms == 0 && self.new_splits == 0 && self.new_escapes == 0 && (n == prev_nodes || round > 0) {
                break;
            }
            prev_nodes = n;
        }
        self.arena.deadline = None;
        self.stats.converged = self.new_syms == 0 && self.new_splits == 0 && self.new_escapes == 0 && !self.stats.cut_short;
        self.stats.nodes = cfg.nodes.len();
        self.stats.insns = cfg.nodes.iter().map(|n| n.insns.len()).sum();
        self.stats.vm_nodes = cfg.nodes.iter().filter(|n| n.entry_ctx.in_vm).count();
        self.stats.symbolized = self.to_symbolize.values().map(|s| s.len()).sum();
        if self.data_vpc.is_some() {
            self.stats.data_dispatches = cfg
                .nodes
                .iter()
                .filter(|n| matches!(n.exit, NodeExit::Branch(_)) && n.insns.last().and_then(|i| flow_op(i)).is_some_and(|op| op.opc == OpCode::CPUI_BRANCHIND))
                .count();
        }
        cfg
    }

    /// The exploration's time budget is spent: the round in progress is the
    /// last, cut short.  Checked between nodes and inside a node's
    /// instructions and merge, so no single step runs on past the budget (a
    /// case split stops at the same instant, [`ExprArena::deadline`]).
    fn out_of_time(&mut self) -> bool {
        if !self.timed_out && self.started.elapsed().as_secs() < self.config.max_seconds {
            return false;
        }
        if !self.timed_out {
            self.log.push(format!("time budget of {}s exhausted", self.config.max_seconds));
        }
        self.timed_out = true;
        self.stats.cut_short = true;
        true
    }

    /// The last round runs with every recorded location fully unknown and
    /// no recorded constants trusted, so a location still widening (a wide
    /// loop counter seen one more bit per round) folds nothing it cannot.
    fn widen_all(&mut self) {
        for k in self.sym_known.values_mut() {
            *k = (0, 0);
        }
        for (key, locs) in &self.to_symbolize {
            for l in locs.keys() {
                self.sym_open.insert((*key, *l));
            }
        }
        self.log.push("round limit reached: every recorded location widened".into());
    }

    fn explore_round(&mut self) -> FlatCfg {
        self.stats.unresolved = 0;
        self.stats.halts = 0;
        self.stats.misplaced_tail_calls = 0;
        self.stats.unmerged = 0;
        self.stats.cut_short = false;
        self.node_addrs.clear();
        self.stack_depths.clear();
        self.native_covered.clear();
        self.vm_entry_depth = None;
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0, escapes: Vec::new(), sym_sources: BTreeMap::new() };
        self.sym_sources.clear();
        let mut worklist: VecDeque<NodeId> = VecDeque::new();
        let init = self.initial_state();
        let key = NodeKey::at(self.entry_addr, None);
        let entry = self.reach(&mut cfg, &mut worklist, key, init, VmCtx::default(), None);
        cfg.entry = entry;
        let mut emu = Emulator::new(self.target);
        emu.sym_values = Rc::clone(&self.sym_values);
        emu.track_sp = Some((self.abi.sp, self.abi.ptr_size));
        emu.watch_base = if self.config.trace.is_some() { self.sp0 } else { None };
        emu.stack_base = if self.abi.stack_args > 0 { self.sp0 } else { None };
        emu.vpc = self.data_vpc.map(|(b, o)| VpcSource::Cell(b, o));
        while let Some(id) = worklist.pop_front() {
            if cfg.nodes.len() > self.config.max_nodes {
                self.log.push("node limit reached".into());
                self.stats.cut_short = true;
                break;
            }
            if self.out_of_time() {
                break;
            }
            self.emulate_node(&mut cfg, &mut worklist, &mut emu, id);
            self.stats.dropped_stores = emu.dropped_stores;
            self.stats.lost_sp = emu.lost_sp;
            if let Some(t) = &self.config.trace {
                use std::io::Write;
                let n = &cfg.nodes[id as usize];
                let _ = writeln!(
                    t.borrow_mut(),
                    "[{:>7.3}s] node {} key={:#x}/{} vm={} vpc_reg={:?} vpc_cell={:x?} insns={} exit={} arena={} mem_bytes={} worklist={}",
                    self.started.elapsed().as_secs_f64(),
                    id,
                    n.key.addr,
                    n.key.ctx.map(|c| format!("{c:#x}")).unwrap_or_else(|| "-".into()),
                    n.entry_ctx.in_vm,
                    n.entry_ctx.vpc_reg,
                    n.entry_ctx.vpc_cell,
                    n.insns.len(),
                    exit_brief(&n.exit),
                    self.arena.len(),
                    n.entry.mem_bytes(),
                    worklist.len()
                );
            }
        }
        self.stats.writable_bytecode |= emu.fetched_writable;
        self.stats.user_stack_assumed = self.arena.sp_bound_used;
        for n in std::mem::take(&mut self.arena.sp_bound_notes) {
            self.log.push(format!("identity over the user-mode stack range only: {n}"));
        }
        self.stats.copy_read_unfolded = emu.copy_read_unfolded || self.copy_escaped;
        if let Some(n) = emu.copy_read_note.take() {
            self.log.push(n);
        }
        if std::mem::take(&mut self.copy_escaped) {
            self.log.push("a pointer into a copy reached a call".into());
        }
        cfg
    }

    /// Look up or create the node for `key`, merging/recording the incoming state.
    fn reach(
        &mut self,
        cfg: &mut FlatCfg,
        worklist: &mut VecDeque<NodeId>,
        key: NodeKey,
        mut state: State,
        ctx: VmCtx,
        from: Option<NodeId>,
    ) -> NodeId {
        self.apply_pin(&mut state, key.addr);
        let mut key = self.at_depth(key, &mut state);
        key.holder = if ctx.vpc_cell.is_none() { ctx.vpc_reg } else { None };
        if let Some(&id) = cfg.index.get(&key) {
            let node = &mut cfg.nodes[id as usize];
            if let Some(f) = from {
                if !node.preds.contains(&f) {
                    node.preds.push(f);
                }
            }
            let mut entry = node.entry.clone();
            // States at different stack depths are not merged (angr's
            // symbolizer groups the states it merges by stack pointer).
            let ps = self.abi.ptr_size;
            let (sa, sb) = (entry.read_reg(&mut self.arena, self.abi.sp, ps), state.read_reg(&mut self.arena, self.abi.sp, ps));
            if sa != sb && !self.same_on_stack(sa, sb) {
                self.stats.unmerged += 1;
                if let Some(tr) = &self.config.trace {
                    use std::io::Write;
                    let _ = writeln!(tr.borrow_mut(), "        stack pointer differs at node {id}: not merged: entry {:x?} incoming {:x?}", self.arena.affine(sa), self.arena.affine(sb));
                }
                return id;
            }
            let diffs = entry.diff_constants(&mut state, &mut self.arena, self.target, self.abi.ptr_size);
            if let Some(locs) = self.to_symbolize.get(&key) {
                for l in locs.keys().copied().collect::<Vec<_>>() {
                    let a = read_loc(&mut entry, &mut self.arena, self.target, &l);
                    let b = read_loc(&mut state, &mut self.arena, self.target, &l);
                    if a != b {
                        if let Some(v) = self.arena.as_var(a) {
                            add_source(&mut self.sym_sources, v, b);
                        }
                    }
                }
            }
            let vpc_reg = node.entry_ctx.vpc_reg;
            let vpc_cell = node.entry_ctx.vpc_cell;
            if let Some(tr) = &self.config.trace {
                use std::io::Write;
                let base = self.sp0;
                for off in (-0x400i128..0x40).step_by(8) {
                    let a = entry.read_mem(&mut self.arena, self.target, base, off, 8);
                    let b = state.read_mem(&mut self.arena, self.target, base, off, 8);
                    if a == b {
                        continue;
                    }
                    let _ = writeln!(tr.borrow_mut(), "        rereach node {id} slot {off:#x}: entry {} | incoming {}", self.arena.fmt(a).chars().take(200).collect::<String>(), self.arena.fmt(b).chars().take(200).collect::<String>());
                }
                for d in &diffs {
                    let (a, b) = match d.loc.loc {
                        Loc::Reg { off } => (entry.read_reg(&mut self.arena, off, d.loc.size), state.read_reg(&mut self.arena, off, d.loc.size)),
                        Loc::Mem { base, off } => (
                            entry.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                            state.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                        ),
                    };
                    let _ = writeln!(
                        tr.borrow_mut(),
                        "        diff at node {id} {:?}: entry {} | incoming {}",
                        d.loc.loc,
                        self.arena.fmt(a).chars().take(300).collect::<String>(),
                        self.arena.fmt(b).chars().take(300).collect::<String>()
                    );
                }
            }
            let mut kept = Vec::with_capacity(diffs.len());
            for d in diffs {
                if self.out_of_time() {
                    return id;
                }
                let (a, b) = match d.loc.loc {
                    Loc::Reg { off } => (entry.read_reg(&mut self.arena, off, d.loc.size), state.read_reg(&mut self.arena, off, d.loc.size)),
                    Loc::Mem { base, off } => (
                        entry.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                        state.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                    ),
                };
                if !self.same_on_stack(a, b) {
                    kept.push((d, a, b));
                }
            }
            let diffs = kept;
            // Paths that left different external calls last disagree on all
            // mutable memory they did not read: it starts unknown at the node.
            if entry.global_epoch != state.global_epoch && self.fresh_epoch.insert(key) {
                self.new_syms += 1;
            }
            // Paths that copied different image memory disagree on every
            // unwritten byte of the copies: the node starts without them.
            if entry.copies != state.copies && self.forget_copies.insert(key) {
                self.new_syms += 1;
            }
            // The VPC is part of the key and never symbolized, as long as the
            // incoming path's VPC is the one the key names.
            let vpc_keyed = match (vpc_reg, vpc_cell) {
                (_, Some((b, a))) => {
                    let v = state.read_mem(&mut self.arena, self.target, b, a, self.abi.ptr_size);
                    if self.data_vpc == Some((b, a)) {
                        key.ctx.is_some() && self.data_vpc_key(&state, v) == key.ctx
                    } else {
                        self.keys_vpc(v, key.ctx)
                    }
                }
                (Some(r), None) => {
                    let v = state.read_reg(&mut self.arena, r, self.abi.ptr_size);
                    self.keys_vpc(v, key.ctx)
                }
                _ => false,
            };
            let diffs = match vpc_cell.filter(|_| vpc_keyed) {
                Some(cell) => self.outside_vpc_cell(&mut entry, &mut state, diffs, cell),
                None => diffs,
            };
            for (d, a, b) in diffs {
                if self.out_of_time() {
                    return id;
                }
                let sides = [a, b].map(|v| self.side_values(v, key, &d.loc));
                if let Loc::Reg { off } = d.loc.loc {
                    if vpc_keyed && vpc_reg.is_some_and(|r| off >= r && off < r + self.abi.ptr_size as u64) {
                        continue;
                    }
                }
                if let (Loc::Reg { off }, Some((pc, pcs))) = (d.loc.loc, self.abi.pc) {
                    if off >= pc && off < pc + pcs as u64 {
                        continue;
                    }
                }
                if let Loc::Mem { base, off } = d.loc.loc {
                    if vpc_keyed && vpc_cell.is_some_and(|(b, a)| b == base && off < a + self.abi.ptr_size as i128 && a < off + d.loc.size as i128) {
                        continue;
                    }
                }
                // Memory under a base that is a fresh variable every round (a
                // call's result) cannot be named again: the node starts with
                // every such region forgotten instead.
                if let Loc::Mem { base: Some(b), .. } = d.loc.loc {
                    if Some(b) != self.sp0 && !self.arena.is_stable_var(b) {
                        if self.forget_fresh.insert(key) {
                            self.new_syms += 1;
                        }
                        continue;
                    }
                }
                // A difference inside a location already recorded is that location.
                let set = self.to_symbolize.entry(key).or_default();
                let mut d = d;
                if let Some(cover) = set.keys().find(|l| covers(l, &d.loc)).copied() {
                    d.loc = cover;
                }
                let is_new = !set.contains_key(&d.loc);
                let k = self.sym_known.entry((key, d.loc)).or_insert(d.known);
                let mut narrowed = (k.0 & d.known.0, k.1 & d.known.1);
                let shrank = narrowed != *k;
                if shrank {
                    let n = self.sym_shrinks.entry((key, d.loc)).or_insert(0);
                    *n = n.saturating_add(1);
                    if *n >= 2 {
                        let lost = (k.0 & !narrowed.0) | (k.1 & !narrowed.1);
                        let keep = (1u128 << lost.trailing_zeros().min(127)) - 1;
                        narrowed = (narrowed.0 & keep, narrowed.1 & keep);
                    }
                }
                *k = narrowed;
                let values = set.entry(d.loc).or_default();
                let mut grew = false;
                let mut open = false;
                for side in sides {
                    match side {
                        Some(vs) => {
                            for v in vs {
                                if values.contains(&v) {
                                    continue;
                                }
                                if values.len() < MAX_SYM_VALUES {
                                    values.insert(v);
                                    grew = true;
                                } else {
                                    open = true;
                                }
                            }
                        }
                        None => open = true,
                    }
                }
                // The constants seen stand for every value the location
                // takes only while no path brought anything else.
                if open && self.sym_open.insert((key, d.loc)) {
                    grew = true;
                }
                if is_new || grew || shrank {
                    self.new_syms += 1;
                    if self.stats.rounds > 3 && self.log.len() < 400 {
                        self.log.push(format!("  round {} new sym at {:#x}/{:?}: {:?} new={is_new} grew={grew} shrank={shrank} known={:x?}", self.stats.rounds, key.addr, key.ctx, d.loc, d.known));
                    }
                }
            }
            return id;
        }
        if self.forget_fresh.contains(&key) {
            let arena = &self.arena;
            let sp0 = self.sp0;
            state.mem.retain(|b, _| b.is_none() || *b == sp0 || b.is_some_and(|v| arena.is_stable_var(v)));
        }
        if self.fresh_epoch.contains(&key) {
            self.call_epochs += 1;
            self.clobber_call(&mut state);
        }
        if self.forget_copies.contains(&key) {
            state.copies.clear();
        }
        if let Some(locs) = self.to_symbolize.get(&key) {
            let with_known: Vec<(SymLoc, (u128, u128))> =
                locs.keys().map(|l| (*l, self.sym_known.get(&(key, *l)).copied().unwrap_or((0, 0)))).collect();
            let tag = key_tag(&key);
            let before: Vec<ExprId> = with_known.iter().map(|(l, _)| read_loc(&mut state, &mut self.arena, self.target, l)).collect();
            let made = state.symbolize_known(&mut self.arena, tag, &with_known);
            for (&(_, e), &b) in made.iter().zip(&before) {
                if let Some(v) = self.arena.as_var(e).filter(|_| e != b) {
                    add_source(&mut self.sym_sources, v, b);
                }
            }
            for (l, e) in made {
                if let Some(tr) = &self.config.trace {
                    use std::io::Write;
                    let _ = writeln!(tr.borrow_mut(), "        symbolize {:#x}/{:?} {:?} -> {} known={:x?} vals={:x?}", key.addr, key.ctx, l, self.arena.fmt(e), self.sym_known.get(&(key, l)), locs.get(&l));
                }
                if let Some(v) = self.arena.as_var(e) {
                    self.own_syms.insert(v, (key, l));
                    match locs.get(&l).filter(|_| !self.sym_open.contains(&(key, l))) {
                        Some(vals) => self.sym_values.borrow_mut().insert(v, vals.clone()),
                        None => self.sym_values.borrow_mut().remove(&v),
                    };
                }
            }
        }
        let id = cfg.nodes.len() as NodeId;
        cfg.nodes.push(Node {
            call_pop: None,
            call_clobbers: None,
            exit_locs: None,
            vpc_fork: None,
            key,
            entry: state,
            entry_ctx: ctx,
            insns: Vec::new(),
            traces: Vec::new(),
            exit: NodeExit::Halt("unexplored".into()),
            preds: from.into_iter().collect(),
        });
        cfg.index.insert(key, id);
        self.node_addrs.insert(key.addr);
        if !ctx.in_vm && !self.target.is_foreign_code(key.addr) && self.native_starts.insert(key.addr) && self.native_covered.contains(&key.addr) {
            self.new_splits += 1;
        }
        worklist.push_back(id);
        id
    }

    /// The differences of a merge outside the VPC cell, which the key names:
    /// a slot only partly in the cell is narrowed to its bytes outside it.
    /// Themida keeps a branch handler's condition byte right after the cell,
    /// and dropping the whole slot drops the branch.
    fn outside_vpc_cell(&mut self, entry: &mut State, state: &mut State, diffs: Vec<(Diff, ExprId, ExprId)>, cell: Cell) -> Vec<(Diff, ExprId, ExprId)> {
        let (cb, ca) = cell;
        let ce = ca + self.abi.ptr_size as i128;
        let mut out = Vec::with_capacity(diffs.len());
        for (d, a, b) in diffs {
            let Loc::Mem { base, off } = d.loc.loc else {
                out.push((d, a, b));
                continue;
            };
            let end = off + d.loc.size as i128;
            if base != cb || end <= ca || ce <= off {
                out.push((d, a, b));
                continue;
            }
            for (lo, hi) in [(off, ca.min(end)), (ce.max(off), end)] {
                if lo >= hi {
                    continue;
                }
                let len = (hi - lo) as u8;
                let x = entry.read_mem(&mut self.arena, self.target, base, lo, len);
                let y = state.read_mem(&mut self.arena, self.target, base, lo, len);
                if super::kuna_pushanstate::same_value(&mut self.arena, x, y) {
                    continue;
                }
                let ((zx, ox), (zy, oy)) = (self.arena.known_bits(x), self.arena.known_bits(y));
                let (cx, cy) = (self.arena.is_const(x), self.arena.is_const(y));
                let loc = SymLoc { loc: Loc::Mem { base, off: lo }, size: len };
                out.push((Diff { loc, values: cx.zip(cy), entry_const: cx, incoming_const: cy, known: (zx & zy, ox & oy) }, x, y));
            }
        }
        out
    }

    /// `key` at the incoming stack depth.  Paths that reach one handler with
    /// one virtual PC at different stack depths are different nodes: VMProtect
    /// places its register context below the stack the last re-entry found,
    /// so after a call that left a different number of argument words the
    /// same bytecode runs with its context elsewhere, and neither state can
    /// stand for the other (angr likewise merges only states at the same
    /// stack pointer).  At most [`MAX_STACK_DEPTHS`] depths are kept apart
    /// per address and context; a further one is reached at the first.
    fn at_depth(&mut self, mut key: NodeKey, state: &mut State) -> NodeKey {
        let depth = self.sp_depth(state);
        let seen = self.stack_depths.entry((key.addr, key.ctx)).or_default();
        key.sp = if seen.contains(&depth) {
            depth
        } else if seen.len() < MAX_STACK_DEPTHS {
            seen.push(depth);
            depth
        } else {
            seen[0]
        };
        key
    }

    /// The stack pointer's offset from its entry value, when it has one.
    fn sp_depth(&mut self, state: &mut State) -> Option<i128> {
        let sp = state.read_reg(&mut self.arena, self.abi.sp, self.abi.ptr_size);
        match self.arena.affine_probe(sp) {
            Some((Some(b), k)) if Some(b) == self.sp0 => Some(k),
            _ => None,
        }
    }

    /// Emulate one node.  An interpreter instruction that branches back to
    /// itself with a decided guard (a `rep movsb` with a concrete count)
    /// repeats inside the node, one instance per iteration, as angr's
    /// rep-movsb shortcut does, instead of re-reaching its own node and
    /// symbolizing its counters.  Such an instruction of the function's own
    /// code starts a node of its own and loops there, so the program's copy
    /// loop stays a loop over its source pointer.  A node of the function's
    /// own code outside the interpreter ends where another such node starts,
    /// so paths that join inside a straight run of native code share its
    /// tail, as a disassembler's basic blocks do.
    fn emulate_node(&mut self, cfg: &mut FlatCfg, worklist: &mut VecDeque<NodeId>, emu: &mut Emulator<'a>, id: NodeId) {
        let mut state = cfg.nodes[id as usize].entry.clone();
        let ctx = cfg.nodes[id as usize].entry_ctx;
        let mut addr = cfg.nodes[id as usize].key.addr;
        let mut insns: Vec<Rc<LiftedInsn>> = Vec::new();
        let mut traces: Vec<InsnTrace> = Vec::new();
        let mut exit;
        let native = !ctx.in_vm && !self.target.is_foreign_code(addr);
        loop {
            if !insns.is_empty() && self.out_of_time() {
                exit = Exit::Halt("exploration budget exhausted inside the node".into());
                break;
            }
            if native && !insns.is_empty() && self.native_starts.contains(&addr) {
                exit = Exit::Fallthrough(addr);
                break;
            }
            if !self.target.is_mapped(addr) {
                exit = Exit::Halt(format!("flow into unmapped address {addr:#x}"));
                break;
            }
            let insn = match self.target.lift(addr) {
                Ok(i) => i,
                Err(e) => {
                    exit = Exit::Halt(format!("decode failure at {addr:#x}: {e}"));
                    break;
                }
            };
            self.apply_pin(&mut state, addr);
            if self.rep_copy(&mut state, &insn, false) {
                let next = insn.addr.wrapping_add(insn.len as u64);
                insns.push(insn);
                traces.push(InsnTrace { elided: true, ..InsnTrace::default() });
                addr = next;
                continue;
            }
            if !insns.is_empty() && !self.target.is_foreign_code(addr) && branches_to_itself(&insn) {
                exit = Exit::Jump(self.arena.cst(self.abi.ptr_size, addr as u128));
                break;
            }
            let mut trace = InsnTrace::default();
            if let Some(t) = &self.config.trace {
                use std::io::Write;
                let _ = writeln!(
                    t.borrow_mut(),
                    "    insn {:#x} {} ops={} arena={}",
                    insn.addr,
                    insn.asm,
                    insn.ops.len(),
                    self.arena.len()
                );
            }
            exit = emu.exec_insn(&mut state, &mut self.arena, &insn, &mut trace);
            if trace.forked && emu.insn_copy_reads > 0 {
                emu.copy_read_unfolded = true;
                emu.copy_read_note.get_or_insert_with(|| format!("copy read in the forked instruction at {:#x}", insn.addr));
            }
            if let Some(t) = &self.config.trace {
                use std::io::Write;
                for n in &trace.notes {
                    let _ = writeln!(t.borrow_mut(), "        note: {}", n.chars().take(300).collect::<String>());
                }
                for u in &trace.userops {
                    let _ = writeln!(t.borrow_mut(), "        userop: {u}");
                }
                if let Ok(mode) = std::env::var("KUNA_PUSHAN_TRACE_OPS") {
                    for (i, op) in insn.ops.iter().enumerate() {
                        if let Some(Some(v)) = trace.op_out.get(i) {
                            if mode == "all" || self.arena.is_const(*v).is_none() {
                                let _ = writeln!(
                                    t.borrow_mut(),
                                    "        op{} {:?} -> {}",
                                    i,
                                    op.opc,
                                    self.arena.fmt(*v).chars().take(500).collect::<String>()
                                );
                            }
                        }
                    }
                }
                for a in &trace.accesses {
                    let _ = writeln!(
                        t.borrow_mut(),
                        "        {} via={:?} addr={} size={} foreign={}",
                        if a.is_store { "store" } else { "load" },
                        a.via.map(|(r, d)| format!("(reg{r:x}{d:+})")),
                        a.addr.map(|x| format!("{x:#x}")).unwrap_or_else(|| "sym".into()),
                        a.size,
                        a.addr.map(|x| self.target.bytecode_like(x)).unwrap_or(false)
                    );
                }
            }
            let (here, after) = (insn.addr, insn.addr.wrapping_add(insn.len as u64));
            let repeated = insns.last().is_some_and(|p| p.addr == here);
            let forked = trace.forked;
            insns.push(insn);
            traces.push(trace);
            if let (Exit::Jump(t), false) = (&exit, forked) {
                match self.arena.is_const(*t).map(|c| c as u64) {
                    Some(c) if c == here && ctx.in_vm && self.target.is_foreign_code(here) && insns.len() < self.config.max_block_insns => continue,
                    Some(c) if c == after && repeated => exit = Exit::Fallthrough(after),
                    Some(c) if native && insns.len() < self.config.max_block_insns && self.moves_stack_back_to(&mut state, &insns, &traces, c) => {
                        addr = c;
                        continue;
                    }
                    _ => {}
                }
            }
            match exit {
                Exit::Fallthrough(next) => {
                    if insns.len() >= self.config.max_block_insns {
                        let t = self.arena.cst(self.abi.ptr_size, next as u128);
                        exit = Exit::Jump(t);
                        break;
                    }
                    addr = next;
                }
                _ => break,
            }
        }
        if native {
            self.native_covered.extend(insns.iter().skip(1).map(|i| i.addr));
        }
        self.image_ptrs.append(&mut emu.image_loads);
        // A conditional that only skips to the next instruction (a CMOV) joins
        // the fall-through state before the context is read from it, and every
        // other arm reads its context from its own state.
        let mut arm_states: Vec<Option<State>> = Vec::new();
        if let Exit::CondJump { arms, fallthrough } = &exit {
            for (cond, target, taken) in arms {
                if self.arena.is_const(*target) == Some(*fallthrough as u128) {
                    merge_ite(&mut state, &mut taken.as_ref().clone(), &mut self.arena, self.target, *cond);
                    arm_states.push(None);
                } else {
                    arm_states.push(Some(taken.as_ref().clone()));
                }
            }
        }
        let next_ctx = self.successor_ctx(&mut state, &traces, ctx, &insns);
        self.arm_ctxs = arm_states
            .into_iter()
            .map(|s| match s {
                Some(mut st) => self.successor_ctx(&mut st, &traces, ctx, &insns),
                None => next_ctx,
            })
            .collect();
        self.direct_exit = insns.last().and_then(|i| i.ops.iter().rev().find(|op| matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))).is_some_and(|op| op.opc == OpCode::CPUI_BRANCH && classify_space_of(&op.ins[0]) == Some(super::kuna_pushanemu::Space::Ram));
        let node_exit = match self.fork_on_vpc(cfg, worklist, id, &exit, &mut state, next_ctx) {
            Some(e) => e,
            None => self.resolve_exit(cfg, worklist, id, exit, state, next_ctx, &insns),
        };
        self.arm_ctxs.clear();
        let exit_locs = if matches!(node_exit, NodeExit::Select { .. }) { self.pending_locs.take() } else { None };
        let vpc_fork = if matches!(node_exit, NodeExit::Select { .. }) { self.pending_vpc_fork.take() } else { None };
        self.pending_locs = None;
        self.pending_vpc_fork = None;
        let call_pop = if matches!(node_exit, NodeExit::Call { .. }) { self.last_pop.take() } else { None };
        let call_clobbers = if matches!(node_exit, NodeExit::Call { .. }) { self.last_clobbers.take() } else { None };
        self.last_pop = None;
        self.last_clobbers = None;
        let node = &mut cfg.nodes[id as usize];
        node.call_pop = call_pop;
        node.call_clobbers = call_clobbers;
        node.insns = insns;
        node.traces = traces;
        node.exit = node_exit;
        node.exit_locs = exit_locs;
        node.vpc_fork = vpc_fork;
    }

    /// A `rep movs` of the function's own code that copies a constant count
    /// of elements out of file data (direction flag clear) to the entry
    /// stack or a concrete address, as gcc inlines a small `memcpy` (Tigress's
    /// threaded-code array): one [`State::copy_in`], the count register
    /// cleared and both pointers advanced past the copy, in place of one
    /// emulated iteration per element. Only in the probe and in the
    /// exploration of an in-function interpreter. The emitter leaves the
    /// instruction out when only VPC fetches it folds read the copy and no
    /// pointer into it reached a call.
    fn rep_copy(&mut self, state: &mut State, insn: &LiftedInsn, probing: bool) -> bool {
        if !probing && (!self.rep_copies || self.data_vpc.is_none()) || self.target.is_foreign_code(insn.addr) {
            return false;
        }
        let m = insn.asm.to_ascii_uppercase();
        let w: u64 = match m.split_whitespace().next() {
            Some("MOVSB.REP") => 1,
            Some("MOVSW.REP") => 2,
            Some("MOVSD.REP") => 4,
            Some("MOVSQ.REP") => 8,
            _ => return false,
        };
        let ps = self.abi.ptr_size;
        let (Some(&cx), Some(&si), Some(&di)) = (self.abi.gprs.get(1), self.abi.gprs.get(6), self.abi.gprs.get(7)) else { return false };
        if let Some(df) = self.abi.df {
            let d = state.read_reg(&mut self.arena, df, 1);
            if self.arena.is_const(d) != Some(0) {
                return false;
            }
        }
        let (n, src, dst) = (state.read_reg(&mut self.arena, cx, ps), state.read_reg(&mut self.arena, si, ps), state.read_reg(&mut self.arena, di, ps));
        let Some(n) = self.arena.is_const(n).map(|n| n as u64).filter(|n| *n > 0 && n * w <= 1 << 24) else { return false };
        let len = n * w;
        let Some((None, s)) = self.arena.affine(src) else { return false };
        let Some((b, d)) = self.arena.affine_probe(dst) else { return false };
        let Ok(s) = u64::try_from(s) else { return false };
        if !(b.is_none() || b == self.sp0) || !self.target.file_data(s) || !self.target.file_data(s + len - 1) || !state.copy_in(b, d, s, len) {
            return false;
        }
        let zero = self.arena.cst(ps, 0);
        state.write_reg(cx, zero, ps);
        let k = self.arena.cst(ps, len as u128);
        let s2 = self.arena.mk_bin(OpCode::CPUI_INT_ADD, src, k, ps);
        let d2 = self.arena.mk_bin(OpCode::CPUI_INT_ADD, dst, k, ps);
        state.write_reg(si, s2, ps);
        state.write_reg(di, d2, ps);
        self.stats.rep_copies += 1;
        true
    }

    /// Where the run into an interleaved interpreter's dispatch starts, set
    /// its selector to the first program's index, so every turn runs that
    /// program and the node key holds its VPC alone (Pushan pins Tigress's
    /// selector the same way).
    fn apply_pin(&mut self, state: &mut State, addr: u64) {
        let Some((h, locs)) = &self.pin else { return };
        if *h != addr {
            return;
        }
        for (l, v) in locs.clone() {
            let c = self.arena.cst(4, v);
            match l {
                Loc::Reg { off } => state.write_reg(off, c, 4),
                Loc::Mem { base, off } => state.write_mem(base, off, c, 4),
            }
        }
    }

    /// A decided jump of the function's own code back to `c`, an earlier
    /// instruction of the node, after which the stack pointer sits at another
    /// constant depth than it did there: a stack probe (`sub rsp, 0x1000; or
    /// [rsp], 0; cmp rsp, r11; jne`), which repeats inside the node, one
    /// instance per iteration, since keying each iteration at its own depth
    /// would exhaust [`MAX_STACK_DEPTHS`].
    fn moves_stack_back_to(&mut self, state: &mut State, insns: &[Rc<LiftedInsn>], traces: &[InsnTrace], c: u64) -> bool {
        let Some(j) = insns.iter().position(|i| i.addr == c) else { return false };
        let Some(Some(before)) = traces.get(j).and_then(|t| t.sp_before.first().copied()) else { return false };
        let now = state.read_reg(&mut self.arena, self.abi.sp, self.abi.ptr_size);
        match (self.arena.affine(before), self.arena.affine(now)) {
            (Some((Some(a), x)), Some((Some(b), y))) => Some(a) == self.sp0 && a == b && x != y,
            _ => false,
        }
    }

    /// Two values built over the entry stack pointer that agree wherever the
    /// stack is (the interpreter rebuilds a saved stack address through
    /// different arithmetic on two paths).  angr compares merged values under
    /// its stack-pointer preconstraint, so such a pair is never a difference.
    /// Their difference is tested for the constant 0 by the identity test
    /// over every variable; values with comparisons in them that it does not
    /// equate still get the older two-point test over the stack pointer.
    fn same_on_stack(&mut self, a: ExprId, b: ExprId) -> bool {
        if self.arena.is_const(a).is_some() || self.arena.is_const(b).is_some() {
            return false;
        }
        if self.arena.size(a) == self.arena.size(b) {
            let d = self.arena.mk_bin(OpCode::CPUI_INT_SUB, a, b, self.arena.size(a));
            if self.arena.is_const(d) == Some(0) || self.arena.identity_probe(d) == Some((None, 0)) {
                return true;
            }
            if self.arena.bool_atoms(a).is_empty() && self.arena.bool_atoms(b).is_empty() {
                return false;
            }
        }
        let Some(sp) = self.sp0.and_then(|v| self.arena.var_expr(v)) else { return false };
        let low: u128 = if self.abi.ptr_size == 8 { 0xFFF8 } else { 0xFFFC };
        for hi in [0x0012_0000u128, 0x5678_0000] {
            let k = self.arena.cst(self.abi.ptr_size, hi | low);
            let mut map = BTreeMap::new();
            map.insert(sp, k);
            let (x, y) = (self.arena.substitute(a, &map), self.arena.substitute(b, &map));
            match (self.arena.is_const(x), self.arena.is_const(y)) {
                (Some(p), Some(q)) if p == q => {}
                _ => return false,
            }
        }
        true
    }

    /// A register or entry-stack slot holding each variable the atoms read,
    /// in `state`, or else holding it plus a constant (a loop counter the
    /// interpreter already advanced past the value its guard tested).  A
    /// variable that was only ever in flight has no entry.
    fn locate_vars(&mut self, state: &mut State, atoms: &[ExprId]) -> Option<(Vec<(VarId, Loc, i128)>, Option<i128>)> {
        let mut vars = BTreeSet::new();
        for a in atoms {
            vars.extend(self.arena.free_vars(*a));
        }
        let ps = self.abi.ptr_size;
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let sp_off = match self.arena.affine(sp) {
            Some((Some(b), k)) if Some(b) == self.sp0 => Some(k),
            _ => None,
        };
        let mut out = Vec::with_capacity(vars.len());
        for v in vars {
            let info = *self.arena.var_info(v);
            let size = info.size;
            let target = self.arena.var_expr(v)?;
            let mut found = None;
            let mut shifted = None;
            let mut regs: Vec<u64> = self.abi.gprs.clone();
            regs.extend(self.abi.callee_saved.iter().map(|(o, _)| *o));
            let note = |arena: &ExprArena, value: ExprId, loc: Loc, shifted: &mut Option<(Loc, i128)>| {
                if value == target {
                    return true;
                }
                if shifted.is_none() {
                    if let Some((Some(b), k)) = arena.affine(value) {
                        if b == v {
                            *shifted = Some((loc, k));
                        }
                    }
                }
                false
            };
            for r in regs {
                if r == self.abi.sp {
                    continue;
                }
                let value = state.read_reg(&mut self.arena, r, size);
                if note(&self.arena, value, Loc::Reg { off: r }, &mut shifted) {
                    found = Some((Loc::Reg { off: r }, 0));
                    break;
                }
            }
            if found.is_none() && sp_off.is_some() {
                let offs: Vec<i128> = state.mem_offsets(self.sp0).into_iter().filter(|o| o.rem_euclid(size as i128) == 0).collect();
                for o in offs {
                    let value = state.read_mem(&mut self.arena, self.target, self.sp0, o, size);
                    let loc = Loc::Mem { base: self.sp0, off: o };
                    if note(&self.arena, value, loc, &mut shifted) {
                        found = Some((loc, 0));
                        break;
                    }
                }
            }
            if let Some((loc, k)) = found.or(shifted) {
                out.push((v, loc, k));
            }
        }
        Some((out, sp_off))
    }

    /// A virtual PC decided by boolean atoms (the interpreter's conditional
    /// jump computes both successors and selects one arithmetically): split
    /// the state there, one successor per value with its atoms pinned, so
    /// each path decodes its own bytecode with a concrete context.  This is
    /// the state split angr's emulator performs on a two-valued expression.
    fn fork_on_vpc(
        &mut self,
        cfg: &mut FlatCfg,
        worklist: &mut VecDeque<NodeId>,
        id: NodeId,
        exit: &Exit,
        state: &mut State,
        ctx: VmCtx,
    ) -> Option<NodeExit> {
        if !matches!(exit, Exit::Fallthrough(_) | Exit::Jump(_)) {
            return None;
        }
        let v = self.vpc_expr(state, &ctx)?;
        if self.arena.is_const(v).is_some() {
            return None;
        }
        // A VPC still symbolic under its atoms may be decided by merge
        // symbols whose recorded constants are all they held.
        let cases = match self.arena.case_split(v) {
            Some(cs) if cs.iter().all(|(_, e)| self.arena.is_const(*e).is_some()) => cs,
            _ => split_over_symbols(&mut self.arena, &self.sym_values, v)?,
        };
        let mut groups: Vec<(u64, Vec<Vec<(ExprId, bool)>>)> = Vec::new();
        for (assign, e) in &cases {
            let c = self.arena.is_const(*e)? as u64;
            if !self.target.is_mapped(c) {
                return None;
            }
            match groups.iter_mut().find(|(k, _)| *k == c) {
                Some((_, a)) => a.push(assign.clone()),
                None => groups.push((c, vec![assign.clone()])),
            }
        }
        if let [(c, _)] = groups[..] {
            // Every assignment of its atoms gives the same virtual PC: the
            // register or cell holds that constant from here on.
            let mut map = BTreeMap::new();
            map.insert(v, self.arena.cst(self.arena.size(v), c as u128));
            state.map_values(&mut self.arena, &mut |a, e| a.substitute(e, &map));
            return None;
        }
        if groups.len() < 2 || groups.len() > MAX_VPC_FORK {
            return None;
        }
        let mut atoms: Vec<ExprId> = Vec::new();
        for (assign, _) in &cases {
            for (a, _) in assign {
                if !atoms.contains(a) {
                    atoms.push(*a);
                }
            }
        }
        // Every arm must reach a single node; check the targets first.
        let mut plans = Vec::with_capacity(groups.len());
        for (c, assigns) in &groups {
            let mut map = BTreeMap::new();
            for a in &atoms {
                let value_in = |as_: &Vec<(ExprId, bool)>| as_.iter().find(|(x, _)| x == a).map(|(_, v)| *v);
                let Some(first) = value_in(&assigns[0]) else { continue };
                if assigns.iter().all(|as_| value_in(as_) == Some(first)) {
                    map.insert(*a, self.arena.cst(1, first as u128));
                    if let Some((sym, k)) = first.then(|| symbol_pin(&mut self.arena, &self.sym_values, *a)).flatten() {
                        map.insert(sym, k);
                    }
                }
            }
            // Whatever else the arm assumes, the virtual PC itself is `c` on it.
            let size = self.arena.size(v);
            map.insert(v, self.arena.cst(size, *c as u128));
            let next = match exit {
                Exit::Fallthrough(n) => *n,
                Exit::Jump(t) => {
                    let t2 = self.arena.substitute(*t, &map);
                    self.arena.is_const(t2)? as u64
                }
                _ => return None,
            };
            if self.classify_target(next).is_err() {
                return None;
            }
            plans.push((*c, assigns.clone(), map, next));
        }
        if let Some(tr) = &self.config.trace {
            use std::io::Write;
            let _ = writeln!(tr.borrow_mut(), "        fork on vpc {} into {:x?}", self.arena.fmt(v).chars().take(400).collect::<String>(), plans.iter().map(|p| p.0).collect::<Vec<_>>());
            for a in &atoms {
                let vars: Vec<String> = self.arena.free_vars(*a).iter().map(|v| format!("{:?}", self.arena.var_info(*v).kind)).collect();
                let _ = writeln!(tr.borrow_mut(), "          atom {} vars {:?}", self.arena.fmt(*a).chars().take(600).collect::<String>(), vars);
            }
        }
        let mut targets = Vec::with_capacity(plans.len());
        let mut vpcs = Vec::with_capacity(plans.len());
        for (c, assigns, map, next) in plans {
            vpcs.push(c);
            let mut st = state.clone();
            st.map_values(&mut self.arena, &mut |a, e| a.substitute(e, &map));
            let mut sctx = ctx;
            sctx.ctx = Some(c);
            let succ = match self.jump_to(cfg, worklist, id, next, st, sctx) {
                NodeExit::Branch(n) => n,
                other => self.trampoline(cfg, id, next, other),
            };
            targets.push((assigns, succ));
        }
        let mut atoms = atoms;
        fuse_signed_compares(&mut self.arena, &mut atoms, &mut targets);
        self.pending_locs = self.locate_vars(state, &atoms);
        let holder = match (ctx.vpc_cell, ctx.vpc_reg) {
            (Some((base, off)), _) => Some(Loc::Mem { base, off }),
            (None, Some(r)) => Some(Loc::Reg { off: r }),
            _ => None,
        };
        self.pending_vpc_fork = holder.map(|h| (h, vpcs));
        Some(NodeExit::Select { atoms, targets, by_atoms: true })
    }

    /// The interpreter context the block hands to its successors (angr's
    /// find_vip).  A VPC value must be unwritten, high-entropy memory in a
    /// non-writable section of the image.  A concrete memory cell the block loaded a pointer
    /// from and fetched bytecode through (Themida keeps the VPC in its VM
    /// context) is the VPC for as long as it holds unwritten image memory.  Otherwise a
    /// register is a candidate when the block fetched through it: the
    /// previous VPC register wins while it qualifies, then the candidate
    /// nearest the previous context, then the one fetched through most.
    /// Failing that, the previous register's value is followed while it stays
    /// unwritten image memory other than handler code (a handler's address
    /// parked in the register never keys a node); a far move is followed
    /// only once the register still holds it a block later, as angr's
    /// find_vip follows the VIP register sequentially from its previous
    /// candidate (VMProtect's 32-bit vJMP garbles the register before the
    /// real pop), and before the VM is entered a register
    /// just advanced past a concrete access (a table walk) in the protector's
    /// code keys the loop so it unrolls; a loop of the function's own native
    /// code (a `rep movsb` copying a literal) stays a loop.  Otherwise the
    /// context is inherited.
    fn successor_ctx(&mut self, state: &mut State, traces: &[InsnTrace], ctx: VmCtx, insns: &[Rc<LiftedInsn>]) -> VmCtx {
        let ps = self.abi.ptr_size;
        if let Some(cell) = self.data_vpc {
            // An in-function interpreter: the cell is the VPC wherever it
            // holds a bytecode pointer; elsewhere the context carries over.
            let v = state.read_mem(&mut self.arena, self.target, cell.0, cell.1, ps);
            let mut next = ctx;
            if let Some(key) = self.data_vpc_key(state, v) {
                next.vpc_cell = Some(cell);
                next.vpc_reg = None;
                next.ctx = Some(key);
                next.in_vm = true;
                next.pending_vpc = None;
            }
            return next;
        }
        let mut fetch_regs: BTreeMap<u64, usize> = BTreeMap::new();
        let mut touched: Vec<u64> = Vec::new();
        let mut fetches: Vec<u64> = Vec::new();
        let mut cell_loads: Vec<(u64, u64)> = Vec::new();
        for t in traces {
            for a in &t.accesses {
                let Some(addr) = a.addr else { continue };
                touched.push(addr);
                if a.is_store {
                    continue;
                }
                if a.size == ps && !t.forked {
                    if let Some(c) = t.op_out.get(a.op_index).copied().flatten().and_then(|v| self.arena.is_const(v)) {
                        cell_loads.push((addr, c as u64));
                    }
                }
                if self.target.bytecode_near(addr) {
                    fetches.push(addr);
                }
                let Some((reg, disp)) = a.via else { continue };
                if disp.abs() > VPC_MAX_DISP || !self.abi.gprs.contains(&reg) {
                    continue;
                }
                if !self.target.bytecode_near(addr) {
                    continue;
                }
                *fetch_regs.entry(reg).or_insert(0) += 1;
            }
        }
        let mut cell = ctx.vpc_cell.and_then(|(b, a)| if b.is_none() { Some(a as u64) } else { None }).filter(|a| self.cell_values(state, *a, false).is_some());
        if cell.is_none() {
            for &(a, v) in &cell_loads {
                if self.written(state, a, ps) && fetches.iter().any(|f| f.abs_diff(v) <= VPC_MAX_DISP as u64) && self.cell_values(state, a, true).is_some() {
                    cell = Some(a);
                    break;
                }
            }
        }
        let mut next = ctx;
        if let Some(a) = cell {
            let vals = self.cell_values(state, a, false).unwrap_or_default();
            next.vpc_cell = Some((None, a as i128));
            next.vpc_reg = None;
            next.ctx = Some(vpc_key(&vals));
            next.in_vm = true;
            return next;
        }
        next.vpc_cell = None;
        let mut cands: Vec<(u64, Vec<u64>, usize)> = Vec::new();
        for (r, n) in fetch_regs {
            if let Some(vals) = self.vpc_values(state, r, true) {
                cands.push((r, vals, n));
            }
        }
        let prev = ctx.ctx.filter(|p| ctx.in_vm && ctx.vpc_reg.is_some() && self.vpc_like(state, *p, true));
        let near = |vals: &[u64]| prev.and_then(|p| vals.iter().map(|v| v.abs_diff(p)).min()).filter(|d| *d <= VPC_NEAR);
        let chosen = if let Some(c) = cands.iter().find(|c| Some(c.0) == ctx.vpc_reg) {
            Some(c)
        } else if let Some(c) = cands.iter().filter(|c| near(&c.1).is_some()).min_by_key(|c| near(&c.1)) {
            Some(c)
        } else {
            cands.iter().max_by_key(|c| (c.2, std::cmp::Reverse(c.0)))
        };
        if let Some((r, vals, _)) = chosen {
            next.vpc_reg = Some(*r);
            next.ctx = Some(vpc_key(vals));
            next.in_vm = true;
            if let Some(tr) = &self.config.trace {
                use std::io::Write;
                let _ = writeln!(tr.borrow_mut(), "        vpc reg{r:x} = {vals:x?} (candidates {:x?}, prev {prev:x?})", cands.iter().map(|c| (c.0, &c.1)).collect::<Vec<_>>());
            }
            return next;
        }
        if let Some(r) = ctx.vpc_reg {
            let vals = self.vpc_values(state, r, false);
            if let Some(vals) = vals.filter(|vals| !vals.iter().any(|v| self.node_addrs.contains(v) || insns.iter().any(|i| i.addr == *v))) {
                let key = vpc_key(&vals);
                let near = |p: u64| vals.iter().any(|v| v.abs_diff(p) <= VPC_NEAR);
                let from = ctx.ctx.filter(|p| self.target.is_mapped(*p));
                if from.is_none_or(near) || ctx.pending_vpc == Some(key) {
                    next.ctx = Some(key);
                    next.pending_vpc = None;
                } else {
                    next.pending_vpc = Some(key);
                }
                return next;
            }
        }
        if !ctx.in_vm && insns.last().is_some_and(|i| self.target.is_foreign_code(i.addr)) {
            for r in self.abi.gprs.clone() {
                if r == self.abi.sp {
                    continue;
                }
                let v = state.read_reg(&mut self.arena, r, self.abi.ptr_size);
                let Some(c) = self.arena.is_const(v).map(|c| c as u64) else { continue };
                if touched.iter().any(|a| c > *a && c - *a <= VPC_MAX_DISP as u64) && self.vpc_like(state, c, false) {
                    next.vpc_reg = Some(r);
                    next.ctx = Some(c);
                    return next;
                }
            }
        }
        next
    }

    /// The constants one side of a difference at `loc` (of the node `key`)
    /// may be: a constant, or a value its boolean atoms decide.  `Some(empty)`
    /// for the location's own symbol, which adds nothing, and `None` for
    /// anything else.
    fn side_values(&mut self, v: ExprId, key: NodeKey, loc: &SymLoc) -> Option<Vec<u128>> {
        if let Some(c) = self.arena.is_const(v) {
            return Some(vec![c]);
        }
        if let Some((k, l)) = self.arena.as_var(v).and_then(|x| self.own_syms.get(&x)) {
            if *k == key && covers(l, loc) {
                return Some(Vec::new());
            }
        }
        self.arena.possible_values(v).filter(|vs| vs.len() <= MAX_SYM_VALUES)
    }

    /// Whether a node keyed by context `ctx` stands for virtual PC value `v`:
    /// the key names exactly the values `v` can take.  A key carried forward
    /// past a block that left no readable VPC does not.
    fn keys_vpc(&mut self, v: ExprId, ctx: Option<u64>) -> bool {
        let Some(k) = ctx else { return false };
        let vals: Vec<u64> = match self.arena.is_const(v) {
            Some(c) => vec![c as u64],
            None => match self.arena.possible_values(v).filter(|vs| vs.len() <= MAX_VPC_FORK) {
                Some(vs) => vs.into_iter().map(|x| x as u64).collect(),
                None => return false,
            },
        };
        vpc_key(&vals) == k
    }

    /// The VPC's current value, from its memory cell or its register.
    fn vpc_expr(&mut self, state: &mut State, ctx: &VmCtx) -> Option<ExprId> {
        if let Some((b, a)) = ctx.vpc_cell {
            return Some(state.read_mem(&mut self.arena, self.target, b, a, self.abi.ptr_size));
        }
        ctx.vpc_reg.map(|r| state.read_reg(&mut self.arena, r, self.abi.ptr_size))
    }

    /// The values the memory cell at `a` may hold, when every one is a
    /// plausible virtual PC.
    fn cell_values(&mut self, state: &mut State, a: u64, strict: bool) -> Option<Vec<u64>> {
        let v = state.read_mem(&mut self.arena, self.target, None, a as i128, self.abi.ptr_size);
        self.bytecode_values(state, v, strict)
    }

    /// The values register `r` may hold (at most four), when every one is a
    /// plausible virtual PC.
    fn vpc_values(&mut self, state: &mut State, r: u64, strict: bool) -> Option<Vec<u64>> {
        let v = state.read_reg(&mut self.arena, r, self.abi.ptr_size);
        self.bytecode_values(state, v, strict)
    }

    fn bytecode_values(&mut self, state: &mut State, v: ExprId, strict: bool) -> Option<Vec<u64>> {
        let vals: Vec<u64> = match self.arena.is_const(v) {
            Some(c) => vec![c as u64],
            None => self.arena.possible_values(v).filter(|vs| vs.len() <= 4)?.into_iter().map(|x| x as u64).collect(),
        };
        vals.iter().all(|c| self.vpc_like(state, *c, strict)).then_some(vals)
    }

    /// Image memory outside the writable sections that no path has written
    /// (angr's test excludes only the data sections); when `strict`, also
    /// high-entropy (bytecode rather than a table).
    fn vpc_like(&mut self, state: &mut State, c: u64, strict: bool) -> bool {
        self.target.is_mapped(c) && !self.target.is_writable(c) && (!strict || self.target.bytecode_near(c)) && !self.written(state, c, 1)
    }

    /// Some byte of `[a, a + len)` holds a value other than the image's
    /// (reads cache image bytes in the state, so presence alone says nothing).
    fn written(&mut self, state: &mut State, a: u64, len: u8) -> bool {
        (0..len as u64).any(|i| {
            if !state.has_mem(None, (a + i) as i128, 1) {
                return false;
            }
            let v = state.read_mem(&mut self.arena, self.target, None, (a + i) as i128, 1);
            let mut b = [0u8; 1];
            !(self.target.read_bytes(a + i, &mut b) && self.arena.is_const(v) == Some(b[0] as u128))
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_exit(
        &mut self,
        cfg: &mut FlatCfg,
        worklist: &mut VecDeque<NodeId>,
        id: NodeId,
        exit: Exit,
        mut state: State,
        mut ctx: VmCtx,
        insns: &[Rc<LiftedInsn>],
    ) -> NodeExit {
        match exit {
            Exit::Fallthrough(next) => {
                let key = NodeKey::at(next, ctx.ctx);
                NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)))
            }
            Exit::Halt(m) => {
                self.stats.halts += 1;
                NodeExit::Halt(m)
            }
            Exit::NoReturn => NodeExit::NoReturn,
            Exit::Jump(t) => self.resolve_jump(cfg, worklist, id, t, state, ctx, false),
            Exit::Return(t) => {
                if Some(t) == self.retaddr || self.is_caller_slot(t) {
                    return NodeExit::Return;
                }
                // Returning to the caller's address plus a stack adjustment is
                // still a return (the entry stub's extra pops are folded).
                self.resolve_jump(cfg, worklist, id, t, state, ctx, true)
            }
            Exit::CondJump { arms, fallthrough } => {
                let one = self.arena.cst(1, 1);
                let zero = self.arena.cst(1, 0);
                let fkey = NodeKey::at(fallthrough, ctx.ctx);
                let arm_ctxs = std::mem::take(&mut self.arm_ctxs);
                let mut earlier: Vec<ExprId> = Vec::new();
                // Per CBRANCH, in program order: the arm it joins (None: it
                // only skips to the fall-through and continues there).
                let mut slots: Vec<(ExprId, Option<usize>)> = Vec::with_capacity(arms.len());
                let mut arm_targets: Vec<(NodeKey, VmCtx, State)> = Vec::new();
                for (i, (cond, target, mut taken_state)) in arms.into_iter().enumerate() {
                    let Some(tc) = self.arena.is_const(target) else {
                        self.stats.unresolved += 1;
                        return NodeExit::Unresolved("conditional branch with symbolic target".into());
                    };
                    let actx = arm_ctxs.get(i).copied().unwrap_or(ctx);
                    let tkey = NodeKey::at(tc as u64, actx.ctx);
                    if tkey == fkey {
                        // Both sides continue at the same place (a CMOV-style
                        // skip): one state, selecting on the guard.
                        merge_ite(&mut state, &mut taken_state, &mut self.arena, self.target, cond);
                        slots.push((cond, None));
                        continue;
                    }
                    if let Some(k) = arm_targets.iter().position(|(a, _, _)| *a == tkey) {
                        merge_ite(&mut arm_targets[k].2, &mut taken_state, &mut self.arena, self.target, cond);
                        slots.push((cond, Some(k)));
                        continue;
                    }
                    // This arm: its guard holds and every earlier guard failed.
                    pin(&mut self.arena, &mut taken_state, cond, one);
                    for e in &earlier {
                        pin(&mut self.arena, &mut taken_state, *e, zero);
                    }
                    pin(&mut self.arena, &mut state, cond, zero);
                    earlier.push(cond);
                    slots.push((cond, Some(arm_targets.len())));
                    arm_targets.push((tkey, actx, *taken_state));
                }
                let mut taken_ids = Vec::with_capacity(arm_targets.len());
                for (tkey, actx, st) in arm_targets {
                    taken_ids.push(self.reach(cfg, worklist, tkey, st, actx, Some(id)));
                }
                let fall = self.reach(cfg, worklist, fkey, state, ctx, Some(id));
                if slots.is_empty() {
                    return NodeExit::Branch(fall);
                }
                let resolved = slots.into_iter().map(|(c, k)| (c, k.map(|k| taken_ids[k]).unwrap_or(fall))).collect();
                NodeExit::CondBranch { arms: resolved, fall }
            }
            Exit::Call { target, fallthrough } => {
                let tc = self.arena.is_const(target).map(|c| c as u64);
                if let Some(c) = tc {
                    if self.target.is_foreign_code(c) && !self.target_is_function(c) {
                        // Dispatch through the interpreter: the pushed return
                        // address salts the context so each entry stub is distinct.
                        if !ctx.in_vm || (ctx.vpc_reg.is_none() && ctx.vpc_cell.is_none()) {
                            let site = cfg.nodes[id as usize].key.addr;
                            ctx = self.entering_vm(&mut state, ctx, site, c);
                            let salt = ctx.ctx.unwrap_or(0).rotate_left(17) ^ fallthrough;
                            ctx.ctx = Some(salt);
                        }
                        let key = NodeKey::at(c, ctx.ctx);
                        return NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)));
                    }
                }
                // A target read from a data slot of the image: an unbound import.
                let slot = tc.filter(|c| self.classify_target(*c).is_err()).and_then(|c| self.image_ptrs.get(&c).copied()).filter(|s| !self.target.is_executable(*s));
                let tc = if slot.is_some() { None } else { tc };
                let callee = Callee { target: tc, slot, ret: Some(fallthrough) };
                self.apply_call_summary(&mut state, insns.last().map(|i| i.addr).unwrap_or(0), callee);
                let ctx = after_call(ctx, fallthrough);
                let key = NodeKey::at(fallthrough, ctx.ctx);
                let ret = self.reach(cfg, worklist, key, state, ctx, Some(id));
                NodeExit::Call { target: tc, ret, slot }
            }
        }
    }

    /// `t` is the contents of a stack slot above the entry stack pointer the
    /// function never wrote: entered past a native prologue (Themida
    /// virtualizes from inside the function), the return address sits there.
    /// A call that may have written the slot (an escaped local of the
    /// function sits below it, [`Explorer::clobber_call`]) leaves it the
    /// caller's slot all the same.
    fn is_caller_slot(&self, t: ExprId) -> bool {
        let Some(v) = self.arena.as_var(t) else { return false };
        let info = self.arena.var_info(v);
        let ps = self.abi.ptr_size as i128;
        info.size == self.abi.ptr_size
            && match info.kind {
                VarKind::InitMem { base, offset } | VarKind::CallOutputAt { base, offset, .. } => base.is_some() && base == self.sp0 && offset > 0 && offset % ps == 0,
                _ => false,
            }
    }

    fn target_is_function(&self, addr: u64) -> bool {
        addr != self.entry_addr && self.target.is_function_entry(addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_jump(
        &mut self,
        cfg: &mut FlatCfg,
        worklist: &mut VecDeque<NodeId>,
        id: NodeId,
        t: ExprId,
        state: State,
        ctx: VmCtx,
        is_ret: bool,
    ) -> NodeExit {
        let t = self.arena.decide_atoms(t);
        if let Some(c) = self.arena.is_const(t) {
            return self.jump_to(cfg, worklist, id, c as u64, state, ctx);
        }
        if let Some(tr) = &self.config.trace {
            use std::io::Write;
            let atoms = self.arena.bool_atoms(t);
            let _ = writeln!(
                tr.borrow_mut(),
                "        resolve {} target={} atoms={}",
                if is_ret { "ret" } else { "jump" },
                self.arena.fmt(t).chars().take(6000).collect::<String>(),
                atoms.len()
            );
            for a in atoms.iter().take(12) {
                let _ = writeln!(tr.borrow_mut(), "          atom {a} {}", self.arena.fmt(*a).chars().take(1500).collect::<String>());
            }
            let vars = self.arena.free_vars(t);
            let _ = writeln!(tr.borrow_mut(), "          free vars: {:?}", vars.iter().map(|v| format!("{v}:{:?}", self.arena.var_info(*v).kind)).collect::<Vec<_>>());
        }
        let vars_before = self.arena.num_vars() as u32;
        let Some(cases) = self.enumerate_symbol_target(t).or_else(|| self.arena.case_split(t)) else {
            if let Some(e) = self.call_through(cfg, worklist, id, t, &state, ctx) {
                return e;
            }
            self.stats.unresolved += 1;
            return NodeExit::Unresolved(format!(
                "{} target not enumerable: {}",
                if is_ret { "return" } else { "indirect jump" },
                self.arena.fmt(t)
            ));
        };
        let mut by_target: BTreeMap<u64, Vec<Vec<(ExprId, bool)>>> = BTreeMap::new();
        if let Some(tr) = &self.config.trace {
            use std::io::Write;
            for (assign, e) in cases.iter().take(64) {
                let _ = writeln!(tr.borrow_mut(), "          case {:?} -> {}", assign.iter().map(|(a, v)| (*a, *v)).collect::<Vec<_>>(), self.arena.fmt(*e).chars().take(200).collect::<String>());
            }
        }
        for (assign, e) in &cases {
            let Some(c) = self.arena.is_const(*e) else {
                // A case that only fails through an unknown born while folding
                // it (a shift or divide by zero under that assignment) is an
                // infeasible combination of atoms.
                if self.arena.free_vars(*e).iter().all(|v| *v >= vars_before) {
                    continue;
                }
                self.stats.unresolved += 1;
                if let Some(tr) = &self.config.trace {
                    use std::io::Write;
                    let _ = writeln!(tr.borrow_mut(), "          residual {}", self.arena.fmt(*e).chars().take(3000).collect::<String>());
                }
                return NodeExit::Unresolved(format!("target stays symbolic under case split: {}", self.arena.fmt(*e)));
            };
            by_target.entry(c as u64).or_default().push(assign.clone());
        }
        if by_target.len() > self.config.max_select_targets {
            self.stats.unresolved += 1;
            return NodeExit::Unresolved(format!("{} targets", by_target.len()));
        }
        if by_target.len() == 1 {
            let c = *by_target.keys().next().unwrap();
            return self.jump_to(cfg, worklist, id, c, state, ctx);
        }
        let mut atoms: Vec<ExprId> = Vec::new();
        for (assign, _) in &cases {
            for (a, _) in assign {
                if !atoms.contains(a) {
                    atoms.push(*a);
                }
            }
        }
        // A symbol enumeration encodes (symbol, constant) pairs instead of atoms.
        let symbol_pins = cases.iter().all(|(a, _)| {
            a.len() == 2 && self.arena.as_var(a[0].0).is_some_and(|v| self.sym_values.borrow().contains_key(&v)) && self.arena.is_const(a[1].0).is_some()
        });
        let mut targets = Vec::new();
        for (c, assigns) in by_target {
            let mut st = state.clone();
            if symbol_pins {
                let (sym, kc) = (assigns[0][0].0, assigns[0][1].0);
                pin(&mut self.arena, &mut st, sym, kc);
            } else if assigns.len() == 1 {
                for (a, v) in &assigns[0] {
                    let k = self.arena.cst(1, *v as u128);
                    pin(&mut self.arena, &mut st, *a, k);
                }
            } else {
                // Atoms fixed across every assignment reaching this target can be pinned.
                for a in atoms.iter() {
                    let value_in = |as_: &Vec<(ExprId, bool)>| as_.iter().find(|(x, _)| x == a).map(|(_, v)| *v);
                    let Some(first) = value_in(&assigns[0]) else { continue };
                    if assigns.iter().all(|as_| value_in(as_) == Some(first)) {
                        let k = self.arena.cst(1, first as u128);
                        pin(&mut self.arena, &mut st, *a, k);
                    }
                }
            }
            let mut sctx = ctx;
            // The pinned state may make the VPC concrete.
            if let Some(v) = self.vpc_expr(&mut st, &ctx) {
                if let Some(vc) = self.arena.is_const(v) {
                    sctx.ctx = Some(vc as u64);
                }
            }
            // Each destination gets the same treatment as a direct jump; a
            // call or tail call becomes a trampoline node the select can target.
            let succ = match self.jump_to(cfg, worklist, id, c, st, sctx) {
                NodeExit::Branch(n) => n,
                NodeExit::Halt(_) => continue,
                other => self.trampoline(cfg, id, c, other),
            };
            targets.push((assigns.clone(), succ));
        }
        if targets.is_empty() {
            self.stats.unresolved += 1;
            return NodeExit::Unresolved("no reachable select target".into());
        }
        if targets.len() == 1 {
            return NodeExit::Branch(targets[0].1);
        }
        let mut atoms = atoms;
        let mut targets = targets;
        if !symbol_pins {
            fuse_signed_compares(&mut self.arena, &mut atoms, &mut targets);
            let mut st = state;
            self.pending_locs = self.locate_vars(&mut st, &atoms);
        }
        NodeExit::Select { atoms, targets, by_atoms: !symbol_pins }
    }

    /// A jump to a target no enumeration resolves, below a pushed code
    /// address, where the target is program data (a call's result or output,
    /// an input of the function, a merged location) rather than a value the
    /// emulation lost: a call through a pointer (the interpreter leaving to
    /// `GetProcAddress`'s result), continuing at the pushed return address.
    fn call_through(&mut self, cfg: &mut FlatCfg, worklist: &mut VecDeque<NodeId>, id: NodeId, t: ExprId, state: &State, ctx: VmCtx) -> Option<NodeExit> {
        let vars = self.arena.free_vars(t);
        if vars.is_empty() || vars.iter().any(|v| matches!(self.arena.var_info(*v).kind, VarKind::Unknown { .. })) {
            return None;
        }
        let mut state = state.clone();
        let ret = self.pushed_return_address(&mut state)?;
        let site = cfg.nodes[id as usize].key.addr;
        self.apply_call_summary(&mut state, site, Callee { target: None, slot: None, ret: Some(ret) });
        let ctx = after_call(ctx, ret);
        let key = NodeKey::at(ret, ctx.ctx);
        let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
        Some(NodeExit::Call { target: None, ret: ret_node, slot: None })
    }

    /// A target over one symbolized location (`sym + k`, or a table entry
    /// indexed by it) that is constant for every constant the location was
    /// seen to hold: one case per constant, pinning the symbol to it.
    fn enumerate_symbol_target(&mut self, t: ExprId) -> Option<Vec<(Vec<(ExprId, bool)>, ExprId)>> {
        let (var, values) = {
            let known = self.sym_values.borrow();
            let mut hit = None;
            for v in self.arena.free_vars(t) {
                if let Some(vals) = known.get(&v) {
                    if hit.is_some() {
                        return None;
                    }
                    hit = Some((v, vals.clone()));
                }
            }
            hit?
        };
        if values.is_empty() {
            return None;
        }
        let var_expr = self.arena.var_expr(var)?;
        // A guard other than the symbol's comparison with a recorded constant
        // (a loop's exit test) can take values the recorded constants never
        // showed: that target is split on its atoms instead.
        for a in self.arena.bool_atoms(t) {
            let super::kuna_pushanexpr::ExprKind::Bin(OpCode::CPUI_INT_EQUAL, x, k) = self.arena.kind(a).clone() else { return None };
            let (x, k) = if self.arena.is_const(x).is_some() { (k, x) } else { (x, k) };
            if x != var_expr || !self.arena.is_const(k).is_some_and(|c| values.contains(&c)) {
                return None;
            }
        }
        let mut out = Vec::new();
        for v in values {
            let kc = self.arena.cst(self.arena.size(var_expr), v);
            let mut map = BTreeMap::new();
            map.insert(var_expr, kc);
            let target = self.arena.substitute(t, &map);
            self.arena.is_const(target)?;
            // Encode the pin as a pseudo-atom pair: the symbol and its constant.
            out.push((vec![(var_expr, true), (kc, true)], target));
        }
        Some(out)
    }

    fn classify_target(&self, c: u64) -> Result<(), String> {
        if !self.target.is_mapped(c) || !self.target.is_executable(c) {
            return Err(format!("target {c:#x} is not executable"));
        }
        Ok(())
    }

    fn jump_to(&mut self, cfg: &mut FlatCfg, worklist: &mut VecDeque<NodeId>, id: NodeId, c: u64, mut state: State, ctx: VmCtx) -> NodeExit {
        // A target read from a data slot of the image that is not code
        // itself is an unbound import or GOT entry: a call through the slot.
        if self.classify_target(c).is_err() {
            if let Some(&slot) = self.image_ptrs.get(&c) {
                if !self.target.is_executable(slot) {
                    if let Some(ret) = self.pushed_return_address(&mut state) {
                        let site = cfg.nodes[id as usize].key.addr;
                        self.apply_call_summary(&mut state, site, Callee { target: None, slot: Some(slot), ret: Some(ret) });
                        let ctx = after_call(ctx, ret);
                        let key = NodeKey::at(ret, ctx.ctx);
                        let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
                        return NodeExit::Call { target: None, ret: ret_node, slot: Some(slot) };
                    }
                    if let Some(why) = self.misplaced_tail_call(&mut state) {
                        self.stats.misplaced_tail_calls += 1;
                        return NodeExit::Unresolved(format!("jump through import slot {slot:#x} {why}"));
                    }
                    return NodeExit::TailCallSlot(slot);
                }
            }
        }
        if let Err(m) = self.classify_target(c) {
            self.stats.halts += 1;
            return NodeExit::Halt(m);
        }
        // Compiled code can pass the bytecode entropy test: a known function
        // entry is native whatever its bytes look like, and so is a
        // high-entropy target the interpreter transferred to through a
        // computed jump or return with a return into itself pushed.  A
        // direct jump is the interpreter's own handler chain (VMProtect
        // ultra lays handlers over the protected function's bytes).
        let interp = self.data_vpc.is_some() && ctx.in_vm;
        let pushed = if interp { None } else { self.pushed_return_address(&mut state) };
        let native = !interp
            && !self.target.is_foreign_code(c)
            && (self.target_is_function(c)
                || !self.target.bytecode_like(c)
                || (!self.direct_exit && !self.handler_fragment(c) && pushed.is_some_and(|r| r != c && self.target.is_foreign_code(r))));
        if native {
            if let Some(tr) = &self.config.trace {
                use std::io::Write;
                let ps = self.abi.ptr_size;
                let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
                let top = self.arena.affine_probe(sp).map(|(b, o)| state.read_mem(&mut self.arena, self.target, b, o, ps));
                let _ = writeln!(tr.borrow_mut(), "        native target {c:#x}: stack top {}", top.map(|t| self.arena.fmt(t).chars().take(300).collect::<String>()).unwrap_or_else(|| "?".into()));
            }
            // `push return; jmp/ret target`: a call whose return address the
            // interpreter already pushed.
            if let Some(ret) = pushed {
                if ret != c {
                    let site = cfg.nodes[id as usize].key.addr;
                    self.apply_call_summary(&mut state, site, Callee { target: Some(c), slot: None, ret: Some(ret) });
                    let ctx = after_call(ctx, ret);
                    let key = NodeKey::at(ret, ctx.ctx);
                    let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
                    return NodeExit::Call { target: Some(c), ret: ret_node, slot: None };
                }
            }
            if self.target_is_function(c) {
                if let Some(why) = self.misplaced_tail_call(&mut state) {
                    self.stats.misplaced_tail_calls += 1;
                    return NodeExit::Unresolved(format!("jump to function {c:#x} {why}"));
                }
                return NodeExit::TailCall(c);
            }
        }
        let site = cfg.nodes[id as usize].key.addr;
        let ctx = if pushed.is_none() && self.leaves_vm(&mut state, &ctx, c) {
            self.forget_below_sp(&mut state);
            VmCtx { exited: true, ..VmCtx::default() }
        } else {
            self.entering_vm(&mut state, ctx, site, c)
        };
        let key = NodeKey::at(c, ctx.ctx);
        NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)))
    }

    /// Why a jump to another function is no tail call: a tail call leaves
    /// the caller's return address on top of the stack, so the stack pointer
    /// is back at its entry value.  Anywhere else something the emulation
    /// could not place was pushed (the return address of a call whose push
    /// went to an unknown address), and ending the path there would drop the
    /// rest of the function without a word.
    fn misplaced_tail_call(&mut self, state: &mut State) -> Option<String> {
        match self.sp_depth(state) {
            Some(0) => None,
            Some(d) => Some(format!("with the stack pointer at its entry value {d:+}")),
            None => Some("with the stack pointer not relative to its entry value".into()),
        }
    }

    /// The interpreter hands control back to the function's own code at `c`
    /// (angr's `_vm_mode_should_end`): a computed jump or return out of the
    /// interpreter's context, to code outside the protector's sections that
    /// is not a fragment of its handlers, once the VPC register no longer
    /// holds a virtual PC (the exit restored the native registers), or, for
    /// a VPC kept in a memory cell, which keeps its last value, once the stack
    /// pointer is back where native code entered the interpreter (angr's
    /// `_on_vm_entry_frame`).  Every exit to one address then reaches one
    /// node, with the native context, as angr keys post-exit blocks with no
    /// VPC: the function's epilogue is not copied once per virtual exit.
    fn leaves_vm(&mut self, state: &mut State, ctx: &VmCtx, c: u64) -> bool {
        if !ctx.in_vm || self.direct_exit || self.target.is_foreign_code(c) {
            return false;
        }
        let left = if ctx.vpc_cell.is_some() {
            let depth = self.sp_depth(state);
            depth.is_some() && self.vm_entry_depth == Some(depth)
        } else {
            ctx.vpc_reg.is_some_and(|r| self.vpc_values(state, r, false).is_none())
        };
        left && !self.handler_fragment(c)
    }

    /// Drop what the path stored on the entry stack below the stack pointer
    /// (past the x86-64 red zone): the interpreter's context the exit left
    /// there is dead to the program, and paths that left by different exits
    /// would otherwise differ there at the node they join.
    fn forget_below_sp(&mut self, state: &mut State) {
        let Some(depth) = self.sp_depth(state) else { return };
        let red = if self.abi.ptr_size == 8 { 128 } else { 0 };
        state.forget_copies(self.sp0, i128::MIN, depth - red);
        if let Some(region) = state.mem.get_mut(&self.sp0) {
            region.remove_where(|o| o < depth - red);
        }
    }

    /// The context for a transfer from native code into the protector's
    /// code at `c`.  The first one records the stack depth the interpreter
    /// is entered at.  After an exit, a jump back in starts a new region
    /// keyed by its site (angr's re-entry region tag), so it never shares the
    /// first entry's nodes.
    fn entering_vm(&mut self, state: &mut State, mut ctx: VmCtx, site: u64, c: u64) -> VmCtx {
        if ctx.in_vm || !self.target.is_foreign_code(c) || self.target.is_foreign_code(site) {
            return ctx;
        }
        if self.vm_entry_depth.is_none() {
            self.vm_entry_depth = Some(self.sp_depth(state));
        }
        if ctx.exited {
            ctx.exited = false;
            ctx.ctx = Some(REGION_TAG ^ site);
        }
        ctx
    }

    /// The code at `c` is a fragment of the interpreter's handlers: its
    /// straight-line run jumps directly into the protector's section
    /// (VMProtect ultra lays handlers over the protected function's own
    /// bytes, while compiled code the interpreter calls does not jump back
    /// into the protector before its first branch or call).
    fn handler_fragment(&mut self, c: u64) -> bool {
        let mut a = c;
        for _ in 0..FRAGMENT_INSNS {
            let Ok(insn) = self.target.lift(a) else { return false };
            let mut next = a + insn.len as u64;
            for op in &insn.ops {
                match op.opc {
                    OpCode::CPUI_BRANCH if classify_space_of(&op.ins[0]) == Some(super::kuna_pushanemu::Space::Ram) => {
                        if self.target.is_foreign_code(op.ins[0].offset) {
                            return true;
                        }
                        next = op.ins[0].offset;
                    }
                    OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => {
                        if op.opc != OpCode::CPUI_CBRANCH || classify_space_of(&op.ins[0]) != Some(super::kuna_pushanemu::Space::Const) {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
            a = next;
        }
        false
    }

    /// A node with no instructions that only performs `exit` (a call or tail
    /// call reached through a multi-way select).
    fn trampoline(&mut self, cfg: &mut FlatCfg, from: NodeId, addr: u64, exit: NodeExit) -> NodeId {
        let id = cfg.nodes.len() as NodeId;
        let tag = 0x7f00_0000_0000_0000u64 ^ (from as u64).rotate_left(32) ^ addr;
        cfg.nodes.push(Node {
            call_pop: None,
            call_clobbers: None,
            exit_locs: None,
            vpc_fork: None,
            key: NodeKey::at(addr, Some(tag)),
            entry: State::new(self.abi.big_endian, self.abi.flag_regs),
            entry_ctx: VmCtx::default(),
            insns: Vec::new(),
            traces: Vec::new(),
            exit,
            preds: vec![from],
        });
        id
    }

    /// The concrete code address on top of the stack, if any.
    fn pushed_return_address(&mut self, state: &mut State) -> Option<u64> {
        let ps = self.abi.ptr_size;
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let (base, off) = self.arena.affine_probe(sp)?;
        let top = state.read_mem(&mut self.arena, self.target, base, off, ps);
        let c = self.arena.is_const(top)? as u64;
        (self.target.is_mapped(c) && self.target.is_executable(c)).then_some(c)
    }

    /// What an external call may have written (`State::clobber_call`): all
    /// mutable image memory and every symbolic region, and the frame an
    /// escaped frame address reaches.  An address of a local may reach any
    /// local at or above it, up to the return address; one of the incoming
    /// stack arguments reaches every argument at or above it.  The return
    /// address and the frame below every escaped offset are kept: no address
    /// the callee received reaches them.
    fn clobber_call(&mut self, state: &mut State) {
        let Some(frame) = self.sp0 else {
            state.clobber_mutable(self.target, self.call_epochs);
            return;
        };
        let ps = self.abi.ptr_size as i128;
        let mut ranges = Vec::new();
        if let Some(&lo) = self.escapes.iter().find(|o| **o < 0) {
            ranges.push((lo, 0));
        }
        if let Some(&lo) = self.escapes.iter().find(|o| **o >= ps) {
            ranges.push((lo, i128::MAX));
        }
        state.clobber_call(self.target, self.call_epochs, frame, ranges);
    }

    /// Model an external call: a fresh return value, clobbered registers, the
    /// return address popped, and everything the callee could reach through a
    /// pointer its caller gave it left as the call's output
    /// ([`Explorer::clobber_call`]): a `scanf("%d", &v)` writes `v`, and
    /// `png_inflate` writes through the pointers in its stack-passed
    /// arguments.  The pointers are the argument registers and the outgoing
    /// argument area's words; every frame offset among them escapes.  An
    /// argument register still holding an earlier call's clobber was never
    /// set and points at nothing.
    fn apply_call_summary(&mut self, state: &mut State, site: u64, callee: Callee) {
        let ps = self.abi.ptr_size;
        let scan = callee.target.filter(|c| Some(*c) != callee.ret).and_then(|c| self.scan_callee(c));
        let pop = self.callee_pop(state, &callee, scan.as_ref());
        self.last_pop = Some(pop);
        let probed = callee.target.filter(|c| Some(*c) != callee.ret && self.target.is_executable(*c)).and_then(|c| self.probe_stack_effect(state, c));
        if self.config.trace.is_some() {
            self.log.push(format!("call at {site:#x}: pops {pop}"));
        }
        let copy = callee.target.or(callee.slot).filter(|a| self.target.copy_routine(*a)).and_then(|_| self.copy_arguments(state));
        let args: Vec<(u64, u8)> = self.abi.args.iter().copied().filter(|(o, _)| *o != self.abi.sp).collect();
        let mut values: Vec<ExprId> = args.iter().map(|&(off, size)| state.read_reg(&mut self.arena, off, size)).collect();
        values.retain(|v| !matches!(self.arena.var_kind(*v), Some(VarKind::CallClobber { .. })));
        if let Some((dst, src, _, _)) = copy {
            values.retain(|v| *v != dst && *v != src);
        }
        // Stack-passed arguments sit above the return address and the
        // callee's home space: the outgoing argument area, and every word a
        // callee-cleanup callee pops.
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        if let Some((sb, so)) = self.arena.affine_probe(sp) {
            let home = self.abi.call_home_words as i128;
            let last = (home + self.abi.call_arg_words as i128).max(pop as i128 / ps as i128);
            for k in 1 + home..=last {
                values.push(state.read_mem(&mut self.arena, self.target, sb, so + k * ps as i128, ps));
            }
        }
        // Every frame offset handed over escapes before the call is applied,
        // so the call may already write all the frame an escaped address reaches.
        let mut pointers: Vec<(Option<VarId>, i128)> = Vec::new();
        for v in values {
            if self.config.trace.is_some() {
                self.log.push(format!("call at {site:#x}: arg {} affine={:?}", self.arena.fmt(v), self.arena.affine(v)));
            }
            if let Some((b, o)) = self.arena.affine(v).filter(|_| !callee.target.is_some_and(|t| self.target.noreturn_function(t))) {
                if state.copies.iter().any(|c| c.base == b && c.off <= o && o < c.off + c.len as i128) {
                    self.copy_escaped = true;
                }
            }
            if let Some((b, o)) = self.arena.affine_probe(v) {
                if b.is_some() && b == self.sp0 && self.escapes.insert(o) {
                    self.new_escapes += 1;
                }
                pointers.push((b, o));
            }
        }
        let copy_dst = copy.and_then(|(dst, _, s, n)| self.arena.affine_probe(dst).map(|(b, o)| (b, o, s, n))).filter(|(b, ..)| *b == self.sp0 || b.is_none());
        if let Some((b, o, _, _)) = copy_dst {
            if b == self.sp0 && self.escapes.insert(o) {
                self.new_escapes += 1;
            }
        }
        for &(b, o) in &pointers {
            if b.is_none() && self.target.is_mapped(o as u64) && self.target.is_writable(o as u64) {
                for k in 0..(CALL_POINTEE_BYTES / ps as i128) {
                    let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallOutput { site }, ps, false));
                    state.write_mem(None, o + k * ps as i128, u, ps);
                }
            }
        }
        if let Some(c) = copy_dst.filter(|c| c.0.is_none()) {
            place_copy(state, c);
        }
        self.call_epochs += 1;
        self.clobber_call(state);
        if let Some(c) = copy_dst.filter(|c| c.0.is_some()) {
            place_copy(state, c);
        }
        let kept = |off: u64, size: u8| scan.as_ref().is_some_and(|s| s.closed && !s.writes.iter().any(|&(o, n)| o < off + size as u64 && off < o + n as u64));
        let mut clobbered: Vec<(u64, u8)> = Vec::new();
        if !kept(self.abi.return_reg, ps) {
            let ret = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallResult { site }, ps, false));
            state.write_reg(self.abi.return_reg, ret, ps);
        }
        let flags = self.abi.flag_regs;
        for &(off, size) in &self.abi.clobbers {
            if kept(off, size) {
                continue;
            }
            clobbered.push((off, size));
            let boolean = size == 1 && off >= flags.0 && off < flags.1;
            let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallClobber { site, offset: off }, size, boolean));
            state.write_reg(off, u, size);
        }
        self.last_clobbers = Some(clobbered);
        state.stack_stores.clear();
        let nsp = match probed {
            Some(p) => p,
            None => {
                let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
                let k = self.arena.cst(ps, ps as u128 + pop as u128);
                self.arena.mk_bin(OpCode::CPUI_INT_ADD, sp, k, ps)
            }
        };
        state.write_reg(self.abi.sp, nsp, ps);
    }

    /// The arguments of a `memcpy`-like call that copies a constant number of
    /// bytes (at most 16 MiB) out of file data to the stack or a concrete
    /// address: (destination value, source value, source address, count).
    fn copy_arguments(&mut self, state: &mut State) -> Option<(ExprId, ExprId, u64, u64)> {
        let ps = self.abi.ptr_size;
        let regs: Vec<(u64, u8)> = self.abi.args.iter().copied().filter(|(o, _)| *o != self.abi.sp).collect();
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let (sb, so) = self.arena.affine_probe(sp)?;
        let mut vals = Vec::with_capacity(3);
        for k in 0..3usize {
            vals.push(match regs.get(k) {
                Some(&(off, size)) => state.read_reg(&mut self.arena, off, size),
                None => state.read_mem(&mut self.arena, self.target, sb, so + ps as i128 * (1 + k - regs.len()) as i128, ps),
            });
        }
        let (Some((None, s)), Some(n)) = (self.arena.affine(vals[1]), self.arena.is_const(vals[2])) else { return None };
        let (s, n) = (u64::try_from(s).ok()?, n as u64);
        let dst = self.arena.affine_probe(vals[0])?;
        if n == 0 || n > 1 << 24 || !(dst.0.is_none() || dst.0 == self.sp0) || !self.target.file_data(s) || !self.target.file_data(s + n - 1) {
            return None;
        }
        Some((vals[0], vals[1], s, n))
    }

    /// The stack pointer a callee in the image returns with, when emulating
    /// it from the call's state decides every branch on the way to its
    /// return: a stack probe such as 32-bit `__chkstk` moves the stack by
    /// EAX, not by its `ret` (angr follows native calls and sees the same).
    fn probe_stack_effect(&mut self, state: &State, entry: u64) -> Option<ExprId> {
        let ps = self.abi.ptr_size;
        let mut st = state.clone();
        let ret = self.pushed_return_address(&mut st)?;
        let mut emu = Emulator::new(self.target);
        emu.track_sp = Some((self.abi.sp, ps));
        let mut addr = entry;
        for _ in 0..PROBE_INSNS {
            let insn = self.target.lift(addr).ok()?;
            let mut trace = InsnTrace::default();
            addr = match emu.exec_insn(&mut st, &mut self.arena, &insn, &mut trace) {
                Exit::Fallthrough(next) => next,
                Exit::Jump(t) => self.arena.is_const(t)? as u64,
                Exit::Return(t) if self.arena.is_const(t) == Some(ret as u128) => {
                    return Some(st.read_reg(&mut self.arena, self.abi.sp, ps));
                }
                _ => return None,
            };
            if !self.target.is_executable(addr) {
                return None;
            }
        }
        None
    }

    /// Bytes a callee pops beyond its return address.  Where each callee
    /// cleans up after itself (32-bit Windows `__stdcall`), this is read from
    /// the callee's own `ret imm16`, else from the caller restoring the stack
    /// at the return address (`__cdecl`), else from its declared prototype,
    /// else from the arguments pushed for it ([`Self::pushed_arg_bytes`]).
    /// angr gets the same number by executing the callee or its
    /// calling-convention-aware SimProcedure.
    fn callee_pop(&mut self, state: &mut State, callee: &Callee, scan: Option<&CalleeScan>) -> u32 {
        if !self.target.callee_cleans() || (callee.target.is_some() && callee.target == callee.ret) {
            return 0;
        }
        if let Some(p) = scan.and_then(|s| s.pop) {
            return p;
        }
        if let Some(r) = callee.ret {
            if !self.target.is_foreign_code(r) && self.caller_cleans(r) {
                return 0;
            }
        }
        if let Some(p) = callee.slot.or(callee.target).and_then(|a| self.target.declared_pop(a)) {
            return p;
        }
        if callee.target.is_some_and(|c| self.target.is_executable(c)) {
            return 0;
        }
        self.pushed_arg_bytes(state)
    }

    /// The instruction at a call's return address takes the arguments back
    /// off the stack (`add esp, n`, or a pop into a scratch register).
    fn caller_cleans(&mut self, ret: u64) -> bool {
        let Ok(insn) = self.target.lift(ret) else { return false };
        let sp = self.abi.sp;
        let mut raises = false;
        for op in &insn.ops {
            let Some(out) = &op.out else { continue };
            if classify_space_of(out) != Some(super::kuna_pushanemu::Space::Register) {
                continue;
            }
            if out.offset == sp {
                let k = op.ins.get(1).filter(|v| classify_space_of(v) == Some(super::kuna_pushanemu::Space::Const)).map(|v| v.offset as i64);
                if op.opc == OpCode::CPUI_INT_ADD && op.ins.first().is_some_and(|v| v.offset == sp) && k.is_some_and(|k| k > 0) {
                    raises = true;
                }
            } else if self.abi.callee_saved.iter().any(|&(o, n)| o < out.offset + out.size as u64 && out.offset < o + n as u64) {
                return false;
            }
        }
        raises
    }

    /// The arguments pushed for a call: the stack words above the return
    /// address the program stored to since the previous external call
    /// ([`State::stack_stores`]), up to the first it did not (the caller's
    /// frame) or the first holding a callee-saved register's entry value (the
    /// prologue's saves, before the first call). 0 when the run reaches
    /// [`MAX_PUSHED_WORDS`] without such an end.
    fn pushed_arg_bytes(&mut self, state: &mut State) -> u32 {
        let ps = self.abi.ptr_size;
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let Some((base, off)) = self.arena.affine(sp) else { return 0 };
        if base.is_none() || base != self.sp0 {
            return 0;
        }
        let saved: Vec<u64> = self.abi.callee_saved.iter().map(|(o, _)| *o).filter(|o| *o != self.abi.sp).collect();
        for k in 1..=MAX_PUSHED_WORDS {
            let o = off + k * ps as i128;
            if o >= 0 || !state.stored_since_call(o) || !state.stored_since_call(o + ps as i128 - 1) {
                return ((k - 1) * ps as i128) as u32;
            }
            let v = state.read_mem(&mut self.arena, self.target, base, o, ps);
            if matches!(self.arena.var_kind(v), Some(VarKind::InitReg { offset }) if saved.contains(&offset)) {
                return ((k - 1) * ps as i128) as u32;
            }
        }
        0
    }

    /// What a callee in the image does, read off its code: the registers
    /// any of its instructions writes and the bytes its `ret` pops.
    fn scan_callee(&mut self, c: u64) -> Option<CalleeScan> {
        if let Some(s) = self.callees.get(&c) {
            return s.clone();
        }
        let scan = self.scan_callee_code(c);
        self.callees.insert(c, scan.clone());
        scan
    }

    fn scan_callee_code(&mut self, c: u64) -> Option<CalleeScan> {
        if !self.target.is_executable(c) || self.target.is_foreign_code(c) {
            return None;
        }
        let mut scan = CalleeScan { writes: Vec::new(), pop: None, closed: true };
        let mut pops: BTreeSet<u32> = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut work = vec![c];
        while let Some(a) = work.pop() {
            if !seen.insert(a) {
                continue;
            }
            if seen.len() > MAX_CALLEE_SCAN {
                scan.closed = false;
                break;
            }
            let Ok(insn) = self.target.lift(a) else {
                scan.closed = false;
                continue;
            };
            let mut falls = true;
            for op in &insn.ops {
                if let Some(out) = &op.out {
                    if classify_space_of(out) == Some(super::kuna_pushanemu::Space::Register) {
                        let w = (out.offset, out.size.min(255) as u8);
                        if !scan.writes.contains(&w) {
                            scan.writes.push(w);
                        }
                    }
                }
                let ram_target = op.ins.first().filter(|v| classify_space_of(v) == Some(super::kuna_pushanemu::Space::Ram)).map(|v| v.offset);
                match op.opc {
                    OpCode::CPUI_BRANCH => {
                        if let Some(t) = ram_target {
                            work.push(t);
                            falls = false;
                        }
                    }
                    OpCode::CPUI_CBRANCH => {
                        if let Some(t) = ram_target {
                            work.push(t);
                        }
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => scan.closed = false,
                    OpCode::CPUI_BRANCHIND => {
                        scan.closed = false;
                        falls = false;
                    }
                    OpCode::CPUI_RETURN => {
                        falls = false;
                        let mut b = [0u8; 4];
                        if self.target.read_bytes(insn.addr, &mut b) {
                            let i = b.iter().position(|x| *x != 0xF2 && *x != 0xF3).unwrap_or(0);
                            match b[i] {
                                0xC3 => {
                                    pops.insert(0);
                                }
                                0xC2 if i + 2 < b.len() => {
                                    pops.insert(u16::from_le_bytes([b[i + 1], b[i + 2]]) as u32);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            if falls {
                work.push(insn.addr + insn.len as u64);
            }
        }
        if pops.len() == 1 {
            scan.pop = pops.first().copied();
        }
        Some(scan)
    }

    pub fn retaddr(&self) -> Option<ExprId> {
        self.retaddr
    }

    /// The symbolic variable holding the stack pointer at entry.
    pub fn sp0(&self) -> Option<u32> {
        self.sp0
    }
}

pub fn exit_brief(e: &NodeExit) -> String {
    match e {
        NodeExit::Branch(s) => format!("branch->{s}"),
        NodeExit::CondBranch { arms, fall } => format!("cbranch->{:?}/{fall}", arms.iter().map(|(_, t)| *t).collect::<Vec<_>>()),
        NodeExit::Select { targets, .. } => format!("select->{:?}", targets.iter().map(|(_, s)| *s).collect::<Vec<_>>()),
        NodeExit::Call { target, ret, .. } => format!("call({:?})->{ret}", target.map(|t| format!("{t:#x}"))),
        NodeExit::Return => "return".into(),
        NodeExit::TailCall(c) => format!("tailcall {c:#x}"),
        NodeExit::TailCallSlot(c) => format!("tailcall [{c:#x}]"),
        NodeExit::NoReturn => "noreturn".into(),
        NodeExit::Unresolved(m) => format!("unresolved: {}", m.chars().take(120).collect::<String>()),
        NodeExit::Halt(m) => format!("halt: {}", m.chars().take(120).collect::<String>()),
    }
}

/// Fix a boolean to a constant throughout the state.
pub fn pin(arena: &mut ExprArena, state: &mut State, atom: ExprId, value: ExprId) {
    let mut map = BTreeMap::new();
    map.insert(atom, value);
    state.map_values(arena, &mut |a, e| a.substitute(e, &map));
}

/// Replace each pair of atoms `SLESS(a + b, 0)` / `SCARRY(a, b)` (the sign
/// and overflow flags of one addition) that the assignments only consult
/// through their exclusive or by the single atom `a < -b`, which is what that
/// exclusive or states when `b` is a constant other than the minimum.  This is
/// the signed comparison a `jl`/`jge`/`jg`/`jle` pair of flags encodes.
pub fn fuse_signed_compares(arena: &mut ExprArena, atoms: &mut Vec<ExprId>, targets: &mut [(Vec<Vec<(ExprId, bool)>>, NodeId)]) {
    use super::kuna_pushanexpr::ExprKind;
    use OpCode::*;
    let mut pairs = Vec::new();
    for &o in atoms.iter() {
        let ExprKind::Bin(CPUI_INT_SCARRY, p, q) = arena.kind(o).clone() else { continue };
        let (a, b) = if arena.is_const(q).is_some() { (p, q) } else { (q, p) };
        let size = arena.size(a);
        let Some(bc) = arena.is_const(b) else { continue };
        if bc == 1u128 << (8 * size as u32 - 1) {
            continue;
        }
        let sum = arena.mk_bin(CPUI_INT_ADD, a, b, size);
        let zero = arena.cst(size, 0);
        let sf_expr = arena.mk_bin(CPUI_INT_SLESS, sum, zero, 1);
        // The sign test may be stored as the negation of an atom.
        let not_sum = arena.mk_un(CPUI_INT_NEGATE, sum, size);
        let sf_not = arena.mk_bin(CPUI_INT_SLESS, not_sum, zero, 1);
        let (sf, flip) = if atoms.contains(&sf_expr) {
            (sf_expr, false)
        } else if atoms.contains(&sf_not) {
            // ~x < 0 exactly when x >= 0.
            (sf_not, true)
        } else if let ExprKind::Bin(CPUI_INT_XOR, inner, k) = arena.kind(sf_expr).clone() {
            if arena.is_const(k) == Some(1) && atoms.contains(&inner) {
                (inner, true)
            } else {
                continue;
            }
        } else {
            continue;
        };
        let x = if let ExprKind::Un(CPUI_INT_NEGATE, y) = arena.kind(a).clone() {
            // ~y < -b  is  b - 1 < y.
            let one = arena.cst(size, 1);
            let lo = arena.mk_bin(CPUI_INT_SUB, b, one, size);
            arena.mk_bin(CPUI_INT_SLESS, lo, y, 1)
        } else {
            let nb = arena.mk_un(CPUI_INT_2COMP, b, size);
            arena.mk_bin(CPUI_INT_SLESS, a, nb, 1)
        };
        pairs.push((sf, o, x, flip));
    }
    for (sf, of, x, flip) in pairs {
        let mut rewritten: Vec<(Vec<Vec<(ExprId, bool)>>, NodeId)> = Vec::with_capacity(targets.len());
        let mut seen: Vec<(Vec<(ExprId, bool)>, NodeId)> = Vec::new();
        let mut ok = true;
        'outer: for (assigns, succ) in targets.iter() {
            let mut out = Vec::new();
            for assign in assigns {
                let s = assign.iter().find(|(a, _)| *a == sf).map(|(_, v)| *v);
                let o = assign.iter().find(|(a, _)| *a == of).map(|(_, v)| *v);
                let (Some(s), Some(o)) = (s, o) else {
                    ok = false;
                    break 'outer;
                };
                let mut lits: Vec<(ExprId, bool)> = assign.iter().filter(|(a, _)| *a != sf && *a != of).copied().collect();
                lits.push((x, s ^ o ^ flip));
                if let Some((_, t)) = seen.iter().find(|(l, _)| *l == lits) {
                    if t != succ {
                        ok = false;
                        break 'outer;
                    }
                    continue;
                }
                seen.push((lits.clone(), *succ));
                out.push(lits);
            }
            rewritten.push((out, *succ));
        }
        if !ok {
            continue;
        }
        for (t, r) in targets.iter_mut().zip(rewritten) {
            *t = r;
        }
        atoms.retain(|a| *a != sf && *a != of);
        atoms.push(x);
    }
}

/// Does `outer` contain every byte of `inner`?
fn covers(outer: &SymLoc, inner: &SymLoc) -> bool {
    let (o0, i0) = match (outer.loc, inner.loc) {
        (Loc::Reg { off: a }, Loc::Reg { off: b }) => (a as i128, b as i128),
        (Loc::Mem { base: ba, off: a }, Loc::Mem { base: bb, off: b }) if ba == bb => (a, b),
        _ => return false,
    };
    o0 <= i0 && i0 + inner.size as i128 <= o0 + outer.size as i128
}

/// A copy routine's effect on its destination (`State::copy_in`, or the
/// destination forgotten when the copy cannot be modelled).
fn place_copy(state: &mut State, (b, o, s, n): (Option<VarId>, i128, u64, u64)) {
    if !state.copy_in(b, o, s, n) {
        state.forget_copies(b, o, o + n as i128);
        if let Some(region) = state.mem.get_mut(&b) {
            region.remove_where(|x| x >= o && x < o + n as i128);
        }
    }
}

/// The context a call's return continues in.  The interpreter exits to
/// make the call and restores the native registers, so a register that
/// held the VPC holds a program value afterwards (a memory cell in the VM
/// context keeps it); the return address salts the context so two re-entry
/// stubs never share a key through a stale VPC.
fn after_call(mut ctx: VmCtx, ret: u64) -> VmCtx {
    if ctx.vpc_reg.take().is_some() {
        ctx.ctx = Some(ctx.ctx.unwrap_or(0).rotate_left(17) ^ ret);
    }
    ctx
}

/// The context for a VPC with these possible values.
fn vpc_key(vals: &[u64]) -> u64 {
    if let [c] = vals {
        return *c;
    }
    let mut h: u64 = 0x5bd1e995;
    for x in vals {
        h = h.wrapping_mul(0x100000001b3).wrapping_add(*x);
    }
    h
}

/// The last control-flow op of `insn`.
fn flow_op(insn: &LiftedInsn) -> Option<&super::kuna_pushanemu::RawOp> {
    insn.ops.iter().rev().find(|op| {
        matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN)
    })
}

/// Whether `insn` branches back to its own start (an x86 `rep` prefix).
fn branches_to_itself(insn: &LiftedInsn) -> bool {
    insn.ops.iter().any(|op| {
        matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
            && classify_space_of(&op.ins[0]) == Some(super::kuna_pushanemu::Space::Ram)
            && op.ins[0].offset == insn.addr
    })
}

/// The value of symbolized location `l` in `state`.
fn read_loc(state: &mut State, arena: &mut ExprArena, image: &dyn PushanImage, l: &SymLoc) -> ExprId {
    match l.loc {
        Loc::Reg { off } => state.read_reg(arena, off, l.size),
        Loc::Mem { base, off } => state.read_mem(arena, image, base, off, l.size),
    }
}

/// Record that merge symbol `v` stood for `e` on one path into its node.
fn add_source(sources: &mut BTreeMap<VarId, Option<BTreeSet<ExprId>>>, v: VarId, e: ExprId) {
    let entry = sources.entry(v).or_insert_with(|| Some(BTreeSet::new()));
    if let Some(set) = entry {
        set.insert(e);
        if set.len() > MAX_SYM_SOURCES {
            *entry = None;
        }
    }
}

pub fn key_tag(key: &NodeKey) -> u64 {
    key.addr.wrapping_mul(0x9E3779B97F4A7C15)
        ^ key.ctx.unwrap_or(0xFFFF_FFFF_FFFF_FFFF).rotate_left(29)
        ^ key.sp.map(|k| (k as u64).wrapping_mul(0xff51afd7ed558ccd)).unwrap_or(0)
        ^ key.holder.map(|r| (r + 1).wrapping_mul(0xc4ceb9fe1a85ec53).rotate_left(13)).unwrap_or(0)
}
