//! Port of `org/vineflower/kotlin/stat/KSequenceStatement.java`.

use crate::expr_processor::ExprProcessor;
use crate::k_expr_processor::KExprProcessor;
use crate::k_if_statement::copy_statement_fields;
use crate::kotlin_statement;
use crate::sequence_statement::SequenceStatement;
use crate::text_buffer::TextBuffer;

kotlin_statement!(pub KSequenceStatement: SequenceStatement, Sequence,
                  is_k_sequence, as_k_sequence, try_as_k_sequence);

impl KSequenceStatement {
    /// Java: KSequenceStatement(SequenceStatement)
    pub fn new(statement: SequenceStatement) -> KSequenceStatement {
        let this = SequenceStatement::new_empty();
        this.0.set_kt(true);

        copy_statement_fields(this.0, statement.0);

        KSequenceStatement(this)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut indent = indent;
        let mut buf = TextBuffer::new();
        let labeled = self.is_labeled();

        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        if labeled {
            buf.append_indent(indent)
                .append("run label")
                .append_int(self.id())
                .append("@{")
                .append_line_separator();
            indent += 1;
        }

        let stats = self.get_stats();
        let mut not_empty = false;
        let mut i = 0;
        while i < stats.borrow().len() {
            let st = stats.borrow().get(i);
            let str = KExprProcessor::jmp_wrapper(st, indent, false);

            if i > 0 && !str.contains_only_whitespaces() && not_empty {
                buf.append_line_separator();
            }

            buf.append_buffer(&str);

            not_empty = !str.contains_only_whitespaces();
            i += 1;
        }

        if labeled {
            indent -= 1;
            buf.append_indent(indent)
                .append("}")
                .append_line_separator();
        }

        buf
    }
}
