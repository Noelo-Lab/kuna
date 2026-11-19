//! (kuna `spsolve`) Solve an x86-32 function's stack-pointer deltas for the
//! calls that pop arguments or do not return.
//!
//! # The gap
//!
//! Under a compiler spec whose default model states its extrapop
//! (`x86gcc.cspec`'s `__cdecl`, extrapop 4), `ActionExtraPopSetup` adds that
//! constant after every call and nothing checks it. Two kinds of call break it:
//!
//! * a callee that pops more than its return address: a function returning an
//!   aggregate pops the hidden result pointer (`ret 4`), and a `stdcall` or
//!   `fastcall` callee pops its arguments;
//! * a callee that does not return and is not known as such. The code after
//!   the call is another path's, laid out there because nothing falls into it,
//!   and it expects the stack pointer that path left.
//!
//! Either way two paths reach a block with different stack pointers, or a
//! return is reached with the stack pointer away from the return address. The
//! stack pointer then stays a variable (`// esp`), stack arguments are read
//! through it (`*(int *)&v5[-4] = v1`), and every stack reference after the
//! call is displaced.
//!
//! # The mechanism
//!
//! Before the constants are planted, the raw p-code is walked block by block
//! with the stack pointer, and every register or temporary holding a stack
//! address, as a linear form over the entry stack pointer: a constant plus one
//! unknown per call (what the call moves the stack pointer by, beyond its
//! model's extrapop) and one per place the stack pointer is set to something
//! the walk cannot read (an alignment mask, a runtime reservation). The forms
//! give one equation per constraint:
//!
//! * a block reached from two predecessors has one stack pointer on entry;
//! * the stack pointer of a return, or of a jump that leaves the function,
//!   is the entry stack pointer;
//! * a direct callee whose own returns prove its extrapop
//!   ([`crate::p4_calls::kuna_calleepurge`]) contributes that constant.
//!
//! The system is first checked with every call unknown at zero. When that
//! holds the model is consistent and only the proven extrapops are applied.
//! Otherwise each call is one of two kinds:
//!
//! * code of its own block follows it, so it returned, and its unknown is a
//!   pop: zero or more. When the callee's reached returns were read and pop
//!   only the return address, it is zero;
//! * it ends its block and falls into a block with another predecessor, so it
//!   may not have returned.
//!
//! A call of the second kind is marked no-return when the system has no
//! solution in which it returned, that is with its unknown zero or more (zero,
//! when its callee's returns pop only the return address) and every pop zero
//! or more. The other unknowns are eliminated over the rationals and the
//! first phase of the simplex method decides what is left. The call sites are
//! recorded on the function's override and the function is flowed again
//! without their fall-through.
//!
//! The remaining system is then solved by elimination, with one rule the sign
//! adds: pops that sum to nothing are each nothing. A call whose unknown has
//! exactly one value, a positive multiple of the stack slot that the callee's
//! reached returns do not contradict, pops that much.
//!
//! A direct callee with no reachable return is not followed at all
//! ([`callee_never_returns`]).
//!
//! # What it will not do
//!
//! * A call that ends its block and whose callee's code is not available (an
//!   import, a call through a pointer) may pop its arguments or not return;
//!   when a pop explains the system it is left alone.
//! * An unknown with more than one solution is left at zero.
//! * A call that returned with a value no callee pops says the model is wrong
//!   in a way the unknowns do not cover; no pop of the function is applied.
//! * A software interrupt is lifted as a call through a user-defined
//!   operation that pushes nothing; its stack pointer is not solved.
//! * A model whose extrapop is unknown (`x86win.cspec`) keeps
//!   `ActionStackPtrFlow`'s own solve and guesses.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::fspec::{effect_type, EXTRAPOP_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashMap as HashMap;
use crate::p4_calls::kuna_calleepurge::{summary, Effect};

/// How many levels of nested direct calls a callee walk reads.
const DEPTH: u8 = 3;
/// Blocks a function may have and still be solved.
const MAX_BLOCKS: usize = 1 << 13;
/// Unknowns a system may have.
const MAX_VARS: usize = 1 << 10;
/// Passes of the register fixpoint.
const MAX_PASSES: usize = 8;
/// Term updates one elimination may make.
const MAX_WORK: usize = 1 << 22;
/// Rows, columns and pivots of one feasibility test, and the calls tested.
const MAX_LP_ROWS: usize = 128;
const MAX_LP_COLS: usize = 256;
const MAX_LP_STEPS: usize = 4096;
const MAX_TESTS: usize = 64;
/// The most a callee is taken to pop above its return address.
const MAX_POP: i64 = 0x400;

/// What the returns of a direct callee do to the stack pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalleeReturns {
    /// The extrapop every reachable return proves.
    pub proven: Option<int4>,
    /// The most any reached return adds to the stack pointer.
    pub most: Option<int4>,
}

