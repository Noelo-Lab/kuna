//! Port of `decompiler/cpp/kuna_booleanmask.{cc,hh}` — boolean
//! sign-extension-mask simplification (kuna GH-1282, S3 simplification-quiescence).
//!
//! `RuleBoolSignShift` folds `(b << k) s>> k` into the sign-extended-mask form
//! `0 - b` = `INT_2COMP(b)` (0 or all-ones) when the pre-shift value's NZMask
//! occupies only the low `(width*8 - k)` bits.  Gated default-on by the arch flag
//! `fold_boolean_mask` (option `booleanmask on|off`, shipped default `on`); inert
//! when off, byte-identical to upstream.
//!
//! ## Gate wiring — SEAM(W4)
//!
//! The C++ first line is `if (!data.getArch()->fold_boolean_mask) return 0;`.
//! The W3 `Funcdata::glb` is the *seam* [`Architecture`](crate::seams::Architecture),
//! which carries only the address-space manager and not the kuna analysis flags
//! (those live on the full `architecture.rs` [`Architecture`](crate::architecture::Architecture),
//! unreachable through the `&mut Funcdata` of the fixed [`Rule::apply_op`]
//! signature).  Following the established convention for these gated kuna modules
//! (`kuna_v850indbranch`, `kuna_sparcstructret`, `kuna_switchmodbound`), the gate
//! is carried as a resolved `enabled: bool` on the rule struct, set at
//! construction time from the architecture flag / the merged `kuna_phases`
//! [`OptionValues`](crate::kuna_phases::OptionValues).  W8 threads the live arch
//! flag through [`RuleBoolSignShift::new`]; [`specs`] builds the rule with the
//! shipped default (`on`).
//!
//! ## SEAM(W6) — opcode-flag resolution
//!
//! The rewrite calls `opSetOpcode(op, CPUI_INT_2COMP)`, which the C++ resolves to
//! `glb->inst[CPUI_INT_2COMP]` (the interned [`TypeOp`] carrying the op-code's
//! property-flag word).  That `inst` table is the W6 type/typeop subsystem; until
//! it lands the rewrite builds the [`TypeOp`] skeleton directly
//! ([`TypeOp::new`]) with a zero flag word.  `code()` reads the op-code itself
//! (correct), but the cached property flags are the W6 seam — noted in the
//! structured losses.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, TypeOp, VarnodeId};

/// (kuna GH-1282) Fold `(b << k) s>> k` into the sign-extended-mask form `0 - b`
/// (C++ `RuleBoolSignShift`).
///
/// `enabled` is the resolved `Architecture::fold_boolean_mask` gate — see the
/// module docs (SEAM(W4)).
pub struct RuleBoolSignShift {
    /// Resolved `glb->fold_boolean_mask` gate (the C++ `data.getArch()->...`).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`, set by the `Rule(g,0,"boolsignshift")`
    /// constructor).
    group: String,
}

impl RuleBoolSignShift {
    /// Construct with the resolved gate (C++ `RuleBoolSignShift(const string &g)`
    /// plus the `data.getArch()->fold_boolean_mask` read deferred to construction
    /// — see module docs).  The default group is `"analysis"` (the C++ registers
    /// `new RuleBoolSignShift("analysis")` in oppool1 — `coreaction.cc:5811`).
    pub fn new(enabled: bool) -> RuleBoolSignShift {
        RuleBoolSignShift { enabled, group: String::from("analysis") }
    }

    /// Construct with an explicit group string.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleBoolSignShift {
        RuleBoolSignShift { enabled, group: group.into() }
    }
}

