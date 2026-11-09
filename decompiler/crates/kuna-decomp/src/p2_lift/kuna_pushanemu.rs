//! (kuna pushan) Constraint-free symbolic emulation of lifted p-code.
//!
//! One instruction at a time, the emulator runs the SLEIGH p-code of a native
//! instruction over a [`State`], computing every value as an expression in the
//! [`ExprArena`].  There is no path condition and no solver: a conditional
//! whose guard is a constant takes one side; a guard that stays symbolic is
//! reported as a two-way exit for the exploration to follow both ways; a load
//! or store whose address selects between a few concrete locations becomes a
//! select over those locations (the paper's two-valued VIP/key handling).  The
//! emulator records which register each load dereferenced so the exploration
//! can identify the virtual program counter.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use OpCode::*;

use super::kuna_pushanexpr::{ExprArena, ExprId, MAX_SIZE};
use super::kuna_pushanstate::{PushanImage, State};

/// A lifted p-code operation (the SLEIGH template instantiated at an address).
#[derive(Clone, Debug)]
pub struct RawOp {
    pub opc: OpCode,
    pub out: Option<VarnodeData>,
    pub ins: Vec<VarnodeData>,
}

/// One native instruction's lift.
#[derive(Clone, Debug)]
pub struct LiftedInsn {
    pub addr: u64,
    pub len: u32,
    pub ops: Vec<RawOp>,
    pub asm: String,
}

/// What the emulator needs from the binary and the translator.
pub trait PushanTarget: PushanImage {
    fn lift(&self, addr: u64) -> Result<Rc<LiftedInsn>, String>;
    fn ptr_size(&self) -> u8;
    fn space_by_index(&self, index: i32) -> Option<Rc<AddrSpace>>;
    /// A code section other than the one the analysed function starts in.
    fn is_foreign_code(&self, addr: u64) -> bool;
    fn is_executable(&self, addr: u64) -> bool;
    fn is_mapped(&self, addr: u64) -> bool;
    /// The name of the user-defined op (CALLOTHER) with this index.
    fn userop_name(&self, index: u64) -> Option<String>;
}

/// Storage class of a varnode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Space {
    Const,
    Register,
    Unique,
    Ram,
    Other,
}

pub fn classify_space(sp: &Rc<AddrSpace>) -> Space {
    match sp.get_type() {
        spacetype::IPTR_CONSTANT => Space::Const,
        spacetype::IPTR_INTERNAL => Space::Unique,
        spacetype::IPTR_PROCESSOR => {
            if sp.get_name() == "register" {
                Space::Register
            } else {
                Space::Ram
            }
        }
        _ => Space::Other,
    }
}

/// A memory access the instruction performed.
#[derive(Clone, Debug)]
pub struct AccessRecord {
    /// The register (offset) and constant displacement the address was built from.
    pub via: Option<(u64, i128)>,
    /// The concrete address, when known.
    pub addr: Option<u64>,
    pub size: u8,
    pub is_store: bool,
}

/// How control leaves an instruction.
#[derive(Clone, Debug)]
pub enum Exit {
    Fallthrough(u64),
    Jump(ExprId),
    /// Symbolic guard: `taken_state` is the state on the taken side; the
    /// caller's state is the fall-through side.
    CondJump { cond: ExprId, target: ExprId, taken_state: Box<State>, fallthrough: u64 },
    Call { target: ExprId, fallthrough: u64 },
    Return(ExprId),
    Halt(String),
}

#[derive(Clone, Debug, Default)]
pub struct InsnTrace {
    pub accesses: Vec<AccessRecord>,
    pub userops: Vec<String>,
}

const MAX_SPLIT_CASES: usize = 4;
const MAX_FORK_DEPTH: u32 = 4;

pub struct Emulator<'a> {
    pub target: &'a dyn PushanTarget,
    /// Per-instruction provenance: unique offset -> (register offset, displacement).
    prov: BTreeMap<u64, (u64, i128)>,
}

impl<'a> Emulator<'a> {
    pub fn new(target: &'a dyn PushanTarget) -> Self {
        Emulator { target, prov: BTreeMap::new() }
    }

    fn vn_space(&self, vn: &VarnodeData) -> Space {
        vn.space.as_ref().map(classify_space).unwrap_or(Space::Other)
    }

