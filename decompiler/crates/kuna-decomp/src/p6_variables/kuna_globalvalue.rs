//! A value read by a sign-sensitive operation keeps its own variable rather
//! than merging into a global it is stored to.
//!
//! `Merge` joins a register value with the global it is copied to whenever their
//! Covers allow it.  The joined value then prints as a read of the global, and an
//! operation on it takes the global's signedness:
//!
//! ```text
//! u = init * 3; sink = u; r = (int)(u >> 4);   (u unsigned: INT_RIGHT, a logical shift)
//! dat_4068 = a0 * 3; v2 = dat_4068 >> 4;       (kuna before: re-reads the global)
//! ```
//!
//! kuna never declares the global, so a reader who gives it its real type —
//! `sink` is `volatile int` — makes `dat_4068 >> 4` an arithmetic shift: for
//! `init = 0x80000000` the rebuilt program returns `-134217728` where the binary
//! returns `134217728`.  An expression printed around the value takes the
//! global's type as well (`dat_4068 + 1 >> 4`), so its readers count too.
//! [`keeps_apart`] refuses that join in the two optional
//! merges that make it, the `COPY`'s copy-shadow merge and the adjacent-op merge,
//! so the value keeps its own variable and the `COPY` prints as the store.
//! `p3_dataflow/kuna_globalstorekeep.rs` keeps that `COPY` alive in the first
//! place, where the binary stores.  The forced merges of markers are upstream's:
//! a register value `RulePropagateCopy` still fed into a global's marker joins
//! the global as before, since trimming it would re-place the store at the end
//! of each predecessor block.

use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, VarnodeId};
use crate::merge::MergeContext;
use crate::p3_dataflow::kuna_globalstorekeep::{passes_signedness, reads_signedness, WALK_BOUND};

/// Must HighVariables `a` and `b` stay apart because one is a global and the
/// other a value a sign-sensitive operation reads?
pub fn keeps_apart(ctx: &mut dyn MergeContext, a: HighVariableId, b: HighVariableId) -> bool {
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
    if ctx.high_is_persist(value) || ctx.high_is_addr_tied(value) {
        return false;
    }
    let r = read_sign_sensitively(ctx, value) && !holds_global(ctx, value, global);
    if std::env::var_os("KUNA_GV_DEBUG").is_some() {
        eprintln!("GV keeps_apart global={:?} value={:?} -> {}", global, value, r);
    }
    r
}

/// Does an operation read `value` sign-sensitively ([`reads_signedness`]),
/// directly or through an expression printed inline around it?  An implied
/// output of an op that passes its operand's signedness on
/// ([`passes_signedness`]) prints as `value + 1` and takes the type `value` is
/// declared with, so its readers count.  An explicit output is a variable
/// declared with its own type, and a cast states its type, so either ends the
/// walk.
fn read_sign_sensitively(ctx: &mut dyn MergeContext, value: HighVariableId) -> bool {
    let mut stack: Vec<VarnodeId> = (0..ctx.high_num_instances(value)).map(|i| ctx.high_get_instance(value, i)).collect();
    let mut seen = std::collections::BTreeSet::new();
    while let Some(vn) = stack.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > WALK_BOUND {
            return true;
        }
        let size = ctx.vn_size(vn);
        for op in ctx.vn_descend(vn) {
            if ctx.op_is_dead(op) {
                continue;
            }
            let code = ctx.op_code(op);
            let n = ctx.op_num_input(op);
            for slot in 0..n {
                if ctx.op_in(op, slot) != Some(vn) {
                    continue;
                }
                let other = if n == 2 { ctx.op_in(op, 1 - slot).and_then(|o| ctx.vn_constant_value(o)) } else { None };
                if reads_signedness(code, slot, size, other) {
                    return true;
                }
                let passes = passes_signedness(code, slot)
                    || matches!(code, OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL)
                    || (code == OpCode::CPUI_INDIRECT && slot == 0);
                if !passes {
                    continue;
                }
                if let Some(out) = ctx.op_out(op) {
                    if ctx.vn_is_implied(out) && ctx.vn_high(out) != Some(value) {
                        stack.push(out);
                    }
                }
            }
        }
    }
    false
}

/// Is every instance of `value` a copy of `global` itself, through COPYs, CASTs
/// and MULTIEQUALs?  Such a variable is the global read into a register, and
/// keeping it apart would print the COPYs that write it back as stores the
/// binary never makes.
fn holds_global(ctx: &mut dyn MergeContext, value: HighVariableId, global: HighVariableId) -> bool {
    let mut stack: Vec<VarnodeId> = (0..ctx.high_num_instances(value)).map(|i| ctx.high_get_instance(value, i)).collect();
    let mut seen = std::collections::BTreeSet::new();
    while let Some(vn) = stack.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > 256 {
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
            OpCode::CPUI_MULTIEQUAL => stack.extend((0..ctx.op_num_input(def)).filter_map(|i| ctx.op_in(def, i))),
            _ => return false,
        }
    }
    true
}
