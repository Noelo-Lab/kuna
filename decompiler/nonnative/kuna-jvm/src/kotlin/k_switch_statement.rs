//! Port of `org/vineflower/kotlin/stat/KSwitchStatement.java`.
//!
//! kuna fix `kt_when_case_type`: a case constant is written with the
//! selector's type (`':' ->` on a `Char`), where Vineflower retypes a copy it
//! never prints and writes the constant's own type (`58 ->`).
//!
//! kuna fix `kt_jump_to_non_loop_label`: a jump out of a labelled `switch`,
//! written in `run labelN@{ .. }`, is `return@labelN`
//! (`kuna_fix_kt_jump_to_non_loop_label`).
//!
//! kuna fix `kt_when_break`: a `when` that an unlabelled `break` leaves from
//! inside its branch is written in `run labelN@{ .. }`
//! (`kuna_fix_kt_when_break`).
//!
//! kuna fix `kt_switch_head_indent`: the switch's first block is written at
//! the statement's indentation, as Java's `SwitchStatement` writes it;
//! Vineflower writes it at indentation 0 after one indent, so every line of
//! it but the first starts in column 0.
//!
//! kuna fix `kt_enum_when_mappings`: an enum constant label (which only that
//! fix writes into a Kotlin `when`) names its class as Java source does
//! (`ExprProcessor::build_java_class_name`); Vineflower passes the internal
//! name, `java/net/Proxy.Type` for a class outside the input.

use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::k_const_exprent::KConstExprent;
use crate::k_expr_processor::KExprProcessor;
use crate::k_if_statement::copy_statement_fields;
use crate::kotlin_statement;
use crate::kuna_fix_kt_jump_to_non_loop_label::{self, Lambda};
use crate::kuna_fix_kt_when_break;
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
        if self.is_labeled() && kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel) {
            return kuna_fix_kt_jump_to_non_loop_label::within(self.into(), Lambda::Run, || {
                self.write(indent, true, true)
            });
        }
        if kuna_fixes::enabled(KunaFix::KtWhenBreak)
            && !self.is_labeled()
            && !self.is_phantom()
            && kuna_fix_kt_when_break::needs_run(self.into())
        {
            return kuna_fix_kt_jump_to_non_loop_label::within(self.into(), Lambda::Run, || {
                self.write(indent, true, false)
            });
        }
        self.write(indent, self.is_labeled(), true)
    }

    /// `head_in_run`: the switch's first block is written inside the `run`
    /// lambda, not before it.
    fn write(self, indent: i32, run: bool, head_in_run: bool) -> TextBuffer {
        let mut indent = indent;
        let mut buf = TextBuffer::new();
        buf.append_buffer(&ExprProcessor::list_to_java(
            Some(&self.get_var_definitions()),
            indent,
        ));

        let first = self.get_first().expect("NullPointerException: first");
        let indented = kuna_fixes::enabled(KunaFix::KtSwitchHeadIndent);
        let first_buf = if indented {
            first.to_java(if run && head_in_run { indent + 1 } else { indent })
        } else {
            first.to_java_default()
        };
        if !head_in_run && first_buf.length() > 0 {
            if !indented {
                buf.append_indent(indent);
            }
            buf.append_buffer(&first_buf);
        }

        if run {
            buf.append_indent(indent)
                .append("run label")
                .append_int(self.id())
                .append("@{")
                .append_line_separator();
            indent += 1;
        }

        if !(indented && head_in_run && first_buf.length() > 0) {
            buf.append_indent(indent);
        }
        if head_in_run {
            buf.append_buffer(&first_buf);
        }

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

        if head_in_run && first_buf.length() > 0 {
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
                        .get_short_name(&if kuna_fixes::enabled(KunaFix::KtEnumWhenMappings) {
                            ExprProcessor::build_java_class_name(&classname)
                        } else {
                            classname.clone()
                        })
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

        if run {
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
