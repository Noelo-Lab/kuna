//! (kuna) A recovered parameter whose value only ever reaches the variadic tail
//! of a call is not a parameter a caller may be handed.
//!
//! # The symptom
//!
//! openssh `xcalloc(size_t nmemb, size_t size)` is recovered with a third
//! parameter, with `passthrough` off as much as on:
//!
//! ```text
//! unsigned long sub_42ff0(uint8 a0,uint8 a1,unsigned long a2) {
//!   if ((!a1) || (!a0))
//!     sub_3aa40("xmalloc.c","xcalloc",0x34,0,1,0,"xcalloc: zero size",a2);
//!   ...
//! ```
//!
//! and the machine code says why: the call to the variadic `sshfatal` is set up
//! with `push %rdx; ...; push %rax; xor %eax,%eax`, where the `push %rdx` is
//! gcc's stack-alignment filler and `rdx` holds nothing at all. Recovery cannot
//! tell that push from an argument, so `rdx` becomes a parameter, and
//! [`crate::p4_calls::kuna_passthrough`] would then hand it to every caller that
//! forwards the register — `tohex`, `pwcopy` and `init_hostkeys` all gain a third
//! parameter their DWARF prototype does not have. gnulib
//! `open_safer(char const *file, int flags, ...)` is the same shape with the
//! variadic declared: it reads `rdx` only to pass it to `open`'s vararg slot, so
//! `savewd_save`, which calls it with two arguments, gains a third parameter too.
//!
//! # The rule
//!
//! A recovered parameter is *vararg-tail* when every use of the function's own
//! incoming value for it, through value-preserving operations, is an argument at
//! a call that is variadic: one the function set up as variadic (SysV's `xor
//! %eax,%eax` vector-register count, recorded by
//! [`crate::p4_calls::kuna_passthrough`] before heritage) or a declared `...`
//! prototype at a slot past its named parameters. Such a parameter is recorded
//! with the callee's statement and `passthrough` never claims it: the register is
//! read by the callee, but only where the ABI says the caller need not have put
//! anything in it.
//!
//! The parameter itself is left alone. What the callee does with its own
//! registers is its recovery's business; this only stops the claim from
//! travelling to callers, which is the direction that fabricates.

use std::collections::HashSet;

use kuna_base::address::Address;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How many Varnodes the forward walk visits before giving up. Exhaustion
/// answers "not vararg-tail", the answer that leaves the claim as it was.
const MAX_NODES: u32 = 256;

/// The storage of every recovered parameter in `storage` whose value only feeds
/// a variadic tail ([`only_feeds_a_vararg_tail`]).
pub fn vararg_tail_inputs(data: &Funcdata, storage: &[(Address, int4)]) -> Vec<Address> {
    storage
        .iter()
        .filter(|(addr, size)| only_feeds_a_vararg_tail(data, addr, *size))
        .map(|(addr, _)| addr.clone())
        .collect()
}

/// Does the function's own incoming value for `[addr, addr+size)` reach nothing
/// but variadic-tail call arguments?
///
/// `false` for a parameter with no input Varnode, one with no use at all, one
/// the walk cannot follow, and one that reaches any other kind of use — every
/// shape but the one this module names keeps today's answer.
pub fn only_feeds_a_vararg_tail(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(vn) = data.find_varnode_input(size, addr) else { return false };
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut work: Vec<VarnodeId> = vec![vn];
    let mut budget = MAX_NODES;
    let mut tail_use = false;
    while let Some(cur) = work.pop() {
        if budget == 0 {
            return false;
        }
        budget -= 1;
        if !seen.insert(cur) {
            continue;
        }
        let uses = data.descend_snapshot(cur);
        if uses.is_empty() && cur == vn {
            return false;
        }
        for op in uses {
            let Some(o) = data.obank().get(op) else { return false };
            if o.is_dead() {
                continue;
            }
            match o.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    let slot = (0..o.num_input()).find(|&i| o.get_in(i) == Some(cur));
                    match slot {
                        Some(s) if is_vararg_slot(data, op, s) => tail_use = true,
                        _ => return false,
                    }
                }
                code if moves_the_value(data, op, cur, code) => {
                    let Some(out) = o.get_out() else { return false };
                    work.push(out);
                }
                _ => return false,
            }
        }
    }
    tail_use
}

/// Does `op` hand `vn`'s value on unchanged enough that a use of its output is a
/// use of the parameter -- a copy, a phi, a width change, or a mask by a
/// constant (a `mode_t` narrowed out of its register)?
fn moves_the_value(data: &Funcdata, op: OpId, vn: VarnodeId, code: OpCode) -> bool {
    match code {
        OpCode::CPUI_COPY
        | OpCode::CPUI_CAST
        | OpCode::CPUI_INDIRECT
        | OpCode::CPUI_MULTIEQUAL
        | OpCode::CPUI_PIECE
        | OpCode::CPUI_SUBPIECE
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT => true,
        OpCode::CPUI_INT_AND => data
            .obank()
            .get(op)
            .map(|o| {
                (0..o.num_input())
                    .filter_map(|i| o.get_in(i))
                    .filter(|&v| v != vn)
                    .all(|v| data.vbank().get(v).map(|v| v.is_constant()).unwrap_or(false))
            })
            .unwrap_or(false),
        _ => false,
    }
}

/// Is input `slot` of the call `op` a variadic-tail argument -- one the ABI lets
/// a caller leave unset?
///
/// Either the function set the call up as variadic (the recorded SysV
/// vector-register count, [`crate::p4_calls::kuna_passthrough`]), or the callee's
/// prototype is declared `...` and the slot is past its named parameters.
fn is_vararg_slot(data: &Funcdata, op: OpId, slot: int4) -> bool {
    if slot < 1 {
        return false;
    }
    if data.kuna_passthrough_vararg_calls().contains(&op) {
        return true;
    }
    let Some(idx) = data.get_call_specs_index(op) else { return false };
    let fc = data.get_call_specs(idx);
    fc.is_dotdotdot() && slot - 1 >= fc.proto().num_params()
}

#[cfg(test)]
#[path = "kuna_varargtail/tests.rs"]
mod tests;
