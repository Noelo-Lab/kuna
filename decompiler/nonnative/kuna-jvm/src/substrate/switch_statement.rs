//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/SwitchStatement.java`.
//!
//! `caseValues` is shared with the `SwitchHeadExprent` (`setCaseValues`), and
//! its `null` entries mark the default case. `headexprent` is the one-slot list
//! shared with flow nodes (empty while the Java slot is `null`).

use crate::basic_block_statement::BasicBlockStatement;
use crate::const_exprent::ConstExprent;
use crate::dec_helper::DecHelper;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::if_statement::set_head_slot;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::start_end_pair::StartEndPair;
use crate::stat_edge::StatEdge;
use crate::statement::{
    add_instanceof_pattern_vars, EdgeDirection, LastBasicType, Statement, StatementKind,
    StatementType,
};
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::{java_subclass, jthrow, unported};

pub struct SwitchStatementData {
    pub case_statements: JList<Statement>,
    pub case_edges: JList<JList<StatEdge>>,
    pub case_values: JList<JList<Option<Exprent>>>,
    pub case_guards: JList<Option<Exprent>>,
    pub scoped_case_statements: JHashSet<Statement>,
    pub default_edge: Option<StatEdge>,
    pub headexprent: JList<Exprent>,
}

java_subclass!(pub SwitchStatement: Statement, StatementKind::Switch(SwitchStatementData), is_switch, as_switch, try_as_switch);

impl SwitchStatement {
    /// Java: SwitchStatement()
    pub fn new_empty() -> SwitchStatement {
        SwitchStatement(Statement::new(
            StatementType::Switch,
            StatementKind::Switch(SwitchStatementData {
                case_statements: JList::new(),
                case_edges: JList::new(),
                case_values: JList::new(),
                case_guards: JList::new(),
                scoped_case_statements: JHashSet::new(),
                default_edge: None,
                headexprent: JList::new(),
            }),
        ))
    }

    /// Java: SwitchStatement(Statement, Statement)
    pub fn new(head: Statement, poststat: Option<Statement>) -> SwitchStatement {
        let this = SwitchStatement::new_empty();
        let stat = this.0;

        stat.set_first(head);
        stat.get_stats().borrow_mut().add_with_key(head, head.id());

        let mut lst_nodes = JHashSet::from_collection(
            head.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward),
        );

        if let Some(post) = poststat {
            stat.with_mut(|d| d.post = Some(post));
            lst_nodes.remove(&post);
        }

