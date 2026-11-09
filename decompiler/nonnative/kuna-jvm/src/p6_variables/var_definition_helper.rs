//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarDefinitionHelper.java`.
//!
//! The merge maps (`this_vars`, `parent`, `leaked`) are keyed by
//! `varproc.getVarOriginalIndex(..)`, which is null for a var created after
//! version splitting; Java's `HashMap` then holds a `null` key, so the key type
//! is `Option<i32>` (hash 0, like Java's null key). For the same reason the
//! clash-renaming maps key statements by `Option<Statement>` (a root's parent)
//! and hold `Option<String>` names (an unnamed parameter).
#![allow(dead_code)]

use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::array_helper::ArrayHelper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_STATIC, ACC_SYNTHETIC};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::Type as DoType;
use crate::expr_processor::ExprProcessor;
use crate::exprent::{Exprent, ExprentType};
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::generic_type::GenericType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::method_descriptor::MethodDescriptor;
use crate::pair::Pair;
use crate::root_statement::RootStatement;
use crate::rt::{
    objects_equals, objects_hash, rethrow, try_catch, JHashMap, JHashSet, JLinkedHashMap, JList,
    JavaEq, JavaHash, Shared,
};
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::stack_vars_processor::StackVarsProcessor;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::statement_iterator::StatementIterator;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
use crate::struct_method_parameters_attribute::Entry as MethodParameter;
use crate::switch_statement::SwitchStatement;
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_names_collector::VarNamesCollector;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object, jthrow};

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

