//! A value used as an address keeps its own variable rather than merging into a
//! global it is stored to.
//!
//! `Merge` joins a register value with the global it is copied to whenever their
//! Covers allow it.  The joined value then prints as a read of the global, and a
//! dereference through it takes the global's pointee type:
//!
//! ```text
//! int *q = p + k; gc = (char *)q; return q[1] + q[2];
//! gc = &a0[a1]; return gc[2] + gc[1];            (kuna before)
//! ```
//!
//! kuna never declares the global, so a reader who gives it its real type,
//! `char *`, reads two bytes and gets 0 where the binary returns 9.  The binary
//! never reads `gc` here; it dereferences the register.  [`keeps_apart`] refuses
//! that join in the two optional merges that make it, the `COPY`'s copy-shadow
//! merge and the adjacent-op merge, so the value keeps its own variable, typed
//! by its own uses, and the `COPY` prints as the store.
//! `p3_dataflow/kuna_pointeestorekeep.rs` keeps that `COPY` alive in the first
//! place when a marker would otherwise swallow it.
//!
//! The binary's own loads of the global stay loads there too, and a register
//! copy of one that a later `STORE` may outdate ([`load_crosses_write`]) is kept
//! out of the global's variable and out of the implied set, so it prints as its
//! own statement where the binary loads it.

use std::collections::BTreeSet;

use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::merge::MergeContext;
use crate::p3_dataflow::kuna_pointeestorekeep::{
    loads_stored_global, offsets_address, reads_pointee, written_between, WALK_BOUND,
};
use crate::varnode::varnode_flags;

/// Must the HighVariables of `vn1` and `vn2` stay apart because one is a global
/// and the other a value used as an address that is not the global's own value,
/// or a load of the global that a later write may outdate
/// ([`load_crosses_write`])?
pub fn keeps_apart(ctx: &mut dyn MergeContext, vn1: VarnodeId, vn2: VarnodeId) -> bool {
    let (Some(a), Some(b)) = (ctx.vn_high(vn1), ctx.vn_high(vn2)) else {
        return false;
    };
    if a == b {
        return false;
    }
    let (global, value) = if ctx.high_is_persist(a) {
        (a, b)
    } else if ctx.high_is_persist(b) {
        (b, a)
    } else {
        return false;
    };
    if ctx.high_is_persist(value) || ctx.high_is_addr_tied(value) || ctx.high_is_input(value) {
        return false;
    }
    if loaded_from(ctx, vn1, vn2) || loaded_from(ctx, vn2, vn1) {
        return true;
    }
    used_as_address(ctx, value) && !holds_global(ctx, value, global)
}

/// Is `vn` the output of the `COPY` that loads `global` and a write may outdate
/// the load before a reader ([`load_crosses_write`])?
fn loaded_from(ctx: &dyn MergeContext, vn: VarnodeId, global: VarnodeId) -> bool {
    ctx.vn_def(vn)
        .is_some_and(|d| ctx.op_in(d, 0) == Some(global))
        && ctx.vn_loads_across_write(vn)
}

fn instances(ctx: &dyn MergeContext, high: HighVariableId) -> Vec<VarnodeId> {
    (0..ctx.high_num_instances(high))
        .map(|i| ctx.high_get_instance(high, i))
        .collect()
}

/// Does `value` reach an operation that [`reads_pointee`], directly or through
/// additions that [`offsets_address`]?  The value is its instances and every
/// varnode a `COPY`, `INDIRECT` or `MULTIEQUAL` carries it to or from, since a
/// later merge can join those with it; a sum is followed forward only.
/// Globals and constants end the walk.
fn used_as_address(ctx: &mut dyn MergeContext, value: HighVariableId) -> bool {
    let mut stack: Vec<(VarnodeId, bool)> = instances(ctx, value)
        .into_iter()
        .map(|v| (v, true))
        .collect();
    let mut seen = BTreeSet::new();
    let carries = |ctx: &dyn MergeContext, x: VarnodeId| {
        ctx.vn_view(x).flags & (varnode_flags::persist | varnode_flags::constant) == 0
    };
    while let Some((vn, member)) = stack.pop() {
        if !seen.insert((vn, member)) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return true;
        }
        if member {
            if let Some(def) = ctx.vn_def(vn) {
                let joined = match ctx.op_code(def) {
                    OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => 1,
                    OpCode::CPUI_MULTIEQUAL => ctx.op_num_input(def),
                    _ => 0,
                };
                for i in 0..joined {
                    if let Some(x) = ctx.op_in(def, i).filter(|&x| carries(ctx, x)) {
                        stack.push((x, true));
                    }
                }
            }
        }
        for op in ctx.vn_descend(vn) {
            if ctx.op_is_dead(op) {
                continue;
            }
            let code = ctx.op_code(op);
            let out = ctx.op_out(op).filter(|&o| carries(ctx, o));
            match code {
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
                    stack.extend(out.map(|o| (o, member)))
                }
                OpCode::CPUI_INDIRECT => {
                    if ctx.op_in(op, 0) == Some(vn) {
                        stack.extend(out.map(|o| (o, member)));
                    }
                }
                _ => {
                    for slot in 0..ctx.op_num_input(op) {
                        if ctx.op_in(op, slot) != Some(vn) {
                            continue;
                        }
                        if reads_pointee(code, slot) {
                            return true;
                        }
                        if offsets_address(code, slot) {
                            stack.extend(out.map(|o| (o, false)));
                        }
                    }
                }
            }
        }
    }
    false
}

/// Is every instance of `value` a copy of `global` itself, through `COPY`s,
/// `CAST`s and `MULTIEQUAL`s?  Such a variable is the global read into a
/// register, and keeping it apart would print the `COPY`s that write it back as
/// stores the binary never makes.
fn holds_global(ctx: &mut dyn MergeContext, value: HighVariableId, global: HighVariableId) -> bool {
    let mut stack = instances(ctx, value);
    let mut seen = BTreeSet::new();
    while let Some(vn) = stack.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return false;
        }
        if ctx.vn_high(vn) == Some(global) {
            continue;
        }
        let Some(def) = ctx.vn_def(vn) else {
            return false;
        };
        match ctx.op_code(def) {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => stack.extend(ctx.op_in(def, 0)),
            OpCode::CPUI_MULTIEQUAL => {
                stack.extend((0..ctx.op_num_input(def)).filter_map(|i| ctx.op_in(def, i)))
            }
            _ => return false,
        }
    }
    true
}

/// Is `vn` a register copy of a global holding a stored value
/// ([`loads_stored_global`]) with a write between it and one of its readers
/// ([`written_between`])?  Chapter 03 keeps such a load; joined with the
/// global, or printed inline at its reader, it would read the global after a
/// write that may change it, since kuna's SSA puts no `INDIRECT` on a global at
/// a `STORE` to show the Cover tests the conflict.
pub fn load_crosses_write(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    if !v.get_def().is_some_and(|d| loads_stored_global(data, d)) {
        return false;
    }
    v.descend_iter()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead()))
        .any(|r| written_between(data, vn, r))
}
