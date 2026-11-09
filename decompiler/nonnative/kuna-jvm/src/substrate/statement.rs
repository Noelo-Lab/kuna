//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/Statement.java`.
//!
//! `Statement` is the arena handle; each subclass is a `java_subclass!` view
//! over one `StatementKind` variant, declared in its own module. Virtual
//! methods dispatch on `type_` (one variant per subclass) to the overriding
//! view in path form and fall back to the base body (`*_base` when an override
//! calls `super`).

use crate::basic_block_statement::{BasicBlockStatement, BasicBlockStatementData};
use crate::catch_all_statement::{CatchAllStatement, CatchAllStatementData};
use crate::catch_statement::{CatchStatement, CatchStatementData};
use crate::code_constants::{OPC_ATHROW, OPC_MONITORENTER, OPC_MONITOREXIT};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::{DoStatement, DoStatementData};
use crate::dummy_exit_statement::{DummyExitStatement, DummyExitStatementData};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::general_statement::{GeneralStatement, GeneralStatementData};
use crate::i_matchable::IMatchable;
use crate::if_statement::{IfStatement, IfStatementData};
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::pattern::Pattern;
use crate::root_statement::{RootStatement, RootStatementData};
use crate::rt::{BitSet, JHashMap, JHashSet, JList, JavaEq, JavaHash, Shared};
use crate::sequence_statement::{SequenceStatement, SequenceStatementData};
use crate::start_end_pair::StartEndPair;
use crate::stat_edge::StatEdge;
use crate::strong_connectivity_helper::StrongConnectivityHelper;
use crate::switch_statement::{SwitchStatement, SwitchStatementData};
use crate::synchronized_statement::{SynchronizedStatement, SynchronizedStatementData};
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::vb_style_collection::VBStyleCollection;
use crate::{arena_identity, arena_object, jthrow, unported};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum StatementType {
    Root,
    BasicBlock,
    Sequence,
    DummyExit,
    General,
    If,
    Do,
    Switch,
    Synchronized,
    TryCatch,
    CatchAll,
    Other,
}

impl StatementType {
    pub fn pretty_id(self) -> &'static str {
        match self {
            StatementType::Root => "Root",
            StatementType::BasicBlock => "Block",
            StatementType::Sequence => "Seq",
            StatementType::DummyExit => "Exit",
            StatementType::General => "General",
            StatementType::If => "If",
            StatementType::Do => "Do",
            StatementType::Switch => "Switch",
            StatementType::Synchronized => "Monitor",
            StatementType::TryCatch => "Catch",
            StatementType::CatchAll => "CatchAll",
            StatementType::Other => "YouHaveMetATerribleFate",
        }
    }
}

impl JavaHash for StatementType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for StatementType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum EdgeDirection {
    Backward,
    Forward,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum LastBasicType {
    If,
    Switch,
    General,
}

pub enum StatementKind {
    Root(RootStatementData),
    BasicBlock(BasicBlockStatementData),
    Sequence(SequenceStatementData),
    DummyExit(DummyExitStatementData),
    General(GeneralStatementData),
    If(IfStatementData),
    Do(DoStatementData),
    Switch(SwitchStatementData),
    Synchronized(SynchronizedStatementData),
    Catch(CatchStatementData),
    CatchAll(CatchAllStatementData),
}

pub struct StatementData {
    pub type_: StatementType,
    pub id: i32,
    pub map_succ_edges: JHashMap<i32, JList<StatEdge>>,
    pub map_pred_edges: JHashMap<i32, JList<StatEdge>>,
    pub stats: Shared<VBStyleCollection<Statement, i32>>,
    pub parent: Option<Statement>,
    pub first: Option<Statement>,
    pub exprents: Option<JList<Exprent>>,
    pub label_edges: Shared<JHashSet<StatEdge>>,
    pub var_definitions: JList<Exprent>,
    pub copied: bool,
    pub post: Option<Statement>,
    pub last_basic_type: LastBasicType,
    pub is_monitor_enter: bool,
    pub is_last_athrow: bool,
    pub contains_monitor_exit: bool,
    pub continue_set: Option<Shared<JHashSet<Statement>>>,
    pub phantom: bool,
    pub endpoints: Option<StartEndPair>,
    pub kind: StatementKind,
}

arena_object!(pub Statement => StatementData, Class);
arena_identity!(Statement);

const EXCEPTION_EDGE_TYPES: [i32; 2] = [Statement::STATEDGE_ALL, StatEdge::TYPE_EXCEPTION];
const REGULAR_EDGE_TYPES: [i32; 2] = [Statement::STATEDGE_ALL, Statement::STATEDGE_DIRECT_ALL];

fn edge_map(d: &StatementData, direction: EdgeDirection) -> &JHashMap<i32, JList<StatEdge>> {
    match direction {
        EdgeDirection::Backward => &d.map_pred_edges,
        EdgeDirection::Forward => &d.map_succ_edges,
    }
}

fn edge_map_mut(
    d: &mut StatementData,
    direction: EdgeDirection,
) -> &mut JHashMap<i32, JList<StatEdge>> {
    match direction {
        EdgeDirection::Backward => &mut d.map_pred_edges,
        EdgeDirection::Forward => &mut d.map_succ_edges,
    }
}

fn is_single_type(type_: i32) -> bool {
    type_ & type_.wrapping_sub(1) == 0
}

/// `((Pattern) exprent).getPatternVars()` for the two `Pattern` implementors.
pub(crate) fn pattern_vars_of(pattern: Exprent) -> Vec<VarExprent> {
    if let Some(var) = pattern.try_as_var() {
        return Pattern::get_pattern_vars(var);
    }
    Pattern::get_pattern_vars(pattern.as_pattern())
}

/// The instanceof-pattern variables declared by the exprents in `list`.
pub(crate) fn add_instanceof_pattern_vars(list: &[Exprent], vars: &mut Vec<VarExprent>) {
    for &exprent in list {
        if let Some(func) = exprent.try_as_function() {
            if func.get_func_type() == FunctionType::Instanceof && func.get_lst_operands().len() > 2
            {
                vars.extend(pattern_vars_of(func.get_lst_operands().get(2)));
            }
        }
    }
}

impl Statement {
    pub const STATEDGE_ALL: i32 = 0x8000_0000_u32 as i32;
    pub const STATEDGE_DIRECT_ALL: i32 = 0x4000_0000;

