//! Port of `org/jetbrains/java/decompiler/main/ClassesProcessor.java`.
//!
//! `excluded-classes`/`included-classes` are Java regular expressions; they are
//! matched by the small backtracking engine in [`java_regex`].

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;

use crate::cancelation_manager::CanceledException;
use crate::class_pass_location::ClassPassLocation;
use crate::class_wrapper::ClassWrapper;
use crate::class_writer::ClassWriter;
use crate::code_constants::{
    ACC_ABSTRACT, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_MODULE, ACC_STATIC, ACC_SYNTHETIC,
    CLINIT_NAME, CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC, OPC_ANEWARRAY, OPC_CHECKCAST,
    OPC_GETSTATIC, OPC_INSTANCEOF, OPC_MULTIANEWARRAY, OPC_NEW, OPC_PUTSTATIC,
};
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_identifier_renamer::{IIdentifierRenamer, Type as RenamerType};
use crate::import_collector::ImportCollector;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::lambda_processor::LambdaProcessor;
use crate::nested_class_processor::NestedClassProcessor;
use crate::nested_member_access::NestedMemberAccess;
use crate::plugin_context::{LanguageSpec, PluginContext};
use crate::rt::{
    rethrow, try_catch, Caught, JHashMap, JHashSet, JLinkedHashSet, JList, JavaStr, Shared,
};
use crate::struct_class::StructClass;
use crate::struct_context::{is_io_exception, StructContext};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::switch_helper::SwitchHelper;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object, jthrow};

use self::java_regex::JavaPattern;

/// `String.compareTo`: lexicographic over UTF-16 units.
pub fn string_compare_to(a: &str, b: &str) -> Ordering {
    if a.is_ascii() && b.is_ascii() {
        a.as_bytes().cmp(b.as_bytes())
    } else {
        a.utf16().cmp(&b.utf16())
    }
}

pub struct ClassesProcessorData {
    pub context: StructContext,
    pub map_root_classes: Shared<JHashMap<String, ClassNode>>,
    pub whitelist: JHashSet<String>,
}

arena_object!(pub ClassesProcessor => ClassesProcessorData, Session);
arena_identity!(ClassesProcessor);

#[derive(Clone)]
struct Inner {
    simple_name: Option<String>,
    type_: ClassNodeType,
    access_flags: i32,
    source: String,
    enclosing_name: Option<String>,
}

impl Inner {
    fn equal(o1: &Inner, o2: &Inner) -> bool {
        o1.type_ == o2.type_
            && o1.access_flags == o2.access_flags
            && o1.simple_name == o2.simple_name
            && o1.enclosing_name == o2.enclosing_name
    }

    fn get_type(&self) -> String {
        self.type_.to_string()
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} of {}",
            self.simple_name.as_deref().unwrap_or("null"),
            ClassWriter::get_modifiers(self.access_flags),
            self.get_type(),
            self.enclosing_name.as_deref().unwrap_or("null")
        )
    }
}

impl ClassesProcessor {
    pub const AVERAGE_CLASS_SIZE: i32 = 16 * 1024;

    pub fn new(context: StructContext) -> ClassesProcessor {
        ClassesProcessor::alloc(ClassesProcessorData {
            context,
            map_root_classes: Shared::new(JHashMap::new()),
            whitelist: JHashSet::new(),
        })
    }

    pub fn add_whitelist(self, prefix: &str) {
        self.with_mut(|d| d.whitelist.add(prefix.to_string()));
    }

    pub fn is_whitelisted(self, cls: &str) -> bool {
        self.with(|d| {
            if d.whitelist.is_empty() {
                return true;
            }

            for prefix in d.whitelist.iter() {
                if cls.starts_with(prefix.as_str()) {
                    return true;
                }
            }

            false
        })
    }

    pub fn load_classes(self, renamer: Option<&dyn IIdentifierRenamer>) {
        let mut map_inner_classes: JHashMap<String, Inner> = JHashMap::new();
        let mut map_nested_class_references: JHashMap<String, JHashSet<String>> = JHashMap::new();
        let mut map_enclosing_class_references: JHashMap<String, JHashSet<String>> =
            JHashMap::new();
        let mut map_new_simple_names: JHashMap<String, String> = JHashMap::new();

        let b_decompile_inner =
            DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_INNER);
        let verify_anonymous_classes =
            DecompilerContext::get_option(IFernflowerPreferences::VERIFY_ANONYMOUS_CLASSES);

        let mut excluded_matcher = None;
        let excluded_regex =
            DecompilerContext::get_property(IFernflowerPreferences::EXCLUDED_CLASSES)
                .expect("getProperty(EXCLUDED_CLASSES)");
        if !excluded_regex.is_empty() {
            excluded_matcher = Some(JavaPattern::compile(&excluded_regex));
        }

        let mut included_matcher = None;
        let included_regex =
            DecompilerContext::get_property(IFernflowerPreferences::INCLUDED_CLASSES)
                .expect("getProperty(INCLUDED_CLASSES)");
        if !included_regex.is_empty() {
            included_matcher = Some(JavaPattern::compile(&included_regex));
        }

        let context = self.with(|d| d.context);
        let own_classes = context.get_own_classes();
        let mut classes: Vec<StructClass> = Vec::new();
        let mut names: JLinkedHashSet<String> = JLinkedHashSet::new();

        for cl in own_classes {
            let qualified_name = cl.qualified_name();
            if names.add(qualified_name.clone()) {
                classes.push(cl);
            } else {
                DecompilerContext::get_logger().write_message(
                    &format!("Skipping processing already existing class: {qualified_name}"),
                    Severity::Error,
                );
            }
        }

