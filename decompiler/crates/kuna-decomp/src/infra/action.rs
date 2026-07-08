//! Port of `decompiler/cpp/action.{cc,hh}` — the Action/Rule transformation
//! engine (ADR 0005).
//!
//! # What this is
//!
//! The decompiler applies a fixed *schedule* of transformations to a
//! [`Funcdata`].  The schedule is a tree:
//!
//! * **leaf [`Action`]s** — a coarse pass (e.g. `ActionHeritage`,
//!   `ActionDeadCode`).  Each is a `Box<dyn Action>` supplied by the W5 rule
//!   wave; its `apply` mutates the function and signals "I made N changes" by
//!   returning a positive count.
//! * [`ActionGroup`] — an ordered list of sub-actions applied in sequence (the
//!   C++ `ActionGroup`).  May carry [`rule_repeatapply`](ruleflags::rule_repeatapply).
//! * [`ActionPool`] — a bag of [`Rule`]s applied to *every* [`PcodeOp`] in the
//!   function, dispatched per-opcode (the C++ `ActionPool`).
//! * [`ActionRestartGroup`] — an [`ActionGroup`] that re-runs itself when a
//!   sub-action requests a restart via [`Funcdata::set_restart_pending`].
//!
//! The single **universal** action contains every component; a *root* action
//! (e.g. `"decompile"`, `"jumptable"`) is the universal tree filtered down to a
//! set of enabled *groups* by [`ActionGroupList`].  [`ActionDatabase`] owns the
//! universal tree, the group map (built verbatim by [`build_default_groups`]),
//! and the derivation cache.
//!
//! # Faithfulness notes (the load-bearing transcriptions)
//!
//! * [`ActionPool::process_op`] is `action.cc:822-877` transcribed exactly:
//!   dead-op skip with `opDeadAndGone` bookkeeping, the `rule_index` walk over
//!   `perop[opc]` in insertion order, and the **reset-`rule_index`-to-0 when an
//!   apply changes the op's opcode** rule.  That reset lets rules observe each
//!   other's effects mid-op and is output-affecting — do not "clean it up".
//! * Resumability for single-stepping: [`ActionPool`] keeps an `op_state`
//!   cursor (the C++ `PcodeOpTree::const_iterator`) and the `rule_index`
//!   breakpoint cursor so a partial (breakpoint) return resumes where it left
//!   off.  Because Rust cannot hold a live `BTreeMap` iterator across mutation,
//!   the cursor is modelled as the last-visited [`SeqNum`]; the next op is the
//!   first optree entry strictly greater than it (same order as the C++ map
//!   iterator, and stable across `opDeadAndGone` erases).
//! * [`Action::perform`] is `action.cc:298-362`: the `status_*` state machine,
//!   the `rule_repeatapply` loop, the `rule_onceperfunc`/`rule_oneactperfunc`
//!   end conditions, and the warning/breakpoint emission points.
//! * [`ActionRestartGroup::apply`] is `action.cc:553-582`: the `maxrestarts`
//!   loop, the `hasRestartPending` / `isJumptableRecoveryOn` checks, the
//!   `clearAnalysis` call, and the "reset everything but ourselves" sweep.
//!
//! # How a W5 rule registers (READ THIS before adding rules)
//!
//! A rule is a type implementing [`Rule`].  The *only* required methods are
//! [`Rule::clone_rule`] (the group-filter clone — return `Some(boxed self)`
//! iff `grouplist` contains your group) and [`Rule::apply_op`] (the
//! transformation).  Optionally override [`Rule::get_op_list`] to restrict the
//! opcodes you are offered (default = every opcode, the C++
//! `Rule::getOpList` default).
//!
//! Per-rule **state the engine owns** (name, group, enable/disable flag,
//! breakpoints, `count_tests`/`count_apply` statistics, warning flags) lives in
//! [`RuleState`], not in your struct — exactly as the prompt's "engine wrapper
//! owns counts/breakpoints/warnings/flags" mandates.  You construct your rule
//! with its group string and the engine wraps it.  The convention is a
//! [`RuleSpec`] row:
//!
//! ```ignore
//! RuleSpec { group: "analysis", ctor: || Box::new(RuleTermOrder::new()) }
//! ```
//!
//! and the W5 `universal_action` builder pushes specs into pools with
//! [`ActionPool::add_rule`].  A worked dummy rule lives in this file's tests
//! ([`tests::DummyConstFold`]) — copy its shape.
//!
//! Leaf **actions** register the same way via [`ActionGroup::add_action`] with
//! a `Box<dyn Action>`; the group/flags/name live in the action's own
//! [`ActionBase`] (constructed with [`ActionBase::new`]).

use std::collections::BTreeMap;

use kuna_base::address::SeqNum;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use smallvec::SmallVec;

use crate::funcdata::Funcdata;
use crate::context::OpId;

// =============================================================================
// Flag enums (transcribed from action.hh)
// =============================================================================

/// Boolean behavior properties governing an [`Action`] (C++
/// `Action::ruleflags`).
pub mod ruleflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Apply rule repeatedly until no change.
    pub const rule_repeatapply: uint4 = 4;
    /// Apply rule once per function.
    pub const rule_onceperfunc: uint4 = 8;
    /// Makes a change only once per function.
    pub const rule_oneactperfunc: uint4 = 16;
    /// Print debug messages specifically for this action.
    pub const rule_debug: uint4 = 32;
    /// If this action makes a change, issue a warning.
    pub const rule_warnings_on: uint4 = 64;
    /// A warning has been issued for this action.
    pub const rule_warnings_given: uint4 = 128;
}

/// Boolean properties describing the \e status of an [`Action`] (C++
/// `Action::statusflags`).
pub mod statusflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// At start of action.
    pub const status_start: uint4 = 1;
    /// At start after breakpoint.
    pub const status_breakstarthit: uint4 = 2;
    /// Repeating the same action.
    pub const status_repeat: uint4 = 4;
    /// In middle of action (use subclass status).
    pub const status_mid: uint4 = 8;
    /// `getFuncdata` has completed once (for `onceperfunc`).
    pub const status_end: uint4 = 16;
    /// Completed full action last time but indicated action break.
    pub const status_actionbreak: uint4 = 32;
}

/// Break points associated with an [`Action`] (C++ `Action::breakflags`).
pub mod breakflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Break at beginning of action.
    pub const break_start: uint4 = 1;
    /// Temporary break at start of action.
    pub const tmpbreak_start: uint4 = 2;
    /// Break if a change has been made.
    pub const break_action: uint4 = 4;
    /// Temporary break if a change has been made.
    pub const tmpbreak_action: uint4 = 8;
}

/// Properties associated with a [`Rule`] (C++ `Rule::typeflags`).
pub mod typeflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Is this rule disabled.
    pub const type_disable: uint4 = 1;
    /// Print debug info specific for this rule.
    pub const rule_debug: uint4 = 2;
    /// A warning is issued if this rule is applied.
    pub const warnings_on: uint4 = 4;
    /// Set if a warning for this rule has been given before.
    pub const warnings_given: uint4 = 8;
}

// =============================================================================
// ActionGroupList (action.hh:31-40)
// =============================================================================

/// The list of groups defining a *root* [`Action`] (C++ `ActionGroupList`).
///
/// A `Rule` or leaf `Action` belongs to a *group*; this list is the set of
/// group names whose members together form a root action.  Backed by a
/// `BTreeSet`-equivalent `BTreeMap<String,()>` (ADR 0002 bans `HashSet`); the
/// C++ uses `set<string>` whose iteration order is irrelevant here — only
/// membership (`contains`) is ever queried.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActionGroupList {
    list: BTreeMap<String, ()>,
}

impl ActionGroupList {
    /// An empty grouplist.
    pub fn new() -> ActionGroupList {
        ActionGroupList { list: BTreeMap::new() }
    }

