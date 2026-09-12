//! (kuna) `calleeretpreserves` — a decoded helper's own body also answers for
//! the call's RETURN register, so a value the caller placed there before the
//! call is not replaced by an invented result.
//!
//! # The symptom
//!
//! `main` of an MSVC `/GS` PE ends by returning the frame-cookie check:
//!
//! ```text
//!   v12 = v29 ^ (unsigned long long)v14;
//!   return sub_140007d90(v12);
//! ```
//!
//! The disassembly says the answer is zero, and says it twice — the caller sets
//! `EAX` to zero *before* the call, and the callee's returning path never
//! touches `RAX` at all:
//!
//! ```text
//!   1400061fb  xor  eax,eax               ; the value main returns
//!   1400061fd  mov  rcx,[rbp+0x1a0]
//!   140006204  xor  rcx,rsp
//!   140006207  call 140007d90             ; __security_check_cookie
//!   140006217  ret
//!
//!   140007d90  cmp  rcx,[14000b040]       ; the whole callee
//!   140007d97  jnz  140007da9
//!   140007d99  rol  rcx,0x10
//!   140007d9d  test cx,0xffff
//!   140007da2  jnz  140007da5
//!   140007da4  ret
//!   140007da5  ror  rcx,0x10
//!   140007da9  jmp  1400081d8             ; mov ecx,2; int 0x29  (__fastfail)
//! ```
//!
//! # Why the value is lost
//!
//! `x86-64-win.cspec` lists `RAX` in `<killedbycall>`, so `Heritage::guardCalls`
//! plants an INDIRECT *creation* of `RAX` at the call and registers a return
//! trial for it. The caller's `xor eax,eax` is then dead, the trial is used by
//! the `RET`, and the call acquires a return value the machine never computes.
//!
//! [`calleepreserves`](crate::p4_calls::kuna_calleepreserves) is the pass that
//! narrows `killedbycall` from the callee's own decoded body, and it declines
//! here for its own reason: its positive-evidence gate asks the callee to have
//! written a register the convention promises is *preserved*, which is the
//! signature of a get-PC thunk and not of this helper. The cookie checker writes
//! `RCX` and the flags — both registers the convention already allows it to
//! clobber — so the gate that keeps that pass off a body which is not really a
//! body also keeps it off this one.
//!
//! # The evidence this pass adds
//!
//! For the call's **return storage** the question is sharper than for a scratch
//! register, and so is the evidence. A callee that returns a value in `RAX`
//! must *write* `RAX`. When the same bounded, complete decode
//! ([`probe_callee_return_writes`](crate::kuna_rustabi::probe_callee_return_writes))
//! covered every path to a `RETURN` and recorded no write to the return storage
//! at all, the callee cannot be returning anything in it: the effect is
//! downgraded to `unaffected`, no INDIRECT creation is planted, no output trial
//! is registered, and the caller's own definition flows across the call —
//! `return 0;`.
//!
//! A locked `void` declaration removes the call's concrete output entry, but it
//! does not state that the callee destroys a value the caller already placed in
//! an ABI return register. For that case alone, [`characterize_preserved_output`]
//! consults the prototype model's output list to identify eligible storage. This
//! never widens to arbitrary killed scratch registers, and a locked non-void
//! output remains authoritative. If the logical output is only one slice of a
//! wider machine-register heritage range (Win64's 8-byte `XMM0_Qa` inside the
//! 16-byte `XMM0`), Heritage keeps exactly that slice and builds killed INDIRECT
//! creations for the flanks; it never downgrades the whole wider range.
//!
//! # What it will not do
//!
//! * **The call's exact return storage only.** The range must characterize as the
//!   declared call output, or—only for a locked `void` declaration—as storage
//!   from the ABI model's output list. Every other `killedbycall` range stays
//!   exactly where `calleepreserves` left it. A containing XMM range is split so
//!   its output slice survives and its overlapping upper scratch lane is still
//!   killed by the call.
//! * **A callee that returns nothing at all.** If the body writes ANY part of
//!   the return storage -- `RAX` while leaving `RDX` alone, on a model whose
//!   return list is both -- it is a value-returning function whose second
//!   register is merely dead, and the convention keeps its answer for every
//!   register of that call.
//! * **A complete decode for absence claims.** The probe declares itself
//!   incomplete at a nested `CALL`, an unresolved `BRANCHIND`, an undecodable
//!   instruction, or its instruction budget, and an incomplete summary answers
//!   "may write" to every untouched range. Positive register writes and STORE
//!   spaces recovered before that edge remain valid veto evidence.
//! * **A body that is really a body.** A summary recording no write at all is a
//!   maximal claim drawn from the weakest reading, and a one-byte `ret` is what
//!   a stub, a placeholder and a misidentified entry all decode to. The walk
//!   must have decoded more than one instruction AND recorded a write to a
//!   register the convention itself names — an argument register, or one its
//!   `<unaffected>`/`<killedbycall>` lists mention — before its silence about
//!   the return register counts as evidence. The scratch register the cookie
//!   checker rolls is `RCX`, the first argument register, and is exactly that;
//!   an `endbr64; ret` stub writes nothing but the program counter and the
//!   stack pointer and is exactly not.
//! * **An explicitly overridden prototype wins**, as in `calleepreserves`: a
//!   call whose `FuncProto` carries its own effect list has had a deliberate
//!   statement made about it.
//! * **A syntactically returning failure tail with nested calls proves
//!   nothing.** The generic body walk stays conservative. The separate exact
//!   MSVC-cookie marker may preserve ABI output at its seeded call sites; it is
//!   algebraic caller evidence, not a relaxation of body completeness. It may
//!   tolerate an incomplete body summary, but any return-storage write the
//!   summary did recover still wins. A `STORE` through an address in an ABI
//!   output processor space is a possible write to every output in that space,
//!   so it is the same veto even though the probe cannot name a direct range.
//!   Every explicit effect override wins as well.
//!
//! Default-**on**: it fires only against a fully decoded body that provably
//! never writes the register the ABI says carries the answer, and it only ever
//! keeps a value the caller computed. Flip `off` to restore the convention-only
//! answer — the reason to do that is a callee decoded in the wrong instruction
//! mode, where the bytes read are not the instructions that run.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::pcoderaw::VarnodeData;

