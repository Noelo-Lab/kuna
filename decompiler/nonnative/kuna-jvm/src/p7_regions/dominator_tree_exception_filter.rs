//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/DominatorTreeExceptionFilter.java`.

use crate::dominator_engine::DominatorEngine;
use crate::rt::{JHashMap, JLinkedHashMap, JLinkedHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;

pub struct DominatorTreeExceptionFilter {
    statement: Statement,
    map_tree_branches: JLinkedHashMap<i32, JLinkedHashSet<i32>>,
    map_exception_ranges: JLinkedHashMap<i32, JLinkedHashSet<i32>>,
    map_exception_doms: JHashMap<i32, i32>,
    map_exception_range_unique_exit: JHashMap<i32, JHashMap<i32, i32>>,
    dom_engine: Option<DominatorEngine>,
}

impl DominatorTreeExceptionFilter {
    pub fn new(statement: Statement) -> DominatorTreeExceptionFilter {
        DominatorTreeExceptionFilter {
            statement,
            map_tree_branches: JLinkedHashMap::new(),
            map_exception_ranges: JLinkedHashMap::new(),
            map_exception_doms: JHashMap::new(),
            map_exception_range_unique_exit: JHashMap::new(),
            dom_engine: None,
        }
    }

    pub fn initialize(&mut self) {
        let mut dom_engine = DominatorEngine::new(self.statement);
        dom_engine.initialize();
        self.dom_engine = Some(dom_engine);

        self.build_dominator_tree();

        self.build_exception_ranges();

        self.build_filter(self.statement.get_first().expect("first").id());

        self.map_tree_branches.clear();
        self.map_exception_ranges.clear();
    }

    pub fn accept_statement_pair(&self, head: i32, exit: i32) -> bool {
        let filter = self
            .map_exception_range_unique_exit
            .get(&head)
            .expect("filter of head");
        for (key, &filter_exit) in filter.iter() {
            if Some(&head) != self.map_exception_doms.get(key)
                && (filter_exit == -1 || filter_exit != exit)
            {
                return false;
            }
        }

        true
    }

    fn build_dominator_tree(&mut self) {
        let ordered_idoms = self
            .dom_engine
            .as_ref()
            .expect("dominator engine")
            .get_ordered_idoms();

        let lst_keys = ordered_idoms.get_lst_keys().to_vec();
        for index in (0..lst_keys.len()).rev() {
            let key = lst_keys[index];
            let idom = ordered_idoms.get(index).expect("idom");
            self.map_tree_branches
                .compute_if_absent(idom, JLinkedHashSet::new)
                .add(key);
        }

        let firstid = self.statement.get_first().expect("first").id();
        self.map_tree_branches
            .get_mut(&firstid)
            .expect("branches of first")
            .remove(&firstid);
    }

    fn build_exception_ranges(&mut self) {
        let stats = self.statement.get_stats().borrow().to_vec();
        for stat in stats {
            let mut edges = stat.get_predecessor_edges(StatEdge::TYPE_EXCEPTION);
            edges.retain(|e| e.get_exceptions().is_some());

            if !edges.is_empty() {
                let mut set = JLinkedHashSet::new();

                for st in &edges {
                    set.add(st.get_source().id());
                }

                self.map_exception_ranges.put(stat.id(), set);
            }
        }

        self.map_exception_doms =
            self.build_exception_doms(self.statement.get_first().expect("first").id());
    }

    fn build_exception_doms(&self, id: i32) -> JHashMap<i32, i32> {
        let mut map = JHashMap::new();

        if let Some(children) = self.map_tree_branches.get(&id).map(|c| c.to_vec()) {
            for childid in children {
                let map_child = self.build_exception_doms(childid);
                for handler in map_child.key_vec() {
                    let value = if map.contains_key(&handler) {
                        id
                    } else {
                        *map_child.get(&handler).expect("child handler")
                    };
                    map.put(handler, value);
                }
            }
        }

        for (handler, range) in self.map_exception_ranges.iter() {
            if range.contains(&id) {
                map.put(*handler, id);
            }
        }

        map
    }

    fn build_filter(&mut self, id: i32) {
        let mut map: JHashMap<i32, i32> = JHashMap::new();

        if let Some(children) = self.map_tree_branches.get(&id).map(|c| c.to_vec()) {
            for childid in children {
                self.build_filter(childid);

                let map_child = self.map_exception_range_unique_exit.get(&childid);
                for (&handler, range) in self.map_exception_ranges.iter() {
                    if range.contains(&id) {
                        let exit = if !range.contains(&childid) {
                            Some(childid)
                        } else if map.contains_key(&handler) {
                            Some(-1)
                        } else {
                            map_child.expect("child filter").get(&handler).copied()
                        };

                        if let Some(exit) = exit {
                            map.put(handler, exit);
                        }
                    }
                }
            }
        }

        self.map_exception_range_unique_exit.put(id, map);
    }

    pub fn get_dom_engine(&self) -> &DominatorEngine {
        self.dom_engine.as_ref().expect("dominator engine")
    }
}
