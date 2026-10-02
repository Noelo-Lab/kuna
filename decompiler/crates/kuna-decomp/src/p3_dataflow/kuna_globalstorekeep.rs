//! A store of a register value into a global stays where the binary makes it
//! when an operation reads that value sign-sensitively.
//!
//! `RulePropagateCopy` rewrites a marker (`MULTIEQUAL`, `INDIRECT`) that reads a
//! global's `COPY` to read the `COPY`'s input.  The `COPY` then loses its last
//! reader and dies, and the store survives only as `Merge`'s join of the
//! register value into the global: the value's definition prints as the store,
//! and every later use of the value prints as a read of the global.  A use whose
//! result depends on the operand's declared signedness then takes the global's,
//! which the output never states (`sink = a0 * 3; v2 = sink >> 4;`).
//!
//! [`declines`] keeps that propagation from happening when some operation reads
//! the stored value sign-sensitively ([`reads_signedness`]), directly or through
//! an expression C types after it ([`passes_signedness`]: `sink + 1 >> 4` is
//! as wrong as `sink >> 4`), or through an operation a later rule makes
//! sign-sensitive ([`may_read_signedness`]): the marker join this decision
//! allows is forced, so it cannot wait for the rules to finish.  The `COPY` stays
//! alive at the binary's own store, and chapter 06's `kuna_globalvalue`
//! refuses the copy-shadow join, so the value keeps its own variable and the
//! store prints where the binary makes it.  A store after which an earlier
//! value of the global is still used is left alone ([`old_value_read_after`]):
//! kept, it would make `Merge` copy that earlier value out above a pointer store
//! the binary loads the global after.
//!
//! A load is the other reader of that `COPY`: kuna's SSA gives a pointer store
//! no effect on a global, so after `gi = u; *p = k;` the binary's load of `gi`
//! still reads the store's `COPY`, and propagating `u` into it would print `u`
//! where the binary reads memory that `*p` may have changed.  [`declines`]
//! keeps such a load on the global under the same test.  A load it lets through
//! marks the value and the store, and a marked value is joined with the global
//! as upstream joins it.  Every other propagation is upstream's.

use std::collections::BTreeSet;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::expression::functional_equality;
use crate::funcdata::Funcdata;
use crate::jumptable::circlerange_pull_back;
use crate::rangeutil::CircleRange;

/// Does an operation with opcode `code` compute a different result when its
/// operand in `slot`, `size` bytes wide, is declared signed rather than unsigned?
/// `other_const` is the other operand's value when that operand is a constant.
///
/// The ordered comparisons, divide and remainder in either signedness, the
/// shifted operand of `>>`, both extensions and the integer-to-float conversion
/// do.  Below `int` width C promotes the operand first, so `==`/`!=` does too,
/// unless the other side is a constant with the operand's top bit clear.  `+`,
/// `-`, `*`, bitwise ops, `<<`, truncation, concatenation and the carry
/// intrinsics (whose names state their signedness) compute the same bits either
/// way.
pub fn reads_signedness(code: OpCode, slot: int4, size: int4, other_const: Option<u64>) -> bool {
    match code {
        OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_FLOAT_INT2FLOAT => slot == 0,
        OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_SDIV
        | OpCode::CPUI_INT_REM
        | OpCode::CPUI_INT_SREM
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL => true,
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL if size > 0 && size < 4 => {
            other_const.map_or(true, |c| (c >> (size * 8 - 1)) & 1 != 0)
        }
        _ => false,
    }
}

