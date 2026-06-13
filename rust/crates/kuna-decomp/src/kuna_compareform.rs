//! Port of `decompiler/cpp/kuna_compareform.{cc,hh}` — comparison-form
//! presentation sub-stage (kuna GH-558, S3/S9 presentation split).
//!
//! The decompiler canonicalizes `V <= c` into `V < c+1` through one primitive
//! ([`Funcdata::replace_lessequal`](crate::funcdata::Funcdata)), which records
//! provenance on the rewritten op via `PcodeOp::set_canonical_lessequal` (the
//! `canonical_lessequal` `addlflags` bit, `op.rs`).  [`ActionPresentCompareForm`]
//! runs after the last canonicalization site and before prototype/cast/naming
//! fixation; gated default-on by the arch flag `present_lessequal` (option
//! `compareform canonical|original`, shipped default `original` = restore the
//! source LESSEQUAL form), it inverts the canonicalization for marked ops:
//!
//! ```text
//!   V < c+1   =>   V <= c          (canonical `V < c+1` came from `V <= c`)
//!   c-1 < V   =>   c <= V          (canonical `c-1 < V` came from `c <= V`)
//! ```
//!
//! [`restore_lessequal`] is the exact inverse of `Funcdata::replaceLessequal`:
//! the mark only asserts provenance, so the op shape and the overflow guards are
//! re-validated from scratch and an op reshaped by a later transform dies
//! gracefully (no rewrite).
//!
//! ## Gate wiring — SEAM(W4)
//!
//! As in the sibling kuna sub-stages, the C++
//! `if (!data.getArch()->present_lessequal) return 0;` gate is resolved at
//! construction (the seam `Funcdata::glb` does carry `present_lessequal`, but the
//! kuna leaf actions take the resolved gate as a constructor argument to stay
//! uniform with the rule pool).  W8/W9 thread the live
//! `Architecture::present_lessequal`; [`action`] uses the shipped default
//! (`original` => `present_lessequal = true`, kuna DIV-2 default-on, GH-558).
//!
//! The option-apply body that selects that gate value — `OptionCompareForm::apply`
//! (`kuna_compareform.cc:85`) — is ported as [`parse_compare_form`] (mirroring the
//! sibling `parse_return_pair_form` / `parse_memset_recover_form`): it parses
//! `original`/`canonical` into [`CompareForm`] (the `glb->present_lessequal` write
//! becomes the returned form, threaded by `w4-kuna-p0-pack` into
//! [`Architecture::present_lessequal`]) and surfaces the C++ `throw ParseError(...)`
//! as a [`KunaError::parse`].
//!
//! ## SEAM(W6) — opcode-flag resolution
//!
//! `opSetOpcode(...)` resolves `glb->inst[opc]` (the W6 typeop `inst` table); the
//! restored `INT_LESSEQUAL`/`INT_SLESSEQUAL` op-shells carry the verbatim
//! `typeop.cc` opflags (`binary | booloutput`) — mirroring `w6_type_op` in
//! `funcdata_op.rs`.  Noted in the structured losses.
//!
//! ## SEAM(W4) — `copySymbol`
//!
//! The C++ `restoreLessequal` calls `newvn->copySymbol(vn)` to carry the original
//! constant's data-type/Symbol (e.g. enum typing) onto the re-adjusted constant.
//! `Varnode::copySymbol` is the W4 symbol-table seam (not yet ported; the sibling
//! `Funcdata::replace_lessequal` skips it the same way).  The fresh constant keeps
//! its `newConstant` default type.  Noted in the structured losses.

use kuna_base::address::{calc_int_max, calc_int_min, calc_mask, calc_uint_max};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};

/// (kuna GH-558) Restore canonicalized comparisons to their original LESSEQUAL
/// form for presentation (C++ `ActionPresentCompareForm`).
pub struct ActionPresentCompareForm {
    /// Engine-owned bookkeeping (C++ inherited `Action` base fields).
    base: ActionBase,
    /// Resolved `glb->present_lessequal` gate (SEAM(W4); see module docs).
    enabled: bool,
}

impl ActionPresentCompareForm {
    /// Construct in group `g` with the resolved gate
    /// (C++ `ActionPresentCompareForm::ActionPresentCompareForm(const string &g)`).
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionPresentCompareForm {
        // C++: Action(0,"presentcompareform",g)
        ActionPresentCompareForm { base: ActionBase::new(0, "presentcompareform", g), enabled }
    }

    /// Box the action (parallels the `ActionXxx::boxed` constructors in
    /// `coreaction_early`).
    pub fn boxed(enabled: bool, g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionPresentCompareForm::new(enabled, g))
    }
}

