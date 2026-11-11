//! Port of `org/jetbrains/java/decompiler/api/plugin/PluginSource.java`.

use std::rc::Rc;

use crate::plugin::Plugin;

pub trait PluginSource {
    fn find_plugins(&self) -> Vec<Rc<dyn Plugin>>;

    /// `getClass().getSimpleName()`, as `--list-plugins` prints it.
    fn get_simple_name(&self) -> String {
        let name = std::any::type_name::<Self>();
        name.rsplit("::").next().unwrap_or(name).to_string()
    }
}
