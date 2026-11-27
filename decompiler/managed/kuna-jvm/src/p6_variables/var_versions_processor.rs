//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionsProcessor.java`.
//!
//! `mapOriginalVarIndices` is replaced wholesale, never mutated after it is
//! published; it is `Shared` so the getter hands out the map, not a copy.

use crate::code_type::CodeType;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
use crate::fixes::numeric_types::constant_local_int;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, Shared};
use crate::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use crate::struct_method::StructMethod;
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

        constant_local_int::widen(type_processor, graph, method, previous_versions_processor);

        Self::eliminate_non_java_types(type_processor);

        self.set_new_var_indices(type_processor, graph, previous_versions_processor);
    }

    fn merge_phi_versions(ssa: &SSAConstructorSparseEx, graph: DirectGraph) {
        let mut lst: Vec<JHashSet<VarVersionPair>> = Vec::new();
        for (key, versions) in ssa.get_phi().iter() {
            let mut set = JHashSet::new();
            set.add(*key);
            let versions = versions.borrow().iter().collect::<Vec<i32>>();
            for version in versions {
                set.add(VarVersionPair::new(key.var, version));
            }

            for i in (0..lst.len()).rev() {
                let mut intersection = JHashSet::from_collection(set.to_vec());
                intersection.retain_all(&lst[i]);

                if !intersection.is_empty() {
                    set.add_all(lst[i].to_vec());
                    lst.remove(i);
                }
            }

            lst.push(set);
        }

        let mut phi_versions: JHashMap<VarVersionPair, i32> = JHashMap::new();
        for set in &lst {
            let mut min = i32::MAX;
            for paar in set.iter() {
                if paar.version < min {
                    min = paar.version;
                }
            }

            for paar in set.iter() {
                phi_versions.put(VarVersionPair::new(paar.var, paar.version), min);
            }
        }

        Self::update_versions(graph, &phi_versions);
    }

    fn update_versions(graph: DirectGraph, versions: &JHashMap<VarVersionPair, i32>) {
        graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true);
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
        let map_exprent_max_types = type_processor.get_upper_bounds();
        let map_exprent_min_types = type_processor.get_lower_bounds();

        let keys = map_exprent_min_types.borrow().key_vec();
        for paar in keys {
            let type_ = map_exprent_min_types
                .borrow()
                .get_cloned(&paar)
                .expect("mapExprentMinTypes.get(paar)");
            let max_type = map_exprent_max_types.borrow().get_cloned(&paar);

            if type_.type_ == CodeType::Bytechar || type_.type_ == CodeType::Shortchar {
                let type_ = if max_type.is_some_and(|t| t.type_ == CodeType::Char) {
                    VarType::VARTYPE_CHAR()
                } else if type_.type_ == CodeType::Bytechar {
                    VarType::VARTYPE_BYTE()
                } else {
                    VarType::VARTYPE_SHORT()
                };
                map_exprent_min_types.borrow_mut().put(paar, type_);
            } else if type_.type_ == CodeType::Null {
                map_exprent_min_types
                    .borrow_mut()
                    .put(paar, VarType::VARTYPE_OBJECT());
            }
        }
    }

    fn set_new_var_indices(
        self,
        type_processor: VarTypeProcessor,
        graph: DirectGraph,
        previous_versions_processor: Option<VarVersionsProcessor>,
    ) {
        let map_exprent_max_types = type_processor.get_upper_bounds();
        let map_exprent_min_types = type_processor.get_lower_bounds();
        let map_final_vars = type_processor.get_map_final_vars();

        let counters = DecompilerContext::get_counter_container();

        let mut map_var_paar: JHashMap<VarVersionPair, i32> = JHashMap::new();
        let mut map_original_var_indices: JHashMap<i32, VarVersionPair> = JHashMap::new();
        let current = self.get_map_original_var_indices().borrow().entry_vec();
        map_original_var_indices.put_all(current);

        let mut vvps = map_exprent_min_types.borrow().key_vec();
        vvps.sort_by(|o1, o2| {
            let diff = if o1.var != o2.var {
                o1.var.wrapping_sub(o2.var)
            } else {
                o1.version.wrapping_sub(o2.version)
            };
            diff.cmp(&0)
        });

        for pair in vvps {
            if pair.version >= 0 {
                let new_index = if pair.version == 1 {
                    pair.var
                } else {
                    counters.get_counter_and_increment(CounterContainer::VAR_COUNTER)
                };

                let new_var = VarVersionPair::new(new_index, 0);

                let min_type = map_exprent_min_types.borrow().get_cloned(&pair);
                put_nullable(&map_exprent_min_types, new_var, min_type);
                let max_type = map_exprent_max_types.borrow().get_cloned(&pair);
                put_nullable(&map_exprent_max_types, new_var, max_type);

                if map_final_vars.borrow().contains_key(&pair) {
                    let fin = map_final_vars
                        .borrow_mut()
                        .remove(&pair)
                        .expect("mapFinalVars");
                    map_final_vars.borrow_mut().put(new_var, fin);
                }

                map_var_paar.put(pair, new_index);
                map_original_var_indices.put(new_index, pair);
            }
        }

        graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true);
            lst.push(exprent);

            for expr in lst {
                if let Some(new_var) = expr.try_as_var() {
                    if let Some(&new_var_index) =
                        map_var_paar.get(&VarVersionPair::from_var(new_var))
                    {
                        new_var.set_index(new_var_index);
                        new_var.set_version(0);
                    }
                } else if let Some(const_expr) = expr.try_as_const() {
                    let max_type = map_exprent_max_types
                        .borrow()
                        .get_cloned(&VarVersionPair::new(expr.get_id(), -1));
                    if let Some(max_type) = max_type {
                        const_expr.set_const_type(max_type);
                    }
                }
            }

            0
        });

        let published = match previous_versions_processor {
            Some(previous) => {
                let old_indices = previous.get_map_original_var_indices();
                let mut published = JHashMap::with_capacity(map_original_var_indices.len());
                for (key, value) in map_original_var_indices.entry_vec() {
                    let old_value = old_indices.borrow().get_cloned(&value.var);
                    let value = old_value.unwrap_or(value);
                    published.put(key, value);
                }
                published
            }
            None => map_original_var_indices,
        };
        self.with_mut(|d| d.map_original_var_indices = Shared::new(published));
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

/// Java `map.put(key, value)` where `value` may be `null`: a stored `null`
/// reads back exactly like an absent key through `get`, which is the only
/// way these bound maps are read where a `null` can reach them.
fn put_nullable(
    map: &Shared<JHashMap<VarVersionPair, VarType>>,
    key: VarVersionPair,
    value: Option<VarType>,
) {
    match value {
        Some(value) => {
            map.borrow_mut().put(key, value);
        }
        None => {
            map.borrow_mut().remove(&key);
        }
    }
}
