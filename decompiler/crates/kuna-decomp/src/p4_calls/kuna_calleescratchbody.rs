//! (kuna) `calleescratchbody` — a helper that clobbers only the registers its
//! convention already lets it clobber is still a decoded body.
//!
//! # The symptom
//!
//! Every MSVC frame larger than a page opens with the out-of-line stack probe,
//! and the whole frame is lost across it:
//!
//! ```text
//! void sub_140001000(unsigned int a0,unsigned long long a1)
//! {
//!   int8 v2;                                             // rax -- NEVER assigned
//!
//!   sub_1408d0000();
//!   v1 = -v2;
//!   *(unsigned long long *)&(&Stack000000000005dfd0)[v1] = v7;
//!   *(unsigned int *)&(&Stack000000000005df98)[v1] = a0;
//! }
//! ```
//!
//! The disassembly settles it. The caller is the canonical large-frame prologue
//! `mov eax,0x5e028; call __chkstk; sub rsp,rax`, and the probe reads `RAX` and
//! never writes it:
//!
//! ```text
//! 1408d0000  sub  rsp,0x10
//! 1408d0004  mov  [rsp],r10
//! 1408d0008  mov  [rsp+0x8],r11
//! 1408d000d  xor  r11,r11
//! 1408d0010  lea  r10,[rsp+0x18]
//! 1408d0015  sub  r10,rax               ; reads RAX
//! ...
//! 1408d0040  mov  r10,[rsp]
//! 1408d0044  mov  r11,[rsp+0x8]
//! 1408d0049  add  rsp,0x10
//! 1408d004d  ret
//! ```
//!
//! So `RAX` holds `0x5e028` on both sides of the call, `sub rsp,rax` is a
//! constant frame allocation, and every local is at a fixed offset. kuna instead
//! indexes the whole frame off a local with no definition anywhere in the
//! function.
//!
//! # Why the value is lost
//!
//! Not for the reason the shape suggests.
//! [`calleepreserves`](crate::p4_calls::kuna_calleepreserves) exists for exactly
//! this and its evidence is already sufficient: the callee-body write probe
//! walks all nineteen instructions of the probe, completes, and proves `RAX`
//! untouched. What declines is the pass's *positive-evidence* gate. That gate
//! asks the callee to have written a register the convention promises is
//! **preserved** — the signature of the i386 get-PC thunk it was written for,
//! whose `mov ebx,[esp]` writes an `<unaffected>` register. `__chkstk` writes
//! `RSP`, `R10` and `R11`, and `x86-64-win.cspec` names none of them in either
//! `<unaffected>` or `<killedbycall>`: they are scratch, and clobbering scratch
//! is not a departure from the convention. The gate therefore never fires on a
//! well-behaved helper, which is most of them.
//!
//! # The evidence this pass adds
//!
//! A second, independent way for a summary to count as a body, so that being
//! well-behaved is no longer disqualifying. Both marks are required:
//!
//! * **The body wrote memory.** A register-preserving helper's own save slot is
//!   why it can use a register at all — `__chkstk` spills `R10`/`R11` before it
//!   touches them and reloads them before it returns. Every degenerate decode
//!   the gate exists to reject writes no memory at all.
//! * **The body wrote a register other than the stack pointer.** Every `RET`
//!   writes the stack pointer, so a summary carrying only it is the vacuous
//!   one, not a description of work — the same exclusion the shipped gate and
//!   [`calleeretpreserves`](crate::p4_calls::kuna_calleeretpreserves) make.
//!
//! Between them a `ret`, an `endbr64; ret` (how the `ghdec-*` fixtures stub
//! every callee outside the chunk under test), a placeholder and an entry
//! decoded at the wrong address are all still refused: none of them stores.
//!
//! # What it will not do
//!
//! This is one clause of `calleepreserves`'s gate and nothing else — every other
//! guard that pass documents still runs first and unchanged. The summary must be
//! complete, it must prove the queried range untouched, the range must be a
//! register, the prototype must not carry its own effect override, and the
//! effect must have been `killedbycall` to start with. Nothing is ever promoted
//! to killed. The pass can still only ever *keep* a value the machine keeps.
//!
//! Default-**on**: it widens which bodies count as evidence, never what the
//! evidence proves. Flip `off` to require a convention-preserved write again —
//! the reason to do that is the same one `calleepreserves` names, a callee
//! decoded in the wrong instruction mode, where the bytes read are not the
//! instructions that run.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleescratchbody>` (kuna 4000+ range; 4158 = the
/// previous kuna element).
pub const ELEM_CALLEESCRATCHBODY: ElementId = ElementId::new("calleescratchbody", 4159);

/// (kuna) Accept a decoded callee that clobbers only scratch registers as
/// `calleepreserves` evidence: `calleescratchbody on|off`.
pub struct OptionCalleeScratchBody;

impl OptionCalleeScratchBody {
    /// The option name.
    pub const NAME: &'static str = "calleescratchbody";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_scratch_body`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Scratch-only callee bodies as preservation evidence turned {prop}")))
    }
}

/// How many decoded instructions a scratch-only body needs before its silence
/// counts.
///
/// One is a bare `ret`. The store and the register write below are the real
/// discriminators; this only refuses the shapes too short to have either.
const MIN_HELPER_INSTRUCTIONS: u32 = 2;

/// Did this body do work no stub can fake — a memory write, plus a register
/// write that is not the stack pointer?
///
/// The second half of [`crate::p4_calls::kuna_calleepreserves`]'s
/// positive-evidence gate, for the callee that clobbers only what the
/// convention already allows. Answers `false` whenever the option is off, so
/// the shipped gate is the only one that speaks.
pub fn scratch_body_is_a_body(
    data: &Funcdata,
    w: &crate::kuna_rustabi::CalleeReturnWrites,
) -> bool {
    if !data.get_arch().callee_scratch_body {
        return false;
    }
    if w.instruction_count() < MIN_HELPER_INSTRUCTIONS {
        return false;
    }
    if w.store_spaces().is_empty() {
        return false;
    }
    let manage = data.get_arch().manage();
    let stack_pointer = manage
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .and_then(|p| p.space.clone().map(|sp| (sp.get_index(), p.offset, p.size as u64)));
    w.written_ranges().iter().any(|&(idx, off, sz)| !overlaps(stack_pointer, idx, off, sz))
}

/// Does `[off, off+sz)` in space `idx` touch the named register range?
fn overlaps(reg: Option<(int4, u64, u64)>, idx: int4, off: u64, sz: int4) -> bool {
    match reg {
        Some((ridx, roff, rsz)) => idx == ridx && off < roff + rsz && roff < off + sz as u64,
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_calleescratchbody/tests.rs"]
mod tests;
