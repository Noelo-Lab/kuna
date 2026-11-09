//! Port of `org/jetbrains/java/decompiler/modules/decompiler/TryHelper.java`.
use crate::catch_statement::CatchStatement;
use crate::root_statement::RootStatement;
use crate::rt::JHashSet;
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::try_with_resources_processor::TryWithResourcesProcessor;

pub struct TryHelper;

impl TryHelper {
    pub fn enhance_try_stats(root: RootStatement, cl: StructClass) -> bool {
        let mut ret = Self::make_try_with_resource_rec(cl, root.0);

        if ret {
            SequenceHelper::condense_sequences(root.0);
        }

        if Self::merge_trys(root.0) {
            SequenceHelper::condense_sequences(root.0);

            ret = true;
        }

        ret
    }

    fn make_try_with_resource_rec(cl: StructClass, stat: Statement) -> bool {
        let mut ret = false;

        if cl.get_version().has_new_try_with_resources() && stat.is_catch() {
            if TryWithResourcesProcessor::make_try_with_resource_j11(stat.as_catch()) {
                ret = true;
            }
        } else if let Some(try_st) = stat.try_as_catch_all().filter(|t| t.is_finally()) {
            if TryWithResourcesProcessor::make_try_with_resource(try_st) {
                ret = true;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Self::make_try_with_resource_rec(cl, st) {
                ret = true;
            }
        }

        ret
    }

    fn merge_trys(root: Statement) -> bool {
        let mut ret = false;

        if let Some(catch_stat) = root.try_as_catch() {
            if Self::merge_try(catch_stat) {
                ret = true;
            }
        }

        let stats = root.get_stats().borrow().to_vec();

        for stat in stats {
            ret |= Self::merge_trys(stat);
        }

        ret
    }

    fn merge_try(stat: CatchStatement) -> bool {
        if stat.get_stats().borrow().is_empty() {
            return false;
        }

        let mut inner = stat.get_stats().borrow().get(0);

        let mut edges: JHashSet<StatEdge> = JHashSet::new();
        TryWithResourcesProcessor::find_edges_leaving(stat.0, inner, &mut edges);
        let mut parent = stat.0;
        if inner.is_sequence() && edges.is_empty() {
            parent = inner;
            inner = inner.get_stats().borrow().get(0);
        }

        if let Some(inner_try) = inner.try_as_catch() {
            let resources = inner_try.get_resources();
            if !resources.is_empty() && inner.get_stats().borrow().len() == 1 {
                stat.get_resources().add_all(resources.to_vec());
                stat.get_var_definitions()
                    .add_all(inner.get_var_definitions().to_vec());

                let inner_block = inner.get_stats().borrow().get(0);

                let inner_edges = inner.get_all_successor_edges();
                for succ in inner_block.get_all_successor_edges() {
                    let mut found = false;
                    for inner_edge in &inner_edges {
                        if succ.get_destination() == inner_edge.get_destination() {
                            found = true;
                            break;
                        }
                    }

                    if found {
                        inner_block.remove_successor(succ);
                    }
                }

                parent.replace_statement(inner, inner_block);

                return true;
            }
        }

        false
    }
}
