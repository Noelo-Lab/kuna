//! (kuna `msvcstrappend`) Render an inlined MSVC `std::string` append as one call.
//!
//! MSVC inlines `std::string::push_back` (and `operator+=(char)`, which is
//! `push_back`) and the literal form of `append(const char *, size_t)` into
//! every call site. Each one lands as a capacity diamond over the string object
//! `s` (x64 layout: `_Bx` buffer/pointer at `+0`, `_Mysize` at `+16`, `_Myres` at
//! `+24`):
//!
//! ```text
//! H:  RCX = [s+16]; RDX = [s+24]; if (!(RCX < RDX)) goto G     ; _Mysize < _Myres
//! F:  [s+16] = RCX + 1
//!     RAX = &s; if (RDX > 15) RAX = [s]                        ; _Myptr(): SSO or heap
//!     word [RAX + RCX] = ch                                    ; ch and NUL in one store
//!     goto J
//! G:  R9B = ch; EDX = 1; RCX = &s; call _Reallocate_grow_by
//! J:
//! ```
//!
//! and the C shows the whole diamond once per append, so four appends read as
//! four near-identical `if`/`else` blocks whose count is easy to get wrong.
//!
//! # What is proved
//!
//! Nothing is recognised by name. The pass symbolically executes the raw p-code
//! of the head, both arms and the join edge, and declines unless all of these
//! hold over the SAME object address `A`:
//!
//! * the head loads `_Mysize = [A+16]` and `_Myres = [A+24]` after its last store
//!   or call, and branches to the fast arm exactly when `_Mysize < _Myres`
//!   (`push_back`) or `_Myres - _Mysize >= n` (`append` of `n` bytes). The
//!   branch condition is decided exactly, by evaluation over a test set that is
//!   complete for the comparisons it is allowed to contain;
//! * every feasible path through the fast arm stores `_Mysize + n` to `A+16`,
//!   stores exactly the bytes `ch...` and a terminating zero at `ptr[_Mysize]`,
//!   and nothing else; `ptr` is `A` on every path taken only when `_Myres <= 15`
//!   and `[A]` on every path taken only when `_Myres > 15`, and both kinds of path
//!   exist. That is `_Myptr()` with 15 as the small-string capacity;
//! * the grow arm is one direct call, with no store but (for `append`) the
//!   outgoing count, receiving `A` in `RCX` and the same character in `R9B`
//!   (`push_back`, with `RDX` either untouched or 1), or `n` in `RDX` and a
//!   pointer in `R9` into read-only memory whose image bytes are the stored bytes
//!   (`append`);
//! * no register either arm writes is one the default model preserves, so each
//!   is already undefined at the join on the grow path;
//! * no op after the join reads a register byte the rewrite changes before
//!   overwriting it, on any path up to the next call or return
//!   ([`read_after_join`]). A call ends the walk because every such register is
//!   one the callee may clobber; a return ends it because the only one a caller
//!   may read is `RAX`, which is changed only when the call does not return
//!   `this`.
//!
//! # The rewrite
//!
//! The head's conditional branch and fast-arm edge are removed, so the fast arm
//! becomes unreachable and is collected. The grow block keeps its call
//! instruction (and so its return-address push and stack effect) and loses its
//! argument setup, which is replaced by `RCX = A` and the character (or the
//! literal pointer and count). The call gets a fresh call spec with a locked
//! prototype, `void std::string::push_back(void *this, char c)` or
//! `std::string::append(void *this, char *s, size_t n)`, returning `this` only
//! when every fast path also left `A` in `RAX`. Its entry address is left invalid
//! because the call stands for no single body. A register the model kills at a
//! call but no fast path writes is declared preserved by it: that is its value on
//! the fast path, and a definite kill on every path would erase it from the data
//! flow parameter recovery reads at later calls.
//!
//! It runs in the universal group next to `linuxsyscall`, before
//! `ActionDefaultParams` and `ActionFuncLink`, so the raw p-code is still
//! register- and `LOAD`/`STORE`-level and the locked prototype is materialized
//! like any other.
//!
//! x86-64 only: the 32-bit MSVC layout (`_Mysize` at `+16` as four bytes,
//! `__thiscall` with stacked arguments) is not matched.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, ParameterPieces, ProtoModel};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// The printed name of a recovered `push_back`.
pub const PUSH_BACK_NAME: &str = "std::string::push_back";
/// The printed name of a recovered literal `append`.
pub const APPEND_NAME: &str = "std::string::append";

const WORD: int4 = 8;
const SSO_CAPACITY: u64 = 15;
const SIZE_FIELD: u64 = 16;
const RES_FIELD: u64 = 24;
const STACK_ARG_FIVE: u64 = 0x20;
const MAX_APPEND_BYTES: u64 = 15;
const MAX_ARM_DEPTH: usize = 6;
const MAX_ARM_PATHS: usize = 8;
const MAX_ARM_OPS: usize = 96;
const MAX_HEAD_OPS: usize = 4096;
const MAX_SCAN_OPS: usize = 4096;
const MAX_SITES: usize = 4096;

fn mask(size: int4) -> u64 {
    if size >= 8 {
        u64::MAX
    } else if size <= 0 {
        0
    } else {
        (1u64 << (8 * size)) - 1
    }
}

/// A value as a linear combination of opaque atoms plus a constant, modulo the
/// value's byte size. Every atom in a value is at least as wide as the value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Val {
    terms: Vec<(u32, u64)>,
    k: u64,
    size: int4,
}

impl Val {
    fn konst(k: u64, size: int4) -> Val {
        Val { terms: Vec::new(), k: k & mask(size), size }
    }

    fn as_const(&self) -> Option<u64> {
        self.terms.is_empty().then_some(self.k)
    }

    fn resized(terms: Vec<(u32, u64)>, k: u64, size: int4) -> Val {
        let m = mask(size);
        let terms = terms.into_iter().map(|(a, c)| (a, c & m)).filter(|&(_, c)| c != 0).collect();
        Val { terms, k: k & m, size }
    }

    fn add(&self, o: &Val) -> Val {
        let mut acc: BTreeMap<u32, u64> = BTreeMap::new();
        for &(a, c) in self.terms.iter().chain(o.terms.iter()) {
            let e = acc.entry(a).or_insert(0);
            *e = e.wrapping_add(c);
        }
        Val::resized(acc.into_iter().collect(), self.k.wrapping_add(o.k), self.size)
    }

    fn scale(&self, c: u64) -> Val {
        let terms = self.terms.iter().map(|&(a, x)| (a, x.wrapping_mul(c))).collect();
        Val::resized(terms, self.k.wrapping_mul(c), self.size)
    }

