//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericClassDescriptor.java`.

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::decompiler_context::DecompilerContext;
use crate::generic_type::GenericType;
use crate::generics_checker::GenericsChecker;
use crate::i_fernflower_logger::Severity;
use crate::var_type::VarType;

/// A shared, mutable Java object reference (`verifyTypes` rewrites fields).
#[derive(Clone)]
pub struct GenericClassDescriptor(Rc<RefCell<GenericClassDescriptorData>>);

pub struct GenericClassDescriptorData {
    pub superclass: VarType,
    pub generic_type: GenericType,
    pub superinterfaces: Vec<VarType>,
    pub fparameters: Vec<String>,
    pub fbounds: Vec<Vec<VarType>>,
    checker: Option<GenericsChecker>,
}

impl GenericClassDescriptor {
    pub fn new(
        superclass: VarType,
        generic_type: GenericType,
        superinterfaces: Vec<VarType>,
        fparameters: Vec<String>,
        fbounds: Vec<Vec<VarType>>,
    ) -> GenericClassDescriptor {
        GenericClassDescriptor(Rc::new(RefCell::new(GenericClassDescriptorData {
            superclass,
            generic_type,
            superinterfaces,
            fparameters,
            fbounds,
            checker: None,
        })))
    }

    pub fn borrow(&self) -> Ref<'_, GenericClassDescriptorData> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, GenericClassDescriptorData> {
        self.0.borrow_mut()
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &GenericClassDescriptor) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn superclass(&self) -> VarType {
        self.0.borrow().superclass.clone()
    }

    pub fn set_superclass(&self, superclass: VarType) {
        self.0.borrow_mut().superclass = superclass;
    }

    pub fn generic_type(&self) -> GenericType {
        self.0.borrow().generic_type.clone()
    }

    pub fn superinterfaces(&self) -> Ref<'_, Vec<VarType>> {
        Ref::map(self.0.borrow(), |d| &d.superinterfaces)
    }

    pub fn fparameters(&self) -> Ref<'_, Vec<String>> {
        Ref::map(self.0.borrow(), |d| &d.fparameters)
    }

    pub fn fbounds(&self) -> Ref<'_, Vec<Vec<VarType>>> {
        Ref::map(self.0.borrow(), |d| &d.fbounds)
    }

    pub fn get_checker(&self) -> GenericsChecker {
        if let Some(checker) = &self.0.borrow().checker {
            return checker.clone();
        }
        let checker = {
            let d = self.0.borrow();
            GenericsChecker::new_bounds(&d.fparameters, &d.fbounds)
        };
        self.0.borrow_mut().checker = Some(checker.clone());
        checker
    }

    pub fn verify_types(&self, actual_superclass: &VarType, actual_super_interfaces: &[VarType]) {
        let checker = self.get_checker();

        let superclass = self.superclass();
        if !checker.is_properly_bounded(actual_superclass, &superclass) {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "Mismatched superclass signature, expected: {}, actual: {}",
                    actual_superclass,
                    superclass.value_or_null()
                ),
                Severity::Warn,
            );
            self.set_superclass(actual_superclass.clone());
        }

        let mut i = 0;
        while i < self.superinterfaces().len() {
            let actual_super_interface = actual_super_interfaces.get(i);
            let super_interface = self.superinterfaces()[i].clone();

            let Some(actual_super_interface) = actual_super_interface else {
                DecompilerContext::get_logger().write_message(
                    "Actual superinterface count is less than expected",
                    Severity::Warn,
                );
                break;
            };

            if !checker.is_properly_bounded(actual_super_interface, &super_interface) {
                DecompilerContext::get_logger().write_message(
                    &format!(
                        "Mismatched superinterface signature, expected: {}, actual: {}",
                        actual_super_interface,
                        super_interface.value_or_null()
                    ),
                    Severity::Warn,
                );
                self.0.borrow_mut().superinterfaces[i] = actual_super_interface.clone();
            }
            i += 1;
        }
    }
}