    /// Build a grouplist from an iterator of group names (used by
    /// [`ActionDatabase::set_group`]).
    pub fn from_names<I, S>(names: I) -> ActionGroupList
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut list = BTreeMap::new();
        for n in names {
            list.insert(n.into(), ());
        }
        ActionGroupList { list }
    }

    /// Check if `this` contains a given group (C++ `ActionGroupList::contains`).
    pub fn contains(&self, nm: &str) -> bool {
        self.list.contains_key(nm)
    }

    /// Add a group; returns `true` if newly inserted (C++
    /// `ActionDatabase::addToGroup` / `set<string>::insert().second`).
    pub fn insert(&mut self, basegroup: impl Into<String>) -> bool {
        self.list.insert(basegroup.into(), ()).is_none()
    }

    /// Remove a group; returns `true` if it existed (C++
    /// `ActionDatabase::removeFromGroup` / `set<string>::erase()>0`).
    pub fn erase(&mut self, basegrp: &str) -> bool {
        self.list.remove(basegrp).is_some()
    }

    /// Clear all members (C++ `ActionDatabase::setGroup` clears `curgrp.list`).
    pub fn clear(&mut self) {
        self.list.clear();
    }
}

// =============================================================================
// Engine-owned per-Action bookkeeping (ActionBase)
// =============================================================================

/// The bookkeeping every [`Action`] carries (C++ `Action`'s protected members).
///
/// Per the wave brief the *engine wrapper* owns counts/breakpoints/warnings/
/// flags; this struct is that ownership made concrete.  Each concrete action
/// (leaf, [`ActionGroup`], [`ActionPool`], [`ActionRestartGroup`]) embeds one
/// and exposes it via [`Action::base`]/[`Action::base_mut`], the way the C++
/// base class fields are inherited.
#[derive(Debug, Clone)]
pub struct ActionBase {
    /// Changes not including the last call to `apply()` (C++ `lcount`).
    pub lcount: int4,
    /// Number of changes made by this action so far (C++ `count`).
    pub count: int4,
    /// Current status (C++ `status`, a [`statusflags`] word).
    pub status: uint4,
    /// Breakpoint properties (C++ `breakpoint`, a [`breakflags`] word).
    pub breakpoint: uint4,
    /// Behavior properties (C++ `flags`, a [`ruleflags`] word).
    pub flags: uint4,
    /// Number of times `apply()` has been called (C++ `count_tests`).
    pub count_tests: uint4,
    /// Number of times `apply()` made changes (C++ `count_apply`).
    pub count_apply: uint4,
    /// Name of the action (C++ `name`).
    pub name: String,
    /// Base group this action belongs to (C++ `basegroup`).
    pub basegroup: String,
}

impl ActionBase {
    /// Base constructor for an Action (C++ `Action::Action(uint4,string,string)`).
    pub fn new(f: uint4, nm: impl Into<String>, g: impl Into<String>) -> ActionBase {
        ActionBase {
            lcount: 0,
            count: 0,
            status: statusflags::status_start,
            breakpoint: 0,
            flags: f,
            count_tests: 0,
            count_apply: 0,
            name: nm.into(),
            basegroup: g.into(),
        }
    }

    fn turn_on_warnings(&mut self) {
        self.flags |= ruleflags::rule_warnings_on;
    }
    fn turn_off_warnings(&mut self) {
        self.flags &= !ruleflags::rule_warnings_on;
    }

    /// Check if there was an active *start* breakpoint (C++
    /// `Action::checkStartBreak`).  Clears any temporary start breakpoint.
    fn check_start_break(&mut self) -> bool {
        if (self.breakpoint & (breakflags::break_start | breakflags::tmpbreak_start)) != 0 {
            self.breakpoint &= !breakflags::tmpbreak_start; // Clear if temporary
            return true;
        }
        false
    }

    /// Check if there was an active *action* breakpoint (C++
    /// `Action::checkActionBreak`).  Clears any temporary action breakpoint.
    fn check_action_break(&mut self) -> bool {
        if (self.breakpoint & (breakflags::break_action | breakflags::tmpbreak_action)) != 0 {
            self.breakpoint &= !breakflags::tmpbreak_action; // Clear temporary
            return true;
        }
        false
    }

    /// C++ `Action::issueWarning`.  The C++ prints to `glb->printMessage`; that
    /// console channel is the W4 `Architecture` subsystem (not yet ported), so
    /// the message is appended to the engine-owned [`WarningSink`] instead and
    /// the `rule_warnings_given` latch is set so it fires at most once.
    fn issue_warning(&mut self, sink: &mut WarningSink) {
        if (self.flags & (ruleflags::rule_warnings_on | ruleflags::rule_warnings_given))
            == ruleflags::rule_warnings_on
        {
            self.flags |= ruleflags::rule_warnings_given;
            sink.messages.push(format!("WARNING: Applied action {}", self.name));
        }
    }

    /// Reset for a new function (C++ `Action::reset`): status back to start and
    /// clear the warning-given latch.
    fn reset_base(&mut self) {
        self.status = statusflags::status_start;
        self.flags &= !ruleflags::rule_warnings_given;
    }

    /// Reset the statistics (C++ `Action::resetStats`).
    fn reset_stats_base(&mut self) {
        self.count_tests = 0;
        self.count_apply = 0;
    }
}

/// Collector for messages the engine would otherwise print to the
/// `Architecture` console (`glb->printMessage`) or the comment database
/// (`Funcdata::warningHeader`).
///
/// LOCAL STUB(W4/W9): the W4 `Architecture` console and the comment DB are not
/// yet ported.  W9 (the console) and the comment subsystem will drain this; for
/// now it makes the warning emission points observable and testable without a
/// global side channel.  The C++ `printMessage`/`warningHeader` text format is
/// reproduced verbatim so a later wave can route it straight through.
#[derive(Debug, Clone, Default)]
pub struct WarningSink {
    /// Messages in emission order.
    pub messages: Vec<String>,
}

impl WarningSink {
    /// A fresh, empty sink.
    pub fn new() -> WarningSink {
        WarningSink::default()
    }
}

// =============================================================================
// The Action trait (action.hh:52-135)
// =============================================================================

/// The result of a single [`Action::apply`] / [`Action::perform`] step,
/// mirroring the C++ convention that `int4` doubles as a status code: a
/// non-negative value is a change count, `-1` is a breakpoint/partial return.
pub type ApplyResult = int4;

/// Large-scale transformation applied to the varnode/op graph (C++ `Action`).
///
/// A concrete action implements [`apply`](Action::apply) (the C++ virtual
/// `apply`) and [`clone_filtered`](Action::clone_filtered) (the C++ `clone(grouplist)`)
/// and exposes its engine-owned [`ActionBase`].  The shared `perform` state
/// machine is provided by [`Action::perform`] (a default method, the C++
/// non-virtual `Action::perform`), so leaf actions get the
/// repeat/once/breakpoint semantics for free.
pub trait Action {
    /// Borrow the engine-owned bookkeeping (C++ inherited base fields).
    fn base(&self) -> &ActionBase;
    /// Mutably borrow the engine-owned bookkeeping.
    fn base_mut(&mut self) -> &mut ActionBase;

    /// Make a single attempt to apply this Action (C++ virtual `apply`).
    ///
    /// Returns the number of changes via `base_mut().count` increments (as in
    /// C++) and an `int4`: `0` for complete application, `-1` for a partial
    /// completion (breakpoint).  `ctx` carries the engine-side sinks the C++
    /// reaches through `glb`.
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult;

    /// Clone this Action filtered by `grouplist` (C++ virtual
    /// `clone(grouplist)`).  Returns `None` if the action is filtered out.
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>>;

    /// Reset the Action for a new function (C++ virtual `reset`).  The default
    /// resets only the base; containers override to recurse.
    fn reset(&mut self, _data: &mut Funcdata) {
        self.base_mut().reset_base();
    }

    /// Reset the statistics (C++ virtual `resetStats`).  Containers override to
    /// recurse.
    fn reset_stats(&mut self) {
        self.base_mut().reset_stats_base();
    }

    /// Get the Action's name (C++ `getName`).
    fn get_name(&self) -> &str {
        &self.base().name
    }
    /// Get the Action's group (C++ `getGroup`).
    fn get_group(&self) -> &str {
        &self.base().basegroup
    }
    /// Get the current status (C++ `getStatus`).
    fn get_status(&self) -> uint4 {
        self.base().status
    }

    /// Clear all breakpoints set on this Action (C++ virtual
    /// `clearBreakPoints`).  Containers override to recurse.
    fn clear_break_points(&mut self) {
        self.base_mut().breakpoint = 0;
    }