    fn neg(&self) -> Val {
        self.scale(u64::MAX)
    }

    fn sub(&self, o: &Val) -> Val {
        self.add(&o.neg())
    }

    fn plus(&self, c: u64) -> Val {
        Val::resized(self.terms.clone(), self.k.wrapping_add(c), self.size)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Atom {
    /// Register or temporary storage never written since the head began or
    /// since the last call (`epoch`).
    Leaf { space: int4, off: u64, size: int4, epoch: u32 },
    /// A `LOAD` from the data space at memory version `ver`.
    Load { addr: Val, size: int4, ver: u32 },
    Op { code: OpCode, ins: Vec<Val>, size: int4 },
    Fresh(u32),
}

#[derive(Default)]
struct Syms {
    atoms: Vec<Atom>,
    index: BTreeMap<Atom, u32>,
    fresh: u32,
}

impl Syms {
    fn intern(&mut self, a: Atom) -> u32 {
        if let Some(&id) = self.index.get(&a) {
            return id;
        }
        let id = self.atoms.len() as u32;
        self.atoms.push(a.clone());
        self.index.insert(a, id);
        id
    }

    fn lookup(&self, a: &Atom) -> Option<u32> {
        self.index.get(a).copied()
    }

    fn atom(&mut self, a: Atom, size: int4) -> Val {
        Val { terms: vec![(self.intern(a), 1)], k: 0, size }
    }

    fn id_val(id: u32, size: int4) -> Val {
        Val { terms: vec![(id, 1)], k: 0, size }
    }

    fn fresh(&mut self, size: int4) -> Val {
        self.fresh += 1;
        let f = self.fresh;
        self.atom(Atom::Fresh(f), size)
    }

    fn single(&self, v: &Val) -> Option<&Atom> {
        match v.terms.as_slice() {
            [(id, 1)] if v.k == 0 => self.atoms.get(*id as usize),
            _ => None,
        }
    }

    /// Truncate to the low `size` bytes, looking through zero extensions that are
    /// at least that wide so a register read back at its narrow width matches
    /// the value that was extended into it.
    fn truncate(&self, v: &Val, size: int4) -> Val {
        let mut acc = Val::konst(v.k, size);
        for &(id, c) in &v.terms {
            let piece = match self.atoms.get(id as usize) {
                Some(Atom::Op { code: OpCode::CPUI_INT_ZEXT, ins, .. }) if ins[0].size >= size => {
                    self.truncate(&ins[0], size)
                }
                _ => Syms::id_val(id, size),
            };
            acc = acc.add(&piece.scale(c));
        }
        acc
    }

    fn subpiece(&mut self, v: Val, byte_off: u64, size: int4) -> Val {
        if byte_off == 0 && size == v.size {
            return v;
        }
        if let Some(c) = v.as_const() {
            let shifted = if byte_off >= 8 { 0 } else { c >> (8 * byte_off) };
            return Val::konst(shifted, size);
        }
        if byte_off == 0 && size < v.size {
            return self.truncate(&v, size);
        }
        let off = Val::konst(byte_off, 4);
        self.atom(Atom::Op { code: OpCode::CPUI_SUBPIECE, ins: vec![v, off], size }, size)
    }

    fn op(&mut self, code: OpCode, ins: Vec<Val>, size: int4) -> Val {
        self.atom(Atom::Op { code, ins, size }, size)
    }

    fn eval(&mut self, code: OpCode, ins: Vec<Val>, size: int4) -> Val {
        use OpCode::*;
        let c0 = ins.first().and_then(Val::as_const);
        let c1 = ins.get(1).and_then(Val::as_const);
        match code {
            CPUI_COPY if ins.len() == 1 => {
                let v = &ins[0];
                Val::resized(v.terms.clone(), v.k, size)
            }
            CPUI_INT_ADD if ins.len() == 2 => ins[0].add(&ins[1]),
            CPUI_INT_SUB if ins.len() == 2 => ins[0].sub(&ins[1]),
            CPUI_INT_2COMP if ins.len() == 1 => ins[0].neg(),
            CPUI_INT_MULT if ins.len() == 2 => match (c0, c1) {
                (_, Some(c)) => ins[0].scale(c),
                (Some(c), _) => ins[1].scale(c),
                _ => self.op(code, ins, size),
            },
            CPUI_INT_ZEXT if ins.len() == 1 => match c0 {
                Some(c) => Val::konst(c, size),
                None => self.op(code, ins, size),
            },
            CPUI_SUBPIECE if ins.len() == 2 => match c1 {
                Some(off) => {
                    let v = ins[0].clone();
                    self.subpiece(v, off, size)
                }
                None => self.fresh(size),
            },
            CPUI_BOOL_NEGATE if ins.len() == 1 => {
                if let Some(c) = c0 {
                    return Val::konst((c ^ 1) & 1, size);
                }
                if let Some(Atom::Op { code: CPUI_BOOL_NEGATE, ins: inner, .. }) = self.single(&ins[0]) {
                    return inner[0].clone();
                }
                self.op(code, ins, size)
            }
            _ => match (c0, c1, ins.len()) {
                (Some(a), Some(b), 2) => match concrete_binary(code, a, b, ins[0].size) {
                    Some(r) => Val::konst(r, size),
                    None => self.op(code, ins, size),
                },
                _ => self.op(code, ins, size),
            },
        }
    }
}

fn concrete_binary(code: OpCode, a: u64, b: u64, insize: int4) -> Option<u64> {
    use OpCode::*;
    let m = mask(insize);
    let (a, b) = (a & m, b & m);
    Some(match code {
        CPUI_INT_EQUAL => (a == b) as u64,
        CPUI_INT_NOTEQUAL => (a != b) as u64,
        CPUI_INT_LESS => (a < b) as u64,
        CPUI_INT_LESSEQUAL => (a <= b) as u64,
        CPUI_BOOL_AND | CPUI_INT_AND => a & b,
        CPUI_BOOL_OR | CPUI_INT_OR => a | b,
        CPUI_BOOL_XOR | CPUI_INT_XOR => a ^ b,
        _ => return None,
    })
}

/// The variables a branch condition may be decided over.
enum Spec {
    /// One value compared against constants.
    One(Val),
    /// Two atoms related only to each other.
    Two(u32, u32),
}

impl Syms {
    /// Is `v` built only from comparisons whose truth the test set of
    /// [`Syms::test_points`] decides exactly? Collects the comparison constants.
    /// Every value it admits is 0 or 1.
    fn lang_ok(&self, v: &Val, spec: &Spec, bps: &mut Vec<u64>) -> bool {
        use OpCode::*;
        if let Some(c) = v.as_const() {
            return c <= 1;
        }
        let Some(Atom::Op { code, ins, .. }) = self.single(v) else { return false };
        match *code {
            CPUI_BOOL_NEGATE => self.lang_ok(&ins[0], spec, bps),
            CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_BOOL_XOR => {
                self.lang_ok(&ins[0], spec, bps) && self.lang_ok(&ins[1], spec, bps)
            }
            CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_XOR if v.size == 1 => {
                ins.iter().all(|i| i.size == 1) && self.lang_ok(&ins[0], spec, bps) && self.lang_ok(&ins[1], spec, bps)
            }
            CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL => {
                let d = ins[0].sub(&ins[1]);
                if d.terms.is_empty() {
                    return true;
                }
                match spec {
                    Spec::One(x) => {
                        if d.size != x.size {
                            return false;
                        }
                        if d.terms == x.terms {
                            bps.push(x.k.wrapping_sub(d.k) & mask(x.size));
                            true
                        } else if d.terms == x.neg().terms {
                            bps.push(x.k.wrapping_add(d.k) & mask(x.size));
                            true
                        } else {
                            false
                        }
                    }
                    Spec::Two(s, r) => {
                        let m = mask(d.size);
                        let a = vec![(*s.min(r), 1u64), (*s.max(r), m)];
                        let b = vec![(*s.min(r), m), (*s.max(r), 1u64)];
                        d.size == WORD && d.k == 0 && (d.terms == a || d.terms == b)
                    }
                }
            }
            CPUI_INT_LESS | CPUI_INT_LESSEQUAL => match spec {
                Spec::One(x) => match (ins[0].as_const(), ins[1].as_const()) {
                    (Some(_), Some(_)) => true,
                    (None, Some(c)) if ins[0] == *x => {
                        bps.push(c);
                        true
                    }
                    (Some(c), None) if ins[1] == *x => {
                        bps.push(c);
                        true
                    }
                    _ => false,
                },
                Spec::Two(s, r) => {
                    let sv = Syms::id_val(*s, WORD);
                    let rv = Syms::id_val(*r, WORD);
                    (ins[0] == sv && ins[1] == rv) || (ins[0] == rv && ins[1] == sv)
                }
            },
            _ => false,
        }
    }

    fn conc(&self, v: &Val, env: &Env) -> Option<u64> {
        use OpCode::*;
        let m = mask(v.size);
        if let Some(c) = v.as_const() {
            return Some(c);
        }
        match env {
            Env::One(x, xv) => {
                let base = xv.wrapping_sub(x.k);
                if v.terms == x.terms {
                    return Some(base.wrapping_add(v.k) & m);
                }
                if v.terms == x.neg().terms {
                    return Some(base.wrapping_neg().wrapping_add(v.k) & m);
                }
            }
            Env::Two(s, sv, r, rv) => {
                if v.terms.iter().all(|(a, _)| a == s || a == r) {
                    let mut acc = v.k;
                    for &(a, c) in &v.terms {
                        let x = if a == *s { *sv } else { *rv };
                        acc = acc.wrapping_add(c.wrapping_mul(x));
                    }
                    return Some(acc & m);
                }
            }
        }
        let Some(Atom::Op { code, ins, .. }) = self.single(v) else { return None };
        let a = self.conc(&ins[0], env)?;
        if *code == CPUI_BOOL_NEGATE {
            return Some((a ^ 1) & 1);
        }
        let b = self.conc(ins.get(1)?, env)?;
        concrete_binary(*code, a, b, ins[0].size)
    }

    /// The test values that decide a one-variable condition whose comparison
    /// constants are `bps`: the ends of the range and each constant with its two
    /// neighbours. The truth of such a condition only changes at a constant.
    fn test_points(x: &Val, bps: &[u64]) -> Vec<u64> {
        let m = mask(x.size);
        let mut pts: BTreeSet<u64> = BTreeSet::new();
        pts.insert(0);
        pts.insert(m);
        for &c in bps {
            pts.insert(c & m);
            pts.insert(c.wrapping_sub(1) & m);
            pts.insert(c.wrapping_add(1) & m);
        }
        pts.into_iter().collect()
    }

    /// For each test value of `x`, whether every condition holds with its
    /// recorded sense. `None` when a condition leaves the decidable language.
    fn classify_one(&self, conds: &[(Val, bool)], x: &Val, extra: &[u64]) -> Option<Vec<(u64, bool)>> {
        let spec = Spec::One(x.clone());
        let mut bps: Vec<u64> = extra.to_vec();
        for (c, _) in conds {
            if c.size != 1 || !self.lang_ok(c, &spec, &mut bps) {
                return None;
            }
        }
        let mut out = Vec::new();
        for xv in Syms::test_points(x, &bps) {
            let env = Env::One(x.clone(), xv);
            let mut all = true;
            for (c, taken) in conds {
                let t = self.conc(c, &env)? & 1 == 1;
                all &= t == *taken;
            }
            out.push((xv, all));
        }
        Some(out)
    }

    /// Does the condition, with sense `taken`, equal `target` over every ordering
    /// of the two atoms? Exact because the language only relates them to each
    /// other.
    fn decide_two(&self, cond: &Val, taken: bool, s: u32, r: u32, target: fn(u64, u64) -> bool) -> bool {
        let spec = Spec::Two(s, r);
        if cond.size != 1 || !self.lang_ok(cond, &spec, &mut Vec::new()) {
            return false;
        }
        [(0u64, 1u64), (1, 1), (1, 0)].iter().all(|&(sv, rv)| {
            match self.conc(cond, &Env::Two(s, sv, r, rv)) {
                Some(v) => ((v & 1 == 1) == taken) == target(sv, rv),
                None => false,
            }
        })
    }
}

enum Env {
    One(Val, u64),
    Two(u32, u64, u32, u64),
}

/// x86-64 storage the matcher and the rewrite name, resolved only under a
/// Win64-shaped default model (`RSI`/`RDI` preserved). `bases` and `bytes` are
/// the registers an argument may be copied from; they exclude `RCX`, `RDX` and
/// `R8`, which the rewrite writes.
struct Regs {
    reg_space: int4,
    ram: Rc<AddrSpace>,
    rax: Address,
    rcx: Address,
    rdx: Address,
    r8: Address,
    r9: Address,
    rsp: Address,
    bases: Vec<Address>,
    bytes: Vec<Address>,
    model: Rc<ProtoModel>,
}

impl Regs {
    fn resolve(data: &Funcdata) -> Option<Regs> {
        let manage = data.get_arch().manage();
        if manage.get_default_code_space()?.get_addr_size() != WORD as u32 {
            return None;
        }
        let ram = manage.get_default_data_space()?.clone();
        let lookup = manage.register_lookup()?;
        let reg = |nm: &str, size: u32| -> Option<Address> {
            let st = lookup.probe_register(nm)?;
            if st.size != size {
                return None;
            }
            Some(Address::new(st.space.clone()?, st.offset))
        };
        let rax = reg("RAX", 8)?;
        let reg_space = rax.get_space()?.get_index();
        let mut bases = Vec::new();
        for nm in ["RSP", "RBP", "RBX", "RSI", "RDI", "R12", "R13", "R14", "R15", "RAX", "R9", "R10", "R11"] {
            bases.push(reg(nm, 8)?);
        }
        let mut bytes = Vec::new();
        for nm in ["AL", "BL", "SIL", "DIL", "BPL", "R9B", "R10B", "R11B", "R12B", "R13B", "R14B", "R15B"] {
            bytes.push(reg(nm, 1)?);
        }
        let model = data.get_arch().eval_fp_called()?.clone();
        let space = rax.get_space()?.clone();
        for nm in ["RSI", "RDI"] {
            let st = lookup.probe_register(nm)?;
            let at = Address::new(space.clone(), st.offset);
            if model.has_effect(&at, st.size as int4) != crate::fspec::effect_type::UNAFFECTED {
                return None;
            }
        }
        Some(Regs {
            reg_space,
            ram,
            rcx: reg("RCX", 8)?,
            rdx: reg("RDX", 8)?,
            r8: reg("R8", 8)?,
            r9: reg("R9", 8)?,
            rsp: reg("RSP", 8)?,
            rax,
            bases,
            bytes,
            model,
        })
    }
}

#[derive(Clone)]
struct StoreFx {
    addr: Val,
    val: Val,
    size: int4,
}

#[derive(Clone, Default)]
struct State {
    reg: Vec<(int4, u64, int4, Val)>,
    epoch: u32,
    ver: u32,
    stores: Vec<StoreFx>,
    calls: usize,
    conds: Vec<(Val, bool)>,
    written: BTreeSet<(u64, int4)>,
    bad: bool,
}

struct Engine<'a> {
    data: &'a Funcdata,
    regs: &'a Regs,
    syms: Syms,
}

impl<'a> Engine<'a> {
    fn read_storage(&mut self, st: &State, space: int4, off: u64, size: int4) -> Val {
        for (s, o, z, v) in st.reg.iter().rev() {
            if *s != space {
                continue;
            }
            let (we, re) = (o + *z as u64, off + size as u64);
            if off >= *o && re <= we {
                let v = v.clone();
                return self.syms.subpiece(v, off - o, size);
            }
            if off < we && *o < re {
                return self.syms.fresh(size);
            }
        }
        self.syms.atom(Atom::Leaf { space, off, size, epoch: st.epoch }, size)
    }

