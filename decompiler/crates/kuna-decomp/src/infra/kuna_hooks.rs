//! Embedder hooks into the default pipeline: extra rules for the rule pools
//! and callbacks at phase boundaries.
//!
//! [`HookRegistry`] lives on the [`Architecture`](crate::architecture::Architecture)
//! and is read by [`universal_sched_hooked`](crate::universalaction::universal_sched_hooked)
//! each time the universal tree is built.  An empty registry changes nothing:
//! no rule row and no leaf action is added.  There is no option and no CLI
//! surface; the registry is reachable only from code that links `kuna-decomp`.
//!
//! A phase boundary is one fixed schedule point per phase:
//!
//! | Phase | Point | Group of the leaf |
//! |---|---|---|
//! | P2 | after `start` (ops lifted, CFG built) | `base` |
//! | P3, P4, P5 | after `fullloop` reaches its fixed point | `base` |
//! | P6 | after `copymarker` (HighVariables merged) | `merge` |
//! | P7 | after the final `blockstructure` | `blockrecovery` |
//! | P8 | after the last structure pass, before `prototypewarnings` | `blockrecovery` |
//!
//! P1 ends before a function exists and P9 is the printer, so neither has a
//! point inside the action tree.  A leaf carries the group of its neighbours,
//! so a root that does not run a phase does not run its callbacks either, and
//! no callback runs on a jump-table recovery partial.  A restart of the
//! universal group runs the callbacks again.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult, Rule, RuleState};
use crate::funcdata::Funcdata;
use crate::kuna_phases::KunaPhase;
use crate::universalaction::{universal_sched, RuleRow, SchedNode};

/// A callback run at a phase boundary.
pub type PhaseCallback = Box<dyn FnMut(&mut Funcdata)>;

type PhaseCallbacks = Rc<RefCell<Vec<PhaseCallback>>>;

struct HookRule {
    after_group: String,
    state: RuleState,
    rule: Rc<dyn Rule>,
}

/// The rules and phase callbacks an embedder added to the default pipeline.
#[derive(Default)]
pub struct HookRegistry {
    rules: Vec<HookRule>,
    phases: BTreeMap<KunaPhase, PhaseCallbacks>,
}

impl HookRegistry {
    /// Whether nothing is registered.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.phases.is_empty()
    }

    /// Register `rule` behind the last rule of `after_group`, in the first
    /// pool of the schedule that holds a rule of that group.  `state` names
    /// the rule and gives the group that decides which roots keep it.
    ///
    /// Errors when no pool holds `after_group`, or when `rule` does not clone
    /// for its own group.
    pub fn add_rule(&mut self, after_group: &str, state: RuleState, rule: Box<dyn Rule>) -> KunaResult<()> {
        if !pool_owns_group(&universal_sched(None, None, Vec::new(), false), after_group) {
            return Err(KunaError::lowlevel(format!("No rule pool holds the group: {after_group}")));
        }
        let own = ActionGroupList::from_names([state.basegroup.as_str()]);
        if rule.clone_rule(&own).is_none() {
            return Err(KunaError::lowlevel(format!(
                "Rule {} does not clone for its group {}",
                state.name, state.basegroup
            )));
        }
        self.rules.push(HookRule { after_group: after_group.to_string(), state, rule: Rc::from(rule) });
        Ok(())
    }

    /// Register `callback` at the boundary of `phase`; callbacks of one phase
    /// run in registration order.  Errors for a phase with no point inside
    /// the action tree.
    pub fn on_phase(&mut self, phase: KunaPhase, callback: PhaseCallback) -> KunaResult<()> {
        if boundary_group(phase).is_none() {
            return Err(KunaError::lowlevel(format!(
                "Phase {} has no boundary inside the action tree",
                phase.code()
            )));
        }
        self.phases.entry(phase).or_default().borrow_mut().push(callback);
        Ok(())
    }

    /// Insert the registered rules into `pools`, given in schedule order.
    pub(crate) fn insert_rules(&self, pools: &mut [&mut Vec<RuleRow>]) {
        let mut anchors: Vec<&str> = Vec::new();
        for hook in &self.rules {
            if !anchors.contains(&hook.after_group.as_str()) {
                anchors.push(&hook.after_group);
            }
        }
        for anchor in anchors {
            let Some((pool, last)) = pools
                .iter_mut()
                .find_map(|pool| pool.iter().rposition(|row| row.group == anchor).map(|last| (pool, last)))
            else {
                continue;
            };
            let rows = self.rules.iter().filter(|hook| hook.after_group == anchor).map(HookRule::row);
            pool.splice(last + 1..last + 1, rows);
        }
    }

    /// The schedule node for the boundary of `phase`.
    pub(crate) fn boundary(&self, phase: KunaPhase) -> SchedNode {
        let (Some(group), Some(callbacks)) = (boundary_group(phase), self.phases.get(&phase)) else {
            return SchedNode::Empty;
        };
        let callbacks = Rc::clone(callbacks);
        SchedNode::Action(Box::new(move || {
            Box::new(ActionPhaseHook {
                base: ActionBase::new(0, format!("phasehook_{}", phase.code().to_ascii_lowercase()), group),
                callbacks: Rc::clone(&callbacks),
            })
        }))
    }
}

impl HookRule {
    fn row(&self) -> RuleRow {
        let rule = Rc::clone(&self.rule);
        let own = ActionGroupList::from_names([self.state.basegroup.as_str()]);
        RuleRow {
            name: Cow::Owned(self.state.name.clone()),
            group: Cow::Owned(self.state.basegroup.clone()),
            flags: self.state.flags,
            ctor: Rc::new(move || rule.clone_rule(&own).expect("checked by HookRegistry::add_rule")),
        }
    }
}

/// The group the boundary leaf of `phase` is scheduled under.
fn boundary_group(phase: KunaPhase) -> Option<&'static str> {
    match phase {
        KunaPhase::P2 | KunaPhase::P3 | KunaPhase::P4 | KunaPhase::P5 => Some("base"),
        KunaPhase::P6 => Some("merge"),
        KunaPhase::P7 | KunaPhase::P8 => Some("blockrecovery"),
        KunaPhase::Infra | KunaPhase::P0 | KunaPhase::P1 | KunaPhase::P9 => None,
    }
}

fn pool_owns_group(node: &SchedNode, group: &str) -> bool {
    match node {
        SchedNode::Pool { rules, .. } => rules.iter().any(|row| row.group == group),
        SchedNode::Group { children, .. } | SchedNode::RestartGroup { children, .. } => {
            children.iter().any(|child| pool_owns_group(child, group))
        }
        SchedNode::Action(_) | SchedNode::Empty => false,
    }
}

/// The leaf action that runs the callbacks of one phase boundary.
struct ActionPhaseHook {
    base: ActionBase,
    callbacks: PhaseCallbacks,
}

impl Action for ActionPhaseHook {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionPhaseHook { base: self.base.clone(), callbacks: Rc::clone(&self.callbacks) }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if data.is_jumptable_recovery_on() {
            return 0;
        }
        for callback in self.callbacks.borrow_mut().iter_mut() {
            callback(data);
        }
        0
    }
}
