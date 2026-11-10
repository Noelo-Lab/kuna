//! The language-plugin hooks (KT0), driven by a toy language plugin over
//! `data/kt0` (`p/HelloToy` with a member class, `p/Plain`, `p/SoloToy`; one
//! unit, in that entry order): its chooser claims classes whose name ends in
//! `Toy`, its pipeline runs part of the Java one, and its writer marks the
//! output and drops semicolons. Also: plugin option defaults, the bundled
//! plugin switch, and the routing of Kotlin-flagged exprents, statements and
//! import collectors to the Kotlin classes.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_jvm::class_pass::ClassPass;
use kuna_jvm::class_pass_location::ClassPassLocation;
use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::class_writer::ClassWriter;
use kuna_jvm::classes_processor::ClassNode;
use kuna_jvm::clear_struct_helper::ClearStructHelper;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::context_source::{Entries, Entry, IContextSource};
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::do_statement::DoStatement;
use kuna_jvm::dom_helper::DomHelper;
use kuna_jvm::exit_helper::ExitHelper;
use kuna_jvm::expr_processor::ExprProcessor;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::graph_parser::GraphParser;
use kuna_jvm::i_fernflower_logger::CollectingLogger;
use kuna_jvm::i_fernflower_preferences::{IFernflowerPreferences, OptionType, PreferenceOption};
use kuna_jvm::import_collector::{ImportCollector, ImportCollectorData};
use kuna_jvm::java_pass_location::JavaPassLocation;
use kuna_jvm::java_pass_registrar::JavaPassRegistrar;
use kuna_jvm::k_dispatch::{KExprentExt, KExprentType};
use kuna_jvm::k_element::KElement;
use kuna_jvm::k_function_exprent::{KFunctionExprentData, KFunctionType};
use kuna_jvm::k_hidden_element::KHiddenElement;
use kuna_jvm::k_property::PropertyMethod;
use kuna_jvm::k_type::KType;
use kuna_jvm::k_var_exprent::{DeclarationType, KVarExprentData};
use kuna_jvm::key::Key;
use kuna_jvm::kt_attrs::KtAttrs;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_plugins::{self, JVM_PLUGINS};
use kuna_jvm::label_helper::LabelHelper;
use kuna_jvm::language_spec::LanguageSpec;
use kuna_jvm::main_pass_builder::MainPassBuilder;
use kuna_jvm::named_pass::NamedPass;
use kuna_jvm::pair::Pair;
use kuna_jvm::pass::Pass;
use kuna_jvm::pass_context::PassContext;
use kuna_jvm::plugin::Plugin;
use kuna_jvm::plugin_context::PluginContext;
use kuna_jvm::plugin_options::{AddDefaults, DefaultsConsumer, PluginOptions};
use kuna_jvm::plugin_source::PluginSource;
use kuna_jvm::rt::{try_catch, JHashMap, JHashSet};
use kuna_jvm::sequence_helper::SequenceHelper;
use kuna_jvm::stack_vars_processor::StackVarsProcessor;
use kuna_jvm::statement_writer::StatementWriter;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_field::StructField;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;
use kuna_jvm::wrapped_pass::WrappedPass;

use crate::common;

type Log = Rc<RefCell<Vec<String>>>;

const ENTRIES: [&str; 4] = ["p/HelloToy", "p/HelloToy$Inner", "p/Plain", "p/SoloToy"];

fn push(log: &Log, event: String) {
    log.borrow_mut().push(event);
}

fn class_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt0/classes")
}

/// The fixture classes as one unit, in [`ENTRIES`] order.
struct ToyClasses;

impl IContextSource for ToyClasses {
    fn get_name(&self) -> String {
        "kt0".to_string()
    }

    fn get_entries(&self) -> Entries {
        let classes = ENTRIES.iter().map(|name| Entry::at_base(name)).collect();
        Entries::new(classes, Vec::new(), Vec::new())
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        std::fs::read(class_dir().join(resource)).ok()
    }
}

