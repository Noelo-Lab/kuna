//! (kuna pushan) VPC-sensitive flat-CFG recovery.
//!
//! The paper's stage 1.  A node is a native address plus the virtual program
//! counter the interpreter holds when it gets there, so every bytecode
//! instruction's handler copy is its own block and the recovered graph is the
//! protected program's control flow laid flat through the interpreter.  Each
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

use super::kuna_pushanemu::{merge_ite, Emulator, Exit, InsnTrace, LiftedInsn, PushanTarget, SymValues};
use super::kuna_pushanexpr::{ExprArena, ExprId, VarId, VarKind};
use super::kuna_pushanstate::{Loc, State, SymLoc};

pub type NodeId = u32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NodeKey {
    pub addr: u64,
    pub ctx: Option<u64>,
}

/// Per-path interpreter context carried along edges (not part of the state).
#[derive(Clone, Copy, Debug, Default)]
pub struct VmCtx {
    pub vpc_reg: Option<u64>,
    /// A concrete memory cell holding the VPC (Themida's VM context); it
    /// takes precedence over `vpc_reg`.
    pub vpc_cell: Option<u64>,
    pub ctx: Option<u64>,
    pub in_vm: bool,
    /// A far value the VPC register took without a fetch, followed once it
    /// survives a second block.
    pub pending_vpc: Option<u64>,
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
    /// A target the emulation could not enumerate; the original op is kept.
    Unresolved(String),
    Halt(String),
}

#[derive(Clone, Debug)]
pub struct Node {
    /// Where each variable of a select's deciding atoms lives when the node
    /// exits, as the constant it is offset by there, with the stack pointer's
    /// offset from its entry value there.
    pub exit_locs: Option<(Vec<(VarId, Loc, i128)>, Option<i128>)>,
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
    /// Pointer values the emulator read from image slots (value -> slot).
    image_ptrs: BTreeMap<u64, u64>,
    /// Entry-stack offsets passed to external calls, over every round.
    escapes: BTreeSet<i128>,
    /// Locations recorded by the last select built, for its node.
    pending_locs: Option<(Vec<(VarId, Loc, i128)>, Option<i128>)>,
    /// Callees in the image already read.
    callees: BTreeMap<u64, Option<CalleeScan>>,
    /// The native addresses nodes start at this round (handler entries).
    node_addrs: BTreeSet<u64>,
    pub stats: Stats,
    pub log: Vec<String>,
    retaddr: Option<ExprId>,
    sp0: Option<u32>,
    new_syms: usize,
    started: std::time::Instant,
    pub timed_out: bool,
}

