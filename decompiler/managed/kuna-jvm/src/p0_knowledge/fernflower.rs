//! Port of `org/jetbrains/java/decompiler/main/Fernflower.java`, adapted to
//! kuna: finished classes are handed to a callback instead of an
//! `IResultSaver`; Vineflower's bundled Kotlin and IdeaNotNull plugins are part
//! of the pipeline (the Kotlin `LanguageSpec` is installed here, IdeaNotNull
//! runs in the method schedule), so nothing is loaded; renamer class loading
//! and the class path (`include-classpath`) have no Rust counterpart.
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

use kuna_managed::{MemberSpan, TypeSpan};

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
use crate::jdk_runtime::JdkRuntime;
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_plugin::KotlinPlugin;
use crate::kuna_partition::RootSelection;
use crate::language_spec::LanguageSpec;
use crate::pool_interceptor::PoolInterceptor;
use crate::rt::{try_catch, Caught, JHashMap, JavaException, JavaStr};
use crate::struct_class::StructClass;
use crate::struct_context::{qualified_name_of, StructContext};
use crate::text_buffer::TextBuffer;

/// One decompiled root class, as the Java's `IOutputSink.acceptClass` receives it,
/// plus where its classes and methods lie in `content` (kuna).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DecompiledClass {
    pub qualified_name: String,
    /// The output entry, e.g. `a/b/C.java`.
    pub path: String,
    pub content: String,
    /// Why the class could not be written normally (`content` is then the
    /// error comment Vineflower writes in its place).
    pub error: Option<String>,
    /// Every class written into `content`, in writing order.
    pub classes: Vec<TypeSpan>,
    /// Every method written into `content`, in writing order.
    pub methods: Vec<MemberSpan>,
}

/// `JrtFinder.CURRENT`.
const JRT_CURRENT: &str = "current";

/// The note under a failure comment (kuna; Vineflower names its own tracker).
pub const KUNA_ERROR_MESSAGE: &str = "Please report this to the kuna issue tracker, at https://github.com/Noelo-Lab/kuna/issues with a copy of the class file (if you have the rights to distribute it!)";

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
    pub fn new(
        custom_properties: &[(String, String)],
        logger: Rc<dyn IFernflowerLogger>,
    ) -> Fernflower {
        let mut properties = IFernflowerPreferences::DEFAULTS();
        for (key, value) in custom_properties {
            if key.jlen() == 3 {
                OptionParser::parse_short(&format!("-{key}={value}"), &mut properties);
            } else {
                properties.put(key.clone(), value.clone());
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

        context.with_mut(|d| {
            for (key, value) in KotlinOptions::DEFAULTS {
                d.properties.put_if_absent(key.to_string(), value.to_string());
            }
        });
        let kotlin = Rc::new(KotlinPlugin::language_spec());
        struct_context.with_mut(|d| d.language_spec = Some(kotlin));

        logger.write_message(
            &format!(
                "JVM info: {} - {} - {}",
                "missing vendor", "missing java version", "missing jvm version"
            ),
            Severity::Info,
        );

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

        self.end_class_scope();
        self.loaded = false;
    }

    /// Renames (when enabled, once per engine, as Vineflower renames once
    /// per instance) and loads every
    /// own class into class nodes, once per run;
    /// [`Fernflower::decompile_context`] does it itself when needed.
    pub fn load(&mut self) {
        if self.loaded {
            return;
        }

        if let Some(mut converter) = self.converter.take() {
            converter.rename();
        }

        crate::kuna_driver::trace(|| "load start".into());
        self.class_processor.load_classes(self.helper.as_deref());
        crate::kuna_driver::trace(|| "load_classes done".into());
        self.loaded = true;
    }

    /// [`Fernflower::root_classes`] with the index of each root's own unit:
    /// a run pre-processes and writes one unit's roots before the next unit's.
    pub fn root_classes_in_units(&mut self) -> Vec<(usize, String)> {
        self.load();
        let mut result = Vec::new();
        let interceptor = DecompilerContext::get_pool_interceptor();
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
                        .map(|info| renamed_class(interceptor, info.this_class)),
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

        crate::kt_attrs::KtAttrs::undo_marks();
        self.end_class_scope();
        self.loaded = false;
    }

    /// The name the run gives the class file named `original`: its new name
    /// when the renamer renamed it.
    pub fn renamed_class(&self, original: &str) -> String {
        renamed_class(DecompilerContext::get_pool_interceptor(), original.to_string())
    }

    /// `rt::reset_class_scope()` for the whole decompilation, keeping the
    /// current (root) context usable for a later run.
    fn end_class_scope(&self) {
        self.class_processor.end_class_scope();
        let root_context = DecompilerContext::get_current_context();
        let counters = root_context.as_ref().map(DecompilerContext::counter_values);
        crate::rt::reset_class_scope();
        if let (Some(root_context), Some(counters)) = (root_context, counters) {
            root_context.survive_class_scope_reset(counters);
        }
    }

    pub fn clear_context(&mut self) {
        self.struct_context.clear();
        DecompilerContext::set_current_context(None);
    }
}

/// `name` after renaming: the interceptor's new name for a renamed class.
fn renamed_class(interceptor: Option<PoolInterceptor>, name: String) -> String {
    interceptor.and_then(|i| i.get_name(&name)).unwrap_or(name)
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
        let spec = LanguageSpec::of(cl);
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
        let mut buffer = TextBuffer::with_capacity(ClassesProcessor::AVERAGE_CLASS_SIZE);
        let result = try_catch(|| {
            class_processor.write_class(cl, &mut buffer);
            let res = buffer.convert_to_string_and_allow_data_discard();
            crate::kuna_spans::finish(&buffer, &res);
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
                let mut lines = vec![
                    "/*".to_string(),
                    "$VF: Unable to decompile class".to_string(),
                ];
                lines.extend(ClassWriter::get_error_comment());
                ClassWriter::collect_error_lines(&t, &mut lines);
                lines.push("*/".to_string());
                Some(lines.join(DecompilerContext::get_new_line_separator()))
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

/// The option a custom property `(key, value)` sets once the constructor has
/// read it: a three-letter name becomes its long name (`mpm` is
/// `max-time-per-method`), with `true`/`false` read as `1`/`0`.
pub fn canonical_option(key: &str, value: &str) -> (String, String) {
    if key.jlen() != 3 {
        return (key.to_string(), value.to_string());
    }
    let mut parsed = JHashMap::new();
    OptionParser::parse_short(&format!("-{key}={value}"), &mut parsed);
    let canonical = parsed.iter().next().map(|(k, v)| (k.clone(), v.clone()));
    canonical.unwrap_or_else(|| (key.to_string(), value.to_string()))
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
        IFernflowerPreferences::all_options()
            .iter()
            .rev()
            .find(|o| o.short_name == Some(short_name))
            .map(|o| o.key.to_string())
    }
}

impl DecompiledClass {
    /// The language the class was written in: `kotlin` when the Kotlin
    /// pipeline claimed it (its `path` then ends in `.kt`), else `java`.
    pub fn language(&self) -> &'static str {
        match self.path.rsplit_once('.') {
            Some((_, "kt")) => "kotlin",
            _ => "java",
        }
    }
}
