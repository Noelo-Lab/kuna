//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionsProcessor.java`.
//!
//! `mapOriginalVarIndices` is replaced wholesale, never mutated after it is
//! published; it is `Shared` so the getter hands out the map, not a copy.
#![allow(dead_code, unused_variables)]

use crate::direct_graph::DirectGraph;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, Shared};
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::struct_method::StructMethod;
use crate::unported;
use crate::var_type::VarType;
use crate::var_type_processor::{FinalType, VarTypeProcessor};
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object};

pub struct VarVersionsProcessorData {
    method: StructMethod,
    map_original_var_indices: Shared<JHashMap<i32, VarVersionPair>>,
    type_processor: VarTypeProcessor,
}

arena_object!(pub VarVersionsProcessor => VarVersionsProcessorData, Class);
arena_identity!(VarVersionsProcessor);

impl VarVersionsProcessor {
    pub fn new(mt: StructMethod, md: &MethodDescriptor) -> VarVersionsProcessor {
        let type_processor = VarTypeProcessor::new(mt, md);
        VarVersionsProcessor::alloc(VarVersionsProcessorData {
            method: mt,
            map_original_var_indices: Shared::new(JHashMap::new()),
            type_processor,
        })
    }

    pub fn set_var_versions(
        self,
        root: RootStatement,
        previous_versions_processor: Option<VarVersionsProcessor>,
    ) {
        let (method, type_processor) = self.with(|d| (d.method, d.type_processor));

        let mut ssa = SSAConstructorSparseEx::new();
        ssa.split_variables(root, method);

        let mut flatten_helper = FlattenStatementsHelper::new();
        let graph = flatten_helper.build_direct_graph(root);

        Self::merge_phi_versions(&ssa, graph);

        type_processor.calculate_var_types(root, graph);

        Self::eliminate_non_java_types(type_processor);

        self.set_new_var_indices(type_processor, graph, previous_versions_processor);
    }

    fn merge_phi_versions(ssa: &SSAConstructorSparseEx, graph: DirectGraph) {
        unported!("VarVersionsProcessor.mergePhiVersions")
    }

    fn update_versions(graph: DirectGraph, versions: &JHashMap<VarVersionPair, i32>) {
        graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true).to_vec();
            lst.push(exprent);

            for expr in lst {
                if let Some(var) = expr.try_as_var() {
                    if let Some(&version) = versions.get(&VarVersionPair::from_var(var)) {
                        var.set_version(version);
                    }
                }
            }

            0
        });
    }

    fn eliminate_non_java_types(type_processor: VarTypeProcessor) {
        unported!("VarVersionsProcessor.eliminateNonJavaTypes")
    }

    fn simple_merge(type_processor: VarTypeProcessor, graph: DirectGraph, mt: StructMethod) {
        unported!("VarVersionsProcessor.simpleMerge")
    }

    fn set_new_var_indices(
        self,
        type_processor: VarTypeProcessor,
        graph: DirectGraph,
        previous_versions_processor: Option<VarVersionsProcessor>,
    ) {
        unported!("VarVersionsProcessor.setNewVarIndices")
    }

    pub fn get_var_type(self, pair: VarVersionPair) -> Option<VarType> {
        self.get_type_processor().get_var_type(pair)
    }

    pub fn set_var_type(self, pair: VarVersionPair, type_: &VarType) {
        self.get_type_processor().set_var_type(pair, type_);
    }

    pub fn get_var_final(self, pair: VarVersionPair) -> FinalType {
        let fin = self
            .get_type_processor()
            .get_map_final_vars()
            .borrow()
            .get(&pair)
            .copied();
        fin.unwrap_or(FinalType::Final)
    }

    pub fn set_var_final(self, pair: VarVersionPair, final_type: FinalType) {
        self.get_type_processor()
            .get_map_final_vars()
            .borrow_mut()
            .put(pair, final_type);
    }

    pub fn get_map_original_var_indices(self) -> Shared<JHashMap<i32, VarVersionPair>> {
        self.with(|d| d.map_original_var_indices.clone())
    }

    pub fn get_type_processor(self) -> VarTypeProcessor {
        self.with(|d| d.type_processor)
    }
}