        let map_root_classes = self.get_map_root_classes();
        for cl in classes {
            let qualified_name = cl.qualified_name();
            if let Some(matcher) = &excluded_matcher {
                if matcher.matches(&qualified_name) {
                    continue;
                }
            }
            if let Some(matcher) = &included_matcher {
                if !matcher.matches(&qualified_name) {
                    continue;
                }
            }

            if !map_root_classes.borrow().contains_key(&qualified_name) {
                if b_decompile_inner {
                    let inner = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES);

                    if let Some(inner) = inner {
                        for entry in inner.get_entries() {
                            let inner_name = entry.inner_name.clone();

                            let mut simple_name = entry.simple_name.clone();
                            let saved_name = map_new_simple_names.get_cloned(&inner_name);
                            if saved_name.is_some() {
                                simple_name = saved_name;
                            } else if let (Some(name), Some(renamer)) = (&simple_name, renamer) {
                                if renamer.to_be_renamed(
                                    RenamerType::ElementClass,
                                    Some(name),
                                    None,
                                    None,
                                ) {
                                    let new_name =
                                        renamer.get_next_class_name(&inner_name, Some(name));
                                    map_new_simple_names.put(inner_name.clone(), new_name.clone());
                                    simple_name = Some(new_name);
                                }
                            }

                            let mut rec = Inner {
                                simple_name,
                                type_: if entry.simple_name_idx == 0 {
                                    ClassNodeType::Anonymous
                                } else if entry.outer_name_idx == 0 {
                                    ClassNodeType::Local
                                } else {
                                    ClassNodeType::Member
                                },
                                access_flags: entry.access_flags,
                                source: qualified_name.clone(),
                                enclosing_name: None,
                            };

                            if entry.enclosing_name.is_some() {
                                rec.enclosing_name = entry.enclosing_name.clone();
                            } else if let Some(in_) = context.get_class(&entry.inner_name) {
                                let attr = in_.get_attribute(
                                    StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD,
                                );
                                if let Some(attr) = attr {
                                    rec.enclosing_name = attr.get_class_name();
                                }
                            }

                            if entry.simple_name.is_none() {
                                rec.type_ = ClassNodeType::Anonymous;
                            } else {
                                match context.get_class(&entry.inner_name) {
                                    None => rec.type_ = ClassNodeType::Member,
                                    Some(in_) => {
                                        let attr = in_.get_attribute(
                                            StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD,
                                        );
                                        if attr.is_some_and(|a| a.get_method_name().is_some()) {
                                            rec.type_ = ClassNodeType::Local;
                                        } else {
                                            rec.type_ = ClassNodeType::Member;
                                        }
                                    }
                                }
                            }

                            let encl_class_name = if entry.outer_name_idx != 0 {
                                entry.enclosing_name.clone()
                            } else {
                                Some(qualified_name.clone())
                            };
                            let Some(encl_class_name) = encl_class_name else {
                                continue;
                            };
                            if inner_name == encl_class_name {
                                continue;
                            }
                            if DecompilerContext::get_option(
                                IFernflowerPreferences::VALIDATE_INNER_CLASSES_NAMES,
                            ) && rec.type_ == ClassNodeType::Member
                                && inner_name
                                    != format!(
                                        "{}${}",
                                        encl_class_name,
                                        entry.simple_name.as_deref().unwrap_or("null")
                                    )
                            {
                                continue;
                            }

                            let enclosing_class = context.get_class(&encl_class_name);
                            if enclosing_class.is_some_and(|c| c.is_own()) {
                                let existing_rec = map_inner_classes.get_cloned(&inner_name);
                                match existing_rec {
                                    None => {
                                        map_inner_classes.put(inner_name.clone(), rec.clone());
                                    }
                                    Some(existing_rec) if !Inner::equal(&existing_rec, &rec) => {
                                        let priority = |source: &str| {
                                            if source == inner_name {
                                                1
                                            } else if source == encl_class_name {
                                                2
                                            } else {
                                                3
                                            }
                                        };
                                        let old_priority = priority(&existing_rec.source);
                                        let new_priority = priority(&rec.source);
                                        if DecompilerContext::get_option(
                                            IFernflowerPreferences::WARN_INCONSISTENT_INNER_CLASSES,
                                        ) {
                                            let logger = DecompilerContext::get_logger();
                                            let message = format!(
                                                "Inconsistent inner class entries for {inner_name}!"
                                            );
                                            logger.write_message(&message, Severity::Warn);
                                            logger.write_message(
                                                &format!(
                                                    "  {}: {}",
                                                    existing_rec.source, existing_rec
                                                ),
                                                Severity::Warn,
                                            );
                                            logger.write_message(
                                                &format!("  {}: {}", rec.source, rec),
                                                Severity::Warn,
                                            );
                                            if new_priority < old_priority {
                                                logger.write_message(
                                                    &format!(
                                                        "Changing mapping to {inner_name} -> {rec}"
                                                    ),
                                                    Severity::Warn,
                                                );
                                            }
                                        }
                                        if new_priority < old_priority {
                                            map_inner_classes.put(inner_name.clone(), rec.clone());
                                        }
                                    }
                                    Some(_) => {}
                                }

                                map_nested_class_references
                                    .compute_if_absent(encl_class_name.clone(), JHashSet::new)
                                    .add(inner_name.clone());
                                map_enclosing_class_references
                                    .compute_if_absent(inner_name.clone(), JHashSet::new)
                                    .add(encl_class_name.clone());
                            }
                        }
                    }
                }

                if self.is_whitelisted(&qualified_name) {
                    let node = ClassNode::new(ClassNodeType::Root, cl);
                    node.with_mut(|d| d.access = cl.get_access_flags());
                    map_root_classes
                        .borrow_mut()
                        .put(qualified_name.clone(), node);
                }
            }
        }

        if b_decompile_inner {
            let root_names = map_root_classes.borrow().key_vec();
            for ent_key in root_names {
                if !map_inner_classes.contains_key(&ent_key) {
                    let mut set_visited: JHashSet<String> = JHashSet::new();
                    let mut stack: VecDeque<String> = VecDeque::new();

                    stack.push_back(ent_key.clone());
                    set_visited.add(ent_key.clone());

                    while let Some(super_class) = stack.pop_front() {
                        let super_node = map_root_classes
                            .borrow()
                            .get_cloned(&super_class)
                            .expect("mapRootClasses.get(superClass)");

                        let set_nested_classes =
                            map_nested_class_references.get_cloned(&super_class);
                        if let Some(set_nested_classes) = set_nested_classes {
                            let scl = super_node.with(|d| d.class_struct);
                            let inner =
                                scl.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES);

                            let entries = inner.map(|i| i.get_entries()).unwrap_or_default();
                            if entries.is_empty() {
                                DecompilerContext::get_logger().write_message(
                                    &format!("{super_class} does not contain inner classes!"),
                                    Severity::Warn,
                                );
                                continue;
                            }

                            let scl_name = scl.qualified_name();
                            for entry in entries {
                                let nested_class = entry.inner_name.clone();
                                if !set_nested_classes.contains(&nested_class) {
                                    continue;
                                }

                                let rec = map_inner_classes
                                    .get_cloned(&nested_class)
                                    .expect("mapInnerClasses.get(nestedClass)");
                                if rec.enclosing_name.as_deref() != Some(scl_name.as_str()) {
                                    continue;
                                }

                                if !set_visited.add(nested_class.clone()) {
                                    continue;
                                }

                                let nested_node =
                                    map_root_classes.borrow().get_cloned(&nested_class);
                                let Some(nested_node) = nested_node else {
                                    DecompilerContext::get_logger().write_message(
                                        &format!("Nested class {nested_class} missing!"),
                                        Severity::Warn,
                                    );
                                    continue;
                                };

                                nested_node.with_mut(|d| {
                                    d.simple_name = rec.simple_name.clone();
                                    d.type_ = rec.type_;
                                    if d.type_ == ClassNodeType::Anonymous {
                                        d.enclosing_method = None;
                                    }
                                    d.access = rec.access_flags;
                                });

                                let (nested_type, nested_struct) =
                                    nested_node.with(|d| (d.type_, d.class_struct));
                                if verify_anonymous_classes
                                    && nested_type == ClassNodeType::Anonymous
                                    && !Self::is_anonymous(nested_struct, scl)
                                {
                                    nested_node.with_mut(|d| d.type_ = ClassNodeType::Local);
                                }

                                let nested_type = nested_node.with(|d| d.type_);
                                if nested_type == ClassNodeType::Anonymous {
                                    let cl = nested_struct;
                                    nested_node.with_mut(|d| d.access &= !ACC_STATIC);

                                    let interfaces = cl.get_interfaces();
                                    let anonymous_class_type = if !interfaces.is_empty() {
                                        VarType::new_signature_cl(&cl.get_interface(0), true)
                                    } else {
                                        let super_class =
                                            cl.super_class().expect("superClass").get_string();
                                        VarType::new_signature_cl(&super_class, true)
                                    };
                                    nested_node.with_mut(|d| {
                                        d.anonymous_class_type = Some(anonymous_class_type)
                                    });
                                } else if nested_type == ClassNodeType::Local {
                                    let mut allowed_flags = ACC_ABSTRACT | ACC_FINAL;
                                    if scl.get_version().has_local_enums_and_interfaces() {
                                        allowed_flags |= ACC_INTERFACE | ACC_ENUM;
                                        if (nested_node.with(|d| d.access) & ACC_ENUM) != 0 {
                                            allowed_flags |= ACC_STATIC;
                                        }
                                    }
                                    nested_node.with_mut(|d| d.access &= allowed_flags);
                                }

                                super_node.with(|d| d.nested.clone()).add(nested_node);
                                nested_node.with_mut(|d| d.parent = Some(super_node));

                                let references = map_enclosing_class_references
                                    .get_cloned(&nested_class)
                                    .expect("mapEnclosingClassReferences.get(nestedClass)");
                                nested_node
                                    .with(|d| d.enclosing_classes.clone())
                                    .borrow_mut()
                                    .add_all(references.to_vec());

                                stack.push_back(nested_class);
                            }
                        }
                        sort_nodes(&super_node.with(|d| d.nested.clone()));
                    }
                }
            }
        }
    }

    fn is_anonymous(cl: StructClass, enclosing_cl: StructClass) -> bool {
        let cl_name = cl.qualified_name();
        let interfaces = cl.get_interfaces();
        if !interfaces.is_empty() {
            let has_non_trivial_super_class = cl.super_class().is_some_and(|s| {
                !VarType::VARTYPE_OBJECT().equals(&VarType::new_signature_cl(&s.get_string(), true))
            });
            if has_non_trivial_super_class || interfaces.len() > 1 {
                let message = format!(
                    "Inconsistent anonymous class definition: '{cl_name}'. Multiple interfaces and/or super class defined."
                );
                DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                return false;
            }
        } else if cl.super_class().is_none() {
            let message = format!(
                "Inconsistent anonymous class definition: '{cl_name}'. Neither interface nor super class defined."
            );
            DecompilerContext::get_logger().write_message(&message, Severity::Warn);
            return false;
        }

        let pool = enclosing_cl.get_pool();

        let mut ref_counter = 0;
        let mut ref_not_new = false;

        let attribute = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD);
        let enclosing_method = attribute.and_then(|a| a.get_method_name());

        let methods = enclosing_cl.get_methods().to_vec();
        for mt in methods {
            if let Some(enclosing_method) = &enclosing_method {
                if *enclosing_method != mt.get_name() {
                    continue;
                }
            }

            let scanned = try_catch(|| {
                mt.expand_data(enclosing_cl);

                let seq = mt.get_instruction_sequence();
                if let Some(seq) = seq {
                    for instr in seq.iter() {
                        match instr.opcode() {
                            OPC_CHECKCAST | OPC_INSTANCEOF => {
                                let constant = pool
                                    .get_primitive_constant(instr.operand(0))
                                    .expect("getPrimitiveConstant");
                                if cl_name == *constant.get_string() {
                                    ref_counter += 1;
                                    ref_not_new = true;
                                }
                            }
                            OPC_NEW | OPC_ANEWARRAY | OPC_MULTIANEWARRAY => {
                                let constant = pool
                                    .get_primitive_constant(instr.operand(0))
                                    .expect("getPrimitiveConstant");
                                if cl_name == *constant.get_string() {
                                    ref_counter += 1;
                                }
                            }
                            OPC_GETSTATIC | OPC_PUTSTATIC => {
                                let constant = pool
                                    .get_link_constant(instr.operand(0))
                                    .expect("getLinkConstant");
                                if constant.classname.as_deref() == Some(cl_name.as_str()) {
                                    ref_counter += 1;
                                    ref_not_new = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                mt.release_resources();
            });
            if let Err(ex) = scanned {
                if !is_io_exception(&ex) {
                    rethrow(ex);
                }
                let message = format!(
                    "Could not read method while checking anonymous class definition: '{}', '{}'",
                    enclosing_cl.qualified_name(),
                    InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor())
                );
                DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                return false;
            }

            if ref_counter > 1 || ref_not_new {
                let message = format!(
                    "Inconsistent references to the class '{cl_name}' which is supposed to be anonymous"
                );
                DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                return false;
            }
        }

        true
    }

    pub fn process_class(self, cl: StructClass) {
        let qualified_name = cl.qualified_name();
        let root = self
            .get_map_root_classes()
            .borrow()
            .get_cloned(&qualified_name)
            .expect("mapRootClasses.get(cl.qualifiedName)");
        if root.with(|d| d.type_) != ClassNodeType::Root {
            return;
        }

        let package_info =
            cl.is_synthetic() && root.with(|d| d.simple_name.as_deref() == Some("package-info"));
        let module_info = cl.has_modifier(ACC_MODULE)
            && cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_MODULE);

        DecompilerContext::get_logger().start_processing_class(&qualified_name);
        let import_collector = ImportCollector::new(root);
        DecompilerContext::start_class(import_collector);
        let result = try_catch(|| {
            if !package_info && !module_info {
                DecompilerContext::get_struct_context()
                    .get_plugin_context()
                    .run_class_passes(ClassPassLocation::BeforeProcessing, root);
                LambdaProcessor::new().process_class(root);

                Self::add_class_name_to_import(root, import_collector);

                let spec = PluginContext::get_current_context().get_language_spec(cl);

                Self::init_wrappers(root, spec.as_ref());

                match spec {
                    None => {
                        NestedClassProcessor::new().process_class(root, root);

                        NestedMemberAccess::new().propagate_member_access(root);
                    }
                    Some(spec) => match spec {},
                }

                DecompilerContext::get_struct_context()
                    .get_plugin_context()
                    .run_class_passes(ClassPassLocation::AfterProcessing, root);
            }
        });
        DecompilerContext::get_logger().end_processing_class();
        if let Err(e) = result {
            if CanceledException::is(&e) || is_java_error(&e) {
                rethrow(e);
            }
            jthrow!("{}", e.message);
        }
    }

    pub fn write_class(self, cl: StructClass, buffer: &mut TextBuffer) {
        let qualified_name = cl.qualified_name();
        let root = self
            .get_map_root_classes()
            .borrow()
            .get_cloned(&qualified_name)
            .expect("mapRootClasses.get(cl.qualifiedName)");
        if root.with(|d| d.type_) != ClassNodeType::Root {
            return;
        }

        let package_info =
            cl.is_synthetic() && root.with(|d| d.simple_name.as_deref() == Some("package-info"));
        let module_info = cl.has_modifier(ACC_MODULE)
            && cl.has_attribute(StructGeneralAttribute::ATTRIBUTE_MODULE);

        DecompilerContext::get_logger().start_reading_class(&qualified_name);
        let result = try_catch(|| {
            if package_info {
                ClassWriter::package_info_to_java(cl, buffer);

                DecompilerContext::get_import_collector().write_imports(buffer, false);
            } else if module_info {
                let mut module_buffer = TextBuffer::with_capacity(Self::AVERAGE_CLASS_SIZE);
                ClassWriter::module_info_to_java(cl, &mut module_buffer);

                DecompilerContext::get_import_collector().write_imports(buffer, true);

                buffer.append_buffer(&module_buffer);
            } else {
                let spec = PluginContext::get_current_context().get_language_spec(cl);
                let mut class_buffer = TextBuffer::with_capacity(Self::AVERAGE_CLASS_SIZE);
                let writer = match spec {
                    Some(spec) => match spec {},
                    None => ClassWriter::new(),
                };

                DecompilerContext::get_struct_context()
                    .get_plugin_context()
                    .run_class_passes(ClassPassLocation::BeforeWriting, root);

                writer.write_class(root, &mut class_buffer, 0);
                class_buffer.reformat();

                for (class_and_method, tracer) in class_buffer.get_tracers().entry_vec() {
                    let clazz =
                        DecompilerContext::get_struct_context().get_class(&class_and_method.a);
                    if let Some(clazz) = clazz {
                        let method = class_and_method
                            .b
                            .as_deref()
                            .and_then(|key| clazz.get_method_key(key));

                        if let Some(method) = method {
                            let line_number_table = method
                                .get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE);
                            tracer.set_line_number_table(line_number_table);
                            DecompilerContext::get_bytecode_source_mapper().add_tracer(
                                &class_and_method.a,
                                class_and_method.b.as_deref().unwrap_or_default(),
                                tracer,
                            );
                        }
                    }
                }

                writer.write_class_header(cl, buffer, DecompilerContext::get_import_collector());
                let offset_lines = buffer.count_lines();

                buffer.append_buffer(&class_buffer);

                if DecompilerContext::get_option(IFernflowerPreferences::BYTECODE_SOURCE_MAPPING) {
                    let mapper = DecompilerContext::get_bytecode_source_mapper();
                    mapper.add_total_offset(offset_lines);
                    if DecompilerContext::get_option(IFernflowerPreferences::DUMP_ORIGINAL_LINES) {
                        buffer.dump_original_line_numbers(&mapper.get_original_lines_mapping());
                    }
                    if DecompilerContext::get_option(IFernflowerPreferences::UNIT_TEST_MODE) {
                        buffer.append_line_separator();
                        mapper.dump_mapping(buffer, true);
                    }
                }
            }
        });
        Self::destroy_wrappers(root);
        DecompilerContext::get_logger().end_reading_class();
        if let Err(e) = result {
            rethrow(e);
        }
    }

    fn init_wrappers(node: ClassNode, spec: Option<&LanguageSpec>) {
        if node.with(|d| d.type_) == ClassNodeType::Lambda {
            return;
        }

        let nested = node.with(|d| d.nested.to_vec());
        let mut nested_copy = nested.clone();

        for nd in nested {
            if Self::should_init_early(nd) {
                Self::init_wrappers(nd, spec);
                if let Some(i) = nested_copy.iter().position(|n| *n == nd) {
                    nested_copy.remove(i);
                }
            }
        }

        let wrapper = ClassWrapper::new(node);
        wrapper.init(spec);

        node.with_mut(|d| d.wrapper = Some(wrapper));

        for nd in nested_copy {
            Self::init_wrappers(nd, spec);
        }
    }

    fn should_init_early(node: ClassNode) -> bool {
        let class_struct = node.with(|d| d.class_struct);
        if class_struct.has_modifier(ACC_SYNTHETIC) {
            let methods = class_struct.get_methods();
            if methods.size() == 1 && methods.get(0).get_name() == CLINIT_NAME {
                return class_struct
                    .get_fields()
                    .to_vec()
                    .into_iter()
                    .all(|st_field| {
                        let descriptor = st_field.get_descriptor();
                        (descriptor == "[I" || descriptor == "Z")
                            && st_field.has_modifier(SwitchHelper::STATIC_FINAL_SYNTHETIC)
                    });
            }
        }
        false
    }

    fn add_class_name_to_import(node: ClassNode, imp: ImportCollector) {
        let (simple_name, type_, class_struct) =
            node.with(|d| (d.simple_name.clone(), d.type_, d.class_struct));
        if let Some(simple_name) = simple_name.filter(|s| !s.is_empty()) {
            let name = if type_ == ClassNodeType::Root {
                class_struct.qualified_name()
            } else {
                simple_name
            };
            imp.get_short_name_imported(&name, false);
        }

        let nested = node.with(|d| d.nested.to_vec());
        for nd in nested {
            Self::add_class_name_to_import(nd, imp);
        }
    }

    fn destroy_wrappers(node: ClassNode) {
        let class_struct = node.with_mut(|d| {
            d.wrapper = None;
            d.class_struct
        });
        class_struct.release_resources();
        let methods = class_struct.get_methods().to_vec();
        for m in methods {
            m.clear_variable_namer();
        }
    }

    pub fn get_map_root_classes(self) -> Shared<JHashMap<String, ClassNode>> {
        self.with(|d| d.map_root_classes.clone())
    }
}

