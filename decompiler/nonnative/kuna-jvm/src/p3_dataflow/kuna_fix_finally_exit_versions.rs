//! kuna fix `finally_exit_versions` (spec 10.16): the variable versions on a
//! try-finally's normal exits.
//!
//! The flattened graph keeps the finally body off the normal path: it hangs
//! off the try's combined catch and ends in `FINALLY_END`, which has no
//! successors, while the edges that leave the protected code go straight to
//! the continuation. `SFormsConstructor.getFilteredOutMap` was meant to replay
//! the body on those edges, but it looks the body's state up under a key no
//! node has, so the code after the `try` reads the versions from before the
//! body: a value the body assigns to a local is invisible there, the variable
//! is split from the one the body writes, and the stack-variable passes inline
//! the old value.
//!
//! With the fix, a normal exit of a verified `finally` takes, for each local
//! the body assigns, the versions of it created inside the body that reach
//! `FINALLY_END`, and keeps its own versions as well when some path through
//! the body leaves the local alone. The finally processing removed the inlined
//! copies of the body, which may keep a body-local in another slot than the
//! handler does; a slot such a copy renamed (`kuna_remapped_slots`) is left
//! out, so the copy's slots never meet the handler's. A semaphore `finally`
//! keeps its copies and skips its body on normal exits, so it replays nothing.
//! Two passes that walk the flattened graph see those exits too: the
//! stack-variable inliner, whose walk carries a value only along the graph's
//! edges, does not move a value from a finally body past its end
//! ([`FinallyExitVersions::leaves_body`]), and the path search of the fix
//! `var_merge_liveness` goes from the body's end to the exits' targets
//! ([`FinallyExitVersions::exit_edges`]).

use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::do_statement::Type as DoType;
use crate::exprent::Exprent;
use crate::rt::{JHashMap, JHashSet, Shared};
use crate::s_forms_fast_map_direct::SFormsFastMapDirect;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

#[derive(Default)]
pub struct FinallyExitVersions {
    /// The try-finally statements whose finally bodies hold the node being processed.
    bodies: Vec<i32>,
    /// Per try-finally statement id: the versions created in its finally body.
    created: JHashMap<i32, JHashSet<VarVersionPair>>,
    /// Per try-finally statement id: the slots its finally body assigns that
    /// the copies did not rename; `None` for a semaphore `finally`.
    written: JHashMap<i32, Option<Vec<i32>>>,
}

impl FinallyExitVersions {
    /// Notes the finally bodies `node` lies in, for the versions created next.
    pub fn enter(&mut self, node: DirectNode) {
        self.bodies.clear();
        let mut try_finally = node.try_finally();
        while let Some(enclosing) = try_finally {
            if enclosing.type_() == DirectNodeType::FinallyEnd {
                self.bodies.push(enclosing.statement().id());
            }
            try_finally = enclosing.try_finally();
        }
    }

    pub fn leave(&mut self) {
        self.bodies.clear();
    }

    /// Records a new version made while processing the current node.
    pub fn created(&mut self, var: i32, version: i32) {
        for &id in &self.bodies {
            match self.created.get_mut(&id) {
                Some(set) => {
                    set.add(VarVersionPair::new(var, version));
                }
                None => {
                    let mut set = JHashSet::new();
                    set.add(VarVersionPair::new(var, version));
                    self.created.put(id, set);
                }
            }
        }
    }

    /// Applies the finally body of the try-finally `stat` to `target`, the
    /// state on an edge that leaves its protected code normally; `end` is the
    /// state at the body's end (absent while no path has reached it).
    pub fn replay(
        &mut self,
        stat: Statement,
        end: Option<&Shared<SFormsFastMapDirect>>,
        target: &Shared<SFormsFastMapDirect>,
    ) {
        let Some(end) = end else {
            return;
        };
        let id = stat.id();
        if !self.written.contains_key(&id) {
            let written = Self::replayed_slots(stat);
            self.written.put(id, written);
        }
        let Some(Some(written)) = self.written.get(&id) else {
            return;
        };
        let created = self.created.get(&id);
        for &slot in written {
            let Some(versions) = end.borrow().get(slot) else {
                continue;
            };
            let mut defined = versions.borrow().get_copy();
            let mut passed = false;
            for version in versions.borrow().iter() {
                if !created.is_some_and(|c| c.contains(&VarVersionPair::new(slot, version))) {
                    defined.remove(&version);
                    passed = true;
                }
            }
            if defined.is_empty() {
                continue;
            }
            let existing = if passed {
                target.borrow().get(slot)
            } else {
                None
            };
            match existing {
                Some(set) => set.borrow_mut().union(&defined),
                None => target.borrow_mut().put(slot, Shared::new(defined)),
            }
        }
    }

