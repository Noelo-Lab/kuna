//! Port of `org/jetbrains/java/decompiler/struct/gen/MethodDescriptor.java`.
//! kuna fix `constructor_signature_alignment`: `parseDescriptor` also trims a
//! signature that omits only the leading synthetic parameters.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_ENUM, INIT_NAME};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::kuna_fixes::{self, KunaFix};
use crate::new_class_name_builder::NewClassNameBuilder;
use crate::rt::{JavaEq, JavaHash};
use crate::struct_method::StructMethod;
use crate::substrate::kuna_fix_constructor_signature_alignment;
use crate::var_type::{JStr, VarType};

/// Immutable except for `genericInfo`, which Java assigns after construction.
#[derive(Clone)]
pub struct MethodDescriptor(Rc<MethodDescriptorData>);

pub struct MethodDescriptorData {
    pub params: Vec<VarType>,
    pub ret: VarType,
    descriptor: String,
    pub generic_info: RefCell<Option<GenericMethodDescriptor>>,
}

impl Deref for MethodDescriptor {
    type Target = MethodDescriptorData;
    fn deref(&self) -> &MethodDescriptorData {
        &self.0
    }
}

impl MethodDescriptor {
    fn new(params: Vec<VarType>, ret: VarType, descriptor: String) -> MethodDescriptor {
        MethodDescriptor(Rc::new(MethodDescriptorData {
            params,
            ret,
            descriptor,
            generic_info: RefCell::new(None),
        }))
    }

    /// kuna fix `kt_inlined_stdlib_call`: this descriptor returning `ret`.
    pub fn with_ret(&self, ret: VarType) -> MethodDescriptor {
        MethodDescriptor::new(self.params.clone(), ret, self.descriptor.clone())
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &MethodDescriptor) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// Java: `parseDescriptor(String)`
    pub fn parse_descriptor(descriptor: &str) -> MethodDescriptor {
        let d = JStr::new(descriptor);
        let parenth = d.last_index_of(b')' as u16);
        if d.len() < 2 || parenth < 0 || d.at(0) != b'(' as u16 {
            crate::jthrow!(IllegalArgumentException; "Invalid descriptor: {}", descriptor);
        }
        let parenth = parenth as usize;

        let params = if parenth > 1 {
            let parameters_string = d.sub(1, parenth);
            let parameters = JStr::new(&parameters_string);
            let mut lst: Vec<String> = Vec::new();

            let mut index_from: i32 = -1;
            let len = parameters.len() as i32;
            let mut index: i32 = 0;
            while index < len {
                let start = if index_from < 0 { index } else { index_from } as usize;
                match parameters.at(index as usize) {
                    c if c == b'[' as u16 => {
                        if index_from < 0 {
                            index_from = index;
                        }
                    }
                    c if c == b'L' as u16 => {
                        let ind = parameters.index_of_from(b';' as u16, index as usize);
                        if ind < 0 && start == 0 {
                            // Java loops here forever, growing the list until OutOfMemoryError.
                            crate::jthrow!(OutOfMemoryError; "Invalid descriptor: {}", descriptor);
                        }
                        lst.push(parameters.sub(start, (ind + 1) as usize));
                        index = ind;
                        index_from = -1;
                    }
                    _ => {
                        lst.push(parameters.sub(start, (index + 1) as usize));
                        index_from = -1;
                    }
                }
                index += 1;
            }

            lst.iter().map(|s| VarType::new_signature(s)).collect()
        } else {
            VarType::EMPTY_ARRAY()
        };

        let ret = VarType::new_signature(&d.sub_from(parenth + 1));

        MethodDescriptor::new(params, ret, descriptor.to_string())
    }

