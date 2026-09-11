//! (kuna) `kuna_tailcallsaved` — a stack teardown that never restores what the
//! entry block saved is argument cleanup, not a frame teardown (S2 flow
//! classification).  Narrows [`kuna_tailcallframe`](crate::kuna_tailcallframe).
//!
//! ## The shape this rejects
//!
//! [`kuna_tailcallframe`](crate::kuna_tailcallframe) reads a direct `jmp` as a
//! tail call when the straight-line run ending at it raises the stack pointer by
//! exactly what the straight-line run leaving the entry address lowered it by.
//! Those two runs are measured where they sit, and on a cdecl caller the second
//! one measures the wrong thing:
//!
//! ```text
//! 004010b0  push ebx                 ; entry run: -4
//! 004010b8  push esi                 ;            -4   (frame = -8)
//!   ...
//! 004010e4  push 0x103               ; call arguments
//! 004010e9  push esi
//! 004010ea  call 0x40167d
//! 004010ef  add  esp,8               ; run ending at the branch: +8
//! 004010f2  jmp  0x401102            ; <- read as a tail call
//! ```
//!
//! The `add esp,8` discards the two arguments the caller pushed for the call on
//! the line above it; `ebx` and `esi` are still on the stack and the stack
//! pointer is still 8 bytes below its entry value when the `jmp` executes.  The
//! backward scan cannot see that, because it stops at the `call` — everything
//! that moved the stack pointer in between is on the far side of its horizon.
//! 0x401102 is an ordinary block of the same function, reached by the other side
//! of the branch at 0x4010e2 as well, so the recovered call truncates the
//! function at a join point and the rest of its body is emitted as a callee.
//!
//! ## The discriminator
//!
//! A tail jump runs with the stack exactly as `ret` would find it, so every
//! callee-saved register the entry block pushed has to have been popped back
//! before the branch — the ABI requires it, and a teardown that leaves them on
//! the stack is not a teardown.  Argument cleanup restores nothing: `add esp,8`
//! moves the stack pointer and loads no register.
//!
//! So each scan is asked for a second number beside its delta: how many bytes it
//! moved *through* the stack pointer.  An instruction that lowers the stack
//! pointer and stores through it is saving (`push`); one that raises the stack
//! pointer and loads through it is restoring (`pop`).  The transfer is capped at
//! the instruction's own stack-pointer motion, so a multi-register `push`/`pop`
//! can never account for more than it moved, and a plain `mov [esp],eax` — which
//! moves the stack pointer not at all — accounts for nothing.
//!
//! The rule is then `restored >= saved`: whatever the entry block put on the
//! stack, the run ending at the branch has taken back off it.  A frame built
//! purely by `sub rsp,N` saves nothing and so demands nothing, which keeps the
//! `sub rsp,8; call f; add rsp,8; jmp g` alignment shape gcc emits at `-O2`.
//! The under-count on an architecture whose multi-register push writes only its
//! first word through the stack pointer is symmetric — its pop reads only the
//! first word back — so the comparison survives it.

use kuna_base::address::Address;
use kuna_base::space::VarnodeStorage;
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;

/// Bytes the instruction at `at` pushed onto the stack, given that its net
/// stack-pointer motion is `delta`.  Zero unless the instruction lowers the
/// stack pointer *and* stores through it.
pub fn kuna_stack_saved_bytes(
    data: &Funcdata,
    at: &Address,
    sp: &VarnodeStorage,
    delta: i64,
) -> u64 {
    if delta >= 0 {
        return 0;
    }
    transfer_bytes(data, at, sp, OpCode::CPUI_STORE).min(delta.unsigned_abs())
}

/// Bytes the instruction at `at` took back off the stack, given that its net
/// stack-pointer motion is `delta`.  Zero unless the instruction raises the
/// stack pointer *and* loads through it.
pub fn kuna_stack_restored_bytes(
    data: &Funcdata,
    at: &Address,
    sp: &VarnodeStorage,
    delta: i64,
) -> u64 {
    if delta <= 0 {
        return 0;
    }
    transfer_bytes(data, at, sp, OpCode::CPUI_LOAD).min(delta as u64)
}

/// (kuna) Does the run ending at the branch give back everything the entry run
/// put on the stack?  With the gate off this is the pre-`tailcallsaved`
/// behaviour: the deltas alone decide.
pub fn kuna_teardown_restores_saves(gate: bool, saved: u64, restored: u64) -> bool {
    !gate || restored >= saved
}

/// Total width of the `LOAD`s or `STORE`s at `at` whose pointer operand is
/// exactly the stack-pointer register — the accesses a `push` or a `pop` makes.
fn transfer_bytes(data: &Funcdata, at: &Address, sp: &VarnodeStorage, want: OpCode) -> u64 {
    let mut bytes: u64 = 0;
    for (_, id) in data.obank().iter_at(at) {
        let Some(o) = data.obank().get(id) else { continue };
        if o.code() != want {
            continue;
        }
        if !is_stack_pointer(data, o.get_in(1), sp) {
            continue;
        }
        let width = match want {
            OpCode::CPUI_STORE => o.get_in(2).and_then(|v| data.vbank().get(v)).map(|v| v.get_size()),
            _ => o.get_out().and_then(|v| data.vbank().get(v)).map(|v| v.get_size()),
        };
        bytes = bytes.saturating_add(width.unwrap_or(0).max(0) as u64);
    }
    bytes
}

/// Is `vn` exactly the stack-pointer register location?
fn is_stack_pointer(
    data: &Funcdata,
    vn: Option<crate::context::VarnodeId>,
    sp: &VarnodeStorage,
) -> bool {
    let Some(id) = vn else { return false };
    let Some(v) = data.vbank().get(id) else { return false };
    let Some(spspace) = sp.space.as_ref() else { return false };
    std::rc::Rc::ptr_eq(v.get_space(), spspace)
        && v.get_offset() == sp.offset
        && v.get_size() as u32 == sp.size
}

#[cfg(test)]
#[path = "kuna_tailcallsaved/tests.rs"]
mod tests;