fn method_name(ctx: &PassContext) -> String {
    format!(
        "{}.{}",
        ctx.get_enclosing_class().qualified_name(),
        ctx.get_method().get_name()
    )
}

fn class_name(node: ClassNode) -> String {
    node.with(|d| d.class_struct).qualified_name()
}

struct ToyWriter {
    log: Log,
}

impl StatementWriter for ToyWriter {
    fn write_class_header(
        &self,
        cl: StructClass,
        buffer: &mut TextBuffer,
        import_collector: ImportCollector,
    ) {
        push(
            &self.log,
            format!("writeClassHeader {}", cl.qualified_name()),
        );
        buffer
            .append("// toy header ")
            .append(&cl.qualified_name())
            .append_line_separator();
        import_collector.write_imports(buffer, true);
    }

    fn write_class(&self, node: ClassNode, buffer: &mut TextBuffer, indent: i32) {
        push(&self.log, format!("writeClass {}", class_name(node)));
        buffer
            .append("// toy class ")
            .append(&class_name(node))
            .append_line_separator();
        StatementWriter::write_class(&ClassWriter::new(), node, buffer, indent);
    }

    fn write_field(
        &self,
        wrapper: ClassWrapper,
        cl: StructClass,
        fd: StructField,
        buffer: &mut TextBuffer,
        indent: i32,
    ) {
        StatementWriter::write_field(&ClassWriter::new(), wrapper, cl, fd, buffer, indent);
    }

    fn write_method(
        &self,
        node: ClassNode,
        mt: StructMethod,
        method_index: i32,
        buffer: &mut TextBuffer,
        indent: i32,
    ) -> bool {
        StatementWriter::write_method(&ClassWriter::new(), node, mt, method_index, buffer, indent)
    }

    fn ends_with_semicolon(&self, _expr: Exprent) -> bool {
        push(&self.log, "endsWithSemicolon".to_string());
        false
    }
}

static TOY_OPTIONS: &[PreferenceOption] = &[PreferenceOption {
    field: "TOY_FLAG",
    key: "toy-flag",
    short_name: None,
    name: Some("Toy flag"),
    description: Some("A toy option."),
    type_: Some(OptionType::Boolean),
    dynamic_default_value: None,
}];

struct Toy {
    log: Log,
}

impl Toy {
    /// Enough of the Java schedule for straight-line methods.
    fn main_pass(&self) -> Rc<dyn Pass> {
        let log = self.log.clone();
        let mut builder = MainPassBuilder::new();
        builder.add_pass(
            "Record",
            Rc::new(move |ctx: &mut PassContext| {
                push(
                    &log,
                    format!(
                        "pass {} root={}",
                        method_name(ctx),
                        ctx.get_root().is_some()
                    ),
                );
                false
            }),
        );
        builder.add_pass(
            "Pipeline",
            WrappedPass::of(|ctx: &mut PassContext| {
                let root = ctx.get_root().unwrap();
                let (mt, cl, var_proc) = (
                    ctx.get_method(),
                    ctx.get_enclosing_class(),
                    ctx.get_var_proc(),
                );
                let md = ctx.get_method_descriptor();
                ClearStructHelper::clear_statements(root);
                ExprProcessor::new(&md, var_proc).process_statement(root, cl);
                SequenceHelper::condense_sequences(root.0);
                StackVarsProcessor::simplify_stack_vars(root, mt, cl);
                var_proc.set_var_versions(root);
                LabelHelper::identify_labels(root);
                ExitHelper::remove_redundant_returns(root);
                var_proc.set_var_definitions(root);
            }),
        );
        builder.build()
    }
}

impl Plugin for Toy {
    fn id(&self) -> String {
        "Toy".to_string()
    }

    fn description(&self) -> String {
        "A toy JVM language.".to_string()
    }

    fn initialize(&self) {
        push(&self.log, "initialize".to_string());
    }

    fn before_decompile(&self) {
        push(&self.log, "beforeDecompile".to_string());
    }

    fn close(&self) {
        push(&self.log, "close".to_string());
    }

