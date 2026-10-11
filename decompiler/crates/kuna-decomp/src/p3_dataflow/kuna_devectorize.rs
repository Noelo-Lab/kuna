//! Re-roll a vectorized integer reduction into its scalar loop — the
//! `devectorize` decision point.
//!
//! clang and gcc vectorize `for (i) sum += a[i]` into a stride-S loop with one
//! accumulator per lane, a horizontal fold after it, a guard `length >= S` in
//! front of it, and a scalar copy of the loop that either finishes the
//! remainder `[length & -S, length)` or, when the guard failed, runs from zero.
//! Once [`kuna_wideslice`](crate::kuna_wideslice) has reduced the lanes to byte
//! reads of the loaded word, the vector arm is S copies of the scalar body in a
//! different order, and reading it means reconstructing the source loop by
//! hand.
//!
//! The pass matches the whole shape and proves it on SSA: the accumulators and
//! the induction variable start at zero; the lanes zero-extend disjoint E-byte
//! slices of loads at `base + i*E + c` and tile exactly S elements; the fold
//! adds each accumulator once; the vector loop exits when `i == (length & -S)`
//! for a power-of-two S the guard proves is at most `length`; the scalar loop
//! starts at that index with the folded sum, or at zero with zero, steps one
//! element and exits at `length`; the early return on `length & -S == length`
//! returns the fold where the scalar exit returns the scalar sum; the arm has
//! no side effects and nothing defined in it is read outside except through
//! the phis the edge removal patches.  Integer addition is associative and
//! commutative modulo 2^(8Z), so the scalar loop from zero computes the same
//! value on every input, and the guard's vector edge is removed
//! (`Funcdata::remove_branch`), leaving the arm to the unreachable-block
//! collector.  The rewrite erases the stride, the alignment of the handoff and
//! the duplicated body, which a reverse engineer may want to see, so the option
//! ships off.  Floats, saturating and sign-extended lanes are not matched: their
//! fold order or extension would change the value.

use kuna_num::opcodes::OpCode::{self, *};

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::BlockKind;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;
use kuna_base::types::int4;

pub struct ActionDevectorize {
    base: ActionBase,
    /// Unit-test override of the `devectorize_reduction` gate; the scheduled
    /// instance passes `false` and reads the live flag.
    enabled: bool,
}

impl ActionDevectorize {
    pub fn new(enabled: bool, g: impl Into<String>) -> Self {
        Self {
            base: ActionBase::new(0, "devectorize", g),
            enabled,
        }
    }
}

impl Action for ActionDevectorize {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
                enabled: self.enabled,
            }) as Box<dyn Action>
        })
    }

    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !self.enabled && !data.get_arch().devectorize_reduction {
            return 0;
        }
        for i in 0..data.bblocks_get_size() {
            let Some((guard, edge)) = match_reduction(data, data.bblocks_get_block(i)) else {
                continue;
            };
            data.remove_branch(guard, edge).expect("devectorize: remove_branch");
            data.remove_unreachable_blocks(false, true)
                .expect("devectorize: remove_unreachable_blocks");
            self.base.count += 1;
            return 1;
        }
        0
    }
}

/// Opcodes without side effects: everything the arm may contain.
const PURE: &[OpCode] = &[
    CPUI_COPY,
    CPUI_LOAD,
    CPUI_MULTIEQUAL,
    CPUI_INT_ADD,
    CPUI_INT_SUB,
    CPUI_INT_MULT,
    CPUI_INT_AND,
    CPUI_INT_OR,
    CPUI_INT_XOR,
    CPUI_INT_LEFT,
    CPUI_INT_RIGHT,
    CPUI_INT_SRIGHT,
    CPUI_INT_ZEXT,
    CPUI_INT_SEXT,
    CPUI_INT_NEGATE,
    CPUI_INT_2COMP,
    CPUI_SUBPIECE,
    CPUI_PIECE,
    CPUI_INT_EQUAL,
    CPUI_INT_NOTEQUAL,
    CPUI_INT_LESS,
    CPUI_INT_LESSEQUAL,
    CPUI_INT_SLESS,
    CPUI_INT_SLESSEQUAL,
    CPUI_BOOL_NEGATE,
    CPUI_BOOL_AND,
    CPUI_BOOL_OR,
    CPUI_BOOL_XOR,
    CPUI_CBRANCH,
    CPUI_BRANCH,
    CPUI_RETURN,
    CPUI_PTRADD,
    CPUI_PTRSUB,
    CPUI_CAST,
];