/// `option spsolve` on its target: x86-32 under a default model that states
/// its extrapop.
pub fn applies(arch: &Architecture) -> bool {
    arch.sp_solve
        && arch.archid.starts_with("x86:LE:32:")
        && arch.default_fp().is_some_and(|m| m.get_extra_pop() != EXTRAPOP_UNKNOWN)
}

/// What the returns of the direct call target `entry` do, when its code says.
pub fn callee_returns(arch: &Architecture, entry: &Address) -> Option<CalleeReturns> {
    if !applies(arch) || entry.is_invalid() {
        return None;
    }
    let s = summary(arch, entry, DEPTH);
    let proven = match s.effect {
        Effect::Balanced(ep) => Some(ep),
        _ => None,
    };
    (proven.is_some() || s.ret_pops.is_some()).then_some(CalleeReturns { proven, most: s.ret_pops })
}

/// Does the direct call target `entry` have no reachable return?
pub fn callee_never_returns(arch: &Architecture, entry: &Address) -> bool {
    applies(arch) && !entry.is_invalid() && summary(arch, entry, DEPTH).effect == Effect::NoReturn
}

/// A rational number in lowest terms with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rat {
    n: i64,
    d: i64,
}

impl Rat {
    const ZERO: Rat = Rat { n: 0, d: 1 };

    fn new(n: i128, d: i128) -> Option<Rat> {
        if d == 0 {
            return None;
        }
        let g = gcd(n.unsigned_abs(), d.unsigned_abs()) as i128;
        let (n, d) = if d < 0 { (-(n / g), -(d / g)) } else { (n / g, d / g) };
        Some(Rat { n: i64::try_from(n).ok()?, d: i64::try_from(d).ok()? })
    }

    fn int(n: i64) -> Rat {
        Rat { n, d: 1 }
    }

    fn is_zero(self) -> bool {
        self.n == 0
    }

    fn mul(self, o: Rat) -> Option<Rat> {
        Rat::new(i128::from(self.n) * i128::from(o.n), i128::from(self.d) * i128::from(o.d))
    }

    fn sub(self, o: Rat) -> Option<Rat> {
        let n = i128::from(self.n) * i128::from(o.d) - i128::from(o.n) * i128::from(self.d);
        Rat::new(n, i128::from(self.d) * i128::from(o.d))
    }

    fn less(self, o: Rat) -> bool {
        i128::from(self.n) * i128::from(o.d) < i128::from(o.n) * i128::from(self.d)
    }

    fn div(self, o: Rat) -> Option<Rat> {
        Rat::new(i128::from(self.n) * i128::from(o.d), i128::from(self.d) * i128::from(o.n))
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

/// `c + sum(coefficient * unknown)`, relative to the entry stack pointer.
/// Terms are sorted by unknown and never zero.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Form {
    c: i64,
    terms: Vec<(u32, i64)>,
}

impl Form {
    fn var(v: u32) -> Form {
        Form { c: 0, terms: vec![(v, 1)] }
    }

    fn shift(mut self, k: i64) -> Option<Form> {
        self.c = self.c.checked_add(k)?;
        Some(self)
    }

    fn add_var(&mut self, v: u32, k: i64) -> Option<()> {
        match self.terms.binary_search_by_key(&v, |t| t.0) {
            Ok(i) => {
                self.terms[i].1 = self.terms[i].1.checked_add(k)?;
                if self.terms[i].1 == 0 {
                    self.terms.remove(i);
                }
            }
            Err(i) => self.terms.insert(i, (v, k)),
        }
        Some(())
    }

    fn minus(&self, o: &Form) -> Option<Form> {
        let mut out = Form { c: self.c.checked_sub(o.c)?, terms: self.terms.clone() };
        for &(v, k) in &o.terms {
            out.add_var(v, k.checked_neg()?)?;
        }
        Some(out)
    }

    fn is_zero(&self) -> bool {
        self.c == 0 && self.terms.is_empty()
    }
}

/// One equation `sum(terms) = rhs`, terms sorted by unknown and never zero.
#[derive(Clone, Debug)]
struct Row {
    terms: Vec<(u32, Rat)>,
    rhs: Rat,
}

impl Row {
    fn of(form: &Form) -> Option<Row> {
        let terms = form.terms.iter().map(|&(v, k)| (v, Rat::int(k))).collect();
        Some(Row { terms, rhs: Rat::int(form.c.checked_neg()?) })
    }

    fn coefficient(&self, v: u32) -> Option<Rat> {
        self.terms.binary_search_by_key(&v, |t| t.0).ok().map(|i| self.terms[i].1)
    }

    /// Divide the equation by `k`.
    fn scale(&mut self, k: Rat) -> Option<()> {
        for t in &mut self.terms {
            t.1 = t.1.div(k)?;
        }
        self.rhs = self.rhs.div(k)?;
        Some(())
    }

