//! (kuna `chkstkdarwin`) Remove the Darwin AArch64 stack probe before SSA, so
//! the registers it preserves keep their values across it.
//!
//! Apple clang calls `___chkstk_darwin`, with the byte count in `x9`, before it
//! allocates a frame larger than a page or a variable-length array. The probe
//! touches each page below the stack pointer and returns. It writes no register
//! but `x9`, the `x16`/`x17` call scratch and the flags, and compiled code relies
//! on that: `int big_frame(int k)` keeps `k` in `w0` across the call. The
//! AArch64 prototype model kills `x0`, `x1` and `x8`-`x18` at every call, so an
//! argument register read after the probe became its result or a local nothing
//! assigns, and the function lost its parameters (`int _big_frame(void)`,
//! `v2 = __chkstk_darwin();`).
//!
//! The call is matched on the raw p-code before heritage and removed. A direct
//! `CALL` (through a `__stubs` entry) matches by the callee's name. A `CALLIND`
//! matches when its target is loaded, earlier in the same block, from a
//! constant address that holds a function symbol of that name: the
//! `adrp x16; ldr x16,[x16,#off]; blr x16` GOT call clang emits. A call-fixup
//! cannot reach that call, because it only becomes direct after
//! `ActionDeindirect` and a restart. Calls that carry an inject, inline or
//! no-return fact are left alone, and so is a jump to the probe that the flow
//! turned into a call (a `__stubs` entry's own `br x16`): no code follows it.
//! The pass serves AArch64 only: on x86-64 the probe takes its size in `RAX`,
//! and removing the `CALL` would leave its pushed return address behind.

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::OpId;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::kuna_x64syscall::overlaps;

/// The probe as the Mach-O loader names the import (one leading underscore
/// stripped) and as an image that defines it names it.
const PROBE_NAMES: [&str; 2] = ["__chkstk_darwin", "___chkstk_darwin"];

/// How many raw ops the walk from the call back to its target may visit.
const MAX_OPS: u32 = 32;

/// Whether `archid` names an AArch64 language, the only one this pass serves.
pub fn applies_to(archid: &str) -> bool {
    archid.split(':').next() == Some("AARCH64")
}

/// Whether `name` is the probe's symbol.
pub fn is_probe(name: &str) -> bool {
    PROBE_NAMES.contains(&name)
}

enum Value {
    Constant(u64),
    Loaded(Address),
}

/// The value the raw p-code before `at`, in its block, leaves in `addr`/`size`:
/// a constant, or the word loaded from a constant address.
fn value_before(
    data: &Funcdata,
    at: OpId,
    addr: &Address,
    size: int4,
    budget: &mut u32,
) -> Option<Value> {
    let mut cur = at;
    loop {
        *budget = budget.checked_sub(1)?;
        cur = data.op_previous_op(cur)?;
        let op = data.obank().get(cur)?;
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            return None;
        }
        let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        if !overlaps(out.get_addr(), out.get_size(), addr, size) {
            continue;
        }
        if out.get_addr() != addr || out.get_size() != size {
            return None;
        }
        return match op.code() {
            OpCode::CPUI_COPY => input_value(data, cur, 0, budget),
            OpCode::CPUI_INT_ADD => match (
                input_value(data, cur, 0, budget)?,
                input_value(data, cur, 1, budget)?,
            ) {
                (Value::Constant(a), Value::Constant(b)) => Some(Value::Constant(a.wrapping_add(b))),
                _ => None,
            },
            OpCode::CPUI_LOAD => {
                let Value::Constant(pointer) = input_value(data, cur, 1, budget)? else {
                    return None;
                };
                let index = data.vbank().get(op.get_in(0)?)?.get_offset();
                let manage = data.get_arch().manage();
                if index >= manage.num_spaces() as u64 {
                    return None;
                }
                let space = std::rc::Rc::clone(manage.get_space(index as int4)?);
                let offset = space.wrap_offset(pointer);
                Some(Value::Loaded(Address::new(space, offset)))
            }
            _ => None,
        };
    }
}

fn input_value(data: &Funcdata, op: OpId, slot: int4, budget: &mut u32) -> Option<Value> {
    let vn = data.vbank().get(data.obank().get(op)?.get_in(slot)?)?;
    if vn.is_constant() {
        return Some(Value::Constant(vn.get_offset()));
    }
    let addr = vn.get_addr().clone();
    value_before(data, op, &addr, vn.get_size(), budget)
}

/// Is `call` a jump the flow turned into a call, followed by the return it
/// planted at the same address?
fn is_tail_call(data: &Funcdata, call: OpId) -> bool {
    let Some(op) = data.obank().get(call) else {
        return false;
    };
    op.basic_neighbours()
        .1
        .and_then(|next| data.obank().get(next))
        .is_some_and(|next| next.code() == OpCode::CPUI_RETURN && next.get_addr() == op.get_addr())
}

/// Is `fc` a call to the probe, with code after it, that nothing else has
/// claimed?
fn calls_probe(data: &Funcdata, fc: &FuncCallSpecs) -> bool {
    let proto = fc.proto();
    if proto.is_inline() || proto.is_no_return() || proto.get_inject_id() >= 0 {
        return false;
    }
    let call = fc.get_op();
    if is_tail_call(data, call) {
        return false;
    }
    match data.obank().get(call).map(|op| op.code()) {
        Some(OpCode::CPUI_CALL) => is_probe(fc.get_name()),
        Some(OpCode::CPUI_CALLIND) => {
            let mut budget = MAX_OPS;
            let Some(Value::Loaded(slot)) = input_value(data, call, 0, &mut budget) else {
                return false;
            };
            data.get_arch()
                .query_function(&slot)
                .is_some_and(|(name, _, _)| is_probe(&name))
        }
        _ => false,
    }
}

/// Remove every call to `___chkstk_darwin` from the raw p-code (`option
/// chkstkdarwin`, default on).
pub struct ActionChkstkDarwin {
    base: ActionBase,
}

impl ActionChkstkDarwin {
    pub fn boxed(group: impl Into<String>) -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(
                crate::action::ruleflags::rule_onceperfunc,
                "chkstkdarwin",
                group,
            ),
        })
    }
}

impl Action for ActionChkstkDarwin {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().chkstk_darwin || data.get_heritage_pass() != 0 || data.num_calls() == 0 {
            return 0;
        }
        let probes: Vec<OpId> = data
            .call_specs()
            .iter()
            .filter(|fc| calls_probe(data, fc))
            .map(|fc| fc.get_op())
            .collect();
        for call in probes {
            data.delete_call_specs(call);
            data.op_destroy(call);
            self.base.count += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests;
