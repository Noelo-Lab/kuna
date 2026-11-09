//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/IfStatement.java`.
//!
//! `headexprent` is the Java one-slot list shared with flow nodes; its `null`
//! slot (before `initExprents`) is the empty list.

use crate::dec_helper::DecHelper;
use crate::exprent::Exprent;
use crate::i_matchable::IMatchable;
use crate::if_exprent::IfExprent;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::JList;
use crate::start_end_pair::StartEndPair;
use crate::stat_edge::StatEdge;
use crate::statement::{
    add_instanceof_pattern_vars, LastBasicType, Statement, StatementKind, StatementType,
};
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::{java_subclass, unported};

pub struct IfStatementData {
    pub iftype: i32,
    pub ifstat: Option<Statement>,
    pub elsestat: Option<Statement>,
    pub ifedge: Option<StatEdge>,
    pub elseedge: Option<StatEdge>,
    pub negated: bool,
    pub pattern_matched: bool,
    pub has_ppmm: bool,
    pub headexprent: JList<Exprent>,
}

java_subclass!(pub IfStatement: Statement, StatementKind::If(IfStatementData), is_if, as_if, try_as_if);

pub(crate) fn set_head_slot(list: &JList<Exprent>, exprent: Exprent) {
    if list.is_empty() {
        list.add(exprent);
    } else {
        list.set(0, exprent);
    }
}

impl IfStatement {
    pub const IFTYPE_IF: i32 = 0;
    pub const IFTYPE_IFELSE: i32 = 1;

    /// Java: IfStatement()
    pub fn new_empty() -> IfStatement {
        IfStatement(Statement::new(
            StatementType::If,
            StatementKind::If(IfStatementData {
                iftype: 0,
                ifstat: None,
                elsestat: None,
                ifedge: None,
                elseedge: None,
                negated: false,
                pattern_matched: false,
                has_ppmm: false,
                headexprent: JList::new(),
            }),
        ))
    }

    /// Java: IfStatement(Statement, int, Statement)
    pub fn new(head: Statement, mut regedges: i32, postst: Option<Statement>) -> IfStatement {
        let this = IfStatement::new_empty();
        let stat = this.0;

        stat.set_first(head);
        stat.get_stats().borrow_mut().add_with_key(head, head.id());

        let lst_head_succs = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);

        let mut ifstat: Option<Statement> = None;
        let mut elsestat: Option<Statement> = None;
        let mut post: Option<Statement> = None;
        let mut negated = false;

        match regedges {
            0 => {}
            1 => {
                let edgeif = lst_head_succs[1];
                if edgeif.get_type() != StatEdge::TYPE_REGULAR {
                    post = Some(lst_head_succs[0].get_destination());
                } else {
                    post = Some(edgeif.get_destination());
                    negated = true;
                }
            }
            2 => {
                let else_dest = lst_head_succs[0].get_destination();
                let if_dest = lst_head_succs[1].get_destination();
                elsestat = Some(else_dest);
                ifstat = Some(if_dest);

                let lst_succ = if_dest.get_successor_edges(StatEdge::TYPE_REGULAR);
                let lst_succ1 = else_dest.get_successor_edges(StatEdge::TYPE_REGULAR);

                if if_dest.get_predecessor_edges(StatEdge::TYPE_REGULAR).len() > 1
                    || lst_succ.len() > 1
                {
                    post = ifstat;
                } else if else_dest
                    .get_predecessor_edges(StatEdge::TYPE_REGULAR)
                    .len()
                    > 1
                    || lst_succ1.len() > 1
                {
                    post = elsestat;
                } else if lst_succ.is_empty() {
                    post = elsestat;
                } else if lst_succ1.is_empty() {
                    post = ifstat;
                }

                if ifstat == post {
                    if elsestat != post {
                        ifstat = elsestat;
                        negated = true;
                    } else {
                        ifstat = None;
                    }
                    elsestat = None;
                } else if elsestat == post {
                    elsestat = None;
                } else {
                    post = postst;
                }

                if elsestat.is_none() {
                    regedges = 1;
                }
            }
            _ => {}
        }

        let ifedge = lst_head_succs[if negated { 0 } else { 1 }];
        let elseedge = (regedges == 2).then(|| lst_head_succs[if negated { 1 } else { 0 }]);
        let iftype = if regedges == 2 {
            IfStatement::IFTYPE_IFELSE
        } else {
            IfStatement::IFTYPE_IF
        };

        this.d_mut(|d| {
            d.ifstat = ifstat;
            d.elsestat = elsestat;
            d.negated = negated;
            d.ifedge = Some(ifedge);
            d.elseedge = elseedge;
            d.iftype = iftype;
        });
        stat.with_mut(|d| d.post = post);

        if iftype == IfStatement::IFTYPE_IF {
            if regedges == 0 {
                let edge = lst_head_succs[0];
                head.remove_successor(edge);
                edge.set_source(stat);
                stat.add_successor(edge);
            } else if regedges == 1 {
                let edge = lst_head_succs[if negated { 1 } else { 0 }];
                head.remove_successor(edge);
            }
        }

