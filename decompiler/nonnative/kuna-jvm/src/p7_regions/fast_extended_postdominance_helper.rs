//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/FastExtendedPostdominanceHelper.java`.

use crate::dominator_tree_exception_filter::DominatorTreeExceptionFilter;
use crate::fast_fixed_set::FastFixedSet;
use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::rt::{JHashMap, JLinkedHashMap, JLinkedHashSet};
use crate::statement::Statement;
use crate::unported;

/// `IReachabilityAction`: `(node, mapSets) -> iterate again`. A `None` value in
/// `mapSets` is a Java `null` entry (a fully processed node).
pub type IReachabilityAction<'a> =
    dyn FnMut(Statement, &JHashMap<i32, Option<FastFixedSet<i32>>>) -> bool + 'a;

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

    pub fn get_extended_postdominators(
        &mut self,
        _statement: Statement,
    ) -> JHashMap<i32, JLinkedHashSet<i32>> {
        unported!("FastExtendedPostdominanceHelper.getExtendedPostdominators")
    }

    fn add_supported_components(&mut self, _filter: &DominatorTreeExceptionFilter) {
        unported!("FastExtendedPostdominanceHelper.addSupportedComponents")
    }

    fn filter_on_dominance(&mut self, _filter: &DominatorTreeExceptionFilter) {
        unported!("FastExtendedPostdominanceHelper.filterOnDominance")
    }

    fn filter_on_exception_ranges(&mut self, _filter: &DominatorTreeExceptionFilter) {
        unported!("FastExtendedPostdominanceHelper.filterOnExceptionRanges")
    }

    fn remove_erroneous_nodes(&mut self) {
        unported!("FastExtendedPostdominanceHelper.removeErroneousNodes")
    }

    fn calc_default_reachable_sets(&mut self) {
        unported!("FastExtendedPostdominanceHelper.calcDefaultReachableSets")
    }

    fn calc_reachability_supp_points(&mut self, _edgetype: i32) {
        unported!("FastExtendedPostdominanceHelper.calcReachabilitySuppPoints")
    }

    fn calc_reachability_supp_points_ex(&mut self) {
        unported!("FastExtendedPostdominanceHelper.calcReachabilitySuppPointsEx")
    }

    fn iterate_reachability(
        &mut self,
        _action: Option<&mut IReachabilityAction<'_>>,
        _edgetype: i32,
    ) {
        unported!("FastExtendedPostdominanceHelper.iterateReachability")
    }
}
