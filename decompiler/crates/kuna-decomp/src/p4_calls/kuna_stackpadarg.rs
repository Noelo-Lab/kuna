//! (kuna) The upper bytes of a stack argument slot that only pad an `int`.
//!
//! clang -O2 aligns the stack for a call with `push %rax` and then stores a
//! seventh `int` argument over the low half of that slot: `push %rax; movl
//! $7,(%rsp); ...; call in7`. The 8-byte stack trial pieces the stored `int`
//! with the upper half of the caller's incoming RAX, and the call printed
//! `in7(1,0,0,0,0,6,(unsigned long)v1 & 0xffffffff00000000 | 7)` with `v1`
//! never set. A wrapper that pushes its own declared `int` register as the
//! slot (`push %r8` for `int ns` in r8d) pads it with the register's upper half
//! the same way.
//!
//! Bytes the caller cannot have been handed as a parameter hold no value the
//! source could have passed, so the argument is the low `int` (or the low
//! `char`/`short` when the padding starts inside it).
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_varargforward::undeclared_bytes;

/// Narrow stack trial `i` of call spec `idx` when it fills exactly one stack
/// slot and every byte above its low int comes from incoming register bytes the
/// caller cannot take as a parameter. It keeps the low 1, 2 or int-width bytes
/// up to the first such byte; a `char` clang stores with `movl %edi` pads bytes
/// 1-3 with the upper part of its own declared `dil` too.
pub fn narrow_pad_upper(data: &mut Funcdata, idx: int4, i: int4) {
    let arch = data.get_arch();
    let Some(width) = arch.types().map(|t| t.get_size_of_int()) else { return };
    let fc = data.get_call_specs(idx);
    if !fc.proto().has_model() {
        return;
    }
    let trial = fc.active_input().get_trial(i);
    let (addr, size, slot) = (trial.get_address().clone(), trial.get_size(), trial.get_slot());
    if size <= width
        || addr.get_space().is_none_or(|s| s.get_type() != spacetype::IPTR_SPACEBASE)
        || !fc.proto().model().input().get_entry().iter().any(|e| {
            e.get_align() == size && e.justified_contain(&addr, size) >= 0
        })
    {
        return;
    }
    let Some(vn) = data.obank().get(fc.get_op()).and_then(|o| o.get_in(slot)) else { return };
    if !undeclared_bytes(data, vn, width, size - width, &mut 64) {
        return;
    }
    let keep = (0..width).find(|&b| undeclared_bytes(data, vn, b, 1, &mut 64)).unwrap_or(width);
    if !(keep == width || (keep > 0 && (keep & (keep - 1)) == 0))
        || (keep < width && !undeclared_bytes(data, vn, keep, width - keep, &mut 64))
    {
        return;
    }
    let low = if addr.is_big_endian() { &addr + (size - keep) as i64 } else { addr.clone() };
    if fc.active_input().test_shrink(i, &low, keep) {
        data.get_call_specs_mut(idx).get_active_input().shrink(i, low, keep);
    }
}
