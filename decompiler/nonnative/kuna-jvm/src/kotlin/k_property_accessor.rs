//! Port of `org/vineflower/kotlin/struct/KPropertyAccessor.java`.

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
}
