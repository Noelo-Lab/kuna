//! Port of `org/jetbrains/java/decompiler/api/plugin/PluginOptions.java`. The
//! options interface Vineflower reads by reflection (`.a`, a `Class`) is the
//! table of its option fields here.

use std::rc::Rc;

use crate::i_fernflower_preferences::PreferenceOption;
use crate::pair::Pair;

/// `PluginOptions.AddDefaults`.
pub trait AddDefaults {
    fn add_default(&mut self, key: &str, default_val: &str);
}

impl<F: FnMut(&str, &str)> AddDefaults for F {
    fn add_default(&mut self, key: &str, default_val: &str) {
        self(key, default_val)
    }
}

/// The adder of a plugin's defaults (`Consumer<AddDefaults>`).
pub type DefaultsConsumer = Rc<dyn Fn(&mut dyn AddDefaults)>;

pub trait PluginOptions {
    fn provide_options(&self) -> Pair<&'static [PreferenceOption], DefaultsConsumer>;
}

impl<F: Fn() -> Pair<&'static [PreferenceOption], DefaultsConsumer>> PluginOptions for F {
    fn provide_options(&self) -> Pair<&'static [PreferenceOption], DefaultsConsumer> {
        self()
    }
}
