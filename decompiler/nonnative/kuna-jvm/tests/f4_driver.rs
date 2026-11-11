//! The `Fernflower` driver over `data/f4_classes/`: option overlay (long and
//! three-letter names), runtime registration, renamer loading, per-class
//! failure handling in `ContextUnit.save` (pre-processing and writing errors,
//! duplicate classes) and in `getClassContent`, and a root context that stays
//! usable whatever the engine does. End-to-end delivery needs every pass of the
//! method pipeline and the class writer, and is ignored until they are ported.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::classes_processor::ClassesProcessor;
use kuna_jvm::context_source::Entry;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_decompiled_data::IDecompiledData;
use kuna_jvm::i_fernflower_logger::{CollectingLogger, IFernflowerLogger, LogRecord, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::jdk_runtime::JdkRuntime;
use kuna_jvm::jthrow;
use kuna_jvm::rt::try_catch;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;

use crate::f4_util::{class_bytes, MemorySource};

const OWN: [&str; 7] = ["p/I", "p/J", "p/A", "p/B", "p/C", "p/D", "p/E"];

fn options(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn sources() -> (MemorySource, MemorySource) {
    let mut own = MemorySource::new("own", false);
    for n in OWN {
        own = own.cls(n, Entry::BASE_VERSION, n);
    }
    let lib = MemorySource::new("lib", false).cls("q/L", Entry::BASE_VERSION, "q/L");
    (own, lib)
}

fn decompile(fernflower: &mut Fernflower) -> Vec<DecompiledClass> {
    let mut out = Vec::new();
    fernflower.decompile_context(&mut |class| out.push(class));
    out
}

fn record(logger: &CollectingLogger, message: &str) -> Option<LogRecord> {
    logger
        .records()
        .borrow()
        .iter()
        .find(|r| r.message == message)
        .cloned()
}

#[test]
fn options_overlay_defaults_by_long_and_short_name() {
    let logger = Rc::new(CollectingLogger::new());
    let fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            ("log", "warn"),
            ("ind", "\t"),
        ]),
        logger.clone(),
    );
    assert_eq!(logger.severity().get(), Severity::Warn);
    assert_eq!(
        DecompilerContext::get_property(IFernflowerPreferences::LOG_LEVEL).as_deref(),
        Some("warn")
    );
    assert_eq!(
        DecompilerContext::get_property(IFernflowerPreferences::INDENT_STRING).as_deref(),
        Some("\t")
    );
    assert!(DecompilerContext::get_option(
        IFernflowerPreferences::DECOMPILE_INNER
    ));
    assert!(DecompilerContext::get_pool_interceptor().is_none());
    let _ = DecompilerContext::get_naming_factory();
    assert!(logger.messages().is_empty());
    drop(fernflower);
}

#[test]
fn constructor_logs_plugins_jvm_and_renamer() {
    let logger = Rc::new(CollectingLogger::new());
    let mut fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            (IFernflowerPreferences::RENAME_ENTITIES, "1"),
            (
                IFernflowerPreferences::USER_RENAMER_CLASS,
                "no.such.Renamer",
            ),
        ]),
        logger.clone(),
    );
    assert!(DecompilerContext::get_pool_interceptor().is_some());
    let renamer = record(&logger, "Cannot load renamer 'no.such.Renamer'").unwrap();
    assert_eq!(renamer.severity, Severity::Warn);
    assert!(renamer
        .throwable
        .unwrap()
        .contains("ClassNotFoundException"));
    let info: Vec<String> = logger
        .records()
        .borrow()
        .iter()
        .filter(|r| r.severity == Severity::Info)
        .map(|r| r.message.clone())
        .collect();
    assert_eq!(
        info,
        vec![
            format!(
                "Loaded {} plugins",
                kuna_jvm::kuna_plugins::bundled_plugins().len()
            ),
            "JVM info: missing vendor - missing java version - missing jvm version".to_string(),
        ]
    );
    fernflower.clear_context();
}

