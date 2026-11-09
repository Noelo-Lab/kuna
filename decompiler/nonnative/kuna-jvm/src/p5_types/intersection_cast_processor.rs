//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IntersectionCastProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::assignment_exprent::AssignmentExprent;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::invocation_exprent::InvocationExprent;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{JavaEq, JavaHash};
use crate::statement::Statement;
use crate::type_family::TypeFamily;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

pub struct IntersectionCastProcessor;

#[derive(Clone, Copy)]
struct VariablePosition {
    position: VariablePositionEnum,
    exp: Exprent,
    index: i32,
}

impl VariablePosition {
    fn new(position: VariablePositionEnum, exp: Exprent, index: i32) -> VariablePosition {
        VariablePosition {
            position,
            exp,
            index,
        }
    }

    pub fn position(&self) -> VariablePositionEnum {
        self.position
    }

    pub fn exp(&self) -> Exprent {
        self.exp
    }

    pub fn index(&self) -> i32 {
        self.index
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum VariablePositionEnum {
    MethodParameter,
    Casted,
}

impl JavaHash for VariablePositionEnum {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for VariablePositionEnum {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl IntersectionCastProcessor {
    /// Java: makeIntersectionCasts(RootStatement)
    pub fn make_intersection_casts(root: RootStatement) -> bool {
        Self::make_intersection_casts_rec(root.0, root)
    }

    fn make_intersection_casts_rec(stat: Statement, root: RootStatement) -> bool {
        let mut result = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            result |= Self::make_intersection_casts_rec(st, root);
        }
        match stat.get_exprents() {
            Some(exprents) => {
                for e in exprents.iter() {
                    result |= Self::make_intersection_casts_exprent(e, root);
                }
            }
            None => {
                for e in stat.get_stat_exprents() {
                    result |= Self::make_intersection_casts_exprent(e, root);
                }
            }
        }
        result
    }

    /// Java: makeIntersectionCasts(Exprent, RootStatement)
    fn make_intersection_casts_exprent(exp: Exprent, root: RootStatement) -> bool {
        if let Some(inv) = exp.try_as_invocation() {
            if Self::handle_invocation(inv, root) {
                return true;
            }
        } else if let Some(assignment) = exp.try_as_assignment() {
            if Self::handle_assignment(assignment, root) {
                return true;
            }
        }
        let mut result = false;
        for sub in exp.get_all_exprents() {
            result |= Self::make_intersection_casts_exprent(sub, root);
        }
        result
    }

    fn handle_invocation(exp: InvocationExprent, root: RootStatement) -> bool {
        unported!("IntersectionCastProcessor.handleInvocation")
    }

    fn handle_assignment(exp: AssignmentExprent, root: RootStatement) -> bool {
        unported!("IntersectionCastProcessor.handleAssignment")
    }

    fn get_bounds(exp: InvocationExprent, parameter: i32) -> Vec<VarType> {
        unported!("IntersectionCastProcessor.getBounds")
    }

    /// Java: findReferences(VarExprent, RootStatement)
    fn find_references(var_exp: VarExprent, root: RootStatement) -> Vec<VariablePosition> {
        let mut list = Vec::new();
        Self::find_references_rec(var_exp, root.0, root, &mut list);
        list
    }

    fn find_references_rec(
        var_exp: VarExprent,
        stat: Statement,
        root: RootStatement,
        list: &mut Vec<VariablePosition>,
    ) {
        let stats = stat.get_stats().borrow().to_vec();
        for s in stats {
            Self::find_references_rec(var_exp, s, root, list);
        }
        match stat.get_exprents() {
            Some(exprents) => {
                for e in exprents.iter() {
                    Self::find_references_exprent(var_exp, e, root, list);
                }
            }
            None => {
                for e in stat.get_stat_exprents() {
                    Self::find_references_exprent(var_exp, e, root, list);
                }
            }
        }
    }

    /// Java: findReferences(VarExprent, Exprent, RootStatement, List)
    fn find_references_exprent(
        var_exp: VarExprent,
        exp: Exprent,
        root: RootStatement,
        list: &mut Vec<VariablePosition>,
    ) {
        if let Some(inv) = exp.try_as_invocation() {
            Self::find_references_invocation(var_exp, inv, list);
        } else if let Some(func) = exp.try_as_function() {
            if func.get_func_type() == FunctionType::Cast {
                if let Some(other_var) = func.get_lst_operands().get(0).try_as_var() {
                    if var_exp.get_var_version_pair() == other_var.get_var_version_pair() {
                        list.push(VariablePosition::new(VariablePositionEnum::Casted, exp, -1));
                    }
                }
            }
        }
        for sub in exp.get_all_exprents() {
            Self::find_references_exprent(var_exp, sub, root, list);
        }
    }

    /// Java: findReferences(VarExprent, InvocationExprent, List)
    fn find_references_invocation(
        var_exp: VarExprent,
        inv: InvocationExprent,
        list: &mut Vec<VariablePosition>,
    ) {
        let lst_parameters = inv.get_lst_parameters();
        for i in 0..lst_parameters.len() {
            let parameter = lst_parameters.get(i);
            if let Some(other_var) = parameter.try_as_var() {
                if var_exp.get_var_version_pair() == other_var.get_var_version_pair() {
                    list.push(VariablePosition::new(
                        VariablePositionEnum::MethodParameter,
                        inv.0,
                        i as i32,
                    ));
                }
            }
        }
    }

    fn get_casts(exp: Exprent) -> Pair<Vec<Exprent>, Exprent> {
        let mut types = Vec::new();
        let mut inner = exp;
        while let Some(cast) = inner.try_as_function() {
            if !Self::is_valid_cast(cast) {
                break;
            }
            let operands = cast.get_lst_operands();
            types.push(operands.get(1));
            inner = operands.get(0);
        }
        Pair::of(types, inner)
    }

    fn is_valid_cast(cast: FunctionExprent) -> bool {
        if cast.get_func_type() == FunctionType::Cast && cast.get_lst_operands().len() == 2 {
            let type_ = cast.get_lst_operands().get(1).get_expr_type();
            return type_.type_family == TypeFamily::Object && type_.array_dim == 0;
        }
        false
    }

    fn replace_casts(cast: FunctionExprent, types: &mut Vec<Exprent>, inner: Exprent) -> bool {
        unported!("IntersectionCastProcessor.replaceCasts")
    }
}