    /// Java: `parseDescriptor(StructMethod, ClassNode)`
    pub fn parse_descriptor_struct(
        struct_: StructMethod,
        node: Option<ClassNode>,
    ) -> MethodDescriptor {
        let md = MethodDescriptor::parse_descriptor(&struct_.get_descriptor());

        let Some(mut sig) = struct_.get_signature() else {
            return md;
        };

        if let Some(node) = node {
            let method_wrapper = node
                .get_wrapper()
                .expect("NullPointerException")
                .get_method_wrapper(&struct_.get_name(), &struct_.get_descriptor());
            let init = INIT_NAME == struct_.get_name()
                && node.with(|d| d.type_) != ClassNodeType::Anonymous;
            let mut actual_params: i64 = md.params.len() as i64;
            let sig_fields: Option<Vec<bool>> = method_wrapper.and_then(|mw| {
                mw.with(|d| {
                    d.synth_parameters
                        .as_ref()
                        .map(|v| v.iter().map(|p| p.is_none()).collect())
                })
            });
            if let Some(sig_fields) = &sig_fields {
                actual_params = sig_fields.iter().filter(|&&is_null| is_null).count() as i64;
            } else if init && DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ENUM)
            {
                let mut class_name = struct_.get_class_qualified_name();
                if DecompilerContext::get_option(IFernflowerPreferences::RENAME_ENTITIES) {
                    class_name = DecompilerContext::get_pool_interceptor()
                        .expect("NullPointerException")
                        .get_old_name(&class_name)
                        .unwrap_or_else(|| struct_.get_class_qualified_name());
                }

                if DecompilerContext::get_struct_context()
                    .get_class(&class_name)
                    .expect("NullPointerException")
                    .has_modifier(ACC_ENUM)
                {
                    actual_params -= 2;
                }
            }

            let sig_param_count = sig.parameter_types().len();
            if actual_params != sig_param_count as i64 {
                let describe = || {
                    format!(
                        "{} {}{}",
                        struct_.get_class_qualified_name(),
                        struct_.get_name(),
                        struct_.get_descriptor()
                    )
                };
                if sig_fields
                    .as_ref()
                    .is_none_or(|f| f.len() != sig_param_count)
                {
                    if kuna_fixes::enabled(KunaFix::ConstructorSignatureAlignment) {
                        if let Some(aligned) = sig_fields.as_ref().and_then(|f| {
                            kuna_fix_constructor_signature_alignment::align(&sig, &md.params, f)
                        }) {
                            let message = format!(
                                "Descriptor and signature parameter counts mismatch, attempting to correct: {}",
                                describe()
                            );
                            DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                            md.add_generic_descriptor(Some(aligned));
                            return md;
                        }
                    }
                    let message = format!(
                        "Descriptor and signature parameter counts mismatch, assuming ok: {}",
                        describe()
                    );
                    DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                    md.add_generic_descriptor(Some(sig));
                    return md;
                }

                let message = format!(
                    "Descriptor and signature parameter counts mismatch, attempting to correct: {}",
                    describe()
                );
                DecompilerContext::get_logger().write_message(&message, Severity::Warn);
                let sig_fields = sig_fields.unwrap();
                let mut new_var_types = Vec::new();
                for (i, &is_null) in sig_fields.iter().enumerate() {
                    if is_null {
                        new_var_types.push(sig.parameter_types()[i].clone());
                    }
                }
                let (type_parameters, type_parameter_bounds, return_type, exception_types) = {
                    let d = sig.borrow();
                    (
                        d.type_parameters.clone(),
                        d.type_parameter_bounds.clone(),
                        d.return_type.clone(),
                        d.exception_types.clone(),
                    )
                };
                sig = GenericMethodDescriptor::new(
                    type_parameters,
                    type_parameter_bounds,
                    new_var_types,
                    return_type,
                    exception_types,
                );
            }
        }
        md.add_generic_descriptor(Some(sig));

        md
    }

    pub fn add_generic_descriptor(&self, desc: Option<GenericMethodDescriptor>) {
        *self.generic_info.borrow_mut() = desc;
    }

    /// The `genericInfo` field.
    pub fn generic_info(&self) -> Option<GenericMethodDescriptor> {
        self.generic_info.borrow().clone()
    }

    /// `genericInfo = desc`.
    pub fn set_generic_info(&self, desc: Option<GenericMethodDescriptor>) {
        *self.generic_info.borrow_mut() = desc;
    }

    pub fn build_new_descriptor(&self, builder: &dyn NewClassNameBuilder) -> Option<String> {
        let mut updated = false;

        let new_params: Vec<VarType> = if !self.params.is_empty() {
            let mut new_params = self.params.clone();
            for (i, param) in self.params.iter().enumerate() {
                if let Some(substitute) = MethodDescriptor::build_new_type(param, builder) {
                    new_params[i] = substitute;
                    updated = true;
                }
            }
            new_params
        } else {
            VarType::EMPTY_ARRAY()
        };

        let mut new_ret = self.ret.clone();
        if let Some(substitute) = MethodDescriptor::build_new_type(&self.ret, builder) {
            new_ret = substitute;
            updated = true;
        }

        if updated {
            let mut res = String::from("(");
            for param in &new_params {
                res.push_str(&param.to_string());
            }
            res.push(')');
            res.push_str(&new_ret.to_string());
            return Some(res);
        }

        None
    }

    fn build_new_type(type_: &VarType, builder: &dyn NewClassNameBuilder) -> Option<VarType> {
        if type_.type_ == CodeType::Object {
            if let Some(new_class_name) = builder.build_new_classname(type_.value_str()) {
                return Some(VarType::new_value(
                    type_.type_,
                    type_.array_dim,
                    new_class_name,
                ));
            }
        }
        None
    }

    pub fn equals(&self, o: &MethodDescriptor) -> bool {
        if o.ptr_eq(self) {
            return true;
        }
        self.ret.equals(&o.ret)
            && self.params.len() == o.params.len()
            && self.params.iter().zip(&o.params).all(|(a, b)| a.equals(b))
    }

    pub fn hash_code(&self) -> i32 {
        let mut result = self.ret.hash_code();
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.params.len() as i32);
        result
    }
}

impl std::fmt::Display for MethodDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.descriptor)
    }
}

impl std::fmt::Debug for MethodDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodDescriptor({})", self.descriptor)
    }
}

impl JavaHash for MethodDescriptor {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for MethodDescriptor {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
