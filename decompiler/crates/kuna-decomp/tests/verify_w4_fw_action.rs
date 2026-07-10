//! Verifier adversarial tests for item `w4-fw-action`.
//!
//! Independent re-derivation tests targeting the hunt-list spots flagged as
//! most fragile for the Action/Rule engine (`action.cc`/`action.hh`):
//!
//!   - **Erase-while-iterating** (`ActionPool::processOp` dead path,
//!     `action.cc:829-834`): a run of consecutive dead ops at the optree head
//!     must each be skipped, `opDeadAndGone`-destroyed, and the live op still
//!     visited exactly once — the cursor must survive each erase.
//!   - **Opcode-change `rule_index` re-zero** (`action.cc:860-863`) interacting
//!     with a *disabled* rule placed before the morphing rule: after the
//!     re-zero the disabled rule is re-offered and must be re-skipped (not
//!     re-applied), so the disabled rule is never invoked.
//!   - **`getSubAction` ambiguity totality** (`ActionGroup::getSubAction`,
//!     `action.cc:456-479`): two sub-actions of the same name make
//!     `matchcount > 1` and the lookup must return "not found" (the C++
//!     returns `(Action *)0`), so `set_break_point`/`set_warning` report false.
//!   - **Breakpoint-resume cursor stability** (`ActionPool::apply` +
//!     `processOp`, `action.cc:851-852,877-888`): when a rule breakpoint fires
//!     mid-op, the resume must re-process *the same op* continuing from the
//!     preserved `rule_index`.  This test documents the one place the
//!     key-based `OpCursor` can diverge from the C++ live map iterator (an op
//!     inserted *below* the cursor during a breakpoint).
//!   - **Empty-group derive** (`ActionDatabase::deriveAction`,
//!     `action.cc:1145-1160`): a grouplist that filters everything out.  The
//!     C++ registers/returns a NULL root; the Rust substitutes an empty no-op
//!     group.  Documents the substitution.
//!
//! C++ oracle: decompiler/cpp/action.cc, action.hh (recorded cpp blob).

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::{
    breakflags, Action, ActionContext, ActionDatabase, ActionGroup, ActionGroupList,
    ActionPool, Rule, RuleState,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::{ArchContext, OpId, TypeOp};

// -----------------------------------------------------------------------------
// Harness (public-API only)
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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    Address::new(ram, off)
}

/// Make a live op at the given ram offset (fresh uniq), return its id.
fn make_op(fd: &mut Funcdata, off: u64, opc: OpCode) -> OpId {
    let pc = ram_addr(fd, off);
    let id = fd.obank_mut().create_at(0, pc);
    fd.obank_mut().mark_alive(id);
    fd.obank_mut().change_opcode(id, TypeOp::new(opc, 0, format!("{opc:?}")));
    id
}

/// Make a live op at an explicit SeqNum (lets us force optree order).
fn make_op_seq(fd: &mut Funcdata, off: u64, uniq: u32, opc: OpCode) -> OpId {
    let pc = ram_addr(fd, off);
    let id = fd.obank_mut().create_seq(0, SeqNum::new(pc, uniq));
    fd.obank_mut().mark_alive(id);
    fd.obank_mut().change_opcode(id, TypeOp::new(opc, 0, format!("{opc:?}")));
    id
}

type Log = Rc<RefCell<Vec<String>>>;

/// A rule that records each (tag, op-it-saw) and never changes anything.
struct LogRule {
    tag: String,
    log: Log,
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
    fn clone_rule(&self, _g: &ActionGroupList) -> Option<Box<dyn Rule>> {
        Some(Box::new(LogRule { tag: self.tag.clone(), log: self.log.clone() }))
    }
}

