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

use super::kuna_symexpr::{VarId, ExprArena, ExprId, ExprKind, MAX_SIZE};
use super::kuna_symstate::{SymImage, State};

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
pub trait SymTarget: SymImage {
    fn lift(&self, addr: u64) -> Result<Rc<LiftedInsn>, String>;
    fn ptr_size(&self) -> u8;
    fn space_by_index(&self, index: i32) -> Option<Rc<AddrSpace>>;
    /// A code section other than the one the analysed function starts in.
    fn is_foreign_code(&self, addr: u64) -> bool;
    fn is_executable(&self, addr: u64) -> bool;
    fn is_mapped(&self, addr: u64) -> bool;
    /// The name of the user-defined op (CALLOTHER) with this index.
    fn userop_name(&self, index: u64) -> Option<String>;
    /// A known function starts here (symbol table, exports, unwind data).
    fn is_function_entry(&self, addr: u64) -> bool;
    /// The image's default convention leaves argument cleanup to each callee
    /// (`__stdcall` on 32-bit Windows).
    fn callee_cleans(&self) -> bool;
    /// Bytes the callee declared at `addr` (a function or an import slot)
    /// pops beyond its return address, from its prototype and convention.
    fn declared_pop(&self, addr: u64) -> Option<u32>;
    /// `addr` lies in a mapped region that looks like VM bytecode: the paper's
    /// test, a 256-byte window with Shannon entropy above 5 bits per byte.
    fn bytecode_like(&self, addr: u64) -> bool;
    /// The same test over the 256 bytes around `addr` (128 on each side,
    /// shifted to stay inside its section), as angr's VIP finder reads the
    /// window: a fetch near the end of a bytecode region still passes.
    fn bytecode_near(&self, addr: u64) -> bool {
        self.bytecode_like(addr)
    }
    /// `addr` lies in data the file holds: a loaded section that is neither
    /// code nor uninitialized (where an in-function interpreter's bytecode lives).
    fn file_data(&self, addr: u64) -> bool {
        self.is_mapped(addr) && !self.is_executable(addr)
    }
    /// The code at `addr` is a C library routine with `memcpy`'s contract
    /// (destination, source, byte count; only the destination is written).
    fn copy_routine(&self, _addr: u64) -> bool {
        false
    }
    /// The function at `addr` is known never to return (`__stack_chk_fail`).
    fn noreturn_function(&self, _addr: u64) -> bool {
        false
    }
    /// A software interrupt with this vector never returns (Windows `int 0x29`, `__fastfail`).
    fn noreturn_interrupt(&self, vector: u64) -> bool;
    /// The top bits of the stack pointer the platform keeps clear for user
    /// code (a 32-bit Windows process without the large-address flag lives
    /// below 2 GiB; a 64-bit one below 128 TiB), as a mask of those bits.
    fn user_stack_clear_bits(&self) -> u128 {
        0
    }
    /// Instructions lifted so far (the entries of the target's lift cache).
    fn lift_count(&self) -> usize {
        0
    }
}

/// Shannon entropy (bits per byte) of a window.
pub fn entropy(buf: &[u8]) -> f64 {
    let mut counts = [0u32; 256];
    for b in buf {
        counts[*b as usize] += 1;
    }
    let n = buf.len() as f64;
    let mut h = 0.0;
    for c in counts {
        if c > 0 {
            let p = c as f64 / n;
            h -= p * p.log2();
        }
    }
    h
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
    /// Index of the LOAD/STORE op within the instruction.
    pub op_index: usize,
    /// The address as the emulator saw it.
    pub addr_expr: ExprId,
    /// The register (offset) and constant displacement the address was built from.
    pub via: Option<(u64, i128)>,
    /// The concrete address, when known.
    pub addr: Option<u64>,
    pub size: u8,
    pub is_store: bool,
    /// A load through the virtual PC that read the file's bytes.
    pub trusted: bool,
    /// A store the emulation placed nowhere while memory its address
    /// depends on kept its contents ([`Emulator::reaches_kept`]): what it
    /// wrote there is lost to the exploration.
    pub lost: bool,
}

/// Where an in-function interpreter keeps its virtual PC.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VpcSource {
    Reg(u64),
    Cell(Option<VarId>, i128),
}

/// Bytes on either side of the virtual PC a fetch through it may read.
pub const FETCH_WINDOW: i128 = 16;

