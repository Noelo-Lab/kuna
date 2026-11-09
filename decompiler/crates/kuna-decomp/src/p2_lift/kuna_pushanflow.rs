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

use super::kuna_pushanemu::{merge_ite, Emulator, Exit, InsnTrace, LiftedInsn, PushanTarget};
use super::kuna_pushanexpr::{ExprArena, ExprId, VarKind};
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
    pub ctx: Option<u64>,
    pub in_vm: bool,
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
    Call { target: Option<u64>, ret: NodeId },
    /// The protected function returns to its caller.
    Return,
    /// A jump to another function's entry.
    TailCall(u64),
    /// A target the emulation could not enumerate; the original op is kept.
    Unresolved(String),
    Halt(String),
}

#[derive(Clone, Debug)]
pub struct Node {
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
    sym_values: BTreeMap<u32, BTreeSet<u128>>,
    /// Bits every value seen at a symbolized location agreed on.
    sym_known: BTreeMap<(NodeKey, SymLoc), (u128, u128)>,
    pub stats: Stats,
    pub log: Vec<String>,
    retaddr: Option<ExprId>,
    sp0: Option<u32>,
    new_syms: usize,
    started: std::time::Instant,
    pub timed_out: bool,
}

const VPC_MAX_DISP: i128 = 16;
/// Most constants remembered for one symbolized location.
const MAX_SYM_VALUES: usize = 32;
/// Distinct virtual-PC values a node may fork into.
const MAX_VPC_FORK: usize = 4;
/// Bytes behind each pointer argument an external call may write.
const CALL_POINTEE_BYTES: i128 = 16;

