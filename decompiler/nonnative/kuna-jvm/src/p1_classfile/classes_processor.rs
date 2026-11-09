//! Port of `org/jetbrains/java/decompiler/main/ClassesProcessor.java`.

#![allow(dead_code, unused_variables)]

use std::cmp::Ordering;
use std::fmt;

use crate::class_wrapper::ClassWrapper;
use crate::class_writer::ClassWriter;
use crate::code_constants::CONSTANT_METHOD_HANDLE_REF_INVOKE_STATIC;
use crate::i_identifier_renamer::IIdentifierRenamer;
use crate::import_collector::ImportCollector;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::plugin_context::LanguageSpec;
use crate::rt::{JHashMap, JHashSet, JLinkedHashSet, JList, JavaStr, Shared};
use crate::struct_class::StructClass;
use crate::struct_context::StructContext;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object, unported};

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
        unported!("ClassesProcessor.loadClasses")
    }

    fn is_anonymous(cl: StructClass, enclosing_cl: StructClass) -> bool {
        unported!("ClassesProcessor.isAnonymous")
    }

    pub fn process_class(self, cl: StructClass) {
        unported!("ClassesProcessor.processClass")
    }

    pub fn write_class(self, cl: StructClass, buffer: &mut TextBuffer) {
        unported!("ClassesProcessor.writeClass")
    }

    fn init_wrappers(node: ClassNode, spec: Option<&LanguageSpec>) {
        unported!("ClassesProcessor.initWrappers")
    }

    fn should_init_early(node: ClassNode) -> bool {
        unported!("ClassesProcessor.shouldInitEarly")
    }

    fn add_class_name_to_import(node: ClassNode, imp: ImportCollector) {
        unported!("ClassesProcessor.addClassNameToImport")
    }

    fn destroy_wrappers(node: ClassNode) {
        let class_struct = node.with_mut(|d| {
            d.wrapper = None;
            d.class_struct
        });
        class_struct.release_resources();
        let methods = class_struct.get_methods().borrow().to_vec();
        for m in methods {
            m.clear_variable_namer();
        }
    }

    pub fn get_map_root_classes(self) -> Shared<JHashMap<String, ClassNode>> {
        self.with(|d| d.map_root_classes.clone())
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
    /// Class scope: read it through [`ClassNode::get_wrapper`], which drops a stale handle.
    pub wrapper: Option<ClassWrapper>,
    pub enclosing_method: Option<String>,
    /// Class scope; only meaningful while its root class is being decompiled.
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

        let anonymous_class_type = VarType::new_cl(lambda_class_name, true);

        let mut is_method_reference = class_struct.with(|d| d.qualified_name != content_class_name);
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
        let qualified_name = class_struct.with(|d| d.qualified_name.clone());
        let simple_name = match qualified_name.rfind('/') {
            Some(i) => qualified_name[i + 1..].to_string(),
            None => qualified_name.clone(),
        };

        let mut enclosing_method = None;
        let enclosing_method_attr =
            class_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD());
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
            if class_struct.with(|d| d.qualified_name == qualified_name) {
                return Some(node);
            }
        }
        None
    }

    /// The wrapper of this node, or of its nearest non-lambda ancestor; `None`
    /// before the wrapper is built and once its root class has been finished.
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
        let a = self
            .with(|d| d.class_struct)
            .with(|d| d.qualified_name.clone());
        let b = o
            .with(|d| d.class_struct)
            .with(|d| d.qualified_name.clone());
        string_compare_to(&a, &b)
    }
}

impl fmt::Display for ClassNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (type_, class_struct) = self.with(|d| (d.type_, d.class_struct));
        let qualified_name = class_struct.with(|d| d.qualified_name.clone());
        write!(f, "{type_} class {qualified_name}")
    }
}