    /// Retrieve a specific sub-action by name (C++ virtual `getSubAction`).
    /// Returns `true` if a (sub)action with that name path was found and the
    /// closure was applied to its base.  The default matches only `this`.
    fn with_sub_action(&mut self, specify: &str, f: &mut dyn FnMut(&mut ActionBase)) -> bool {
        if self.base().name == specify {
            f(self.base_mut());
            return true;
        }
        false
    }

    /// Retrieve a specific sub-rule by name path (C++ virtual `getSubRule`).
    /// Returns `true` if a rule was found and `f` applied to its [`RuleState`].
    /// Leaf actions own no rules, so the default never matches; [`ActionPool`]
    /// and the container actions override.  This is the W9 console's rule
    /// control surface (`disableRule`/`setBreak`/...).
    fn with_sub_rule(&mut self, _specify: &str, _f: &mut dyn FnMut(&mut RuleState)) -> bool {
        false
    }

    /// Set a breakpoint on this Action or a sub-(action|rule) by name path (C++
    /// `Action::setBreakPoint`).  `tp` is a [`breakflags`] word.  Returns `true`
    /// if a target was found.  The W9 console drives this.
    fn set_break_point(&mut self, tp: uint4, specify: &str) -> bool {
        if self.with_sub_action(specify, &mut |b| b.breakpoint |= tp) {
            return true;
        }
        self.with_sub_rule(specify, &mut |r| r.breakpoint |= tp)
    }

    /// Toggle warnings on this Action or a sub-(action|rule) by name path (C++
    /// `Action::setWarning`).  Returns `true` if a target was found.
    fn set_warning(&mut self, val: bool, specify: &str) -> bool {
        if self.with_sub_action(specify, &mut |b| {
            if val {
                b.turn_on_warnings();
            } else {
                b.turn_off_warnings();
            }
        }) {
            return true;
        }
        self.with_sub_rule(specify, &mut |r| {
            if val {
                r.flags |= typeflags::warnings_on;
            } else {
                r.flags &= !typeflags::warnings_on;
            }
        })
    }

    /// Disable a specific Rule by name path within this Action (C++
    /// `Action::disableRule`).  Returns `true` if the rule was found.
    fn disable_rule(&mut self, specify: &str) -> bool {
        self.with_sub_rule(specify, &mut |r| r.set_disable())
    }

    /// Enable a specific Rule by name path within this Action (C++
    /// `Action::enableRule`).  Returns `true` if the rule was found.
    fn enable_rule(&mut self, specify: &str) -> bool {
        self.with_sub_rule(specify, &mut |r| r.clear_disable())
    }

    /// Run this Action until completion or a breakpoint (C++ non-virtual
    /// `Action::perform`, `action.cc:298`).  Returns the change count, or `-1`
    /// on a breakpoint.  Resumes from a prior partial return via the `status`
    /// state.
    fn perform(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        loop {
            // --- the status switch with C++ fall-through preserved ---------
            let st = self.base().status;
            if st == statusflags::status_start {
                self.base_mut().count = 0; // No changes made yet by this action
                if self.base_mut().check_start_break() {
                    self.base_mut().status = statusflags::status_breakstarthit;
                    return -1; // Indicate partial completion
                }
                self.base_mut().count_tests += 1;
            }
            // status_start falls through to breakstarthit/repeat ...
            if st == statusflags::status_start
                || st == statusflags::status_breakstarthit
                || st == statusflags::status_repeat
            {
                self.base_mut().lcount = self.base().count;
            }
            // ... which falls through to status_mid (the apply body).
            if st == statusflags::status_start
                || st == statusflags::status_breakstarthit
                || st == statusflags::status_repeat
                || st == statusflags::status_mid
            {
                let res = self.apply(data, ctx); // Start or continue action
                if res < 0 {
                    // negative indicates partial completion
                    self.base_mut().status = statusflags::status_mid;
                    return res;
                } else if self.base().lcount < self.base().count {
                    // Action has been applied
                    let (mut_base, sink) = (self.base_mut(), &mut ctx.warnings);
                    mut_base.issue_warning(sink);
                    self.base_mut().count_apply += 1;
                    if self.base_mut().check_action_break() {
                        self.base_mut().status = statusflags::status_actionbreak;
                        return -1; // Indicate action breakpoint
                    }
                }
                // (the apply body's C++ `break` only exits the switch)
            } else if st == statusflags::status_end {
                return 0; // Rule applied, do not repeat until reset
            } else if st == statusflags::status_actionbreak {
                // Returned -1 last time, but we do not reapply; we either repeat
                // or return our count.  Falls through to the loop tail.
            }

            self.base_mut().status = statusflags::status_repeat;

            // while((lcount<count)&&((flags&rule_repeatapply)!=0))
            if !((self.base().lcount < self.base().count)
                && ((self.base().flags & ruleflags::rule_repeatapply) != 0))
            {
                break;
            }
            // (kuna decompile-all watchdog) Cooperative deadline: a repeatapply
            // action that keeps finding changes must still yield once the
            // per-function budget expires (a no-op compare when no budget).
            if ctx.deadline_expired() {
                break;
            }
        }

        if (self.base().flags & (ruleflags::rule_onceperfunc | ruleflags::rule_oneactperfunc)) != 0 {
            if (self.base().count > 0)
                || ((self.base().flags & ruleflags::rule_onceperfunc) != 0)
            {
                self.base_mut().status = statusflags::status_end;
            } else {
                self.base_mut().status = statusflags::status_start;
            }
        } else {
            self.base_mut().status = statusflags::status_start;
        }

        self.base().count
    }
}

/// The engine-side context the C++ reaches through `glb` (the console message
/// channel and, later, the comment database).  Threaded into every
/// [`Action::apply`] / [`Action::perform`] so the warning emission points are
/// observable.  // LOCAL STUB(W4/W9)
#[derive(Debug, Default)]
pub struct ActionContext {
    /// Where `issueWarning` / `warningHeader` text lands.
    pub warnings: WarningSink,
    /// Whether the restart loop ([`ActionRestartGroup`]) can re-follow flow after
    /// a `data.clear()` (C++ `ActionStart::startProcessing` → `followFlow`).
    ///
    /// The Rust flow follower needs the [`FlowEnvironment`](crate::flow::FlowEnvironment)
    /// (the SLEIGH translator + typeop table), which lives on the `Architecture`,
    /// not on the IR-boundary `glb` handle the action loop carries.  Until that is
    /// threaded into the action loop (`ActionStart` re-flow port), a restart cannot
    /// rebuild the IR — so when this is `false` the restart group completes with
    /// the *current* (already-analyzed) IR instead of clearing it to an empty
    /// function.  See `LOSS` in the call-site recovery review (restart re-flow is
    /// the next blocker).
    pub can_reflow: bool,
    /// Set by [`ActionRestartGroup::apply`] when a restart is pending but the
    /// action loop itself cannot re-follow flow (`!can_reflow`): the outer driver
    /// ([`crate::decompile_drive`]'s `run_pipeline`) owns the `Architecture` and
    /// the SLEIGH translator, so it re-follows flow and re-performs the root.  The
    /// `restart_pending` flag on the Funcdata is left SET in this case (the outer
    /// loop consumes it after re-flow), unlike the in-loop restart which clears it.
    pub reflow_requested: bool,
    /// (kuna decompile-all watchdog) Optional wall-clock deadline for the
    /// per-function decompile drive (`kuna decompile-all --max-fn-seconds N`).
    /// Consulted **cooperatively** at the action boundaries ([`ActionGroup::apply`],
    /// [`ActionRestartGroup::apply`], [`Action::perform`]'s repeat gate) and every
    /// [`POOL_DEADLINE_STRIDE`] ops in the [`ActionPool`] rule loop; on expiry the
    /// containers stop scheduling further work and unwind with their current
    /// counts, and the outer driver (`run_pipeline`) turns the expiry into a
    /// per-function `Err`.  `None` (every path except `decompile-all` with a
    /// budget) is a no-op compare — the console/`decomp_dbg` parity pipeline
    /// never sets it.
    pub deadline: Option<std::time::Instant>,
}

/// How many [`ActionPool`] op visits between deadline probes (the tight rule
/// loop only pays an `Instant::now()` every this-many ops).
const POOL_DEADLINE_STRIDE: u32 = 1024;

impl ActionContext {
    /// A fresh context.
    pub fn new() -> ActionContext {
        ActionContext::default()
    }