impl<'a> Explorer<'a> {
    pub fn new(target: &'a dyn PushanTarget, abi: Abi, config: Config, entry_addr: u64) -> Self {
        Explorer {
            target,
            abi,
            config,
            arena: ExprArena::new(),
            entry_addr,
            to_symbolize: BTreeMap::new(),
            sym_values: BTreeMap::new(),
            sym_known: BTreeMap::new(),
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
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0 };
        let mut prev_nodes = usize::MAX;
        for round in 0..self.config.max_rounds {
            self.stats.rounds = round + 1;
            self.new_syms = 0;
            if self.timed_out {
                break;
            }
            cfg = self.explore_round();
            let n = cfg.nodes.len();
            self.log.push(format!(
                "round {}: {} nodes, {} new symbolized locations, {} unresolved, {} halts",
                round + 1,
                n,
                self.new_syms,
                self.stats.unresolved,
                self.stats.halts
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
        let mut cfg = FlatCfg { nodes: Vec::new(), index: BTreeMap::new(), entry: 0 };
        let mut worklist: VecDeque<NodeId> = VecDeque::new();
        let init = self.initial_state();
        let key = NodeKey { addr: self.entry_addr, ctx: None };
        let entry = self.reach(&mut cfg, &mut worklist, key, init, VmCtx::default(), None);
        cfg.entry = entry;
        let mut emu = Emulator::new(self.target);
        emu.track_sp = Some((self.abi.sp, self.abi.ptr_size));
        emu.watch_base = if self.config.trace.is_some() { self.sp0 } else { None };
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
                    "[{:>7.3}s] node {} key={:#x}/{} vm={} vpc_reg={:?} insns={} exit={} arena={} mem_bytes={} worklist={}",
                    self.started.elapsed().as_secs_f64(),
                    id,
                    n.key.addr,
                    n.key.ctx.map(|c| format!("{c:#x}")).unwrap_or_else(|| "-".into()),
                    n.entry_ctx.in_vm,
                    n.entry_ctx.vpc_reg,
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
            let diffs = entry.diff_constants(&mut state, &mut self.arena, self.target);
            let vpc_reg = node.entry_ctx.vpc_reg;
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
            let set = self.to_symbolize.entry(key).or_default();
            for d in diffs {
                // The VPC itself is part of the key; never symbolize it.
                if let Loc::Reg { off } = d.loc.loc {
                    if vpc_reg.is_some_and(|r| off >= r && off < r + self.abi.ptr_size as u64) {
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
                    self.sym_values.insert(v, vals.clone());
                }
            }
        }
        let id = cfg.nodes.len() as NodeId;
        cfg.nodes.push(Node {
            key,
            entry: state,
            entry_ctx: ctx,
            insns: Vec::new(),
            traces: Vec::new(),
            exit: NodeExit::Halt("unexplored".into()),
            preds: from.into_iter().collect(),
        });
        cfg.index.insert(key, id);
        worklist.push_back(id);
        id
    }

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
                if std::env::var("KUNA_PUSHAN_TRACE_OPS").is_ok() {
                    for (i, op) in insn.ops.iter().enumerate() {
                        if let Some(Some(v)) = trace.op_out.get(i) {
                            if self.arena.is_const(*v).is_none() {
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
            insns.push(insn);
            traces.push(trace);
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
        let next_ctx = self.successor_ctx(&mut state, &traces, ctx);
        let node_exit = match self.fork_on_vpc(cfg, worklist, id, &exit, &mut state, next_ctx) {
            Some(e) => e,
            None => self.resolve_exit(cfg, worklist, id, exit, state, next_ctx, &insns),
        };
        let node = &mut cfg.nodes[id as usize];
        node.insns = insns;
        node.traces = traces;
        node.exit = node_exit;
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
        let r = ctx.vpc_reg?;
        if !matches!(exit, Exit::Fallthrough(_) | Exit::Jump(_)) {
            return None;
        }
        let v = state.read_reg(&mut self.arena, r, self.abi.ptr_size);
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
        Some(NodeExit::Select { atoms, targets, by_atoms: true })
    }

    /// The interpreter context the block hands to its successors.
    fn successor_ctx(&mut self, state: &mut State, traces: &[InsnTrace], ctx: VmCtx) -> VmCtx {
        let ps = self.abi.ptr_size;
        let mut fetch_regs: BTreeMap<u64, usize> = BTreeMap::new();
        for t in traces {
            for a in &t.accesses {
                if a.is_store {
                    continue;
                }
                let (Some((reg, disp)), Some(addr)) = (a.via, a.addr) else { continue };
                if disp.abs() > VPC_MAX_DISP || !self.abi.gprs.contains(&reg) {
                    continue;
                }
                if !self.target.bytecode_like(addr) {
                    continue;
                }
                *fetch_regs.entry(reg).or_insert(0) += 1;
            }
        }
        let mut next = ctx;
        let chosen = if let Some(r) = ctx.vpc_reg.filter(|r| fetch_regs.contains_key(r)) {
            Some(r)
        } else {
            fetch_regs.iter().max_by_key(|(r, n)| (**n, std::cmp::Reverse(**r))).map(|(r, _)| *r)
        };
        if let Some(r) = chosen {
            let v = state.read_reg(&mut self.arena, r, ps);
            if let Some(c) = self.arena.is_const(v) {
                next.vpc_reg = Some(r);
                next.ctx = Some(c as u64);
                next.in_vm = true;
                return next;
            }
            if let Some(vals) = self.arena.possible_values(v) {
                if vals.len() <= 4 {
                    let mut h: u64 = 0x5bd1e995;
                    for x in &vals {
                        h = h.wrapping_mul(0x100000001b3).wrapping_add(*x as u64);
                    }
                    next.vpc_reg = Some(r);
                    next.ctx = Some(h);
                    next.in_vm = true;
                    return next;
                }
            }
        }
        // No fetch in this block: a VPC register that still holds a pointer
        // into the image may have been advanced (or point at a low-entropy
        // stretch of bytecode); follow its value.
        if let Some(r) = ctx.vpc_reg {
            let v = state.read_reg(&mut self.arena, r, ps);
            if let Some(c) = self.arena.is_const(v) {
                if self.target.is_mapped(c as u64) {
                    next.ctx = Some(c as u64);
                    return next;
                }
            } else if let Some(vals) = self.arena.possible_values(v) {
                if vals.len() <= 4 && vals.iter().all(|x| self.target.is_mapped(*x as u64)) {
                    let mut h: u64 = 0x5bd1e995;
                    for x in &vals {
                        h = h.wrapping_mul(0x100000001b3).wrapping_add(*x as u64);
                    }
                    next.ctx = Some(h);
                    return next;
                }
            }
        }
        next
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
                if Some(t) == self.retaddr {
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
                        if !ctx.in_vm || ctx.vpc_reg.is_none() {
                            let salt = ctx.ctx.unwrap_or(0).rotate_left(17) ^ fallthrough;
                            ctx.ctx = Some(salt);
                        }
                        let key = NodeKey { addr: c, ctx: ctx.ctx };
                        return NodeExit::Branch(self.reach(cfg, worklist, key, state, ctx, Some(id)));
                    }
                }
                self.apply_call_summary(&mut state, insns.last().map(|i| i.addr).unwrap_or(0));
                let key = NodeKey { addr: fallthrough, ctx: ctx.ctx };
                let ret = self.reach(cfg, worklist, key, state, ctx, Some(id));
                NodeExit::Call { target: tc, ret }
            }
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
        let Some(cases) = self.arena.case_split(t).or_else(|| self.enumerate_symbol_target(t)) else {
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
        let symbol_pins = atoms.len() == 2 && self.arena.as_var(atoms[0]).is_some_and(|v| self.sym_values.contains_key(&v));
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
            // The pinned state may make the VPC register concrete.
            if let Some(r) = ctx.vpc_reg {
                let v = st.read_reg(&mut self.arena, r, self.abi.ptr_size);
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
        NodeExit::Select { atoms, targets, by_atoms: !symbol_pins }
    }

    /// A target `sym + k` over one symbolized location: one case per constant
    /// the location was seen to hold, pinning the symbol to it.
    fn enumerate_symbol_target(&mut self, t: ExprId) -> Option<Vec<(Vec<(ExprId, bool)>, ExprId)>> {
        let (Some(var), k) = self.arena.affine(t)? else { return None };
        let values = self.sym_values.get(&var)?.clone();
        if values.is_empty() {
            return None;
        }
        let size = self.arena.size(t);
        let var_expr = self.arena.var_of(self.arena.var_info(var).kind, self.arena.var_info(var).size, false);
        let mut out = Vec::new();
        for v in values {
            let kc = self.arena.cst(self.arena.size(var_expr), v);
            let target = self.arena.cst(size, (v as i128).wrapping_add(k) as u128);
            // Encode the pin as a pseudo-atom: the symbol itself, resolved by
            // the caller through `pin_symbol`.
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
        if let Err(m) = self.classify_target(c) {
            self.stats.halts += 1;
            return NodeExit::Halt(m);
        }
        let native = !self.target.is_foreign_code(c) && !self.target.bytecode_like(c);
        if native {
            // `push return; jmp/ret target`: a call whose return address the
            // interpreter already pushed.
            if let Some(ret) = self.pushed_return_address(&mut state) {
                if ret != c {
                    let site = cfg.nodes[id as usize].key.addr;
                    self.apply_call_summary(&mut state, site);
                    let key = NodeKey { addr: ret, ctx: ctx.ctx };
                    let ret_node = self.reach(cfg, worklist, key, state, ctx, Some(id));
                    return NodeExit::Call { target: Some(c), ret: ret_node };
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
    fn apply_call_summary(&mut self, state: &mut State, site: u64) {
        let ps = self.abi.ptr_size;
        let args: Vec<(u64, u8)> = self.abi.args.iter().copied().filter(|(o, _)| *o != self.abi.sp).collect();
        for (off, size) in args {
            let v = state.read_reg(&mut self.arena, off, size);
            if self.config.trace.is_some() {
                self.log.push(format!("call at {site:#x}: arg reg{off:x} = {} affine={:?}", self.arena.fmt(v), self.arena.affine(v)));
            }
            match self.arena.affine_probe(v) {
                Some((Some(base), o)) => {
                    for k in 0..(CALL_POINTEE_BYTES / ps as i128) {
                        let u = self.arena.unknown(ps);
                        state.write_mem(Some(base), o + k * ps as i128, u, ps);
                    }
                }
                Some((None, a)) => {
                    let a = a as u64;
                    if self.target.is_mapped(a) && self.target.is_writable(a) {
                        for k in 0..(CALL_POINTEE_BYTES / ps as i128) {
                            let u = self.arena.unknown(ps);
                            state.write_mem(None, a as i128 + k * ps as i128, u, ps);
                        }
                    }
                }
                _ => {}
            }
        }
        let ret = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallResult { site }, ps, false));
        state.write_reg(self.abi.return_reg, ret, ps);
        let flags = self.abi.flag_regs;
        for &(off, size) in &self.abi.clobbers {
            let boolean = size == 1 && off >= flags.0 && off < flags.1;
            let u = self.arena.var(super::kuna_pushanexpr::VarInfo::new(VarKind::CallClobber { site, offset: off }, size, boolean));
            state.write_reg(off, u, size);
        }
        let sp = state.read_reg(&mut self.arena, self.abi.sp, ps);
        let mut pop = ps as u128;
        if let Some(extra) = self.target.callee_extra_pop(site) {
            pop += extra as u128;
        }
        let k = self.arena.cst(ps, pop);
        let nsp = self.arena.mk_bin(OpCode::CPUI_INT_ADD, sp, k, ps);
        state.write_reg(self.abi.sp, nsp, ps);
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
        NodeExit::Call { target, ret } => format!("call({:?})->{ret}", target.map(|t| format!("{t:#x}"))),
        NodeExit::Return => "return".into(),
        NodeExit::TailCall(c) => format!("tailcall {c:#x}"),
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

/// Does `outer` contain every byte of `inner`?
fn covers(outer: &SymLoc, inner: &SymLoc) -> bool {
    let (o0, i0) = match (outer.loc, inner.loc) {
        (Loc::Reg { off: a }, Loc::Reg { off: b }) => (a as i128, b as i128),
        (Loc::Mem { base: ba, off: a }, Loc::Mem { base: bb, off: b }) if ba == bb => (a, b),
        _ => return false,
    };
    o0 <= i0 && i0 + inner.size as i128 <= o0 + outer.size as i128
}

pub fn key_tag(key: &NodeKey) -> u64 {
    key.addr.wrapping_mul(0x9E3779B97F4A7C15) ^ key.ctx.unwrap_or(0xFFFF_FFFF_FFFF_FFFF).rotate_left(29)
}