    /// `self -= k * other`.
    fn reduce(&mut self, k: Rat, other: &Row) -> Option<()> {
        let mut out = Vec::with_capacity(self.terms.len() + other.terms.len());
        let (mut i, mut j) = (0, 0);
        while i < self.terms.len() || j < other.terms.len() {
            let a = self.terms.get(i).copied();
            let b = other.terms.get(j).copied();
            let (v, x) = match (a, b) {
                (Some((va, xa)), Some((vb, _))) if va < vb => {
                    i += 1;
                    (va, xa)
                }
                (Some((va, xa)), Some((vb, xb))) if va == vb => {
                    i += 1;
                    j += 1;
                    (va, xa.sub(k.mul(xb)?)?)
                }
                (Some((va, xa)), None) => {
                    i += 1;
                    (va, xa)
                }
                (_, Some((vb, xb))) => {
                    j += 1;
                    (vb, Rat::ZERO.sub(k.mul(xb)?)?)
                }
                (None, None) => break,
            };
            if !x.is_zero() {
                out.push((v, x));
            }
        }
        self.terms = out;
        self.rhs = self.rhs.sub(k.mul(other.rhs)?)?;
        Some(())
    }
}

/// Solve `form = 0` for every form over `nvars` unknowns by elimination: the
/// value of each unknown the system pins to one, or `None` when the system has
/// no solution or the work or the arithmetic runs out.
fn eliminate(forms: &[Form], nvars: usize) -> Option<Vec<Option<Rat>>> {
    let mut rows: Vec<Row> = Vec::new();
    let mut pivot: Vec<Option<usize>> = vec![None; nvars];
    let mut work = 0usize;
    for form in forms {
        let mut row = Row::of(form)?;
        while let Some(&(v, k)) = row.terms.iter().find(|t| pivot.get(t.0 as usize).is_some_and(Option::is_some)) {
            let other = &rows[pivot[v as usize]?];
            work = work.checked_add(row.terms.len() + other.terms.len())?;
            row.reduce(k, other)?;
        }
        let Some(&(p, k)) = row.terms.first() else {
            if !row.rhs.is_zero() {
                return None;
            }
            continue;
        };
        row.scale(k)?;
        for old in &mut rows {
            if let Some(k) = old.coefficient(p) {
                work = work.checked_add(old.terms.len() + row.terms.len())?;
                old.reduce(k, &row)?;
            }
        }
        if work > MAX_WORK {
            return None;
        }
        *pivot.get_mut(p as usize)? = Some(rows.len());
        rows.push(row);
    }
    Some(pivot.iter().map(|p| p.map(|i| &rows[i]).filter(|r| r.terms.len() == 1).map(|r| r.rhs)).collect())
}

/// What a tracked location holds.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Val {
    Const(i64),
    Stack(Form),
}

/// The stack pointer and the registers holding a constant or a stack address,
/// sorted by register offset.
#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    sp: Form,
    regs: Vec<(u64, Val)>,
    /// The stack pointer before a call no instruction has followed yet. A
    /// return reached in that state is the one the flow planted after a jump
    /// out of the function that it made a call.
    tail: Option<Form>,
}

/// What one unknown stands for.
#[derive(Clone, Copy)]
enum Var {
    /// What the call spec at this index moves the stack pointer by, beyond its
    /// model's extrapop.
    Call(int4),
    /// A stack pointer the walk cannot read.
    Free,
}

struct Tracker<'a> {
    data: &'a Funcdata,
    reg: Rc<AddrSpace>,
    sp: u64,
    size: int4,
    sites: Vec<Site>,
    /// Are the calls that are not [`Site::Fixed`] given an unknown?
    symbolic: bool,
    vars: Vec<Var>,
    var_of: HashMap<OpId, u32>,
    overflow: bool,
}

