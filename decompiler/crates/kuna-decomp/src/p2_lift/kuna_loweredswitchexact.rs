//! (kuna `loweredswitchexact`) Install a re-rolled lowered switch only when it is
//! the compare tree it replaces.
//!
//! [`crate::kuna_loweredswitch`] recovers a case map and a default from a
//! comparison cascade, and the install replaces the cascade with a switch over
//! that map. Three things made the switch a different program. The install kept
//! one label per case body, so a value sharing its body with another case fell
//! into the default. The recovered map can route a value somewhere the tree does
//! not: a range edge the binary search pins to one value is neither a case nor a
//! default candidate, and a tie between two default candidates is broken by
//! address. And the install deletes every compare block behind the head, with
//! whatever a block computes for code outside the cascade.
//!
//! With this on every case value keeps its label, and a record is kept only when
//! [`switch_routes_like_cascade`] shows the switch the install builds sends every
//! value of the switch variable to the block the tree sends it to, and
//! [`install_keeps_statements`] shows no block the install deletes has an effect
//! outside the deleted blocks.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::{calc_int_min, calc_mask, sign_extend_sized, Address};
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::block::{block_get_start, BlockType};
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::KunaLoweredSwitchRecord;

/// Marshaling element `<loweredswitchexact>` (kuna 4000+ range).
pub const ELEM_LOWEREDSWITCHEXACT: ElementId = ElementId::new("loweredswitchexact", 4171);

/// True when the switch built from `rec` behaves as the compare tree at `head`:
/// [`switch_routes_like_cascade`] and [`install_keeps_statements`].
pub(crate) fn record_is_exact(
    data: &Funcdata,
    head: BlockId,
    spine: &BTreeSet<BlockId>,
    swvar: VarnodeId,
    rec: &KunaLoweredSwitchRecord,
    all_labels: bool,
) -> bool {
    let mut targets: Vec<BlockId> = Vec::with_capacity(rec.case_targets.len() + 1);
    for addr in rec.case_targets.iter().chain(std::iter::once(&rec.default_target)) {
        match block_at_start(data, addr) {
            Some(b) => targets.push(b),
            None => return false,
        }
    }
    switch_routes_like_cascade(data, head, spine, swvar, rec, all_labels) && install_keeps_statements(data, head, &targets)
}

/// One compare block, decoded for evaluation.
struct Node {
    cmp: OpCode,
    var_on_left: bool,
    operand: VarnodeId,
    cval: uintb,
    width: int4,
    on_true: BlockId,
    on_false: BlockId,
}

/// True when the switch `rec` describes routes every value of `swvar` as the
/// compare tree rooted at `head` does.
///
/// The switch is the one the install builds: every case value labels its body
/// when `all_labels`, otherwise only the first value of each body does, and a
/// case whose body is the default folds into it. The tree is evaluated through
/// the blocks of `spine` reachable from `head` until it leaves them. Each compare
/// is an equality or a threshold against a constant over the switch variable
/// widened by zero or sign extension, so its outcome is constant between the
/// constant's neighbourhood and the switch variable's signed and unsigned wrap
/// points; evaluating all of those points covers every value.
pub(crate) fn switch_routes_like_cascade(
    data: &Funcdata,
    head: BlockId,
    spine: &BTreeSet<BlockId>,
    swvar: VarnodeId,
    rec: &KunaLoweredSwitchRecord,
    all_labels: bool,
) -> bool {
    let size = match data.vbank().get(swvar) {
        Some(v) => v.get_size(),
        None => return false,
    };
    if !(1..=8).contains(&size) || rec.case_vals.len() != rec.case_targets.len() {
        return false;
    }
    let mask = calc_mask(size);

    let mut labels: BTreeMap<uintb, &Address> = BTreeMap::new();
    let mut bodies: Vec<&Address> = Vec::new();
    for (val, tgt) in rec.case_vals.iter().zip(rec.case_targets.iter()) {
        if *tgt == rec.default_target {
            continue;
        }
        if bodies.contains(&tgt) && !all_labels {
            continue;
        }
        bodies.push(tgt);
        let low = *val & mask;
        if *val != low && *val != sign_extend_sized(low, size, 8) {
            return false;
        }
        if labels.insert(low, tgt).is_some() {
            return false;
        }
    }

    let mut nodes: BTreeMap<BlockId, Node> = BTreeMap::new();
    let mut queue: Vec<BlockId> = vec![head];
    while let Some(bl) = queue.pop() {
        if nodes.contains_key(&bl) || !spine.contains(&bl) {
            continue;
        }
        let n = match decode_node(data, bl) {
            Some(n) => n,
            None => return false,
        };
        queue.push(n.on_true);
        queue.push(n.on_false);
        nodes.insert(bl, n);
    }

    let mut points: BTreeSet<uintb> = BTreeSet::new();
    let int_min = calc_int_min(size);
    for p in [0, mask, int_min, int_min.wrapping_sub(1)] {
        points.insert(p & mask);
    }
    for n in nodes.values() {
        for p in [n.cval.wrapping_sub(1), n.cval, n.cval.wrapping_add(1)] {
            points.insert(p & mask);
        }
    }

    for &v in &points {
        let reached = match walk(data, head, &nodes, swvar, size, v) {
            Some(b) => b,
            None => return false,
        };
        let expected = labels.get(&v).copied().unwrap_or(&rec.default_target);
        if block_get_start(&data.bblocks_ref().arena, reached) != *expected {
            return false;
        }
    }
    true
}

