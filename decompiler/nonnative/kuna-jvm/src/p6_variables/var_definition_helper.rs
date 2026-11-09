//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarDefinitionHelper.java`.
//!
//! The merge maps (`this_vars`, `parent`, `leaked`) are keyed by
//! `varproc.getVarOriginalIndex(..)`, which is null for a var created after
//! version splitting; Java's `HashMap` then holds a `null` key, so the key type
//! is `Option<i32>` (hash 0, like Java's null key).
#![allow(dead_code, unused_variables)]

use std::ops::{Deref, DerefMut};

use crate::exprent::Exprent;
use crate::root_statement::RootStatement;
use crate::rt::{objects_hash, JHashMap, JHashSet, JLinkedHashMap, JavaEq, JavaHash, Shared};
use crate::statement::Statement;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
use crate::switch_statement::SwitchStatement;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object};

/// One element of `getSequentialObjects`' `List<Object>`.
#[derive(Clone, Copy, Debug)]
pub enum SequentialObject {
    Exprent(Exprent),
    Statement(Statement),
}

pub struct VarDefinitionHelperData {
    map_var_def_statements: JHashMap<i32, Statement>,
    map_statement_vars: JHashMap<i32, JHashSet<i32>>,
    impl_def_vars: JHashSet<i32>,
    varproc: VarProcessor,
    root: RootStatement,
    mt: StructMethod,
    clashing_names: Shared<JHashMap<VarVersionPair, String>>,
}

arena_object!(pub VarDefinitionHelper => VarDefinitionHelperData, Class);
arena_identity!(VarDefinitionHelper);

impl VarDefinitionHelper {
    /// Java: VarDefinitionHelper(RootStatement, StructMethod, VarProcessor)
    pub fn new(
        root: RootStatement,
        mt: StructMethod,
        varproc: VarProcessor,
    ) -> VarDefinitionHelper {
        Self::new_run(root, mt, varproc, true)
    }

    /// Java: VarDefinitionHelper(RootStatement, StructMethod, VarProcessor, boolean)
    pub fn new_run(
        root: RootStatement,
        mt: StructMethod,
        varproc: VarProcessor,
        run: bool,
    ) -> VarDefinitionHelper {
        let helper = VarDefinitionHelper::alloc(VarDefinitionHelperData {
            map_var_def_statements: JHashMap::new(),
            map_statement_vars: JHashMap::new(),
            impl_def_vars: JHashSet::new(),
            varproc,
            root,
            mt,
            clashing_names: Shared::new(JHashMap::new()),
        });

        if !run {
            return helper;
        }

        unported!("VarDefinitionHelper.<init>")
    }

    pub fn set_var_definitions(self) {
        unported!("VarDefinitionHelper.setVarDefinitions")
    }

    /// Java: findLVT(int, Statement)
    fn find_lvt(self, index: i32, stat: Statement) -> Option<LocalVariable> {
        unported!("VarDefinitionHelper.findLVT")
    }

    /// Java: findLVT(int, Exprent)
    fn find_lvt_exprent(self, index: i32, exp: Exprent) -> Option<LocalVariable> {
        unported!("VarDefinitionHelper.findLVT")
    }

    fn find_first_block(self, stat: Statement, varindex: i32) -> Option<Statement> {
        unported!("VarDefinitionHelper.findFirstBlock")
    }

    /// Returns the elements of the set it records for `stat`, in that set's
    /// iteration order (the Java caller only iterates the returned set).
    fn init_statement(self, stat: Statement) -> Vec<i32> {
        unported!("VarDefinitionHelper.initStatement")
    }

    fn get_all_vars(lst: &[Exprent]) -> Vec<VarExprent> {
        unported!("VarDefinitionHelper.getAllVars")
    }

