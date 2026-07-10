//! Adversarial verification tests for item `w6-s4-coreaction-protos`
//! (the S4 prototype-recovery `Action` classes from
//! `decompiler/cpp/coreaction.{cc,hh}`).
//!
//! Written by the INDEPENDENT verifier.
//!
//! **W10 update (`rport/w10-proto-recovery`).**  Two of these bodies are no
//! longer inert: `ActionPrototypeTypes` now selects the model + strips RETURN-in0
//! + calls `Funcdata::init_active_output` (the real proto-recovery init), and
//! `ActionReturnRecovery` runs the active-output trial machinery.  On the
//! hand-built fixture here (a `context::ArchContext` with *no* registered proto
//! model and *no* RETURN ops) `ActionPrototypeTypes` still has one real effect —
//! it initializes the active-output container (the C++ `initActiveOutput`, which
//! always runs when the output is unlocked) — so it bumps `count`.  The remaining
//! call-spec-gated bodies (`qlst` absent) stay inert.  The hunt list therefore
//! concentrates on:
//!
//!   - **Change-signal totality.** Every `apply` MUST leave `count == 0` and
//!     return `0`.  A STUB that spuriously bumps `count` would falsely signal a
//!     change to the `rule_repeatapply` fixpoint driver (silent infinite-loop /
//!     extra-pass corruption).  The most fragile case is `ActionExtraPopSetup`
//!     with a **non-null** stack space (`Some(idx)`): it falls *through* the one
//!     realized control-path (the `stackspace == 0` early return) into the
//!     deferred per-call insertion, which must still be inert.
//!   - **`clone_filtered` totality + type/flags fidelity.** C++ `clone(grouplist)`
//!     returns null out-of-group and a same-class, same-name/group/flags object
//!     in-group.  A copy-paste slip (cloning the wrong type, or dropping the
//!     flags word) is silent.
//!   - **Flags-by-name provenance.** The C++ constructors pin each `name()` to a
//!     specific flags word (`rule_onceperfunc` vs `0`); `ActionParamDouble` /
//!     `ActionActiveParam` / `ActionActiveReturn` / `ActionReturnRecovery` /
//!     `ActionRestrictLocal` are the `0`-flag minority and the easiest to get
//!     wrong.
//!
//! These use ONLY the public (`boxed`) constructors and the `Action` trait, so
//! they exercise exactly the surface W8 will consume.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};

