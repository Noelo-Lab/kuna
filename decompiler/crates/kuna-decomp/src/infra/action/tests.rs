//! Tests for the Action/Rule engine (`action.rs`).
//!
//! The mini-pipeline below builds a real [`Funcdata`] with hand-made ops and
//! exercises the load-bearing behaviours the port must reproduce exactly:
//! per-opcode dispatch *order*, the opcode-change `rule_index`-to-0 re-zero,
//! the `rule_repeatapply` fixpoint loop, the [`ActionRestartGroup`] restart
//! loop, and the [`ActionGroupList`] filtered-clone parity (two different root
//! actions derived from one spec tree).

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;

use super::*;
use crate::context::{ArchContext, TypeOp};

// -----------------------------------------------------------------------------
// Harness: a Funcdata with hand-built ops
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
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    Address::new(ram, off)
}

/// Make a live op with the given opcode at the given ram offset, returning its id.
fn make_op(fd: &mut Funcdata, off: u64, opc: OpCode) -> OpId {
    let pc = ram_addr(fd, off);
    let id = fd.obank_mut().create_at(0, pc);
    fd.obank_mut().mark_alive(id);
    fd.obank_mut().change_opcode(id, TypeOp::new(opc, 0, format!("{opc:?}")));
    id
}

// -----------------------------------------------------------------------------
// Test rules — interior-mutable logs so we can observe dispatch
// -----------------------------------------------------------------------------

type Log = Rc<RefCell<Vec<String>>>;

/// A rule that records its name + the op it was offered, never changing anything.
struct LogRule {
    tag: String,
    log: Log,
    group: &'static str,
}

impl Rule for LogRule {
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let opc = data.obank().get(op).unwrap().code();
        self.log.borrow_mut().push(format!("{}:{:?}", self.tag, opc));
        0
    }
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY, OpCode::CPUI_INT_ADD]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains(self.group) {
            Some(Box::new(LogRule { tag: self.tag.clone(), log: self.log.clone(), group: self.group }))
        } else {
            None
        }
    }
}

/// A rule that, on its FIRST encounter with a COPY, rewrites it to INT_ADD
/// (returning 1).  Used to test the opcode-change re-zero in `process_op`.
struct MorphRule {
    log: Log,
    fired: Rc<RefCell<bool>>,
}

impl Rule for MorphRule {
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let opc = data.obank().get(op).unwrap().code();
        self.log.borrow_mut().push(format!("morph_sees:{opc:?}"));
        if opc == OpCode::CPUI_COPY && !*self.fired.borrow() {
            *self.fired.borrow_mut() = true;
            data.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
            return 1; // changed the opcode -> must return non-zero
        }
        0
    }
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY, OpCode::CPUI_INT_ADD]
    }
    fn clone_rule(&self, _g: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(MorphRule { log: self.log.clone(), fired: self.fired.clone() }))
    }
}

/// A leaf action that increments `count` `budget` times, then stops (used to
/// drive `rule_repeatapply` and the group sequence).
struct CountdownAction {
    base: ActionBase,
    budget: Rc<RefCell<i32>>,
    log: Log,
    tag: String,
}

impl CountdownAction {
    fn boxed(group: &str, name: &str, budget: Rc<RefCell<i32>>, log: Log) -> Box<dyn Action> {
        Box::new(CountdownAction {
            base: ActionBase::new(0, name, group),
            budget,
            log,
            tag: name.to_string(),
        })
    }
}

impl Action for CountdownAction {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        self.log.borrow_mut().push(self.tag.clone());
        let mut b = self.budget.borrow_mut();
        if *b > 0 {
            *b -= 1;
            self.base.count += 1; // signal a change
        }
        0
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if grouplist.contains(&self.base.basegroup) {
            Some(Box::new(CountdownAction {
                base: self.base.clone(),
                budget: self.budget.clone(),
                log: self.log.clone(),
                tag: self.tag.clone(),
            }))
        } else {
            None
        }
    }
}

