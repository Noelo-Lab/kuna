//! Port of `org/jetbrains/java/decompiler/modules/renamer/IdentifierConverter.java`.

use std::collections::VecDeque;
use std::rc::Rc;

use crate::class_wrapper_node::ClassWrapperNode;
use crate::code_constants::{ACC_INTERFACE, ACC_NATIVE, ACC_PRIVATE};
use crate::converter_helper::ConverterHelper;
use crate::field_descriptor::FieldDescriptor;
use crate::i_identifier_renamer::{IIdentifierRenamer, Type};
use crate::method_descriptor::MethodDescriptor;
use crate::new_class_name_builder::NewClassNameBuilder;
use crate::pool_interceptor::PoolInterceptor;
use crate::rt::{rethrow, try_catch, JHashSet, JLinkedHashMap};
use crate::struct_class::StructClass;
use crate::struct_context::{
    is_io_exception, qualified_name_of, super_class_name_of, StructContext,
};

type NameMap = JLinkedHashMap<String, String>;

pub struct IdentifierConverter {
    context: StructContext,
    helper: Rc<dyn IIdentifierRenamer>,
    interceptor: PoolInterceptor,
    root_classes: Vec<ClassWrapperNode>,
    root_interfaces: Vec<ClassWrapperNode>,
    interface_name_maps: JLinkedHashMap<String, NameMap>,
}

impl IdentifierConverter {
    pub fn new(
        context: StructContext,
        helper: Rc<dyn IIdentifierRenamer>,
        interceptor: PoolInterceptor,
    ) -> IdentifierConverter {
        IdentifierConverter {
            context,
            helper,
            interceptor,
            root_classes: Vec::new(),
            root_interfaces: Vec::new(),
            interface_name_maps: JLinkedHashMap::new(),
        }
    }

    pub fn rename(&mut self) {
        let result = try_catch(|| {
            self.build_inheritance_tree();
            self.rename_all_classes();
            self.rename_interfaces();
            self.rename_classes();
            self.context.reload_context();
        });
        if let Err(ex) = result {
            if is_io_exception(&ex) {
                crate::jthrow!("Renaming failed with exception!");
            }
            rethrow(ex);
        }
    }

    fn rename_classes(&self) {
        let lst_classes = Self::get_reverse_post_order_list_iterative(&self.root_classes);
        let mut class_name_maps: JLinkedHashMap<String, NameMap> = JLinkedHashMap::new();

        for node in lst_classes {
            let cl = node.get_class_struct();
            let mut names = NameMap::new();

            if let Some(super_class) = super_class_name_of(cl) {
                if let Some(map_class) = class_name_maps.get(&super_class) {
                    names.put_all(map_class.entry_vec());
                }
            }

            for if_name in cl.get_interface_names() {
                if let Some(map_int) = self.interface_name_maps.get(&if_name) {
                    names.put_all(map_int.entry_vec());
                } else {
                    let clintr = self.context.get_class(&if_name);
                    if let Some(clintr) = clintr {
                        names.put_all(self.process_external_interface(clintr).entry_vec());
                    }
                }
            }

            self.rename_class_identifiers(cl, &mut names);

            if !node.get_subclasses().is_empty() {
                class_name_maps.put(qualified_name_of(cl), names);
            }
        }
    }

    fn process_external_interface(&self, cl: StructClass) -> NameMap {
        let mut names = NameMap::new();

        for if_name in cl.get_interface_names() {
            if let Some(map_int) = self.interface_name_maps.get(&if_name) {
                names.put_all(map_int.entry_vec());
            } else {
                let clintr = self.context.get_class(&if_name);
                if let Some(clintr) = clintr {
                    names.put_all(self.process_external_interface(clintr).entry_vec());
                }
            }
        }

        self.rename_class_identifiers(cl, &mut names);

        names
    }

    fn rename_interfaces(&mut self) {
        let lst_interfaces = Self::get_reverse_post_order_list_iterative(&self.root_interfaces);
        let mut interface_name_maps: JLinkedHashMap<String, NameMap> = JLinkedHashMap::new();

        for node in lst_interfaces {
            let cl = node.get_class_struct();
            let mut names = NameMap::new();

            for if_name in cl.get_interface_names() {
                if let Some(map_int) = interface_name_maps.get(&if_name) {
                    names.put_all(map_int.entry_vec());
                }
            }

            self.rename_class_identifiers(cl, &mut names);

            interface_name_maps.put(qualified_name_of(cl), names);
        }

        self.interface_name_maps = interface_name_maps;
    }

