//! (kuna pushan) Flat CFG to synthetic p-code.
//!
//! Every recovered node becomes a run of synthetic instructions, one per
//! native instruction it executed, at addresses no section maps; the
//! translator's overlay ([`PcodeOverlay`]) serves them to the ordinary flow
//! follower, so the whole decompiler runs unchanged on the devirtualized
//! program.  The native p-code is copied with two kinds of edits.  Control
//! flow is rewritten to what the exploration resolved: a pruned conditional
//! becomes a jump, a computed target becomes a compare chain over its
//! enumerated destinations, a dispatch call becomes a jump, a return to the
//! caller stays a return.  Data flow is simplified from the emulation: an op
//! whose result the interpreter state fixes to a constant becomes that
//! constant (bytecode fetches, key arithmetic, handler addresses), and an op
//! whose result is the entry stack pointer plus a constant is rebuilt as
//! `sp + k`, which undoes the obfuscated stack arithmetic the paper's stage 3
//! has to repair downstream.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::kuna_pcodeoverlay::{OverlayInsn, OverlayOp, PcodeOverlay};
use OpCode::*;

use super::kuna_pushanemu::{classify_space, LiftedInsn, PushanTarget, RawOp, Space};
use super::kuna_pushanexpr::{ExprArena, ExprId};
use super::kuna_pushanflow::{FlatCfg, NodeExit, NodeId};

pub struct EmitConfig {
    pub syn_base: u64,
    pub stride: u64,
    pub ram: Rc<AddrSpace>,
    pub const_space: Rc<AddrSpace>,
    pub unique: Rc<AddrSpace>,
    pub register: Rc<AddrSpace>,
    /// Unique-space offset reserved for the emitter's own temporaries.
    pub scratch: u64,
    pub ptr_size: u8,
    pub sp_reg: u64,
    pub sp0: Option<u32>,
    pub fold_constants: bool,
    pub remat_sp: bool,
    /// Registers live when the function returns (return value, callee-saved, sp).
    pub live_at_exit: Vec<(u64, u8)>,
    /// Registers an external call reads (argument registers, sp).
    pub call_args: Vec<(u64, u8)>,
    pub eliminate_dead: bool,
    /// User-op indices with no effect beyond their outputs (rdtsc, pause, fences).
    pub pure_userops: Vec<u64>,
    /// Register-space offset where the emitter's boolean witness slots start.
    pub slot_base: u64,
}

#[derive(Default, Debug, Clone)]
pub struct EmitStats {
    pub insns: usize,
    pub ops: usize,
    pub folded: usize,
    pub remat: usize,
    pub kept_unresolved: usize,
    pub dead: usize,
}

/// A set of register-space bytes.
#[derive(Clone, PartialEq, Eq)]
struct RegSet {
    words: Vec<u64>,
}

impl RegSet {
    fn new(bytes: usize) -> Self {
        RegSet { words: vec![0; bytes.div_ceil(64)] }
    }
    fn full(bytes: usize) -> Self {
        RegSet { words: vec![u64::MAX; bytes.div_ceil(64)] }
    }
    fn add_range(&mut self, off: u64, size: u32) {
        for b in off..off + size as u64 {
            let i = b as usize;
            if i / 64 < self.words.len() {
                self.words[i / 64] |= 1 << (i % 64);
            }
        }
    }
    fn remove_range(&mut self, off: u64, size: u32) {
        for b in off..off + size as u64 {
            let i = b as usize;
            if i / 64 < self.words.len() {
                self.words[i / 64] &= !(1 << (i % 64));
            }
        }
    }
    fn intersects_range(&self, off: u64, size: u32) -> bool {
        (off..off + size as u64).any(|b| {
            let i = b as usize;
            i / 64 < self.words.len() && self.words[i / 64] & (1 << (i % 64)) != 0
        })
    }
    fn union_with(&mut self, o: &RegSet) {
        for (a, b) in self.words.iter_mut().zip(o.words.iter()) {
            *a |= *b;
        }
    }
}