/// A leaf action that requests a restart on its first apply only.  Drives the
/// [`ActionRestartGroup`] loop.
struct RestartOnceAction {
    base: ActionBase,
    requests_left: Rc<RefCell<i32>>,
    log: Log,
}

impl Action for RestartOnceAction {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let n = *self.requests_left.borrow();
        self.log.borrow_mut().push(format!("restart_action(left={n})"));
        if n > 0 {
            *self.requests_left.borrow_mut() -= 1;
            data.set_restart_pending(true);
        }
        0
    }
    fn clone_filtered(&self, _g: &ActionGroupList) -> Option<Box<dyn Action>> {
        Some(Box::new(RestartOnceAction {
            base: self.base.clone(),
            requests_left: self.requests_left.clone(),
            log: self.log.clone(),
        }))
    }
}

// -----------------------------------------------------------------------------
// Worked dummy rule for the module docs (a real, minimal const-fold shape)
// -----------------------------------------------------------------------------

/// The documented worked example: a minimal rule that operates only on
/// `INT_ADD`, never mutates (so it is safe to run anywhere), and registers
/// against the `"analysis"` group.  This is the shape W5 rules copy.
pub struct DummyConstFold;

impl Rule for DummyConstFold {
    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        0 // a real rule would fold and return 1
    }
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ADD]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(DummyConstFold))
        } else {
            None
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[test]
fn perop_registration_is_insertion_order() {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut pool = ActionPool::new(ruleflags::rule_repeatapply, "pool");
    for tag in ["A", "B", "C"] {
        pool.add_rule(
            RuleState::new("analysis", 0, tag),
            Box::new(LogRule { tag: tag.to_string(), log: log.clone(), group: "analysis" }),
        );
    }
    // Each registered rule lists COPY and INT_ADD; both per-op lists must be in
    // registration order A,B,C.
    assert_eq!(pool.perop_names(OpCode::CPUI_COPY), vec!["A", "B", "C"]);
    assert_eq!(pool.perop_names(OpCode::CPUI_INT_ADD), vec!["A", "B", "C"]);
    // A rule on an opcode it didn't register for sees nothing.
    assert!(pool.perop_names(OpCode::CPUI_STORE).is_empty());
}

#[test]
fn pool_dispatches_each_op_through_every_rule_in_order() {
    let mut fd = build_fd();
    // Two ops: a COPY then an INT_ADD (distinct addresses -> optree order).
    make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);
    make_op(&mut fd, 0x1004, OpCode::CPUI_INT_ADD);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut pool = ActionPool::new(0, "pool");
    for tag in ["A", "B"] {
        pool.add_rule(
            RuleState::new("analysis", 0, tag),
            Box::new(LogRule { tag: tag.to_string(), log: log.clone(), group: "analysis" }),
        );
    }
    let mut ctx = ActionContext::new();
    let res = pool.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0);
    // op1 (COPY): A then B; op2 (INT_ADD): A then B.
    assert_eq!(
        *log.borrow(),
        vec!["A:CPUI_COPY", "B:CPUI_COPY", "A:CPUI_INT_ADD", "B:CPUI_INT_ADD"]
    );
}

#[test]
fn opcode_change_rezeros_rule_index() {
    // A pool: [Morph, Log].  Morph turns the COPY into INT_ADD and returns 1,
    // which must reset rule_index to 0 -> the whole list re-runs against the
    // NEW opcode (so Morph sees INT_ADD, then Log sees INT_ADD).  Without the
    // re-zero we would only see Log once on the COPY's stale list.
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let fired = Rc::new(RefCell::new(false));
    let mut pool = ActionPool::new(0, "pool");
    pool.add_rule(
        RuleState::new("analysis", 0, "morph"),
        Box::new(MorphRule { log: log.clone(), fired: fired.clone() }),
    );
    pool.add_rule(
        RuleState::new("analysis", 0, "log"),
        Box::new(LogRule { tag: "log".into(), log: log.clone(), group: "analysis" }),
    );

    let mut ctx = ActionContext::new();
    assert_eq!(pool.apply(&mut fd, &mut ctx), 0);
    // 1. morph sees COPY, rewrites -> INT_ADD, returns 1, rule_index reset to 0.
    // 2. morph sees INT_ADD (no change, fired latch set).
    // 3. log sees INT_ADD.
    assert_eq!(
        *log.borrow(),
        vec!["morph_sees:CPUI_COPY", "morph_sees:CPUI_INT_ADD", "log:CPUI_INT_ADD"]
    );
}