/// Decode a block's trailing `CBRANCH` over a comparison with a constant.
fn decode_node(data: &Funcdata, bl: BlockId) -> Option<Node> {
    let cb = data.bb_op_tail(bl)?;
    let cbop = data.obank().get(cb)?;
    if cbop.code() != OpCode::CPUI_CBRANCH || data.bblocks_ref().block(bl).size_out() != 2 {
        return None;
    }
    let flip = cbop.is_boolean_flip();
    let cmpop = data.obank().get(data.vbank().get(cbop.get_in(1)?)?.get_def()?)?;
    let cmp = cmpop.code();
    if !matches!(
        cmp,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
    ) {
        return None;
    }
    let in0 = data.vbank().get(cmpop.get_in(0)?)?;
    let in1 = data.vbank().get(cmpop.get_in(1)?)?;
    let (operand, cvn, var_on_left) = match (in0.is_constant(), in1.is_constant()) {
        (false, true) => (cmpop.get_in(0)?, in1, true),
        (true, false) => (cmpop.get_in(1)?, in0, false),
        _ => return None,
    };
    let width = cvn.get_size();
    let block = data.bblocks_ref().block(bl);
    let (t, f) = (block.get_true_out(), block.get_false_out());
    let (on_true, on_false) = if flip { (f, t) } else { (t, f) };
    Some(Node { cmp, var_on_left, operand, cval: cvn.get_offset() & calc_mask(width), width, on_true, on_false })
}

/// Follow the compare tree for switch value `v` until it leaves the tree.
fn walk(data: &Funcdata, head: BlockId, nodes: &BTreeMap<BlockId, Node>, swvar: VarnodeId, size: int4, v: uintb) -> Option<BlockId> {
    let mut bl = head;
    for _ in 0..=nodes.len() {
        let n = match nodes.get(&bl) {
            Some(n) => n,
            None => return Some(bl),
        };
        let x = operand_value(data, n.operand, swvar, size, v)? & calc_mask(n.width);
        let (a, b) = if n.var_on_left { (x, n.cval) } else { (n.cval, x) };
        let sa = sign_extend_sized(a, n.width, 8) as i64;
        let sb = sign_extend_sized(b, n.width, 8) as i64;
        let holds = match n.cmp {
            OpCode::CPUI_INT_EQUAL => a == b,
            OpCode::CPUI_INT_NOTEQUAL => a != b,
            OpCode::CPUI_INT_LESS => a < b,
            OpCode::CPUI_INT_LESSEQUAL => a <= b,
            OpCode::CPUI_INT_SLESS => sa < sb,
            _ => sa <= sb,
        };
        bl = if holds { n.on_true } else { n.on_false };
    }
    None
}