/// How control leaves an instruction.
#[derive(Clone, Debug)]
pub enum Exit {
    Fallthrough(u64),
    Jump(ExprId),
    /// Symbolic guards, in program order: arm `k` is taken when its guard
    /// holds and no earlier guard did; the caller's state is the fall-through
    /// side after every guard failed, which continues at `fallthrough` (the
    /// next instruction, or the constant target the instruction jumps to
    /// after its guards, as a `rep` prefix loops back to itself).
    CondJump { arms: Vec<(ExprId, ExprId, Box<State>)>, fallthrough: u64 },
    Call { target: ExprId, fallthrough: u64 },
    Return(ExprId),
    /// A call that never returns (a no-return interrupt): the path ends.
    NoReturn,
    Halt(String),
}

#[derive(Clone, Debug, Default)]
pub struct InsnTrace {
    pub accesses: Vec<AccessRecord>,
    pub userops: Vec<String>,
    /// Diagnostics: why a value became an unknown symbol.
    pub notes: Vec<String>,
    /// The value each op wrote (by op index), along the path taken.
    pub op_out: Vec<Option<ExprId>>,
    /// The stack pointer before each op, when the emulator tracks one.
    pub sp_before: Vec<Option<ExprId>>,
    /// The instruction forked on a symbolic internal branch; per-op values are
    /// not path-unique.
    pub forked: bool,
    /// The op indices executed, in order (internal loops unrolled).  Empty
    /// when the instruction forked.
    pub path: Vec<usize>,
    /// The instruction was a `rep movs` the exploration modelled as a copy
    /// (`Explorer::rep_copy`) instead of running it.
    pub elided: bool,
}

const MAX_SPLIT_CASES: usize = 4;
/// Assignments a split address may have before the load gives up.
const MAX_SPLIT_ASSIGNMENTS: usize = 64;
const MAX_FORK_DEPTH: u32 = 4;

pub struct Emulator<'a> {
    pub target: &'a dyn SymTarget,
    /// Per-instruction provenance: unique offset -> (register offset, displacement).
    prov: BTreeMap<u64, (u64, i128)>,
    /// Stack pointer register (offset, size) to snapshot before every op.
    pub track_sp: Option<(u64, u8)>,
    /// Pointer-sized values read straight from the image, by value: the slot
    /// each was read from (an import table entry, a GOT slot).
    pub image_loads: BTreeMap<u64, u64>,
    /// Diagnostics: report accesses to this base variable within +-0x40.
    pub watch_base: Option<u32>,
    /// The entry stack pointer: stores relative to it are noted in the state
    /// until the next external call ([`State::note_store`]).
    pub stack_base: Option<u32>,
    notes: Vec<String>,
    /// The constants each symbolized location was seen to hold (shared with the explorer).
    pub sym_values: SymValues,
    /// The handler address a no-return interrupt produced in this instruction.
    noreturn_target: Option<ExprId>,
    /// Stores whose address the emulation could not place, ignored.
    pub dropped_stores: usize,
    /// Stack-pointer writes that are no variable plus a constant: what is
    /// pushed or popped after one has no place the emulation knows.
    pub lost_sp: usize,
    /// An in-function interpreter's virtual PC: a load within
    /// [`FETCH_WINDOW`] of the bytecode pointer it holds reads the file.
    pub vpc: Option<VpcSource>,
    /// Some fetch through `vpc` read writable memory as the file holds it.
    pub fetched_writable: bool,
    /// Every read of file data reads it as the file holds it (the
    /// interpreter probe's concrete run).
    pub trust_file: bool,
    /// Some read of a copy's destination was not a fetch through the VPC
    /// that the emitter folds to a constant: the copy is part of the program.
    pub copy_read_unfolded: bool,
    /// Reads of a copy's destination in the current instruction.
    pub insn_copy_reads: u32,
    /// What first set [`Self::copy_read_unfolded`], for the log.
    pub copy_read_note: Option<String>,
    last_trusted: bool,
    /// The store being executed was lost ([`AccessRecord::lost`]).
    last_lost: bool,
}

/// Symbol -> the constants its location held where paths met.
pub type SymValues = Rc<std::cell::RefCell<BTreeMap<VarId, std::collections::BTreeSet<u128>>>>;

/// Most recorded values a symbol-indexed load is split over.
const MAX_INDEXED_VALUES: usize = 16;

