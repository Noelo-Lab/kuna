//! The `Fernflower` driver end to end over `data/f4_classes/`: option overlay
//! (long and three-letter names), one delivered result per own root class in
//! entry order, entry names, the line mapping switch, and a root context that
//! stays usable after the class scope is reset.

use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::context_source::Entry;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_decompiled_data::IDecompiledData;
use kuna_jvm::i_fernflower_logger::{CollectingLogger, IFernflowerLogger, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::struct_class::StructClass;

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

#[test]
fn delivers_each_root_class_in_entry_order() {
    let logger = Rc::new(CollectingLogger::new());
    let mut fernflower = Fernflower::new(
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
    assert!(DecompilerContext::get_pool_interceptor().is_none());

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

    assert!(DecompilerContext::get_option(
        IFernflowerPreferences::DECOMPILE_INNER
    ));
    let counters = DecompilerContext::get_counter_container();
    assert!(counters.is_live());
    assert!(counters.get_counter(CounterContainer::STATEMENT_COUNTER) >= 1);

    let a = StructClass::create(ByteCursor::new(class_bytes("p/A")), true);
    assert_eq!(
        fernflower.get_class_entry_name(a, "p/A").as_deref(),
        Some("p/A.java")
    );
    assert_eq!(
        fernflower.get_class_entry_name(a, "x/y/A.class").as_deref(),
        Some("x/y/A.java")
    );
    let l = StructClass::create(ByteCursor::new(class_bytes("q/L")), false);
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