/// Must `RulePropagateCopy` leave `vn`, the output of `COPY invn`, as the input
/// of `op`?  Only when `vn` is a global, `invn` a value the function computes
/// (a parameter never merges with a global, so its store keeps upstream's
/// handling), some operation reads that value, a copy of it or an expression
/// computed from it, sign-sensitively, and no earlier value of the global is
/// still used after the store ([`old_value_read_after`]).
///
/// `op` is then either the global's own marker, or a `COPY` into the same
/// global (what a duplicated join block leaves of its marker), or a load: an
/// operation the binary makes on the global after the store.  A `PIECE` that
/// joins the stored part into the whole of a wider global is neither, and stays
/// upstream's.  A load that upstream lets through reads the value from then on,
/// and after `gi = u; *p = k;` that load may see `k`, so the value must keep
/// printing as the global: the store and the value are marked
/// ([`Varnode::is_global_load`](crate::varnode::Varnode::is_global_load)), the
/// mark follows the value into the global's markers and later stores, and
/// chapter 06 never keeps a marked value apart.  A marked store takes
/// upstream's handling from then on.
pub fn declines(data: &mut Funcdata, op: OpId, vn: VarnodeId, invn: VarnodeId) -> bool {
    let (Some(v), Some(iv)) = (data.vbank().get(vn), data.vbank().get(invn)) else {
        return false;
    };
    if !v.is_persist() || iv.is_persist() || iv.is_addr_tied() || iv.is_constant() || iv.is_input() {
        return false;
    }
    let Some(reader) = data.obank().get(op) else {
        return false;
    };
    let out = reader.get_out();
    let (vspace, voff, vsize) = (v.get_addr().get_space().map(|s| s.get_index()), v.get_offset(), v.get_size() as u64);
    let writes = |same: bool| {
        out.and_then(|o| data.vbank().get(o)).is_some_and(|o| {
            let (ooff, osize) = (o.get_offset(), o.get_size() as u64);
            o.get_addr().get_space().map(|s| s.get_index()) == vspace
                && if same { ooff == voff && osize == vsize } else { ooff <= voff && voff + vsize <= ooff + osize }
        })
    };
    if reader.code() == OpCode::CPUI_PIECE && writes(false) {
        return false;
    }
    let own = (reader.is_marker() || reader.code() == OpCode::CPUI_COPY) && writes(true);
    let marked = v.is_global_load() || iv.is_global_load();
    if !marked && value_read_sign_sensitively(data, invn) && !old_value_read_after(data, vn) {
        return true;
    }
    if marked || !own {
        let targets = [Some(vn), Some(invn), out.filter(|_| own)];
        for x in targets.into_iter().flatten() {
            if let Some(x) = data.vbank_mut().get_mut(x) {
                x.set_global_load();
            }
        }
    }
    false
}

/// How many blocks and operations [`old_value_read_after`] visits before it
/// answers yes.
const REACH_BOUND: usize = 1 << 16;