impl<'a> Emulator<'a> {
    pub fn new(target: &'a dyn SymTarget) -> Self {
        Emulator {
            target,
            prov: BTreeMap::new(),
            track_sp: None,
            watch_base: None,
            stack_base: None,
            notes: Vec::new(),
            image_loads: BTreeMap::new(),
            sym_values: Default::default(),
            noreturn_target: None,
            dropped_stores: 0,
            lost_sp: 0,
            vpc: None,
            fetched_writable: false,
            trust_file: false,
            copy_read_unfolded: false,
            insn_copy_reads: 0,
            copy_read_note: None,
            last_trusted: false,
            last_lost: false,
        }
    }

    /// A read of `size` bytes at `base + off` is a fetch through the virtual
    /// PC: the VPC holds a pointer into file data (or into a copy of it)
    /// within [`FETCH_WINDOW`] bytes of the access.
    fn through_vpc(&mut self, state: &mut State, arena: &mut ExprArena, base: Option<VarId>, off: i128, size: u8) -> bool {
        let Some(src) = self.vpc else { return false };
        let ps = self.target.ptr_size();
        let v = match src {
            VpcSource::Reg(r) => state.read_reg(arena, r, ps),
            VpcSource::Cell(b, o) => state.read_mem(arena, self.target, b, o, ps),
        };
        let Some((vb, p)) = arena.affine(v) else { return false };
        if vb != base || off < p - FETCH_WINDOW || off + size as i128 > p + FETCH_WINDOW {
            return false;
        }
        match base {
            None => u64::try_from(p).is_ok_and(|a| self.target.file_data(a)),
            Some(_) => state.copy_source(base, p).is_some_and(|a| self.target.file_data(a)),
        }
    }

    /// Read `size` bytes at `base + off`, through the virtual PC as the file
    /// holds them when [`Self::through_vpc`] says the load is a fetch.
    fn read_data(&mut self, state: &mut State, arena: &mut ExprArena, base: Option<VarId>, off: i128, size: u8) -> ExprId {
        let copied = (0..size as i128).any(|i| state.copy_source(base, off + i).is_some());
        let v = self.read_data_inner(state, arena, base, off, size);
        if copied {
            self.insn_copy_reads += 1;
            if !self.last_trusted || arena.is_const(v).is_none() {
                if self.copy_read_note.is_none() {
                    self.copy_read_note = Some(format!("copy read at {off:#x} outside a folded fetch: {}", arena.fmt(v)));
                }
                self.copy_read_unfolded = true;
            }
        }
        v
    }

    fn read_data_inner(&mut self, state: &mut State, arena: &mut ExprArena, base: Option<VarId>, off: i128, size: u8) -> ExprId {
        if self.trust_file {
            let file = match base {
                None => u64::try_from(off).is_ok_and(|a| self.target.file_data(a)),
                Some(_) => state.copy_source(base, off).is_some(),
            };
            if file {
                return state.read_mem_file(arena, self.target, base, off, size);
            }
        }
        if self.through_vpc(state, arena, base, off, size) {
            self.last_trusted = true;
            let src = match base {
                None => u64::try_from(off).ok(),
                Some(_) => state.copy_source(base, off),
            };
            if src.is_some_and(|a| self.target.is_mutable(a) || self.target.is_mutable(a.wrapping_add(size as u64 - 1))) {
                self.fetched_writable = true;
            }
            return state.read_mem_file(arena, self.target, base, off, size);
        }
        state.read_mem(arena, self.target, base, off, size)
    }

    fn vn_space(&self, vn: &VarnodeData) -> Space {
        vn.space.as_ref().map(classify_space).unwrap_or(Space::Other)
    }

    fn read_vn(&mut self, state: &mut State, arena: &mut ExprArena, vn: &VarnodeData) -> ExprId {
        let size = vn.size.min(MAX_SIZE as u32) as u8;
        if vn.size > MAX_SIZE as u32 {
            return self.unknown(arena, MAX_SIZE, "wide operand");
        }
        match self.vn_space(vn) {
            Space::Const => arena.cst(size, vn.offset as u128),
            Space::Register => state.read_reg(arena, vn.offset, size),
            Space::Unique => state.read_unique(arena, vn.offset, size),
            Space::Ram => {
                let from_image = !state.mem_written(None, vn.offset as i128, size);
                let v = state.read_mem(arena, self.target, None, vn.offset as i128, size);
                if from_image && size == self.target.ptr_size() {
                    if let Some(c) = arena.is_const(v) {
                        self.image_loads.insert(c as u64, vn.offset);
                    }
                }
                v
            }
            Space::Other => self.unknown(arena, size, "operand in an unmodelled space"),
        }
    }

