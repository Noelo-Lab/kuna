//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/DominatorEngine.java`.

use std::collections::VecDeque;

use crate::rt::JHashSet;
use crate::statement::Statement;
use crate::vb_style_collection::VBStyleCollection;

pub struct DominatorEngine {
    statement: Statement,
    col_ordered_idoms: VBStyleCollection<Option<i32>, i32>,
}

impl DominatorEngine {
    pub fn new(statement: Statement) -> DominatorEngine {
        DominatorEngine {
            statement,
            col_ordered_idoms: VBStyleCollection::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.calc_idoms();
    }

    fn order_statements(&mut self) {
        for stat in self.statement.get_reverse_post_order_list() {
            self.col_ordered_idoms.add_with_key(None, stat.id());
        }
    }

    fn get_common_idom(
        key1: Option<i32>,
        key2: Option<i32>,
        ordered_idoms: &VBStyleCollection<Option<i32>, i32>,
    ) -> Option<i32> {
        let (mut key1, mut key2) = match (key1, key2) {
            (None, key2) => return key2,
            (key1, None) => return key1,
            (Some(key1), Some(key2)) => (key1, key2),
        };

        let mut index1 = ordered_idoms.get_index_by_key(&key1);
        let mut index2 = ordered_idoms.get_index_by_key(&key2);

        while index1 != index2 {
            if index1 > index2 {
                key1 = ordered_idoms
                    .get_with_key(&key1)
                    .flatten()
                    .expect("idom of key1");
                index1 = ordered_idoms.get_index_by_key(&key1);
            } else {
                key2 = ordered_idoms
                    .get_with_key(&key2)
                    .flatten()
                    .expect("idom of key2");
                index2 = ordered_idoms.get_index_by_key(&key2);
            }
        }

        Some(key1)
    }

    fn calc_idoms(&mut self) {
        self.order_statements();

        let first_id = self.statement.get_first().expect("first").id();
        self.col_ordered_idoms
            .put_with_key(Some(first_id), first_id);

        let lst_ids: Vec<i32> = self.col_ordered_idoms.get_lst_keys()[1..].to_vec();

        loop {
            let mut changed = false;

            for &id in &lst_ids {
                let stat = self
                    .statement
                    .get_stats()
                    .borrow()
                    .get_with_key(&id)
                    .expect("statement of id");
                let mut idom: Option<i32> = None;

                for edge in stat.get_all_predecessor_edges() {
                    let source_id = edge.get_source().id();
                    if self
                        .col_ordered_idoms
                        .get_with_key(&source_id)
                        .flatten()
                        .is_some()
                    {
                        idom = DominatorEngine::get_common_idom(
                            idom,
                            Some(source_id),
                            &self.col_ordered_idoms,
                        );
                    }
                }

                let oldidom = self.col_ordered_idoms.put_with_key(idom, id).flatten();
                if Some(idom.expect("idom")) != oldidom {
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
    }

    pub fn get_ordered_idoms(&self) -> &VBStyleCollection<Option<i32>, i32> {
        &self.col_ordered_idoms
    }

    /// Whether `dom` dominates `node`.
    pub fn is_dominator(&self, node: i32, dom: i32) -> bool {
        let mut node = node;
        while node != dom {
            let idom = self
                .col_ordered_idoms
                .get_with_key(&node)
                .flatten()
                .expect("idom");

            if idom == node {
                return false;
            } else {
                node = idom;
            }
        }

        true
    }

    pub fn all_doms_for(&self, start: i32) -> JHashSet<i32> {
        let mut ret = JHashSet::new();

        let mut stack: VecDeque<i32> = VecDeque::new();
        stack.push_back(start);

        while let Some(id) = stack.pop_front() {
            if ret.contains(&id) {
                continue;
            }

            ret.add(id);

            for key in self.col_ordered_idoms.get_lst_keys().to_vec() {
                let ndid = self
                    .col_ordered_idoms
                    .get_with_key(&key)
                    .flatten()
                    .expect("idom");

                if ndid == id {
                    stack.push_back(key);
                }
            }
        }

        ret
    }
}