/// `Collections.sort(nodes)` by [`ClassNode::compare_to`] (stable).
pub(crate) fn sort_nodes(nodes: &JList<ClassNode>) {
    let mut sorted = nodes.to_vec();
    sorted.sort_by(|a, b| a.compare_to(*b));
    nodes.set_all(sorted);
}

/// A `java.lang.Error`, which `catch (Exception e)` lets through.
fn is_java_error(caught: &Caught) -> bool {
    caught
        .exception
        .as_ref()
        .is_some_and(|e| e.class.ends_with("Error"))
}

/// `java.util.regex.Pattern.compile(regex).matcher(s).matches()` for the
/// syntax class filters use: literals, `.`, escapes (`\d\D\w\W\s\S\b\B\A\z\Z`,
/// `\t\n\r\f\a\e`, `\0`, `\x`, `\u`, `\c`, `\Q…\E`), classes with ranges,
/// negation and nested unions, groups (`(…)`, `(?:…)`, `(?<name>…)`, `(?>…)`,
/// `(?=…)`, `(?!…)`), alternation, anchors and greedy/lazy/possessive
/// quantifiers. Anything else throws instead of silently matching differently.
pub mod java_regex {
    use crate::jthrow;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Greedy,
        Lazy,
        Possessive,
    }

    enum Node {
        Char(char),
        Any,
        Class(Class),
        Begin,
        End,
        EndOfInput,
        WordBoundary(bool),
        Group(Vec<Vec<Node>>),
        Atomic(Vec<Vec<Node>>),
        Look(Vec<Vec<Node>>, bool),
        Repeat(Box<Node>, u32, Option<u32>, Mode),
    }

    struct Class {
        negated: bool,
        items: Vec<Item>,
    }

    enum Item {
        Range(char, char),
        Predefined(char),
        Nested(Class),
    }

    impl Class {
        fn matches(&self, c: char) -> bool {
            let hit = self.items.iter().any(|item| match item {
                Item::Range(lo, hi) => *lo <= c && c <= *hi,
                Item::Predefined(p) => predefined(*p, c),
                Item::Nested(class) => class.matches(c),
            });
            hit != self.negated
        }
    }

    fn predefined(p: char, c: char) -> bool {
        match p {
            'd' => c.is_ascii_digit(),
            'D' => !c.is_ascii_digit(),
            'w' => is_word(c),
            'W' => !is_word(c),
            's' => matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r'),
            'S' => !matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r'),
            _ => unreachable!(),
        }
    }

    fn is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn is_line_terminator(c: char) -> bool {
        matches!(c, '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}')
    }

    pub struct JavaPattern {
        alternatives: Vec<Vec<Node>>,
    }

    impl JavaPattern {
        /// `Pattern.compile(regex)`; throws `PatternSyntaxException`.
        pub fn compile(regex: &str) -> JavaPattern {
            let mut parser = Parser {
                chars: regex.chars().collect(),
                pos: 0,
                regex,
            };
            let alternatives = parser.alternation();
            if parser.pos < parser.chars.len() {
                parser.error("Unmatched closing ')'");
            }
            JavaPattern { alternatives }
        }

        /// `matcher.reset(input).matches()`.
        pub fn matches(&self, input: &str) -> bool {
            let input: Vec<char> = input.chars().collect();
            let matcher = Matcher { input: &input };
            matcher.alternatives(&self.alternatives, 0, &mut |p| p == input.len())
        }
    }

    struct Parser<'a> {
        chars: Vec<char>,
        pos: usize,
        regex: &'a str,
    }

    impl Parser<'_> {
        fn error(&self, description: &str) -> ! {
            jthrow!(PatternSyntaxException; "{} near index {}\n{}", description, self.pos, self.regex)
        }

        fn unsupported(&self, what: &str) -> ! {
            jthrow!(UnsupportedOperationException; "regex construct {} is not supported (index {} of {})", what, self.pos, self.regex)
        }

        fn peek(&self) -> Option<char> {
            self.chars.get(self.pos).copied()
        }

        fn peek_at(&self, offset: usize) -> Option<char> {
            self.chars.get(self.pos + offset).copied()
        }

        fn next(&mut self) -> Option<char> {
            let c = self.peek();
            if c.is_some() {
                self.pos += 1;
            }
            c
        }

        fn alternation(&mut self) -> Vec<Vec<Node>> {
            let mut alternatives = vec![self.sequence()];
            while self.peek() == Some('|') {
                self.pos += 1;
                alternatives.push(self.sequence());
            }
            alternatives
        }

        fn sequence(&mut self) -> Vec<Node> {
            let mut nodes = Vec::new();
            loop {
                match self.peek() {
                    None | Some('|') | Some(')') => return nodes,
                    _ => {}
                }
                if self.peek() == Some('\\') && self.peek_at(1) == Some('Q') {
                    self.pos += 2;
                    let mut quoted = Vec::new();
                    while self.pos < self.chars.len()
                        && !(self.peek() == Some('\\') && self.peek_at(1) == Some('E'))
                    {
                        quoted.push(Node::Char(self.chars[self.pos]));
                        self.pos += 1;
                    }
                    if self.pos < self.chars.len() {
                        self.pos += 2;
                    }
                    let Some(last) = quoted.pop() else {
                        continue;
                    };
                    nodes.extend(quoted);
                    let atom = self.quantifier(last);
                    nodes.push(atom);
                    continue;
                }
                let atom = self.atom();
                let atom = self.quantifier(atom);
                nodes.push(atom);
            }
        }

        fn atom(&mut self) -> Node {
            let c = self.next().expect("atom");
            match c {
                '(' => self.group(),
                '[' => Node::Class(self.class()),
                '.' => Node::Any,
                '^' => Node::Begin,
                '$' => Node::End,
                '\\' => self.escape(),
                '*' | '+' | '?' => {
                    self.pos -= 1;
                    self.error(&format!("Dangling meta character '{c}'"))
                }
                '{' => {
                    self.pos -= 1;
                    self.error("Illegal repetition")
                }
                c => Node::Char(c),
            }
        }

        fn group(&mut self) -> Node {
            let node = if self.peek() == Some('?') {
                self.pos += 1;
                match self.next() {
                    Some(':') => Node::Group(self.alternation()),
                    Some('>') => Node::Atomic(self.alternation()),
                    Some('=') => Node::Look(self.alternation(), false),
                    Some('!') => Node::Look(self.alternation(), true),
                    Some('<') if self.peek().is_some_and(|c| c.is_ascii_alphabetic()) => {
                        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
                            self.pos += 1;
                        }
                        if self.next() != Some('>') {
                            self.error("named capturing group is missing trailing '>'");
                        }
                        Node::Group(self.alternation())
                    }
                    _ => self.unsupported("(?"),
                }
            } else {
                Node::Group(self.alternation())
            };
            if self.next() != Some(')') {
                self.error("Unclosed group");
            }
            node
        }

        fn quantifier(&mut self, atom: Node) -> Node {
            let (min, max) = match self.peek() {
                Some('*') => (0, None),
                Some('+') => (1, None),
                Some('?') => (0, Some(1)),
                Some('{') => {
                    let start = self.pos;
                    self.pos += 1;
                    let min = self.number();
                    let max = if self.peek() == Some(',') {
                        self.pos += 1;
                        if self.peek() == Some('}') {
                            None
                        } else {
                            Some(self.number())
                        }
                    } else {
                        Some(min)
                    };
                    if self.peek() != Some('}') {
                        self.pos = start;
                        self.error("Unclosed counted closure");
                    }
                    if max.is_some_and(|max| max < min) {
                        self.error("Illegal repetition range");
                    }
                    (min, max)
                }
                _ => return atom,
            };
            self.pos += 1;
            let mode = match self.peek() {
                Some('?') => {
                    self.pos += 1;
                    Mode::Lazy
                }
                Some('+') => {
                    self.pos += 1;
                    Mode::Possessive
                }
                _ => Mode::Greedy,
            };
            Node::Repeat(Box::new(atom), min, max, mode)
        }

        fn number(&mut self) -> u32 {
            let start = self.pos;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.pos += 1;
            }
            if start == self.pos {
                self.error("Illegal repetition");
            }
            let digits: String = self.chars[start..self.pos].iter().collect();
            digits
                .parse()
                .unwrap_or_else(|_| self.error("Illegal repetition"))
        }

        fn escape(&mut self) -> Node {
            let Some(c) = self.peek() else {
                self.error("Unexpected internal error");
            };
            match c {
                'd' | 'D' | 'w' | 'W' | 's' | 'S' => {
                    self.pos += 1;
                    Node::Class(Class {
                        negated: false,
                        items: vec![Item::Predefined(c)],
                    })
                }
                'b' | 'B' => {
                    self.pos += 1;
                    Node::WordBoundary(c == 'B')
                }
                'A' => {
                    self.pos += 1;
                    Node::Begin
                }
                'z' => {
                    self.pos += 1;
                    Node::EndOfInput
                }
                'Z' => {
                    self.pos += 1;
                    Node::End
                }
                _ => Node::Char(self.escaped_char()),
            }
        }

        fn escaped_char(&mut self) -> char {
            let c = self.next().expect("escape");
            match c {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                'f' => '\u{c}',
                'a' => '\u{7}',
                'e' => '\u{1b}',
                '0' => {
                    let mut value = 0u32;
                    let mut digits = 0;
                    while digits < 3
                        && self.peek().is_some_and(|d| ('0'..='7').contains(&d))
                        && (digits < 2 || value <= 0o37)
                    {
                        value = value * 8 + self.next().unwrap().to_digit(8).unwrap();
                        digits += 1;
                    }
                    if digits == 0 {
                        self.error("Illegal octal escape sequence");
                    }
                    char::from_u32(value).unwrap()
                }
                'x' => {
                    if self.peek() == Some('{') {
                        self.pos += 1;
                        let value = self.hex(usize::MAX);
                        if self.next() != Some('}') {
                            self.error("Unclosed hexadecimal escape sequence");
                        }
                        char::from_u32(value)
                            .unwrap_or_else(|| self.error("Hexadecimal codepoint is too big"))
                    } else {
                        let value = self.hex_exact(2);
                        char::from_u32(value).unwrap()
                    }
                }
                'u' => {
                    let value = self.hex_exact(4);
                    char::from_u32(value).unwrap_or_else(|| self.unsupported("\\u surrogate"))
                }
                'c' => {
                    let Some(control) = self.next() else {
                        self.error("Illegal control escape sequence");
                    };
                    char::from_u32(control as u32 ^ 64).unwrap()
                }
                '1'..='9' => self.unsupported("back reference"),
                c if c.is_ascii_alphabetic() => {
                    if matches!(
                        c,
                        'p' | 'P' | 'h' | 'H' | 'v' | 'V' | 'R' | 'X' | 'G' | 'k' | 'N'
                    ) {
                        self.unsupported(&format!("\\{c}"));
                    }
                    self.pos -= 1;
                    self.error("Illegal/unsupported escape sequence")
                }
                c => c,
            }
        }

        fn hex(&mut self, max_digits: usize) -> u32 {
            let mut value = 0u32;
            let mut digits = 0;
            while digits < max_digits && self.peek().is_some_and(|d| d.is_ascii_hexdigit()) {
                value = value
                    .checked_mul(16)
                    .and_then(|v| v.checked_add(self.next().unwrap().to_digit(16).unwrap()))
                    .unwrap_or_else(|| self.error("Hexadecimal codepoint is too big"));
                digits += 1;
            }
            if digits == 0 {
                self.error("Illegal hexadecimal escape sequence");
            }
            value
        }

        fn hex_exact(&mut self, digits: usize) -> u32 {
            let start = self.pos;
            let value = self.hex(digits);
            if self.pos - start != digits {
                self.error("Illegal hexadecimal escape sequence");
            }
            value
        }

        fn class(&mut self) -> Class {
            let mut class = Class {
                negated: false,
                items: Vec::new(),
            };
            if self.peek() == Some('^') {
                self.pos += 1;
                class.negated = true;
            }
            let mut first = true;
            loop {
                let Some(c) = self.next() else {
                    self.error("Unclosed character class");
                };
                match c {
                    ']' if !first => return class,
                    '[' => class.items.push(Item::Nested(self.class())),
                    '&' if self.peek() == Some('&') => self.unsupported("&&"),
                    _ => {
                        let lo = if c == '\\' {
                            match self.peek() {
                                Some(p @ ('d' | 'D' | 'w' | 'W' | 's' | 'S')) => {
                                    self.pos += 1;
                                    class.items.push(Item::Predefined(p));
                                    first = false;
                                    continue;
                                }
                                Some('Q') => self.unsupported("\\Q in a class"),
                                _ => self.escaped_char(),
                            }
                        } else {
                            c
                        };
                        if self.peek() == Some('-')
                            && self.peek_at(1).is_some_and(|n| n != ']' && n != '[')
                        {
                            self.pos += 1;
                            let hi = match self.next().unwrap() {
                                '\\' => self.escaped_char(),
                                hi => hi,
                            };
                            if hi < lo {
                                self.error("Illegal character range");
                            }
                            class.items.push(Item::Range(lo, hi));
                        } else {
                            class.items.push(Item::Range(lo, lo));
                        }
                    }
                }
                first = false;
            }
        }
    }

    struct Matcher<'a> {
        input: &'a [char],
    }

    impl Matcher<'_> {
        fn alternatives(
            &self,
            alternatives: &[Vec<Node>],
            pos: usize,
            k: &mut dyn FnMut(usize) -> bool,
        ) -> bool {
            for sequence in alternatives {
                if self.sequence(sequence, pos, k) {
                    return true;
                }
            }
            false
        }

        fn sequence(&self, nodes: &[Node], pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
            match nodes.split_first() {
                None => k(pos),
                Some((first, rest)) => self.node(first, pos, &mut |p| self.sequence(rest, p, k)),
            }
        }

        fn node(&self, node: &Node, pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
            let len = self.input.len();
            match node {
                Node::Char(c) => pos < len && self.input[pos] == *c && k(pos + 1),
                Node::Any => pos < len && !is_line_terminator(self.input[pos]) && k(pos + 1),
                Node::Class(class) => pos < len && class.matches(self.input[pos]) && k(pos + 1),
                Node::Begin => pos == 0 && k(pos),
                Node::End => {
                    let at_end = pos == len
                        || (pos + 1 == len && is_line_terminator(self.input[pos]))
                        || (pos + 2 == len
                            && self.input[pos] == '\r'
                            && self.input[pos + 1] == '\n');
                    at_end && k(pos)
                }
                Node::EndOfInput => pos == len && k(pos),
                Node::WordBoundary(negated) => {
                    let before = pos > 0 && is_word(self.input[pos - 1]);
                    let after = pos < len && is_word(self.input[pos]);
                    ((before != after) != *negated) && k(pos)
                }
                Node::Group(alternatives) => self.alternatives(alternatives, pos, k),
                Node::Atomic(alternatives) => {
                    let mut end = None;
                    self.alternatives(alternatives, pos, &mut |p| {
                        end = Some(p);
                        true
                    });
                    end.is_some_and(k)
                }
                Node::Look(alternatives, negated) => {
                    let found = self.alternatives(alternatives, pos, &mut |_| true);
                    found != *negated && k(pos)
                }
                Node::Repeat(inner, min, max, mode) => {
                    self.repeat(inner, *min, *max, *mode, 0, pos, k)
                }
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn repeat(
            &self,
            inner: &Node,
            min: u32,
            max: Option<u32>,
            mode: Mode,
            count: u32,
            pos: usize,
            k: &mut dyn FnMut(usize) -> bool,
        ) -> bool {
            if mode == Mode::Possessive {
                let (mut count, mut pos) = (count, pos);
                while max.is_none_or(|max| count < max) {
                    let mut next = None;
                    self.node(inner, pos, &mut |p| {
                        next = Some(p);
                        true
                    });
                    match next {
                        Some(p) if p != pos || count < min => {
                            pos = p;
                            count += 1;
                        }
                        _ => break,
                    }
                }
                return count >= min && k(pos);
            }
            if count < min {
                return self.node(inner, pos, &mut |p| {
                    self.repeat(inner, min, max, mode, count + 1, p, k)
                });
            }
            if max == Some(count) {
                return k(pos);
            }
            if mode == Mode::Lazy && k(pos) {
                return true;
            }
            if self.node(inner, pos, &mut |p| {
                p != pos && self.repeat(inner, min, max, mode, count + 1, p, k)
            }) {
                return true;
            }
            mode == Mode::Greedy && k(pos)
        }
    }
}