    fn register_java_passes(&self, registrar: &mut JavaPassRegistrar) {
        push(&self.log, "registerJavaPasses".to_string());
        let log = self.log.clone();
        registrar.register(
            JavaPassLocation::AtEnd,
            NamedPass::of(
                "ToyAtEnd",
                Rc::new(move |ctx: &mut PassContext| {
                    push(&log, format!("javaPass {}", method_name(ctx)));
                    false
                }),
            ),
        );
        for location in [
            ClassPassLocation::BeforeProcessing,
            ClassPassLocation::AfterProcessing,
            ClassPassLocation::BeforeWriting,
        ] {
            let log = self.log.clone();
            let pass: Rc<dyn ClassPass> = Rc::new(move |node: ClassNode| {
                push(&log, format!("classPass {location:?} {}", class_name(node)));
                false
            });
            registrar.register_class_pass(location, pass);
        }
    }

    fn get_language_spec(&self) -> Option<LanguageSpec> {
        push(&self.log, "getLanguageSpec".to_string());
        let (l1, l2, l3, l4) = (
            self.log.clone(),
            self.log.clone(),
            self.log.clone(),
            self.log.clone(),
        );
        Some(LanguageSpec::new(
            "toy",
            Rc::new(move |cl: StructClass| {
                push(&l1, format!("isLanguage {}", cl.qualified_name()));
                cl.qualified_name().ends_with("Toy")
            }),
            Rc::new(move |graph, mt: StructMethod| {
                push(&l2, format!("createStatement {}", mt.get_name()));
                GraphParser::create_statement(&DomHelper, graph, mt)
            }),
            Rc::new(ToyWriter {
                log: self.log.clone(),
            }),
            self.main_pass(),
            Rc::new(move |ctx: &mut PassContext| {
                push(
                    &l3,
                    format!(
                        "cfgPass {} root={} graph={}",
                        method_name(ctx),
                        ctx.get_root().is_some(),
                        ctx.get_graph().is_some()
                    ),
                );
                false
            }),
            Rc::new(move |root: ClassNode| {
                push(&l4, format!("rootProcessor {}", class_name(root)));
                true
            }),
            "toy",
        ))
    }

    fn get_plugin_options(&self) -> Option<Rc<dyn PluginOptions>> {
        push(&self.log, "getPluginOptions".to_string());
        Some(Rc::new(|| {
            let defaults: DefaultsConsumer = Rc::new(|add: &mut dyn AddDefaults| {
                add.add_default("toy-flag", "1");
                add.add_default(IFernflowerPreferences::INDENT_STRING, "XX");
                add.add_default("toy-user", "default");
            });
            Pair::of(TOY_OPTIONS, defaults)
        }))
    }

    fn get_renaming_factory(
        &self,
    ) -> Option<Rc<dyn kuna_jvm::i_variable_naming_factory::IVariableNamingFactory>> {
        push(&self.log, "getRenamingFactory".to_string());
        None
    }
}

struct ToySource {
    log: Log,
}

impl PluginSource for ToySource {
    fn find_plugins(&self) -> Vec<Rc<dyn Plugin>> {
        vec![Rc::new(Toy {
            log: self.log.clone(),
        })]
    }
}

struct Run {
    units: Vec<DecompiledClass>,
    log: Vec<String>,
    properties: Vec<Option<String>>,
    plugin_count: usize,
    source_name: Option<String>,
}

