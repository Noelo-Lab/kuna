//! Port of `org/jetbrains/java/decompiler/main/Fernflower.java`, adapted to
//! kuna: finished classes are handed to a callback instead of an
//! `IResultSaver`; plugins come from the bundled list (`kuna_plugins`, chosen
//! by the custom property `jvm_plugins`) instead of the class path, renamer
//! class loading has no Rust counterpart, and `include-classpath` adds the
//! current JDK runtime (the system-module half of `ClasspathScanner`; the
//! JVM's own class path has no analogue here).
//!
//! Class-scope arenas are reset at the end of [`Fernflower::decompile_context`],
//! once every class of every unit has been delivered: Vineflower emits all
//! classes only after pre-processing all of them, and the emit phase reads the
//! wrappers of other root classes' nested classes (`ClassesProcessor.destroyWrappers`
//! keeps them for that), so no class-scope object may die earlier.

use std::cell::{Cell, OnceCell};
use std::collections::BTreeSet;
use std::path::Path;
use std::rc::Rc;

use crate::class_writer::ClassWriter;
use crate::classes_processor::{ClassNode, ClassNodeType, ClassesProcessor};
use crate::context_source::{read_class_info, IContextSource};
use crate::converter_helper::ConverterHelper;
use crate::decompiler_context::DecompilerContext;
use crate::i_decompiled_data::IDecompiledData;
use crate::i_fernflower_logger::{IFernflowerLogger, Severity};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_identifier_renamer::IIdentifierRenamer;
use crate::identifier_converter::IdentifierConverter;
use crate::identity_renamer_factory::IdentityRenamerFactory;
use crate::jdk_runtime::JdkRuntime;
use crate::kuna_partition::RootSelection;
use crate::kuna_plugins;
use crate::kuna_spans::{ClassSpan, MethodSpan};
use crate::plugin_context::PluginContext;
use crate::plugin_source::PluginSource;
use crate::pool_interceptor::PoolInterceptor;
use crate::rt::{try_catch, Caught, JHashMap, JavaException, JavaStr, Shared};
use crate::struct_class::StructClass;
use crate::struct_context::{qualified_name_of, StructContext};
use crate::text_buffer::TextBuffer;
use crate::text_token_dump_visitor::TextTokenDumpVisitor;
use crate::text_token_visitor::{Factory, TextTokenVisitor};

/// One decompiled root class, as the Java's `IOutputSink.acceptClass` receives it,
/// plus where its classes and methods lie in `content` (kuna).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DecompiledClass {
    pub qualified_name: String,
    /// The output entry, e.g. `a/b/C.java`.
    pub path: String,
    pub content: String,
    /// The original line mapping when `bytecode-source-mapping` is on.
    pub mapping: Option<Vec<i32>>,
    /// Why the class could not be written normally (`content` is then the
    /// error comment Vineflower writes in its place).
    pub error: Option<String>,
    /// Every class written into `content`, in writing order.
    pub classes: Vec<ClassSpan>,
    /// Every method written into `content`, in writing order.
    pub methods: Vec<MethodSpan>,
}

/// `JrtFinder.CURRENT`.
const JRT_CURRENT: &str = "current";

pub struct Fernflower {
    struct_context: StructContext,
    class_processor: ClassesProcessor,
    helper: Option<Rc<dyn IIdentifierRenamer>>,
    converter: Option<IdentifierConverter>,
    data: Rc<FernflowerDecompiledData>,
    loaded: bool,
}

/// The `IDecompiledData` half of `Fernflower`, which the struct context and its
/// units hold while the driver itself stays with the caller.
struct FernflowerDecompiledData {
    class_processor: OnceCell<ClassesProcessor>,
    has_converter: Cell<bool>,
}

impl Fernflower {
    /// Java: Fernflower(IResultSaver, Map<String, Object>, IFernflowerLogger).
    /// `custom_properties` overlay the defaults in order; three-letter keys are short names.
    /// The bundled plugins are loaded unless `jvm_plugins` is `0`.
    pub fn new(
        custom_properties: &[(String, String)],
        logger: Rc<dyn IFernflowerLogger>,
    ) -> Fernflower {
        let (plugins, custom_properties) = kuna_plugins::take_switch(custom_properties);
        Fernflower::new_with_plugin_sources(
            &custom_properties,
            logger,
            kuna_plugins::plugin_sources(plugins),
        )
    }