impl Action for ActionPresentCompareForm {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    /// C++ `ActionPresentCompareForm::clone`:
    /// `if (!grouplist.contains(getGroup())) return (Action *)0;`
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionPresentCompareForm { base: self.base.clone(), enabled: self.enabled }))
    }

    /// C++ `ActionPresentCompareForm::apply` (`kuna_compareform.cc:67`) —
    /// transcribed.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // if (!data.getArch()->present_lessequal) return 0;  // P0 assertion not set
        if !self.enabled {
            return 0;
        }
        // for(iter=data.beginOpAlive();iter!=data.endOpAlive();++iter) {
        //   PcodeOp *op = *iter;  OpCode opc = op->code();
        //   if ((opc != CPUI_INT_LESS)&&(opc != CPUI_INT_SLESS)) continue;
        //   if (!op->isCanonicalLessequal()) continue;
        //   if (restoreLessequal(op,data)) count += 1;
        // }
        //
        // The C++ iterates the live `alivelist` while `restoreLessequal` mutates
        // ops in place (opcode + one input; never inserts/removes ops), so the
        // iteration order is stable.  Snapshot the op ids up front (the same
        // snapshot idiom `constseq` uses) so the mutable `restore_lessequal`
        // calls do not alias the bank iterator.
        let ops: Vec<OpId> = data.obank().iter_alive().collect();
        for op in ops {
            let opc = match data.obank().get(op) {
                Some(o) => o.code(),
                None => continue, // (cannot happen: restore_lessequal never frees ops)
            };
            if opc != OpCode::CPUI_INT_LESS && opc != OpCode::CPUI_INT_SLESS {
                continue;
            }
            if !data.obank().get(op).expect("op live").is_canonical_lessequal() {
                continue;
            }
            if restore_lessequal(op, data) {
                self.base.count += 1;
            }
        }
        0
    }
}

/// Invert one canonicalized comparison:  `V < c+1  =>  V <= c`,
/// `c-1 < V  =>  c <= V` (C++ static `restoreLessequal`).
///
/// Inverse of `Funcdata::replaceLessequal`, gated on the
/// `canonical_lessequal` provenance mark.  The mark only asserts provenance; the
/// op shape and overflow conditions are re-validated from scratch, so an op
/// reshaped by later transforms dies gracefully (no rewrite).  Returns `true` if
/// the original LESSEQUAL form was restored.
fn restore_lessequal(op: OpId, data: &mut Funcdata) -> bool {
    // if ((vn=op->getIn(0))->isConstant()) { diff = 1;  i = 0; }   // c-1 < V from c <= V
    // else if ((vn=op->getIn(1))->isConstant()) { diff = -1; i = 1; } // V < c+1 from V <= c
    // else return false;
    let in0 = match op_in(data, op, 0) {
        Some(v) => v,
        None => return false,
    };
    let in1 = match op_in(data, op, 1) {
        Some(v) => v,
        None => return false,
    };
    let (vn, diff, i): (VarnodeId, intb, int4) = if vn_is_constant(data, in0) {
        (in0, 1, 0) // canonical `c-1 < V` came from `c <= V`
    } else if vn_is_constant(data, in1) {
        (in1, -1, 1) // canonical `V < c+1` came from `V <= c`
    } else {
        return false; // shape changed since canonicalization; leave as-is
    };

    // val = vn->getOffset();
    let val: uintb = vn_offset(data, vn);
    let size: int4 = vn_size(data, vn);
    let code = op_code(data, op);
    let newcode = if code == OpCode::CPUI_INT_SLESS {
        // Mirror replaceLessequal's signed overflow guards in the inverse direction
        // if ((diff == -1) && (val == calc_int_min(vn->getSize()))) return false;
        if diff == -1 && val == calc_int_min(size) {
            return false;
        }
        // if ((diff ==  1) && (val == calc_int_max(vn->getSize()))) return false;
        if diff == 1 && val == calc_int_max(size) {
            return false;
        }
        // data.opSetOpcode(op,CPUI_INT_SLESSEQUAL);
        OpCode::CPUI_INT_SLESSEQUAL
    } else {
        // Unsigned overflow guards
        // if ((diff == -1) && (val == 0)) return false;
        if diff == -1 && val == 0 {
            return false;
        }
        // if ((diff ==  1) && (val == calc_uint_max(vn->getSize()))) return false;
        if diff == 1 && val == calc_uint_max(size) {
            return false;
        }
        // data.opSetOpcode(op,CPUI_INT_LESSEQUAL);
        OpCode::CPUI_INT_LESSEQUAL
    };
    // data.opSetOpcode(op,...);  // SEAM(W6): glb->inst[opc] (binary | booloutput)
    data.op_set_opcode(op, lessequal_type_op(newcode));

    // uintb res = (val + diff) & calc_mask(vn->getSize());
    let res: uintb = val.wrapping_add(diff as u64) & calc_mask(size);
    // Varnode *newvn = data.newConstant(vn->getSize(),res);
    let newvn = data.new_constant(size, res);
    // newvn->copySymbol(vn);  -- SEAM(W4): data-type / Symbol propagation (see module docs).
    // data.opSetInput(op,newvn,i);
    data.op_set_input(op, newvn, i).expect("restore_lessequal: opSetInput");
    // op->clearCanonicalLessequal();
    data.obank_mut().get_mut(op).expect("restore_lessequal: stale op").clear_canonical_lessequal();
    true
}

