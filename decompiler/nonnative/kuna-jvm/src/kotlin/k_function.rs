//! Port of `org/vineflower/kotlin/struct/KFunction.java`.
//!
//! The record lives for the session (it is attached to members by `kt_attrs`);
//! its suppliers take the writing class's `ClassWrapper` and build what they
//! return in that class's scope.

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::default_args_map::{DefaultArgsMap, DefaultArgsSupplier};
use crate::jthrow;
use crate::k_contract::KContract;
use crate::k_parameter::KParameter;
use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::unported;

/// `Function<ClassWrapper, MethodWrapper>`; `getMethodWrapper` may answer `null`.
pub type MethodSupplier = Rc<dyn Fn(ClassWrapper) -> Option<MethodWrapper>>;

pub struct KFunction {
    pub name: String,
    pub parameters: Rc<[KParameter]>,
    pub type_parameters: Vec<KTypeParameter>,
    pub return_type: KType,
    pub flags: i32,
    pub context_receiver_types: Vec<KType>,
    pub method_supplier: MethodSupplier,
    pub receiver_type: Option<KType>,
    pub contract: Option<KContract>,
    pub known_override: bool,
    pub default_args_supplier: DefaultArgsSupplier,
    /// `null` only in `FAILED_LAMBDA`.
    pub class_struct: Option<StructClass>,
    /// `null` only in `FAILED_LAMBDA`.
    pub method_struct: Option<StructMethod>,
}

thread_local! {
    static FAILED_LAMBDA: Rc<KFunction> = Rc::new(KFunction::new(
        "failed-lambda".to_string(),
        Rc::from(Vec::new()),
        Vec::new(),
        KType::NOTHING(),
        0,
        Vec::new(),
        Rc::new(|_: ClassWrapper| None),
        None,
        None,
        false,
        Rc::new(|_: ClassWrapper| -> DefaultArgsMap {
            jthrow!(IllegalStateException; "Attempted to read default arguments for failed lambda")
        }),
        None,
        None,
    ));
}

impl KFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        parameters: Rc<[KParameter]>,
        type_parameters: Vec<KTypeParameter>,
        return_type: KType,
        flags: i32,
        context_receiver_types: Vec<KType>,
        method_supplier: MethodSupplier,
        receiver_type: Option<KType>,
        contract: Option<KContract>,
        known_override: bool,
        default_args_supplier: DefaultArgsSupplier,
        class_struct: Option<StructClass>,
        method_struct: Option<StructMethod>,
    ) -> KFunction {
        KFunction {
            name,
            parameters,
            type_parameters,
            return_type,
            flags,
            context_receiver_types,
            method_supplier,
            receiver_type,
            contract,
            known_override,
            default_args_supplier,
            class_struct,
            method_struct,
        }
    }

    /// `KFunction.FAILED_LAMBDA`; compare with `Rc::ptr_eq` (Java `==`).
    #[allow(non_snake_case)]
    pub fn FAILED_LAMBDA() -> Rc<KFunction> {
        FAILED_LAMBDA.with(Rc::clone)
    }

    pub fn parse(
        class_struct: StructClass,
        resolver: Option<&MetadataNameResolver>,
        type_table: &kt_proto::TypeTable,
        proto_functions: &[kt_proto::Function],
        is_synthetic_function: bool,
        companion_parent: Option<StructClass>,
    ) {
        unported!("KFunction.parse")
    }

    pub fn stringify(&self, wrapper: ClassWrapper, indent: i32) -> TextBuffer {
        unported!("KFunction.stringify")
    }
}
