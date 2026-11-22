//! (kuna) Fold a branchless two-word compare back into one double-precision
//! compare (P5).
//!
//! On a 32-bit target a 64-bit `a < b` is a subtract-with-borrow chain: the low
//! words are compared (`cmp`/`subs`) and the high words subtracted with that
//! borrow (`sbb`/`sbcs`), and the condition is read off the flags of the second
//! step. Lifted and simplified, the result is a tree over the high words and the
//! low-word borrow `c = alo < blo`:
//!
//! * signed `a s< b`: the XOR of `SBORROW(ahi,bhi)`, `SBORROW(ahi-bhi,c)` and
//!   `(ahi-bhi-c) s< 0` (the overflow and sign flags of the borrowing subtract),
//!   or `ahi-bhi s< c` once two of those have been folded together;
//! * unsigned `a < b`: `ahi < bhi || ahi-bhi < c` (borrow out), or
//!   `ahi <= bhi && 1-c <= ahi-bhi` where the target has a carry flag (`!(a >= b)`).
//!
//! Upstream's `LessThreeWay` only recognizes the branching form of the same
//! compare. [`apply_rule`] is tried from the low-word compare of a split value,
//! as the other double-precision forms are: it collects the boolean ops the
//! borrow reaches, reads each operand of their leaf compares as a linear sum of
//! varnodes and the borrow, and replaces the root with one compare of the two
//! wholes when the leaves are exactly one of the shapes above (either polarity).
//! A high word of `b` that is the constant 0 lets the terms that vanish with it
//! be absent (`ahi s< c`, `ahi < c`).

use std::collections::BTreeMap;

use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::double::{prepare_bool_op, replace_bool_op, SplitVarnode};
use crate::funcdata::Funcdata;

const MAX_DEPTH: u32 = 8;
const MAX_REACH: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Leaf {
    Vn(VarnodeId),
    Borrow,
}

/// A value as a sum of leaves times coefficients plus a constant, modulo the
/// high-word width.
#[derive(Clone, PartialEq, Eq)]
struct Lin {
    terms: BTreeMap<Leaf, uintb>,
    k: uintb,
}

impl Lin {
    fn konst(k: uintb, mask: uintb) -> Lin {
        Lin { terms: BTreeMap::new(), k: k & mask }
    }
    fn leaf(l: Leaf) -> Lin {
        let mut terms = BTreeMap::new();
        terms.insert(l, 1);
        Lin { terms, k: 0 }
    }
    fn add(&self, o: &Lin, mask: uintb) -> Lin {
        let mut r = self.clone();
        for (l, c) in &o.terms {
            let e = r.terms.entry(*l).or_insert(0);
            *e = e.wrapping_add(*c) & mask;
        }
        r.terms.retain(|_, c| *c != 0);
        r.k = r.k.wrapping_add(o.k) & mask;
        r
    }
    fn scale(&self, s: uintb, mask: uintb) -> Lin {
        let mut r = Lin { terms: BTreeMap::new(), k: self.k.wrapping_mul(s) & mask };
        for (l, c) in &self.terms {
            let v = c.wrapping_mul(s) & mask;
            if v != 0 {
                r.terms.insert(*l, v);
            }
        }
        r
    }
    fn sub(&self, o: &Lin, mask: uintb) -> Lin {
        self.add(&o.scale(mask, mask), mask)
    }
    /// The varnode or constant this sum is, if it is a single one.
    fn single(&self) -> Option<Lin> {
        match self.terms.len() {
            0 => Some(self.clone()),
            1 if self.k == 0 && self.terms.values().all(|c| *c == 1) => {
                matches!(self.terms.keys().next(), Some(Leaf::Vn(_))).then(|| self.clone())
            }
            _ => None,
        }
    }
}

struct Ctx {
    xlo: VarnodeId,
    ylo: VarnodeId,
    /// The low-word carry of an addition, read as the borrow leaf as well.
    carry: Option<VarnodeId>,
    size: int4,
    mask: uintb,
}

fn vn_const(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

fn same_value(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    a == b || matches!((vn_const(data, a), vn_const(data, b)), (Some(x), Some(y)) if x == y)
}

fn def_of(data: &Funcdata, vn: VarnodeId) -> Option<(OpId, OpCode)> {
    let op = data.vbank().get(vn)?.get_def()?;
    Some((op, data.obank().get(op)?.code()))
}

fn input(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op)?.get_in(slot)
}

