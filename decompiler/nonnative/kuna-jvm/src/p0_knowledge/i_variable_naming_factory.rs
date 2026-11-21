//! Port of `org/jetbrains/java/decompiler/main/extern/IVariableNamingFactory.java`.

use std::rc::Rc;

use crate::i_variable_name_provider::IVariableNameProvider;
use crate::struct_method::StructMethod;

pub trait IVariableNamingFactory {
    fn create_factory(&self, struct_method: StructMethod) -> Rc<dyn IVariableNameProvider>;
}