    fn read_reg(&mut self, st: &State, addr: &Address, size: int4) -> Val {
        self.read_storage(st, self.regs.reg_space, addr.get_offset(), size)
    }

    fn space_kind(&self, vn: VarnodeId) -> Option<(bool, int4, u64, int4)> {
        let v = self.data.vbank().get(vn)?;
        let sp = v.get_addr().get_space()?;
        let local = sp.get_index() == self.regs.reg_space || sp.get_type() == spacetype::IPTR_INTERNAL;
        Some((local, sp.get_index(), v.get_offset(), v.get_size()))
    }

    fn read_vn(&mut self, st: &State, vn: VarnodeId) -> Val {
        let Some(v) = self.data.vbank().get(vn) else { return self.syms.fresh(1) };
        let size = v.get_size();
        if v.is_constant() {
            return Val::konst(v.get_offset(), size);
        }
        match self.space_kind(vn) {
            Some((true, space, off, size)) => self.read_storage(st, space, off, size),
            _ => self.syms.fresh(size),
        }
    }

    fn write_vn(&mut self, st: &mut State, vn: VarnodeId, val: Val) {
        match self.space_kind(vn) {
            Some((true, space, off, size)) => {
                let end = off + size as u64;
                st.reg.retain(|(s, o, z, _)| !(*s == space && *o >= off && o + *z as u64 <= end));
                st.reg.push((space, off, size, val));
                if space == self.regs.reg_space {
                    st.written.insert((off, size));
                }
            }
            _ => st.bad = true,
        }
    }

