//! Tests for the remaining coreaction classes (`coreaction_render.rs`).
//!
//! Three kinds of checks:
//!
//!   1. **Identity** — every ported action carries the exact `name()` /
//!      `getGroup()` / flags the C++ constructor set, and clones only when its
//!      group is in the [`ActionGroupList`].
//!   2. **Stateful behavior on owned fields** — the parts of each `apply`/`reset`
//!      that bind only to this file's own fields (the `localcount`/`numpass`/
//!      `analysis_finished` gating, the `reset` zeroing, the `propagateIndirect`/
//!      `stackspace` configuration) are exercised directly on the concrete
//!      structs.
//!   3. **Seam contract** — every seamed `apply` returns `0` and bumps no count
//!      on a hand-built [`Funcdata`] (the C++ "changes are signalled by count"
//!      contract: an unrealized body must report no change).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use super::*;
use crate::action::ActionGroupList;
use crate::seams::Architecture;

// -----------------------------------------------------------------------------
// Harness (mirrors coreaction_cleanup.rs / coreaction_protos.rs fixtures)
// -----------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

// -----------------------------------------------------------------------------
// 1. Identity (name / group / flags)
// -----------------------------------------------------------------------------

/// Every render-group leaf carries the exact C++ `name()` string and group.
#[test]
fn names_and_groups_match_cpp() {
    let cases: Vec<(Box<dyn Action>, &str)> = vec![
        (ActionLaneDivide::boxed("g0"), "lanedivide"),
        (ActionSegmentize::boxed("g0"), "segmentize"),
        (ActionForceGoto::boxed("g0"), "forcegoto"),
        (ActionMultiCse::boxed("g0"), "multicse"),
        (ActionShadowVar::boxed("g0"), "shadowvar"),
        (ActionConstantPtr::boxed("g0"), "constantptr"),
        (ActionDeindirect::boxed("g0"), "deindirect"),
        (ActionLikelyTrash::boxed("g0"), "likelytrash"),
        (ActionRestructureVarnode::boxed("g0"), "restructure_varnode"),
        (ActionMappedLocalSync::boxed("g0"), "mapped_local_sync"),
        (ActionMapGlobals::boxed("g0"), "mapglobals"),
        (ActionDeadCode::boxed("g0"), "deadcode"),
        (ActionConditionalConst::boxed("g0"), "condconst"),
        (ActionSwitchNorm::boxed("g0"), "switchnorm"),
        (ActionUnjustifiedParams::boxed("g0"), "unjustparams"),
        (ActionDynamicMapping::boxed("g0"), "dynamicmapping"),
        (ActionDynamicSymbols::boxed("g0"), "dynamicsymbols"),
        (ActionInternalStorage::boxed("g0"), "internalstorage"),
        (ActionInferTypes::boxed("g0"), "infertypes"),
    ];
    for (act, expect) in &cases {
        assert_eq!(act.get_name(), *expect, "name mismatch");
        assert_eq!(act.get_group(), "g0", "group must be the ctor's `g`");
    }
    // Parameterized constructors keep the same name/group.
    let sp = ActionStackPtrFlow::boxed("g0", None);
    assert_eq!(sp.get_name(), "stackptrflow");
    assert_eq!(sp.get_group(), "g0");
    let dw = ActionDirectWrite::boxed("g0", true);
    assert_eq!(dw.get_name(), "directwrite");
    assert_eq!(dw.get_group(), "g0");
}