/// `ClassesProcessor.ClassNode.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum ClassNodeType {
    Root,
    Member,
    Anonymous,
    Local,
    Lambda,
}

impl ClassNodeType {
    pub fn name(self) -> &'static str {
        match self {
            ClassNodeType::Root => "ROOT",
            ClassNodeType::Member => "MEMBER",
            ClassNodeType::Anonymous => "ANONYMOUS",
            ClassNodeType::Local => "LOCAL",
            ClassNodeType::Lambda => "LAMBDA",
        }
    }
}

impl fmt::Display for ClassNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::rt::JavaHash for ClassNodeType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl crate::rt::JavaEq for ClassNodeType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// `ClassesProcessor.ClassNode.LambdaInformation`; owned by its lambda node.
#[derive(Clone, Default, Debug)]
pub struct LambdaInformation {
    pub method_name: String,
    pub method_descriptor: String,
    pub content_class_name: String,
    pub content_method_name: String,
    pub content_method_descriptor: String,
    pub content_method_invocation_type: i32,
    pub content_method_key: String,
    pub is_method_reference: bool,
    pub is_content_method_static: bool,
}

pub struct ClassNodeData {
    pub type_: ClassNodeType,
    pub access: i32,
    pub simple_name: Option<String>,
    pub class_struct: StructClass,
    /// Class scope, alive until the run's class-scope reset; read it through
    /// [`ClassNode::get_wrapper`].
    pub wrapper: Option<ClassWrapper>,
    pub enclosing_method: Option<String>,
    /// Class scope; stale once the run has finished.
    pub super_invocation: Option<InvocationExprent>,
    pub map_fields_to_vars: JHashMap<String, VarVersionPair>,
    pub anonymous_class_type: Option<VarType>,
    pub nested: JList<ClassNode>,
    pub enclosing_classes: Shared<JHashSet<String>>,
    pub parent: Option<ClassNode>,
    pub lambda_information: Option<LambdaInformation>,
}

