//! Port of `org/vineflower/kotlin/struct/KElement.java`: the sealed interface
//! is an enum over its permitted records.

use std::rc::Rc;

use crate::k_class::KClass;
use crate::k_constructor::KConstructor;
use crate::k_file::KFile;
use crate::k_function::KFunction;
use crate::k_function_reference::KFunctionReference;
use crate::k_hidden_element::KHiddenElement;
use crate::k_multifile_facade::KMultifileFacade;
use crate::k_property::KProperty;
use crate::key::Key;

#[derive(Clone)]
pub enum KElement {
    Class(Rc<KClass>),
    Constructor(Rc<KConstructor>),
    File(Rc<KFile>),
    Function(Rc<KFunction>),
    FunctionReference(KFunctionReference),
    HiddenElement(KHiddenElement),
    MultifileFacade(Rc<KMultifileFacade>),
    Property(Rc<KProperty>),
}

impl KElement {
    /// `KElement.KEY`: the member attribute holding a member's element (see `kt_attrs`).
    #[allow(non_snake_case)]
    pub fn KEY() -> Key<KElement> {
        Key::of("KElement")
    }
}