/// Is an earlier value of the global that `vn`'s `COPY` stores to still read
/// after that store, directly or through a copy that a later propagation can
/// fold into its reader?  `Merge` then has to give that earlier value its own
/// variable, and copies it right where it is defined: the function entry, a
/// call, a loop head.  That copy can land above a pointer store or a call the
/// binary reads the global after, so the store takes upstream's handling.
///
/// A read is after the store when some path from the store reaches it without
/// passing the earlier value's definition; a `MULTIEQUAL` reads at the end of
/// the predecessor block it reads from.  Only a value defined in a block that
/// dominates the store can be read so, since its definition dominates every
/// read.  Past [`WALK_BOUND`] copies or [`REACH_BOUND`] blocks and operations
/// the answer is yes.
fn old_value_read_after(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return true;
    };
    let Some(store) = v.get_def() else {
        return true;
    };
    let Some(sb) = data.obank().get(store).and_then(|s| s.get_parent()) else {
        return true;
    };
    let mut budget = REACH_BOUND;
    let mut tick = || {
        budget = budget.saturating_sub(1);
        budget == 0
    };
    let succ = |b| {
        let blk = data.bblocks_ref().block(b);
        (0..blk.size_out()).map(|i| blk.get_out(i)).collect::<Vec<_>>()
    };
    let mut later = Vec::new();
    let mut cur = data.obank().get(store).and_then(|s| s.basic_neighbours().1);
    while let Some(o) = cur {
        if tick() {
            return true;
        }
        later.push(o);
        cur = data.obank().get(o).and_then(|x| x.basic_neighbours().1);
    }
    let mut reach = BTreeSet::new();
    let mut work = succ(sb);
    while let Some(b) = work.pop() {
        if reach.insert(b) {
            if tick() {
                return true;
            }
            work.extend(succ(b));
        }
    }
    for w in data.vbank().iter_loc_size_addr(v.get_size(), v.get_addr()) {
        let Some(wv) = data.vbank().get(w) else {
            continue;
        };
        if w == vn || wv.get_def() == Some(store) {
            continue;
        }
        let def = wv.get_def();
        let def_block = def.and_then(|d| data.obank().get(d)).and_then(|d| d.get_parent());
        if def_block.is_some_and(|db| db != sb && !data.bblocks_ref().dominates(db, Some(sb))) {
            continue;
        }
        let Some((ops, ends)) = reads_of(data, w) else {
            return true;
        };
        let blocks: BTreeSet<_> = ops
            .iter()
            .filter_map(|&o| data.obank().get(o).and_then(|x| x.get_parent()))
            .chain(ends.iter().copied())
            .collect();
        if !blocks.iter().any(|b| reach.contains(b)) && !ends.contains(&sb) && !later.iter().any(|o| ops.contains(o)) {
            continue;
        }
        if let Some(i) = later.iter().position(|&o| Some(o) == def || ops.contains(&o)) {
            if Some(later[i]) == def {
                continue;
            }
            return true;
        }
        if ends.contains(&sb) {
            return true;
        }
        let mut work = succ(sb);
        let mut seen = BTreeSet::new();
        while let Some(b) = work.pop() {
            if !seen.insert(b) || Some(b) == def_block {
                continue;
            }
            if tick() {
                return true;
            }
            if blocks.contains(&b) {
                return true;
            }
            work.extend(succ(b));
        }
    }
    false
}

/// The operations that read `w`, or a copy of it a later propagation can fold
/// into them, and the blocks at whose end a `MULTIEQUAL` reads one; `None`
/// past [`WALK_BOUND`] copies.
fn reads_of(data: &Funcdata, w: VarnodeId) -> Option<(BTreeSet<OpId>, BTreeSet<BlockId>)> {
    let (mut ops, mut ends) = (BTreeSet::new(), BTreeSet::new());
    let mut stack = vec![w];
    let mut seen = BTreeSet::new();
    while let Some(x) = stack.pop() {
        if !seen.insert(x) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return None;
        }
        let Some(xv) = data.vbank().get(x) else {
            continue;
        };
        for r in xv.descend_iter() {
            let Some(rop) = data.obank().get(r) else {
                continue;
            };
            if rop.is_dead() {
                continue;
            }
            match rop.code() {
                OpCode::CPUI_MULTIEQUAL => {
                    let Some(rb) = rop.get_parent() else {
                        return None;
                    };
                    let blk = data.bblocks_ref().block(rb);
                    for i in 0..rop.num_input().min(blk.size_in()) {
                        if rop.get_in(i) == Some(x) {
                            ends.insert(blk.get_in(i));
                        }
                    }
                }
                OpCode::CPUI_COPY => {
                    ops.insert(r);
                    stack.extend(rop.get_out());
                }
                _ => {
                    ops.insert(r);
                }
            }
        }
    }
    Some((ops, ends))
}

/// Is the result of `code` typed after its operand in `slot`, and the same bits
/// whatever that operand's signedness?  `+`, `-`, `*`, the bitwise ops, `~`,
/// unary `-` and the shifted operand of `<<` are: C gives their result the
/// operand's (promoted) type, so an operation that reads the result
/// sign-sensitively reads the operand's signedness too.  A cast, a truncation or
/// an extension states its own result type and ends that.
pub fn passes_signedness(code: OpCode, slot: int4) -> bool {
    match code {
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR => true,
        OpCode::CPUI_INT_NEGATE | OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_LEFT => slot == 0,
        _ => false,
    }
}