    /// (kuna decompile-all watchdog) `true` once the cooperative per-function
    /// [`deadline`](ActionContext::deadline) has passed.  A single
    /// `Instant::now()` compare; `None` (no budget — every console/parity path)
    /// short-circuits without touching the clock.
    #[inline]
    pub fn deadline_expired(&self) -> bool {
        match self.deadline {
            Some(d) => std::time::Instant::now() >= d,
            None => false,
        }
    }
}

// =============================================================================
// ActionGroup (action.cc:364-527)
// =============================================================================

/// A group of [`Action`]s applied in sequence (C++ `ActionGroup`).
pub struct ActionGroup {
    base: ActionBase,
    /// List of actions to perform in the group (C++ `list`).
    list: Vec<Box<dyn Action>>,
    /// Index of the action currently being applied (C++ `state` iterator).
    state: usize,
}

impl ActionGroup {
    /// Construct given properties and a name (C++
    /// `ActionGroup::ActionGroup(uint4,string)` — group is empty).
    pub fn new(f: uint4, nm: impl Into<String>) -> ActionGroup {
        ActionGroup { base: ActionBase::new(f, nm, ""), list: Vec::new(), state: 0 }
    }

    /// Add an Action to the end of this group's list (C++
    /// `ActionGroup::addAction`).
    pub fn add_action(&mut self, ac: Box<dyn Action>) {
        self.list.push(ac);
    }

    /// Clone the contained list filtered by `grouplist`, returning the surviving
    /// boxed actions.  Shared by [`ActionGroup`] and [`ActionRestartGroup`]
    /// (C++ both loop `(*iter)->clone(grouplist)`).  Returns `None` if every
    /// child was filtered out (the C++ `res == 0` case → whole group drops).
    fn clone_children(&self, grouplist: &ActionGroupList) -> Option<Vec<Box<dyn Action>>> {
        let mut res: Option<Vec<Box<dyn Action>>> = None;
        for child in &self.list {
            if let Some(ac) = child.clone_filtered(grouplist) {
                res.get_or_insert_with(Vec::new).push(ac);
            }
        }
        res
    }
}

impl Action for ActionGroup {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        self.clone_children(grouplist).map(|list| {
            Box::new(ActionGroup { base: self.base.clone(), list, state: 0 }) as Box<dyn Action>
        })
    }

    /// C++ `ActionGroup::reset`: reset base, then each sub-action.
    fn reset(&mut self, data: &mut Funcdata) {
        self.base.reset_base();
        for child in &mut self.list {
            child.reset(data);
        }
    }

    /// C++ `ActionGroup::resetStats`: reset base, then each sub-action.
    fn reset_stats(&mut self) {
        self.base.reset_stats_base();
        for child in &mut self.list {
            child.reset_stats();
        }
    }

    fn clear_break_points(&mut self) {
        for child in &mut self.list {
            child.clear_break_points();
        }
        self.base.breakpoint = 0;
    }

    /// C++ `ActionGroup::getSubAction` — the disambiguating `:`-path search.
    /// Transcribed: a token matching this group's name strips one path segment;
    /// otherwise the whole `specify` must still match a child.  More than one
    /// match yields no match (`matchcount > 1` → `None`).
    fn with_sub_action(&mut self, specify: &str, f: &mut dyn FnMut(&mut ActionBase)) -> bool {
        let (token, remain0) = next_specify_term(specify);
        let remain: String = if self.base.name == token {
            if remain0.is_empty() {
                f(&mut self.base);
                return true;
            }
            remain0
        } else {
            specify.to_string() // Still have to match entire specify
        };

        // Probe children: count matches, apply to the last, reject on ambiguity.
        let mut match_idx: Option<usize> = None;
        let mut matchcount = 0;
        for (i, child) in self.list.iter_mut().enumerate() {
            let mut hit = false;
            child.with_sub_action(&remain, &mut |_| {
                hit = true;
            });
            if hit {
                match_idx = Some(i);
                matchcount += 1;
                if matchcount > 1 {
                    return false;
                }
            }
        }
        match match_idx {
            Some(i) => self.list[i].with_sub_action(&remain, f),
            None => false,
        }
    }

    /// C++ `ActionGroup::getSubRule`, `action.cc:481` — same `:`-path search as
    /// [`with_sub_action`](ActionGroup::with_sub_action) but descending to a
    /// rule.  A token matching this group's name with no remainder yields no
    /// rule (a group is not a rule).
    fn with_sub_rule(&mut self, specify: &str, f: &mut dyn FnMut(&mut RuleState)) -> bool {
        let (token, remain0) = next_specify_term(specify);
        let remain: String = if self.base.name == token {
            if remain0.is_empty() {
                return false; // matched the group, but a group is not a rule
            }
            remain0
        } else {
            specify.to_string()
        };
        let mut match_idx: Option<usize> = None;
        let mut matchcount = 0;
        for (i, child) in self.list.iter_mut().enumerate() {
            let mut hit = false;
            child.with_sub_rule(&remain, &mut |_| {
                hit = true;
            });
            if hit {
                match_idx = Some(i);
                matchcount += 1;
                if matchcount > 1 {
                    return false;
                }
            }
        }
        match match_idx {
            Some(i) => self.list[i].with_sub_rule(&remain, f),
            None => false,
        }
    }

    /// C++ `ActionGroup::apply`, `action.cc:506`.
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        if self.base.status != statusflags::status_mid {
            self.state = 0; // Initialize the derived action (list.begin())
        }
        while self.state < self.list.len() {
            // (kuna decompile-all watchdog) Cooperative deadline at every action
            // boundary: stop scheduling further sub-actions once the per-function
            // budget expires and unwind with the current count (the outer driver
            // `run_pipeline` turns the expiry into a per-function `Err`).  A
            // no-op compare when no budget is set (every console/parity path).
            if ctx.deadline_expired() {
                return 0;
            }
            let res = self.list[self.state].perform(data, ctx);
            if res > 0 {
                // A change was made
                self.base.count += res;
                if self.base.check_action_break() {
                    self.state += 1;
                    return -1;
                }
            } else if res < 0 {
                // Partial completion of member == partial completion of group
                return -1;
            }
            self.state += 1;
        }
        0 // Indicate successful completion
    }
}

// =============================================================================
// ActionRestartGroup (action.cc:529-582)
// =============================================================================

/// An [`ActionGroup`] that re-runs itself when a sub-action requests a restart
/// (C++ `ActionRestartGroup`).
pub struct ActionRestartGroup {
    group: ActionGroup,
    /// Maximum number of restarts allowed (C++ `maxrestarts`).
    maxrestarts: int4,
    /// Current restart iteration (C++ `curstart`; `-1` means completed).
    curstart: int4,
}

impl ActionRestartGroup {
    /// Construct providing the maximum number of restarts (C++
    /// `ActionRestartGroup::ActionRestartGroup(uint4,string,int4)`).
    pub fn new(f: uint4, nm: impl Into<String>, max: int4) -> ActionRestartGroup {
        ActionRestartGroup { group: ActionGroup::new(f, nm), maxrestarts: max, curstart: 0 }
    }

    /// Add an Action to the underlying group (C++ inherited `addAction`).
    pub fn add_action(&mut self, ac: Box<dyn Action>) {
        self.group.add_action(ac);
    }
}

impl Action for ActionRestartGroup {
    fn base(&self) -> &ActionBase {
        &self.group.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.group.base
    }