fn run(toy: bool, extra: &[(&str, &str)]) -> Run {
    let mut options: Vec<(String, String)> = vec![(
        IFernflowerPreferences::INCLUDE_JAVA_RUNTIME.to_string(),
        "0".to_string(),
    )];
    options.extend(extra.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    with_engine_stack(move || {
        reset_engine_thread();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sources: Vec<Rc<dyn PluginSource>> = if toy {
            vec![Rc::new(ToySource { log: log.clone() })]
        } else {
            Vec::new()
        };
        let mut fernflower = Fernflower::new_with_plugin_sources(
            &options,
            Rc::new(CollectingLogger::new()),
            sources,
        );
        let properties = [
            "toy-flag",
            IFernflowerPreferences::INDENT_STRING,
            "toy-user",
            JVM_PLUGINS,
        ]
        .map(DecompilerContext::get_property)
        .to_vec();
        let plugins = PluginContext::get_current_context().get_plugins();
        let source_name = plugins.first().and_then(|p| {
            PluginContext::get_current_context()
                .get_source(p)
                .map(|s| s.get_simple_name())
        });
        fernflower.add_source(Rc::new(ToyClasses));
        let mut units = Vec::new();
        fernflower.decompile_context(&mut |unit| units.push(unit));
        fernflower.clear_context();
        reset_engine_thread();
        let log = log.borrow().clone();
        Run {
            units,
            log,
            properties,
            plugin_count: plugins.len(),
            source_name,
        }
    })
}

fn unit<'a>(run: &'a Run, name: &str) -> &'a DecompiledClass {
    run.units
        .iter()
        .find(|u| u.qualified_name == name)
        .unwrap_or_else(|| panic!("no unit {name}"))
}

fn method_events(class: &str, methods: &[&str]) -> Vec<String> {
    let mut events = Vec::new();
    for method in methods {
        events.push(format!("cfgPass {class}.{method} root=false graph=true"));
        events.push(format!("createStatement {method}"));
        events.push(format!("pass {class}.{method} root=true"));
    }
    events
}

#[test]
fn toy_language_hooks_fire_in_vineflower_order() {
    let run = run(true, &[]);
    assert_eq!(run.plugin_count, 1);
    assert_eq!(run.source_name.as_deref(), Some("ToySource"));

    let paths: Vec<&str> = run.units.iter().map(|u| u.path.as_str()).collect();
    assert_eq!(paths, ["p/HelloToy.toy", "p/Plain.java", "p/SoloToy.toy"]);

    let chooser_calls: Vec<&String> = run
        .log
        .iter()
        .take_while(|e| !e.starts_with("classPass"))
        .filter(|e| e.starts_with("isLanguage"))
        .collect();
    assert_eq!(
        chooser_calls,
        ENTRIES
            .map(|e| format!("isLanguage {e}"))
            .iter()
            .collect::<Vec<_>>(),
        "every entry's spec is looked up, in entry order, before any class is processed"
    );

    let events: Vec<&str> = run
        .log
        .iter()
        .map(String::as_str)
        .filter(|e| !e.starts_with("isLanguage") && *e != "endsWithSemicolon")
        .collect();
    let mut expected: Vec<String> = [
        "initialize",
        "registerJavaPasses",
        "getLanguageSpec",
        "getPluginOptions",
        "getRenamingFactory",
        "beforeDecompile",
        "classPass BeforeProcessing p/HelloToy",
    ]
    .map(String::from)
    .to_vec();
    expected.extend(method_events("p/HelloToy", &["<init>", "twice"]));
    expected.extend(method_events("p/HelloToy$Inner", &["<init>", "inc"]));
    expected.extend(
        [
            "rootProcessor p/HelloToy",
            "classPass AfterProcessing p/HelloToy",
            "classPass BeforeProcessing p/Plain",
            "javaPass p/Plain.<init>",
            "javaPass p/Plain.sum",
            "classPass AfterProcessing p/Plain",
            "classPass BeforeProcessing p/SoloToy",
        ]
        .map(String::from),
    );
    expected.extend(method_events("p/SoloToy", &["<init>", "answer"]));
    expected.extend(
        [
            "rootProcessor p/SoloToy",
            "classPass AfterProcessing p/SoloToy",
            "classPass BeforeWriting p/HelloToy",
            "writeClass p/HelloToy",
            "writeClassHeader p/HelloToy",
            "classPass BeforeWriting p/Plain",
            "classPass BeforeWriting p/SoloToy",
            "writeClass p/SoloToy",
            "writeClassHeader p/SoloToy",
            "close",
        ]
        .map(String::from),
    );
    assert_eq!(events, expected);
}

