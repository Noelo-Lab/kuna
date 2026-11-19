//! (kuna `indexrescale`) A byte offset into an array becomes an element index.
//!
//! A loop or a table walk often keeps a byte offset in a register and adds it
//! to the base, which prints `*(int *)((long)rule + v2)` with `v2 = 4;` and
//! `v2 = 0xc;`. Rescaled, it prints `rule[v2]` with `v2 = 1;` and `v2 = 3;`.
//!
//! [`ActionIndexRescale`] starts at an `INT_ADD(p, w)` whose `p` points to
//! `k`-byte elements (`k > 1`; not the address of a structure field that is
//! not an array, where the offset picks a field) and collects the web of `w`: the varnodes joined
//! to it through MULTIEQUAL, INDIRECT, COPY and steps `INT_ADD(x, c)`. It
//! rescales the web when every member is written by a COPY of a constant, a
//! step by a constant, a MULTIEQUAL or INDIRECT of members, or a COPY of a
//! member, every constant is a multiple of `k`, and every reader of a member is
//! one of those writes, an `INT_ADD` with a base pointing to `k`-byte elements,
//! or an equality or ordering compare with a constant multiple of `k`. No
//! member is an input, address-tied or persistent. The constants are divided by
//! `k` (signed), each `INT_ADD(p, w)` becomes `PTRADD(p, w, k)`, and compares
//! take the divided constant. This assumes the byte offset does not wrap (it
//! stays within the signed range of its size), as an array offset does.

use std::collections::BTreeSet;

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

const MAX_WEB: usize = 64;

/// The element size `p` points to, when it is a pointer to elements of more
/// than one byte.
fn element_size(data: &Funcdata, p: VarnodeId) -> Option<u64> {
    let t = data.vbank().get(p)?.get_type();
    if t.get_metatype() != crate::dtype::type_metatype::TYPE_PTR {
        return None;
    }
    let k = t.get_ptr_to()?.get_size() as u64;
    (k > 1 && !scalar_field(data, p)).then_some(k)
}

/// Is `p` the address of a structure field that is not an array? An offset
/// added to it selects another field, not an element.
fn scalar_field(data: &Funcdata, p: VarnodeId) -> bool {
    use crate::dtype::type_metatype::{TYPE_ARRAY, TYPE_STRUCT};
    let Some(d) = data.vbank().get(p).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else { return false };
    if d.code() != OpCode::CPUI_PTRSUB {
        return false;
    }
    let (Some(base), Some(off)) = (d.get_in(0).and_then(|b| data.vbank().get(b)), d.get_in(1).and_then(|o| data.vbank().get(o))) else { return false };
    let Some(record) = base.get_type().get_ptr_to().filter(|t| t.get_metatype() == TYPE_STRUCT) else { return false };
    match record.get_sub_type(off.get_offset() as i64) {
        Ok((Some(field), _)) => field.get_metatype() != TYPE_ARRAY,
        _ => true,
    }
}

fn constant(data: &Funcdata, v: VarnodeId) -> Option<(u64, int4)> {
    let n = data.vbank().get(v)?;
    n.is_constant().then(|| (n.get_offset(), n.get_size()))
}

/// `c` as a signed value of `size` bytes divided by `k`, when exact.
fn divide(c: u64, size: int4, k: u64) -> Option<u64> {
    let bits = 8 * size.clamp(1, 8) as u32;
    let signed = ((c << (64 - bits)) as i64) >> (64 - bits);
    (signed % k as i64 == 0).then(|| ((signed / k as i64) as u64) & kuna_base::address::calc_mask(size))
}

/// What a reader of a web member does with it.
enum Use {
    /// Writes another member (MULTIEQUAL, INDIRECT, COPY, step).
    Member(VarnodeId),
    /// `INT_ADD(p, w)`: becomes `PTRADD(p, w, k)`; the slot of `p`.
    Address(OpId, int4),
    /// A compare with a constant in the given slot.
    Compare(OpId, int4),
}

struct Web {
    members: BTreeSet<VarnodeId>,
    k: u64,
    /// Ops whose constant input (slot) is divided.
    constants: Vec<(OpId, int4)>,
    addresses: Vec<(OpId, int4)>,
}

fn classify(data: &Funcdata, op: OpId, w: VarnodeId, k: u64) -> Option<Use> {
    use OpCode::*;
    let o = data.obank().get(op)?;
    let out = o.get_out();
    match o.code() {
        CPUI_MULTIEQUAL | CPUI_COPY => out.map(Use::Member),
        CPUI_INDIRECT if o.get_in(0) == Some(w) => out.map(Use::Member),
        CPUI_INT_ADD => {
            let (a, b) = (o.get_in(0)?, o.get_in(1)?);
            let other = if a == w { b } else { a };
            if let Some((c, s)) = constant(data, other) {
                divide(c, s, k)?;
                return out.map(Use::Member);
            }
            (element_size(data, other) == Some(k)).then(|| Use::Address(op, if a == w { 1 } else { 0 }))
        }
        CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL => {
            let slot = if o.get_in(0) == Some(w) { 1 } else { 0 };
            let (c, s) = constant(data, o.get_in(slot)?)?;
            divide(c, s, k)?;
            Some(Use::Compare(op, slot))
        }
        _ => None,
    }
}