    /// C++ `ActionRestartGroup::clone` — clones the children (filtered) into a
    /// new restart group, dropping the whole group if no child survives.
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        self.group.clone_children(grouplist).map(|list| {
            Box::new(ActionRestartGroup {
                group: ActionGroup { base: self.group.base.clone(), list, state: 0 },
                maxrestarts: self.maxrestarts,
                curstart: 0,
            }) as Box<dyn Action>
        })
    }

    /// C++ `ActionRestartGroup::reset`: zero `curstart`, then group reset.
    fn reset(&mut self, data: &mut Funcdata) {
        self.curstart = 0;
        self.group.reset(data);
    }

    fn reset_stats(&mut self) {
        self.group.reset_stats();
    }

    fn clear_break_points(&mut self) {
        self.group.clear_break_points();
    }

    fn with_sub_action(&mut self, specify: &str, f: &mut dyn FnMut(&mut ActionBase)) -> bool {
        self.group.with_sub_action(specify, f)
    }

    fn with_sub_rule(&mut self, specify: &str, f: &mut dyn FnMut(&mut RuleState)) -> bool {
        self.group.with_sub_rule(specify, f)
    }

    /// C++ `ActionRestartGroup::apply`, `action.cc:553`.
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        if self.curstart == -1 {
            return 0; // Already completed
        }
        loop {
            let res = self.group.apply(data, ctx);
            if res != 0 {
                return res;
            }
            // (kuna decompile-all watchdog) Never begin another restart iteration
            // past the per-function deadline — the observed non-convergence hang
            // cycles heritage/deadcode restarts forever.  Return with the current
            // state; `run_pipeline` converts the expiry into a per-function `Err`.
            if ctx.deadline_expired() {
                return 0;
            }
            if !data.has_restart_pending() {
                self.curstart = -1;
                return 0;
            }
            if data.is_jumptable_recovery_on() {
                // Don't restart within jumptable recovery
                return 0;
            }
            if !ctx.can_reflow {
                // The C++ restart re-follows flow (`startProcessing` →
                // `followFlow`) to rebuild the IR; the Rust flow follower needs the
                // `Architecture`'s SLEIGH translator, which is NOT available inside
                // the action loop (the loop carries only the IR-boundary `glb`
                // handle).  Hand the restart up to the outer driver
                // (`run_pipeline`), which owns `&mut Architecture`: it re-follows
                // flow and re-performs this root.  Leave `restart_pending` SET so
                // the outer loop sees the request; signal it via the context.
                ctx.reflow_requested = true;
                return 0;
            }
            self.curstart += 1;
            if self.curstart > self.maxrestarts {
                // C++ `data.warningHeader("Exceeded maximum restarts with more pending");`
                ctx.warnings.messages.push(warning_header_text(
                    data,
                    "Exceeded maximum restarts with more pending",
                ));
                self.curstart = -1;
                return 0;
            }
            // C++ `data.getArch()->clearAnalysis(&data);` — fd->clear() plus the
            // comment-DB clear (comment subsystem not yet ported // STUB(W4)).
            data.clear();

            // Reset everything but ourselves (the sub-actions, not the group base).
            for child in &mut self.group.list {
                child.reset(data);
            }
            self.group.base.status = statusflags::status_start;
        }
    }
}

/// Reproduce `Funcdata::warningHeader`'s text prefix (`funcdata.cc:135`): the
/// jumptable-recovery flag selects `"WARNING (jumptable): "` vs `"WARNING: "`.
/// The comment-DB insertion is the W4 seam; only the text is formed here.
fn warning_header_text(data: &Funcdata, txt: &str) -> String {
    if data.is_jumptable_recovery_on() {
        format!("WARNING (jumptable): {txt}")
    } else {
        format!("WARNING: {txt}")
    }
}

// =============================================================================
// The Rule trait (action.hh:194-254)
// =============================================================================

/// A single transformation on a [`PcodeOp`] (C++ `Rule`).
///
/// Concrete rules implement [`apply_op`](Rule::apply_op) (the transformation),
/// [`clone_rule`](Rule::clone_rule) (the group-filter clone), and optionally
/// [`get_op_list`](Rule::get_op_list) (the opcode whitelist).  All per-rule
/// engine state (name/group/flags/breakpoints/statistics) lives in the
/// [`RuleState`] the owning [`ActionPool`] holds alongside the rule object —
/// see the module docs for the registration convention.
pub trait Rule {
    /// Attempt to apply this Rule at `op` (C++ virtual `applyOp`).  Returns `1`
    /// (non-zero) if it applied and made changes, `0` otherwise.  A rule that
    /// changes `op`'s opcode MUST return non-zero (the C++ enforces this with
    /// an error message — see [`ActionPool::process_op`]).
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4;

    /// List of op codes this rule operates on (C++ virtual `getOpList`).  The
    /// default is *every* opcode (the C++ `Rule::getOpList` default pushes
    /// `0..CPUI_MAX`).
    fn get_op_list(&self) -> Vec<OpCode> {
        all_opcodes()
    }

    /// Clone this Rule filtered by `grouplist` (C++ virtual `clone(grouplist)`).
    /// Returns `None` if filtered out.  The implementor decides membership,
    /// conventionally `grouplist.contains(my_group)`; the engine supplies the
    /// group string to the new instance.
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>>;

    /// Reset any function-specific state (C++ virtual `reset`).  The default is
    /// a no-op; the engine separately clears the [`RuleState`] warning latch.
    fn reset(&mut self, _data: &mut Funcdata) {}
}

/// Engine-owned per-[`Rule`] bookkeeping (C++ `Rule`'s private members).
///
/// The prompt mandates the engine wrapper own counts/breakpoints/warnings/
/// flags; this is that store.  An [`ActionPool`] holds one of these next to
/// each boxed rule.
#[derive(Debug, Clone)]
pub struct RuleState {
    /// Properties (C++ `flags`, a [`typeflags`] word).
    pub flags: uint4,
    /// Breakpoint(s) enabled (C++ `breakpoint`, a [`breakflags`] word).
    pub breakpoint: uint4,
    /// Name of the Rule (C++ `name`).
    pub name: String,
    /// Group to which this Rule belongs (C++ `basegroup`).
    pub basegroup: String,
    /// Number of attempted applications (C++ `count_tests`).
    pub count_tests: uint4,
    /// Number of successful applications (C++ `count_apply`).
    pub count_apply: uint4,
}

impl RuleState {
    /// Construct given group, properties, name (C++
    /// `Rule::Rule(string,uint4,string)`).
    pub fn new(g: impl Into<String>, fl: uint4, nm: impl Into<String>) -> RuleState {
        RuleState {
            flags: fl,
            breakpoint: 0,
            name: nm.into(),
            basegroup: g.into(),
            count_tests: 0,
            count_apply: 0,
        }
    }

    /// Return `true` if disabled (C++ `Rule::isDisabled`).
    pub fn is_disabled(&self) -> bool {
        (self.flags & typeflags::type_disable) != 0
    }
    /// Disable this Rule within its pool (C++ `Rule::setDisable`).
    pub fn set_disable(&mut self) {
        self.flags |= typeflags::type_disable;
    }
    /// Enable this Rule within its pool (C++ `Rule::clearDisable`).
    pub fn clear_disable(&mut self) {
        self.flags &= !typeflags::type_disable;
    }

    /// Check if an action breakpoint is on, clearing the temporary one (C++
    /// `Rule::checkActionBreak`).
    fn check_action_break(&mut self) -> bool {
        if (self.breakpoint & (breakflags::break_action | breakflags::tmpbreak_action)) != 0 {
            self.breakpoint &= !breakflags::tmpbreak_action;
            return true;
        }
        false
    }

    /// C++ `Rule::issueWarning` — see [`ActionBase::issue_warning`] for why this
    /// routes to the [`WarningSink`].
    fn issue_warning(&mut self, sink: &mut WarningSink) {
        if (self.flags & (typeflags::warnings_on | typeflags::warnings_given))
            == typeflags::warnings_on
        {
            self.flags |= typeflags::warnings_given;
            sink.messages.push(format!("WARNING: Applied rule {}", self.name));
        }
    }

    /// Reset for a new function (C++ `Rule::reset`): clear the warning latch.
    fn reset_state(&mut self) {
        self.flags &= !typeflags::warnings_given;
    }
}

/// A boxed [`Rule`] together with its engine-owned [`RuleState`], the unit an
/// [`ActionPool`] manages.
struct RuleEntry {
    state: RuleState,
    rule: Box<dyn Rule>,
}

impl RuleEntry {
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<RuleEntry> {
        self.rule
            .clone_rule(grouplist)
            .map(|rule| RuleEntry { state: self.state.clone(), rule })
    }
}

/// A registration row for a [`Rule`] (the W5 convention).  `group` is the rule's
/// stage group; `ctor` builds a fresh boxed rule.  The pool builder reads these
/// to populate [`ActionPool`]s — see the module docs.
pub struct RuleSpec {
    /// The rule's stage group (matched against [`ActionGroupList`]).
    pub group: &'static str,
    /// Constructor for a fresh instance of the rule.
    pub ctor: fn() -> Box<dyn Rule>,
}

