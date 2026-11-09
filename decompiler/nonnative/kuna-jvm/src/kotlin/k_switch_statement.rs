//! Port of `org/vineflower/kotlin/stat/KSwitchStatement.java`.
//!
//! kuna fix `kt_when_case_type`: a case constant is written with the
//! selector's type (`':' ->` on a `Char`), where Vineflower retypes a copy it
//! never prints and writes the constant's own type (`58 ->`).

use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::k_const_exprent::KConstExprent;
use crate::k_expr_processor::KExprProcessor;
use crate::k_if_statement::copy_statement_fields;
use crate::kotlin_statement;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::JavaStr;
use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

kotlin_statement!(pub KSwitchStatement: SwitchStatement, Switch,
                  is_k_switch, as_k_switch, try_as_k_switch);

impl KSwitchStatement {
    /// Java: KSwitchStatement(SwitchStatement)
    pub fn new(statement: SwitchStatement) -> KSwitchStatement {
        let this = SwitchStatement::new_empty();
        this.0.set_kt(true);

        this.get_case_statements()
            .add_all(statement.get_case_statements().to_vec());
        this.get_case_edges()
            .add_all(statement.get_case_edges().to_vec());
        this.get_case_values()
            .add_all(statement.get_case_values().to_vec());
        this.get_case_guards()
            .add_all(statement.get_case_guards().to_vec());
        this.get_headexprent_list()
            .add_all(statement.get_headexprent_list().to_vec());

        this.0.set_phantom(statement.is_phantom());
        copy_statement_fields(this.0, statement.0);

        this.0.init_simple_copy();

        KSwitchStatement(this)
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
                .append("run label")
                .append_int(self.id())
                .append("@{")
                .append_line_separator();
            indent += 1;
        }

        let first_buf = self
            .get_first()
            .expect("NullPointerException: first")
            .to_java_default();
        buf.append_indent(indent).append_buffer(&first_buf);

        let show_if_hidden =
            DecompilerContext::get_option(IFernflowerPreferences::SHOW_HIDDEN_STATEMENTS);

        if self.is_phantom() {
            if !show_if_hidden {
                return buf;
            } else {
                buf.append_indent(indent)
                    .append("/*")
                    .append_line_separator();
            }
        }

        if first_buf.length() > 0 {
            buf.append_indent(indent);
        }

        buf.append_buffer(&self.get_headexprent().to_java(indent))
            .append(" {")
            .append_line_separator();

        let switch_type = self.get_headexprent().get_expr_type();

        let case_statements = self.get_case_statements();
        let case_edges = self.get_case_edges();
        let case_values = self.get_case_values();
        let mut i = 0;
        while i < case_statements.len() {
            let stat = case_statements.get(i);
            let edges = case_edges.get(i);
            let values = case_values.get(i);

            let mut any_non_default = false;
            for j in 0..edges.len() {
                if Some(edges.get(j)) == self.get_default_edge() {
                    continue;
                }

                any_non_default = true;

                let mut value = values.get(j);

                if j == 0 {
                    buf.append_indent(indent + 1);
                } else {
                    buf.append(", ");
                }

                if let Some(v) = value {
                    if v.is_const() && !VarType::VARTYPE_NULL().equals(&v.get_expr_type()) {
                        if kuna_fixes::enabled(KunaFix::KtWhenCaseType) {
                            let copied = KConstExprent::new(v.copy().as_const());
                            copied.set_const_type(switch_type.clone());
                            value = Some(copied.0 .0);
                        } else {
                            let const_value = v.copy().as_const();
                            const_value.set_const_type(switch_type.clone());
                        }
                    }
                }

                if let Some(field) = value
                    .and_then(|v| v.try_as_field())
                    .filter(|f| f.is_static())
                {
                    let import_collector = DecompilerContext::get_import_collector();
                    let classname = field.get_classname();
                    let short_name = import_collector
                        .get_short_name(&classname)
                        .expect("NullPointerException: short name");
                    let name = field.get_name();
                    buf.append_class(&short_name, false, &classname)
                        .append_char('.')
                        .append_field_descriptor(
                            &name,
                            false,
                            &classname,
                            &name,
                            &field.get_descriptor(),
                        );
                } else {
                    buf.append_buffer(
                        &value
                            .expect("NullPointerException: case value")
                            .to_java(indent),
                    );
                }
            }

            if any_non_default {
                buf.append(" -> ");
                self.write_case(indent + 1, &mut buf, stat);
            }
            i += 1;
        }

        if let Some(default_edge) = self.get_default_edge() {
            buf.append_indent(indent + 1).append("else -> ");

            self.write_case(indent + 1, &mut buf, default_edge.get_destination());
        }

        buf.append_indent(indent)
            .append("}")
            .append_line_separator();

        if self.is_labeled() {
            indent -= 1;
            buf.append_indent(indent)
                .append("}")
                .append_line_separator();
        }

        if self.is_phantom() {
            buf.append_indent(indent)
                .append("*/")
                .append_line_separator();
        }

        buf
    }

    fn write_case(self, indent: i32, buf: &mut TextBuffer, stat: Statement) {
        let mut body = KExprProcessor::jmp_wrapper(stat, indent + 1, true);
        if body.count_lines() > 1 {
            buf.append("{")
                .append_line_separator()
                .append_buffer(&body)
                .append_indent(indent)
                .append("}")
                .append_line_separator();
        } else if !body.contains_only_whitespaces() {
            let indent_size = DecompilerContext::get_property(IFernflowerPreferences::INDENT_STRING)
                .expect("NullPointerException: indent-string")
                .jlen() as i32;
            body.set_start(indent_size * (indent + 1));
            buf.append_buffer(&body);
        } else {
            buf.append("{}").append_line_separator();
        }
    }
}
