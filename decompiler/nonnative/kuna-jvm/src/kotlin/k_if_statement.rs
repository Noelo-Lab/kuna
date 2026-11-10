//! Port of `org/vineflower/kotlin/stat/KIfStatement.java`.
//!
//! kuna fix `kt_jump_to_non_loop_label`: a labelled `if` that a written jump
//! leaves is written as `run labelN@{ .. }`, and such jumps as `return@labelN`
//! (`kuna_fix_kt_jump_to_non_loop_label`).
//!
//! kuna fix `kt_when_break`: an unlabelled `break` out of a `when` that is
//! the `if`'s only statement is left out or written `return@labelN`
//! (`kuna_fix_kt_when_break`).

use crate::expr_processor::ExprProcessor;
use crate::if_statement::{set_head_slot, IfStatement};
use crate::k_expr_processor::KExprProcessor;
use crate::kotlin_statement;
use crate::kuna_fix_kt_jump_to_non_loop_label::{self, Lambda};
use crate::kuna_fix_kt_when_break;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::JavaStr;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;

kotlin_statement!(pub KIfStatement: IfStatement, If,
                  is_k_if, as_k_if, try_as_k_if);

/// The field copies every Kotlin statement copy constructor makes from the
/// Java statement it wraps (children are not re-parented).
pub(crate) fn copy_statement_fields(this: Statement, statement: Statement) {
    let first = statement.get_first();
    this.with_mut(|d| d.first = first);
    let (stats, keys) = {
        let stats = statement.get_stats();
        let stats = stats.borrow();
        (stats.to_vec(), stats.get_lst_keys().clone())
    };
    {
        let own = this.get_stats();
        let mut own = own.borrow_mut();
        for (stat, key) in stats.into_iter().zip(keys) {
            match key {
                Some(key) => own.add_with_key(stat, key),
                None => {
                    own.add(stat);
                }
            }
        }
    }
    let (
        parent,
        first_again,
        exprents,
        post,
        last_basic_type,
        is_monitor_enter,
        contains_monitor_exit,
    ) = statement.with(|d| {
        (
            d.parent,
            d.first,
            d.exprents.clone(),
            d.post,
            d.last_basic_type,
            d.is_monitor_enter,
            d.contains_monitor_exit,
        )
    });
    let label_edges = statement.get_label_edges().borrow().to_vec();
    let var_definitions = statement.get_var_definitions().to_vec();
    let contains_monitor_exit_or_athrow = statement.contains_monitor_exit_or_athrow();
    let continue_set = statement.with(|d| d.continue_set.clone());

    this.with_mut(|d| {
        d.parent = parent;
        d.first = first_again;
        d.exprents = exprents;
    });
    this.get_label_edges().borrow_mut().add_all(label_edges);
    this.get_var_definitions().add_all(var_definitions);
    this.with_mut(|d| {
        d.post = post;
        d.last_basic_type = last_basic_type;
        d.is_monitor_enter = is_monitor_enter;
        d.contains_monitor_exit = contains_monitor_exit;
        d.is_last_athrow = contains_monitor_exit_or_athrow && !d.contains_monitor_exit;
        d.continue_set = continue_set;
    });
}

impl KIfStatement {
    /// Java: KIfStatement(IfStatement)
    pub fn new(statement: IfStatement) -> KIfStatement {
        let this = IfStatement::new_empty();
        this.0.set_kt(true);

        copy_statement_fields(this.0, statement.0);

        this.set_ifstat(statement.get_ifstat());
        this.set_elsestat(statement.get_elsestat());
        this.set_if_edge(statement.get_if_edge());
        this.set_else_edge(statement.get_else_edge());
        this.set_negated(statement.is_negated());
        this.set_pattern_matched(statement.is_pattern_matched());
        this.set_has_ppmm(statement.has_ppmm());
        if let Some(head) = statement.get_headexprent_list().first() {
            set_head_slot(&this.get_headexprent_list(), head.as_if().0);
        }

        KIfStatement(this)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();

        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        buf.append_buffer(&self.get_first().expect("first").to_java(indent));

        if self.is_labeled()
            && kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel)
            && kuna_fix_kt_jump_to_non_loop_label::jumped_out_of(self.into())
        {
            buf.append_indent(indent)
                .append("run label")
                .append_int(self.id())
                .append("@{")
                .append_line_separator();
            kuna_fix_kt_jump_to_non_loop_label::within(self.into(), Lambda::Run, || {
                self.write_if(&mut buf, indent + 1)
            });
            buf.append_indent(indent)
                .append("}")
                .append_line_separator();
            return buf;
        }

