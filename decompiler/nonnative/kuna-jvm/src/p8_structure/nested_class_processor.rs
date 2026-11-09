//! Port of `org/jetbrains/java/decompiler/main/rels/NestedClassProcessor.java`.
//!
//! Java lets a variable name be `null` (an unnamed enclosing variable); the
//! name maps here carry `Option<String>`, and a `None` name is not written
//! into a `VarProcessor` or name set, which cannot hold one.

#![allow(clippy::collapsible_if)]

use std::collections::{BTreeSet, VecDeque};
use std::fmt;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{
    ACC_FINAL, ACC_INTERFACE, ACC_MANDATED, ACC_PRIVATE, ACC_STATIC, ACC_SYNTHETIC, INIT_NAME,
};
use crate::code_type::CodeType;
use crate::const_exprent::ConstValue;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::exprent::{Exprent, ExprentType};
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationKind};
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::rt::jstring::java_string_hash;
use crate::rt::{JHashMap, JHashSet, JList, JavaEq, JavaHash};
use crate::statement::{Statement, StatementType};
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::var_exprent::VarExprent;
use crate::var_names_collector::VarNamesCollector;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;

/// `AbstractCollection.toString()` over strings: `[a, b]`.
fn collection_to_string<'a>(items: impl Iterator<Item = &'a String>) -> String {
    let parts: Vec<&str> = items.map(String::as_str).collect();
    format!("[{}]", parts.join(", "))
}

fn warn(message: &str) {
    DecompilerContext::get_logger().write_message(message, Severity::Warn);
}

/// `methods.getWithKey(key)` where Java may pass a `null` key.
fn method_with_key(wrapper: ClassWrapper, key: Option<&String>) -> Option<MethodWrapper> {
    key.and_then(|key| wrapper.get_methods().borrow().get_with_key(key))
}

/// Recovers the outer-variable captures of local, anonymous and lambda classes
/// and places local class definitions in their enclosing methods.
#[derive(Default)]
pub struct NestedClassProcessor;