#[test]
fn claimed_classes_are_written_by_the_spec_and_others_stay_java() {
    let toy = run(true, &[]);
    let java = run(false, &[]);

    let plain = unit(&toy, "p/Plain");
    let plain_java = unit(&java, "p/Plain");
    assert_eq!(plain.path, plain_java.path);
    assert_eq!(plain.content, plain_java.content);
    assert!(plain.content.contains("return c * c;"), "{}", plain.content);

    let java_paths: Vec<&str> = java.units.iter().map(|u| u.path.as_str()).collect();
    assert_eq!(
        java_paths,
        ["p/HelloToy.java", "p/Plain.java", "p/SoloToy.java"]
    );
    assert!(java.log.is_empty());

    let solo = &unit(&toy, "p/SoloToy").content;
    assert!(
        solo.starts_with("// toy header p/SoloToy\n// toy class p/SoloToy\npublic class SoloToy {"),
        "{solo}"
    );
    assert!(solo.contains("int y = x + 40\n"), "{solo}");
    assert!(solo.contains("return y + 2\n"), "{solo}");
    assert!(!solo.contains(';'), "{solo}");
    assert!(toy.log.iter().any(|e| e == "endsWithSemicolon"));

    let hello = &unit(&toy, "p/HelloToy").content;
    assert!(
        hello.starts_with(
            "// toy header p/HelloToy\n// toy class p/HelloToy\npublic class HelloToy {"
        ),
        "{hello}"
    );
    assert!(hello.contains("public static class Inner {"), "{hello}");
    // listToJava asks the chooser about the current class, the last one whose
    // wrapper was built (the member class, which the chooser does not claim).
    assert!(hello.contains("return x * 2;"), "{hello}");
    assert!(hello.contains("return x + 1;"), "{hello}");
}

#[test]
fn plugin_option_defaults_never_override_a_value() {
    let defaults = IFernflowerPreferences::DEFAULTS();
    let indent = defaults
        .get_cloned(&IFernflowerPreferences::INDENT_STRING.to_string())
        .unwrap();

    let toy = run(true, &[("toy-user", "user")]);
    assert_eq!(
        toy.properties,
        [
            Some("1".to_string()),
            Some(indent.clone()),
            Some("user".to_string()),
            None
        ]
    );

    let java = run(false, &[]);
    assert_eq!(java.properties, [None, Some(indent), None, None]);
}

#[test]
fn the_bundled_plugin_switch() {
    let pairs = |p: &[(&str, &str)]| -> Vec<(String, String)> {
        p.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    };
    let (on, rest) =
        kuna_plugins::take_switch(&pairs(&[("a", "1"), (JVM_PLUGINS, "0"), ("b", "2")]));
    assert!(!on);
    assert_eq!(rest, pairs(&[("a", "1"), ("b", "2")]));
    assert!(kuna_plugins::take_switch(&pairs(&[("a", "1")])).0);
    assert!(kuna_plugins::take_switch(&pairs(&[(JVM_PLUGINS, "off"), (JVM_PLUGINS, "on")])).0);
    for off in ["0", "off", "OFF", "false"] {
        assert!(!kuna_plugins::switch_value(off), "{off}");
    }
    assert_eq!(kuna_plugins::plugin_sources(false).len(), 0);
    let sources = kuna_plugins::plugin_sources(true);
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].get_simple_name(), "ServicePluginSource");
    assert_eq!(
        sources[0].find_plugins().len(),
        kuna_plugins::bundled_plugins().len()
    );

    for value in ["1", "0"] {
        let run = with_engine_stack(move || {
            reset_engine_thread();
            let logger = Rc::new(CollectingLogger::new());
            let mut fernflower = Fernflower::new(
                &[
                    (
                        IFernflowerPreferences::INCLUDE_JAVA_RUNTIME.to_string(),
                        "0".to_string(),
                    ),
                    (JVM_PLUGINS.to_string(), value.to_string()),
                ],
                logger.clone(),
            );
            let property = DecompilerContext::get_property(JVM_PLUGINS);
            let plugins = PluginContext::get_current_context().get_plugins().len();
            fernflower.clear_context();
            reset_engine_thread();
            (property, plugins, logger.messages())
        });
        let expected = if value == "1" {
            kuna_plugins::bundled_plugins().len()
        } else {
            0
        };
        assert_eq!(
            run.0, None,
            "jvm_plugins never reaches the engine's properties"
        );
        assert_eq!(run.1, expected);
        assert!(
            run.2.contains(&format!("Loaded {expected} plugins")),
            "{:?}",
            run.2
        );
    }
}

