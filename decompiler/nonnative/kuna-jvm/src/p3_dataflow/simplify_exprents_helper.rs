//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SimplifyExprentsHelper.java`.

use std::collections::VecDeque;

use crate::array_exprent::ArrayExprent;
use crate::assignment_exprent::AssignmentExprent;
use crate::classes_processor::ClassNodeType;
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_helper::IfHelper;
use crate::if_statement::IfStatement;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::{InvocationType, Type as InvocationFuncType};
use crate::jthrow;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchValue;
use crate::monitor_exprent::Type as MonitorType;
use crate::new_exprent::NewExprent;
use crate::pair::Pair;
use crate::primitive_constant::ConstValue as PoolConstValue;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::sequence_helper::SequenceHelper;
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct SimplifyExprentsHelper;

thread_local! {
    static CLASS14_ENGINE: MatchEngine = SimplifyExprentsHelper::class14_builder();
}

/// `x instanceof AssignmentExprent a && a.getLeft() instanceof ArrayExprent e &&
/// e.getArray() instanceof VarExprent v && arr_var.equals(v) && e.getIndex() instanceof ConstExprent c`.
fn array_store(expr: Exprent, arr_var: VarExprent) -> Option<(AssignmentExprent, ConstExprent)> {
    let aas = expr.try_as_assignment()?;
    let arr_expr: ArrayExprent = aas.get_left().try_as_array()?;
    let arr_var2 = arr_expr.get_array().try_as_var()?;
    if !arr_var.equals(arr_var2.0) {
        return None;
    }
    let index_expr = arr_expr.get_index().try_as_const()?;
    Some((aas, index_expr))
}

/// `(String) value` of a match variable bound by a string constant.
#[track_caller]
fn match_string(value: Option<MatchValue>) -> Option<String> {
    match value {
        None => None,
        Some(MatchValue::String(s)) => Some(s),
        Some(MatchValue::Constant(PoolConstValue::String(s))) => Some(s.to_string()),
        Some(_) => crate::rt::throw("ClassCastException", "match value is not a String"),
    }
}

