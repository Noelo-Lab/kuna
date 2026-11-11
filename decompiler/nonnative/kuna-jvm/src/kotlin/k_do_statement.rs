//! Port of `org/vineflower/kotlin/stat/KDoStatement.java`.
//!
//! kuna fixes (`kuna_fixes`) in the `FOR` case: `kt_for_loop_exit_condition`
//! writes a loop it cannot resugar as `while (COND)`, where Vineflower writes
//! `while (true) { if (COND) break` and so leaves the loop when it should go on;
//! `kt_for_range_counter` (`kuna_fix_kt_for_range_counter`) rewrites only
//! loops that count their initializer's variable to the bound;
//! `kt_for_range_exclusive_bound` keeps the bound exclusive (`until` for a
//! variable bound, `repeat(N)` with the bound itself), where Vineflower writes
//! the inclusive `..` of a variable and `repeat(N - 1)`;
//! `kt_for_loop_definitions` writes the declarations of such a loop's
//! variables, which Vineflower leaves out;
//! `kt_for_loop_continue_update` writes such a loop whose body continues it
//! as a `do`-`while` whose condition runs the update, where Vineflower's
//! `while` puts the update at the end of the body, which `continue` skips;
//! `kt_for_range_unmatched_bound` writes the loop of a bound it cannot match
//! as a `while`, where Vineflower's `break resugar` writes no loop at all.
//! `kt_jump_to_non_loop_label` (`kuna_fix_kt_jump_to_non_loop_label`) writes
//! a jump out of such a loop wrapped in `run labelN@{ .. }` as
//! `return@labelN`, and a `continue` of a `repeat` loop as a `return` from
//! its lambda.
//! `kt_range_operand_precedence` parenthesises a range bound that binds less
//! tightly than the range operator, where Vineflower writes
//! `for (i in if (c) 1 else 0 until n)`, whose `else` branch is the range.

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
use crate::kuna_fix_kt_for_range_counter;
use crate::kuna_fix_kt_jump_to_non_loop_label::{self, Lambda};
use crate::kuna_fixes::{self, KunaFix};
use crate::stat_edge::StatEdge;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

kotlin_statement!(pub KDoStatement: DoStatement, Do,
                  is_k_do, as_k_do, try_as_k_do);

/// `KUtils.replaceExprent(x)`, which maps `null` to `null`.
fn replace_exprent(ex: Option<Exprent>) -> Option<Exprent> {
    ex.and_then(KUtils::replace_exprent)
}