#[test]
fn dead_op_is_skipped_and_destroyed() {
    // A pool over two ops where the first is already dead.  process_op must
    // skip the dead op, destroy it (opDeadAndGone), and dispatch only to the
    // live one.
    let mut fd = build_fd();
    let dead = make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);
    fd.obank_mut().mark_dead(dead); // back to the dead list, dead flag set
    make_op(&mut fd, 0x1004, OpCode::CPUI_INT_ADD);
    assert_eq!(fd.obank().iter_all().count(), 2);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut pool = ActionPool::new(0, "pool");
    pool.add_rule(
        RuleState::new("analysis", 0, "A"),
        Box::new(LogRule { tag: "A".into(), log: log.clone(), group: "analysis" }),
    );
    let mut ctx = ActionContext::new();
    assert_eq!(pool.apply(&mut fd, &mut ctx), 0);
    // Only the live INT_ADD was dispatched.
    assert_eq!(*log.borrow(), vec!["A:CPUI_INT_ADD"]);
    // The dead op was destroyed (opDeadAndGone removed it from the optree).
    assert_eq!(fd.obank().iter_all().count(), 1);
}

#[test]
fn group_applies_actions_in_sequence() {
    let mut fd = build_fd();
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let budget = Rc::new(RefCell::new(0));
    let mut group = ActionGroup::new(0, "grp");
    group.add_action(CountdownAction::boxed("g", "one", budget.clone(), log.clone()));
    group.add_action(CountdownAction::boxed("g", "two", budget.clone(), log.clone()));
    group.add_action(CountdownAction::boxed("g", "three", budget.clone(), log.clone()));

    let mut ctx = ActionContext::new();
    assert_eq!(group.apply(&mut fd, &mut ctx), 0);
    assert_eq!(*log.borrow(), vec!["one", "two", "three"]);
}

#[test]
fn repeatapply_loops_until_no_change() {
    // A repeatapply group with one countdown action of budget 3 must apply it
    // until it stops signalling a change: 3 changing passes + 1 final no-change
    // pass = 4 applies.
    let mut fd = build_fd();
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let budget = Rc::new(RefCell::new(3));
    let mut group = ActionGroup::new(ruleflags::rule_repeatapply, "grp");
    group.add_action(CountdownAction::boxed("g", "act", budget.clone(), log.clone()));

    let mut ctx = ActionContext::new();
    let total = group.perform(&mut fd, &mut ctx);
    assert_eq!(*log.borrow(), vec!["act", "act", "act", "act"]);
    // The group counts 3 changes (one per budget tick).
    assert_eq!(total, 3);
}

#[test]
fn restart_group_reruns_on_restart_pending() {
    // requests_left=2: the action requests a restart twice, so the group body
    // runs three times (initial + 2 restarts), then settles.  maxrestarts=5 is
    // generous enough not to trip.
    let mut fd = build_fd();
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let requests_left = Rc::new(RefCell::new(2));
    let mut rg = ActionRestartGroup::new(ruleflags::rule_onceperfunc, "universal", 5);
    rg.add_action(Box::new(RestartOnceAction {
        base: ActionBase::new(0, "ra", "base"),
        requests_left: requests_left.clone(),
        log: log.clone(),
    }));

    let mut ctx = ActionContext::new();
    // This test exercises the restart RE-RUN path, which is gated on re-flow
    // availability; enable it (the decompile drive sets this when the flow
    // environment is threaded in).
    ctx.can_reflow = true;
    assert_eq!(rg.apply(&mut fd, &mut ctx), 0);
    assert_eq!(
        *log.borrow(),
        vec![
            "restart_action(left=2)",
            "restart_action(left=1)",
            "restart_action(left=0)"
        ]
    );
    assert!(!fd.has_restart_pending());
}

