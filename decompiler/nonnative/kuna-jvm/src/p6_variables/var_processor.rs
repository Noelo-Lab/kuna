//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarProcessor.java`.
//!
//! `thisVars`, `externalVars`, `syntheticSemaphores` and `inheritedNames` are
//! `Shared`: their getters hand out the collection and callers mutate it.
//! Getters that return a key/value *view* (`getUsedVarVersions`,
//! `getVarNames`, `clashingNames`) return a snapshot in iteration order.

use crate::kuna_trace;
use crate::method_descriptor::MethodDescriptor;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JavaListExt, Shared};
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
use crate::text_util::TextUtil;
use crate::var_definition_helper::VarDefinitionHelper;
use crate::var_exprent::VarExprent;
use crate::var_names_collector::VarNamesCollector;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_processor::VarVersionsProcessor;
use crate::{arena_identity, arena_object};

pub struct VarProcessorData {
    var_names_collector: VarNamesCollector,
    method: StructMethod,
    method_descriptor: MethodDescriptor,
    map_var_names: JHashMap<VarVersionPair, String>,
    params: Vec<VarVersionPair>,
    map_var_lvts: JHashMap<VarVersionPair, LocalVariable>,
    var_versions: Option<VarVersionsProcessor>,
    this_vars: Shared<JHashMap<VarVersionPair, String>>,
    external_vars: Shared<JHashSet<VarVersionPair>>,
    clashing_names: JHashMap<VarVersionPair, String>,
    inherited_names: Shared<JHashMap<VarVersionPair, String>>,
    synthetic_semaphores: Shared<JHashSet<i32>>,
    var_sources: JHashMap<VarVersionPair, Pair<String, VarVersionPair>>,
    pub nested_processed: bool,
}

arena_object!(pub VarProcessor => VarProcessorData, Class);
arena_identity!(VarProcessor);

impl VarProcessor {
    pub fn new(mt: StructMethod, md: &MethodDescriptor) -> VarProcessor {
        let var_names_collector = VarNamesCollector::new();
        VarProcessor::alloc(VarProcessorData {
            var_names_collector,
            method: mt,
            method_descriptor: md.clone(),
            map_var_names: JHashMap::new(),
            params: Vec::new(),
            map_var_lvts: JHashMap::new(),
            var_versions: None,
            this_vars: Shared::new(JHashMap::new()),
            external_vars: Shared::new(JHashSet::new()),
            clashing_names: JHashMap::new(),
            inherited_names: Shared::new(JHashMap::new()),
            synthetic_semaphores: Shared::new(JHashSet::new()),
            var_sources: JHashMap::new(),
            nested_processed: false,
        })
    }

    pub fn nested_processed(self) -> bool {
        self.with(|d| d.nested_processed)
    }

    pub fn set_nested_processed(self, nested_processed: bool) {
        self.with_mut(|d| d.nested_processed = nested_processed)
    }

    pub fn set_var_versions(self, root: RootStatement) {
        let (old_processor, method, md) =
            self.with(|d| (d.var_versions, d.method, d.method_descriptor.clone()));
        let var_versions = VarVersionsProcessor::new(method, &md);
        self.with_mut(|d| d.var_versions = Some(var_versions));
        var_versions.set_var_versions(root, old_processor);
    }

    pub fn set_var_definitions(self, root: RootStatement) {
        self.with_mut(|d| d.map_var_names = JHashMap::new());
        let method = self.with(|d| d.method);
        let var_def = VarDefinitionHelper::new(root, method, self);
        var_def.set_var_definitions();
        let clashing = var_def.get_clashing_names().borrow().entry_vec();
        self.with_mut(|d| d.clashing_names.put_all(clashing));
    }

