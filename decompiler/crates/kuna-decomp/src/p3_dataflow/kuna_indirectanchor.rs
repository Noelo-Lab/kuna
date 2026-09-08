//! (kuna) `indirectanchor` — an op inserted after a call's guard INDIRECT
//! belongs after the CALL, not inside the guard run.
//!
//! # The symptom
//!
//! A Windows PE calls `BCryptGetProperty` through its IAT slot and branches on
//! the `NTSTATUS` it hands back:
//!
//! ```text
//!   140004762  mov  rcx,[rbp-0x50]
//!   140004766  call qword ptr [0x1400282b8]   ; BCryptGetProperty
//!   14000476c  test eax,eax
//!   14000476e  js   140004d32
//! ```
//!
//! kuna emitted the call for its side effect and then tested a local nothing in
//! the function assigns:
//!
//! ```text
//!   BCryptGetProperty(v48,v27,&v50,4,v49,v70 & 0xffffffff00000000);
//!   if ((v16 < 0) || (v28 = (unsigned long long)v50, !v50)) {
//! ```
//!
//! Every other `bcrypt` import in the same function recovered its result
//! (`v15 = BCryptSetProperty(...)`), including the one immediately before it
//! whose result is consumed by the very same `test eax,eax; js` pair.
//!
//! # Why the value is lost
//!
//! `Heritage::guardCalls` plants one INDIRECT per killed range immediately
//! before a CALL, and the return-value trial's Varnode is the output of the
//! INDIRECT *creation* among them. `FuncCallSpecs::collectOutputTrialVarnodes`
//! (`fspec.cc:5543`) finds it by walking backwards from the CALL and stopping at
//! the first op that is not a `CPUI_INDIRECT` — exact only while that run is
//! unbroken.
//!
//! Upstream keeps it unbroken from both sides. `opInsertBefore` skips back over
//! the run, and `opInsertAfter` (`funcdata_op.cc:373`) *redirects*: asked to
//! insert after an INDIRECT, it decodes the iop annotation in the marker's
//! second input and inserts after the CALL (or STORE) the INDIRECT speaks for.
//! kuna carried that redirect as a `STUB(W3-varnode)` — the iop decode was a
//! later wave's — so an op inserted after a guard INDIRECT landed *between* the
//! guards and their CALL.
//!
//! At the call site above, heritage refines the 8-byte frame slot at `-0x2d0`
//! into two 4-byte halves and defines the upper half with a SUBPIECE inserted
//! after that slot's own guard INDIRECT. The run became
//!
//! ```text
//!   INDIRECT EAX          [create]   <-- the return trial
//!   INDIRECT <16 globals>
//!   INDIRECT stack -0x2d0
//!   SUBPIECE stack -0x2cc            <-- landed inside the run
//!   INDIRECT <79 more>
//!   CALL     BCryptGetProperty
//! ```
//!
//! and the backward walk stopped at the SUBPIECE, 18 ops short of the `EAX`
//! creation. No trial Varnode was found, `checkOutputTrialUse` marked every
//! trial *no-use*, `buildOutputFromTrials` gave the CALL no output, and the
//! INDIRECT creation was left standing — which is precisely a local the emitted
//! C reads and never assigns.
//!
//! # What this pass does
//!
//! It completes the redirect: [`anchor_of`] decodes the iop annotation and
//! answers the CALL or STORE the INDIRECT belongs to, so `op_insert_after`
//! places the new op after *that* and the guard run stays whole. Nothing else
//! changes — a `prev` that is not an INDIRECT marker, an INDIRECT whose second
//! input is not an iop annotation, and an iop that decodes to a dead op all keep
//! the op exactly where it was going.
//!
//! Default-**on**: it is upstream's own behaviour and the invariant the rest of
//! the call machinery is written against. Flip `off` to get the stubbed
//! placement back when bisecting whether op placement rather than trial scoring
//! moved a call's surroundings.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;

use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<indirectanchor>` (kuna 4000+ range; 4154 =
/// calleearitycut).
pub const ELEM_INDIRECTANCHOR: ElementId = ElementId::new("indirectanchor", 4155);

/// (kuna) Anchor an op inserted after a guard INDIRECT to the op that INDIRECT
/// speaks for: `indirectanchor on|off`.
pub struct OptionIndirectAnchor;

impl OptionIndirectAnchor {
    /// The option name.
    pub const NAME: &'static str = "indirectanchor";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::indirect_anchor`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("INDIRECT-marker insertion anchoring turned {prop}")))
    }
}

/// The op `prev` really stands for, when `prev` is a guard INDIRECT: the CALL or
/// STORE named by the iop annotation in its second input (C++
/// `Funcdata::opInsertAfter`, `funcdata_op.cc:373`).
///
/// `None` leaves the caller on `prev` itself, which is what a non-marker, a
/// non-INDIRECT, a second input that is not an iop annotation, and an iop that
/// decodes to a dead op all mean.
pub fn anchor_of(data: &Funcdata, prev: OpId) -> Option<OpId> {
    if !data.get_arch().indirect_anchor {
        return None;
    }
    let op = data.obank().get(prev)?;
    if !op.is_marker() || op.code() != OpCode::CPUI_INDIRECT {
        return None;
    }
    let invn = data.vbank().get(op.get_in(1)?)?;
    if invn.get_space().get_type() != spacetype::IPTR_IOP {
        return None;
    }
    let targ = crate::funcdata_varnode::op_iop_decode(invn.get_offset());
    let alive = data.obank().get(targ).map(|o| !o.is_dead()).unwrap_or(false);
    alive.then_some(targ)
}

#[cfg(test)]
#[path = "kuna_indirectanchor/tests.rs"]
mod tests;