arena_object!(pub ClassNode => ClassNodeData, Session);
arena_identity!(ClassNode);

impl ClassNode {
    /// Java: ClassNode(String, String, String, int, String, String, String, StructClass) (lambda class)
    #[allow(clippy::too_many_arguments)]
    pub fn new_lambda(
        content_class_name: &str,
        content_method_name: &str,
        content_method_descriptor: &str,
        content_method_invocation_type: i32,
        lambda_class_name: &str,
        lambda_method_name: &str,
        lambda_method_descriptor: &str,
        class_struct: StructClass,
    ) -> ClassNode {
        let mut lambda_information = LambdaInformation {
            method_name: lambda_method_name.to_string(),
            method_descriptor: lambda_method_descriptor.to_string(),
            content_class_name: content_class_name.to_string(),
            content_method_name: content_method_name.to_string(),
            content_method_descriptor: content_method_descriptor.to_string(),
            content_method_invocation_type,
            ..LambdaInformation::default()
        };

        lambda_information.content_method_key = InterpreterUtil::make_unique_key(
            &lambda_information.content_method_name,
            &lambda_information.content_method_descriptor,
        );

        let anonymous_class_type = VarType::new_signature_cl(lambda_class_name, true);

        let mut is_method_reference = class_struct.qualified_name() != content_class_name;
        if !is_method_reference {
            let mt = class_struct
                .get_method(content_method_name, content_method_descriptor)
                .unwrap();
            is_method_reference = !mt.is_synthetic();
        }

        lambda_information.is_method_reference = is_method_reference;
        lambda_information.is_content_method_static = lambda_information
            .content_method_invocation_type
            == CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC;

        ClassNode::alloc(ClassNodeData {
            type_: ClassNodeType::Lambda,
            access: 0,
            simple_name: None,
            class_struct,
            wrapper: None,
            enclosing_method: None,
            super_invocation: None,
            map_fields_to_vars: JHashMap::new(),
            anonymous_class_type: Some(anonymous_class_type),
            nested: JList::new(),
            enclosing_classes: Shared::new(JHashSet::new()),
            parent: None,
            lambda_information: Some(lambda_information),
        })
    }