    fn read_vn(&mut self, state: &mut State, arena: &mut ExprArena, vn: &VarnodeData) -> ExprId {
        let size = vn.size.min(MAX_SIZE as u32) as u8;
        if vn.size > MAX_SIZE as u32 {
            return arena.unknown(MAX_SIZE);
        }
        match self.vn_space(vn) {
            Space::Const => arena.cst(size, vn.offset as u128),
            Space::Register => state.read_reg(arena, vn.offset, size),
            Space::Unique => state.read_unique(arena, vn.offset, size),
            Space::Ram => state.read_mem(arena, self.target, None, vn.offset as i128, size),
            Space::Other => arena.unknown(size),
        }
    }

    fn write_vn(&mut self, state: &mut State, arena: &mut ExprArena, vn: &VarnodeData, e: ExprId) {
        if vn.size > MAX_SIZE as u32 {
            // Wide (vector) writes: unknown chunks.
            let mut off = 0u64;
            while off < vn.size as u64 {
                let chunk = ((vn.size as u64 - off).min(MAX_SIZE as u64)) as u8;
                let u = arena.unknown(chunk);
                match self.vn_space(vn) {
                    Space::Register => state.write_reg(vn.offset + off, u, chunk),
                    Space::Unique => state.write_unique(vn.offset + off, u, chunk),
                    Space::Ram => state.write_mem(None, (vn.offset + off) as i128, u, chunk),
                    _ => {}
                }
                off += chunk as u64;
            }
            return;
        }
        let size = vn.size as u8;
        let e = arena.resize(e, size);
        match self.vn_space(vn) {
            Space::Register => state.write_reg(vn.offset, e, size),
            Space::Unique => state.write_unique(vn.offset, e, size),
            Space::Ram => state.write_mem(None, vn.offset as i128, e, size),
            _ => {}
        }
    }

    /// Track `unique = register (+ constant)` so a later load reports the register it dereferenced.
    fn note_provenance(&mut self, op: &RawOp) {
        let Some(out) = &op.out else { return };
        if self.vn_space(out) != Space::Unique {
            return;
        }
        let src = |this: &Self, vn: &VarnodeData| -> Option<(u64, i128)> {
            match this.vn_space(vn) {
                Space::Register => Some((vn.offset, 0)),
                Space::Unique => this.prov.get(&vn.offset).copied(),
                _ => None,
            }
        };
        let cst = |this: &Self, vn: &VarnodeData| -> Option<i128> {
            (this.vn_space(vn) == Space::Const).then(|| {
                let v = vn.offset as u128;
                let b = 8 * vn.size.min(16);
                if b >= 128 { v as i128 } else { ((v << (128 - b)) as i128) >> (128 - b) }
            })
        };
        let r = match (op.opc, op.ins.len()) {
            (CPUI_COPY, 1) | (CPUI_INT_ZEXT, 1) | (CPUI_INT_SEXT, 1) => src(self, &op.ins[0]),
            (CPUI_INT_ADD, 2) => match (src(self, &op.ins[0]), cst(self, &op.ins[1])) {
                (Some((r, d)), Some(c)) => Some((r, d + c)),
                _ => match (cst(self, &op.ins[0]), src(self, &op.ins[1])) {
                    (Some(c), Some((r, d))) => Some((r, d + c)),
                    _ => None,
                },
            },
            (CPUI_INT_SUB, 2) => match (src(self, &op.ins[0]), cst(self, &op.ins[1])) {
                (Some((r, d)), Some(c)) => Some((r, d - c)),
                _ => None,
            },
            _ => None,
        };
        match r {
            Some(p) => {
                self.prov.insert(out.offset, p);
            }
            None => {
                self.prov.remove(&out.offset);
            }
        }
    }

    fn address_provenance(&self, vn: &VarnodeData) -> Option<(u64, i128)> {
        match self.vn_space(vn) {
            Space::Register => Some((vn.offset, 0)),
            Space::Unique => self.prov.get(&vn.offset).copied(),
            _ => None,
        }
    }

    fn as_bool(&self, arena: &mut ExprArena, c: ExprId) -> ExprId {
        if arena.is_bool(c) {
            return c;
        }
        let z = arena.cst(arena.size(c), 0);
        arena.mk_bin(CPUI_INT_NOTEQUAL, c, z, 1)
    }

