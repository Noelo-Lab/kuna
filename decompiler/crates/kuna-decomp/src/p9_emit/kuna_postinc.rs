//! (kuna `postinc`) `v = *p; p = p + 1;` prints as `v = *p++;`.
//!
//! C source steps a pointer through a buffer with `*p++`, which compilers lower
//! to a load or store followed by a pointer add. [`ActionPostIncrement`]
//! runs once variables, casts and structure are final. An in-place step
//! `p = p + c` (`INT_ADD` or `PTRADD` whose output and first input are the same
//! variable) folds when the value it steps is read by exactly one other op, a
//! LOAD or STORE through it of `c` bytes, `p` points to a `c`-byte type (so `p++`
//! steps the same `c`), and the statement holding that LOAD or STORE is the one
//! printed right before the step in the same block. The step is marked
//! non-printing and the LOAD or STORE prints its pointer as `p++`. C output
//! only.

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Does `op` print a statement of its own?
fn prints(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    if o.not_printed() || matches!(o.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT) {
        return false;
    }
    o.get_out().and_then(|v| data.vbank().get(v)).is_none_or(|v| v.is_explicit())
}

/// The op whose statement prints `op`'s value: `op` itself when it prints,
/// else the single reader of its implied output, repeatedly.
fn statement_of(data: &Funcdata, mut op: OpId) -> Option<OpId> {
    for _ in 0..32 {
        if prints(data, op) {
            return Some(op);
        }
        let out = data.obank().get(op)?.get_out()?;
        let readers = data.descend_snapshot(out);
        let [next] = readers.as_slice() else { return None };
        op = *next;
    }
    None
}

/// The step `c` of an in-place `p = p + c`.
fn step(data: &Funcdata, op: OpId) -> Option<(VarnodeId, u64)> {
    let o = data.obank().get(op)?;
    let constant = |i: i32| data.vbank().get(o.get_in(i)?).filter(|v| v.is_constant()).map(|v| v.get_offset());
    let c = match o.code() {
        OpCode::CPUI_INT_ADD => constant(1)?,
        OpCode::CPUI_PTRADD if constant(1)? == 1 => constant(2)?,
        _ => return None,
    };
    let (out, p) = (o.get_out()?, o.get_in(0)?);
    let high = |v: VarnodeId| data.vbank().get(v).and_then(|n| n.get_high());
    (c > 0 && high(out).is_some() && high(out) == high(p)).then_some((p, c))
}

fn fold(data: &Funcdata, inc: OpId) -> Option<OpId> {
    let (p, c) = step(data, inc)?;
    let pv = data.vbank().get(p)?;
    if !pv.is_explicit() || pv.is_addr_tied() || pv.is_persist() {
        return None;
    }
    let ty = pv.get_type();
    if ty.get_metatype() != crate::dtype::type_metatype::TYPE_PTR || ty.get_ptr_to().is_none_or(|t| t.get_size() as u64 != c) {
        return None;
    }
    let readers = data.descend_snapshot(p);
    let [a, b] = readers.as_slice() else { return None };
    let access = if *a == inc { *b } else if *b == inc { *a } else { return None };
    if crate::kuna_stackmem::access(data, access).is_some() {
        return None;
    }
    let ao = data.obank().get(access)?;
    let size = match ao.code() {
        OpCode::CPUI_LOAD if ao.get_in(1) == Some(p) => data.vbank().get(ao.get_out()?)?.get_size(),
        OpCode::CPUI_STORE if ao.get_in(1) == Some(p) && ao.get_in(2) != Some(p) => data.vbank().get(ao.get_in(2)?)?.get_size(),
        _ => return None,
    };
    if size as u64 != c {
        return None;
    }
    let block = data.obank().get(inc)?.get_parent()?;
    let stmt = statement_of(data, access)?;
    if data.obank().get(stmt)?.get_parent()? != block {
        return None;
    }
    let ops = data.bb_ops(block);
    let (s, i) = (ops.iter().position(|&o| o == stmt)?, ops.iter().position(|&o| o == inc)?);
    (s < i && ops[s + 1..i].iter().all(|&o| !prints(data, o))).then_some(access)
}

/// Fold pointer steps into post-increments (see the module docs).
pub struct ActionPostIncrement {
    base: ActionBase,
}

impl ActionPostIncrement {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionPostIncrement { base: ActionBase::new(0, "postinc", g) })
    }
}

impl Action for ActionPostIncrement {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionPostIncrement { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().post_increment {
            return 0;
        }
        let ops: Vec<OpId> = data.obank().iter_alive().collect();
        for inc in ops {
            if !data.obank().get(inc).is_some_and(|o| o.get_parent().is_some() && matches!(o.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD)) || !prints(data, inc) {
                continue;
            }
            let Some(access) = fold(data, inc) else { continue };
            data.op_mark_non_printing_pub(inc);
            if let Some(o) = data.obank_mut().get_mut(access) {
                o.set_additional_flag(crate::op::pcodeop_addlflags::kuna_postinc);
            }
            self.base.count += 1;
        }
        0
    }
}