use kuna_decomp::action::{ruleflags, Action, ActionContext, ActionGroupList};
use kuna_decomp::coreaction_protos::{
    proto_actions, ActionActiveParam, ActionActiveReturn, ActionDefaultParams,
    ActionExtraPopSetup, ActionFuncLink, ActionFuncLinkOutOnly, ActionInputPrototype,
    ActionOutputPrototype, ActionParamDouble, ActionPrototypeTypes, ActionPrototypeWarnings,
    ActionRestrictLocal, ActionReturnRecovery,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::ArchContext;

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
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

/// Every owned action, each constructed in a fixed group.  `ExtraPopSetup` is
/// listed twice on purpose: once with a NULL stack space (`None`, the realized
/// early-return) and once with a NON-null stack space (`Some`, which falls
/// through into the deferred per-call seam).
fn all_actions(g: &str) -> Vec<Box<dyn Action>> {
    vec![
        ActionPrototypeTypes::boxed(g),
        ActionDefaultParams::boxed(g),
        ActionExtraPopSetup::boxed(g, None),
        ActionExtraPopSetup::boxed(g, Some(2)),
        ActionFuncLink::boxed(g),
        ActionFuncLinkOutOnly::boxed(g),
        ActionParamDouble::boxed(g),
        ActionActiveParam::boxed(g),
        ActionActiveReturn::boxed(g),
        ActionReturnRecovery::boxed(g),
        ActionRestrictLocal::boxed(g),
        ActionInputPrototype::boxed(g),
        ActionOutputPrototype::boxed(g),
        ActionPrototypeWarnings::boxed(g),
    ]
}

/// HUNT: change-signal totality / `apply` return contract.  Every proto action
/// must return `0` from `apply` (the C++ `return 0`; change signalling is via
/// `count`).  The call-spec-gated bodies (`qlst` absent) AND the output-recovery
/// bodies on this model-less, RETURN-less fixture must additionally leave
/// `count == 0` — EXCEPT `ActionPrototypeTypes`, whose `init_active_output`
/// (C++ `initActiveOutput`, always run when the output is unlocked) is a real
/// effect that bumps `count`.  This pins the realized-vs-seam boundary so a
/// future change that makes a different body active trips here.
#[test]
fn proto_action_apply_change_signal_contract() {
    let mut ctx = ActionContext::new();
    for mut act in all_actions("protorecovery") {
        // Fresh fixture per action so prototypetypes' init_active_output does not
        // leak into the next action's view.
        let mut data = build_fd();
        let name = act.get_name().to_string();
        let before = act.base().count;
        assert_eq!(before, 0, "{name}: fresh action must start with count 0");
        let res: int4 = act.apply(&mut data, &mut ctx);
        assert_eq!(res, 0, "{name}: apply must return 0 (change signalling is via count)");
        if name == "prototypetypes" {
            // The realized init_active_output bumps count exactly once (one
            // effect: the active-output container was created).
            assert_eq!(
                act.base().count,
                1,
                "prototypetypes: init_active_output is a real effect (count == 1)"
            );
            assert!(
                data.get_active_output().is_some(),
                "prototypetypes: init_active_output must leave an active-output container"
            );
        } else {
            assert_eq!(
                act.base().count,
                0,
                "{name}: call-spec-gated body must not bump the change counter on this fixture"
            );
        }
    }
    // The warning channel IS realized; PrototypeWarnings must emit nothing with
    // the model-less proto (no errors to report).
    assert!(
        ctx.warnings.messages.is_empty(),
        "no proto-recovery body may emit a warning on the model-less fixture"
    );
}

/// HUNT: idempotence of the STUB across repeated application.  `rule_onceperfunc`
/// actions can be re-applied by the driver; a STUB must stay at count 0 no
/// matter how many times it runs (no hidden accumulation/state).
#[test]
fn repeated_apply_stays_inert() {
    let mut data = build_fd();
    let mut ctx = ActionContext::new();
    let mut act = ActionExtraPopSetup::boxed("protorecovery", Some(2));
    for _ in 0..5 {
        let res: int4 = act.apply(&mut data, &mut ctx);
        assert_eq!(res, 0);
        assert_eq!(act.base().count, 0);
    }
}

/// HUNT: `clone_filtered` totality + type/flags fidelity.  For every action:
/// in-group clone yields a same-name, same-group, same-flags object; out-of-group
/// clone is `None` (C++ `clone` returning `(Action *)0`).
#[test]
fn clone_filtered_total_and_preserves_identity() {
    let in_group = ActionGroupList::from_names(["protorecovery"]);
    let out_group = ActionGroupList::from_names(["someothergroup"]);

    for act in all_actions("protorecovery") {
        let name = act.get_name().to_string();
        let flags = act.base().flags;

        let cloned = act
            .clone_filtered(&in_group)
            .unwrap_or_else(|| panic!("{name}: in-group clone must succeed"));
        assert_eq!(cloned.get_name(), name, "{name}: clone keeps name");
        assert_eq!(
            cloned.get_group(),
            "protorecovery",
            "{name}: clone keeps group"
        );
        assert_eq!(cloned.base().flags, flags, "{name}: clone keeps flags word");

        // A fresh same-class action built in a group that is NOT in the
        // grouplist must filter out.
        let outsider = clone_fresh(&name);
        assert!(
            outsider.clone_filtered(&out_group).is_none(),
            "{name}: out-of-group clone must be None"
        );
    }
}

/// Rebuild a fresh boxed action of the given C++ name in group "notlisted".
fn clone_fresh(name: &str) -> Box<dyn Action> {
    let g = "notlisted";
    match name {
        "prototypetypes" => ActionPrototypeTypes::boxed(g),
        "defaultparams" => ActionDefaultParams::boxed(g),
        "extrapopsetup" => ActionExtraPopSetup::boxed(g, Some(2)),
        "funclink" => ActionFuncLink::boxed(g),
        "funclink_outonly" => ActionFuncLinkOutOnly::boxed(g),
        "paramdouble" => ActionParamDouble::boxed(g),
        "activeparam" => ActionActiveParam::boxed(g),
        "activereturn" => ActionActiveReturn::boxed(g),
        "returnrecovery" => ActionReturnRecovery::boxed(g),
        "restrictlocal" => ActionRestrictLocal::boxed(g),
        "inputprototype" => ActionInputPrototype::boxed(g),
        "outputprototype" => ActionOutputPrototype::boxed(g),
        "prototypewarnings" => ActionPrototypeWarnings::boxed(g),
        other => panic!("unexpected action name {other}"),
    }
}

/// HUNT: flags-by-name provenance against the exact C++ constructors
/// (`coreaction.hh`).  Pins which actions are `rule_onceperfunc` and which are
/// `0` — a swap is silent.  `proto_actions` (the W8 enumeration helper) is the
/// vehicle, so this also pins the helper's per-entry flags.
#[test]
fn proto_actions_flags_match_cpp_constructors() {
    // name -> exact C++ flags word.
    let expect: &[(&str, uint4)] = &[
        ("prototypetypes", ruleflags::rule_onceperfunc),
        ("defaultparams", ruleflags::rule_onceperfunc),
        ("funclink", ruleflags::rule_onceperfunc),
        ("funclink_outonly", ruleflags::rule_onceperfunc),
        ("paramdouble", 0),
        ("activeparam", 0),
        ("activereturn", 0),
        ("returnrecovery", 0),
        ("restrictlocal", 0),
        ("inputprototype", ruleflags::rule_onceperfunc),
        ("outputprototype", ruleflags::rule_onceperfunc),
        ("prototypewarnings", ruleflags::rule_onceperfunc),
    ];
    let acts = proto_actions("g");
    assert_eq!(
        acts.len(),
        expect.len(),
        "proto_actions count must match the pinned set (ExtraPopSetup is excluded)"
    );
    for (act, (name, flags)) in acts.iter().zip(expect.iter()) {
        assert_eq!(act.get_name(), *name, "proto_actions order/name drift");
        assert_eq!(
            act.base().flags,
            *flags,
            "{name}: flags word diverged from the C++ constructor"
        );
    }
    // ExtraPopSetup is deliberately NOT in proto_actions (needs a stack space at
    // build time); confirm it stays out so a W8 assembler can't double-insert it.
    assert!(
        !acts.iter().any(|a| a.get_name() == "extrapopsetup"),
        "extrapopsetup must be excluded from proto_actions"
    );
}

/// HUNT: the one realized vs deferred boundary in `ExtraPopSetup::apply`.
/// C++: `if (stackspace == (AddrSpace *)0) return 0;`.  Both the null path
/// (realized early-return) and the non-null path (fall-through to the deferred
/// per-call insertion) are inert TODAY; this documents the boundary so the
/// future W8 wiring that makes `Some(idx)` actually insert INT_ADD/INDIRECT ops
/// will trip this test and force a re-verify.
#[test]
fn extrapop_null_and_nonnull_both_inert_for_now() {
    let mut data = build_fd();
    let mut ctx = ActionContext::new();

    let mut null_space = ActionExtraPopSetup::boxed("protorecovery", None);
    assert_eq!(null_space.apply(&mut data, &mut ctx), 0);
    assert_eq!(null_space.base().count, 0);

    let mut some_space = ActionExtraPopSetup::boxed("protorecovery", Some(2));
    assert_eq!(some_space.apply(&mut data, &mut ctx), 0);
    assert_eq!(
        some_space.base().count,
        0,
        "non-null stackspace currently still routes through the STUB (no call list yet)"
    );
}