// =============================================================================
// ActionPool (action.cc:728-972)
// =============================================================================

/// Per-opcode rule list — a `SmallVec` because most opcodes attract only a
/// handful of rules (the C++ `vector<Rule*> perop[CPUI_MAX]`).  Holds *indices*
/// into [`ActionPool::allrules`] (the C++ holds `Rule*` aliases; an index is the
/// arena-safe equivalent and preserves insertion order).
type PerOp = SmallVec<[usize; 4]>;

/// A pool of [`Rule`]s applied simultaneously to every [`PcodeOp`] (C++
/// `ActionPool`).
pub struct ActionPool {
    base: ActionBase,
    /// The set of rules in this pool, in insertion order (C++ `allrules`).
    allrules: Vec<RuleEntry>,
    /// Rules associated with each OpCode, in insertion order — a flat
    /// `CPUI_MAX`-element table indexed by `opc as usize`, exactly the C++
    /// `vector<Rule*> perop[CPUI_MAX]`.  An unregistered opcode is an empty
    /// list.  (Was a `BTreeMap<OpCode,_>`; the per-op rule loop probes this
    /// millions of times, so O(1) indexing replaces the O(log n) tree walk.)
    perop: Vec<PerOp>,
    /// Resumable cursor over the function's ops (C++ `op_state`, a
    /// `PcodeOpTree::const_iterator`).  `None` before the first op / after the
    /// last; otherwise the [`SeqNum`] of the *next* op to visit (see module
    /// docs for why this is a key, not a live iterator).
    op_state: OpCursor,
    /// Iterator over rules for one OpCode (C++ `rule_index`).
    rule_index: usize,
    /// Messages emitted mid-[`process_op`] (rule warnings).  `process_op` is a
    /// faithful `(op, data)` 1:1 of the C++ body and so cannot also carry the
    /// [`ActionContext`]; warnings stash here and [`drain_into`](ActionPool::drain_into)
    /// flushes them at the [`apply`](ActionPool::apply) boundary.  // LOCAL STUB(W4/W9)
    warnings: WarningSink,
    /// An invariant-violation message (C++ "Rule changed op without returning
    /// result of 1!") stashed for the same reason as [`warnings`](ActionPool::warnings).
    pending_error: Option<String>,
}

/// The resumable op cursor.  Modelled as the last *consumed* [`SeqNum`] so the
/// next op is "the first optree key strictly greater than this", which is
/// exactly what the C++ `op_state` map iterator yields and which survives
/// `opDeadAndGone` erasing the current op (the C++ advances `op_state++` before
/// the erase for precisely this reason).
#[derive(Debug, Clone, Default)]
enum OpCursor {
    /// Not yet started this pass (C++ iterator == `begin()` not yet taken).
    #[default]
    Unstarted,
    /// The next op to visit is the first optree entry `> last`.
    After(SeqNum),
    /// Past the end (C++ iterator == `end()`).
    Done,
}

impl ActionPool {
    /// Construct providing properties and name (C++
    /// `ActionPool::ActionPool(uint4,string)` — group empty).
    pub fn new(f: uint4, nm: impl Into<String>) -> ActionPool {
        ActionPool {
            base: ActionBase::new(f, nm, ""),
            allrules: Vec::new(),
            perop: vec![PerOp::new(); OpCode::CPUI_MAX as usize],
            op_state: OpCursor::Unstarted,
            rule_index: 0,
            warnings: WarningSink::new(),
            pending_error: None,
        }
    }

    /// Add a Rule to the pool with its engine state (C++ `ActionPool::addRule`).
    /// The rule's [`get_op_list`](Rule::get_op_list) is consulted and the rule's
    /// index is appended to each opcode's list, preserving insertion order.
    pub fn add_rule(&mut self, state: RuleState, rule: Box<dyn Rule>) {
        let idx = self.allrules.len();
        let oplist = rule.get_op_list();
        self.allrules.push(RuleEntry { state, rule });
        for opc in oplist {
            self.perop[opc as usize].push(idx);
        }
    }

    /// Convenience: register from a [`RuleSpec`] (the W5 path).  Builds the
    /// [`RuleState`] from the spec's group and the rule's debug name.
    pub fn add_spec(&mut self, spec: &RuleSpec, name: impl Into<String>) {
        let rule = (spec.ctor)();
        self.add_rule(RuleState::new(spec.group, 0, name), rule);
    }

    /// Number of rules in the pool (test/inspection helper).
    pub fn num_rules(&self) -> usize {
        self.allrules.len()
    }

    /// The per-opcode rule list for `opc`, by name (test/inspection helper).
    pub fn perop_names(&self, opc: OpCode) -> Vec<String> {
        match self.perop.get(opc as usize) {
            Some(list) => list.iter().map(|&i| self.allrules[i].state.name.clone()).collect(),
            None => Vec::new(),
        }
    }

    /// Apply the next possible Rule(s) to a PcodeOp (C++ `ActionPool::processOp`,
    /// `action.cc:822-877`) — transcribed exactly.
    ///
    /// Returns `0` if no breakpoint, `-1` on a breakpoint (the caller then
    /// re-enters to resume).  The `op_state` cursor and `rule_index` carry the
    /// resume position.
    fn process_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->isDead()) { op_state++; data.opDeadAndGone(op); rule_index=0; return 0; }
        if data.obank().get(op).map(|o| o.is_dead()).unwrap_or(true) {
            self.advance_op_state(data, op);
            // data.opDeadAndGone(op) == obank.destroy(op)
            data.obank_mut().destroy(op);
            self.rule_index = 0;
            return 0;
        }
        let mut opc = data.obank().get(op).expect("process_op: live op vanished").code();
        while self.rule_index < self.perop_len(opc) {
            let rl_idx = self.perop_idx(opc, self.rule_index);
            self.rule_index += 1;
            if self.allrules[rl_idx].state.is_disabled() {
                continue;
            }
            self.allrules[rl_idx].state.count_tests += 1;
            let res = self.allrules[rl_idx].rule.apply_op(op, data);
            if res > 0 {
                self.allrules[rl_idx].state.count_apply += 1;
                self.base.count += res;
                // rl->issueWarning(data.getArch()): route to the pool's stash.
                self.allrules[rl_idx].state.issue_warning(&mut self.warnings);
                if self.allrules[rl_idx].state.check_action_break() {
                    return -1;
                }
                // if (op->isDead()) break;
                if data.obank().get(op).map(|o| o.is_dead()).unwrap_or(true) {
                    break;
                }
                let newopc = data.obank().get(op).expect("process_op: op vanished").code();
                if opc != newopc {
                    // Set of rules to apply to this op has changed
                    opc = newopc;
                    self.rule_index = 0;
                }
            } else {
                let newopc = match data.obank().get(op) {
                    Some(o) => o.code(),
                    None => break, // op went away without returning a result (treat as dead)
                };
                if opc != newopc {
                    // C++ prints an ERROR via glb->printMessage; routed to the
                    // sink so the invariant violation stays observable. // STUB(W4)
                    self.pending_error = Some(format!(
                        "ERROR: Rule {} changed op without returning result of 1!",
                        self.allrules[rl_idx].state.name
                    ));
                    opc = newopc;
                    self.rule_index = 0;
                }
            }
        }
        self.advance_op_state(data, op);
        self.rule_index = 0;
        0
    }

    /// Length of `perop[opc]` (C++ `perop[opc].size()`); 0 for an unregistered
    /// opcode.
    fn perop_len(&self, opc: OpCode) -> usize {
        self.perop.get(opc as usize).map(|v| v.len()).unwrap_or(0)
    }

    /// `perop[opc][i]` (C++ index into the per-opcode rule list).
    fn perop_idx(&self, opc: OpCode, i: usize) -> usize {
        self.perop[opc as usize][i]
    }

    /// Advance `op_state` past `op` (the C++ `op_state++`).  Records `op`'s
    /// SeqNum as the cursor so the next op is the first optree key strictly
    /// greater than it; if `op` was already the last, the cursor goes `Done`.
    fn advance_op_state(&mut self, data: &Funcdata, op: OpId) {
        // The op may already be destroyed (dead path destroys after this call);
        // read its SeqNum while it is still live in this function, else fall back
        // to the recorded cursor.  We always have a live `op` here because the
        // dead path advances before destroy and the live path advances after.
        let sq = data
            .obank()
            .get(op)
            .map(|o| o.get_seq_num().clone())
            .expect("advance_op_state: op already gone");
        // Is there any optree entry strictly greater than sq?  Only existence
        // matters here, so probe with first_after_seq (O(log n)) rather than
        // cloning the successor key just to throw it away.
        self.op_state = if data.obank().first_after_seq(&sq).is_some() {
            OpCursor::After(sq)
        } else {
            OpCursor::Done
        };
    }

    /// The op the cursor currently points at (C++ `(*op_state).second`), or
    /// `None` at end.  `Unstarted` points at the first optree entry.
    fn current_op(&self, data: &Funcdata) -> Option<OpId> {
        match &self.op_state {
            OpCursor::Unstarted => data.obank().iter_all().next().map(|(_, id)| id),
            OpCursor::After(sq) => first_op_after(data, sq),
            OpCursor::Done => None,
        }
    }

    /// Flush the mid-[`process_op`] warning/error stash into the context at the
    /// [`apply`](ActionPool::apply) boundary.
    fn drain_into(&mut self, ctx: &mut ActionContext) {
        ctx.warnings.messages.append(&mut self.warnings.messages);
        if let Some(err) = self.pending_error.take() {
            ctx.warnings.messages.push(err);
        }
    }
}