    /// Load `size` bytes at a symbolic address in `space`.
    pub fn load(&mut self, state: &mut State, arena: &mut ExprArena, space: Space, addr: ExprId, size: u8) -> ExprId {
        match space {
            Space::Register => {
                if let Some(c) = arena.is_const(addr) {
                    return state.read_reg(arena, c as u64, size);
                }
                state.unknown_loads += 1;
                return arena.unknown(size);
            }
            Space::Ram => {}
            _ => {
                state.unknown_loads += 1;
                return arena.unknown(size);
            }
        }
        if let Some((base, off)) = arena.affine(addr) {
            return state.read_mem(arena, self.target, base, off, size);
        }
        if let Some(cases) = arena.case_split(addr) {
            if cases.len() <= MAX_SPLIT_CASES && cases.iter().all(|(_, a)| arena.affine(*a).is_some()) {
                let mut vals = Vec::with_capacity(cases.len());
                for (_, a) in &cases {
                    let (base, off) = arena.affine(*a).unwrap();
                    vals.push(state.read_mem(arena, self.target, base, off, size));
                }
                let atoms: Vec<ExprId> = {
                    let (assign, _) = &cases[0];
                    assign.iter().map(|(a, _)| *a).collect()
                };
                return ite_tree(arena, &atoms, &vals, 0, 0);
            }
        }
        state.unknown_loads += 1;
        arena.unknown(size)
    }

    /// Store `val` at a symbolic address in `space`.
    pub fn store(&mut self, state: &mut State, arena: &mut ExprArena, space: Space, addr: ExprId, val: ExprId) {
        let size = arena.size(val);
        match space {
            Space::Register => {
                if let Some(c) = arena.is_const(addr) {
                    state.write_reg(c as u64, val, size);
                } else {
                    state.unknown_stores += 1;
                }
                return;
            }
            Space::Ram => {}
            _ => {
                state.unknown_stores += 1;
                return;
            }
        }
        if let Some((base, off)) = arena.affine(addr) {
            state.write_mem(base, off, val, size);
            return;
        }
        if let Some(cases) = arena.case_split(addr) {
            if cases.len() <= MAX_SPLIT_CASES && cases.iter().all(|(_, a)| arena.affine(*a).is_some()) {
                for (assign, a) in &cases {
                    let (base, off) = arena.affine(*a).unwrap();
                    let old = state.read_mem(arena, self.target, base, off, size);
                    let cond = assignment_cond(arena, assign);
                    let new = arena.mk_ite(cond, val, old);
                    state.write_mem(base, off, new, size);
                }
                return;
            }
        }
        state.unknown_stores += 1;
    }

    /// Execute one instruction; the state is left as the fall-through side.
    pub fn exec_insn(&mut self, state: &mut State, arena: &mut ExprArena, insn: &LiftedInsn, trace: &mut InsnTrace) -> Exit {
        state.clear_uniques();
        self.prov.clear();
        let mut visited = vec![false; insn.ops.len() + 1];
        self.run_from(state, arena, insn, 0, trace, &mut visited, 0)
    }