fn reg_vn(v: &VarnodeData) -> Option<(u64, u32)> {
    (v.space.as_ref().map(classify_space) == Some(Space::Register)).then(|| (v.offset, v.size))
}

/// Ops whose effect is not captured by their register outputs.
fn has_side_effect(op: &OverlayOp, pure_userops: &[u64]) -> bool {
    match op.opc {
        CPUI_CALLOTHER => !pure_userops.contains(&op.ins[0].offset),
        CPUI_STORE | CPUI_BRANCHIND | CPUI_CALL | CPUI_CALLIND | CPUI_RETURN => true,
        CPUI_BRANCH | CPUI_CBRANCH => op.ins[0].space.as_ref().map(classify_space) != Some(Space::Const),
        _ => op.out.as_ref().is_some_and(|o| o.space.as_ref().map(classify_space) == Some(Space::Ram)),
    }
}

/// Backward liveness over register bytes; marks instructions whose every
/// defined register byte is dead and that have no other effect.
fn mark_dead(cfg: &FlatCfg, conf: &EmitConfig, code: &[Vec<Vec<OverlayOp>>], dead: &mut [Vec<bool>]) -> usize {
    let mut extent = 0u64;
    for node in code {
        for insn in node {
            for op in insn {
                for v in op.out.iter().chain(op.ins.iter()) {
                    if let Some((o, s)) = reg_vn(v) {
                        extent = extent.max(o + s as u64);
                    }
                }
            }
        }
    }
    for (o, s) in conf.live_at_exit.iter().chain(conf.call_args.iter()) {
        extent = extent.max(o + *s as u64);
    }
    let bytes = (extent as usize).min(0x10000).max(64);
    let n = cfg.nodes.len();
    let mut total_dead = 0usize;
    loop {
        // Per-node gen/kill over the live instructions.
        let mut gen: Vec<RegSet> = Vec::with_capacity(n);
        let mut kill: Vec<RegSet> = Vec::with_capacity(n);
        for i in 0..n {
            let mut g = RegSet::new(bytes);
            let mut k = RegSet::new(bytes);
            for (j, insn) in code[i].iter().enumerate().rev() {
                if dead[i][j] {
                    continue;
                }
                for op in insn.iter().rev() {
                    if let Some((o, s)) = op.out.as_ref().and_then(reg_vn) {
                        g.remove_range(o, s);
                        k.add_range(o, s);
                    }
                    for v in &op.ins {
                        if let Some((o, s)) = reg_vn(v) {
                            g.add_range(o, s);
                        }
                    }
                }
            }
            gen.push(g);
            kill.push(k);
        }
        let exit_set = |regs: &[(u64, u8)]| {
            let mut s = RegSet::new(bytes);
            for (o, sz) in regs {
                s.add_range(*o, *sz as u32);
            }
            s
        };
        let live_exit = exit_set(&conf.live_at_exit);
        let call_args = exit_set(&conf.call_args);
        let full = RegSet::full(bytes);
        let mut live_in: Vec<RegSet> = (0..n).map(|_| RegSet::new(bytes)).collect();
        let mut live_out: Vec<RegSet> = (0..n).map(|_| RegSet::new(bytes)).collect();
        loop {
            let mut changed = false;
            for i in (0..n).rev() {
                let mut out = RegSet::new(bytes);
                match &cfg.nodes[i].exit {
                    NodeExit::Branch(s) => out.union_with(&live_in[*s as usize]),
                    NodeExit::CondBranch { arms, fall } => {
                        for (_, t) in arms {
                            out.union_with(&live_in[*t as usize]);
                        }
                        out.union_with(&live_in[*fall as usize]);
                    }
                    NodeExit::Select { targets, .. } => {
                        for (_, t) in targets {
                            out.union_with(&live_in[*t as usize]);
                        }
                    }
                    NodeExit::Call { ret, .. } => {
                        out.union_with(&live_in[*ret as usize]);
                        out.union_with(&call_args);
                    }
                    NodeExit::Return => out.union_with(&live_exit),
                    NodeExit::TailCall(_) | NodeExit::Unresolved(_) | NodeExit::Halt(_) => out = full.clone(),
                }
                let mut inn = out.clone();
                for (a, k) in inn.words.iter_mut().zip(kill[i].words.iter()) {
                    *a &= !*k;
                }
                inn.union_with(&gen[i]);
                if inn != live_in[i] || out != live_out[i] {
                    live_in[i] = inn;
                    live_out[i] = out;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut newly = 0usize;
        for i in 0..n {
            let mut live = live_out[i].clone();
            for (j, insn) in code[i].iter().enumerate().rev() {
                if dead[i][j] {
                    continue;
                }
                let effect = insn.iter().any(|op| has_side_effect(op, &conf.pure_userops));
                let defs: Vec<(u64, u32)> = insn.iter().filter_map(|op| op.out.as_ref().and_then(reg_vn)).collect();
                if !effect && defs.iter().all(|(o, s)| !live.intersects_range(*o, *s)) {
                    dead[i][j] = true;
                    newly += 1;
                    continue;
                }
                for op in insn.iter().rev() {
                    if let Some((o, s)) = op.out.as_ref().and_then(reg_vn) {
                        live.remove_range(o, s);
                    }
                    for v in &op.ins {
                        if let Some((o, s)) = reg_vn(v) {
                            live.add_range(o, s);
                        }
                    }
                }
            }
        }
        total_dead += newly;
        if newly == 0 {
            break;
        }
    }
    total_dead
}

struct Emitter<'a> {
    cfg: &'a FlatCfg,
    arena: &'a mut ExprArena,
    target: &'a dyn PushanTarget,
    conf: &'a EmitConfig,
    stats: EmitStats,
    /// Deciding booleans of multi-way selects, each with its witness slot.
    slots: BTreeMap<ExprId, u64>,
    /// The negation of a witnessed boolean, mapped back to it.
    negated: BTreeMap<ExprId, ExprId>,
    /// Booleans some emitted op actually witnessed.
    witnessed: BTreeSet<ExprId>,
    /// The scratch holding the rematerialized address of the access being emitted.
    last_addr_scratch: Option<VarnodeData>,
}

pub fn syn_addr(conf: &EmitConfig, node: NodeId) -> u64 {
    conf.syn_base + node as u64 * conf.stride
}

/// Build the overlay for a recovered graph; `entry_addr` maps to the entry node.
pub fn emit(cfg: &FlatCfg, arena: &mut ExprArena, target: &dyn PushanTarget, conf: &EmitConfig, entry_addr: u64) -> (PcodeOverlay, EmitStats) {
    let mut em = Emitter {
        cfg,
        arena,
        target,
        conf,
        stats: EmitStats::default(),
        slots: BTreeMap::new(),
        negated: BTreeMap::new(),
        witnessed: BTreeSet::new(),
        last_addr_scratch: None,
    };
    em.assign_slots();
    let mut overlay = PcodeOverlay::new(conf.ram.get_index());
    overlay.insns.insert(
        entry_addr,
        OverlayInsn {
            ops: vec![em.branch_op(syn_addr(conf, cfg.entry))],
            length: 1,
            mnemonic: "VM".into(),
            body: format!("entry -> node {}", cfg.entry),
        },
    );
    let mut code: Vec<Vec<Vec<OverlayOp>>> = Vec::with_capacity(cfg.nodes.len());
    for node in cfg.nodes.iter() {
        let mut insns = Vec::with_capacity(node.insns.len());
        for (k, insn) in node.insns.iter().enumerate() {
            insns.push(em.rewrite_insn(node, k, insn));
        }
        code.push(insns);
    }
    // Exits go in once every witness is known.
    for (idx, node) in cfg.nodes.iter().enumerate() {
        if let (Some(last), Some(insn)) = (code[idx].last_mut(), node.insns.last()) {
            let kept_flow = last.iter().any(|op| matches!(op.opc, CPUI_CALL | CPUI_CALLIND | CPUI_RETURN | CPUI_BRANCHIND)
                || (op.opc == CPUI_CBRANCH && op.ins[0].space.as_ref().map(classify_space) == Some(Space::Ram)));
            em.append_exit(node, insn, last, kept_flow);
        }
    }
    let mut dead: Vec<Vec<bool>> = code.iter().map(|n| vec![false; n.len()]).collect();
    if conf.eliminate_dead {
        em.stats.dead = mark_dead(cfg, conf, &code, &mut dead);
    }
    for (idx, node) in cfg.nodes.iter().enumerate() {
        let base = syn_addr(conf, idx as NodeId);
        let n = node.insns.len();
        if n == 0 {
            // A trampoline carries only its exit; an unexplored node is a dead end.
            match &node.exit {
                NodeExit::Halt(_) => {
                    overlay.insns.insert(base, em.halt_insn(node.key.addr, "unexplored"));
                }
                _ => {
                    let mut ops = Vec::new();
                    let fake = LiftedInsn { addr: node.key.addr, len: 1, ops: Vec::new(), asm: String::new() };
                    em.append_exit(node, &fake, &mut ops, false);
                    overlay.insns.insert(
                        base,
                        OverlayInsn { ops, length: 1, mnemonic: "VM".into(), body: format!("{:#x} trampoline", node.key.addr) },
                    );
                }
            }
            continue;
        }
        for (k, insn) in node.insns.iter().enumerate() {
            let ops = if dead[idx][k] { vec![em.nop_op()] } else { std::mem::take(&mut code[idx][k]) };
            em.stats.insns += 1;
            em.stats.ops += ops.len();
            overlay.insns.insert(
                base + k as u64,
                OverlayInsn {
                    ops,
                    length: 1,
                    mnemonic: "VM".into(),
                    body: format!("{:#x} {}{}", insn.addr, insn.asm, if dead[idx][k] { " (dead)" } else { "" }),
                },
            );
        }
    }
    (overlay, em.stats)
}

impl<'a> Emitter<'a> {
    fn vn(&self, space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }
    fn cst(&self, v: u64, size: u32) -> VarnodeData {
        self.vn(&self.conf.const_space, v, size)
    }
    fn code(&self, addr: u64) -> VarnodeData {
        self.vn(&self.conf.ram, addr, self.conf.ptr_size as u32)
    }
    fn scratch(&self, slot: u64, size: u32) -> VarnodeData {
        self.vn(&self.conf.unique, self.conf.scratch + slot * 16, size)
    }
    /// One witness slot per deciding boolean of every atom-based select.
    fn assign_slots(&mut self) {
        if std::env::var("KUNA_PUSHAN_NO_WITNESS").is_ok() {
            return;
        }
        let mut atoms: Vec<ExprId> = Vec::new();
        for node in &self.cfg.nodes {
            if let NodeExit::Select { atoms: a, by_atoms: true, .. } = &node.exit {
                for x in a {
                    if !atoms.contains(x) {
                        atoms.push(*x);
                    }
                }
            }
        }
        for (i, a) in atoms.iter().enumerate() {
            self.slots.insert(*a, self.conf.slot_base + 8 * i as u64);
            let one = self.arena.cst(1, 1);
            let na = self.arena.mk_bin(CPUI_INT_XOR, *a, one, 1);
            self.negated.insert(na, *a);
        }
    }

    fn slot_vn(&self, slot: u64) -> VarnodeData {
        self.vn(&self.conf.register, slot, 1)
    }

    fn branch_op(&self, target: u64) -> OverlayOp {
        OverlayOp { opc: CPUI_BRANCH, out: None, ins: vec![self.code(target)] }
    }

    fn nop_op(&self) -> OverlayOp {
        OverlayOp { opc: CPUI_COPY, out: Some(self.scratch(0, 1)), ins: vec![self.cst(0, 1)] }
    }

    fn halt_insn(&self, native: u64, why: &str) -> OverlayInsn {
        // A return of an unknown address: the decompiler shows the dead end.
        let out = self.scratch(1, self.conf.ptr_size as u32);
        OverlayInsn {
            ops: vec![
                OverlayOp { opc: CPUI_COPY, out: Some(out.clone()), ins: vec![self.cst(0, self.conf.ptr_size as u32)] },
                OverlayOp { opc: CPUI_RETURN, out: None, ins: vec![out] },
            ],
            length: 1,
            mnemonic: "VM".into(),
            body: format!("{native:#x} halt: {why}"),
        }
    }

    fn is_flow(op: &RawOp) -> bool {
        match op.opc {
            CPUI_BRANCH | CPUI_CBRANCH => {
                let t = &op.ins[0];
                t.space.as_ref().map(classify_space) != Some(Space::Const)
            }
            CPUI_BRANCHIND | CPUI_CALL | CPUI_CALLIND | CPUI_RETURN => true,
            _ => false,
        }
    }

    fn raw(op: &RawOp) -> OverlayOp {
        OverlayOp { opc: op.opc, out: op.out.clone(), ins: op.ins.clone() }
    }

    fn rewrite_insn(&mut self, node: &super::kuna_pushanflow::Node, k: usize, insn: &LiftedInsn) -> Vec<OverlayOp> {
        let is_last = k + 1 == node.insns.len();
        let trace = node.traces.get(k);
        let mut out: Vec<OverlayOp> = Vec::with_capacity(insn.ops.len() + 2);
        let mut kept_flow = false;
        let mut arm_index = 0usize;
        // Without a fork the emulation fixed every internal branch: emit the
        // executed path (loops unrolled) and drop the resolved branches.  A
        // forked instruction keeps its original op list and relative flow.
        let linear = trace.is_some_and(|t| !t.forked && !t.path.is_empty());
        let order: Vec<usize> = if linear { trace.unwrap().path.clone() } else { (0..insn.ops.len()).collect() };
        for i in order {
            let op = &insn.ops[i];
            if linear && !Self::is_flow(op) && matches!(op.opc, CPUI_BRANCH | CPUI_CBRANCH) {
                self.stats.folded += 1;
                continue;
            }
            if Self::is_flow(op) {
                if !is_last {
                    // A conditional the emulation decided against: never taken here.
                    out.push(self.nop_op());
                    continue;
                }
                match &node.exit {
                    NodeExit::Call { target: Some(t), .. } => {
                        out.push(OverlayOp { opc: CPUI_CALL, out: None, ins: vec![self.code(*t)] });
                        kept_flow = true;
                    }
                    NodeExit::Call { target: None, .. } => {
                        let tv = match op.opc {
                            CPUI_BRANCHIND | CPUI_RETURN | CPUI_CALLIND | CPUI_CALL => op.ins.first().cloned(),
                            _ => None,
                        };
                        match tv {
                            Some(tv) => out.push(OverlayOp { opc: CPUI_CALLIND, out: None, ins: vec![tv] }),
                            None => out.push(Self::raw(op)),
                        }
                        kept_flow = true;
                    }
                    NodeExit::Return if matches!(op.opc, CPUI_RETURN | CPUI_BRANCHIND) => {
                        out.push(OverlayOp { opc: CPUI_RETURN, out: None, ins: vec![op.ins[0].clone()] });
                        kept_flow = true;
                    }
                    NodeExit::Unresolved(_) | NodeExit::Halt(_) => {
                        out.push(Self::raw(op));
                        kept_flow = true;
                        self.stats.kept_unresolved += 1;
                    }
                    NodeExit::CondBranch { arms, .. } if op.opc == CPUI_CBRANCH => {
                        match arms.get(arm_index) {
                            Some((_, taken)) => {
                                out.push(OverlayOp {
                                    opc: CPUI_CBRANCH,
                                    out: None,
                                    ins: vec![self.code(syn_addr(self.conf, *taken)), op.ins[1].clone()],
                                });
                                kept_flow = true;
                            }
                            None => out.push(self.nop_op()),
                        }
                        arm_index += 1;
                    }
                    _ => out.push(self.nop_op()),
                }
                continue;
            }
            // A stack access addresses its slot as `sp + k` directly, instead
            // of through the interpreter's register chains.
            if linear && matches!(op.opc, CPUI_LOAD | CPUI_STORE) {
                if let Some(a) = self.stack_address(op, i, trace) {
                    out.push(a);
                }
            }
            let mut rewritten = self.rewrite_data(op, i, trace);
            if let Some(scratch) = self.last_addr_scratch.take() {
                // A folded load no longer has an address operand.
                if rewritten.opc == op.opc && rewritten.ins.len() > 1 {
                    rewritten.ins[1] = scratch;
                }
            }
            out.push(rewritten);
            // Witness a deciding boolean the moment an op computes it.
            if let (Some(trace), Some(out_vn)) = (trace, &op.out) {
                if !trace.forked && out_vn.size == 1 {
                    if let Some(Some(v)) = trace.op_out.get(i).copied() {
                        let direct = self.slots.get(&v).copied().map(|s| (s, false));
                        let negated = self.negated.get(&v).and_then(|a| self.slots.get(a).copied()).map(|s| (s, true));
                        if let Some((slot, neg)) = direct.or(negated) {
                            let sv = self.slot_vn(slot);
                            if neg {
                                out.push(OverlayOp { opc: CPUI_BOOL_NEGATE, out: Some(sv), ins: vec![out_vn.clone()] });
                            } else {
                                out.push(OverlayOp { opc: CPUI_COPY, out: Some(sv), ins: vec![out_vn.clone()] });
                            }
                            let atom = if neg { self.negated[&v] } else { v };
                            self.witnessed.insert(atom);
                        }
                    }
                }
            }
        }
        let _ = (is_last, kept_flow);
        out
    }

    fn append_exit(&mut self, node: &super::kuna_pushanflow::Node, insn: &LiftedInsn, out: &mut Vec<OverlayOp>, kept_flow: bool) {
        match &node.exit {
            NodeExit::Branch(s) => out.push(self.branch_op(syn_addr(self.conf, *s))),
            NodeExit::CondBranch { fall, .. } => out.push(self.branch_op(syn_addr(self.conf, *fall))),
            NodeExit::Select { targets, atoms, by_atoms } => {
                // Branch on the witnessed booleans when every deciding atom
                // was computed by some emitted op; otherwise compare the
                // computed target against each destination's native address.
                let total: usize = targets.iter().map(|(a, _)| a.len()).sum();
                if *by_atoms && total <= 16 && atoms.iter().all(|a| self.witnessed.contains(a)) {
                    let n = targets.len();
                    let mut scratch = 2u64;
                    for (i, (assigns, succ)) in targets.iter().enumerate() {
                        if i + 1 == n {
                            out.push(self.branch_op(syn_addr(self.conf, *succ)));
                            break;
                        }
                        for assign in assigns {
                            let mut acc: Option<VarnodeData> = None;
                            for (atom, v) in assign {
                                let sv = self.slot_vn(self.slots[atom]);
                                let lit = if *v {
                                    sv
                                } else {
                                    let t = self.scratch(scratch, 1);
                                    scratch += 1;
                                    out.push(OverlayOp { opc: CPUI_BOOL_NEGATE, out: Some(t.clone()), ins: vec![sv] });
                                    t
                                };
                                acc = Some(match acc {
                                    None => lit,
                                    Some(prev) => {
                                        let t = self.scratch(scratch, 1);
                                        scratch += 1;
                                        out.push(OverlayOp { opc: CPUI_BOOL_AND, out: Some(t.clone()), ins: vec![prev, lit] });
                                        t
                                    }
                                });
                            }
                            if let Some(c) = acc {
                                out.push(OverlayOp { opc: CPUI_CBRANCH, out: None, ins: vec![self.code(syn_addr(self.conf, *succ)), c] });
                            }
                        }
                    }
                    return;
                }
                let tv = insn.ops.iter().rev().find(|op| Self::is_flow(op)).and_then(|op| match op.opc {
                    CPUI_BRANCHIND | CPUI_RETURN | CPUI_CALLIND => op.ins.first().cloned(),
                    _ => None,
                });
                let Some(tv) = tv else {
                    out.push(self.branch_op(syn_addr(self.conf, targets[0].1)));
                    return;
                };
                let n = targets.len();
                for (i, (_, succ)) in targets.iter().enumerate() {
                    if i + 1 == n {
                        out.push(self.branch_op(syn_addr(self.conf, *succ)));
                        break;
                    }
                    let native = self.cfg.nodes[*succ as usize].key.addr;
                    let c = self.scratch(2 + i as u64, 1);
                    out.push(OverlayOp {
                        opc: CPUI_INT_EQUAL,
                        out: Some(c.clone()),
                        ins: vec![tv.clone(), self.cst(native, tv.size)],
                    });
                    out.push(OverlayOp {
                        opc: CPUI_CBRANCH,
                        out: None,
                        ins: vec![self.code(syn_addr(self.conf, *succ)), c],
                    });
                }
            }
            NodeExit::Call { target, ret } => {
                if !kept_flow {
                    match target {
                        Some(t) => out.push(OverlayOp { opc: CPUI_CALL, out: None, ins: vec![self.code(*t)] }),
                        None => {}
                    }
                }
                out.push(self.branch_op(syn_addr(self.conf, *ret)));
            }
            NodeExit::Return => {
                if !kept_flow {
                    self.dead_end(out);
                }
            }
            NodeExit::TailCall(c) => {
                let ptr = self.conf.ptr_size as u32;
                out.push(OverlayOp { opc: CPUI_CALL, out: None, ins: vec![self.code(*c)] });
                let ra = self.scratch(1, ptr);
                let sp_vn = self.sp_varnode();
                out.push(OverlayOp {
                    opc: CPUI_LOAD,
                    out: Some(ra.clone()),
                    ins: vec![self.cst(self.conf.ram.get_index() as u64, ptr), sp_vn],
                });
                out.push(OverlayOp { opc: CPUI_RETURN, out: None, ins: vec![ra] });
            }
            NodeExit::Unresolved(_) | NodeExit::Halt(_) => {
                if !kept_flow {
                    self.dead_end(out);
                }
            }
        }
    }

    fn sp_varnode(&self) -> VarnodeData {
        self.vn(&self.conf.register, self.conf.sp_reg, self.conf.ptr_size as u32)
    }

    fn dead_end(&self, out: &mut Vec<OverlayOp>) {
        let ptr = self.conf.ptr_size as u32;
        let out_vn = self.scratch(1, ptr);
        out.push(OverlayOp { opc: CPUI_COPY, out: Some(out_vn.clone()), ins: vec![self.cst(0, ptr)] });
        out.push(OverlayOp { opc: CPUI_RETURN, out: None, ins: vec![out_vn] });
    }

    /// `scratch = sp + k` for a memory access whose address is stack-relative.
    fn stack_address(&mut self, op: &RawOp, i: usize, trace: Option<&super::kuna_pushanemu::InsnTrace>) -> Option<OverlayOp> {
        if !self.conf.remat_sp {
            return None;
        }
        let trace = trace?;
        let acc = trace.accesses.iter().find(|a| a.op_index == i)?;
        let (Some(base), k) = self.arena.affine(acc.addr_expr)? else { return None };
        if Some(base) != self.conf.sp0 {
            return None;
        }
        let spb = (*trace.sp_before.get(i)?)?;
        let (Some(b2), k1) = self.arena.affine(spb)? else { return None };
        if b2 != base {
            return None;
        }
        let ptr = self.conf.ptr_size as u32;
        if op.ins[1].size != ptr {
            return None;
        }
        let delta = k.wrapping_sub(k1);
        let scratch = self.scratch(32 + (i as u64 % 8), ptr);
        let sp_vn = self.sp_varnode();
        self.last_addr_scratch = Some(scratch.clone());
        self.stats.remat += 1;
        if delta == 0 {
            return Some(OverlayOp { opc: CPUI_COPY, out: Some(scratch), ins: vec![sp_vn] });
        }
        let mask = if ptr >= 8 { u64::MAX as u128 } else { (1u128 << (8 * ptr)) - 1 };
        Some(OverlayOp { opc: CPUI_INT_ADD, out: Some(scratch), ins: vec![sp_vn, self.cst((delta as u128 & mask) as u64, ptr)] })
    }

    /// Fold a data op to its emulated constant or stack-relative form.
    fn rewrite_data(&mut self, op: &RawOp, i: usize, trace: Option<&super::kuna_pushanemu::InsnTrace>) -> OverlayOp {
        let Some(trace) = trace else { return Self::raw(op) };
        if trace.forked {
            return Self::raw(op);
        }
        let Some(out_vn) = &op.out else { return Self::raw(op) };
        let space = out_vn.space.as_ref().map(classify_space);
        if !matches!(space, Some(Space::Register) | Some(Space::Unique)) || out_vn.size > 16 {
            return Self::raw(op);
        }
        let Some(Some(v)) = trace.op_out.get(i).copied() else { return Self::raw(op) };
        if self.conf.fold_constants {
            if let Some(c) = self.arena.is_const(v) {
                if op.opc == CPUI_COPY && op.ins[0].space.as_ref().map(classify_space) == Some(Space::Const) {
                    return Self::raw(op);
                }
                if op.opc == CPUI_LOAD {
                    // A load from writable memory keeps its load: the image value is not a fact.
                    if let Some(acc) = trace.accesses.iter().find(|a| a.op_index == i) {
                        if let Some(a) = acc.addr {
                            if self.target.is_writable(a) {
                                return Self::raw(op);
                            }
                        }
                    }
                }
                if out_vn.size <= 8 {
                    self.stats.folded += 1;
                    return OverlayOp { opc: CPUI_COPY, out: Some(out_vn.clone()), ins: vec![self.cst(c as u64, out_vn.size)] };
                }
                return Self::raw(op);
            }
        }
        if self.conf.remat_sp && out_vn.size == self.conf.ptr_size as u32 {
            if let (Some(sp0), Some((Some(base), k))) = (self.conf.sp0, self.arena.affine(v)) {
                if base == sp0 {
                    if let Some(Some(spb)) = trace.sp_before.get(i).copied() {
                        if let Some((Some(b2), k1)) = self.arena.affine(spb) {
                            if b2 == sp0 {
                                let delta = k.wrapping_sub(k1);
                                let already = matches!(op.opc, CPUI_INT_ADD | CPUI_INT_SUB | CPUI_COPY)
                                    && op.ins[0].space.as_ref().map(classify_space) == Some(Space::Register)
                                    && op.ins[0].offset == self.conf.sp_reg
                                    && op.ins.get(1).map_or(true, |b| b.space.as_ref().map(classify_space) == Some(Space::Const));
                                if already {
                                    return Self::raw(op);
                                }
                                self.stats.remat += 1;
                                let sp_vn = self.sp_varnode();
                                if delta == 0 {
                                    return OverlayOp { opc: CPUI_COPY, out: Some(out_vn.clone()), ins: vec![sp_vn] };
                                }
                                let ptr = self.conf.ptr_size as u32;
                                let mask = if ptr >= 8 { u64::MAX as u128 } else { (1u128 << (8 * ptr)) - 1 };
                                return OverlayOp {
                                    opc: CPUI_INT_ADD,
                                    out: Some(out_vn.clone()),
                                    ins: vec![sp_vn, self.cst((delta as u128 & mask) as u64, ptr)],
                                };
                            }
                        }
                    }
                }
            }
        }
        Self::raw(op)
    }
}