fn op(data: &Funcdata, id: OpId) -> &PcodeOp {
    data.obank().get(id).expect("devectorize: op")
}

fn size(data: &Funcdata, v: VarnodeId) -> i64 {
    data.vbank().get(v).expect("devectorize: varnode").get_size() as i64
}

fn constant(data: &Funcdata, v: VarnodeId) -> Option<u64> {
    let node = data.vbank().get(v)?;
    node.is_constant().then(|| node.get_offset())
}

fn def(data: &Funcdata, v: VarnodeId) -> Option<(OpId, &PcodeOp)> {
    let id = data.vbank().get(v)?.get_def()?;
    Some((id, op(data, id)))
}

/// `v` with its COPY (and, when `zext`, INT_ZEXT) wrappers peeled: the same value.
fn strip(data: &Funcdata, mut v: VarnodeId, zext: bool) -> VarnodeId {
    while let Some((_, o)) = def(data, v) {
        if o.code() != CPUI_COPY && !(zext && o.code() == CPUI_INT_ZEXT) {
            break;
        }
        v = o.get_in(0).expect("devectorize: unary input");
    }
    v
}

fn is_zero(data: &Funcdata, v: VarnodeId) -> bool {
    constant(data, strip(data, v, false)) == Some(0)
}

fn same(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    let (a, b) = (strip(data, a, false), strip(data, b, false));
    a == b || matches!((constant(data, a), constant(data, b)), (Some(x), Some(y)) if x == y)
}

fn basic(data: &Funcdata, b: BlockId) -> bool {
    matches!(data.bblocks_ref().block(b).kind(), BlockKind::Basic(_))
}

fn pure_block(data: &Funcdata, b: BlockId) -> bool {
    basic(data, b) && data.bb_ops(b).iter().all(|&o| PURE.contains(&op(data, o).code()))
}

fn phis(data: &Funcdata, b: BlockId) -> Vec<OpId> {
    data.bb_ops(b)
        .into_iter()
        .filter(|&o| op(data, o).code() == CPUI_MULTIEQUAL)
        .collect()
}

/// The other operand of a binary op whose one operand is `v`.
fn operand_besides(o: &PcodeOp, v: VarnodeId) -> Option<VarnodeId> {
    let (a, b) = (o.get_in(0)?, o.get_in(1)?);
    if a == v {
        Some(b)
    } else if b == v {
        Some(a)
    } else {
        None
    }
}

/// The successors a CBRANCH block takes on a true and on a false condition.
fn taken(data: &Funcdata, b: BlockId) -> Option<(BlockId, BlockId)> {
    let fb = data.bblocks_ref().block(b);
    if fb.size_out() != 2 {
        return None;
    }
    let tail = op(data, data.bb_op_tail(b)?);
    if tail.code() != CPUI_CBRANCH {
        return None;
    }
    let (t, f) = (fb.get_true_out(), fb.get_false_out());
    Some(if tail.is_boolean_flip() { (f, t) } else { (t, f) })
}

/// For a block ending in `x == y` or `x != y` (either operand order, through
/// COPY/ZEXT), the successors taken when the two differ and when they agree.
fn branch_pair(data: &Funcdata, b: BlockId, x: VarnodeId, y: VarnodeId) -> Option<(BlockId, BlockId)> {
    let (t, f) = taken(data, b)?;
    let (_, cond) = def(data, op(data, data.bb_op_tail(b)?).get_in(1)?)?;
    let equal = match cond.code() {
        CPUI_INT_EQUAL => true,
        CPUI_INT_NOTEQUAL => false,
        _ => return None,
    };
    let (a, c) = (strip(data, cond.get_in(0)?, true), strip(data, cond.get_in(1)?, true));
    if !((a == x && c == y) || (a == y && c == x)) {
        return None;
    }
    Some(if equal { (f, t) } else { (t, f) })
}

/// `v` as `INT_ZEXT(LOAD)` or a LOAD of the sum's own width: the element read.
fn element_load(data: &Funcdata, v: VarnodeId, width: i64) -> Option<OpId> {
    let (id, o) = def(data, v)?;
    match o.code() {
        CPUI_LOAD if size(data, v) == width => Some(id),
        CPUI_INT_ZEXT => {
            let (id, o) = def(data, o.get_in(0)?)?;
            (o.code() == CPUI_LOAD).then_some(id)
        }
        _ => None,
    }
}

