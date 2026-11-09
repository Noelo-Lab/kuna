//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarTypeProcessor.java`.
//!
//! The three bound maps are `Shared`: `VarVersionsProcessor` and
//! `VarDefinitionHelper` take them through the getters and write into them.

use std::collections::VecDeque;

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JavaEq, JavaHash, Shared};
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum FinalType {
    NonFinal,
    ExplicitFinal,
    Final,
}

impl FinalType {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            FinalType::NonFinal => "NON_FINAL",
            FinalType::ExplicitFinal => "EXPLICIT_FINAL",
            FinalType::Final => "FINAL",
        }
    }
}

impl JavaHash for FinalType {
    fn java_hash(&self) -> i32 {
        self.ordinal()
    }
}

impl JavaEq for FinalType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Java: `VarTypeProcessor.Bound`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum Bound {
    Lower,
    Upper,
}

pub struct VarTypeProcessorData {
    method: StructMethod,
    method_descriptor: MethodDescriptor,
    lower_bounds: Shared<JHashMap<VarVersionPair, VarType>>,
    upper_bounds: Shared<JHashMap<VarVersionPair, VarType>>,
    map_final_vars: Shared<JHashMap<VarVersionPair, FinalType>>,
}

arena_object!(pub VarTypeProcessor => VarTypeProcessorData, Class);
arena_identity!(VarTypeProcessor);

impl VarTypeProcessor {
    pub fn new(mt: StructMethod, md: &MethodDescriptor) -> VarTypeProcessor {
        VarTypeProcessor::alloc(VarTypeProcessorData {
            method: mt,
            method_descriptor: md.clone(),
            lower_bounds: Shared::new(JHashMap::new()),
            upper_bounds: Shared::new(JHashMap::new()),
            map_final_vars: Shared::new(JHashMap::new()),
        })
    }

    pub fn calculate_var_types(self, root: RootStatement, graph: DirectGraph) {
        self.set_init_vars(root);

        Self::reset_exprent_types(graph);

        while !self.process_var_types(graph) {}

        let (lower_bounds, upper_bounds) =
            self.with(|d| (d.lower_bounds.clone(), d.upper_bounds.clone()));
        let keys = lower_bounds.borrow().key_vec();
        for p in keys {
            let lower = lower_bounds
                .borrow()
                .get_cloned(&p)
                .expect("lowerBounds.get(p)");
            let upper = upper_bounds.borrow().get_cloned(&p);

            if let Some(upper) = upper {
                if lower.type_family != TypeFamily::Object && lower.higher_in_lattice_than(&upper) {
                    ValidationHelper::validate_true(
                        false,
                        &format!("lower bound {lower} > upper bound {upper} for var {p}"),
                    );
                }
            }
        }

        ValidationHelper::validate_vars(
            graph,
            root,
            |var| !var.get_var_type().ptr_eq(&VarType::VARTYPE_UNKNOWN()),
            "Var type not set!",
        );
    }

    fn set_init_vars(self, root: RootStatement) {
        let (method, md, lower_bounds, upper_bounds) = self.with(|d| {
            (
                d.method,
                d.method_descriptor.clone(),
                d.lower_bounds.clone(),
                d.upper_bounds.clone(),
            )
        });
        let this_var = !method.has_modifier(ACC_STATIC);

        if this_var {
            let cl = DecompilerContext::get_current_class();
            ValidationHelper::assert_true(cl.is_some(), "Current class name should not be null");
            let cl = cl.expect("Current class name should not be null");
            let cl_type = VarType::new_value(CodeType::Object, 0, cl.qualified_name());
            lower_bounds
                .borrow_mut()
                .put(VarVersionPair::new(0, 1), cl_type.clone());
            upper_bounds
                .borrow_mut()
                .put(VarVersionPair::new(0, 1), cl_type);
        }

        let mut var_index: i32 = 0;
        for param in &md.params {
            let index = var_index + if this_var { 1 } else { 0 };
            lower_bounds
                .borrow_mut()
                .put(VarVersionPair::new(index, 1), param.clone());
            upper_bounds
                .borrow_mut()
                .put(VarVersionPair::new(index, 1), param.clone());
            var_index += param.stack_size;
        }

        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(root.0);

        while let Some(stat) = stack.pop_front() {
            let vars = stat.get_implicitly_defined_vars();

            if let Some(vars) = vars {
                for var in vars {
                    let var_type = var.get_var_type();
                    lower_bounds
                        .borrow_mut()
                        .put(VarVersionPair::new(var.get_index(), 1), var_type);
                    if stat.is_catch() || stat.is_catch_all() {
                        let var_type = var.get_var_type();
                        upper_bounds
                            .borrow_mut()
                            .put(VarVersionPair::new(var.get_index(), 1), var_type);
                    }
                }
            }

            let stats = stat.get_stats().borrow().to_vec();
            stack.extend(stats);
        }
    }

