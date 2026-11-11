//! Port of `org/jetbrains/java/decompiler/api/plugin/Plugin.java`.

use std::rc::Rc;

use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::java_pass_registrar::JavaPassRegistrar;
use crate::language_spec::LanguageSpec;
use crate::plugin_options::PluginOptions;

pub trait Plugin {
    fn id(&self) -> String;

    fn description(&self) -> String;

    fn initialize(&self) {}

    fn before_decompile(&self) {}

    fn close(&self) {}

    fn register_java_passes(&self, _registrar: &mut JavaPassRegistrar) {}

    fn get_language_spec(&self) -> Option<LanguageSpec> {
        None
    }

    fn get_plugin_options(&self) -> Option<Rc<dyn PluginOptions>> {
        None
    }

    fn get_renaming_factory(&self) -> Option<Rc<dyn IVariableNamingFactory>> {
        None
    }
}