    pub fn set_debug_var_names(
        self,
        root: RootStatement,
        map_debug_var_names: &JHashMap<VarVersionPair, String>,
    ) {
        let Some(var_versions) = self.with(|d| d.var_versions) else {
            return;
        };

        let map_original_var_indices = var_versions.get_map_original_var_indices();

        let mut list_vars = self.with(|d| d.map_var_names.key_vec());
        list_vars.sort_by_key(|o| o.var);

        let mut map_names: JHashMap<String, i32> = JHashMap::new();

        let method = self.with(|d| d.method);
        for pair in list_vars {
            let mut name = self
                .with(|d| d.map_var_names.get_cloned(&pair))
                .expect("mapVarNames.get(pair)");

            let mut lvt_name = false;
            let key = map_original_var_indices.borrow().get_cloned(&pair.var);
            if let Some(key) = key {
                if let Some(debug_name) = map_debug_var_names.get(&key) {
                    if TextUtil::is_valid_identifier(
                        debug_name,
                        &method.get_bytecode_version(),
                        root.mt(),
                    ) {
                        name = debug_name.clone();
                        lvt_name = true;
                    }
                }
            }

            let counter = match map_names.get_cloned(&name) {
                None => 0,
                Some(counter) => counter + 1,
            };
            map_names.put(name.clone(), counter);

            if counter > 0 && !lvt_name {
                name.push_str(&counter.to_string());
            }

            self.with_mut(|d| d.map_var_names.put(pair, name));
        }

        self.rerun_clashing(root);
    }

    pub fn rerun_clashing(self, root: RootStatement) {
        let method = self.with(|d| d.method);
        let vardef = VarDefinitionHelper::new_run(root, method, self, false);
        vardef.remap_clashing_names(root.into(), method);

        let clashing = vardef.get_clashing_names().borrow().entry_vec();
        for (key, value) in clashing {
            if !self.with(|d| d.params.j_contains(&key)) {
                self.with_mut(|d| d.clashing_names.put(key, value));
            }
        }

        if kuna_trace::enabled() {
            kuna_trace::step_vars(method, "RerunClashing", root, self);
        }
    }

    pub fn get_var_original_index(self, index: i32) -> Option<i32> {
        let var_versions = self.with(|d| d.var_versions)?;
        let pair = var_versions
            .get_map_original_var_indices()
            .borrow()
            .get(&index)
            .copied();
        pair.map(|p| p.var)
    }

    pub fn refresh_var_names(self, vc: VarNamesCollector) {
        let temp_var_names = self.with(|d| d.map_var_names.clone());
        for (key, value) in temp_var_names.entry_vec() {
            let name = vc.get_free_name(&value);
            self.with_mut(|d| d.map_var_names.put(key, name));
        }
    }

    pub fn get_synthetic_semaphores(self) -> Shared<JHashSet<i32>> {
        self.with(|d| d.synthetic_semaphores.clone())
    }

    pub fn get_var_names_collector(self) -> VarNamesCollector {
        self.with(|d| d.var_names_collector)
    }

    pub fn get_var_type(self, pair: VarVersionPair) -> Option<VarType> {
        self.with(|d| d.var_versions)?.get_var_type(pair)
    }

    pub fn mark_param(self, pair: VarVersionPair) {
        self.with_mut(|d| d.params.push(pair));
    }

    pub fn get_params(self) -> Vec<VarVersionPair> {
        self.with(|d| d.params.clone())
    }

    pub fn set_var_type(self, pair: VarVersionPair, type_: &VarType) {
        if let Some(var_versions) = self.with(|d| d.var_versions) {
            var_versions.set_var_type(pair, type_);
        }
    }

    pub fn get_var_name(self, pair: VarVersionPair) -> Option<String> {
        self.with(|d| d.map_var_names.get(&pair).cloned())
    }

    pub fn get_clashing_name(self, pair: VarVersionPair) -> Option<String> {
        self.with(|d| d.clashing_names.get(&pair).cloned())
    }

    pub fn set_clashing_name(self, pair: VarVersionPair, name: impl Into<String>) {
        let name = name.into();
        self.with_mut(|d| d.clashing_names.put(pair, name));
    }

