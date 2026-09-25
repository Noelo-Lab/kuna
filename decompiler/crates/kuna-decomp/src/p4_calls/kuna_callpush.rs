//! (kuna `callpush`) A call's own return-address push is part of the call.
//!
//! An x86 `call` lifts as three p-code ops at one instruction address: the stack
//! pointer steps down one word, the fall-through address is STOREd at the new
//! stack pointer, and the CALL transfers.  In a frame whose stack pointer stays
//! a constant offset from its entry value, `RuleStoreVarnode` turns that STORE
//! into a COPY to a stack slot nothing reads, and dead-code elimination removes
//! it.  After an `alloca` (`sub rsp, rax`) the stack pointer is the entry value
//! minus a run-time size, the STORE keeps its pointer, and every call in the
//! rest of the function prints its push as a statement of its own:
//!
//! ```text
//! *(unsigned long *)&v28[-8] = 0xbc79;
//! v14 = fstatat(v39,v37,&v21,0x100);
//! ```
//!
//! The stored word is read by exactly one instruction, the callee's `ret`, and
//! the C call already performs that transfer, so the statement says nothing the
//! call does not.  [`RuleCallPush`] deletes such a STORE when all of these hold:
//!
//! * the stored value is a constant, one pointer word wide, that lies within
//!   15 bytes (the longest x86 instruction) after the instruction's own address;
//! * a CALL or CALLIND follows it in the same basic block at the same
//!   instruction address, so both ops came from one `call` instruction;
//! * that call's destination is not the stored address: `call $+5; pop` reads
//!   the pushed word back, and it is the one idiom whose caller does;
//! * the pointer is the stack pointer register as that same instruction wrote
//!   it (through COPY and CAST), so the store is the push and not a store the
//!   instruction's operands asked for;
//! * `RuleLoadVarnode::check_spacebase` cannot place the pointer on the stack.
//!   The rule is registered after `RuleStoreVarnode` in the same pool, so a
//!   push in a tracked frame is that rule's COPY before this one sees it.
//!
//! A callee that pops its return address as data (`call over data; pop`) is
//! lifted as a BRANCH by `callpopret`, so no CALL is left for this rule to
//! match.  Destroying the STORE leaves its INDIRECT guards pointing at a dead
//! op, which `RuleIndirectCollapse` folds on the next pass.
//!
//! Stores of stack-passed arguments through the same pointer, and a stack
//! probe's `*p = *p` touch, are not the call's push and are left alone.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::ruleaction_4::RuleLoadVarnode;

/// The longest x86 instruction, which bounds how far past a `call`'s own
/// address its fall-through address can be.
const MAX_INSN_LEN: u64 = 15;

/// Delete a call's own return-address push (see the module docs).
pub struct RuleCallPush {
    enabled: bool,
    group: String,
}

impl RuleCallPush {
    /// Build the rule for `group`; `enabled` forces it on independently of the
    /// architecture flag (used by the unit tests).
    pub fn with_group(enabled: bool, group: &str) -> RuleCallPush {
        RuleCallPush { enabled, group: group.to_string() }
    }
}

impl Rule for RuleCallPush {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleCallPush { enabled: self.enabled, group: self.group.clone() }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !data.get_arch().drop_call_push {
            return 0;
        }
        if !is_call_push(data, op) {
            return 0;
        }
        data.op_destroy(op);
        1
    }
}

/// Is the STORE `op` the push a `call` instruction makes of its own return
/// address, through a stack pointer the frame could not track?
pub fn is_call_push(data: &Funcdata, op: OpId) -> bool {
    let (addr, ptr, val) = {
        let Some(o) = data.obank().get(op) else { return false };
        if o.code() != OpCode::CPUI_STORE {
            return false;
        }
        match (o.get_in(1), o.get_in(2)) {
            (Some(p), Some(v)) => (o.get_addr().clone(), p, v),
            _ => return false,
        }
    };
    let Some(v) = data.vbank().get(val) else { return false };
    if !v.is_constant() {
        return false;
    }
    let ret = v.get_offset();
    let word = addr.get_space().map(|s| s.get_addr_size() as int4).unwrap_or(0);
    if word == 0 || v.get_size() != word {
        return false;
    }
    let here = addr.get_offset();
    if ret <= here || ret - here > MAX_INSN_LEN {
        return false;
    }
    let Some(call) = same_insn_call(data, op) else { return false };
    if call_target(data, call) == Some(ret) {
        return false;
    }
    if !is_pushed_sp(data, ptr, &addr) {
        return false;
    }
    RuleLoadVarnode::check_spacebase(data, op).is_none()
}

/// The CALL/CALLIND that follows `op` in its block at the same instruction
/// address, if any.
fn same_insn_call(data: &Funcdata, op: OpId) -> Option<OpId> {
    let addr = data.obank().get(op)?.get_addr().clone();
    let mut cur = data.obank().get(op)?.basic_neighbours().1;
    while let Some(n) = cur {
        let o = data.obank().get(n)?;
        if o.get_addr() != &addr {
            return None;
        }
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            return Some(n);
        }
        cur = o.basic_neighbours().1;
    }
    None
}

/// The direct destination offset of a CALL, or `None` for a CALLIND.
fn call_target(data: &Funcdata, call: OpId) -> Option<u64> {
    let o = data.obank().get(call)?;
    if o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let dest = data.vbank().get(o.get_in(0)?)?;
    Some(dest.get_offset())
}

/// Is `ptr` (through COPY and CAST) the stack pointer register as written by
/// an op of the instruction at `addr`?
fn is_pushed_sp(data: &Funcdata, ptr: VarnodeId, addr: &kuna_base::address::Address) -> bool {
    let Some(stackspc) = data.get_arch().manage().get_stack_space() else { return false };
    let Ok(sp) = stackspc.get_spacebase(0) else { return false };
    let Some(sp_space) = sp.space else { return false };
    let mut vn = ptr;
    for _ in 0..8 {
        let Some(v) = data.vbank().get(vn) else { return false };
        if !v.is_written() {
            return false;
        }
        let Some(def) = v.get_def() else { return false };
        let Some(d) = data.obank().get(def) else { return false };
        let is_sp = v.get_space().get_index() == sp_space.get_index()
            && v.get_offset() == sp.offset
            && v.get_size() as u32 == sp.size as u32;
        if is_sp {
            return d.get_addr() == addr;
        }
        match d.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => match d.get_in(0) {
                Some(src) => vn = src,
                None => return false,
            },
            _ => return false,
        }
    }
    false
}
