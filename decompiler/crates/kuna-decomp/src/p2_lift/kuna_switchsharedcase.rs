//! (kuna) Loop-carried-base PIC jump-table option — `option switchsharedcase`
//! (angr port: `test_switch_case_shared_case_nodes_b2sum_digest`; S2 switch-model).
//!
//! Third sibling of [`kuna_switchmodbound`](crate::p2_lift::kuna_switchmodbound)
//! and [`kuna_switchguardbound`](crate::p2_lift::kuna_switchguardbound).  All
//! three address the same surface failure — the basic `JumpBasic` model cannot
//! bound a LOAD-table index, `JumpTable::recoverAddresses` aborts with *"Too many
//! branches"*, and `FlowInfo::truncateIndirectJump` rewrites the `BRANCHIND` into
//! a `CALLIND` (`/* WARNING: Treating indirect jump as call */`), collapsing the
//! `switch` (and the loop around it) into goto spaghetti.
//!
//! ## The shape this option covers
//!
//! A textbook GCC PIC **relative-offset** jump table —
//! `target = base + sext32(load4(base + idx*4))` — whose table base register
//! (`lea .rodata, %rbp`) is established **before** an enclosing loop (a
//! `getopt_long` option-dispatch `while`) while the `BRANCHIND` is **inside** it.
//! The base therefore reaches the indirect jump through a loop-header
//! `MULTIEQUAL` (a phi).  Because the same base phi feeds *both* the table-load
//! address (`base + idx*4`) and the final `base + offset` add, `JumpBasic::
//! findDeterminingVarnodes` melds the two paths down to a single common varnode
//! (the final 8-byte `INT_ADD` result).  The normalized index (`getopt result
//! - 0x62`, the `sub $0x62`) never enters the meld, so the out-of-band CBRANCH
//! range guard (`cmp $0x22; ja DEFAULT`, i.e. `idx <=u 0x22`) cannot bound it —
//! `findSmallestNormal` leaves the index range unbounded, no model is built, and
//! the branch downgrades to a call.
//!
//! This is distinct from `switchguardbound` (where the index *is* a meld varnode
//! and only the guard's flag form / a stack spill hides the bound): here the
//! index is not in the meld **at all** because the path collapsed, so the
//! recovery has to *rebuild* the meld as a clean single path down to the guarded
//! load index before the existing model machinery can size the table.
//!
//! ## What this module owns vs. the recovery walk
//!
//! As with its two siblings, the assigned scope here is **only** the `ElementId`
//! and the `ArchOption` that flips the arch flag — *not* the recovery logic.  The
//! recovery walk `JumpBasic::kuna_try_loop_carried_guard_table(Funcdata*, uint4
//! maxtablesize)` is a member of `JumpBasic` and lives next to the rest of the
//! model in `jumptable.rs` (it needs the protected `pathMeld`/`jrange`/
//! `selectguards`/`varnodeIndex` state).  It is invoked from `recoverModel()`
//! only after the normal model — and the modulo/and-mask and CBRANCH-guard
//! extensions — fail to bound the table.  Only the gate
//! (`Architecture::switch_shared_case`, default off) is owned here.
//!
//! The option ships **default-off** (upstream byte-identical): rebuilding the
//! meld and reading the relative-offset table at a base recovered heuristically
//! across a loop phi is a control-flow heuristic that asserts the guarded value
//! is the load index over a memory round-trip it cannot prove in dataflow, so a
//! loose match could mis-size or over-accept an unrelated indirect jump inside a
//! loop.  Flip it per program (`option switchsharedcase on`) when a getopt/
//! dispatch switch inside a loop renders as a computed call with *"Could not
//! recover jumptable … Too many branches"*.

use kuna_base::marshal::ElementId;

/// Marshaling element `<switchsharedcase>` (kuna). ElementIds live in the 4000+
/// range (C++ `ELEM_SWITCHSHAREDCASE = ElementId("switchsharedcase",4106)`;
/// renumbered from 4103 at the merge into main — main now uses 4103
/// `noreturn_extern`, 4104 `noreturn_externmatch`, 4105 `crossjumprevert`, so the
/// next free id above `regionstructure`'s 4102 is 4106).
pub const ELEM_SWITCHSHAREDCASE: ElementId = ElementId::new("switchsharedcase", 4106);

/// (kuna, angr `test_switch_case_shared_case_nodes_b2sum_digest`) Toggle
/// recovery of a GCC PIC relative-offset jump table whose base register is a
/// loop-carried MULTIEQUAL (C++ `OptionSwitchSharedCase`).
///
/// The C++ `OptionSwitchSharedCase::apply` flips `glb->switch_shared_case`; here
/// the flag is carried as a plain `bool` whose [`Default`] is the *shipped*
/// default (`option switchsharedcase off`, i.e. \b false — upstream
/// byte-identical).  `JumpBasic::recoverModel` reads this gate before calling
/// `kuna_try_loop_carried_guard_table`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchSharedCaseOption {
    /// True if a loop-carried-base PIC relative-offset jump table is recovered by
    /// rebuilding the meld to its guarded load index (C++
    /// `Architecture::switch_shared_case`).
    pub enabled: bool,
}

impl Default for SwitchSharedCaseOption {
    /// Shipped default: `option switchsharedcase off` (upstream byte-identical).
    fn default() -> Self {
        SwitchSharedCaseOption { enabled: false }
    }
}

impl SwitchSharedCaseOption {
    /// (kuna) Set the gate (C++ `OptionSwitchSharedCase::apply`: `bool val =
    /// onOrOff(p1); glb->switch_shared_case = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Loop-carried-base PIC jump-table recovery turned on"
        } else {
            "Loop-carried-base PIC jump-table recovery turned off"
        }
    }

    /// Read the gate (C++ `glb->switch_shared_case`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_switchsharedcase/tests.rs"]
mod tests;
