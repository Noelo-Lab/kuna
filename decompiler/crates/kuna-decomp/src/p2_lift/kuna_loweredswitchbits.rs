//! (kuna `loweredswitchbits`) A switch the compiler lowered to bit tests and
//! offset range checks becomes a switch again.
//!
//! clang lowers `switch (t) { case A: case B: case F: ...; }` over a narrow
//! selector into `if (t - A <= N && (1 << (t - A)) & mask)`: a range check on
//! an offset copy of the selector and a test of a bit of a constant mask, with
//! several tests sharing one shift.  [`crate::p2_lift::kuna_loweredswitch`]
//! reads only compares of the selector itself against a constant, so these
//! cascades stay `if` chains.
//!
//! When that detection records nothing, this one looks for a cascade of
//! condition blocks whose conditions are pure functions of one selector of at
//! most two bytes (an unsigned 16-bit type tag, a char).  It evaluates each
//! condition on every selector value: walking from a head, each block splits
//! the values that reach it between its two edges, a block of the cascade
//! passes its values on, and any other block is a target that receives them.
//! The target that receives the most values is the default, and so is any
//! target that does what it does (the same ops on the same inputs, no
//! successors: the compiler's copies of one `return 0;`); every other value is a
//! case of the target it reaches.  The partition is exact, so the switch
//! routes every value where the tests did.  The cascade must hold a bit test
//! (a shift by a selector-derived amount: `1 << t` on arm64, `bt` on x86), at most [`MAX_CASES`] cases and
//! at least two case targets; every block after the head must compute with pure
//! ops only, be read only by the cascade and be entered only from it.  The record goes to the
//! lowered-switch store and the same restart install manufactures the switch,
//! reading the register that holds the selector at the head.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::space::spacetype;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

use crate::block::BlockType;
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::KunaLoweredSwitchRecord;
use crate::p2_lift::kuna_loweredswitchvalue::ValueCheck;

/// The most case values a recovered switch holds.
pub const MAX_CASES: usize = 64;
/// The most cascade blocks a walk visits.
const MAX_NODES: usize = 32;

fn mask(size: i32) -> u64 {
    if size >= 8 {
        u64::MAX
    } else {
        (1u64 << (size.max(0) * 8)) - 1
    }
}

fn sext(v: u64, size: i32) -> u64 {
    if size >= 8 {
        return v;
    }
    let bits = size * 8;
    let v = v & mask(size);
    if v >> (bits - 1) & 1 == 1 {
        v | !mask(size)
    } else {
        v
    }
}

/// The ops a condition may compute with.
fn pure_code(c: OpCode) -> bool {
    use OpCode::*;
    matches!(
        c,
        CPUI_COPY
            | CPUI_CAST
            | CPUI_INT_ZEXT
            | CPUI_INT_SEXT
            | CPUI_SUBPIECE
            | CPUI_INT_ADD
            | CPUI_INT_SUB
            | CPUI_INT_AND
            | CPUI_INT_OR
            | CPUI_INT_XOR
            | CPUI_INT_LEFT
            | CPUI_INT_RIGHT
            | CPUI_INT_SRIGHT
            | CPUI_INT_EQUAL
            | CPUI_INT_NOTEQUAL
            | CPUI_INT_LESS
            | CPUI_INT_LESSEQUAL
            | CPUI_INT_SLESS
            | CPUI_INT_SLESSEQUAL
            | CPUI_BOOL_NEGATE
            | CPUI_BOOL_AND
            | CPUI_BOOL_OR
            | CPUI_BOOL_XOR
    )
}

/// The one non-constant leaf `vn` is a pure function of, and whether the
/// expression shifts by a non-constant (so leaf-derived) amount.  `None` for two leaves or a
/// deeper expression than the walk follows.
fn leaf_of(
    data: &Funcdata,
    vn: VarnodeId,
    depth: u32,
    leaf: &mut Option<VarnodeId>,
    shifts: &mut bool,
) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if v.is_constant() {
        return true;
    }
    let def = v
        .get_def()
        .and_then(|d| data.obank().get(d))
        .filter(|o| pure_code(o.code()));
    let Some(op) = def.filter(|_| depth < 16) else {
        return match leaf {
            Some(l) => *l == vn,
            None => {
                *leaf = Some(vn);
                true
            }
        };
    };
    if matches!(
        op.code(),
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT
    ) && op
        .get_in(1)
        .and_then(|k| data.vbank().get(k))
        .is_some_and(|k| !k.is_constant())
    {
        *shifts = true;
    }
    (0..op.num_input()).all(|i| {
        op.get_in(i)
            .is_some_and(|x| leaf_of(data, x, depth + 1, leaf, shifts))
    })
}

