//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/SequenceStatement.java`.

use crate::dec_helper::DecHelper;
use crate::expr_processor::ExprProcessor;
use crate::java_subclass;
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementKind, StatementType};
use crate::text_buffer::TextBuffer;

pub struct SequenceStatementData {}

java_subclass!(pub SequenceStatement: Statement, StatementKind::Sequence(SequenceStatementData), is_sequence, as_sequence, try_as_sequence);

impl SequenceStatement {
    /// Java: SequenceStatement()
    pub fn new_empty() -> SequenceStatement {
        SequenceStatement(Statement::new(
            StatementType::Sequence,
            StatementKind::Sequence(SequenceStatementData {}),
        ))
    }

    /// Java: SequenceStatement(Statement...)
    pub fn new_varargs(stats: &[Statement]) -> SequenceStatement {
        SequenceStatement::new(stats.to_vec())
    }

    /// Java: SequenceStatement(List<? extends Statement>)
    pub fn new(lst: Vec<Statement>) -> SequenceStatement {
        let this = SequenceStatement::new_empty();

        let last_basic_type = lst[lst.len() - 1].get_last_basic_type();
        this.with_mut(|d| d.last_basic_type = last_basic_type);

        let stats = this.get_stats();
        for st in &lst {
            stats.borrow_mut().add_with_key(*st, st.id());
        }

        let first = stats.borrow().get(0);
        this.set_first(first);
        this
    }

    /// Java: SequenceStatement(Statement, Statement); only reachable from this class.
    pub fn new_head_tail(head: Statement, tail: Statement) -> SequenceStatement {
        let this = SequenceStatement::new(vec![head, tail]);

        let lst_succs = tail.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if let Some(&edge) = lst_succs.first() {
            if edge.get_type() == StatEdge::TYPE_REGULAR && edge.get_destination() != head {
                let post = edge.get_destination();
                this.with_mut(|d| d.post = Some(post));
            }
        }
        this
    }

    pub fn is_head2_block(head: Statement) -> Option<Statement> {
        if head.get_last_basic_type() != LastBasicType::General {
            return None;
        }

        let lst_succs = head.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        let edge = lst_succs.first().copied();

        if let Some(edge) = edge {
            if edge.get_type() == StatEdge::TYPE_REGULAR {
                let stat = edge.get_destination();

                if stat != head
                    && stat.get_predecessor_edges(StatEdge::TYPE_REGULAR).len() == 1
                    && !stat.is_monitor_enter()
                    && stat.get_last_basic_type() == LastBasicType::General
                    && DecHelper::check_statement_exceptions(&[head, stat])
                {
                    return Some(SequenceStatement::new_head_tail(head, stat).0);
                }
            }
        }

        None
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut indent = indent;
        let mut buf = TextBuffer::new();
        let islabeled = self.is_labeled();

        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        if islabeled {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append(": {")
                .append_line_separator();
            indent += 1;
        }

        let mut notempty = false;

        let stats = self.get_stats();
        let mut i = 0;
        while i < stats.borrow().len() {
            let st = stats.borrow().get(i);

            let str_ = ExprProcessor::jmp_wrapper(st, indent, false);

            if i > 0 && !str_.contains_only_whitespaces() && notempty {
                buf.append_line_separator();
            }

            buf.append_buffer(&str_);

            notempty = !str_.contains_only_whitespaces();
            i += 1;
        }

        if islabeled {
            buf.append_indent(indent - 1)
                .append("}")
                .append_line_separator();
        }

        buf
    }

    pub fn get_simple_copy(self) -> Statement {
        SequenceStatement::new_empty().0
    }
}