#[test]
fn restart_group_honors_maxrestarts() {
    // The action always requests a restart (requests_left huge); maxrestarts=2
    // caps it: body runs initial + 2 restarts = 3 times, then the "Exceeded
    // maximum restarts" warning fires and the loop ends.
    let mut fd = build_fd();
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let requests_left = Rc::new(RefCell::new(1000));
    let mut rg = ActionRestartGroup::new(ruleflags::rule_onceperfunc, "universal", 2);
    rg.add_action(Box::new(RestartOnceAction {
        base: ActionBase::new(0, "ra", "base"),
        requests_left: requests_left.clone(),
        log: log.clone(),
    }));

    let mut ctx = ActionContext::new();
    // This test exercises the restart RE-RUN path, which is gated on re-flow
    // availability; enable it (the decompile drive sets this when the flow
    // environment is threaded in).
    ctx.can_reflow = true;
    assert_eq!(rg.apply(&mut fd, &mut ctx), 0);
    assert_eq!(log.borrow().len(), 3);
    assert!(ctx
        .warnings
        .messages
        .iter()
        .any(|m| m.contains("Exceeded maximum restarts")));
}

// NOTE: the `is_jumptable_recovery_on()` suppression branch in
// `ActionRestartGroup::apply` (C++ "Don't restart within jumptable recovery")
// is transcribed faithfully but cannot be unit-tested from a pure-W3 harness:
// the `jumptablerecovery_on` flag (0x100) has no public W3 setter — in C++ it is
// set internally by `Funcdata::stageJumpTable` (W7+).  `set_jumptable_recovery`
// toggles the *other* flag (`jumptablerecovery_dont`, the flow-analysis gate),
// not the recovery-active flag the branch reads.  A W7 integration test can
// cover it once jump-table recovery is wired up.

// -----------------------------------------------------------------------------
// Group-filter parity: derive two root actions from one spec tree
// -----------------------------------------------------------------------------

/// Build a universal tree of three pools/actions whose groups are
/// `alpha`/`beta`/`gamma`, so a grouplist selects a subset.
fn build_demo_universal(log: &Log) -> Box<dyn Action> {
    let mut root = ActionRestartGroup::new(ruleflags::rule_onceperfunc, "universal", 1);

    // A leaf action in group "alpha".
    root.add_action(CountdownAction::boxed("alpha", "leaf_alpha", Rc::new(RefCell::new(0)), log.clone()));

    // A pool with a "beta" rule and a "gamma" rule.
    let mut pool = ActionPool::new(ruleflags::rule_repeatapply, "pool");
    pool.add_rule(
        RuleState::new("beta", 0, "rule_beta"),
        Box::new(LogRule { tag: "rule_beta".into(), log: log.clone(), group: "beta" }),
    );
    pool.add_rule(
        RuleState::new("gamma", 0, "rule_gamma"),
        Box::new(LogRule { tag: "rule_gamma".into(), log: log.clone(), group: "gamma" }),
    );
    root.add_action(Box::new(pool));

    // A nested group in "gamma" that itself holds a "beta" leaf.
    let mut inner = ActionGroup::new(0, "inner");
    inner.add_action(CountdownAction::boxed("beta", "leaf_beta", Rc::new(RefCell::new(0)), log.clone()));
    root.add_action(Box::new(inner));

    Box::new(root)
}

