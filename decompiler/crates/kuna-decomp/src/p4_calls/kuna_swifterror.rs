//! (kuna `swiftcall`) Name the Swift error register's two moments: the
//! error a throwing call leaves behind, and the one a throwing function
//! hands back.
//!
//! # The defect
//!
//! A throwing Swift function reports its error in a register the caller
//! zeroes before the call and tests after it (`r12` on x86-64, `x21` on
//! arm64). The `__swiftcall_throws` model lists that register as killed by
//! the call, which stops the zero from reaching the test, but the register
//! then has no definition at all: the test reads an unnamed, uninitialized
//! variable (`void *v5; // r12` ... `if (v5)`). On the callee side the write
//! of the error before `ret` reaches nothing, so the error value is dead and
//! `swift_allocError(...)`'s result is dropped on the floor.
//!
//! # The fix
//!
//! Before heritage, once every call site has its model:
//!
//! - after a call under a throwing Swift model, an op defines the error
//!   register with the opaque `swift_error()`, so the caller's test reads the
//!   call's error as a call result; where nothing reads it, the op is dead
//!   code and goes;
//! - in a function under a throwing model, the last write of the error
//!   register before a `return` (no call between them) feeds a
//!   `swift_throw(e)`, so the error handed back is visible at the return.
//!
//! Both ops are ordinary writes and reads; heritage treats them like any
//! other. They act only on an image whose Swift ABI the loader bound
//! (option `swiftcall`).

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::kuna_x64syscall::overlaps;
use crate::op::{PcodeOpBank, pcodeop_flags};
use crate::userop::{BUILTIN_SWIFT_ERROR, BUILTIN_SWIFT_THROW};
use crate::varnode::VarnodeBank;

/// Is `op` the `swift_error()` this action placed after a throwing call?
pub fn is_error(data: &Funcdata, op: OpId) -> bool {
    is_error_op(data.obank(), data.vbank(), op)
}

/// [`is_error`] over the op and varnode banks alone.
pub fn is_error_op(obank: &PcodeOpBank, vbank: &VarnodeBank, op: OpId) -> bool {
    obank.get(op).is_some_and(|o| {
        o.code() == OpCode::CPUI_CALLOTHER
            && o.get_in(0)
                .and_then(|v| vbank.get(v))
                .is_some_and(|v| v.is_constant() && v.get_offset() == BUILTIN_SWIFT_ERROR as u64)
    })
}

/// A Swift model under which the error register carries a value.
fn throws(name: &str) -> bool {
    name.starts_with("__swift") && name.ends_with("_throws")
}

/// The op is a call or an opaque operation that may change the register.
fn is_call(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
    )
}

/// Define the error register right after the throwing call at `call` with
/// `swift_error()`.
fn define_error(data: &mut Funcdata, call: OpId, reg: &Address, size: int4) -> KunaResult<()> {
    let pc = data
        .obank()
        .get(call)
        .map(|o| o.get_addr().clone())
        .ok_or_else(|| KunaError::lowlevel("swifterror: stale call"))?;
    let op = data.new_op(1, pc);
    data.op_set_opcode_code(op, OpCode::CPUI_CALLOTHER);
    if let Some(o) = data.obank_mut().get_mut(op) {
        o.clear_flag(pcodeop_flags::call);
    }
    let id = data.new_constant(4, BUILTIN_SWIFT_ERROR as u64);
    data.op_set_input(op, id, 0)?;
    data.new_varnode_out(size, reg, op)?;
    data.op_insert_after(op, call);
    Ok(())
}

/// Read the error register with `swift_throw(e)` right after `write`, the
/// last write of it before a return.
fn throw_error(data: &mut Funcdata, write: OpId, reg: &Address, size: int4) -> KunaResult<()> {
    let pc = data
        .obank()
        .get(write)
        .map(|o| o.get_addr().clone())
        .ok_or_else(|| KunaError::lowlevel("swifterror: stale write"))?;
    let op = data.new_op(2, pc);
    data.op_set_opcode_code(op, OpCode::CPUI_CALLOTHER);
    if let Some(o) = data.obank_mut().get_mut(op) {
        o.clear_flag(pcodeop_flags::call);
    }
    let id = data.new_constant(4, BUILTIN_SWIFT_THROW as u64);
    data.op_set_input(op, id, 0)?;
    let err = data.new_varnode(size, reg, None);
    data.op_set_input(op, err, 1)?;
    data.op_insert_after(op, write);
    Ok(())
}

/// The last whole write of `reg` in the block of the return `ret`, with no
/// call between it and the return.
fn write_before_return(data: &Funcdata, ret: OpId, reg: &Address, size: int4) -> Option<OpId> {
    let mut cur = data.op_previous_op(ret);
    while let Some(id) = cur {
        let op = data.obank().get(id)?;
        if is_call(op.code()) {
            return None;
        }
        if let Some(out) = op.get_out().and_then(|o| data.vbank().get(o)) {
            if overlaps(out.get_addr(), out.get_size(), reg, size) {
                return (out.get_addr() == reg && out.get_size() == size).then_some(id);
            }
        }
        cur = data.op_previous_op(id);
    }
    None
}

/// (kuna) `ActionSwiftError`: name the error a throwing Swift call leaves and
/// the one a throwing function returns (option `swiftcall`).
pub struct ActionSwiftError {
    base: ActionBase,
}

impl ActionSwiftError {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSwiftError {
            base: ActionBase::new(crate::action::ruleflags::rule_onceperfunc, "swifterror", g),
        })
    }
}

impl Action for ActionSwiftError {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionSwiftError {
            base: self.base.clone(),
        }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let Some(abi) = data.get_arch().swift_abi.clone() else {
            return 0;
        };
        let (reg, size) = abi.error_reg.clone();
        let calls: Vec<OpId> = (0..data.num_calls())
            .map(|i| data.get_call_specs(i))
            .filter(|fc| throws(fc.proto().get_model_name()))
            .map(|fc| fc.get_op())
            .collect();
        let writes: Vec<OpId> = if throws(data.get_func_proto().get_model_name()) {
            data.obank()
                .iter_code(OpCode::CPUI_RETURN)
                .filter_map(|ret| write_before_return(data, ret, &reg, size))
                .collect()
        } else {
            Vec::new()
        };
        let mut changed = 0;
        for call in calls {
            if define_error(data, call, &reg, size).is_ok() {
                changed = 1;
            }
        }
        for write in writes {
            if throw_error(data, write, &reg, size).is_ok() {
                changed = 1;
            }
        }
        changed
    }
}
