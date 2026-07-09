//! Port of `decompiler/cpp/kuna_v850indbranch.{cc,hh}` — V850 "jmp [reg]"
//! indirect-branch reclassification (kuna GH-8817, S2 flow-classification).
//!
//! The V850 SLEIGH spec lifts `jmp [reg]` (reg != lp) to `call [reg]`, i.e.
//! `CPUI_CALLIND`, so a compiler switch dispatch never reaches jump-table
//! recovery (gated on `CPUI_BRANCHIND`).  When `option v850indirectbranch on`,
//! at flow-classification time such a CALLIND is reclassified to BRANCHIND.
//!
//! ## What this module ports
//!
//! [`kuna_is_v850_indirect_jmp`] is a faithful free-function transcription of
//! the C++ `kunaIsV850IndirectJmp(Funcdata&, const PcodeOp*)` predicate.  The
//! reclassification itself (the `opSetOpcode(op, CPUI_BRANCHIND)` rewrite) is
//! driven by `flow.rs` at `FlowInfo::xrefControlFlow`; this module only owns
//! the *decision*.
//!
//! ## STUB(W4): Rule/Action + ArchOption wrappers
//!
//! Two pieces of the C++ live on the W4 `Architecture` and are taken here as
//! resolved inputs / a closure rather than reached through `glb`:
//!
//!   - the **gate** `glb->v850_indirect_branch` (default \b false, shipped
//!     `option v850indirectbranch off` — see `kuna_stages.cc` settableTable):
//!     modelled by [`V850IndirectBranchOption`], whose `default()` is the
//!     shipped off-by-default flag.  `flow.rs::FlowEnv::is_v850_indirect_jmp`
//!     defaults to `false` and W4 will route it through this option + the
//!     predicate below.
//!   - the **register-name resolution** `glb->translate->getRegisterName(spc,
//!     off, size)` (W2 `Translate` is available, but it is reached through the
//!     W4 `glb`): supplied to the predicate as the already-resolved register
//!     name (`Option<&str>`, `None` == the C++ empty string, i.e. "not a named
//!     register").  W4 wires the real `getRegisterName` call.

use kuna_base::space::spacetype;

use crate::funcdata::Funcdata;
use crate::context::OpId;
use kuna_num::opcodes::OpCode;

/// (kuna) Toggle V850 "jmp [reg]" CALLIND -> BRANCHIND reclassification
/// (C++ `OptionV850IndirectBranch`, GH-8817).
///
/// STUB(W4): the C++ `OptionV850IndirectBranch::apply` flips
/// `glb->v850_indirect_branch`; here the flag is carried as a plain `bool` whose
/// [`Default`] is the *shipped* default (`option v850indirectbranch off`,
/// i.e. \b false — upstream byte-identical).  W4's `ArchOption` registry plugs
/// this into the option table; [`apply`](V850IndirectBranchOption::apply)
/// transcribes the `onOrOff(p1)` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct V850IndirectBranchOption {
    /// True if V850 "jmp [reg]" CALLIND is reclassified to BRANCHIND
    /// (C++ `Architecture::v850_indirect_branch`).
    pub enabled: bool,
}

impl Default for V850IndirectBranchOption {
    /// Shipped default: `option v850indirectbranch off` (upstream byte-identical;
    /// `architecture.cc:1433` sets `v850_indirect_branch = false`).
    fn default() -> Self {
        V850IndirectBranchOption { enabled: false }
    }
}

impl V850IndirectBranchOption {
    /// (kuna) Set the gate (C++ `OptionV850IndirectBranch::apply`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "V850 indirect-branch (jmp [reg]) reclassification turned on"
        } else {
            "V850 indirect-branch (jmp [reg]) reclassification turned off"
        }
    }

    /// Read the gate (C++ `glb->v850_indirect_branch`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// (kuna) Is `op` a V850 "jmp [reg]" CALLIND that should become a BRANCHIND?
/// (C++ `kunaIsV850IndirectJmp`, kuna_v850indbranch.cc:13-36, GH-8817).
///
/// `gate` is the resolved `glb->v850_indirect_branch` (see
/// [`V850IndirectBranchOption`]).  `register_name` is the already-resolved
/// `glb->translate->getRegisterName(...)` of `op`'s input-0 varnode: `None`
/// transcribes the C++ empty string ("not a named register").  // STUB(W4)
pub fn kuna_is_v850_indirect_jmp(
    data: &Funcdata,
    op: OpId,
    gate: bool,
    register_name: Option<&str>,
) -> bool {
    // default-off gate
    if !gate {
        return false;
    }
    let opref = match data.obank().get(op) {
        Some(o) => o,
        None => return false,
    };
    if opref.code() != OpCode::CPUI_CALLIND {
        return false;
    }
    let vn = match opref.get_in(0) {
        Some(v) => v,
        None => return false,
    };
    let vnref = match data.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    // a hardware register
    if vnref.get_space().get_type() != spacetype::IPTR_PROCESSOR {
        return false;
    }
    // only a named register (not raw memory)
    let regname = match register_name {
        Some(name) if !name.is_empty() => name,
        _ => return false,
    };
    // exclude the PC
    if regname == "PC" || regname == "pc" {
        return false;
    }
    true
}

#[cfg(test)]
#[path = "kuna_v850indbranch/tests.rs"]
mod tests;