#[test]
fn filtered_clone_selects_group_members() {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let universal = build_demo_universal(&log);

    // Root A: only "alpha" + "beta".
    let gl_a = ActionGroupList::from_names(["alpha", "beta"]);
    let root_a = universal.clone_filtered(&gl_a).expect("alpha/beta survives");
    // Root B: only "gamma".
    let gl_b = ActionGroupList::from_names(["gamma"]);
    let root_b = universal.clone_filtered(&gl_b).expect("gamma survives");

    // Walk each root naming the surviving leaves/rules (by running it).
    let names_a = run_and_collect(root_a, &log);
    let names_b = run_and_collect(root_b, &log);

    // A keeps: leaf_alpha (alpha), rule_beta (beta), leaf_beta (beta inside the
    // inner group).  The inner group survives because leaf_beta does; rule_gamma
    // and leaf_alpha-only-in-other-group are filtered.
    assert!(names_a.contains(&"leaf_alpha".to_string()));
    assert!(names_a.iter().any(|n| n.starts_with("rule_beta")));
    assert!(names_a.contains(&"leaf_beta".to_string()));
    assert!(!names_a.iter().any(|n| n.starts_with("rule_gamma")));

    // B keeps only rule_gamma; the inner group (only beta member) wholly drops,
    // and leaf_alpha drops.
    assert!(names_b.iter().any(|n| n.starts_with("rule_gamma")));
    assert!(!names_b.contains(&"leaf_alpha".to_string()));
    assert!(!names_b.iter().any(|n| n.starts_with("rule_beta")));
    assert!(!names_b.contains(&"leaf_beta".to_string()));
}

#[test]
fn default_groups_match_cpp_member_lists() {
    let mut db = ActionDatabase::new();
    build_default_groups(&mut db);

    // Spot-check membership exactly against coreaction.cc:5675.
    let decompile = db.get_group("decompile").unwrap();
    assert!(decompile.contains("base"));
    assert!(decompile.contains("canonicalcompare"));
    assert!(decompile.contains("presentcompare"));
    assert!(decompile.contains("conditionalexe"));
    assert!(!decompile.contains("noproto")); // jumptable-only

    let jumptable = db.get_group("jumptable").unwrap();
    assert!(jumptable.contains("noproto"));
    assert!(jumptable.contains("normalizebranches"));
    assert!(!jumptable.contains("presentcompare")); // decompile-only

    let register = db.get_group("register").unwrap();
    // register = { base, analysis, canonicalcompare, subvar }
    assert!(register.contains("base"));
    assert!(register.contains("analysis"));
    assert!(register.contains("canonicalcompare"));
    assert!(register.contains("subvar"));
    assert!(!register.contains("deadcode"));

    let firstpass = db.get_group("firstpass").unwrap();
    assert!(firstpass.contains("base"));
    assert!(!firstpass.contains("analysis"));

    // A missing group errors.
    assert!(db.get_group("nope").is_err());
}

#[test]
fn database_derives_distinct_roots_from_one_universal() {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut db = ActionDatabase::new();
    db.set_universal(build_demo_universal(&log));
    db.set_group("root_ab", ["alpha", "beta"]);
    db.set_group("root_g", ["gamma"]);

    let root_ab = db.derive_fresh("root_ab").unwrap().expect("alpha/beta survives");
    let root_g = db.derive_fresh("root_g").unwrap().expect("gamma survives");

    // Run each over a tiny function with a COPY+INT_ADD so the pool rules fire,
    // and observe which named rules/actions appear in the log.
    let names_ab = run_and_collect(root_ab, &log);
    let names_g = run_and_collect(root_g, &log);

    assert!(names_ab.contains(&"leaf_alpha".to_string()));
    assert!(names_ab.iter().any(|n| n.starts_with("rule_beta")));
    assert!(names_ab.contains(&"leaf_beta".to_string()));
    assert!(!names_ab.iter().any(|n| n.starts_with("rule_gamma")));

    assert!(names_g.iter().any(|n| n.starts_with("rule_gamma")));
    assert!(!names_g.contains(&"leaf_alpha".to_string()));
    assert!(!names_g.iter().any(|n| n.starts_with("rule_beta")));
    assert!(!names_g.contains(&"leaf_beta".to_string()));
}

