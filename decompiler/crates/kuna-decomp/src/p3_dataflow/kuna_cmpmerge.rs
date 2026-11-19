//! P3 merge of two comparisons on the same operands (kuna `cmpmerge`).
//!
//! Two tests of the same values, combined by `||`, `&&` or `^`, or by two
//! branches in a row to the same target, often decide one relation a single
//! comparison states: `a < b || a == b` is `a <= b`, and on floats
//! `a != b && !(a < b)` is `!(a <= b)`.  A comparison's outcome is a set of the
//! relations `LT`, `EQ`, `GT` (and `UN`, unordered, for floats), so the pair is
//! the union, intersection or symmetric difference of two sets; it folds when
//! that set is one comparison or its negation.  A comparison of one varnode
//! against two constants is a pair of `CircleRange`s instead, which folds when
//! the combined range is one comparison against a constant.
//!
//! [`RuleCompareMerge`] folds `BOOL_OR`, `BOOL_AND` and `BOOL_XOR` of two
//! comparisons.  [`ActionBranchMerge`] folds a branch whose one successor is a
//! block that holds nothing but a second test of the same operands and shares
//! the branch's other target: the first branch takes the combined test and the
//! second can no longer reach the shared target, so `ActionDeterminedBranch`
//! removes its edge.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult, Rule};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::rangeutil::CircleRange;

const LT: u8 = 1;
const EQ: u8 = 2;
const GT: u8 = 4;
const UN: u8 = 8;

/// A comparison as the set of relations between `a` and `b` that make it true.
#[derive(Clone, Copy)]
struct Relation {
    a: VarnodeId,
    b: VarnodeId,
    mask: u8,
    float: bool,
    /// `Some(true)` signed, `Some(false)` unsigned, `None` either (`==`, `!=`).
    signed: Option<bool>,
}

impl Relation {
    fn all(&self) -> u8 {
        if self.float { LT | EQ | GT | UN } else { LT | EQ | GT }
    }
    fn swapped(self) -> Relation {
        let m = self.mask;
        let mask = (m & !(LT | GT)) | if m & LT != 0 { GT } else { 0 } | if m & GT != 0 { LT } else { 0 };
        Relation { a: self.b, b: self.a, mask, ..self }
    }
}

fn op_of(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn)?.get_def()
}

fn same_value(data: &Funcdata, x: VarnodeId, y: VarnodeId) -> bool {
    if x == y {
        return true;
    }
    match (data.vbank().get(x), data.vbank().get(y)) {
        (Some(a), Some(b)) => a.is_constant() && b.is_constant() && a.get_offset() == b.get_offset() && a.get_size() == b.get_size(),
        _ => false,
    }
}

/// The relation `bool_vn` states, through `BOOL_NEGATE`s.
fn relation(data: &Funcdata, bool_vn: VarnodeId) -> Option<Relation> {
    let op = op_of(data, bool_vn)?;
    let o = data.obank().get(op)?;
    if o.code() == OpCode::CPUI_BOOL_NEGATE {
        let mut r = relation(data, o.get_in(0)?)?;
        r.mask = r.all() & !r.mask;
        return Some(r);
    }
    if o.num_input() != 2 {
        return None;
    }
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    let (mask, float, signed) = match o.code() {
        OpCode::CPUI_INT_EQUAL => (EQ, false, None),
        OpCode::CPUI_INT_NOTEQUAL => (LT | GT, false, None),
        OpCode::CPUI_INT_LESS => (LT, false, Some(false)),
        OpCode::CPUI_INT_LESSEQUAL => (LT | EQ, false, Some(false)),
        OpCode::CPUI_INT_SLESS => (LT, false, Some(true)),
        OpCode::CPUI_INT_SLESSEQUAL => (LT | EQ, false, Some(true)),
        OpCode::CPUI_FLOAT_EQUAL => (EQ, true, None),
        OpCode::CPUI_FLOAT_NOTEQUAL => (LT | GT | UN, true, None),
        OpCode::CPUI_FLOAT_LESS => (LT, true, None),
        OpCode::CPUI_FLOAT_LESSEQUAL => (LT | EQ, true, None),
        _ => return None,
    };
    Some(Relation { a, b, mask, float, signed })
}

