//! The plugins bundled with kuna-jvm and the kuna option `jvm_plugins` that
//! turns them on or off. Vineflower finds plugins on the JVM class path
//! (`PluginSources`: plugin jars and `ServiceLoader`); `java -jar
//! vineflower.jar` carries VariableRenaming, Kotlin and IdeaNotNull that way,
//! its test fixture none. Here the bundled list is the one plugin source, on
//! by default as in `java -jar`; the corpus fixture runner turns it off.

use std::rc::Rc;

use crate::i_fernflower_preferences::PreferenceOption;
use crate::plugin::Plugin;
use crate::plugin_source::PluginSource;

/// The custom property that selects the bundled plugins (`1`/`on`, the
/// default, or `0`/`off`). `Fernflower::new` takes it off the property list.
pub const JVM_PLUGINS: &str = "jvm_plugins";

/// The bundled plugins, in the order `java -jar vineflower.jar` registers
/// them (`--list-plugins`): VariableRenaming, Kotlin, IdeaNotNull.
const BUNDLED: &[fn() -> Rc<dyn Plugin>] = &[];

/// A fresh instance of every bundled plugin, in registration order.
pub fn bundled_plugins() -> Vec<Rc<dyn Plugin>> {
    BUNDLED.iter().map(|make| make()).collect()
}

/// The plugin source of the bundled plugins (Vineflower's `ServicePluginSource`).
pub struct BundledPluginSource;

impl PluginSource for BundledPluginSource {
    fn find_plugins(&self) -> Vec<Rc<dyn Plugin>> {
        bundled_plugins()
    }

    fn get_simple_name(&self) -> String {
        "ServicePluginSource".to_string()
    }
}

/// `PluginSources.PLUGIN_SOURCES` of one engine instance.
pub fn plugin_sources(enabled: bool) -> Vec<Rc<dyn PluginSource>> {
    if enabled {
        vec![Rc::new(BundledPluginSource)]
    } else {
        Vec::new()
    }
}

/// Whether a `jvm_plugins` value turns the bundled plugins on.
pub fn switch_value(value: &str) -> bool {
    !(value == "0" || value.eq_ignore_ascii_case("off") || value.eq_ignore_ascii_case("false"))
}

/// Splits `jvm_plugins` off a custom property list: whether the bundled
/// plugins are on (the last value wins; absent: on) and the other pairs.
pub fn take_switch(custom_properties: &[(String, String)]) -> (bool, Vec<(String, String)>) {
    let mut enabled = true;
    let mut rest = Vec::with_capacity(custom_properties.len());
    for (key, value) in custom_properties {
        if key == JVM_PLUGINS {
            enabled = switch_value(value);
        } else {
            rest.push((key.clone(), value.clone()));
        }
    }
    (enabled, rest)
}

/// One option a bundled plugin declares (`DecompilerOption` with a plugin).
#[derive(Clone, Debug)]
pub struct PluginOption {
    pub plugin: String,
    pub option: &'static PreferenceOption,
    /// What the plugin's `AddDefaults` puts for the key.
    pub default: Option<String>,
}

/// Every option of every bundled plugin, plugin by plugin in registration order.
pub fn bundled_plugin_options() -> Vec<PluginOption> {
    let mut out = Vec::new();
    for plugin in bundled_plugins() {
        let Some(options) = plugin.get_plugin_options() else {
            continue;
        };
        let provided = options.provide_options();
        let mut defaults: Vec<(String, String)> = Vec::new();
        (provided.b)(&mut |key: &str, value: &str| {
            defaults.push((key.to_string(), value.to_string()))
        });
        for option in provided.a {
            let default = defaults
                .iter()
                .rev()
                .find(|(key, _)| key == option.key)
                .map(|(_, value)| value.clone());
            out.push(PluginOption {
                plugin: plugin.id(),
                option,
                default,
            });
        }
    }
    out
}

/// The bundled plugin option with key `key`.
pub fn find_plugin_option(key: &str) -> Option<PluginOption> {
    bundled_plugin_options()
        .into_iter()
        .find(|o| o.option.key == key)
}
