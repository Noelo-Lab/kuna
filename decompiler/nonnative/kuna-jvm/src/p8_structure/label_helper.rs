//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LabelHelper.java`.
//!
//! kuna fix `loop_body_exit_explicit` (`kuna_fixes`): `setExplicitEdges` returns
//! no edges for a loop, whose body never falls through to the loop's end.

use crate::do_statement::Type as DoType;
use crate::function_exprent::FunctionType;
use crate::inline_single_block_helper::InlineSingleBlockHelper;
use crate::kuna_fixes::{self, KunaFix};
use crate::merge_helper::MergeHelper;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement, StatementType};

pub struct LabelHelper;

struct LabelSets {
    breaks: JHashSet<Statement>,
    continues: JHashSet<Statement>,
}

impl LabelSets {
    fn new() -> LabelSets {
        LabelSets {
            breaks: JHashSet::new(),
            continues: JHashSet::new(),
        }
    }
}

impl LabelHelper {
    pub fn clean_up_edges(root: RootStatement) {
        Self::reset_all_edges(root.0);

        Self::remove_non_immediate_edges(root.0);

        Self::lift_closures(root.0);

        Self::low_continue_labels(root.0, &mut JLinkedHashSet::new());

        Self::low_closures(root.0);
    }

    pub fn identify_labels(root: RootStatement) {
        Self::set_explicit_edges(root.0);

        Self::process_statement_label(root.0);

        Self::set_ret_edges_unlabeled(root);

        Self::lift_sequence_label(root.0);
    }