    /// Java: Statement(StatementType)
    pub fn new(type_: StatementType, kind: StatementKind) -> Statement {
        let id = DecompilerContext::get_counter_container()
            .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER);
        Statement::new_with_id(type_, id, kind)
    }

    /// Java: Statement(StatementType, int)
    pub fn new_with_id(type_: StatementType, id: i32, kind: StatementKind) -> Statement {
        Statement::alloc(StatementData {
            type_,
            id,
            map_succ_edges: JHashMap::new(),
            map_pred_edges: JHashMap::new(),
            stats: Shared::new(VBStyleCollection::new()),
            parent: None,
            first: None,
            exprents: None,
            label_edges: Shared::new(JHashSet::new()),
            var_definitions: JList::new(),
            copied: false,
            post: None,
            last_basic_type: LastBasicType::General,
            is_monitor_enter: false,
            is_last_athrow: false,
            contains_monitor_exit: false,
            continue_set: Some(Shared::new(JHashSet::new())),
            phantom: false,
            endpoints: None,
            kind,
        })
    }

    /// Field read of the public Java field `id`.
    pub fn id(self) -> i32 {
        self.with(|d| d.id)
    }

    /// Field read of the public Java field `type`.
    pub fn type_(self) -> StatementType {
        self.with(|d| d.type_)
    }

    pub fn clear_temp_information(self) {
        self.with_mut(|d| {
            d.post = None;
            d.continue_set = None;
            d.copied = false;
            d.is_monitor_enter = false;
            d.contains_monitor_exit = false;
            Statement::process_map(&mut d.map_succ_edges);
            Statement::process_map(&mut d.map_pred_edges);
        });
    }

    fn process_map(map: &mut JHashMap<i32, JList<StatEdge>>) {
        map.remove(&StatEdge::TYPE_EXCEPTION);
        match map.get(&Statement::STATEDGE_DIRECT_ALL).cloned() {
            Some(lst) => {
                map.put(Statement::STATEDGE_ALL, lst.copy());
            }
            None => {
                map.remove(&Statement::STATEDGE_ALL);
            }
        }
    }

