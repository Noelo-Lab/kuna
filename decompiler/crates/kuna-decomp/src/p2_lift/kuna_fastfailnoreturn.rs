//! **fastfailnoreturn** — a Windows `int 0x29` (`__fastfail`) ends the flow,
//! instead of returning eight bytes higher up the stack than it started.
//!
//! ```text
//!   (*(void *)swi(0x29))(5);                         (*(void *)swi(0x29))(5);
//!   v62 = &v61[8];                             -->   /* flow stops here */
//!   ...
//!   *(unsigned long long *)&v66[-8] = 0x140003e16;
//!   v12 = BCryptGenerateSymmetricKey(*(unsigned long long *)&v66[0x50],...);
//!                                              -->   v15 = BCryptGenerateSymmetricKey(v52,&v55,v62,v54);
//! ```
//!
//! # The defect
//!
//! x86 SLEIGH lowers `INT imm8` to a black-box userop feeding an indirect call
//! (`ia.sinc:3671`):
//!
//! ```text
//!   tmp:1 = imm8;
//!   intloc:$(SIZE) = swi(tmp);
//!   call [intloc];
//! ```
//!
//! That `call` has **no matching push** — every other x86 `CALL` lifts as
//! `RSP = RSP - 8; push &next; call target` — but it is an ordinary modelled call
//! as far as the rest of the pipeline is concerned, so `x86-64-win.cspec`'s
//! default `__fastcall` (`extrapop="8" stackshift="8"`) hands eight bytes back
//! after it. Every `int` in the function therefore raises `RSP` by 8, and the
//! printer says so: `(*(void *)swi(0x29))(5);` is followed by `v62 = &v61[8];`.
//!
//! The damage is not local. Once two paths join carrying stack-pointer values
//! that differ by 8, the frame is no longer a constant offset from the spacebase:
//! stack locals stop being locals and degenerate into offsets off a `char *`, and
//! each `CALL`'s return-address push — normally a dead store into a slot nothing
//! maps — survives as an explicit `*(unsigned long long *)&v66[-8] = 0x140003e16;`.
//! Outgoing stack arguments go the same way, which is why a Win32 call renders
//! with stack blobs for arguments instead of values.
//!
//! # The fix
//!
//! On Windows, `int 0x29` is `__fastfail`: the MSVC `/GS` and STL `_STL_VERIFY`
//! failure path, which terminates the process and by contract never returns. When
//! `option fastfailnoreturn on` (default), the call spec built for that `CALLIND`
//! is marked no-return and `flow.rs` plants the `artificialHalt(noreturn)` it
//! already plants for a no-return callee, so the block ends at the interrupt. The
//! unbalanced stack pointer then reaches no join and the frame stays a frame.
//!
//! This module owns only the *decision* ([`is_fastfail_callind`]); the plant is
//! driven by `flow.rs::setup_callind_specs`, the same seam
//! [`check_for_flow_modification`](crate::flow) uses for a named no-return callee.
//!
//! # What it will not do
//!
//! * **Only `int 0x29`.** `INT1`/`INT3`/`INTO` carry a `return [0:1]` in their own
//!   SLEIGH semantics and genuinely do return; `int 0x80` is a Linux syscall and
//!   is [`linuxsyscall`](crate::kuna_linuxsyscall)'s. The vector must be the
//!   1-byte constant `0x29`.
//! * **Only a Windows image.** `int 0x29` is `__fastfail` by Windows convention
//!   alone. The gate is the compiler-spec component of the resolved language id
//!   ([`archid_is_windows`]), so an `x86:LE:64:default:gcc` image is untouched.
//! * **Only the exact lifted shape.** The `CALLIND` must read the storage the
//!   `swi` `CALLOTHER` in the same instruction wrote. A hand-written `call`
//!   through a pointer that happens to sit next to an interrupt is not a match.
//!
//! No `"Subroutine does not return"` warning is buffered. That warning exists to
//! flag a *surprising* no-return callee; `int 0x29` is no-return by definition,
//! and the witness this closes contains eleven of them in one function whose
//! whole complaint is noise.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::marshal::ElementId;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

