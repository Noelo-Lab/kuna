//! Port of `org/vineflower/kotlin/stat/KDoStatement.java`.

use crate::const_exprent::{ConstExprent, ConstValue};
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_const_exprent::KConstExprent;
use crate::k_expr_processor::KExprProcessor;
use crate::k_if_statement::copy_statement_fields;
use crate::k_utils::KUtils;
use crate::k_var_exprent::{DeclarationType, KVarExprent};
use crate::kotlin_statement;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

kotlin_statement!(pub KDoStatement: DoStatement, Do,
                  is_k_do, as_k_do, try_as_k_do);

/// `KUtils.replaceExprent(x)`, which maps `null` to `null`.
fn replace_exprent(ex: Option<Exprent>) -> Option<Exprent> {
    ex.and_then(KUtils::replace_exprent)
}

impl KDoStatement {
    /// Java: KDoStatement(DoStatement)
    pub fn new(statement: DoStatement) -> KDoStatement {
        let this = DoStatement::new_empty();
        this.0.set_kt(true);
        this.set_looptype(statement.get_looptype());
        if let Some(init) = statement.get_init_exprent() {
            this.set_init_exprent(init);
        }
        if let Some(condition) = statement.get_condition_exprent() {
            this.set_condition_exprent(condition);
        }
        if let Some(inc) = statement.get_inc_exprent() {
            this.set_inc_exprent(inc);
        }

        copy_statement_fields(this.0, statement.0);

        KDoStatement(this)
    }

    fn is_integer_type(type_: &VarType) -> bool {
        VarType::VARTYPE_INT().equals(type_)
            || VarType::VARTYPE_BYTE().equals(type_)
            || VarType::VARTYPE_SHORT().equals(type_)
            || VarType::VARTYPE_CHAR().equals(type_)
            || VarType::VARTYPE_LONG().equals(type_)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut indent = indent;
        let mut buf = TextBuffer::new();
        let labeled = self.is_labeled();
        let id = self.id();
        let first = || self.get_first().expect("NullPointerException: first");

        if self.get_looptype() != LoopType::For {
            buf.append_buffer(&ExprProcessor::list_to_java(
                Some(&self.get_var_definitions()),
                indent,
            ));

            buf.append_indent(indent);

            if labeled {
                buf.append("label").append_int(id).append("@ ");
            }
        }

        match self.get_looptype() {
            LoopType::Infinite => {
                buf.append("while (true) {").append_line_separator();
                buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            LoopType::DoWhile => {
                let expr = replace_exprent(self.get_condition_exprent())
                    .or_else(|| self.get_condition_exprent());

                buf.append("do {").append_line_separator();
                buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                buf.append_indent(indent).append("} while (");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                buf.append_buffer(
                    &expr
                        .expect("NullPointerException: conditionExprent")
                        .to_java(indent),
                );
                buf.append_possible_newline_dedent("", true);
                buf.pop_newline_group();
                buf.append(")").append_line_separator();
            }
            LoopType::While => {
                let expr = replace_exprent(self.get_condition_exprent())
                    .or_else(|| self.get_condition_exprent());

                buf.append("while (");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                buf.append_buffer(
                    &expr
                        .expect("NullPointerException: conditionExprent")
                        .to_java(indent),
                );
                buf.append_possible_newline_dedent("", true);
                buf.pop_newline_group();
                buf.append(") {").append_line_separator();
                buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            LoopType::ForEach => {
                let init = KVarExprent::new(
                    self.get_init_exprent()
                        .expect("NullPointerException: initExprent")
                        .as_var(),
                );
                init.set_declaration_type(Some(DeclarationType::ForLoopVariable));

                let inc =
                    replace_exprent(self.get_inc_exprent()).or_else(|| self.get_inc_exprent());

                buf.append("for (");
                buf.append_buffer(&init.0 .0.to_java(indent));
                let inc = inc.expect("NullPointerException: incExprent");
                inc.get_inferred_expr_type(None);
                buf.append(" in ");
                buf.append_buffer(&inc.to_java(indent))
                    .append(") {")
                    .append_line_separator();
                buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                buf.append_indent(indent)
                    .append("}")
                    .append_line_separator();
            }
            LoopType::For => {
                buf.append_indent(indent);

                match self.resugar_header() {
                    Resugar::Range(header) => {
                        let RangeHeader {
                            init,
                            var_expr,
                            inc_type,
                            condition_expr,
                        } = header;

                        let repeat = inc_type.is_pp()
                            && init
                                .get_right()
                                .try_as_const()
                                .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::Int(0))));
                        if repeat {
                            buf.append("repeat(");
                            buf.append_buffer(&condition_expr.to_java_default())
                                .append(") ");

                            if labeled {
                                buf.append("label").append_int(id).append("@");
                            }

                            buf.append("{");

                            if "it" != var_expr.0.get_name() {
                                buf.append(" ");
                                buf.append_buffer(&var_expr.0 .0.to_java(indent))
                                    .append(" ->");
                            }
                            buf.append_line_separator();
                        } else {
                            if labeled {
                                buf.append("label").append_int(id).append("@ ");
                            }

                            buf.append("for (");
                            buf.append_buffer(&var_expr.0 .0.to_java(indent))
                                .append(" in ");
                            buf.append_buffer(&init.get_right().to_java_default())
                                .append(if inc_type.is_pp() { ".." } else { " downTo " });
                            buf.append_buffer(&condition_expr.to_java_default())
                                .append(") {")
                                .append_line_separator();
                        }

                        buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                        buf.append_indent(indent)
                            .append("}")
                            .append_line_separator();
                    }
                    Resugar::Abandoned => {}
                    Resugar::NotRange => {
                        let init = replace_exprent(self.get_init_exprent())
                            .or_else(|| self.get_init_exprent());

                        let condition = replace_exprent(self.get_condition_exprent())
                            .or_else(|| self.get_condition_exprent());

                        let inc = replace_exprent(self.get_inc_exprent())
                            .or_else(|| self.get_inc_exprent());

                        if labeled {
                            buf.set_length(0);
                            buf.append_indent(indent);
                        }

                        buf.append("// $VF: Unable to resugar Kotlin loop from Java for loop")
                            .append_line_separator()
                            .append_indent(indent);

                        if labeled {
                            indent += 1;
                            buf.append("run label")
                                .append_int(id)
                                .append("@{")
                                .append_line_separator()
                                .append_indent(indent);
                        }

                        if let Some(init) = init {
                            buf.append_buffer(&init.to_java(indent))
                                .append_line_separator()
                                .append_indent(indent);
                        }

                        buf.append("while (true) {").append_line_separator();
                        buf.append_indent(indent + 1);
                        buf.append("if (");
                        buf.append_buffer(
                            &condition
                                .expect("NullPointerException: condition")
                                .to_java(indent + 1),
                        );
                        buf.append(") break").append_line_separator();
                        buf.append_buffer(&KExprProcessor::jmp_wrapper(first(), indent + 1, false));
                        buf.append_line_separator();
                        buf.append_indent(indent + 1);
                        buf.append_buffer(
                            &inc.expect("NullPointerException: inc").to_java(indent + 1),
                        )
                        .append_line_separator();
                        buf.append_indent(indent)
                            .append("}")
                            .append_line_separator();

                        if labeled {
                            indent -= 1;
                            buf.append_indent(indent)
                                .append("}")
                                .append_line_separator();
                        }
                    }
                }
            }
        }

