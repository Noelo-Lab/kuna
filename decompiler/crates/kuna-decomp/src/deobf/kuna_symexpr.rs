//! (kuna pushan) Symbolic expressions for the devirtualizer's constraint-free
//! emulation (Sudhir et al., "Pushan", NDSS 2027; angr PR #7680).
//!
//! A hash-consed DAG of p-code operations over symbolic variables.  Every node
//! is normalized at construction so that structurally equal values share one
//! id: constants fold; ADD/SUB/2COMP/NEGATE/MULT-by-constant chains collapse to
//! one linear normal form over their non-linear atoms; masks are pruned and
//! pushed through shifts with a known-bits lattice; truncations distribute over
//! the low-order arithmetic they commute with; `Ite` nodes carry a value that
//! differs between the two outcomes of a boolean.  The arena never needs an SMT
//! solver: the two questions the exploration asks -- "which constants can this
//! jump target take" and "is this guard fixed" -- are answered by case-splitting
//! on the boolean atoms of the expression ([`ExprArena::possible_values`]).

use std::collections::{BTreeMap, BTreeSet};

use kuna_num::opcodes::OpCode;
use OpCode::*;

pub type ExprId = u32;
pub type VarId = u32;

/// Largest value size the arena models (bytes); wider values become unknowns.
pub const MAX_SIZE: u8 = 16;
/// Tag base of unknowns produced by a division by zero.
pub const DIV_ZERO_TAG: u64 = 0x1000_0000;
/// Most boolean atoms a case split enumerates (2^N substitutions).
pub const MAX_SPLIT_ATOMS: usize = 10;
/// Levels of nested splitting (atoms exposed by an outer substitution).
const MAX_SPLIT_DEPTH: u32 = 4;
/// Substitutions one case split may perform in total.
const MAX_SPLIT_TOTAL: usize = 4096;
const MAX_DEPTH: u32 = 900;

#[inline]
pub fn mask(size: u8) -> u128 {
    if size >= 16 { u128::MAX } else { (1u128 << (8 * size as u32)) - 1 }
}
#[inline]
fn bits(size: u8) -> u32 {
    8 * size as u32
}
#[inline]
fn sext(v: u128, size: u8) -> i128 {
    let b = bits(size);
    if b >= 128 { v as i128 } else { ((v << (128 - b)) as i128) >> (128 - b) }
}
#[inline]
pub fn sign(v: u128, size: u8) -> bool {
    (v >> (bits(size) - 1)) & 1 == 1
}
#[inline]
fn neg(v: u128, size: u8) -> u128 {
    0u128.wrapping_sub(v) & mask(size)
}

/// Where a symbolic variable came from.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VarKind {
    /// The value a register held when emulation started (register-space offset).
    InitReg { offset: u64 },
    /// Initial memory contents: `base` is the symbolic base variable (None = concrete).
    InitMem { base: Option<VarId>, offset: i128 },
    /// The value an external call returned (site = native address of the call).
    CallResult { site: u64 },
    /// Whatever an external call left in a scratch register it may clobber.
    CallClobber { site: u64, offset: u64 },
    /// Memory an external call may have written through a pointer argument.
    CallOutput { site: u64 },
    /// Memory an external call may have written, read after the call that
    /// opened `epoch` (one per emulated call): mutable image memory at a
    /// concrete address (`base` None), or `base + offset` behind a symbolic
    /// base ([`State::clobber_call`](super::kuna_symstate::State::clobber_call)).
    CallOutputAt { epoch: u64, base: Option<VarId>, offset: i128 },
    /// A location that held different constants on two paths into a node.
    Merged { node: u32, tag: u64 },
    /// Anything the emulator does not model.
    Unknown { tag: u64 },
}

#[derive(Clone, Copy, Debug)]
pub struct VarInfo {
    pub kind: VarKind,
    pub size: u8,
    /// Only 0 or 1 (a flag register, a call's boolean result).
    pub boolean: bool,
    /// Bits known in advance: (known-zero mask, known-one mask).  The entry
    /// stack pointer's alignment is the main user.
    pub known: (u128, u128),
    /// A valid pointer: `var + c` never equals a small constant (the stack pointer).
    pub pointer: bool,
}

