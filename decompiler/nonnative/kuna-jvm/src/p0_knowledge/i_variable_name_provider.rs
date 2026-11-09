//! Port of `org/jetbrains/java/decompiler/main/extern/IVariableNameProvider.java`.

use std::rc::Rc;

use crate::code_constants::{ACC_ABSTRACT, ACC_NATIVE};
use crate::pair::Pair;
use crate::rt::{JHashMap, JLinkedHashMap};
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub trait IVariableNameProvider {
    /// `None` keeps every name (Java returns `null`).
    fn rename(
        &self,
        variables: &JLinkedHashMap<VarVersionPair, Pair<Option<VarType>, Option<String>>>,
    ) -> Option<JHashMap<VarVersionPair, String>>;

    /// `name` is `None` where the caller had no name (Java `null`).
    fn rename_abstract_parameter(&self, name: Option<&str>, _index: i32) -> Option<String> {
        name.map(str::to_string)
    }

    fn rename_parameter(
        &self,
        flags: i32,
        _type_: &VarType,
        name: Option<&str>,
        index: i32,
    ) -> Option<String> {
        if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0 {
            return self.rename_abstract_parameter(name, index);
        }

        name.map(str::to_string)
    }

    fn add_parent_context(&self, renamer: Rc<dyn IVariableNameProvider>);
}