    fn reset_exprent_types(graph: DirectGraph) {
        graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true);
            lst.push(exprent);

            for expr in lst {
                if let Some(ve) = expr.try_as_var() {
                    ve.set_var_type(VarType::VARTYPE_UNKNOWN());
                } else if let Some(const_expr) = expr.try_as_const() {
                    if const_expr.get_const_type().type_family == TypeFamily::Integer {
                        let fresh = ConstExprent::new_int(
                            const_expr.get_int_value(),
                            const_expr.is_bool_permitted(),
                            None,
                        );
                        const_expr.set_const_type(fresh.get_const_type());
                    }
                }
            }
            0
        });
    }

    fn process_var_types(self, graph: DirectGraph) -> bool {
        graph.iterate_exprents(|exprent: Exprent| {
            if self.check_type_exprent(exprent) {
                0
            } else {
                1
            }
        })
    }

    fn check_type_exprent(self, exprent: Exprent) -> bool {
        for expr in exprent.get_all_exprents_rec(true) {
            if !self.check_type_expr(expr) {
                return false;
            }
        }

        self.check_type_expr(exprent)
    }

    fn check_type_expr(self, exprent: Exprent) -> bool {
        if let Some(const_expr) = exprent.try_as_const() {
            let family = const_expr.get_const_type().type_family;
            if family.int_or_bool() {
                let pair = VarVersionPair::new(const_expr.get_id(), -1);
                let lower_bounds = self.get_lower_bounds();
                if !lower_bounds.borrow().contains_key(&pair) {
                    let const_type = const_expr.get_const_type();
                    lower_bounds.borrow_mut().put(pair, const_type);
                }
            }
        }

        let result = exprent.check_expr_type_bounds();

        let mut res = true;
        if let Some(result) = result {
            for entry in result.get_upper_bounds() {
                self.change_exprent_type(entry.exprent, &entry.type_, Bound::Upper);
            }

            for entry in result.get_lower_bounds() {
                res &= self.change_exprent_type(entry.exprent, &entry.type_, Bound::Lower);
            }
        }
        res
    }

    fn change_exprent_type(self, exprent: Exprent, new_type: &VarType, bound: Bound) -> bool {
        ValidationHelper::assert_true(true, "Null type passed to CheckTypesResult!");

        match exprent.get_type() {
            ExprentType::Const => {
                let const_expr = exprent.as_const();
                let const_type = const_expr.get_const_type();

                let mut new_type = new_type.clone();
                if !new_type.type_family.int_or_bool() || !const_type.type_family.int_or_bool() {
                    return true;
                } else if new_type.type_family == TypeFamily::Integer {
                    let value = match const_expr.get_value() {
                        Some(value) => value.as_int(),
                        None => crate::jthrow!(NullPointerException; "ConstExprent.value is null"),
                    };
                    let min_integer = ConstExprent::new_int(value, false, None).get_const_type();
                    if min_integer.higher_in_lattice_than(&new_type) {
                        new_type = min_integer;
                    }
                }

                return self.change_var_exprent_type(
                    exprent,
                    &new_type,
                    bound,
                    VarVersionPair::new(exprent.get_id(), -1),
                );
            }
            ExprentType::Var => {
                return self.change_var_exprent_type(
                    exprent,
                    new_type,
                    bound,
                    VarVersionPair::from_var(exprent.as_var()),
                );
            }
            ExprentType::Assignment => {
                return self.change_exprent_type(
                    exprent.as_assignment().get_right(),
                    new_type,
                    bound,
                );
            }
            ExprentType::Function => {
                return self.change_function_exprent_type(new_type, bound, exprent.as_function());
            }
            ExprentType::Switch => {
                let sw = exprent.as_switch();
                if new_type.type_family == TypeFamily::Integer
                    && new_type.higher_in_lattice_than(&sw.get_expr_type())
                {
                    sw.set_type(new_type.clone());
                    return false;
                }
            }
            _ => {}
        }

        true
    }

    fn change_var_exprent_type(
        self,
        exprent: Exprent,
        new_type: &VarType,
        bound: Bound,
        pair: VarVersionPair,
    ) -> bool {
        if bound == Bound::Lower {
            let lower_bounds = self.get_lower_bounds();
            let current_min_type = lower_bounds.borrow().get_cloned(&pair);
            let new_min_type = match &current_min_type {
                None => Some(new_type.clone()),
                Some(current) if new_type.type_family.is_greater(current.type_family) => {
                    Some(new_type.clone())
                }
                Some(current) if new_type.type_family.is_lesser(current.type_family) => {
                    return true;
                }
                Some(current) => VarType::join(current, new_type),
            };
            ValidationHelper::assert_true(
                new_min_type.is_some(),
                "Trying to raise the minimum type of disjoint variables!",
            );
            let new_min_type = new_min_type.expect("newMinType");

            lower_bounds.borrow_mut().put(pair, new_min_type.clone());
            if let Some(const_expr) = exprent.try_as_const() {
                const_expr.set_const_type(new_min_type.clone());
            }

            if let Some(current) = &current_min_type {
                if new_min_type.type_family.is_greater(current.type_family)
                    || new_min_type.higher_in_lattice_than(current)
                {
                    return false;
                }
            }
        } else {
            let upper_bounds = self.get_upper_bounds();
            let current_max_type = upper_bounds.borrow().get_cloned(&pair);
            let new_max_type = match &current_max_type {
                None => Some(new_type.clone()),
                Some(current) if new_type.type_family.is_lesser(current.type_family) => {
                    Some(new_type.clone())
                }
                Some(current) if new_type.type_family.is_greater(current.type_family) => {
                    Some(new_type.clone())
                }
                Some(current) => VarType::meet(current, new_type),
            };
            ValidationHelper::assert_true(
                new_max_type.is_some(),
                "Trying to lower the maximum type of disjoint variables!",
            );
            let new_max_type = new_max_type.expect("newMaxType");

            upper_bounds.borrow_mut().put(pair, new_max_type);
        }
        true
    }

    fn change_function_exprent_type(
        self,
        new_type: &VarType,
        bound: Bound,
        func: FunctionExprent,
    ) -> bool {
        let mut offset = 0;
        let func_type = func.get_func_type();
        if func_type == FunctionType::Ternary {
            offset += 1;
        }
        match func_type {
            FunctionType::Ternary | FunctionType::And | FunctionType::Or | FunctionType::Xor => {
                let operands = func.get_lst_operands();
                let first = self.change_exprent_type(operands.get(offset), new_type, bound);
                let second = self.change_exprent_type(operands.get(offset + 1), new_type, bound);
                first & second
            }
            _ => true,
        }
    }

    pub fn get_upper_bounds(self) -> Shared<JHashMap<VarVersionPair, VarType>> {
        self.with(|d| d.upper_bounds.clone())
    }

    pub fn get_lower_bounds(self) -> Shared<JHashMap<VarVersionPair, VarType>> {
        self.with(|d| d.lower_bounds.clone())
    }

    pub fn get_map_final_vars(self) -> Shared<JHashMap<VarVersionPair, FinalType>> {
        self.with(|d| d.map_final_vars.clone())
    }

    pub fn set_var_type(self, pair: VarVersionPair, type_: &VarType) {
        self.get_lower_bounds()
            .borrow_mut()
            .put(pair, type_.clone());
    }

    pub fn get_var_type(self, pair: VarVersionPair) -> Option<VarType> {
        self.get_lower_bounds().borrow().get(&pair).cloned()
    }
}