    fn data_space(&self, vn: Option<VarnodeId>) -> bool {
        let Some(v) = vn.and_then(|vn| self.data.vbank().get(vn)) else { return false };
        v.is_constant() && v.get_offset() == self.regs.ram.get_index() as u64
    }

    fn exec_op(&mut self, st: &mut State, op: OpId) {
        use OpCode::*;
        let Some(o) = self.data.obank().get(op) else {
            st.bad = true;
            return;
        };
        let code = o.code();
        let out = o.get_out();
        let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
        match code {
            CPUI_LOAD => {
                let Some(out) = out else { return };
                let size = self.data.vbank().get(out).map(|v| v.get_size()).unwrap_or(1);
                let val = if ins.len() == 2 && self.data_space(Some(ins[0])) {
                    let addr = self.read_vn(st, ins[1]);
                    self.syms.atom(Atom::Load { addr, size, ver: st.ver }, size)
                } else {
                    self.syms.fresh(size)
                };
                self.write_vn(st, out, val);
            }
            CPUI_STORE => {
                if ins.len() != 3 || !self.data_space(Some(ins[0])) {
                    st.bad = true;
                    return;
                }
                let addr = self.read_vn(st, ins[1]);
                let val = self.read_vn(st, ins[2]);
                let size = val.size;
                st.stores.push(StoreFx { addr, val, size });
                st.ver += 1;
            }
            CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER => {
                st.calls += 1;
                st.ver += 1;
                st.epoch += 1;
                st.reg.clear();
                if let Some(out) = out {
                    let size = self.data.vbank().get(out).map(|v| v.get_size()).unwrap_or(1);
                    let f = self.syms.fresh(size);
                    self.write_vn(st, out, f);
                }
            }
            CPUI_BRANCH => {}
            CPUI_CBRANCH | CPUI_BRANCHIND | CPUI_RETURN | CPUI_MULTIEQUAL | CPUI_INDIRECT => st.bad = true,
            _ => {
                let Some(out) = out else { return };
                let size = self.data.vbank().get(out).map(|v| v.get_size()).unwrap_or(1);
                let vals: Vec<Val> = ins.iter().map(|&vn| self.read_vn(st, vn)).collect();
                let val = self.syms.eval(code, vals, size);
                self.write_vn(st, out, val);
            }
        }
    }