    #[allow(clippy::too_many_arguments)]
    fn run_from(
        &mut self,
        state: &mut State,
        arena: &mut ExprArena,
        insn: &LiftedInsn,
        mut pc: usize,
        trace: &mut InsnTrace,
        visited: &mut Vec<bool>,
        depth: u32,
    ) -> Exit {
        let next = insn.addr.wrapping_add(insn.len as u64);
        let mut steps = 0usize;
        while pc < insn.ops.len() {
            steps += 1;
            if steps > 4 * insn.ops.len() + 16 {
                return Exit::Halt(format!("p-code loop inside instruction at {:#x}", insn.addr));
            }
            let op = &insn.ops[pc];
            match op.opc {
                CPUI_BRANCH => {
                    let t = &op.ins[0];
                    if self.vn_space(t) == Space::Const {
                        pc = (pc as i64 + t.offset as i64) as usize;
                        continue;
                    }
                    return Exit::Jump(arena.cst(self.target.ptr_size(), t.offset as u128));
                }
                CPUI_CBRANCH => {
                    let t = op.ins[0].clone();
                    let raw = self.read_vn(state, arena, &op.ins[1]);
                    let c = self.as_bool(arena, raw);
                    let relative = self.vn_space(&t) == Space::Const;
                    if let Some(v) = arena.is_const(c) {
                        if v & 1 == 1 {
                            if relative {
                                pc = (pc as i64 + t.offset as i64) as usize;
                                continue;
                            }
                            return Exit::Jump(arena.cst(self.target.ptr_size(), t.offset as u128));
                        }
                        pc += 1;
                        continue;
                    }
                    if !relative {
                        let taken = Box::new(state.clone());
                        let target = arena.cst(self.target.ptr_size(), t.offset as u128);
                        // Ops after an external conditional branch run on the fall-through side.
                        let rest = self.run_from(state, arena, insn, pc + 1, trace, visited, depth);
                        return match rest {
                            Exit::Fallthrough(_) => Exit::CondJump { cond: c, target, taken_state: taken, fallthrough: next },
                            other => other,
                        };
                    }
                    if depth >= MAX_FORK_DEPTH {
                        return Exit::Halt(format!("deep p-code fork inside instruction at {:#x}", insn.addr));
                    }
                    let mut taken_state = state.clone();
                    let saved_prov = self.prov.clone();
                    let tpc = (pc as i64 + t.offset as i64) as usize;
                    let r_taken = self.run_from(&mut taken_state, arena, insn, tpc, trace, visited, depth + 1);
                    self.prov = saved_prov;
                    let r_fall = self.run_from(state, arena, insn, pc + 1, trace, visited, depth + 1);
                    return self.join_exits(state, taken_state, arena, c, r_taken, r_fall, next);
                }
                CPUI_BRANCHIND => {
                    let t = self.read_vn(state, arena, &op.ins[0]);
                    return Exit::Jump(t);
                }
                CPUI_CALL => {
                    let t = &op.ins[0];
                    let target = arena.cst(self.target.ptr_size(), t.offset as u128);
                    return Exit::Call { target, fallthrough: next };
                }
                CPUI_CALLIND => {
                    let target = self.read_vn(state, arena, &op.ins[0]);
                    return Exit::Call { target, fallthrough: next };
                }
                CPUI_RETURN => {
                    let t = self.read_vn(state, arena, &op.ins[0]);
                    return Exit::Return(t);
                }
                CPUI_CALLOTHER => {
                    let idx = op.ins[0].offset;
                    let name = self.target.userop_name(idx).unwrap_or_else(|| format!("userop_{idx}"));
                    trace.userops.push(name);
                    if let Some(out) = &op.out {
                        let u = arena.unknown(out.size.min(MAX_SIZE as u32) as u8);
                        self.write_vn(state, arena, out, u);
                    }
                }
                CPUI_LOAD => {
                    let space = self
                        .target
                        .space_by_index(op.ins[0].offset as i32)
                        .map(|s| classify_space(&s))
                        .unwrap_or(Space::Other);
                    let addr = self.read_vn(state, arena, &op.ins[1]);
                    let out = op.out.clone().expect("LOAD without output");
                    let size = out.size.min(MAX_SIZE as u32) as u8;
                    trace.accesses.push(AccessRecord {
                        via: self.address_provenance(&op.ins[1]),
                        addr: arena.is_const(addr).map(|c| c as u64),
                        size,
                        is_store: false,
                    });
                    let v = if out.size > MAX_SIZE as u32 { arena.unknown(size) } else { self.load(state, arena, space, addr, size) };
                    self.write_vn(state, arena, &out, v);
                }
                CPUI_STORE => {
                    let space = self
                        .target
                        .space_by_index(op.ins[0].offset as i32)
                        .map(|s| classify_space(&s))
                        .unwrap_or(Space::Other);
                    let addr = self.read_vn(state, arena, &op.ins[1]);
                    let val = self.read_vn(state, arena, &op.ins[2]);
                    trace.accesses.push(AccessRecord {
                        via: self.address_provenance(&op.ins[1]),
                        addr: arena.is_const(addr).map(|c| c as u64),
                        size: op.ins[2].size.min(MAX_SIZE as u32) as u8,
                        is_store: true,
                    });
                    if op.ins[2].size <= MAX_SIZE as u32 {
                        self.store(state, arena, space, addr, val);
                    } else {
                        state.unknown_stores += 1;
                    }
                }
                _ => {
                    self.note_provenance(op);
                    let Some(out) = &op.out else {
                        pc += 1;
                        continue;
                    };
                    if out.size > MAX_SIZE as u32 || op.ins.iter().any(|v| v.size > MAX_SIZE as u32) {
                        let u = arena.unknown(out.size.min(MAX_SIZE as u32) as u8);
                        self.write_vn(state, arena, out, u);
                        pc += 1;
                        continue;
                    }
                    let mut ins = Vec::with_capacity(op.ins.len());
                    for vn in &op.ins {
                        ins.push(self.read_vn(state, arena, vn));
                    }
                    let v = match arena.mk_op(op.opc, out.size as u8, &ins) {
                        Some(v) => v,
                        None => arena.unknown(out.size as u8),
                    };
                    self.write_vn(state, arena, out, v);
                }
            }
            pc += 1;
        }
        Exit::Fallthrough(next)
    }