use crate::funcdata::Funcdata;
use crate::fspec::{effect_type, Containment, FuncCallSpecs};
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleeretpreserves>` (kuna 4000+ range; 4152 = the
/// previous kuna element).
pub const ELEM_CALLEERETPRESERVES: ElementId = ElementId::new("calleeretpreserves", 4153);

/// (kuna) Let the callee's decoded body answer for the call's return register:
/// `calleeretpreserves on|off`.
pub struct OptionCalleeRetPreserves;

impl OptionCalleeRetPreserves {
    /// The option name.
    pub const NAME: &'static str = "calleeretpreserves";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_ret_preserves`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body return-register narrowing turned {prop}")))
    }
}

/// Does the decoded body of `fc`'s callee prove it never writes the return
/// storage `[addr, addr+size)`, so the caller's own value survives the call?
///
/// `addr` is in the CALLEE's perspective, as `guardCalls` computes it. Answers
/// `false` — leaving the ABI's effect exactly as it was — unless every condition
/// in the module header holds.
pub fn callee_preserves_return_storage(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> bool {
    // The call's own return storage, and nothing else: every other killedbycall
    // range is `calleepreserves`'s question.
    if characterize_preserved_output(fc, addr, size) != Containment::ContainsJustified {
        return false;
    }
    callee_never_writes(data, fc, addr, size)
}

/// May an exact caller-side MSVC cookie proof preserve this return range?
///
/// The cookie algebra is independent evidence that permits an incomplete body
/// summary, but it does not overrule facts the summary did recover. A known
/// write to any ABI return storage, a STORE into an ABI output processor space,
/// and an explicit prototype effect all win.
pub(crate) fn exact_cookie_preserves_return_storage(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> bool {
    if characterize_preserved_output(fc, addr, size) != Containment::ContainsJustified {
        return false;
    }
    exact_cookie_evidence(data, fc, addr)
}

/// Return the exact ABI-output slice contained by a wider killed range when the
/// generic complete-body proof permits preserving that slice.
pub(crate) fn callee_preserved_output_within(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> Option<VarnodeData> {
    let output = contained_preserved_output(fc, addr, size)?;
    let output_addr = Address::new(output.space.clone()?, output.offset);
    callee_never_writes(data, fc, &output_addr, output.size as int4).then_some(output)
}

/// Return the exact ABI-output slice contained by a wider killed range when an
/// exact cookie site permits preserving that slice.
pub(crate) fn exact_cookie_preserved_output_within(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> Option<VarnodeData> {
    let output = contained_preserved_output(fc, addr, size)?;
    let output_addr = Address::new(output.space.clone()?, output.offset);
    exact_cookie_evidence(data, fc, &output_addr).then_some(output)
}

fn exact_cookie_evidence(data: &Funcdata, fc: &FuncCallSpecs, addr: &Address) -> bool {
    match addr.get_space() {
        Some(sp) if sp.get_type() == spacetype::IPTR_PROCESSOR => {}
        _ => return false,
    }
    if fc.proto().has_effect_override() {
        return false;
    }
    let entry = fc.get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    data.kuna_callee_ret_writes(entry)
        .map(|w| !writes_some_return_storage(data, fc, w))
        .unwrap_or(true)
}

/// Resolve the logical return storage inside a wider heritage range. The
/// concrete locked output wins; only locked `void` falls back to the model.
fn contained_preserved_output(
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> Option<VarnodeData> {
    if characterize_preserved_output(fc, addr, size) != Containment::ContainedBy {
        return None;
    }
    let mut output = VarnodeData::default();
    if fc.proto().get_biggest_contained_output(addr, size, &mut output) {
        return Some(output);
    }
    if !locked_void_uses_model_output(fc)
        || !fc.proto().model().get_biggest_contained_output(addr, size, &mut output)
    {
        return None;
    }
    Some(output)
}

fn locked_void_uses_model_output(fc: &FuncCallSpecs) -> bool {
    fc.proto().is_output_locked()
        && fc.proto().get_output_type().map(|t| t.get_metatype())
            == Some(crate::dtype::type_metatype::TYPE_VOID)
}

/// Characterize the return storage a caller may carry across a declared-void
/// helper.
///
/// A locked void declaration correctly says the call produces no value, but it
/// says nothing about whether the callee preserves a value the caller placed in
/// an ABI return register before the call. In that one state, use the model's
/// output list solely to identify the narrow set of killed registers for which
/// the body proof is valid. A non-void locked return continues to win.
pub(crate) fn characterize_preserved_output(
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> Containment {
    let declared = fc.proto().characterize_as_output(addr, size);
    if declared != Containment::NoContainment {
        return declared;
    }
    if !locked_void_uses_model_output(fc) {
        return Containment::NoContainment;
    }
    fc.proto().model().characterize_as_output(addr, size)
}

/// The evidence half of [`callee_preserves_return_storage`], with the
/// "is this the call's return storage" question already answered.
pub(crate) fn callee_never_writes(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> bool {
    if !data.get_arch().callee_ret_preserves {
        return false;
    }
    match addr.get_space() {
        Some(sp) if sp.get_type() == spacetype::IPTR_PROCESSOR => {}
        _ => return false,
    }
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
    if writes_some_return_storage(data, fc, w) {
        return false;
    }
    body_is_a_body(data, fc, w)
}

/// Does the decoded body write, or possibly STORE through, ANY of the call's
/// return storage?
///
/// The claim this rule makes is that the callee returns nothing -- not that it
/// returns something in a different register than the one being asked about. A
/// callee that writes `RAX` and leaves `RDX` alone is a scalar-returning
/// function whose second return register happens to be dead, and the convention
/// is still the best answer for that register: the caller's pre-call `RDX` is
/// not the callee's business either way. Restricting the rule to a body that
/// touches NONE of the return storage is what keeps it on the void helper it
/// exists for. An indexed STORE cannot name its destination range, so a STORE
/// into a processor space used by an ABI output entry counts as touching every
/// return location in that space.
fn writes_some_return_storage(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    w: &crate::kuna_rustabi::CalleeReturnWrites,
) -> bool {
    if w.store_spaces().iter().any(|&idx| {
        fc.proto()
            .model()
            .output()
            .get_entry()
            .iter()
            .any(|entry| {
                if entry.get_space().get_type() == spacetype::IPTR_PROCESSOR
                    && entry.get_space().get_index() == idx
                {
                    return true;
                }
                entry.get_join_record().is_some_and(|join| {
                    (0..join.num_pieces()).any(|i| {
                        join.get_piece(i).space.as_ref().is_some_and(|space| {
                            space.get_type() == spacetype::IPTR_PROCESSOR
                                && space.get_index() == idx
                        })
                    })
                })
            })
    }) {
        return true;
    }
    let manage = data.get_arch().manage();
    w.written_ranges().iter().any(|&(idx, off, sz)| {
        let Some(space) = manage.get_space(idx) else { return false };
        let waddr = Address::new(std::rc::Rc::clone(space), off);
        characterize_preserved_output(fc, &waddr, sz) != Containment::NoContainment
    })
}

/// How many decoded instructions a callee body has to have before its silence
/// about the return register counts as evidence.
///
/// One is a bare `ret`, and a bare `ret` is what a stub, a placeholder, an
/// import thunk and an entry decoded at the wrong address all look like.  The
/// datatest corpus has this shape on purpose: the `ghdec-*` fixtures stub every
/// callee outside the chunk under test as the single byte `c3`, and reading that
/// stub as a promise about `RAX` deletes the call results the rest of the
/// function is built on.
const MIN_BODY_INSTRUCTIONS: u32 = 2;

/// Did the decoded body do any work at all — more than one instruction, and a
/// write to a register the convention itself names?
///
/// This is the positive half of the evidence, and it is what keeps the rule off
/// a summary that is not really a summary. `proves_untouched` is the *absence*
/// of a write, and absence is also what an entry decoded at the wrong address, a
/// placeholder and a stub all produce. The stack pointer and the program counter
/// are written by every `ret` and say nothing, so the body has to have touched a
/// register the calling convention has an opinion about — an argument register,
/// or one its `<unaffected>`/`<killedbycall>` lists mention. `endbr64; ret` (how
/// the `ghdec-*` fixtures stub every callee outside the chunk under test) writes
/// neither, and reading it as a promise about `RAX` deletes the call results the
/// rest of the function is built on.
fn body_is_a_body(
    data: &Funcdata,
    fc: &FuncCallSpecs,
    w: &crate::kuna_rustabi::CalleeReturnWrites,
) -> bool {
    if w.instruction_count() < MIN_BODY_INSTRUCTIONS {
        return false;
    }
    let manage = data.get_arch().manage();
    let stack_pointer = manage
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .and_then(|p| p.space.clone().map(|sp| (sp.get_index(), p.offset, p.size as u64)));
    w.written_ranges().iter().any(|&(idx, off, sz)| {
        if let Some((sidx, soff, ssz)) = stack_pointer {
            if idx == sidx && off < soff + ssz && soff < off + sz as u64 {
                return false;
            }
        }
        let Some(space) = manage.get_space(idx) else { return false };
        let waddr = Address::new(std::rc::Rc::clone(space), off);
        fc.proto().characterize_as_input_param(&waddr, sz) != Containment::NoContainment
            || matches!(
                fc.proto().has_effect(&waddr, sz),
                effect_type::UNAFFECTED | effect_type::KILLEDBYCALL
            )
    })
}

#[cfg(test)]
#[path = "kuna_calleeretpreserves/tests.rs"]
mod tests;