/// Is `op`, the def of a member, a write the web allows? Its constant slot,
/// if any, is recorded.
fn writer(data: &Funcdata, op: OpId, members: &BTreeSet<VarnodeId>, k: u64, constants: &mut Vec<(OpId, int4)>, frontier: &mut Vec<VarnodeId>) -> bool {
    use OpCode::*;
    let Some(o) = data.obank().get(op) else { return false };
    let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
    match o.code() {
        CPUI_COPY => match constant(data, ins[0]) {
            Some((c, s)) => {
                if divide(c, s, k).is_none() {
                    return false;
                }
                constants.push((op, 0));
                true
            }
            None => {
                frontier.push(ins[0]);
                true
            }
        },
        CPUI_MULTIEQUAL => {
            frontier.extend(ins);
            true
        }
        CPUI_INDIRECT => {
            frontier.push(ins[0]);
            true
        }
        CPUI_INT_ADD if ins.len() == 2 => {
            let (slot, c) = match (constant(data, ins[0]), constant(data, ins[1])) {
                (None, Some(c)) => (1, c),
                (Some(c), None) => (0, c),
                _ => return false,
            };
            if divide(c.0, c.1, k).is_none() {
                return false;
            }
            let x = ins[1 - slot as usize];
            if !members.contains(&x) {
                frontier.push(x);
            }
            constants.push((op, slot));
            true
        }
        _ => false,
    }
}

fn collect(data: &Funcdata, start: VarnodeId, k: u64) -> Option<Web> {
    let mut members: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut constants = Vec::new();
    let mut addresses = Vec::new();
    let mut compares = Vec::new();
    let mut work = vec![start];
    while let Some(v) = work.pop() {
        if !members.insert(v) {
            continue;
        }
        if members.len() > MAX_WEB {
            return None;
        }
        let n = data.vbank().get(v)?;
        if n.is_constant() || n.is_input() || n.is_addr_tied() || n.is_persist() || n.is_free() {
            return None;
        }
        let def = n.get_def()?;
        let mut frontier = Vec::new();
        if !writer(data, def, &members, k, &mut constants, &mut frontier) {
            return None;
        }
        work.extend(frontier);
        for d in data.descend_snapshot(v) {
            match classify(data, d, v, k)? {
                Use::Member(out) => work.push(out),
                Use::Address(op, slot) => addresses.push((op, slot)),
                Use::Compare(op, slot) => compares.push((op, slot)),
            }
        }
    }
    if addresses.is_empty() {
        return None;
    }
    constants.extend(compares);
    constants.sort();
    constants.dedup();
    addresses.sort();
    addresses.dedup();
    Some(Web { members, k, constants, addresses })
}

fn rewrite(data: &mut Funcdata, web: &Web) -> kuna_base::error::KunaResult<()> {
    for &(op, slot) in &web.constants {
        let Some(v) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { continue };
        let Some((c, s)) = constant(data, v) else { continue };
        let Some(q) = divide(c, s, web.k) else { continue };
        let nc = data.new_constant(s, q as uintb);
        data.op_set_input(op, nc, slot)?;
    }
    for &(op, base_slot) in &web.addresses {
        let (Some(p), Some(w)) = (data.obank().get(op).and_then(|o| o.get_in(base_slot)), data.obank().get(op).and_then(|o| o.get_in(1 - base_slot))) else {
            continue;
        };
        let size = data.vbank().get(w).map_or(8, |v| v.get_size());
        let k = data.new_constant(size, web.k as uintb);
        data.op_set_all_input(op, &[p, w, k])?;
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_PTRADD));
    }
    Ok(())
}

/// Rescale byte offsets into element indexes (see the module docs).
pub struct ActionIndexRescale {
    base: ActionBase,
}

impl ActionIndexRescale {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionIndexRescale { base: ActionBase::new(0, "indexrescale", g) })
    }
}

impl Action for ActionIndexRescale {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionIndexRescale { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().index_rescale {
            return 0;
        }
        let ops: Vec<OpId> = data.obank().iter_alive().collect();
        let mut done: BTreeSet<VarnodeId> = BTreeSet::new();
        for op in ops {
            let Some(o) = data.obank().get(op) else { continue };
            if o.code() != OpCode::CPUI_INT_ADD || o.get_parent().is_none() {
                continue;
            }
            let (Some(a), Some(b)) = (o.get_in(0), o.get_in(1)) else { continue };
            let (w, k) = match (element_size(data, a), element_size(data, b)) {
                (Some(k), None) => (b, k),
                (None, Some(k)) => (a, k),
                _ => continue,
            };
            if done.contains(&w) || constant(data, w).is_some() {
                continue;
            }
            let Some(web) = collect(data, w, k) else {
                done.insert(w);
                continue;
            };
            if rewrite(data, &web).is_ok() {
                self.base.count += 1;
            }
            done.extend(web.members.iter().copied());
        }
        0
    }
}
