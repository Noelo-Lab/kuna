//! The collecting and no-op loggers and severity parsing.

use kuna_jvm::i_fernflower_logger::{CollectingLogger, IFernflowerLogger, LogRecord, Severity};
use kuna_jvm::rt::try_catch;

#[test]
fn severities_filter_and_parse() {
    let logger = CollectingLogger::new();
    assert_eq!(logger.severity().get(), Severity::Info);
    logger.write_message("hidden", Severity::Trace);
    logger.write_message("shown", Severity::Info);
    logger.set_severity(Severity::Warn);
    assert!(!logger.accepts(Severity::Info) && logger.accepts(Severity::Error));
    logger.write_message("hidden too", Severity::Info);
    let caught =
        try_catch(|| -> i32 { kuna_jvm::jthrow!(IllegalStateException; "bad") }).unwrap_err();
    logger.write_message_throwable("failed", &caught);
    logger.write_message_ex("dropped", Severity::Info, &caught);
    assert_eq!(
        logger.records().borrow().clone(),
        vec![
            LogRecord {
                severity: Severity::Info,
                message: "shown".into(),
                throwable: None
            },
            LogRecord {
                severity: Severity::Error,
                message: "failed".into(),
                throwable: Some("IllegalStateException: bad".into())
            },
        ]
    );
    assert_eq!(Severity::value_of("WARN"), Some(Severity::Warn));
    assert_eq!(Severity::value_of("DEBUG"), None);
    assert_eq!(Severity::value_of("warn"), None);
    assert!(Severity::Trace < Severity::Info && Severity::Warn.ordinal() == 2);

    let silent = <dyn IFernflowerLogger>::NO_OP();
    silent.write_message("x", Severity::Error);
    silent.start_class("A");
    assert!(silent.accepts(Severity::Info) && !silent.accepts(Severity::Trace));
}