/// Byte index, in significance order, of `v`'s low byte within the LOAD it was
/// sliced from.
fn sliced(data: &Funcdata, v: VarnodeId) -> Option<(OpId, i64)> {
    let (id, o) = def(data, v)?;
    match o.code() {
        CPUI_LOAD => Some((id, 0)),
        CPUI_COPY | CPUI_INT_ZEXT => sliced(data, o.get_in(0)?),
        CPUI_INT_RIGHT => {
            let bits = constant(data, o.get_in(1)?)?;
            let (load, n) = sliced(data, o.get_in(0)?)?;
            (bits % 8 == 0).then(|| (load, n + (bits / 8) as i64))
        }
        CPUI_SUBPIECE => {
            let (load, n) = sliced(data, o.get_in(0)?)?;
            Some((load, n + constant(data, o.get_in(1)?)? as i64))
        }
        _ => None,
    }
}

/// `v` as the zero-extension of one lane of a LOAD: the load, the lane's first
/// byte in significance order, and the lane width.
fn lane(data: &Funcdata, v: VarnodeId) -> Option<(OpId, i64, i64)> {
    let (id, o) = def(data, v)?;
    let width = size(data, v);
    match o.code() {
        CPUI_INT_AND => {
            let mask = constant(data, o.get_in(1)?)?;
            let e = (1..=width.min(8)).find(|&e| mask == u64::MAX >> (64 - 8 * e))?;
            let (load, n) = sliced(data, o.get_in(0)?)?;
            Some((load, n, e))
        }
        CPUI_INT_ZEXT => {
            let inner = o.get_in(0)?;
            let (load, n) = sliced(data, inner)?;
            Some((load, n, size(data, inner)))
        }
        CPUI_INT_RIGHT => {
            let bits = constant(data, o.get_in(1)?)? as i64;
            let inner = o.get_in(0)?;
            let (id, o) = def(data, inner)?;
            (o.code() == CPUI_LOAD && size(data, inner) == width && bits % 8 == 0 && bits > 0 && bits < 8 * width)
                .then(|| (id, bits / 8, width - bits / 8))
        }
        CPUI_LOAD => Some((id, 0, width)),
        CPUI_SUBPIECE => {
            let (load, n) = sliced(data, o.get_in(0)?)?;
            Some((load, n + constant(data, o.get_in(1)?)? as i64, width))
        }
        _ => None,
    }
}

fn big_endian(data: &Funcdata, load: OpId) -> bool {
    let manage = data.get_arch().manage();
    match op(data, load).get_in(0).and_then(|v| constant(data, v)) {
        Some(i) if i < manage.num_spaces() as u64 => manage.get_space(i as i32).is_some_and(|s| s.is_big_endian()),
        _ => false,
    }
}

/// `base + idx * scale + off` read off an address expression.
#[derive(Default)]
struct Addr {
    base: Option<VarnodeId>,
    scale: Option<i64>,
    off: i64,
}

impl Addr {
    fn set_scale(&mut self, k: i64) -> bool {
        self.scale.is_none() && {
            self.scale = Some(k);
            true
        }
    }
    fn set_base(&mut self, data: &Funcdata, v: VarnodeId) -> bool {
        let v = strip(data, v, false);
        self.base.is_none_or(|b| b == v) && {
            self.base = Some(v);
            true
        }
    }
}