    fn rename_all_classes(&self) {
        let mut lst_all_classes =
            Self::get_reverse_post_order_list_iterative(&self.root_interfaces);
        lst_all_classes.extend(Self::get_reverse_post_order_list_iterative(
            &self.root_classes,
        ));

        for node in lst_all_classes {
            self.rename_class(node.get_class_struct());
        }
    }

    fn rename_class(&self, cl: StructClass) {
        if !cl.is_own() {
            return;
        }

        let class_old_full_name = qualified_name_of(cl);

        let cl_simple_name = ConverterHelper::get_simple_class_name(&class_old_full_name);
        if self
            .helper
            .to_be_renamed(Type::ElementClass, Some(&cl_simple_name), None, None)
        {
            let mut class_new_full_name;

            loop {
                let classname = self.helper.get_next_class_name(
                    &class_old_full_name,
                    Some(&ConverterHelper::get_simple_class_name(
                        &class_old_full_name,
                    )),
                );
                class_new_full_name =
                    ConverterHelper::replace_simple_class_name(&class_old_full_name, &classname);
                if !self.context.has_class(&class_new_full_name) {
                    break;
                }
            }

            self.interceptor
                .add_name(&class_old_full_name, &class_new_full_name);
        }
    }

    fn rename_class_identifiers(&self, cl: StructClass, names: &mut NameMap) {
        let class_old_full_name = qualified_name_of(cl);
        let class_new_full_name = self
            .interceptor
            .get_name(&class_old_full_name)
            .unwrap_or_else(|| class_old_full_name.clone());

        let methods = cl.get_methods();
        let mut set_method_names: JHashSet<String> = JHashSet::new();
        for i in 0..methods.size() {
            set_method_names.add(methods.get(i).get_name());
        }

        for i in 0..methods.size() {
            let mt = methods.get(i);
            let key = methods.get_key(i);

            let is_private = mt.has_modifier(ACC_PRIVATE);

            let mut name = mt.get_name();
            if !cl.is_own() || mt.has_modifier(ACC_NATIVE) {
                if !is_private {
                    names.put(key, name);
                }
            } else if self.helper.to_be_renamed(
                Type::ElementMethod,
                Some(&class_old_full_name),
                Some(&name),
                Some(&mt.get_descriptor()),
            ) {
                if is_private || !names.contains_key(&key) {
                    loop {
                        name = self.helper.get_next_method_name(
                            &class_old_full_name,
                            &name,
                            &mt.get_descriptor(),
                        );
                        if !set_method_names.contains(&name) {
                            break;
                        }
                    }

                    if !is_private {
                        names.put(key, name.clone());
                    }
                } else {
                    name = names.get_cloned(&key).expect("names.get(key)");
                }

                self.interceptor.add_name(
                    &format!(
                        "{} {} {}",
                        class_old_full_name,
                        mt.get_name(),
                        mt.get_descriptor()
                    ),
                    &format!(
                        "{} {} {}",
                        class_new_full_name,
                        name,
                        self.build_new_descriptor(false, &mt.get_descriptor())
                    ),
                );
            }
        }

        if !cl.is_own() {
            return;
        }

        let fields = cl.get_fields();
        let mut set_field_names: JHashSet<String> = JHashSet::new();
        for i in 0..fields.size() {
            set_field_names.add(fields.get(i).get_name());
        }

        for i in 0..fields.size() {
            let fd = fields.get(i);
            if self.helper.to_be_renamed(
                Type::ElementField,
                Some(&class_old_full_name),
                Some(&fd.get_name()),
                Some(&fd.get_descriptor()),
            ) {
                let mut new_name;
                loop {
                    new_name = self.helper.get_next_field_name(
                        &class_old_full_name,
                        &fd.get_name(),
                        &fd.get_descriptor(),
                    );
                    if !set_field_names.contains(&new_name) {
                        break;
                    }
                }

                self.interceptor.add_name(
                    &format!(
                        "{} {} {}",
                        class_old_full_name,
                        fd.get_name(),
                        fd.get_descriptor()
                    ),
                    &format!(
                        "{} {} {}",
                        class_new_full_name,
                        new_name,
                        self.build_new_descriptor(true, &fd.get_descriptor())
                    ),
                );
            }
        }
    }