    fn set_definition(self, expr: Exprent, index: i32) -> bool {
        if let Some(assign) = expr.try_as_assignment() {
            let left = assign.get_left();
            if let Some(var) = left.try_as_var() {
                if var.get_index() == index {
                    var.set_definition(true);
                    return true;
                }
            }
        }
        false
    }

    fn populate_type_bounds(self, proc_: VarProcessor, stat: Statement) {
        unported!("VarDefinitionHelper.populateTypeBounds")
    }

    /// `null` when the SSAU pass fails.
    fn get_var_exprent_sources(self) -> Option<JHashMap<VarID, JHashSet<VarID>>> {
        unported!("VarDefinitionHelper.getVarExprentSources")
    }

    fn find_all_var_exprents(stat: Statement, lookup: &mut JHashMap<VarVersionPair, VarID>) {
        unported!("VarDefinitionHelper.findAllVarExprents")
    }

    fn compare_var_exprent_sources(
        self,
        old_sources: &JHashMap<VarID, JHashSet<VarID>>,
        new_sources: Option<&JHashMap<VarID, JHashSet<VarID>>>,
    ) {
        unported!("VarDefinitionHelper.compareVarExprentSources")
    }

    /// Java: mergeVars(RootStatement)
    fn merge_vars(self, stat: RootStatement) -> Option<VPPEntry> {
        unported!("VarDefinitionHelper.mergeVars")
    }

    fn get_sequential_objects(stat: Statement) -> Vec<SequentialObject> {
        unported!("VarDefinitionHelper.getSequentialObjects")
    }