impl VarInfo {
    pub fn new(kind: VarKind, size: u8, boolean: bool) -> Self {
        VarInfo { kind, size, boolean, known: (0, 0), pointer: false }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ExprKind {
    Const(u128),
    Var(VarId),
    Un(OpCode, ExprId),
    Bin(OpCode, ExprId, ExprId),
    /// Low-order truncation starting at byte `off` of the operand.
    SubPiece(ExprId, u8),
    /// Concatenation (high, low).
    Piece(ExprId, ExprId),
    /// Select: boolean condition, value if 1, value if 0.
    Ite(ExprId, ExprId, ExprId),
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub size: u8,
}

/// A linear combination `sum(coef * atom) + c` modulo 2^bits.
#[derive(Clone, Debug, Default)]
struct Lin {
    terms: BTreeMap<ExprId, i128>,
    c: i128,
}

impl Lin {
    fn add(mut self, o: &Lin) -> Lin {
        for (t, k) in &o.terms {
            let e = self.terms.entry(*t).or_insert(0);
            *e = e.wrapping_add(*k);
        }
        self.c = self.c.wrapping_add(o.c);
        self
    }
    fn scale(mut self, k: i128) -> Lin {
        for v in self.terms.values_mut() {
            *v = v.wrapping_mul(k);
        }
        self.c = self.c.wrapping_mul(k);
        self
    }
    fn negate(self) -> Lin {
        self.scale(-1)
    }
}

pub struct ExprArena {
    /// Each node's values at the [`QUICK_LANES`] quick sample points, its
    /// atoms computed from their operands (None: a division by zero).
    quick: crate::kuna_fasthash::FxHashMap<ExprId, Option<[u128; QUICK_LANES]>>,
    /// Variables by size and the differences of their quick values.
    quick_sig: crate::kuna_fasthash::FxHashMap<(u8, [u128; QUICK_LANES - 1]), VarId>,
    /// The stack pointer and the bits of it the platform keeps clear
    /// (a user-mode stack's top bits), which the identity test may assume.
    pub sp_bound: Option<(VarId, u128)>,
    /// Identity decisions that held only with [`Self::sp_bound`] assumed.
    pub sp_bound_used: usize,
    /// The first few of those expressions, for the exploration log.
    pub sp_bound_notes: Vec<String>,
    nodes: Vec<Expr>,
    /// (known-zero, known-one) bit masks per node.
    known: Vec<(u128, u128)>,
    intern: BTreeMap<(ExprKind, u8), ExprId>,
    vars: Vec<VarInfo>,
    /// One variable per (kind, size): a register's initial value or a merged
    /// location is the same symbol in every exploration round.
    var_memo: BTreeMap<(VarKind, u8), ExprId>,
    unknown_tag: u64,
    probe_memo: BTreeMap<ExprId, Option<(Option<VarId>, i128)>>,
    /// Each constructor's result by its operands ([`ExprArena::memo`]).
    op_memo: crate::kuna_fasthash::FxHashMap<OpKey, ExprId>,
    /// The steps counted so far, by kind ([`Work`]): what the exploration's
    /// budget counts.
    steps: [std::cell::Cell<u64>; 5],
    /// A case split gives up once [`Self::work`] reaches this (the
    /// exploration's work budget).
    pub work_limit: Option<u64>,
    /// After this instant a case split gives up (the exploration's wall-clock backstop).
    pub deadline: Option<std::time::Instant>,
}

/// A constructor call: the rules recurse into the arms of selects and the
/// terms of sums, so a DAG that shares subterms is otherwise walked once per
/// path through it.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum OpKey {
    Bin(OpCode, ExprId, ExprId, u8),
    Un(OpCode, ExprId, u8),
    Sub(ExprId, u8, u8),
    Ite(ExprId, ExprId, ExprId),
    AndK(ExprId, u128, u8),
    Or(ExprId, ExprId, u8),
    Xor(ExprId, ExprId, u8),
}

/// Constructor results remembered before the memo starts over.
const OP_MEMO_LIMIT: usize = 1 << 21;
/// The steps the exploration's work budget counts ([`ExprArena::charge`]).
#[derive(Clone, Copy, Debug)]
pub enum Work {
    /// A p-code op emulated.
    Op,
    /// A constructor call the memo did not answer.
    Build,
    /// A constructor call the memo answered.
    Reuse,
    /// A node a substitution visited.
    Subst,
    /// A node evaluated at one sample point of the identity test.
    Eval,
}

/// The cost of one step of each kind of [`Work`], in units of about a tenth
/// of a microsecond of CPU time (fitted over the 48 test targets, spec 10.2).
const WORK_WEIGHTS: [u64; 5] = [2, 34, 13, 2, 1];
/// Bytes a B-tree map spends per entry beyond the entry itself.
pub const MAP_ENTRY_OVERHEAD: usize = 24;

impl Default for ExprArena {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprArena {
    pub fn new() -> Self {
        ExprArena {
            quick: Default::default(),
            quick_sig: Default::default(),
            sp_bound: None,
            sp_bound_used: 0,
            sp_bound_notes: Vec::new(),
            nodes: Vec::new(),
            known: Vec::new(),
            intern: BTreeMap::new(),
            vars: Vec::new(),
            var_memo: BTreeMap::new(),
            unknown_tag: 0,
            probe_memo: BTreeMap::new(),
            op_memo: Default::default(),
            steps: Default::default(),
            work_limit: None,
            deadline: None,
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Count `n` steps of kind `w`.
    #[inline]
    pub fn charge(&self, w: Work, n: u64) {
        let c = &self.steps[w as usize];
        c.set(c.get() + n);
    }

    /// The steps counted so far, by kind ([`Work`]).
    pub fn steps(&self) -> [u64; 5] {
        self.steps.each_ref().map(|c| c.get())
    }

    /// The work done so far: the steps counted, each weighted by its cost
    /// ([`WORK_WEIGHTS`]).
    pub fn work(&self) -> u64 {
        self.steps.iter().zip(WORK_WEIGHTS).map(|(c, w)| c.get() * w).sum()
    }

    /// The bytes the arena holds, estimated from the capacities of its node
    /// tables and caches (each a function of the work done alone).
    pub fn mem_estimate(&self) -> u64 {
        use std::mem::size_of;
        let intern = MAP_ENTRY_OVERHEAD + size_of::<((ExprKind, u8), ExprId)>();
        let quick = size_of::<(ExprId, Option<[u128; QUICK_LANES]>)>() + 1;
        let sig = size_of::<((u8, [u128; QUICK_LANES - 1]), VarId)>() + 1;
        let memo = size_of::<(OpKey, ExprId)>() + 1;
        let probe = size_of::<(ExprId, Option<(Option<VarId>, i128)>)>() + MAP_ENTRY_OVERHEAD;
        let var = size_of::<VarInfo>() + size_of::<((VarKind, u8), ExprId)>() + MAP_ENTRY_OVERHEAD;
        (self.nodes.capacity() * size_of::<Expr>()
            + self.known.capacity() * size_of::<(u128, u128)>()
            + self.intern.len() * intern
            + self.quick.capacity() * quick
            + self.quick_sig.capacity() * sig
            + self.op_memo.capacity() * memo
            + self.probe_memo.len() * probe
            + self.vars.len() * var) as u64
    }

    /// `build()`, or its result for the same call before.  A call that made
    /// a fresh unknown (a division by zero) is not remembered: two such
    /// unknowns must stay different values.
    fn memo(&mut self, key: OpKey, build: impl FnOnce(&mut Self) -> ExprId) -> ExprId {
        if let Some(&r) = self.op_memo.get(&key) {
            self.charge(Work::Reuse, 1);
            return r;
        }
        self.charge(Work::Build, 1);
        let tag = self.unknown_tag;
        let r = build(self);
        if self.unknown_tag == tag {
            if self.op_memo.len() >= OP_MEMO_LIMIT {
                self.op_memo.clear();
            }
            self.op_memo.insert(key, r);
        }
        r
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn expr(&self, e: ExprId) -> &Expr {
        &self.nodes[e as usize]
    }
    pub fn kind(&self, e: ExprId) -> &ExprKind {
        &self.nodes[e as usize].kind
    }
    pub fn size(&self, e: ExprId) -> u8 {
        self.nodes[e as usize].size
    }
    pub fn var_info(&self, v: VarId) -> &VarInfo {
        &self.vars[v as usize]
    }
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }
    pub fn known_bits(&self, e: ExprId) -> (u128, u128) {
        self.known[e as usize]
    }
    pub fn is_const(&self, e: ExprId) -> Option<u128> {
        match self.kind(e) {
            ExprKind::Const(c) => Some(*c),
            _ => None,
        }
    }
    /// The expression that is exactly variable `v`.
    pub fn var_expr(&self, v: VarId) -> Option<ExprId> {
        let size = self.vars.get(v as usize)?.size;
        self.intern.get(&(ExprKind::Var(v), size)).copied()
    }

    /// The origin of a bare symbolic variable.
    pub fn var_kind(&self, e: ExprId) -> Option<VarKind> {
        self.as_var(e).map(|v| self.vars[v as usize].kind)
    }

    pub fn as_var(&self, e: ExprId) -> Option<VarId> {
        match self.kind(e) {
            ExprKind::Var(v) => Some(*v),
            _ => None,
        }
    }
    /// The bits that may be one.
    pub fn possible_bits(&self, e: ExprId) -> u128 {
        !self.known[e as usize].0 & mask(self.size(e))
    }
    /// A one-byte value known to be 0 or 1.
    pub fn is_bool(&self, e: ExprId) -> bool {
        self.size(e) == 1 && self.known[e as usize].0 & 0xFE == 0xFE
    }

    // -- construction ---------------------------------------------------------

    fn intern(&mut self, kind: ExprKind, size: u8) -> ExprId {
        let key = (kind, size);
        if let Some(&id) = self.intern.get(&key) {
            return id;
        }
        let kb = self.compute_known(&key.0, size);
        // Every bit determined: the value is that constant.
        if !matches!(key.0, ExprKind::Const(_)) && (kb.0 | kb.1) & mask(size) == mask(size) {
            return self.cst(size, kb.1);
        }
        let id = self.nodes.len() as ExprId;
        self.nodes.push(Expr { kind: key.0.clone(), size });
        self.known.push(kb);
        self.intern.insert(key, id);
        id
    }

    pub fn cst(&mut self, size: u8, v: u128) -> ExprId {
        self.intern(ExprKind::Const(v & mask(size)), size)
    }

    pub fn var(&mut self, info: VarInfo) -> ExprId {
        let id = self.vars.len() as VarId;
        self.vars.push(info);
        self.intern(ExprKind::Var(id), info.size)
    }

    /// The variable for a stable location kind, created once per (kind, size).
    pub fn var_of(&mut self, kind: VarKind, size: u8, boolean: bool) -> ExprId {
        self.var_of_known(kind, size, boolean, (0, 0))
    }

    /// [`var_of`](Self::var_of) with bits known in advance (first creation wins).
    pub fn var_of_known(&mut self, kind: VarKind, size: u8, boolean: bool, known: (u128, u128)) -> ExprId {
        if let Some(&e) = self.var_memo.get(&(kind, size)) {
            return e;
        }
        let e = self.var(VarInfo { kind, size, boolean, known, pointer: false });
        self.var_memo.insert((kind, size), e);
        e
    }

    /// Whether `v` names the same location in every exploration round: the
    /// one variable of its kind ([`var_of`](Self::var_of)) for a register's
    /// or memory's entry value, behind no other base.  A call's outputs and
    /// the unknowns are made afresh each round, and a merge symbol whenever
    /// its known bits narrow.
    pub fn is_stable_var(&self, v: VarId) -> bool {
        let info = &self.vars[v as usize];
        if self.var_memo.get(&(info.kind, info.size)).and_then(|e| self.as_var(*e)) != Some(v) {
            return false;
        }
        match info.kind {
            VarKind::InitReg { .. } | VarKind::InitMem { base: None, .. } => true,
            VarKind::InitMem { base: Some(b), .. } => b != v && self.is_stable_var(b),
            _ => false,
        }
    }

    /// Mark a variable as a valid pointer (see [`VarInfo::pointer`]).
    pub fn mark_pointer(&mut self, e: ExprId) {
        if let Some(v) = self.as_var(e) {
            self.vars[v as usize].pointer = true;
        }
    }

    pub fn unknown(&mut self, size: u8) -> ExprId {
        self.unknown_tag += 1;
        let tag = self.unknown_tag;
        self.var(VarInfo::new(VarKind::Unknown { tag }, size, false))
    }

    /// Build any data-flow p-code op; `None` for ops the arena does not model.
    pub fn mk_op(&mut self, opc: OpCode, out: u8, ins: &[ExprId]) -> Option<ExprId> {
        if out == 0 || out > MAX_SIZE {
            return None;
        }
        Some(match (opc, ins.len()) {
            (CPUI_COPY, 1) => self.resize(ins[0], out),
            (CPUI_INT_ZEXT | CPUI_INT_SEXT | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_BOOL_NEGATE
            | CPUI_POPCOUNT | CPUI_LZCOUNT, 1) => self.mk_un(opc, ins[0], out),
            (CPUI_SUBPIECE, 2) => {
                let off = self.is_const(ins[1])?;
                if off > MAX_SIZE as u128 { return None; }
                self.mk_subpiece(ins[0], off as u8, out)
            }
            (CPUI_PIECE, 2) => self.mk_piece(ins[0], ins[1]),
            (CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR
            | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS
            | CPUI_INT_SLESSEQUAL | CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_INT_LEFT
            | CPUI_INT_RIGHT | CPUI_INT_SRIGHT | CPUI_INT_DIV | CPUI_INT_REM | CPUI_INT_SDIV | CPUI_INT_SREM
            | CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_BOOL_XOR, 2) => self.mk_bin(opc, ins[0], ins[1], out),
            _ => return None,
        })
    }

    /// Fit a value into `size` bytes (truncate or zero-extend) -- a COPY between
    /// differently sized storage never happens in p-code, but the state layer
    /// composes bytes through here.
    pub fn resize(&mut self, e: ExprId, size: u8) -> ExprId {
        let s = self.size(e);
        if s == size {
            e
        } else if size < s {
            self.mk_subpiece(e, 0, size)
        } else {
            self.mk_un(CPUI_INT_ZEXT, e, size)
        }
    }

    pub fn mk_un(&mut self, opc: OpCode, a: ExprId, out: u8) -> ExprId {
        self.memo(OpKey::Un(opc, a, out), |s| s.mk_un_raw(opc, a, out))
    }

    fn mk_un_raw(&mut self, opc: OpCode, a: ExprId, out: u8) -> ExprId {
        let asz = self.size(a);
        if let Some(c) = self.is_const(a) {
            if let Some(v) = fold_un(opc, out, c, asz) {
                return self.cst(out, v);
            }
        }
        match opc {
            CPUI_INT_2COMP => {
                let l = self.lin(a).negate();
                return self.build_linear(l, out);
            }
            CPUI_INT_NEGATE => {
                let mut l = self.lin(a).negate();
                l.c = l.c.wrapping_sub(1);
                return self.build_linear(l, out);
            }
            CPUI_BOOL_NEGATE => {
                let one = self.cst(1, 1);
                let a1 = self.resize(a, 1);
                return self.mk_bin(CPUI_INT_XOR, a1, one, 1);
            }
            CPUI_INT_ZEXT => {
                if out <= asz {
                    return self.resize(a, out);
                }
                if let Some(r) = self.zext_disjoint_sum(a, out) {
                    return r;
                }
                match self.kind(a).clone() {
                    ExprKind::Un(CPUI_INT_ZEXT, b) => return self.mk_un(CPUI_INT_ZEXT, b, out),
                    ExprKind::Ite(c, x, y) => {
                        let x2 = self.mk_un(CPUI_INT_ZEXT, x, out);
                        let y2 = self.mk_un(CPUI_INT_ZEXT, y, out);
                        return self.mk_ite(c, x2, y2);
                    }
                    _ => {}
                }
            }
            CPUI_INT_SEXT => {
                if out <= asz {
                    return self.resize(a, out);
                }
                // A value whose sign bit is known clear extends with zeros.
                if self.known[a as usize].0 >> (bits(asz) - 1) & 1 == 1 {
                    return self.mk_un(CPUI_INT_ZEXT, a, out);
                }
                if let ExprKind::Un(CPUI_INT_SEXT, b) = self.kind(a).clone() {
                    return self.mk_un(CPUI_INT_SEXT, b, out);
                }
            }
            _ => {}
        }
        self.intern(ExprKind::Un(opc, a), out)
    }

    pub fn mk_subpiece(&mut self, x: ExprId, off: u8, out: u8) -> ExprId {
        self.memo(OpKey::Sub(x, off, out), |s| s.mk_subpiece_raw(x, off, out))
    }

    fn mk_subpiece_raw(&mut self, x: ExprId, off: u8, out: u8) -> ExprId {
        let xs = self.size(x);
        if off as u32 + out as u32 > xs as u32 {
            // Reading past the operand: p-code pads with zero.
            let avail = xs.saturating_sub(off);
            if avail == 0 {
                return self.cst(out, 0);
            }
            let low = self.mk_subpiece(x, off, avail);
            return self.mk_un(CPUI_INT_ZEXT, low, out);
        }
        if off == 0 && out == xs {
            return x;
        }
        if let Some(c) = self.is_const(x) {
            return self.cst(out, (c >> (8 * off as u32)) & mask(out));
        }
        // Everything within the slice is known zero.
        if (self.possible_bits(x) >> (8 * off as u32)) & mask(out) == 0 {
            return self.cst(out, 0);
        }
        match self.kind(x).clone() {
            ExprKind::Un(CPUI_INT_ZEXT, y) => {
                let ys = self.size(y);
                if off >= ys {
                    return self.cst(out, 0);
                }
                if off == 0 {
                    return self.resize(y, out);
                }
                if off as u32 + out as u32 <= ys as u32 {
                    return self.mk_subpiece(y, off, out);
                }
            }
            ExprKind::SubPiece(y, off2) => return self.mk_subpiece(y, off + off2, out),
            ExprKind::Piece(hi, lo) => {
                let ls = self.size(lo);
                if off as u32 + out as u32 <= ls as u32 {
                    return self.mk_subpiece(lo, off, out);
                }
                if off >= ls {
                    return self.mk_subpiece(hi, off - ls, out);
                }
                if off == 0 {
                    let hi_part = self.mk_subpiece(hi, 0, out - ls);
                    return self.mk_piece(hi_part, lo);
                }
            }
            ExprKind::Ite(c, p, q) => {
                let p2 = self.mk_subpiece(p, off, out);
                let q2 = self.mk_subpiece(q, off, out);
                return self.mk_ite(c, p2, q2);
            }
            ExprKind::Bin(opc, a, b) if off == 0 => match opc {
                CPUI_INT_ADD | CPUI_INT_MULT | CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR => {
                    let a2 = self.mk_subpiece(a, 0, out);
                    let b2 = self.mk_subpiece(b, 0, out);
                    return self.mk_bin(opc, a2, b2, out);
                }
                CPUI_INT_LEFT => {
                    if let Some(sh) = self.is_const(b) {
                        if sh < bits(out) as u128 {
                            let a2 = self.mk_subpiece(a, 0, out);
                            return self.mk_bin(CPUI_INT_LEFT, a2, b, out);
                        }
                        return self.cst(out, 0);
                    }
                }
                CPUI_INT_RIGHT => {
                    // The low bytes of (y >> 8k) are a slice of y.
                    if let Some(sh) = self.is_const(b) {
                        if sh % 8 == 0 && (sh / 8) as u32 + out as u32 <= xs as u32 {
                            return self.mk_subpiece(a, (sh / 8) as u8, out);
                        }
                    }
                }
                _ => {}
            },
            ExprKind::Un(opc @ (CPUI_INT_2COMP | CPUI_INT_NEGATE), a) if off == 0 => {
                let a2 = self.mk_subpiece(a, 0, out);
                return self.mk_un(opc, a2, out);
            }
            _ => {}
        }
        self.intern(ExprKind::SubPiece(x, off), out)
    }

    pub fn mk_piece(&mut self, hi: ExprId, lo: ExprId) -> ExprId {
        let hs = self.size(hi);
        let ls = self.size(lo);
        let out = hs + ls;
        if let (Some(h), Some(l)) = (self.is_const(hi), self.is_const(lo)) {
            return self.cst(out, (h << bits(ls)) | l);
        }
        if self.is_const(hi) == Some(0) {
            return self.mk_un(CPUI_INT_ZEXT, lo, out);
        }
        if let (ExprKind::SubPiece(x, xo), ExprKind::SubPiece(y, yo)) = (self.kind(hi).clone(), self.kind(lo).clone()) {
            if x == y && xo == yo + ls {
                return self.mk_subpiece(x, yo, out);
            }
        }
        if let ExprKind::SubPiece(x, xo) = self.kind(hi).clone() {
            if xo == ls && self.size(x) == out {
                let lo2 = self.mk_subpiece(x, 0, ls);
                if lo2 == lo {
                    return x;
                }
            }
        }
        if let ExprKind::Ite(c, p, q) = self.kind(lo).clone() {
            if self.is_const(hi).is_some() {
                let p2 = self.mk_piece(hi, p);
                let q2 = self.mk_piece(hi, q);
                return self.mk_ite(c, p2, q2);
            }
        }
        self.intern(ExprKind::Piece(hi, lo), out)
    }

    pub fn mk_ite(&mut self, c: ExprId, a: ExprId, b: ExprId) -> ExprId {
        self.memo(OpKey::Ite(c, a, b), |s| s.mk_ite_raw(c, a, b))
    }

    fn mk_ite_raw(&mut self, c: ExprId, a: ExprId, b: ExprId) -> ExprId {
        if a == b {
            return a;
        }
        if let Some(v) = self.is_const(c) {
            return if v & 1 == 1 { a } else { b };
        }
        let size = self.size(a);
        if size == 1 && self.is_bool(c) {
            if self.is_const(a) == Some(1) && self.is_const(b) == Some(0) {
                return c;
            }
            if self.is_const(a) == Some(0) && self.is_const(b) == Some(1) {
                let one = self.cst(1, 1);
                return self.mk_bin(CPUI_INT_XOR, c, one, 1);
            }
        }
        if let ExprKind::Bin(CPUI_INT_XOR, x, k) = self.kind(c).clone() {
            if self.is_const(k) == Some(1) && self.is_bool(x) {
                return self.mk_ite(x, b, a);
            }
        }
        if let ExprKind::Ite(c2, p, _) = self.kind(a).clone() {
            if c2 == c {
                return self.mk_ite(c, p, b);
            }
        }
        if let ExprKind::Ite(c2, _, q) = self.kind(b).clone() {
            if c2 == c {
                return self.mk_ite(c, a, q);
            }
        }
        self.intern(ExprKind::Ite(c, a, b), size)
    }

    pub fn mk_bin(&mut self, opc: OpCode, a: ExprId, b: ExprId, out: u8) -> ExprId {
        self.memo(OpKey::Bin(opc, a, b, out), |s| s.mk_bin_raw(opc, a, b, out))
    }

    fn mk_bin_raw(&mut self, opc: OpCode, a: ExprId, b: ExprId, out: u8) -> ExprId {
        let asz = self.size(a);
        let bsz = self.size(b);
        if let (Some(x), Some(y)) = (self.is_const(a), self.is_const(b)) {
            if let Some(v) = fold_bin(opc, out, x, asz, y, bsz) {
                return self.cst(out, v);
            }
            if matches!(opc, CPUI_INT_DIV | CPUI_INT_REM | CPUI_INT_SDIV | CPUI_INT_SREM) {
                // Division by zero: an unknown tagged so diagnostics can tell.
                self.unknown_tag += 1;
                let tag = DIV_ZERO_TAG + self.unknown_tag;
                return self.var(VarInfo::new(VarKind::Unknown { tag }, out, false));
            }
        }
        match opc {
            CPUI_INT_ADD => {
                let l = self.lin(a).add(&self.lin(b));
                self.build_linear(l, out)
            }
            CPUI_INT_SUB => {
                let l = self.lin(a).add(&self.lin(b).negate());
                self.build_linear(l, out)
            }
            CPUI_INT_MULT => {
                if let Some(k) = self.is_const(b) {
                    let l = self.lin(a).scale(k as i128);
                    return self.build_linear(l, out);
                }
                if let Some(k) = self.is_const(a) {
                    let l = self.lin(b).scale(k as i128);
                    return self.build_linear(l, out);
                }
                let (a, b) = if a <= b { (a, b) } else { (b, a) };
                self.intern(ExprKind::Bin(CPUI_INT_MULT, a, b), out)
            }
            CPUI_BOOL_AND => self.mk_bin(CPUI_INT_AND, a, b, out),
            CPUI_BOOL_OR => self.mk_bin(CPUI_INT_OR, a, b, out),
            CPUI_BOOL_XOR => self.mk_bin(CPUI_INT_XOR, a, b, out),
            CPUI_INT_AND => self.mk_and(a, b, out),
            CPUI_INT_OR => self.mk_or(a, b, out),
            CPUI_INT_XOR => self.mk_xor(a, b, out),
            CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT => self.mk_shift(opc, a, b, out),
            CPUI_INT_NOTEQUAL => {
                let eq = self.mk_bin(CPUI_INT_EQUAL, a, b, 1);
                let one = self.cst(1, 1);
                self.mk_bin(CPUI_INT_XOR, eq, one, 1)
            }
            CPUI_INT_LESSEQUAL => {
                let lt = self.mk_bin(CPUI_INT_LESS, b, a, 1);
                let one = self.cst(1, 1);
                self.mk_bin(CPUI_INT_XOR, lt, one, 1)
            }
            CPUI_INT_SLESSEQUAL => {
                let lt = self.mk_bin(CPUI_INT_SLESS, b, a, 1);
                let one = self.cst(1, 1);
                self.mk_bin(CPUI_INT_XOR, lt, one, 1)
            }
            CPUI_INT_EQUAL => self.mk_equal(a, b),
            CPUI_INT_LESS => self.mk_less(a, b),
            CPUI_INT_SLESS => self.mk_sless(a, b),
            CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
                if let Some(r) = self.ite_distribute(opc, a, b, out) {
                    return r;
                }
                let (a, b) = if opc != CPUI_INT_SBORROW && a > b { (b, a) } else { (a, b) };
                self.intern(ExprKind::Bin(opc, a, b), 1)
            }
            _ => {
                if let Some(r) = self.ite_distribute(opc, a, b, out) {
                    return r;
                }
                self.intern(ExprKind::Bin(opc, a, b), out)
            }
        }
    }

    /// Fold a binary op over two constants (the direct `mk_*` entry points must
    /// never see two constants unfolded: several rules swap operands).
    fn fold2(&mut self, opc: OpCode, a: ExprId, b: ExprId, out: u8) -> Option<ExprId> {
        let (x, y) = (self.is_const(a)?, self.is_const(b)?);
        let v = fold_bin(opc, out, x, self.size(a), y, self.size(b))?;
        Some(self.cst(out, v))
    }

    /// `op(Ite(c,x,y), k)` with constant `k` selects between the two folded arms.
    fn ite_distribute(&mut self, opc: OpCode, a: ExprId, b: ExprId, out: u8) -> Option<ExprId> {
        if let ExprKind::Ite(c, x, y) = self.kind(a).clone() {
            if self.is_const(b).is_some() {
                let x2 = self.mk_bin(opc, x, b, out);
                let y2 = self.mk_bin(opc, y, b, out);
                return Some(self.mk_ite(c, x2, y2));
            }
            if let ExprKind::Ite(c2, p, q) = self.kind(b).clone() {
                if c2 == c {
                    let x2 = self.mk_bin(opc, x, p, out);
                    let y2 = self.mk_bin(opc, y, q, out);
                    return Some(self.mk_ite(c, x2, y2));
                }
            }
        }
        if let ExprKind::Ite(c, x, y) = self.kind(b).clone() {
            if self.is_const(a).is_some() {
                let x2 = self.mk_bin(opc, a, x, out);
                let y2 = self.mk_bin(opc, a, y, out);
                return Some(self.mk_ite(c, x2, y2));
            }
        }
        None
    }

    fn mk_and(&mut self, a: ExprId, b: ExprId, out: u8) -> ExprId {
        let a = self.resize(a, out);
        let b = self.resize(b, out);
        if let Some(r) = self.fold2(CPUI_INT_AND, a, b, out) {
            return r;
        }
        if a == b {
            return a;
        }
        let m = mask(out);
        // Known-bits: a bit that is zero in either side is zero in the result.
        let pa = self.possible_bits(a);
        let pb = self.possible_bits(b);
        if pa & pb == 0 {
            return self.cst(out, 0);
        }
        let (ka0, ka1) = self.known[a as usize];
        let (kb0, kb1) = self.known[b as usize];
        if kb1 & pa == pa {
            return a; // b has ones everywhere a may be one
        }
        if ka1 & pb == pb {
            return b;
        }
        let _ = (ka0, kb0);
        if let Some(c) = self.is_const(b) {
            return self.mk_and_const(a, c & m, out);
        }
        if let Some(c) = self.is_const(a) {
            return self.mk_and_const(b, c & m, out);
        }
        if let Some(r) = self.ite_distribute(CPUI_INT_AND, a, b, out) {
            return r;
        }
        // x & ~x
        if self.is_complement(a, b) {
            return self.cst(out, 0);
        }
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        self.intern(ExprKind::Bin(CPUI_INT_AND, a, b), out)
    }

    fn mk_and_const(&mut self, x: ExprId, c: u128, out: u8) -> ExprId {
        self.memo(OpKey::AndK(x, c, out), |s| s.mk_and_const_raw(x, c, out))
    }

    fn mk_and_const_raw(&mut self, x: ExprId, c: u128, out: u8) -> ExprId {
        if let Some(v) = self.is_const(x) {
            return self.cst(out, v & c);
        }
        let px = self.possible_bits(x);
        if px & c == 0 {
            return self.cst(out, 0);
        }
        if px & !c == 0 {
            return x;
        }
        // Clearing bits whose values are known is a subtraction (stack alignment).
        let cleared = !c & mask(out);
        let (kz, ko) = self.known[x as usize];
        if (kz | ko) & cleared == cleared {
            let mut l = self.lin(x);
            l.c = l.c.wrapping_sub((ko & cleared) as i128);
            return self.build_linear(l, out);
        }
        if let Some(r) = self.and_disjoint_sum(x, c, out) {
            return r;
        }
        if let Some(r) = self.and_complement_sum(x, c, out) {
            return r;
        }
        match self.kind(x).clone() {
            ExprKind::Bin(CPUI_INT_AND, y, k) if self.is_const(k).is_some() => {
                let k2 = self.is_const(k).unwrap() & c;
                return self.mk_and_const(y, k2, out);
            }
            ExprKind::Bin(CPUI_INT_OR, y, z) => {
                let py = self.possible_bits(y);
                let pz = self.possible_bits(z);
                if py & c == 0 {
                    return self.mk_and_const(z, c, out);
                }
                if pz & c == 0 {
                    return self.mk_and_const(y, c, out);
                }
                if let Some(kz) = self.is_const(z) {
                    let y2 = self.mk_and_const(y, c, out);
                    let kc = self.cst(out, kz & c);
                    return self.mk_or(y2, kc, out);
                }
            }
            ExprKind::Bin(CPUI_INT_XOR, y, z) => {
                if let Some(kz) = self.is_const(z) {
                    let y2 = self.mk_and_const(y, c, out);
                    let kc = self.cst(out, kz & c);
                    return self.mk_xor(y2, kc, out);
                }
                let py = self.possible_bits(y);
                let pz = self.possible_bits(z);
                if py & c == 0 {
                    return self.mk_and_const(z, c, out);
                }
                if pz & c == 0 {
                    return self.mk_and_const(y, c, out);
                }
            }
            ExprKind::Bin(CPUI_INT_ADD, y, z) => {
                // Addends with no bits in or below the mask behave like OR.
                let low = lowmask_through(c);
                let py = self.possible_bits(y);
                let pz = self.possible_bits(z);
                if py & low == 0 {
                    return self.mk_and_const(z, c, out);
                }
                if pz & low == 0 {
                    return self.mk_and_const(y, c, out);
                }
            }
            ExprKind::Bin(CPUI_INT_RIGHT, y, sh) => {
                if let Some(s) = self.is_const(sh) {
                    if s < bits(out) as u128 && (c << s) >> s == c {
                        let y2 = self.mk_and_const(y, c << s, out);
                        return self.mk_shift(CPUI_INT_RIGHT, y2, sh, out);
                    }
                }
            }
            ExprKind::Bin(CPUI_INT_LEFT, y, sh) => {
                if let Some(s) = self.is_const(sh) {
                    if s < bits(out) as u128 && c & lowmask(s as u32) == 0 {
                        let y2 = self.mk_and_const(y, c >> s, out);
                        return self.mk_shift(CPUI_INT_LEFT, y2, sh, out);
                    }
                }
            }
            ExprKind::Un(CPUI_INT_ZEXT, y) => {
                let ys = self.size(y);
                if c & !mask(ys) == 0 {
                    let y2 = self.mk_and_const(y, c, ys);
                    return self.mk_un(CPUI_INT_ZEXT, y2, out);
                }
            }
            ExprKind::Piece(_hi, lo) => {
                let ls = self.size(lo);
                if c & !mask(ls) == 0 {
                    let lo2 = self.mk_and_const(lo, c, ls);
                    return self.mk_un(CPUI_INT_ZEXT, lo2, out);
                }
            }
            ExprKind::Ite(cond, p, q) => {
                let p2 = self.mk_and_const(p, c, out);
                let q2 = self.mk_and_const(q, c, out);
                return self.mk_ite(cond, p2, q2);
            }
            _ => {}
        }
        let k = self.cst(out, c);
        self.intern(ExprKind::Bin(CPUI_INT_AND, x, k), out)
    }

    fn mk_or(&mut self, a: ExprId, b: ExprId, out: u8) -> ExprId {
        self.memo(OpKey::Or(a, b, out), |s| s.mk_or_raw(a, b, out))
    }

    fn mk_or_raw(&mut self, a: ExprId, b: ExprId, out: u8) -> ExprId {
        let a = self.resize(a, out);
        let b = self.resize(b, out);
        if let Some(r) = self.fold2(CPUI_INT_OR, a, b, out) {
            return r;
        }
        if a == b {
            return a;
        }
        let m = mask(out);
        let pa = self.possible_bits(a);
        let pb = self.possible_bits(b);
        let (_, ka1) = self.known[a as usize];
        let (_, kb1) = self.known[b as usize];
        if pb == 0 || (ka1 & pb) == pb {
            return a;
        }
        if pa == 0 || (kb1 & pa) == pa {
            return b;
        }
        if (ka1 | kb1) & m == m {
            return self.cst(out, m);
        }
        if self.is_const(a).is_some() {
            return self.mk_or(b, a, out);
        }
        if let Some(c) = self.is_const(b) {
            if let ExprKind::Ite(cond, p, q) = self.kind(a).clone() {
                let p2 = self.mk_or(p, b, out);
                let q2 = self.mk_or(q, b, out);
                return self.mk_ite(cond, p2, q2);
            }
            // x | c == (x & ~c) + c: the set bits leave the linear form, so a
            // later mask sees through De Morgan shapes like ~(~x | c).
            let masked = self.mk_and_const(a, !c & m, out);
            let mut l = self.lin(masked);
            l.c = l.c.wrapping_add(c as i128);
            return self.build_linear(l, out);
        } else if let Some(r) = self.ite_distribute(CPUI_INT_OR, a, b, out) {
            return r;
        }
        if self.is_complement(a, b) {
            return self.cst(out, m);
        }
        // Disjoint operands: OR is ADD; keep the linear normal form canonical.
        if pa & pb == 0 {
            let l = self.lin(a).add(&self.lin(b));
            return self.build_linear(l, out);
        }
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        self.intern(ExprKind::Bin(CPUI_INT_OR, a, b), out)
    }

    fn mk_xor(&mut self, a: ExprId, b: ExprId, out: u8) -> ExprId {
        self.memo(OpKey::Xor(a, b, out), |s| s.mk_xor_raw(a, b, out))
    }

    fn mk_xor_raw(&mut self, a: ExprId, b: ExprId, out: u8) -> ExprId {
        let a = self.resize(a, out);
        let b = self.resize(b, out);
        if let Some(r) = self.fold2(CPUI_INT_XOR, a, b, out) {
            return r;
        }
        if a == b {
            return self.cst(out, 0);
        }
        // x ^ (x ^ y) = y: the XOR swap obfuscators use to exchange the stack pointer.
        for (x, y) in [(a, b), (b, a)] {
            if let ExprKind::Bin(CPUI_INT_XOR, p, q) = self.kind(y).clone() {
                if p == x {
                    return q;
                }
                if q == x {
                    return p;
                }
            }
        }
        let m = mask(out);
        if self.is_const(a).is_some() {
            return self.mk_xor(b, a, out);
        }
        if let Some(c) = self.is_const(b) {
            if c == 0 {
                return a;
            }
            if c == m {
                return self.mk_un(CPUI_INT_NEGATE, a, out);
            }
            match self.kind(a).clone() {
                ExprKind::Bin(CPUI_INT_XOR, y, k) if self.is_const(k).is_some() => {
                    let k2 = self.cst(out, self.is_const(k).unwrap() ^ c);
                    return self.mk_xor(y, k2, out);
                }
                ExprKind::Ite(cond, p, q) => {
                    let p2 = self.mk_xor(p, b, out);
                    let q2 = self.mk_xor(q, b, out);
                    return self.mk_ite(cond, p2, q2);
                }
                _ => {}
            }
            // Flipping a bit the operand never sets is an OR (hence an ADD).
            if self.possible_bits(a) & c == 0 {
                return self.mk_or(a, b, out);
            }
            // A boolean's negation stays `b ^ 1` (the canonical form the
            // comparison and select rules key on); anything else is the
            // disjoint sum x ^ c == (x & ~c) + (~x & c).
            if !(self.is_bool(a) && c == 1) {
                let keep = self.mk_and_const(a, !c & m, out);
                let na = self.mk_un(CPUI_INT_NEGATE, a, out);
                let flip = self.mk_and_const(na, c, out);
                return self.mk_or(keep, flip, out);
            }
            return self.intern(ExprKind::Bin(CPUI_INT_XOR, a, b), out);
        }
        if let Some(r) = self.ite_distribute(CPUI_INT_XOR, a, b, out) {
            return r;
        }
        if self.is_complement(a, b) {
            return self.cst(out, m);
        }
        let pa = self.possible_bits(a);
        let pb = self.possible_bits(b);
        if pa & pb == 0 {
            let l = self.lin(a).add(&self.lin(b));
            return self.build_linear(l, out);
        }
        // (x ^ k1) ^ y  ->  (x ^ y) ^ k1
        if let ExprKind::Bin(CPUI_INT_XOR, y, k) = self.kind(a).clone() {
            if self.is_const(k).is_some() {
                let inner = self.mk_xor(y, b, out);
                return self.mk_xor(inner, k, out);
            }
        }
        if let ExprKind::Bin(CPUI_INT_XOR, y, k) = self.kind(b).clone() {
            if self.is_const(k).is_some() {
                let inner = self.mk_xor(a, y, out);
                return self.mk_xor(inner, k, out);
            }
        }
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        self.intern(ExprKind::Bin(CPUI_INT_XOR, a, b), out)
    }

    /// `b == ~a` structurally (through the linear form).
    fn is_complement(&self, a: ExprId, b: ExprId) -> bool {
        let la = self.lin(a);
        let lb = self.lin(b);
        let sum = la.add(&lb);
        let size = self.size(a);
        sum.terms.values().all(|k| (*k as u128) & mask(size) == 0) && (sum.c as u128) & mask(size) == mask(size)
    }

    fn mk_shift(&mut self, opc: OpCode, a: ExprId, b: ExprId, out: u8) -> ExprId {
        let a = self.resize(a, out);
        if let Some(r) = self.fold2(opc, a, b, out) {
            return r;
        }
        let nbits = bits(out) as u128;
        if let Some(sh) = self.is_const(b) {
            if sh == 0 {
                return a;
            }
            if sh >= nbits {
                return match opc {
                    CPUI_INT_SRIGHT => {
                        let s = self.sign_bit(a, out);
                        let allones = self.cst(out, mask(out));
                        let zero = self.cst(out, 0);
                        self.mk_ite(s, allones, zero)
                    }
                    _ => self.cst(out, 0),
                };
            }
            let s = sh as u32;
            let pa = self.possible_bits(a);
            match opc {
                CPUI_INT_LEFT => {
                    if (pa << s) & mask(out) == 0 {
                        return self.cst(out, 0);
                    }
                    let l = self.lin(a).scale(1i128 << s);
                    return self.build_linear(l, out);
                }
                CPUI_INT_RIGHT => {
                    if pa >> s == 0 {
                        return self.cst(out, 0);
                    }
                    if let Some(t) = self.scaled_down(a, s, out) {
                        return self.mk_and_const(t, mask(out) >> s, out);
                    }
                    // Terms entirely below the shift fall off: prune them first.
                    let high = (mask(out) << s) & mask(out);
                    if let Some(pruned) = self.and_disjoint_sum(a, high, out) {
                        if pruned != a {
                            return self.mk_shift(CPUI_INT_RIGHT, pruned, b, out);
                        }
                    }
                }
                CPUI_INT_SRIGHT => {
                    if self.known[a as usize].0 >> (nbits - 1) & 1 == 1 {
                        return self.mk_shift(CPUI_INT_RIGHT, a, b, out);
                    }
                }
                _ => {}
            }
            match self.kind(a).clone() {
                ExprKind::Bin(op2, y, sh2) if op2 == opc && self.is_const(sh2).is_some() => {
                    let total = self.is_const(sh2).unwrap() + sh;
                    let k = self.cst(self.size(sh2), total.min(nbits));
                    return self.mk_shift(opc, y, k, out);
                }
                ExprKind::Bin(CPUI_INT_LEFT, y, sh2) if opc == CPUI_INT_RIGHT && self.is_const(sh2) == Some(sh) => {
                    return self.mk_and_const(y, mask(out) >> s, out);
                }
                ExprKind::Bin(CPUI_INT_RIGHT, y, sh2) if opc == CPUI_INT_LEFT && self.is_const(sh2) == Some(sh) => {
                    return self.mk_and_const(y, (mask(out) << s) & mask(out), out);
                }
                ExprKind::Ite(c, p, q) => {
                    let p2 = self.mk_shift(opc, p, b, out);
                    let q2 = self.mk_shift(opc, q, b, out);
                    return self.mk_ite(c, p2, q2);
                }
                ExprKind::Un(CPUI_INT_ZEXT, y) if opc == CPUI_INT_RIGHT => {
                    let ys = bits(self.size(y)) as u128;
                    if sh >= ys {
                        return self.cst(out, 0);
                    }
                    if sh % 8 == 0 {
                        let off = (sh / 8) as u8;
                        let keep = self.size(y) - off;
                        let part = self.mk_subpiece(y, off, keep);
                        return self.mk_un(CPUI_INT_ZEXT, part, out);
                    }
                }
                ExprKind::Piece(hi, lo) if opc == CPUI_INT_RIGHT && sh == bits(self.size(lo)) as u128 => {
                    let _ = lo;
                    return self.mk_un(CPUI_INT_ZEXT, hi, out);
                }
                _ => {}
            }
        }
        self.intern(ExprKind::Bin(opc, a, b), out)
    }

    /// `a / 2^s` when every term and the constant of `a`'s linear form is a
    /// multiple of `2^s` and `a` is not a lone atom.
    fn scaled_down(&mut self, a: ExprId, s: u32, size: u8) -> Option<ExprId> {
        let l = self.lin(a);
        let m = mask(size);
        let div = 1u128 << s;
        if l.terms.len() == 1 && (l.c as u128) & m == 0 && l.terms.values().all(|k| (*k as u128) & m == 1) {
            return None;
        }
        if (l.c as u128) & m % div != 0 || l.terms.values().any(|k| ((*k as u128) & m) % div != 0) {
            return None;
        }
        let mut scaled = Lin::default();
        for (t, k) in &l.terms {
            let kk = ((*k as u128) & m) >> s;
            scaled.terms.insert(*t, kk as i128);
        }
        scaled.c = (((l.c as u128) & m) >> s) as i128;
        Some(self.build_linear(scaled, size))
    }

    /// `~y & c` where `x = ~y` and `y` is a sum of pairwise disjoint terms:
    /// each term inside the mask is complemented in place, so a De Morgan
    /// shape over a flags word stays a prunable disjoint sum.
    fn and_complement_sum(&mut self, x: ExprId, c: u128, out: u8) -> Option<ExprId> {
        let m = mask(out);
        let mut ly = self.lin(x).negate();
        ly.c = ly.c.wrapping_sub(1);
        let terms: Vec<(ExprId, u128)> =
            ly.terms.iter().map(|(t, k)| (*t, (*k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        if terms.is_empty() {
            return None;
        }
        let c0 = (ly.c as u128) & m;
        let mut masks = Vec::with_capacity(terms.len());
        let mut seen = c0;
        for (t, k) in &terms {
            let pm = self.scaled_possible(*t, *k, out)?;
            if seen & pm != 0 {
                return None;
            }
            seen |= pm;
            masks.push(pm);
        }
        let mut l = Lin::default();
        // Bits the sum never sets are one in its complement.
        l.c = (c & !seen) as i128;
        for ((t, k), pm) in terms.iter().zip(masks.iter()) {
            if pm & c == 0 {
                continue;
            }
            if pm & !c != 0 {
                return None;
            }
            let s = k.trailing_zeros();
            let inner = if self.possible_bits(*t) == 1 {
                // k * (1 - b) for a 0/1 term.
                let b = self.resize(*t, 1);
                let one = self.cst(1, 1);
                let nb = self.mk_bin(CPUI_INT_XOR, b, one, 1);
                self.mk_un(CPUI_INT_ZEXT, nb, out)
            } else {
                let nt = self.mk_un(CPUI_INT_NEGATE, *t, out);
                self.mk_and_const(nt, pm >> s, out)
            };
            *l.terms.entry(inner).or_insert(0) += *k as i128;
        }
        Some(self.build_linear(l, out))
    }

    /// `zext(sum of pairwise disjoint terms)`: no carry leaves the narrow
    /// width, so the extension distributes over the terms.
    fn zext_disjoint_sum(&mut self, a: ExprId, out: u8) -> Option<ExprId> {
        let asz = self.size(a);
        let l = self.lin(a);
        let m = mask(asz);
        let terms: Vec<(ExprId, u128)> =
            l.terms.iter().map(|(t, k)| (*t, (*k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        let cl = (l.c as u128) & m;
        if terms.is_empty() || (terms.len() == 1 && terms[0].1 == 1 && cl == 0) {
            return None;
        }
        let mut seen = 0u128;
        for (t, k) in &terms {
            let pm = self.scaled_possible(*t, *k, asz)?;
            if seen & pm != 0 {
                return None;
            }
            seen |= pm;
        }
        if seen & cl != 0 {
            return None;
        }
        let mut wide = Lin::default();
        for (t, k) in terms {
            let zt = self.mk_un(CPUI_INT_ZEXT, t, out);
            wide.terms.insert(zt, k as i128);
        }
        wide.c = cl as i128;
        Some(self.build_linear(wide, out))
    }

    /// Possible bits of `coef * term` when the coefficient is a power of two.
    fn scaled_possible(&self, t: ExprId, k: u128, size: u8) -> Option<u128> {
        if !k.is_power_of_two() {
            return None;
        }
        Some((self.possible_bits(t) << k.trailing_zeros()) & mask(size))
    }

    /// `(sum of pairwise disjoint terms) & c`: drop the terms outside the mask.
    fn and_disjoint_sum(&mut self, x: ExprId, c: u128, out: u8) -> Option<ExprId> {
        let l = self.lin(x);
        let m = mask(out);
        let terms: Vec<(ExprId, u128)> =
            l.terms.iter().map(|(t, k)| (*t, (*k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        let cl = (l.c as u128) & m;
        if terms.is_empty() || (terms.len() == 1 && terms[0].1 == 1 && cl == 0) {
            return None;
        }
        let mut masks = Vec::with_capacity(terms.len() + 1);
        for (t, k) in &terms {
            masks.push(self.scaled_possible(*t, *k, out)?);
        }
        masks.push(cl);
        let mut seen = 0u128;
        for pm in &masks {
            if seen & pm != 0 {
                return None;
            }
            seen |= pm;
        }
        let keep: Vec<(ExprId, u128)> =
            terms.iter().zip(masks.iter()).filter(|(_, pm)| **pm & c != 0).map(|(tk, _)| *tk).collect();
        let kc = cl & c;
        if keep.is_empty() {
            return Some(self.cst(out, kc));
        }
        if keep.len() == 1 && kc == 0 {
            let (t, k) = keep[0];
            let s = k.trailing_zeros();
            let pm = (self.possible_bits(t) << s) & m;
            if pm & !c == 0 {
                let mut l1 = Lin::default();
                l1.terms.insert(t, k as i128);
                return Some(self.build_linear(l1, out));
            }
            if c & lowmask(s) == 0 {
                let inner = self.mk_and_const(t, c >> s, out);
                let mut l1 = Lin::default();
                l1.terms.insert(inner, k as i128);
                return Some(self.build_linear(l1, out));
            }
        }
        if keep.len() == terms.len() && kc == cl {
            return None;
        }
        let mut l1 = Lin::default();
        for (t, k) in keep {
            l1.terms.insert(t, k as i128);
        }
        l1.c = kc as i128;
        let sum = self.build_linear(l1, out);
        Some(self.mk_and_const(sum, c, out))
    }

    fn sign_bit(&mut self, a: ExprId, size: u8) -> ExprId {
        let sh = self.cst(1, bits(size) as u128 - 1);
        let top = self.mk_shift(CPUI_INT_RIGHT, a, sh, size);
        self.resize(top, 1)
    }

    fn mk_equal(&mut self, a: ExprId, b: ExprId) -> ExprId {
        if a == b {
            return self.cst(1, 1);
        }
        if let Some(r) = self.fold2(CPUI_INT_EQUAL, a, b, 1) {
            return r;
        }
        let size = self.size(a);
        let d = self.lin(a).add(&self.lin(b).negate());
        let m = mask(size);
        let nz: Vec<(ExprId, i128)> =
            d.terms.iter().filter(|(_, k)| (**k as u128) & m != 0).map(|(t, k)| (*t, *k)).collect();
        if nz.is_empty() {
            return self.cst(1, ((d.c as u128) & m == 0) as u128);
        }
        // k * b == c for a 0/1-valued b: decided by whether c is 0 or k.
        if nz.len() == 1 && self.possible_bits(nz[0].0) == 1 {
            let (t, k) = (nz[0].0, (nz[0].1 as u128) & m);
            let c = neg(d.c as u128, size);
            let b = self.resize(t, 1);
            if c == 0 {
                let one = self.cst(1, 1);
                return self.mk_bin(CPUI_INT_XOR, b, one, 1);
            }
            if c == k {
                return b;
            }
            return self.cst(1, 0);
        }
        if nz.len() == 1 && ((nz[0].1 as u128) & m == 1 || (nz[0].1 as u128) & m == m) {
            let t = nz[0].0;
            let c = if (nz[0].1 as u128) & m == 1 { neg(d.c as u128, size) } else { (d.c as u128) & m };
            // A pointer offset by a constant never lands on a small constant.
            if let Some(v) = self.as_var(t) {
                if self.vars[v as usize].pointer && c < (1u128 << 32) {
                    return self.cst(1, 0);
                }
            }
            // Bits set in the constant the term can never produce.
            if c & !self.possible_bits(t) != 0 {
                return self.cst(1, 0);
            }
            if self.is_bool(t) {
                if c == 1 {
                    return t;
                }
                let one = self.cst(1, 1);
                return self.mk_bin(CPUI_INT_XOR, t, one, 1);
            }
            match self.kind(t).clone() {
                ExprKind::Un(CPUI_INT_ZEXT, y) => {
                    let ys = self.size(y);
                    if c & !mask(ys) != 0 {
                        return self.cst(1, 0);
                    }
                    let k = self.cst(ys, c);
                    return self.mk_equal(y, k);
                }
                ExprKind::Ite(cond, p, q) => {
                    let k = self.cst(size, c);
                    let p2 = self.mk_equal(p, k);
                    let q2 = self.mk_equal(q, k);
                    return self.mk_ite(cond, p2, q2);
                }
                ExprKind::Bin(CPUI_INT_XOR, y, k) if self.is_const(k).is_some() => {
                    let k2 = self.cst(size, self.is_const(k).unwrap() ^ c);
                    return self.mk_equal(y, k2);
                }
                _ => {}
            }
            let k = self.cst(size, c);
            return self.intern(ExprKind::Bin(CPUI_INT_EQUAL, t, k), 1);
        }
        // Canonical sign: the lowest atom carries a "positive" coefficient.
        let first = nz[0].1;
        let d = if sign(first as u128 & m, size) { d.negate() } else { d };
        let lhs = self.build_linear(d, size);
        let zero = self.cst(size, 0);
        if let Some(c) = self.is_const(lhs) {
            return self.cst(1, (c == 0) as u128);
        }
        self.intern(ExprKind::Bin(CPUI_INT_EQUAL, lhs, zero), 1)
    }

    /// `base + c1 <op> base + c2` for a pointer base that cannot wrap: the
    /// constants decide (the interpreter's stack-bound checks).
    fn pointer_relative_compare(&mut self, a: ExprId, b: ExprId, signed: bool) -> Option<ExprId> {
        let (Some(va), ca) = self.affine(a)? else { return None };
        let (Some(vb), cb) = self.affine(b)? else { return None };
        if va != vb || !self.vars[va as usize].pointer {
            return None;
        }
        if ca.abs() >= (1i128 << 32) || cb.abs() >= (1i128 << 32) {
            return None;
        }
        let _ = signed;
        Some(self.cst(1, (ca < cb) as u128))
    }

    fn mk_less(&mut self, a: ExprId, b: ExprId) -> ExprId {
        if let Some(r) = self.fold2(CPUI_INT_LESS, a, b, 1) {
            return r;
        }
        if let Some(r) = self.pointer_relative_compare(a, b, false) {
            return r;
        }
        let size = self.size(a);
        let (ka0, ka1) = self.known[a as usize];
        let (kb0, kb1) = self.known[b as usize];
        let amax = !ka0 & mask(size);
        let bmax = !kb0 & mask(size);
        if amax < kb1 {
            return self.cst(1, 1);
        }
        if ka1 >= bmax {
            return self.cst(1, 0);
        }
        if let Some(c) = self.is_const(b) {
            if c == 0 {
                return self.cst(1, 0);
            }
            if c == 1 {
                let z = self.cst(size, 0);
                return self.mk_equal(a, z);
            }
        }
        if let Some(c) = self.is_const(a) {
            if c == mask(size) {
                return self.cst(1, 0);
            }
            if c == 0 {
                let z = self.cst(size, 0);
                let eq = self.mk_equal(b, z);
                let one = self.cst(1, 1);
                return self.mk_bin(CPUI_INT_XOR, eq, one, 1);
            }
        }
        if let Some(r) = self.ite_distribute(CPUI_INT_LESS, a, b, 1) {
            return r;
        }
        self.intern(ExprKind::Bin(CPUI_INT_LESS, a, b), 1)
    }

    fn mk_sless(&mut self, a: ExprId, b: ExprId) -> ExprId {
        if let Some(r) = self.fold2(CPUI_INT_SLESS, a, b, 1) {
            return r;
        }
        if let Some(r) = self.pointer_relative_compare(a, b, true) {
            return r;
        }
        let size = self.size(a);
        if let Some(r) = self.ite_distribute(CPUI_INT_SLESS, a, b, 1) {
            return r;
        }
        // Both sign bits known clear: an unsigned compare.
        let top = 1u128 << (bits(size) - 1);
        if self.known[a as usize].0 & top != 0 && self.known[b as usize].0 & top != 0 {
            return self.mk_less(a, b);
        }
        self.intern(ExprKind::Bin(CPUI_INT_SLESS, a, b), 1)
    }

    // -- linear normal form ---------------------------------------------------

    fn lin(&self, e: ExprId) -> Lin {
        self.lin_depth(e, 0)
    }

    fn lin_depth(&self, e: ExprId, depth: u32) -> Lin {
        let mut atom = Lin::default();
        if depth > MAX_DEPTH {
            atom.terms.insert(e, 1);
            return atom;
        }
        match self.kind(e) {
            ExprKind::Const(c) => Lin { terms: BTreeMap::new(), c: *c as i128 },
            ExprKind::Bin(CPUI_INT_ADD, a, b) => self.lin_depth(*a, depth + 1).add(&self.lin_depth(*b, depth + 1)),
            ExprKind::Bin(CPUI_INT_SUB, a, b) => {
                self.lin_depth(*a, depth + 1).add(&self.lin_depth(*b, depth + 1).negate())
            }
            ExprKind::Bin(CPUI_INT_MULT, a, b) => {
                if let Some(k) = self.is_const(*b) {
                    self.lin_depth(*a, depth + 1).scale(k as i128)
                } else if let Some(k) = self.is_const(*a) {
                    self.lin_depth(*b, depth + 1).scale(k as i128)
                } else {
                    atom.terms.insert(e, 1);
                    atom
                }
            }
            ExprKind::Un(CPUI_INT_2COMP, a) => self.lin_depth(*a, depth + 1).negate(),
            ExprKind::Un(CPUI_INT_NEGATE, a) => {
                let mut l = self.lin_depth(*a, depth + 1).negate();
                l.c = l.c.wrapping_sub(1);
                l
            }
            _ => {
                atom.terms.insert(e, 1);
                atom
            }
        }
    }

    /// `zext(b)` to `size`, when `a` is `b ^ 1` for a boolean `b` and that
    /// node exists.
    fn zext_of_not(&self, a: ExprId, size: u8) -> Option<ExprId> {
        let ExprKind::Bin(CPUI_INT_XOR, b, one) = self.kind(a) else { return None };
        if self.is_const(*one) != Some(1) || !self.is_bool(*b) {
            return None;
        }
        self.intern.get(&(ExprKind::Un(CPUI_INT_ZEXT, *b), size)).copied()
    }

    fn build_linear(&mut self, l: Lin, size: u8) -> ExprId {
        let m = mask(size);
        // zext(b ^ 1) == 1 - zext(b) for a boolean b, when both terms are in
        // the sum: a flags word and the word of its complement cancel.
        let mut l = l;
        let pairs: Vec<(ExprId, ExprId)> = l
            .terms
            .keys()
            .filter_map(|t| match self.kind(*t) {
                ExprKind::Un(CPUI_INT_ZEXT, a) => self.zext_of_not(*a, self.size(*t)).filter(|zb| l.terms.contains_key(zb)).map(|zb| (*t, zb)),
                _ => None,
            })
            .collect();
        for (t, zb) in pairs {
            if let Some(k) = l.terms.remove(&t) {
                let e = l.terms.entry(zb).or_insert(0);
                *e = e.wrapping_sub(k);
                l.c = l.c.wrapping_add(k);
            }
        }
        // A 0/1-valued term with a "negative" coefficient is rewritten on its
        // complement with a positive one (-k*b == k*(1-b) - k), so masks see a
        // sum of disjoint positive bits.
        let flips: Vec<(ExprId, i128)> = l
            .terms
            .iter()
            .filter(|(t, k)| {
                let kk = (**k as u128) & m;
                kk != 0 && sign(kk, size) && !sign(neg(kk, size), size) && self.possible_bits(**t) == 1
            })
            .map(|(t, k)| (*t, *k))
            .collect();
        for (t, k) in flips {
            l.terms.remove(&t);
            let b = self.resize(t, 1);
            let one = self.cst(1, 1);
            let nb = self.mk_bin(CPUI_INT_XOR, b, one, 1);
            let nt = self.resize(nb, size);
            let pos = (neg(k as u128, size)) as i128;
            *l.terms.entry(nt).or_insert(0) += pos;
            l.c = l.c.wrapping_add(k);
        }
        let c = (l.c as u128) & m;
        let terms: Vec<(ExprId, u128)> =
            l.terms.into_iter().map(|(t, k)| (t, (k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        if terms.is_empty() {
            return self.cst(size, c);
        }
        if terms.len() == 1 && terms[0].1 == m && c == m {
            let t = terms[0].0;
            return self.intern(ExprKind::Un(CPUI_INT_NEGATE, t), size);
        }
        let mut acc: Option<ExprId> = None;
        for (t, k) in terms {
            let t = self.resize(t, size);
            let piece = if k == 1 {
                t
            } else if k == m {
                self.intern(ExprKind::Un(CPUI_INT_2COMP, t), size)
            } else {
                let kc = self.cst(size, k);
                self.intern(ExprKind::Bin(CPUI_INT_MULT, t, kc), size)
            };
            acc = Some(match acc {
                None => piece,
                Some(x) => self.intern(ExprKind::Bin(CPUI_INT_ADD, x, piece), size),
            });
        }
        let mut r = acc.unwrap();
        if c != 0 {
            let kc = self.cst(size, c);
            r = self.intern(ExprKind::Bin(CPUI_INT_ADD, r, kc), size);
        }
        r
    }

    /// `e` as `sum(coef * term) + c`: its non-zero terms with their
    /// coefficients and its constant, sign-extended.
    pub fn linear_terms(&self, e: ExprId) -> (Vec<(ExprId, u128)>, i128) {
        let size = self.size(e);
        let l = self.lin(e);
        let m = mask(size);
        let terms = l.terms.iter().map(|(t, k)| (*t, (*k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        (terms, sext((l.c as u128) & m, size))
    }

    /// `e == base + c` for a symbolic base variable (None = a plain constant).
    pub fn affine(&self, e: ExprId) -> Option<(Option<VarId>, i128)> {
        let size = self.size(e);
        let l = self.lin(e);
        let m = mask(size);
        let nz: Vec<(ExprId, u128)> = l.terms.iter().map(|(t, k)| (*t, (*k as u128) & m)).filter(|(_, k)| *k != 0).collect();
        let c = sext((l.c as u128) & m, size);
        match nz.len() {
            0 => Some((None, c)),
            1 if nz[0].1 == 1 => self.as_var(nz[0].0).map(|v| (Some(v), c)),
            _ => None,
        }
    }

    /// [`affine`](Self::affine), and when the linear form does not see it,
    /// a probe: an expression over one variable that moves by exactly the
    /// variable's change at three dissimilar points is `var + d`; over a
    /// pointer variable, one that holds the same value at all three is that
    /// constant (an image base the obfuscator derives from the stack pointer).
    /// The points differ in the bits of the variable that are not known; a
    /// variable with at most four possible values is evaluated at each of
    /// them instead, so a select over it is never read as `var + d`.
    /// Without boolean atoms that answer must also pass
    /// [`identity_probe`](Self::identity_probe), which alone decides an
    /// expression over several variables, and a constant over one variable
    /// that is no pointer (a context offset decoded through a key,
    /// `x ^ ((x & !k) + (!x & k))`, is `k`); the cached quick points reject
    /// most expressions before either runs.  This is how angr's concretizing
    /// solver sees through mixed boolean-arithmetic rewrites of the stack
    /// pointer, without fixing the pointer's value.
    pub fn affine_probe(&mut self, e: ExprId) -> Option<(Option<VarId>, i128)> {
        if let Some(r) = self.affine(e) {
            return Some(r);
        }
        if let Some(r) = self.probe_memo.get(&e) {
            return *r;
        }
        if !self.quick_candidate(e) {
            self.probe_memo.insert(e, None);
            return None;
        }
        let vars = self.free_vars(e);
        let r = if vars.len() == 1 {
            let v = *vars.iter().next().unwrap();
            let info = self.vars[v as usize];
            let size = self.size(e);
            if info.size != size || size < 4 {
                None
            } else {
                let var_e = self.intern.get(&(ExprKind::Var(v), info.size)).copied();
                let var_e = var_e.unwrap_or(e);
                let m = mask(size);
                let mut d: Option<u128> = None;
                let mut same: Option<u128> = None;
                let mut ok = true;
                let free = m & !(info.known.0 | info.known.1);
                let mut points: Vec<u128> =
                    [0x0000_7ffd_3a5c_9e40u128, 0x0000_1234_5678_9ab0, 0x0000_6b8b_4567_3270].iter().map(|p| ((p & m) & !info.known.0) | info.known.1).collect();
                let mut constant = info.pointer;
                if info.pointer {
                    // A user-mode stack may sit below 4 GiB (a Windows main
                    // thread's does): a value that only holds high up is no constant.
                    points.push(((0x0014_fe40u128 & m) & !info.known.0) | info.known.1);
                }
                if free.count_ones() <= 2 {
                    points = (0..1u128 << free.count_ones()).map(|i| deposit(i, free) | info.known.1).collect();
                    constant = true;
                } else if points.iter().collect::<BTreeSet<_>>().len() < points.len() {
                    points = [0x9e40_3a5cu128, 0x9ab0_5678, 0x3270_4567].iter().map(|p| deposit(*p, free) | info.known.1).collect();
                }
                for pv in points {
                    let k = self.cst(size, pv);
                    let mut map = BTreeMap::new();
                    map.insert(var_e, k);
                    let r = self.substitute(e, &map);
                    let Some(c) = self.is_const(r) else {
                        ok = false;
                        constant = false;
                        break;
                    };
                    constant &= same.is_none_or(|x| x == c);
                    same = Some(c);
                    let diff = c.wrapping_sub(pv) & m;
                    if d.is_some_and(|x| x != diff) {
                        ok = false;
                        if !constant {
                            break;
                        }
                    }
                    d = Some(diff);
                }
                if ok && var_e != e {
                    d.map(|x| (Some(v), sext(x, size)))
                } else if constant && var_e != e {
                    same.map(|c| (None, sext(c, size)))
                } else {
                    None
                }
            }
        } else if vars.len() > 1 && self.size(e) >= 4 {
            self.identity_probe(e)
        } else {
            None
        };
        // A one-variable answer from the few probe points stands only if the
        // full identity test agrees (where it applies: no boolean atoms).
        let r = match r {
            Some(x) if vars.len() == 1 && self.bool_atoms(e).is_empty() => (self.identity_probe(e) == Some(x)).then_some(x),
            None if vars.len() == 1 && self.size(e) >= 4 => self.identity_probe(e).filter(|(b, _)| b.is_none()),
            r => r,
        };
        self.probe_memo.insert(e, r);
        r
    }

    // -- identity by evaluation -----------------------------------------------

    /// The value of input `x` at sample point `i` of the identity test: in
    /// the bits `free` leaves open (the rest is `ones`), a structured pattern
    /// (every fourth sample among the first [`ID_STRUCTURED`] * 4) or a
    /// pseudo-random word, both fixed by `seed` and `i` alone.  The structured
    /// patterns put each 8/16/32/64-bit lane of the value at the edges where a
    /// carry or a borrow crosses it: all zeros, all ones, `0111..1` and
    /// `1000..0`, with random bits above the lane.
    fn sample_bits(seed: u64, i: usize, size: u8, free: u128, ones: u128) -> u128 {
        let r = sample_word(seed, i);
        let raw = if i % 4 == 0 && i / 4 < ID_STRUCTURED {
            let slot = (i / 4 + (sample_word(seed, usize::MAX) as usize)) % ID_STRUCTURED;
            let n = bits(size);
            let (w, kind) = if slot < 4 { (n, slot) } else { (8u32 << ((slot - 4) / 4), (slot - 4) % 4) };
            if w > n {
                r
            } else {
                let low = lowmask(w);
                let edge = match kind {
                    0 => 0,
                    1 => low,
                    2 => low >> 1,
                    _ => 1u128 << (w - 1),
                };
                (r & !low) | edge
            }
        } else {
            r
        };
        (raw & free) | ones
    }

    /// Variable `v` at sample `i`: its known bits, the rest sampled.
    pub fn sample_value(&self, v: VarId, i: usize) -> u128 {
        let (free, ones) = self.input_bits(v as ExprId, true, None);
        Self::sample_bits(v as u64, i, self.vars[v as usize].size, free, ones)
    }

    /// The open bits and the known one bits of an input: a variable (by id;
    /// `clear` holds bits one variable keeps clear) or a boolean atom (by
    /// node, always one open bit).
    fn input_bits(&self, x: ExprId, is_var: bool, clear: Option<(VarId, u128)>) -> (u128, u128) {
        if !is_var {
            return (1, 0);
        }
        let info = &self.vars[x as usize];
        let m = if info.boolean { 1 } else { mask(info.size) };
        let cleared = clear.filter(|(v, _)| *v == x).map_or(0, |(_, b)| b);
        (m & !(info.known.0 | info.known.1 | cleared), info.known.1 & m)
    }

    /// `e`'s values at the quick sample points (cached per node, operands
    /// first): the variables at points of their own, the stack pointer
    /// within [`Self::sp_bound`], atoms computed from their operands.  A
    /// necessary condition of every identity the test can find, so a value
    /// these points already rule out costs no walk of its expression.
    fn quick_vals(&mut self, e: ExprId) -> Option<[u128; QUICK_LANES]> {
        if let Some(r) = self.quick.get(&e) {
            return *r;
        }
        if self.quick.len() > QUICK_MAX {
            self.quick.clear();
        }
        let mut stack = vec![(e, false)];
        while let Some((x, ready)) = stack.pop() {
            if self.quick.contains_key(&x) {
                continue;
            }
            let kids: [Option<ExprId>; 3] = match self.kind(x) {
                ExprKind::Const(_) | ExprKind::Var(_) => [None, None, None],
                ExprKind::Un(_, a) | ExprKind::SubPiece(a, _) => [Some(*a), None, None],
                ExprKind::Bin(_, a, b) | ExprKind::Piece(a, b) => [Some(*a), Some(*b), None],
                ExprKind::Ite(c, a, b) => [Some(*c), Some(*a), Some(*b)],
            };
            if !ready {
                stack.push((x, true));
                for k in kids.into_iter().flatten() {
                    if !self.quick.contains_key(&k) {
                        stack.push((k, false));
                    }
                }
                continue;
            }
            let get = |q: &crate::kuna_fasthash::FxHashMap<ExprId, Option<[u128; QUICK_LANES]>>, k: Option<ExprId>| k.and_then(|k| q.get(&k).copied().flatten());
            let size = self.size(x);
            let m = mask(size);
            let mut out = [0u128; QUICK_LANES];
            let ok = match self.kind(x).clone() {
                ExprKind::Const(c) => {
                    out = [c & m; QUICK_LANES];
                    true
                }
                ExprKind::Var(v) => {
                    let (free, ones) = self.input_bits(v as ExprId, true, self.sp_bound);
                    for (j, o) in out.iter_mut().enumerate() {
                        *o = Self::sample_bits(v as u64, ID_SAMPLES + 1 + j, self.vars[v as usize].size, free, ones) & m;
                    }
                    let info = &self.vars[v as usize];
                    if !info.boolean && info.size == size {
                        let sig = [1, 2, 3].map(|j| out[j].wrapping_sub(out[0]) & m);
                        self.quick_sig.entry((size, sig)).or_insert(v);
                    }
                    true
                }
                ExprKind::Un(opc, a) => match get(&self.quick, kids[0]) {
                    Some(va) => (0..QUICK_LANES).all(|j| fold_un(opc, size, va[j], self.size(a)).map(|r| out[j] = r).is_some()),
                    None => false,
                },
                ExprKind::Bin(opc, a, b) => match (get(&self.quick, kids[0]), get(&self.quick, kids[1])) {
                    (Some(va), Some(vb)) => (0..QUICK_LANES).all(|j| fold_bin(opc, size, va[j], self.size(a), vb[j], self.size(b)).map(|r| out[j] = r).is_some()),
                    _ => false,
                },
                ExprKind::SubPiece(_, off) => match get(&self.quick, kids[0]) {
                    Some(va) => {
                        for j in 0..QUICK_LANES {
                            out[j] = (va[j] >> (8 * off as u32)) & m;
                        }
                        true
                    }
                    None => false,
                },
                ExprKind::Piece(_, l) => match (get(&self.quick, kids[0]), get(&self.quick, kids[1])) {
                    (Some(vh), Some(vl)) => {
                        for j in 0..QUICK_LANES {
                            out[j] = ((vh[j] << bits(self.size(l))) | vl[j]) & m;
                        }
                        true
                    }
                    _ => false,
                },
                ExprKind::Ite(..) => match (get(&self.quick, kids[0]), get(&self.quick, kids[1]), get(&self.quick, kids[2])) {
                    (Some(vc), Some(va), Some(vb)) => {
                        for j in 0..QUICK_LANES {
                            out[j] = if vc[j] & 1 == 1 { va[j] } else { vb[j] };
                        }
                        true
                    }
                    _ => false,
                },
            };
            self.quick.insert(x, ok.then_some(out));
        }
        self.quick.get(&e).copied().flatten()
    }

    /// The quick points allow `e` to be a constant or one variable plus a constant.
    fn quick_candidate(&mut self, e: ExprId) -> bool {
        let Some(q) = self.quick_vals(e) else { return true };
        let size = self.size(e);
        let m = mask(size);
        if q.iter().all(|x| *x == q[0]) {
            return true;
        }
        let sig = [1, 2, 3].map(|j| q[j].wrapping_sub(q[0]) & m);
        self.quick_sig.contains_key(&(size, sig))
    }

    /// The values of `e` at the first `n` sample points, its variables and
    /// the boolean atoms `atoms` all inputs ([`Self::sample_value`]; an atom
    /// is drawn as a free bit, not computed from its operands), or at every
    /// assignment of the inputs' open bits when `exhaustive`.  Also each
    /// variable's values.  None when the expression spans more than
    /// [`ID_MAX_NODES`] nodes or a sample divides by zero.
    #[allow(clippy::too_many_arguments)]
    fn eval_samples(&self, e: ExprId, vars: &[VarId], atoms: &[ExprId], n: usize, exhaustive: bool, clear: Option<(VarId, u128)>) -> Option<(Vec<u128>, Vec<Vec<u128>>)> {
        let atom_set: std::collections::HashSet<ExprId> = atoms.iter().copied().collect();
        let mut nodes: Vec<ExprId> = Vec::new();
        let mut seen: std::collections::HashSet<ExprId> = std::collections::HashSet::new();
        let mut stack = vec![e];
        while let Some(x) = stack.pop() {
            if !seen.insert(x) {
                continue;
            }
            if seen.len() > ID_MAX_NODES {
                return None;
            }
            nodes.push(x);
            if atom_set.contains(&x) {
                continue;
            }
            match self.kind(x) {
                ExprKind::Const(_) | ExprKind::Var(_) => {}
                ExprKind::Un(_, a) | ExprKind::SubPiece(a, _) => stack.push(*a),
                ExprKind::Bin(_, a, b) | ExprKind::Piece(a, b) => {
                    stack.push(*a);
                    stack.push(*b);
                }
                ExprKind::Ite(c, a, b) => {
                    stack.push(*c);
                    stack.push(*a);
                    stack.push(*b);
                }
            }
        }
        // Operands are interned before the nodes over them: ascending ids are
        // a topological order.
        nodes.sort_unstable();
        let inputs: Vec<(ExprId, bool)> = vars.iter().map(|v| (*v as ExprId, true)).chain(atoms.iter().map(|a| (*a, false))).collect();
        let mut in_vals: Vec<Vec<u128>> = Vec::with_capacity(inputs.len());
        if exhaustive {
            let opens: Vec<(u128, u128)> = inputs.iter().map(|(x, v)| self.input_bits(*x, *v, clear)).collect();
            let total: u32 = opens.iter().map(|(f, _)| f.count_ones()).sum();
            let n = 1usize << total;
            let mut shift = 0u32;
            for (f, ones) in &opens {
                let w = f.count_ones();
                in_vals.push((0..n).map(|a| deposit(((a >> shift) as u128) & lowmask(w), *f) | ones).collect());
                shift += w;
            }
        } else {
            for (x, is_var) in &inputs {
                let (free, ones) = self.input_bits(*x, *is_var, clear);
                let (seed, size) = if *is_var { (*x as u64, self.vars[*x as usize].size) } else { ((1u64 << 40) | *x as u64, 1) };
                in_vals.push((0..n).map(|i| Self::sample_bits(seed, i, size, free, ones)).collect());
            }
        }
        let n = in_vals.first().map(|x| x.len()).unwrap_or(n);
        self.charge(Work::Eval, (nodes.len() * n) as u64);
        let pos: std::collections::HashMap<ExprId, usize> = nodes.iter().enumerate().map(|(i, x)| (*x, i)).collect();
        let var_pos: std::collections::HashMap<VarId, usize> = vars.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let atom_pos: std::collections::HashMap<ExprId, usize> = atoms.iter().enumerate().map(|(i, a)| (*a, vars.len() + i)).collect();
        let mut vals = vec![0u128; nodes.len() * n];
        for (k, &x) in nodes.iter().enumerate() {
            let size = self.size(x);
            let m = mask(size);
            let base = k * n;
            if let Some(&ai) = atom_pos.get(&x) {
                vals[base..base + n].copy_from_slice(&in_vals[ai]);
                continue;
            }
            match self.kind(x) {
                ExprKind::Const(c) => vals[base..base + n].fill(*c & m),
                ExprKind::Var(v) => {
                    let src = &in_vals[*var_pos.get(v)?];
                    for j in 0..n {
                        vals[base + j] = src[j] & m;
                    }
                }
                ExprKind::Un(opc, a) => {
                    let (pa, asz) = (pos[a] * n, self.size(*a));
                    for j in 0..n {
                        vals[base + j] = fold_un(*opc, size, vals[pa + j], asz)?;
                    }
                }
                ExprKind::Bin(opc, a, b) => {
                    let (pa, asz) = (pos[a] * n, self.size(*a));
                    let (pb, bsz) = (pos[b] * n, self.size(*b));
                    for j in 0..n {
                        vals[base + j] = fold_bin(*opc, size, vals[pa + j], asz, vals[pb + j], bsz)?;
                    }
                }
                ExprKind::SubPiece(a, off) => {
                    let pa = pos[a] * n;
                    for j in 0..n {
                        vals[base + j] = (vals[pa + j] >> (8 * *off as u32)) & m;
                    }
                }
                ExprKind::Piece(h, l) => {
                    let (ph, pl, ls) = (pos[h] * n, pos[l] * n, bits(self.size(*l)));
                    for j in 0..n {
                        vals[base + j] = ((vals[ph + j] << ls) | vals[pl + j]) & m;
                    }
                }
                ExprKind::Ite(c, a, b) => {
                    let (pc, pa, pb) = (pos[c] * n, pos[a] * n, pos[b] * n);
                    for j in 0..n {
                        vals[base + j] = if vals[pc + j] & 1 == 1 { vals[pa + j] } else { vals[pb + j] };
                    }
                }
            }
        }
        let root = pos[&e] * n;
        in_vals.truncate(vars.len());
        Some((vals[root..root + n].to_vec(), in_vals))
    }

    /// `e == base + k` for one of its variables, or `e == k`, decided by
    /// evaluating `e` at [`ID_SAMPLES`] points that assign every input at
    /// once (exhaustively when the inputs have at most
    /// [`ID_EXHAUSTIVE_BITS`] open bits): its variables, and every boolean
    /// atom it reads ([`Self::bool_atoms`]: comparisons and flags, a
    /// comparison over other comparisons included) as a free bit of its
    /// own.  Treating an atom as free asks for more than the program needs
    /// (the identity must hold whatever the comparison says), and spares the
    /// test a guard on program data that holds at one value only, which no
    /// sample of its operands would hit.
    /// The spec (10-deobfuscation 10.2) argues the error probability.
    pub fn identity_probe(&mut self, e: ExprId) -> Option<(Option<VarId>, i128)> {
        let size = self.size(e);
        if let Some(c) = self.is_const(e) {
            return Some((None, sext(c, size)));
        }
        if size < 2 || !self.quick_candidate(e) {
            return None;
        }
        let vars: Vec<VarId> = self.free_vars(e).into_iter().collect();
        if vars.is_empty() {
            return None;
        }
        let atoms: Vec<ExprId> = self.bool_atoms(e).into_iter().filter(|a| self.as_var(*a).is_none()).collect();
        if let Some(r) = self.identity_pass(e, &vars, &atoms, None) {
            return Some(r);
        }
        // Over the stack the platform gives user code (its top bits clear).
        let bound = self.sp_bound.filter(|(v, _)| vars.contains(v))?;
        let r = self.identity_pass(e, &vars, &atoms, Some(bound))?;
        self.sp_bound_used += 1;
        if self.sp_bound_notes.len() < 8 {
            let note = format!("{:?} = {}", r, self.fmt(e).chars().take(600).collect::<String>());
            self.sp_bound_notes.push(note);
        }
        Some(r)
    }

    /// `e` with each comparison atom whose outcome an identity decides
    /// replaced by that outcome: `x == y` where `x - y` is a constant, and
    /// `x < y` (signed or not) where both sides are one pointer plus
    /// constants (no wrap, as [`Self::pointer_relative_compare`] assumes) or
    /// both constants.  An opaque predicate built from mixed
    /// boolean-arithmetic identities folds this way; a comparison on program
    /// data stays.
    pub fn decide_atoms(&mut self, e: ExprId) -> ExprId {
        if self.is_const(e).is_some() {
            return e;
        }
        let mut map = BTreeMap::new();
        for a in self.leaf_atoms(e) {
            let ExprKind::Bin(opc, x, y) = self.kind(a).clone() else { continue };
            let size = self.size(x);
            let v = match opc {
                CPUI_INT_EQUAL => {
                    let d = self.mk_bin(CPUI_INT_SUB, x, y, size);
                    match self.identity_probe(d) {
                        Some((None, k)) => Some((k as u128 & mask(size) == 0) as u128),
                        _ => None,
                    }
                }
                CPUI_INT_LESS | CPUI_INT_SLESS | CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW => {
                    match (self.identity_probe(x), self.identity_probe(y)) {
                        (Some((None, cx)), Some((None, cy))) => fold_bin(opc, 1, cx as u128 & mask(size), size, cy as u128 & mask(size), size),
                        (Some((Some(bx), cx)), Some((Some(by), cy)))
                            if bx == by && self.vars[bx as usize].pointer && matches!(opc, CPUI_INT_LESS | CPUI_INT_SLESS) && cx.abs() < (1i128 << 32) && cy.abs() < (1i128 << 32) =>
                        {
                            Some((cx < cy) as u128)
                        }
                        _ => None,
                    }
                }
                _ => None,
            };
            if let Some(v) = v {
                let k = self.cst(1, v);
                map.insert(a, k);
            }
        }
        if map.is_empty() {
            return e;
        }
        self.substitute(e, &map)
    }

    /// One run of [`Self::identity_probe`], with one variable's `clear` bits held zero.
    fn identity_pass(&self, e: ExprId, vars: &[VarId], atoms: &[ExprId], clear: Option<(VarId, u128)>) -> Option<(Option<VarId>, i128)> {
        let size = self.size(e);
        let open_bits: u32 = vars.iter().map(|v| self.input_bits(*v as ExprId, true, clear).0.count_ones()).sum::<u32>() + atoms.len() as u32;
        let exhaustive = open_bits <= ID_EXHAUSTIVE_BITS;
        let m = mask(size);
        let mut found = None;
        for n in if exhaustive { &[0usize][..] } else { &[ID_QUICK, ID_SAMPLES][..] } {
            let (ev, var_vals) = self.eval_samples(e, vars, atoms, *n, exhaustive, clear)?;
            found = None;
            if ev.iter().all(|x| *x == ev[0]) {
                found = Some((None, ev[0]));
            } else {
                for (vi, v) in vars.iter().enumerate() {
                    let info = &self.vars[*v as usize];
                    if info.size != size || info.boolean {
                        continue;
                    }
                    let bv = &var_vals[vi];
                    let d = ev[0].wrapping_sub(bv[0]) & m;
                    if ev.iter().zip(bv).all(|(x, b)| x.wrapping_sub(*b) & m == d) {
                        found = Some((Some(*v), d));
                        break;
                    }
                }
            }
            found?;
        }
        found.map(|(b, d)| (b, sext(d, size)))
    }

    // -- known bits -----------------------------------------------------------

    fn compute_known(&self, kind: &ExprKind, size: u8) -> (u128, u128) {
        let m = mask(size);
        let unknown = (0u128, 0u128);
        let boolish = |k: (u128, u128)| (k.0 | 0xFE, k.1 & 1);
        match kind {
            ExprKind::Const(c) => (!c & m, c & m),
            ExprKind::Var(v) => {
                let info = &self.vars[*v as usize];
                if info.boolean { (0xFE, 0) } else { (info.known.0 & m, info.known.1 & m) }
            }
            ExprKind::Un(opc, a) => {
                let (az, ao) = self.known[*a as usize];
                let asz = self.size(*a);
                match opc {
                    CPUI_INT_ZEXT => (az | (m & !mask(asz)), ao),
                    CPUI_INT_SEXT => {
                        let top = 1u128 << (bits(asz) - 1);
                        if az & top != 0 {
                            (az | (m & !mask(asz)), ao)
                        } else if ao & top != 0 {
                            (az, ao | (m & !mask(asz)))
                        } else {
                            (az & mask(asz), ao & mask(asz))
                        }
                    }
                    CPUI_INT_NEGATE => (ao & m, az & m),
                    CPUI_INT_2COMP => {
                        // Low zero bits survive negation; the lowest set bit does too.
                        let tz = (az & m).trailing_ones();
                        let low = lowmask(tz.min(128));
                        if (az | ao) & m == m {
                            let v = neg(ao & m, size);
                            (!v & m, v)
                        } else if tz < bits(size) && ao >> tz & 1 == 1 {
                            (low & m, (1u128 << tz) & m)
                        } else {
                            (low & m, 0)
                        }
                    }
                    CPUI_BOOL_NEGATE => boolish(unknown),
                    CPUI_POPCOUNT => {
                        let maxc = bits(asz).next_power_of_two().trailing_zeros() + 1;
                        (!lowmask(maxc) & m, 0)
                    }
                    CPUI_LZCOUNT => {
                        let maxc = (bits(asz) + 1).next_power_of_two().trailing_zeros();
                        (!lowmask(maxc) & m, 0)
                    }
                    _ => unknown,
                }
            }
            ExprKind::Bin(opc, a, b) => {
                let (az, ao) = self.known[*a as usize];
                let (bz, bo) = self.known[*b as usize];
                let asz = self.size(*a);
                match opc {
                    CPUI_INT_AND => ((az | bz) & m, ao & bo & m),
                    CPUI_INT_OR => (az & bz & m, (ao | bo) & m),
                    CPUI_INT_XOR => {
                        let kb = (az | ao) & (bz | bo);
                        let v = ao ^ bo;
                        (kb & !v & m, kb & v & m)
                    }
                    CPUI_INT_ADD | CPUI_INT_SUB => {
                        let (z, o) = add_known(az, ao, bz, bo, *opc == CPUI_INT_SUB, size);
                        (z & m, o & m)
                    }
                    CPUI_INT_MULT => {
                        if let Some(k) = self.is_const(*b) {
                            if k.is_power_of_two() {
                                let s = k.trailing_zeros();
                                return (((az << s) | lowmask(s)) & m, (ao << s) & m);
                            }
                        }
                        let tz = (az & m).trailing_ones() + (bz & m).trailing_ones();
                        let low = lowmask(tz.min(128));
                        // Leading zeros: product of n-bit and k-bit values fits n+k bits.
                        let an = width_of(!az & m);
                        let bn = width_of(!bz & m);
                        let hi = if an + bn < bits(size) { !lowmask(an + bn) & m } else { 0 };
                        ((low | hi) & m, 0)
                    }
                    CPUI_INT_LEFT => {
                        if let Some(s) = self.is_const(*b) {
                            if s >= bits(size) as u128 {
                                (m, 0)
                            } else {
                                let s = s as u32;
                                (((az << s) | lowmask(s)) & m, (ao << s) & m)
                            }
                        } else {
                            // Shifting left keeps low zero bits.
                            let tz = (az & m).trailing_ones();
                            (lowmask(tz.min(128)) & m, 0)
                        }
                    }
                    CPUI_INT_RIGHT => {
                        if let Some(s) = self.is_const(*b) {
                            if s >= bits(size) as u128 {
                                (m, 0)
                            } else {
                                let s = s as u32;
                                ((((az & m) >> s) | (!(m >> s) & m)) & m, ((ao & m) >> s) & m)
                            }
                        } else {
                            // The result never exceeds the operand's width.
                            let w = width_of(!az & m);
                            (!lowmask(w) & m, 0)
                        }
                    }
                    CPUI_INT_SRIGHT => {
                        let top = 1u128 << (bits(size) - 1);
                        if let Some(s) = self.is_const(*b) {
                            let s = (s as u32).min(bits(size) - 1);
                            let fill = !(m >> s) & m;
                            let z = ((az & m) >> s) & m;
                            let o = ((ao & m) >> s) & m;
                            if az & top != 0 {
                                ((z | fill) & m, o)
                            } else if ao & top != 0 {
                                (z & !fill, (o | fill) & m)
                            } else {
                                (z & !fill, o & !fill)
                            }
                        } else if az & top != 0 {
                            let w = width_of(!az & m);
                            (!lowmask(w) & m, 0)
                        } else {
                            unknown
                        }
                    }
                    CPUI_INT_REM => {
                        if let Some(c) = self.is_const(*b) {
                            let w = width_of(c);
                            (!lowmask(w) & m, 0)
                        } else {
                            let w = width_of(!bz & m);
                            (!lowmask(w) & m, 0)
                        }
                    }
                    CPUI_INT_DIV => {
                        let w = width_of(!az & mask(asz));
                        (!lowmask(w) & m, 0)
                    }
                    CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS
                    | CPUI_INT_SLESSEQUAL | CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_BOOL_AND
                    | CPUI_BOOL_OR | CPUI_BOOL_XOR | CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_LESS
                    | CPUI_FLOAT_LESSEQUAL | CPUI_FLOAT_NAN => boolish(unknown),
                    _ => unknown,
                }
            }
            ExprKind::SubPiece(a, off) => {
                let (az, ao) = self.known[*a as usize];
                let s = 8 * *off as u32;
                ((az >> s) & m, (ao >> s) & m)
            }
            ExprKind::Piece(hi, lo) => {
                let (hz, ho) = self.known[*hi as usize];
                let (lz, lo1) = self.known[*lo as usize];
                let ls = bits(self.size(*lo));
                let lm = mask(self.size(*lo));
                (((hz << ls) | (lz & lm)) & m, ((ho << ls) | (lo1 & lm)) & m)
            }
            ExprKind::Ite(_, a, b) => {
                let (az, ao) = self.known[*a as usize];
                let (bz, bo) = self.known[*b as usize];
                (az & bz & m, ao & bo & m)
            }
        }
    }

    // -- queries ----------------------------------------------------------------

    /// Every symbolic variable `e` depends on.
    pub fn free_vars(&self, e: ExprId) -> BTreeSet<VarId> {
        let mut out = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut stack = vec![e];
        while let Some(n) = stack.pop() {
            if !seen.insert(n) {
                continue;
            }
            match self.kind(n) {
                ExprKind::Const(_) => {}
                ExprKind::Var(v) => {
                    out.insert(*v);
                }
                ExprKind::Un(_, a) | ExprKind::SubPiece(a, _) => stack.push(*a),
                ExprKind::Bin(_, a, b) | ExprKind::Piece(a, b) => {
                    stack.push(*a);
                    stack.push(*b);
                }
                ExprKind::Ite(c, a, b) => {
                    stack.push(*c);
                    stack.push(*a);
                    stack.push(*b);
                }
            }
        }
        out
    }

    /// The boolean leaves a value is decided by: comparisons and boolean
    /// variables, reached through the boolean connectives and the arithmetic
    /// built over them.  Other symbolic variables are passed over; a case
    /// split then leaves them in place.
    pub fn bool_atoms(&self, e: ExprId) -> Vec<ExprId> {
        let mut atoms = Vec::new();
        let mut seen = BTreeSet::new();
        let mut stack = vec![e];
        while let Some(n) = stack.pop() {
            if !seen.insert(n) {
                continue;
            }
            if self.is_const(n).is_some() {
                continue;
            }
            if self.is_bool(n) {
                match self.kind(n) {
                    ExprKind::Bin(CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR, a, b) => {
                        stack.push(*a);
                        stack.push(*b);
                    }
                    ExprKind::Ite(c, a, b) => {
                        stack.push(*c);
                        stack.push(*a);
                        stack.push(*b);
                    }
                    ExprKind::SubPiece(a, _) => stack.push(*a),
                    _ => atoms.push(n),
                }
                continue;
            }
            match self.kind(n) {
                ExprKind::Var(_) => {}
                ExprKind::Un(_, a) | ExprKind::SubPiece(a, _) => stack.push(*a),
                ExprKind::Bin(_, a, b) | ExprKind::Piece(a, b) => {
                    stack.push(*a);
                    stack.push(*b);
                }
                ExprKind::Ite(c, a, b) => {
                    stack.push(*c);
                    stack.push(*a);
                    stack.push(*b);
                }
                ExprKind::Const(_) => {}
            }
        }
        atoms.sort_unstable();
        atoms.dedup();
        atoms
    }

    /// Rebuild `e` with the given nodes replaced (and everything re-simplified).
    pub fn substitute(&mut self, e: ExprId, map: &BTreeMap<ExprId, ExprId>) -> ExprId {
        let mut memo = BTreeMap::new();
        self.subst_rec(e, map, &mut memo, 0)
    }

    fn subst_rec(&mut self, e: ExprId, map: &BTreeMap<ExprId, ExprId>, memo: &mut BTreeMap<ExprId, ExprId>, depth: u32) -> ExprId {
        if let Some(r) = map.get(&e) {
            return *r;
        }
        if let Some(r) = memo.get(&e) {
            return *r;
        }
        if depth > MAX_DEPTH {
            return e;
        }
        self.charge(Work::Subst, 1);
        let size = self.size(e);
        let r = match self.kind(e).clone() {
            ExprKind::Const(_) | ExprKind::Var(_) => e,
            ExprKind::Un(opc, a) => {
                let a2 = self.subst_rec(a, map, memo, depth + 1);
                if a2 == a { e } else { self.mk_un(opc, a2, size) }
            }
            ExprKind::Bin(opc, a, b) => {
                let a2 = self.subst_rec(a, map, memo, depth + 1);
                let b2 = self.subst_rec(b, map, memo, depth + 1);
                if a2 == a && b2 == b { e } else { self.mk_bin(opc, a2, b2, size) }
            }
            ExprKind::SubPiece(a, off) => {
                let a2 = self.subst_rec(a, map, memo, depth + 1);
                if a2 == a { e } else { self.mk_subpiece(a2, off, size) }
            }
            ExprKind::Piece(a, b) => {
                let a2 = self.subst_rec(a, map, memo, depth + 1);
                let b2 = self.subst_rec(b, map, memo, depth + 1);
                if a2 == a && b2 == b { e } else { self.mk_piece(a2, b2) }
            }
            ExprKind::Ite(c, a, b) => {
                let c2 = self.subst_rec(c, map, memo, depth + 1);
                let a2 = self.subst_rec(a, map, memo, depth + 1);
                let b2 = self.subst_rec(b, map, memo, depth + 1);
                if c2 == c && a2 == a && b2 == b { e } else { self.mk_ite(c2, a2, b2) }
            }
        };
        memo.insert(e, r);
        r
    }

    /// The values `e` takes over every assignment of the boolean atoms it
    /// actually depends on, as expressions (constants when nothing else is
    /// symbolic).  Atoms are split innermost first: a comparison whose own
    /// operands are decided by other atoms (the interpreter's flags computed
    /// from flags) is left to fold under the inner assignment, and only the
    /// atoms that survive the substitution are split in turn.  An atom whose
    /// flip never changes the result is dropped from the assignments.
    /// Different cases may therefore assign different atoms.
    pub fn case_split(&mut self, e: ExprId) -> Option<Vec<(Vec<(ExprId, bool)>, ExprId)>> {
        if self.is_const(e).is_some() {
            return Some(vec![(Vec::new(), e)]);
        }
        let mut budget = MAX_SPLIT_TOTAL;
        let out = self.split_rec(e, 0, &mut budget)?;
        if out.len() == 1 && out[0].0.is_empty() && self.is_const(out[0].1).is_none() {
            return None;
        }
        Some(out)
    }

    /// The innermost atoms of `e`: atoms whose operands hold no atom, found
    /// through the operands of every atom that holds one (a comparison over
    /// flags computed from other comparisons is decided by those).
    fn leaf_atoms(&self, e: ExprId) -> Vec<ExprId> {
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        let mut stack = self.bool_atoms(e);
        while let Some(a) = stack.pop() {
            if !seen.insert(a) {
                continue;
            }
            let inner: Vec<ExprId> = match self.kind(a).clone() {
                ExprKind::Bin(_, x, y) => self.bool_atoms(x).into_iter().chain(self.bool_atoms(y)).collect(),
                ExprKind::Un(_, x) => self.bool_atoms(x),
                _ => Vec::new(),
            };
            if inner.is_empty() {
                out.push(a);
            } else {
                stack.extend(inner);
            }
        }
        out.sort_unstable();
        out
    }

    fn split_rec(&mut self, e: ExprId, depth: u32, budget: &mut usize) -> Option<Vec<(Vec<(ExprId, bool)>, ExprId)>> {
        if self.is_const(e).is_some() {
            return Some(vec![(Vec::new(), e)]);
        }
        let atoms = self.leaf_atoms(e);
        if atoms.is_empty() {
            return Some(vec![(Vec::new(), e)]);
        }
        if atoms.len() > MAX_SPLIT_ATOMS || depth > MAX_SPLIT_DEPTH {
            return None;
        }
        let n = atoms.len();
        let total = 1usize << n;
        if total > *budget {
            return None;
        }
        *budget -= total;
        let mut results = Vec::with_capacity(total);
        for bitsv in 0..total {
            if self.work_limit.is_some_and(|l| self.work() >= l) || self.deadline.is_some_and(|d| std::time::Instant::now() >= d) {
                return None;
            }
            let mut map = BTreeMap::new();
            for (i, a) in atoms.iter().enumerate() {
                let k = self.cst(1, ((bitsv >> i) & 1) as u128);
                map.insert(*a, k);
            }
            results.push(self.substitute(e, &map));
        }
        let essential: Vec<usize> = (0..n)
            .filter(|&i| (0..total).any(|b| results[b] != results[b ^ (1 << i)]))
            .collect();
        let mut out = Vec::new();
        for bitsv in 0..(1usize << essential.len()) {
            let mut full = 0usize;
            let mut assign = Vec::with_capacity(essential.len());
            for (j, &i) in essential.iter().enumerate() {
                let v = (bitsv >> j) & 1 == 1;
                if v {
                    full |= 1 << i;
                }
                assign.push((atoms[i], v));
            }
            let r = results[full];
            if self.is_const(r).is_some() || self.bool_atoms(r).is_empty() {
                out.push((assign, r));
                continue;
            }
            for (sub, v) in self.split_rec(r, depth + 1, budget)? {
                let mut a2 = assign.clone();
                a2.extend(sub);
                out.push((a2, v));
            }
        }
        Some(out)
    }

    /// The constants `e` can evaluate to, or `None` when it stays symbolic.
    pub fn possible_values(&mut self, e: ExprId) -> Option<Vec<u128>> {
        if let Some(c) = self.is_const(e) {
            return Some(vec![c]);
        }
        let cases = self.case_split(e)?;
        let mut vals = BTreeSet::new();
        for (_, r) in cases {
            vals.insert(self.is_const(r)?);
        }
        Some(vals.into_iter().collect())
    }

    /// Render for diagnostics, cut off after [`MAX_FMT`] bytes (a DAG prints
    /// as a tree, which can be exponentially long).
    pub fn fmt(&self, e: ExprId) -> String {
        let mut s = String::new();
        self.fmt_rec(e, &mut s, 0);
        s
    }

    fn fmt_rec(&self, e: ExprId, s: &mut String, depth: u32) {
        use std::fmt::Write;
        if depth > 40 || s.len() > MAX_FMT {
            s.push_str("...");
            return;
        }
        match self.kind(e) {
            ExprKind::Const(c) => {
                let _ = write!(s, "{:#x}:{}", c, self.size(e));
            }
            ExprKind::Var(v) => {
                let info = &self.vars[*v as usize];
                let _ = match info.kind {
                    VarKind::InitReg { offset } => write!(s, "reg{:x}_{}", offset, info.size),
                    VarKind::InitMem { base, offset } => write!(s, "mem[{:?}{:+#x}]_{}", base, offset, info.size),
                    VarKind::CallResult { site } => write!(s, "ret@{:x}", site),
                    VarKind::CallClobber { site, offset } => write!(s, "clobber{:x}@{:x}", offset, site),
                    VarKind::CallOutput { site } => write!(s, "out@{:x}", site),
                    VarKind::CallOutputAt { epoch, base: None, offset } => write!(s, "out#{}[{:#x}]", epoch, offset),
                    VarKind::CallOutputAt { epoch, base: Some(b), offset } => write!(s, "out#{}[v{}+{:#x}]", epoch, b, offset),
                    VarKind::Merged { node, tag } => write!(s, "sym{}_{:x}", node, tag),
                    VarKind::Unknown { tag } => write!(s, "unk{}", tag),
                };
            }
            ExprKind::Un(opc, a) => {
                let _ = write!(s, "{}(", kuna_num::opcodes::get_opname(*opc));
                self.fmt_rec(*a, s, depth + 1);
                let _ = write!(s, "):{}", self.size(e));
            }
            ExprKind::Bin(opc, a, b) => {
                let _ = write!(s, "{}(", kuna_num::opcodes::get_opname(*opc));
                self.fmt_rec(*a, s, depth + 1);
                s.push_str(", ");
                self.fmt_rec(*b, s, depth + 1);
                s.push(')');
            }
            ExprKind::SubPiece(a, off) => {
                let _ = write!(s, "SUB{}[", self.size(e));
                self.fmt_rec(*a, s, depth + 1);
                let _ = write!(s, ", {}]", off);
            }
            ExprKind::Piece(a, b) => {
                s.push_str("PIECE(");
                self.fmt_rec(*a, s, depth + 1);
                s.push_str(", ");
                self.fmt_rec(*b, s, depth + 1);
                s.push(')');
            }
            ExprKind::Ite(c, a, b) => {
                s.push_str("ITE(");
                self.fmt_rec(*c, s, depth + 1);
                s.push_str(", ");
                self.fmt_rec(*a, s, depth + 1);
                s.push_str(", ");
                self.fmt_rec(*b, s, depth + 1);
                s.push(')');
            }
        }
    }
}

/// Longest rendering [`ExprArena::fmt`] builds.
const MAX_FMT: usize = 8192;

/// Lanes of the cached quick evaluation ([`ExprArena::quick_vals`]).
const QUICK_LANES: usize = 4;
/// Cached quick evaluations kept before the cache starts over.
const QUICK_MAX: usize = 1 << 22;
/// Sample points of [`ExprArena::identity_probe`]; the first [`ID_QUICK`] are
/// a filter evaluated before the rest.
pub const ID_SAMPLES: usize = 96;
const ID_QUICK: usize = 8;
/// Structured patterns ([`ExprArena::sample_value`]): 4 edges of the whole
/// value, 4 of each 8/16/32/64-bit lane.
const ID_STRUCTURED: usize = 20;
/// Variables with at most this many free bits in total are tested at every assignment.
const ID_EXHAUSTIVE_BITS: u32 = 8;
/// Largest expression (distinct nodes) the identity test evaluates.
const ID_MAX_NODES: usize = 1 << 12;
const ID_SEED: u64 = 0x5eed_9e37_79b9_7f4a;

/// The fixed pseudo-random word of sample `i` for variable `v` (splitmix64).
fn sample_word(v: u64, i: usize) -> u128 {
    let mix = |mut z: u64| {
        z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    let s = ID_SEED ^ v.wrapping_mul(0xd6e8_feb8_6659_fd93) ^ (i as u64).rotate_left(32);
    let lo = mix(s);
    let hi = mix(lo ^ 0xa076_1d64_78bd_642f);
    ((hi as u128) << 64) | lo as u128
}

/// The low bits of `v` spread, in order, over the set bits of `m`.
fn deposit(mut v: u128, mut m: u128) -> u128 {
    let mut out = 0;
    while m != 0 {
        let bit = m & m.wrapping_neg();
        if v & 1 == 1 {
            out |= bit;
        }
        v >>= 1;
        m &= m - 1;
    }
    out
}

#[inline]
fn lowmask(n: u32) -> u128 {
    if n >= 128 { u128::MAX } else { (1u128 << n) - 1 }
}

/// Bits at or below the highest set bit of `c`.
fn lowmask_through(c: u128) -> u128 {
    if c == 0 { 0 } else { lowmask(128 - c.leading_zeros()) }
}

/// Number of significant bits of a mask of possible bits.
fn width_of(possible: u128) -> u32 {
    128 - possible.leading_zeros()
}

/// Known bits of `a + b` (or `a - b`): walk up while the carry is determined.
/// A known-zero addend under a known-zero carry passes the other operand's bit
/// through (the OR-like sums `pushfq` builds), a known-one addend under a
/// known-one carry likewise.
fn add_known(az: u128, ao: u128, bz: u128, bo: u128, sub: bool, size: u8) -> (u128, u128) {
    let n = bits(size);
    let m = mask(size);
    let (bz, bo) = if sub { (bo & m, bz & m) } else { (bz, bo) };
    let mut carry: Option<bool> = Some(sub);
    let mut z = 0u128;
    let mut o = 0u128;
    for i in 0..n {
        let Some(c) = carry else { break };
        let av = if ao >> i & 1 == 1 { Some(true) } else if az >> i & 1 == 1 { Some(false) } else { None };
        let bv = if bo >> i & 1 == 1 { Some(true) } else if bz >> i & 1 == 1 { Some(false) } else { None };
        match (av, bv) {
            (Some(x), Some(y)) => {
                let sum = x ^ y ^ c;
                carry = Some((x & y) | (x & c) | (y & c));
                if sum { o |= 1 << i } else { z |= 1 << i }
            }
            (Some(false), None) | (None, Some(false)) if !c => carry = Some(false),
            (Some(true), None) | (None, Some(true)) if c => carry = Some(true),
            _ => carry = None,
        }
    }
    if !sub {
        let aw = width_of(!az & m);
        let bw = width_of(!bz & m);
        let w = aw.max(bw) + 1;
        if w < n {
            z |= !lowmask(w) & m;
        }
    }
    (z & m, o & m)
}

fn fold_un(opc: OpCode, out: u8, a: u128, asz: u8) -> Option<u128> {
    let m = mask(out);
    Some(match opc {
        CPUI_COPY => a & m,
        CPUI_INT_ZEXT => a & m,
        CPUI_INT_SEXT => (sext(a, asz) as u128) & m,
        CPUI_INT_2COMP => neg(a, out),
        CPUI_INT_NEGATE => !a & m,
        CPUI_BOOL_NEGATE => (a ^ 1) & 1,
        CPUI_POPCOUNT => (a & mask(asz)).count_ones() as u128 & m,
        CPUI_LZCOUNT => ((a & mask(asz)).leading_zeros() - (128 - bits(asz))) as u128 & m,
        _ => return None,
    })
}

fn fold_bin(opc: OpCode, out: u8, a: u128, asz: u8, b: u128, bsz: u8) -> Option<u128> {
    let m = mask(out);
    let ma = mask(asz);
    let a = a & ma;
    let b = b & mask(bsz);
    Some(match opc {
        CPUI_INT_ADD => a.wrapping_add(b) & m,
        CPUI_INT_SUB => a.wrapping_sub(b) & m,
        CPUI_INT_MULT => a.wrapping_mul(b) & m,
        CPUI_INT_AND | CPUI_BOOL_AND => a & b & m,
        CPUI_INT_OR | CPUI_BOOL_OR => (a | b) & m,
        CPUI_INT_XOR | CPUI_BOOL_XOR => (a ^ b) & m,
        CPUI_INT_EQUAL => (a == b) as u128,
        CPUI_INT_NOTEQUAL => (a != b) as u128,
        CPUI_INT_LESS => (a < b) as u128,
        CPUI_INT_LESSEQUAL => (a <= b) as u128,
        CPUI_INT_SLESS => (sext(a, asz) < sext(b, asz)) as u128,
        CPUI_INT_SLESSEQUAL => (sext(a, asz) <= sext(b, asz)) as u128,
        CPUI_INT_CARRY => ((a.wrapping_add(b) & ma) < a) as u128,
        CPUI_INT_SCARRY => {
            let r = a.wrapping_add(b) & ma;
            (sign(a, asz) == sign(b, asz) && sign(r, asz) != sign(a, asz)) as u128
        }
        CPUI_INT_SBORROW => {
            let r = a.wrapping_sub(b) & ma;
            (sign(a, asz) != sign(b, asz) && sign(r, asz) != sign(a, asz)) as u128
        }
        CPUI_INT_LEFT => {
            if b >= bits(out) as u128 { 0 } else { (a << b) & m }
        }
        CPUI_INT_RIGHT => {
            if b >= bits(asz) as u128 { 0 } else { (a >> b) & m }
        }
        CPUI_INT_SRIGHT => {
            if b >= bits(asz) as u128 {
                if sign(a, asz) { m } else { 0 }
            } else {
                ((sext(a, asz) >> b) as u128) & m
            }
        }
        CPUI_INT_DIV => {
            if b == 0 { return None } else { (a / b) & m }
        }
        CPUI_INT_REM => {
            if b == 0 { return None } else { (a % b) & m }
        }
        CPUI_INT_SDIV => {
            if b == 0 { return None } else { (sext(a, asz).wrapping_div(sext(b, bsz)) as u128) & m }
        }
        CPUI_INT_SREM => {
            if b == 0 { return None } else { (sext(a, asz).wrapping_rem(sext(b, bsz)) as u128) & m }
        }
        CPUI_PIECE => ((a << bits(bsz)) | b) & m,
        CPUI_SUBPIECE => (a >> (8 * b.min(16) as u32)) & m,
        _ => return None,
    })
}

#[cfg(test)]
mod tests;