    fn build_new_descriptor(&self, is_field: bool, descriptor: &str) -> String {
        let new_descriptor = if is_field {
            FieldDescriptor::parse_descriptor(descriptor).build_new_descriptor(self)
        } else {
            MethodDescriptor::parse_descriptor(descriptor).build_new_descriptor(self)
        };
        new_descriptor.unwrap_or_else(|| descriptor.to_string())
    }

    fn get_reverse_post_order_list_iterative(roots: &[ClassWrapperNode]) -> Vec<ClassWrapperNode> {
        let mut res: VecDeque<ClassWrapperNode> = VecDeque::new();

        let mut stack_node: Vec<ClassWrapperNode> = Vec::new();
        let mut stack_index: Vec<usize> = Vec::new();

        let mut set_visited: JHashSet<ClassWrapperNode> = JHashSet::new();

        for &root in roots {
            stack_node.push(root);
            stack_index.push(0);
        }

        while let Some(&node) = stack_node.last() {
            let mut index = stack_index.pop().expect("stackIndex.removeLast()");

            set_visited.add(node);

            let lst_subs = node.get_subclasses();

            while index < lst_subs.len() {
                let sub = lst_subs[index];
                if !set_visited.contains(&sub) {
                    stack_index.push(index + 1);
                    stack_node.push(sub);
                    stack_index.push(0);
                    break;
                }
                index += 1;
            }

            if index == lst_subs.len() {
                res.push_front(node);
                stack_node.pop();
            }
        }

        res.into()
    }

    fn build_inheritance_tree(&mut self) {
        let mut nodes: JLinkedHashMap<String, ClassWrapperNode> = JLinkedHashMap::new();
        let classes = self.context.get_own_classes();

        let mut root_classes: Vec<ClassWrapperNode> = Vec::new();
        let mut root_interfaces: Vec<ClassWrapperNode> = Vec::new();

        for cl in classes {
            let mut stack: VecDeque<StructClass> = VecDeque::new();
            let mut stack_sub_nodes: VecDeque<Option<ClassWrapperNode>> = VecDeque::new();

            stack.push_back(cl);
            stack_sub_nodes.push_back(None);

            while let Some(cl_str) = stack.pop_front() {
                let child = stack_sub_nodes
                    .pop_front()
                    .expect("stackSubNodes.removeFirst()");

                let cl_str_name = qualified_name_of(cl_str);
                let existing = nodes.get_cloned(&cl_str_name);
                let is_new_node = existing.is_none();

                let node = match existing {
                    Some(node) => node,
                    None => {
                        let node = ClassWrapperNode::new(cl_str);
                        nodes.put(cl_str_name, node);
                        node
                    }
                };

                if let Some(child) = child {
                    node.add_subclass(child);
                }

                if !is_new_node {
                    break;
                }

                let is_interface = cl_str.has_modifier(ACC_INTERFACE);
                let mut found_parent = false;

                if is_interface {
                    for if_name in cl_str.get_interface_names() {
                        if let Some(cl_parent) = self.context.get_class(&if_name) {
                            stack.push_back(cl_parent);
                            stack_sub_nodes.push_back(Some(node));
                            found_parent = true;
                        }
                    }
                } else if let Some(super_class) = super_class_name_of(cl_str) {
                    if let Some(cl_parent) = self.context.get_class(&super_class) {
                        stack.push_back(cl_parent);
                        stack_sub_nodes.push_back(Some(node));
                        found_parent = true;
                    }
                }

                if !found_parent {
                    if is_interface {
                        root_interfaces.push(node);
                    } else {
                        root_classes.push(node);
                    }
                }
            }
        }

        self.root_classes = root_classes;
        self.root_interfaces = root_interfaces;
    }
}

impl NewClassNameBuilder for IdentifierConverter {
    fn build_new_classname(&self, class_name: &str) -> Option<String> {
        self.interceptor.get_name(class_name)
    }
}
