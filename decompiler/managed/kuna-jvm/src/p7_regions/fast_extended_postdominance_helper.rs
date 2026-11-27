//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/FastExtendedPostdominanceHelper.java`.

use std::collections::VecDeque;

use crate::cancelation_manager::CancelationManager;
use crate::dominator_tree_exception_filter::DominatorTreeExceptionFilter;
use crate::fast_fixed_set::FastFixedSet;
use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::jthrow;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JavaEq};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::strong_connectivity_helper::StrongConnectivityHelper;
use crate::support_component::SupportComponent;

/// `mapSets`: a `None` value is a Java `null` entry (a fully processed node).
pub type ReachabilitySets = JHashMap<i32, Option<FastFixedSet<i32>>>;

/// `IReachabilityAction`; the helper is passed explicitly instead of captured.
pub type IReachabilityAction<'a> =
    dyn FnMut(&mut FastExtendedPostdominanceHelper, Statement, &ReachabilitySets) -> bool + 'a;

pub struct FastExtendedPostdominanceHelper {
    lst_reverse_post_order_list: Vec<Statement>,
    /// Declared `HashMap`; never iterated, so its runtime class is immaterial.
    map_support_points: JHashMap<i32, FastFixedSet<i32>>,
    map_ext_postdominators: JLinkedHashMap<i32, FastFixedSet<i32>>,
    statement: Option<Statement>,
    factory: Option<FastFixedSetFactory<i32>>,
}

impl Default for FastExtendedPostdominanceHelper {
    fn default() -> Self {
        FastExtendedPostdominanceHelper::new()
    }
}

fn reachability_set(map_sets: &ReachabilitySets, id: i32) -> Option<&FastFixedSet<i32>> {
    map_sets.get(&id).and_then(Option::as_ref)
}

impl FastExtendedPostdominanceHelper {
    pub fn new() -> FastExtendedPostdominanceHelper {
        FastExtendedPostdominanceHelper {
            lst_reverse_post_order_list: Vec::new(),
            map_support_points: JHashMap::new(),
            map_ext_postdominators: JLinkedHashMap::new(),
            statement: None,
            factory: None,
        }
    }

    fn statement(&self) -> Statement {
        self.statement.expect("statement")
    }

    fn factory(&self) -> &FastFixedSetFactory<i32> {
        self.factory.as_ref().expect("factory")
    }

    pub fn get_extended_postdominators(
        &mut self,
        statement: Statement,
    ) -> JHashMap<i32, JLinkedHashSet<i32>> {
        if !statement.is_general() {
            jthrow!(IllegalStateException; "Cannot find extended post dominators of non generalized statement");
        }

        self.statement = Some(statement);

        let mut set = JLinkedHashSet::new();
        let stats = statement.get_stats().borrow().to_vec();
        for st in &stats {
            set.add(st.id());
        }
        self.factory = Some(FastFixedSetFactory::create(set.to_vec()));

        self.lst_reverse_post_order_list = statement.get_reverse_post_order_list();

        self.calc_default_reachable_sets();

        self.remove_erroneous_nodes();

        let mut filter = DominatorTreeExceptionFilter::new(statement);
        filter.initialize();

        self.filter_on_exception_ranges(&filter);

        self.filter_on_dominance(&filter);

        self.add_supported_components(&filter);

        let mut res = JHashMap::with_capacity(self.map_ext_postdominators.len());
        for (&key, value) in self.map_ext_postdominators.iter() {
            let mut lst: Vec<i32> = value.iter().collect();
            lst.sort();
            let mut sorted = JLinkedHashSet::new();
            for id in lst {
                sorted.add(id);
            }
            res.put(key, sorted);
        }

        res
    }

    fn add_supported_components(&mut self, filter: &DominatorTreeExceptionFilter) {
        let schelp = StrongConnectivityHelper::new(self.statement());

        for comp in schelp.get_components() {
            CancelationManager::check_canceled();
            let supcomp =
                SupportComponent::identify(comp, &self.map_support_points, filter.get_dom_engine());

            if let Some(supcomp) = supcomp {
                let supported_id = supcomp.supported_point.id();
                for &st in &supcomp.stats {
                    if st != supcomp.supported_point {
                        let factory = self.factory.as_ref().expect("factory");
                        self.map_ext_postdominators
                            .compute_if_absent(st.id(), || factory.create_empty_set())
                            .add(supported_id);
                    }
                }
            }
        }
    }

