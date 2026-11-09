//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericMethodDescriptor.java`.

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::decompiler_context::DecompilerContext;
use crate::generics_checker::GenericsChecker;
use crate::i_fernflower_logger::Severity;
use crate::var_type::VarType;

/// A shared, mutable Java object reference (`verifyTypes` rewrites fields).
#[derive(Clone)]
pub struct GenericMethodDescriptor(Rc<RefCell<GenericMethodDescriptorData>>);

pub struct GenericMethodDescriptorData {
    pub type_parameters: Vec<String>,
    pub type_parameter_bounds: Vec<Vec<VarType>>,
    pub parameter_types: Vec<VarType>,
    pub return_type: Option<VarType>,
    pub exception_types: Vec<VarType>,
}

impl GenericMethodDescriptor {
    pub fn new(
        type_parameters: Vec<String>,
        type_parameter_bounds: Vec<Vec<VarType>>,
        parameter_types: Vec<VarType>,
        return_type: Option<VarType>,
        exception_types: Vec<VarType>,
    ) -> GenericMethodDescriptor {
        GenericMethodDescriptor(Rc::new(RefCell::new(GenericMethodDescriptorData {
            type_parameters,
            type_parameter_bounds,
            parameter_types,
            return_type,
            exception_types,
        })))
    }

    pub fn borrow(&self) -> Ref<'_, GenericMethodDescriptorData> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, GenericMethodDescriptorData> {
        self.0.borrow_mut()
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &GenericMethodDescriptor) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn type_parameters(&self) -> Ref<'_, Vec<String>> {
        Ref::map(self.0.borrow(), |d| &d.type_parameters)
    }

    pub fn type_parameter_bounds(&self) -> Ref<'_, Vec<Vec<VarType>>> {
        Ref::map(self.0.borrow(), |d| &d.type_parameter_bounds)
    }

    pub fn parameter_types(&self) -> Ref<'_, Vec<VarType>> {
        Ref::map(self.0.borrow(), |d| &d.parameter_types)
    }

    pub fn return_type(&self) -> Option<VarType> {
        self.0.borrow().return_type.clone()
    }

    pub fn set_return_type(&self, return_type: Option<VarType>) {
        self.0.borrow_mut().return_type = return_type;
    }

    pub fn exception_types(&self) -> Ref<'_, Vec<VarType>> {
        Ref::map(self.0.borrow(), |d| &d.exception_types)
    }

    #[allow(clippy::needless_range_loop)]
    pub fn verify_types(
        &self,
        checker: Option<&GenericsChecker>,
        real_param_types: &[VarType],
        real_return_type: &VarType,
        real_exception_types: &[VarType],
    ) {
        let checker = {
            let d = self.0.borrow();
            match checker {
                None => GenericsChecker::new_bounds(&d.type_parameters, &d.type_parameter_bounds),
                Some(c) => c.copy(&d.type_parameters, &d.type_parameter_bounds),
            }
        };

        let count = self.parameter_types().len();
        for i in 0..count {
            let parameter_type = self.parameter_types()[i].clone();
            let real_type = &real_param_types[i];

            if !checker.is_properly_bounded(&parameter_type, real_type) {
                DecompilerContext::get_logger().write_message(
                    &format!(
                        "Mismatched method parameter signature, expected: {}, actual: {}",
                        real_type.value_or_null(),
                        parameter_type.value_or_null()
                    ),
                    Severity::Warn,
                );
                self.0.borrow_mut().parameter_types[i] = real_type.clone();
            }
        }

        let return_type = self.return_type().expect("NullPointerException");
        if !checker.is_properly_bounded(&return_type, real_return_type) {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "Mismatched method return signature, expected: {}, actual: {}",
                    real_return_type.value_or_null(),
                    return_type.value_or_null()
                ),
                Severity::Warn,
            );
            self.set_return_type(Some(real_return_type.clone()));
        }

        let count = self.exception_types().len();
        for i in 0..count {
            let exception_type = self.exception_types()[i].clone();
            let real_type = &real_exception_types[i];
            if !checker.is_properly_bounded(&exception_type, real_type) {
                DecompilerContext::get_logger().write_message(
                    &format!(
                        "Mismatched method exception signature, expected: {}, actual: {}",
                        real_type.value_or_null(),
                        exception_type.value_or_null()
                    ),
                    Severity::Warn,
                );
            }
        }
    }
}