impl NestedClassProcessor {
    pub fn new() -> NestedClassProcessor {
        NestedClassProcessor
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn process_class(&self, root: ClassNode, node: ClassNode) {
        let (node_type, lambda_information, class_struct, nested) = node.with(|d| {
            (
                d.type_,
                d.lambda_information.clone(),
                d.class_struct,
                d.nested.clone(),
            )
        });

        if node_type == ClassNodeType::Lambda
            && !lambda_information
                .as_ref()
                .expect("lambdaInformation")
                .is_method_reference
        {
            let node_content = DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get_cloned(&class_struct.qualified_name());
            if let Some(node_content) = node_content {
                if let Some(wrapper) = node_content.get_wrapper() {
                    wrapper.get_hidden_members().borrow_mut().add(
                        lambda_information
                            .as_ref()
                            .unwrap()
                            .content_method_key
                            .clone(),
                    );
                }
            }
        }

        if nested.is_empty() {
            return;
        }

        let mut double_nested: JHashSet<ClassNode> = JHashSet::new();
        for child in nested.to_vec() {
            double_nested.add_all(child.get_all_nested().to_vec());
        }
        nested.borrow_mut().retain(|c| !double_nested.contains(c));

        if node_type != ClassNodeType::Lambda {
            Self::compute_local_vars_and_definitions(node);

            Self::check_not_found_classes(root, node);
        }

        let mut nameless = 0;
        let mut synthetics = 0;
        for child in nested.to_vec() {
            let (child_type, child_access, unnamed, cl) =
                child.with(|d| (d.type_, d.access, d.simple_name.is_none(), d.class_struct));
            if (child_type == ClassNodeType::Local || child_type == ClassNodeType::Member)
                && unnamed
            {
                if (child_access & ACC_SYNTHETIC) != 0 || cl.is_synthetic() {
                    synthetics += 1;
                    let name = format!("SyntheticClass_{synthetics}");
                    child.with_mut(|d| d.simple_name = Some(name));
                } else {
                    let message =
                        format!("Nameless local or member class {}!", cl.qualified_name());
                    warn(&message);
                    nameless += 1;
                    let name = format!("NamelessClass_{nameless}");
                    child.with_mut(|d| d.simple_name = Some(name));
                }
            }
        }

        for child in nested.to_vec() {
            let (child_type, child_access) = child.with(|d| (d.type_, d.access));
            if child_type == ClassNodeType::Lambda {
                Self::set_lambda_vars(node, child);
            } else if child_type != ClassNodeType::Member || (child_access & ACC_STATIC) == 0 {
                Self::insert_local_vars(node, child);

                let (child_type, enclosing_method) =
                    child.with(|d| (d.type_, d.enclosing_method.clone()));
                if child_type == ClassNodeType::Local && enclosing_method.is_some() {
                    let enclosing_method_wrapper =
                        method_with_key(node.get_wrapper().unwrap(), enclosing_method.as_ref());
                    if let Some(enclosing_method_wrapper) = enclosing_method_wrapper {
                        Self::set_local_class_definition(enclosing_method_wrapper, child);
                    }
                }
            }
        }

        for child in nested.to_vec() {
            self.process_class(root, child);
        }
    }

    fn set_lambda_vars(parent: ClassNode, child: ClassNode) {
        let (lambda_information, child_enclosing_method, child_simple_name) = child.with(|d| {
            (
                d.lambda_information.clone().expect("lambdaInformation"),
                d.enclosing_method.clone(),
                d.simple_name.clone(),
            )
        });
        if lambda_information.is_method_reference {
            return;
        }

        let parent_wrapper = parent.get_wrapper().unwrap();
        let method = parent_wrapper
            .get_methods()
            .borrow()
            .get_with_key(&lambda_information.content_method_key)
            .expect("lambda content method");
        let var_proc = method.with(|d| d.varproc);
        if var_proc.nested_processed() {
            let parent_name = parent.with(|d| d.class_struct).qualified_name();
            warn(&format!("{parent_name}.{method} processed twice"));
            return;
        }
        let enclosing_method = method_with_key(parent_wrapper, child_enclosing_method.as_ref())
            .expect("lambda enclosing method");

        if enclosing_method.with(|d| d.decompile_error.is_some())
            || method.with(|d| d.decompile_error.is_some())
        {
            return;
        }

        let md_lambda = MethodDescriptor::parse_descriptor(&lambda_information.method_descriptor);
        let md_content =
            MethodDescriptor::parse_descriptor(&lambda_information.content_method_descriptor);

        let vars_count = md_content.params.len() as i32 - md_lambda.params.len() as i32;
        let is_static_lambda_content = lambda_information.is_content_method_static;

        let parent_class_name = parent_wrapper.get_class_struct().qualified_name();
        let lambda_class_name = child_simple_name;

        let lambda_class_type = VarType::new_signature_cl(
            lambda_class_name.as_deref().expect("lambda class name"),
            true,
        );

        if !is_static_lambda_content
            && DecompilerContext::get_option(IFernflowerPreferences::LAMBDA_TO_ANONYMOUS_CLASS)
        {
            var_proc
                .get_this_vars()
                .borrow_mut()
                .put(VarVersionPair::new(0, 0), parent_class_name.clone());
            let parent_simple_name = parent.with(|d| d.simple_name.clone());
            var_proc.set_var_name(
                VarVersionPair::new(0, 0),
                format!("{}.this", parent_simple_name.as_deref().unwrap_or("null")),
            );
        }

        let enclosing_var_proc = enclosing_method.with(|d| d.varproc);
        let mut used_before: JHashSet<String> = JHashSet::from_collection(
            enclosing_var_proc
                .get_var_names_collector()
                .get_used_names(),
        );
        used_before.add_all(enclosing_var_proc.get_var_names());
        used_before.add_all(enclosing_var_proc.clashing_names());
        let this_vars = enclosing_var_proc.get_this_vars().borrow().key_vec();
        for this_var in this_vars {
            if let Some(name) = enclosing_var_proc.get_var_name(this_var) {
                used_before.remove(&name);
            }
        }
        let enclosing_collector = VarNamesCollector::new_with_names(used_before.to_vec());

        let mut original_versions: JHashMap<VarVersionPair, VarVersionPair> = JHashMap::new();
        let mut map_new_names: JHashMap<VarVersionPair, Option<String>> = JHashMap::new();
        let mut lvts: JHashMap<VarVersionPair, Option<LocalVariable>> = JHashMap::new();

        for local in var_proc.get_used_var_versions() {
            let mut name: Option<String> = None;
            let lvt = var_proc.get_var_lvt(local);
            if let Some(lvt) = &lvt {
                name = Some(lvt.get_name());
            }
            if name.is_none() {
                name = var_proc.get_var_name(local);
            }
            if let Some(old_name) = name {
                if used_before.contains(&old_name) && old_name != "this" {
                    let new_name = enclosing_collector.get_free_name(&old_name);
                    map_new_names.put(local, Some(new_name.clone()));
                    if let Some(lvt) = &lvt {
                        lvts.put(local, Some(lvt.rename(new_name)));
                    }
                }
            }
        }

        let Some(enclosing_graph) = enclosing_method.get_or_build_graph() else {
            return;
        };

        enclosing_graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true);
            lst.push(exprent);

            for expr in lst {
                if let Some(new_expr) = expr.try_as_new() {
                    if new_expr.is_lambda() && lambda_class_type.equals(&new_expr.get_new_type()) {
                        let inv_dynamic = new_expr.get_constructor().expect("lambda constructor");

                        let param_index: usize = if is_static_lambda_content { 0 } else { 1 };
                        let mut var_index: i32 = if is_static_lambda_content { 0 } else { 1 };

                        for i in 0..md_content.params.len() {
                            let var_version = VarVersionPair::new(var_index, 0);
                            if (i as i32) < vars_count {
                                let param = inv_dynamic.get_lst_parameters().get(param_index + i);

                                if let Some(param) = param.try_as_var() {
                                    let param_version = VarVersionPair::from_var(param);
                                    original_versions.put(var_version, param_version);
                                    map_new_names.put(
                                        var_version,
                                        enclosing_var_proc.get_var_name(param_version),
                                    );
                                    lvts.put(var_version, param.get_lvt());
                                    let _ = enclosing_var_proc.get_var_final(param_version)
                                        == FinalType::NonFinal;
                                }
                            } else if !map_new_names.contains_key(&var_version) {
                                let name = var_proc
                                    .get_var_name(var_version)
                                    .map(|name| enclosing_collector.get_free_name(&name));
                                map_new_names.put(var_version, name);
                            }

                            var_index += md_content.params[i].stack_size;
                        }
                    }
                }
            }

            0
        });

        let mut set_new_outer_names: JHashSet<String> = JHashSet::from_collection(
            map_new_names
                .values()
                .flatten()
                .cloned()
                .collect::<Vec<_>>(),
        );
        method.with(|d| {
            set_new_outer_names.remove_all(&d.set_outer_var_names);
        });

        let new_outer_names = set_new_outer_names.to_vec();
        method.with_mut(|d| {
            d.set_outer_var_names.add_all(new_outer_names);
        });
        var_proc
            .get_var_names_collector()
            .add_names(enclosing_collector.get_used_names());

        for (pair, name) in map_new_names.entry_vec() {
            let lvt = lvts.get(&pair).cloned().flatten();
            let original = original_versions.get(&pair).copied();

            if let Some(name) = name {
                var_proc.set_inherited_name(pair, name);
            }
            if let Some(lvt) = lvt {
                var_proc.set_var_lvt(pair, lvt);
            }
            if let Some(original) = original {
                var_proc.set_var_source(
                    pair,
                    child_enclosing_method.clone().expect("enclosingMethod"),
                    original,
                );
            }
        }

        method
            .get_or_build_graph()
            .expect("DirectGraph")
            .iterate_exprents_deep(|exp: Exprent| {
                if let Some(var) = exp.try_as_var() {
                    let lv = lvts.get(&var.get_var_version_pair()).cloned().flatten();
                    if let Some(lv) = lv {
                        var.set_lvt(Some(lv));
                    } else if map_new_names.contains_key(&var.get_var_version_pair()) {
                        var.set_lvt(None);
                    }
                }
                0
            });
        var_proc.set_nested_processed(true);
    }

    fn check_not_found_classes(root: ClassNode, node: ClassNode) {
        let (nested, node_class) = node.with(|d| (d.nested.clone(), d.class_struct));
        let copy = nested.to_vec();

        for child in copy {
            let (child_class, child_type, no_enclosing_method, set_enclosing) = child.with(|d| {
                (
                    d.class_struct,
                    d.type_,
                    d.enclosing_method.is_none(),
                    d.enclosing_classes.clone(),
                )
            });
            if child_class.is_synthetic() {
                continue;
            }

            if (child_type == ClassNodeType::Local || child_type == ClassNodeType::Anonymous)
                && no_enclosing_method
            {
                if !set_enclosing.borrow().is_empty() {
                    let attr = child_class
                        .get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD);
                    if let Some(attr) = attr {
                        if let Some(method_name) = attr.get_method_name() {
                            let method_descriptor = attr.get_method_descriptor();
                            let method_descriptor = method_descriptor.as_deref().unwrap_or("null");
                            if attr.get_class_name().as_deref()
                                == Some(node_class.qualified_name().as_str())
                                && node_class
                                    .get_method(&method_name, method_descriptor)
                                    .is_some()
                            {
                                let key = InterpreterUtil::make_unique_key(
                                    &method_name,
                                    method_descriptor,
                                );
                                child.with_mut(|d| d.enclosing_method = Some(key));
                                continue;
                            }
                        }
                    }
                }

                nested.remove_item(&child);
                child.with_mut(|d| d.parent = None);
                set_enclosing
                    .borrow_mut()
                    .remove(&node_class.qualified_name());

                let enclosing_non_empty = !set_enclosing.borrow().is_empty();
                let has_enclosing = enclosing_non_empty && Self::insert_nested_class(root, child);

                if !has_enclosing {
                    if child_type == ClassNodeType::Anonymous {
                        let message = format!(
                            "Unreferenced anonymous class {}!",
                            child_class.qualified_name()
                        );
                        warn(&message);
                    } else if child_type == ClassNodeType::Local {
                        let message =
                            format!("Unreferenced local class {}!", child_class.qualified_name());
                        warn(&message);
                    }
                }
            }
        }
    }

    fn insert_nested_class(root: ClassNode, child: ClassNode) -> bool {
        let set_enclosing = child.with(|d| d.enclosing_classes.clone());

        let mut stack: VecDeque<ClassNode> = VecDeque::new();
        stack.push_back(root);

        while let Some(node) = stack.pop_front() {
            let qualified_name = node.with(|d| d.class_struct).qualified_name();
            if set_enclosing.borrow().contains(&qualified_name) {
                let nested = node.with(|d| d.nested.clone());
                nested.add(child);
                child.with_mut(|d| d.parent = Some(node));
                let mut sorted = nested.to_vec();
                sorted.sort_by(|a, b| a.compare_to(*b));
                nested.set_all(sorted);

                return true;
            }

            stack.extend(node.with(|d| d.nested.to_vec()));
        }

        false
    }

    fn compute_local_vars_and_definitions(node: ClassNode) {
        let mut map_var_masks: JHashMap<String, JHashMap<String, Vec<Option<VarFieldPair>>>> =
            JHashMap::new();

        let mut cl_types: BTreeSet<ClassNodeType> = BTreeSet::new();

        let nested = node.with(|d| d.nested.to_vec());
        for nd in nested {
            let (nd_type, nd_class, nd_access) = nd.with(|d| (d.type_, d.class_struct, d.access));
            if nd_type != ClassNodeType::Lambda
                && !nd_class.is_synthetic()
                && (nd_access & ACC_STATIC) == 0
                && (nd_access & ACC_INTERFACE) == 0
                && nd_class.get_record_components().is_none()
            {
                cl_types.insert(nd_type);

                let mask = Self::get_mask_local_vars(nd.get_wrapper().expect("ClassWrapper"));
                if mask.is_empty() {
                    let message = format!(
                        "Nested class {} has no constructor!",
                        nd_class.qualified_name()
                    );
                    warn(&message);
                } else {
                    map_var_masks.put(nd_class.qualified_name(), mask);
                }
            }
        }

        let mut map_var_field_pairs: JHashMap<String, JHashMap<String, Vec<Option<VarFieldPair>>>> =
            JHashMap::new();

        if !(cl_types.len() == 1 && cl_types.contains(&ClassNodeType::Member)) {
            let wrapper = node.get_wrapper().expect("ClassWrapper");
            let methods = wrapper.get_methods().borrow().to_vec();
            for method in methods {
                if method.with(|d| d.root.is_some()) {
                    let graph = method.get_or_build_graph().expect("DirectGraph");
                    graph.iterate_exprents(|exprent: Exprent| {
                        let mut lst = exprent.get_all_exprents_rec(true);
                        lst.push(exprent);

                        for expr in lst {
                            let Some(new_expr) = expr.try_as_new() else {
                                continue;
                            };
                            let Some(constructor) = new_expr.get_constructor() else {
                                continue;
                            };
                            if !map_var_masks.contains_key(&constructor.get_classname()) {
                                continue;
                            }

                            let ref_class_name = constructor.get_classname();
                            let nested_class_node = node
                                .get_class_node(&ref_class_name)
                                .expect("nested class node");

                            if nested_class_node.with(|d| d.type_) != ClassNodeType::Member {
                                let constructor_descriptor = constructor.get_string_descriptor();
                                let mask = map_var_masks
                                    .get(&ref_class_name)
                                    .unwrap()
                                    .get_cloned(&constructor_descriptor)
                                    .expect("constructor mask");

                                if !map_var_field_pairs.contains_key(&ref_class_name) {
                                    map_var_field_pairs
                                        .put(ref_class_name.clone(), JHashMap::new());
                                }

                                let mut lst_temp: Vec<Option<VarFieldPair>> = Vec::new();

                                for (i, mask_pair) in mask.iter().enumerate() {
                                    let param = constructor.get_lst_parameters().get(i);
                                    let mut pair: Option<VarFieldPair> = None;

                                    if let (Some(param), Some(mask_pair)) =
                                        (param.try_as_var(), mask_pair)
                                    {
                                        let var_pair = VarVersionPair::from_var(param);

                                        let (field_key, assigned) =
                                            mask_pair.with(|d| (d.field_key.clone(), d.assigned));
                                        pair =
                                            Some(VarFieldPair::new(&field_key, var_pair, assigned));
                                    }

                                    lst_temp.push(pair);
                                }

                                let pair_mask = map_var_field_pairs
                                    .get(&ref_class_name)
                                    .unwrap()
                                    .get_cloned(&constructor_descriptor);
                                let pair_mask = match pair_mask {
                                    None => lst_temp,
                                    Some(mut pair_mask) => {
                                        for i in 0..pair_mask.len() {
                                            if !InterpreterUtil::equal_objects(
                                                pair_mask[i].as_ref(),
                                                lst_temp[i].as_ref(),
                                            ) {
                                                pair_mask[i] = None;
                                            }
                                        }
                                        pair_mask
                                    }
                                };

                                map_var_field_pairs
                                    .get_mut(&ref_class_name)
                                    .unwrap()
                                    .put(constructor_descriptor, pair_mask);

                                let nested_enclosing_method =
                                    nested_class_node.with(|d| d.enclosing_method.clone());
                                if nested_enclosing_method.is_none()
                                    || method_with_key(wrapper, nested_enclosing_method.as_ref())
                                        .is_none()
                                {
                                    let mt = method.with(|d| d.method_struct);
                                    let key = InterpreterUtil::make_unique_key(
                                        &mt.get_name(),
                                        &mt.get_descriptor(),
                                    );
                                    nested_class_node.with_mut(|d| d.enclosing_method = Some(key));
                                }
                            }
                        }

                        0
                    });
                }
            }
        }

        for (enclosing_key, enclosing_value) in map_var_masks.entry_vec() {
            let nested_node = node
                .get_class_node(&enclosing_key)
                .expect("nested class node");

            let mut inter_pair_mask: Option<Vec<Option<VarFieldPair>>> = None;
            if map_var_field_pairs.contains_key(&enclosing_key) {
                let masks = map_var_field_pairs.get(&enclosing_key).unwrap().value_vec();
                for mut mask in masks {
                    match inter_pair_mask.as_mut() {
                        None => inter_pair_mask = Some(mask),
                        Some(inter) => Self::merge_list_signatures(inter, &mut mask, false),
                    }
                }
            }

            let mut inter_mask: Option<Vec<Option<VarFieldPair>>> = None;
            for mut mask in enclosing_value.value_vec() {
                match inter_mask.as_mut() {
                    None => inter_mask = Some(mask),
                    Some(inter) => Self::merge_list_signatures(inter, &mut mask, false),
                }
            }

            let mut inter_pair_mask = match inter_pair_mask {
                Some(inter_pair_mask) => inter_pair_mask,
                None => {
                    let mut inter_pair_mask = inter_mask.clone().unwrap_or_default();

                    let mut found = false;

                    for slot in inter_pair_mask.iter_mut() {
                        if slot.is_some() {
                            if found {
                                *slot = None;
                            }
                            found = true;
                        }
                    }
                    inter_pair_mask
                }
            };

            let mut inter_mask = inter_mask.expect("interMask");
            Self::merge_list_signatures(&mut inter_pair_mask, &mut inter_mask, true);

            for pair in inter_pair_mask.iter().flatten() {
                let (field_key, assigned, var_pair) =
                    pair.with(|d| (d.field_key.clone(), d.assigned, d.var_pair));
                if !field_key.is_empty() && assigned {
                    nested_node.with_mut(|d| d.map_fields_to_vars.put(field_key, var_pair));
                }
            }

            for (descriptor, mut signature) in enclosing_value.entry_vec() {
                let mut second = inter_pair_mask.clone();
                Self::merge_list_signatures(&mut signature, &mut second, false);

                let mut mask: Vec<Option<VarVersionPair>> = Vec::with_capacity(signature.len());
                for pair in &signature {
                    mask.push(pair.and_then(|pair| {
                        pair.with(|d| (!d.field_key.is_empty()).then_some(d.var_pair))
                    }));
                }
                nested_node
                    .get_wrapper()
                    .expect("ClassWrapper")
                    .get_method_wrapper(INIT_NAME, &descriptor)
                    .expect("constructor wrapper")
                    .with_mut(|d| d.synth_parameters = Some(mask));
            }
        }
    }

    fn find_enclosing_method(child: ClassNode) -> Option<MethodWrapper> {
        let (enclosing_method, parent, type_, enclosing_classes, class_struct) = child.with(|d| {
            (
                d.enclosing_method.clone(),
                d.parent,
                d.type_,
                d.enclosing_classes.clone(),
                d.class_struct,
            )
        });
        let enclosing_method = enclosing_method?;
        if let Some(parent) = parent {
            let from_parent = parent
                .get_wrapper()
                .unwrap()
                .get_methods()
                .borrow()
                .get_with_key(&enclosing_method);
            if from_parent.is_some() {
                return from_parent;
            }
        }
        let enclosing_class_names = enclosing_classes.borrow().to_vec();
        for enclosing_class_name in enclosing_class_names {
            let enclosing_class = DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get_cloned(&enclosing_class_name);
            if let Some(enclosing_class) = enclosing_class {
                let from_enclosing = enclosing_class
                    .get_wrapper()
                    .unwrap()
                    .get_methods()
                    .borrow()
                    .get_with_key(&enclosing_method);
                if from_enclosing.is_some() {
                    return from_enclosing;
                }
            }
        }
        if type_ != ClassNodeType::Member {
            let qualified_name = class_struct.qualified_name();
            let enclosing = collection_to_string(enclosing_classes.borrow().iter());
            warn(&format!(
                "Couldn't find enclosing method \"{enclosing_method}\" of {qualified_name} in {enclosing}"
            ));
        }
        None
    }

    fn insert_local_vars(parent: ClassNode, child: ClassNode) {
        let enclosing_method = Self::find_enclosing_method(child);

        let (child_type, child_class) = child.with(|d| (d.type_, d.class_struct));
        let methods = child
            .get_wrapper()
            .expect("ClassWrapper")
            .get_methods()
            .borrow()
            .to_vec();
        for method in methods {
            if method.with(|d| d.root.is_none()) {
                continue;
            }

            let mut map_new_names: JHashMap<VarVersionPair, Option<String>> = JHashMap::new();
            let mut map_new_types: JHashMap<VarVersionPair, Option<VarType>> = JHashMap::new();
            let mut map_new_lvts: JHashMap<VarVersionPair, Option<LocalVariable>> = JHashMap::new();

            let (method_struct, varproc, counter, synth_parameters) = method.with(|d| {
                (
                    d.method_struct,
                    d.varproc,
                    d.counter,
                    d.synth_parameters.clone(),
                )
            });

            if let Some(enclosing_method) = enclosing_method {
                let namer = method_struct.get_variable_namer();
                namer.add_parent_context(
                    enclosing_method
                        .with(|d| d.method_struct)
                        .get_variable_namer(),
                );
            }

            let mut map_params_to_new_vars: JHashMap<i32, VarVersionPair> = JHashMap::new();
            if let Some(synth_parameters) = synth_parameters {
                let mut var_index: i32 = 1;
                let md = MethodDescriptor::parse_descriptor(&method_struct.get_descriptor());

                for (index, pair) in synth_parameters.into_iter().enumerate() {
                    if let Some(pair) = pair {
                        let new_var = VarVersionPair::new(
                            counter.get_counter_and_increment(CounterContainer::VAR_COUNTER),
                            0,
                        );

                        map_params_to_new_vars.put(var_index, new_var);

                        let mut var_name: Option<String> = None;
                        let mut var_type: Option<VarType> = None;
                        let mut var_lvt: Option<LocalVariable> = None;

                        if child_type != ClassNodeType::Member {
                            let enclosing_varproc = enclosing_method
                                .expect("enclosing method")
                                .with(|d| d.varproc);
                            var_name = enclosing_varproc.get_var_name(pair);
                            var_type = enclosing_varproc.get_var_type(pair);
                            var_lvt = enclosing_varproc.get_var_lvt(pair);

                            enclosing_varproc.set_var_final(pair, FinalType::ExplicitFinal);
                        }

                        if pair.var == -1
                            || var_name.as_deref() == Some("this")
                            || var_lvt.as_ref().is_some_and(|lvt| lvt.get_name() == "this")
                        {
                            let (parent_simple_name, parent_class) =
                                parent.with(|d| (d.simple_name.clone(), d.class_struct));
                            let name = match parent_simple_name {
                                None => VarExprent::VAR_NAMELESS_ENCLOSURE.to_string(),
                                Some(simple_name) => format!("{simple_name}.this"),
                            };
                            if let Some(lvt) = &var_lvt {
                                var_lvt = Some(lvt.rename(name.clone()));
                            }
                            var_name = Some(name);
                            varproc
                                .get_this_vars()
                                .borrow_mut()
                                .put(new_var, parent_class.qualified_name());
                        }

                        map_new_names.put(new_var, var_name);
                        map_new_types.put(new_var, var_type);
                        map_new_lvts.put(new_var, var_lvt);
                    }

                    var_index += md.params[index].stack_size;
                }
            }

            let mut map_fields_to_new_vars: JHashMap<String, VarVersionPair> = JHashMap::new();
            let mut class_node = Some(child);
            while let Some(cn) = class_node {
                let (entries, cn_type, cn_class) =
                    cn.with(|d| (d.map_fields_to_vars.entry_vec(), d.type_, d.class_struct));
                for (field_key, field_var) in entries {
                    let new_var = VarVersionPair::new(
                        counter.get_counter_and_increment(CounterContainer::VAR_COUNTER),
                        0,
                    );

                    map_fields_to_new_vars.put(
                        InterpreterUtil::make_unique_key(&cn_class.qualified_name(), &field_key),
                        new_var,
                    );

                    let mut var_name: Option<String> = None;
                    let mut var_type: Option<VarType> = None;
                    let mut var_lvt: Option<LocalVariable> = None;

                    if cn_type != ClassNodeType::Member {
                        let enclosing_method =
                            Self::find_enclosing_method(cn).expect("enclosing method");
                        let enclosing_varproc = enclosing_method.with(|d| d.varproc);

                        var_name = enclosing_varproc.get_var_name(field_var);
                        var_type = enclosing_varproc.get_var_type(field_var);
                        var_lvt = enclosing_varproc.get_var_lvt(field_var);

                        enclosing_varproc.set_var_final(field_var, FinalType::ExplicitFinal);
                    }

                    if field_var.var == -1
                        || var_name.as_deref() == Some("this")
                        || var_lvt.as_ref().is_some_and(|lvt| lvt.get_name() == "this")
                    {
                        let cn_parent = cn.with(|d| d.parent).expect("ClassNode.parent");
                        let (parent_simple_name, parent_class) =
                            cn_parent.with(|d| (d.simple_name.clone(), d.class_struct));
                        let name = match parent_simple_name {
                            None => VarExprent::VAR_NAMELESS_ENCLOSURE.to_string(),
                            Some(simple_name) => format!("{simple_name}.this"),
                        };
                        if let Some(lvt) = &var_lvt {
                            var_lvt = Some(lvt.rename(name.clone()));
                        }
                        var_name = Some(name);
                        varproc
                            .get_this_vars()
                            .borrow_mut()
                            .put(new_var, parent_class.qualified_name());
                    }

                    map_new_names.put(new_var, var_name);
                    map_new_types.put(new_var, var_type);
                    map_new_lvts.put(new_var, var_lvt);

                    if cn == child {
                        let fd = child_class.get_fields().get_with_key(&field_key);
                        if let Some(fd) = fd {
                            child
                                .get_wrapper()
                                .expect("ClassWrapper")
                                .get_hidden_members()
                                .borrow_mut()
                                .add(InterpreterUtil::make_unique_key(
                                    &fd.get_name(),
                                    &fd.get_descriptor(),
                                ));
                        }
                    }
                }
                class_node = cn.with(|d| d.parent);
            }

            let mut set_new_outer_names: JHashSet<String> = JHashSet::from_collection(
                map_new_names
                    .values()
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>(),
            );
            method.with(|d| {
                set_new_outer_names.remove_all(&d.set_outer_var_names);
            });

            varproc.refresh_var_names(VarNamesCollector::new_with_names(
                set_new_outer_names.to_vec(),
            ));
            let new_outer_names = set_new_outer_names.to_vec();
            method.with_mut(|d| {
                d.set_outer_var_names.add_all(new_outer_names);
            });

            for (pair, name) in map_new_names.entry_vec() {
                let type_ = map_new_types.get(&pair).cloned().flatten();
                let lvt = map_new_lvts.get(&pair).cloned().flatten();

                if let Some(name) = name {
                    varproc.set_inherited_name(pair, name);
                }
                if let Some(type_) = type_ {
                    varproc.set_var_type(pair, &type_);
                }
                if let Some(lvt) = lvt {
                    varproc.set_var_lvt(pair, lvt);
                }
            }

            let graph = method.get_or_build_graph().expect("DirectGraph");
            Self::iterate_exprents(graph, &mut |exprent: Exprent| -> Option<Exprent> {
                if let Some(assign_expr) = exprent.try_as_assignment() {
                    if let Some(f_expr) = assign_expr.get_left().try_as_field() {
                        let q_name = child_class.qualified_name();
                        if f_expr.get_classname() == q_name
                            && map_fields_to_new_vars.contains_key(
                                &InterpreterUtil::make_unique_key_3(
                                    &q_name,
                                    &f_expr.get_name(),
                                    &f_expr.get_descriptor().descriptor_string,
                                ),
                            )
                        {
                            return None;
                        }
                    }
                }

                if child_type == ClassNodeType::Anonymous
                    && INIT_NAME == method_struct.get_name()
                    && exprent.is_invocation()
                {
                    let invoke_expr = exprent.as_invocation();
                    if invoke_expr.get_functype() == InvocationKind::Init {
                        child.with_mut(|d| d.super_invocation = Some(invoke_expr));
                        return None;
                    }
                }

                if INIT_NAME == method_struct.get_name() {
                    if let Some(inv) = exprent.try_as_invocation() {
                        if inv.is_static()
                            && inv.get_classname() == "java/util/Objects"
                            && inv.get_name() == "requireNonNull"
                            && inv.get_string_descriptor()
                                == "(Ljava/lang/Object;)Ljava/lang/Object;"
                        {
                            if let Some(var_exp) = inv.get_lst_parameters().get(0).try_as_var() {
                                if map_params_to_new_vars.contains_key(&var_exp.get_index()) {
                                    return None;
                                }
                            }
                        }
                    }
                }

                let ret = Self::replace_exprent(
                    exprent,
                    method,
                    &map_params_to_new_vars,
                    &map_fields_to_new_vars,
                );

                Some(ret.unwrap_or(exprent))
            });
        }
    }

    /// The `replaceExprent` helper of `insertLocalVars`'s exprent iterator.
    fn replace_exprent(
        exprent: Exprent,
        method: MethodWrapper,
        map_params_to_new_vars: &JHashMap<i32, VarVersionPair>,
        map_fields_to_new_vars: &JHashMap<String, VarVersionPair>,
    ) -> Option<Exprent> {
        if let Some(var) = exprent.try_as_var() {
            let var_index = var.get_index();
            if map_params_to_new_vars.contains_key(&var_index) {
                let new_var = *map_params_to_new_vars.get(&var_index).unwrap();
                let varproc = method.with(|d| d.varproc);
                varproc.get_external_vars().borrow_mut().add(new_var);
                let ret = VarExprent::new_bytecode(
                    new_var.var,
                    varproc.get_var_type(new_var),
                    Some(varproc),
                    exprent.get_bytecode().as_ref(),
                );
                let lvt = varproc.get_var_lvt(new_var);
                if let Some(lvt) = lvt {
                    ret.set_lvt(Some(lvt));
                }
                return Some(ret.0);
            }
        } else if let Some(f_expr) = exprent.try_as_field() {
            let key = InterpreterUtil::make_unique_key_3(
                &f_expr.get_classname(),
                &f_expr.get_name(),
                &f_expr.get_descriptor().descriptor_string,
            );
            if map_fields_to_new_vars.contains_key(&key) {
                let new_var = *map_fields_to_new_vars.get(&key).unwrap();
                let varproc = method.with(|d| d.varproc);
                varproc.get_external_vars().borrow_mut().add(new_var);
                let ret = VarExprent::new_bytecode(
                    new_var.var,
                    varproc.get_var_type(new_var),
                    Some(varproc),
                    exprent.get_bytecode().as_ref(),
                );
                let lvt = varproc.get_var_lvt(new_var);
                if let Some(lvt) = lvt {
                    ret.set_lvt(Some(lvt));
                }
                return Some(ret.0);
            }
        }

        let mut replaced = true;
        while replaced {
            replaced = false;

            for expr in exprent.get_all_exprents() {
                let ret_expr = Self::replace_exprent(
                    expr,
                    method,
                    map_params_to_new_vars,
                    map_fields_to_new_vars,
                );
                if let Some(ret_expr) = ret_expr {
                    exprent.replace_exprent(expr, ret_expr);
                    replaced = true;
                    break;
                }
            }
        }

        None
    }

    fn get_mask_local_vars(wrapper: ClassWrapper) -> JHashMap<String, Vec<Option<VarFieldPair>>> {
        let mut map_masks: JHashMap<String, Vec<Option<VarFieldPair>>> = JHashMap::new();

        let cl = wrapper.get_class_struct();

        for p in 0..2 {
            let mut found = false;

            let methods = cl.get_methods().to_vec();
            for mt in methods {
                if INIT_NAME == mt.get_name() {
                    let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
                    let method = wrapper
                        .get_method_wrapper(INIT_NAME, &mt.get_descriptor())
                        .expect("constructor wrapper");
                    let graph = method.get_or_build_graph();

                    if let Some(graph) = graph {
                        let mut fields: Vec<Option<VarFieldPair>> =
                            Vec::with_capacity(md.params.len());

                        let mut var_index: i32 = 1;
                        for i in 0..md.params.len() {
                            let key_field = Self::get_enclosing_var_field(
                                cl,
                                method,
                                graph,
                                var_index,
                                i as i32,
                                p == 1 || cl.get_methods().len() == 1,
                            );
                            if key_field.as_ref().is_some_and(|(a, _)| !a.is_empty()) {
                                found = true;
                            }

                            fields.push(key_field.map(|(a, b)| {
                                VarFieldPair::new(&a, VarVersionPair::new(-1, 0), b)
                            }));
                            var_index += md.params[i].stack_size;
                        }

                        map_masks.put(mt.get_descriptor(), fields);
                    }
                }
            }

            if found {
                break;
            }
        }

        map_masks
    }

    fn get_enclosing_var_field(
        cl: StructClass,
        method: MethodWrapper,
        graph: DirectGraph,
        index: i32,
        outer_idx: i32,
        use_heuristic: bool,
    ) -> Option<(String, bool)> {
        let mut field = String::new();
        let mut assigned = false;

        let var = VarVersionPair::new(index, 0);
        let varproc = method.with(|d| d.varproc);
        if varproc.get_var_final(var) == FinalType::NonFinal {
            return None;
        }

        let no_synth_flag =
            DecompilerContext::get_option(IFernflowerPreferences::SYNTHETIC_NOT_SET);

        let first_node: DirectNode = graph.first().expect("DirectGraph.first");
        if !first_node.has_predecessors(DirectEdgeType::Regular) {
            for exprent in first_node.exprents().to_vec() {
                if let Some(assign_expr) = exprent.try_as_assignment() {
                    let right = assign_expr.get_right();
                    let left = assign_expr.get_left();
                    if right.try_as_var().is_some_and(|v| v.get_index() == index) && left.is_field()
                    {
                        let left = left.as_field();
                        let left_descriptor = left.get_descriptor().descriptor_string.clone();
                        let fd = cl.get_field(&left.get_name(), &left_descriptor);
                        if let Some(fd) = fd {
                            if cl.qualified_name() == left.get_classname()
                                && (fd.is_synthetic()
                                    || no_synth_flag && Self::possibly_synthetic_field(fd))
                            {
                                field = InterpreterUtil::make_unique_key(
                                    &left.get_name(),
                                    &left_descriptor,
                                );
                                assigned = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if field.is_empty() && cl.get_version().major >= 21 {
            let mt = method.with(|d| d.method_struct);
            if mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS) {
                let attr = mt
                    .get_attribute(StructGeneralAttribute::ATTRIBUTE_METHOD_PARAMETERS)
                    .expect("MethodParameters");

                let entries = attr.get_entries();
                if (outer_idx as usize) < entries.len()
                    && (entries[outer_idx as usize].my_access_flags
                        & (ACC_MANDATED | ACC_SYNTHETIC))
                        != 0
                {
                    let name = varproc.get_var_name(var);
                    let type_ = varproc.get_var_type(var);
                    field = InterpreterUtil::make_unique_key(
                        name.as_deref().unwrap_or("null"),
                        &type_.expect("VarType").to_string(),
                    );
                    assigned = false;
                }
            }
        }

        if field.is_empty() && cl.get_version().major >= 21 {
            if index == 1 && cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_NEST_HOST) {
                let host = cl
                    .get_attribute(StructGeneralAttribute::ATTRIBUTE_NEST_HOST)
                    .expect("NestHost");
                let host_name = host.get_host_class(cl.get_pool());

                let name = varproc.get_var_name(var);
                let type_ = varproc.get_var_type(var).expect("VarType");
                if type_.value.as_deref() == Some(host_name.as_str()) && use_heuristic {
                    field = InterpreterUtil::make_unique_key(
                        name.as_deref().unwrap_or("null"),
                        &type_.to_string(),
                    );
                    assigned = false;
                } else {
                    let nd = DecompilerContext::get_class_processor()
                        .get_map_root_classes()
                        .borrow()
                        .get_cloned(&cl.qualified_name());

                    if let Some(nd) = nd {
                        if nd.with(|d| d.type_) == ClassNodeType::Anonymous {
                            let enclosing_classes = nd.with(|d| d.enclosing_classes.clone());
                            let enclosing_classes = enclosing_classes.borrow().to_vec();
                            for clazz in enclosing_classes {
                                if type_.value.as_deref() == Some(clazz.as_str()) {
                                    field = InterpreterUtil::make_unique_key(
                                        name.as_deref().unwrap_or("null"),
                                        &type_.to_string(),
                                    );
                                    assigned = false;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        Some((field, assigned))
    }

    fn possibly_synthetic_field(fd: StructField) -> bool {
        fd.get_name().contains('$') && fd.has_modifier(ACC_FINAL) && fd.has_modifier(ACC_PRIVATE)
    }

    fn merge_list_signatures(
        first: &mut [Option<VarFieldPair>],
        second: &mut [Option<VarFieldPair>],
        both: bool,
    ) {
        let mut i: i32 = 1;

        while first.len() as i32 > i && second.len() as i32 > i {
            let fi = (first.len() as i32 - i) as usize;
            let si = (second.len() as i32 - i) as usize;
            let f_obj = first[fi];
            let s_obj = second[si];

            if !Self::is_equal(both, f_obj, s_obj) {
                first[fi] = None;
                if both {
                    second[si] = None;
                }
            } else if let Some(f_obj) = f_obj {
                Self::merge_var_pairs(f_obj, s_obj.unwrap());
            }

            i += 1;
        }

        let mut j: i32 = 1;
        while j <= first.len() as i32 - i {
            first[j as usize] = None;
            j += 1;
        }

        if both {
            let mut j: i32 = 1;
            while j <= second.len() as i32 - i {
                second[j as usize] = None;
                j += 1;
            }
        }

        if first.is_empty() {
            if !second.is_empty() && both {
                second[0] = None;
            }
        } else if second.is_empty() {
            first[0] = None;
        } else {
            let f_obj = first[0];
            let s_obj = second[0];

            if !Self::is_equal(both, f_obj, s_obj) {
                first[0] = None;
                if both {
                    second[0] = None;
                }
            } else if let Some(f_obj) = f_obj {
                Self::merge_var_pairs(f_obj, s_obj.unwrap());
            }
        }
    }

    /// The `fObj.varPair = sObj.varPair` / `sObj.varPair = fObj.varPair` step of `mergeListSignatures`.
    fn merge_var_pairs(f_obj: VarFieldPair, s_obj: VarFieldPair) {
        let f_pair = f_obj.with(|d| d.var_pair);
        if f_pair.var == -1 {
            let s_pair = s_obj.with(|d| d.var_pair);
            f_obj.with_mut(|d| d.var_pair = s_pair);
        } else {
            s_obj.with_mut(|d| d.var_pair = f_pair);
        }
    }

    fn is_equal(both: bool, f_obj: Option<VarFieldPair>, s_obj: Option<VarFieldPair>) -> bool {
        let (f_obj, s_obj) = match (f_obj, s_obj) {
            (Some(f), Some(s)) => (f, s),
            (f, s) => return f == s,
        };

        let mut eq = true;
        let (f_assigned, f_key) = f_obj.with(|d| (d.assigned, d.field_key.clone()));
        let (s_assigned, s_key) = s_obj.with(|d| (d.assigned, d.field_key.clone()));
        if !f_assigned || f_key.is_empty() {
            f_obj.with_mut(|d| {
                d.field_key = s_key;
                d.assigned = s_assigned;
            });
        } else if !s_assigned || s_key.is_empty() {
            if both {
                s_obj.with_mut(|d| {
                    d.field_key = f_key;
                    d.assigned = f_assigned;
                });
            }
        } else {
            eq = f_key == s_key;
        }
        eq
    }

    fn set_local_class_definition(method: MethodWrapper, node: ClassNode) {
        let root = method.with(|d| d.root);

        let Some(root) = root else {
            method.add_comment(
                "$VF: Couldn't set local class definition as local class statement was null",
            );
            method.with_mut(|d| d.add_error_comment = true);

            return;
        };

        let mut set_stats: JHashSet<Statement> = JHashSet::new();
        let class_type =
            VarType::new_signature_cl(&node.with(|d| d.class_struct).qualified_name(), true);

        let statement = Self::get_def_statement(root.0, &class_type, &mut set_stats);
        let statement = match statement {
            Some(statement) => statement,
            None => root.get_first().expect("RootStatement.first"),
        };

        let first = Self::find_first_block(statement, &set_stats);

        let lst: JList<Exprent> = match first {
            None => statement.get_var_definitions(),
            Some(first) => match first.get_exprents() {
                None => first.get_var_definitions(),
                Some(exprents) => exprents,
            },
        };

        let mut add_index = 0;
        for expr in lst.to_vec() {
            if Self::search_for_class(expr, &class_type) {
                break;
            }
            add_index += 1;
        }

        let (counter, varproc) = method.with(|d| (d.counter, d.varproc));
        let index = counter.get_counter_and_increment(CounterContainer::VAR_COUNTER);
        let var = VarExprent::new(index, Some(class_type), Some(varproc));
        var.set_definition(true);
        var.set_class_def(true);

        lst.add_at(add_index, var.0);
    }

    fn find_first_block(stat: Statement, set_stats: &JHashSet<Statement>) -> Option<Statement> {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(stat);

        while let Some(st) = stack.pop_front() {
            if stack.is_empty() || set_stats.contains(&st) {
                if st.is_labeled() && !stack.is_empty() || st.get_exprents().is_some() {
                    return Some(st);
                }

                stack.clear();

                match st.type_() {
                    StatementType::Sequence => {
                        let stats = st.get_stats().borrow().to_vec();
                        for (k, s) in stats.into_iter().enumerate() {
                            stack.insert(k, s);
                        }
                    }
                    StatementType::If
                    | StatementType::Root
                    | StatementType::Switch
                    | StatementType::Synchronized => {
                        stack.push_back(st.get_first().expect("Statement.first"));
                    }
                    _ => return Some(st),
                }
            }
        }

        None
    }

    fn get_def_statement(
        stat: Statement,
        class_type: &VarType,
        set_stats: &mut JHashSet<Statement>,
    ) -> Option<Statement> {
        let mut lst: Vec<Exprent> = Vec::new();
        let mut ret_stat: Option<Statement> = None;

        match stat.get_exprents() {
            None => {
                let mut counter = 0;

                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    let st_temp = Self::get_def_statement(st, class_type, set_stats);

                    if st_temp.is_some() {
                        if counter == 1 {
                            ret_stat = Some(stat);
                            break;
                        }
                        ret_stat = st_temp;
                        counter += 1;
                    }

                    if let Some(dost) = st.try_as_do() {
                        lst.extend(dost.get_init_exprent_list().to_vec());
                        lst.extend(dost.get_condition_exprent_list().to_vec());
                    }
                }

                lst.extend(stat.get_stat_exprents());
            }
            Some(exprents) => {
                lst = exprents.to_vec();
            }
        }

        if ret_stat != Some(stat) {
            for exprent in lst {
                if Self::search_for_class(exprent, class_type) {
                    ret_stat = Some(stat);
                    break;
                }
            }
        }

        if ret_stat.is_some() {
            set_stats.add(stat);
        }

        ret_stat
    }

    fn search_for_class(exprent: Exprent, class_type: &VarType) -> bool {
        let mut lst = exprent.get_all_exprents_rec(true);
        lst.push(exprent);

        let classname = class_type.value_str();

        for expr in lst {
            let mut res = false;

            match expr.get_type() {
                ExprentType::Const => {
                    let const_expr = expr.as_const();
                    let const_type = const_expr.get_const_type();
                    res = VarType::VARTYPE_CLASS().equals(&const_type)
                        && matches!(const_expr.get_value(), Some(ConstValue::String(value)) if value == classname)
                        || class_type.equals(&const_type);
                }
                ExprentType::Field => {
                    res = classname == expr.as_field().get_classname();
                }
                ExprentType::Invocation => {
                    res = Self::contains_type(Some(expr.as_invocation()), class_type);
                }
                ExprentType::New => {
                    let new_expr = expr.as_new();
                    let new_type = new_expr.get_new_type();
                    if new_expr.is_lambda() {
                        res = true;
                    } else {
                        res = new_type.type_ == CodeType::Object
                            && new_type.value.as_deref() == Some(classname)
                            || Self::contains_type(new_expr.get_constructor(), class_type);
                    }
                }
                ExprentType::Var => {
                    let var_expr = expr.as_var();
                    if var_expr.is_definition() {
                        let inferred = var_expr.get_inferred_expr_type(None);
                        res = Self::contains_type_var_type(Some(&inferred), Some(class_type));
                    }
                }
                _ => {}
            }

            if res {
                return true;
            }
        }

        false
    }

    /// Java: containsType(InvocationExprent, VarType)
    fn contains_type(haystack: Option<InvocationExprent>, needle: &VarType) -> bool {
        let Some(haystack) = haystack else {
            return false;
        };
        if needle.value_str() == haystack.get_classname() {
            return true;
        }
        let Some(bootstrap_args) = haystack.get_bootstrap_arguments() else {
            return false;
        };
        for bootstrap_arg in bootstrap_args.into_iter().flatten() {
            if let Some(link) = bootstrap_arg.try_as_link() {
                if link.classname.as_deref() == Some(needle.value_str()) {
                    return true;
                }
            }
        }
        false
    }

    /// Java: containsType(VarType, VarType)
    fn contains_type_var_type(haystack: Option<&VarType>, needle: Option<&VarType>) -> bool {
        let (Some(haystack), Some(needle)) = (haystack, needle) else {
            return false;
        };
        if needle.equals(haystack)
            || (haystack.array_dim > 0 && needle.value.as_deref() == Some(haystack.value_str()))
        {
            return true;
        }
        if haystack.is_generic() {
            let arguments = haystack.as_generic().get_arguments().to_vec();
            for arg in arguments {
                if Self::contains_type_var_type(arg.as_ref(), Some(needle)) {
                    return true;
                }
            }
        }
        false
    }

    /// `processExprent` returns `None` to remove the exprent, or its replacement.
    fn iterate_exprents(graph: DirectGraph, iter: &mut ExprentIteratorWithReplace<'_>) {
        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        stack.push_back(graph.first().expect("DirectGraph.first"));

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();

        while let Some(node) = stack.pop_front() {
            if set_visited.contains(&node) {
                continue;
            }
            set_visited.add(node);

            let mut i: i32 = 0;
            while (i as usize) < node.exprents().len() {
                let res = iter(node.exprents().get(i as usize));

                match res {
                    None => {
                        node.exprents().remove_at(i as usize);
                        i -= 1;
                    }
                    Some(res) => {
                        if res != node.exprents().get(i as usize) {
                            node.exprents().set(i as usize, res);
                        }
                    }
                }
                i += 1;
            }

            for suc in node.get_successors(DirectEdgeType::Regular).iter() {
                stack.push_back(suc.get_destination());
            }
        }
    }
}

/// `ExprentIteratorWithReplace.processExprent`.
type ExprentIteratorWithReplace<'a> = dyn FnMut(Exprent) -> Option<Exprent> + 'a;

struct VarFieldPairData {
    field_key: String,
    var_pair: VarVersionPair,
    assigned: bool,
}

#[allow(dead_code)]
mod var_field_pair_handle {
    crate::arena_object!(pub(super) VarFieldPair => super::VarFieldPairData, Class);
}
use var_field_pair_handle::VarFieldPair;

impl VarFieldPair {
    fn new(field: &str, var_pair: VarVersionPair, assigned: bool) -> VarFieldPair {
        VarFieldPair::alloc(VarFieldPairData {
            field_key: field.to_string(),
            var_pair,
            assigned,
        })
    }

    fn equals(self, o: VarFieldPair) -> bool {
        if o == self {
            return true;
        }

        let (a_key, a_pair, a_assigned) =
            self.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        let (b_key, b_pair, b_assigned) = o.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        a_key == b_key && a_pair == b_pair && a_assigned == b_assigned
    }
}

impl JavaEq for VarFieldPair {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(*other)
    }
}

impl JavaHash for VarFieldPair {
    fn java_hash(&self) -> i32 {
        self.with(|d| {
            java_string_hash(&d.field_key)
                .wrapping_add(d.var_pair.java_hash())
                .wrapping_add(d.assigned.java_hash())
        })
    }
}

impl fmt::Display for VarFieldPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (field_key, var_pair, assigned) =
            self.with(|d| (d.field_key.clone(), d.var_pair, d.assigned));
        let link = if assigned {
            " assigned to "
        } else {
            " stored in variable "
        };
        write!(f, "{var_pair}{link}{field_key}")
    }
}
