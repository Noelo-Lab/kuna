//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/CatchAllStatement.java`.
//!
//! kuna fix `protected_back_edge` (`kuna_fix_protected_back_edge`): no
//! `try` is built around a loop body whose back edge it would protect.

use crate::catch_statement::new_object_var;
use crate::counter_container::CounterContainer;
use crate::dec_helper::DecHelper;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::java_subclass;
use crate::kuna_fixes::{self, KunaFix};
use crate::p7_regions::kuna_fix_protected_back_edge;
use crate::rt::{JList, JavaStr};
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

pub struct CatchAllStatementData {
    pub handler: Option<Statement>,
    pub is_finally: bool,
    pub monitor: Option<VarExprent>,
    pub vars: JList<VarExprent>,
    /// kuna fix `finally_exit_versions`: the handler slots the finally
    /// processing matched to other slots in a removed copy of the body.
    pub kuna_remapped_slots: Vec<i32>,
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
                kuna_remapped_slots: Vec::new(),
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

        if kuna_fixes::enabled(KunaFix::ProtectedBackEdge)
            && kuna_fix_protected_back_edge::is_open_loop_body(head)
        {
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

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let new_line_separator = DecompilerContext::get_new_line_separator();

        let mut buf = TextBuffer::new();

        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        let labeled = self.is_labeled();
        if labeled {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append(":")
                .append_line_separator();
        }

        let (is_finally, monitor, handler) = self.d(|d| (d.is_finally, d.monitor, d.handler));
        let first = self.get_first().expect("first");
        let lst_succs = first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if first.is_catch()
            && first.get_var_definitions().is_empty()
            && is_finally
            && !labeled
            && !first.is_labeled()
            && (lst_succs.is_empty() || !lst_succs[0].explicit())
        {
            let mut content = ExprProcessor::jmp_wrapper(first, indent, true);
            content.set_length(content.length() - new_line_separator.jlen() as i32);
            buf.append_buffer(&content);
        } else {
            buf.append_indent(indent)
                .append("try {")
                .append_line_separator();
            buf.append_buffer(&ExprProcessor::jmp_wrapper(first, indent + 1, true));
            buf.append_indent(indent).append("}");
        }

        if is_finally {
            buf.append(" finally");
        } else {
            buf.append(" catch (")
                .append_buffer(&self.get_vars().get(0).0.to_java(indent))
                .append(")");
        }
        buf.append(" {").append_line_separator();

        if let Some(monitor) = monitor {
            buf.append_indent(indent + 1)
                .append("if (")
                .append_buffer(&monitor.0.to_java(indent))
                .append(") {")
                .append_line_separator();
        }

        buf.append_buffer(&ExprProcessor::jmp_wrapper(
            handler.expect("NullPointerException"),
            indent + 1 + if monitor.is_some() { 1 } else { 0 },
            true,
        ));

        if monitor.is_some() {
            buf.append_indent(indent + 1)
                .append("}")
                .append_line_separator();
        }

        buf.append_indent(indent)
            .append("}")
            .append_line_separator();

        buf
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
        let remapped = self.get_kuna_remapped_slots();
        cas.set_kuna_remapped_slots(remapped);

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

    pub fn get_kuna_remapped_slots(self) -> Vec<i32> {
        self.d(|d| d.kuna_remapped_slots.clone())
    }

    pub fn set_kuna_remapped_slots(self, slots: Vec<i32>) {
        self.d_mut(|d| d.kuna_remapped_slots = slots)
    }

    /// The live list.
    pub fn get_vars(self) -> JList<VarExprent> {
        self.d(|d| d.vars.clone())
    }

    pub fn get_implicitly_defined_vars(self) -> Option<Vec<VarExprent>> {
        Some(self.get_vars().to_vec())
    }
}
