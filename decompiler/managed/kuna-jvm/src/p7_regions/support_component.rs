//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/SupportComponent.java`.

use crate::dominator_engine::DominatorEngine;
use crate::fast_fixed_set::FastFixedSet;
use crate::rt::{JHashMap, JHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;

pub struct SupportComponent {
    pub stats: Vec<Statement>,
    pub self_support_points: JHashMap<i32, FastFixedSet<i32>>,
    pub supported_point: Statement,
}

impl SupportComponent {
    pub fn new(
        stats: Vec<Statement>,
        self_support_points: JHashMap<i32, FastFixedSet<i32>>,
        supported_point: Statement,
    ) -> SupportComponent {
        SupportComponent {
            stats,
            self_support_points,
            supported_point,
        }
    }

    pub fn identify(
        component: &[Statement],
        map_support_points: &JHashMap<i32, FastFixedSet<i32>>,
        dom: &DominatorEngine,
    ) -> Option<SupportComponent> {
        let mut self_support_points = JHashMap::new();
        let mut supported_all = JHashSet::new();
        for &st in component {
            let Some(sup_reach) = map_support_points.get(&st.id()) else {
                continue;
            };

            for edge in st.get_successor_edge_view(StatEdge::TYPE_REGULAR).iter() {
                let dest = edge.get_destination();

                if !component.contains(&dest) {
                    return None;
                } else if dom.is_dominator(st.id(), dest.id()) {
                    supported_all.add(dest);
                }
            }

            self_support_points.put(st.id(), sup_reach.clone());
        }

        if supported_all.len() != 1 {
            return None;
        }

        if self_support_points.is_empty() {
            return None;
        }

        let mut outgoing = Vec::new();
        for &st in component {
            for edge in st.get_predecessor_edges(StatEdge::TYPE_REGULAR) {
                if !component.contains(&edge.get_source()) {
                    outgoing.push(st);
                }
            }
        }

        if outgoing.len() != 1 {
            return None;
        }

        let head = outgoing[0];
        for &st in component {
            if !dom.is_dominator(st.id(), head.id()) {
                return None;
            }
        }

        Some(SupportComponent::new(
            component.to_vec(),
            self_support_points,
            head,
        ))
    }
}

impl std::fmt::Display for SupportComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats: Vec<String> = self.stats.iter().map(|s| s.to_string()).collect();
        let points: Vec<String> = self
            .self_support_points
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        write!(
            f,
            "SupportComponent[[{}], selfSupportPoints={{{}}}, header={}]",
            stats.join(", "),
            points.join(", "),
            self.supported_point
        )
    }
}