    /// [`Fernflower::new`] with `plugin_sources` as `PluginSources.PLUGIN_SOURCES`
    /// (`jvm_plugins` is ignored).
    pub fn new_with_plugin_sources(
        custom_properties: &[(String, String)],
        logger: Rc<dyn IFernflowerLogger>,
        plugin_sources: Vec<Rc<dyn PluginSource>>,
    ) -> Fernflower {
        let (_, custom_properties) = kuna_plugins::take_switch(custom_properties);
        let mut properties = IFernflowerPreferences::DEFAULTS();
        for (key, value) in &custom_properties {
            if key.jlen() == 3 {
                OptionParser::parse_short(&format!("-{key}={value}"), &mut properties);
            } else {
                properties.put(key.clone(), value.clone());
            }
        }

        let level = properties.get_cloned(&IFernflowerPreferences::LOG_LEVEL.to_string());
        if let Some(level) = level {
            if let Some(severity) = Severity::value_of(&level.to_uppercase()) {
                logger.set_severity(severity);
            }
        }

        let data = Rc::new(FernflowerDecompiledData {
            class_processor: OnceCell::new(),
            has_converter: Cell::new(false),
        });
        let struct_context = StructContext::new(data.clone());
        let class_processor = ClassesProcessor::new(struct_context);
        let _ = data.class_processor.set(class_processor);

        let mut interceptor = None;
        let helper;
        let converter;
        if properties
            .get(&IFernflowerPreferences::RENAME_ENTITIES.to_string())
            .map(String::as_str)
            == Some("1")
        {
            let user_renamer =
                properties.get_cloned(&IFernflowerPreferences::USER_RENAMER_CLASS.to_string());
            let renamer = Self::load_helper(user_renamer.as_deref(), &*logger);
            let pool_interceptor = PoolInterceptor::new();
            interceptor = Some(pool_interceptor);
            converter = Some(IdentifierConverter::new(
                struct_context,
                renamer.clone(),
                pool_interceptor,
            ));
            helper = Some(renamer);
        } else {
            helper = None;
            converter = None;
        }
        data.has_converter.set(converter.is_some());

        let context = DecompilerContext::new(
            properties,
            logger.clone(),
            struct_context,
            class_processor,
            interceptor,
        );
        DecompilerContext::set_current_context(Some(context.clone()));

        let plugins = struct_context.get_plugin_context();
        plugins.set_plugin_sources(plugin_sources);
        let plugin_count = plugins.find_plugins();

        logger.write_message(&format!("Loaded {plugin_count} plugins"), Severity::Info);

        plugins.initialize();

        let renamer = plugins
            .get_variable_renamer()
            .unwrap_or_else(|| Rc::new(IdentityRenamerFactory));

        context.with_mut(|d| d.renamer_factory = Some(renamer));

        logger.write_message(
            &format!(
                "JVM info: {} - {} - {}",
                "missing vendor", "missing java version", "missing jvm version"
            ),
            Severity::Info,
        );

        if DecompilerContext::get_option(IFernflowerPreferences::INCLUDE_ENTIRE_CLASSPATH) {
            JdkRuntime::add_runtime(struct_context, JRT_CURRENT);
        } else {
            let java_runtime =
                DecompilerContext::get_property(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME)
                    .expect("getProperty(INCLUDE_JAVA_RUNTIME)");
            if !java_runtime.is_empty() {
                let current = equals_ignore_case(&java_runtime, JRT_CURRENT)
                    || equals_ignore_case(&java_runtime, "1");
                if current || !equals_ignore_case(&java_runtime, "0") {
                    JdkRuntime::add_runtime(struct_context, &java_runtime);
                }
            }
        }

        Fernflower {
            struct_context,
            class_processor,
            helper,
            converter,
            data,
            loaded: false,
        }
    }