impl Tracker<'_> {
    fn var(&mut self, op: OpId, kind: Var) -> u32 {
        if let Some(&v) = self.var_of.get(&op) {
            return v;
        }
        if self.vars.len() >= MAX_VARS {
            self.overflow = true;
            return 0;
        }
        let v = self.vars.len() as u32;
        self.vars.push(kind);
        self.var_of.insert(op, v);
        v
    }

    fn fresh(&mut self, op: OpId) -> Form {
        Form::var(self.var(op, Var::Free))
    }

    fn eval(&self, st: &State, tmps: &[(u64, Val)], vn: Option<VarnodeId>) -> Option<Val> {
        let v = self.data.vbank().get(vn?)?;
        if v.is_constant() {
            return Some(Val::Const(sign_extend(v.get_offset(), v.get_size())));
        }
        if v.get_size() != self.size {
            return None;
        }
        let off = v.get_offset();
        if Rc::ptr_eq(v.get_space(), &self.reg) {
            if off == self.sp {
                return Some(Val::Stack(st.sp.clone()));
            }
            return st.regs.iter().find(|r| r.0 == off).map(|r| r.1.clone());
        }
        if v.get_space().get_type() == spacetype::IPTR_INTERNAL {
            return tmps.iter().find(|t| t.0 == off).map(|t| t.1.clone());
        }
        None
    }

    fn value(&self, st: &State, tmps: &[(u64, Val)], op: OpId) -> Option<Val> {
        let o = self.data.obank().get(op)?;
        let a = self.eval(st, tmps, o.get_in(0));
        match o.code() {
            OpCode::CPUI_COPY => a,
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let b = self.eval(st, tmps, o.get_in(1))?;
                let sub = o.code() == OpCode::CPUI_INT_SUB;
                match (a?, b) {
                    (Val::Stack(f), Val::Const(k)) => Some(Val::Stack(f.shift(if sub { k.checked_neg()? } else { k })?)),
                    (Val::Const(k), Val::Stack(f)) if !sub => Some(Val::Stack(f.shift(k)?)),
                    (Val::Const(x), Val::Const(y)) => {
                        let r = if sub { x.wrapping_sub(y) } else { x.wrapping_add(y) };
                        Some(Val::Const(sign_extend(r as u64, self.size)))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn assign(&mut self, st: &mut State, tmps: &mut Vec<(u64, Val)>, op: OpId, out: VarnodeId, val: Option<Val>) {
        let Some(v) = self.data.vbank().get(out) else { return };
        let (off, size) = (v.get_offset(), v.get_size().max(0) as u64);
        let whole = v.get_size() == self.size;
        if Rc::ptr_eq(v.get_space(), &self.reg) {
            let overlaps = |o: u64| o < off.saturating_add(size) && off < o.saturating_add(self.size as u64);
            if overlaps(self.sp) {
                st.sp = match val {
                    Some(Val::Stack(f)) if whole && off == self.sp => f,
                    _ => self.fresh(op),
                };
                return;
            }
            st.regs.retain(|r| !overlaps(r.0));
            if let (true, Some(val)) = (whole, val) {
                let at = st.regs.partition_point(|r| r.0 < off);
                st.regs.insert(at, (off, val));
            }
        } else if v.get_space().get_type() == spacetype::IPTR_INTERNAL {
            tmps.retain(|t| t.0 != off);
            if let (true, Some(val)) = (whole, val) {
                tmps.push((off, val));
            }
        }
    }

    fn call(&mut self, st: &mut State, op: OpId) {
        let Some(i) = self.data.get_call_specs_index(op) else {
            st.regs.clear();
            st.sp = self.fresh(op);
            return;
        };
        let fc = self.data.get_call_specs(i);
        let (reg, size) = (&self.reg, self.size);
        st.regs.retain(|r| fc.proto().has_effect(&Address::new(reg.clone(), r.0), size) == effect_type::UNAFFECTED);
        let site = self.sites.get(i as usize).copied().unwrap_or(Site::Opaque);
        let moved = match site {
            Site::Opaque => None,
            Site::Fixed(extrapop) => st.sp.clone().shift(extrapop),
            Site::Join { model, .. } | Site::Open { model } => st.sp.clone().shift(model),
        };
        let Some(mut sp) = moved else {
            st.sp = self.fresh(op);
            return;
        };
        if self.symbolic && !matches!(site, Site::Fixed(_)) {
            let v = self.var(op, Var::Call(i));
            if sp.add_var(v, 1).is_none() {
                self.overflow = true;
            }
        }
        st.sp = sp;
    }

    /// Run the block starting at `head` from `st`. With `anchors`, the stack
    /// pointer each return starts from is pushed onto it.
    fn block(&mut self, head: Option<OpId>, mut st: State, mut anchors: Option<&mut Vec<Form>>) -> State {
        let mut tmps: Vec<(u64, Val)> = Vec::new();
        let mut at: Option<Address> = None;
        let mut start = st.sp.clone();
        let mut userop = false;
        let mut cur = head;
        while let Some(id) = cur {
            let Some(o) = self.data.obank().get(id) else { break };
            cur = o.basic_neighbours().1;
            if at.as_ref() != Some(o.get_addr()) {
                at = Some(o.get_addr().clone());
                tmps.clear();
                start = st.sp.clone();
                userop = false;
            }
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    tmps.clear();
                    let before = st.sp.clone();
                    self.call(&mut st, id);
                    st.tail = Some(before);
                }
                OpCode::CPUI_CALLOTHER => {
                    userop = true;
                    st.tail = None;
                    if let Some(out) = o.get_out() {
                        self.assign(&mut st, &mut tmps, id, out, None);
                    }
                }
                OpCode::CPUI_RETURN => {
                    if let (0, false, Some(anchors)) = (o.get_halt_type(), userop, anchors.as_deref_mut()) {
                        anchors.push(st.tail.take().unwrap_or_else(|| start.clone()));
                    }
                }
                _ => {
                    st.tail = None;
                    if let Some(out) = o.get_out() {
                        let val = self.value(&st, &tmps, id);
                        self.assign(&mut st, &mut tmps, id, out, val);
                    }
                }
            }
        }
        st
    }

    /// The equations of the function, each a form that is zero; `None` when
    /// the function is too large or its registers do not settle.
    fn equations(&mut self, symbolic: bool) -> Option<Vec<Form>> {
        self.symbolic = symbolic;
        let data = self.data;
        let n = usize::try_from(data.bblocks_get_size()).ok()?;
        if n == 0 || n > MAX_BLOCKS {
            return None;
        }
        let ids: Vec<_> = (0..n).map(|i| data.bblocks_get_block(i as int4)).collect();
        let index: HashMap<_, usize> = ids.iter().enumerate().map(|(i, &b)| (b, i)).collect();
        let graph = data.bblocks_ref();
        let succs: Vec<Vec<usize>> = ids
            .iter()
            .map(|&b| {
                let bl = graph.block(b);
                (0..bl.size_out()).filter_map(|j| index.get(&bl.get_out(j)).copied()).collect()
            })
            .collect();
        let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (b, out) in succs.iter().enumerate() {
            for &s in out {
                preds[s].push(b);
            }
        }
        let (order, parent) = spanning_order(&succs);
        let mut ins: Vec<Option<State>> = vec![None; n];
        let mut outs: Vec<Option<State>> = vec![None; n];
        let mut settled = false;
        for _ in 0..MAX_PASSES {
            let mut changed = false;
            for &b in &order {
                let Some(incoming) = entry_state(b, &parent, &preds[b], &outs) else { continue };
                if ins[b].as_ref() == Some(&incoming) {
                    continue;
                }
                changed = true;
                outs[b] = Some(self.block(data.bb_op_head(ids[b]), incoming.clone(), None));
                ins[b] = Some(incoming);
            }
            if self.overflow {
                return None;
            }
            if !changed {
                settled = true;
                break;
            }
        }
        if !settled {
            return None;
        }
        let mut eqs = Vec::new();
        for &b in &order {
            let Some(incoming) = ins[b].clone() else { continue };
            for &p in &preds[b] {
                if let Some(out) = &outs[p] {
                    let diff = out.sp.minus(&incoming.sp)?;
                    if !diff.is_zero() {
                        eqs.push(diff);
                    }
                }
            }
            let mut anchors = Vec::new();
            self.block(data.bb_op_head(ids[b]), incoming, Some(&mut anchors));
            eqs.extend(anchors.into_iter().filter(|f| !f.is_zero()));
        }
        (!self.overflow).then_some(eqs)
    }
}

/// The blocks reachable from block 0 in reverse postorder, and for each the
/// predecessor it was first reached from.
fn spanning_order(succs: &[Vec<usize>]) -> (Vec<usize>, Vec<Option<usize>>) {
    let n = succs.len();
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut seen = vec![false; n];
    let mut post = Vec::with_capacity(n);
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    seen[0] = true;
    while let Some(&mut (b, ref mut next)) = stack.last_mut() {
        match succs[b].get(*next) {
            Some(&s) => {
                *next += 1;
                if !seen[s] {
                    seen[s] = true;
                    parent[s] = Some(b);
                    stack.push((s, 0));
                }
            }
            None => {
                post.push(b);
                stack.pop();
            }
        }
    }
    post.reverse();
    (post, parent)
}

/// The state block `b` starts from: the stack pointer its spanning-tree
/// predecessor left, and the registers every walked predecessor agrees on.
fn entry_state(b: usize, parent: &[Option<usize>], preds: &[usize], outs: &[Option<State>]) -> Option<State> {
    let Some(p) = parent[b] else {
        return (b == 0).then(|| State { sp: Form::default(), regs: Vec::new(), tail: None });
    };
    let first = outs[p].as_ref()?;
    let mut regs = first.regs.clone();
    for out in preds.iter().filter_map(|&q| outs[q].as_ref()) {
        regs.retain(|r| out.regs.contains(r));
    }
    let tail = first.tail.clone().filter(|_| preds.len() == 1);
    Some(State { sp: first.sp.clone(), regs, tail })
}

fn sign_extend(val: u64, size: int4) -> i64 {
    if size <= 0 || size >= 8 {
        return val as i64;
    }
    let shift = 64 - 8 * size as u32;
    ((val << shift) as i64) >> shift
}

/// What is known of one call before the solve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Site {
    /// Its model states no extrapop.
    Opaque,
    /// It moves the stack pointer by this much: the callee's returns prove
    /// it, or code of the same block follows the call and the callee's
    /// reached returns pop only the return address.
    Fixed(i64),
    /// It ends its block and falls into a block with another predecessor, so
    /// it may not return. `plain`: the callee's reached returns pop only the
    /// return address.
    Join { model: i64, plain: bool },
    /// Code of the same block follows it and nothing says what its callee
    /// pops.
    Open { model: i64 },
}