/// Marshaling element `<fastfailnoreturn>` (kuna).  ElementIds live in the 4000+
/// range; 4143 is the last one taken, so this is 4144.
pub const ELEM_FASTFAILNORETURN: ElementId = ElementId::new("fastfailnoreturn", 4144);

/// The Windows `__fastfail` interrupt vector (`int 0x29`).
pub const FASTFAIL_VECTOR: uintb = 0x29;

/// The `INT` userop x86 SLEIGH routes the vector through (`define pcodeop swi`,
/// `ia.sinc:779`).
pub const SWI_USEROP: &str = "swi";

/// Is `archid` — a resolved five-part SLEIGH language id such as
/// `x86:LE:64:default:windows` — a Windows image?
///
/// The last `:`-component is the compiler-spec id; the Windows ones x86 ships are
/// `windows` (MSVC) and `clangwindows` (clang-cl), both of which emit `__fastfail`.
/// Borland's two predate it and are not accepted.
pub fn archid_is_windows(archid: &str) -> bool {
    matches!(archid.rsplit(':').next(), Some("windows") | Some("clangwindows"))
}

/// Is `op` the `CALLIND` half of a Windows `int 0x29` (`__fastfail`)?
///
/// `userop_name` resolves a user-op id (the C++ `glb->userops.getOp(id)->getName()`,
/// `None` for a null `UserPcodeOp *`).  Runs at flow-classification time, before
/// basic blocks exist, so the `CALLOTHER` is located by the same positional
/// backward walk over the raw dead list that
/// [`kuna_sparcstructret`](crate::kuna_sparcstructret) uses — pre-SSA there is no
/// def-use edge to follow, the `CALLIND`'s input being a free Varnode.
pub fn is_fastfail_callind<F>(data: &Funcdata, op: OpId, userop_name: F) -> bool
where
    F: Fn(u32) -> Option<String>,
{
    let Some(opref) = data.obank().get(op) else { return false };
    if opref.code() != OpCode::CPUI_CALLIND {
        return false;
    }
    let insn = opref.get_addr().clone();
    let Some(target) = opref.get_in(0).and_then(|v| data.vbank().get(v)) else { return false };
    let (target_addr, target_size) = (target.get_addr().clone(), target.get_size());

    let deadlist: Vec<OpId> = data.obank().iter_dead().collect();
    let Some(mut pos) = deadlist.iter().position(|&o| o == op) else { return false };

    loop {
        let cur = deadlist[pos];
        let Some(curref) = data.obank().get(cur) else { return false };
        if curref.code() == OpCode::CPUI_CALLOTHER
            && curref.num_input() == 2
            && curref.get_addr() == &insn
        {
            let vector_ok = curref
                .get_in(1)
                .and_then(|v| data.vbank().get(v))
                .map(|v| {
                    v.is_constant() && v.get_size() == 1 && v.get_offset() == FASTFAIL_VECTOR
                })
                .unwrap_or(false);
            let feeds_call = curref
                .get_out()
                .and_then(|v| data.vbank().get(v))
                .map(|v| v.get_addr() == &target_addr && v.get_size() == target_size)
                .unwrap_or(false);
            let is_swi = curref
                .get_in(0)
                .and_then(|v| data.vbank().get(v))
                .filter(|v| v.is_constant())
                .and_then(|v| userop_name(v.get_offset() as u32))
                .map(|n| n == SWI_USEROP)
                .unwrap_or(false);
            if vector_ok && feeds_call && is_swi {
                return true;
            }
        }
        if curref.is_instruction_start() || pos == 0 {
            return false;
        }
        pos -= 1;
    }
}

#[cfg(test)]
#[path = "kuna_fastfailnoreturn/tests.rs"]
mod tests;