    /// No renamer class can be loaded by name, so a named one is reported as
    /// missing and the default renamer is used, as on a failed load in Java.
    fn load_helper(
        class_name: Option<&str>,
        logger: &dyn IFernflowerLogger,
    ) -> Rc<dyn IIdentifierRenamer> {
        if let Some(class_name) = class_name {
            let exception = JavaException {
                class: "ClassNotFoundException",
                message: class_name.to_string(),
            };
            let caught = Caught {
                message: exception.to_string(),
                exception: Some(exception),
                location: String::new(),
            };
            logger.write_message_ex(
                &format!("Cannot load renamer '{class_name}'"),
                Severity::Warn,
                &caught,
            );
        }

        Rc::new(ConverterHelper::new())
    }

    /// Java: addSource(IContextSource)
    pub fn add_source(&self, source: Rc<dyn IContextSource>) {
        self.struct_context.add_space(source, true);
    }

    /// Java: addSource(File)
    pub fn add_source_file(&self, source: &Path) {
        self.struct_context.add_space_file(source, true);
    }

    /// Java: addLibrary(IContextSource)
    pub fn add_library(&self, library: Rc<dyn IContextSource>) {
        self.struct_context.add_space(library, false);
    }

    /// Java: addLibrary(File)
    pub fn add_library_file(&self, library: &Path) {
        self.struct_context.add_space_file(library, false);
    }

    /// Renames (when enabled), loads and decompiles every own class; each
    /// finished root class goes to `sink`, then the class scope is reset.
    pub fn decompile_context(&mut self, sink: &mut dyn FnMut(DecompiledClass)) {
        self.load();

        self.struct_context.save_context(sink);

        Self::end_class_scope();
        self.loaded = false;
    }

    /// Tells the plugins decompilation starts, renames (when enabled) and loads
    /// every own class into class nodes, once per run;
    /// [`Fernflower::decompile_context`] does it itself when needed.
    pub fn load(&mut self) {
        if self.loaded {
            return;
        }
        self.loaded = true;

        for plugin in self.struct_context.get_plugin_context().get_plugins() {
            plugin.before_decompile();
        }

        if let Some(converter) = &mut self.converter {
            converter.rename();
        }

        crate::kuna_driver::trace(|| "load start".into());
        self.class_processor.load_classes(self.helper.as_deref());
        crate::kuna_driver::trace(|| "load_classes done".into());
    }

    /// The root classes a full run delivers, in its order (own units in order,
    /// each in entry order); loads first. Reads only cached classes and class
    /// headers, so it leaves the run unchanged.
    pub fn root_classes(&mut self) -> Vec<String> {
        self.root_classes_in_units()
            .into_iter()
            .map(|(_, name)| name)
            .collect()
    }

    /// [`Fernflower::root_classes`] with the index of each root's own unit:
    /// a run pre-processes and writes one unit's roots before the next unit's.
    pub fn root_classes_in_units(&mut self) -> Vec<(usize, String)> {
        self.load();
        let mut result = Vec::new();
        let own_units = self
            .struct_context
            .get_units()
            .into_iter()
            .filter(|unit| unit.is_own());
        for (unit_index, unit) in own_units.enumerate() {
            let mut seen = BTreeSet::new();
            for entry in unit.get_class_names() {
                let cached = self
                    .struct_context
                    .with(|d| d.classes.get(&entry).copied().flatten());
                let name = match cached {
                    Some(cl) => Some(cl.qualified_name()),
                    None => unit
                        .get_class_bytes(&entry)
                        .and_then(|bytes| read_class_info(&bytes).ok())
                        .map(|info| info.this_class),
                };
                let Some(name) = name else {
                    continue;
                };
                let is_root = root_class_node(self.class_processor, &name)
                    .is_some_and(|node| class_node_type(node) == ClassNodeType::Root);
                if is_root && seen.insert(name.clone()) {
                    result.push((unit_index, name));
                }
            }
        }
        result
    }

