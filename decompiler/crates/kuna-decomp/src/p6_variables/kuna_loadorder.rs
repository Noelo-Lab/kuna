//! A value loaded through a pointer stays ahead of a store to a global the
//! pointer may reach (`option indexaliasguard global`).
//!
//! `ActionMarkImplied` lets a `LOAD`'s output print inline at its reader when
//! no `STORE` or call that may change the loaded memory lies on the way
//! (chapter 06, `check_implied_cover`).  A store to a global is neither: it is
//! a `COPY` (or arithmetic) into the global's own varnode, so
//! `x = *p; gi = b; return x;` printed `gi = a1; return *a0;`, which reads `b`
//! when `p` points at `gi`.  At the `global` level heritage's STORE guard
//! leaves each global store where the binary makes it, which moves such a
//! store in front of loads that used to print after it; [`crosses_global_write`]
//! keeps the load's own variable when its value is live across a store to a
//! global in the space it loads from, and [`keeps_apart`] keeps that variable
//! out of a global's: joined, the load would print as a write of the global
//! where the binary loads it, ahead of the store it crosses.

use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_indexaliasguard::LEVEL_GLOBAL;
use crate::merge::MergeContext;

/// Is `vn`, a `LOAD`'s output whose Cover is current, live across an operation
/// that writes a global in the space the `LOAD` reads?  A write is a live,
/// non-marker operation whose output is a persistent varnode in that space that
/// heritage's STORE guard covered ([`Funcdata::global_store_guarded`]), or
/// is merged into a HighVariable holding one (a store on a branch or in a loop
/// reaches the global through a `MULTIEQUAL`), other than a `COPY` of the same
/// storage (heritage's return copy) or a `COPY` of the global into a temporary
/// of its own HighVariable.  When the `LOAD`'s address is a constant, only a
/// global whose bytes overlap the loaded ones counts.  A write to the
/// HighVariable `except` does not count.
pub fn crosses_global_write(data: &mut Funcdata, vn: VarnodeId, except: Option<HighVariableId>) -> bool {
    if data.get_arch().index_alias_guard != LEVEL_GLOBAL {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let Some(load) = v.get_def().and_then(|d| data.obank().get(d)) else {
        return false;
    };
    if load.code() != OpCode::CPUI_LOAD {
        return false;
    }
    let Some(cover) = v.cover() else {
        return false;
    };
    let Some(index) = load.get_in(0).and_then(|s| data.vbank().get(s)).map(|s| s.get_offset()) else {
        return false;
    };
    let window = load
        .get_in(1)
        .and_then(|p| data.vbank().get(p))
        .filter(|p| p.is_constant())
        .map(|p| (p.get_offset(), p.get_offset().wrapping_add(v.get_size() as u64)));
    let mut writes: Vec<(VarnodeId, Option<HighVariableId>)> = Vec::new();
    for (blk, cb) in cover.iter() {
        if blk < 0 || blk >= data.bblocks_get_size() {
            continue;
        }
        for op in data.bb_ops(data.bblocks_get_block(blk)) {
            let point = data.op_cover_point_pub(op);
            if !(cb.contain(Some(point)) && cb.boundary(Some(point)) == 0) {
                continue;
            }
            if let Some(w) = written(data, op) {
                writes.push(w);
            }
        }
    }
    let in_window = |data: &Funcdata, w: VarnodeId| {
        let Some(w) = data.vbank().get(w) else {
            return false;
        };
        if !w.is_persist() || w.get_space().get_index() as u64 != index || !data.global_store_guarded(w) {
            return false;
        }
        let Some((lo, hi)) = window else {
            return true;
        };
        let (wlo, whi) = (w.get_offset(), w.get_offset().wrapping_add(w.get_size() as u64));
        wlo < hi && lo < whi
    };
    writes.into_iter().any(|(w, high)| {
        if except.is_some() && high == except {
            return false;
        }
        if in_window(data, w) {
            return true;
        }
        let Some(h) = high else {
            return false;
        };
        if !data.high_is_persist(h) {
            return false;
        }
        let Some(hv) = data.high_bank().get(h) else {
            return false;
        };
        (0..hv.num_instances()).any(|i| in_window(data, hv.get_instance(i)))
    })
}

/// The output of `op` and its HighVariable when `op` may write a global: a live,
/// non-marker op other than a `COPY` of the same storage, or a `COPY` into a
/// temporary of its input's HighVariable.
fn written(data: &Funcdata, op: OpId) -> Option<(VarnodeId, Option<HighVariableId>)> {
    let o = data.obank().get(op)?;
    if o.is_dead() || o.is_marker() || o.is_return_copy() {
        return None;
    }
    let wid = o.get_out()?;
    let w = data.vbank().get(wid)?;
    if o.code() == OpCode::CPUI_COPY {
        let i = o.get_in(0).and_then(|i| data.vbank().get(i))?;
        if (i.get_addr() == w.get_addr() && i.get_size() == w.get_size())
            || (!w.is_persist() && i.get_high().is_some() && i.get_high() == w.get_high())
        {
            return None;
        }
    }
    Some((wid, w.get_high()))
}

/// Must the HighVariables of `vn1` and `vn2` stay apart because one is a global
/// and the other holds a `LOAD` output whose value is live across a write of
/// another global ([`crosses_global_write`])?
pub fn keeps_apart(ctx: &mut dyn MergeContext, vn1: VarnodeId, vn2: VarnodeId) -> bool {
    let (Some(a), Some(b)) = (ctx.vn_high(vn1), ctx.vn_high(vn2)) else {
        return false;
    };
    if a == b {
        return false;
    }
    let (global, value) = match (ctx.high_is_persist(a), ctx.high_is_persist(b)) {
        (true, false) => (a, b),
        (false, true) => (b, a),
        _ => return false,
    };
    let loads: Vec<VarnodeId> = (0..ctx.high_num_instances(value))
        .map(|i| ctx.high_get_instance(value, i))
        .filter(|&v| ctx.vn_def(v).is_some_and(|d| ctx.op_code(d) == OpCode::CPUI_LOAD))
        .collect();
    loads.into_iter().any(|v| ctx.vn_load_crosses_global_write(v, global))
}
