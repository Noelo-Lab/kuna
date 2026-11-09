//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarTypeProcessor.java`.
//!
//! The three bound maps are `Shared`: `VarVersionsProcessor` and
//! `VarDefinitionHelper` take them through the getters and write into them.
#![allow(dead_code, unused_variables)]

use crate::direct_graph::DirectGraph;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JavaEq, JavaHash, Shared};
use crate::struct_method::StructMethod;
use crate::unported;
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
        unported!("VarTypeProcessor.calculateVarTypes")
    }

    fn set_init_vars(self, root: RootStatement) {
        unported!("VarTypeProcessor.setInitVars")
    }

    fn reset_exprent_types(graph: DirectGraph) {
        unported!("VarTypeProcessor.resetExprentTypes")
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
        for expr in exprent.get_all_exprents_rec(true).to_vec() {
            if !self.check_type_expr(expr) {
                return false;
            }
        }

        self.check_type_expr(exprent)
    }

    fn check_type_expr(self, exprent: Exprent) -> bool {
        unported!("VarTypeProcessor.checkTypeExpr")
    }

    fn change_exprent_type(self, exprent: Exprent, new_type: &VarType, bound: Bound) -> bool {
        unported!("VarTypeProcessor.changeExprentType")
    }

    fn change_var_exprent_type(
        self,
        exprent: Exprent,
        new_type: &VarType,
        bound: Bound,
        pair: VarVersionPair,
    ) -> bool {
        unported!("VarTypeProcessor.changeVarExprentType")
    }

    fn change_function_exprent_type(
        self,
        new_type: &VarType,
        bound: Bound,
        func: FunctionExprent,
    ) -> bool {
        unported!("VarTypeProcessor.changeFunctionExprentType")
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