impl Action for ActionPool {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    /// C++ `ActionPool::clone` — clone surviving rules into a fresh pool,
    /// dropping the pool entirely if none survive.
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        let mut res: Option<ActionPool> = None;
        for entry in &self.allrules {
            if let Some(cloned) = entry.clone_filtered(grouplist) {
                let pool = res.get_or_insert_with(|| {
                    ActionPool::new(self.base.flags, self.base.name.clone())
                });
                pool.add_rule(cloned.state, cloned.rule);
            }
        }
        res.map(|p| Box::new(p) as Box<dyn Action>)
    }

    /// C++ `ActionPool::reset`: reset base, then each rule's state and object.
    fn reset(&mut self, data: &mut Funcdata) {
        self.base.reset_base();
        for entry in &mut self.allrules {
            entry.state.reset_state();
            entry.rule.reset(data);
        }
    }

    fn reset_stats(&mut self) {
        self.base.reset_stats_base();
        for entry in &mut self.allrules {
            entry.state.count_tests = 0;
            entry.state.count_apply = 0;
        }
    }

    fn clear_break_points(&mut self) {
        for entry in &mut self.allrules {
            entry.state.breakpoint = 0;
        }
        self.base.breakpoint = 0;
    }

    /// C++ `ActionPool::apply`, `action.cc:877`.
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        if self.base.status != statusflags::status_mid {
            self.op_state = OpCursor::Unstarted; // op_state = data.beginOpAll()
            self.rule_index = 0;
        }
        // for(;op_state!=data.endOpAll();)
        let mut ops_since_deadline_probe: u32 = 0;
        while let Some(op) = self.current_op(data) {
            // (kuna decompile-all watchdog) Cooperative deadline in the tight
            // rule loop, probed only every POOL_DEADLINE_STRIDE ops so the hot
            // path stays cheap.  On expiry, stop visiting ops and report
            // completion with the current count; the outer driver errors out.
            ops_since_deadline_probe += 1;
            if ops_since_deadline_probe >= POOL_DEADLINE_STRIDE {
                ops_since_deadline_probe = 0;
                if ctx.deadline_expired() {
                    self.drain_into(ctx);
                    return 0;
                }
            }
            let r = self.process_op(op, data);
            if r != 0 {
                self.drain_into(ctx);
                return -1;
            }
        }
        self.drain_into(ctx);
        0 // Indicate successful completion
    }

    fn with_sub_action(&mut self, specify: &str, f: &mut dyn FnMut(&mut ActionBase)) -> bool {
        // A pool matches its own name (C++ ActionPool has no getSubAction
        // override, so the base Action::getSubAction matches `this`).
        if self.base.name == specify {
            f(&mut self.base);
            return true;
        }
        false
    }

    /// C++ `ActionPool::getSubRule`, `action.cc:789` — strip the pool name from
    /// the path, then match the remainder against rule names exactly; ambiguity
    /// (more than one match) yields no rule.
    fn with_sub_rule(&mut self, specify: &str, f: &mut dyn FnMut(&mut RuleState)) -> bool {
        let (token, remain0) = next_specify_term(specify);
        let remain: String = if self.base.name == token {
            if remain0.is_empty() {
                return false; // Match, but not a rule
            }
            remain0
        } else {
            specify.to_string()
        };
        let mut match_idx: Option<usize> = None;
        let mut matchcount = 0;
        for (i, entry) in self.allrules.iter().enumerate() {
            if entry.state.name == remain {
                match_idx = Some(i);
                matchcount += 1;
                if matchcount > 1 {
                    return false;
                }
            }
        }
        match match_idx {
            Some(i) => {
                f(&mut self.allrules[i].state);
                true
            }
            None => false,
        }
    }
}

// =============================================================================
// Free helpers
// =============================================================================

/// Every real opcode (the C++ `Rule::getOpList` default range is `0..CPUI_MAX`).
/// Slots 0 (`"BLANK"`) and 45 (`"UNUSED1"`) are not real operations — no
/// PcodeOp ever carries them, so the C++ `perop[0]`/`perop[45]` lists are dead
/// storage; skipping them ([`OpCode::from_i32`] returns `None`) is observably
/// identical to registering them.
fn all_opcodes() -> Vec<OpCode> {
    let mut v = Vec::with_capacity(74);
    for raw in 1..(OpCode::CPUI_MAX as i32) {
        if let Some(opc) = OpCode::from_i32(raw) {
            v.push(opc);
        }
    }
    v
}

/// The op at the first optree entry strictly greater than `sq` (the C++
/// `++op_state` then deref).
fn first_op_after(data: &Funcdata, sq: &SeqNum) -> Option<OpId> {
    data.obank().first_after_seq(sq).map(|(_, id)| id)
}

/// Pull the next token from a `:`-separated list of Action/Rule names (C++
/// `next_specifyterm`, `action.cc:257`).  Returns `(token, remain)`.
fn next_specify_term(specify: &str) -> (String, String) {
    match specify.find(':') {
        Some(pos) => (specify[..pos].to_string(), specify[pos + 1..].to_string()),
        None => (specify.to_string(), String::new()),
    }
}

// =============================================================================
// ActionDatabase (action.cc:974-1160, build_default_groups @ coreaction.cc:5675)
// =============================================================================

/// The name of the *universal* root Action (C++ `ActionDatabase::universalname`).
pub const UNIVERSAL_NAME: &str = "universal";

/// Database of root [`Action`] objects (C++ `ActionDatabase`).
///
/// Owns the universal action tree, the group map ([`build_default_groups`]),
/// and the derived-action cache.  A *root* action is the universal tree
/// `clone_filtered`-filtered by the grouplist named for it.
///
/// The W5 wave installs the universal tree via [`set_universal`] (its builder
/// is `universal_action`, the transcription of `coreaction.cc:5722`, which lives
/// in W5 because it instantiates the 170 concrete rules).  This module supplies
/// the *mechanism*: the group map, derivation, and the filtered-clone semantics
/// that must be observably identical to C++ `clone(grouplist)`.
#[derive(Default)]
pub struct ActionDatabase {
    /// The name of the current root Action (C++ `currentactname`).  The current
    /// action itself is `actionmap[currentactname]` (the C++ `currentact` is a
    /// borrowed pointer into `actionmap`; an owned copy would desync from the
    /// breakpoints/disable flags the W9 console sets on the cached action, so we
    /// hold the name and index the map — observably identical to C++).
    currentactname: String,
    /// Map from root Action name to its grouplist (C++ `groupmap`).
    groupmap: BTreeMap<String, ActionGroupList>,
    /// Map from name to derived root Action (C++ `actionmap`).  The universal
    /// action is stored here under [`UNIVERSAL_NAME`].
    actionmap: BTreeMap<String, Box<dyn Action>>,
    /// `true` if only the default groups are set (C++ `isDefaultGroups`).
    is_default_groups: bool,
}