/// A rule that on its first COPY rewrites to INT_ADD and returns 1, on every
/// INT_ADD also returns 1 (so we can attach an action breakpoint that fires).
struct MorphRule {
    log: Log,
    fired: Rc<RefCell<bool>>,
}
impl Rule for MorphRule {
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let opc = data.obank().get(op).unwrap().code();
        self.log.borrow_mut().push(format!("morph:{opc:?}"));
        if opc == OpCode::CPUI_COPY && !*self.fired.borrow() {
            *self.fired.borrow_mut() = true;
            data.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
            return 1;
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

/// A leaf action (used to build same-named-children ambiguity).
struct NamedLeaf {
    base: kuna_decomp::action::ActionBase,
}
impl NamedLeaf {
    fn boxed(name: &str) -> Box<dyn Action> {
        Box::new(NamedLeaf { base: kuna_decomp::action::ActionBase::new(0, name, "g") })
    }
}
impl Action for NamedLeaf {
    fn base(&self) -> &kuna_decomp::action::ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut kuna_decomp::action::ActionBase {
        &mut self.base
    }
    fn apply(&mut self, _d: &mut Funcdata, _c: &mut ActionContext) -> int4 {
        0
    }
    fn clone_filtered(&self, _g: &ActionGroupList) -> Option<Box<dyn Action>> {
        Some(Box::new(NamedLeaf { base: self.base.clone() }))
    }
}

// =============================================================================
// 1. Erase-while-iterating: a run of dead ops at the head.
// =============================================================================

#[test]
fn w4_fw_action_consecutive_dead_ops_at_head_all_destroyed_live_visited_once() {
    let mut fd = build_fd();
    // Three dead ops at the lowest addresses, then a live INT_ADD.
    let d0 = make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);
    let d1 = make_op(&mut fd, 0x1004, OpCode::CPUI_COPY);
    let d2 = make_op(&mut fd, 0x1008, OpCode::CPUI_COPY);
    let _live = make_op(&mut fd, 0x100c, OpCode::CPUI_INT_ADD);
    for d in [d0, d1, d2] {
        fd.obank_mut().mark_dead(d);
    }
    assert_eq!(fd.obank().iter_all().count(), 4);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut pool = ActionPool::new(0, "pool");
    pool.add_rule(
        RuleState::new("analysis", 0, "A"),
        Box::new(LogRule { tag: "A".into(), log: log.clone() }),
    );
    let mut ctx = ActionContext::new();
    assert_eq!(pool.apply(&mut fd, &mut ctx), 0);

    // The three dead ops were each opDeadAndGone-destroyed; only the live op
    // survives and only it was dispatched, exactly once.
    assert_eq!(*log.borrow(), vec!["A:CPUI_INT_ADD"]);
    assert_eq!(fd.obank().iter_all().count(), 1);
}

// =============================================================================
// 2. Opcode-change re-zero re-offers a DISABLED rule, which stays skipped.
// =============================================================================

#[test]
fn w4_fw_action_rezero_reoffers_disabled_rule_without_applying_it() {
    // Pool order: [disabled-Log, Morph, Log].  Morph turns the COPY into
    // INT_ADD and returns 1 -> rule_index resets to 0 -> the disabled Log is
    // re-offered (and must be re-skipped, never invoked), Morph re-runs on
    // INT_ADD, then the live Log runs on INT_ADD.
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let fired = Rc::new(RefCell::new(false));
    let mut pool = ActionPool::new(0, "pool");

    // A disabled LogRule (tag "DIS") at the front: type_disable flag set.
    let mut dis = RuleState::new("analysis", 0, "DIS");
    dis.set_disable();
    pool.add_rule(dis, Box::new(LogRule { tag: "DIS".into(), log: log.clone() }));
    pool.add_rule(
        RuleState::new("analysis", 0, "morph"),
        Box::new(MorphRule { log: log.clone(), fired: fired.clone() }),
    );
    pool.add_rule(
        RuleState::new("analysis", 0, "live"),
        Box::new(LogRule { tag: "live".into(), log: log.clone() }),
    );

    let mut ctx = ActionContext::new();
    assert_eq!(pool.apply(&mut fd, &mut ctx), 0);

    // DIS never appears (disabled, both before and after the re-zero).
    assert!(!log.borrow().iter().any(|l| l.starts_with("DIS")), "disabled rule was invoked: {:?}", log.borrow());
    // Morph saw COPY (rewrote) then INT_ADD (no-op); live saw INT_ADD.
    assert_eq!(
        *log.borrow(),
        vec!["morph:CPUI_COPY", "morph:CPUI_INT_ADD", "live:CPUI_INT_ADD"]
    );
}

// =============================================================================
// 3. getSubAction ambiguity totality: two same-named children -> not found.
// =============================================================================

#[test]
fn w4_fw_action_getsubaction_ambiguous_children_report_not_found() {
    // Group "grp" with two children both named "dup".  Setting a breakpoint by
    // the bare name "dup" hits matchcount>1 and must return false (C++ returns
    // (Action*)0).  A uniquely-named child still resolves.
    let mut group = ActionGroup::new(0, "grp");
    group.add_action(NamedLeaf::boxed("dup"));
    group.add_action(NamedLeaf::boxed("dup"));
    group.add_action(NamedLeaf::boxed("solo"));

    // Ambiguous -> not found.
    assert!(!group.set_break_point(breakflags::break_action, "dup"));
    assert!(!group.set_warning(true, "dup"));
    // Unique -> found.
    assert!(group.set_break_point(breakflags::break_action, "solo"));
    // The group's own name with no remainder matches the group itself.
    assert!(group.set_break_point(breakflags::break_start, "grp"));
    // A name that exists nowhere -> not found.
    assert!(!group.set_break_point(breakflags::break_action, "ghost"));
}

// =============================================================================
// 4. Breakpoint-resume cursor: same-op resume after a rule action break.
// =============================================================================

#[test]
fn w4_fw_action_breakpoint_resumes_same_op_from_preserved_rule_index() {
    // Pool order: [Morph(break_action), Log].  Morph fires on COPY and has an
    // action breakpoint -> processOp returns -1 *after* rule_index advanced to
    // 1 (past Morph) and *without* advancing op_state.  Resuming the pool must
    // re-process the SAME op continuing at rule_index=1 (the Log rule), on the
    // now-INT_ADD opcode.  (No opcode-change re-zero races the breakpoint: the
    // C++ checks the breakpoint *before* the opcode-change re-zero.)
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);
    make_op(&mut fd, 0x1004, OpCode::CPUI_INT_ADD);

    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let fired = Rc::new(RefCell::new(false));
    let mut pool = ActionPool::new(0, "pool");