/// The value of `vn` when the switch variable holds `v`, through the copies,
/// casts, zero-offset same-width pieces and extensions between them.
fn operand_value(data: &Funcdata, mut vn: VarnodeId, swvar: VarnodeId, size: int4, v: uintb) -> Option<uintb> {
    let mut chain: Vec<(OpCode, int4, int4)> = Vec::new();
    for _ in 0..8 {
        if vn == swvar {
            let mut val = v & calc_mask(size);
            for &(oc, insize, outsize) in chain.iter().rev() {
                if oc == OpCode::CPUI_INT_SEXT {
                    val = sign_extend_sized(val, insize, outsize);
                }
                val &= calc_mask(outsize);
            }
            return Some(val);
        }
        let node = data.vbank().get(vn)?;
        let def = data.obank().get(node.get_def()?)?;
        let input = def.get_in(0)?;
        let insize = data.vbank().get(input)?.get_size();
        let outsize = node.get_size();
        let transparent = match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => insize == outsize,
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => insize < outsize,
            OpCode::CPUI_SUBPIECE => {
                insize == outsize
                    && def.get_in(1).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0)
            }
            _ => false,
        };
        if !transparent {
            return None;
        }
        chain.push((def.code(), insize, outsize));
        vn = input;
    }
    None
}

/// True when no basic block the install would delete has an effect outside the
/// deleted blocks.
///
/// The install replaces `head`'s out-edges with edges to `targets` and removes
/// what is no longer reachable. Each removed block may branch and may compute
/// values read only by other removed blocks; a store, a call, a return, an
/// indirect branch, or a value read anywhere else is a statement the switch
/// would lose.
pub(crate) fn install_keeps_statements(data: &Funcdata, head: BlockId, targets: &[BlockId]) -> bool {
    let removed = removed_blocks(data, head, targets);
    for &b in &removed {
        for op in data.bb_ops(b) {
            let o = match data.obank().get(op) {
                Some(o) => o,
                None => return false,
            };
            if deleted_op_is_statement(o.code()) {
                return false;
            }
            let out = match o.get_out().and_then(|v| data.vbank().get(v)) {
                Some(out) => out,
                None => continue,
            };
            for reader in out.descend_iter() {
                let parent = data.obank().get(reader).and_then(|r| r.get_parent());
                if !parent.is_some_and(|p| removed.contains(&p)) {
                    return false;
                }
            }
        }
    }
    true
}

/// True when, on the raw p-code the install edits, nothing a deleted block
/// writes can be read again.
///
/// Detection judges statements on the simplified graph, where a register a compare
/// block loads for a case body has been propagated into its reader
/// (`mov esi,7 ; cmp edi,200 ; je T` with `T: mov edi,esi ; jmp hv` reads as
/// `hv(7)`), so the load no longer shows in the compare block. Here every storage
/// location a deleted block writes must, on every path from a target, be written
/// again (by an op, or by a call when the prototype model says calls kill it)
/// before an op reads it, before a call or tail call that may take it as a
/// parameter, and before a return that may return it.
pub(crate) fn raw_install_keeps_values(data: &Funcdata, head: BlockId, targets: &[BlockId]) -> bool {
    let removed = removed_blocks(data, head, targets);
    let mut written: Vec<(Address, int4)> = Vec::new();
    for &b in &removed {
        for op in data.bb_ops(b) {
            let o = match data.obank().get(op) {
                Some(o) => o,
                None => return false,
            };
            if deleted_op_is_statement(o.code()) {
                return false;
            }
            let out = match o.get_out().and_then(|v| data.vbank().get(v)) {
                Some(out) => out,
                None => continue,
            };
            if out.get_addr().get_space().map(|s| s.get_type()) == Some(spacetype::IPTR_INTERNAL) {
                continue;
            }
            let loc = (out.get_addr().clone(), out.get_size());
            if !written.contains(&loc) {
                written.push(loc);
            }
        }
    }
    written.iter().all(|(addr, size)| !read_before_written(data, &removed, targets, addr, *size))
}