    fn write_vn(&mut self, state: &mut State, arena: &mut ExprArena, vn: &VarnodeData, e: ExprId) {
        if vn.size > MAX_SIZE as u32 {
            // Wide (vector) writes: unknown chunks.
            let mut off = 0u64;
            while off < vn.size as u64 {
                let chunk = ((vn.size as u64 - off).min(MAX_SIZE as u64)) as u8;
                let u = self.unknown(arena, chunk, "wide write");
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
        // A stack pointer rebuilt through boolean-arithmetic noise is stored
        // in its plain `sp0 + d` form, and a register XOR-swapped through it
        // that comes out constant as that constant.
        let e = match self.track_sp {
            Some((sp, sps)) if self.vn_space(vn) == Space::Register && vn.offset == sp && size == sps && arena.affine(e).is_none() => {
                match arena.affine_probe(e) {
                    Some((Some(v), d)) => {
                        let info = *arena.var_info(v);
                        let base = arena.var_of_known(info.kind, info.size, info.boolean, info.known);
                        let k = arena.cst(size, d as u128);
                        arena.mk_bin(CPUI_INT_ADD, base, k, size)
                    }
                    _ => {
                        self.lost_sp += 1;
                        e
                    }
                }
            }
            _ if self.vn_space(vn) == Space::Register && matches!(arena.kind(e), ExprKind::Bin(CPUI_INT_XOR, _, _)) => match arena.affine_probe(e) {
                Some((None, c)) => arena.cst(size, c as u128),
                _ => e,
            },
            _ => e,
        };
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
    /// A load whose address depends on one symbolized location (a handler
    /// table indexed by an opcode the merge turned into a symbol): one read
    /// per constant the location was seen to hold, selected by comparing the
    /// symbol against it.  Only a table at concrete addresses is read this
    /// way: an array on the stack or behind a pointer indexed by a program
    /// variable (a loop counter) holds more than the indices the merges saw.
    fn indexed_load(&mut self, state: &mut State, arena: &mut ExprArena, addr: ExprId, size: u8) -> Option<ExprId> {
        let (var, values) = {
            let known = self.sym_values.borrow();
            let mut hit = None;
            for v in arena.free_vars(addr) {
                if let Some(vals) = known.get(&v) {
                    if hit.is_some() {
                        return None;
                    }
                    hit = Some((v, vals.iter().copied().collect::<Vec<u128>>()));
                }
            }
            hit?
        };
        if values.is_empty() || values.len() > MAX_INDEXED_VALUES {
            return None;
        }
        let ve = arena.var_expr(var)?;
        let vsize = arena.size(ve);
        let mut arms: Vec<(ExprId, ExprId)> = Vec::with_capacity(values.len());
        for c in values {
            let k = arena.cst(vsize, c);
            let mut map = BTreeMap::new();
            map.insert(ve, k);
            let a = arena.substitute(addr, &map);
            let (None, off) = arena.affine_probe(a)? else { return None };
            let val = state.read_mem(arena, self.target, None, off, size);
            arms.push((k, val));
        }
        let mut result = arms[arms.len() - 1].1;
        for (k, val) in arms[..arms.len() - 1].iter().rev() {
            let cond = arena.mk_bin(CPUI_INT_EQUAL, ve, *k, 1);
            result = arena.mk_ite(cond, *val, result);
        }
        Some(result)
    }

    /// A load at an address decided by merge symbols with recorded
    /// constants and boolean atoms ([`split_over_symbols`]: a virtual
    /// branch whose decision was merged at a join): one read per concrete
    /// address, selected by the disjunction of the cases that reach it.
    fn symbol_split_load(&mut self, state: &mut State, arena: &mut ExprArena, addr: ExprId, size: u8) -> Option<ExprId> {
        let cases = split_over_symbols(arena, &self.sym_values, addr)?;
        if cases.len() > MAX_SPLIT_ASSIGNMENTS {
            return None;
        }
        let mut groups: Vec<(i128, Vec<ExprId>)> = Vec::new();
        for (assign, a) in &cases {
            let (None, off) = arena.affine_probe(*a)? else { return None };
            let cond = assignment_cond(arena, assign);
            match groups.iter_mut().find(|(k, _)| *k == off) {
                Some((_, conds)) => conds.push(cond),
                None => groups.push((off, vec![cond])),
            }
        }
        if groups.len() > MAX_INDEXED_VALUES {
            return None;
        }
        let mut vals: Vec<ExprId> = Vec::with_capacity(groups.len());
        for (off, _) in &groups {
            vals.push(self.read_data(state, arena, None, *off, size));
        }
        let mut result = vals[vals.len() - 1];
        for i in (0..groups.len() - 1).rev() {
            let mut cond = groups[i].1[0];
            for c in &groups[i].1[1..] {
                cond = arena.mk_bin(CPUI_INT_OR, cond, *c, 1);
            }
            result = arena.mk_ite(cond, vals[i], result);
        }
        Some(result)
    }

    pub fn load(&mut self, state: &mut State, arena: &mut ExprArena, space: Space, addr: ExprId, size: u8) -> ExprId {
        match space {
            Space::Register => {
                if let Some(c) = arena.is_const(addr) {
                    return state.read_reg(arena, c as u64, size);
                }
                state.unknown_loads += 1;
                return self.unknown(arena, size, "register load at a symbolic offset");
            }
            Space::Ram => {}
            _ => {
                state.unknown_loads += 1;
                return self.unknown(arena, size, "load from an unmodelled space");
            }
        }
        if let Some((base, off)) = arena.affine_probe(addr) {
            let from_image = base.is_none() && !state.mem_written(None, off, size);
            let v = self.read_data(state, arena, base, off, size);
            if from_image && size == self.target.ptr_size() {
                if let Some(c) = arena.is_const(v) {
                    self.image_loads.insert(c as u64, off as u64);
                }
            }
            if base.is_some() && base == self.watch_base && (-0x40..0x40).contains(&off) {
                self.note(format!("watch load [{off:#x}] size {size} = {}", arena.fmt(v)));
            }
            return v;
        }
        // An opaque predicate an identity decides is no select.
        let decided = arena.decide_atoms(addr);
        if decided != addr {
            return self.load(state, arena, space, decided, size);
        }
        // An address decided by boolean atoms (a conditional jump's next
        // bytecode pointer): one load per distinct destination, selected by
        // the disjunction of the assignments that reach it.
        if let Some(cases) = arena.case_split(addr) {
            let mut groups: Vec<((Option<VarId>, i128), Vec<ExprId>)> = Vec::new();
            let mut ok = cases.len() <= MAX_SPLIT_ASSIGNMENTS;
            for (assign, a) in &cases {
                let Some(key) = arena.affine_probe(*a) else {
                    ok = false;
                    break;
                };
                let cond = assignment_cond(arena, assign);
                match groups.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, conds)) => conds.push(cond),
                    None => groups.push((key, vec![cond])),
                }
            }
            if ok && groups.len() <= MAX_SPLIT_CASES {
                let mut vals: Vec<ExprId> = Vec::with_capacity(groups.len());
                for ((base, off), _) in &groups {
                    vals.push(self.read_data(state, arena, *base, *off, size));
                }
                let mut result = vals[vals.len() - 1];
                for i in (0..groups.len() - 1).rev() {
                    let mut cond = groups[i].1[0];
                    for c in &groups[i].1[1..] {
                        cond = arena.mk_bin(CPUI_INT_OR, cond, *c, 1);
                    }
                    result = arena.mk_ite(cond, vals[i], result);
                }
                return result;
            }
        }
        if let Some(v) = self.indexed_load(state, arena, addr, size) {
            return v;
        }
        if let Some(v) = self.symbol_split_load(state, arena, addr, size) {
            return v;
        }
        state.unknown_loads += 1;
        let atoms = arena.bool_atoms(addr).len();
        let vars: Vec<String> = arena.free_vars(addr).iter().map(|v| format!("{:?}", arena.var_info(*v).kind)).collect();
        self.note(format!("unknown load address (atoms {atoms}, vars {vars:?}) {}", arena.fmt(addr)));
        arena.unknown(size)
    }

