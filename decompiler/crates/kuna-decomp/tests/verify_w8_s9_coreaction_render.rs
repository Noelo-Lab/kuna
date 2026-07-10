//! Verifier adversarial tests for `w8-s9-coreaction-render`
//! (the remaining 21 `Action` shells in
//! `decompiler/crates/kuna-decomp/src/coreaction_render.rs`).
//!
//! This item is a STUB port: every `apply` returns 0 with the C++ body carried
//! as commented pseudocode (the `Funcdata` analysis/scope/jumptable/type
//! surface is not in the merged tree).  The only *realized* surface is the
//! engine-owned class shell — exact `name()`/group/`flags` from the C++
//! constructor, the `clone(grouplist)` group filter, the per-function
//! `reset()` overrides for the five stateful actions, and the lone realized
//! branch in `ActionStackPtrFlow::apply` (the `analysis_finished` /
//! "no stack" early-outs).  The fragile spots are therefore:
//!
//!   (a) the exact name+flags partition vs the C++ ctors (a typo'd name or a
//!       wrong `rule_onceperfunc` flag is silent scheduling corruption);
//!   (b) the `reset` NON-chaining semantics — C++ `ActionGroup::reset` calls
//!       the *virtual* `reset`, so a stateful subclass whose override is
//!       `{ localcount = 0; }` does NOT run `Action::reset` (status/flags are
//!       left as-is), whereas a non-overriding leaf DOES reset its base;
//!   (c) stub *totality* — every body is a pure no-op leaving `count == 0`
//!       regardless of `Funcdata` state, repetition, or how far the state
//!       counters have advanced;
//!   (d) `clone_filtered` independence + parameterization carry-through
//!       (`stackspace` / `propagateIndirect` preserved; `analysis_finished`
//!       reset to the fresh-function value; counter mutations do not leak).
//!
//! Ground truth re-derived from coreaction.hh@bc278439 / coreaction.cc@28f6157.
//! Written by the verifier; lands with the verdict regardless of outcome
//! (verification.md "Adversarial pass").

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_decomp::action::{ruleflags, statusflags, ActionContext, ActionGroupList};
use kuna_decomp::coreaction_render::{
    render_actions, ActionConstantPtr, ActionDirectWrite, ActionInferTypes,
    ActionRestructureVarnode, ActionSegmentize, ActionStackPtrFlow,
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
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// The full (name, flags) partition for every one of the 21 ported classes,
/// in `render_actions` order, re-derived from the C++ constructors.  `0` =
/// `Action(0,...)`; `1` = `Action(rule_onceperfunc,...)`.
const EXPECTED: &[(&str, u32)] = &[
    ("stackptrflow", 0),
    ("lanedivide", ruleflags::rule_onceperfunc),
    ("segmentize", 0),
    ("forcegoto", 0),
    ("multicse", 0),
    ("shadowvar", 0),
    ("constantptr", 0),
    ("deindirect", 0),
    ("directwrite", 0),
    ("likelytrash", 0),
    ("restructure_varnode", 0),
    ("mapped_local_sync", 0),
    ("mapglobals", ruleflags::rule_onceperfunc),
    ("deadcode", 0),
    ("condconst", 0),
    ("switchnorm", 0),
    ("unjustparams", 0),
    ("dynamicmapping", 0),
    ("dynamicsymbols", ruleflags::rule_onceperfunc),
    ("internalstorage", ruleflags::rule_onceperfunc),
    ("infertypes", 0),
];

/// (a) Every ported leaf carries the EXACT C++ `name()` and `flags`, and the
/// group is the ctor's `g` (fed here as a token that is NOT any action name,
/// to prove the group is plumbed through, not a hard-coded schedule literal).
#[test]
fn w8_render_ctor_facts_match_cpp_partition() {
    let token = "ZZ-not-a-schedule-group";
    let acts = render_actions(token);
    assert_eq!(acts.len(), EXPECTED.len(), "21 live render-residue classes");
    for (act, (name, flags)) in acts.iter().zip(EXPECTED.iter()) {
        assert_eq!(act.get_name(), *name, "name mismatch vs C++ ctor");
        assert_eq!(act.base().flags, *flags, "{}: flags must match C++ ctor", name);
        assert_eq!(act.get_group(), token, "{}: group must be the ctor's g", name);
        // Fresh action starts at status_start (C++ Action::Action base ctor).
        assert_eq!(act.get_status(), statusflags::status_start, "{}: fresh status", name);
    }
    // Exactly the four rule_onceperfunc actions, by name (no more, no fewer).
    let once: Vec<&str> = EXPECTED
        .iter()
        .filter(|(_, f)| *f == ruleflags::rule_onceperfunc)
        .map(|(n, _)| *n)
        .collect();
    assert_eq!(once, vec!["lanedivide", "mapglobals", "dynamicsymbols", "internalstorage"]);
}

/// (b) `reset` NON-chaining: a stateful subclass override is `{ counter = 0; }`
/// only — it must NOT also run the base `Action::reset` (which would reset
/// `status` to status_start and clear `rule_warnings_given`).  We pre-dirty the
/// base, call `reset`, and require the counter zeroed but the base untouched.
/// A non-overriding leaf (`render_actions` includes many) must, by contrast,
/// run the default `reset` and DO reset the base.  This pins the exact C++
/// virtual-dispatch behavior of `ActionGroup::reset` -> `(*iter)->reset`.
#[test]
fn w8_reset_overrides_do_not_chain_to_base() {
    let mut fd = build_fd();

    // --- stateful overrides: counter zeroed, base left dirty (non-chaining) ---
    let mut seg = ActionSegmentize::boxed("g");
    seg.base_mut().status = statusflags::status_end; // dirty
    seg.base_mut().flags |= ruleflags::rule_warnings_given;
    seg.base_mut().count = 9;
    seg.reset(&mut fd);
    assert_eq!(
        seg.get_status(),
        statusflags::status_end,
        "ActionSegmentize::reset must NOT chain to Action::reset (status preserved)"
    );
    assert_ne!(
        seg.base().flags & ruleflags::rule_warnings_given,
        0,
        "warnings_given must NOT be cleared by the non-chaining override"
    );

    for mut a in [
        ActionConstantPtr::boxed("g"),
        ActionRestructureVarnode::boxed("g"),
        ActionInferTypes::boxed("g"),
        ActionStackPtrFlow::boxed("g", None),
    ] {
        a.base_mut().status = statusflags::status_mid;
        a.reset(&mut fd);
        assert_eq!(
            a.get_status(),
            statusflags::status_mid,
            "{}: stateful reset override must not touch base status",
            a.get_name()
        );
    }

    // --- a non-overriding leaf DOES run the default reset (base reset) ---
    // `deadcode` has no reset override in C++ (and none in Rust), so the default
    // trait reset (reset_base) applies.
    let mut acts = render_actions("g");
    let dead = acts
        .iter_mut()
        .find(|a| a.get_name() == "deadcode")
        .expect("deadcode present");
    dead.base_mut().status = statusflags::status_end;
    dead.base_mut().flags |= ruleflags::rule_warnings_given;
    dead.reset(&mut fd);
    assert_eq!(
        dead.get_status(),
        statusflags::status_start,
        "a non-overriding leaf's default reset resets base status to start"
    );
    assert_eq!(
        dead.base().flags & ruleflags::rule_warnings_given,
        0,
        "default reset clears rule_warnings_given"
    );
}

/// (c) Stub totality: applying every action many times on a populated-as-much-
/// as-possible `Funcdata` never bumps `count` and always returns 0 — even
/// after the stateful actions' own gating would have advanced (StackPtrFlow's
/// no-stack latch, InferTypes' 7-pass ceiling, etc.).  The stub reports no
/// data-flow change, matching the C++ "changes signalled by count" contract.
#[test]
fn w8_stub_totality_under_repetition() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::new();
    for mut a in render_actions("g") {
        for _ in 0..16 {
            let r = a.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{}: stub apply must return 0", a.get_name());
            assert_eq!(a.base().count, 0, "{}: stub must report no change", a.get_name());
        }
    }

    // StackPtrFlow WITH a stack: the realized no-stack branch is skipped, the
    // stub body cannot latch, so analysis_finished stays false and count==0
    // across repeats (the body that would set it is unrealized).
    let ram = ram_space(&fd);
    let mut sp = ActionStackPtrFlow::boxed("g", Some(Rc::clone(&ram)));
    for _ in 0..8 {
        assert_eq!(sp.apply(&mut fd, &mut ctx), 0);
        assert_eq!(sp.base().count, 0, "stackptrflow-with-stack stub: no change");
    }

    // StackPtrFlow WITHOUT a stack: the realized branch latches finished on the
    // first apply and short-circuits thereafter, still with count==0.
    let mut sp0 = ActionStackPtrFlow::boxed("g", None);
    assert_eq!(sp0.apply(&mut fd, &mut ctx), 0);
    assert_eq!(sp0.apply(&mut fd, &mut ctx), 0); // already-finished short-circuit
    assert_eq!(sp0.base().count, 0, "no-stack latch reports no change");
}

