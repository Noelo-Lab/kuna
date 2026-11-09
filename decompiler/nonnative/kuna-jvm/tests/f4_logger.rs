//! `PrintStreamLogger` against Vineflower 1.12.0 (`data/GenF4Oracle.java`) at
//! every severity, plus the collecting and no-op loggers.

use std::io::Write;
use std::rc::Rc;

use kuna_jvm::i_fernflower_logger::{
    CollectingLogger, IFernflowerLogger, LogRecord, PrintStreamLogger, Severity,
};
use kuna_jvm::rt::{try_catch, Shared};

use crate::f4_util::{golden, install_context, unesc};

#[derive(Clone)]
struct SharedBuffer(Shared<Vec<u8>>);

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn script(logger: &dyn IFernflowerLogger) {
    logger.write_message("t", Severity::Trace);
    logger.write_message("i", Severity::Info);
    logger.write_message("w", Severity::Warn);
    logger.write_message("e", Severity::Error);
    logger.start_processing_class("A");
    logger.write_message("in A", Severity::Warn);
    logger.start_reading_class("B");
    logger.start_class("C");
    logger.start_method("m ()V");
    logger.write_message("deep", Severity::Error);
    logger.write_message("deep trace", Severity::Trace);
    logger.end_method();
    logger.start_write_class("W");
    logger.write_message("w2", Severity::Info);
    logger.end_write_class();
    logger.end_class();
    logger.end_reading_class();
    logger.end_processing_class();
    logger.write_message("out", Severity::Info);
}

#[test]
fn print_stream_logger_matches_vineflower() {
    install_context(<dyn IFernflowerLogger>::NO_OP());
    let mut expected: Vec<String> = Vec::new();
    for line in golden("f4_logger.txt") {
        if !line.starts_with("== ") {
            expected.push(unesc(&line).unwrap());
        }
    }
    let mut actual: Vec<String> = Vec::new();
    for severity in Severity::values() {
        let buffer = SharedBuffer(Shared::new(Vec::new()));
        let logger = PrintStreamLogger::new(Box::new(buffer.clone()));
        logger.set_severity(severity);
        script(&logger);
        let text = String::from_utf8(buffer.0.borrow().clone()).unwrap();
        actual.extend(text.split('\n').map(str::to_string));
    }
    assert_eq!(actual, expected);
}

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
    assert_eq!(Severity::Error.prefix(), "ERROR: ");
    assert!(Severity::Trace < Severity::Info && Severity::Warn.ordinal() == 2);

    let silent = <dyn IFernflowerLogger>::NO_OP();
    silent.write_message("x", Severity::Error);
    silent.start_class("A");
    assert!(silent.accepts(Severity::Info) && !silent.accepts(Severity::Trace));
}

#[test]
fn print_stream_logger_prints_the_throwable() {
    install_context(Rc::new(CollectingLogger::new()));
    let buffer = SharedBuffer(Shared::new(Vec::new()));
    let logger = PrintStreamLogger::new(Box::new(buffer.clone()));
    let caught = try_catch(|| -> i32 { kuna_jvm::jthrow!("boom") }).unwrap_err();
    logger.write_message_ex(
        "Class a/B couldn't be fully decompiled.",
        Severity::Error,
        &caught,
    );
    logger.write_message_ex("filtered", Severity::Trace, &caught);
    let text = String::from_utf8(buffer.0.borrow().clone()).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "ERROR: Class a/B couldn't be fully decompiled.");
    assert_eq!(lines[1], "RuntimeException: boom");
    assert!(
        lines[2].starts_with("\tat ") && lines[2].contains("f4_logger.rs"),
        "{text}"
    );
    assert_eq!(lines.len(), 3);
}