    /// A fresh unknown, with the reason on record for the trace.
    fn unknown(&mut self, arena: &mut ExprArena, size: u8, why: &str) -> ExprId {
        let u = arena.unknown(size);
        self.note(format!("{why}: {}", arena.fmt(u)));
        u
    }

    fn note(&mut self, s: String) {
        if self.notes.len() < 64 {
            self.notes.push(s);
        }
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
        if let Some((base, off)) = arena.affine_probe(addr) {
            if base.is_some() && base == self.watch_base && (-0x40..0x40).contains(&off) {
                self.note(format!("watch store [{off:#x}] size {size} = {}", arena.fmt(val)));
            }
            if base.is_some() && base == self.stack_base {
                state.note_store(off, size);
            }
            state.write_mem(base, off, val, size);
            return;
        }
        let decided = arena.decide_atoms(addr);
        if decided != addr {
            return self.store(state, arena, space, decided, val);
        }
        if let Some(cases) = arena.case_split(addr) {
            let mut groups: Vec<((Option<VarId>, i128), Vec<ExprId>)> = Vec::new();
            let mut ok = cases.len() <= MAX_SPLIT_ASSIGNMENTS;
            for (assign, a) in &cases {
                let Some(key) = arena.affine_probe(*a) else {
                    ok = false;
                    break;
                };
                let cond = assignment_cond(arena, assign);
                match groups.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, conds)) => conds.push(cond),
                    None => groups.push((key, vec![cond])),
                }
            }
            if ok && groups.len() <= MAX_SPLIT_CASES {
                for ((base, off), conds) in &groups {
                    let mut cond = conds[0];
                    for c in &conds[1..] {
                        cond = arena.mk_bin(CPUI_INT_OR, cond, *c, 1);
                    }
                    let old = state.read_mem(arena, self.target, *base, *off, size);
                    let new = arena.mk_ite(cond, val, old);
                    if base.is_some() && *base == self.stack_base {
                        state.note_store(*off, size);
                    }
                    state.write_mem(*base, *off, new, size);
                }
                return;
            }
        }
        state.unknown_stores += 1;
        self.note(format!("unknown store address {}", arena.fmt(addr)));
        let forgot = self.havoc_store(state, arena, addr);
        if !forgot {
            self.dropped_stores += 1;
        }
        self.last_lost = !forgot || self.reaches_kept(state, arena, addr);
    }

    /// The address of an unplaced store depends on a variable whose memory
    /// the state keeps (the entry stack, or the region under a symbolic
    /// base) without being one of the bases [`Self::havoc_store`] forgot:
    /// the store may have written there, and the exploration goes on reading
    /// what that memory held before it.
    fn reaches_kept(&self, state: &State, arena: &ExprArena, addr: ExprId) -> bool {
        let (terms, _) = arena.linear_terms(addr);
        let bases: Vec<VarId> = terms.iter().filter(|(_, c)| *c == 1).filter_map(|(t, _)| arena.as_var(*t)).collect();
        arena.free_vars(addr).into_iter().any(|v| !bases.contains(&v) && (Some(v) == self.stack_base || state.mem.contains_key(&Some(v))))
    }

    /// A store to an address over a symbolic index (`base + c * i + k`, an
    /// array element) that no split places: what it may have overwritten is
    /// forgotten.  Distinct symbolic bases never alias, so only the regions
    /// of the address's bases are touched; on the entry stack only the
    /// function's own frame from the array's start upward (`[k, 0)`, the
    /// index taken non-negative when every index term scales up), and below
    /// it too otherwise.  An address with no base is an array in the image:
    /// all mutable memory is forgotten.  False when nothing could be placed.
    fn havoc_store(&mut self, state: &mut State, arena: &mut ExprArena, addr: ExprId) -> bool {
        let (terms, k) = arena.linear_terms(addr);
        let size = arena.size(addr);
        if terms.is_empty() {
            return false;
        }
        let bases: Vec<u32> = terms.iter().filter(|(_, c)| *c == 1).filter_map(|(t, _)| arena.as_var(*t)).collect();
        if bases.is_empty() {
            let fresh = arena.unknown(1);
            let epoch = (1u64 << 63) | arena.as_var(fresh).unwrap_or(0) as u64;
            state.clobber_mutable(self.target, epoch);
            return true;
        }
        let upward = terms.iter().all(|(_, c)| !super::kuna_symexpr::sign(*c, size));
        for b in bases {
            let (lo, hi) = if arena.var_info(b).pointer { (if upward { k } else { i128::MIN }, 0) } else { (i128::MIN, i128::MAX) };
            state.forget_copies(Some(b), lo, hi);
            let Some(region) = state.mem.get_mut(&Some(b)) else { continue };
            region.remove_where(|o| o >= lo && o < hi);
        }
        true
    }

    /// Execute one instruction; the state is left as the fall-through side.
    pub fn exec_insn(&mut self, state: &mut State, arena: &mut ExprArena, insn: &LiftedInsn, trace: &mut InsnTrace) -> Exit {
        arena.charge(super::kuna_symexpr::Work::Op, insn.ops.len() as u64);
        state.clear_uniques();
        self.prov.clear();
        trace.op_out = vec![None; insn.ops.len()];
        trace.sp_before = vec![None; insn.ops.len()];
        trace.forked = false;
        trace.path.clear();
        self.insn_copy_reads = 0;
        self.notes.clear();
        self.noreturn_target = None;
        let mut visited = vec![false; insn.ops.len() + 1];
        let r = self.run_from(state, arena, insn, 0, trace, &mut visited, 0);
        trace.notes = std::mem::take(&mut self.notes);
        if trace.forked {
            trace.path.clear();
        }
        r
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
            trace.path.push(pc);
            if let Some((sp, sps)) = self.track_sp {
                let v = state.read_reg(arena, sp, sps);
                trace.sp_before[pc] = Some(v);
            }
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
                    let c = arena.decide_atoms(c);
                    // A CBRANCH has no output: its slot records the decision.
                    trace.op_out[pc] = Some(c);
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
                            Exit::Fallthrough(_) => Exit::CondJump { arms: vec![(c, target, taken)], fallthrough: next },
                            Exit::CondJump { mut arms, fallthrough } => {
                                arms.insert(0, (c, target, taken));
                                Exit::CondJump { arms, fallthrough }
                            }
                            Exit::Jump(t2) if arena.is_const(t2).is_some() => Exit::CondJump {
                                arms: vec![(c, target, taken)],
                                fallthrough: arena.is_const(t2).unwrap_or(0) as u64,
                            },
                            Exit::Jump(t2) => {
                                // `if c goto T; goto T2`: a two-way select.
                                let one = arena.cst(1, 1);
                                let nc = arena.mk_bin(CPUI_INT_XOR, c, one, 1);
                                let _ = nc;
                                let sel = arena.mk_ite(c, target, t2);
                                merge_ite(state, &mut taken.clone(), arena, self.target, c);
                                Exit::Jump(sel)
                            }
                            other => other,
                        };
                    }
                    if depth >= MAX_FORK_DEPTH {
                        return Exit::Halt(format!("deep p-code fork inside instruction at {:#x}", insn.addr));
                    }
                    trace.forked = true;
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
                    if self.noreturn_target == Some(target) {
                        return Exit::NoReturn;
                    }
                    return Exit::Call { target, fallthrough: next };
                }
                CPUI_RETURN => {
                    let t = self.read_vn(state, arena, &op.ins[0]);
                    return Exit::Return(t);
                }
                CPUI_CALLOTHER => {
                    let idx = op.ins[0].offset;
                    let name = self.target.userop_name(idx).unwrap_or_else(|| format!("userop_{idx}"));
                    let vector = op.ins.get(1).map(|v| self.read_vn(state, arena, v)).and_then(|v| arena.is_const(v));
                    let noreturn = name == "swi" && vector.is_some_and(|v| self.target.noreturn_interrupt(v as u64));
                    trace.userops.push(name);
                    if let Some(out) = &op.out {
                        let u = self.unknown(arena, out.size.min(MAX_SIZE as u32) as u8, "user-op result");
                        trace.op_out[pc] = Some(u);
                        self.write_vn(state, arena, out, u);
                        if noreturn {
                            self.noreturn_target = Some(u);
                        }
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
                        op_index: pc,
                        addr_expr: addr,
                        via: self.address_provenance(&op.ins[1]),
                        addr: arena.is_const(addr).map(|c| c as u64),
                        size,
                        is_store: false,
                        trusted: false,
                        lost: false,
                    });
                    self.last_trusted = false;
                    let v = if out.size > MAX_SIZE as u32 { self.unknown(arena, size, "wide load") } else { self.load(state, arena, space, addr, size) };
                    if self.last_trusted {
                        if let Some(a) = trace.accesses.last_mut() {
                            a.trusted = true;
                        }
                    }
                    trace.op_out[pc] = Some(v);
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
                        op_index: pc,
                        addr_expr: addr,
                        via: self.address_provenance(&op.ins[1]),
                        addr: arena.is_const(addr).map(|c| c as u64),
                        size: op.ins[2].size.min(MAX_SIZE as u32) as u8,
                        is_store: true,
                        trusted: false,
                        lost: false,
                    });
                    self.last_lost = false;
                    if op.ins[2].size <= MAX_SIZE as u32 {
                        self.store(state, arena, space, addr, val);
                    } else {
                        state.unknown_stores += 1;
                    }
                    if self.last_lost {
                        if let Some(a) = trace.accesses.last_mut() {
                            a.lost = true;
                        }
                    }
                }
                _ => {
                    self.note_provenance(op);
                    let Some(out) = &op.out else {
                        pc += 1;
                        continue;
                    };
                    if out.size > MAX_SIZE as u32 || op.ins.iter().any(|v| v.size > MAX_SIZE as u32) {
                        let u = self.unknown(arena, out.size.min(MAX_SIZE as u32) as u8, "wide arithmetic");
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
                        None => {
                            self.note(format!("unmodelled op {:?}", op.opc));
                            arena.unknown(out.size as u8)
                        }
                    };
                    trace.op_out[pc] = Some(v);
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
                Exit::CondJump { arms: vec![(c, t, Box::new(taken))], fallthrough: next }
            }
            (Exit::Fallthrough(_), Exit::Jump(t)) => {
                let one = arena.cst(1, 1);
                let nc = arena.mk_bin(CPUI_INT_XOR, c, one, 1);
                let fall_state = std::mem::replace(state, taken);
                Exit::CondJump { arms: vec![(nc, t, Box::new(fall_state))], fallthrough: next }
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

/// Most combinations of recorded constants [`split_over_symbols`] enumerates.
const MAX_SYMBOL_CASES: usize = 16;

/// The values `e` takes over every recorded constant of each merge symbol
/// it depends on whose constants are all the values its location held
/// (`sym_values`), and then over the boolean atoms that remain: each case
/// assigns `sym == k` true for its constants, followed by the atoms'
/// assignment.  `None` when `e` depends on no such symbol, when the
/// constants combine into more than [`MAX_SYMBOL_CASES`] cases, or when a
/// case stays symbolic without atoms to split.
pub fn split_over_symbols(arena: &mut ExprArena, sym_values: &SymValues, e: ExprId) -> Option<Vec<(Vec<(ExprId, bool)>, ExprId)>> {
    let syms: Vec<(VarId, Vec<u128>)> = {
        let known = sym_values.borrow();
        arena.free_vars(e).into_iter().filter_map(|v| known.get(&v).filter(|vals| !vals.is_empty()).map(|vals| (v, vals.iter().copied().collect()))).collect()
    };
    if syms.is_empty() {
        return None;
    }
    let mut total = 1usize;
    for (_, vals) in &syms {
        total = total.checked_mul(vals.len()).filter(|t| *t <= MAX_SYMBOL_CASES)?;
    }
    let exprs: Vec<ExprId> = syms.iter().map(|(v, _)| arena.var_expr(*v)).collect::<Option<_>>()?;
    let mut out = Vec::new();
    'combo: for idx in 0..total {
        let mut rest = idx;
        let mut map = BTreeMap::new();
        let mut pins = Vec::with_capacity(syms.len());
        for ((_, vals), &ve) in syms.iter().zip(&exprs) {
            let k = arena.cst(arena.size(ve), vals[rest % vals.len()]);
            rest /= vals.len();
            let atom = arena.mk_bin(CPUI_INT_EQUAL, ve, k, 1);
            match arena.is_const(atom) {
                Some(0) => continue 'combo,
                Some(_) => {}
                None => pins.push((atom, true)),
            }
            map.insert(ve, k);
        }
        let v = arena.substitute(e, &map);
        for (assign, r) in arena.case_split(v)? {
            let mut a = pins.clone();
            a.extend(assign);
            out.push((a, r));
        }
    }
    (!out.is_empty()).then_some(out)
}