fn classify(data: &Funcdata) -> Vec<Site> {
    (0..data.num_calls())
        .map(|i| {
            let fc = data.get_call_specs(i);
            let model = fc.get_extra_pop();
            if model == EXTRAPOP_UNKNOWN || calls_through_userop(data, fc.get_op()) {
                return Site::Opaque;
            }
            let returns = fc.callee_returns();
            if let Some(ep) = returns.and_then(|r| r.proven) {
                return Site::Fixed(i64::from(ep));
            }
            let model = i64::from(model);
            let plain = returns.and_then(|r| r.most).is_some_and(|m| i64::from(m) == model);
            match (falls_into_join(data, fc.get_op()), plain) {
                (true, plain) => Site::Join { model, plain },
                (false, true) => Site::Fixed(model),
                (false, false) => Site::Open { model },
            }
        })
        .collect()
}

/// Is `op` an indirect call whose target a user-defined p-code operation of
/// the same instruction produced, as a software interrupt is lifted? Such a
/// call pushes no return address and nothing says where it goes.
fn calls_through_userop(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_CALLIND) else { return false };
    let storage = |vn: Option<VarnodeId>| vn.and_then(|v| data.vbank().get(v)).map(|v| (v.get_addr().clone(), v.get_size()));
    let Some(target) = storage(o.get_in(0)) else { return false };
    let mut cur = o.basic_neighbours().0;
    while let Some(prev) = cur.and_then(|id| data.obank().get(id)).filter(|p| p.get_addr() == o.get_addr()) {
        if storage(prev.get_out()).as_ref() == Some(&target) {
            return prev.code() == OpCode::CPUI_CALLOTHER;
        }
        cur = prev.basic_neighbours().0;
    }
    false
}