    /// [`Fernflower::decompile_context`] for the roots of `selection` only: they
    /// are pre-processed and written in the full run's order; only the
    /// delivered ones reach `sink`. The whole class universe stays loaded.
    pub fn decompile_selected(
        &mut self,
        selection: &RootSelection,
        sink: &mut dyn FnMut(DecompiledClass),
    ) {
        self.load();

        self.struct_context
            .save_context_selected(sink, Some(selection));

        Self::end_class_scope();
        self.loaded = false;
    }

    /// `rt::reset_class_scope()` for the whole decompilation, keeping the
    /// current (root) context usable for a later run.
    fn end_class_scope() {
        let root_context = DecompilerContext::get_current_context();
        let counters = root_context.as_ref().map(DecompilerContext::counter_values);
        crate::rt::reset_class_scope();
        if let (Some(root_context), Some(counters)) = (root_context, counters) {
            root_context.survive_class_scope_reset(counters);
        }
    }

    pub fn add_whitelist(&self, prefix: &str) {
        self.class_processor.add_whitelist(prefix);
    }

    pub fn clear_context(&mut self) {
        self.struct_context.clear();
        DecompilerContext::set_current_context(None);
    }
}

impl IDecompiledData for Fernflower {
    fn get_class_entry_name(&self, cl: StructClass, entry_name: &str) -> Option<String> {
        self.data.get_class_entry_name(cl, entry_name)
    }

    fn process_class(&self, cl: StructClass) {
        self.data.process_class(cl)
    }

    fn get_class_content(&self, cl: StructClass) -> Option<String> {
        self.data.get_class_content(cl)
    }
}

impl FernflowerDecompiledData {
    fn class_processor(&self) -> ClassesProcessor {
        *self.class_processor.get().expect("classProcessor")
    }
}

impl IDecompiledData for FernflowerDecompiledData {
    fn get_class_entry_name(&self, cl: StructClass, entry_name: &str) -> Option<String> {
        let spec = PluginContext::get_current_context().get_language_spec(cl);
        let extension = spec.as_ref().map_or("java", |spec| spec.extension.as_str());

        let qualified_name = qualified_name_of(cl);
        let node = root_class_node(self.class_processor(), &qualified_name);
        match node {
            Some(node) if class_node_type(node) == ClassNodeType::Root => {}
            _ => return None,
        }
        if self.has_converter.get() {
            let simple_class_name =
                &qualified_name[qualified_name.rfind('/').map_or(0, |i| i + 1)..];
            let dir = &entry_name[..entry_name.rfind('/').map_or(0, |i| i + 1)];
            Some(format!("{dir}{simple_class_name}.{extension}"))
        } else {
            match entry_name.rfind(".class") {
                None => Some(format!("{entry_name}.{extension}")),
                Some(clazz_idx) => Some(format!("{}.{}", &entry_name[..clazz_idx], extension)),
            }
        }
    }

    fn process_class(&self, cl: StructClass) {
        self.class_processor().process_class(cl);
    }

    fn get_class_content(&self, cl: StructClass) -> Option<String> {
        let class_processor = self.class_processor();
        let buffer = Shared::new(TextBuffer::with_capacity(
            ClassesProcessor::AVERAGE_CLASS_SIZE,
        ));
        let result = try_catch(|| {
            buffer.borrow_mut().append(
                &DecompilerContext::get_property(IFernflowerPreferences::BANNER)
                    .expect("getProperty(BANNER)"),
            );
            class_processor.write_class(cl, &mut buffer.borrow_mut());

            if DecompilerContext::get_option(IFernflowerPreferences::DUMP_TEXT_TOKENS) {
                let dumped = buffer.clone();
                let factory: Rc<dyn Factory> = Rc::new(move |next| {
                    Box::new(TextTokenDumpVisitor::new(next, dumped.clone()))
                        as Box<dyn TextTokenVisitor>
                });
                let mut visitor = <dyn TextTokenVisitor>::create_visitor_with(factory);
                TextBuffer::visit_tokens_shared(&buffer, Some(&mut *visitor));
            } else {
                let mut visitor = <dyn TextTokenVisitor>::create_visitor();
                buffer.borrow().visit_tokens(Some(&mut *visitor));
            }

            let res = buffer.borrow().convert_to_string_and_allow_data_discard();
            crate::kuna_spans::finish(&buffer.borrow(), &res);
            Some(res)
        });
        match result {
            Ok(res) => res,
            Err(t) => {
                crate::kuna_spans::fail(&qualified_name_of(cl), &t);
                DecompilerContext::get_logger().write_message_throwable(
                    &format!(
                        "Class {} couldn't be fully decompiled.",
                        qualified_name_of(cl)
                    ),
                    &t,
                );
                if DecompilerContext::get_option(IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR) {
                    let mut lines = vec![
                        "/*".to_string(),
                        "$VF: Unable to decompile class".to_string(),
                    ];
                    lines.extend(ClassWriter::get_error_comment());
                    ClassWriter::collect_error_lines(&t, &mut lines);
                    lines.push("*/".to_string());
                    Some(lines.join(DecompilerContext::get_new_line_separator()))
                } else {
                    None
                }
            }
        }
    }
}