fn decompose(data: &Funcdata, v: VarnodeId, idx: VarnodeId, a: &mut Addr) -> bool {
    if strip(data, v, true) == idx {
        return a.set_scale(1);
    }
    if let Some(c) = constant(data, v) {
        let shift = 64 - 8 * size(data, v).min(8) as u32;
        a.off = a.off.wrapping_add(((c << shift) as i64) >> shift);
        return true;
    }
    let Some((_, o)) = def(data, v) else {
        return a.set_base(data, v);
    };
    if o.code() == CPUI_CAST {
        return decompose(data, o.get_in(0).unwrap(), idx, a);
    }
    let (Some(x), Some(y)) = (o.get_in(0), o.get_in(1)) else {
        return a.set_base(data, v);
    };
    match o.code() {
        CPUI_INT_ADD => decompose(data, x, idx, a) && decompose(data, y, idx, a),
        CPUI_PTRADD => {
            let Some(k) = o.get_in(2).and_then(|k| constant(data, k)) else {
                return false;
            };
            let k = k as i64;
            let index_ok = if let Some(c) = constant(data, y) {
                a.off = a.off.wrapping_add(c as i64 * k);
                true
            } else if strip(data, y, true) == idx {
                a.set_scale(k)
            } else {
                k == 1 && decompose(data, y, idx, a)
            };
            index_ok && decompose(data, x, idx, a)
        }
        CPUI_INT_MULT => {
            let (var, k) = if let Some(k) = constant(data, y) {
                (x, k)
            } else {
                (y, constant(data, x).unwrap_or(0))
            };
            k > 0 && strip(data, var, true) == idx && a.set_scale(k as i64)
        }
        CPUI_INT_LEFT => {
            let Some(bits) = constant(data, y) else { return false };
            bits < 63 && strip(data, x, true) == idx && a.set_scale(1 << bits)
        }
        _ => a.set_base(data, v),
    }
}

/// `v` as `length & -S` (through COPY/ZEXT): the AND's output, its mask, and `length`.
fn rounded(data: &Funcdata, v: VarnodeId) -> Option<(VarnodeId, u64, VarnodeId)> {
    let core = strip(data, v, true);
    let (_, o) = def(data, core)?;
    if o.code() != CPUI_INT_AND {
        return None;
    }
    let (x, y) = (o.get_in(0)?, o.get_in(1)?);
    let (len, mask) = match (constant(data, x), constant(data, y)) {
        (None, Some(m)) => (x, m),
        (Some(m), None) => (y, m),
        _ => return None,
    };
    Some((core, mask, strip(data, len, true)))
}

/// Leaves of the INT_ADD tree rooted at `v` inside block `b`.
fn fold_leaves(data: &Funcdata, v: VarnodeId, b: BlockId, out: &mut Vec<VarnodeId>) {
    match def(data, v) {
        Some((_, o)) if o.code() == CPUI_INT_ADD && o.get_parent() == Some(b) => {
            fold_leaves(data, o.get_in(0).unwrap(), b, out);
            fold_leaves(data, o.get_in(1).unwrap(), b, out);
        }
        _ => out.push(v),
    }
}

/// The lower bound on `len` that the guard proves on the edge into the arm.
fn guard_bound(data: &Funcdata, g: BlockId, arm: BlockId, len: VarnodeId) -> Option<u64> {
    let (t, _) = taken(data, g)?;
    let (_, cond) = def(data, op(data, data.bb_op_tail(g)?).get_in(1)?)?;
    let strict = match cond.code() {
        CPUI_INT_LESS | CPUI_INT_SLESS => true,
        CPUI_INT_LESSEQUAL | CPUI_INT_SLESSEQUAL => false,
        _ => return None,
    };
    let (x, y) = (cond.get_in(0)?, cond.get_in(1)?);
    let arm_on_true = t == arm;
    match (constant(data, x), constant(data, y)) {
        // c < len / c <= len holds on the arm
        (Some(c), None) if strip(data, y, true) == len && arm_on_true => Some(c + strict as u64),
        // len < c / len <= c fails on the arm
        (None, Some(c)) if strip(data, x, true) == len && !arm_on_true => Some(c + !strict as u64),
        _ => None,
    }
}

/// The value a RETURN block hands back along the edge from `pred`, through
/// COPY and a MULTIEQUAL of its own.
fn returned_from(data: &Funcdata, r: BlockId, pred: BlockId) -> Option<VarnodeId> {
    let tail = op(data, data.bb_op_tail(r)?);
    if tail.code() != CPUI_RETURN || tail.num_input() != 2 {
        return None;
    }
    let v = strip(data, tail.get_in(1)?, false);
    match def(data, v) {
        Some((_, o)) if o.code() == CPUI_MULTIEQUAL && o.get_parent() == Some(r) => {
            let slot = data.bblocks_ref().block(r).get_in_index(pred);
            (slot >= 0).then(|| strip(data, o.get_in(slot).unwrap(), false))
        }
        _ => Some(v),
    }
}