#[test]
fn runtime_option_registers_the_current_jdk() {
    let object = |pairs: &[(&str, &str)]| {
        let mut fernflower = Fernflower::new(&options(pairs), Rc::new(CollectingLogger::new()));
        let object = DecompilerContext::get_struct_context().get_class("java/lang/Object");
        let found = object.map(|cl| (cl.is_own(), cl.qualified_name()));
        fernflower.clear_context();
        found
    };
    assert_eq!(
        object(&[(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")]),
        None
    );
    if JdkRuntime::locate(JdkRuntime::CURRENT).is_none() {
        eprintln!("skipping the JDK half: no current JDK found");
        return;
    }
    let lib = Some((false, "java/lang/Object".to_string()));
    assert_eq!(object(&[]), lib);
    assert_eq!(
        object(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            (IFernflowerPreferences::INCLUDE_ENTIRE_CLASSPATH, "1"),
        ]),
        lib
    );
}

#[test]
fn a_failing_engine_leaves_the_root_context_usable() {
    let logger = Rc::new(CollectingLogger::new());
    let mut fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            ("dee", "0"),
        ]),
        logger.clone(),
    );
    let root = DecompilerContext::get_current_context().unwrap();
    let (own, lib) = sources();
    fernflower.add_library(Rc::new(lib));
    fernflower.add_source(Rc::new(own));

    let mut delivered = Vec::new();
    let result = try_catch(|| fernflower.decompile_context(&mut |c| delivered.push(c)));
    assert!(result.is_ok(), "{:?}", result.err().map(|c| c.message));

    assert!(DecompilerContext::get_current_context()
        .unwrap()
        .ptr_eq(&root));
    assert!(DecompilerContext::get_option(
        IFernflowerPreferences::DECOMPILE_INNER
    ));
    let counters = DecompilerContext::get_counter_container();
    assert!(counters.is_live());
    counters.get_counter_and_increment(CounterContainer::STATEMENT_COUNTER);
    assert!(DecompilerContext::get_struct_context()
        .get_class("p/A")
        .is_some());
    DecompilerContext::get_logger().write_message("still logging", Severity::Warn);
    assert!(logger.messages().contains(&"still logging".to_string()));

    fernflower.clear_context();
    assert!(DecompilerContext::get_current_context().is_none());
}

#[test]
fn class_content_failure_is_logged_and_skipped() {
    let logger = Rc::new(CollectingLogger::new());
    let mut fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            ("dee", "0"),
        ]),
        logger.clone(),
    );
    let root = DecompilerContext::get_current_context().unwrap();
    let (own, _) = sources();
    fernflower.add_source(Rc::new(own));
    let a = DecompilerContext::get_struct_context()
        .get_class("p/A")
        .unwrap();

    assert!(try_catch(|| fernflower.process_class(a)).is_err());
    assert_eq!(fernflower.get_class_content(a), None);
    let failure = record(&logger, "Class p/A couldn't be fully decompiled.").unwrap();
    assert_eq!(failure.severity, Severity::Error);
    assert!(failure.throwable.is_some());
    assert!(DecompilerContext::get_current_context()
        .unwrap()
        .ptr_eq(&root));
    assert_eq!(fernflower.get_class_entry_name(a, "p/A"), None);
    fernflower.clear_context();
}