fn size_of(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

/// Whether the boolean `vn` is the low-word borrow `xlo < ylo` (true) or its
/// complement (false).
fn borrow_polarity(data: &Funcdata, cx: &Ctx, vn: VarnodeId, depth: u32) -> Option<bool> {
    if depth > MAX_DEPTH {
        return None;
    }
    if cx.carry == Some(vn) {
        return Some(true);
    }
    let (op, opc) = def_of(data, vn)?;
    let a = input(data, op, 0)?;
    match opc {
        OpCode::CPUI_BOOL_NEGATE => borrow_polarity(data, cx, a, depth + 1).map(|p| !p),
        OpCode::CPUI_COPY => borrow_polarity(data, cx, a, depth + 1),
        OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => {
            let b = input(data, op, 1)?;
            if opc == OpCode::CPUI_INT_LESS && same_value(data, a, cx.xlo) && same_value(data, b, cx.ylo) {
                Some(true)
            } else if opc == OpCode::CPUI_INT_LESSEQUAL && same_value(data, a, cx.ylo) && same_value(data, b, cx.xlo) {
                Some(false)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn lin_of(data: &Funcdata, cx: &Ctx, vn: VarnodeId, depth: u32) -> Option<Lin> {
    if let Some(k) = vn_const(data, vn) {
        return Some(Lin::konst(k, cx.mask));
    }
    if size_of(data, vn) != cx.size || depth > MAX_DEPTH {
        return None;
    }
    let mask = cx.mask;
    let Some((op, opc)) = def_of(data, vn) else {
        return Some(Lin::leaf(Leaf::Vn(vn)));
    };
    let arg = |slot: int4| -> Option<Lin> { lin_of(data, cx, input(data, op, slot)?, depth + 1) };
    match opc {
        OpCode::CPUI_COPY => arg(0),
        OpCode::CPUI_INT_ADD => Some(arg(0)?.add(&arg(1)?, mask)),
        OpCode::CPUI_INT_SUB => Some(arg(0)?.sub(&arg(1)?, mask)),
        OpCode::CPUI_INT_2COMP => Some(Lin::konst(0, mask).sub(&arg(0)?, mask)),
        OpCode::CPUI_INT_NEGATE => Some(Lin::konst(mask, mask).sub(&arg(0)?, mask)),
        OpCode::CPUI_INT_MULT => {
            let k = vn_const(data, input(data, op, 1)?);
            match k {
                Some(k) => Some(arg(0)?.scale(k, mask)),
                None => Some(Lin::leaf(Leaf::Vn(vn))),
            }
        }
        OpCode::CPUI_INT_ZEXT => {
            let b = input(data, op, 0)?;
            if size_of(data, b) == 1 {
                if let Some(p) = borrow_polarity(data, cx, b, depth + 1) {
                    let c = Lin::leaf(Leaf::Borrow);
                    return Some(if p { c } else { Lin::konst(1, mask).sub(&c, mask) });
                }
            }
            Some(Lin::leaf(Leaf::Vn(vn)))
        }
        _ => Some(Lin::leaf(Leaf::Vn(vn))),
    }
}

/// One leaf compare `p OP q` of the boolean tree, after `<=` has been turned
/// into a negated `<`.
#[derive(Clone)]
struct Atom {
    signed_borrow: bool,
    signed: bool,
    p: Lin,
    q: Lin,
    neg: bool,
}

fn atom_of(data: &Funcdata, cx: &Ctx, op: OpId, opc: OpCode, neg: bool) -> Option<Atom> {
    let a = input(data, op, 0)?;
    let b = input(data, op, 1)?;
    let (sb, signed, swap, flip) = match opc {
        OpCode::CPUI_INT_SBORROW => (true, true, false, false),
        OpCode::CPUI_INT_SLESS => (false, true, false, false),
        OpCode::CPUI_INT_SLESSEQUAL => (false, true, true, true),
        OpCode::CPUI_INT_LESS => (false, false, false, false),
        OpCode::CPUI_INT_LESSEQUAL => (false, false, true, true),
        _ => return None,
    };
    let (a, b) = if swap { (b, a) } else { (a, b) };
    Some(Atom {
        signed_borrow: sb,
        signed,
        p: lin_of(data, cx, a, 0)?,
        q: lin_of(data, cx, b, 0)?,
        neg: neg != flip,
    })
}

/// Flatten the XOR tree under `vn` (`!=`, `==`, `^^`, `!`) into its leaf
/// compares, folding every negation into the returned parity.
fn xor_leaves(data: &Funcdata, cx: &Ctx, vn: VarnodeId, depth: u32, out: &mut Vec<Atom>) -> Option<bool> {
    if depth > MAX_DEPTH || out.len() > 4 {
        return None;
    }
    let (op, opc) = def_of(data, vn)?;
    match opc {
        OpCode::CPUI_BOOL_NEGATE => xor_leaves(data, cx, input(data, op, 0)?, depth + 1, out).map(|p| !p),
        OpCode::CPUI_BOOL_XOR | OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_EQUAL => {
            let a = input(data, op, 0)?;
            let b = input(data, op, 1)?;
            if size_of(data, a) != 1 || size_of(data, b) != 1 {
                return None;
            }
            let pa = xor_leaves(data, cx, a, depth + 1, out)?;
            let pb = xor_leaves(data, cx, b, depth + 1, out)?;
            Some((pa != pb) != (opc == OpCode::CPUI_INT_EQUAL))
        }
        _ => {
            out.push(atom_of(data, cx, op, opc, false)?);
            Some(false)
        }
    }
}

/// Flatten the OR/AND tree under `vn` into a disjunction of leaf compares:
/// returns `pol` with `vn == pol ^ OR(leaves)`.
fn or_leaves(data: &Funcdata, cx: &Ctx, vn: VarnodeId, neg: bool, depth: u32, out: &mut Vec<Atom>) -> Option<bool> {
    if depth > MAX_DEPTH || out.len() > 2 {
        return None;
    }
    let (op, opc) = def_of(data, vn)?;
    match opc {
        OpCode::CPUI_BOOL_NEGATE => or_leaves(data, cx, input(data, op, 0)?, !neg, depth + 1, out),
        OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_AND => {
            // An OR under an even number of negations and an AND under an odd one
            // join disjuncts; the other two would need distribution.
            if (opc == OpCode::CPUI_BOOL_AND) != neg {
                return None;
            }
            or_leaves(data, cx, input(data, op, 0)?, neg, depth + 1, out)?;
            or_leaves(data, cx, input(data, op, 1)?, neg, depth + 1, out)?;
            Some(neg)
        }
        _ => {
            out.push(atom_of(data, cx, op, opc, neg)?);
            Some(neg)
        }
    }
}

/// Whether the leaves are the signed borrowing subtract `xh - yh - c`'s
/// overflow XOR sign: `xh s< yh` across both words, up to their parity.
fn signed_match(atoms: &[Atom], xh: &Lin, yh: &Lin, mask: uintb) -> Option<bool> {
    let c = Lin::leaf(Leaf::Borrow);
    let d = xh.sub(yh, mask);
    let zero = Lin::konst(0, mask);
    let (mut t1, mut t2, mut t3) = (0, 0, 0);
    let mut parity = false;
    for a in atoms {
        if !a.signed {
            return None;
        }
        parity ^= a.neg;
        if a.signed_borrow {
            if &a.p == xh && &a.q == yh {
                t1 += 1;
            } else if &a.p == &d && &a.q == &c {
                t2 += 1;
            } else {
                return None;
            }
        } else if &a.q == &zero && &a.p == &d.sub(&c, mask) {
            t3 += 1;
        } else if &a.p == &d && &a.q == &c {
            t2 += 1;
            t3 += 1;
        } else {
            return None;
        }
    }
    let t1_ok = t1 == 1 || (t1 == 0 && yh == &zero);
    (t1_ok && t2 == 1 && t3 == 1).then_some(parity)
}

/// Whether the leaves are the unsigned borrow (or carry) out of `xh - yh - c`:
/// returns the parity relative to `x < y` across both words.
fn unsigned_match(atoms: &[Atom], pol: bool, xh: &Lin, yh: &Lin, mask: uintb) -> Option<bool> {
    if atoms.iter().any(|a| a.signed || a.neg) {
        return None;
    }
    let c = Lin::leaf(Leaf::Borrow);
    let notc = Lin::konst(1, mask).sub(&c, mask);
    let d = xh.sub(yh, mask);
    let has = |p: &Lin, q: &Lin| atoms.iter().any(|a| &a.p == p && &a.q == q);
    let zero = Lin::konst(0, mask);
    match atoms.len() {
        2 if has(xh, yh) && has(&d, &c) => Some(pol),
        2 if has(yh, xh) && has(&d, &notc) => Some(!pol),
        1 if yh == &zero && has(xh, &c) => Some(pol),
        _ => None,
    }
}

/// Candidate values of the unknown high word, read off the leaves.
fn other_hi_candidates(atoms: &[Atom], known: &Lin, known_is_x: bool, mask: uintb) -> Vec<Lin> {
    let c = Lin::leaf(Leaf::Borrow);
    let mut out: Vec<Lin> = Vec::new();
    let mut push = |l: Lin| {
        if let Some(s) = l.single() {
            if !out.contains(&s) {
                out.push(s);
            }
        }
    };
    for a in atoms {
        for side in [&a.p, &a.q] {
            push(side.clone());
            for d in [side.clone(), side.add(&c, mask)] {
                // d = xh - yh
                if known_is_x {
                    push(known.sub(&d, mask));
                } else {
                    push(known.add(&d, mask));
                }
            }
        }
    }
    out.push(Lin::konst(0, mask));
    out
}

/// The ops the low-word borrow reaches through arithmetic and boolean ops,
/// outermost first.
fn reach(data: &Funcdata, start: VarnodeId) -> Vec<OpId> {
    let mut seen: Vec<OpId> = Vec::new();
    let mut work: Vec<(VarnodeId, u32)> = vec![(start, 0)];
    while let Some((vn, depth)) = work.pop() {
        if depth >= MAX_DEPTH {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else { continue };
        for op in v.descend_iter() {
            if seen.contains(&op) || seen.len() >= MAX_REACH {
                continue;
            }
            let Some(o) = data.obank().get(op) else { continue };
            let follow = matches!(
                o.code(),
                OpCode::CPUI_COPY
                    | OpCode::CPUI_INT_ZEXT
                    | OpCode::CPUI_BOOL_NEGATE
                    | OpCode::CPUI_INT_ADD
                    | OpCode::CPUI_INT_SUB
                    | OpCode::CPUI_INT_MULT
                    | OpCode::CPUI_INT_2COMP
                    | OpCode::CPUI_INT_NEGATE
                    | OpCode::CPUI_INT_SBORROW
                    | OpCode::CPUI_INT_LESS
                    | OpCode::CPUI_INT_LESSEQUAL
                    | OpCode::CPUI_INT_SLESS
                    | OpCode::CPUI_INT_SLESSEQUAL
                    | OpCode::CPUI_INT_EQUAL
                    | OpCode::CPUI_INT_NOTEQUAL
                    | OpCode::CPUI_BOOL_XOR
                    | OpCode::CPUI_BOOL_OR
                    | OpCode::CPUI_BOOL_AND
            );
            if !follow {
                continue;
            }
            seen.push(op);
            if let Some(out) = o.get_out() {
                work.push((out, depth + 1));
            }
        }
    }
    seen.reverse();
    seen
}

/// The binary boolean ops the borrow reaches, outermost first, so a whole
/// compare is folded before any of its parts.
fn roots(data: &Funcdata, start: VarnodeId) -> Vec<OpId> {
    let mut seen = reach(data, start);
    seen.retain(|op| {
        data.obank().get(*op).is_some_and(|o| {
            o.num_input() == 2 && o.get_out().is_some_and(|v| size_of(data, v) == 1) && o.code() != OpCode::CPUI_INT_SBORROW
        })
    });
    seen
}

/// Try the fold from the low-word compare `loop_` reading `i`'s low piece
/// (called from `apply_rule_in` alongside upstream's compare forms).
pub(crate) fn apply_rule(i: &SplitVarnode, loop_: OpId, workishi: bool, data: &mut Funcdata) -> bool {
    if workishi {
        return false;
    }
    let (Some(lo), Some(hi)) = (i.get_lo(), i.get_hi()) else { return false };
    let Some(o) = data.obank().get(loop_) else { return false };
    let opc = o.code();
    let (Some(a), Some(b), Some(cvn)) = (o.get_in(0), o.get_in(1), o.get_out()) else { return false };
    // INT_LESS(x,y) is the borrow of x - y; INT_LESSEQUAL(y,x) its complement.
    let (xlo, ylo) = match opc {
        OpCode::CPUI_INT_LESS => (a, b),
        OpCode::CPUI_INT_LESSEQUAL => (b, a),
        _ => return false,
    };
    let known_is_x = if xlo == lo {
        true
    } else if ylo == lo {
        false
    } else {
        return false;
    };
    let size = size_of(data, hi);
    if size <= 0 || size > 4 || size_of(data, lo) != size {
        return false;
    }
    let cx = Ctx { xlo, ylo, carry: None, size, mask: calc_mask(size) };
    let Some(known) = lin_of(data, &cx, hi, 0) else { return false };
    let other_lo = if known_is_x { ylo } else { xlo };

    for root in roots(data, cvn) {
        let Some(rout) = data.obank().get(root).and_then(|o| o.get_out()) else { continue };
        let mut xs = Vec::new();
        let xpar = xor_leaves(data, &cx, rout, 0, &mut xs);
        let mut os = Vec::new();
        let mut opol = or_leaves(data, &cx, rout, false, 0, &mut os);
        if opol.is_none() {
            os.clear();
            opol = or_leaves(data, &cx, rout, true, 0, &mut os);
        }
        let mut cands: Vec<Lin> = Vec::new();
        for c in other_hi_candidates(&xs, &known, known_is_x, cx.mask)
            .into_iter()
            .chain(other_hi_candidates(&os, &known, known_is_x, cx.mask))
        {
            if !cands.contains(&c) {
                cands.push(c);
            }
        }
        for other in cands {
            let (xh, yh) = if known_is_x { (&known, &other) } else { (&other, &known) };
            let found = xpar
                .and_then(|p| signed_match(&xs, xh, yh, cx.mask).map(|q| (true, p ^ q)))
                .or_else(|| opol.and_then(|p| unsigned_match(&os, p, xh, yh, cx.mask).map(|q| (false, q))));
            let Some((signed, negated)) = found else { continue };
            if replace(data, i, root, other_lo, &other, known_is_x, signed, negated) {
                return true;
            }
        }
    }
    false
}

/// Rewrite `root` as one compare of the two wholes: `x < y`, or `y <= x` when
/// the tree computes the complement.
#[allow(clippy::too_many_arguments)]
fn replace(
    data: &mut Funcdata,
    i: &SplitVarnode,
    root: OpId,
    other_lo: VarnodeId,
    other_hi: &Lin,
    known_is_x: bool,
    signed: bool,
    negated: bool,
) -> bool {
    let wholesize = i.get_size();
    let losize = size_of(data, other_lo);
    let mut other = SplitVarnode::empty();
    match (other_hi.terms.keys().next(), vn_const(data, other_lo)) {
        (None, Some(lo)) => other.init_partial_const(wholesize, (other_hi.k << (8 * losize)) | lo),
        (None, None) => {
            let h = data.new_constant(losize, other_hi.k);
            other.init_partial_pieces(data, wholesize, Some(other_lo), Some(h));
        }
        (Some(Leaf::Vn(h)), _) => other.init_partial_pieces(data, wholesize, Some(other_lo), Some(*h)),
        (Some(Leaf::Borrow), _) => return false,
    }
    if other.exceeds_const_precision() {
        return false;
    }
    let (mut x, mut y) = if known_is_x { (i.clone(), other) } else { (other, i.clone()) };
    let opc = match (signed, negated) {
        (true, false) => OpCode::CPUI_INT_SLESS,
        (true, true) => OpCode::CPUI_INT_SLESSEQUAL,
        (false, false) => OpCode::CPUI_INT_LESS,
        (false, true) => OpCode::CPUI_INT_LESSEQUAL,
    };
    if negated {
        std::mem::swap(&mut x, &mut y);
    }
    if !prepare_bool_op(data, &mut x, &mut y, root) {
        return false;
    }
    replace_bool_op(data, root, &mut x, &mut y, opc);
    true
}

/// (kuna) Join two adjacent unlocked input words into one double-precision
/// input when the low word's borrow or carry is consumed together with the high
/// word (option `pairparam`).
///
/// A `long long` parameter on a 32-bit target arrives in two stack words or two
/// registers, and with no prototype upstream recovers two `int` parameters, so
/// the double-precision forms never see a whole to fold onto. The rule fires on
/// the low-word compare or carry `c = xlo < ylo` (`CARRY(xlo,ylo)`) when `xlo`
/// (or `ylo`) is an input and the input word directly above it appears, with
/// coefficient +-1, in the same sum as `c` among the values `c` reaches: the
/// propagation of a borrow or carry from one word into the next is what makes
/// the two words one value. The pair is merged with upstream's
/// `combineInputVarnodes`, its pieces are marked as double-precision pieces, and
/// `RuleDoubleIn` folds the arithmetic or compare onto the whole, which
/// prototype recovery then reports as one 8-byte parameter.
pub struct RuleDoubleInputPair {
    group: String,
}

impl RuleDoubleInputPair {
    pub fn new(g: impl Into<String>) -> RuleDoubleInputPair {
        RuleDoubleInputPair { group: g.into() }
    }
}

/// The unlocked input word directly above `lo` in the same space, if `lo` is
/// itself an unlocked input, the function never writes either word (a compiler
/// that spills into an argument slot writes one half, which a joined parameter
/// can only print as a partial store), and the prototype model accepts the two
/// words as one parameter (a stack slot; not two separate argument registers).
fn input_above(data: &Funcdata, lo: VarnodeId) -> Option<VarnodeId> {
    let v = data.vbank().get(lo)?;
    if !v.is_input() || v.is_type_lock() || v.is_constant() {
        return None;
    }
    let size = v.get_size();
    let addr = v.get_addr().clone();
    let hiaddr = if addr.is_big_endian() { &addr + -(size as i64) } else { &addr + size as i64 };
    let hi = data.find_varnode_input(size, &hiaddr)?;
    let h = data.vbank().get(hi)?;
    let base = if addr.is_big_endian() { hiaddr } else { addr };
    let end = &base + 2 * size as i64;
    let written = data.vbank().iter_loc_addr_range(&base, &end).any(|id| data.vbank().get(id).is_some_and(|v| v.is_written()));
    (!h.is_type_lock() && !written && data.get_func_proto().possible_input_param(&base, 2 * size)).then_some(hi)
}

/// Whether some value the carry reaches sums the carry with `hi`.
fn carried_into(sums: &[Lin], hi: VarnodeId, mask: uintb) -> bool {
    let unit = |k: Option<&uintb>| matches!(k, Some(&k) if k == 1 || k == mask);
    sums.iter().any(|l| unit(l.terms.get(&Leaf::Borrow)) && unit(l.terms.get(&Leaf::Vn(hi))))
}

fn join(data: &mut Funcdata, lo: VarnodeId, hi: VarnodeId) -> bool {
    let size = size_of(data, lo);
    let Some(addr) = data.vbank().get(lo).map(|v| v.get_addr().clone()) else { return false };
    let base = if addr.is_big_endian() { &addr + -(size as i64) } else { addr };
    if data.combine_input_varnodes(hi, lo).is_err() {
        return false;
    }
    let Some(whole) = data.find_varnode_input(2 * size, &base) else { return false };
    let subs: Vec<OpId> = data.vbank().get(whole).map(|v| v.descend_iter().collect()).unwrap_or_default();
    for op in subs {
        let Some(o) = data.obank().get(op) else { continue };
        if o.code() != OpCode::CPUI_SUBPIECE {
            continue;
        }
        let (Some(out), Some(off)) = (o.get_out(), o.get_in(1).and_then(|c| vn_const(data, c))) else { continue };
        let Some(v) = data.vbank_mut().get_mut(out) else { continue };
        if off == 0 {
            v.set_precis_lo();
        } else if off == size as uintb {
            v.set_precis_hi();
        }
    }
    true
}

impl crate::action::Rule for RuleDoubleInputPair {
    fn clone_rule(&self, grouplist: &crate::action::ActionGroupList) -> Option<Box<dyn crate::action::Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDoubleInputPair::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_LESS, OpCode::CPUI_INT_LESSEQUAL, OpCode::CPUI_INT_CARRY]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().pair_params || data.get_func_proto().is_input_locked() {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let opc = o.code();
        let (Some(a), Some(b), Some(c)) = (o.get_in(0), o.get_in(1), o.get_out()) else { return 0 };
        let (xlo, ylo) = if opc == OpCode::CPUI_INT_LESSEQUAL { (b, a) } else { (a, b) };
        let xhi = input_above(data, xlo);
        let yhi = input_above(data, ylo);
        if xhi.is_none() && yhi.is_none() {
            return 0;
        }
        let size = size_of(data, xlo);
        if size <= 0 || size > 4 {
            return 0;
        }
        let cx = Ctx {
            xlo,
            ylo,
            carry: (opc == OpCode::CPUI_INT_CARRY).then_some(c),
            size,
            mask: calc_mask(size),
        };
        let mut sums: Vec<Lin> = Vec::new();
        for r in reach(data, c) {
            let Some(ro) = data.obank().get(r) else { continue };
            let Some(out) = ro.get_out() else { continue };
            let (ri0, ri1, rcode) = (ro.get_in(0), ro.get_in(1), ro.code());
            if size_of(data, out) == size {
                if let Some(l) = lin_of(data, &cx, out, 0) {
                    sums.push(l);
                }
            } else if matches!(
                rcode,
                OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL
            ) {
                let (Some(p), Some(q)) = (ri0, ri1) else { continue };
                if let (Some(p), Some(q)) = (lin_of(data, &cx, p, 0), lin_of(data, &cx, q, 0)) {
                    sums.push(p.sub(&q, cx.mask));
                }
            }
        }
        let mut joined = false;
        for (lo, hi) in [(xlo, xhi), (ylo, yhi)] {
            let Some(hi) = hi else { continue };
            if data.vbank().get(lo).is_some_and(|v| v.is_input()) && carried_into(&sums, hi, cx.mask) {
                joined |= join(data, lo, hi);
            }
        }
        joined as int4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: uintb = 0xffff_ffff;

    fn ids() -> Vec<VarnodeId> {
        let mut map: slotmap::SlotMap<VarnodeId, ()> = slotmap::SlotMap::with_key();
        (0..10).map(|_| map.insert(())).collect()
    }
    fn v(n: usize) -> Lin {
        Lin::leaf(Leaf::Vn(ids()[n]))
    }
    fn atom(signed_borrow: bool, signed: bool, p: Lin, q: Lin, neg: bool) -> Atom {
        Atom { signed_borrow, signed, p, q, neg }
    }

    #[test]
    fn signed_borrow_xor_tree_is_a_signed_compare() {
        let (xh, yh) = (v(1), v(2));
        let c = Lin::leaf(Leaf::Borrow);
        let d = xh.sub(&yh, M);
        let atoms = [
            atom(true, true, xh.clone(), yh.clone(), false),
            atom(true, true, d.clone(), c.clone(), false),
            atom(false, true, d.sub(&c, M), Lin::konst(0, M), false),
        ];
        assert_eq!(signed_match(&atoms, &xh, &yh, M), Some(false));
        let folded = [atom(false, true, d.clone(), c.clone(), true)];
        assert_eq!(signed_match(&folded, &xh, &Lin::konst(0, M), M), None);
        let zero = Lin::konst(0, M);
        let against_zero = [atom(false, true, xh.clone(), c.clone(), true)];
        assert_eq!(signed_match(&against_zero, &xh, &zero, M), Some(true));
        let extra = [atoms[0].clone(), atoms[1].clone(), atom(false, true, xh.clone(), zero, false)];
        assert_eq!(signed_match(&extra, &xh, &yh, M), None);
    }

    #[test]
    fn unsigned_borrow_and_carry_out() {
        let (xh, yh) = (v(1), v(2));
        let c = Lin::leaf(Leaf::Borrow);
        let d = xh.sub(&yh, M);
        let borrow = [atom(false, false, xh.clone(), yh.clone(), false), atom(false, false, d.clone(), c.clone(), false)];
        assert_eq!(unsigned_match(&borrow, false, &xh, &yh, M), Some(false));
        let notc = Lin::konst(1, M).sub(&c, M);
        let carry = [atom(false, false, yh.clone(), xh.clone(), false), atom(false, false, d.clone(), notc, false)];
        assert_eq!(unsigned_match(&carry, false, &xh, &yh, M), Some(true));
        let wrong = [atom(false, false, xh.clone(), yh.clone(), false), atom(false, false, d, Lin::konst(1, M), false)];
        assert_eq!(unsigned_match(&wrong, false, &xh, &yh, M), None);
    }

    #[test]
    fn carried_into_needs_the_carry_and_the_high_word_in_one_sum() {
        let c = Lin::leaf(Leaf::Borrow);
        let hi = ids()[7];
        let sum = v(7).add(&v(8), M).add(&c, M);
        assert!(carried_into(&[sum], hi, M));
        let apart = [v(7).add(&v(8), M), v(9).add(&c, M)];
        assert!(!carried_into(&apart, hi, M));
    }
}