/// The two relations over one operand pair, the second oriented like the first.
fn pair(data: &Funcdata, x: VarnodeId, y: VarnodeId) -> Option<(Relation, Relation)> {
    let r1 = relation(data, x)?;
    let mut r2 = relation(data, y)?;
    if r1.float != r2.float {
        return None;
    }
    if !(same_value(data, r1.a, r2.a) && same_value(data, r1.b, r2.b)) {
        r2 = r2.swapped();
        if !(same_value(data, r1.a, r2.a) && same_value(data, r1.b, r2.b)) {
            return None;
        }
    }
    let signed = match (r1.signed, r2.signed) {
        (Some(s), Some(t)) if s != t => return None,
        (s, t) => s.or(t),
    };
    Some((Relation { signed, ..r1 }, r2))
}

/// One comparison op (opcode, swap the operands) stating exactly `mask`.
fn single(mask: u8, float: bool, signed: Option<bool>) -> Option<(OpCode, bool)> {
    use OpCode::*;
    let order = match (float, signed) {
        (true, _) => Some((CPUI_FLOAT_LESS, CPUI_FLOAT_LESSEQUAL)),
        (false, Some(true)) => Some((CPUI_INT_SLESS, CPUI_INT_SLESSEQUAL)),
        (false, Some(false)) => Some((CPUI_INT_LESS, CPUI_INT_LESSEQUAL)),
        (false, None) => None,
    };
    let ne = if float { LT | GT | UN } else { LT | GT };
    match mask {
        m if m == EQ => Some((if float { CPUI_FLOAT_EQUAL } else { CPUI_INT_EQUAL }, false)),
        m if m == ne => Some((if float { CPUI_FLOAT_NOTEQUAL } else { CPUI_INT_NOTEQUAL }, false)),
        m if m == LT => order.map(|(lt, _)| (lt, false)),
        m if m == GT => order.map(|(lt, _)| (lt, true)),
        m if m == LT | EQ => order.map(|(_, le)| (le, false)),
        m if m == GT | EQ => order.map(|(_, le)| (le, true)),
        _ => None,
    }
}

/// What a set of relations becomes.
enum Form {
    Constant(bool),
    /// The combination is one of the two inputs.
    Reuse(VarnodeId),
    Compare { opc: OpCode, a: VarnodeId, b: VarnodeId, negate: bool },
}

fn form_for(r: &Relation, mask: u8) -> Option<Form> {
    let all = r.all();
    if mask == 0 || mask == all {
        return Some(Form::Constant(mask == all));
    }
    let (opc, swap, negate) = match single(mask, r.float, r.signed) {
        Some((opc, swap)) => (opc, swap, false),
        None => {
            let (opc, swap) = single(all & !mask, r.float, r.signed)?;
            (opc, swap, true)
        }
    };
    let (a, b) = if swap { (r.b, r.a) } else { (r.a, r.b) };
    Some(Form::Compare { opc, a, b, negate })
}

fn constant(data: &Funcdata, vn: VarnodeId) -> Option<uintb> {
    let v = data.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

/// The values of the non-constant operand for which `bool_vn` holds, when it
/// compares one varnode against a constant.
fn range_of(data: &Funcdata, bool_vn: VarnodeId) -> Option<(VarnodeId, CircleRange)> {
    let mut vn = bool_vn;
    let mut value = true;
    while let Some(op) = op_of(data, vn).filter(|&op| data.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_BOOL_NEGATE)) {
        value = !value;
        vn = data.obank().get(op)?.get_in(0)?;
    }
    let op = op_of(data, vn)?;
    let o = data.obank().get(op)?;
    if !matches!(
        o.code(),
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL
    ) {
        return None;
    }
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    let (v, k, slot) = match (constant(data, a), constant(data, b)) {
        (None, Some(k)) => (a, k, 0),
        (Some(k), None) => (b, k, 1),
        _ => return None,
    };
    let size = data.vbank().get(v)?.get_size();
    let mut range = CircleRange::new_bool(value);
    range.pull_back_binary(o.code(), k, slot, size, 1).then_some((v, range))
}