/// Can a rule that runs after this decision turn an operation with opcode
/// `code`, reading a `size`-byte operand in `slot`, into one that
/// [`reads_signedness`]?  `folded` is true when the operand was computed from
/// the stored value through an operator a fold moves a constant across
/// ([`moves_constants`]).
///
/// The fold moves the compare's constant onto the value (`u + 1 == 0` becomes
/// `u == 0xffff`, and `-u`, `~u`, `u - c`, `u ^ c` alike), so such a `==`/`!=`
/// below `int` width counts whatever its constant.  A carry becomes an ordered
/// comparison (`carry(u, c)` is `-c <= u`), and the low half of a concatenation
/// becomes a zero extension (`concat(y, u) & mask` is `zext(u)`).  A compare
/// against a constant whose result [`melds`] with another into an ordered compare
/// counts as well; the walk checks that, since it needs the compare's readers.
pub fn may_read_signedness(code: OpCode, slot: int4, size: int4, folded: bool) -> bool {
    match code {
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => folded && size > 0 && size < 4,
        OpCode::CPUI_INT_CARRY => true,
        OpCode::CPUI_PIECE => slot == 1,
        _ => false,
    }
}

/// Would `RuleRangeMeld` rewrite `op`, a boolean `&&` or `||` of two
/// comparisons (or the `&`/`|` of their results that `RuleLogic2Bool` makes one),
/// into a single comparison that [`reads_signedness`] (`u == 0 || u == 1`
/// becomes `u < 2`)?  It asks the rule's own question: both comparisons pull back
/// to one value, and their ranges combine into one range.  Comparisons of values
/// of different sizes, which the rule pulls back once more, count as yes.
fn melds(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    let and = match o.code() {
        OpCode::CPUI_BOOL_AND | OpCode::CPUI_INT_AND => true,
        OpCode::CPUI_BOOL_OR | OpCode::CPUI_INT_OR => false,
        _ => return false,
    };
    if o.is_dead() || o.num_input() != 2 {
        return false;
    }
    let compare = |i: int4| {
        o.get_in(i)
            .and_then(|v| data.vbank().get(v))
            .and_then(|v| v.get_def())
            .filter(|&d| data.obank().get(d).is_some_and(|x| x.is_bool_output()))
    };
    let pull = |cmp: OpId| {
        let mut range = CircleRange::new_bool(true);
        let mut a = circlerange_pull_back(data, &mut range, cmp, false)?;
        if data.obank().get(cmp)?.code() == OpCode::CPUI_BOOL_NEGATE {
            let def = data.vbank().get(a)?.get_def()?;
            a = circlerange_pull_back(data, &mut range, def, false)?;
        }
        Some((range, a))
    };
    let (Some(c1), Some(c2)) = (compare(0), compare(1)) else {
        return false;
    };
    let (Some((mut r1, a1)), Some((r2, a2))) = (pull(c1), pull(c2)) else {
        return false;
    };
    if !functional_equality(a1, a2, data.vbank(), data.obank()) {
        let size = |a: VarnodeId| data.vbank().get(a).map_or(0, |v| v.get_size());
        return size(a1) != size(a2);
    }
    let combined = if and { r1.intersect(&r2) } else { r1.circle_union(&r2) };
    if combined != 0 {
        return false;
    }
    let (mut code, mut c, mut cslot) = (OpCode::CPUI_COPY, 0, 0);
    r1.translate2_op(&mut code, &mut c, &mut cslot) == 0
        && reads_signedness(code, 1 - cslot, data.vbank().get(a1).map_or(0, |v| v.get_size()), Some(c))
}