    /// `clashingNames.values()`, in iteration order.
    pub fn clashing_names(self) -> Vec<String> {
        self.with(|d| d.clashing_names.value_vec())
    }

    pub fn set_var_name(self, pair: VarVersionPair, name: impl Into<String>) {
        let name = name.into();
        self.with_mut(|d| d.map_var_names.put(pair, name));
    }

    pub fn set_var_source(
        self,
        pair: VarVersionPair,
        method: impl Into<String>,
        original: VarVersionPair,
    ) {
        let source = Pair::of(method.into(), original);
        self.with_mut(|d| d.var_sources.put(pair, source));
    }

    pub fn set_inherited_name(self, pair: VarVersionPair, name: impl Into<String>) {
        let name = name.into();
        self.set_var_name(pair, name.clone());

        self.with(|d| d.inherited_names.clone())
            .borrow_mut()
            .put(pair, name);
    }

    pub fn get_inherited_names(self) -> Shared<JHashMap<VarVersionPair, String>> {
        self.with(|d| d.inherited_names.clone())
    }

    /// `mapVarNames.keySet()`, in iteration order.
    pub fn get_used_var_versions(self) -> Vec<VarVersionPair> {
        self.with(|d| d.map_var_names.key_vec())
    }

    /// `mapVarNames.values()`, in iteration order.
    pub fn get_var_names(self) -> Vec<String> {
        self.with(|d| d.map_var_names.value_vec())
    }

    pub fn get_var_final(self, pair: VarVersionPair) -> FinalType {
        match self.with(|d| d.var_versions) {
            None => FinalType::Final,
            Some(var_versions) => var_versions.get_var_final(pair),
        }
    }

    pub fn get_var_source(self, pair: VarVersionPair) -> Option<Pair<String, VarVersionPair>> {
        self.with(|d| d.var_sources.get(&pair).cloned())
    }

    pub fn set_var_final(self, pair: VarVersionPair, final_type: FinalType) {
        if let Some(var_versions) = self.with(|d| d.var_versions) {
            var_versions.set_var_final(pair, final_type);
        }
    }

    pub fn get_this_vars(self) -> Shared<JHashMap<VarVersionPair, String>> {
        self.with(|d| d.this_vars.clone())
    }

    pub fn get_external_vars(self) -> Shared<JHashSet<VarVersionPair>> {
        self.with(|d| d.external_vars.clone())
    }

    pub fn get_candidates(self, origindex: i32) -> Option<Vec<LocalVariable>> {
        if !self.has_lvt() {
            return None;
        }
        let method = self.with(|d| d.method);
        let attr = method
            .get_local_variable_attr()
            .expect("LocalVariableTable");
        Some(attr.matching_vars_index(origindex))
    }

    pub fn find_lvt(self, exprent: VarExprent, start: i32) {
        if !self.has_lvt() {
            return;
        }

        let method = self.with(|d| d.method);
        let attr = method
            .get_local_variable_attr()
            .expect("LocalVariableTable");
        let index = exprent.get_index();
        let lvt = attr
            .get_variables()
            .into_iter()
            .find(|v| v.get_version().var == index && v.get_start() == start);

        if let Some(lvt) = lvt {
            exprent.set_lvt(Some(lvt));
        }
    }

    pub fn has_lvt(self) -> bool {
        self.with(|d| d.method).get_local_variable_attr().is_some()
    }

    pub fn get_var_versions(self) -> Option<VarVersionsProcessor> {
        self.with(|d| d.var_versions)
    }

    pub fn set_var_lvt(self, var: VarVersionPair, lvt: LocalVariable) {
        self.with_mut(|d| d.map_var_lvts.put(var, lvt));
    }

    pub fn get_var_lvt(self, var: VarVersionPair) -> Option<LocalVariable> {
        self.with(|d| d.map_var_lvts.get(&var).cloned())
    }
}
