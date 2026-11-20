//! (kuna) A Swift string object is not a pointer to the bytes it lands on.
//!
//! swiftc passes a large string literal (16 UTF-8 bytes or more; a shorter one is
//! a small string packed into the words themselves) as two words: the count with
//! its flags, and an object word holding the literal's address minus the 32-byte
//! native bias, tagged in bit 63:
//!
//! ```text
//!   adrp x10, lit@PAGE ; add x10, x10, lit@PAGEOFF ; sub x10, x10, #0x20
//!   ...                                            ; csel x8, x10, x11, ne
//!   orr  x1, x8, #0x8000000000000000
//! ```
//!
//! Constant folding leaves the untagged half as a constant, `lit - 0x20`, and
//! when the tag is ORed in after a join that constant reaches `ActionConstantPtr`
//! on its own. Large literals are 16-byte aligned, so `lit - 0x20` is the start
//! of, or a point inside, the literal stored before `lit`, and both character
//! array arms of the action print that other literal's text.
//!
//! [`is_string_object`] recognizes the constant so the action leaves it as a
//! number: every use, followed through COPY, MULTIEQUAL and INDIRECT, ORs in the
//! tag, and the bytes the bias skips over open a large literal. A literal address
//! biased after the join has an INT_ADD on that path and keeps its literal.

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// `_StringObject.nativeBias` on 64-bit targets: the object word holds the
/// literal's address minus this.
pub const NATIVE_BIAS: u64 = 0x20;

/// The immortal large-string discriminator ORed into the object word.
pub const IMMORTAL_TAG: u64 = 1 << 63;

/// The shortest literal stored out of line; a shorter one is a small string.
pub const MIN_LARGE_LITERAL: usize = 16;

/// How many def-use hops the walk to the tag follows before giving up.
const HOP_CAP: usize = 8;

/// Is the constant `vn`, resolved to `addr`, the untagged object word of a
/// large Swift string literal?
pub fn is_string_object(data: &Funcdata, vn: VarnodeId, addr: &Address) -> bool {
    tagged_by_every_use(data, vn) && opens_large_literal(data, addr)
}

/// Breadth-first over the copies of `vn`: true when at least one use ORs in
/// [`IMMORTAL_TAG`] and every other use is a COPY, MULTIEQUAL or INDIRECT.
fn tagged_by_every_use(data: &Funcdata, vn: VarnodeId) -> bool {
    if data.vbank().get(vn).map(|v| v.get_size()) != Some(8) {
        return false;
    }
    let first = data.lone_descend(vn).and_then(|op| data.obank().get(op)).map(|o| o.code());
    if !matches!(
        first,
        Some(OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
    ) {
        return false;
    }
    let mut seen: HashSet<VarnodeId> = HashSet::from([vn]);
    let mut work: VecDeque<(VarnodeId, usize)> = VecDeque::from([(vn, 0)]);
    let mut tagged = false;
    while let Some((cur, hops)) = work.pop_front() {
        let Some(v) = data.vbank().get(cur) else { return false };
        for op in v.descend_iter() {
            let Some(o) = data.obank().get(op) else { return false };
            let slot = o.get_slot(cur);
            match o.code() {
                OpCode::CPUI_INT_OR => {
                    let tag = o
                        .get_in(1 - slot)
                        .and_then(|t| data.vbank().get(t))
                        .is_some_and(|t| t.is_constant() && t.get_offset() == IMMORTAL_TAG);
                    if !tag {
                        return false;
                    }
                    tagged = true;
                }
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
                    if o.code() == OpCode::CPUI_INDIRECT && slot != 0 {
                        return false;
                    }
                    let Some(out) = o.get_out() else { return false };
                    if seen.insert(out) {
                        if hops + 1 >= HOP_CAP {
                            return false;
                        }
                        work.push_back((out, hops + 1));
                    }
                }
                _ => return false,
            }
        }
    }
    tagged
}

/// Do the read-only bytes [`NATIVE_BIAS`] past `addr` open a large literal?
fn opens_large_literal(data: &Funcdata, addr: &Address) -> bool {
    let Some(space) = addr.get_space() else { return false };
    if space.get_word_size() != 1 {
        return false;
    }
    let lit = Address::new(
        Rc::clone(space),
        space.wrap_offset(addr.get_offset().wrapping_add(NATIVE_BIAS)),
    );
    let arch = data.get_arch();
    if arch.query_global_properties(&lit, 1, &Address::new_invalid()) & varnode_flags::readonly == 0 {
        return false;
    }
    let mut head = [0u8; MIN_LARGE_LITERAL];
    arch.loader_fill(&mut head, &lit).is_ok() && is_large_literal_head(&head)
}

/// Can `head`, the first [`MIN_LARGE_LITERAL`] bytes at an address, open a
/// large literal: no NUL, and UTF-8 up to any character the window cuts off?
pub fn is_large_literal_head(head: &[u8]) -> bool {
    head.len() >= MIN_LARGE_LITERAL
        && !head.contains(&0)
        && std::str::from_utf8(head).map_or_else(|e| e.error_len().is_none(), |_| true)
}

#[cfg(test)]
#[path = "kuna_swiftstrobj/tests.rs"]
mod tests;