/// One comparison of `v` against a constant stating `range`, in the strict
/// canonical form (`c - 1 < v`, `v < c`) the rule pool keeps.
fn range_form(data: &mut Funcdata, v: VarnodeId, range: &CircleRange) -> Option<Form> {
    let size = data.vbank().get(v)?.get_size();
    if range.is_empty() || range.is_full() {
        return Some(Form::Constant(range.is_full()));
    }
    if range.get_step() != 1 {
        return None;
    }
    let mask = kuna_base::address::calc_mask(size);
    let smin = (mask >> 1) + 1;
    let (l, r) = (range.get_min(), range.get_end() & mask);
    let k = |data: &mut Funcdata, c: uintb| data.new_constant(size, c & mask);
    let f = if r == (l + 1) & mask {
        Form::Compare { opc: OpCode::CPUI_INT_EQUAL, a: v, b: k(data, l), negate: false }
    } else if l == (r + 1) & mask {
        Form::Compare { opc: OpCode::CPUI_INT_NOTEQUAL, a: v, b: k(data, r), negate: false }
    } else if l == 0 {
        Form::Compare { opc: OpCode::CPUI_INT_LESS, a: v, b: k(data, r), negate: false }
    } else if r == 0 {
        Form::Compare { opc: OpCode::CPUI_INT_LESS, a: k(data, l.wrapping_sub(1)), b: v, negate: false }
    } else if l == smin {
        Form::Compare { opc: OpCode::CPUI_INT_SLESS, a: v, b: k(data, r), negate: false }
    } else if r == smin {
        Form::Compare { opc: OpCode::CPUI_INT_SLESS, a: k(data, l.wrapping_sub(1)), b: v, negate: false }
    } else {
        return None;
    };
    Some(f)
}

#[derive(Clone, Copy, PartialEq)]
enum Combine {
    Or,
    And,
    Xor,
}

/// The form two booleans combined by `how` take, when one comparison states it.
/// Each boolean is read as itself (`true`) or as its negation (`false`).
fn merged(data: &mut Funcdata, (x, px): (VarnodeId, bool), (y, py): (VarnodeId, bool), how: Combine) -> Option<Form> {
    let reuse = |vn: VarnodeId, positive: bool| positive.then_some(Form::Reuse(vn));
    if let (Some((v1, mut r1)), Some((v2, mut r2))) = (range_of(data, x), range_of(data, y)) {
        if v1 == v2 && how != Combine::Xor && (px || r1.invert() == 0) && (py || r2.invert() == 0) {
            let mut r = r1.clone();
            let ok = match how {
                Combine::Or => r.circle_union(&r2) == 0,
                _ => r.intersect(&r2) == 0,
            };
            if ok && r.equals(&r1) {
                return reuse(x, px);
            }
            if ok && r.equals(&r2) {
                return reuse(y, py);
            }
            if ok {
                if let Some(f) = range_form(data, v1, &r) {
                    return Some(f);
                }
            }
        }
    }
    let (mut r1, mut r2) = pair(data, x, y)?;
    if !px {
        r1.mask = r1.all() & !r1.mask;
    }
    if !py {
        r2.mask = r2.all() & !r2.mask;
    }
    let mask = match how {
        Combine::Or => r1.mask | r2.mask,
        Combine::And => r1.mask & r2.mask,
        Combine::Xor => r1.mask ^ r2.mask,
    };
    if mask == r1.mask {
        return reuse(x, px);
    }
    if mask == r2.mask {
        return reuse(y, py);
    }
    form_for(&r1, mask)
}

/// Rewrite `op` (whose output is a boolean) to compute `form`; a negated
/// comparison gets a new comparison op in front of it.
fn rewrite(data: &mut Funcdata, op: OpId, form: Form) {
    let num = data.obank().get(op).map_or(0, |o| o.num_input());
    match form {
        Form::Reuse(vn) => {
            for slot in (1..num).rev() {
                data.op_remove_input(op, slot);
            }
            data.op_set_input(op, vn, 0).expect("cmpmerge: reuse");
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        }
        Form::Constant(value) => {
            let k = data.new_constant(1, value as uintb);
            for slot in (1..num).rev() {
                data.op_remove_input(op, slot);
            }
            data.op_set_input(op, k, 0).expect("cmpmerge: constant");
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        }
        Form::Compare { opc, a, b, negate: false } => {
            data.op_set_opcode(op, crate::typeop::type_op_for(opc));
            data.op_set_input(op, a, 0).expect("cmpmerge: a");
            data.op_set_input(op, b, 1).expect("cmpmerge: b");
        }
        Form::Compare { opc, a, b, negate: true } => {
            let addr = data.obank().get(op).unwrap().get_addr().clone();
            let cmp = data.new_op(2, addr);
            data.op_set_opcode(cmp, crate::typeop::type_op_for(opc));
            let out = data.new_unique_out(1, cmp).expect("cmpmerge: output");
            data.op_set_input(cmp, a, 0).expect("cmpmerge: a");
            data.op_set_input(cmp, b, 1).expect("cmpmerge: b");
            data.op_insert_before(cmp, op);
            for slot in (1..num).rev() {
                data.op_remove_input(op, slot);
            }
            data.op_set_input(op, out, 0).expect("cmpmerge: negated");
            data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_BOOL_NEGATE));
        }
    }
}