/// An operand of a range operator (`..`, `until`, `downTo`). With fix
/// `kt_range_operand_precedence` an operand that binds less tightly than
/// addition is parenthesised: Kotlin's `if`, elvis, infix calls and
/// comparisons take the range operator into their own operand.
fn range_operand(expr: Exprent) -> TextBuffer {
    let mut buf = expr.to_java_default();
    if kuna_fixes::enabled(KunaFix::KtRangeOperandPrecedence)
        && expr.get_precedence() > FunctionType::Add.precedence()
    {
        buf.enclose("(", ")");
    }
    buf
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
                            count_expr,
                            exclusive,
                        } = header;

                        let repeat = inc_type.is_pp()
                            && init
                                .get_right()
                                .try_as_const()
                                .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::Int(0))));
                        if repeat {
                            buf.append("repeat(");
                            buf.append_buffer(&count_expr.to_java_default())
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
                            buf.append_buffer(&range_operand(init.get_right())).append(
                                if exclusive {
                                    " until "
                                } else if inc_type.is_pp() {
                                    ".."
                                } else {
                                    " downTo "
                                },
                            );
                            buf.append_buffer(&range_operand(condition_expr))
                                .append(") {")
                                .append_line_separator();
                        }

                        let body = || KExprProcessor::jmp_wrapper(first(), indent + 1, false);
                        buf.append_buffer(&if repeat && kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel) {
                            kuna_fix_kt_jump_to_non_loop_label::within(self.into(), Lambda::Repeat, body)
                        } else {
                            body()
                        });
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

                        let definitions = self.get_var_definitions();
                        if kuna_fixes::enabled(KunaFix::KtForLoopDefinitions)
                            && !definitions.is_empty()
                        {
                            buf.set_length(0);
                            buf.append_buffer(&ExprProcessor::list_to_java(
                                Some(&definitions),
                                indent,
                            ));
                            buf.append_indent(indent);
                        }

                        buf.append("// $VF: Unable to resugar Kotlin loop from Java for loop")
                            .append_line_separator()
                            .append_indent(indent);

                        let do_form = kuna_fixes::enabled(KunaFix::KtForLoopContinueUpdate)
                            && self
                                .get_predecessor_edges(StatEdge::TYPE_CONTINUE)
                                .iter()
                                .any(|edge| edge.explicit());
                        if labeled && !do_form {
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

                        let condition = condition.expect("NullPointerException: condition");
                        let inc = inc.expect("NullPointerException: inc");
                        if do_form {
                            self.write_continue_update_loop(&mut buf, indent, condition, inc);
                            return buf;
                        }
                        if kuna_fixes::enabled(KunaFix::KtForLoopExitCondition) {
                            buf.append("while (");
                            buf.push_newline_group(indent, 1);
                            buf.append_possible_newline();
                            buf.append_buffer(&condition.to_java(indent));
                            buf.append_possible_newline_dedent("", true);
                            buf.pop_newline_group();
                            buf.append(") {").append_line_separator();
                        } else {
                            buf.append("while (true) {").append_line_separator();
                            buf.append_indent(indent + 1);
                            buf.append("if (");
                            buf.append_buffer(&condition.to_java(indent + 1));
                            buf.append(") break").append_line_separator();
                        }
                        let body = || KExprProcessor::jmp_wrapper(first(), indent + 1, false);
                        buf.append_buffer(&if labeled && kuna_fixes::enabled(KunaFix::KtJumpToNonLoopLabel) {
                            kuna_fix_kt_jump_to_non_loop_label::within(self.into(), Lambda::Run, body)
                        } else {
                            body()
                        });
                        buf.append_line_separator();
                        buf.append_indent(indent + 1);
                        buf.append_buffer(&inc.to_java(indent + 1))
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

    /// `kt_for_loop_continue_update`: `if (C) { do { BODY } while (run { INC; C }) }`,
    /// the form of a `for (; C; INC)` whose body continues it: a `continue` of
    /// a `do`-`while` evaluates its condition, which runs `INC` first.
    fn write_continue_update_loop(
        self,
        buf: &mut TextBuffer,
        indent: i32,
        condition: Exprent,
        inc: Exprent,
    ) {
        let first = self.get_first().expect("NullPointerException: first");
        buf.append("if (");
        buf.append_buffer(&condition.to_java(indent));
        buf.append(") {").append_line_separator();
        buf.append_indent(indent + 1);
        if self.is_labeled() {
            buf.append("label").append_int(self.id()).append("@ ");
        }
        buf.append("do {").append_line_separator();
        buf.append_buffer(&KExprProcessor::jmp_wrapper(first, indent + 2, false));
        buf.append_indent(indent + 1).append("} while (run { ");
        buf.append_buffer(&inc.to_java(indent + 1)).append("; ");
        buf.append_buffer(&condition.to_java(indent + 1))
            .append(" })");
        buf.append_line_separator();
        buf.append_indent(indent)
            .append("}")
            .append_line_separator();
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
        if kuna_fixes::enabled(KunaFix::KtForRangeCounter)
            && !kuna_fix_kt_for_range_counter::is_counted_loop(self.0, var_expr, inc, condition)
        {
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
            } else if kuna_fixes::enabled(KunaFix::KtForRangeUnmatchedBound) {
                return Resugar::NotRange;
            } else {
                return Resugar::Abandoned;
            }
        };

        let exclusive_bound = kuna_fixes::enabled(KunaFix::KtForRangeExclusiveBound);
        let mut exclusive = false;
        if exclusive_bound {
            match condition_expr.try_as_const().and_then(|c| c.get_value()) {
                Some(ConstValue::Int(i))
                    if Self::step_past(i as i64, inc.get_func_type())
                        .is_none_or(|v| v != v as i32 as i64) =>
                {
                    return Resugar::NotRange;
                }
                Some(ConstValue::Long(l)) if Self::step_past(l, inc.get_func_type()).is_none() => {
                    return Resugar::NotRange;
                }
                Some(_) => {}
                None if inc.get_func_type().is_pp() => exclusive = true,
                None => return Resugar::NotRange,
            }
        }
        let mut count_expr = condition_expr;

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
            } else if let (true, Some(ConstValue::Long(l))) =
                (exclusive_bound, condition_const.get_value())
            {
                let new_value = Self::step_past(l, inc.get_func_type()).expect("checked above");
                condition_expr = KConstExprent::new(ConstExprent::new(
                    var_expr.0 .0.get_expr_type(),
                    Some(ConstValue::Long(new_value)),
                    condition_const.get_bytecode().as_ref(),
                ))
                .0
                 .0;
            }
        }
        if !exclusive_bound {
            count_expr = condition_expr;
        }

        if let Some(const_expr) = init.get_right().try_as_const() {
            const_expr.set_const_type(var_expr.0 .0.get_expr_type());
        }

        Resugar::Range(RangeHeader {
            init,
            var_expr,
            inc_type: inc.get_func_type(),
            condition_expr,
            count_expr,
            exclusive,
        })
    }

    /// `kt_for_range_exclusive_bound`: the last value a loop stepping by
    /// `inc` reaches before the exclusive bound `bound`, if it exists.
    fn step_past(bound: i64, inc: FunctionType) -> Option<i64> {
        if inc.is_pp() {
            bound.checked_sub(1)
        } else {
            bound.checked_add(1)
        }
    }
}

struct RangeHeader {
    init: crate::assignment_exprent::AssignmentExprent,
    var_expr: KVarExprent,
    inc_type: FunctionType,
    condition_expr: Exprent,
    count_expr: Exprent,
    exclusive: bool,
}

enum Resugar {
    Range(RangeHeader),
    Abandoned,
    NotRange,
}
