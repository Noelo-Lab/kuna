//! (kuna) `calleepreserves` — a decoded helper's own body narrows the ABI's
//! killed-register set at the call.
//!
//! # The symptom
//!
//! An i386 PIE built by gcc reaches every global through a get-PC thunk, and the
//! argument the caller loaded before it disappears:
//!
//! ```text
//! int sub_8049f20(char *path,void *buf)
//! {
//!   unsigned int v1;                  // edx  -- declared, read once, NEVER assigned
//!
//!   sub_8049f55();
//!   return __lxstat(3,path,v1);       // the `buf` the caller was handed
//! }
//! ```
//!
//! The disassembly settles what `v1` is:
//!
//! ```text
//! 8049f29  mov edx,[ebp+0xc]          ; the `buf` parameter
//! 8049f2c  call 8049f55               ; 8049f55: mov ebx,[esp]; ret
//! 8049f3e  mov [esp+0x8],edx          ; __lxstat's third argument
//! 8049f42  mov edx,[ebp+0x8]          ; the `path` parameter
//! ```
//!
//! and the controlled comparison is inside the one function: `path` is loaded
//! from the frame AFTER the call and is recovered, `buf` is loaded BEFORE it and
//! is lost. The only difference is that `EDX` crosses the `CALL`.
//!
//! # Why the value is lost
//!
//! `Heritage::guardCalls` (`heritage.cc:1444`) asks the call's prototype what it
//! does to each heritaged range, and `x86gcc.cspec` lists `EAX`/`ECX`/`EDX` in
//! `<killedbycall>` because cdecl says a callee may clobber them. That answer is
//! about *the convention*, not about this callee: the effect comes back
//! `killedbycall`, an INDIRECT *creation* is planted at the call, and every read
//! of `EDX` after it reads a value with no definition anywhere in the function.
//!
//! The convention is right in general and wrong here. `sub_8049f55` is two bytes
//! of body — `mov ebx,[esp]; ret` — and writes `EBX` and nothing else. gcc emits
//! it precisely because it is cheaper than a convention-abiding call, and every
//! i386 PIE contains one per register it uses for the GOT base.
//!
//! # The evidence this pass adds
//!
//! The callee's own instructions, read directly. [`probe_callee_return_writes`]
//! already decodes a direct-call target's body for the call-output seam and
//! reports the processor-space ranges it is *proven* to write; this pass consults
//! the same summary on the *input* side. When it proves the callee never writes
//! the heritaged register, the `killedbycall` effect is downgraded to
//! `unaffected` for that one call, no INDIRECT is planted, and the caller's value
//! flows across the call — which is what the machine does.
//!
//! # What it will not do
//!
//! The claim is one-sided: it is the absence of a write on a walk that covered
//! every path, and everything the walk cannot see makes it decline.
//!
//! * **A fully decoded, call-free callee only.** The probe declares itself
//!   incomplete — proving nothing — at a nested `CALL`/`CALLIND`/`CALLOTHER`, an
//!   unresolved `BRANCHIND`, an undecodable instruction, or its instruction
//!   budget. A PLT stub (`jmp [got]`) is a `BRANCHIND` and so is never narrowed,
//!   which is what keeps every library call on the ABI's answer. Recursion is a
//!   `CALL` and lands in the same place.
//! * **Register storage only.** The probe records processor-space writes; a
//!   callee's writes to memory are `STORE`s whose address is a runtime value, so
//!   a stack or `ram` range keeps the ABI's answer. Only a range in the
//!   processor space is ever downgraded.
//! * **Only `killedbycall`, and only downward.** An `unaffected` or
//!   `return_address` record is left exactly as the spec wrote it, and nothing
//!   here ever promotes a range to killed. The pass can only ever *keep* a value
//!   that the machine keeps.
//! * **The return register is not affected.** The downgrade is applied to the
//!   effect the spec supplies; `guardCalls`'s own output-active branch
//!   re-promotes the range to `killedbycall` when it is the call's return
//!   storage, so return-value recovery is reached with the same input it had
//!   before.
//! * **An explicitly overridden prototype wins.** A call whose `FuncProto`
//!   carries its own effect list (a decoded `<unaffected>`/`<killedbycall>`
//!   override) has had a deliberate statement made about it and is left alone.
//!
//! Default-**on**: it fires only against a decoded body that contradicts the
//! convention, and only in the direction of keeping a value the caller computed.
//! Flip `off` to restore the ABI-only answer — the reason to do that is a callee
//! decoded in the wrong instruction mode (an ARM/Thumb boundary), where the bytes
//! read are not the instructions that run.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::fspec::FuncCallSpecs;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleepreserves>` (kuna 4000+ range; 4146 = the previous
/// kuna element).
pub const ELEM_CALLEEPRESERVES: ElementId = ElementId::new("calleepreserves", 4147);