/// `vn` when the selector `leaf` holds `x`, by each op's p-code behavior.
fn eval(data: &Funcdata, vn: VarnodeId, leaf: VarnodeId, x: u64) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if vn == leaf {
        return Some(x & mask(size));
    }
    if v.is_constant() {
        return Some(v.get_offset() & mask(size));
    }
    let op = data.obank().get(v.get_def()?)?;
    let input = |i: i32| op.get_in(i).and_then(|a| eval(data, a, leaf, x));
    if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
        return input(0);
    }
    let in_size = op
        .get_in(0)
        .and_then(|a| data.vbank().get(a))
        .map(|a| a.get_size())?;
    let behave = data.get_arch().op_behavior(op.code())?;
    if behave.is_unary() {
        behave.evaluate_unary(size, in_size, input(0)?).ok()
    } else {
        behave
            .evaluate_binary(size, in_size, input(0)?, input(1)?)
            .ok()
    }
}

/// A condition block: its CBRANCH condition, the selector it reads, whether it
/// tests a bit, whether it computes with pure ops only, and the other blocks
/// that read what it computes.
struct Node {
    cond: VarnodeId,
    leaf: VarnodeId,
    bits: bool,
    pure: bool,
    readers: BTreeSet<BlockId>,
}

fn node(data: &Funcdata, bl: BlockId) -> Option<Node> {
    let graph = data.bblocks_ref();
    let b = graph.block(bl);
    if b.get_type() != BlockType::Basic || b.size_out() != 2 || b.is_switch_out() {
        return None;
    }
    let tail = data.bb_op_tail(bl)?;
    let op = data.obank().get(tail)?;
    if op.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let cond = op.get_in(1)?;
    let (mut leaf, mut bits) = (None, false);
    if !leaf_of(data, cond, 0, &mut leaf, &mut bits) {
        return None;
    }
    let leaf = leaf?;
    if data.vbank().get(leaf)?.get_size() > 2 {
        return None;
    }
    let mut pure = true;
    let mut readers = BTreeSet::new();
    for o in data.bb_ops(bl) {
        let Some(p) = data.obank().get(o) else {
            return None;
        };
        if o == tail {
            continue;
        }
        pure &= pure_code(p.code());
        if let Some(out) = p.get_out().and_then(|out| data.vbank().get(out)) {
            readers.extend(
                out.descend_iter()
                    .filter_map(|d| data.obank().get(d).and_then(|d| d.get_parent())),
            );
        }
    }
    readers.remove(&bl);
    Some(Node {
        cond,
        leaf,
        bits,
        pure,
        readers,
    })
}

/// The register or stack varnode that holds the selector `leaf` (or its zero
/// or sign extension, the narrowest one), and whether it is sign-extended.
fn selector_storage(data: &Funcdata, leaf: VarnodeId) -> Option<(VarnodeId, bool)> {
    let stable = |v: VarnodeId| {
        data.vbank()
            .get(v)
            .and_then(|n| n.get_addr().get_space().map(|s| s.get_type()))
            .is_some_and(|t| t == spacetype::IPTR_PROCESSOR || t == spacetype::IPTR_SPACEBASE)
    };
    if stable(leaf) {
        return Some((leaf, false));
    }
    let mut best: Option<(VarnodeId, bool, i32)> = None;
    for d in data.vbank().get(leaf)?.descend_iter() {
        let Some(op) = data.obank().get(d) else {
            continue;
        };
        let Some(out) = op.get_out().filter(|&o| stable(o)) else {
            continue;
        };
        if !matches!(
            op.code(),
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_COPY
        ) {
            continue;
        }
        let size = data.vbank().get(out)?.get_size();
        if best.is_none_or(|(_, _, s)| size < s) {
            best = Some((out, op.code() == OpCode::CPUI_INT_SEXT, size));
        }
    }
    best.map(|(v, s, _)| (v, s))
}

/// Where every selector value goes from `head`, or `None` when the cascade
/// leaves its bounds.  `spine` collects the cascade blocks.
fn partition(
    data: &Funcdata,
    head: BlockId,
    leaf: VarnodeId,
    nodes: &BTreeMap<BlockId, Node>,
    spine: &mut BTreeSet<BlockId>,
) -> Option<BTreeMap<BlockId, Vec<u64>>> {
    let size = data.vbank().get(leaf)?.get_size();
    let all: Vec<u64> = (0..=mask(size)).collect();
    let mut out: BTreeMap<BlockId, Vec<u64>> = BTreeMap::new();
    let mut stack = vec![(head, all)];
    let mut visits = 0;
    while let Some((bl, vals)) = stack.pop() {
        visits += 1;
        if visits > MAX_NODES {
            return None;
        }
        spine.insert(bl);
        let n = &nodes[&bl];
        let (mut t, mut f) = (Vec::new(), Vec::new());
        for x in vals {
            if eval(data, n.cond, leaf, x)? != 0 {
                t.push(x)
            } else {
                f.push(x)
            }
        }
        let b = data.bblocks_ref().block(bl);
        let flip = data
            .bb_op_tail(bl)
            .and_then(|o| data.obank().get(o))
            .is_some_and(|o| o.is_boolean_flip());
        let (tb, fb) = if flip {
            (b.get_false_out(), b.get_true_out())
        } else {
            (b.get_true_out(), b.get_false_out())
        };
        for (succ, set) in [(tb, t), (fb, f)] {
            if set.is_empty() {
                continue;
            }
            match nodes.get(&succ) {
                Some(m) if m.leaf == leaf && m.pure && succ != head => stack.push((succ, set)),
                _ => out.entry(succ).or_default().extend(set),
            }
        }
    }
    Some(out)
}

