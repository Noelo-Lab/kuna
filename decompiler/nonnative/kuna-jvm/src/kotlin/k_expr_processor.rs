//! Port of `org/vineflower/kotlin/util/KExprProcessor.java`.
//!
//! kuna fix `kt_jump_to_non_loop_label`: a jump out of a statement written as
//! a lambda is written `return@label` (`kuna_fix_kt_jump_to_non_loop_label`).

use crate::expr_processor::ExprProcessor;
use crate::jthrow;
use crate::kuna_fix_kt_jump_to_non_loop_label;
use crate::kuna_fixes::{self, KunaFix};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;

pub struct KExprProcessor;

impl KExprProcessor {
    pub fn jmp_wrapper(stat: Statement, indent: i32, is_switch: bool) -> TextBuffer {
        let mut buf = stat.to_java(indent);

        let successors = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
        if successors.len() == 1 {
            let edge = successors[0];
            if edge.get_type() != StatEdge::TYPE_REGULAR
                && edge.explicit()
                && !edge.get_destination().is_dummy_exit()
            {
                let mut inner_buf = TextBuffer::new();
                inner_buf.append_indent(indent);

                let closure_is_run = edge.closure().is_some_and(|c| c.is_k_sequence());
                let lambda_return = kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel)
                    && kuna_fix_kt_jump_to_non_loop_label::returns(edge);
                match edge.get_type() {
                    StatEdge::TYPE_BREAK => {
                        ExprProcessor::add_deleted_goto_instruction_mapping(stat, &mut buf);
                        if !is_switch || edge.labeled() {
                            if closure_is_run || lambda_return {
                                inner_buf.append("return");
                            } else {
                                inner_buf.append("break");
                            }
                        }
                    }
                    StatEdge::TYPE_CONTINUE => {
                        ExprProcessor::add_deleted_goto_instruction_mapping(stat, &mut buf);
                        inner_buf.append(if lambda_return { "return" } else { "continue" });
                    }
                    _ => {}
                }

                if edge.labeled() {
                    let closure = edge.closure().unwrap_or_else(
                        || jthrow!(NullPointerException; "StatEdge.closure is null"),
                    );
                    inner_buf.append("@label").append_int(closure.id());
                } else if closure_is_run {
                    inner_buf.append("@run");
                } else if lambda_return {
                    inner_buf.append("@repeat");
                }

                if !inner_buf.contains_only_whitespaces() {
                    buf.append_buffer(&inner_buf).append_line_separator();
                } else {
                    inner_buf.convert_to_string_and_allow_data_discard();
                }
            }
        }

        buf
    }
}