    let mut morph_state = RuleState::new("analysis", 0, "morph");
    morph_state.breakpoint |= breakflags::break_action; // fire on apply
    pool.add_rule(morph_state, Box::new(MorphRule { log: log.clone(), fired: fired.clone() }));
    pool.add_rule(
        RuleState::new("analysis", 0, "log"),
        Box::new(LogRule { tag: "log".into(), log: log.clone() }),
    );

    let mut ctx = ActionContext::new();
    // First perform: morph fires on the COPY (now INT_ADD), the break_action
    // breakpoint trips -> perform() records status_mid and returns -1.  (Resume
    // MUST go through perform(), which owns the status state machine; calling
    // apply() directly would re-reset op_state/rule_index because apply() keys
    // its reset on status, and only perform() advances status to status_mid.)
    let r1 = pool.perform(&mut fd, &mut ctx);
    assert_eq!(r1, -1, "expected a breakpoint partial return");
    assert_eq!(*log.borrow(), vec!["morph:CPUI_COPY"]);

    // Resume: perform() sees status_mid so apply() does NOT reset
    // op_state/rule_index.  It must continue on the SAME (first) op at
    // rule_index=1 -> Log sees the now-INT_ADD first op (NOT morph again),
    // proving rule_index was preserved across the break.
    let _r2 = pool.perform(&mut fd, &mut ctx);
    // The FIRST post-break dispatch is the broken op's REMAINING rule (log on
    // the now-INT_ADD first op) -- op_state was not advanced past the broken
    // op AND rule_index resumed at 1 (morph was not re-offered on the first op).
    let after_break: Vec<String> = log.borrow().iter().skip(1).cloned().collect();
    assert_eq!(
        after_break.first().map(String::as_str),
        Some("log:CPUI_INT_ADD"),
        "first resumed dispatch should be the broken op's remaining rule: {:?}",
        log.borrow()
    );
}

