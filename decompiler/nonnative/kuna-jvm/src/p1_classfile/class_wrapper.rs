//! Port of `org/jetbrains/java/decompiler/main/rels/ClassWrapper.java`.
//!
//! `killThread` is not ported: methods are never run on a watchdog thread
//! (see `method_processor`).

#![allow(dead_code, unused_variables)]

use std::fmt;

use crate::classes_processor::ClassNode;
use crate::exprent::Exprent;
use crate::interpreter_util::InterpreterUtil;
use crate::method_properties::MethodProperties;
use crate::method_wrapper::MethodWrapper;
use crate::plugin_context::LanguageSpec;
use crate::rt::{JHashMap, JHashSet, Shared};
use crate::struct_class::StructClass;
use crate::vb_style_collection::VBStyleCollection;
use crate::{arena_identity, arena_object, unported};

pub struct ClassWrapperData {
    pub class_struct: StructClass,
    pub node: ClassNode,
    pub hidden_members: Shared<JHashSet<String>>,
    pub static_field_initializers: Shared<VBStyleCollection<Exprent, String>>,
    pub dynamic_field_initializers: Shared<VBStyleCollection<Exprent, String>>,
    pub methods: Shared<VBStyleCollection<MethodWrapper, String>>,
    pub method_properties: Shared<JHashMap<String, MethodProperties>>,
}

arena_object!(pub ClassWrapper => ClassWrapperData, Class);
arena_identity!(ClassWrapper);

impl ClassWrapper {
    const DEBUG_METHOD_FILTER: Option<&'static str> = None;

    pub fn new(node: ClassNode) -> ClassWrapper {
        let class_struct = node.with(|d| d.class_struct);
        ClassWrapper::alloc(ClassWrapperData {
            class_struct,
            node,
            hidden_members: Shared::new(JHashSet::new()),
            static_field_initializers: Shared::new(VBStyleCollection::new()),
            dynamic_field_initializers: Shared::new(VBStyleCollection::new()),
            methods: Shared::new(VBStyleCollection::new()),
            method_properties: Shared::new(JHashMap::new()),
        })
    }

    /// Decompiles every method of the class into a [`MethodWrapper`].
    pub fn init(self, spec: Option<&LanguageSpec>) {
        unported!("ClassWrapper.init")
    }

    /// Java: getMethodWrapper(String, String)
    pub fn get_method_wrapper(self, name: &str, descriptor: &str) -> Option<MethodWrapper> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.get_methods().borrow().get_with_key(&key)
    }

    /// Java: getMethodProperties(String, String)
    pub fn get_method_properties(self, name: &str, descriptor: &str) -> Option<MethodProperties> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.with(|d| d.method_properties.borrow().get_cloned(&key))
    }

    /// Java: getMethodWrapper(int)
    pub fn get_method_wrapper_index(self, index: i32) -> MethodWrapper {
        self.get_methods().borrow().get(index as usize)
    }

    pub fn get_class_struct(self) -> StructClass {
        self.with(|d| d.class_struct)
    }

    pub fn get_methods(self) -> Shared<VBStyleCollection<MethodWrapper, String>> {
        self.with(|d| d.methods.clone())
    }

    /// Java: getMethodProperties()
    pub fn get_method_properties_map(self) -> Shared<JHashMap<String, MethodProperties>> {
        self.with(|d| d.method_properties.clone())
    }

    pub fn get_hidden_members(self) -> Shared<JHashSet<String>> {
        self.with(|d| d.hidden_members.clone())
    }

    pub fn get_static_field_initializers(self) -> Shared<VBStyleCollection<Exprent, String>> {
        self.with(|d| d.static_field_initializers.clone())
    }

    pub fn get_dynamic_field_initializers(self) -> Shared<VBStyleCollection<Exprent, String>> {
        self.with(|d| d.dynamic_field_initializers.clone())
    }
}

impl fmt::Display for ClassWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class_struct = self.get_class_struct();
        let qualified_name = class_struct.with(|d| d.qualified_name.clone());
        f.write_str(&qualified_name)
    }
}