/// Do blocks `a` and `b` do the same thing: the same ops on the same inputs,
/// constants by value and in-block values pairwise, and no successors?
fn same_exit(data: &Funcdata, a: BlockId, b: BlockId) -> bool {
    let graph = data.bblocks_ref();
    if graph.block(a).size_out() != 0 || graph.block(b).size_out() != 0 {
        return false;
    }
    let (oa, ob) = (data.bb_ops(a), data.bb_ops(b));
    if oa.len() != ob.len() {
        return false;
    }
    let mut pair: BTreeMap<VarnodeId, VarnodeId> = BTreeMap::new();
    oa.iter().zip(&ob).all(|(&x, &y)| {
        let (Some(p), Some(q)) = (data.obank().get(x), data.obank().get(y)) else {
            return false;
        };
        if p.code() != q.code() || p.num_input() != q.num_input() {
            return false;
        }
        let same_in = (0..p.num_input()).all(|i| {
            let (Some(u), Some(w)) = (p.get_in(i), q.get_in(i)) else {
                return false;
            };
            let (Some(un), Some(wn)) = (data.vbank().get(u), data.vbank().get(w)) else {
                return false;
            };
            if un.is_constant() || wn.is_constant() {
                return un.is_constant()
                    && wn.is_constant()
                    && un.get_offset() == wn.get_offset()
                    && un.get_size() == wn.get_size();
            }
            u == w || pair.get(&u) == Some(&w)
        });
        if let (Some(u), Some(w)) = (p.get_out(), q.get_out()) {
            pair.insert(u, w);
        }
        same_in
    })
}

/// The record for a bit-test cascade in `data`, if there is one.
pub(crate) fn recover(data: &Funcdata) -> Option<KunaLoweredSwitchRecord> {
    let mut nodes = BTreeMap::new();
    for i in 0..data.bblocks_get_size() {
        let bl = data.bblocks_get_block(i);
        if let Some(n) = node(data, bl) {
            nodes.insert(bl, n);
        }
    }
    let graph = data.bblocks_ref();
    for (&head, h) in &nodes {
        let hb = graph.block(head);
        let continued = (0..hb.size_in()).any(|k| {
            nodes
                .get(&hb.get_in(k))
                .is_some_and(|p| p.leaf == h.leaf && p.pure)
        });
        if continued {
            continue;
        }
        let mut spine = BTreeSet::new();
        let Some(parts) = partition(data, head, h.leaf, &nodes, &mut spine) else {
            continue;
        };
        if spine.len() < 2 || !spine.iter().any(|b| nodes[b].bits) {
            continue;
        }
        let entered_outside = spine.iter().filter(|&&b| b != head).any(|&b| {
            let bb = graph.block(b);
            (0..bb.size_in()).any(|k| !spine.contains(&bb.get_in(k)))
        });
        let read_outside = spine
            .iter()
            .filter(|&&b| b != head)
            .any(|b| !nodes[b].readers.is_subset(&spine));
        if entered_outside || read_outside {
            continue;
        }
        let Some((&default, _)) = parts.iter().max_by_key(|(_, v)| v.len()) else {
            continue;
        };
        let mut parts = parts;
        let twins: Vec<BlockId> = parts
            .keys()
            .copied()
            .filter(|&b| b != default && same_exit(data, b, default))
            .collect();
        for b in twins {
            let vals = parts.remove(&b).unwrap_or_default();
            parts.entry(default).or_default().extend(vals);
        }
        let cases: Vec<(u64, BlockId)> = parts
            .iter()
            .filter(|(b, _)| **b != default)
            .flat_map(|(&b, v)| v.iter().map(move |&x| (x, b)))
            .collect();
        let targets: BTreeSet<BlockId> = cases.iter().map(|&(_, b)| b).collect();
        if cases.len() > MAX_CASES || targets.len() < 2 {
            continue;
        }
        let Some((storage, signed)) = selector_storage(data, h.leaf) else {
            continue;
        };
        let sv = data.vbank().get(storage)?;
        let leaf_size = data.vbank().get(h.leaf)?.get_size();
        let label = |x: u64| -> uintb {
            if signed {
                sext(x, leaf_size) & mask(sv.get_size())
            } else {
                x
            }
        };
        let start = |b: BlockId| crate::block::block_get_start(&graph.arena, b);
        let mut sorted = cases.clone();
        sorted.sort_by_key(|&(x, _)| label(x));
        let mut value = ValueCheck::for_value(data, storage);
        value.read_home = true;
        return Some(KunaLoweredSwitchRecord {
            branch_addr: data.obank().get(data.bb_op_tail(head)?)?.get_addr().clone(),
            var_addr: sv.get_addr().clone(),
            var_size: sv.get_size(),
            case_vals: sorted.iter().map(|&(x, _)| label(x)).collect(),
            case_targets: sorted.iter().map(|&(_, b)| start(b)).collect(),
            default_target: start(default),
            signed_labels: signed,
            value,
        });
    }
    None
}
