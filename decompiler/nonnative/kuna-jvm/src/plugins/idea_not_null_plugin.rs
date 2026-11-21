//! Port of `org/vineflower/ideanotnull/IdeaNotNullPlugin.java`.

use std::rc::Rc;

use crate::idea_not_null_options::IdeaNotNullOptions;
use crate::idea_not_null_pass::IdeaNotNullPass;
use crate::java_pass_location::JavaPassLocation;
use crate::java_pass_registrar::JavaPassRegistrar;
use crate::named_pass::NamedPass;
use crate::pair::Pair;
use crate::plugin::Plugin;
use crate::plugin_options::{DefaultsConsumer, PluginOptions};

#[derive(Clone, Copy, Debug, Default)]
pub struct IdeaNotNullPlugin;

impl IdeaNotNullPlugin {
    pub fn new() -> IdeaNotNullPlugin {
        IdeaNotNullPlugin
    }
}

impl Plugin for IdeaNotNullPlugin {
    fn id(&self) -> String {
        "IdeaNotNull".to_string()
    }

    fn description(&self) -> String {
        "Decompiles code inserted by Intellij IDEA's @NotNull annotation.".to_string()
    }

    fn register_java_passes(&self, registrar: &mut JavaPassRegistrar) {
        registrar.register(
            JavaPassLocation::MainLoop,
            NamedPass::new("IdeaNotNull", Rc::new(IdeaNotNullPass::new())),
        );
    }

    fn get_plugin_options(&self) -> Option<Rc<dyn PluginOptions>> {
        Some(Rc::new(|| {
            let defaults: DefaultsConsumer = Rc::new(IdeaNotNullOptions::add_defaults);
            Pair::of(IdeaNotNullOptions::OPTIONS, defaults)
        }))
    }
}
