//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/StrongConnectivityHelper.java`.

use crate::rt::{JHashMap, JHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};

pub struct StrongConnectivityHelper {
    components: Vec<Vec<Statement>>,
    processed: JHashSet<Statement>,
    stack: Vec<Statement>,
    visited_statements: JHashSet<Statement>,
    index_map: JHashMap<Statement, i32>,
    low_link_map: JHashMap<Statement, i32>,
    on_stack: JHashMap<Statement, bool>,
    index: i32,
}

impl StrongConnectivityHelper {
    pub fn new(stat: Statement) -> StrongConnectivityHelper {
        let mut helper = StrongConnectivityHelper {
            components: Vec::new(),
            processed: JHashSet::new(),
            stack: Vec::new(),
            visited_statements: JHashSet::new(),
            index_map: JHashMap::new(),
            low_link_map: JHashMap::new(),
            on_stack: JHashMap::new(),
            index: 0,
        };

        helper.visit_tree(stat.get_first().expect("first"));

        let stats = stat.get_stats().borrow().to_vec();
        for &st in &stats {
            if !helper.processed.contains(&st)
                && st
                    .get_predecessor_edges(Statement::STATEDGE_DIRECT_ALL)
                    .is_empty()
            {
                helper.visit_tree(st);
            }
        }

        for &st in &stats {
            if !helper.processed.contains(&st) {
                helper.visit_tree(st);
            }
        }

        helper
    }

    fn visit_tree(&mut self, stat: Statement) {
        self.stack.clear();
        self.index = 0;
        self.visited_statements.clear();
        self.index_map.clear();
        self.low_link_map.clear();
        self.on_stack.clear();

        self.visit(stat);

        let visited = self.visited_statements.to_vec();
        self.processed.add_all(visited);
        self.processed.add(stat);
    }

    fn visit(&mut self, stat: Statement) {
        self.stack.push(stat);
        self.index_map.put(stat, self.index);
        self.low_link_map.put(stat, self.index);
        self.index += 1;
        self.on_stack.put(stat, true);

        let mut successors = stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward);
        successors.retain(|s| !self.processed.contains(s));

        for succ in successors {
            let new_value;

            if self.visited_statements.contains(&succ) {
                if !*self.on_stack.get(&succ).expect("onStack entry") {
                    continue;
                }

                new_value = *self.index_map.get(&succ).expect("index entry");
            } else {
                self.visited_statements.add(succ);
                self.visit(succ);

                new_value = *self.low_link_map.get(&succ).expect("lowlink entry");
            }

            let low = *self.low_link_map.get(&stat).expect("lowlink entry");
            self.low_link_map.put(stat, low.min(new_value));
        }

        if self.low_link_map.get(&stat) == self.index_map.get(&stat) {
            let mut component = Vec::new();

            loop {
                let v = self.stack.pop().expect("tarjan stack");
                self.on_stack.put(v, false);

                component.push(v);
                if v == stat {
                    break;
                }
            }

            self.components.push(component);
        }
    }

    pub fn is_exit_component(lst: &[Statement]) -> bool {
        let mut set = JHashSet::new();

        for stat in lst {
            set.add_all(stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward));
        }

        for stat in lst {
            set.remove(stat);
        }

        set.is_empty()
    }

    pub fn get_exit_reps(lst: &[Vec<Statement>]) -> Vec<Statement> {
        let mut res = Vec::new();

        for comp in lst {
            if StrongConnectivityHelper::is_exit_component(comp) {
                res.push(comp[0]);
            }
        }

        res
    }

    pub fn get_components(&self) -> &[Vec<Statement>] {
        &self.components
    }
}