/// The merge symbol and constant an atom of [`split_over_symbols`] pins:
/// the atom depends on one symbol with recorded constants and holds for
/// exactly one of them.
pub fn symbol_pin(arena: &mut ExprArena, sym_values: &SymValues, atom: ExprId) -> Option<(ExprId, ExprId)> {
    let vars = arena.free_vars(atom);
    let [v] = vars.iter().copied().collect::<Vec<_>>()[..] else { return None };
    let vals: Vec<u128> = sym_values.borrow().get(&v)?.iter().copied().collect();
    let x = arena.var_expr(v)?;
    let mut hit = None;
    for k in vals {
        let kc = arena.cst(arena.size(x), k);
        let map = BTreeMap::from([(x, kc)]);
        let r = arena.substitute(atom, &map);
        match arena.is_const(r)? {
            0 => {}
            _ if hit.is_none() => hit = Some(kc),
            _ => return None,
        }
    }
    hit.map(|k| (x, k))
}

/// `state := c ? taken : state`, location by location.
pub fn merge_ite(state: &mut State, taken: &mut State, arena: &mut ExprArena, image: &dyn SymImage, c: ExprId) {
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
    let empty = super::kuna_symstate::ByteStore::default();
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
    state.merge_stores(taken);
    if state.copies != taken.copies {
        state.copies.retain(|c| taken.copies.contains(c));
    }
}