type MergeMap = JHashMap<Option<i32>, VarVersionPair>;
type VarDefs = JHashMap<Option<Statement>, JHashSet<VarInMethod>>;
type NameMap = JHashMap<VarInMethod, Option<String>>;

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

        let vc = varproc.get_var_names_collector();

        let thisvar = !mt.has_modifier(ACC_STATIC);

        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());

        let mut paramcount = 0;
        if thisvar {
            paramcount = 1;
        }
        paramcount += md.params.len() as i32;

        let mut method_parameters: Option<Vec<MethodParameter>> = None;
        if DecompilerContext::get_option(IFernflowerPreferences::USE_METHOD_PARAMETERS) {
            let attr = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS);
            if let Some(attr) = attr {
                method_parameters = Some(attr.get_entries());
            }
        }

        let mut null_name_used = false;
        let mut varindex = 0;
        let mut param_index = 0usize;
        for i in 0..paramcount {
            helper.with_mut(|d| d.impl_def_vars.add(varindex));
            let vpp = VarVersionPair::new(varindex, 0);
            if varindex != 0 || !thisvar {
                match &method_parameters {
                    Some(params) if param_index < params.len() => {
                        let name = free_name_nullable(
                            vc,
                            params[param_index].my_name.as_deref(),
                            &mut null_name_used,
                        );
                        if let Some(name) = name {
                            varproc.set_var_name(vpp, name);
                        }
                        param_index += 1;
                    }
                    _ => {
                        varproc.set_var_name(vpp, vc.get_free_name_index(varindex));
                    }
                }
            }

            if thisvar {
                if i == 0 {
                    varindex += 1;
                } else {
                    varindex += md.params[(i - 1) as usize].stack_size;
                }
            } else {
                varindex += md.params[i as usize].stack_size;
            }
        }

        if thisvar {
            let current_class = DecompilerContext::get_current_class()
                .expect("NullPointerException: current class");

            varproc
                .get_this_vars()
                .borrow_mut()
                .put(VarVersionPair::new(0, 0), current_class.qualified_name());
            varproc.set_var_name(VarVersionPair::new(0, 0), "this");
            vc.add_name("this");
        }

        helper.merge_vars(root);

        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(root.0);

        while let Some(st) = stack.pop_front() {
            let lst_vars = st.get_implicitly_defined_vars();

            if let Some(lst_vars) = lst_vars {
                for var in lst_vars {
                    let index = var.get_index();
                    helper.with_mut(|d| d.impl_def_vars.add(index));
                    varproc
                        .set_var_name(VarVersionPair::from_var(var), vc.get_free_name_index(index));
                    var.set_definition(true);
                }
            }

            let stats = st.get_stats().borrow().to_vec();
            stack.extend(stats);
        }

        helper.init_statement(root.0);

        let mut flatten_helper = FlattenStatementsHelper::new();
        let graph = flatten_helper.build_direct_graph(root);
        ValidationHelper::validate_vars(
            graph,
            root,
            |var| !var.get_var_type().ptr_eq(&VarType::VARTYPE_UNKNOWN()),
            "Var type not set!",
        );

        helper
    }

    fn varproc(self) -> VarProcessor {
        self.with(|d| d.varproc)
    }

    fn root(self) -> RootStatement {
        self.with(|d| d.root)
    }

    fn mt(self) -> StructMethod {
        self.with(|d| d.mt)
    }

    pub fn set_var_definitions(self) {
        let (varproc, root, mt) = self.with(|d| (d.varproc, d.root, d.mt));
        let vc = varproc.get_var_names_collector();

        let entries = self.with(|d| d.map_var_def_statements.entry_vec());
        for (index, stat) in entries {
            let mut stat = stat;

            if self.with(|d| d.impl_def_vars.contains(&index)) {
                continue;
            }

            varproc.set_var_name(VarVersionPair::new(index, 0), vc.get_free_name_index(index));

            if let Some(dstat) = stat.try_as_do() {
                if dstat.get_looptype() == DoType::For {
                    let init = dstat.get_init_exprent();
                    if init.is_some_and(|init| self.set_definition(init, index)) {
                        continue;
                    } else {
                        let lst_special = vec![
                            dstat
                                .get_condition_exprent()
                                .expect("NullPointerException: for condition"),
                            dstat
                                .get_inc_exprent()
                                .expect("NullPointerException: for increment"),
                        ];
                        for var in Self::get_all_vars(&lst_special) {
                            if var.get_index() == index {
                                stat = stat.get_parent().expect("NullPointerException: parent");
                                break;
                            }
                        }
                    }
                } else if dstat.get_looptype() == DoType::ForEach {
                    if let Some(var) = dstat.get_init_exprent().and_then(|e| e.try_as_var()) {
                        if var.get_index() == index {
                            var.set_definition(true);
                            continue;
                        }
                    }
                }
            }

            let first = self.find_first_block(stat, index);

            let lst: JList<Exprent> = match first {
                None => stat.get_var_definitions(),
                Some(first) => match first.get_exprents() {
                    None => first.get_var_definitions(),
                    Some(exprents) => exprents,
                },
            };

            let mut defset = false;

            let mut addindex = 0usize;
            for expr in lst.iter() {
                if self.set_definition(expr, index) {
                    defset = true;
                    break;
                } else {
                    let mut foundvar = false;
                    for exp in expr.get_all_exprents_rec(true) {
                        if exp.try_as_var().is_some_and(|v| v.get_index() == index) {
                            foundvar = true;
                            break;
                        }
                    }
                    if foundvar {
                        break;
                    }
                }
                addindex += 1;
            }

            if !defset {
                let var = VarExprent::new(
                    index,
                    varproc.get_var_type(VarVersionPair::new(index, 0)),
                    Some(varproc),
                );
                var.set_definition(true);

                let lvt = self.find_lvt(index, stat);
                if lvt.is_some() {
                    var.set_lvt(lvt);
                }

                lst.add_at(addindex, var.0);
            }
        }

        self.merge_vars(root);
        self.propagate_lvts(root.0);
        self.set_non_final(root.0, &mut JHashSet::new());
        self.remap_clashing_names(root.0, mt);
    }

    /// Java: findLVT(int, Statement)
    fn find_lvt(self, index: i32, stat: Statement) -> Option<LocalVariable> {
        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    let lvt = self.find_lvt(index, st);
                    if lvt.is_some() {
                        return lvt;
                    }
                }

                for exp in stat.get_stat_exprents() {
                    let lvt = self.find_lvt_exprent(index, exp);
                    if lvt.is_some() {
                        return lvt;
                    }
                }
            }
            Some(exprents) => {
                for exp in exprents.iter() {
                    let lvt = self.find_lvt_exprent(index, exp);
                    if lvt.is_some() {
                        return lvt;
                    }
                }
            }
        }
        None
    }

    /// Java: findLVT(int, Exprent)
    fn find_lvt_exprent(self, index: i32, exp: Exprent) -> Option<LocalVariable> {
        for e in exp.get_all_exprents_rec(false) {
            let lvt = self.find_lvt_exprent(index, e);
            if lvt.is_some() {
                return lvt;
            }
        }

        let var = exp.try_as_var()?;
        if var.get_index() == index {
            var.get_lvt()
        } else {
            None
        }
    }

    fn find_first_block(self, stat: Statement, varindex: i32) -> Option<Statement> {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(stat);

        while let Some(st) = stack.pop_front() {
            let contains = stack.is_empty() || {
                let id = st.id();
                self.with(|d| {
                    d.map_statement_vars
                        .get(&id)
                        .expect("NullPointerException: mapStatementVars")
                        .contains(&varindex)
                })
            };

            if contains {
                if st.is_labeled() && !stack.is_empty() {
                    return Some(st);
                }

                if st.get_exprents().is_some() {
                    return Some(st);
                } else {
                    stack.clear();

                    match st.type_() {
                        StatementType::Sequence => {
                            let stats = st.get_stats().borrow().to_vec();
                            stack.extend(stats);
                        }
                        StatementType::If
                        | StatementType::Root
                        | StatementType::Switch
                        | StatementType::Synchronized => {
                            stack.push_back(st.get_first().expect("NullPointerException: first"));
                        }
                        _ => return Some(st),
                    }
                }
            }
        }

        None
    }

    /// Returns the elements of the set it records for `stat`, in that set's
    /// iteration order (the Java caller only iterates the returned set).
    fn init_statement(self, stat: Statement) -> Vec<i32> {
        let mut map_count: JHashMap<i32, i32> = JHashMap::new();

        let condlst: Vec<VarExprent> = match stat.get_exprents() {
            None => {
                let mut child_vars: Vec<i32> = Vec::new();
                let mut curr_vars: Vec<Exprent> = Vec::new();

                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    child_vars.extend(self.init_statement(st));

                    if let Some(dost) = st.try_as_do() {
                        let looptype = dost.get_looptype();
                        if looptype != DoType::For
                            && looptype != DoType::ForEach
                            && looptype != DoType::Infinite
                        {
                            curr_vars.push(
                                dost.get_condition_exprent()
                                    .expect("NullPointerException: loop condition"),
                            );
                        }
                    } else if let Some(fin) = st.try_as_catch_all() {
                        if fin.is_finally() {
                            if let Some(monitor) = fin.get_monitor() {
                                curr_vars.push(monitor.0);
                            }
                        }
                    }
                }

                curr_vars.extend(stat.get_stat_exprents());

                for index in child_vars {
                    let count = map_count.get(&index).copied().unwrap_or(0);
                    map_count.put(index, count + 1);
                }

                Self::get_all_vars(&curr_vars)
            }
            Some(exprents) => Self::get_all_vars(&exprents.to_vec()),
        };

        for var in condlst {
            map_count.put(var.get_index(), 2);
        }

        let set: JHashSet<i32> = JHashSet::from_collection(map_count.key_vec());

        let id = stat.id();
        self.with_mut(|d| {
            for (key, value) in map_count.iter() {
                if *value > 1 {
                    d.map_var_def_statements.put(*key, stat);
                }
            }
        });

        let result = set.to_vec();
        self.with_mut(|d| d.map_statement_vars.put(id, set));

        result
    }

    fn get_all_vars(lst: &[Exprent]) -> Vec<VarExprent> {
        let mut res = Vec::new();
        let mut list_temp: Vec<Exprent> = Vec::new();

        for &expr in lst {
            list_temp.extend(expr.get_all_exprents_rec(true));
            list_temp.push(expr);
        }

        for exprent in list_temp {
            if let Some(var) = exprent.try_as_var() {
                res.push(var);
            }
        }

        res
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

    fn populate_type_bounds(self, _proc: VarProcessor, _stat: Statement) {
        let type_processor = self
            .varproc()
            .get_var_versions()
            .expect("NullPointerException: varVersions")
            .get_type_processor();
        let map_exprent_max_types = type_processor.get_upper_bounds();
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(self.root().0);

        while let Some(st) = stack.pop_front() {
            if let Some(exprents) = st.get_exprents() {
                let mut exps: VecDeque<Exprent> = exprents.to_vec().into();
                while let Some(exp) = exps.pop_front() {
                    match exp.get_type() {
                        ExprentType::Invocation | ExprentType::Field | ExprentType::Exit => {
                            let mut instance: Option<Exprent> = None;
                            let mut target: Option<String> = None;
                            if let Some(inv) = exp.try_as_invocation() {
                                instance = inv.get_instance();
                                target = Some(inv.get_classname());
                            } else if let Some(field) = exp.try_as_field() {
                                instance = field.get_instance();
                                target = Some(field.get_classname());
                            } else if let Some(exit) = exp.try_as_exit() {
                                if exit.get_exit_type() == crate::exit_exprent::Type::Return {
                                    instance = exit.get_value();
                                    target = exit
                                        .get_ret_type()
                                        .expect("NullPointerException: retType")
                                        .value
                                        .as_deref()
                                        .map(str::to_string);
                                }
                            }

                            if target.as_deref() == Some("java/lang/Object") {
                                continue;
                            }

                            if let Some(var) = instance.and_then(|i| i.try_as_var()) {
                                let key = var.get_var_version_pair();
                                let new_type = VarType::new_value(
                                    CodeType::Object,
                                    0,
                                    target.map(Rc::<str>::from),
                                );
                                let old_max = map_exprent_max_types.borrow().get_cloned(&key);

                                if !new_type.equals_opt(old_max.as_ref()) {
                                    match &old_max {
                                        Some(old_max) if old_max.type_ == CodeType::Object => {
                                            if instance_of(
                                                new_type.value.as_deref(),
                                                old_max.value.as_deref(),
                                            ) {
                                                map_exprent_max_types
                                                    .borrow_mut()
                                                    .put(key, new_type);
                                            }
                                        }
                                        _ => {
                                            map_exprent_max_types.borrow_mut().put(key, new_type);
                                        }
                                    }
                                }
                            }
                        }
                        _ => exps.extend(exp.get_all_exprents()),
                    }
                }
            }

            let stats = st.get_stats().borrow().to_vec();
            stack.extend(stats);
        }
    }

    /// `null` when the SSAU pass fails.
    fn get_var_exprent_sources(self) -> Option<JHashMap<VarID, JHashSet<VarID>>> {
        let (root, mt) = self.with(|d| (d.root, d.mt));
        let mut ssau = SSAUConstructorSparseEx::new();
        if let Err(caught) = try_catch(|| ssau.split_variables(root, mt)) {
            if caught.is("NullPointerException") || caught.exception.is_none() {
                StackVarsProcessor::set_versions_to_null(root.0);
                return None;
            }
            rethrow(caught);
        }

        let mut lookup: JHashMap<VarVersionPair, VarID> = JHashMap::new();
        Self::find_all_var_exprents(root.0, &mut lookup);

        let mut sources: JHashMap<VarID, JHashSet<VarID>> = JHashMap::new();
        for node in ssau.get_ssu_versions().nodes() {
            let Some(target) = lookup.get_cloned(&node.as_pair()) else {
                continue;
            };

            let mut source_vars: JHashSet<VarID> = JHashSet::new();

            for predecessor in node.get_predecessors() {
                if let Some(source) = lookup.get_cloned(&predecessor.as_pair()) {
                    source_vars.add(source);
                }
            }

            if let Some(phantom) = node.phantom_node() {
                if let Some(source) = lookup.get_cloned(&phantom.as_pair()) {
                    source_vars.add(source);
                }
            }

            if !source_vars.is_empty() {
                sources.put(target, source_vars);
            }
        }

        StackVarsProcessor::set_versions_to_null(root.0);

        Some(sources)
    }

    fn find_all_var_exprents(stat: Statement, lookup: &mut JHashMap<VarVersionPair, VarID>) {
        for exprent in stat.get_var_definitions().iter() {
            if let Some(var_exprent) = exprent.try_as_var() {
                lookup.put(
                    VarVersionPair::from_var(var_exprent),
                    VarID::new(var_exprent),
                );
            }
        }
        if let Some(lst) = stat.get_exprents() {
            for exprent in lst.iter() {
                for exp in exprent.get_all_exprents_rec_self(true, true) {
                    if let Some(var_exprent) = exp.try_as_var() {
                        lookup.put(
                            VarVersionPair::from_var(var_exprent),
                            VarID::new(var_exprent),
                        );
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for sub_stat in stats {
            Self::find_all_var_exprents(sub_stat, lookup);
        }
    }

    fn compare_var_exprent_sources(
        self,
        old_sources: &JHashMap<VarID, JHashSet<VarID>>,
        new_sources: Option<&JHashMap<VarID, JHashSet<VarID>>>,
    ) {
        let Some(new_sources) = new_sources else {
            return;
        };
        let root = self.root();

        for (key, old_set) in old_sources.iter() {
            let new_set = new_sources.get_cloned(key);

            if !objects_equals(&Some(old_set.clone()), &new_set) {
                root.add_comment(&format!(
                    "$VF: Variable merging failed for merge {}. Code has semantic differences!",
                    key.var.to_string()
                ));
            }
        }

        for new_var in new_sources.keys() {
            if !old_sources.contains_key(new_var) {
                root.add_comment(&format!(
                    "$VF: Variable merging added a var? {}",
                    new_var.var.to_string()
                ));
            }
        }
    }

    /// Java: mergeVars(RootStatement)
    fn merge_vars(self, stat: RootStatement) -> Option<VPPEntry> {
        let (varproc, mt) = self.with(|d| (d.varproc, d.mt));
        let mut parent: MergeMap = JHashMap::new();
        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());

        let mut index = 0;
        if !mt.has_modifier(ACC_STATIC) {
            parent.put(Some(index), VarVersionPair::new(index, 0));
            index += 1;
        }

        for var in md.params.iter() {
            parent.put(Some(index), VarVersionPair::new(index, 0));
            index += var.stack_size;
        }

        self.populate_type_bounds(varproc, stat.0);

        let mut sources: Option<JHashMap<VarID, JHashSet<VarID>>> = None;
        if DecompilerContext::get_option(IFernflowerPreferences::VERIFY_PRE_POST_VARIABLE_MERGES) {
            sources = self.get_var_exprent_sources();
        }

        let mut denylist: JHashMap<VarVersionPair, VarVersionPair> = JHashMap::new();
        let mut remap = self.merge_vars_rec(stat.0, &parent, &mut JHashMap::new(), &denylist);
        while let Some(entry) = remap {
            if !self.remap_var(stat.0, entry.get_key(), entry.get_value()) {
                denylist.put(entry.get_key(), entry.get_value());
            }

            remap = self.merge_vars_rec(stat.0, &parent, &mut JHashMap::new(), &denylist);
        }

        if let Some(sources) = &sources {
            let new_sources = self.get_var_exprent_sources();
            self.compare_var_exprent_sources(sources, new_sources.as_ref());
        }
        None
    }

    fn get_sequential_objects(stat: Statement) -> Vec<SequentialObject> {
        let mut lst = Vec::new();
        lst.extend(
            stat.get_stat_exprents()
                .into_iter()
                .map(SequentialObject::Exprent),
        );
        let stats = stat.get_stats().borrow().to_vec();
        lst.extend(stats.into_iter().map(SequentialObject::Statement));
        lst
    }

    /// Java: mergeVars(Statement, Map, Map, Map)
    fn merge_vars_rec(
        self,
        stat: Statement,
        parent: &MergeMap,
        leaked: &mut MergeMap,
        denylist: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> Option<VPPEntry> {
        let varproc = self.varproc();
        let mut this_vars: MergeMap = JHashMap::new();
        if !parent.is_empty() {
            this_vars.put_all(parent.entry_vec());
        }

        let var_definitions = stat.get_var_definitions();
        if !var_definitions.is_empty() {
            let mut x = 0;
            while x < var_definitions.len() {
                let exp = var_definitions.get(x);
                if let Some(var) = exp.try_as_var() {
                    let index = varproc.get_var_original_index(var.get_index());
                    if index.is_some() {
                        if this_vars.contains_key(&index) {
                            var_definitions.remove_at(x);
                            return Some(VPPEntry::new(
                                var,
                                this_vars.get_cloned(&index).expect("contained"),
                            ));
                        }
                        this_vars.put(index, VarVersionPair::from_var(var));
                        leaked.put(index, VarVersionPair::from_var(var));
                    } else {
                        let root = stat.get_top_parent();

                        root.add_comment_error("$VF: One or more variable merging failures!", true);
                    }
                }
                x += 1;
            }
        }

        let scoped = matches!(
            stat.type_(),
            StatementType::BasicBlock
                | StatementType::General
                | StatementType::Root
                | StatementType::Sequence
        );

        match stat.get_exprents() {
            None => {
                let objs = Self::get_sequential_objects(stat);
                for (i, &obj) in objs.iter().enumerate() {
                    match obj {
                        SequentialObject::Statement(st) => {
                            let mut leaked_n: MergeMap = JHashMap::new();
                            let remap =
                                self.merge_vars_rec(st, &this_vars, &mut leaked_n, denylist);

                            if remap.is_some() {
                                return remap;
                            }

                            if !leaked_n.is_empty() {
                                if let Some(ifst) = stat.try_as_if() {
                                    if Some(st) == ifst.get_ifstat()
                                        || Some(st) == ifst.get_elsestat()
                                    {
                                        leaked_n.clear();
                                    } else if Some(st) == ifst.get_first() {
                                        leaked.put_all(leaked_n.entry_vec());
                                    }
                                } else if stat.is_switch() || stat.is_synchronized() {
                                    if Some(st) == stat.get_first() {
                                        leaked.put_all(leaked_n.entry_vec());
                                    } else {
                                        leaked_n.clear();
                                    }
                                } else if stat.is_catch() || stat.is_catch_all() {
                                    leaked_n.clear();
                                }
                                this_vars.put_all(leaked_n.entry_vec());
                            }
                        }
                        SequentialObject::Exprent(exp) => {
                            let ret = self.process_exprent(
                                exp,
                                &mut this_vars,
                                if scoped { Some(&mut *leaked) } else { None },
                                denylist,
                            );
                            if let Some(ret) = ret {
                                if Self::is_var_read_first(ret.get_value(), stat, i as i32 + 1, &[])
                                {
                                    let t1 = varproc
                                        .get_var_type(ret.get_key())
                                        .expect("NullPointerException: var type");
                                    let t2 = varproc
                                        .get_var_type(ret.get_value())
                                        .expect("NullPointerException: var type");

                                    if t1.higher_equal_in_lattice_than(&t2)
                                        || t2.higher_equal_in_lattice_than(&t1)
                                    {
                                        return Some(ret);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(exps) => {
                let mut i = 0;
                while i < exps.len() {
                    let exp = exps.get(i);
                    let ret = self.process_exprent(
                        exp,
                        &mut this_vars,
                        if scoped { Some(&mut *leaked) } else { None },
                        denylist,
                    );
                    if let Some(ret) = ret {
                        if !Self::is_var_read_first(ret.get_value(), stat, i as i32 + 1, &[]) {
                            let t1 = varproc.get_var_type(ret.get_key());
                            let t2 = varproc.get_var_type(ret.get_value());

                            ValidationHelper::assert_true(
                                t1.is_some() && t2.is_some(),
                                "Var types not processed! Is the graph broken?",
                            );
                            let t1 = t1.expect("NullPointerException: var type");
                            let t2 = t2.expect("NullPointerException: var type");

                            if t1.higher_equal_in_lattice_than(&t2)
                                || t2.higher_equal_in_lattice_than(&t1)
                            {
                                let mut ok = true;
                                if DecompilerContext::get_option(
                                    IFernflowerPreferences::VERIFY_VARIABLE_MERGES,
                                ) {
                                    if let Some(assign) = exp.try_as_assignment() {
                                        if let Some(var) = assign.get_left().try_as_var() {
                                            if var.get_index() == ret.get_key().var {
                                                let parent_stat = Self::identify_parent(stat)
                                                    .expect("NullPointerException: parent");
                                                if assign.get_right().contains_var(ret.get_value())
                                                {
                                                    if Self::is_var_read_remote(
                                                        parent_stat,
                                                        ret.get_key(),
                                                        false,
                                                        &[stat],
                                                    ) {
                                                        ok = false;
                                                    }
                                                } else if Self::is_var_read_remote(
                                                    parent_stat,
                                                    ret.get_key(),
                                                    true,
                                                    &[stat],
                                                ) {
                                                    ok = false;
                                                }
                                            }
                                        }
                                    }
                                }

                                if ok {
                                    return Some(ret);
                                }
                            }
                        }
                    }
                    i += 1;
                }
            }
        }
        None
    }

    fn identify_parent(stat: Statement) -> Option<Statement> {
        let parent = stat.get_parent();

        if let Some(p) = parent {
            if (p.is_if() || p.is_switch()) && p.get_basichead().0 == stat {
                return p.get_parent();
            }
        }

        parent
    }

    fn is_var_read_remote(
        stat: Statement,
        var: VarVersionPair,
        check_assign: bool,
        filter: &[Statement],
    ) -> bool {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Self::is_var_read_remote(st, var, check_assign, filter) {
                return true;
            }
        }

        if ArrayHelper::contains_by_ref(filter, &stat) {
            return false;
        }

        if stat.is_basic_block() {
            let exprents = stat
                .get_exprents()
                .expect("NullPointerException: exprents")
                .to_vec();
            if check_assign {
                for ex in exprents {
                    for e in ex.get_all_exprents_rec_self(true, true) {
                        if let Some(assign) = e.try_as_assignment() {
                            if let Some(var2) = assign.get_left().try_as_var() {
                                if var2.get_index() == var.var {
                                    return true;
                                }
                            }
                        }

                        if let Some(func) = e.try_as_function() {
                            if func.get_func_type().is_ppmm() {
                                if let Some(var2) = func.get_lst_operands().get(0).try_as_var() {
                                    if var2.get_index() == var.var {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                for ex in exprents {
                    if ex.contains_var(var) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// `leaked` is the caller's `scoped` map: `null`, or an alias of its own `leaked`.
    fn process_exprent(
        self,
        exp: Exprent,
        this_vars: &mut MergeMap,
        leaked: Option<&mut MergeMap>,
        denylist: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> Option<VPPEntry> {
        let var = if let Some(ass) = exp.try_as_assignment() {
            ass.get_left().try_as_var()?
        } else {
            exp.try_as_var()?
        };

        if !var.is_definition() {
            return None;
        }

        let index = self.varproc().get_var_original_index(var.get_index());
        if let Some(new) = this_vars.get_cloned(&index) {
            let old = VarVersionPair::from_var(var);
            let deny = denylist.get_cloned(&old);
            if deny.is_none_or(|deny| !deny.equals(&new)) {
                return Some(VPPEntry::new(var, new));
            }
        }
        this_vars.put(index, VarVersionPair::from_var(var));

        if let Some(leaked) = leaked {
            leaked.put(index, VarVersionPair::from_var(var));
        }

        None
    }

    /// Java: remapVar(Statement, VarVersionPair, VarVersionPair)
    fn remap_var(self, stat: Statement, from: VarVersionPair, to: VarVersionPair) -> bool {
        if from.equals(&to) {
            jthrow!(IllegalStateException; "Trying to remap var version {} in statement {} to itself!", from, stat);
        }
        let mut success = false;
        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    success |= self.remap_var(st, from, to);
                }

                for exp in stat.get_stat_exprents() {
                    if self.remap_var_exprent(Some(exp), from, to) {
                        success = true;
                    }
                }
            }
            Some(exprents) => {
                let mut remapped = false;
                let mut x: isize = 0;
                while (x as usize) < exprents.len() {
                    let exp = exprents.get(x as usize);
                    if self.remap_var_exprent(Some(exp), from, to) {
                        remapped = true;
                        if let Some(var) = exp.try_as_var() {
                            if !var.is_definition() {
                                exprents.remove_at(x as usize);
                                x -= 1;
                            }
                        }
                    }
                    x += 1;
                }
                success |= remapped;
            }
        }

        if success {
            let defs = stat.get_var_definitions();
            let mut x = 0;
            while x < defs.len() {
                let exp = defs.get(x);
                if let Some(var) = exp.try_as_var() {
                    if from.equals(&var.get_var_version_pair()) {
                        defs.remove_at(x);
                        continue;
                    } else if to.var == var.get_index() && to.version == var.get_version() {
                        if let Some(merged) = self.get_merged_type(from, to) {
                            var.set_var_type(merged);
                        }
                    }
                }
                x += 1;
            }
        }

        success
    }

    /// Java: remapVar(Exprent, VarVersionPair, VarVersionPair)
    fn remap_var_exprent(
        self,
        exprent: Option<Exprent>,
        from: VarVersionPair,
        to: VarVersionPair,
    ) -> bool {
        let Some(exprent) = exprent else {
            return false;
        };
        let mut lst = exprent.get_all_exprents_rec(true);
        lst.push(exprent);

        let mut remapped = false;

        for expr in lst {
            if let Some(ass) = expr.try_as_assignment() {
                let (left, right) = (ass.get_left(), ass.get_right());
                if let (Some(left), Some(right)) = (left.try_as_var(), right.try_as_const()) {
                    let left = VarVersionPair::from_var(left);
                    if !left.equals(&from) && !left.equals(&to) {
                        continue;
                    }

                    if right.get_const_type().ptr_eq(&VarType::VARTYPE_NULL()) {
                        continue;
                    }
                    let Some(merged) = self.get_merged_type(from, to) else {
                        continue;
                    };

                    let type_ = right.get_const_type();

                    if merged.higher_equal_in_lattice_than(&type_)
                        && Self::can_const_type_merge(&merged)
                    {
                        right.set_const_type(merged);
                    }
                }
            } else if let Some(var) = expr.try_as_var() {
                let old = VarVersionPair::from_var(var);
                if !old.equals(&from) {
                    continue;
                }
                let Some(merged) = self.get_merged_type(from, to) else {
                    continue;
                };

                var.set_index(to.var);
                var.set_version(to.version);
                var.set_var_type(merged.clone());
                if var.is_definition() {
                    var.set_definition(false);
                }
                self.varproc().set_var_type(to, &merged);
                remapped = true;
            }
        }
        remapped
    }

    fn can_const_type_merge(type_: &VarType) -> bool {
        if type_.type_family == TypeFamily::Object {
            return type_.ptr_eq(&VarType::VARTYPE_STRING())
                || type_.ptr_eq(&VarType::VARTYPE_CLASS())
                || type_.ptr_eq(&VarType::VARTYPE_NULL());
        }

        true
    }

    /// Java: getMergedType(VarVersionPair, VarVersionPair)
    fn get_merged_type(self, from: VarVersionPair, to: VarVersionPair) -> Option<VarType> {
        let type_processor = self
            .varproc()
            .get_var_versions()
            .expect("NullPointerException: varVersions")
            .get_type_processor();
        let min_types = type_processor.get_lower_bounds();
        let max_types = type_processor.get_upper_bounds();

        let (from_min, to_min) = {
            let min = min_types.borrow();
            (min.get_cloned(&from), min.get_cloned(&to))
        };
        let (from_max, to_max) = {
            let max = max_types.borrow();
            (max.get_cloned(&from), max.get_cloned(&to))
        };

        self.get_merged_type_bounds(
            from_min.as_ref(),
            to_min.as_ref(),
            from_max.as_ref(),
            to_max.as_ref(),
        )
    }

    /// Java: getMergedType(VarType, VarType, VarType, VarType)
    fn get_merged_type_bounds(
        self,
        from_min: Option<&VarType>,
        to_min: Option<&VarType>,
        from_max: Option<&VarType>,
        to_max: Option<&VarType>,
    ) -> Option<VarType> {
        if let Some(from_min) = from_min {
            if from_min.equals_opt(to_min) {
                return Some(from_min.clone());
            }
        }
        let type_ = match (from_min, to_min) {
            (None, _) => to_min.cloned(),
            (Some(from_min), None) => Some(from_min.clone()),
            (Some(from_min), Some(to_min)) => VarType::join(from_min, to_min),
        };
        let (Some(type_), Some(from_min), Some(to_min)) = (type_, from_min, to_min) else {
            return None;
        };
        if type_.type_ == CodeType::Object {
            if let Some(to_max) = to_max {
                if let Some(from_max) = from_max {
                    if instance_of(from_max.value.as_deref(), to_max.value.as_deref()) {
                        return Some(from_max.clone());
                    }
                } else if instance_of(from_min.value.as_deref(), to_max.value.as_deref()) {
                    return Some(from_min.clone());
                }
            } else if let Some(from_max) = from_max {
                if instance_of(from_max.value.as_deref(), to_min.value.as_deref()) {
                    return Some(from_max.clone());
                }
            } else {
                if instance_of(to_min.value.as_deref(), from_min.value.as_deref()) {
                    return Some(to_min.clone());
                }

                if instance_of(from_min.value.as_deref(), to_min.value.as_deref()) {
                    return Some(to_min.clone());
                }
            }

            None
        } else {
            if !from_min.higher_in_lattice_than(to_min) {
                return None;
            }

            Some(type_)
        }
    }

    fn propagate_lvts(self, stat: Statement) {
        let (varproc, mt, root) = self.with(|d| (d.varproc, d.mt, d.root));
        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
        let mut types: JLinkedHashMap<VarVersionPair, VarInfo> = JLinkedHashMap::new();

        if varproc.has_lvt() {
            let mut index = 0;
            if !mt.has_modifier(ACC_STATIC) {
                let lvt = varproc.get_candidates(index);
                if let Some(first) = lvt.and_then(|lvt| lvt.first().cloned()) {
                    types.put(
                        VarVersionPair::new(index, 0),
                        VarInfo::new(Some(first), None),
                    );
                }
                index += 1;
            }

            for var in md.params.iter() {
                let lvt = varproc.get_candidates(index);
                if let Some(first) = lvt.and_then(|lvt| lvt.first().cloned()) {
                    types.put(
                        VarVersionPair::new(index, 0),
                        VarInfo::new(Some(first), None),
                    );
                }
                index += var.stack_size;
            }
        }

        self.find_types(Some(stat), &mut types);

        let mut type_names: JLinkedHashMap<VarVersionPair, Pair<Option<VarType>, Option<String>>> =
            JLinkedHashMap::new();
        for (key, info) in types.iter() {
            type_names.put(*key, Pair::of(info.get_type(), Some(info.get_cast())));
        }

        let renames = mt.get_variable_namer().rename(&type_names);

        let mut methods: JHashSet<Option<StructMethod>> = JHashSet::new();

        StatementIterator::iterate(root.0, &mut |exprent: Exprent| {
            if let Some(new) = exprent.try_as_new() {
                if new.is_anonymous() {
                    if let Some(child) = root_class_node(&new.get_new_type()) {
                        let (lambda_information, class_struct) =
                            child.with(|d| (d.lambda_information.clone(), d.class_struct));
                        if new.is_lambda() {
                            let li = lambda_information
                                .expect("NullPointerException: lambdaInformation");
                            if !li.is_method_reference {
                                methods.add(class_struct.get_method(
                                    &li.content_method_name,
                                    &li.content_method_descriptor,
                                ));
                            }
                        } else {
                            methods
                                .add_all(class_struct.get_methods().to_vec().into_iter().map(Some));
                        }
                    }
                }
            }
            0
        });

        let this_key = InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor());
        let owner = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get_cloned(&mt.get_class_qualified_name())
            .expect("NullPointerException: class node");
        let nested_nodes = owner.with(|d| d.nested.to_vec());
        for nested in nested_nodes {
            let (type_, enclosing_method, class_struct) =
                nested.with(|d| (d.type_, d.enclosing_method.clone(), d.class_struct));
            if type_ == ClassNodeType::Local
                && enclosing_method.as_deref() == Some(this_key.as_str())
            {
                methods.add_all(class_struct.get_methods().to_vec().into_iter().map(Some));
            }
        }

        for meth in methods.to_vec() {
            let namer = meth
                .expect("NullPointerException: method")
                .get_variable_namer();
            namer.add_parent_context(mt.get_variable_namer());
        }

        let mut lvts: JHashMap<VarVersionPair, LocalVariable> = JHashMap::new();

        for (idx, info) in types.entry_vec() {
            if idx.var == 0 && !mt.has_modifier(ACC_STATIC) {
                continue;
            }
            let lvt = info.get_lvt();
            let rename = renames.as_ref().and_then(|r| r.get_cloned(&idx));

            if let Some(rename) = &rename {
                varproc.set_var_name(idx, rename.clone());
            }

            if let Some(mut lvt) = lvt {
                if let Some(rename) = &rename {
                    lvt = lvt.rename(rename.clone());
                }
                varproc.set_var_lvt(idx, lvt.clone());
                lvts.put(idx, lvt);
            }
        }

        self.apply_types(Some(stat), &lvts);
    }

    /// Java: findTypes(Statement, Map)
    fn find_types(
        self,
        stat: Option<Statement>,
        types: &mut JLinkedHashMap<VarVersionPair, VarInfo>,
    ) {
        let Some(stat) = stat else {
            return;
        };

        for exp in stat.get_var_definitions().iter() {
            self.find_types_exprent(exp, types);
        }

        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    self.find_types(Some(st), types);
                }

                for exp in stat.get_stat_exprents() {
                    self.find_types_exprent(exp, types);
                }
            }
            Some(exprents) => {
                for exp in exprents.iter() {
                    self.find_types_exprent(exp, types);
                }
            }
        }
    }

    /// Java: findTypes(Exprent, Map)
    fn find_types_exprent(self, exp: Exprent, types: &mut JLinkedHashMap<VarVersionPair, VarInfo>) {
        let mut lst = exp.get_all_exprents_rec(true);
        lst.push(exp);

        for exprent in lst {
            if let Some(var) = exprent.try_as_var() {
                let ver = VarVersionPair::from_var(var);
                if var.is_definition() {
                    types.put(ver, VarInfo::new(var.get_lvt(), Some(var.get_var_type())));
                } else {
                    let existing = match types.get_cloned(&ver) {
                        None => VarInfo::new(var.get_lvt(), Some(var.get_var_type())),
                        Some(info) if info.get_lvt().is_none() && var.get_lvt().is_some() => {
                            VarInfo::new(var.get_lvt(), info.get_type())
                        }
                        Some(info) => info,
                    };

                    types.put(ver, existing);
                }
            }
        }
    }

    /// Java: applyTypes(Statement, Map)
    fn apply_types(self, stat: Option<Statement>, types: &JHashMap<VarVersionPair, LocalVariable>) {
        let Some(stat) = stat else {
            return;
        };
        if types.is_empty() {
            return;
        }

        for exp in stat.get_var_definitions().iter() {
            self.apply_types_exprent(Some(exp), types);
        }

        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    self.apply_types(Some(st), types);
                }

                for exp in stat.get_stat_exprents() {
                    self.apply_types_exprent(Some(exp), types);
                }
            }
            Some(exprents) => {
                for exp in exprents.iter() {
                    self.apply_types_exprent(Some(exp), types);
                }
            }
        }
    }

    /// Java: applyTypes(Exprent, Map)
    fn apply_types_exprent(
        self,
        exprent: Option<Exprent>,
        types: &JHashMap<VarVersionPair, LocalVariable>,
    ) {
        let Some(exprent) = exprent else {
            return;
        };
        let mut lst = exprent.get_all_exprents_rec(true);
        lst.push(exprent);

        for expr in lst {
            if let Some(var) = expr.try_as_var() {
                let lvt = types.get_cloned(&VarVersionPair::from_var(var));
                if lvt.is_some() {
                    var.set_lvt(lvt);
                }
            }
        }
    }

    /// Java: isVarReadFirst(VarVersionPair, Statement, int, VarExprent...)
    fn is_var_read_first(
        var: VarVersionPair,
        stat: Statement,
        index: i32,
        allowlist: &[VarExprent],
    ) -> bool {
        match stat.get_exprents() {
            None => {
                let objs = Self::get_sequential_objects(stat);
                let mut x = index.max(0) as usize;
                while x < objs.len() {
                    match objs[x] {
                        SequentialObject::Statement(st) => {
                            if Self::is_var_read_first(var, st, 0, allowlist) {
                                return true;
                            }
                        }
                        SequentialObject::Exprent(exp) => {
                            if Self::is_var_read_first_exprent(var, exp, allowlist) {
                                return true;
                            }
                        }
                    }
                    x += 1;
                }
            }
            Some(exprents) => {
                let mut x = index.max(0) as usize;
                while x < exprents.len() {
                    if Self::is_var_read_first_exprent(var, exprents.get(x), allowlist) {
                        return true;
                    }
                    x += 1;
                }
            }
        }
        false
    }

    /// Java: isVarReadFirst(VarVersionPair, Exprent, VarExprent...)
    fn is_var_read_first_exprent(
        target: VarVersionPair,
        exp: Exprent,
        allowlist: &[VarExprent],
    ) -> bool {
        let assign = exp.try_as_assignment();
        let func = exp
            .try_as_function()
            .filter(|func| func.get_func_type().is_ppmm());

        let lst = exp.get_all_exprents_rec_self(true, true);

        for ex in lst {
            if let Some(var) = ex.try_as_var() {
                if var.get_index() == target.var && var.get_version() == target.version {
                    let mut allowed = false;

                    if let Some(assign) = assign {
                        if var.0 == assign.get_left() {
                            allowed = true;
                        }
                    }

                    if let Some(func) = func {
                        if var.0 == func.get_lst_operands().get(0) {
                            allowed = true;
                        }
                    }

                    for &allow in allowlist {
                        if var == allow {
                            allowed = true;
                        }
                    }

                    if !allowed {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Java: setNonFinal(Statement, Set)
    fn set_non_final(self, stat: Statement, un_initialized: &mut JHashSet<VarVersionPair>) {
        if let Some(exprents) = stat.get_exprents().filter(|e| !e.is_empty()) {
            for exp in exprents.iter() {
                if let Some(var) = exp.try_as_var() {
                    un_initialized.add(VarVersionPair::from_var(var));
                } else {
                    self.set_non_final_exprent(Some(exp), un_initialized);
                }
            }
        }

        let var_definitions = stat.get_var_definitions();
        if !var_definitions.is_empty() && stat.is_do() {
            for var in var_definitions.iter() {
                un_initialized.add(VarVersionPair::from_var(var.as_var()));
            }
        }

        if let Some(dostat) = stat.try_as_do() {
            self.set_non_final_exprent(dostat.get_init_exprent(), un_initialized);
            self.set_non_final_exprent(dostat.get_inc_exprent(), un_initialized);
        } else if let Some(ifstat) = stat.try_as_if() {
            if let (Some(if_branch), Some(else_branch)) =
                (ifstat.get_ifstat(), ifstat.get_elsestat())
            {
                self.set_non_final(
                    ifstat.get_first().expect("NullPointerException: first"),
                    un_initialized,
                );
                self.set_non_final(if_branch, &mut un_initialized.clone());
                self.set_non_final(else_branch, un_initialized);
                return;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            self.set_non_final(st, un_initialized);
        }
    }

    /// Java: setNonFinal(Exprent, Set)
    fn set_non_final_exprent(
        self,
        exp: Option<Exprent>,
        un_initialized: &mut JHashSet<VarVersionPair>,
    ) {
        let Some(exp) = exp else {
            return;
        };

        let mut var: Option<VarExprent> = None;

        if let Some(assign) = exp.try_as_assignment() {
            var = assign.get_left().try_as_var();
        } else if let Some(func) = exp.try_as_function() {
            if func.get_func_type().is_ppmm() {
                var = func.get_lst_operands().get(0).try_as_var();
            }
        }

        if let Some(var) = var {
            if !var.is_definition() && !un_initialized.remove(&var.get_var_version_pair()) {
                var.get_processor()
                    .expect("NullPointerException: processor")
                    .set_var_final(var.get_var_version_pair(), FinalType::NonFinal);
            }
        }

        for ex in exp.get_all_exprents() {
            self.set_non_final_exprent(Some(ex), un_initialized);
        }
    }

    pub fn remap_clashing_names(self, root: Statement, mt: StructMethod) {
        let varproc = self.varproc();
        let mut var_definitions: VarDefs = JHashMap::new();
        let mut live_var_defs: JHashSet<VarInMethod> = JHashSet::new();
        let mut name_map: NameMap = JHashMap::new();

        let md = mt.method_descriptor();
        let mut start = if mt.has_modifier(ACC_STATIC) { 0 } else { 1 };
        for i in 0..md.params.len() {
            let vvp = VarVersionPair::new(i as i32 + start, 0);
            let vim = VarInMethod::new(vvp, mt);
            live_var_defs.add(vim);
            name_map.put(vim, varproc.get_var_name(vvp));

            start += md.params[i].stack_size - 1;
        }

        let mut seen_methods: JHashSet<String> = JHashSet::new();
        seen_methods.add(InterpreterUtil::make_unique_key(
            &mt.get_name(),
            &mt.get_descriptor(),
        ));

        self.iterate_clashing_names(
            root,
            mt,
            &mut var_definitions,
            &mut live_var_defs,
            &mut JHashSet::new(),
            &mut name_map,
            &mut seen_methods,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_clashing_names(
        self,
        stat: Statement,
        mt: StructMethod,
        var_definitions: &mut VarDefs,
        live_var_defs: &mut JHashSet<VarInMethod>,
        cur_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut NameMap,
        seen_methods: &mut JHashSet<String>,
    ) {
        let mut should_remove_at_end = false;

        for exprent in stat.get_var_definitions().iter() {
            let mut up_defs: JHashSet<VarInMethod> = JHashSet::new();
            self.iterate_clashing_exprent(
                stat,
                mt,
                var_definitions,
                exprent,
                live_var_defs,
                &mut up_defs,
                name_map,
                seen_methods,
                &mut JHashSet::new(),
                &mut JHashSet::new(),
            );
            live_var_defs.add_all(up_defs.iter().copied());
            var_definitions
                .compute_if_absent(stat.get_parent(), JHashSet::new)
                .add_all(up_defs.iter().copied());
        }

        if stat.is_if() {
            let mut up_defs: JHashSet<VarInMethod> = JHashSet::new();
            let basic = stat.get_basichead();
            let exprents = basic
                .get_exprents()
                .expect("NullPointerException: exprents")
                .to_vec();
            for exprent in exprents {
                self.iterate_clashing_exprent(
                    basic.0,
                    mt,
                    var_definitions,
                    exprent,
                    live_var_defs,
                    &mut up_defs,
                    name_map,
                    seen_methods,
                    &mut JHashSet::new(),
                    &mut JHashSet::new(),
                );
            }

            live_var_defs.add_all(up_defs.iter().copied());
            var_definitions
                .compute_if_absent(stat.get_parent(), JHashSet::new)
                .add_all(up_defs.iter().copied());
        }

        let mut statement_vars: JHashMap<Statement, JHashSet<VarInMethod>> = JHashMap::new();
        let mut post: JHashSet<VarInMethod> = JHashSet::new();
        if let Some(exprents) = stat.get_exprents() {
            for exprent in exprents.iter() {
                self.iterate_clashing_exprent(
                    stat,
                    mt,
                    var_definitions,
                    exprent,
                    live_var_defs,
                    cur_var_defs,
                    name_map,
                    seen_methods,
                    &mut JHashSet::new(),
                    &mut JHashSet::new(),
                );
            }
        } else {
            if let Some(catch_stat) = stat.try_as_catch() {
                let mut i = 0;
                while i < catch_stat.get_stats().borrow().size() {
                    let st = catch_stat.get_stats().borrow().get(i);
                    let mut st_vars: JHashSet<VarInMethod> = JHashSet::new();
                    if i == 0 {
                        for exp in catch_stat.get_resources().iter() {
                            self.iterate_clashing_exprent(
                                st,
                                mt,
                                var_definitions,
                                exp,
                                live_var_defs,
                                &mut st_vars,
                                name_map,
                                seen_methods,
                                &mut JHashSet::new(),
                                &mut JHashSet::new(),
                            );
                        }
                    } else {
                        let exp = catch_stat.get_vars().get(i - 1);
                        self.iterate_clashing_exprent(
                            st,
                            mt,
                            var_definitions,
                            exp.0,
                            live_var_defs,
                            &mut st_vars,
                            name_map,
                            seen_methods,
                            &mut JHashSet::new(),
                            &mut JHashSet::new(),
                        );
                    }
                    live_var_defs.remove_all(&st_vars);
                    statement_vars.put(st, st_vars);
                    i += 1;
                }
            } else if let Some(switch_stat) = stat.try_as_switch() {
                let head_exp = switch_stat.get_headexprent();
                self.iterate_clashing_exprent(
                    stat,
                    mt,
                    var_definitions,
                    head_exp,
                    live_var_defs,
                    cur_var_defs,
                    name_map,
                    seen_methods,
                    &mut JHashSet::new(),
                    &mut JHashSet::new(),
                );
                let mut i = 0;
                while i < switch_stat.get_case_statements().len() {
                    let st = switch_stat.get_case_statements().get(i);
                    let mut st_vars: JHashSet<VarInMethod> = JHashSet::new();
                    for exp in switch_stat.get_case_values().get(i).iter().flatten() {
                        self.iterate_clashing_exprent(
                            st,
                            mt,
                            var_definitions,
                            exp,
                            live_var_defs,
                            &mut st_vars,
                            name_map,
                            seen_methods,
                            &mut JHashSet::new(),
                            &mut JHashSet::new(),
                        );
                    }
                    let case_guards = switch_stat.get_case_guards();
                    if case_guards.len() > i {
                        if let Some(exp) = case_guards.get(i) {
                            self.iterate_clashing_exprent(
                                st,
                                mt,
                                var_definitions,
                                exp,
                                live_var_defs,
                                &mut st_vars,
                                name_map,
                                seen_methods,
                                &mut JHashSet::new(),
                                &mut JHashSet::new(),
                            );
                        }
                    }
                    live_var_defs.remove_all(&st_vars);
                    statement_vars.put(st, st_vars);
                    i += 1;
                }
            } else if let Some(if_stat) = stat.try_as_if() {
                let mut when_true: JHashSet<VarInMethod> = JHashSet::new();
                let mut when_false: JHashSet<VarInMethod> = JHashSet::new();
                self.iterate_clashing_exprent(
                    stat,
                    mt,
                    var_definitions,
                    if_stat.get_headexprent().get_condition(),
                    live_var_defs,
                    cur_var_defs,
                    name_map,
                    seen_methods,
                    &mut when_true,
                    &mut when_false,
                );
                let if_complete_normally = if_stat
                    .get_ifstat()
                    .is_none_or(|s| !s.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty());
                let else_complete_normally = if_stat
                    .get_elsestat()
                    .is_none_or(|s| !s.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty());
                if let Some(ifstat) = if_stat.get_ifstat() {
                    statement_vars.put(ifstat, when_true.clone());
                }

                if let Some(elsestat) = if_stat.get_elsestat() {
                    statement_vars.put(elsestat, when_false.clone());
                }

                if !if_complete_normally && else_complete_normally {
                    post = when_false;
                } else if if_complete_normally && !else_complete_normally {
                    post = when_true;
                }
            } else if let Some(do_stat) = stat.try_as_do() {
                let mut when_true: JHashSet<VarInMethod> = JHashSet::new();
                let mut when_false: JHashSet<VarInMethod> = JHashSet::new();
                if let Some(init) = do_stat.get_init_exprent() {
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        init,
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut JHashSet::new(),
                        &mut JHashSet::new(),
                    );
                }
                if let Some(condition) = do_stat.get_condition_exprent() {
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        condition,
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut when_true,
                        &mut when_false,
                    );
                }
                if let Some(inc) = do_stat.get_inc_exprent() {
                    live_var_defs.add_all(when_true.iter().copied());
                    self.iterate_clashing_exprent(
                        do_stat.0,
                        mt,
                        var_definitions,
                        inc,
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut JHashSet::new(),
                        &mut JHashSet::new(),
                    );
                    live_var_defs.remove_all(&when_true);
                }

                let looptype = do_stat.get_looptype();
                if looptype == DoType::While || looptype == DoType::For {
                    statement_vars.put(
                        do_stat.get_first().expect("NullPointerException: first"),
                        when_true.clone(),
                    );
                }
                if do_stat.get_successor_edges(StatEdge::TYPE_BREAK).is_empty() {
                    post = when_false;
                }
            } else {
                for exp in stat.get_stat_exprents() {
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        exp,
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut JHashSet::new(),
                        &mut JHashSet::new(),
                    );
                }
            }

            should_remove_at_end = true;
        }

        live_var_defs.add_all(cur_var_defs.iter().copied());
        var_definitions.put(Some(stat), std::mem::take(cur_var_defs));

        let mut iterate = true;
        if let Some(switch_stat) = stat.try_as_switch() {
            if switch_stat.is_phantom() {
                iterate = false;
            }
        }

        let deferred: Vec<Statement> = Vec::new();
        let seperate = stat.is_if() || stat.is_catch();
        if iterate {
            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                let mut st_vars: JHashSet<VarInMethod> = JHashSet::new();
                let tmp_vars = statement_vars.get_or_default(&st, JHashSet::new());
                if stat.is_if() && st == stat.get_basichead().0 {
                    continue;
                }
                live_var_defs.add_all(tmp_vars.iter().copied());
                self.iterate_clashing_names(
                    st,
                    mt,
                    var_definitions,
                    live_var_defs,
                    &mut st_vars,
                    name_map,
                    seen_methods,
                );
                live_var_defs.remove_all(&tmp_vars);
                if seperate {
                    self.clear_statement(var_definitions, live_var_defs, name_map, st);
                    self.clear_statement(var_definitions, live_var_defs, name_map, stat);
                }
            }
        }

        if should_remove_at_end {
            self.clear_statement(var_definitions, live_var_defs, name_map, stat);
        }

        for st in JHashSet::from_collection(var_definitions.key_vec()).to_vec() {
            let st = st.expect("NullPointerException: statement");
            if st.get_parent() == Some(stat) {
                self.clear_statement(var_definitions, live_var_defs, name_map, st);
            }
        }

        if iterate {
            for &st in &deferred {
                let tmp_vars = statement_vars.get_or_default(&st, JHashSet::new());
                live_var_defs.add_all(tmp_vars.iter().copied());
                self.iterate_clashing_names(
                    st,
                    mt,
                    var_definitions,
                    live_var_defs,
                    &mut JHashSet::new(),
                    name_map,
                    seen_methods,
                );
                live_var_defs.remove_all(&tmp_vars);
            }
        }

        for st in JHashSet::from_collection(var_definitions.key_vec()).to_vec() {
            let st = st.expect("NullPointerException: statement");
            if st.get_parent() == Some(stat) && deferred.contains(&st) {
                self.clear_statement(var_definitions, live_var_defs, name_map, st);
            }
        }

        if !stat.is_root() {
            var_definitions
                .compute_if_absent(stat.get_parent(), JHashSet::new)
                .add_all(post.iter().copied());
            live_var_defs.add_all(post.iter().copied());
        }
    }

    fn clear_statement(
        self,
        var_definitions: &mut VarDefs,
        live_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut NameMap,
        st: Statement,
    ) {
        if let Some(removed) = var_definitions.remove(&Some(st)) {
            live_var_defs.remove_all(&removed);

            for vvp in removed.iter() {
                name_map.remove(vvp);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_clashing_exprent(
        self,
        stat: Statement,
        mt: StructMethod,
        var_definitions: &mut VarDefs,
        exprent: Exprent,
        live_var_defs: &mut JHashSet<VarInMethod>,
        cur_var_defs: &mut JHashSet<VarInMethod>,
        name_map: &mut NameMap,
        seen_methods: &mut JHashSet<String>,
        when_true: &mut JHashSet<VarInMethod>,
        when_false: &mut JHashSet<VarInMethod>,
    ) {
        let mut iterate = true;
        if let Some(new_exprent) = exprent.try_as_new() {
            if new_exprent.is_lambda() && !new_exprent.is_method_reference() {
                let node = root_class_node(&new_exprent.get_new_type());
                if let Some((node, wrapper)) = node.and_then(|n| n.get_wrapper().map(|w| (n, w))) {
                    let li = node
                        .with(|d| d.lambda_information.clone())
                        .expect("NullPointerException: lambdaInformation");
                    let mw = wrapper
                        .get_methods()
                        .borrow()
                        .get_with_key(&li.content_method_key);
                    let mt2 = wrapper
                        .get_class_struct()
                        .get_method_key(&li.content_method_key);
                    if let (Some(mt2), Some(mw)) = (mt2, mw) {
                        let key = InterpreterUtil::make_unique_key(
                            &mt2.get_name(),
                            &mt2.get_descriptor(),
                        );
                        if !seen_methods.contains(&key) {
                            seen_methods.add(key);
                            let (mw_root, mw_varproc) = mw.with(|d| (d.root, d.varproc));

                            let md =
                                MethodDescriptor::parse_descriptor(&li.content_method_descriptor);
                            let start_idx = md.params.len() as i32
                                - MethodDescriptor::parse_descriptor(&li.method_descriptor)
                                    .params
                                    .len() as i32;
                            let mut start = if li.is_content_method_static { 0 } else { 1 };
                            for i in 0..md.params.len() {
                                if i as i32 >= start_idx {
                                    let vvp = VarVersionPair::new(start, 0);

                                    let original = mw_varproc.get_var_name(vvp);
                                    let name = Self::rename(
                                        name_map,
                                        original.as_deref().expect("NullPointerException: name"),
                                        live_var_defs,
                                    );

                                    if original.as_deref() != Some(name.as_str()) {
                                        mw_varproc.set_clashing_name(vvp, name.clone());
                                        name_map.put(VarInMethod::new(vvp, mt2), Some(name));
                                    }
                                }

                                start += md.params[i].stack_size;
                            }

                            let mw_root = mw_root.expect("NullPointerException: lambda root");
                            let vardef =
                                VarDefinitionHelper::new_run(mw_root, mt2, mw_varproc, false);
                            vardef.iterate_clashing_names(
                                mw_root.0,
                                mt2,
                                var_definitions,
                                live_var_defs,
                                &mut JHashSet::new(),
                                name_map,
                                seen_methods,
                            );

                            let clashing = vardef.get_clashing_names().borrow().entry_vec();
                            for (k, v) in clashing {
                                mw_varproc.set_clashing_name(k, v);
                            }

                            for k in name_map.key_vec() {
                                if k.mt == mt2 {
                                    name_map.remove(&k);
                                }
                            }
                        }
                    }
                }
            } else if !new_exprent.is_lambda() && new_exprent.is_anonymous() {
                let node = root_class_node(&new_exprent.get_new_type());
                if let Some(wrapper) = node.and_then(|n| n.get_wrapper()) {
                    let keys = wrapper.get_methods().borrow().get_lst_keys().clone();
                    for mth_key in keys {
                        let mw = mth_key
                            .as_ref()
                            .and_then(|k| wrapper.get_methods().borrow().get_with_key(k));
                        let mt2 = mth_key
                            .as_ref()
                            .and_then(|k| wrapper.get_class_struct().get_method_key(k));
                        let (Some(mt2), Some(mw)) = (mt2, mw) else {
                            continue;
                        };
                        if mt2.has_modifier(ACC_SYNTHETIC) {
                            continue;
                        }
                        let key = InterpreterUtil::make_unique_key(
                            &mt2.get_name(),
                            &mt2.get_descriptor(),
                        );
                        if seen_methods.contains(&key) {
                            continue;
                        }
                        seen_methods.add(key);
                        let (mw_root, mw_varproc) = mw.with(|d| (d.root, d.varproc));

                        let md = mt2.method_descriptor();
                        let mut start = if mt2.has_modifier(ACC_STATIC) { 0 } else { 1 };
                        let start_idx = start;
                        for i in 0..md.params.len() {
                            if i as i32 >= start_idx {
                                let vvp = VarVersionPair::new(start, 0);

                                if let Some(original) = mw_varproc.get_var_name(vvp) {
                                    let name = Self::rename(name_map, &original, live_var_defs);

                                    if original != name {
                                        mw_varproc.set_clashing_name(vvp, name.clone());
                                        name_map.put(VarInMethod::new(vvp, mt2), Some(name));
                                    }
                                }
                            }

                            start += md.params[i].stack_size;
                        }

                        let mw_root = mw_root.expect("NullPointerException: method root");
                        let vardef = VarDefinitionHelper::new_run(mw_root, mt2, mw_varproc, false);
                        vardef.iterate_clashing_names(
                            mw_root.0,
                            mt2,
                            var_definitions,
                            live_var_defs,
                            &mut JHashSet::new(),
                            name_map,
                            seen_methods,
                        );

                        let clashing = vardef.get_clashing_names().borrow().entry_vec();
                        for (k, v) in clashing {
                            mw_varproc.set_clashing_name(k, v);
                        }

                        for k in name_map.key_vec() {
                            if k.mt == mt2 {
                                name_map.remove(&k);
                            }
                        }
                    }
                }
            }
        }

        if let Some(var) = exprent.try_as_var() {
            if var.is_definition() {
                let varproc = self.varproc();
                let vvp = var.get_var_version_pair();
                let def = VarInMethod::new(vvp, mt);
                cur_var_defs.add(def);
                live_var_defs.add(def);

                let lvt = var.get_lvt();
                if lvt.is_some() || varproc.get_var_name(vvp).is_some() {
                    let original_name = match &lvt {
                        None => varproc.get_var_name(vvp).expect("checked above"),
                        Some(lvt) => lvt.get_name(),
                    };

                    let name = Self::rename(name_map, &original_name, live_var_defs);

                    let mut scoped_switch = false;
                    if original_name != name {
                        let parent = Self::direct_parent(stat);
                        if let Some(switch_stat) = parent.and_then(|p| p.try_as_switch()) {
                            let mut same_var_name: JHashSet<VarInMethod> = JHashSet::new();

                            for (key, value) in name_map.iter() {
                                let value = value.as_deref().expect("NullPointerException: name");
                                if value == original_name {
                                    same_var_name.add(*key);
                                }
                            }

                            for st in switch_stat.get_case_statements().iter() {
                                if let Some(case_var_defs) = var_definitions.get(&Some(st)) {
                                    for pair in same_var_name.iter() {
                                        if case_var_defs.contains(pair) {
                                            switch_stat.scope_case_statement(st);
                                            let found_case =
                                                Self::find_case_owning(stat, switch_stat);

                                            if let Some(found_case) = found_case {
                                                switch_stat.scope_case_statement(found_case);
                                            }

                                            scoped_switch = true;
                                        }
                                    }
                                }
                            }
                        }

                        if !scoped_switch {
                            self.get_clashing_names()
                                .borrow_mut()
                                .put(var.get_var_version_pair(), name.clone());
                        }
                    }

                    let value = if scoped_switch { original_name } else { name };
                    name_map.put(
                        VarInMethod::new(var.get_var_version_pair(), mt),
                        Some(value),
                    );
                }
            }
        }

        if let Some(function) = exprent.try_as_function() {
            iterate = false;
            match function.get_func_type() {
                FunctionType::BooleanAnd => {
                    for exp in function.get_lst_operands().iter() {
                        live_var_defs.add_all(when_true.iter().copied());
                        self.iterate_clashing_exprent(
                            stat,
                            mt,
                            var_definitions,
                            exp,
                            live_var_defs,
                            cur_var_defs,
                            name_map,
                            seen_methods,
                            when_true,
                            &mut JHashSet::new(),
                        );
                    }
                    live_var_defs.remove_all(when_true);
                }
                FunctionType::BooleanOr => {
                    for exp in function.get_lst_operands().iter() {
                        live_var_defs.add_all(when_false.iter().copied());
                        self.iterate_clashing_exprent(
                            stat,
                            mt,
                            var_definitions,
                            exp,
                            live_var_defs,
                            cur_var_defs,
                            name_map,
                            seen_methods,
                            &mut JHashSet::new(),
                            when_false,
                        );
                    }
                    live_var_defs.remove_all(when_false);
                }
                FunctionType::BoolNot => {
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        function.get_lst_operands().get(0),
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        when_false,
                        when_true,
                    );
                }
                FunctionType::Ternary => {
                    let operands = function.get_lst_operands();
                    let mut sub_when_true: JHashSet<VarInMethod> = JHashSet::new();
                    let mut sub_when_false: JHashSet<VarInMethod> = JHashSet::new();
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        operands.get(0),
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut sub_when_true,
                        &mut sub_when_false,
                    );
                    live_var_defs.add_all(sub_when_true.iter().copied());
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        operands.get(1),
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut JHashSet::new(),
                        &mut JHashSet::new(),
                    );
                    live_var_defs.remove_all(&sub_when_true);
                    live_var_defs.add_all(sub_when_false.iter().copied());
                    self.iterate_clashing_exprent(
                        stat,
                        mt,
                        var_definitions,
                        operands.get(2),
                        live_var_defs,
                        cur_var_defs,
                        name_map,
                        seen_methods,
                        &mut JHashSet::new(),
                        &mut JHashSet::new(),
                    );
                    live_var_defs.remove_all(&sub_when_false);
                }
                FunctionType::Instanceof => {
                    let operands = function.get_lst_operands();
                    if operands.len() > 2 {
                        self.iterate_clashing_exprent(
                            stat,
                            mt,
                            var_definitions,
                            operands.get(0),
                            live_var_defs,
                            cur_var_defs,
                            name_map,
                            seen_methods,
                            &mut JHashSet::new(),
                            &mut JHashSet::new(),
                        );
                        self.iterate_clashing_exprent(
                            stat,
                            mt,
                            var_definitions,
                            operands.get(2),
                            live_var_defs,
                            when_true,
                            name_map,
                            seen_methods,
                            &mut JHashSet::new(),
                            &mut JHashSet::new(),
                        );
                        live_var_defs.remove_all(when_true);
                    } else {
                        iterate = true;
                    }
                }
                _ => iterate = true,
            }
        }

        if iterate {
            for exp in exprent.get_all_exprents() {
                self.iterate_clashing_exprent(
                    stat,
                    mt,
                    var_definitions,
                    exp,
                    live_var_defs,
                    cur_var_defs,
                    name_map,
                    seen_methods,
                    &mut JHashSet::new(),
                    &mut JHashSet::new(),
                );
            }
        }
    }

    fn rename(name_map: &NameMap, name: &str, live_var_defs: &JHashSet<VarInMethod>) -> String {
        let mut name = name.to_string();
        while live_var_defs
            .iter()
            .map(|v| name_map.get(v).and_then(|n| n.as_deref()))
            .any(|n| n == Some(name.as_str()))
        {
            name.push('x');
        }
        name
    }

    fn find_case_owning(stat: Statement, switch_stat: SwitchStatement) -> Option<Statement> {
        switch_stat
            .get_case_statements()
            .iter()
            .find(|case_statement| case_statement.contains_statement(stat))
    }

    fn direct_parent(stat: Statement) -> Option<Statement> {
        let mut parent = stat.get_parent();

        while let Some(p) = parent {
            if p.is_sequence() || (p.get_first() == Some(stat) && p.is_if()) {
                parent = p.get_parent();
            } else {
                break;
            }
        }

        parent
    }

    pub fn get_clashing_names(self) -> Shared<JHashMap<VarVersionPair, String>> {
        self.with(|d| d.clashing_names.clone())
    }
}

/// `StructContext.instanceOf(String, String)`, which is `false` for a `null` class.
fn instance_of(valclass: Option<&str>, refclass: Option<&str>) -> bool {
    match (valclass, refclass) {
        (Some(valclass), Some(refclass)) => {
            DecompilerContext::get_struct_context().instance_of(valclass, refclass)
        }
        _ => false,
    }
}

/// `getMapRootClasses().get(type.value)`.
fn root_class_node(type_: &VarType) -> Option<ClassNode> {
    let value = type_.value.as_deref()?.to_string();
    DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get_cloned(&value)
}

/// `vc.getFreeName(String)` for a name that may be `null`: Java's collector
/// stores the `null` and answers `"nullx"`-style names for later `null`s.
fn free_name_nullable(
    vc: VarNamesCollector,
    proposition: Option<&str>,
    null_used: &mut bool,
) -> Option<String> {
    match proposition {
        Some(name) => Some(vc.get_free_name(name)),
        None if !*null_used => {
            *null_used = true;
            None
        }
        None => Some(vc.get_free_name("nullx")),
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
        let cast = match (&lvt, &type_) {
            (Some(lvt), _) => match lvt.get_signature() {
                Some(signature) => {
                    ExprProcessor::get_cast_type_name_short(&GenericType::parse(&signature), false)
                }
                None => ExprProcessor::get_cast_type_name_short(&lvt.get_var_type(), false),
            },
            (None, Some(type_)) => ExprProcessor::get_cast_type_name_short(type_, false),
            (None, None) => "this".to_string(),
        };
        VarInfo { lvt, cast, type_ }
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
