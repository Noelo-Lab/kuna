//! Port of `org/jetbrains/java/decompiler/api/plugin/PluginSource.java`.

use std::rc::Rc;

use crate::plugin::Plugin;

pub trait PluginSource {
    fn find_plugins(&self) -> Vec<Rc<dyn Plugin>>;
}