/// Run a derived root over a small function and return the distinct log lines.
fn run_and_collect(mut root: Box<dyn Action>, log: &Log) -> Vec<String> {
    log.borrow_mut().clear();
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD); // a live INT_ADD for the rules
    let mut ctx = ActionContext::new();
    root.reset(&mut fd);
    let _ = root.perform(&mut fd, &mut ctx);
    log.borrow().clone()
}

#[test]
fn set_current_caches_and_exposes_the_root() {
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut db = ActionDatabase::new();
    db.set_universal(build_demo_universal(&log));
    db.set_group("root_g", ["gamma"]);

    db.set_current("root_g").unwrap();
    assert_eq!(db.get_current_name(), "root_g");
    assert!(db.get_current().is_some());

    // Run the current action via get_current_mut over a tiny function.
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let mut ctx = ActionContext::new();
    log.borrow_mut().clear();
    {
        let act = db.get_current_mut().expect("current set");
        act.reset(&mut fd);
        let _ = act.perform(&mut fd, &mut ctx);
    }
    // Only the gamma rule survived the filter, and it logs once per INT_ADD.
    assert!(log.borrow().iter().any(|n| n.starts_with("rule_gamma")));
    assert!(!log.borrow().iter().any(|n| n.starts_with("rule_beta")));
    assert!(!log.borrow().contains(&"leaf_alpha".to_string()));

    // A second set_current reuses the cache (deriveAction's "already derived").
    db.set_current("root_g").unwrap();
    assert!(db.get_current().is_some());
}

#[test]
fn dummy_rule_registers_and_dispatches_only_on_its_opcode() {
    // The documented worked example: DummyConstFold is offered only INT_ADD.
    let mut pool = ActionPool::new(0, "pool");
    pool.add_rule(RuleState::new("analysis", 0, "DummyConstFold"), Box::new(DummyConstFold));
    assert_eq!(pool.perop_names(OpCode::CPUI_INT_ADD), vec!["DummyConstFold"]);
    assert!(pool.perop_names(OpCode::CPUI_COPY).is_empty());

    // It survives an "analysis" grouplist and drops otherwise.
    assert!(pool.clone_filtered(&ActionGroupList::from_names(["analysis"])).is_some());
    assert!(pool.clone_filtered(&ActionGroupList::from_names(["other"])).is_none());
}

#[test]
fn repeatapply_deadline_bounds_nonconverging_action() {
    // (kuna decompile-all watchdog) The cooperative per-function deadline must
    // bound a repeatapply loop whose child NEVER stops reporting changes — the
    // exact shape of the stripped-ELF `decompile-all` hang (a `mainloop` child
    // reported one change per iteration forever).  With an already-expired
    // deadline the `Action::perform` repeat gate and the `ActionGroup::apply`
    // scheduling boundary must both bail out after at most one body pass
    // instead of looping on the huge budget.
    let mut fd = build_fd();
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    // A budget so large that, without the deadline, the loop would effectively
    // run forever (1<<30 change-reporting passes).
    let budget = Rc::new(RefCell::new(1 << 30));
    let mut group = ActionGroup::new(ruleflags::rule_repeatapply, "grp");
    group.add_action(CountdownAction::boxed("g", "act", budget.clone(), log.clone()));

    let mut ctx = ActionContext::new();
    ctx.deadline = Some(std::time::Instant::now()); // already expired
    let total = group.perform(&mut fd, &mut ctx);
    // The child applied at most twice (the group body may schedule the first
    // pass before probing, and the repeat gate stops the loop), not 2^30 times.
    assert!(
        log.borrow().len() <= 2,
        "expired deadline must stop the repeatapply loop, got {} applies",
        log.borrow().len()
    );
    // Whatever ran is reported as the change count (the drive then turns the
    // expiry into a per-function error).
    assert!(total <= 2);
}