fn collector() -> ImportCollector {
    let mut simple_names = JHashMap::new();
    simple_names.put("List".to_string(), "java.util".to_string());
    ImportCollector::alloc(ImportCollectorData {
        map_simple_names: simple_names,
        set_not_imported_names: JHashSet::new(),
        set_field_names: JHashSet::new(),
        map_inner_class_names: JHashMap::new(),
        current_package_slash: "p/".to_string(),
        current_package_point: "p".to_string(),
        write_locked: false,
        kotlin: false,
        kuna_inner_class_names: JHashMap::new(),
    })
}

fn kotlin_function(func_type: FunctionType, operands: Vec<Exprent>) -> FunctionExprent {
    let func = FunctionExprent::new(func_type, operands, None);
    func.0
        .set_kt(Some(KExprentExt::Function(KFunctionExprentData {
            k_type: KFunctionType::Equals3,
            identity: false,
            nullable_cast: false,
        })));
    func
}

fn int(value: i32) -> Exprent {
    ConstExprent::new_int(value, false, None).0
}

#[test]
fn kotlin_exprents_dispatch_to_their_kotlin_class() {
    let _guard = common::install_default_context();

    let func = kotlin_function(FunctionType::Add, vec![int(1), int(2)]);
    let e = func.0;
    assert!(e.is_kotlin() && e.is_function() && e.is_k_function());
    assert!(!e.is_k_var());
    assert_eq!(e.get_kotlin_type(), Some(KExprentType::Function));
    assert_eq!(e.as_k_function().kd(|d| d.k_type), KFunctionType::Equals3);
    assert_eq!(
        e.to_java(0).convert_to_string_and_allow_data_discard(),
        "1 + 2"
    );
    assert_eq!(e.get_precedence(), 6, "KFunctionExprent.getPrecedence");
    assert!(e.get_expr_type().equals(&VarType::VARTYPE_BOOLEAN()));
    assert!(e
        .get_expr_type_opt()
        .is_some_and(|t| t.equals(&VarType::VARTYPE_BOOLEAN())));
    let bounds = e.check_expr_type_bounds().expect("FunctionExprent bounds");
    assert_eq!(bounds.get_lower_bounds().len(), 2);
    let copy = e.copy();
    assert!(copy != e && copy.is_k_function());
    assert_eq!(
        copy.as_k_function().kd(|d| d.k_type),
        KFunctionType::Equals3
    );
    assert_eq!(copy.as_function().get_func_type(), FunctionType::Add);
    assert!(copy
        .as_function()
        .get_lst_operands()
        .iter()
        .all(|o| o.is_k_const()));
    // Java-virtual calls inside the FunctionExprent bodies reach the override.
    assert!(e
        .get_inferred_expr_type(None)
        .equals(&VarType::VARTYPE_BOOLEAN()));
    let shift = FunctionExprent::new(FunctionType::Shl, vec![int(1), int(2)], None);
    assert_eq!(
        func.wrap_operand_string(shift.0, false, 0)
            .convert_to_string_and_allow_data_discard(),
        "1 << 2",
        "an EQUALS3 precedence of 6 needs no parentheses"
    );
    let ternary = kotlin_function(FunctionType::Ternary, vec![int(1), int(2), int(3)]);
    let ternary_bounds = FunctionExprent::check_expr_type_bounds(ternary).expect("ternary bounds");
    assert!(ternary_bounds
        .get_upper_bounds()
        .iter()
        .all(|p| p.type_.equals(&VarType::VARTYPE_BOOLEAN())));
    FunctionExprent::set_func_type_base(func, FunctionType::Sub);
    assert_eq!(func.get_func_type(), FunctionType::Sub);
    assert_eq!(e.as_k_function().kd(|d| d.k_type), KFunctionType::Equals3);
    func.set_func_type(FunctionType::Mul);
    assert_eq!(func.get_func_type(), FunctionType::Mul);
    assert_eq!(e.as_k_function().kd(|d| d.k_type), KFunctionType::None);

    let java = FunctionExprent::new(FunctionType::Add, vec![int(1), int(2)], None);
    assert!(!java.0.is_kotlin() && java.0.try_as_k_function().is_none());
    assert_eq!(java.0.get_precedence(), FunctionType::Add.precedence());
    java.set_func_type(FunctionType::Sub);
    assert_eq!(java.get_func_type(), FunctionType::Sub);

    let var = VarExprent::new(1, Some(VarType::VARTYPE_INT()), None);
    var.0.set_kt(Some(KExprentExt::Var(KVarExprentData {
        declaration_type: None,
        nullable: false,
    })));
    let v = var.0;
    assert_eq!(var.get_name(), "var1");
    assert_eq!(
        v.to_java(0).convert_to_string_and_allow_data_discard(),
        "var1"
    );
    let var_copy = v.copy();
    assert!(var_copy != v && var_copy.is_k_var());
    assert_eq!(
        var_copy.as_k_var().kd(|d| d.declaration_type),
        Some(DeclarationType::Usage),
        "KVarExprent.copy"
    );
    assert_eq!(v.get_precedence(), 0, "KVarExprent inherits getPrecedence");
    assert!(v.get_expr_type().equals(&VarType::VARTYPE_INT()));
    assert_eq!(VarExprent::get_name_base(var), "var1");
    v.as_k_var()
        .set_declaration_type(Some(DeclarationType::Definition));
    assert_eq!(
        v.as_k_var().kd(|d| d.declaration_type),
        Some(DeclarationType::Definition)
    );

    DecompilerContext::start_class(collector());
    let constant = ConstExprent::new_int(7, false, None);
    constant.0.set_kt(Some(KExprentExt::Const));
    assert!(constant.0.is_k_const());
    assert_eq!(
        constant
            .0
            .to_java(0)
            .convert_to_string_and_allow_data_discard(),
        "7",
        "KConstExprent.toJava"
    );
    assert_eq!(constant.to_string(), "const(7)");
    let copy = constant.0.copy();
    assert!(
        copy.is_const() && !copy.is_kotlin(),
        "KConstExprent inherits copy"
    );

    let mismatch = try_catch(|| int(1).set_kt(Some(KExprentExt::Field))).unwrap_err();
    assert!(mismatch.is("IllegalStateException"), "{}", mismatch.message);
    let cast = try_catch(|| {
        int(1).as_k_const();
    })
    .unwrap_err();
    assert!(cast.is("ClassCastException"), "{}", cast.message);
}