    fn lift_closures(stat: Statement) {
        for edge in stat.get_all_successor_edges() {
            match edge.get_type() {
                StatEdge::TYPE_CONTINUE => {
                    if Some(edge.get_destination()) != edge.closure() {
                        edge.get_destination().add_labeled_edge(edge);
                    }
                }
                StatEdge::TYPE_BREAK => {
                    let dest = edge.get_destination();

                    if !dest.is_dummy_exit() {
                        let parent = dest.get_parent();

                        let mut lst: Vec<Statement> = Vec::new();
                        if let Some(parent) = parent {
                            if parent.is_sequence() {
                                lst.extend(parent.get_stats().borrow().to_vec());
                            } else if let Some(swst) = parent.try_as_switch() {
                                lst.extend(swst.get_case_statements().to_vec());
                            }
                        }

                        for i in 1..lst.len() {
                            if lst[i] == dest {
                                let st = lst[i - 1];
                                if !st.is_basic_block() {
                                    st.add_labeled_edge(edge);
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::lift_closures(st);
        }
    }

    fn remove_non_immediate_edges(stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::remove_non_immediate_edges(st);
        }

        if !stat.has_basic_succ_edge() {
            for edge in stat.get_successor_edges(StatEdge::TYPE_CONTINUE | StatEdge::TYPE_BREAK) {
                stat.remove_successor(edge);
            }
        }
    }

    /// Java: lowContinueLabels(Statement, HashSet). Every caller passes a
    /// `LinkedHashSet`, whose insertion order the edge loop depends on.
    pub fn low_continue_labels(stat: Statement, edges: &mut JLinkedHashSet<StatEdge>) {
        let mut ok = !stat.is_do();
        if !ok {
            let dostat = stat.as_do();
            ok = dostat.get_looptype() == DoType::Infinite
                || dostat.get_looptype() == DoType::While
                    && dostat.get_condition_exprent().is_none()
                || (dostat.get_looptype() == DoType::For && dostat.get_inc_exprent().is_none());
        }

        if ok {
            edges.add_all(stat.get_predecessor_edges(StatEdge::TYPE_CONTINUE));
        }

        if ok && stat.is_do() {
            for edge in edges.to_vec() {
                if stat.contains_statement_strict(edge.get_source()) {
                    edge.get_destination().remove_predecessor(edge);
                    edge.get_source()
                        .change_edge_node(EdgeDirection::Forward, edge, stat);
                    stat.add_predecessor(edge);

                    stat.add_labeled_edge(edge);
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Some(st) == stat.get_first() {
                Self::low_continue_labels(st, edges);
            } else {
                Self::low_continue_labels(st, &mut JLinkedHashSet::new());
            }
        }
    }

    pub fn low_closures(stat: Statement) {
        let label_edges = stat.get_label_edges().borrow().to_vec();
        for edge in label_edges {
            if edge.get_type() == StatEdge::TYPE_BREAK {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    if st.contains_statement_strict(edge.get_source())
                        && MergeHelper::is_direct_path(st, edge.get_destination())
                    {
                        st.add_labeled_edge(edge);
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::low_closures(st);
        }
    }

    fn lift_sequence_label(stat: Statement) {
        if let Some(if_stat) = stat.get_parent().and_then(|p| p.try_as_if()) {
            if if_stat.get_ifstat() == Some(stat) && stat.is_sequence() {
                let mut edges: JHashSet<StatEdge> = JHashSet::new();

                let label_edges = stat.get_label_edges().borrow().to_vec();
                for edge in label_edges {
                    if MergeHelper::is_direct_path(if_stat.0, edge.get_destination()) {
                        edges.add(edge);
                    }
                }

                if edges.len() == stat.get_label_edges().borrow().len() {
                    let copy = stat.get_label_edges().borrow().clone();
                    for edge in copy.to_vec() {
                        if_stat.add_labeled_edge(edge);
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::lift_sequence_label(st);
        }
    }

    fn reset_all_edges(stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::reset_all_edges(st);
        }

        for edge in stat.get_all_successor_edges() {
            edge.with_mut(|d| {
                d.explicit = true;
                d.labeled = true;
            });
        }
    }

    fn set_ret_edges_unlabeled(root: RootStatement) {
        let exit = root.get_dummy_exit().0;
        for edge in exit.get_all_predecessor_edges() {
            let lst = edge.get_source().get_exprents();
            if edge.get_type() == StatEdge::TYPE_FINALLYEXIT
                || lst
                    .filter(|l| !l.is_empty())
                    .is_some_and(|l| l.get(l.len() - 1).is_exit())
            {
                edge.with_mut(|d| d.labeled = false);
            }
        }
    }

    fn set_explicit_edges(stat: Statement) -> JHashMap<Statement, Vec<StatEdge>> {
        let mut map_edges: JHashMap<Statement, Vec<StatEdge>> = JHashMap::new();

        if stat.get_exprents().is_some() {
            return map_edges;
        }

        match stat.type_() {
            StatementType::TryCatch | StatementType::CatchAll => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    let mut map_edges1 = Self::set_explicit_edges(st);
                    Self::process_edges_with_next(st, &mut map_edges1, None);

                    if stat.is_catch() || Some(st) == stat.get_first() {
                        merge_edge_maps(&mut map_edges, map_edges1);
                    }
                }
            }
            StatementType::Do => {
                let first = stat.get_first().unwrap();
                map_edges = Self::set_explicit_edges(first);
                Self::process_edges_with_next(first, &mut map_edges, Some(stat));
                if kuna_fixes::enabled(KunaFix::LoopBodyExitExplicit) {
                    map_edges.clear();
                }
            }
            StatementType::If => {
                let ifstat = stat.as_if();
                match ifstat.get_ifstat() {
                    None => {
                        Self::process_edges_with_next(
                            ifstat.get_first().unwrap(),
                            &mut map_edges,
                            None,
                        );
                    }
                    Some(if_body) => {
                        map_edges = Self::set_explicit_edges(if_body);
                        Self::process_edges_with_next(if_body, &mut map_edges, None);

                        let mut map_edges1 = None;
                        if let Some(else_body) = ifstat.get_elsestat() {
                            let mut else_edges = Self::set_explicit_edges(else_body);
                            Self::process_edges_with_next(else_body, &mut else_edges, None);
                            map_edges1 = Some(else_edges);
                        }

                        if let Some(map_edges1) = map_edges1 {
                            merge_edge_maps(&mut map_edges, map_edges1);
                        }
                    }
                }
            }
            StatementType::Root => {
                let first = stat.get_first().unwrap();
                map_edges = Self::set_explicit_edges(first);
                Self::process_edges_with_next(
                    first,
                    &mut map_edges,
                    Some(stat.as_root().get_dummy_exit().0),
                );
            }
            StatementType::Sequence => {
                let mut index: usize = 0;
                while (index as i32) < stat.get_stats().borrow().len() as i32 - 1 {
                    let st = stat.get_stats().borrow().get(index);
                    let mut st_edges = Self::set_explicit_edges(st);
                    let next = stat.get_stats().borrow().get(index + 1);
                    Self::process_edges_with_next(st, &mut st_edges, Some(next));
                    index += 1;
                }

                let st = stat.get_stats().borrow().get(index);
                map_edges = Self::set_explicit_edges(st);
                Self::process_edges_with_next(st, &mut map_edges, None);
            }
            StatementType::Switch => {
                let swst = stat.as_switch();
                let is_pattern_switch = swst.is_pattern();
                let last = swst.get_case_statements().len() as i32 - 1;

                let mut i = 0;
                while i < last {
                    let stt = swst.get_case_statements().get(i as usize);
                    let mut stnext = swst.get_case_statements().get(i as usize + 1);

                    if stnext.get_exprents().is_some_and(|e| e.is_empty())
                        && stnext.has_any_successor()
                        && !is_pattern_switch
                    {
                        stnext = stnext.get_all_successor_edges()[0].get_destination();
                    }
                    let mut stt_edges = Self::set_explicit_edges(stt);
                    Self::process_edges_with_next(stt, &mut stt_edges, Some(stnext));
                    i += 1;
                }

                if last >= 0 {
                    let stlast = swst.get_case_statements().get(last as usize);
                    if stlast.get_exprents().is_some_and(|e| e.is_empty())
                        && stlast.has_any_successor()
                    {
                        let edge = stlast.get_all_successor_edges()[0];
                        map_edges.put(edge.get_destination(), vec![edge]);
                    } else {
                        map_edges = Self::set_explicit_edges(stlast);
                        Self::process_edges_with_next(stlast, &mut map_edges, None);
                    }
                }
            }
            StatementType::Synchronized => {
                let synstat = stat.as_synchronized();

                let first = synstat.get_first().unwrap();
                let mut first_edges = Self::set_explicit_edges(stat.get_first().unwrap());
                Self::process_edges_with_next(first, &mut first_edges, Some(synstat.get_body()));
                map_edges = Self::set_explicit_edges(synstat.get_body());
                Self::process_edges_with_next(synstat.get_body(), &mut map_edges, None);
            }
            _ => {}
        }

        map_edges
    }

    fn process_edges_with_next(
        stat: Statement,
        map_edges: &mut JHashMap<Statement, Vec<StatEdge>>,
        next: Option<Statement>,
    ) {
        let mut next = next;
        let mut statedge: Option<StatEdge> = None;

        if stat.has_any_successor() {
            let first_successor = stat.get_first_successor();

            if Some(first_successor.get_destination()) == next {
                first_successor.with_mut(|d| d.explicit = false);
            } else {
                next = Some(first_successor.get_destination());
                statedge = Some(first_successor);
            }
        }

        if stat
            .try_as_do()
            .is_some_and(|dostat| dostat.get_looptype() == DoType::Infinite)
        {
            next = None;
        }

        if stat.try_as_switch().is_some_and(|swst| swst.is_phantom()) {
            let label_edges = stat.get_label_edges().borrow().to_vec();
            for edge in label_edges {
                edge.with_mut(|d| {
                    d.explicit = false;
                    d.labeled = false;
                });
            }
        }

        match next {
            None => {
                if map_edges.len() == 1 {
                    let lst_edges = map_edges.values().next().unwrap().clone();
                    let first_key = *map_edges.keys().next().unwrap();
                    if lst_edges.len() > 1 && !first_key.is_dummy_exit() {
                        let newedge = Self::add_implicit_successor(stat, &lst_edges);

                        map_edges.put(newedge.get_destination(), vec![newedge]);
                    }
                }
            }
            Some(next) => {
                let mut implfound = false;

                for (key, value) in map_edges.iter() {
                    if *key == next {
                        for &edge in value {
                            if stat.is_do() && edge.closure() == Some(stat) {
                                continue;
                            }

                            if edge.closure().is_some_and(|c| c.is_do())
                                && stat.is_if()
                                && edge.get_destination().is_dummy_exit()
                            {
                                continue;
                            }

                            edge.with_mut(|d| d.explicit = false);
                        }
                        implfound = true;
                        break;
                    }
                }

                if !stat.has_any_successor() && !implfound {
                    let mut lst_edges: Option<&Vec<StatEdge>> = None;
                    for (key, value) in map_edges.iter() {
                        if !key.is_dummy_exit()
                            && lst_edges.is_none_or(|lst| value.len() > lst.len())
                        {
                            lst_edges = Some(value);
                        }
                    }

                    if let Some(lst_edges) = lst_edges.filter(|lst| lst.len() > 1).cloned() {
                        Self::add_implicit_successor(stat, &lst_edges);
                    }
                }

                map_edges.clear();
            }
        }

        if let Some(statedge) = statedge {
            map_edges.put(statedge.get_destination(), vec![statedge]);
        }
    }

    /// The edge both arms of `processEdgesWithNext` add from `stat` in place of
    /// the several edges in `lst_edges`, which become implicit.
    fn add_implicit_successor(stat: Statement, lst_edges: &[StatEdge]) -> StatEdge {
        let edge_example = lst_edges[0];

        let example_closure = edge_example.closure();
        let parent = stat.get_parent().expect("parent");
        let mut closure = Some(parent);
        if !example_closure.is_some_and(|c| parent.contains_statement_strict(c)) {
            closure = example_closure;
        }

        let newedge = StatEdge::new_closure(
            edge_example.get_type(),
            stat,
            edge_example.get_destination(),
            closure,
        );
        stat.add_successor(newedge);

        for edge in lst_edges {
            edge.with_mut(|d| d.explicit = false);
        }

        newedge
    }

    pub fn hide_default_switch_edges(stat: Statement) -> bool {
        let mut res = false;
        if let Some(swst) = stat.try_as_switch() {
            let last = swst.get_case_statements().len() as i32 - 1;
            if last >= 0 {
                let stlast = swst.get_case_statements().get(last as usize);

                let needs_exhaustive = swst.get_case_values().iter().any(|values| {
                    values.iter().flatten().any(|exp| {
                        exp.try_as_function()
                            .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof)
                    })
                });
                if stlast.get_exprents().is_some_and(|e| e.is_empty()) && !needs_exhaustive {
                    let edges = stlast.get_all_successor_edges();
                    if edges.is_empty() || !edges[0].explicit() {
                        let lst_edges = swst.get_case_edges().get(last as usize);
                        if let Some(default_edge) = swst.get_default_edge() {
                            lst_edges.remove_item(&default_edge);
                        }

                        if lst_edges.is_empty() {
                            swst.get_case_statements().remove_at(last as usize);
                            swst.get_case_edges().remove_at(last as usize);
                        }

                        res = true;
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::hide_default_switch_edges(st);
        }

        res
    }

    fn process_statement_label(stat: Statement) -> LabelSets {
        let mut sets = LabelSets::new();

        if stat.get_exprents().is_none() {
            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                let nested = Self::process_statement_label(st);
                sets.breaks.add_all(nested.breaks.to_vec());
                sets.continues.add_all(nested.continues.to_vec());
            }

            let shield_type = stat.is_do() || stat.is_switch();
            if shield_type {
                let label_edges = stat.get_label_edges().borrow().to_vec();
                for edge in label_edges {
                    if edge.with(|d| d.explicit)
                        && ((edge.get_type() == StatEdge::TYPE_BREAK
                            && sets.breaks.contains(&edge.get_source()))
                            || (edge.get_type() == StatEdge::TYPE_CONTINUE
                                && sets.continues.contains(&edge.get_source())))
                    {
                        edge.with_mut(|d| d.labeled = false);
                    }
                }
            }

            match stat.with(|d| d.type_) {
                StatementType::Do => {
                    sets.continues.clear();
                    sets.breaks.clear();
                }
                StatementType::Switch => {
                    sets.breaks.clear();
                }
                _ => {}
            }
        }

        sets.breaks.add(stat);
        sets.continues.add(stat);

        sets
    }

    pub fn replace_continue_with_break(stat: Statement) -> bool {
        let mut res = false;

        if stat.is_do() {
            let mut seen_edges: JHashSet<StatEdge> = JHashSet::new();
            loop {
                let mut changed = false;

                let continues = stat.get_predecessor_edges(StatEdge::TYPE_CONTINUE);

                for edge in continues {
                    if seen_edges.contains(&edge) {
                        continue;
                    }

                    if edge.explicit() {
                        let minclosure = Self::get_min_continue_closure(edge);

                        if Some(minclosure) != edge.closure()
                            && !InlineSingleBlockHelper::is_break_edge_labeled(
                                edge.get_source(),
                                minclosure,
                            )
                        {
                            edge.get_source().change_edge_type(
                                EdgeDirection::Forward,
                                edge,
                                StatEdge::TYPE_BREAK,
                            );
                            edge.with_mut(|d| d.labeled = false);
                            minclosure.add_labeled_edge(edge);

                            seen_edges.add(edge);

                            res = true;
                            changed = true;
                        }

                        let enclosing =
                            Self::find_max_jump(Self::get_enclosing_shield_type(edge.get_source()));
                        if enclosing.is_do()
                            && !enclosing
                                .get_successor_edges(StatEdge::TYPE_BREAK)
                                .is_empty()
                            && Self::get_enclosing_shield_type(enclosing.get_parent().unwrap())
                                == minclosure
                        {
                            edge.get_source().change_edge_type(
                                EdgeDirection::Forward,
                                edge,
                                StatEdge::TYPE_BREAK,
                            );
                            edge.with_mut(|d| {
                                d.labeled = false;
                                d.phantom_continue = true;
                            });
                            enclosing.add_labeled_edge(edge);

                            seen_edges.add(edge);

                            res = true;
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::replace_continue_with_break(st);
        }

        res
    }

    fn find_max_jump(enclosing: Statement) -> Statement {
        let mut enclosing = enclosing;
        let mut last = enclosing;
        while enclosing.is_do()
            && !enclosing
                .get_successor_edges(StatEdge::TYPE_BREAK)
                .is_empty()
        {
            last = enclosing;
            enclosing = Self::get_enclosing_shield_type(enclosing.get_parent().unwrap());
        }

        last
    }

    fn get_enclosing_shield_type(stat: Statement) -> Statement {
        let mut res = stat;

        while !res.is_switch() && !res.is_do() {
            match res.get_parent() {
                None => break,
                Some(parent) => res = parent,
            }
        }

        res
    }

    fn get_min_continue_closure(edge: StatEdge) -> Statement {
        let mut closure = edge.closure().unwrap();
        loop {
            let mut found = false;

            let stats = closure.get_stats().borrow().to_vec();
            for st in stats {
                if st.contains_statement_strict(edge.get_source())
                    && MergeHelper::is_direct_path(st, edge.get_destination())
                {
                    closure = st;
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }
        }

        closure
    }
}

/// The "merge the maps" loop `setExplicitEdges` runs for try/catch and if/else.
fn merge_edge_maps(
    map_edges: &mut JHashMap<Statement, Vec<StatEdge>>,
    map_edges1: JHashMap<Statement, Vec<StatEdge>>,
) {
    for (key, value) in map_edges1.entry_vec() {
        if map_edges.contains_key(&key) {
            map_edges.get_mut(&key).unwrap().extend(value);
        } else {
            map_edges.put(key, value);
        }
    }
}