/// (kuna) Narrow a call's killed-register set to the callee's decoded writes:
/// `calleepreserves on|off`.
pub struct OptionCalleePreserves;

impl OptionCalleePreserves {
    /// The option name.
    pub const NAME: &'static str = "calleepreserves";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_preserves`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body killed-register narrowing turned {prop}")))
    }
}

/// Take the callee-body write probe for every direct call in `data`, so the
/// call-guard seam can consult it.
///
/// Shares the per-image cache and the walk with
/// [`crate::kuna_rustabi::seed_callee_return_writes`]; whichever of the two rules
/// is live pays for the decode once. Driven from the driver for the same reason
/// that one is — the per-function `ArchContext` the pipeline runs against carries
/// the load image but no translator, so this is the last point the callee's
/// instructions can be read at all.
pub fn seed_callee_preserves(
    arch: &mut crate::architecture::Architecture,
    data: &mut Funcdata,
) {
    if !arch.callee_preserves {
        return;
    }
    crate::kuna_rustabi::seed_callee_write_probe(arch, data);
}

/// Does the decoded body of `fc`'s callee prove it never writes
/// `[addr, addr+size)`?
///
/// `addr` is in the CALLEE's perspective, as `guardCalls` computes it. Answers
/// `false` — leaving the ABI's effect exactly as it was — unless every condition
/// in the module header holds.
pub fn callee_preserves_range(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> bool {
    if !data.get_arch().callee_preserves {
        return false;
    }
    // Only a register. The probe answers processor-space writes; a callee's
    // memory writes are STOREs through a runtime address it cannot follow.
    match addr.get_space() {
        Some(sp) if sp.get_type() == spacetype::IPTR_PROCESSOR => {}
        _ => return false,
    }
    // A prototype that carries its own effect override has had a deliberate
    // statement made about it.
    if fc.proto().has_effect_override() {
        return false;
    }
    let entry = fc.get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    let Some(w) = data.kuna_callee_ret_writes(entry) else { return false };
    if !w.proves_untouched(addr, size) {
        return false;
    }
    body_departs_from_convention(data, fc, w)
}

/// Does the decoded body demonstrably depart from the convention the effect list
/// describes -- does it write a register the model itself promises is preserved?
///
/// This is the positive half of the evidence, and it is what keeps the rule off
/// a body that is not really a body. A summary that records no write at all is a
/// maximal claim -- *every* register survives this call -- drawn from the
/// weakest possible reading, and a one-byte `ret` is exactly what a stub, a
/// placeholder and a misidentified entry all decode to. Requiring the callee to
/// have written a callee-saved register is the signature of the hand-rolled
/// helper this rule exists for: a get-PC thunk loads the GOT base into `EBX`,
/// which `x86gcc.cspec` lists as `<unaffected>`, so the convention is already
/// not a description of it. The stack pointer does not count -- every `RET`
/// writes it.
fn body_departs_from_convention(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    w: &crate::kuna_rustabi::CalleeReturnWrites,
) -> bool {
    let manage = data.get_arch().manage();
    let stack_pointer = manage
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .and_then(|p| p.space.clone().map(|sp| (sp.get_index(), p.offset, p.size as u64)));
    for &(idx, off, sz) in w.written_ranges() {
        if let Some((sidx, soff, ssz)) = stack_pointer {
            if idx == sidx && off < soff + ssz && soff < off + sz as u64 {
                continue;
            }
        }
        let Some(space) = manage.get_space(idx) else { continue };
        let waddr = Address::new(std::rc::Rc::clone(space), off);
        if fc.proto().has_effect(&waddr, sz) == crate::fspec::effect_type::UNAFFECTED {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[path = "kuna_calleepreserves/tests.rs"]
mod tests;
