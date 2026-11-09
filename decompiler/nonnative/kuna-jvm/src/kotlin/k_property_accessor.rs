//! Port of `org/vineflower/kotlin/struct/KPropertyAccessor.java`.

use std::rc::Rc;

use crate::k_function::MethodSupplier;

#[derive(Clone)]
pub struct KPropertyAccessor {
    pub flags: i32,
    pub method_supplier: MethodSupplier,
}

impl KPropertyAccessor {
    pub fn new(flags: i32, method_supplier: MethodSupplier) -> KPropertyAccessor {
        KPropertyAccessor {
            flags,
            method_supplier,
        }
    }

    /// The record's `equals` (the supplier compares by identity).
    pub fn equals(&self, o: &KPropertyAccessor) -> bool {
        self.flags == o.flags && Rc::ptr_eq(&self.method_supplier, &o.method_supplier)
    }

    /// The record's `hashCode`, with the supplier's address as its identity hash.
    pub fn hash_code(&self) -> i32 {
        let supplier = Rc::as_ptr(&self.method_supplier) as *const () as usize as i32;
        self.flags.wrapping_mul(31).wrapping_add(supplier)
    }
}