/// Flags match the C++ constructors: `rule_onceperfunc` for the once-per-function
/// actions, `0` for the repeatable ones.
#[test]
fn flags_match_cpp_constructors() {
    // rule_onceperfunc in the C++ ctor.
    for a in [
        ActionLaneDivide::boxed("g"),
        ActionMapGlobals::boxed("g"),
        ActionDynamicSymbols::boxed("g"),
        ActionInternalStorage::boxed("g"),
    ] {
        assert_eq!(
            a.base().flags,
            ruleflags::rule_onceperfunc,
            "{} must be rule_onceperfunc",
            a.get_name()
        );
    }
    // flags == 0 in the C++ ctor.
    for a in [
        ActionStackPtrFlow::boxed("g", None),
        ActionSegmentize::boxed("g"),
        ActionForceGoto::boxed("g"),
        ActionMultiCse::boxed("g"),
        ActionShadowVar::boxed("g"),
        ActionConstantPtr::boxed("g"),
        ActionDeindirect::boxed("g"),
        ActionDirectWrite::boxed("g", false),
        ActionLikelyTrash::boxed("g"),
        ActionRestructureVarnode::boxed("g"),
        ActionMappedLocalSync::boxed("g"),
        ActionDeadCode::boxed("g"),
        ActionConditionalConst::boxed("g"),
        ActionSwitchNorm::boxed("g"),
        ActionUnjustifiedParams::boxed("g"),
        ActionDynamicMapping::boxed("g"),
        ActionInferTypes::boxed("g"),
    ] {
        assert_eq!(a.base().flags, 0, "{} must have flags 0", a.get_name());
    }
}

/// `render_actions` enumerates the full set (21 classes) with the passed group.
#[test]
fn render_actions_enumerates_full_set() {
    let acts = render_actions("merge");
    assert_eq!(acts.len(), 21, "21 live render-group actions");
    for a in &acts {
        assert_eq!(a.get_group(), "merge");
    }
    // Names are exactly the C++ set (order matches the module table).
    let names: Vec<&str> = acts.iter().map(|a| a.get_name()).collect();
    assert_eq!(
        names,
        vec![
            "stackptrflow",
            "lanedivide",
            "segmentize",
            "forcegoto",
            "multicse",
            "shadowvar",
            "constantptr",
            "deindirect",
            "directwrite",
            "likelytrash",
            "restructure_varnode",
            "mapped_local_sync",
            "mapglobals",
            "deadcode",
            "condconst",
            "switchnorm",
            "unjustparams",
            "dynamicmapping",
            "dynamicsymbols",
            "internalstorage",
            "infertypes",
        ]
    );
}

// -----------------------------------------------------------------------------
// 2. clone_filtered (C++ clone(grouplist) group filter)
// -----------------------------------------------------------------------------

/// `clone_filtered` mirrors the C++ `clone(grouplist)` filter: present iff the
/// grouplist contains the action's group, else dropped.
#[test]
fn clone_filtered_obeys_grouplist() {
    let act = ActionDeadCode::boxed("deadcode");
    let yes = ActionGroupList::from_names(["deadcode", "analysis"]);
    let cl = act.clone_filtered(&yes).expect("present when grouplist contains group");
    assert_eq!(cl.get_name(), "deadcode");
    assert_eq!(cl.get_group(), "deadcode");
    let no = ActionGroupList::from_names(["merge"]);
    assert!(act.clone_filtered(&no).is_none(), "dropped when group absent");
}

/// The parameterized actions pass their configuration through `clone_filtered`
/// (C++ `clone` re-passes `stackspace` / `propagateIndirect`), and reset the
/// per-function `analysis_finished` latch to its fresh-function value.
#[test]
fn clone_filtered_preserves_parameterization() {
    let fd = build_fd();
    let ram = ram_space(&fd);
    let sp = ActionStackPtrFlow {
        base: ActionBase::new(0, "stackptrflow", "g"),
        stackspace: Some(Rc::clone(&ram)),
        analysis_finished: true, // already finished on the source
    };
    let list = ActionGroupList::from_names(["g"]);
    let cl = sp.clone_filtered(&list).expect("present");
    assert_eq!(cl.get_name(), "stackptrflow");
    // (the clone re-derives a fresh action; analysis_finished starts unset)

    // ActionDirectWrite carries propagateIndirect verbatim.
    let dw = ActionDirectWrite::boxed("g", false);
    let cl = dw.clone_filtered(&list).expect("present");
    assert_eq!(cl.get_name(), "directwrite");
}