/// SEAM(W6): the `glb->inst[opc]` lookup for the restored LESSEQUAL ops, supplying
/// the verbatim `typeop.cc` opflags (`TypeOpIntLessEqual` / `TypeOpIntSlessEqual`
/// are `binary | booloutput`).  Mirrors `Funcdata::w6_type_op` (`funcdata_op.rs`).
fn lessequal_type_op(opc: OpCode) -> TypeOp {
    use pcodeop_flags as f;
    let (flags, name): (u32, &str) = match opc {
        // TypeOpIntLessEqual: binary | booloutput
        OpCode::CPUI_INT_LESSEQUAL => (f::binary | f::booloutput, "<="),
        // TypeOpIntSlessEqual: binary | booloutput
        OpCode::CPUI_INT_SLESSEQUAL => (f::binary | f::booloutput, "<="),
        // restore_lessequal installs only the two ops above.
        _ => (f::binary, "?"),
    };
    TypeOp::new(opc, flags, name.to_string())
}

/// (kuna GH-558) How canonicalized comparisons are presented
/// (the two values of `option compareform`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareForm {
    /// `original` (default): restore the source LESSEQUAL form for presentation
    /// (`glb->present_lessequal = true`; [`ActionPresentCompareForm`] runs).
    Original,
    /// `canonical`: leave the canonicalized `<`/`s<` form in place
    /// (`glb->present_lessequal = false`).
    Canonical,
}

impl CompareForm {
    /// The resolved `glb->present_lessequal` flag for this form
    /// (C++ `glb->present_lessequal = (p1 == "original")`).
    pub fn present_lessequal(self) -> bool {
        matches!(self, CompareForm::Original)
    }

    /// The `option compareform <p1>` token for this form.
    pub fn as_str(self) -> &'static str {
        match self {
            CompareForm::Original => "original",
            CompareForm::Canonical => "canonical",
        }
    }
}

/// Parse the `option compareform original|canonical` argument and produce the
/// resolved form plus the confirmation message (C++ `OptionCompareForm::apply`,
/// `kuna_compareform.cc:85`).
///
/// ```text
///   if (p1 == "original")       glb->present_lessequal = true;
///   else if (p1 == "canonical") glb->present_lessequal = false;
///   else throw ParseError("Must specify compareform as 'canonical' or 'original'");
///   return "Comparison presentation set to " + p1 + " form";
/// ```
///
/// The C++ `throw ParseError(...)` surfaces as a [`KunaError::parse`] (the
/// established option-parse error idiom shared with the sibling
/// [`parse_return_pair_form`](crate::kuna_returnpair::parse_return_pair_form) /
/// [`parse_memset_recover_form`](crate::kuna_memsetsequence::parse_memset_recover_form)).
/// The caller writes the resolved [`CompareForm::present_lessequal`] into
/// [`Architecture::present_lessequal`].
pub fn parse_compare_form(p1: &str) -> KunaResult<(CompareForm, String)> {
    // if (p1 == "original")       glb->present_lessequal = true;
    // else if (p1 == "canonical") glb->present_lessequal = false;
    // else throw ParseError("Must specify compareform as 'canonical' or 'original'");
    let form = if p1 == "original" {
        CompareForm::Original
    } else if p1 == "canonical" {
        CompareForm::Canonical
    } else {
        return Err(KunaError::parse(
            "Must specify compareform as 'canonical' or 'original'",
        ));
    };
    // glb->present_lessequal = ...;  -- left to the caller (Architecture::present_lessequal).
    // return "Comparison presentation set to " + p1 + " form";
    Ok((form, format!("Comparison presentation set to {p1} form")))
}

/// Per-file registration row (C++ `ActionPresentCompareForm`, placed after the
/// last canonicalization site and before prototype/cast/naming fixation).
///
/// Shipped default: `option compareform original` (`present_lessequal = true`;
/// kuna DIV-2 default-on, GH-558).  The consuming action registry is wired by a
/// later wave; this is the per-file export, the analogue of the rule pool's
/// `specs()`.
pub fn action(group: impl Into<String>) -> Box<dyn Action> {
    ActionPresentCompareForm::boxed(true, group)
}

// --- Local IR read helpers (see kuna_addcarrychain for the rationale) --------

fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

#[cfg(test)]
#[path = "kuna_compareform/tests.rs"]
mod tests;
