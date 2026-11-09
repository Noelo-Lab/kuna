//! Port of `org/vineflower/kotlin/struct/KConstructor.java`.

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::decompiler_context::DecompilerContext;
use crate::default_args_map::DefaultArgsSupplier;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::k_function::MethodSupplier;
use crate::k_parameter::KParameter;
use crate::key::Key;
use crate::kt_flags::Flags;
use crate::kt_proto::{self, ClassKind};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::unported;
use crate::var_type::VarType;

#[derive(Clone)]
pub struct KConstructor {
    pub parameters: Rc<[KParameter]>,
    pub flags: i32,
    /// `null` for an annotation class's constructor (it has no `<init>`).
    pub method_supplier: Option<MethodSupplier>,
    pub is_primary: bool,
    pub default_args_supplier: DefaultArgsSupplier,
    pub class_struct: StructClass,
    /// `null` for an annotation class's constructor.
    pub method_struct: Option<StructMethod>,
    pub class_flags: i32,
}

thread_local! {
    static DEFAULT_CONSTRUCTOR_MARKER: VarType =
        VarType::new_signature_cl("kotlin/jvm/internal/DefaultConstructorMarker", true);
}

impl KConstructor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        parameters: Rc<[KParameter]>,
        flags: i32,
        method_supplier: Option<MethodSupplier>,
        is_primary: bool,
        default_args_supplier: DefaultArgsSupplier,
        class_struct: StructClass,
        method_struct: Option<StructMethod>,
        class_flags: i32,
    ) -> KConstructor {
        KConstructor {
            parameters,
            flags,
            method_supplier,
            is_primary,
            default_args_supplier,
            class_struct,
            method_struct,
            class_flags,
        }
    }

    /// `KConstructor.PRIMARY_CONSTRUCTOR`: the class attribute holding its
    /// primary constructor, the same object as the `KElement` of its `<init>`.
    #[allow(non_snake_case)]
    pub fn PRIMARY_CONSTRUCTOR() -> Key<Rc<KConstructor>> {
        Key::of("primary-constructor")
    }

    #[allow(non_snake_case)]
    fn DEFAULT_CONSTRUCTOR_MARKER() -> VarType {
        DEFAULT_CONSTRUCTOR_MARKER.with(VarType::clone)
    }

    pub fn parse(
        class_struct: StructClass,
        proto_class: &kt_proto::Class,
        resolver: Option<&MetadataNameResolver>,
    ) {
        unported!("KConstructor.parse")
    }

    fn should_hide_constructor(&self) -> bool {
        if !self.is_primary
            || !self.parameters.is_empty()
            || !DecompilerContext::get_option(IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR)
        {
            return false;
        }

        if Flags::VISIBILITY.get(self.flags) != Flags::VISIBILITY.get(self.class_flags)
            && Flags::CLASS_KIND.get(self.class_flags) != Some(ClassKind::EnumClass)
        {
            return false;
        }

        true
    }

    pub fn stringify(&self, wrapper: ClassWrapper, buffer: &mut TextBuffer, indent: i32) -> bool {
        unported!("KConstructor.stringify")
    }

    pub fn write_primary_constructor(
        &self,
        wrapper: ClassWrapper,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        unported!("KConstructor.writePrimaryConstructor")
    }
}
