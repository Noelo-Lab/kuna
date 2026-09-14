//! Loop-head address-tied MULTIEQUAL input trim — the `tiedphitrim` decision point.
//!
//! `Merge::mergeOp` (`merge.cc:719`) forces a MULTIEQUAL's output and every
//! input it has not trimmed into one HighVariable.  When one input is an
//! instance of an address-tied variable at storage `S` and the output is not,
//! that HighVariable is `S`, and every other member's definition prints as a
//! store into `S`.  Upstream trims only on a required-test or Cover conflict,
//! and a read of `S` that is dead once the loop is entered conflicts with
//! nothing, so a loop that walks memory prints each value it loads as a store
//! into the slot or global it was entered from:
//!
//! ```text
//! u = MULTIEQUAL(stack buf[0], LOAD buf[i])  ->  while (buf[0]) { ...; buf[0] = buf[i]; }
//! ```
//!
//! [`tied_slots`] runs inside `merge_op` after upstream's own trims and names
//! the slots to trim when all of these hold:
//!
//! - the output's HighVariable is not address-tied;
//! - every address-tied input reads storage `S` that a pointer can observe (a
//!   global, or a frame local below the parameter area that is not proven
//!   unaliased; never the function's entry value of either) and enters from
//!   outside the loop (a block the MULTIEQUAL's block does not dominate);
//! - some input on a back edge is read by a LOAD, directly or through untied
//!   COPYs and MULTIEQUALs;
//! - no write into `S` happens inside the loop or stores a value that comes
//!   from inside it.
//!
//! Each named slot gets upstream's `trimOpInput` COPY on its entering edge, so
//! the loop reads `S` once into its own variable.  Upstream's exclusion stays in
//! force: `Merge` records the loop variable and `S`, and [`intersects`] keeps the
//! loop variable out of every address-tied variable and keeps anything live in
//! the loop out of `S`.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::merge::MergeContext;

/// The input slots of MULTIEQUAL `op` to trim before its forced merge, with
/// the storage they are tied to, or `None` when the merge is left to upstream.
pub fn tied_slots(ctx: &mut dyn MergeContext, op: OpId) -> Option<(Vec<int4>, Address)> {
    if !ctx.kuna_tied_phi_trim() || ctx.op_code(op) != OpCode::CPUI_MULTIEQUAL {
        return None;
    }
    let out = ctx.op_out(op)?;
    let out_high = ctx.vn_high(out)?;
    if ctx.high_is_addr_tied(out_high) {
        return None;
    }
    let mut slots = Vec::new();
    let mut storage: Option<Address> = None;
    let mut untied = Vec::new();
    for slot in 0..ctx.op_num_input(op) {
        let vn = ctx.op_in(op, slot)?;
        let high = ctx.vn_high(vn)?;
        if !ctx.high_is_addr_tied(high) {
            untied.push((slot, vn));
            continue;
        }
        let addr = ctx.bank_tied_addr(high);
        if !observable(ctx, vn, high, &addr) || storage.as_ref().is_some_and(|s| *s != addr) {
            return None;
        }
        storage = Some(addr);
        slots.push(slot);
    }
    let storage = storage?;
    let bl = ctx.op_parent(op);
    if slots.iter().any(|&slot| ctx.block_dominates(bl, ctx.block_in(bl, slot))) {
        return None;
    }
    let latch: Vec<VarnodeId> = untied
        .iter()
        .filter(|&&(slot, _)| ctx.block_dominates(bl, ctx.block_in(bl, slot)))
        .map(|&(_, vn)| vn)
        .collect();
    if !latch.iter().any(|&vn| loaded(ctx, vn)) {
        return None;
    }
    let body = loop_body(ctx, bl);
    let slot_high = ctx.vn_high(ctx.op_in(op, slots[0])?)?;
    for i in 0..ctx.high_num_instances(slot_high) {
        let inst = ctx.high_get_instance(slot_high, i);
        let Some(w) = ctx.vn_def(inst) else { continue };
        if w == op {
            continue;
        }
        if body.contains(&ctx.op_parent(w)) {
            return None;
        }
        for s in 0..ctx.op_num_input(w) {
            let Some(src) = ctx.op_in(w, s) else { continue };
            if derives_from_loop(ctx, src, &body) {
                return None;
            }
        }
    }
    Some((slots, storage))
}