/// `(T) value` of a match variable bound by an exprent.
#[track_caller]
fn match_exprent(value: Option<MatchValue>) -> Option<Exprent> {
    match value {
        None => None,
        Some(MatchValue::Exprent(e)) => Some(e),
        Some(_) => crate::rt::throw("ClassCastException", "match value is not an Exprent"),
    }
}

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
        MatchEngine::new(&Self::CLASS14_BUILDER)
    }

    pub fn simplify_stack_vars_statement(
        stat: Statement,
        set_reordered_ifs: &mut JHashSet<i32>,
        ssa: Option<&SSAConstructorSparseEx>,
        cl: StructClass,
        first_invocation: bool,
    ) -> bool {
        let mut res = false;

        let expressions = stat.get_exprents();
        match expressions {
            None => {
                let process_class14 =
                    DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_CLASS_1_4);

                loop {
                    let mut changed = false;

                    let stats = stat.get_stats().borrow().to_vec();
                    for st in stats {
                        res |= Self::simplify_stack_vars_statement(
                            st,
                            set_reordered_ifs,
                            ssa,
                            cl,
                            first_invocation,
                        );

                        changed = IfHelper::merge_ifs(st, set_reordered_ifs)
                            || Self::build_iff(st, ssa)
                            || process_class14 && Self::collapse_inlined_class14(st);

                        if changed {
                            break;
                        }

                        if !st.get_stats().borrow().is_empty() {
                            let first_child = st.get_stats().borrow().get(0);
                            if Self::has_qualified_new_get_class(st, first_child) {
                                break;
                            }
                        }
                    }

                    res |= changed;

                    if !changed {
                        break;
                    }
                }
            }
            Some(expressions) => {
                res = Self::simplify_stack_vars_exprents(
                    &expressions,
                    cl,
                    ssa,
                    stat,
                    first_invocation,
                );
            }
        }

        res
    }

    fn simplify_stack_vars_exprents(
        list: &JList<Exprent>,
        cl: StructClass,
        ssa: Option<&SSAConstructorSparseEx>,
        stat: Statement,
        first_invocation: bool,
    ) -> bool {
        let mut res = false;

        let mut index: usize = 0;
        while index < list.len() {
            let current = list.get(index);

            let ret = Self::is_simple_constructor_invocation(current);
            if let Some(ret) = ret {
                list.set(index, ret);
                res = true;
                continue;
            }

            let ret = Self::is_lambda(current, cl);
            if let Some(ret) = ret {
                list.set(index, ret);
                res = true;
                continue;
            }

            if Self::is_monitor_exit(current) {
                list.remove_at(index);
                res = true;
                continue;
            }

            if Self::is_trivial_stack_assignment(current) {
                list.remove_at(index);
                res = true;
                continue;
            }

            if index == list.len() - 1 {
                break;
            }

            let next = list.get(index + 1);

            if index > 0 {
                let prev = list.get(index - 1);

                if Self::is_swap_constructor_invocation(prev, current, next) {
                    list.remove_at(index - 1);
                    list.remove_at(index);
                    res = true;
                    continue;
                }
            }

            if Self::is_assignment_return(current, next, stat) {
                list.remove_at(index);
                res = true;
                continue;
            }

            if Self::is_constructor_invocation_remote(list, index as i32) {
                list.remove_at(index);
                res = true;
                continue;
            }

            if DecompilerContext::get_option(IFernflowerPreferences::REMOVE_GET_CLASS_NEW)
                && Self::is_qualified_new_get_class(current, next)
            {
                list.remove_at(index);
                res = true;
                continue;
            }

            let arr_count = Self::is_array_initializer(list, index as i32);
            if arr_count > 0 {
                for _ in 0..arr_count {
                    list.remove_at(index + 1);
                }
                res = true;
                continue;
            }

            if Self::add_array_initializer(current, next) {
                list.remove_at(index + 1);
                res = true;
                continue;
            }

            let func = Self::is_ppi_or_mmi(current);
            if let Some(func) = func {
                list.set(index, func);
                res = true;
                continue;
            }

            if Self::is_ipp_or_imm(current, next) || Self::is_ipp_or_imm2(current, next) {
                list.remove_at(index + 1);
                res = true;
                continue;
            }

            if Self::is_stack_assignment(current, next) {
                list.remove_at(index + 1);
                res = true;
                continue;
            }

            if Self::is_inlined_method_reference_assign(ssa, current, next) {
                list.remove_at(index);
                res = true;
                continue;
            }

            if !first_invocation && Self::is_stack_assignment2(current, next) {
                list.remove_at(index + 1);
                res = true;
                continue;
            }

            if first_invocation && Self::inline_ppi_and_mmi(current, next) {
                list.remove_at(index);
                res = true;
                continue;
            }

            index += 1;
        }

        res
    }

    fn add_array_initializer(first: Exprent, second: Exprent) -> bool {
        let Some(as_) = first.try_as_assignment() else {
            return false;
        };
        let Some(new_expr) = as_.get_right().try_as_new() else {
            return false;
        };
        if new_expr.get_lst_array_elements().is_empty() {
            return false;
        }
        let Some(arr_var1) = as_.get_left().try_as_var() else {
            return false;
        };
        let Some((aas, const_expr)) = array_store(second, arr_var1) else {
            return false;
        };

        let const_value = const_expr.get_int_value();

        if const_value < new_expr.get_lst_array_elements().len() as i32 {
            let init = new_expr.get_lst_array_elements().get(const_value as usize);
            if let Some(const_init) = init.try_as_const() {
                let arr_type = new_expr.get_new_type().decrease_array_dim();
                let default_val = ExprProcessor::get_default_array_value(&arr_type);

                if const_init.equals(default_val.0) {
                    let temp_expr = aas.get_right();

                    if (temp_expr.get_exprent_use() & Exprent::SIDE_EFFECTS_FREE) == 0 {
                        let mut i = const_value + 1;
                        while i < new_expr.get_lst_array_elements().len() as i32 {
                            let exprent = new_expr.get_lst_array_elements().get(i as usize);
                            if (exprent.get_exprent_use() & Exprent::SIDE_EFFECTS_FREE) == 0 {
                                return false;
                            }
                            i += 1;
                        }
                    }

                    if !temp_expr.contains_exprent(arr_var1.0) {
                        new_expr
                            .get_lst_array_elements()
                            .set(const_value as usize, temp_expr);

                        if let Some(temp_new_expr) = temp_expr.try_as_new() {
                            let dims = new_expr.get_new_type().array_dim;
                            if dims > 1 && !temp_new_expr.get_lst_array_elements().is_empty() {
                                temp_new_expr.set_direct_array_init(true);
                            }
                        }

                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_array_initializer(list: &JList<Exprent>, index: i32) -> i32 {
        let current = list.get(index as usize);
        let Some(as_) = current.try_as_assignment() else {
            return 0;
        };
        let Some(arr_var) = as_.get_left().try_as_var() else {
            return 0;
        };
        let Some(new_expr) = as_.get_right().try_as_new() else {
            return 0;
        };
        if !(new_expr.get_expr_type().array_dim > 0
            && new_expr.get_lst_dims().len() == 1
            && new_expr.get_lst_array_elements().is_empty())
        {
            return 0;
        }
        let Some(const_expr) = new_expr.get_lst_dims().get(0).try_as_const() else {
            return 0;
        };

        let size = const_expr.get_int_value();
        if size == 0 {
            return 0;
        }

        let mut map_init: JHashMap<i32, Exprent> = JHashMap::new();

        let mut i: i32 = 1;
        let mut last_impure: i32 = -1;
        while index + i < list.len() as i32 && i <= size {
            let expr = list.get((index + i) as usize);
            if let Some((aas, index_expr)) = array_store(expr, arr_var) {
                let const_value = index_expr.get_int_value();
                if const_value < size && !map_init.contains_key(&const_value) {
                    if (aas.get_right().get_exprent_use() & Exprent::SIDE_EFFECTS_FREE) == 0 {
                        if const_value < last_impure {
                            break;
                        }
                        last_impure = const_value;
                    }
                    if !aas.get_right().contains_exprent(arr_var.0) {
                        map_init.put(const_value, aas.get_right());
                        i += 1;
                        continue;
                    }
                }
            }
            break;
        }

        let fraction = map_init.len() as f64 / size as f64;

        if (arr_var.is_stack() && fraction > 0.0)
            || (size <= 7 && fraction >= 0.3)
            || (size > 7 && fraction >= 0.7)
        {
            let mut lst_ret: Vec<Exprent> = Vec::new();

            let array_type = new_expr.get_new_type().decrease_array_dim();
            let default_val = ExprProcessor::get_default_array_value(&array_type);
            for _ in 0..size {
                lst_ret.push(default_val.copy());
            }

            let dims = new_expr.get_new_type().array_dim;
            for (key, temp_expr) in map_init.entry_vec() {
                lst_ret[key as usize] = temp_expr;

                if let Some(temp_new_expr) = temp_expr.try_as_new() {
                    if dims > 1 && !temp_new_expr.get_lst_array_elements().is_empty() {
                        temp_new_expr.set_direct_array_init(true);
                    }
                }
            }

            new_expr.set_lst_array_elements(lst_ret);

            return map_init.len() as i32;
        }

        0
    }

    fn is_assignment_return(first: Exprent, second: Exprent, stat: Statement) -> bool {
        let Some(assignment) = first.try_as_assignment() else {
            return false;
        };
        if assignment.get_cond_type().is_some() {
            return false;
        }
        let Some(assignment_left) = assignment.get_left().try_as_var() else {
            return false;
        };
        if assignment_left.is_stack() {
            return false;
        }
        let Some(exit) = second.try_as_exit() else {
            return false;
        };
        if exit.get_exit_type() != ExitType::Return {
            return false;
        }
        let Some(exit_value) = exit.get_value().and_then(|v| v.try_as_var()) else {
            return false;
        };
        if !assignment_left.equals(exit_value.0) {
            return false;
        }

        if stat
            .get_top_parent()
            .mt()
            .get_bytecode_version()
            .has_if_pattern_matching()
        {
            if let Some(ifst) = stat.get_parent().and_then(|p| p.try_as_if()) {
                if !ifst.is_pattern_matched()
                    && stat
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .index_of(&first)
                        == 0
                    && assignment
                        .get_right()
                        .try_as_function()
                        .is_some_and(|func| func.get_func_type() == FunctionType::Cast)
                    && ifst
                        .get_headexprent()
                        .get_all_exprents_rec_self(true, false)
                        .iter()
                        .any(|e| {
                            e.try_as_function()
                                .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof)
                        })
                {
                    return false;
                }
            }
        }

        exit.replace_exprent(exit_value.0, assignment.get_right());
        true
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
        if let Some(asf) = first.try_as_assignment() {
            if Self::is_stack_var(asf.get_left()) {
                if let Some(ass) = second.try_as_assignment() {
                    if !Self::is_stack_var(ass.get_left()) && asf.get_left().equals(ass.get_right())
                    {
                        asf.set_right(
                            AssignmentExprent::new(
                                ass.get_left(),
                                asf.get_right(),
                                ass.get_bytecode().as_ref(),
                            )
                            .0,
                        );
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_stack_var(exprent: Exprent) -> bool {
        exprent.try_as_var().is_some_and(|v| v.is_stack())
    }

    fn is_stack_assignment(first: Exprent, second: Exprent) -> bool {
        if let (Some(mut asf), Some(ass)) = (first.try_as_assignment(), second.try_as_assignment())
        {
            loop {
                if asf.get_right().equals(ass.get_right())
                    && Self::is_stack_var(asf.get_left())
                    && !Self::is_stack_var(ass.get_left())
                    && !ass.get_left().contains_exprent(asf.get_left())
                {
                    asf.set_right(ass.0);
                    return true;
                }
                match asf.get_right().try_as_assignment() {
                    Some(inner_assign) => asf = inner_assign,
                    None => break,
                }
            }
        }

        false
    }

    fn is_inlined_method_reference_assign(
        ssa: Option<&SSAConstructorSparseEx>,
        first: Exprent,
        second: Exprent,
    ) -> bool {
        let Some(ssa) = ssa else {
            return false;
        };

        let Some(assign) = first.try_as_assignment() else {
            return false;
        };
        let Some(var) = assign.get_left().try_as_var() else {
            return false;
        };
        if var.get_var_type().type_ != CodeType::Object || var.get_lvt().is_some() {
            return false;
        }
        if ssa.get_read_count(var.get_var_version_pair()) == 1 {
            for expr in second.get_all_exprents_rec(true) {
                let Some(lambda) = expr.try_as_new() else {
                    continue;
                };
                if lambda.is_lambda()
                    && !lambda.is_method_reference()
                    && lambda
                        .get_constructor()
                        .expect("NullPointerException: constructor")
                        .contains_exprent(var.0)
                    && lambda
                        .get_constructor()
                        .expect("NullPointerException: constructor")
                        .get_lst_parameters()
                        .len()
                        == 1
                {
                    let wrapper = DecompilerContext::get_current_class_wrapper()
                        .expect("NullPointerException: CURRENT_CLASS_WRAPPER");
                    let key = lambda.get_lambda_content_method_key();
                    let is_synthetic_reference_lambda = wrapper
                        .get_method_properties_map()
                        .borrow()
                        .get(&key)
                        .map(|p| p.is_synthetic_reference_lambda)
                        .expect("NullPointerException: method properties");
                    if is_synthetic_reference_lambda {
                        let constructor = lambda
                            .get_constructor()
                            .expect("NullPointerException: constructor");
                        constructor.replace_exprent(
                            constructor.get_lst_parameters().get(0),
                            assign.get_right(),
                        );
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Java: isPPIorMMI(Exprent)
    fn is_ppi_or_mmi(first: Exprent) -> Option<Exprent> {
        let as_ = first.try_as_assignment()?;
        let func = as_.get_right().try_as_function()?;

        if func.get_func_type() == FunctionType::Add || func.get_func_type() == FunctionType::Sub {
            let mut econd = func.get_lst_operands().get(0);
            let mut econst = func.get_lst_operands().get(1);

            if !econst.is_const() && econd.is_const() && func.get_func_type() == FunctionType::Add {
                econd = econst;
                econst = func.get_lst_operands().get(0);
            }

            if econst.is_const() && econst.as_const().has_value_one() {
                let left = as_.get_left();

                if !left.is_var() && left.equals(econd) {
                    let type_ = if func.get_func_type() == FunctionType::Add {
                        FunctionType::Ppi
                    } else {
                        FunctionType::Mmi
                    };
                    let ret =
                        FunctionExprent::new_single(type_, econd, func.get_bytecode().as_ref());
                    ret.set_implicit_type(Some(econd.get_expr_type()));
                    return Some(ret.0);
                }
            }
        }

        None
    }

    /// Java: isIPPorIMM(Exprent, Exprent)
    fn is_ipp_or_imm(first: Exprent, second: Exprent) -> bool {
        if let (Some(as_), Some(in_)) = (first.try_as_assignment(), second.try_as_function()) {
            if (in_.get_func_type() == FunctionType::Mmi
                || in_.get_func_type() == FunctionType::Ppi)
                && in_.get_lst_operands().get(0).equals(as_.get_right())
            {
                if in_.get_func_type() == FunctionType::Mmi {
                    in_.set_func_type(FunctionType::Imm);
                } else {
                    in_.set_func_type(FunctionType::Ipp);
                }
                as_.set_right(in_.0);

                return true;
            }
        }

        false
    }

    /// Java: isIPPorIMM2(Exprent, Exprent)
    fn is_ipp_or_imm2(first: Exprent, second: Exprent) -> bool {
        let Some(af) = first.try_as_assignment() else {
            return false;
        };
        let Some(as_) = second.try_as_assignment() else {
            return false;
        };
        let Some(func) = as_.get_right().try_as_function() else {
            return false;
        };

        if func.get_func_type() != FunctionType::Add && func.get_func_type() != FunctionType::Sub {
            return false;
        }

        let mut econd = func.get_lst_operands().get(0);
        let mut econst = func.get_lst_operands().get(1);

        if !econst.is_const() && econd.is_const() && func.get_func_type() == FunctionType::Add {
            econd = econst;
            econst = func.get_lst_operands().get(0);
        }

        if let Some(const_expr) = econst.try_as_const() {
            if const_expr.has_value_one()
                && af.get_left().equals(econd)
                && af.get_right().equals(as_.get_left())
                && (af.get_left().get_exprent_use() & Exprent::MULTIPLE_USES) != 0
            {
                let type_ = if func.get_func_type() == FunctionType::Add {
                    FunctionType::Ipp
                } else {
                    FunctionType::Imm
                };

                let ret = FunctionExprent::new_single(
                    type_,
                    af.get_right(),
                    func.get_bytecode().as_ref(),
                );
                ret.set_implicit_type(Some(VarType::VARTYPE_INT()));

                af.set_right(ret.0);
                return true;
            }
        }
        false
    }

    fn inline_ppi_and_mmi(expr: Exprent, next: Exprent) -> bool {
        let Some(func) = expr.try_as_function() else {
            return false;
        };
        if !(func.get_func_type() == FunctionType::Ppi || func.get_func_type() == FunctionType::Mmi)
        {
            return false;
        }
        let Some(var) = func.get_lst_operands().get(0).try_as_var() else {
            return false;
        };

        if next.try_as_function().is_some_and(Self::is_ppmm) {
            return false;
        }

        let usage = Self::find_first_valid_usage(var, next);

        if let Some(usage) = usage {
            usage.a.replace_exprent(usage.b.0, func.0);
            return true;
        }

        false
    }

    fn find_first_valid_usage(
        match_: VarExprent,
        next: Exprent,
    ) -> Option<Pair<Exprent, VarExprent>> {
        let mut stack: Vec<Exprent> = Vec::new();
        let mut parent: Vec<Option<Exprent>> = Vec::new();
        stack.push(next);
        parent.push(None);

        while let Some(expr) = stack.pop() {
            let parent_expr = parent
                .pop()
                .expect("parent stack in step with exprent stack");

            let exprents = expr.get_all_exprents();

            if let Some(parent_expr) = parent_expr {
                if let Some(ve) = expr.try_as_var() {
                    if ve.get_index() == match_.get_index()
                        && ve.get_version() == match_.get_version()
                    {
                        return Some(Pair::of(parent_expr, ve));
                    }
                }
            }

            if let Some(func) = expr.try_as_function() {
                if Self::is_ppmm(func) {
                    return None;
                }

                if func.get_func_type() == FunctionType::BooleanOr
                    || func.get_func_type() == FunctionType::BooleanAnd
                {
                    return None;
                }

                if func.get_func_type() == FunctionType::Ternary {
                    return None;
                }

                if func.get_func_type() == FunctionType::Sub
                    || func.get_func_type() == FunctionType::Div
                {
                    return None;
                }
            }

            for &ex in exprents.iter().rev() {
                if let Some(as_expr) = expr.try_as_assignment() {
                    if ex == as_expr.get_left()
                        && ex
                            .try_as_var()
                            .is_some_and(|inner_ex| inner_ex.get_index() == match_.get_index())
                    {
                        continue;
                    }
                }

                stack.push(ex);
                parent.push(Some(expr));
            }
        }

        None
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
        if child.is_basic_block() && child.get_exprents().is_some_and(|e| !e.is_empty()) {
            let exprents = child.get_exprents().expect("checked above");
            let first_expr = exprents.get(exprents.len() - 1);

            if let Some(if_stat) = parent.try_as_if() {
                if Self::is_qualified_new_get_class(
                    first_expr,
                    if_stat.get_headexprent().get_condition(),
                ) {
                    child
                        .get_exprents()
                        .expect("checked above")
                        .remove_item(&first_expr);
                    return true;
                }
            }
        }
        false
    }

    fn is_qualified_new_get_class(first: Exprent, second: Exprent) -> bool {
        let Some(invocation) = first.try_as_invocation() else {
            return false;
        };

        if (!invocation.is_static()
            && invocation.get_name() == "getClass"
            && invocation.get_string_descriptor() == "()Ljava/lang/Class;")
            || (invocation.is_static()
                && invocation.get_classname() == "java/util/Objects"
                && invocation.get_name() == "requireNonNull"
                && invocation.get_string_descriptor() == "(Ljava/lang/Object;)Ljava/lang/Object;")
        {
            if invocation.is_synthetic_null_check() {
                return true;
            }

            let mut lst_exprents: VecDeque<Exprent> = VecDeque::new();
            lst_exprents.push_back(second);

            let target: Option<Exprent> = if invocation.is_static() {
                Some(invocation.get_lst_parameters().get(0))
            } else {
                invocation.get_instance()
            };

            while let Some(expr) = lst_exprents.pop_front() {
                lst_exprents.extend(expr.get_all_exprents());
                let Some(new_expr) = expr.try_as_new() else {
                    continue;
                };
                let Some(constructor) = new_expr.get_constructor() else {
                    continue;
                };
                let parameters = constructor.get_lst_parameters();
                if !parameters.is_empty()
                    && (target.is_some_and(|t| parameters.get(0).equals(t))
                        || Self::is_unambiguously_same_param(
                            invocation.is_static(),
                            target.expect("NullPointerException: target"),
                            &parameters,
                        ))
                {
                    let classname = new_expr.get_new_type().value.clone();
                    let node = classname.and_then(|classname| {
                        DecompilerContext::get_class_processor()
                            .get_map_root_classes()
                            .borrow()
                            .get(&classname.to_string())
                            .copied()
                    });
                    if let Some(node) = node {
                        if node.with(|d| d.type_) != ClassNodeType::Root {
                            return true;
                        }
                    }
                }
            }
        }

        false
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

    #[allow(dead_code)]
    fn is_method_array_assign(expr: Exprent, next: Exprent) -> bool {
        let Some(asf) = expr.try_as_assignment() else {
            return false;
        };
        let Some(ass) = next.try_as_assignment() else {
            return false;
        };
        let Some(first_left) = asf.get_left().try_as_var() else {
            return false;
        };
        let Some(second_left) = ass.get_left().try_as_array() else {
            return false;
        };
        let Some(second_base) = second_left.get_array().try_as_var() else {
            return false;
        };
        if !(first_left.get_index() == second_base.get_index() && second_base.is_stack()) {
            return false;
        }

        let mut found_assign = false;
        let second_right = ass.get_right();
        for exprent in second_right.get_all_exprents() {
            if let Some(arr_expr) = exprent.try_as_array() {
                if arr_expr
                    .get_array()
                    .try_as_var()
                    .is_some_and(|arr_var| arr_var.get_index() == first_left.get_index())
                {
                    exprent.replace_exprent(arr_expr.get_array(), asf.get_right().copy());
                    found_assign = true;
                }
            }
        }

        if found_assign {
            second_left.replace_exprent(second_base.0, asf.get_right());
            return true;
        }

        false
    }

    fn is_constructor_invocation_remote(list: &JList<Exprent>, index: i32) -> bool {
        let Some(as_) = list.get(index as usize).try_as_assignment() else {
            return false;
        };
        let Some(var_expr) = as_.get_left().try_as_var() else {
            return false;
        };
        let Some(new_expr) = as_.get_right().try_as_new() else {
            return false;
        };

        let new_type = new_expr.get_new_type();
        let left_pair = VarVersionPair::from_var(var_expr);

        if new_type.type_ == CodeType::Object
            && new_type.array_dim == 0
            && new_expr.get_constructor().is_none()
        {
            let mut i = index as usize + 1;
            while i < list.len() {
                let remote = list.get(i);

                if let Some(in_) = remote.try_as_invocation() {
                    if in_.get_functype() == InvocationFuncType::Init {
                        if let Some(instance) = in_.get_instance().and_then(|e| e.try_as_var()) {
                            if var_expr.equals(instance.0) {
                                new_expr.set_constructor(Some(in_));
                                in_.set_instance(None);

                                list.set(i, as_.copy());

                                return true;
                            }
                        }
                    }
                }

                let set_vars = remote.get_all_variables();
                if set_vars.contains(&left_pair) {
                    return false;
                }
                i += 1;
            }
        }

        false
    }

    fn is_swap_constructor_invocation(last: Exprent, expr: Exprent, next: Exprent) -> bool {
        let Some(as_last) = last.try_as_assignment() else {
            return false;
        };
        let Some(var_last) = as_last.get_left().try_as_var() else {
            return false;
        };
        let Some(new_expr) = as_last.get_right().try_as_new() else {
            return false;
        };
        let Some(as_expr) = expr.try_as_assignment() else {
            return false;
        };
        let Some(var_expr) = as_expr.get_right().try_as_var() else {
            return false;
        };
        if var_last.get_index() != var_expr.get_index() {
            return false;
        }
        let Some(in_next) = next.try_as_invocation() else {
            return false;
        };
        if in_next.get_functype() != InvocationFuncType::Init {
            return false;
        }
        let Some(var_next) = in_next.get_instance().and_then(|e| e.try_as_var()) else {
            return false;
        };
        if var_expr.get_index() != var_next.get_index() {
            return false;
        }

        in_next.set_instance(None);
        new_expr.set_constructor(Some(in_next));

        as_expr.set_right(new_expr.0);

        true
    }

    fn is_lambda(exprent: Exprent, cl: StructClass) -> Option<Exprent> {
        let lst = exprent.get_all_exprents();
        for expr in lst {
            if let Some(ret) = Self::is_lambda(expr, cl) {
                exprent.replace_exprent(expr, ret);
            }
        }

        if let Some(in_) = exprent.try_as_invocation() {
            if in_.get_invocation_type() == InvocationType::Dynamic {
                let suffix = in_.get_invoke_dynamic_class_suffix();
                let lambda_class_name = format!(
                    "{}{}",
                    cl.qualified_name(),
                    suffix.as_deref().unwrap_or("null")
                );
                let lambda_class = DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get(&lambda_class_name)
                    .copied();

                if lambda_class.is_some() {
                    let new_expr = NewExprent::new(
                        VarType::new_signature_cl(&lambda_class_name, true),
                        Vec::new(),
                        in_.get_bytecode().as_ref(),
                    );
                    new_expr.set_constructor(Some(in_));

                    return Some(new_expr.0);
                }
            }
        }

        None
    }

    fn is_simple_constructor_invocation(exprent: Exprent) -> Option<Exprent> {
        let lst = exprent.get_all_exprents();
        for expr in lst {
            if let Some(ret) = Self::is_simple_constructor_invocation(expr) {
                exprent.replace_exprent(expr, ret);
            }
        }

        if let Some(in_) = exprent.try_as_invocation() {
            if in_.get_functype() == InvocationFuncType::Init {
                if let Some(new_expr) = in_.get_instance().and_then(|e| e.try_as_new()) {
                    new_expr.set_constructor(Some(in_));
                    in_.set_instance(None);
                    return Some(new_expr.0);
                }
            }
        }

        None
    }

    fn build_iff(stat: Statement, ssa: Option<&SSAConstructorSparseEx>) -> bool {
        let Some(statement) = stat.try_as_if() else {
            return false;
        };
        if stat.get_exprents().is_some() {
            return false;
        }

        let if_head_expr = statement.get_headexprent_list().first();
        let if_head_expr_bytecode = if_head_expr.and_then(|e| e.get_bytecode());
        let bytecode = if_head_expr_bytecode.as_ref();

        if statement.iftype() != IfStatement::IFTYPE_IFELSE {
            return false;
        }

        let if_statement = statement
            .get_ifstat()
            .expect("NullPointerException: ifstat");
        let else_statement = statement
            .get_elsestat()
            .expect("NullPointerException: elsestat");

        if !(if_statement.get_exprents().is_some_and(|e| e.len() == 1)
            && else_statement.get_exprents().is_some_and(|e| e.len() == 1)
            && if_statement.get_all_successor_edges().len() == 1
            && else_statement.get_all_successor_edges().len() == 1
            && if_statement.get_all_successor_edges()[0].get_destination()
                == else_statement.get_all_successor_edges()[0].get_destination())
        {
            return false;
        }

        let if_expr = if_statement.get_exprents().expect("checked above").get(0);
        let else_expr = else_statement.get_exprents().expect("checked above").get(0);

        if let (Some(if_assign), Some(else_assign)) =
            (if_expr.try_as_assignment(), else_expr.try_as_assignment())
        {
            let Some(if_var) = if_assign.get_left().try_as_var() else {
                return false;
            };
            if !if_var.is_stack() {
                return false;
            }
            let Some(else_var) = else_assign.get_left().try_as_var() else {
                return false;
            };
            if if_var.get_index() != else_var.get_index() {
                return false;
            }

            let mut found = false;

            let Some(ssa) = ssa else {
                jthrow!(IllegalStateException; "Trying to make ternary but have no SSA-Form! How is this possible?");
            };

            for (key, value) in ssa.get_phi().iter() {
                if key.var == if_var.get_index() {
                    let value = value.borrow();
                    if value.contains(&if_var.get_version())
                        && value.contains(&else_var.get_version())
                    {
                        found = true;
                        break;
                    }
                }
            }

            if found {
                let mut data: Vec<Exprent> = statement
                    .get_first()
                    .expect("NullPointerException: first")
                    .get_exprents()
                    .expect("NullPointerException: first exprents")
                    .to_vec();

                let operands = vec![
                    statement.get_headexprent().get_condition(),
                    if_assign.get_right(),
                    else_assign.get_right(),
                ];
                data.push(
                    AssignmentExprent::new(
                        if_var.0,
                        FunctionExprent::new(FunctionType::Ternary, operands, bytecode).0,
                        bytecode,
                    )
                    .0,
                );
                statement.set_exprents(JList::from(data));

                if statement.get_all_successor_edges().is_empty() {
                    let if_edge = if_statement.get_all_successor_edges()[0];
                    let edge =
                        StatEdge::new(if_edge.get_type(), statement.0, if_edge.get_destination());

                    statement.add_successor(edge);
                    if let Some(closure) = if_edge.closure() {
                        closure.add_labeled_edge(edge);
                    }
                }

                SequenceHelper::destroy_and_flatten_statement(statement.0);

                return true;
            }
        } else if let (Some(if_exit), Some(else_exit)) =
            (if_expr.try_as_exit(), else_expr.try_as_exit())
        {
            if if_exit.get_exit_type() == else_exit.get_exit_type()
                && if_exit.get_value().is_some()
                && else_exit.get_value().is_some()
                && if_exit.get_exit_type() == ExitType::Return
            {
                let if_value = if_exit.get_value().expect("checked above");
                let else_value = else_exit.get_value().expect("checked above");

                if if_exit.get_exit_type() == ExitType::Throw
                    && !if_value.get_expr_type().equals(&else_value.get_expr_type())
                {
                    return false;
                }

                if Self::is_iff(if_value) || Self::is_iff(else_value) {
                    return false;
                }

                let mut data: Vec<Exprent> = statement
                    .get_first()
                    .expect("NullPointerException: first")
                    .get_exprents()
                    .expect("NullPointerException: first exprents")
                    .to_vec();

                data.push(
                    ExitExprent::new(
                        if_exit.get_exit_type(),
                        Some(
                            FunctionExprent::new(
                                FunctionType::Ternary,
                                vec![
                                    statement.get_headexprent().get_condition(),
                                    if_value,
                                    else_value,
                                ],
                                bytecode,
                            )
                            .0,
                        ),
                        if_exit.get_ret_type(),
                        bytecode,
                        if_exit.get_method_descriptor(),
                    )
                    .0,
                );
                statement.set_exprents(JList::from(data));

                let ret_edge = if_statement.get_all_successor_edges()[0];
                let closure = if ret_edge.closure() == Some(statement.0) {
                    statement.get_parent()
                } else {
                    ret_edge.closure()
                };
                statement.add_successor(StatEdge::new_closure(
                    StatEdge::TYPE_BREAK,
                    statement.0,
                    ret_edge.get_destination(),
                    closure,
                ));

                SequenceHelper::destroy_and_flatten_statement(statement.0);

                return true;
            }
        }

        false
    }

    fn is_iff(exp: Exprent) -> bool {
        exp.try_as_function()
            .is_some_and(|f| f.get_func_type() == FunctionType::Ternary)
    }

    fn collapse_inlined_class14(stat: Statement) -> bool {
        let matched = CLASS14_ENGINE.with(|engine| {
            if engine.match_(&stat) {
                Some((
                    match_string(engine.get_variable_value("$classname$")),
                    match_exprent(engine.get_variable_value("$assignfield$")),
                    match_exprent(engine.get_variable_value("$field$")),
                ))
            } else {
                None
            }
        });
        let ret = matched.is_some();
        if let Some((class_name, assignment, field_expr)) = matched {
            let assignment: AssignmentExprent = assignment
                .expect("NullPointerException: $assignfield$")
                .as_assignment();
            let field_expr: Option<FieldExprent> = field_expr.map(|e| e.as_field());

            assignment.replace_exprent(
                assignment.get_right(),
                ConstExprent::new(
                    VarType::VARTYPE_CLASS(),
                    class_name.map(ConstValue::String),
                    None,
                )
                .0,
            );

            let data: Vec<Exprent> = stat
                .get_first()
                .expect("NullPointerException: first")
                .get_exprents()
                .expect("NullPointerException: first exprents")
                .to_vec();

            stat.set_exprents(JList::from(data));

            SequenceHelper::destroy_and_flatten_statement(stat);

            if let Some(wrapper) = DecompilerContext::get_current_class_wrapper() {
                let field_expr = field_expr.expect("NullPointerException: $field$");
                let key = InterpreterUtil::make_unique_key(
                    &field_expr.get_name(),
                    &field_expr.get_descriptor().descriptor_string,
                );
                wrapper.get_hidden_members().borrow_mut().add(key);
            }
        }

        ret
    }
}