        if self.is_labeled() {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append("@")
                .append_line_separator();
        }

        self.write_if(&mut buf, indent);
        buf
    }

    /// The `if` from its condition on.
    fn write_if(self, buf: &mut TextBuffer, indent: i32) {
        let condition = self.get_headexprent();
        buf.append_indent(indent);
        buf.append_buffer(&condition.0.to_java(indent))
            .append(" {")
            .append_line_separator();

        match self.get_ifstat() {
            None => {
                let if_edge = self.get_if_edge().expect("NullPointerException: ifEdge");
                let when_break = (kuna_fixes::enabled(KunaFix::KtWhenBreak) && if_edge.explicit())
                    .then(|| kuna_fix_kt_when_break::jump(if_edge, self.into()))
                    .flatten();
                if let Some(when_break) = when_break {
                    if !when_break.is_empty() {
                        buf.append_indent(indent + 1)
                            .append(&when_break)
                            .append_line_separator();
                    }
                } else if if_edge.explicit() {
                    let lambda_return = kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel)
                        && kuna_fix_kt_jump_to_non_loop_label::returns(if_edge);
                    buf.append_indent(indent + 1);
                    if lambda_return {
                        buf.append("return");
                    } else if if_edge.get_type() == StatEdge::TYPE_BREAK {
                        if if_edge.closure().is_some_and(|c| c.is_k_sequence()) {
                            buf.append("return");
                        } else {
                            buf.append("break");
                        }
                    } else {
                        buf.append("continue");
                    }

                    if if_edge.labeled() {
                        let closure = if_edge.closure().expect("NullPointerException: closure");
                        buf.append("@label").append_int(closure.id());
                    } else if if_edge.closure().is_some_and(|c| c.is_k_sequence()) {
                        buf.append("@run");
                    } else if lambda_return {
                        buf.append("@repeat");
                    }

                    buf.append_line_separator();
                }
            }
            Some(ifstat) => {
                buf.append_buffer(&KExprProcessor::jmp_wrapper(ifstat, indent + 1, false));
            }
        }

        let mut else_if = false;

        if let Some(elsestat) = self.get_elsestat() {
            if elsestat.is_k_if()
                && elsestat.get_var_definitions().is_empty()
                && elsestat
                    .get_first()
                    .expect("NullPointerException: first")
                    .get_exprents()
                    .is_some_and(|e| e.is_empty())
                && !elsestat.is_labeled()
                && (elsestat
                    .get_successor_edges(Statement::STATEDGE_DIRECT_ALL)
                    .is_empty()
                    || !elsestat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)[0].explicit())
            {
                else_if = true;
                buf.append_indent(indent).append("} else ");

                let mut content = KExprProcessor::jmp_wrapper(elsestat, indent, false);
                content.set_start(TextUtil::get_indent_string(indent).jlen() as i32);
                buf.append_buffer(&content);
            } else {
                let content = KExprProcessor::jmp_wrapper(elsestat, indent + 1, false);

                if content.length() > 0 {
                    buf.append_indent(indent)
                        .append("} else {")
                        .append_line_separator();
                    buf.append_buffer(&content);
                }
            }
        }

        if !else_if {
            buf.append_indent(indent)
                .append("}")
                .append_line_separator();
        }
    }
}
