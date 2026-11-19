//! (kuna `helperroles`) Role tags for helper calls, and the chained rewrites
//! that read them.
//!
//! A helper call does the same thing whether it is a named library call or one
//! of the engine's own builtins, so a rewrite that matches on what the call does
//! should not care which it is. [`role_of`] is that tag: a `CALL`'s callee name,
//! decorations stripped (`_memcpy`, `j_strlen`, `__imp_memcpy`, `memcpy@GLIBC`),
//! or a `CALLOTHER` builtin's id. [`RuleStrcpyChain`] is the first rewrite on
//! it: `memcpy(d, s, strlen(s) + 1)` is `strcpy(d, s)`.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::userop;

/// What a helper call does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperRole {
    Memcpy,
    Memmove,
    Memset,
    Memcmp,
    Strlen,
    Strcpy,
    Strncpy,
    Strcmp,
}

/// The role of a library function by name, decorations stripped.
pub fn role_of_name(name: &str) -> Option<HelperRole> {
    let mut n = name.split('@').next().unwrap_or(name);
    for p in ["j_", "__imp_", "_imp_", "imp_"] {
        n = n.strip_prefix(p).unwrap_or(n);
    }
    let n = n.trim_start_matches('_');
    let n = n.strip_prefix("builtin_").unwrap_or(n);
    Some(match n {
        "memcpy" | "memcpy_chk" => HelperRole::Memcpy,
        "memmove" | "memmove_chk" => HelperRole::Memmove,
        "memset" | "memset_chk" => HelperRole::Memset,
        "memcmp" | "bcmp" => HelperRole::Memcmp,
        "strlen" => HelperRole::Strlen,
        "strcpy" | "strcpy_chk" => HelperRole::Strcpy,
        "strncpy" => HelperRole::Strncpy,
        "strcmp" => HelperRole::Strcmp,
        _ => return None,
    })
}

/// The role of a `CALL` (by its callee's name) or a `CALLOTHER` builtin.
pub fn role_of(data: &Funcdata, op: OpId) -> Option<HelperRole> {
    let o = data.obank().get(op)?;
    match o.code() {
        OpCode::CPUI_CALL => (0..data.num_calls())
            .map(|i| data.get_call_specs(i))
            .find(|fc| fc.get_op() == op)
            .and_then(|fc| role_of_name(fc.get_name())),
        OpCode::CPUI_CALLOTHER => {
            let id = data.vbank().get(o.get_in(0)?).filter(|v| v.is_constant())?.get_offset() as u32;
            Some(match id {
                userop::BUILTIN_MEMCPY => HelperRole::Memcpy,
                userop::BUILTIN_MEMSET => HelperRole::Memset,
                userop::BUILTIN_STRLEN => HelperRole::Strlen,
                userop::BUILTIN_STRNCPY => HelperRole::Strncpy,
                userop::BUILTIN_STRCPY => HelperRole::Strcpy,
                _ => return None,
            })
        }
        _ => None,
    }
}

/// `v` with `COPY`s, zero/sign extensions and the `INDIRECT`s a call leaves on
/// a register it preserves peeled off.
fn peel(data: &Funcdata, mut v: VarnodeId) -> VarnodeId {
    for _ in 0..8 {
        let Some(def) = data.vbank().get(v).and_then(|n| n.get_def()) else { break };
        let Some(o) = data.obank().get(def) else { break };
        match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => match o.get_in(0) {
                Some(i) => v = i,
                None => break,
            },
            _ => break,
        }
    }
    v
}

/// A `strlen` whose length nobody reads is gone: it has no other effect.
fn drop_unread_strlen(op: OpId, data: &mut Funcdata) -> int4 {
    let Some(o) = data.obank().get(op) else { return 0 };
    if o.get_out().is_some_and(|l| !data.descend_snapshot(l).is_empty()) || o.get_out().is_none() {
        return 0;
    }
    if o.code() == OpCode::CPUI_CALL {
        data.delete_call_specs(op);
    }
    data.op_destroy(op);
    1
}

/// `memcpy(d, s, strlen(s) + 1)` (by role, either side a library call or a
/// builtin) becomes `strcpy(d, s)`; the `strlen` goes when nothing else reads
/// it.
pub struct RuleStrcpyChain {
    group: String,
}

impl RuleStrcpyChain {
    pub fn new(group: &str) -> Self {
        Self { group: group.to_string() }
    }
}

impl Rule for RuleStrcpyChain {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CALL, OpCode::CPUI_CALLOTHER]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains(&self.group).then(|| Box::new(Self::new(&self.group)) as Box<dyn Rule>)
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.get_arch().helper_roles {
            return 0;
        }
        let role = role_of(data, op);
        if role == Some(HelperRole::Strlen) {
            return drop_unread_strlen(op, data);
        }
        if role != Some(HelperRole::Memcpy) {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.num_input() < 4 {
            return 0;
        }
        let (Some(d), Some(s), Some(n)) = (o.get_in(1), o.get_in(2), o.get_in(3)) else { return 0 };
        // n == len + 1
        let Some(add) = data.vbank().get(peel(data, n)).and_then(|v| v.get_def()) else { return 0 };
        let Some(a) = data.obank().get(add) else { return 0 };
        if a.code() != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let one = |v: Option<VarnodeId>| v.and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_constant() && v.get_offset() == 1);
        let len = match (a.get_in(0), a.get_in(1)) {
            (Some(x), y) if one(y) => x,
            (x, Some(y)) if one(x) => y,
            _ => return 0,
        };
        let Some(sl) = data.vbank().get(peel(data, len)).and_then(|v| v.get_def()) else { return 0 };
        if role_of(data, sl) != Some(HelperRole::Strlen) {
            return 0;
        }
        let Some(slo) = data.obank().get(sl) else { return 0 };
        let arg = slo.get_in(1);
        if arg.map(|v| peel(data, v)) != Some(peel(data, s)) {
            return 0;
        }
        let strlen_out = slo.get_out();
        let was_call = o.code() == OpCode::CPUI_CALL;
        let id = data.new_constant(4, userop::BUILTIN_STRCPY as uintb);
        if was_call {
            data.delete_call_specs(op);
        }
        if data.op_set_all_input(op, &[id, d, s]).is_err() {
            return 0;
        }
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
        // The strlen is dead once the copy no longer reads it.
        if let Some(l) = strlen_out {
            let dead = |data: &Funcdata, u: OpId| {
                data.obank().get(u).and_then(|p| p.get_out()).is_some_and(|w| data.descend_snapshot(w).is_empty())
            };
            let users = data.descend_snapshot(l);
            if users.iter().all(|&u| dead(data, u)) {
                for u in users {
                    data.op_destroy(u);
                }
                if data.obank().get(sl).is_some_and(|p| p.code() == OpCode::CPUI_CALL) {
                    data.delete_call_specs(sl);
                }
                data.op_destroy(sl);
            }
        }
        1
    }
}