impl ActionDatabase {
    /// Constructor (C++ `ActionDatabase::ActionDatabase`).
    pub fn new() -> ActionDatabase {
        ActionDatabase::default()
    }

    /// Install the universal action tree (the W5 `universal_action` output).
    /// Registers it under [`UNIVERSAL_NAME`].  Replaces any prior universal.
    pub fn set_universal(&mut self, act: Box<dyn Action>) {
        self.register_action(UNIVERSAL_NAME, act);
    }

    /// Get the current root Action (C++ `getCurrent`).
    pub fn get_current(&self) -> Option<&dyn Action> {
        self.actionmap.get(&self.currentactname).map(|b| b.as_ref())
    }

    /// Mutable access to the current root Action (so a driver can `perform` it).
    pub fn get_current_mut(&mut self) -> Option<&mut (dyn Action + 'static)> {
        self.actionmap.get_mut(&self.currentactname).map(|b| b.as_mut())
    }

    /// Get the name of the current root Action (C++ `getCurrentName`).
    pub fn get_current_name(&self) -> &str {
        &self.currentactname
    }

    /// Get a specific grouplist by name (C++ `getGroup`).  Errors if absent.
    pub fn get_group(&self, grp: &str) -> KunaResult<&ActionGroupList> {
        self.groupmap
            .get(grp)
            .ok_or_else(|| KunaError::lowlevel(format!("Action group does not exist: {grp}")))
    }

    /// (Re)set the grouplist for a particular root Action (C++ `setGroup`).
    /// Do not use to redefine an already-instantiated root Action.
    pub fn set_group<I, S>(&mut self, grp: impl Into<String>, members: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let grp = grp.into();
        let curgrp = self.groupmap.entry(grp).or_default();
        curgrp.clear();
        for m in members {
            let s: String = m.into();
            // C++ stops at the first NULL/empty-string terminator.
            if s.is_empty() {
                break;
            }
            curgrp.insert(s);
        }
        self.is_default_groups = false;
    }

    /// Copy an existing root Action's grouplist under a new name (C++
    /// `cloneGroup`).
    pub fn clone_group(&mut self, oldname: &str, newname: impl Into<String>) -> KunaResult<()> {
        let curgrp = self.get_group(oldname)?.clone();
        self.groupmap.insert(newname.into(), curgrp);
        self.is_default_groups = false;
        Ok(())
    }

    /// Add a group to a root Action's grouplist (C++ `addToGroup`).  Returns
    /// `true` for a new addition.
    pub fn add_to_group(&mut self, grp: impl Into<String>, basegroup: impl Into<String>) -> bool {
        self.is_default_groups = false;
        let curgrp = self.groupmap.entry(grp.into()).or_default();
        curgrp.insert(basegroup)
    }

    /// Remove a group from a root Action's grouplist (C++ `removeFromGroup`).
    /// Returns `true` if it existed.
    pub fn remove_from_group(&mut self, grp: impl Into<String>, basegrp: &str) -> bool {
        self.is_default_groups = false;
        let curgrp = self.groupmap.entry(grp.into()).or_default();
        curgrp.erase(basegrp)
    }

    /// Look up a registered root Action by name (C++ `getAction`).  Errors if
    /// not registered.
    fn get_action(&self, nm: &str) -> KunaResult<&dyn Action> {
        self.actionmap
            .get(nm)
            .map(|b| b.as_ref())
            .ok_or_else(|| KunaError::lowlevel(format!("No registered action: {nm}")))
    }

    /// Register a root Action under a name, taking ownership (C++
    /// `registerAction`).  Replaces any prior registration.
    fn register_action(&mut self, nm: impl Into<String>, act: Box<dyn Action>) {
        self.actionmap.insert(nm.into(), act);
    }

    /// Set the current root Action, deriving it from the universal via its
    /// grouplist if not already derived (C++ `setCurrent`).  The derived action
    /// is cached in `actionmap`; the current action is then `actionmap[actname]`
    /// (see [`get_current`](ActionDatabase::get_current)).
    pub fn set_current(&mut self, actname: &str) -> KunaResult<()> {
        self.currentactname = actname.to_string();
        self.derive_action(UNIVERSAL_NAME, actname)?;
        Ok(())
    }

    /// Derive (or fetch the cached) root Action by cloning the base action's
    /// tree through the named grouplist (C++ `deriveAction`).
    fn derive_action(&mut self, baseaction: &str, grp: &str) -> KunaResult<()> {
        if self.actionmap.contains_key(grp) {
            return Ok(()); // Already derived this action
        }
        let curgrp = self.get_group(grp)?.clone();
        let newact = {
            let base = self.get_action(baseaction)?;
            base.clone_filtered(&curgrp)
        };
        // Register under the group name even if the clone produced nothing
        // (C++ registers the possibly-null result; here `None` means an empty
        // root, which we represent as an empty ActionGroup so the map is total).
        let boxed: Box<dyn Action> =
            newact.unwrap_or_else(|| Box::new(ActionGroup::new(0, grp.to_string())));
        self.register_action(grp, boxed);
        Ok(())
    }

    /// Derive a fresh root Action from the universal through the named group,
    /// returning it by value (does not touch the cache).  This is the testable
    /// face of the filtered-clone semantics.
    pub fn derive_fresh(&self, grp: &str) -> KunaResult<Option<Box<dyn Action>>> {
        let curgrp = self.get_group(grp)?;
        let base = self.get_action(UNIVERSAL_NAME)?;
        Ok(base.clone_filtered(curgrp))
    }
}

/// (Re)build the default *root* Actions: decompile, jumptable, normalize,
/// paramid, register, firstpass — transcribed verbatim from
/// `ActionDatabase::buildDefaultGroups` (`coreaction.cc:5675`).
///
/// The W5 `universal_action` builder registers concrete rules against these
/// group names; the member lists are the C++ kuna-divergent lists (note the
/// `"canonicalcompare"`/`"presentcompare"` substages split out of `"analysis"`).
/// Returns the populated [`ActionDatabase`] group map (leaving the universal
/// tree to be installed by W5).
pub fn build_default_groups(db: &mut ActionDatabase) {
    if db.is_default_groups {
        return;
    }
    db.groupmap.clear();
    // (kuna) "canonicalcompare" (comparison canonicalization, split out of
    // "analysis") accompanies "analysis" in every variant; "presentcompare"
    // (restore original comparison form for presentation) joins the full
    // decompile pipeline only.
    db.set_group(
        "decompile",
        [
            "base", "protorecovery", "protorecovery_a", "deindirect", "localrecovery",
            "deadcode", "typerecovery", "stackptrflow",
            "blockrecovery", "stackvars", "deadcontrolflow", "switchnorm",
            "cleanup", "splitcopy", "splitpointer", "merge", "dynamic", "casts", "analysis",
            "canonicalcompare", "presentcompare",
            "fixateglobals", "fixateproto", "constsequence", "bitfields",
            "segment", "returnsplit", "nodejoin", "doubleload", "doubleprecis",
            "unreachable", "subvar", "floatprecision",
            "conditionalexe",
        ],
    );

    db.set_group(
        "jumptable",
        [
            "base", "noproto", "localrecovery", "deadcode", "stackptrflow",
            "stackvars", "analysis", "canonicalcompare", "segment", "subvar",
            "normalizebranches", "conditionalexe",
        ],
    );

    db.set_group(
        "normalize",
        [
            "base", "protorecovery", "protorecovery_b", "deindirect", "localrecovery",
            "deadcode", "stackptrflow", "normalanalysis",
            "stackvars", "deadcontrolflow", "analysis", "canonicalcompare", "fixateproto",
            "nodejoin", "unreachable", "subvar", "floatprecision", "normalizebranches",
            "conditionalexe",
        ],
    );

    db.set_group(
        "paramid",
        [
            "base", "protorecovery", "protorecovery_b", "deindirect", "localrecovery",
            "deadcode", "typerecovery", "stackptrflow", "siganalysis",
            "stackvars", "deadcontrolflow", "analysis", "canonicalcompare", "fixateproto",
            "unreachable", "subvar", "floatprecision",
            "conditionalexe",
        ],
    );

    db.set_group("register", ["base", "analysis", "canonicalcompare", "subvar"]);

    db.set_group("firstpass", ["base"]);
    db.is_default_groups = true;
}

#[cfg(test)]
mod tests;
