//! Port of `org/jetbrains/java/decompiler/main/plugins/PluginContext.java`.
//!
//! Vineflower discovers plugins through its static plugin sources (plugin
//! jars, `ServiceLoader`); here each context holds its own sources
//! ([`PluginContext::set_plugin_sources`], see `kuna_plugins`), and a plugin
//! can also be registered directly with [`PluginContext::register_plugin`].

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
use crate::plugin_source::PluginSource;
use crate::rt::{JHashMap, JHashSet};
use crate::struct_class::StructClass;
use crate::{arena_identity, arena_object, jthrow};

pub use crate::language_spec::LanguageSpec;
pub use crate::plugin::Plugin;

pub struct PluginContextData {
    pub plugins: Vec<Rc<dyn Plugin>>,
    /// `bySource`, by position in `plugins`.
    pub by_source: Vec<Option<Rc<dyn PluginSource>>>,
    /// `PluginSources.PLUGIN_SOURCES` for this context (none by default).
    pub plugin_sources: Vec<Rc<dyn PluginSource>>,
    pub initialized: bool,
    pub passes: JHashMap<JavaPassLocation, Vec<NamedPass>>,
    pub class_passes: JHashMap<ClassPassLocation, Vec<Rc<dyn ClassPass>>>,
    /// `languageSpecs`, in the order of their plugins.
    pub language_specs: Vec<Rc<LanguageSpec>>,
    pub ids: JHashSet<String>,
}

arena_object!(pub PluginContext => PluginContextData, Session);
arena_identity!(PluginContext);

impl PluginContext {
    pub fn new() -> PluginContext {
        PluginContext::alloc(PluginContextData {
            plugins: Vec::new(),
            by_source: Vec::new(),
            plugin_sources: Vec::new(),
            initialized: false,
            passes: JHashMap::new(),
            class_passes: JHashMap::new(),
            language_specs: Vec::new(),
            ids: JHashSet::new(),
        })
    }

    pub fn get_current_context() -> PluginContext {
        DecompilerContext::get_struct_context().get_plugin_context()
    }

    pub fn set_plugin_sources(self, sources: Vec<Rc<dyn PluginSource>>) {
        self.with_mut(|d| d.plugin_sources = sources);
    }

    /// A plugin from no source.
    pub fn register_plugin(self, plugin: Rc<dyn Plugin>) {
        self.with_mut(|d| {
            d.plugins.push(plugin);
            d.by_source.push(None);
        });
    }

    /// Java: registerPlugin(Plugin, PluginSource)
    pub fn register_plugin_from(self, plugin: Rc<dyn Plugin>, source: Rc<dyn PluginSource>) {
        self.with_mut(|d| {
            d.plugins.push(plugin);
            d.by_source.push(Some(source));
        });
    }

    pub fn find_plugins(self) -> i32 {
        let mut plugin_count = 0;
        let sources = self.with(|d| d.plugin_sources.clone());
        for source in sources {
            for plugin in source.find_plugins() {
                self.register_plugin_from(plugin, source.clone());
                plugin_count += 1;
            }
        }

        plugin_count
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
                self.with_mut(|d| d.language_specs.push(Rc::new(spec)));
            }

            if let Some(opt) = plugin.get_plugin_options() {
                let add_defaults = opt.provide_options().b;
                let mut defaults: Vec<(String, String)> = Vec::new();
                add_defaults(&mut |key: &str, default_val: &str| {
                    defaults.push((key.to_string(), default_val.to_string()))
                });
                let Some(context) = DecompilerContext::get_current_context() else {
                    jthrow!(NullPointerException; "DecompilerContext.getCurrentContext()");
                };
                context.with_mut(|d| {
                    for (key, default_val) in defaults {
                        d.properties.put_if_absent(key, default_val);
                    }
                });
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

    /// The spec of the first plugin whose chooser claims `cl`.
    pub fn get_language_spec(self, cl: StructClass) -> Option<Rc<LanguageSpec>> {
        let specs = self.with(|d| {
            if d.language_specs.is_empty() {
                None
            } else {
                Some(d.language_specs.clone())
            }
        })?;
        specs.into_iter().find(|spec| spec.chooser.is_language(cl))
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

    pub fn get_source(self, plugin: &Rc<dyn Plugin>) -> Option<Rc<dyn PluginSource>> {
        self.with(|d| {
            d.plugins
                .iter()
                .position(|p| Rc::ptr_eq(p, plugin))
                .and_then(|i| d.by_source[i].clone())
        })
    }

    pub fn get_plugins(self) -> Vec<Rc<dyn Plugin>> {
        self.with(|d| d.plugins.clone())
    }
}