/// Match the vectorized reduction whose scalar remainder loop is block `h`;
/// on success, the guard block and the index of its edge into the vector arm.
fn match_reduction(data: &Funcdata, h: BlockId) -> Option<(BlockId, int4)> {
    let blocks = data.bblocks_ref();
    let hb = blocks.block(h);
    if !pure_block(data, h) || hb.size_in() != 3 || hb.size_out() != 2 {
        return None;
    }
    let back = (0..3).find(|&k| hb.get_in(k) == h)?;

    // The scalar body: j steps by one, sum accumulates the element at base + j*E.
    let (mut j, mut sum) = (None, None);
    for p in phis(data, h) {
        let (out, next) = (op(data, p).get_out()?, op(data, p).get_in(back)?);
        let Some((_, add)) = def(data, next) else { continue };
        if add.code() != CPUI_INT_ADD {
            continue;
        }
        let Some(other) = operand_besides(add, out) else {
            continue;
        };
        if constant(data, other) == Some(1) {
            j = Some((p, out, next));
        } else if let Some(load) = element_load(data, other, size(data, out)) {
            sum = Some((p, out, next, load));
        }
    }
    let (j_phi, j_out, j_next) = j?;
    let (s_phi, s_out, s_next, load) = sum?;
    let width = size(data, s_out);
    let elem = size(data, op(data, load).get_out()?);
    if op(data, load).get_parent() != Some(h) {
        return None;
    }
    let mut addr = Addr::default();
    if !decompose(data, op(data, load).get_in(1)?, j_out, &mut addr) || addr.scale != Some(elem) || addr.off != 0 {
        return None;
    }
    let base = addr.base?;

    // The scalar exit: back iff j' != length.
    let (_, cond) = def(data, op(data, data.bb_op_tail(h)?).get_in(1)?)?;
    let len = [cond.get_in(0)?, cond.get_in(1)?]
        .into_iter()
        .map(|v| strip(data, v, true))
        .find(|&v| v != j_next)?;
    let (ne, rs) = branch_pair(data, h, j_next, len)?;
    if ne != h {
        return None;
    }

    // The two entries: zeros from the else arm, (length & -S, fold) from the vector arm.
    let (mut else_k, mut vec_k) = (None, None);
    for k in (0..3).filter(|&k| k != back) {
        let (ji, si) = (op(data, j_phi).get_in(k)?, op(data, s_phi).get_in(k)?);
        if is_zero(data, ji) && is_zero(data, si) {
            else_k = Some(k);
        } else if rounded(data, ji).is_some_and(|(_, _, l)| l == len) {
            vec_k = Some(k);
        }
    }
    let (else_k, vec_k) = (else_k?, vec_k?);
    let (v4, mask, _) = rounded(data, op(data, j_phi).get_in(vec_k)?)?;
    let sum_v = strip(data, op(data, s_phi).get_in(vec_k)?, false);
    let (e, x) = (hb.get_in(else_k), hb.get_in(vec_k));
    for p in phis(data, h) {
        if p != j_phi && p != s_phi && !same(data, op(data, p).get_in(else_k)?, op(data, p).get_in(vec_k)?) {
            return None;
        }
    }

    // The fold block: sums the accumulators, returns early iff length & -S == length.
    let xb = blocks.block(x);
    if !pure_block(data, x) || xb.size_in() != 1 {
        return None;
    }
    let (ne, r) = branch_pair(data, x, v4, len)?;
    if ne != h {
        return None;
    }
    let v = xb.get_in(0);

    // The vector loop: i steps by S, every other phi accumulates one lane.
    let vb = blocks.block(v);
    if !pure_block(data, v) || vb.size_in() != 2 {
        return None;
    }
    let kb = (0..2).find(|&k| vb.get_in(k) == v)?;
    let pre = vb.get_in(1 - kb);
    let (mut induction, mut accs) = (None, Vec::new());
    for p in phis(data, v) {
        let (out, next) = (op(data, p).get_out()?, op(data, p).get_in(kb)?);
        if !is_zero(data, op(data, p).get_in(1 - kb)?) {
            return None;
        }
        let (_, add) = def(data, next)?;
        if add.code() != CPUI_INT_ADD {
            return None;
        }
        let other = operand_besides(add, out)?;
        if let Some(step) = constant(data, other) {
            if induction.is_some() {
                return None;
            }
            induction = Some((out, next, step));
        } else if size(data, out) == width {
            accs.push((next, other));
        } else {
            return None;
        }
    }
    let (i_out, i_next, stride) = induction?;
    if stride == 0 || !stride.is_power_of_two() || stride > i64::MAX as u64 / 8 {
        return None;
    }
    let (ne, eq) = branch_pair(data, v, i_next, v4)?;
    if ne != v || eq != x {
        return None;
    }
    let bits = 8 * size(data, v4).min(8);
    if mask != !(stride - 1) & (u64::MAX >> (64 - bits)) {
        return None;
    }
    let stride = stride as i64;
    let mut lanes = Vec::new();
    for &(_, lane_vn) in &accs {
        let (load, n, e) = lane(data, lane_vn)?;
        let wide = size(data, op(data, load).get_out()?);
        if e != elem || n + e > wide || op(data, load).get_parent() != Some(v) {
            return None;
        }
        let mut addr = Addr::default();
        if !decompose(data, op(data, load).get_in(1)?, i_out, &mut addr)
            || addr.base != Some(base)
            || addr.scale != Some(elem)
        {
            return None;
        }
        lanes.push(addr.off + if big_endian(data, load) { wide - n - e } else { n });
    }
    lanes.sort_unstable();
    if lanes != (0..stride).map(|k| k * elem).collect::<Vec<_>>() {
        return None;
    }
    let mut leaves = Vec::new();
    fold_leaves(data, sum_v, x, &mut leaves);
    if leaves.len() != accs.len()
        || accs
            .iter()
            .any(|&(next, _)| leaves.iter().filter(|&&l| l == next).count() != 1)
    {
        return None;
    }

    // The guard: branches into the preheader (or the loop itself) iff length >= S.
    let (g, arm_entry, pre) = if blocks.block(pre).size_out() == 1 {
        if !pure_block(data, pre) || blocks.block(pre).size_in() != 1 {
            return None;
        }
        (blocks.block(pre).get_in(0), pre, Some(pre))
    } else {
        (pre, v, None)
    };
    let gb = blocks.block(g);
    if !basic(data, g) || guard_bound(data, g, arm_entry, len)? < stride as u64 {
        return None;
    }
    let arm_edge = gb.get_out_index(arm_entry);
    if arm_edge < 0 {
        return None;
    }
    let other = gb.get_out(1 - arm_edge);
    let else_ok = (other == h && e == g)
        || (other == e
            && pure_block(data, e)
            && blocks.block(e).size_in() == 1
            && blocks.block(e).size_out() == 1
            && blocks.block(e).get_out(0) == h);
    if !else_ok {
        return None;
    }

    // Both exits hand back their sums; a shared join merges them slot for slot.
    let mut arm: Vec<BlockId> = pre.into_iter().chain([v, x]).collect();
    if r == rs {
        let (sx, sh) = (blocks.block(r).get_in_index(x), blocks.block(r).get_in_index(h));
        if sx < 0 || sh < 0 {
            return None;
        }
        for p in phis(data, r) {
            let (a, b) = (op(data, p).get_in(sx)?, op(data, p).get_in(sh)?);
            if !((strip(data, a, false) == sum_v && strip(data, b, false) == s_next) || same(data, a, b)) {
                return None;
            }
        }
    } else {
        if blocks.block(r).size_in() != 1 || !pure_block(data, r) || returned_from(data, r, x)? != sum_v {
            return None;
        }
        let simple = |o: OpId| matches!(op(data, o).code(), CPUI_MULTIEQUAL | CPUI_COPY | CPUI_RETURN);
        if !data.bb_ops(rs).into_iter().all(simple) || returned_from(data, rs, h)? != s_next {
            return None;
        }
        arm.push(r);
    }

    // Nothing defined in the arm is read outside it except on a phi slot the
    // edge removal deletes.
    for &b in &arm {
        for o in data.bb_ops(b) {
            let Some(out) = op(data, o).get_out() else { continue };
            for d in data.descend_snapshot(out) {
                let reader = op(data, d);
                let Some(parent) = reader.get_parent() else { return None };
                if arm.contains(&parent) {
                    continue;
                }
                if reader.code() != CPUI_MULTIEQUAL {
                    return None;
                }
                for slot in 0..reader.num_input() {
                    if reader.get_in(slot) == Some(out) && !arm.contains(&blocks.block(parent).get_in(slot)) {
                        return None;
                    }
                }
            }
        }
    }
    Some((g, arm_edge))
}

#[cfg(test)]
mod tests;