/// Does the call `op` end its block and fall into a block with another
/// predecessor?
fn falls_into_join(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    if o.basic_neighbours().1.is_some() {
        return false;
    }
    let Some(parent) = o.get_parent() else { return false };
    let graph = data.bblocks_ref();
    let bl = graph.block(parent);
    bl.size_out() == 1 && graph.block(bl.get_out(0)).size_in() >= 2
}

/// Has `rows` a solution in which every unknown marked in `nonneg` is zero or
/// more? The other unknowns are eliminated, and what is left is decided by
/// the first phase of the simplex method with Bland's rule. `None` when the
/// system is too large or the arithmetic runs out.
fn feasible(rows: &[Row], nonneg: &[bool]) -> Option<bool> {
    let mut pivots: Vec<Row> = Vec::new();
    let mut pivot_of: Vec<Option<usize>> = vec![None; nonneg.len()];
    let mut bounded: Vec<Row> = Vec::new();
    for row in rows {
        let mut row = row.clone();
        while let Some(&(v, k)) = row.terms.iter().find(|t| pivot_of.get(t.0 as usize).is_some_and(Option::is_some)) {
            row.reduce(k, &pivots[pivot_of[v as usize]?])?;
        }
        let free = row.terms.iter().find(|t| !nonneg.get(t.0 as usize).copied().unwrap_or(false)).copied();
        match free {
            Some((p, k)) => {
                row.scale(k)?;
                for old in &mut pivots {
                    if let Some(k) = old.coefficient(p) {
                        old.reduce(k, &row)?;
                    }
                }
                *pivot_of.get_mut(p as usize)? = Some(pivots.len());
                pivots.push(row);
            }
            None if row.terms.is_empty() => {
                if !row.rhs.is_zero() {
                    return Some(false);
                }
            }
            None => bounded.push(row),
        }
    }
    simplex(&bounded)
}

/// Has `rows` a solution with every unknown zero or more?
fn simplex(rows: &[Row]) -> Option<bool> {
    let mut cols: Vec<u32> = rows.iter().flat_map(|r| r.terms.iter().map(|t| t.0)).collect();
    cols.sort_unstable();
    cols.dedup();
    let (m, n) = (rows.len(), cols.len());
    if m > MAX_LP_ROWS || n > MAX_LP_COLS {
        return None;
    }
    let mut tab: Vec<Vec<Rat>> = Vec::with_capacity(m);
    for row in rows {
        let flip = row.rhs.n < 0;
        let mut line = vec![Rat::ZERO; n + 1];
        for &(v, k) in &row.terms {
            let j = cols.binary_search(&v).ok()?;
            line[j] = if flip { Rat::ZERO.sub(k)? } else { k };
        }
        line[n] = if flip { Rat::ZERO.sub(row.rhs)? } else { row.rhs };
        tab.push(line);
    }
    let mut cost = vec![Rat::ZERO; n + 1];
    for line in &tab {
        for (c, x) in cost.iter_mut().zip(line) {
            *c = c.sub(*x)?;
        }
    }
    let mut basis: Vec<usize> = (n..n + m).collect();
    for _ in 0..MAX_LP_STEPS {
        let Some(enter) = (0..n).find(|&j| cost[j].n < 0) else {
            return Some(cost[n].is_zero());
        };
        let mut leave: Option<(usize, Rat)> = None;
        for (i, line) in tab.iter().enumerate() {
            if line[enter].n <= 0 {
                continue;
            }
            let ratio = line[n].div(line[enter])?;
            let better = match leave {
                None => true,
                Some((l, best)) => ratio.less(best) || (ratio == best && basis[i] < basis[l]),
            };
            if better {
                leave = Some((i, ratio));
            }
        }
        let (row, _) = leave?;
        let k = tab[row][enter];
        for x in &mut tab[row] {
            *x = x.div(k)?;
        }
        let pivot = tab[row].clone();
        for (i, line) in tab.iter_mut().enumerate() {
            if i != row && !line[enter].is_zero() {
                let k = line[enter];
                for (x, p) in line.iter_mut().zip(&pivot) {
                    *x = x.sub(k.mul(*p)?)?;
                }
            }
        }
        let k = cost[enter];
        for (x, p) in cost.iter_mut().zip(&pivot) {
            *x = x.sub(k.mul(*p)?)?;
        }
        basis[row] = enter;
    }
    None
}