#[test]
fn class_content_failure_dumps_the_exception() {
    let mut fernflower = Fernflower::new(
        &options(&[(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")]),
        Rc::new(CollectingLogger::new()),
    );
    let (own, _) = sources();
    fernflower.add_source(Rc::new(own));
    let a = DecompilerContext::get_struct_context()
        .get_class("p/A")
        .unwrap();
    let content = fernflower.get_class_content(a).unwrap();
    assert!(
        content.starts_with("/*\n$VF: Unable to decompile class\nPlease report"),
        "{content}"
    );
    assert!(content.ends_with("\n*/"), "{content}");
    fernflower.clear_context();
}

/// An engine stand-in: `p/B` fails while pre-processing, `p/C` goes to the
/// real (not yet runnable) `ClassesProcessor`, `p/D` produces no text.
#[derive(Default)]
struct ScriptedData {
    contexts: RefCell<Vec<(String, DecompilerContext)>>,
}

impl IDecompiledData for ScriptedData {
    fn get_class_entry_name(&self, _cl: StructClass, entry_name: &str) -> Option<String> {
        Some(format!("{entry_name}.java"))
    }

    fn process_class(&self, cl: StructClass) {
        let name = cl.qualified_name();
        let context = DecompilerContext::get_current_context().unwrap();
        self.contexts.borrow_mut().push((name.clone(), context));
        DecompilerContext::get_counter_container()
            .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER);
        match name.as_str() {
            "p/B" => jthrow!("boom in processClass {}", name),
            "p/C" => DecompilerContext::get_class_processor().process_class(cl),
            _ => {}
        }
    }

    fn get_class_content(&self, cl: StructClass) -> Option<String> {
        let name = cl.qualified_name();
        let current = DecompilerContext::get_current_context().unwrap();
        let same = self
            .contexts
            .borrow()
            .iter()
            .any(|(n, c)| *n == name && c.ptr_eq(&current));
        let counter = DecompilerContext::get_counter_container()
            .get_counter(CounterContainer::STATEMENT_COUNTER);
        (name != "p/D").then(|| format!("class {name} same-context {same} counter {counter}"))
    }
}

fn save_scripted(dee: &str) -> (Vec<DecompiledClass>, Rc<CollectingLogger>) {
    let logger = Rc::new(CollectingLogger::new());
    let data = Rc::new(ScriptedData::default());
    let sc = StructContext::new(data.clone());
    let mut properties = IFernflowerPreferences::DEFAULTS();
    properties.put(
        IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR.to_string(),
        dee.to_string(),
    );
    let root = DecompilerContext::new(
        properties,
        logger.clone(),
        sc,
        ClassesProcessor::new(sc),
        None,
    );
    DecompilerContext::set_current_context(Some(root.clone()));
    let mut own = MemorySource::new("own", false);
    for n in ["p/A", "p/B", "p/C", "p/D", "p/A"] {
        own = own.cls(n, Entry::BASE_VERSION, n);
    }
    sc.add_space(Rc::new(own), true);

    let mut out = Vec::new();
    sc.save_context(&mut |class| out.push(class));

    assert!(DecompilerContext::get_current_context()
        .unwrap()
        .ptr_eq(&root));
    assert_eq!(
        DecompilerContext::get_counter_container().get_counter(CounterContainer::STATEMENT_COUNTER),
        1
    );
    let contexts = data.contexts.borrow();
    assert_eq!(contexts.len(), 4);
    assert!(contexts.iter().all(|(_, c)| !c.ptr_eq(&root)));
    drop(contexts);
    (out, logger)
}

#[test]
fn unit_save_reports_failing_classes_and_keeps_going() {
    let (out, logger) = save_scripted("0");
    let names: Vec<&str> = out.iter().map(|c| c.qualified_name.as_str()).collect();
    assert_eq!(names, ["p/A", "p/B", "p/C"]);
    assert_eq!(out[0].path, "p/A.java");
    assert_eq!(out[0].content, "class p/A same-context true counter 2");
    assert_eq!(out[0].mapping, None);
    assert_eq!(out[1].content, "// $VF: Couldn't be decompiled\n");
    assert_eq!(out[2].content, "// $VF: Couldn't be decompiled\n");

    let boom = record(&logger, "Class p/B couldn't be fully decompiled.").unwrap();
    assert_eq!(boom.severity, Severity::Error);
    assert!(boom.throwable.unwrap().contains("boom in processClass p/B"));
    assert!(record(&logger, "Class p/C couldn't be fully decompiled.").is_some());
    let duplicate = record(&logger, "Skipping writing already existing class: p/A").unwrap();
    assert_eq!(duplicate.severity, Severity::Error);
    assert!(record(&logger, "Class p/A couldn't be fully decompiled.").is_none());
    assert!(record(&logger, "Class p/D couldn't be fully decompiled.").is_none());
}

#[test]
fn unit_save_dumps_the_pre_processing_exception() {
    let (out, _) = save_scripted("1");
    assert_eq!(out.len(), 3);
    assert!(out[1]
        .content
        .starts_with("// $VF: Couldn't be decompiled\n// Please report"));
    assert!(out[1].content.contains("boom in processClass p/B"));
}

#[test]
fn delivers_each_root_class_in_entry_order() {
    let logger = Rc::new(CollectingLogger::new());
    let mut fernflower = Fernflower::new(
        &options(&[(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")]),
        logger.clone(),
    );
    let (own, lib) = sources();
    fernflower.add_library(Rc::new(lib));
    fernflower.add_source(Rc::new(own));
    let classes = decompile(&mut fernflower);

    let names: Vec<&str> = classes.iter().map(|c| c.qualified_name.as_str()).collect();
    assert_eq!(names, OWN.to_vec());
    for class in &classes {
        assert_eq!(class.path, format!("{}.java", class.qualified_name));
        let simple = class.qualified_name.rsplit('/').next().unwrap();
        assert!(class.content.contains(simple), "{}", class.content);
        assert_eq!(class.mapping, None);
    }

    let counters = DecompilerContext::get_counter_container();
    assert!(counters.is_live());

    let a = StructClass::create(&mut ByteCursor::new(class_bytes("p/A")), true);
    assert_eq!(
        fernflower.get_class_entry_name(a, "p/A").as_deref(),
        Some("p/A.java")
    );
    assert_eq!(
        fernflower.get_class_entry_name(a, "x/y/A.class").as_deref(),
        Some("x/y/A.java")
    );
    let l = StructClass::create(&mut ByteCursor::new(class_bytes("q/L")), false);
    assert_eq!(fernflower.get_class_entry_name(l, "q/L"), None);

    fernflower.clear_context();
    assert!(DecompilerContext::get_current_context().is_none());
}

#[test]
fn line_mapping_follows_the_option() {
    let mut fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            ("bsm", "true"),
            ("log-level", "error"),
        ]),
        Rc::new(CollectingLogger::new()),
    );
    assert!(DecompilerContext::get_option(
        IFernflowerPreferences::BYTECODE_SOURCE_MAPPING
    ));
    let (own, lib) = sources();
    fernflower.add_source(Rc::new(own));
    fernflower.add_library(Rc::new(lib));
    let classes = decompile(&mut fernflower);
    assert_eq!(classes.len(), OWN.len());
    assert!(classes.iter().all(|c| c.mapping.is_some()));
    fernflower.clear_context();
}

#[test]
fn unknown_log_level_keeps_the_logger_severity() {
    let logger: Rc<dyn IFernflowerLogger> = Rc::new(CollectingLogger::new());
    logger.set_severity(Severity::Error);
    let mut fernflower = Fernflower::new(
        &options(&[
            (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
            ("log-level", "debug"),
        ]),
        logger.clone(),
    );
    assert_eq!(logger.severity().get(), Severity::Error);
    fernflower.clear_context();
}
