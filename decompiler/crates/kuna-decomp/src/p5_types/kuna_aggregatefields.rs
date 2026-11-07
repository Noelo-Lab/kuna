//! (kuna `aggregatefields`) Keep a structured value whole until its fields can
//! be named.
//!
//! Upstream splits or misnames such a value at four places: `RuleConcatZero`
//! rewrites a struct-typed concatenation with zero low bytes into a shifted
//! integer before `RulePieceStructure` can see it; single-component scoring
//! resolves a returned, unchanged declared container to its only field;
//! cleanup splitting runs after the main simplification pool and leaves the
//! nested and unused truncations it creates; and the partial-symbol walk
//! descends into a single-field struct even where the operation's resolved type
//! is the whole struct. Each place asks one predicate here, which answers as
//! upstream when the option is off. ABI padding pieces of a join are outside
//! this option: they have no storage, so they are always logical constants.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::action::Rule;
use crate::context::{OpId, VarnodeId};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;

/// Longest equal-width COPY chain followed back to a declared input.
const MAX_COPY_CHAIN: usize = 16;

/// Whether `RuleConcatZero` leaves `op` a PIECE: its output is a structured
/// value whose fields `RulePieceStructure` recovers.
pub fn keeps_structured_concat(data: &Funcdata, op: OpId) -> bool {
    data.get_arch().aggregate_fields
        && data.obank().get(op).and_then(|o| o.get_out())
            .and_then(|out| data.vbank().get(out))
            .is_some_and(|out| out.get_type().is_piece_structured())
}

/// Whether single-component scoring resolves edge `slot` of `op` to the whole
/// container `parent`: a RETURN value of the function's locked output type, or
/// either edge of a COPY on the chain feeding it, that is an unchanged copy of a
/// type-locked input of that container.
pub fn returns_whole_container(fd: &Funcdata, op: OpId, slot: i32, parent: &Rc<Datatype>) -> bool {
    let proto = fd.get_func_proto();
    if !fd.get_arch().aggregate_fields || !proto.is_output_locked()
        || !proto.get_output_type().is_some_and(|ty| Rc::ptr_eq(ty, parent))
    {
        return false;
    }
    let Some(o) = fd.obank().get(op) else { return false };
    match o.code() {
        OpCode::CPUI_RETURN if slot >= 1 => {
            o.get_in(slot).is_some_and(|value| whole_container_copy(fd, value, parent))
        }
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT if slot <= 0 => {
            o.get_in(0).is_some_and(|input| whole_container_copy(fd, input, parent))
                && o.get_out().is_some_and(|out| copies_reach_return(fd, out))
        }
        _ => false,
    }
}

fn copies_reach_return(fd: &Funcdata, value: VarnodeId) -> bool {
    let mut pending = vec![value];
    for _ in 0..MAX_COPY_CHAIN {
        let Some(value) = pending.pop() else { return false };
        for reader in fd.descend_snapshot(value) {
            match fd.obank().get(reader) {
                Some(op) if op.code() == OpCode::CPUI_RETURN => return true,
                Some(op) if op.code() == OpCode::CPUI_COPY => pending.extend(op.get_out()),
                _ => {}
            }
        }
    }
    false
}

fn whole_container_copy(fd: &Funcdata, mut value: VarnodeId, parent: &Rc<Datatype>) -> bool {
    for _ in 0..MAX_COPY_CHAIN {
        let Some(vn) = fd.vbank().get(value) else { return false };
        if vn.get_size() != parent.get_size() {
            return false;
        }
        if vn.is_input() && vn.is_type_lock() && Rc::ptr_eq(vn.get_type(), parent) {
            return true;
        }
        let Some(op) = vn.get_def().and_then(|id| fd.obank().get(id)) else { return false };
        if op.code() != OpCode::CPUI_COPY {
            return false;
        }
        let Some(input) = op.get_in(0) else { return false };
        value = input;
    }
    false
}

/// Collapse the nested truncation a cleanup-split SUBPIECE may read through the
/// existing double-subpiece rule: the main simplification pool has already
/// finished when cleanup splitting runs.
pub fn collapse_split_truncation(data: &mut Funcdata, subpiece: OpId) {
    if data.get_arch().aggregate_fields {
        while crate::ruleaction_2::RuleDoubleSub.apply_op(subpiece, data) != 0 {}
    }
}

/// Remove the truncations cleanup splitting left unread, from `value` back
/// through its SUBPIECE sources, using the normal dead-code rule.
pub fn discard_unused_truncations(data: &mut Funcdata, mut value: VarnodeId) {
    if !data.get_arch().aggregate_fields {
        return;
    }
    while let Some(op) = data.vbank().get(value).and_then(|vn| vn.get_def()) {
        let Some(def) = data.obank().get(op) else { break };
        if def.code() != OpCode::CPUI_SUBPIECE {
            break;
        }
        let Some(source) = def.get_in(0) else { break };
        if crate::ruleaction_1::RuleEarlyRemoval::new("cleanup").apply_op(op, data) == 0 {
            break;
        }
        value = source;
    }
}

/// The type of `vn` on the edge the partial-symbol walk renders (read by `op`
/// at `slot`, or as written), or `None` when the option is off.
pub fn edge_type(fd: &Funcdata, vn: VarnodeId, op: OpId, slot: i32) -> Option<Rc<Datatype>> {
    if !fd.get_arch().aggregate_fields {
        None
    } else if slot >= 0 {
        Some(fd.vn_type_read_facing(vn, op))
    } else if fd.vbank().get(vn).is_some_and(|v| v.is_written()) {
        Some(fd.vn_type_def_facing(vn))
    } else {
        None
    }
}

/// Whether the walk stops at single-field struct `cur` because the operation's
/// cached resolution of the `size`-byte access is the whole struct.
pub fn resolves_whole_struct(fd: &Funcdata, cur: &Rc<Datatype>, size: i32, op: OpId, slot: i32) -> bool {
    fd.get_arch().aggregate_fields && cur.needs_resolution() && cur.get_size() == size
        && Rc::ptr_eq(cur, &fd.find_resolve_facing(cur, op, slot))
}
