//! The Kotlin element a method's metadata declares, whatever the writing of
//! its class has marked it as since.
//!
//! When `KotlinWriter` writes a class with a companion object, it reads the
//! companion's functions and properties again with the class as their parent
//! (`KotlinObjectProcessors::parse_with_companion`) and marks each method of
//! the class that has the JVM name and descriptor of one of them
//! `HiddenElement(CompanionItem)`, in place of the element the class's own
//! metadata gave it (`kotlin.random.Random.nextInt(until: Int)`, which the
//! companion `Default` overrides). A fix that asks what members another class
//! declares therefore got one answer before that class was written and
//! another after: the output depended on the order of the classes in the
//! input, and in the parallel driver on which batch wrote which class.
//! [`keep`] records the declared elements before the marks and [`get`]
//! answers from them.

use crate::k_element::KElement;
use crate::key::Key;
use crate::kt_attrs::KtAttrs;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

#[allow(non_snake_case)]
fn DECLARED() -> Key<KElement> {
    Key::of("kuna.declared_element")
}

/// Records the element of each method of `cl` that has one and none recorded.
pub fn keep(cl: StructClass) {
    for mt in cl.get_methods().to_vec() {
        if KtAttrs::has(mt, &DECLARED()) {
            continue;
        }
        if let Some(element) = KtAttrs::get(mt, &KElement::KEY()) {
            KtAttrs::put(mt, &DECLARED(), element);
        }
    }
}

/// The element `mt`'s metadata declares (`KtAttrs` `KElement`, as it was
/// before its class's writing marked it).
pub fn get(mt: StructMethod) -> Option<KElement> {
    KtAttrs::get(mt, &DECLARED()).or_else(|| KtAttrs::get(mt, &KElement::KEY()))
}