    /// Combine the two sides of a symbolic intra-instruction fork.
    #[allow(clippy::too_many_arguments)]
    fn join_exits(
        &mut self,
        state: &mut State,
        mut taken: State,
        arena: &mut ExprArena,
        c: ExprId,
        r_taken: Exit,
        r_fall: Exit,
        next: u64,
    ) -> Exit {
        match (r_taken, r_fall) {
            (Exit::Fallthrough(_), Exit::Fallthrough(_)) => {
                merge_ite(state, &mut taken, arena, self.target, c);
                Exit::Fallthrough(next)
            }
            (Exit::Jump(t), Exit::Fallthrough(_)) => {
                Exit::CondJump { cond: c, target: t, taken_state: Box::new(taken), fallthrough: next }
            }
            (Exit::Fallthrough(_), Exit::Jump(t)) => {
                let one = arena.cst(1, 1);
                let nc = arena.mk_bin(CPUI_INT_XOR, c, one, 1);
                let fall_state = std::mem::replace(state, taken);
                Exit::CondJump { cond: nc, target: t, taken_state: Box::new(fall_state), fallthrough: next }
            }
            (Exit::Jump(t1), Exit::Jump(t2)) => {
                merge_ite(state, &mut taken, arena, self.target, c);
                Exit::Jump(arena.mk_ite(c, t1, t2))
            }
            (Exit::Halt(m), _) | (_, Exit::Halt(m)) => Exit::Halt(m),
            (a, b) => Exit::Halt(format!("unsupported intra-instruction fork: {a:?} / {b:?}")),
        }
    }
}

/// `Ite` tree over `atoms` selecting `vals[index]` where bit `i` of the index
/// is atom `i`'s value.
fn ite_tree(arena: &mut ExprArena, atoms: &[ExprId], vals: &[ExprId], i: usize, prefix: usize) -> ExprId {
    if i == atoms.len() {
        return vals[prefix];
    }
    let f = ite_tree(arena, atoms, vals, i + 1, prefix);
    let t = ite_tree(arena, atoms, vals, i + 1, prefix | (1 << i));
    arena.mk_ite(atoms[i], t, f)
}

/// The conjunction of the literals of a case-split assignment.
pub fn assignment_cond(arena: &mut ExprArena, assign: &[(ExprId, bool)]) -> ExprId {
    let mut acc = arena.cst(1, 1);
    for (a, v) in assign {
        let lit = if *v {
            *a
        } else {
            let one = arena.cst(1, 1);
            arena.mk_bin(CPUI_INT_XOR, *a, one, 1)
        };
        acc = arena.mk_bin(CPUI_INT_AND, acc, lit, 1);
    }
    acc
}

/// `state := c ? taken : state`, location by location.
pub fn merge_ite(state: &mut State, taken: &mut State, arena: &mut ExprArena, image: &dyn PushanImage, c: ExprId) {
    for (o, len) in state.regs.diff_ranges(&taken.regs) {
        let a = taken.read_reg(arena, o as u64, len);
        let b = state.read_reg(arena, o as u64, len);
        let v = arena.mk_ite(c, a, b);
        state.write_reg(o as u64, v, len);
    }
    for (o, len) in state.uniq.diff_ranges(&taken.uniq) {
        let a = taken.read_unique(arena, o as u64, len);
        let b = state.read_unique(arena, o as u64, len);
        let v = arena.mk_ite(c, a, b);
        state.write_unique(o as u64, v, len);
    }
    let bases: Vec<_> = state
        .mem
        .keys()
        .chain(taken.mem.keys())
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let empty = super::kuna_pushanstate::ByteStore::default();
    for base in bases {
        let ranges = state.mem.get(&base).unwrap_or(&empty).diff_ranges(taken.mem.get(&base).unwrap_or(&empty));
        for (o, len) in ranges {
            let a = taken.read_mem(arena, image, base, o, len);
            let b = state.read_mem(arena, image, base, o, len);
            let v = arena.mk_ite(c, a, b);
            state.write_mem(base, o, v, len);
        }
    }
    state.unknown_loads = state.unknown_loads.max(taken.unknown_loads);
    state.unknown_stores = state.unknown_stores.max(taken.unknown_stores);
}