    pub fn collapse_nodes_to_statement(self, stat: Statement) {
        let head = stat.get_first().expect("collapsed statement has a head");
        let post = stat.get_post();
        let set_nodes = stat.get_stats();

        if let Some(post) = post {
            for edge in post.get_edges(Statement::STATEDGE_DIRECT_ALL, EdgeDirection::Backward) {
                if stat.contains_statement_strict(edge.get_source()) {
                    edge.get_source().change_edge_type(
                        EdgeDirection::Forward,
                        edge,
                        StatEdge::TYPE_BREAK,
                    );
                    stat.add_labeled_edge(edge);
                }
            }
        }

        for prededge in head.get_all_predecessor_edges() {
            if prededge.get_type() != StatEdge::TYPE_EXCEPTION
                && stat.contains_statement_strict(prededge.get_source())
            {
                prededge.get_source().change_edge_type(
                    EdgeDirection::Forward,
                    prededge,
                    StatEdge::TYPE_CONTINUE,
                );
                stat.add_labeled_edge(prededge);
            }

            head.remove_predecessor(prededge);
            prededge
                .get_source()
                .change_edge_node(EdgeDirection::Forward, prededge, stat);
            stat.add_predecessor(prededge);
        }

        let first_id = self.get_first().expect("first").id();
        if set_nodes.borrow().contains_key(&first_id) {
            self.set_first(stat);
        }

        let nodes = set_nodes.borrow().to_vec();
        let mut set_handlers = JHashSet::from_collection(
            head.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward),
        );
        for node in &nodes {
            let neighbours = node.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward);
            set_handlers.retain(|h| neighbours.contains(h));
        }

        if !set_handlers.is_empty() {
            for edge in head.get_edges(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward) {
                let handler = edge.get_destination();

                if set_handlers.contains(&handler)
                    && !set_nodes.borrow().contains_key(&handler.id())
                {
                    stat.add_successor(StatEdge::new_exceptions(
                        stat,
                        handler,
                        edge.get_exceptions(),
                    ));
                }
            }

            for node in &nodes {
                for edge in node.get_edges(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward) {
                    if set_handlers.contains(&edge.get_destination()) {
                        node.remove_successor(edge);
                    }
                }
            }
        }

        if let Some(post) = post {
            if !stat
                .get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward)
                .contains(&post)
            {
                stat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, stat, post));
            }
        }

        let stats = self.get_stats();
        for st in &nodes {
            stats.borrow_mut().remove_with_key(&st.id());
        }
        stats.borrow_mut().add_with_key(stat, stat.id());

        stat.set_all_parent();
        stat.set_parent(Some(self));

        stat.build_continue_set();
        stat.build_monitor_flags();

        stat.on_node_collapse();
    }

    pub fn on_node_collapse(self) {
        if self.type_() == StatementType::Switch {
            SwitchStatement::on_node_collapse(SwitchStatement(self));
        }
    }

    pub fn set_all_parent(self) {
        let stats = self.get_stats().borrow().to_vec();
        for st in stats {
            st.set_parent(Some(self));
        }
    }

    pub fn add_labeled_edge(self, edge: StatEdge) {
        if let Some(closure) = edge.closure() {
            closure.get_label_edges().borrow_mut().remove(&edge);
        }
        edge.with_mut(|d| d.closure = Some(self));
        self.get_label_edges().borrow_mut().add(edge);
    }

    fn add_edge_direct_internal(self, direction: EdgeDirection, edge: StatEdge, edgetype: i32) {
        let lst = self.with_mut(|d| {
            edge_map_mut(d, direction)
                .compute_if_absent(edgetype, JList::new)
                .clone()
        });
        lst.add(edge);
    }

    /// Only public so `StatEdge` can call it.
    pub fn add_edge_internal(self, direction: EdgeDirection, edge: StatEdge) {
        let type_ = edge.get_type();
        let arrtypes = if type_ == StatEdge::TYPE_EXCEPTION {
            EXCEPTION_EDGE_TYPES
        } else {
            self.add_edge_direct_internal(direction, edge, type_);
            REGULAR_EDGE_TYPES
        };
        for edgetype in arrtypes {
            self.add_edge_direct_internal(direction, edge, edgetype);
        }
    }

    fn remove_edge_direct_internal(self, direction: EdgeDirection, edge: StatEdge, edgetype: i32) {
        let lst = self.with(|d| edge_map(d, direction).get(&edgetype).cloned());
        if let Some(lst) = lst {
            let index = lst.index_of(&edge);
            if index >= 0 {
                lst.remove_at(index as usize);
            }
        }
    }

    /// Only public so `StatEdge` can call it.
    pub fn remove_edge_internal(self, direction: EdgeDirection, edge: StatEdge) {
        let type_ = edge.get_type();
        let arrtypes = if type_ == StatEdge::TYPE_EXCEPTION {
            EXCEPTION_EDGE_TYPES
        } else {
            self.remove_edge_direct_internal(direction, edge, type_);
            REGULAR_EDGE_TYPES
        };
        for edgetype in arrtypes {
            self.remove_edge_direct_internal(direction, edge, edgetype);
        }
    }

    pub fn add_predecessor(self, edge: StatEdge) {
        self.add_edge_internal(EdgeDirection::Backward, edge);
    }

    /// Java tolerates `null`; a caller holding an `Option` skips `None`.
    pub fn remove_predecessor(self, edge: StatEdge) {
        self.remove_edge_internal(EdgeDirection::Backward, edge);
    }

    pub fn add_successor(self, edge: StatEdge) {
        self.add_edge_internal(EdgeDirection::Forward, edge);
        if let Some(closure) = edge.closure() {
            closure.get_label_edges().borrow_mut().add(edge);
        }
        edge.get_destination().add_predecessor(edge);
    }

    /// Java tolerates `null`; a caller holding an `Option` skips `None`.
    pub fn remove_successor(self, edge: StatEdge) {
        self.remove_edge_internal(EdgeDirection::Forward, edge);
        if let Some(closure) = edge.closure() {
            closure.get_label_edges().borrow_mut().remove(&edge);
        }
        edge.get_destination().remove_predecessor(edge);
    }

    pub fn remove_all_successors(self, stat: Statement) {
        for edge in self.get_all_successor_edges() {
            if edge.get_destination() == stat {
                self.remove_successor(edge);
            }
        }
    }

    pub fn build_continue_set(self) -> Shared<JHashSet<Statement>> {
        match self.type_() {
            StatementType::Do => DoStatement::build_continue_set(DoStatement(self)),
            _ => self.build_continue_set_base(),
        }
    }

    pub fn build_continue_set_base(self) -> Shared<JHashSet<Statement>> {
        let continue_set = self.get_continue_set();
        continue_set.borrow_mut().clear();

        let stats = self.get_stats().borrow().to_vec();
        for st in stats {
            let child = st.build_continue_set();
            let items = child.borrow().to_vec();
            continue_set.borrow_mut().add_all(items);
            if Some(st) != self.get_first() {
                let basichead = st.get_basichead().0;
                continue_set.borrow_mut().remove(&basichead);
            }
        }

        for edge in self.get_edges(StatEdge::TYPE_CONTINUE, EdgeDirection::Forward) {
            let basichead = edge.get_destination().get_basichead().0;
            continue_set.borrow_mut().add(basichead);
        }

        continue_set
    }

    pub fn build_monitor_flags(self) {
        let stats = self.get_stats().borrow().to_vec();
        for st in &stats {
            st.build_monitor_flags();
        }

        match self.type_() {
            StatementType::BasicBlock => {
                let bblock = BasicBlockStatement(self);
                let seq = bblock.get_block().get_seq();

                if seq.length() > 0 {
                    for i in 0..seq.length() {
                        if seq.get_instr(i).with(|d| d.opcode) == OPC_MONITOREXIT {
                            self.with_mut(|d| d.contains_monitor_exit = true);
                            break;
                        }
                    }
                    let last = seq.get_last_instr().with(|d| d.opcode);
                    self.with_mut(|d| {
                        d.is_monitor_enter = last == OPC_MONITORENTER;
                        d.is_last_athrow = last == OPC_ATHROW;
                    });
                }
            }
            StatementType::Synchronized | StatementType::Root | StatementType::General => {}
            _ => {
                let mut contains_monitor_exit = false;
                let mut is_last_athrow = false;
                for st in &stats {
                    contains_monitor_exit |= st.contains_monitor_exit();
                    is_last_athrow |= st.with(|d| d.is_last_athrow);
                }
                self.with_mut(|d| {
                    d.contains_monitor_exit = contains_monitor_exit;
                    d.is_last_athrow = is_last_athrow;
                });
            }
        }
    }

    pub fn mark_monitorexit_dead(self) {
        match self.type_() {
            StatementType::BasicBlock => {
                BasicBlockStatement::mark_monitorexit_dead(BasicBlockStatement(self))
            }
            _ => self.mark_monitorexit_dead_base(),
        }
    }

    pub fn mark_monitorexit_dead_base(self) {
        let stats = self.get_stats().borrow().to_vec();
        for st in stats {
            st.mark_monitorexit_dead();
        }
    }

    /// Java: getReversePostOrderList()
    pub fn get_reverse_post_order_list(self) -> Vec<Statement> {
        self.get_reverse_post_order_list_from(self.get_first().expect("first"))
    }

    /// Java: getReversePostOrderList(Statement)
    pub fn get_reverse_post_order_list_from(self, stat: Statement) -> Vec<Statement> {
        let mut res = Vec::new();
        Statement::add_to_reverse_post_order_list_iterative(stat, &mut res);
        res
    }

    /// Java: getPostReversePostOrderList()
    pub fn get_post_reverse_post_order_list(self) -> Vec<Statement> {
        self.get_post_reverse_post_order_list_exits(None)
    }

    /// Java: getPostReversePostOrderList(List<Statement>)
    pub fn get_post_reverse_post_order_list_exits(
        self,
        lstexits: Option<Vec<Statement>>,
    ) -> Vec<Statement> {
        let mut res = Vec::new();

        let lstexits = match lstexits {
            Some(lstexits) => lstexits,
            None => {
                let schelper = StrongConnectivityHelper::new(self);
                StrongConnectivityHelper::get_exit_reps(schelper.get_components())
            }
        };

        let mut set_visited = JHashSet::new();
        for exit in lstexits {
            Statement::add_to_post_reverse_post_order_list(exit, &mut res, &mut set_visited);
        }

        if res.len() != self.get_stats().borrow().len() {
            jthrow!("computing post reverse post order failed!");
        }

        res.reverse();
        res
    }

    pub fn contains_statement(self, stat: Statement) -> bool {
        self == stat || self.contains_statement_strict(stat)
    }

    pub fn contains_statement_strict(self, stat: Statement) -> bool {
        let stats = self.get_stats();
        let stats = stats.borrow();
        if stats.contains(&stat) {
            return true;
        }
        for st in stats.iter() {
            if st.contains_statement_strict(stat) {
                return true;
            }
        }
        false
    }

    /// Java: toJava()
    pub fn to_java_default(self) -> TextBuffer {
        self.to_java(0)
    }

    /// Java: toJava(int)
    pub fn to_java(self, indent: i32) -> TextBuffer {
        match self.type_() {
            StatementType::Root => RootStatement::to_java(RootStatement(self), indent),
            StatementType::BasicBlock => {
                BasicBlockStatement::to_java(BasicBlockStatement(self), indent)
            }
            StatementType::Sequence => SequenceStatement::to_java(SequenceStatement(self), indent),
            StatementType::General => GeneralStatement::to_java(GeneralStatement(self), indent),
            StatementType::If => IfStatement::to_java(IfStatement(self), indent),
            StatementType::Do => DoStatement::to_java(DoStatement(self), indent),
            StatementType::Switch => SwitchStatement::to_java(SwitchStatement(self), indent),
            StatementType::Synchronized => {
                SynchronizedStatement::to_java(SynchronizedStatement(self), indent)
            }
            StatementType::TryCatch => CatchStatement::to_java(CatchStatement(self), indent),
            StatementType::CatchAll => CatchAllStatement::to_java(CatchAllStatement(self), indent),
            StatementType::DummyExit | StatementType::Other => jthrow!("not implemented"),
        }
    }

    /// Extra, non-body exprents (headers, resources, catch variables). A Java
    /// `null` head slot is absent rather than present as `null`.
    pub fn get_stat_exprents(self) -> Vec<Exprent> {
        match self.type_() {
            StatementType::If => IfStatement::get_stat_exprents(IfStatement(self)),
            StatementType::Do => DoStatement::get_stat_exprents(DoStatement(self)),
            StatementType::Switch => SwitchStatement::get_stat_exprents(SwitchStatement(self)),
            StatementType::TryCatch => CatchStatement::get_stat_exprents(CatchStatement(self)),
            StatementType::Synchronized => {
                SynchronizedStatement::get_stat_exprents(SynchronizedStatement(self))
            }
            _ => Vec::new(),
        }
    }

    pub fn init_exprents(self) {
        match self.type_() {
            StatementType::If => IfStatement::init_exprents(IfStatement(self)),
            StatementType::Switch => SwitchStatement::init_exprents(SwitchStatement(self)),
            StatementType::Synchronized => {
                SynchronizedStatement::init_exprents(SynchronizedStatement(self))
            }
            _ => {}
        }
    }

    pub fn replace_exprent(self, oldexpr: Exprent, newexpr: Exprent) {
        match self.type_() {
            StatementType::If => IfStatement::replace_exprent(IfStatement(self), oldexpr, newexpr),
            StatementType::Do => DoStatement::replace_exprent(DoStatement(self), oldexpr, newexpr),
            StatementType::Switch => {
                SwitchStatement::replace_exprent(SwitchStatement(self), oldexpr, newexpr)
            }
            StatementType::Synchronized => SynchronizedStatement::replace_exprent(
                SynchronizedStatement(self),
                oldexpr,
                newexpr,
            ),
            StatementType::BasicBlock => {
                BasicBlockStatement::replace_exprent(BasicBlockStatement(self), oldexpr, newexpr)
            }
            _ => {}
        }
    }

    pub fn get_simple_copy(self) -> Statement {
        match self.type_() {
            StatementType::If => IfStatement::get_simple_copy(IfStatement(self)),
            StatementType::Do => DoStatement::get_simple_copy(DoStatement(self)),
            StatementType::Switch => SwitchStatement::get_simple_copy(SwitchStatement(self)),
            StatementType::Sequence => SequenceStatement::get_simple_copy(SequenceStatement(self)),
            StatementType::TryCatch => CatchStatement::get_simple_copy(CatchStatement(self)),
            StatementType::CatchAll => CatchAllStatement::get_simple_copy(CatchAllStatement(self)),
            StatementType::Synchronized => {
                SynchronizedStatement::get_simple_copy(SynchronizedStatement(self))
            }
            StatementType::BasicBlock => {
                BasicBlockStatement::get_simple_copy(BasicBlockStatement(self))
            }
            _ => jthrow!("not implemented"),
        }
    }

    pub fn init_simple_copy(self) {
        match self.type_() {
            StatementType::If => IfStatement::init_simple_copy(IfStatement(self)),
            StatementType::Switch => SwitchStatement::init_simple_copy(SwitchStatement(self)),
            StatementType::CatchAll => CatchAllStatement::init_simple_copy(CatchAllStatement(self)),
            StatementType::Synchronized => {
                SynchronizedStatement::init_simple_copy(SynchronizedStatement(self))
            }
            _ => self.init_simple_copy_base(),
        }
    }

    pub fn init_simple_copy_base(self) {
        let stats = self.get_stats();
        let first = {
            let stats = stats.borrow();
            (!stats.is_empty()).then(|| stats.get(0))
        };
        if let Some(first) = first {
            self.set_first(first);
        }
    }

    pub fn replace_with(self, stat: Statement) {
        self.get_parent()
            .expect("parent")
            .replace_statement(self, stat);
    }

    pub fn replace_with_empty(self) -> BasicBlockStatement {
        let new_stat = BasicBlockStatement::create();
        self.replace_with(new_stat.0);
        new_stat
    }

    pub fn replace_statement(self, oldstat: Statement, newstat: Statement) {
        match self.type_() {
            StatementType::If => {
                IfStatement::replace_statement(IfStatement(self), oldstat, newstat)
            }
            StatementType::Switch => {
                SwitchStatement::replace_statement(SwitchStatement(self), oldstat, newstat)
            }
            StatementType::CatchAll => {
                CatchAllStatement::replace_statement(CatchAllStatement(self), oldstat, newstat)
            }
            StatementType::Synchronized => SynchronizedStatement::replace_statement(
                SynchronizedStatement(self),
                oldstat,
                newstat,
            ),
            _ => self.replace_statement_base(oldstat, newstat),
        }
    }

    pub fn replace_statement_base(self, oldstat: Statement, newstat: Statement) {
        let stats = self.get_stats();
        if !stats.borrow().contains_key(&oldstat.id()) {
            let listed = stats
                .borrow()
                .to_vec()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            jthrow!(
                IllegalStateException;
                "[{}] Cannot replace {} with {} because it wasn't found in [{}]",
                self, oldstat, newstat, listed
            );
        }

        for edge in oldstat.get_all_predecessor_edges() {
            oldstat.remove_predecessor(edge);
            edge.get_source()
                .change_edge_node(EdgeDirection::Forward, edge, newstat);
            newstat.add_predecessor(edge);
        }

        for edge in oldstat.get_all_successor_edges() {
            oldstat.remove_successor(edge);
            edge.set_source(newstat);
            newstat.add_successor(edge);
        }

        let statindex = stats.borrow().get_index_by_key(&oldstat.id());
        stats.borrow_mut().remove_with_key(&oldstat.id());
        stats
            .borrow_mut()
            .add_with_key_and_index(statindex as usize, newstat, newstat.id());

        newstat.set_parent(Some(self));
        let oldpost = oldstat.get_post();
        newstat.with_mut(|d| d.post = oldpost);

        if self.get_first() == Some(oldstat) {
            self.set_first(newstat);
        }

        let lst = oldstat.get_label_edges().borrow().to_vec();

        for &edge in lst.iter().rev() {
            if edge.get_source() != newstat {
                newstat.add_labeled_edge(edge);
            } else if self == edge.get_destination()
                || self.contains_statement_strict(edge.get_destination())
            {
                edge.with_mut(|d| d.closure = None);
            } else {
                self.add_labeled_edge(edge);
            }
        }

        Statement::replace_closure(self, oldstat, newstat);

        oldstat.get_label_edges().borrow_mut().clear();
    }

    fn replace_closure(stat: Statement, oldstat: Statement, newstat: Statement) {
        for edge in stat.get_all_successor_edges() {
            if edge.closure() == Some(oldstat) {
                edge.with_mut(|d| d.closure = Some(newstat));
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Statement::replace_closure(st, oldstat, newstat);
        }
    }

    /// `null` (`None`) or the implicitly defined variables.
    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        match self.type_() {
            StatementType::If => IfStatement::get_implicitly_defined_vars(IfStatement(self)),
            StatementType::Do => DoStatement::get_implicitly_defined_vars(DoStatement(self)),
            StatementType::Switch => {
                SwitchStatement::get_implicitly_defined_vars(SwitchStatement(self))
            }
            StatementType::TryCatch => {
                CatchStatement::get_implicitly_defined_vars(CatchStatement(self))
            }
            StatementType::CatchAll => {
                CatchAllStatement::get_implicitly_defined_vars(CatchAllStatement(self))
            }
            StatementType::BasicBlock => {
                BasicBlockStatement::get_implicitly_defined_vars(BasicBlockStatement(self))
            }
            _ => None,
        }
    }

    /// Prepends to `lst` like the Java `lst.add(0, node)`.
    fn add_to_reverse_post_order_list_iterative(root: Statement, lst: &mut Vec<Statement>) {
        let mut stack_node: Vec<Statement> = vec![root];
        let mut stack_index: Vec<usize> = vec![0];
        let mut set_visited = JHashSet::new();
        let mut prepended = Vec::new();

        while let Some(&node) = stack_node.last() {
            let mut index = stack_index.pop().expect("index stack");

            set_visited.add(node);

            let lst_edges =
                node.edge_list_or_create(EdgeDirection::Forward, Statement::STATEDGE_ALL);

            while index < lst_edges.len() {
                let edge = lst_edges.get(index);
                let succ = edge.get_destination();
                let type_ = edge.get_type();

                if !set_visited.contains(&succ)
                    && (type_ == StatEdge::TYPE_REGULAR || type_ == StatEdge::TYPE_EXCEPTION)
                {
                    stack_index.push(index + 1);
                    stack_node.push(succ);
                    stack_index.push(0);
                    break;
                }
                index += 1;
            }

            if index == lst_edges.len() {
                prepended.push(node);
                stack_node.pop();
            }
        }

        prepended.reverse();
        prepended.append(lst);
        *lst = prepended;
    }

    /// Appends in reverse of Java's `lst.add(0, stat)`; the caller reverses.
    fn add_to_post_reverse_post_order_list(
        stat: Statement,
        lst: &mut Vec<Statement>,
        set_visited: &mut JHashSet<Statement>,
    ) {
        if set_visited.contains(&stat) {
            return;
        }

        set_visited.add(stat);

        let regular = stat.edge_list_or_create(EdgeDirection::Backward, StatEdge::TYPE_REGULAR);
        for prededge in regular.iter() {
            let pred = prededge.get_source();
            if !set_visited.contains(&pred) {
                Statement::add_to_post_reverse_post_order_list(pred, lst, set_visited);
            }
        }

        let exceptions =
            stat.edge_list_or_create(EdgeDirection::Backward, StatEdge::TYPE_EXCEPTION);
        for prededge in exceptions.iter() {
            let pred = prededge.get_source();
            if !set_visited.contains(&pred) {
                Statement::add_to_post_reverse_post_order_list(pred, lst, set_visited);
            }
        }

        lst.push(stat);
    }

    fn edge_list_or_create(self, direction: EdgeDirection, type_: i32) -> JList<StatEdge> {
        self.with_mut(|d| {
            edge_map_mut(d, direction)
                .compute_if_absent(type_, JList::new)
                .clone()
        })
    }

    fn edge_list(self, direction: EdgeDirection, type_: i32) -> Option<JList<StatEdge>> {
        self.with(|d| edge_map(d, direction).get(&type_).cloned())
    }

    pub fn change_edge_node(self, direction: EdgeDirection, edge: StatEdge, value: Statement) {
        if direction == EdgeDirection::Backward {
            edge.set_source(value);
        } else {
            edge.set_destination(value);
        }
    }

    pub fn change_edge_type(self, direction: EdgeDirection, edge: StatEdge, newtype: i32) {
        let oldtype = edge.get_type();
        if oldtype == newtype {
            return;
        }

        if oldtype == StatEdge::TYPE_EXCEPTION || newtype == StatEdge::TYPE_EXCEPTION {
            jthrow!("Invalid edge type!");
        }

        self.remove_edge_direct_internal(direction, edge, oldtype);
        self.add_edge_direct_internal(direction, edge, newtype);

        if direction == EdgeDirection::Forward {
            edge.get_destination()
                .change_edge_type(EdgeDirection::Backward, edge, newtype);
        }

        edge.set_type(newtype);
    }

    fn get_edges(self, type_: i32, direction: EdgeDirection) -> Vec<StatEdge> {
        self.with(|d| {
            let map = edge_map(d, direction);
            if is_single_type(type_) {
                map.get(&type_).map_or_else(Vec::new, JList::to_vec)
            } else {
                let mut res = Vec::new();
                for edgetype in StatEdge::TYPES {
                    if type_ & edgetype != 0 {
                        if let Some(lst) = map.get(&edgetype) {
                            res.extend(lst.to_vec());
                        }
                    }
                }
                res
            }
        })
    }

    pub fn get_neighbours(self, type_: i32, direction: EdgeDirection) -> Vec<Statement> {
        let end = |edge: StatEdge| {
            if direction == EdgeDirection::Forward {
                edge.get_destination()
            } else {
                edge.get_source()
            }
        };
        self.with(|d| {
            let map = edge_map(d, direction);
            let mut res = Vec::new();
            if is_single_type(type_) {
                if let Some(stat_edges) = map.get(&type_) {
                    res.extend(stat_edges.to_vec().into_iter().map(end));
                }
            } else {
                for edgetype in StatEdge::TYPES {
                    if type_ & edgetype != 0 {
                        if let Some(lst) = map.get(&edgetype) {
                            res.extend(lst.to_vec().into_iter().map(end));
                        }
                    }
                }
            }
            res
        })
    }

    pub fn get_neighbours_set(self, type_: i32, direction: EdgeDirection) -> JHashSet<Statement> {
        JHashSet::from_collection(self.get_neighbours(type_, direction))
    }

    pub fn get_successor_edges(self, type_: i32) -> Vec<StatEdge> {
        self.get_edges(type_, EdgeDirection::Forward)
    }

    /// The live internal list (created on first request); do not mutate it.
    pub fn get_successor_edge_view(self, type_: i32) -> JList<StatEdge> {
        self.edge_list_or_create(EdgeDirection::Forward, type_)
    }

    pub fn get_predecessor_edges(self, type_: i32) -> Vec<StatEdge> {
        self.get_edges(type_, EdgeDirection::Backward)
    }

    pub fn get_all_successor_edges(self) -> Vec<StatEdge> {
        self.get_edges(Statement::STATEDGE_ALL, EdgeDirection::Forward)
    }

    pub fn get_all_direct_successor_edges(self) -> Vec<StatEdge> {
        self.get_edges(Statement::STATEDGE_DIRECT_ALL, EdgeDirection::Forward)
    }

    pub fn has_any_successor(self) -> bool {
        self.has_successor(Statement::STATEDGE_ALL)
    }

    pub fn has_any_direct_successor(self) -> bool {
        self.has_successor(Statement::STATEDGE_DIRECT_ALL)
    }

    pub fn has_successor(self, type_: i32) -> bool {
        self.with(|d| {
            let map = &d.map_succ_edges;
            let mut res = false;
            if is_single_type(type_) {
                res = map.get(&type_).is_some_and(|edges| !edges.is_empty());
            } else {
                for edgetype in StatEdge::TYPES {
                    if type_ & edgetype != 0 {
                        if let Some(lst) = map.get(&edgetype) {
                            res = !lst.is_empty();
                            if res {
                                return true;
                            }
                        }
                    }
                }
            }
            res
        })
    }

    pub fn get_first_successor(self) -> StatEdge {
        ValidationHelper::successors_exist(self);
        if let Some(edge) = self
            .edge_list(EdgeDirection::Forward, Statement::STATEDGE_ALL)
            .and_then(|res| res.first())
        {
            return edge;
        }
        jthrow!(IllegalStateException; "No successor exists for {}", self)
    }

    pub fn get_first_direct_successor(self) -> StatEdge {
        ValidationHelper::successors_exist(self);
        if let Some(edge) = self
            .edge_list(EdgeDirection::Forward, Statement::STATEDGE_DIRECT_ALL)
            .and_then(|res| res.first())
        {
            return edge;
        }
        jthrow!(IllegalStateException; "No successor exists for {}", self)
    }

    pub fn get_all_predecessor_edges(self) -> Vec<StatEdge> {
        self.get_edges(Statement::STATEDGE_ALL, EdgeDirection::Backward)
    }

    pub fn get_first(self) -> Option<Statement> {
        self.with(|d| d.first)
    }

    pub fn set_first(self, first: Statement) {
        self.with_mut(|d| d.first = Some(first))
    }

    pub fn get_post(self) -> Option<Statement> {
        self.with(|d| d.post)
    }

    /// The live child collection (aliases the statement's own).
    pub fn get_stats(self) -> Shared<VBStyleCollection<Statement, i32>> {
        self.with(|d| d.stats.clone())
    }

    pub fn get_last_basic_type(self) -> LastBasicType {
        self.with(|d| d.last_basic_type)
    }

    /// The live set; `null` after `clearTempInformation`, where this panics like the NPE.
    pub fn get_continue_set(self) -> Shared<JHashSet<Statement>> {
        self.with(|d| d.continue_set.clone())
            .expect("continueSet cleared by clearTempInformation")
    }

    pub fn contains_monitor_exit(self) -> bool {
        self.with(|d| d.contains_monitor_exit)
    }

    pub fn contains_monitor_exit_or_athrow(self) -> bool {
        self.with(|d| d.contains_monitor_exit || d.is_last_athrow)
    }

    pub fn is_monitor_enter(self) -> bool {
        self.with(|d| d.is_monitor_enter)
    }

    pub fn get_basichead(self) -> BasicBlockStatement {
        match self.type_() {
            StatementType::BasicBlock => {
                BasicBlockStatement::get_basichead(BasicBlockStatement(self))
            }
            _ => self.get_first().expect("first").get_basichead(),
        }
    }

    pub fn is_labeled(self) -> bool {
        let label_edges = self.get_label_edges();
        let labeled = label_edges
            .borrow()
            .iter()
            .any(|edge| edge.labeled() && edge.explicit());
        labeled
    }

    pub fn has_basic_succ_edge(self) -> bool {
        match self.type_() {
            StatementType::If => IfStatement::has_basic_succ_edge(IfStatement(self)),
            StatementType::Do => DoStatement::has_basic_succ_edge(DoStatement(self)),
            StatementType::Switch => SwitchStatement::has_basic_succ_edge(SwitchStatement(self)),
            StatementType::BasicBlock => {
                BasicBlockStatement::has_basic_succ_edge(BasicBlockStatement(self))
            }
            _ => false,
        }
    }

    pub fn get_parent(self) -> Option<Statement> {
        self.with(|d| d.parent)
    }

    pub fn set_parent(self, parent: Option<Statement>) {
        self.with_mut(|d| d.parent = parent)
    }

    pub fn get_top_parent(self) -> RootStatement {
        let mut ret = self;
        while let Some(parent) = ret.get_parent() {
            ret = parent;
        }
        match ret.try_as_root() {
            Some(root) => root,
            None => {
                jthrow!(IllegalStateException; "Top parent is not a root statement! Malformed IR?")
            }
        }
    }

    /// The live label-edge set (aliases the statement's own).
    pub fn get_label_edges(self) -> Shared<JHashSet<StatEdge>> {
        self.with(|d| d.label_edges.clone())
    }

    /// The live list (aliases the statement's own).
    pub fn get_var_definitions(self) -> JList<Exprent> {
        self.with(|d| d.var_definitions.clone())
    }

    /// Body exprents, only non-`None` for basic blocks; aliases the statement's list.
    pub fn get_exprents(self) -> Option<JList<Exprent>> {
        self.with(|d| d.exprents.clone())
    }

    pub fn set_exprents(self, exprents: JList<Exprent>) {
        ValidationHelper::assert_true(true, "Shouldn't set null exprent array here");
        self.with_mut(|d| d.exprents = Some(exprents))
    }

    pub fn is_copied(self) -> bool {
        self.with(|d| d.copied)
    }

    pub fn set_copied(self, copied: bool) {
        self.with_mut(|d| d.copied = copied)
    }

    pub fn is_phantom(self) -> bool {
        self.with(|d| d.phantom)
    }

    pub fn set_phantom(self, phantom: bool) {
        self.with_mut(|d| d.phantom = phantom)
    }

    pub fn to_debug(self) -> String {
        unported!("Statement.toDebug")
    }

    pub fn get_offset(self, values: &mut BitSet) {
        match self.type_() {
            StatementType::TryCatch => CatchStatement::get_offset(CatchStatement(self), values),
            _ => self.get_offset_base(values),
        }
    }

    pub fn get_offset_base(self, values: &mut BitSet) {
        if self.is_dummy_exit() {
            if let Some(bytecode) = DummyExitStatement(self).d(|d| d.bytecode.clone()) {
                values.or(&bytecode);
            }
        }
        if let Some(exprents) = self.get_exprents() {
            for e in exprents.iter() {
                e.get_bytecode_range(values);
            }
        } else {
            let stats = self.get_stats().borrow().to_vec();
            for st in stats {
                st.get_offset(values);
            }

            for e in self.get_stat_exprents() {
                e.get_bytecode_range(values);
            }
        }
    }

    pub fn get_start_end_range(self) -> StartEndPair {
        match self.type_() {
            StatementType::If => IfStatement::get_start_end_range(IfStatement(self)),
            StatementType::Switch => SwitchStatement::get_start_end_range(SwitchStatement(self)),
            StatementType::BasicBlock => {
                BasicBlockStatement::get_start_end_range(BasicBlockStatement(self))
            }
            StatementType::Root => RootStatement::get_start_end_range(RootStatement(self)),
            _ => self.get_start_end_range_base(),
        }
    }

    pub fn get_start_end_range_base(self) -> StartEndPair {
        if let Some(endpoints) = self.with(|d| d.endpoints) {
            return endpoints;
        }
        let mut set = BitSet::new();
        self.get_offset(&mut set);
        let endpoints = StartEndPair::new(set.next_set_bit(0), set.length() as i32 - 1);
        self.with_mut(|d| d.endpoints = Some(endpoints));
        endpoints
    }

    pub fn find_object(self, match_node: &MatchNode, index: i32) -> Option<Box<dyn IMatchable>> {
        match self.type_() {
            StatementType::If => IfStatement::find_object(IfStatement(self), match_node, index),
            _ => self.find_object_base(match_node, index),
        }
    }

    pub fn find_object_base(
        self,
        _match_node: &MatchNode,
        _index: i32,
    ) -> Option<Box<dyn IMatchable>> {
        unported!("Statement.findObject")
    }

    pub fn match_(self, match_node: &MatchNode, engine: &mut MatchEngine) -> bool {
        match self.type_() {
            StatementType::If => IfStatement::match_(IfStatement(self), match_node, engine),
            _ => self.match_base(match_node, engine),
        }
    }

    pub fn match_base(self, _match_node: &MatchNode, _engine: &mut MatchEngine) -> bool {
        unported!("Statement.match")
    }
}

impl IMatchable for Statement {
    fn find_object(&self, match_node: &MatchNode, index: i32) -> Option<Box<dyn IMatchable>> {
        Statement::find_object(*self, match_node, index)
    }

    fn match_(&self, match_node: &MatchNode, engine: &mut MatchEngine) -> bool {
        Statement::match_(*self, match_node, engine)
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (type_, id) = self.with(|d| (d.type_, d.id));
        write!(f, "{{{}}}:{}", type_.pretty_id(), id)
    }
}