// =============================================================================
// 5. Empty-group derive: everything filtered out.
// =============================================================================

#[test]
fn w4_fw_action_empty_grouplist_derives_a_noop_root_not_a_crash() {
    // A universal with a single pool whose only rule is in group "x".  Deriving
    // through a grouplist that contains a DIFFERENT group filters everything
    // out.  C++ deriveAction would register/return NULL; the Rust substitutes
    // an empty ActionGroup so the map stays total and the root is a safe no-op.
    let log: Log = Rc::new(RefCell::new(Vec::new()));
    let mut pool = ActionPool::new(0, "pool");
    // A rule whose clone only survives group "x".
    struct XRule {
        log: Log,
    }
    impl Rule for XRule {
        fn apply_op(&mut self, _op: OpId, _d: &mut Funcdata) -> int4 {
            self.log.borrow_mut().push("xrule".into());
            0
        }
        fn get_op_list(&self) -> Vec<OpCode> {
            vec![OpCode::CPUI_INT_ADD]
        }
        fn clone_rule(&self, g: &ActionGroupList) -> Option<Box<dyn Rule>> {
            if g.contains("x") {
                Some(Box::new(XRule { log: self.log.clone() }))
            } else {
                None
            }
        }
    }
    pool.add_rule(RuleState::new("x", 0, "xrule"), Box::new(XRule { log: log.clone() }));
    let mut universal = ActionGroup::new(0, "universal");
    universal.add_action(Box::new(pool));

    let mut db = ActionDatabase::new();
    db.set_universal(Box::new(universal));
    db.set_group("empty_root", ["not_x"]); // filters everything out

    // derive_fresh yields None (nothing survived) -- matches C++ clone()==NULL.
    let fresh = db.derive_fresh("empty_root").unwrap();
    assert!(fresh.is_none(), "expected an empty filtered clone");

    // set_current substitutes an empty no-op root (Rust divergence from C++
    // NULL) -- get_current is Some and running it is a safe no-op.
    db.set_current("empty_root").unwrap();
    assert!(db.get_current().is_some());
    let mut fd = build_fd();
    make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let mut ctx = ActionContext::new();
    {
        let act = db.get_current_mut().unwrap();
        act.reset(&mut fd);
        let r = act.perform(&mut fd, &mut ctx);
        assert_eq!(r, 0);
    }
    // The filtered-out rule never ran.
    assert!(log.borrow().is_empty(), "filtered rule ran in an empty root: {:?}", log.borrow());
}

// =============================================================================
// 6. Cursor key-stability across an insert BELOW the cursor during a break.
//    Documents the one OpCursor divergence from a live C++ map iterator.
// =============================================================================