/// Does a rule move a compare's constant across an operation with opcode
/// `code` onto its operand?  `RuleEqual2Constant`, `RuleEqual2Zero`,
/// `RuleXorCollapse` and `RuleShiftCompare` do across `+`, `-`, `*`, `^`, `~`,
/// unary `-` and `<<`; nothing does across `&` or `|`.
fn moves_constants(code: OpCode) -> bool {
    !matches!(code, OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR)
}

/// How a walk reached a varnode from the stored value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reach {
    /// The value itself, or a copy `Merge` joins with it.
    Member,
    /// An expression computed from it through `&` and `|` only.
    Derived,
    /// An expression computed through an operator that [`moves_constants`].
    Folded,
}

/// How many varnodes a walk visits before it answers "sign-sensitive" anyway,
/// which keeps the store and every load of the global where the binary makes
/// them.
pub const WALK_BOUND: usize = 256;

/// Does an operation read the value `start` sign-sensitively, directly or
/// through expressions that pass its signedness on ([`passes_signedness`])?
///
/// The value is every varnode `Merge` joins with it: the copies that carry it
/// unchanged, a `COPY`, `MULTIEQUAL` or `INDIRECT` either way.  An expression
/// computed from it is followed forward only, through the same ops.  Globals and
/// constants end the walk: a copy into another global is that global's own
/// store.
fn value_read_sign_sensitively(data: &Funcdata, start: VarnodeId) -> bool {
    let mut stack = vec![(start, Reach::Member)];
    let mut seen = BTreeSet::new();
    let carries = |x: VarnodeId| data.vbank().get(x).is_some_and(|v| !v.is_persist() && !v.is_constant());
    while let Some((x, reach)) = stack.pop() {
        if !seen.insert((x, reach)) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return true;
        }
        let Some(xv) = data.vbank().get(x) else {
            continue;
        };
        if reach == Reach::Member {
            if let Some(def) = xv.get_def().and_then(|d| data.obank().get(d)) {
                let joined = match def.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => 1,
                    OpCode::CPUI_MULTIEQUAL => def.num_input(),
                    _ => 0,
                };
                stack.extend((0..joined).filter_map(|i| def.get_in(i)).filter(|&i| carries(i)).map(|i| (i, Reach::Member)));
            }
        }
        let size = xv.get_size();
        for d in xv.descend_iter() {
            let Some(dop) = data.obank().get(d) else {
                continue;
            };
            if dop.is_dead() {
                continue;
            }
            let code = dop.code();
            let out = dop.get_out().filter(|&o| carries(o));
            match code {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => stack.extend(out.map(|o| (o, reach))),
                OpCode::CPUI_INDIRECT => {
                    if dop.get_in(0) == Some(x) {
                        stack.extend(out.map(|o| (o, reach)));
                    }
                }
                _ => {
                    for slot in 0..dop.num_input() {
                        if dop.get_in(slot) != Some(x) {
                            continue;
                        }
                        let other = (dop.num_input() == 2)
                            .then(|| dop.get_in(1 - slot))
                            .flatten()
                            .and_then(|o| data.vbank().get(o))
                            .filter(|o| o.is_constant())
                            .map(|o| o.get_offset());
                        if reads_signedness(code, slot, size, other)
                            || may_read_signedness(code, slot, size, reach == Reach::Folded)
                        {
                            return true;
                        }
                        if matches!(code, OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL)
                            && other.is_some()
                            && dop
                                .get_out()
                                .and_then(|o| data.vbank().get(o))
                                .is_some_and(|o| o.descend_iter().any(|m| melds(data, m)))
                        {
                            return true;
                        }
                        if passes_signedness(code, slot) {
                            let next = reach.max(if moves_constants(code) { Reach::Folded } else { Reach::Derived });
                            stack.extend(out.map(|o| (o, next)));
                        }
                    }
                }
            }
        }
    }
    false
}