#[test]
fn kotlin_statements_and_import_collectors_dispatch_to_their_kotlin_class() {
    let _guard = common::install_default_context();

    let stat = DoStatement::new_empty();
    assert!(!stat.0.is_kotlin() && stat.0.try_as_k_do().is_none());
    stat.0.set_kt(true);
    assert!(stat.0.is_kotlin() && stat.0.is_do() && stat.0.is_k_do());
    assert!(!stat.0.is_k_if());
    // KDoStatement.toJava runs and, like the Java, dereferences the missing body.
    let missing_body = try_catch(|| {
        stat.0.to_java(0);
    })
    .unwrap_err();
    assert!(
        !missing_body.is("UnportedError")
            && missing_body.message.contains("NullPointerException: first"),
        "{}",
        missing_body.message
    );

    let collector = collector();
    assert_eq!(collector.pack_imports(), ["java.util.List"]);
    assert!(collector.try_as_kotlin().is_none());
    collector.set_kotlin(true);
    assert!(collector.is_kotlin());
    assert!(collector.keep_import(("List", "java.util")));
    assert!(!collector.keep_import(("List", "kotlin.collections")));
    assert!(!collector.keep_import(("Unit", "kotlin")));
    assert_eq!(collector.pack_imports(), ["java.util.List"]);
    let mut imports = TextBuffer::new();
    collector.write_imports(&mut imports, true);
    assert_eq!(
        imports.convert_to_string_and_allow_data_discard(),
        "import java.util.List\n\n"
    );
    assert!(ImportCollector::keep_import_base(
        collector,
        ("List", "java.util")
    ));
    assert!(!ImportCollector::new_copy(collector).is_kotlin());
}

