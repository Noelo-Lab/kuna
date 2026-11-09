//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SimplifyExprentsHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::match_engine::MatchEngine;
use crate::monitor_exprent::Type as MonitorType;
use crate::pair::Pair;
use crate::rt::{JHashSet, JList};
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::unported;
use crate::var_exprent::VarExprent;

pub struct SimplifyExprentsHelper;

impl SimplifyExprentsHelper {
    /// The `MatchEngine` description lines of the inlined 1.4 `.class` idiom.
    const CLASS14_BUILDER: [&'static str; 22] = [
        "statement type:if iftype:if exprsize:-1",
        " exprent position:head type:if",
        "  exprent type:function functype:eq",
        "   exprent type:field name:$fieldname$",
        "   exprent type:constant consttype:null",
        " statement type:basicblock",
        "  exprent position:-1 type:assignment ret:$assignfield$",
        "   exprent type:var index:$var$",
        "   exprent type:field name:$fieldname$",
        " statement type:sequence statsize:2",
        "  statement type:trycatch",
        "   statement type:basicblock exprsize:1",
        "    exprent type:assignment",
        "     exprent type:var index:$var$",
        "     exprent type:invocation invclass:java/lang/Class signature:forName(Ljava/lang/String;)Ljava/lang/Class;",
        "      exprent position:0 type:constant consttype:string constvalue:$classname$",
        "   statement type:basicblock exprsize:1",
        "    exprent type:exit exittype:throw",
        "  statement type:basicblock exprsize:1",
        "   exprent type:assignment",
        "    exprent type:field name:$fieldname$ ret:$field$",
        "    exprent type:var index:$var$",
    ];

    /// Java: the static `class14Builder`, a `MatchEngine` over [`Self::CLASS14_BUILDER`].
    fn class14_builder() -> MatchEngine {
        unported!("SimplifyExprentsHelper.class14Builder")
    }

    pub fn simplify_stack_vars_statement(
        stat: Statement,
        set_reordered_ifs: &mut JHashSet<i32>,
        ssa: Option<SSAConstructorSparseEx>,
        cl: StructClass,
        first_invocation: bool,
    ) -> bool {
        unported!("SimplifyExprentsHelper.simplifyStackVarsStatement")
    }

    fn simplify_stack_vars_exprents(
        list: &JList<Exprent>,
        cl: StructClass,
        ssa: Option<SSAConstructorSparseEx>,
        stat: Statement,
        first_invocation: bool,
    ) -> bool {
        unported!("SimplifyExprentsHelper.simplifyStackVarsExprents")
    }

    fn add_array_initializer(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.addArrayInitializer")
    }

    fn is_array_initializer(list: &JList<Exprent>, index: i32) -> i32 {
        unported!("SimplifyExprentsHelper.isArrayInitializer")
    }

    fn is_assignment_return(first: Exprent, second: Exprent, stat: Statement) -> bool {
        unported!("SimplifyExprentsHelper.isAssignmentReturn")
    }

    fn is_trivial_stack_assignment(first: Exprent) -> bool {
        let Some(asf) = first.try_as_assignment() else {
            return false;
        };
        let Some(left_var) = asf.get_left().try_as_var() else {
            return false;
        };
        if !left_var.is_stack() {
            return false;
        }
        let Some(right_var) = asf.get_right().try_as_var() else {
            return false;
        };
        left_var.get_index() == right_var.get_index()
    }

    fn is_stack_assignment2(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isStackAssignment2")
    }

    fn is_stack_var(exprent: Exprent) -> bool {
        exprent.try_as_var().is_some_and(|v| v.is_stack())
    }

    fn is_stack_assignment(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isStackAssignment")
    }

    fn is_inlined_method_reference_assign(
        ssa: Option<SSAConstructorSparseEx>,
        first: Exprent,
        second: Exprent,
    ) -> bool {
        unported!("SimplifyExprentsHelper.isInlinedMethodReferenceAssign")
    }

    /// Java: isPPIorMMI(Exprent)
    fn is_ppi_or_mmi(first: Exprent) -> Option<Exprent> {
        unported!("SimplifyExprentsHelper.isPPIorMMI")
    }

    /// Java: isIPPorIMM(Exprent, Exprent)
    fn is_ipp_or_imm(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isIPPorIMM")
    }

    /// Java: isIPPorIMM2(Exprent, Exprent)
    fn is_ipp_or_imm2(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isIPPorIMM2")
    }

    fn inline_ppi_and_mmi(expr: Exprent, next: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.inlinePPIAndMMI")
    }

    fn find_first_valid_usage(
        match_: VarExprent,
        next: Exprent,
    ) -> Option<Pair<Exprent, VarExprent>> {
        unported!("SimplifyExprentsHelper.findFirstValidUsage")
    }

    fn is_ppmm(func: FunctionExprent) -> bool {
        let func_type = func.get_func_type();
        func_type == FunctionType::Ppi
            || func_type == FunctionType::Mmi
            || func_type == FunctionType::Ipp
            || func_type == FunctionType::Imm
    }

    fn is_monitor_exit(first: Exprent) -> bool {
        let Some(expr) = first.try_as_monitor() else {
            return false;
        };
        expr.get_mon_type() == MonitorType::Exit
            && expr
                .get_value()
                .try_as_var()
                .is_some_and(|var_expr| !var_expr.is_stack())
            && expr.is_removable()
    }

    fn has_qualified_new_get_class(parent: Statement, child: Statement) -> bool {
        unported!("SimplifyExprentsHelper.hasQualifiedNewGetClass")
    }

    fn is_qualified_new_get_class(first: Exprent, second: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isQualifiedNewGetClass")
    }

    fn is_unambiguously_same_param(
        is_static: bool,
        target: Exprent,
        parameters: &JList<Exprent>,
    ) -> bool {
        let first_param_of_same_type = parameters
            .get(0)
            .get_expr_type()
            .equals(&target.get_expr_type());
        if !is_static {
            return first_param_of_same_type;
        }
        if !first_param_of_same_type {
            return false;
        }

        for i in 1..parameters.len() {
            if parameters
                .get(i)
                .get_expr_type()
                .equals(&target.get_expr_type())
            {
                return false;
            }
        }

        true
    }

    fn is_method_array_assign(expr: Exprent, next: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isMethodArrayAssign")
    }

    fn is_constructor_invocation_remote(list: &JList<Exprent>, index: i32) -> bool {
        unported!("SimplifyExprentsHelper.isConstructorInvocationRemote")
    }

    fn is_swap_constructor_invocation(last: Exprent, expr: Exprent, next: Exprent) -> bool {
        unported!("SimplifyExprentsHelper.isSwapConstructorInvocation")
    }

    fn is_lambda(exprent: Exprent, cl: StructClass) -> Option<Exprent> {
        unported!("SimplifyExprentsHelper.isLambda")
    }

    fn is_simple_constructor_invocation(exprent: Exprent) -> Option<Exprent> {
        unported!("SimplifyExprentsHelper.isSimpleConstructorInvocation")
    }

    fn build_iff(stat: Statement, ssa: Option<SSAConstructorSparseEx>) -> bool {
        unported!("SimplifyExprentsHelper.buildIff")
    }

    fn is_iff(exp: Exprent) -> bool {
        exp.try_as_function()
            .is_some_and(|f| f.get_func_type() == FunctionType::Ternary)
    }

    fn collapse_inlined_class14(stat: Statement) -> bool {
        unported!("SimplifyExprentsHelper.collapseInlinedClass14")
    }
}