        let default_edge = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)[0];
        this.d_mut(|d| d.default_edge = Some(default_edge));

        let mut sorted = lst_nodes.to_vec();
        sorted.sort_by_key(|o| o.id());
        for st in sorted {
            stat.get_stats().borrow_mut().add_with_key(st, st.id());
        }

        this
    }

    pub fn is_head(head: Statement) -> Option<Statement> {
        if head.is_basic_block() && head.get_last_basic_type() == LastBasicType::Switch {
            let mut lst: Vec<Option<Statement>> = Vec::new();
            if DecHelper::is_choice_statement(head, &mut lst) {
                let post = lst.remove(0);

                let rest: Vec<Statement> =
                    lst.into_iter().map(|s| s.expect("choice member")).collect();
                for st in &rest {
                    if st.is_monitor_enter() {
                        return None;
                    }
                }

                if DecHelper::check_statement_exceptions(&rest) {
                    return Some(SwitchStatement::new(head, post).0);
                }
            }
        }

        None
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("SwitchStatement.toJava")
    }

    pub fn init_exprents(self) {
        let exprents = self
            .get_first()
            .expect("first")
            .get_exprents()
            .expect("exprents");
        let swexpr = exprents.remove_at(exprents.len() - 1).as_switch_head();
        swexpr.set_case_values(self.get_case_values());

        set_head_slot(&self.get_headexprent_list(), swexpr.0);
    }

    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        let mut lst = self.get_headexprent_list().to_vec();
        lst.extend(self.get_case_guards().iter().flatten());

        for case_list in self.get_case_values().iter() {
            lst.extend(case_list.iter().flatten());
        }

        lst
    }

    pub fn is_pattern(self) -> bool {
        if !self.get_case_guards().is_empty() {
            return true;
        }

        for l in self.get_case_values().iter() {
            for e in l.iter().flatten() {
                if e.try_as_function()
                    .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof)
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        let mut vars = Vec::new();

        let mut case_list: Vec<Option<Exprent>> = Vec::new();
        for values in self.get_case_values().iter() {
            case_list.extend(values.iter());
        }
        case_list.extend(self.get_case_guards().iter());
        let case_list: Vec<Exprent> = case_list
            .into_iter()
            .flatten()
            .flat_map(|x| x.get_all_exprents_rec_self(true, true))
            .collect();

        add_instanceof_pattern_vars(&case_list, &mut vars);

        Some(vars)
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        let head = self.get_headexprent_list();
        if head.first() == Some(oldexpr) {
            head.set(0, newexpr);
        } else {
            let guards = self.get_case_guards();
            let idx = guards.index_of(&Some(oldexpr));
            if idx > -1 {
                guards.set(idx as usize, Some(newexpr));
            }
        }
    }

    pub fn replace_statement(self, oldstat: Statement, newstat: Statement) {
        let mut changed_any = false;
        let case_statements = self.get_case_statements();
        for i in 0..case_statements.len() {
            if case_statements.get(i) == oldstat {
                case_statements.set(i, newstat);
                changed_any = true;
            }
        }

        ValidationHelper::assert_true(
            changed_any,
            "Replaced statement in switch without changing any case statements!",
        );

        self.0.replace_statement_base(oldstat, newstat);
    }

    pub fn get_simple_copy(self) -> Statement {
        SwitchStatement::new_empty().0
    }

    pub fn init_simple_copy(self) {
        let first = self.get_stats().borrow().get(0);
        self.set_first(first);
        let default_edge = first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)[0];
        self.d_mut(|d| d.default_edge = Some(default_edge));

        self.sort_edges_and_nodes();
    }

    pub fn get_start_end_range(self) -> StartEndPair {
        let mut sepairs = vec![Some(self.0.get_start_end_range_base())];
        for st in self.get_case_statements().iter() {
            sepairs.push(Some(st.get_start_end_range()));
        }
        StartEndPair::join(&sepairs)
    }

    pub fn sort_edges_and_nodes(self) {
        let case_values = self.get_case_values();
        let skip = case_values.iter().any(|l| {
            l.iter().any(|u| match u.and_then(|u| u.try_as_const()) {
                Some(c) => c.is_null(),
                None => true,
            })
        }) || self.get_case_guards().iter().any(|g| g.is_some());
        if skip {
            return;
        }

        let first = self.get_first().expect("first");
        let stats = self.get_stats();

        let mut map_edge_index: JHashMap<StatEdge, i32> = JHashMap::new();

        let lst_first_succs = first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        for (i, &edge) in lst_first_succs.iter().enumerate() {
            let value = if i == 0 { lst_first_succs.len() } else { i };
            map_edge_index.put(edge, value as i32);
        }

        let bbstat = first.as_basic_block();
        let values: Vec<i32> = bbstat
            .get_block()
            .get_last_instruction()
            .expect("switch instruction")
            .as_switch()
            .get_values();

        let stats_size = stats.borrow().len();
        let mut nodes: Vec<Option<Statement>> = Vec::with_capacity(stats_size - 1);
        let mut edges: Vec<Vec<i32>> = Vec::with_capacity(stats_size - 1);

        for i in 1..stats_size {
            let stat = stats.borrow().get(i);

            let mut lst = Vec::new();
            for edge in stat.get_predecessor_edges(StatEdge::TYPE_REGULAR) {
                if edge.get_source() == first {
                    lst.push(*map_edge_index.get(&edge).expect("edge index"));
                }
            }
            lst.sort();

            nodes.push(Some(stat));
            edges.push(lst);
        }

        let mut lst_exit_edges =
            first.get_successor_edges(StatEdge::TYPE_BREAK | StatEdge::TYPE_CONTINUE);
        while !lst_exit_edges.is_empty() {
            let edge = lst_exit_edges[0];

            let mut lst = Vec::new();
            for i in (0..lst_exit_edges.len()).rev() {
                let edge_temp = lst_exit_edges[i];
                if edge_temp.get_destination() == edge.get_destination()
                    && edge_temp.get_type() == edge.get_type()
                {
                    lst.push(*map_edge_index.get(&edge_temp).expect("edge index"));
                    lst_exit_edges.remove(i);
                }
            }
            lst.sort();

            nodes.push(None);
            edges.push(lst);
        }

        let len = edges.len();
        for i in 0..len.saturating_sub(1) {
            for j in ((i + 1)..len).rev() {
                if edges[j - 1][0] > edges[j][0] {
                    edges.swap(j - 1, j);
                    nodes.swap(j - 1, j);
                }
            }
        }

        let mut index = 0usize;
        while index < nodes.len() {
            if let Some(stat) = nodes[index] {
                let mut set_preds = JHashSet::from_collection(
                    stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward),
                );
                set_preds.remove(&first);

                if !set_preds.is_empty() {
                    let pred = *set_preds.iter().next().expect("predecessor");
                    let mut j = 0usize;
                    while j < nodes.len() {
                        if j as i64 != index as i64 - 1 && nodes[j] == Some(pred) {
                            nodes.insert(j + 1, Some(stat));
                            let moved = edges[index].clone();
                            edges.insert(j + 1, moved);

                            if j > index {
                                nodes.remove(index);
                                edges.remove(index);
                                index = index.wrapping_sub(1);
                            } else {
                                nodes.remove(index + 1);
                                edges.remove(index + 1);
                            }
                            break;
                        }
                        j += 1;
                    }
                }
            }
            index = index.wrapping_add(1);
        }

        let mut lst_edges: Vec<JList<StatEdge>> = Vec::with_capacity(edges.len());
        let mut lst_values: Vec<JList<Option<Exprent>>> = Vec::with_capacity(edges.len());

        for lst in &edges {
            let lste: JList<StatEdge> = JList::new();
            let lstv: JList<Option<Exprent>> = JList::new();

            let lst_succs = first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            for &in_ in lst {
                let index = if in_ as usize == lst_succs.len() {
                    0
                } else {
                    in_ as usize
                };

                lste.add(lst_succs[index]);
                lstv.add(if index == 0 {
                    None
                } else {
                    Some(ConstExprent::new_int(values[index - 1], false, None).0)
                });
            }
            lst_edges.push(lste);
            lst_values.push(lstv);
        }

        for i in 0..nodes.len() {
            if nodes[i].is_none() {
                let bstat = BasicBlockStatement::create();

                let sample_edge = lst_edges[i].get(0);

                bstat.add_successor(StatEdge::new_closure(
                    sample_edge.get_type(),
                    bstat.0,
                    sample_edge.get_destination(),
                    sample_edge.closure(),
                ));

                for edge in lst_edges[i].iter() {
                    edge.get_source().change_edge_type(
                        EdgeDirection::Forward,
                        edge,
                        StatEdge::TYPE_REGULAR,
                    );
                    edge.closure()
                        .expect("closure")
                        .get_label_edges()
                        .borrow_mut()
                        .remove(&edge);

                    edge.get_destination().remove_predecessor(edge);
                    edge.get_source()
                        .change_edge_node(EdgeDirection::Forward, edge, bstat.0);
                    bstat.add_predecessor(edge);
                }

                nodes[i] = Some(bstat.0);
                stats.borrow_mut().add_with_key(bstat.0, bstat.id());
                bstat.set_parent(Some(self.0));
            }
        }

        let case_statements: Vec<Statement> =
            nodes.into_iter().map(|n| n.expect("case node")).collect();
        self.d_mut(|d| {
            d.case_statements = JList::from(case_statements);
            d.case_edges = JList::from(lst_edges);
            d.case_values = JList::from(lst_values);
        });
    }

    pub fn has_basic_succ_edge(self) -> bool {
        false
    }

    pub fn on_node_collapse(self) {
        self.sort_edges_and_nodes();
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_headexprent_list(self) -> JList<Exprent> {
        self.d(|d| d.headexprent.clone())
    }

    pub fn get_headexprent(self) -> Exprent {
        self.d(|d| d.headexprent.first())
            .expect("switch head exprent")
    }

    /// The live list of per-case edge lists.
    pub fn get_case_edges(self) -> JList<JList<StatEdge>> {
        self.d(|d| d.case_edges.clone())
    }

    /// The live list.
    pub fn get_case_statements(self) -> JList<Statement> {
        self.d(|d| d.case_statements.clone())
    }

    pub fn get_default_edge(self) -> Option<StatEdge> {
        self.d(|d| d.default_edge)
    }

    /// The live list (also held by the `SwitchHeadExprent`); `None` is the default case.
    pub fn get_case_values(self) -> JList<JList<Option<Exprent>>> {
        self.d(|d| d.case_values.clone())
    }

    /// The live list; entries may be `None`.
    pub fn get_case_guards(self) -> JList<Option<Exprent>> {
        self.d(|d| d.case_guards.clone())
    }

    pub fn scope_case_statement(self, stat: Statement) {
        if !self.get_case_statements().contains(&stat) {
            jthrow!(IllegalStateException; "Tried to scope a case statement that isn't in the switch!");
        }

        self.d_mut(|d| d.scoped_case_statements.add(stat));
    }

    pub fn set_default_edge(self, edge: Option<StatEdge>) {
        self.d_mut(|d| d.default_edge = edge)
    }

    pub fn get_default_case(self) -> Option<Statement> {
        self.get_default_edge().map(|e| e.get_destination())
    }

    pub fn set_default_case(self, stat: Statement) {
        if let Some(default_case) = self.get_default_case() {
            default_case.replace_with(stat);
            return;
        }

        self.add_case_internal(0, JList::from(vec![None]), stat);
    }

    /// Java: addCase(List<Exprent>, Statement)
    pub fn add_case(self, values: JList<Option<Exprent>>, stat: Statement) {
        let case_count = self.get_case_statements().len() as i32;
        let index = if self.get_default_edge().is_some() {
            case_count - 1
        } else {
            case_count
        };
        self.add_case_internal(index, values, stat);
    }

    /// Java: addCase(int, List<Exprent>, Statement)
    pub fn add_case_at(self, index: i32, values: JList<Option<Exprent>>, stat: Statement) {
        self.add_case_internal(index, values, stat);
    }

    fn add_case_internal(self, index: i32, values: JList<Option<Exprent>>, stat: Statement) {
        let index = index as usize;
        self.get_stats().borrow_mut().add_at(1 + index, stat);
        self.get_case_statements().add_at(index, stat);
        self.get_case_values().add_at(index, values.clone());

        if values.len() == 1 && values.get(0).is_none() {
            let edge = StatEdge::new(StatEdge::TYPE_REGULAR, self.get_basichead().0, stat);
            self.get_case_edges().add_at(index, JList::from(vec![edge]));
            self.set_default_edge(Some(edge));
        } else {
            let new_case_edge_lst: JList<StatEdge> = JList::new();
            for i in 0..values.len() {
                let edge = StatEdge::new(StatEdge::TYPE_REGULAR, self.get_basichead().0, stat);
                new_case_edge_lst.add_at(i, edge);

                self.get_basichead().add_successor(edge);
            }

            self.get_case_edges().add_at(index, new_case_edge_lst);
        }

        stat.set_parent(Some(self.0));
    }

    /// Java: removeCase(Statement)
    pub fn remove_case(self, stat: Statement) {
        let index = self.get_case_statements().index_of(&stat);
        if index == -1 {
            jthrow!(IllegalStateException; "Tried to remove a case statement that isn't in the switch!");
        }

        self.remove_case_at(index);
    }

    /// Java: removeCase(int)
    pub fn remove_case_at(self, index: i32) {
        let index = index as usize;
        let stat = self.get_case_statements().get(index);
        for edge in stat.get_all_predecessor_edges() {
            edge.remove();
        }
        for edge in stat.get_all_successor_edges() {
            edge.remove();
        }

        self.get_stats().borrow_mut().remove_with_key(&stat.id());
        self.get_case_statements().remove_at(index);
        self.get_case_values().remove_at(index);
        for edge in self.get_case_edges().get(index).iter() {
            edge.remove();
        }
        self.get_case_edges().remove_at(index);
    }
}