#[test]
fn k_type_is_a_var_type() {
    let _guard = common::install_default_context();
    let unit = KType::UNIT();
    assert!(unit.is_k_type() && unit.ptr_eq(&KType::UNIT()));
    assert!(
        unit.equals(&VarType::VARTYPE_VOID()),
        "KType keeps VarType.equals"
    );
    assert_eq!(unit.k().kotlin_type, "kotlin/Unit");
    assert!(KType::UINT().k().is_unsigned_type);
    assert_eq!(KType::NOTHING().value.as_deref(), Some("java/lang/Void"));
    assert!(VarType::VARTYPE_INT().try_as_k_type().is_none());
    let resized = unit.resize_array_dim(1);
    assert!(!resized.is_k_type(), "VarType methods build plain VarTypes");
    let as_var: VarType = KType::UINT().into();
    assert!(as_var.as_k_type().k().is_unsigned_type);
    let cast = try_catch(|| {
        VarType::VARTYPE_INT().as_k_type();
    })
    .unwrap_err();
    assert!(cast.is("ClassCastException"));
}

#[test]
fn kt_attrs_is_a_typed_side_table() {
    let guard =
        common::install_context_with(&[(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")]);
    let sc = guard.struct_context();
    sc.add_space_file(&class_dir(), true);
    let hello = sc.get_class("p/HelloToy").unwrap();
    let solo = sc.get_class("p/SoloToy").unwrap();
    let twice = hello.get_method("twice", "(I)I").unwrap();
    KtAttrs::clear();

    assert!(!KtAttrs::has(hello, &KElement::KEY()));
    KtAttrs::put(
        hello,
        &KElement::KEY(),
        KElement::HiddenElement(KHiddenElement::Generic),
    );
    KtAttrs::put(twice, &PropertyMethod::KEY(), PropertyMethod::Getter);
    assert!(KtAttrs::has(hello, &KElement::KEY()));
    assert!(
        KtAttrs::has(hello, &Key::<KElement>::of("KElement")),
        "keys compare by name"
    );
    assert!(!KtAttrs::has(solo, &KElement::KEY()));
    assert!(!KtAttrs::has(twice, &KElement::KEY()));
    assert!(matches!(
        KtAttrs::get(hello, &KElement::KEY()),
        Some(KElement::HiddenElement(KHiddenElement::Generic))
    ));
    assert_eq!(
        KtAttrs::get(twice, &PropertyMethod::KEY()),
        Some(PropertyMethod::Getter)
    );
    assert_eq!(KtAttrs::get(solo, &PropertyMethod::KEY()), None);
    KtAttrs::put(twice, &PropertyMethod::KEY(), PropertyMethod::Setter);
    assert_eq!(
        KtAttrs::get(twice, &PropertyMethod::KEY()),
        Some(PropertyMethod::Setter)
    );
    assert_eq!(KtAttrs::len(), 2);

    let wrong = try_catch(|| {
        KtAttrs::get(twice, &Key::<KElement>::of(&PropertyMethod::KEY().name)).map(|_| ())
    })
    .unwrap_err();
    assert!(wrong.is("ClassCastException"), "{}", wrong.message);

    assert!(KtAttrs::remove(twice, &PropertyMethod::KEY()));
    assert!(!KtAttrs::remove(twice, &PropertyMethod::KEY()));
    assert_eq!(KtAttrs::len(), 1);
    KtAttrs::clear();
    assert_eq!(KtAttrs::len(), 0);
    assert!(!KtAttrs::has(hello, &KElement::KEY()));
}