/// Did the comparison behind `vn` print as `<=` (or was it canonicalized from one)?
fn lessequal_origin(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut vn = vn;
    loop {
        let Some(o) = op_of(data, vn).and_then(|op| data.obank().get(op)) else { return false };
        match o.code() {
            OpCode::CPUI_BOOL_NEGATE => match o.get_in(0) {
                Some(v) => vn = v,
                None => return false,
            },
            OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESSEQUAL => return true,
            _ => return o.is_canonical_lessequal(),
        }
    }
}

/// Let `ActionPresentCompareForm` print the `<` comparison `bool_vn` holds (through
/// a `BOOL_NEGATE`) as the `<=` its inputs were written with.
fn keep_lessequal(data: &mut Funcdata, bool_vn: Option<VarnodeId>) {
    let mut vn = bool_vn;
    while let Some(op) = vn.and_then(|v| op_of(data, v)) {
        let code = data.obank().get(op).map(|o| o.code());
        match code {
            Some(OpCode::CPUI_BOOL_NEGATE) => vn = data.obank().get(op).and_then(|o| o.get_in(0)),
            Some(OpCode::CPUI_INT_LESS) | Some(OpCode::CPUI_INT_SLESS) => {
                if let Some(o) = data.obank_mut().get_mut(op) {
                    o.set_canonical_lessequal();
                }
                return;
            }
            _ => return,
        }
    }
}

/// Fold `BOOL_OR`, `BOOL_AND` or `BOOL_XOR` of two comparisons on the same
/// operands into one comparison, its negation, or a constant.
pub struct RuleCompareMerge {
    group: String,
}

impl RuleCompareMerge {
    pub fn new(group: impl Into<String>) -> RuleCompareMerge {
        RuleCompareMerge { group: group.into() }
    }
}

impl Rule for RuleCompareMerge {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_OR, OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_XOR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(RuleCompareMerge { group: self.group.clone() }) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().cmp_merge {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let how = match o.code() {
            OpCode::CPUI_BOOL_OR => Combine::Or,
            OpCode::CPUI_BOOL_AND => Combine::And,
            _ => Combine::Xor,
        };
        let (Some(x), Some(y)) = (o.get_in(0), o.get_in(1)) else { return 0 };
        let Some(form) = merged(data, (x, true), (y, true), how) else { return 0 };
        let lessequal = lessequal_origin(data, x) || lessequal_origin(data, y);
        rewrite(data, op, form);
        if lessequal {
            let out = data.obank().get(op).and_then(|o| o.get_out());
            keep_lessequal(data, out);
        }
        1
    }
}

fn last_op(data: &Funcdata, bl: BlockId) -> Option<OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_tail,
        _ => None,
    }
}

/// Does `bl` hold nothing but its CBRANCH and side-effect-free ops read only by it?
fn bare_test(data: &Funcdata, bl: BlockId, branch: OpId) -> bool {
    let crate::block::BlockKind::Basic(basic) = data.bblocks_ref().block(bl).kind() else { return false };
    let mut cur = basic.op_head;
    while let Some(op) = cur {
        cur = data.bb_op_next(op);
        if op == branch {
            continue;
        }
        let Some(o) = data.obank().get(op) else { return false };
        if o.is_marker() {
            continue;
        }
        if o.is_call() || o.code() == OpCode::CPUI_STORE || o.code() == OpCode::CPUI_LOAD {
            return false;
        }
        let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) else { return false };
        if out.descend_iter().any(|d| data.obank().get(d).and_then(|d| d.get_parent()) != Some(bl)) {
            return false;
        }
    }
    true
}

/// The condition value that sends `bl`'s CBRANCH down out-edge `edge`.
fn edge_value(data: &Funcdata, branch: OpId, edge: usize) -> bool {
    (edge == 1) != data.obank().get(branch).is_some_and(|o| o.is_boolean_flip())
}