/// Does `vn`'s value come, directly or through defining ops, from a value
/// defined inside `body`?  The backward walk stops at inputs, constants and
/// address-tied values (which carry their own storage's identity).
fn derives_from_loop(ctx: &mut dyn MergeContext, vn: VarnodeId, body: &std::collections::BTreeSet<crate::context::BlockId>) -> bool {
    let mut stack = vec![vn];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(v) = stack.pop() {
        if seen.len() >= 256 || !seen.insert(v) {
            continue;
        }
        let Some(d) = ctx.vn_def(v) else { continue };
        if body.contains(&ctx.op_parent(d)) {
            return true;
        }
        if ctx.vn_high(v).is_some_and(|h| ctx.high_is_addr_tied(h)) {
            continue;
        }
        for i in 0..ctx.op_num_input(d) {
            stack.extend(ctx.op_in(d, i));
        }
    }
    false
}

/// The set of blocks that form the loop headed by `bl`: those `bl` dominates
/// from which `bl` is reachable (walking predecessors that `bl` dominates, back
/// from `bl`'s latch predecessors), plus `bl` itself.
fn loop_body(ctx: &mut dyn MergeContext, bl: crate::context::BlockId) -> std::collections::BTreeSet<crate::context::BlockId> {
    let mut body = std::collections::BTreeSet::new();
    body.insert(bl);
    let mut stack: Vec<crate::context::BlockId> = (0..ctx.block_num_in(bl))
        .map(|s| ctx.block_in(bl, s))
        .filter(|&p| ctx.block_dominates(bl, p))
        .collect();
    while let Some(b) = stack.pop() {
        if !body.insert(b) {
            continue;
        }
        for s in 0..ctx.block_num_in(b) {
            let p = ctx.block_in(b, s);
            if ctx.block_dominates(bl, p) && !body.contains(&p) {
                stack.push(p);
            }
        }
    }
    body
}

/// Would merging `a` and `b` break the exclusion the trimmed loop variables in
/// `phis` had while upstream kept them in their storage's HighVariable?  Two
/// rules restore it.  An address-tied variable (at any storage) never merges
/// with a variable holding a loop variable, as upstream's required test forbids
/// merging two address-tied variables.  And a variable tied to a loop variable's
/// own storage never merges with anything whose Cover intersects the loop
/// variable, as upstream's Cover test would have refused.
pub fn intersects(
    phis: &[(VarnodeId, Address)],
    ctx: &mut dyn MergeContext,
    cache: &mut crate::variable::HighIntersectTest,
    a: HighVariableId,
    b: HighVariableId,
) -> bool {
    if phis.is_empty() {
        return false;
    }
    for (tied, other) in [(a, b), (b, a)] {
        if !ctx.high_is_addr_tied(tied) {
            continue;
        }
        let addr = ctx.bank_tied_addr(tied);
        for (vn, s) in phis {
            let Some(lh) = ctx.vn_high(*vn) else { continue };
            if lh == other {
                return true;
            }
            if *s == addr && crate::merge::MergeIntersect::intersection(ctx, cache, other, lh) {
                return true;
            }
        }
    }
    false
}

/// Can a pointer observe the storage `addr` of `vn`'s address-tied `high`?
fn observable(ctx: &mut dyn MergeContext, vn: VarnodeId, high: HighVariableId, addr: &Address) -> bool {
    let Some(space) = addr.get_space() else {
        return false;
    };
    if ctx.vn_is_input(vn) || space.get_name() == "register" {
        return false;
    }
    if space.get_type() != spacetype::IPTR_SPACEBASE {
        return true;
    }
    if ctx.bank_tied_varnode(high).is_some_and(|tied| ctx.vn_has_no_local_alias(tied)) {
        return false;
    }
    let bits = 64 - 8 * space.get_addr_size().min(8);
    let off = ((addr.get_offset() << bits) as i64) >> bits;
    if space.stack_grows_negative() {
        off < 0
    } else {
        off >= 0
    }
}

/// Is the untied value `vn` read by a LOAD, directly or through untied COPYs
/// and MULTIEQUALs?
fn loaded(ctx: &mut dyn MergeContext, vn: VarnodeId) -> bool {
    let mut stack = vec![vn];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(v) = stack.pop() {
        if seen.len() >= 64 || !seen.insert(v) {
            continue;
        }
        if ctx.vn_high(v).is_some_and(|h| ctx.high_is_addr_tied(h)) {
            continue;
        }
        let Some(def) = ctx.vn_def(v) else {
            continue;
        };
        match ctx.op_code(def) {
            OpCode::CPUI_LOAD => return true,
            OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
                for i in 0..ctx.op_num_input(def) {
                    stack.extend(ctx.op_in(def, i));
                }
            }
            _ => {}
        }
    }
    false
}