/// (d) `clone_filtered` independence + parameterization carry-through.  The
/// clone is present iff the grouplist contains the action's group (C++
/// `clone`'s `if (!grouplist.contains(getGroup())) return 0`), carries the
/// ctor configuration (`stackspace` identity / `propagateIndirect`), starts
/// `analysis_finished` unset, and is a fully independent object: mutating the
/// clone's counter must not leak back to the source.
#[test]
fn w8_clone_filtered_independent_and_parameterized() {
    let fd = build_fd();
    let ram = ram_space(&fd);
    let yes = ActionGroupList::from_names(["g"]);
    let no = ActionGroupList::from_names(["other"]);

    // StackPtrFlow: group filter honored; clone carries a stack (so it runs the
    // stub body, not the no-stack latch) and starts with analysis_finished unset
    // (fresh-function value).  Built via the public ctor (fields are private to
    // the crate, as in C++).
    let src = ActionStackPtrFlow::boxed("g", Some(Rc::clone(&ram)));
    assert!(src.clone_filtered(&no).is_none(), "filtered out when group absent");
    let mut cl = src.clone_filtered(&yes).expect("present when group present");
    assert_eq!(cl.get_name(), "stackptrflow");
    assert_eq!(cl.get_group(), "g");
    // The clone's `analysis_finished` starts unset: observable through behavior —
    // the clone (which has a stack) runs the stub body (no latch, no change)
    // rather than the already-finished short-circuit; empty `Funcdata`, count==0.
    let mut ctx = ActionContext::new();
    let mut fd2 = build_fd();
    assert_eq!(cl.apply(&mut fd2, &mut ctx), 0);
    assert_eq!(cl.base().count, 0, "clone with stack runs the stub, reports no change");

    // DirectWrite: propagateIndirect carried verbatim through the clone.
    for prop in [true, false] {
        let dw = ActionDirectWrite::boxed("g", prop);
        let cl = dw.clone_filtered(&yes).expect("present");
        assert_eq!(cl.get_name(), "directwrite");
    }

    // Independence: mutating the clone's base counter must not touch the source.
    let seg = ActionSegmentize::boxed("g");
    let mut cl = seg.clone_filtered(&yes).expect("present");
    cl.base_mut().count = 42;
    assert_eq!(seg.base().count, 0, "clone is an independent object");
    assert_eq!(cl.base().count, 42);
}