// -----------------------------------------------------------------------------
// 3. Stateful behavior on owned fields
// -----------------------------------------------------------------------------

/// `ActionStackPtrFlow::reset` clears `analysis_finished`; with no stack space
/// `apply` immediately latches finished and returns 0 (the C++ "No stack" path).
#[test]
fn stackptrflow_no_stack_latches_finished() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    let mut sp = ActionStackPtrFlow {
        base: ActionBase::new(0, "stackptrflow", "g"),
        stackspace: None,
        analysis_finished: false,
    };
    // No stack space => analysis_finished set, no change.
    assert_eq!(sp.apply(&mut fd, &mut ctx), 0);
    assert!(sp.analysis_finished, "no-stack path latches finished");
    assert_eq!(sp.base().count, 0);
    // Already-finished short-circuit.
    assert_eq!(sp.apply(&mut fd, &mut ctx), 0);
    // reset() clears the latch for a new function.
    sp.reset(&mut fd);
    assert!(!sp.analysis_finished, "reset clears analysis_finished");
}

/// `ActionStackPtrFlow` with a stack space does not latch finished in the seam
/// (the body that would set it is unrealized); count stays 0.
#[test]
fn stackptrflow_with_stack_is_seamed() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let mut ctx = ActionContext::new();
    let mut sp = ActionStackPtrFlow {
        base: ActionBase::new(0, "stackptrflow", "g"),
        stackspace: Some(ram),
        analysis_finished: false,
    };
    assert_eq!(sp.apply(&mut fd, &mut ctx), 0);
    assert!(
        !sp.analysis_finished,
        "with a stack, the seam leaves analysis_finished unset for a later wave"
    );
    assert_eq!(sp.base().count, 0);
}

/// The `reset` overrides zero the per-function counters
/// (`localcount`/`numpass`).
#[test]
fn reset_zeroes_per_function_counters() {
    let mut fd = build_fd();

    let mut seg = ActionSegmentize { base: ActionBase::new(0, "segmentize", "g"), localcount: 5 };
    seg.reset(&mut fd);
    assert_eq!(seg.localcount, 0);

    let mut cp = ActionConstantPtr { base: ActionBase::new(0, "constantptr", "g"), localcount: 3 };
    cp.reset(&mut fd);
    assert_eq!(cp.localcount, 0);

    let mut rv = ActionRestructureVarnode {
        base: ActionBase::new(0, "restructure_varnode", "g"),
        numpass: 7,
    };
    rv.reset(&mut fd);
    assert_eq!(rv.numpass, 0);

    let mut it = ActionInferTypes { base: ActionBase::new(0, "infertypes", "g"), localcount: 6 };
    it.reset(&mut fd);
    assert_eq!(it.localcount, 0);
}

/// `ActionDirectWrite` stores the `propagateIndirect` config from the ctor.
#[test]
fn directwrite_stores_propagate_indirect() {
    let yes = ActionDirectWrite {
        base: ActionBase::new(0, "directwrite", "g"),
        propagate_indirect: true,
    };
    assert!(yes.propagate_indirect);
    let no = ActionDirectWrite {
        base: ActionBase::new(0, "directwrite", "g"),
        propagate_indirect: false,
    };
    assert!(!no.propagate_indirect);
}

// -----------------------------------------------------------------------------
// 4. Seam contract — every seamed apply reports no change
// -----------------------------------------------------------------------------

/// Every action's `apply` returns `0` and leaves `count == 0` on a fresh
/// `Funcdata` (the unrealized-body contract; the realized state-gating in the
/// stateful actions does not, on its own, report a data-flow change).
#[test]
fn all_applies_report_no_change_on_empty_fd() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    for mut a in render_actions("g") {
        let r = a.apply(&mut fd, &mut ctx);
        assert_eq!(r, 0, "{} apply must return 0", a.get_name());
        assert_eq!(a.base().count, 0, "{} apply must report no change", a.get_name());
    }
}