/// [`eliminate`], with what the sign of a pop adds: when every unknown left
/// in an equation is marked in `nonneg`, their coefficients share a sign and
/// the constant is zero, each of them is zero.
fn eliminate_signed(forms: &[Form], nonneg: &[bool]) -> Option<Vec<Option<Rat>>> {
    let mut forms = forms.to_vec();
    let base = forms.len();
    loop {
        let solved = eliminate(&forms, nonneg.len())?;
        let mut zeros: Vec<u32> = Vec::new();
        for form in &forms[..base] {
            let mut rest = Rat::int(form.c);
            let mut open: Vec<(u32, i64)> = Vec::new();
            for &(v, k) in &form.terms {
                match solved.get(v as usize).copied().flatten() {
                    Some(x) => rest = rest.sub(Rat::ZERO.sub(Rat::int(k).mul(x)?)?)?,
                    None => open.push((v, k)),
                }
            }
            let one_sign = open.iter().all(|t| t.1 > 0) || open.iter().all(|t| t.1 < 0);
            if rest.is_zero() && one_sign && open.iter().all(|t| nonneg.get(t.0 as usize).copied().unwrap_or(false)) {
                zeros.extend(open.iter().map(|t| t.0));
            }
        }
        zeros.sort_unstable();
        zeros.dedup();
        if zeros.is_empty() {
            return Some(solved);
        }
        forms.extend(zeros.into_iter().map(Form::var));
    }
}

/// What the solve reads off one call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    /// Nothing to change.
    Keep,
    /// The callee pops this much beyond the model's extrapop.
    Pop(i64),
    /// The call does not return.
    NoReturn,
}

/// Is `v` what a callee can pop beyond its return address?
fn is_pop(v: Rat, slot: i64) -> bool {
    v.d == 1 && v.n > 0 && v.n % slot == 0 && v.n <= MAX_POP
}

/// The verdict for every call spec of `data`, or `None` when the model is
/// consistent as it stands or the system has no solution.
fn solve_calls(data: &Funcdata, sp: &Address, size: int4) -> Option<Vec<Verdict>> {
    let mut tracker = Tracker {
        data,
        reg: sp.get_space()?.clone(),
        sp: sp.get_offset(),
        size,
        sites: classify(data),
        symbolic: false,
        vars: Vec::new(),
        var_of: HashMap::default(),
        overflow: false,
    };
    let eqs = tracker.equations(false)?;
    if eqs.is_empty() || eliminate(&eqs, tracker.vars.len()).is_some() {
        return None;
    }
    tracker.vars.clear();
    tracker.var_of.clear();
    let mut eqs = tracker.equations(true)?;
    let Tracker { sites, vars, .. } = tracker;
    let calls: Vec<(usize, usize, Site)> = vars
        .iter()
        .enumerate()
        .filter_map(|(v, kind)| match *kind {
            Var::Call(i) => Some((v, i as usize, *sites.get(i as usize)?)),
            Var::Free => None,
        })
        .collect();
    let mut nonneg = vec![false; vars.len()];
    for &(v, _, site) in &calls {
        nonneg[v] = matches!(site, Site::Open { .. });
    }
    let rows: Vec<Row> = eqs.iter().map(Row::of).collect::<Option<_>>()?;
    if !feasible(&rows, &nonneg)? {
        return None;
    }
    let mut verdicts = vec![Verdict::Keep; sites.len()];
    let mut pinned = vec![false; sites.len()];
    for &(v, i, site) in calls.iter().filter(|c| matches!(c.2, Site::Join { .. })).take(MAX_TESTS) {
        let returns = if matches!(site, Site::Join { plain: true, .. }) {
            let mut with = rows.clone();
            with.push(Row::of(&Form::var(v as u32))?);
            feasible(&with, &nonneg)
        } else {
            nonneg[v] = true;
            let returns = feasible(&rows, &nonneg);
            nonneg[v] = false;
            returns
        };
        if returns == Some(false) {
            verdicts[i] = Verdict::NoReturn;
            pinned[i] = true;
        }
    }
    for &(v, i, site) in &calls {
        match site {
            Site::Join { plain: true, .. } if verdicts[i] == Verdict::Keep => eqs.push(Form::var(v as u32)),
            Site::Join { plain: false, .. } => nonneg[v] = verdicts[i] == Verdict::Keep,
            _ => {}
        }
    }
    if let Some(pops) = eliminate_signed(&eqs, &nonneg).and_then(|solved| read_pops(data, &calls, &solved, size)) {
        for (i, extra) in pops {
            if verdicts[i] == Verdict::Keep {
                verdicts[i] = Verdict::Pop(extra);
                pinned[i] = true;
            }
        }
    }
    spread(data, &mut verdicts, &pinned);
    verdicts.iter().any(|v| *v != Verdict::Keep).then_some(verdicts)
}