    /// Execute a block, optionally from just after its last call (a call clears
    /// every register, so nothing before it is observable); returns the
    /// condition of a tail `CBRANCH`.
    fn exec_block(&mut self, st: &mut State, bl: BlockId, from_last_call: bool) -> Option<Val> {
        use OpCode::*;
        let ops = self.data.bb_ops(bl);
        let last_call =
            ops.iter().rposition(|&o| matches!(block_code(self.data, o), Some(CPUI_CALL | CPUI_CALLIND | CPUI_CALLOTHER)));
        let start = match last_call {
            Some(i) if from_last_call => {
                st.epoch += 1;
                st.ver += 1;
                st.reg.clear();
                i + 1
            }
            _ => 0,
        };
        if ops.len() - start > MAX_HEAD_OPS {
            st.bad = true;
            return None;
        }
        let mut cond = None;
        for (i, &op) in ops.iter().enumerate().skip(start) {
            let is_cbranch = self.data.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_CBRANCH);
            if is_cbranch && i + 1 == ops.len() {
                let o = self.data.obank().get(op)?;
                if o.is_boolean_flip() {
                    st.bad = true;
                    return None;
                }
                let c = o.get_in(1)?;
                cond = Some(self.read_vn(st, c));
            } else {
                self.exec_op(st, op);
            }
        }
        cond
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Src {
    Const(u64),
    Reg(Address, u64),
}

#[derive(Clone, Debug, PartialEq)]
enum Kind {
    PushBack(Src),
    Append { n: u64, lit: u64 },
}

struct Site {
    head: BlockId,
    grow: BlockId,
    fast_slot: int4,
    call: OpId,
    first_call_ix: usize,
    this: Src,
    kind: Kind,
    returns_this: bool,
    fast_written: BTreeSet<(u64, int4)>,
}

fn block_code(data: &Funcdata, op: OpId) -> Option<OpCode> {
    data.obank().get(op).map(|o| o.code())
}

/// Every path from `f0` to `join` through a single-entry fast arm, or `None`.
fn fast_paths(data: &Funcdata, head: BlockId, grow: BlockId, join: BlockId, f0: BlockId) -> Option<Vec<Vec<BlockId>>> {
    let g = data.bblocks_ref();
    let mut paths: Vec<Vec<BlockId>> = Vec::new();
    let mut stack: Vec<Vec<BlockId>> = vec![vec![f0]];
    while let Some(path) = stack.pop() {
        let bl = *path.last()?;
        if bl == head || bl == grow || path.len() > MAX_ARM_DEPTH {
            return None;
        }
        let b = g.block(bl);
        if b.size_out() == 0 || b.size_out() > 2 || (b.size_out() == 2 && b.get_out(0) == b.get_out(1)) {
            return None;
        }
        for i in 0..b.size_out() {
            let nx = b.get_out(i);
            if nx == join {
                paths.push(path.clone());
            } else if path.contains(&nx) {
                return None;
            } else {
                let mut p = path.clone();
                p.push(nx);
                stack.push(p);
            }
            if paths.len() + stack.len() > MAX_ARM_PATHS {
                return None;
            }
        }
    }
    let arm: BTreeSet<BlockId> = paths.iter().flatten().copied().collect();
    for &bl in &arm {
        let b = g.block(bl);
        for i in 0..b.size_in() {
            let p = b.get_in(i);
            if !(arm.contains(&p) || (p == head && bl == f0)) {
                return None;
            }
        }
        let n = data.bb_ops(bl).len();
        if n > MAX_ARM_OPS {
            return None;
        }
    }
    (!paths.is_empty()).then_some(paths)
}

/// One fast path's proven effect: the bytes written at `ptr[_Mysize]` and
/// whether the path is the small-string one.
struct FastPath {
    bytes: Vec<Val>,
    sso: bool,
    rax: Val,
}

fn prove_fast_path(
    eng: &mut Engine,
    st: &State,
    base_ver: u32,
    a: &Val,
    size_id: u32,
    res_id: u32,
    n: u64,
) -> Option<Option<FastPath>> {
    let res = Syms::id_val(res_id, WORD);
    let points = eng.syms.classify_one(&st.conds[1..], &res, &[SSO_CAPACITY, SSO_CAPACITY + 1])?;
    let feasible: Vec<u64> = points.iter().filter(|(_, t)| *t).map(|(v, _)| *v).collect();
    if feasible.is_empty() {
        return Some(None);
    }
    let sso = if feasible.iter().all(|&v| v <= SSO_CAPACITY) {
        true
    } else if feasible.iter().all(|&v| v > SSO_CAPACITY) {
        false
    } else {
        return None;
    };
    let size = Syms::id_val(size_id, WORD);
    let size_slot = a.plus(SIZE_FIELD);
    let new_size = size.plus(n);
    let mut size_stores = 0;
    let mut bytes: BTreeMap<u64, Val> = BTreeMap::new();
    let mut heap_ptr: Option<u32> = None;
    for (idx, s) in st.stores.iter().enumerate() {
        if s.addr == size_slot {
            if s.size != WORD || s.val != new_size {
                return None;
            }
            size_stores += 1;
            continue;
        }
        let rel = s.addr.sub(&size);
        let i = if sso {
            if rel.terms != a.terms {
                return None;
            }
            rel.k.wrapping_sub(a.k)
        } else {
            let [(pid, 1)] = rel.terms.as_slice() else { return None };
            let Some(Atom::Load { addr, size: 8, ver }) = eng.syms.atoms.get(*pid as usize) else { return None };
            if addr != a || heap_ptr.is_some_and(|p| p != *pid) {
                return None;
            }
            let Some(before) = ver.checked_sub(base_ver).map(|b| b as usize) else { return None };
            if before > 1 || before > idx || st.stores[..before].iter().any(|b| b.addr != size_slot) {
                return None;
            }
            heap_ptr = Some(*pid);
            rel.k
        };
        if i > n {
            return None;
        }
        match s.val.as_const() {
            Some(c) => {
                for j in 0..s.size as u64 {
                    let b = if j >= 8 { 0 } else { (c >> (8 * j)) & 0xff };
                    if bytes.insert(i + j, Val::konst(b, 1)).is_some() {
                        return None;
                    }
                }
            }
            None if s.size == 1 => {
                if bytes.insert(i, s.val.clone()).is_some() {
                    return None;
                }
            }
            None => return None,
        }
    }
    if size_stores != 1 || bytes.len() as u64 != n + 1 || bytes.keys().copied().ne(0..=n) {
        return None;
    }
    if bytes.get(&n) != Some(&Val::konst(0, 1)) {
        return None;
    }
    let rax_addr = eng.regs.rax.clone();
    let rax = eng.read_reg(st, &rax_addr, WORD);
    Some(Some(FastPath { bytes: (0..n).map(|i| bytes[&i].clone()).collect(), sso, rax }))
}

/// May a call change this register? Anything the default model does not promise
/// to preserve: the Win64 cspec lists only `RAX` and `XMM0` as killed and leaves
/// the other volatile registers and the status flags unmentioned, which heritage
/// already treats as possibly clobbered at the grow arm's real call.
fn volatile(regs: &Regs, off: u64, size: int4) -> bool {
    use crate::fspec::effect_type::{RETURN_ADDRESS, UNAFFECTED};
    let rsp = regs.rsp.get_offset();
    if off < rsp + WORD as u64 && rsp < off + size as u64 {
        return false;
    }
    let addr = Address::new(regs.rax.get_space().expect("register space").clone(), off);
    !matches!(regs.model.has_effect(&addr, size), UNAFFECTED | RETURN_ADDRESS)
}

fn register_bytes(data: &Funcdata, regs: &Regs, vn: VarnodeId) -> Option<std::ops::Range<u64>> {
    let v = data.vbank().get(vn)?;
    let space = v.get_addr().get_space()?;
    (space.get_index() == regs.reg_space).then(|| v.get_offset()..v.get_offset() + v.get_size() as u64)
}

/// Does an op on some path from `join` read a byte of `live` before overwriting
/// it and before the path reaches a call or a return? An indirect branch with a
/// byte still live, or a walk longer than [`MAX_SCAN_OPS`], also answers yes. An
/// op that combines a register with itself into a value independent of it
/// (`xor ecx,ecx`, `sub eax,eax` and their flags) is not a read.
fn read_after_join(data: &Funcdata, regs: &Regs, join: BlockId, live: BTreeSet<u64>) -> bool {
    use OpCode::*;
    let g = data.bblocks_ref();
    let mut explored: BTreeMap<BlockId, BTreeSet<u64>> = BTreeMap::new();
    let mut work = vec![(join, live)];
    let mut budget = MAX_SCAN_OPS;
    while let Some((bl, mut live)) = work.pop() {
        let seen = explored.entry(bl).or_default();
        if live.is_subset(seen) {
            continue;
        }
        seen.extend(live.iter().copied());
        let mut path_ends = false;
        for op in data.bb_ops(bl) {
            let Some(o) = data.obank().get(op) else { return true };
            budget = match budget.checked_sub(1) {
                Some(b) => b,
                None => return true,
            };
            let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
            let self_cancel = matches!(
                o.code(),
                CPUI_INT_XOR
                    | CPUI_BOOL_XOR
                    | CPUI_INT_SUB
                    | CPUI_INT_EQUAL
                    | CPUI_INT_NOTEQUAL
                    | CPUI_INT_LESS
                    | CPUI_INT_LESSEQUAL
                    | CPUI_INT_SLESS
                    | CPUI_INT_SLESSEQUAL
                    | CPUI_INT_SBORROW
            ) && ins.len() == 2
                && register_bytes(data, regs, ins[0]).is_some()
                && register_bytes(data, regs, ins[0]) == register_bytes(data, regs, ins[1]);
            if !self_cancel
                && ins.iter().filter_map(|&vn| register_bytes(data, regs, vn)).any(|mut r| r.any(|b| live.contains(&b)))
            {
                return true;
            }
            match o.code() {
                CPUI_CALL | CPUI_CALLIND | CPUI_RETURN => {
                    path_ends = true;
                    break;
                }
                CPUI_BRANCHIND => return true,
                _ => {}
            }
            if let Some(written) = o.get_out().and_then(|out| register_bytes(data, regs, out)) {
                for b in written {
                    live.remove(&b);
                }
            }
            if live.is_empty() {
                path_ends = true;
                break;
            }
        }
        if !path_ends {
            let b = g.block(bl);
            for i in 0..b.size_out() {
                work.push((b.get_out(i), live.clone()));
            }
        }
    }
    false
}

fn materialize(eng: &mut Engine, st: &State, want: &Val, cands: &[Address]) -> Option<Src> {
    if let Some(c) = want.as_const() {
        return Some(Src::Const(c));
    }
    for r in cands {
        let v = eng.read_reg(st, r, want.size);
        if v.terms == want.terms {
            return Some(Src::Reg(r.clone(), want.k.wrapping_sub(v.k) & mask(want.size)));
        }
    }
    None
}

fn try_match(data: &Funcdata, regs: &Regs, call: OpId) -> Option<Site> {
    let cop = data.obank().get(call)?;
    if cop.code() != OpCode::CPUI_CALL {
        return None;
    }
    let grow = cop.get_parent()?;
    let call_addr = cop.get_addr().clone();
    let g = data.bblocks_ref();
    let gb = g.block(grow);
    if gb.size_in() != 1 || gb.size_out() != 1 {
        return None;
    }
    let head = gb.get_in(0);
    let join = gb.get_out(0);
    if head == grow || join == grow || join == head {
        return None;
    }
    let hb = g.block(head);
    if hb.size_out() != 2 || block_code(data, data.bb_op_tail(head)?) != Some(OpCode::CPUI_CBRANCH) {
        return None;
    }
    let (fast_slot, f0) = match (hb.get_out(0), hb.get_out(1)) {
        (a, b) if b == grow && a != grow => (0, a),
        (a, b) if a == grow && b != grow => (1, b),
        _ => return None,
    };
    if f0 == join || g.block(f0).size_in() != 1 {
        return None;
    }
    let fast_taken = hb.get_true_out() == f0;
    if !fast_taken && hb.get_false_out() != f0 {
        return None;
    }

    let gops = data.bb_ops(grow);
    if gops.len() > MAX_ARM_OPS {
        return None;
    }
    let call_ix = gops.iter().position(|&o| o == call)?;
    if gops[call_ix + 1..].iter().any(|&o| block_code(data, o) != Some(OpCode::CPUI_BRANCH)) {
        return None;
    }
    let first_call_ix =
        gops.iter().position(|&o| data.obank().get(o).is_some_and(|x| x.get_addr() == &call_addr))?;

    let paths = fast_paths(data, head, grow, join, f0)?;
    let arm_stores: usize = paths
        .iter()
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|&bl| data.bb_ops(bl).iter().filter(|&&o| block_code(data, o) == Some(OpCode::CPUI_STORE)).count())
        .sum();
    if arm_stores < 2 {
        return None;
    }