    fn filter_on_dominance(&mut self, filter: &DominatorTreeExceptionFilter) {
        let engine = filter.get_dom_engine();
        let statement = self.statement();
        let mut stack: VecDeque<Statement> = VecDeque::new();
        let mut stack_path: VecDeque<FastFixedSet<i32>> = VecDeque::new();
        let mut set_visited: JHashSet<Statement> = JHashSet::new();

        let heads = JHashSet::from_collection(self.map_ext_postdominators.key_vec());
        for head in heads.to_vec() {
            stack.clear();
            stack_path.clear();

            let head_stat = statement
                .get_stats()
                .borrow()
                .get_with_key(&head)
                .expect("head statement");
            stack.push_back(head_stat);
            stack_path.push_back(self.factory().create_empty_set());

            set_visited.clear();

            set_visited.add(head_stat);

            while let Some(stat) = stack.pop_front() {
                CancelationManager::check_canceled();
                let mut path = stack_path.pop_front().expect("path");
                let set_postdoms = self
                    .map_ext_postdominators
                    .get_mut(&head)
                    .expect("postdominance set");

                if !set_postdoms.contains_key(&stat.id()) {
                    jthrow!(IllegalStateException; "Inconsistent statement structure!");
                }

                if set_postdoms.contains(&stat.id()) {
                    path.add(stat.id());
                }

                if path.contains_set(set_postdoms) {
                    continue;
                }

                if !engine.is_dominator(stat.id(), head) {
                    set_postdoms.complement(&path);
                    continue;
                }

                for edge in
                    stat.get_successor_edges(StatEdge::TYPE_REGULAR | StatEdge::TYPE_CONTINUE)
                {
                    let is_continue = edge.get_type() == StatEdge::TYPE_CONTINUE;
                    if is_continue && edge.get_destination() != statement {
                        continue;
                    }

                    let destination = if is_continue && edge.get_destination() == statement {
                        edge.get_destination().get_first().expect("first")
                    } else {
                        edge.get_destination()
                    };

                    if !set_visited.contains(&destination) {
                        stack.push_back(destination);
                        stack_path.push_back(path.get_copy());

                        set_visited.add(destination);
                    }
                }
            }

            if self
                .map_ext_postdominators
                .get(&head)
                .expect("postdominance set")
                .is_empty()
            {
                self.map_ext_postdominators.remove(&head);
            }
        }
    }

    fn filter_on_exception_ranges(&mut self, filter: &DominatorTreeExceptionFilter) {
        let heads = JHashSet::from_collection(self.map_ext_postdominators.key_vec());
        for head in heads.to_vec() {
            CancelationManager::check_canceled();
            let set = self
                .map_ext_postdominators
                .get_mut(&head)
                .expect("postdominance set");
            let rejected: Vec<i32> = set
                .iter()
                .filter(|&next| !filter.accept_statement_pair(head, next))
                .collect();
            for next in rejected {
                set.remove(&next);
            }

            if set.is_empty() {
                self.map_ext_postdominators.remove(&head);
            }
        }
    }

    fn remove_erroneous_nodes(&mut self) {
        self.map_support_points = JHashMap::new();

        self.calc_reachability_supp_points_ex();

        self.iterate_reachability(
            &mut |this, node, map_sets| {
                let nodeid = node.id();

                let set_reachability =
                    reachability_set(map_sets, nodeid).expect("reachability set");
                let mut lst_pred_sets: Vec<&FastFixedSet<i32>> = Vec::new();

                for prededge in node.get_predecessor_edges(StatEdge::TYPE_REGULAR) {
                    let source_id = prededge.get_source().id();
                    let set_pred = reachability_set(map_sets, source_id)
                        .or_else(|| this.map_support_points.get(&source_id));

                    lst_pred_sets.push(set_pred.expect("predecessor reachability set"));
                }

                for id in set_reachability.iter() {
                    let mut set_reachability_copy = set_reachability.get_copy();

                    let mut set_intersection =
                        this.factory.as_ref().expect("factory").create_empty_set();
                    let mut is_intersection_initialized = false;

                    for predset in &lst_pred_sets {
                        if predset.contains(&id) {
                            if !is_intersection_initialized {
                                set_intersection.union(predset);
                                is_intersection_initialized = true;
                            } else {
                                set_intersection.intersection(predset);
                            }
                        }
                    }

                    if nodeid != id {
                        set_intersection.add(nodeid);
                    } else {
                        set_intersection.remove(&nodeid);
                    }

                    set_reachability_copy.complement(&set_intersection);

                    this.map_ext_postdominators
                        .get_mut(&id)
                        .expect("postdominance set")
                        .complement(&set_reachability_copy);
                }

                false
            },
            StatEdge::TYPE_REGULAR,
        );

        let mut set_handlers = self.factory().create_empty_set();
        let mut handlerfound = false;

        let stats = self.statement().get_stats().borrow().to_vec();
        for stat in stats {
            if stat
                .get_predecessor_edges(Statement::STATEDGE_DIRECT_ALL)
                .is_empty()
                && !stat
                    .get_predecessor_edges(StatEdge::TYPE_EXCEPTION)
                    .is_empty()
            {
                set_handlers.add(stat.id());
                handlerfound = true;
            }
        }

        if handlerfound {
            self.map_ext_postdominators
                .for_each_value_mut(|_, set| set.complement(&set_handlers));
        }
    }

