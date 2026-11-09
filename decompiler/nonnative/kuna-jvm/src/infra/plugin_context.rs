//! Port of `org/jetbrains/java/decompiler/main/plugins/PluginContext.java`.
//!
//! Vineflower discovers plugins on the JVM classpath; kuna has no plugin
//! loading, so a plugin is registered explicitly with
//! [`PluginContext::register_plugin`] and contributes passes through
//! [`Plugin::register_java_passes`]. Plugin options and plugin sources are
//! not ported, and language plugins (`api/plugin/LanguageSpec`) are deferred to
//! the Kotlin stage: [`LanguageSpec`] is uninhabited, so every `spec != null`
//! branch of the Java code is unreachable here.

use std::rc::Rc;

use crate::cancelation_manager::CancelationManager;
use crate::class_pass::ClassPass;
use crate::class_pass_location::ClassPassLocation;
use crate::classes_processor::ClassNode;
use crate::decompiler_context::DecompilerContext;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::java_pass_location::JavaPassLocation;
use crate::java_pass_registrar::JavaPassRegistrar;
use crate::named_pass::NamedPass;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_class::StructClass;
use crate::{arena_identity, arena_object, jthrow};

/// Placeholder for the deferred `api/plugin/LanguageSpec`: no value exists.
#[derive(Clone, Copy, Debug)]
pub enum LanguageSpec {}

/// The part of `api/plugin/Plugin` a pass-contributing plugin needs.
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

    fn get_renaming_factory(&self) -> Option<Rc<dyn IVariableNamingFactory>> {
        None
    }
}

pub struct PluginContextData {
    pub plugins: Vec<Rc<dyn Plugin>>,
    pub initialized: bool,
    pub passes: JHashMap<JavaPassLocation, Vec<NamedPass>>,
    pub class_passes: JHashMap<ClassPassLocation, Vec<Rc<dyn ClassPass>>>,
    pub ids: JHashSet<String>,
}

arena_object!(pub PluginContext => PluginContextData, Session);
arena_identity!(PluginContext);

impl PluginContext {
    pub fn new() -> PluginContext {
        PluginContext::alloc(PluginContextData {
            plugins: Vec::new(),
            initialized: false,
            passes: JHashMap::new(),
            class_passes: JHashMap::new(),
            ids: JHashSet::new(),
        })
    }

    pub fn get_current_context() -> PluginContext {
        DecompilerContext::get_struct_context().get_plugin_context()
    }

    /// Stands in for classpath discovery (`registerPlugin(plugin, source)`).
    pub fn register_plugin(self, plugin: Rc<dyn Plugin>) {
        self.with_mut(|d| d.plugins.push(plugin));
    }

    /// No plugin sources exist, so discovery finds nothing.
    pub fn find_plugins(self) -> i32 {
        0
    }

    pub fn initialize(self) {
        if self.with(|d| d.initialized) {
            return;
        }

        self.with_mut(|d| d.initialized = true);

        let mut registrar = JavaPassRegistrar::new();
        let plugins = self.with(|d| d.plugins.clone());
        for plugin in plugins {
            let id = plugin.id();
            if !self.with_mut(|d| d.ids.add(id.clone())) {
                jthrow!(IllegalStateException; "Duplicate plugin {} with id {}", plugin.description(), id);
            }

            plugin.initialize();

            plugin.register_java_passes(&mut registrar);
            if let Some(spec) = plugin.get_language_spec() {
                match spec {}
            }
        }

        let passes = registrar.get_passes();
        let class_passes = registrar.get_class_passes();
        self.with_mut(|d| {
            d.passes = passes;
            d.class_passes = class_passes;
        });
    }

    pub fn close(self) {
        let plugins = self.with(|d| d.plugins.clone());
        for plugin in plugins {
            plugin.close();
        }
    }

    /// Runs the passes registered at `location`; at a looping location, returns
    /// true as soon as one reports a change.
    pub fn run_passes(self, location: JavaPassLocation, ctx: &mut PassContext) -> bool {
        let passes = self.with(|d| d.passes.get_cloned(&location).unwrap_or_default());

        for pass in passes {
            CancelationManager::check_canceled();
            if pass.run(ctx) && location.is_loop() {
                return true;
            }
        }

        false
    }

    pub fn run_class_passes(self, location: ClassPassLocation, node: ClassNode) {
        let passes = self.with(|d| d.class_passes.get_cloned(&location).unwrap_or_default());
        for pass in passes {
            pass.run(node);
        }
    }

    /// Always `None`: no plugin can supply a [`LanguageSpec`].
    pub fn get_language_spec(self, cl: StructClass) -> Option<LanguageSpec> {
        let _ = cl;
        None
    }

    pub fn get_variable_renamer(self) -> Option<Rc<dyn IVariableNamingFactory>> {
        let plugins = self.with(|d| d.plugins.clone());
        for plugin in plugins {
            let factory = plugin.get_renaming_factory();
            if factory.is_some() {
                return factory;
            }
        }

        None
    }

    pub fn get_plugins(self) -> Vec<Rc<dyn Plugin>> {
        self.with(|d| d.plugins.clone())
    }
}