    /// Java: ClassNode(Type, StructClass)
    pub fn new(type_: ClassNodeType, class_struct: StructClass) -> ClassNode {
        let qualified_name = class_struct.qualified_name();
        let simple_name = match qualified_name.rfind('/') {
            Some(i) => qualified_name[i + 1..].to_string(),
            None => qualified_name.clone(),
        };

        let mut enclosing_method = None;
        let enclosing_method_attr =
            class_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD);
        if let Some(enclosing_method_attr) = enclosing_method_attr {
            let name = enclosing_method_attr.get_method_name();
            let desc = enclosing_method_attr.get_method_descriptor();
            if let (Some(name), Some(desc)) = (name, desc) {
                enclosing_method = Some(InterpreterUtil::make_unique_key(&name, &desc));
            }
        }

        ClassNode::alloc(ClassNodeData {
            type_,
            access: 0,
            simple_name: Some(simple_name),
            class_struct,
            wrapper: None,
            enclosing_method,
            super_invocation: None,
            map_fields_to_vars: JHashMap::new(),
            anonymous_class_type: None,
            nested: JList::new(),
            enclosing_classes: Shared::new(JHashSet::new()),
            parent: None,
            lambda_information: None,
        })
    }

    pub fn has_modifier(self, modifier: i32) -> bool {
        self.with(|d| (d.access & modifier) == modifier)
    }

    pub fn get_class_node(self, qualified_name: &str) -> Option<ClassNode> {
        let nested = self.with(|d| d.nested.clone());
        for node in nested.iter() {
            let class_struct = node.with(|d| d.class_struct);
            if class_struct.qualified_name() == qualified_name {
                return Some(node);
            }
        }
        None
    }

    /// The wrapper of this node, or of its nearest non-lambda ancestor: `None`
    /// before `initWrappers` and, for a root node, after `destroyWrappers`
    /// (nested wrappers stay for the whole run, as in Java). A wrapper left from
    /// a finished run (class scope reset) also reads as `None`.
    pub fn get_wrapper(self) -> Option<ClassWrapper> {
        let mut node = self;
        while node.with(|d| d.type_) == ClassNodeType::Lambda {
            node = node.with(|d| d.parent).unwrap();
        }
        node.with(|d| d.wrapper).filter(|w| w.is_live())
    }

    pub fn get_all_nested(self) -> JLinkedHashSet<ClassNode> {
        let children = self.with(|d| d.nested.to_vec());
        let mut nested = JLinkedHashSet::from_collection(children.clone());
        for child in children {
            nested.add_all(child.get_all_nested().to_vec());
        }
        nested
    }

    pub fn compare_to(self, o: ClassNode) -> Ordering {
        let a = self.with(|d| d.class_struct).qualified_name();
        let b = o.with(|d| d.class_struct).qualified_name();
        string_compare_to(&a, &b)
    }
}

impl fmt::Display for ClassNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (type_, class_struct) = self.with(|d| (d.type_, d.class_struct));
        let qualified_name = class_struct.qualified_name();
        write!(f, "{type_} class {qualified_name}")
    }
}
