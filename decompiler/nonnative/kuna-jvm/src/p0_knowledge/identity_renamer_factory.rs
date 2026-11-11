//! Port of `org/jetbrains/java/decompiler/main/IdentityRenamerFactory.java`.

use std::rc::Rc;

use crate::i_variable_name_provider::IVariableNameProvider;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::pair::Pair;
use crate::rt::{JHashMap, JLinkedHashMap};
use crate::struct_method::StructMethod;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

/// Stateless, so `createFactory` hands out a fresh instance where Java returns `this`.
#[derive(Clone, Copy, Default, Debug)]
pub struct IdentityRenamerFactory;

impl IVariableNamingFactory for IdentityRenamerFactory {
    fn create_factory(&self, _method: StructMethod) -> Rc<dyn IVariableNameProvider> {
        Rc::new(IdentityRenamerFactory)
    }
}

impl IVariableNameProvider for IdentityRenamerFactory {
    fn rename(
        &self,
        _variables: &JLinkedHashMap<VarVersionPair, Pair<Option<VarType>, Option<String>>>,
    ) -> Option<JHashMap<VarVersionPair, String>> {
        None
    }

    fn add_parent_context(&self, _renamer: Rc<dyn IVariableNameProvider>) {}
}