/// The pops the solved values state, by call spec; `None` when a call that
/// returned has a value no callee pops, which says the model is wrong in a
/// way the unknowns do not cover.
fn read_pops(data: &Funcdata, calls: &[(usize, usize, Site)], solved: &[Option<Rat>], size: int4) -> Option<Vec<(usize, i64)>> {
    let mut pops = Vec::new();
    for &(v, i, site) in calls {
        let (Site::Open { model } | Site::Join { model, plain: false }) = site else { continue };
        let Some(value) = solved.get(v).copied().flatten().filter(|x| !x.is_zero()) else { continue };
        let most = data.get_call_specs(i as int4).callee_returns().and_then(|r| r.most).map(i64::from);
        let agrees = is_pop(value, i64::from(size)) && most.is_none_or(|m| m - model == value.n);
        match site {
            Site::Open { .. } if !agrees => return None,
            Site::Join { .. } if !agrees || most.is_none() => {}
            _ => pops.push((i, value.n)),
        }
    }
    Some(pops)
}

/// A callee's pop and its not returning are the callee's, so the other direct
/// calls to it take what the calls the system pinned agree on.
fn spread(data: &Funcdata, verdicts: &mut [Verdict], pinned: &[bool]) {
    let direct = |i: usize| {
        let fc = data.get_call_specs(i as int4);
        let call = data.obank().get(fc.get_op()).is_some_and(|o| o.code() == OpCode::CPUI_CALL);
        (call && !fc.get_entry_address().is_invalid()).then(|| fc.get_entry_address().clone())
    };
    let mut by_callee: BTreeMap<Address, Option<Verdict>> = BTreeMap::new();
    for i in (0..verdicts.len()).filter(|&i| pinned[i]) {
        if let Some(entry) = direct(i) {
            by_callee.entry(entry).and_modify(|seen| *seen = seen.filter(|s| *s == verdicts[i])).or_insert(Some(verdicts[i]));
        }
    }
    for i in (0..verdicts.len()).filter(|&i| !pinned[i]) {
        if let Some(Some(agreed)) = direct(i).and_then(|entry| by_callee.get(&entry)) {
            verdicts[i] = *agreed;
        }
    }
}

/// Settle the extrapop of every call of `data` before `ActionExtraPopSetup`
/// plants it. `sp` and `size` are the stack pointer's storage. Returns `true`
/// when calls that do not return were found and a restart was requested.
pub fn settle(data: &mut Funcdata, sp: &Address, size: int4) -> bool {
    if !data.get_arch().sp_solve || data.is_jumptable_recovery_on() || data.num_calls() == 0 {
        return false;
    }
    let verdicts = solve_calls(data, sp, size);
    let mut restart = false;
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let model = fc.get_extra_pop();
        if model == EXTRAPOP_UNKNOWN {
            continue;
        }
        let proven = fc.callee_returns().and_then(|r| r.proven);
        let verdict = verdicts.as_ref().and_then(|v| v.get(i as usize)).copied().unwrap_or(Verdict::Keep);
        let extrapop = match (proven, verdict) {
            (_, Verdict::NoReturn) => {
                let at = data.obank().get(fc.get_op()).map(|o| o.get_addr().clone());
                if let Some(at) = at {
                    restart |= data.get_override_mut().insert_no_return_call(at);
                }
                None
            }
            (Some(ep), _) => Some(ep),
            (None, Verdict::Pop(extra)) => int4::try_from(extra).ok().and_then(|e| model.checked_add(e)),
            (None, Verdict::Keep) => None,
        };
        if let Some(ep) = extrapop.filter(|ep| *ep != model) {
            data.get_call_specs_mut(i).proto_mut().set_extra_pop(ep);
        }
    }
    if restart {
        data.set_restart_pending(true);
        data.kuna_set_restart_now();
    }
    restart
}

#[cfg(test)]
mod tests;