    /// Java: mergeVars(Statement, Map, Map, Map)
    fn merge_vars_rec(
        self,
        stat: Statement,
        parent: &JHashMap<Option<i32>, VarVersionPair>,
        leaked: &mut JHashMap<Option<i32>, VarVersionPair>,
        denylist: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> Option<VPPEntry> {
        unported!("VarDefinitionHelper.mergeVars")
    }

    fn identify_parent(stat: Statement) -> Option<Statement> {
        unported!("VarDefinitionHelper.identifyParent")
    }

    fn is_var_read_remote(
        stat: Statement,
        var: VarVersionPair,
        check_assign: bool,
        filter: &[Statement],
    ) -> bool {
        unported!("VarDefinitionHelper.isVarReadRemote")
    }

    /// `leaked` is the caller's `scoped` map: `null`, or an alias of its own `leaked`.
    fn process_exprent(
        self,
        exp: Exprent,
        this_vars: &mut JHashMap<Option<i32>, VarVersionPair>,
        leaked: Option<&mut JHashMap<Option<i32>, VarVersionPair>>,
        denylist: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> Option<VPPEntry> {
        unported!("VarDefinitionHelper.processExprent")
    }

    /// Java: remapVar(Statement, VarVersionPair, VarVersionPair)
    fn remap_var(self, stat: Statement, from: VarVersionPair, to: VarVersionPair) -> bool {
        unported!("VarDefinitionHelper.remapVar")
    }

    /// Java: remapVar(Exprent, VarVersionPair, VarVersionPair)
    fn remap_var_exprent(
        self,
        exprent: Option<Exprent>,
        from: VarVersionPair,
        to: VarVersionPair,
    ) -> bool {
        unported!("VarDefinitionHelper.remapVar")
    }

    fn can_const_type_merge(type_: &VarType) -> bool {
        unported!("VarDefinitionHelper.canConstTypeMerge")
    }

    /// Java: getMergedType(VarVersionPair, VarVersionPair)
    fn get_merged_type(self, from: VarVersionPair, to: VarVersionPair) -> Option<VarType> {
        unported!("VarDefinitionHelper.getMergedType")
    }

    /// Java: getMergedType(VarType, VarType, VarType, VarType)
    fn get_merged_type_bounds(
        self,
        from_min: Option<&VarType>,
        to_min: Option<&VarType>,
        from_max: Option<&VarType>,
        to_max: Option<&VarType>,
    ) -> Option<VarType> {
        unported!("VarDefinitionHelper.getMergedType")
    }

    fn propagate_lvts(self, stat: Statement) {
        unported!("VarDefinitionHelper.propagateLVTs")
    }

    /// Java: findTypes(Statement, Map)
    fn find_types(
        self,
        stat: Option<Statement>,
        types: &mut JLinkedHashMap<VarVersionPair, VarInfo>,
    ) {
        unported!("VarDefinitionHelper.findTypes")
    }

    /// Java: findTypes(Exprent, Map)
    fn find_types_exprent(self, exp: Exprent, types: &mut JLinkedHashMap<VarVersionPair, VarInfo>) {
        unported!("VarDefinitionHelper.findTypes")
    }

    /// Java: applyTypes(Statement, Map)
    fn apply_types(self, stat: Option<Statement>, types: &JHashMap<VarVersionPair, LocalVariable>) {
        unported!("VarDefinitionHelper.applyTypes")
    }

    /// Java: applyTypes(Exprent, Map)
    fn apply_types_exprent(
        self,
        exprent: Option<Exprent>,
        types: &JHashMap<VarVersionPair, LocalVariable>,
    ) {
        unported!("VarDefinitionHelper.applyTypes")
    }

    /// Java: isVarReadFirst(VarVersionPair, Statement, int, VarExprent...)
    fn is_var_read_first(
        var: VarVersionPair,
        stat: Statement,
        index: i32,
        allowlist: &[VarExprent],
    ) -> bool {
        unported!("VarDefinitionHelper.isVarReadFirst")
    }

    /// Java: isVarReadFirst(VarVersionPair, Exprent, VarExprent...)
    fn is_var_read_first_exprent(
        target: VarVersionPair,
        exp: Exprent,
        allowlist: &[VarExprent],
    ) -> bool {
        unported!("VarDefinitionHelper.isVarReadFirst")
    }

    /// Java: setNonFinal(Statement, Set)
    fn set_non_final(self, stat: Statement, un_initialized: &mut JHashSet<VarVersionPair>) {
        unported!("VarDefinitionHelper.setNonFinal")
    }

    /// Java: setNonFinal(Exprent, Set)
    fn set_non_final_exprent(
        self,
        exp: Option<Exprent>,
        un_initialized: &mut JHashSet<VarVersionPair>,
    ) {
        unported!("VarDefinitionHelper.setNonFinal")
    }

    pub fn remap_clashing_names(self, root: Statement, mt: StructMethod) {
        unported!("VarDefinitionHelper.remapClashingNames")
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_clashing_names(
        self,
        stat: Statement,
        mt: StructMethod,
        var_definitions: &mut JHashMap<Statement, JHashSet<VarInMethod>>,
        live_var_defs: &mut JHashSet<VarInMethod>,
        cur_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut JHashMap<VarInMethod, String>,
        seen_methods: &mut JHashSet<String>,
    ) {
        unported!("VarDefinitionHelper.iterateClashingNames")
    }

    fn clear_statement(
        self,
        var_definitions: &mut JHashMap<Statement, JHashSet<VarInMethod>>,
        live_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut JHashMap<VarInMethod, String>,
        st: Statement,
    ) {
        unported!("VarDefinitionHelper.clearStatement")
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_clashing_exprent(
        self,
        stat: Statement,
        mt: StructMethod,
        var_definitions: &mut JHashMap<Statement, JHashSet<VarInMethod>>,
        exprent: Exprent,
        live_var_defs: &mut JHashSet<VarInMethod>,
        cur_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut JHashMap<VarInMethod, String>,
        seen_methods: &mut JHashSet<String>,
        when_true: &mut JHashSet<VarInMethod>,
        when_false: &mut JHashSet<VarInMethod>,
    ) {
        unported!("VarDefinitionHelper.iterateClashingExprent")
    }

    fn rename(
        name_map: &JHashMap<VarInMethod, String>,
        name: &str,
        live_var_defs: &JHashSet<VarInMethod>,
    ) -> String {
        let mut name = name.to_string();
        while live_var_defs
            .iter()
            .map(|v| name_map.get(v))
            .any(|n| n.is_some_and(|n| *n == name))
        {
            name.push('x');
        }
        name
    }

    fn find_case_owning(stat: Statement, switch_stat: SwitchStatement) -> Option<Statement> {
        unported!("VarDefinitionHelper.findCaseOwning")
    }

    fn direct_parent(stat: Statement) -> Option<Statement> {
        unported!("VarDefinitionHelper.directParent")
    }

    pub fn get_clashing_names(self) -> Shared<JHashMap<VarVersionPair, String>> {
        self.with(|d| d.clashing_names.clone())
    }
}

/// Java: `VarDefinitionHelper.VarID`: a `VarExprent` compared by identity.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VarID {
    pub var: VarExprent,
}

impl VarID {
    pub fn new(var: VarExprent) -> VarID {
        VarID { var }
    }
}

impl JavaHash for VarID {
    fn java_hash(&self) -> i32 {
        self.var.index() as i32
    }
}

impl JavaEq for VarID {
    fn java_equals(&self, other: &Self) -> bool {
        self.var == other.var
    }
}

/// Java: `VarDefinitionHelper.SimpleEntry`.
#[derive(Clone, Debug)]
pub struct SimpleEntry<K, V> {
    key: K,
    value: V,
}

impl<K: Clone, V: Clone> SimpleEntry<K, V> {
    pub fn new(key: K, value: V) -> SimpleEntry<K, V> {
        SimpleEntry { key, value }
    }

    pub fn get_key(&self) -> K {
        self.key.clone()
    }

    pub fn get_value(&self) -> V {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: V) -> V {
        std::mem::replace(&mut self.value, value)
    }
}

/// Java: `VarDefinitionHelper.VPPEntry`.
#[derive(Clone, Debug)]
pub struct VPPEntry(pub SimpleEntry<VarVersionPair, VarVersionPair>);

impl VPPEntry {
    pub fn new(key: VarExprent, value: VarVersionPair) -> VPPEntry {
        VPPEntry(SimpleEntry::new(VarVersionPair::from_var(key), value))
    }
}

impl Deref for VPPEntry {
    type Target = SimpleEntry<VarVersionPair, VarVersionPair>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VPPEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Java: `VarDefinitionHelper.VarInfo`.
#[derive(Clone)]
pub struct VarInfo {
    lvt: Option<LocalVariable>,
    cast: String,
    type_: Option<VarType>,
}

impl VarInfo {
    fn new(lvt: Option<LocalVariable>, type_: Option<VarType>) -> VarInfo {
        unported!("VarDefinitionHelper.VarInfo.<init>")
    }

    pub fn get_lvt(&self) -> Option<LocalVariable> {
        self.lvt.clone()
    }

    pub fn get_cast(&self) -> String {
        self.cast.clone()
    }

    pub fn get_type(&self) -> Option<VarType> {
        self.type_.clone()
    }
}

/// Java: `VarDefinitionHelper.VarInMethod`.
#[derive(Clone, Copy, Debug)]
pub struct VarInMethod {
    pair: VarVersionPair,
    mt: StructMethod,
}

impl VarInMethod {
    fn new(pair: VarVersionPair, mt: StructMethod) -> VarInMethod {
        VarInMethod { pair, mt }
    }
}

impl JavaEq for VarInMethod {
    fn java_equals(&self, other: &Self) -> bool {
        self.pair.java_equals(&other.pair) && self.mt.java_equals(&other.mt)
    }
}

impl JavaHash for VarInMethod {
    fn java_hash(&self) -> i32 {
        objects_hash(&[self.pair.java_hash(), self.mt.java_hash()])
    }
}

impl std::fmt::Display for VarInMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.mt.get_name(), self.pair)
    }
}