    /// Whether `use_` reads `def` outside a finally body `def` was made in:
    /// past the body's end, which no edge of the flattened graph crosses.
    pub fn leaves_body(&self, def: VarVersionPair, use_: VarVersionPair) -> bool {
        self.created
            .iter()
            .any(|(_, set)| set.contains(&def) && !set.contains(&use_))
    }

    /// Per `FINALLY_END` node id of a verified `finally`: the nodes its
    /// protected code leaves to normally, which the body runs before.
    pub fn exit_edges(nodes: &[DirectNode]) -> JHashMap<String, Vec<DirectNode>> {
        let mut edges: JHashMap<String, Vec<DirectNode>> = JHashMap::new();
        for &pred in nodes {
            for edge in pred.get_successors(DirectEdgeType::Regular).to_vec() {
                let node = edge.get_destination();
                for fnode in Self::finally_exits(pred, node) {
                    let stat = fnode.statement();
                    let verified = stat
                        .try_as_catch_all()
                        .is_some_and(|c| c.is_finally() && c.get_monitor().is_none());
                    if !verified {
                        continue;
                    }
                    let end = DirectNodeType::FinallyEnd.make_id(stat.id());
                    match edges.get_mut(&end) {
                        Some(list) => {
                            if !list.contains(&node) {
                                list.push(node);
                            }
                        }
                        None => {
                            edges.put(end, vec![node]);
                        }
                    }
                }
            }
        }
        edges
    }

    /// The `FINALLY` nodes whose bodies the regular edge `pred -> node` runs,
    /// innermost first, as `SFormsConstructor.getFilteredOutMap` walks them.
    pub fn finally_exits(pred: DirectNode, node: DirectNode) -> Vec<DirectNode> {
        let mut res = Vec::new();
        let (node_try_finally, pred_try_finally) = (node.try_finally(), pred.try_finally());
        if node_try_finally == pred_try_finally
            || node_try_finally.is_some_and(|tf| {
                tf.type_() == DirectNodeType::Finally && tf.try_finally() == pred_try_finally
            })
            || pred.type_() == DirectNodeType::Finally
        {
            return res;
        }
        let mut finally_node = pred_try_finally;
        while finally_node != node_try_finally {
            let Some(fnode) = finally_node else {
                break;
            };
            if fnode.type_() == DirectNodeType::Finally && fnode != node {
                res.push(fnode);
            }
            finally_node = fnode.try_finally();
        }
        res
    }

    fn replayed_slots(stat: Statement) -> Option<Vec<i32>> {
        let catch_all = stat.try_as_catch_all()?;
        if catch_all.get_monitor().is_some() {
            return None;
        }
        let remapped = catch_all.get_kuna_remapped_slots();
        let mut slots = Self::assigned_locals(catch_all.get_handler());
        slots.retain(|slot| !remapped.contains(slot));
        Some(slots)
    }

    /// The local variables `body` assigns, in first-seen order.
    pub fn assigned_locals(body: Statement) -> Vec<i32> {
        let mut res = Vec::new();
        Self::collect(body, &mut res);
        res
    }

    fn collect(stat: Statement, res: &mut Vec<i32>) {
        let mut add = |index: i32| {
            if (0..VarExprent::STACK_BASE).contains(&index) && !res.contains(&index) {
                res.push(index);
            }
        };

        let exprents = match stat.get_exprents() {
            Some(list) => list.to_vec(),
            None => stat.get_stat_exprents(),
        };
        for exprent in exprents {
            for expr in exprent.get_all_exprents_rec_self(true, true) {
                if let Some(index) = Self::assigned_var(expr) {
                    add(index);
                }
            }
        }

        if let Some(loop_) = stat.try_as_do() {
            if loop_.get_looptype() == DoType::ForEach {
                if let Some(var) = loop_.get_init_exprent().and_then(|e| e.try_as_var()) {
                    add(var.get_index());
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::collect(st, res);
        }
    }

    fn assigned_var(expr: Exprent) -> Option<i32> {
        if let Some(assignment) = expr.try_as_assignment() {
            return assignment.get_left().try_as_var().map(|v| v.get_index());
        }
        let function = expr.try_as_function()?;
        if function.get_func_type().is_ppmm() {
            return function
                .get_lst_operands()
                .get(0)
                .try_as_var()
                .map(|v| v.get_index());
        }
        None
    }
}
