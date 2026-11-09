//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IntersectionCastProcessor.java`.

use crate::assignment_exprent::AssignmentExprent;
use crate::code_constants::{ACC_ENUM, ACC_INTERFACE, INIT_NAME};
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::invocation_exprent::InvocationExprent;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{JHashSet, JavaEq, JavaHash, JavaListExt};
use crate::statement::Statement;
use crate::type_family::TypeFamily;
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

    fn handle_invocation(exp: InvocationExprent, _root: RootStatement) -> bool {
        let lst_parameters = exp.get_lst_parameters();
        let mut result = false;
        let mut i = 0;
        while i < lst_parameters.len() {
            let parameter = lst_parameters.get(i);
            if let Some(cast) = parameter
                .try_as_function()
                .filter(|c| Self::is_valid_cast(*c))
            {
                let casts = Self::get_casts(cast.0);
                let mut types = casts.a;
                let inner = casts.b;
                let ctx = DecompilerContext::get_struct_context();
                let bounds: Vec<VarType> = Self::get_bounds(exp, i as i32)
                    .into_iter()
                    .filter(|bound| {
                        !types.iter().any(|constant| {
                            ctx.instance_of(constant.get_expr_type().value_str(), bound.value_str())
                        })
                    })
                    .collect();

                if !bounds.is_empty()
                    && bounds.iter().all(|bound| {
                        ctx.instance_of(inner.get_expr_type().value_str(), bound.value_str())
                    })
                {
                    types.push(ConstExprent::new(inner.get_expr_type(), None, None).0);
                }
                result |= Self::replace_casts(cast, &mut types, inner);
            }
            i += 1;
        }
        result
    }

    fn handle_assignment(exp: AssignmentExprent, root: RootStatement) -> bool {
        if let Some(var_exp) = exp.get_left().try_as_var() {
            let assigned = exp.get_right();
            if let Some(cast) = assigned
                .try_as_function()
                .filter(|c| Self::is_valid_cast(*c))
            {
                let casts = Self::get_casts(cast.0);
                let mut types = casts.a;
                let inner = casts.b;
                let references = Self::find_references(var_exp, root);

                let mut bounds: JHashSet<VarType> = JHashSet::new();
                for position in &references {
                    bounds.add_all(match position.position() {
                        VariablePositionEnum::MethodParameter => {
                            Self::get_bounds(position.exp().as_invocation(), position.index())
                        }
                        VariablePositionEnum::Casted => {
                            let func = position.exp().as_function();
                            if func.get_lst_operands().len() == 2 {
                                vec![func.get_lst_operands().get(1).get_expr_type()]
                            } else {
                                Vec::new()
                            }
                        }
                    });
                }

                let ctx = DecompilerContext::get_struct_context();
                let mut filtered: JHashSet<VarType> = JHashSet::new();
                for bound in bounds.iter() {
                    if !types.iter().any(|constant| {
                        ctx.instance_of(constant.get_expr_type().value_str(), bound.value_str())
                    }) {
                        filtered.add(bound.clone());
                    }
                }
                let bounds = filtered;

                if !bounds.is_empty()
                    && bounds.iter().any(|bound| {
                        ctx.instance_of(inner.get_expr_type().value_str(), bound.value_str())
                    })
                {
                    types.push(ConstExprent::new(inner.get_expr_type(), None, None).0);
                }
                if Self::replace_casts(cast, &mut types, inner) {
                    var_exp.set_intersection_type(true);
                    return true;
                }
            }
        }
        false
    }

    fn get_bounds(exp: InvocationExprent, parameter: i32) -> Vec<VarType> {
        let method = exp.get_desc();
        let gmd = method.and_then(|m| m.get_signature());
        let start = if gmd.is_some()
            && DecompilerContext::get_struct_context()
                .get_class(&method.expect("method").get_class_qualified_name())
                .expect("getClass(method.getClassQualifiedName())")
                .has_modifier(ACC_ENUM)
            && method.expect("method").get_name() == INIT_NAME
        {
            2
        } else {
            0
        };
        if let Some(gmd) = gmd {
            let index = parameter - start;
            let type_ = gmd.parameter_types()[index as usize].clone();
            if type_.type_ == CodeType::Genvar {
                let type_parameter_index = gmd
                    .type_parameters()
                    .iter()
                    .position(|p| type_.value.as_deref() == Some(p.as_str()));
                if let Some(type_parameter_index) = type_parameter_index {
                    return gmd.type_parameter_bounds()[type_parameter_index].clone();
                }
            }
        }
        Vec::new()
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
        if types.len() > 1 {
            let mut non_interface: Option<Exprent> = None;
            for type_ in types.iter() {
                let clazz = DecompilerContext::get_struct_context()
                    .get_class(type_.get_expr_type().value_str());
                if let Some(clazz) = clazz {
                    if !clazz.has_modifier(ACC_INTERFACE) {
                        if non_interface.is_none() {
                            non_interface = Some(*type_);
                        } else {
                            return false;
                        }
                    }
                }
            }
            if let Some(non_interface) = non_interface {
                let index = types.j_index_of(&non_interface);
                types.remove(index as usize);
                types.insert(0, non_interface);
            }
            let operands = cast.get_lst_operands();
            operands.clear();
            operands.add(inner);
            operands.add_all(types.iter().copied());
            return true;
        }
        false
    }
}
