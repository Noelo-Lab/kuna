//! Port of `org/vineflower/variablerenaming/VariableRenamingPlugin.java`.

use std::cell::Cell;
use std::rc::Rc;

use crate::decompiler_context::DecompilerContext;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::jad_name_provider::JADNameProviderFactory;
use crate::pair::Pair;
use crate::plugin::Plugin;
use crate::plugin_options::{DefaultsConsumer, PluginOptions};
use crate::renamers::Renamers;
use crate::tiny_name_provider::TinyNameProviderFactory;
use crate::variable_renaming_options::VariableRenamingOptions;

thread_local! {
    static INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VariableRenamingPlugin;

impl VariableRenamingPlugin {
    pub fn new() -> VariableRenamingPlugin {
        VariableRenamingPlugin::static_init();
        VariableRenamingPlugin
    }

    /// The class's `static` block, run once per thread.
    pub fn static_init() {
        if INITIALIZED.with(|i| i.replace(true)) {
            return;
        }
        Renamers::register_provider("jad", Rc::new(JADNameProviderFactory));
        Renamers::register_provider("tiny", Rc::new(TinyNameProviderFactory));
    }
}

impl Plugin for VariableRenamingPlugin {
    fn id(&self) -> String {
        "VariableRenaming".to_string()
    }

    fn description(&self) -> String {
        "Allows automatic renaming of variables with a common naming scheme.".to_string()
    }

    fn get_plugin_options(&self) -> Option<Rc<dyn PluginOptions>> {
        Some(Rc::new(|| {
            let defaults: DefaultsConsumer = Rc::new(VariableRenamingOptions::add_defaults);
            Pair::of(VariableRenamingOptions::OPTIONS, defaults)
        }))
    }

    fn get_renaming_factory(&self) -> Option<Rc<dyn IVariableNamingFactory>> {
        let name = DecompilerContext::get_property(VariableRenamingOptions::VARIABLE_RENAMER);
        if let Some(name) = name {
            return Renamers::get(&name);
        }

        if DecompilerContext::get_option(VariableRenamingOptions::USE_JAD_VARNAMING) {
            return Renamers::get("jad");
        }

        None
    }
}