impl Rule for RuleBoolSignShift {
    /// C++ `RuleBoolSignShift::getOpList`: `oplist.push_back(CPUI_INT_SRIGHT);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT]
    }

    /// C++ `RuleBoolSignShift::clone(grouplist)`:
    /// `if (!grouplist.contains(getGroup())) return 0; return new ...(getGroup());`
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleBoolSignShift { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleBoolSignShift::applyOp` (`kuna_booleanmask.cc:19`) — transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->fold_boolean_mask) return 0;  // (kuna) default gate
        // The live gate is carried on the seam Architecture (`build_arch_handle`);
        // `enabled` stays as the unit-test OR-override (rule registered false).
        if !self.enabled && !data.get_arch().fold_boolean_mask {
            return 0;
        }

        // if (!op->getIn(1)->isConstant()) return 0;
        let in1 = match op_in(data, op, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, in1) {
            return 0;
        }
        // uintb sa = op->getIn(1)->getOffset();
        let sa: uintb = vn_offset(data, in1);
        // if (sa == 0) return 0;
        if sa == 0 {
            return 0;
        }

        // Varnode *shiftin = op->getIn(0);
        let shiftin = match op_in(data, op, 0) {
            Some(v) => v,
            None => return 0,
        };
        // if (!shiftin->isWritten()) return 0;
        if !vn_is_written(data, shiftin) {
            return 0;
        }
        // PcodeOp *leftshift = shiftin->getDef();
        let leftshift = match vn_def(data, shiftin) {
            Some(d) => d,
            None => return 0,
        };
        // if (leftshift->code() != CPUI_INT_LEFT) return 0;
        if op_code(data, leftshift) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        // if (!leftshift->getIn(1)->isConstant()) return 0;
        let ls1 = match op_in(data, leftshift, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, ls1) {
            return 0;
        }
        // if (leftshift->getIn(1)->getOffset() != sa) return 0;  // same shift amount
        if vn_offset(data, ls1) != sa {
            return 0;
        }

        // RuleLeftRight already handles the byte-aligned case; defer to it there.
        // if ((sa & 7) == 0) return 0;
        if (sa & 7) == 0 {
            return 0;
        }

        // Varnode *boolvn = leftshift->getIn(0);
        let boolvn = match op_in(data, leftshift, 0) {
            Some(v) => v,
            None => return 0,
        };
        // int4 width = boolvn->getSize();
        let width: int4 = vn_size(data, boolvn);
        // The value shifted up must occupy only the low (width*8 - sa) bits.
        // if ((uintb)width * 8 <= sa) return 0;
        if (width as uintb).wrapping_mul(8) <= sa {
            return 0;
        }
        // uintb tophalf = ((uintb)1 << ((uintb)width * 8 - sa)) - 1;
        let tophalf: uintb = (1u64 << ((width as uintb).wrapping_mul(8) - sa)).wrapping_sub(1);
        // if ((boolvn->getNZMask() & ~tophalf) != 0) return 0;
        if (vn_nzmask(data, boolvn) & !tophalf) != 0 {
            return 0;
        }

        // if (shiftin->loneDescend() != op) return 0;
        if data.lone_descend(shiftin) != Some(op) {
            return 0;
        }

        // Rewrite `(b << sa) s>> sa`  =>  `0 - b`  (INT_2COMP gives 0 or all-ones).
        // data.opSetOpcode(op,CPUI_INT_2COMP);
        // SEAM(W6): glb->inst[CPUI_INT_2COMP] property flags (see module docs).
        data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_2COMP, 0, "INT_2COMP"));
        // data.opSetInput(op,boolvn,0);
        data.op_set_input(op, boolvn, 0)
            .expect("RuleBoolSignShift: opSetInput on a live op (internal invariant)");
        // data.opRemoveInput(op,1);  // drop the shift-amount constant
        data.op_remove_input(op, 1);
        1
    }
}

/// Per-file registration rows in C++ definition order (W8 assembles
/// `universalAction` from these).  This file defines exactly one rule.
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        // C++ `new RuleBoolSignShift("analysis")` in oppool1 (coreaction.cc:5811).
        group: "analysis",
        // Shipped default: `option booleanmask on` (architecture.cc resets
        // fold_boolean_mask = true; kuna DIV-2 default-on, GH-1282).
        ctor: || Box::new(RuleBoolSignShift::with_group(true, "analysis")),
    }]
}

// ---------------------------------------------------------------------------
// Local IR read helpers (the C++ `vn->isConstant()` / `op->getIn(i)` cascade).
// These mirror the established `kuna_v850indbranch.rs` access pattern through the
// op/varnode banks.
// ---------------------------------------------------------------------------

/// `op->getIn(slot)` (C++).
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}

/// `op->code()` (C++).
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}

/// `vn->isConstant()` (C++).
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}

/// `vn->isWritten()` (C++).
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
}

/// `vn->getDef()` (C++).
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).and_then(|v| v.get_def())
}

/// `vn->getOffset()` (C++).
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}

/// `vn->getSize()` (C++).
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

/// `vn->getNZMask()` (C++).
fn vn_nzmask(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_nz_mask()).unwrap_or(0)
}

#[cfg(test)]
#[path = "kuna_booleanmask/tests.rs"]
mod tests;