/// True when some path from `targets` that avoids `removed` reads the `size`
/// bytes at `addr` before writing all of them.
fn read_before_written(data: &Funcdata, removed: &BTreeSet<BlockId>, targets: &[BlockId], addr: &Address, size: int4) -> bool {
    let space = match addr.get_space() {
        Some(s) => s.get_index(),
        None => return true,
    };
    let lo = addr.get_offset();
    let hi = lo.wrapping_add(size as uintb);
    let proto = data.get_func_proto();
    let modelled = proto.has_model();
    let param = !modelled || proto.possible_input_param(addr, size);
    let result = !modelled || proto.possible_output_param(addr, size);
    let killed = modelled && proto.has_effect(addr, size) == crate::fspec::effect_type::KILLEDBYCALL;
    let mut seen: BTreeSet<BlockId> = BTreeSet::new();
    let mut stack: Vec<BlockId> = targets.to_vec();
    while let Some(b) = stack.pop() {
        if removed.contains(&b) || !seen.insert(b) {
            continue;
        }
        let mut rewritten = false;
        for op in data.bb_ops(b) {
            let o = match data.obank().get(op) {
                Some(o) => o,
                None => return true,
            };
            for i in 0..o.num_input() {
                let v = match o.get_in(i).and_then(|v| data.vbank().get(v)) {
                    Some(v) => v,
                    None => continue,
                };
                let vlo = v.get_offset();
                if !v.is_constant()
                    && v.get_addr().get_space().map(|s| s.get_index()) == Some(space)
                    && vlo < hi
                    && lo < vlo.wrapping_add(v.get_size() as uintb)
                {
                    return true;
                }
            }
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if param => return true,
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if killed => {
                    rewritten = true;
                    break;
                }
                OpCode::CPUI_RETURN if result => return true,
                _ => {}
            }
            if let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) {
                let olo = out.get_offset();
                if out.get_addr().get_space().map(|s| s.get_index()) == Some(space)
                    && olo <= lo
                    && hi <= olo.wrapping_add(out.get_size() as uintb)
                {
                    rewritten = true;
                    break;
                }
            }
        }
        if rewritten {
            continue;
        }
        let blk = data.bblocks_ref().block(b);
        if blk.size_out() == 0 && param {
            return true;
        }
        stack.extend((0..blk.size_out()).map(|i| blk.get_out(i)));
    }
    false
}

fn deleted_op_is_statement(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_STORE
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_NEW
            | OpCode::CPUI_CPOOLREF
    )
}

/// The basic blocks the install deletes: reachable from an entry now, and not once
/// `head` branches straight to `targets`.
pub(crate) fn removed_blocks(data: &Funcdata, head: BlockId, targets: &[BlockId]) -> BTreeSet<BlockId> {
    let n = data.bblocks_get_size();
    let blocks: Vec<BlockId> = (0..n).map(|i| data.bblocks_get_block(i)).collect();
    let mut roots: Vec<BlockId> =
        blocks.iter().copied().filter(|&b| data.bblocks_ref().block(b).is_entry_point()).collect();
    if roots.is_empty() {
        roots = blocks.iter().copied().filter(|&b| data.bblocks_ref().block(b).size_in() == 0).collect();
    }
    let reach = |rewire: bool| -> BTreeSet<BlockId> {
        let mut reached: BTreeSet<BlockId> = BTreeSet::new();
        let mut stack = roots.clone();
        while let Some(b) = stack.pop() {
            if !reached.insert(b) {
                continue;
            }
            if rewire && b == head {
                stack.extend(targets.iter().copied());
                continue;
            }
            let blk = data.bblocks_ref().block(b);
            stack.extend((0..blk.size_out()).map(|i| blk.get_out(i)));
        }
        reached
    };
    let after = reach(true);
    reach(false)
        .into_iter()
        .filter(|b| !after.contains(b) && data.bblocks_ref().block(*b).get_type() == BlockType::Basic)
        .collect()
}

/// The basic block whose first address is `addr`.
pub(crate) fn block_at_start(data: &Funcdata, addr: &Address) -> Option<BlockId> {
    (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).find(|&b| {
        data.bblocks_ref().block(b).get_type() == BlockType::Basic && block_get_start(&data.bblocks_ref().arena, b) == *addr
    })
}

#[cfg(test)]
#[path = "kuna_loweredswitchexact/tests.rs"]
mod tests;