/// Merge every branch with the bare second test that one of its successors is.
pub(crate) fn branchmerge_apply(data: &mut Funcdata) -> int4 {
    let mut count = 0;
    for i in 0..data.bblocks_get_size() {
        let bl = data.bblocks_get_block(i);
        let Some(b1) = last_op(data, bl) else { continue };
        if data.obank().get(b1).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) || data.bblocks_ref().block(bl).size_out() != 2 {
            continue;
        }
        let outs = [data.bblocks_ref().block(bl).get_out(0), data.bblocks_ref().block(bl).get_out(1)];
        for n_edge in 0..2 {
            let next = outs[n_edge];
            let shared = outs[1 - n_edge];
            if next == bl || next == shared {
                continue;
            }
            let nb = data.bblocks_ref().block(next);
            if nb.size_in() != 1 || nb.size_out() != 2 {
                continue;
            }
            let Some(b2) = last_op(data, next) else { continue };
            if data.obank().get(b2).map(|o| o.code()) != Some(OpCode::CPUI_CBRANCH) || !bare_test(data, next, b2) {
                continue;
            }
            let nouts = [data.bblocks_ref().block(next).get_out(0), data.bblocks_ref().block(next).get_out(1)];
            let Some(s_edge) = nouts.iter().position(|&o| o == shared) else { continue };
            if nouts[1 - s_edge] == shared {
                continue;
            }
            let (Some(c1), Some(c2)) = (data.obank().get(b1).and_then(|o| o.get_in(1)), data.obank().get(b2).and_then(|o| o.get_in(1))) else { continue };
            // Both tests oriented to "goes to the shared target".
            let to_shared_1 = edge_value(data, b1, 1 - n_edge);
            let to_shared_2 = edge_value(data, b2, s_edge);
            let probe1 = relation(data, c1).is_some() || range_of(data, c1).is_some();
            let probe2 = relation(data, c2).is_some() || range_of(data, c2).is_some();
            if !probe1 || !probe2 {
                continue;
            }
            // The merged test is computed in `bl`: everything the second test
            // compares must be defined outside `next`.
            let inputs_ok = op_of(data, c2).and_then(|d| data.obank().get(d)).is_some_and(|d| {
                (0..d.num_input()).all(|s| {
                    let v = d.get_in(s).unwrap();
                    data.vbank().get(v).is_some_and(|vv| vv.is_constant() || vv.get_def().and_then(|dd| data.obank().get(dd)).and_then(|dd| dd.get_parent()) != Some(next))
                })
            });
            if !inputs_ok {
                continue;
            }
            let form = match merged(data, (c1, to_shared_1), (c2, to_shared_2), Combine::Or) {
                Some(Form::Reuse(vn)) if vn == c1 => None,
                Some(Form::Reuse(_)) | Some(Form::Constant(_)) | None => continue,
                Some(f) => Some(f),
            };
            if let Some(form) = form {
                // The merged test, true on the way to the shared target.
                let addr = data.obank().get(b1).unwrap().get_addr().clone();
                let test = data.new_op(2, addr);
                data.op_set_opcode(test, crate::typeop::type_op_for(OpCode::CPUI_BOOL_OR));
                let out = data.new_unique_out(1, test).expect("branchmerge: merged");
                data.op_set_input(test, c1, 0).expect("branchmerge: seed");
                data.op_set_input(test, c1, 1).expect("branchmerge: seed");
                data.op_insert_before(test, b1);
                let lessequal = lessequal_origin(data, c1) || lessequal_origin(data, c2);
                rewrite(data, test, form);
                if lessequal {
                    keep_lessequal(data, Some(out));
                }
                let cond = if to_shared_1 {
                    out
                } else {
                    let neg = data.new_op(1, data.obank().get(b1).unwrap().get_addr().clone());
                    data.op_set_opcode(neg, crate::typeop::type_op_for(OpCode::CPUI_BOOL_NEGATE));
                    let nout = data.new_unique_out(1, neg).expect("branchmerge: negate");
                    data.op_set_input(neg, out, 0).expect("branchmerge: negate input");
                    data.op_insert_before(neg, b1);
                    nout
                };
                data.op_set_input(b1, cond, 1).expect("branchmerge: first branch");
            }
            let never = data.new_constant(1, edge_value(data, b2, 1 - s_edge) as uintb);
            data.op_set_input(b2, never, 1).expect("branchmerge: second branch");
            count += 1;
            break;
        }
    }
    count
}

/// Runs [`branchmerge_apply`] when option `cmpmerge` is on.  Group `unreachable`
/// with `ActionDeterminedBranch`, which removes the edge it decides.
pub struct ActionBranchMerge {
    base: ActionBase,
}

impl ActionBranchMerge {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionBranchMerge { base: ActionBase::new(0, "branchmerge", g) })
    }
}

impl Action for ActionBranchMerge {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(ActionBranchMerge { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().cmp_merge {
            return 0;
        }
        self.base.count += branchmerge_apply(data);
        0
    }
}