    fn calc_default_reachable_sets(&mut self) {
        let edgetype = StatEdge::TYPE_REGULAR | StatEdge::TYPE_EXCEPTION;

        self.calc_reachability_supp_points(edgetype);

        let stats = self.statement().get_stats().borrow().to_vec();
        for stat in stats {
            let empty = self.factory().create_empty_set();
            self.map_ext_postdominators.put(stat.id(), empty);
        }

        self.iterate_reachability(
            &mut |this, node, map_sets| {
                let nodeid = node.id();
                let set_reachability =
                    reachability_set(map_sets, nodeid).expect("reachability set");

                for id in set_reachability.iter() {
                    this.map_ext_postdominators
                        .get_mut(&id)
                        .expect("postdominance set")
                        .add(nodeid);
                }

                false
            },
            edgetype,
        );
    }

    fn calc_reachability_supp_points(&mut self, edgetype: i32) {
        self.iterate_reachability(
            &mut |this, node, map_sets| {
                for sucedge in node.get_all_successor_edges() {
                    if sucedge.get_type() & edgetype != 0
                        && map_sets.contains_key(&sucedge.get_destination().id())
                        && this.update_support_point(node, map_sets)
                    {
                        return true;
                    }
                }

                false
            },
            edgetype,
        );
    }

    fn calc_reachability_supp_points_ex(&mut self) {
        self.iterate_reachability(
            &mut |this, node, map_sets| {
                let statement = this.statement();
                for sucedge in node.get_all_successor_edges() {
                    let type_ = sucedge.get_type();
                    let to_statement = sucedge.get_destination() == statement;
                    if type_ & StatEdge::TYPE_REGULAR != 0
                        || (type_ & StatEdge::TYPE_CONTINUE != 0 && to_statement)
                    {
                        let destination = if type_ == StatEdge::TYPE_CONTINUE && to_statement {
                            sucedge.get_destination().get_first().expect("first")
                        } else {
                            sucedge.get_destination()
                        };

                        if map_sets.contains_key(&destination.id())
                            && this.update_support_point(node, map_sets)
                        {
                            return true;
                        }
                    }
                }

                false
            },
            StatEdge::TYPE_REGULAR,
        );
    }

    /// The shared tail of both support-point actions: true when `node`'s
    /// reachability set changed its support point.
    fn update_support_point(&mut self, node: Statement, map_sets: &ReachabilitySets) -> bool {
        let set_reachability = reachability_set(map_sets, node.id()).expect("reachability set");

        let unchanged = self
            .map_support_points
            .get(&node.id())
            .is_some_and(|old| set_reachability.java_equals(old));
        if !unchanged {
            self.map_support_points
                .put(node.id(), set_reachability.get_copy());
            return true;
        }
        false
    }

    fn iterate_reachability(&mut self, action: &mut IReachabilityAction<'_>, edgetype: i32) {
        let mut map_sets: ReachabilitySets = JHashMap::new();

        loop {
            let mut iterate = false;

            map_sets.clear();

            for stat in self.lst_reverse_post_order_list.clone() {
                CancelationManager::check_canceled();
                let mut set = self.factory().create_empty_set();
                set.add(stat.id());

                for prededge in stat.get_all_predecessor_edges() {
                    if prededge.get_type() & edgetype != 0 {
                        let pred_id = prededge.get_source().id();

                        let set_pred = reachability_set(&map_sets, pred_id)
                            .or_else(|| self.map_support_points.get(&pred_id));

                        if let Some(set_pred) = set_pred {
                            set.union(set_pred);
                        }
                    }
                }

                map_sets.put(stat.id(), Some(set));

                iterate |= action(self, stat, &map_sets);

                for prededge in stat.get_all_predecessor_edges() {
                    if prededge.get_type() & edgetype != 0 {
                        let pred = prededge.get_source();

                        if map_sets.contains_key(&pred.id()) {
                            let mut remstat = true;
                            for sucedge in pred.get_all_successor_edges() {
                                if sucedge.get_type() & edgetype != 0
                                    && !map_sets.contains_key(&sucedge.get_destination().id())
                                {
                                    remstat = false;
                                    break;
                                }
                            }

                            if remstat {
                                map_sets.put(pred.id(), None);
                            }
                        }
                    }
                }
            }

            if !iterate {
                break;
            }
        }
    }
}