        if let Some(ifstat) = ifstat {
            stat.get_stats()
                .borrow_mut()
                .add_with_key(ifstat, ifstat.id());
        }

        if let Some(elsestat) = elsestat {
            stat.get_stats()
                .borrow_mut()
                .add_with_key(elsestat, elsestat.id());
        }

        if stat.get_post() == Some(head) {
            stat.with_mut(|d| d.post = Some(stat));
        }

        this
    }

    pub fn is_head(head: Statement) -> Option<Statement> {
        if head.is_basic_block() && head.get_last_basic_type() == LastBasicType::If {
            let regsize = head.get_successor_edges(StatEdge::TYPE_REGULAR).len() as i32;

            let mut p: Option<Statement> = None;

            let mut ok = regsize < 2;
            if !ok {
                let mut lst: Vec<Option<Statement>> = Vec::new();
                if DecHelper::is_choice_statement(head, &mut lst) {
                    p = lst.remove(0);

                    let rest: Vec<Statement> =
                        lst.into_iter().map(|s| s.expect("choice member")).collect();
                    for st in &rest {
                        if st.is_monitor_enter() {
                            return None;
                        }
                    }

                    ok = DecHelper::check_statement_exceptions(&rest);
                }
            }

            if ok {
                return Some(IfStatement::new(head, regsize, p).0);
            }
        }

        None
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("IfStatement.toJava")
    }

    pub fn init_exprents(self) {
        let exprents = self
            .get_first()
            .expect("first")
            .get_exprents()
            .expect("exprents");
        let mut ifexpr = exprents.remove_at(exprents.len() - 1).as_if();

        if self.is_negated() {
            ifexpr = ifexpr.copy().as_if();
            ifexpr.negate_if();
        }

        let head = self.get_headexprent_list();
        set_head_slot(&head, ifexpr.0);
    }

    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        self.get_headexprent_list().to_vec()
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        let head = self.get_headexprent_list();
        if head.first() == Some(oldexpr) {
            head.set(0, newexpr);
        }
    }

    pub fn replace_statement(self, oldstat: Statement, newstat: Statement) {
        self.0.replace_statement_base(oldstat, newstat);

        self.d_mut(|d| {
            if d.ifstat == Some(oldstat) {
                d.ifstat = Some(newstat);
            }
            if d.elsestat == Some(oldstat) {
                d.elsestat = Some(newstat);
            }
        });

        let lst_succs = self
            .get_first()
            .expect("first")
            .get_successor_edges(Statement::STATEDGE_DIRECT_ALL);

        if self.iftype() == IfStatement::IFTYPE_IF {
            let ifedge = lst_succs[0];
            self.d_mut(|d| {
                d.ifedge = Some(ifedge);
                d.elseedge = None;
            });
        } else {
            let edge0 = lst_succs[0];
            let edge1 = lst_succs[1];
            let (ifedge, elseedge) = if Some(edge0.get_destination()) == self.get_ifstat() {
                (edge0, edge1)
            } else {
                (edge1, edge0)
            };
            self.d_mut(|d| {
                d.ifedge = Some(ifedge);
                d.elseedge = Some(elseedge);
            });
        }
    }

    pub fn has_basic_succ_edge(self) -> bool {
        self.iftype() == IfStatement::IFTYPE_IF
    }

    pub fn get_simple_copy(self) -> Statement {
        let is = IfStatement::new_empty();
        let (iftype, negated) = self.d(|d| (d.iftype, d.negated));
        is.d_mut(|d| {
            d.iftype = iftype;
            d.negated = negated;
        });
        is.0
    }

    pub fn init_simple_copy(self) {
        let stats = self.get_stats();
        let first = stats.borrow().get(0);
        self.set_first(first);

        let (iftype, negated) = self.d(|d| (d.iftype, d.negated));
        let lst_succs = first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        let ifedge = lst_succs[if iftype == IfStatement::IFTYPE_IF || negated {
            0
        } else {
            1
        }];
        self.d_mut(|d| d.ifedge = Some(ifedge));
        if stats.borrow().len() > 1 {
            let ifstat = stats.borrow().get(1);
            self.d_mut(|d| d.ifstat = Some(ifstat));
        }

        if iftype == IfStatement::IFTYPE_IFELSE {
            let elseedge = lst_succs[if negated { 1 } else { 0 }];
            let elsestat = stats.borrow().get(2);
            self.d_mut(|d| {
                d.elseedge = Some(elseedge);
                d.elsestat = Some(elsestat);
            });
        }
    }

    pub fn get_elsestat(self) -> Option<Statement> {
        self.d(|d| d.elsestat)
    }

    pub fn set_elsestat(self, elsestat: Option<Statement>) {
        self.d_mut(|d| d.elsestat = elsestat)
    }

    pub fn get_ifstat(self) -> Option<Statement> {
        self.d(|d| d.ifstat)
    }

    pub fn set_ifstat(self, ifstat: Option<Statement>) {
        self.d_mut(|d| d.ifstat = ifstat)
    }

    pub fn is_negated(self) -> bool {
        self.d(|d| d.negated)
    }

    pub fn set_negated(self, negated: bool) {
        self.d_mut(|d| d.negated = negated)
    }

    /// The live one-slot list (empty while the Java slot is `null`).
    pub fn get_headexprent_list(self) -> JList<Exprent> {
        self.d(|d| d.headexprent.clone())
    }

    pub fn get_headexprent(self) -> IfExprent {
        self.get_headexprent_list()
            .first()
            .expect("if head exprent")
            .as_if()
    }

    pub fn set_else_edge(self, elseedge: Option<StatEdge>) {
        self.d_mut(|d| d.elseedge = elseedge)
    }

    pub fn set_if_edge(self, ifedge: Option<StatEdge>) {
        self.d_mut(|d| d.ifedge = ifedge)
    }

    pub fn get_if_edge(self) -> Option<StatEdge> {
        self.d(|d| d.ifedge)
    }

    pub fn get_else_edge(self) -> Option<StatEdge> {
        self.d(|d| d.elseedge)
    }

    pub fn is_pattern_matched(self) -> bool {
        self.d(|d| d.pattern_matched)
    }

    pub fn set_pattern_matched(self, pattern_matched: bool) {
        self.d_mut(|d| d.pattern_matched = pattern_matched)
    }

    pub fn has_ppmm(self) -> bool {
        self.d(|d| d.has_ppmm)
    }

    pub fn set_has_ppmm(self, has_ppmm: bool) {
        self.d_mut(|d| d.has_ppmm = has_ppmm)
    }

    /// Field read of the public Java field `iftype`.
    pub fn iftype(self) -> i32 {
        self.d(|d| d.iftype)
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        let mut vars = Vec::new();

        let condition = self.get_headexprent().get_condition();
        let mut condition_list = condition.get_all_exprents_rec(true);
        condition_list.push(self.get_headexprent().get_condition());

        add_instanceof_pattern_vars(&condition_list, &mut vars);

        Some(vars)
    }

    pub fn get_start_end_range(self) -> StartEndPair {
        let (ifstat, elsestat) = self.d(|d| (d.ifstat, d.elsestat));
        StartEndPair::join(&[
            Some(self.0.get_start_end_range_base()),
            ifstat.map(|s| s.get_start_end_range()),
            elsestat.map(|s| s.get_start_end_range()),
        ])
    }

    pub fn find_object(self, _match_node: &MatchNode, _index: i32) -> Option<Box<dyn IMatchable>> {
        unported!("IfStatement.findObject")
    }

    pub fn match_(self, _match_node: &MatchNode, _engine: &MatchEngine) -> bool {
        unported!("IfStatement.match")
    }

    pub fn fix_if_invariant_empty_else_branch(self) {
        let else_stat = self.get_elsestat();

        if self.iftype() != IfStatement::IFTYPE_IFELSE {
            return;
        }
        let else_stat = else_stat.expect("else statement of an if-else");
        if else_stat.type_() != StatementType::BasicBlock
            || else_stat.get_exprents().is_none_or(|e| !e.is_empty())
        {
            return;
        }

        self.get_stats()
            .borrow_mut()
            .remove_with_key(&else_stat.id());

        self.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
        self.set_elsestat(None);

        if let Some(else_edge) = self.get_else_edge() {
            self.get_first().expect("first").remove_successor(else_edge);
        }

        self.set_else_edge(None);

        if self.get_all_successor_edges().is_empty() {
            let next_edge = else_stat.get_first_successor();
            next_edge.change_source(self.0);
        } else {
            ValidationHelper::validate_true(
                self.get_first_successor().get_destination()
                    == else_stat.get_first_successor().get_destination(),
                "Expected the empty elseStat of the if statement to have the same destination as the if statement",
            );
            else_stat.get_first_successor().remove();
        }
    }

    pub fn fix_if_invariant_empty_if_branch(self) {
        let if_stat = self.get_ifstat();

        if self.iftype() != IfStatement::IFTYPE_IFELSE {
            return;
        }
        let Some(if_stat) = if_stat else {
            return;
        };
        if if_stat.type_() != StatementType::BasicBlock
            || if_stat.get_exprents().is_none_or(|e| !e.is_empty())
        {
            return;
        }

        self.get_stats().borrow_mut().remove_with_key(&if_stat.id());

        self.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
        self.set_ifstat(self.get_elsestat());
        self.set_elsestat(None);

        if let Some(if_edge) = self.get_if_edge() {
            self.get_first().expect("first").remove_successor(if_edge);
        }

        self.set_if_edge(self.get_else_edge());
        self.set_else_edge(None);

        if self.get_all_successor_edges().is_empty() {
            let next_edge = if_stat.get_first_successor();
            next_edge.change_source(self.0);
        } else {
            ValidationHelper::validate_true(
                self.get_first_successor().get_destination()
                    == if_stat.get_first_successor().get_destination(),
                "Expected the empty ifStat of the if statement to have the same destination as the if statement",
            );
            if_stat.get_first_successor().remove();
        }

        self.set_negated(!self.is_negated());
        let negated = self.get_headexprent().copy().as_if().negate_if();
        self.get_headexprent_list().set(0, negated.0);
    }
}
