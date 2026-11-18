//! kuna fix `kt_companion_property_initializer`: a companion object's
//! property takes its initializer from the class that holds its field.
//!
//! kotlinc stores a companion object's property in a static field of the
//! outer class and initialises it in the outer class's `<clinit>`. The
//! initializer processor moves those stores out of `<clinit>` into the outer
//! class's static field initializers, the outer class hides the field (it
//! belongs to the companion), and `KProperty` looks the initializer up in the
//! companion's own initializers, where it is not: the property is written
//! without it and the value is lost (`val INSTANCES: MutableMap<..>` with no
//! map, every `CipherSuite.TLS_..` constant without its value). With the fix
//! the initializer is read from the static field initializers of the class
//! whose field it is.

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::decompiler_context::DecompilerContext;
use crate::k_property::ExprentSupplier;

/// The initializer of the static field `key` of the class `owner`.
pub fn static_initializer(owner: String, key: String) -> ExprentSupplier {
    Rc::new(move |_writing: ClassWrapper| {
        let node = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get(&owner)
            .copied()?;
        node.get_wrapper()?
            .get_static_field_initializers()
            .borrow()
            .get_with_key(&key)
    })
}