/// `classProcessor.getMapRootClasses().get(name)`.
fn root_class_node(class_processor: ClassesProcessor, name: &str) -> Option<ClassNode> {
    class_processor
        .get_map_root_classes()
        .borrow()
        .get(&name.to_string())
        .copied()
}

/// `node.type`.
fn class_node_type(node: ClassNode) -> ClassNodeType {
    node.with(|d| d.type_)
}

/// `String.equalsIgnoreCase`.
fn equals_ignore_case(a: &str, b: &str) -> bool {
    fn simple(mut mapped: impl Iterator<Item = char>, c: char) -> char {
        match (mapped.next(), mapped.next()) {
            (Some(m), None) => m,
            _ => c,
        }
    }
    let upper = |c: char| simple(c.to_uppercase(), c);
    let lower = |c: char| simple(c.to_lowercase(), c);
    a.jlen() == b.jlen()
        && a.chars().zip(b.chars()).all(|(c1, c2)| {
            if c1 == c2 {
                return true;
            }
            let (u1, u2) = (upper(c1), upper(c2));
            u1 == u2 || lower(u1) == lower(u2)
        })
}

/// The part of `main/decompiler/OptionParser.java` the constructor uses to
/// expand three-letter option names.
struct OptionParser;

impl OptionParser {
    const CUSTOM_CHECKS: [&'static str; 6] = [
        "-e=",
        "--add-external=",
        "-only=",
        "--only=",
        "-s",
        "--silent",
    ];

    fn parse(arg: &str, options: &mut JHashMap<String, String>) -> bool {
        for check in Self::CUSTOM_CHECKS {
            if arg.starts_with(check) {
                return false;
            }
        }

        if !arg.starts_with("--") {
            Self::parse_short(arg, options);
            return true;
        }

        let arg = &arg[2..];
        match arg.find('=') {
            None => {
                if let Some(rest) = arg.strip_prefix("no-") {
                    options.put(rest.to_string(), "0".to_string());
                } else {
                    options.put(arg.to_string(), "1".to_string());
                }
            }
            Some(index) => {
                let key = &arg[..index];
                let mut value = &arg[index + 1..];
                if equals_ignore_case(value, "true") {
                    value = "1";
                } else if equals_ignore_case(value, "false") {
                    value = "0";
                }

                options.put(key.to_string(), value.to_string());
            }
        }
        true
    }

    fn parse_short(arg: &str, options: &mut JHashMap<String, String>) {
        let short_name = arg.jsubstring(1, 4);
        let value = arg.jsubstring_from(5);
        let long_name = Self::short_to_long_name(&short_name).unwrap_or(short_name);
        Self::parse(&format!("--{long_name}={value}"), options);
    }

    /// `SHORT_TO_LONG_NAME_MAP.get(shortName)`.
    fn short_to_long_name(short_name: &str) -> Option<String> {
        match short_name {
            "jvn" => return Some("jad-style-variable-naming".to_string()),
            "jpr" => return Some("jad-style-parameter-naming".to_string()),
            _ => {}
        }
        IFernflowerPreferences::all_options()
            .iter()
            .rev()
            .find(|o| o.short_name == Some(short_name))
            .map(|o| o.key.to_string())
    }
}
