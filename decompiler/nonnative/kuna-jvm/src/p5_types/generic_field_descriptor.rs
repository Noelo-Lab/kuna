//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericFieldDescriptor.java`.

use std::cell::RefCell;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::generic_class_descriptor::GenericClassDescriptor;
use crate::i_fernflower_logger::Severity;
use crate::var_type::VarType;

/// A shared, mutable Java object reference (`verifyType` rewrites `type`).
#[derive(Clone)]
pub struct GenericFieldDescriptor(Rc<RefCell<GenericFieldDescriptorData>>);

pub struct GenericFieldDescriptorData {
    pub type_: VarType,
}

impl GenericFieldDescriptor {
    pub fn new(type_: VarType) -> GenericFieldDescriptor {
        GenericFieldDescriptor(Rc::new(RefCell::new(GenericFieldDescriptorData { type_ })))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &GenericFieldDescriptor) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn type_(&self) -> VarType {
        self.0.borrow().type_.clone()
    }

    pub fn set_type(&self, type_: VarType) {
        self.0.borrow_mut().type_ = type_;
    }

    pub fn verify_type(
        &self,
        containing_class_generics: Option<&GenericClassDescriptor>,
        real_type: &VarType,
    ) {
        let Some(containing_class_generics) = containing_class_generics else {
            self.verify_loosely(real_type);
            return;
        };

        let checker = containing_class_generics.get_checker();

        let type_ = self.type_();
        if !checker.is_properly_bounded(&type_, real_type) {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "Mismatched field signature, expected: {}, actual: {}",
                    real_type.value_or_null(),
                    type_.value_or_null()
                ),
                Severity::Warn,
            );
            self.set_type(real_type.clone());
        }
    }

    pub fn verify_loosely(&self, actual_type: &VarType) {
        let type_ = self.type_();
        if actual_type.higher_equal_in_lattice_than(&type_) {
            return;
        }

        if type_.type_ == CodeType::Genvar {
            return;
        }

        DecompilerContext::get_logger().write_message(
            &format!(
                "Mismatched field signature, expected: {}, actual: {}",
                type_.value_or_null(),
                actual_type.value_or_null()
            ),
            Severity::Warn,
        );
        self.set_type(actual_type.clone());
    }
}