    let mut eng = Engine { data, regs, syms: Syms::default() };
    let mut sh = State::default();
    let cond_h = eng.exec_block(&mut sh, head, true)?;
    if sh.bad {
        return None;
    }
    let base_ver = sh.ver;
    sh.written.clear();
    sh.stores.clear();

    let mut sg = sh.clone();
    for &o in &gops[..first_call_ix] {
        eng.exec_op(&mut sg, o);
    }
    if sg.bad || sg.calls != 0 {
        return None;
    }

    let mut ends: Vec<State> = Vec::new();
    for path in &paths {
        let mut st = sh.clone();
        st.conds.push((cond_h.clone(), fast_taken));
        for (i, &bl) in path.iter().enumerate() {
            let next = path.get(i + 1).copied().unwrap_or(join);
            let cond = eng.exec_block(&mut st, bl, false);
            if st.bad || st.calls != 0 {
                return None;
            }
            let b = data.bblocks_ref().block(bl);
            match cond {
                Some(c) => {
                    let t = b.get_true_out() == next;
                    if !t && b.get_false_out() != next {
                        return None;
                    }
                    st.conds.push((c, t));
                }
                None if b.size_out() == 1 => {}
                None => return None,
            }
        }
        ends.push(st);
    }

    let mut found: Option<(Val, u32, u64)> = None;
    for s in &ends[0].stores {
        if s.size != WORD {
            continue;
        }
        let [(lid, 1)] = s.val.terms.as_slice() else { continue };
        let Some(Atom::Load { addr, size: 8, ver }) = eng.syms.atoms.get(*lid as usize) else { continue };
        if *ver != base_ver || *addr != s.addr || s.val.k == 0 || s.val.k > MAX_APPEND_BYTES {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some((s.addr.clone(), *lid, s.val.k));
    }
    let (size_slot, size_id, n) = found?;
    let a = size_slot.plus(0u64.wrapping_sub(SIZE_FIELD));
    let res_id = eng.syms.lookup(&Atom::Load { addr: a.plus(RES_FIELD), size: WORD, ver: base_ver })?;
    let size_v = Syms::id_val(size_id, WORD);
    let res_v = Syms::id_val(res_id, WORD);

    let push_back = n == 1 && eng.syms.decide_two(&cond_h, fast_taken, size_id, res_id, |s, r| s < r);
    if !push_back {
        let d = res_v.sub(&size_v);
        let pts = eng.syms.classify_one(&[(cond_h.clone(), fast_taken)], &d, &[n])?;
        if !pts.iter().all(|&(dv, t)| t == (dv >= n)) {
            return None;
        }
    }

    let mut proven: Vec<FastPath> = Vec::new();
    for st in &ends {
        if let Some(fp) = prove_fast_path(&mut eng, st, base_ver, &a, size_id, res_id, n)? {
            proven.push(fp);
        }
    }
    if !proven.iter().any(|p| p.sso) || !proven.iter().any(|p| !p.sso) {
        return None;
    }
    let bytes = proven[0].bytes.clone();
    if proven.iter().any(|p| p.bytes != bytes) {
        return None;
    }
    let returns_this = proven.iter().all(|p| p.rax == a);

    let fast_written: BTreeSet<(u64, int4)> = ends.iter().flat_map(|st| st.written.iter().copied()).collect();
    let written: BTreeSet<(u64, int4)> = fast_written.union(&sg.written).copied().collect();
    if !written.iter().all(|&(off, size)| volatile(regs, off, size)) {
        return None;
    }

    if eng.read_reg(&sg, &regs.rcx, WORD) != a {
        return None;
    }
    let kind = if push_back {
        if !sg.stores.is_empty() {
            return None;
        }
        if eng.read_reg(&sg, &regs.r9, 1) != bytes[0] {
            return None;
        }
        let rdx_off = regs.rdx.get_offset();
        if sg.written.iter().any(|&(o, z)| o < rdx_off + 8 && rdx_off < o + z as u64)
            && eng.read_reg(&sg, &regs.rdx, WORD) != Val::konst(1, WORD)
        {
            return None;
        }
        Kind::PushBack(materialize(&mut eng, &sh, &bytes[0], &regs.bytes)?)
    } else {
        if eng.read_reg(&sg, &regs.rdx, WORD) != Val::konst(n, WORD) {
            return None;
        }
        let lit = eng.read_reg(&sg, &regs.r9, WORD).as_const()?;
        let rsp = eng.read_reg(&sg, &regs.rsp, WORD);
        if sg.stores.len() > 1
            || sg.stores.iter().any(|s| s.addr != rsp.plus(STACK_ARG_FIVE) || s.val != Val::konst(n, WORD))
        {
            return None;
        }
        let lit_addr = Address::new(regs.ram.clone(), lit);
        if data.get_arch().query_global_properties(&lit_addr, n as int4, &call_addr) & varnode_flags::readonly == 0 {
            return None;
        }
        for (i, b) in bytes.iter().enumerate() {
            let want = b.as_const()?;
            let at = Address::new(regs.ram.clone(), lit.wrapping_add(i as u64));
            if data.get_arch().get_load_image_value(&at, 1).ok()? != want {
                return None;
            }
        }
        Kind::Append { n, lit }
    };
    let this = materialize(&mut eng, &sh, &a, &regs.bases)?;

    let mut changed: BTreeSet<u64> = BTreeSet::new();
    let mut touch = |off: u64, size: int4| changed.extend(off..off + size as u64);
    for &(off, size) in &written {
        touch(off, size);
    }
    touch(regs.rcx.get_offset(), WORD);
    match kind {
        Kind::PushBack(_) => touch(regs.rdx.get_offset(), 1),
        Kind::Append { .. } => {
            touch(regs.rdx.get_offset(), WORD);
            touch(regs.r8.get_offset(), WORD);
        }
    }
    if returns_this {
        let rax = regs.rax.get_offset();
        changed.retain(|b| !(rax..rax + WORD as u64).contains(b));
    }
    if read_after_join(data, regs, join, changed) {
        return None;
    }
    Some(Site { head, grow, fast_slot, call, first_call_ix, this, kind, returns_this, fast_written })
}

fn emit_assign(data: &mut Funcdata, before: OpId, at: &Address, dest: &Address, size: int4, src: &Src) -> KunaResult<()> {
    let op = match src {
        Src::Const(c) => {
            let op = data.new_op(1, at.clone());
            data.op_set_opcode_code(op, OpCode::CPUI_COPY);
            let k = data.new_constant(size, c & mask(size));
            data.op_set_input(op, k, 0)?;
            op
        }
        Src::Reg(r, 0) => {
            let op = data.new_op(1, at.clone());
            data.op_set_opcode_code(op, OpCode::CPUI_COPY);
            let v = data.new_varnode(size, r, None);
            data.op_set_input(op, v, 0)?;
            op
        }
        Src::Reg(r, k) => {
            let op = data.new_op(2, at.clone());
            data.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
            let v = data.new_varnode(size, r, None);
            let c = data.new_constant(size, *k);
            data.op_set_input(op, v, 0)?;
            data.op_set_input(op, c, 1)?;
            op
        }
    };
    data.new_varnode_out(size, dest, op)?;
    data.op_insert_before(op, before);
    Ok(())
}

fn rewrite(data: &mut Funcdata, regs: &Regs, site: &Site) -> KunaResult<bool> {
    use crate::dtype::type_metatype;
    use crate::fspec::{effect_type, EffectRecord};
    let Some(types) = data.get_arch().types() else { return Ok(false) };
    let (Ok(void_ty), Ok(char_ty), Ok(ulong_ty)) =
        (types.get_type_void(), types.get_type_char(1), types.get_base(WORD, type_metatype::TYPE_UINT))
    else {
        return Ok(false);
    };
    let (Ok(void_ptr), Ok(char_ptr)) =
        (types.get_type_pointer(WORD, void_ty.clone(), WORD as u32), types.get_type_pointer(WORD, char_ty.clone(), WORD as u32))
    else {
        return Ok(false);
    };
    let gops = data.bb_ops(site.grow);
    let Some(call_addr) = data.obank().get(site.call).map(|o| o.get_addr().clone()) else { return Ok(false) };

    let this_piece = ParameterPieces { addr: regs.rcx.clone(), type_: Some(void_ptr.clone()), flags: 0 };
    let (name, params) = match &site.kind {
        Kind::PushBack(_) => {
            let c = ParameterPieces { addr: regs.rdx.clone(), type_: Some(char_ty), flags: 0 };
            (PUSH_BACK_NAME, vec![("this", this_piece), ("c", c)])
        }
        Kind::Append { .. } => {
            let s = ParameterPieces { addr: regs.rdx.clone(), type_: Some(char_ptr), flags: 0 };
            let n = ParameterPieces { addr: regs.r8.clone(), type_: Some(ulong_ty), flags: 0 };
            (APPEND_NAME, vec![("this", this_piece), ("s", s), ("n", n)])
        }
    };
    let mut fc = FuncCallSpecs::new(site.call, Address::default());
    fc.set_funcdata(Address::default(), name)?;
    let out_ty = if site.returns_this { void_ptr } else { void_ty };
    fc.proto_mut().set_internal(regs.model.clone(), out_ty.clone());
    for (slot, (nm, piece)) in params.iter().enumerate() {
        fc.proto_mut().set_param(slot as int4, nm, piece);
    }
    fc.proto_mut().set_input_lock(true);
    if site.returns_this {
        fc.proto_mut().set_output(&ParameterPieces { addr: regs.rax.clone(), type_: Some(out_ty), flags: 0 });
    }
    fc.proto_mut().set_output_lock(true);
    let rax = regs.rax.get_offset();
    for rec in regs.model.effect_list() {
        let at = rec.get_address();
        let (off, size) = (at.get_offset(), rec.get_size() as u64);
        let in_register_space = at.get_space().is_some_and(|sp| sp.get_index() == regs.reg_space);
        let fast_writes = site.fast_written.iter().any(|&(o, z)| o < off + size && off < o + z as u64);
        let returned = site.returns_this && off <= rax && rax < off + size;
        let kind = if rec.get_type() == effect_type::KILLEDBYCALL && in_register_space && !fast_writes && !returned {
            effect_type::UNAFFECTED
        } else {
            rec.get_type()
        };
        let range = kuna_num::pcoderaw::VarnodeData { space: at.get_space().cloned(), offset: off, size: size as u32 };
        fc.proto_mut().push_effect_override(EffectRecord::from_varnode(range, kind));
    }

    let anchor = gops[site.first_call_ix];
    data.remove_branch(site.head, site.fast_slot)?;
    for &o in &gops[..site.first_call_ix] {
        data.op_destroy(o);
    }
    emit_assign(data, anchor, &call_addr, &regs.rcx, WORD, &site.this)?;
    match &site.kind {
        Kind::PushBack(ch) => emit_assign(data, anchor, &call_addr, &regs.rdx, 1, ch)?,
        Kind::Append { n, lit } => {
            emit_assign(data, anchor, &call_addr, &regs.rdx, WORD, &Src::Const(*lit))?;
            emit_assign(data, anchor, &call_addr, &regs.r8, WORD, &Src::Const(*n))?;
        }
    }
    data.delete_call_specs(site.call);
    let idx = data.push_call_specs(fc);
    let handle = crate::flow::next_fspec_handle();
    let style = data.get_arch().kuna_name_style();
    data.get_call_specs(idx).register_in_fspec_space(handle, style);
    let fspecvn = data.new_varnode_call_specs(handle);
    data.op_set_input(site.call, fspecvn, 0)?;
    data.remove_unreachable_blocks(false, false)?;
    Ok(true)
}

/// (kuna) `ActionMsvcStrAppend` — collapse each inlined MSVC `std::string`
/// append to one call (option `msvcstrappend`).
pub struct ActionMsvcStrAppend {
    base: ActionBase,
}

impl ActionMsvcStrAppend {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMsvcStrAppend {
            base: ActionBase::new(crate::action::ruleflags::rule_onceperfunc, "msvcstrappend", g),
        })
    }
}

impl Action for ActionMsvcStrAppend {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMsvcStrAppend { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().msvc_str_append || data.num_calls() == 0 {
            return 0;
        }
        let Some(regs) = Regs::resolve(data) else { return 0 };
        let mut changed = 0;
        let mut seen: BTreeSet<OpId> = BTreeSet::new();
        'sites: while changed < MAX_SITES {
            let calls: Vec<OpId> = (0..data.num_calls()).map(|i| data.get_call_specs(i).get_op()).collect();
            for call in calls {
                if !seen.insert(call) {
                    continue;
                }
                let Some(site) = try_match(data, &regs, call) else { continue };
                match rewrite(data, &regs, &site) {
                    Ok(true) => {
                        changed += 1;
                        continue 'sites;
                    }
                    _ => break 'sites,
                }
            }
            break;
        }
        (changed != 0) as ApplyResult
    }
}

#[cfg(test)]
#[path = "kuna_msvcstrappend/tests.rs"]
mod tests;