        buf
    }

    /// The `resugar:` labeled `if` of the `FOR` case up to the rendering:
    /// its condition, the range parts it builds, and its `break resugar`.
    fn resugar_header(self) -> Resugar {
        let Some(init) = self.get_init_exprent().and_then(|e| e.try_as_assignment()) else {
            return Resugar::NotRange;
        };
        let Some(var_expr) = init.get_left().try_as_var() else {
            return Resugar::NotRange;
        };
        if !KDoStatement::is_integer_type(&var_expr.0.get_expr_type()) {
            return Resugar::NotRange;
        }
        let Some(inc) = self.get_inc_exprent().and_then(|e| e.try_as_function()) else {
            return Resugar::NotRange;
        };
        if !inc.get_func_type().is_ppmm() {
            return Resugar::NotRange;
        }
        let Some(condition) = self
            .get_condition_exprent()
            .and_then(|e| e.try_as_function())
        else {
            return Resugar::NotRange;
        };
        if condition.get_func_type() != FunctionType::Lt {
            return Resugar::NotRange;
        }

        let var_expr = KVarExprent::new(var_expr);
        var_expr.set_declaration_type(Some(DeclarationType::ForLoopVariable));

        let operands = condition.get_lst_operands();
        let mut condition_expr: Exprent = if let Some(c) = operands.get(1).try_as_const() {
            KConstExprent::new(c).0 .0
        } else if let Some(c) = operands.get(0).try_as_const() {
            KConstExprent::new(c).0 .0
        } else {
            let left = operands.get(0);
            let right = operands.get(1);
            if left
                .try_as_var()
                .is_some_and(|var| var.0.equals(var_expr.0 .0))
                && right.is_var()
            {
                KVarExprent::new(right.as_var()).0 .0
            } else if right
                .try_as_var()
                .is_some_and(|var| var.0.equals(var_expr.0 .0))
                && left.is_var()
            {
                KVarExprent::new(left.as_var()).0 .0
            } else {
                return Resugar::Abandoned;
            }
        };

        if let Some(condition_const) = condition_expr.try_as_const() {
            condition_const.set_const_type(var_expr.0 .0.get_expr_type());
            if let Some(ConstValue::Int(i)) = condition_const.get_value() {
                let new_value = if inc.get_func_type().is_pp() {
                    i.wrapping_sub(1)
                } else {
                    i.wrapping_add(1)
                };
                condition_expr = KConstExprent::new(ConstExprent::new(
                    var_expr.0 .0.get_expr_type(),
                    Some(ConstValue::Int(new_value)),
                    condition_const.get_bytecode().as_ref(),
                ))
                .0
                 .0;
            }
        }

        if let Some(const_expr) = init.get_right().try_as_const() {
            const_expr.set_const_type(var_expr.0 .0.get_expr_type());
        }

        Resugar::Range(RangeHeader {
            init,
            var_expr,
            inc_type: inc.get_func_type(),
            condition_expr,
        })
    }
}

struct RangeHeader {
    init: crate::assignment_exprent::AssignmentExprent,
    var_expr: KVarExprent,
    inc_type: FunctionType,
    condition_expr: Exprent,
}

enum Resugar {
    Range(RangeHeader),
    Abandoned,
    NotRange,
}