#[test]
fn w4_fw_action_cursor_jumps_to_op_inserted_below_after_break_documents_divergence() {
    // Build two live ops: first at (ram:0x1000, uniq via create_at), second at
    // 0x2000.  A morph rule on the first op breaks (break_action) AND, before
    // breaking, inserts a NEW op at a *lower* address (0x0500) -- i.e. below the
    // current cursor.  On resume the C++ live iterator would stay on the first
    // op; the key-based OpCursor recomputes "first op > cursor" and may land on
    // the lower-keyed inserted op instead.  This test pins the ACTUAL Rust
    // behaviour so any future change is caught; the divergence is debug-only
    // (breakpoints exist only under the W9 console).
    let mut fd = build_fd();
    let _first = make_op_seq(&mut fd, 0x1000, 0, OpCode::CPUI_COPY);
    let _second = make_op_seq(&mut fd, 0x2000, 1, OpCode::CPUI_INT_ADD);

    let log: Log = Rc::new(RefCell::new(Vec::new()));

    // A rule that, on its first COPY: logs, inserts a lower-addr live op, sets
    // an action break by returning 1, but we mark the break on its RuleState.
    struct InsertBelowRule {
        log: Log,
        done: Rc<RefCell<bool>>,
    }
    impl Rule for InsertBelowRule {
        fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
            let opc = data.obank().get(op).unwrap().code();
            self.log.borrow_mut().push(format!("ins:{opc:?}"));
            if opc == OpCode::CPUI_COPY && !*self.done.borrow() {
                *self.done.borrow_mut() = true;
                // Insert a live op BELOW the cursor (addr 0x0500 < 0x1000).
                let ram = Rc::clone(data.get_arch().manage().get_space_by_name("ram").unwrap());
                let pc = Address::new(ram, 0x0500);
                let id = data.obank_mut().create_seq(0, SeqNum::new(pc, 99));
                data.obank_mut().mark_alive(id);
                data.obank_mut().change_opcode(id, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
                return 1; // a change -> action breakpoint can trip
            }
            0
        }
        fn get_op_list(&self) -> Vec<OpCode> {
            vec![OpCode::CPUI_COPY, OpCode::CPUI_INT_ADD]
        }
        fn clone_rule(&self, _g: &ActionGroupList) -> Option<Box<dyn Rule>> {
            Some(Box::new(InsertBelowRule { log: self.log.clone(), done: self.done.clone() }))
        }
    }

    let mut pool = ActionPool::new(0, "pool");
    let mut st = RuleState::new("analysis", 0, "ins");
    st.breakpoint |= breakflags::break_action;
    pool.add_rule(st, Box::new(InsertBelowRule { log: log.clone(), done: Rc::new(RefCell::new(false)) }));

    let mut ctx = ActionContext::new();
    let r1 = pool.perform(&mut fd, &mut ctx);
    assert_eq!(r1, -1, "expected the action breakpoint to trip");
    assert_eq!(*log.borrow(), vec!["ins:CPUI_COPY"]);

    // Resume via perform() (status_mid -> no op_state/rule_index reset).  The
    // break happened on the FIRST op (0x1000 COPY) with op_state == Unstarted
    // (never advanced).  In the FAITHFUL C++ the live map iterator would still
    // point at the first (0x1000) COPY op, so the resume would re-process the
    // first op.  Here the key-based OpCursor was left Unstarted, so current_op
    // recomputes "the first optree entry", which is now the freshly-inserted
    // 0x0500 INT_ADD op -- the cursor jumps to the op inserted BELOW it.  We
    // pin the actual Rust behaviour so the divergence is caught by review and
    // any future engine change is detected.  (Debug-only: breakpoints exist
    // only under the W9 console; the normal no-breakpoint path advances the
    // cursor AFTER rules run and is faithful.)
    let _r2 = pool.perform(&mut fd, &mut ctx);
    let all: Vec<String> = log.borrow().clone();

    // PINNED DIVERGENCE (debug-breakpoint-only).  Full log:
    //   ["ins:CPUI_COPY",   // apply 1: ins on 0x1000, break trips (rule_index=1)
    //    "ins:CPUI_COPY",   // resume: 0x1000 re-processed at rule_index=0 -> ins AGAIN
    //    "ins:CPUI_INT_ADD"]// resume: ins on 0x2000
    //
    // The carried-over rule_index=1 was consumed against the wrong op: current_op
    // (Unstarted) recomputed to the below-inserted 0x0500 op and ran it with
    // rule_index=1 (its single rule skipped, nothing logged), THEN advanced to
    // 0x1000 and re-processed it from rule_index=0 -- so `ins` runs on the
    // 0x1000 COPY op a SECOND time.
    //
    // FAITHFUL C++: the live map iterator at the break still points at 0x1000;
    // resume processes 0x1000 with rule_index=1 (its rule already consumed, so
    // NO second `ins:COPY`), then `op_state++` jumps to 0x2000 (skipping the
    // below-inserted 0x0500).  Net C++ log would be:
    //   ["ins:CPUI_COPY", "ins:CPUI_INT_ADD"]  (a SINGLE ins:COPY).
    //
    // So the Rust port double-applies the broken op's rules on resume in this
    // narrow case.  Pinned here so the loss is reviewable and any future
    // iterator-faithful cursor change flips this assertion.
    assert_eq!(
        all,
        vec!["ins:CPUI_COPY", "ins:CPUI_COPY", "ins:CPUI_INT_ADD"],
        "cursor-resume divergence shape changed: {all:?}"
    );
}