const VPC_MAX_DISP: i128 = 16;
/// Farthest a new VPC register's value may sit from the previous context.
const VPC_NEAR: u64 = 0x100;
/// Most constants remembered for one symbolized location.
const MAX_SYM_VALUES: usize = 32;
/// Distinct virtual-PC values a node may fork into.
const MAX_VPC_FORK: usize = 4;
/// Bytes behind each pointer argument an external call may write.
const CALL_POINTEE_BYTES: i128 = 16;
/// Most instructions read from a callee in the image when summarizing it.
const MAX_CALLEE_SCAN: usize = 512;
/// Most stack words a call's pushed arguments are counted over.
const MAX_PUSHED_WORDS: i128 = 16;

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
            image_ptrs: BTreeMap::new(),
            escapes: BTreeSet::new(),
            pending_locs: None,
            callees: BTreeMap::new(),
            node_addrs: BTreeSet::new(),
            stats: Stats::default(),
            log: Vec::new(),
            retaddr: None,
            sp0: None,
            new_syms: 0,
            started: std::time::Instant::now(),
            timed_out: false,
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

    /// Run exploration rounds to the symbolization fixpoint.
    pub fn run(&mut self) -> FlatCfg {
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0, escapes: Vec::new() };
        let mut prev_nodes = usize::MAX;
        for round in 0..self.config.max_rounds {
            self.stats.rounds = round + 1;
            self.new_syms = 0;
            if self.timed_out {
                break;
            }
            cfg = self.explore_round();
            cfg.escapes = self.escapes.iter().copied().collect();
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
            if self.new_syms == 0 && n == prev_nodes {
                break;
            }
            if self.new_syms == 0 && round > 0 {
                break;
            }
            prev_nodes = n;
        }
        self.stats.nodes = cfg.nodes.len();
        self.stats.insns = cfg.nodes.iter().map(|n| n.insns.len()).sum();
        self.stats.vm_nodes = cfg.nodes.iter().filter(|n| n.entry_ctx.in_vm).count();
        self.stats.symbolized = self.to_symbolize.values().map(|s| s.len()).sum();
        cfg
    }

    fn explore_round(&mut self) -> FlatCfg {
        self.stats.unresolved = 0;
        self.stats.halts = 0;
        self.node_addrs.clear();
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0, escapes: Vec::new() };
        let mut worklist: VecDeque<NodeId> = VecDeque::new();
        let init = self.initial_state();
        let key = NodeKey { addr: self.entry_addr, ctx: None };
        let entry = self.reach(&mut cfg, &mut worklist, key, init, VmCtx::default(), None);
        cfg.entry = entry;
        let mut emu = Emulator::new(self.target);
        emu.sym_values = Rc::clone(&self.sym_values);
        emu.track_sp = Some((self.abi.sp, self.abi.ptr_size));
        emu.watch_base = if self.config.trace.is_some() { self.sp0 } else { None };
        emu.stack_base = if self.abi.stack_args > 0 { self.sp0 } else { None };
        while let Some(id) = worklist.pop_front() {
            if cfg.nodes.len() > self.config.max_nodes {
                self.log.push("node limit reached".into());
                break;
            }
            if self.started.elapsed().as_secs() >= self.config.max_seconds {
                self.log.push(format!("time budget of {}s exhausted", self.config.max_seconds));
                self.timed_out = true;
                break;
            }
            self.emulate_node(&mut cfg, &mut worklist, &mut emu, id);
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
                if let Some(tr) = &self.config.trace {
                    use std::io::Write;
                    let _ = writeln!(tr.borrow_mut(), "        stack pointer differs at node {id}: not merged");
                }
                return id;
            }
            let diffs = entry.diff_constants(&mut state, &mut self.arena, self.target, self.abi.ptr_size);
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
                let (a, b) = match d.loc.loc {
                    Loc::Reg { off } => (entry.read_reg(&mut self.arena, off, d.loc.size), state.read_reg(&mut self.arena, off, d.loc.size)),
                    Loc::Mem { base, off } => (
                        entry.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                        state.read_mem(&mut self.arena, self.target, base, off, d.loc.size),
                    ),
                };
                if !self.same_on_stack(a, b) {
                    kept.push(d);
                }
            }
            let diffs = kept;
            let set = self.to_symbolize.entry(key).or_default();
            for d in diffs {
                // The VPC itself is part of the key; never symbolize it.
                if let Loc::Reg { off } = d.loc.loc {
                    if vpc_reg.is_some_and(|r| off >= r && off < r + self.abi.ptr_size as u64) {
                        continue;
                    }
                }
                if let (Loc::Reg { off }, Some((pc, pcs))) = (d.loc.loc, self.abi.pc) {
                    if off >= pc && off < pc + pcs as u64 {
                        continue;
                    }
                }
                if let Loc::Mem { base: None, off } = d.loc.loc {
                    if vpc_cell.is_some_and(|a| off < a as i128 + self.abi.ptr_size as i128 && a as i128 <= off + d.loc.size as i128 - 1) {
                        continue;
                    }
                }
                // Memory under any other symbolic base is keyed by a variable
                // that is fresh every round: it can never be re-applied.
                if let Loc::Mem { base: Some(b), .. } = d.loc.loc {
                    if Some(b) != self.sp0 {
                        continue;
                    }
                }
                // A difference inside a location already recorded is that location.
                let mut d = d;
                if let Some(cover) = set.keys().find(|l| covers(l, &d.loc)).copied() {
                    d.loc = cover;
                }
                let is_new = !set.contains_key(&d.loc);
                let k = self.sym_known.entry((key, d.loc)).or_insert(d.known);
                let narrowed = (k.0 & d.known.0, k.1 & d.known.1);
                let shrank = narrowed != *k;
                *k = narrowed;
                let values = set.entry(d.loc).or_default();
                let mut grew = false;
                for v in [d.values.map(|v| v.0), d.values.map(|v| v.1), d.entry_const, d.incoming_const].into_iter().flatten() {
                    if values.len() < MAX_SYM_VALUES && values.insert(v) {
                        grew = true;
                    }
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
        if let Some(locs) = self.to_symbolize.get(&key) {
            let with_known: Vec<(SymLoc, (u128, u128))> =
                locs.keys().map(|l| (*l, self.sym_known.get(&(key, *l)).copied().unwrap_or((0, 0)))).collect();
            let tag = key_tag(&key);
            let made = state.symbolize_known(&mut self.arena, tag, &with_known);
            for (l, e) in made {
                if let (Some(v), Some(vals)) = (self.arena.as_var(e), locs.get(&l)) {
                    self.sym_values.borrow_mut().insert(v, vals.clone());
                }
            }
        }
        let id = cfg.nodes.len() as NodeId;
        cfg.nodes.push(Node {
            exit_locs: None,
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
        worklist.push_back(id);
        id
    }

    /// Emulate one node.  An interpreter instruction that branches back to
    /// itself with a decided guard (a `rep movsb` with a concrete count)
    /// repeats inside the node, one instance per iteration, as angr's
    /// rep-movsb shortcut does, instead of re-reaching its own node and
    /// symbolizing its counters.
    fn emulate_node(&mut self, cfg: &mut FlatCfg, worklist: &mut VecDeque<NodeId>, emu: &mut Emulator<'a>, id: NodeId) {
        let mut state = cfg.nodes[id as usize].entry.clone();
        let ctx = cfg.nodes[id as usize].entry_ctx;
        let mut addr = cfg.nodes[id as usize].key.addr;
        let mut insns: Vec<Rc<LiftedInsn>> = Vec::new();
        let mut traces: Vec<InsnTrace> = Vec::new();
        let mut exit;
        loop {
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
                    Some(c) if c == here && ctx.in_vm && insns.len() < self.config.max_block_insns => continue,
                    Some(c) if c == after && repeated => exit = Exit::Fallthrough(after),
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
        self.image_ptrs.append(&mut emu.image_loads);
        let next_ctx = self.successor_ctx(&mut state, &traces, ctx, &insns);
        let node_exit = match self.fork_on_vpc(cfg, worklist, id, &exit, &mut state, next_ctx) {
            Some(e) => e,
            None => self.resolve_exit(cfg, worklist, id, exit, state, next_ctx, &insns),
        };
        let exit_locs = if matches!(node_exit, NodeExit::Select { .. }) { self.pending_locs.take() } else { None };
        self.pending_locs = None;
        let node = &mut cfg.nodes[id as usize];
        node.insns = insns;
        node.traces = traces;
        node.exit = node_exit;
        node.exit_locs = exit_locs;
    }

    /// Two values built over the entry stack pointer that agree wherever the
    /// stack is (the interpreter rebuilds a saved stack address through
    /// different arithmetic on two paths).  angr compares merged values under
    /// its stack-pointer preconstraint, so such a pair is never a difference.
    fn same_on_stack(&mut self, a: ExprId, b: ExprId) -> bool {
        if self.arena.is_const(a).is_some() || self.arena.is_const(b).is_some() {
            return false;
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
    /// interpreter already advanced past the value its guard tested); None
    /// when one has no home (it was only ever in flight).
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
            let (loc, k) = found.or(shifted)?;
            out.push((v, loc, k));
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
        let cases = self.arena.case_split(v)?;
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
        for (c, assigns, map, next) in plans {
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
    /// just advanced past a concrete access (a table walk) keys the loop so it
    /// unrolls.  Otherwise the context is inherited.
    fn successor_ctx(&mut self, state: &mut State, traces: &[InsnTrace], ctx: VmCtx, insns: &[Rc<LiftedInsn>]) -> VmCtx {
        let ps = self.abi.ptr_size;
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
                if self.target.bytecode_like(addr) {
                    fetches.push(addr);
                }
                let Some((reg, disp)) = a.via else { continue };
                if disp.abs() > VPC_MAX_DISP || !self.abi.gprs.contains(&reg) {
                    continue;
                }
                if !self.target.bytecode_like(addr) {
                    continue;
                }
                *fetch_regs.entry(reg).or_insert(0) += 1;
            }
        }
        let mut cell = ctx.vpc_cell.filter(|a| self.cell_values(state, *a, false).is_some());
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
            next.vpc_cell = Some(a);
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
        if !ctx.in_vm {
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

    /// The VPC's current value, from its memory cell or its register.
    fn vpc_expr(&mut self, state: &mut State, ctx: &VmCtx) -> Option<ExprId> {
        if let Some(a) = ctx.vpc_cell {
            return Some(state.read_mem(&mut self.arena, self.target, None, a as i128, self.abi.ptr_size));
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
        self.target.is_mapped(c) && !self.target.is_writable(c) && (!strict || self.target.bytecode_like(c)) && !self.written(state, c, 1)
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
                let key = NodeKey { addr: next, ctx: ctx.ctx };
                NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)))
            }
            Exit::Halt(m) => {
                self.stats.halts += 1;
                NodeExit::Halt(m)
            }
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
                let fkey = NodeKey { addr: fallthrough, ctx: ctx.ctx };
                let mut out_arms = Vec::with_capacity(arms.len());
                let mut earlier: Vec<ExprId> = Vec::new();
                let mut arm_targets: Vec<(u64, State)> = Vec::new();
                for (cond, target, mut taken_state) in arms {
                    let Some(tc) = self.arena.is_const(target) else {
                        self.stats.unresolved += 1;
                        return NodeExit::Unresolved("conditional branch with symbolic target".into());
                    };
                    let tkey = NodeKey { addr: tc as u64, ctx: ctx.ctx };
                    if tkey == fkey {
                        // Both sides continue at the same place (a CMOV-style
                        // skip): one state, selecting on the guard.
                        merge_ite(&mut state, &mut taken_state, &mut self.arena, self.target, cond);
                        continue;
                    }
                    if let Some((_, prev)) = arm_targets.iter_mut().find(|(a, _)| *a == tc as u64) {
                        merge_ite(prev, &mut taken_state, &mut self.arena, self.target, cond);
                        continue;
                    }
                    // This arm: its guard holds and every earlier guard failed.
                    pin(&mut self.arena, &mut taken_state, cond, one);
                    for e in &earlier {
                        pin(&mut self.arena, &mut taken_state, *e, zero);
                    }
                    pin(&mut self.arena, &mut state, cond, zero);
                    earlier.push(cond);
                    arm_targets.push((tc as u64, *taken_state));
                    out_arms.push((cond, tc as u64));
                }
                let mut resolved = Vec::with_capacity(out_arms.len());
                for ((cond, tc), (_, st)) in out_arms.into_iter().zip(arm_targets.into_iter()) {
                    let tkey = NodeKey { addr: tc, ctx: ctx.ctx };
                    let taken = self.reach(cfg, worklist, tkey, st, ctx, Some(id));
                    resolved.push((cond, taken));
                }
                let fall = self.reach(cfg, worklist, fkey, state, ctx, Some(id));
                if resolved.is_empty() {
                    return NodeExit::Branch(fall);
                }
                NodeExit::CondBranch { arms: resolved, fall }
            }
            Exit::Call { target, fallthrough } => {
                let tc = self.arena.is_const(target).map(|c| c as u64);
                if let Some(c) = tc {
                    if self.target.is_foreign_code(c) && !self.target_is_function(c) {
                        // Dispatch through the interpreter: the pushed return
                        // address salts the context so each entry stub is distinct.
                        if !ctx.in_vm || (ctx.vpc_reg.is_none() && ctx.vpc_cell.is_none()) {
                            let salt = ctx.ctx.unwrap_or(0).rotate_left(17) ^ fallthrough;
                            ctx.ctx = Some(salt);
                        }
                        let key = NodeKey { addr: c, ctx: ctx.ctx };
                        return NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)));
                    }
                }
                // A target read from a data slot of the image: an unbound import.
                let slot = tc.filter(|c| self.classify_target(*c).is_err()).and_then(|c| self.image_ptrs.get(&c).copied()).filter(|s| !self.target.is_executable(*s));
                let tc = if slot.is_some() { None } else { tc };
                let callee = Callee { target: tc, slot, ret: Some(fallthrough) };
                self.apply_call_summary(&mut state, insns.last().map(|i| i.addr).unwrap_or(0), callee);
                let ctx = after_call(ctx, fallthrough);
                let key = NodeKey { addr: fallthrough, ctx: ctx.ctx };
                let ret = self.reach(cfg, worklist, key, state, ctx, Some(id));
                NodeExit::Call { target: tc, ret, slot }
            }
        }
    }

    /// `t` is the untouched initial contents of a stack slot above the entry
    /// stack pointer: entered past a native prologue (Themida virtualizes
    /// from inside the function), the return address sits there.
    fn is_caller_slot(&self, t: ExprId) -> bool {
        let Some(v) = self.arena.as_var(t) else { return false };
        let info = self.arena.var_info(v);
        info.size == self.abi.ptr_size && matches!(info.kind, VarKind::InitMem { base, offset } if base.is_some() && base == self.sp0 && offset > 0 && offset % self.abi.ptr_size as i128 == 0)
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
                let _ = writeln!(tr.borrow_mut(), "          atom {}", self.arena.fmt(*a).chars().take(1500).collect::<String>());
            }
            let vars = self.arena.free_vars(t);
            let _ = writeln!(tr.borrow_mut(), "          free vars: {:?}", vars.iter().map(|v| format!("{:?}", self.arena.var_info(*v).kind)).collect::<Vec<_>>());
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
        let key = NodeKey { addr: ret, ctx: ctx.ctx };
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
                        let key = NodeKey { addr: ret, ctx: ctx.ctx };
                        let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
                        return NodeExit::Call { target: None, ret: ret_node, slot: Some(slot) };
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
        // high-entropy target the interpreter pushed a return into itself for.
        let pushed = self.pushed_return_address(&mut state);
        let native = !self.target.is_foreign_code(c)
            && (self.target_is_function(c) || !self.target.bytecode_like(c) || pushed.is_some_and(|r| r != c && self.target.is_foreign_code(r)));
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
                    let key = NodeKey { addr: ret, ctx: ctx.ctx };
                    let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
                    return NodeExit::Call { target: Some(c), ret: ret_node, slot: None };
                }
            }
            if self.target_is_function(c) {
                return NodeExit::TailCall(c);
            }
        }
        let key = NodeKey { addr: c, ctx: ctx.ctx };
        NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)))
    }

    /// A node with no instructions that only performs `exit` (a call or tail
    /// call reached through a multi-way select).
    fn trampoline(&mut self, cfg: &mut FlatCfg, from: NodeId, addr: u64, exit: NodeExit) -> NodeId {
        let id = cfg.nodes.len() as NodeId;
        let tag = 0x7f00_0000_0000_0000u64 ^ (from as u64).rotate_left(32) ^ addr;
        cfg.nodes.push(Node {
            exit_locs: None,
            key: NodeKey { addr, ctx: Some(tag) },
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

    /// Model an external call: a fresh return value, clobbered registers, the
    /// return address popped, and the memory its pointer arguments reach made
    /// unknown (a `scanf("%d", &v)` writes `v`; its old constant must not
    /// decide the branch that tests it).
    fn apply_call_summary(&mut self, state: &mut State, site: u64, callee: Callee) {
        let ps = self.abi.ptr_size;
        let scan = callee.target.filter(|c| Some(*c) != callee.ret).and_then(|c| self.scan_callee(c));
        let pop = self.callee_pop(state, &callee, scan.as_ref());
        if self.config.trace.is_some() {
            self.log.push(format!("call at {site:#x}: pops {pop}"));
        }
        let args: Vec<(u64, u8)> = self.abi.args.iter().copied().filter(|(o, _)| *o != self.abi.sp).collect();
        let mut values: Vec<ExprId> = args.iter().map(|&(off, size)| state.read_reg(&mut self.arena, off, size)).collect();
        // Stack-passed arguments sit above the return address: the ABI's
        // window, and every word a callee-cleanup callee pops.
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        if let Some((sb, so)) = self.arena.affine_probe(sp) {
            for k in 1..=(self.abi.stack_args as i128).max(pop as i128 / ps as i128) {
                values.push(state.read_mem(&mut self.arena, self.target, sb, so + k * ps as i128, ps));
            }
        }
        for v in values {
            if self.config.trace.is_some() {
                self.log.push(format!("call at {site:#x}: arg {} affine={:?}", self.arena.fmt(v), self.arena.affine(v)));
            }
            match self.arena.affine_probe(v) {
                Some((Some(base), o)) => {
                    if Some(base) == self.sp0 {
                        self.escapes.insert(o);
                    }
                    for k in 0..(CALL_POINTEE_BYTES / ps as i128) {
                        let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallOutput { site }, ps, false));
                        state.write_mem(Some(base), o + k * ps as i128, u, ps);
                    }
                }
                Some((None, a)) => {
                    let a = a as u64;
                    if self.target.is_mapped(a) && self.target.is_writable(a) {
                        for k in 0..(CALL_POINTEE_BYTES / ps as i128) {
                            let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallOutput { site }, ps, false));
                            state.write_mem(None, a as i128 + k * ps as i128, u, ps);
                        }
                    }
                }
                _ => {}
            }
        }
        let kept = |off: u64, size: u8| scan.as_ref().is_some_and(|s| s.closed && !s.writes.iter().any(|&(o, n)| o < off + size as u64 && off < o + n as u64));
        if !kept(self.abi.return_reg, ps) {
            let ret = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallResult { site }, ps, false));
            state.write_reg(self.abi.return_reg, ret, ps);
        }
        let flags = self.abi.flag_regs;
        for &(off, size) in &self.abi.clobbers {
            if kept(off, size) {
                continue;
            }
            let boolean = size == 1 && off >= flags.0 && off < flags.1;
            let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallClobber { site, offset: off }, size, boolean));
            state.write_reg(off, u, size);
        }
        state.stack_stores.clear();
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let k = self.arena.cst(ps, ps as u128 + pop as u128);
        let nsp = self.arena.mk_bin(OpCode::CPUI_INT_ADD, sp, k, ps);
        state.write_reg(self.abi.sp, nsp, ps);
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

pub fn key_tag(key: &NodeKey) -> u64 {
    key.addr.wrapping_mul(0x9E3779B97F4A7C15) ^ key.ctx.unwrap_or(0xFFFF_FFFF_FFFF_FFFF).rotate_left(29)
}
