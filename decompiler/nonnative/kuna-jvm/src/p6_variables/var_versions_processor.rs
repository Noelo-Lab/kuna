//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionsProcessor.java`.
//!
//! `mapOriginalVarIndices` is replaced wholesale, never mutated after it is
//! published; it is `Shared` so the getter hands out the map, not a copy.

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
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

    #[allow(dead_code)]
    fn simple_merge(type_processor: VarTypeProcessor, graph: DirectGraph, mt: StructMethod) {
        let map_exprent_max_types = type_processor.get_upper_bounds();
        let map_exprent_min_types = type_processor.get_lower_bounds();

        let mut map_var_versions: JHashMap<i32, JHashSet<i32>> = JHashMap::new();

        let keys = map_exprent_min_types.borrow().key_vec();
        for pair in keys {
            if pair.version >= 0 {
                map_var_versions
                    .compute_if_absent(pair.var, JHashSet::new)
                    .add(pair.version);
            }
        }

        let is_method_static = mt.has_modifier(ACC_STATIC);

        let mut map_merged_versions: JHashMap<VarVersionPair, i32> = JHashMap::new();

        for (key, versions) in map_var_versions.iter() {
            if versions.len() > 1 {
                let mut lst_versions = versions.to_vec();
                lst_versions.sort();

                let mut i = 0;
                while i < lst_versions.len() {
                    let first_pair = VarVersionPair::new(*key, lst_versions[i]);
                    let mut first_type = map_exprent_min_types.borrow().get_cloned(&first_pair);

                    if first_pair.var == 0 && first_pair.version == 1 && !is_method_static {
                        i += 1;
                        continue;
                    }

                    let mut j = i + 1;
                    while j < lst_versions.len() {
                        let second_pair = VarVersionPair::new(*key, lst_versions[j]);
                        let second_type = map_exprent_min_types.borrow().get_cloned(&second_pair);

                        let first = first_type.clone().expect("firstType");
                        let mergeable = first.equals_opt(second_type.as_ref())
                            || (first.equals(&VarType::VARTYPE_NULL())
                                && second_type.as_ref().expect("secondType").type_
                                    == CodeType::Object)
                            || (second_type
                                .as_ref()
                                .expect("secondType")
                                .equals(&VarType::VARTYPE_NULL())
                                && first.type_ == CodeType::Object);
                        if mergeable {
                            let first_max_type =
                                map_exprent_max_types.borrow().get_cloned(&first_pair);
                            let second_max_type =
                                map_exprent_max_types.borrow().get_cloned(&second_pair);
                            let type_ = match (first_max_type, second_max_type) {
                                (None, second) => second,
                                (first, None) => first,
                                (Some(first), Some(second)) => VarType::meet(&first, &second),
                            };

                            put_nullable(&map_exprent_max_types, first_pair, type_);
                            map_merged_versions.put(second_pair, first_pair.version);
                            map_exprent_max_types.borrow_mut().remove(&second_pair);
                            map_exprent_min_types.borrow_mut().remove(&second_pair);

                            if first.equals(&VarType::VARTYPE_NULL()) {
                                put_nullable(
                                    &map_exprent_min_types,
                                    first_pair,
                                    second_type.clone(),
                                );
                                first_type = second_type;
                            }

                            type_processor
                                .get_map_final_vars()
                                .borrow_mut()
                                .put(first_pair, FinalType::NonFinal);

                            lst_versions.remove(j);
                            continue;
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
        }

        if !map_merged_versions.is_empty() {
            Self::update_versions(graph, &map_merged_versions);
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
