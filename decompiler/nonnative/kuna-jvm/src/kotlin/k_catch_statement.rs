//! Port of `org/vineflower/kotlin/stat/KCatchStatement.java`.

use crate::catch_statement::CatchStatement;
use crate::code_constants::OPC_ASTORE;
use crate::expr_processor::ExprProcessor;
use crate::k_if_statement::copy_statement_fields;
use crate::kotlin_statement;
use crate::text_buffer::TextBuffer;

kotlin_statement!(pub KCatchStatement: CatchStatement, TryCatch,
                  is_k_catch, as_k_catch, try_as_k_catch);

impl KCatchStatement {
    /// Java: KCatchStatement(CatchStatement)
    pub fn new(statement: CatchStatement) -> KCatchStatement {
        let this = CatchStatement::new_empty();
        this.0.set_kt(true);
        this.get_exct_strings()
            .add_all(statement.get_exct_strings().to_vec());
        this.get_vars().add_all(statement.get_vars().to_vec());
        this.get_resources()
            .add_all(statement.get_resources().to_vec());

        copy_statement_fields(this.0, statement.0);

        KCatchStatement(this)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut indent = indent;
        let mut buf = TextBuffer::new();
        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        if self.is_labeled() {
            buf.append_indent(indent)
                .append("label")
                .append_int(self.id())
                .append("@ ");
        }

        buf.append_indent(indent)
            .append("try {")
            .append_line_separator();
        indent += 1;

        let resources = self.get_resources().to_vec();
        for resource in &resources {
            buf.append_int(indent);
            indent += 1;
            buf.append_buffer(&resource.to_java(indent))
                .append(".use {")
                .append_line_separator();
        }

        buf.append_buffer(&ExprProcessor::jmp_wrapper(
            self.get_first().expect("NullPointerException: first"),
            indent,
            false,
        ));

        indent -= 1;
        buf.append_indent(indent).append("}");
        for _ignored in self.get_resources().to_vec() {
            buf.append_line_separator();
            indent -= 1;
            buf.append_indent(indent).append("}");
        }

        let stats = self.get_stats();
        let mut i = 1;
        while i < stats.borrow().len() {
            let stat = stats.borrow().get(i);

            let block = stat.get_basichead().get_block();
            if !block.get_seq().is_empty()
                && block.get_instruction(0).with(|d| d.opcode) == OPC_ASTORE
            {
                let offset = block.get_old_offset(0);
                if offset > -1 {
                    buf.add_bytecode_mapping_offset(offset);
                }
            }

            buf.append(" catch (");
            buf.append_buffer(&self.get_vars().get(i - 1).0.to_java(indent));
            buf.append(") {").append_line_separator();
            buf.append_buffer(&stat.to_java(indent + 1));
            buf.append_indent(indent).append("}");
            i += 1;
        }

        buf.append_line_separator();

        buf
    }
}
