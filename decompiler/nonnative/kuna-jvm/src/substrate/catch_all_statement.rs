//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/CatchAllStatement.java`.

use crate::catch_statement::new_object_var;
use crate::counter_container::CounterContainer;
use crate::dec_helper::DecHelper;
use crate::decompiler_context::DecompilerContext;
use crate::rt::JList;
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

pub struct CatchAllStatementData {
    pub handler: Option<Statement>,
    pub is_finally: bool,
    pub monitor: Option<VarExprent>,
    pub vars: JList<VarExprent>,
}

java_subclass!(pub CatchAllStatement: Statement, StatementKind::CatchAll(CatchAllStatementData), is_catch_all, as_catch_all, try_as_catch_all);

impl CatchAllStatement {
    /// Java: CatchAllStatement()
    pub fn new_empty() -> CatchAllStatement {
        CatchAllStatement(Statement::new(
            StatementType::CatchAll,
            StatementKind::CatchAll(CatchAllStatementData {
                handler: None,
                is_finally: false,
                monitor: None,
                vars: JList::new(),
            }),
        ))
    }

    /// Java: CatchAllStatement(Statement, Statement)
    pub fn new(head: Statement, handler: Statement) -> CatchAllStatement {
        let this = CatchAllStatement::new_empty();

        this.set_first(head);
        let stats = this.get_stats();
        stats.borrow_mut().add_with_key(head, head.id());

        this.d_mut(|d| d.handler = Some(handler));
        stats.borrow_mut().add_with_key(handler, handler.id());

        let lst_succs = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if let Some(&edge) = lst_succs.first() {
            if edge.get_type() == StatEdge::TYPE_REGULAR {
                let post = edge.get_destination();
                this.with_mut(|d| d.post = Some(post));
            }
        }

        this.get_vars().add(new_object_var("java/lang/Throwable"));
        this
    }

    pub fn is_head(head: Statement) -> Option<Statement> {
        if head.get_last_basic_type() != LastBasicType::General {
            return None;
        }

        let set_handlers = DecHelper::get_unique_pred_exceptions(head);
        if set_handlers.len() != 1 {
            return None;
        }

        for edge in head.get_successor_edges(StatEdge::TYPE_EXCEPTION) {
            let exc = edge.get_destination();

            if edge.get_exceptions().is_none()
                && exc.get_last_basic_type() == LastBasicType::General
                && set_handlers.contains(&exc)
            {
                let lst_succs = exc.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                if lst_succs.is_empty() || lst_succs[0].get_type() != StatEdge::TYPE_REGULAR {
                    if head.is_monitor_enter() || exc.is_monitor_enter() {
                        return None;
                    }

                    if DecHelper::invalid_head_merge(head) {
                        return None;
                    }

                    if DecHelper::check_statement_exceptions(&[head, exc]) {
                        return Some(CatchAllStatement::new(head, exc).0);
                    }
                }
            }
        }

        None
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("CatchAllStatement.toJava")
    }

    pub fn replace_statement(self, oldstat: Statement, newstat: Statement) {
        self.d_mut(|d| {
            if d.handler == Some(oldstat) {
                d.handler = Some(newstat);
            }
        });

        self.0.replace_statement_base(oldstat, newstat);
    }

    pub fn get_simple_copy(self) -> Statement {
        let cas = CatchAllStatement::new_empty();

        let (is_finally, has_monitor, has_vars) =
            self.d(|d| (d.is_finally, d.monitor.is_some(), !d.vars.is_empty()));
        cas.d_mut(|d| d.is_finally = is_finally);

        if has_monitor {
            let index = DecompilerContext::get_counter_container()
                .get_counter_and_increment(CounterContainer::VAR_COUNTER);
            let monitor = VarExprent::new(
                index,
                Some(VarType::VARTYPE_INT()),
                Some(DecompilerContext::get_var_processor()),
            );
            cas.d_mut(|d| d.monitor = Some(monitor));
        }

        if has_vars {
            cas.get_vars().add(new_object_var("java/lang/Throwable"));
        }

        cas.0
    }

    pub fn init_simple_copy(self) {
        let stats = self.get_stats();
        let (first, handler) = {
            let stats = stats.borrow();
            (stats.get(0), stats.get(1))
        };
        self.set_first(first);
        self.d_mut(|d| d.handler = Some(handler));
    }

    pub fn get_handler(self) -> Statement {
        self.d(|d| d.handler).expect("catch-all handler")
    }

    pub fn is_finally(self) -> bool {
        self.d(|d| d.is_finally)
    }

    pub fn set_finally(self, is_finally: bool) {
        self.d_mut(|d| d.is_finally = is_finally)
    }

    pub fn get_monitor(self) -> Option<VarExprent> {
        self.d(|d| d.monitor)
    }

    pub fn set_monitor(self, monitor: Option<VarExprent>) {
        self.d_mut(|d| d.monitor = monitor)
    }

    /// The live list.
    pub fn get_vars(self) -> JList<VarExprent> {
        self.d(|d| d.vars.clone())
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        Some(self.get_vars().to_vec())
    }
}
