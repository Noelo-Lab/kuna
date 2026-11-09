//! `DecompilerContext` (option parsing against Vineflower 1.12.0, installing and
//! removing contexts, the per-class copy `ContextUnit.setContext` makes),
//! `CounterContainer` and `CancelationManager`.

use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::cancelation_manager::{CancelationManager, CanceledException};
use kuna_jvm::context_unit::ContextUnit;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_logger::{CollectingLogger, IFernflowerLogger, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::identity_renamer_factory::IdentityRenamerFactory;
use kuna_jvm::rt::{reset_class_scope, try_catch};
use kuna_jvm::struct_class::StructClass;

use crate::f4_util::{class_bytes, golden, install_context, unhex, MemorySource, NoDecompiledData};

#[test]
fn int_and_boolean_options_match_vineflower() {
    install_context(<dyn IFernflowerLogger>::NO_OP());
    for line in golden("f4_context.txt") {
        let cols: Vec<&str> = line.split('\t').collect();
        let (key, value) = match unhex(cols[1]) {
            Some(value) => {
                DecompilerContext::set_property("int-probe", value.clone());
                ("int-probe", Some(value))
            }
            None => ("absent-probe", None),
        };
        assert_eq!(
            DecompilerContext::get_int_option(key).to_string(),
            cols[2],
            "{value:?}"
        );
        assert_eq!(
            DecompilerContext::get_option(key),
            cols[3] == "1",
            "{value:?}"
        );
    }
}

#[test]
fn context_is_installed_per_thread() {
    assert!(DecompilerContext::get_current_context().is_none());
    assert!(
        try_catch(|| DecompilerContext::get_logger().accepts(Severity::Info))
            .unwrap_err()
            .is("NullPointerException")
    );

    let logger = Rc::new(CollectingLogger::new());
    let context = install_context(logger.clone());
    let current = DecompilerContext::get_current_context().unwrap();
    assert!(current.ptr_eq(&context));
    DecompilerContext::get_logger().write_message("hello", Severity::Warn);
    assert_eq!(logger.messages(), vec!["hello".to_string()]);

    assert!(DecompilerContext::get_option(
        IFernflowerPreferences::DECOMPILE_INNER
    ));
    assert_eq!(
        DecompilerContext::get_int_option(IFernflowerPreferences::PREFERRED_LINE_LENGTH),
        160
    );
    assert_eq!(DecompilerContext::get_new_line_separator(), "\n");
    DecompilerContext::set_property(IFernflowerPreferences::NEW_LINE_SEPARATOR, "0");
    assert_eq!(DecompilerContext::get_new_line_separator(), "\r\n");
    assert_eq!(
        DecompilerContext::get_property(IFernflowerPreferences::INDENT_STRING).as_deref(),
        Some("   ")
    );
    assert_eq!(DecompilerContext::get_property("no-such-option"), None);
    assert!(DecompilerContext::get_pool_interceptor().is_none());

    let handle = std::thread::spawn(|| DecompilerContext::get_current_context().is_none());
    assert!(handle.join().unwrap());

    DecompilerContext::set_current_context(None);
    assert!(DecompilerContext::get_current_context().is_none());
    DecompilerContext::set_current_context(Some(context.clone()));
    assert!(!DecompilerContext::get_option(
        IFernflowerPreferences::NEW_LINE_SEPARATOR
    ));
}

#[test]
fn typed_context_properties_round_trip() {
    install_context(<dyn IFernflowerLogger>::NO_OP());
    assert!(DecompilerContext::get_current_class().is_none());
    let cl = StructClass::create(&mut ByteCursor::new(class_bytes("p/A")), true);
    DecompilerContext::set_current_class(Some(cl));
    assert_eq!(DecompilerContext::get_current_class(), Some(cl));
    assert!(DecompilerContext::get_current_class_node().is_none());
    assert!(DecompilerContext::get_current_method_wrapper().is_none());
    assert!(DecompilerContext::get_current_class_wrapper().is_none());
    assert!(DecompilerContext::get_current_var_processor().is_none());
    DecompilerContext::set_current_class(None);
    assert!(DecompilerContext::get_current_class().is_none());
    assert!(try_catch(DecompilerContext::get_import_collector).is_err());
    assert!(try_catch(DecompilerContext::get_var_processor).is_err());
    assert!(try_catch(DecompilerContext::get_naming_factory).is_err());
}

#[test]
fn counters_start_at_one_and_post_increment() {
    let counters = CounterContainer::new();
    assert_eq!(counters.get_counter(CounterContainer::VAR_COUNTER), 1);
    assert_eq!(
        counters.get_counter_and_increment(CounterContainer::VAR_COUNTER),
        1
    );
    assert_eq!(
        counters.get_counter_and_increment(CounterContainer::VAR_COUNTER),
        2
    );
    assert_eq!(counters.get_counter(CounterContainer::VAR_COUNTER), 3);
    assert_eq!(counters.get_counter(CounterContainer::STATEMENT_COUNTER), 1);
    counters.set_counter(CounterContainer::EXPRESSION_COUNTER, i32::MAX);
    assert_eq!(
        counters.get_counter_and_increment(CounterContainer::EXPRESSION_COUNTER),
        i32::MAX
    );
    assert_eq!(
        counters.get_counter(CounterContainer::EXPRESSION_COUNTER),
        i32::MIN
    );
    assert!(try_catch(|| counters.get_counter(4)).is_err());
    assert_ne!(CounterContainer::new(), counters);

    install_context(<dyn IFernflowerLogger>::NO_OP());
    let context_counters = DecompilerContext::get_counter_container();
    assert_eq!(
        context_counters.get_counter_and_increment(CounterContainer::STATEMENT_COUNTER),
        1
    );
    assert_eq!(DecompilerContext::get_counter_container(), context_counters);
    assert_eq!(
        context_counters.get_counter(CounterContainer::STATEMENT_COUNTER),
        2
    );
}

#[test]
fn per_class_context_copies_the_root() {
    let logger: Rc<dyn IFernflowerLogger> = Rc::new(CollectingLogger::new());
    let root = install_context(logger.clone());
    root.with_mut(|d| d.renamer_factory = Some(Rc::new(IdentityRenamerFactory)));
    DecompilerContext::get_counter_container().set_counter(CounterContainer::VAR_COUNTER, 40);
    let unit = ContextUnit::new(
        Rc::new(MemorySource::new("own", false)),
        true,
        true,
        Rc::new(NoDecompiledData),
    );

    unit.set_context(Some(&root));
    assert!(DecompilerContext::get_current_context()
        .unwrap()
        .ptr_eq(&root));

    DecompilerContext::set_current_context(None);
    unit.set_context(Some(&root));
    let task = DecompilerContext::get_current_context().unwrap();
    assert!(!task.ptr_eq(&root));
    assert!(Rc::ptr_eq(&DecompilerContext::get_logger(), &logger));
    assert_eq!(
        DecompilerContext::get_struct_context(),
        root.with(|d| d.struct_context)
    );
    assert_eq!(
        DecompilerContext::get_class_processor(),
        root.with(|d| d.class_processor)
    );
    assert_eq!(
        DecompilerContext::get_counter_container().get_counter(CounterContainer::VAR_COUNTER),
        1
    );
    let _ = DecompilerContext::get_naming_factory();
    DecompilerContext::set_property(IFernflowerPreferences::BANNER, "// task");
    assert_eq!(
        root.with(|d| d.properties.get_cloned(&"banner".to_string()))
            .as_deref(),
        Some("")
    );
    assert_eq!(
        root.counter_values()[CounterContainer::VAR_COUNTER as usize],
        40
    );

    let counters = root.counter_values();
    reset_class_scope();
    root.survive_class_scope_reset(counters);
    DecompilerContext::set_current_context(Some(root));
    assert_eq!(
        DecompilerContext::get_counter_container().get_counter(CounterContainer::VAR_COUNTER),
        40
    );
}

#[test]
fn cancelation_checker_throws_where_installed() {
    CancelationManager::check_canceled();
    let me = std::thread::current().id();
    CancelationManager::set_cancelation_checker(move || {
        if std::thread::current().id() == me {
            CancelationManager::cancel();
        }
    });
    let caught = try_catch(CancelationManager::check_canceled).unwrap_err();
    assert!(CanceledException::is(&caught));
    assert!(
        std::thread::spawn(|| try_catch(CancelationManager::check_canceled).is_ok())
            .join()
            .unwrap()
    );
    CancelationManager::clear_cancelation_checker();
    CancelationManager::check_canceled();
}
