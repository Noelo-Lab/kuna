//! Port of `org/jetbrains/java/decompiler/main/extern/IFernflowerLogger.java`.
//!
//! The abstract class becomes a trait whose implementors expose the base
//! class's `severity` field; loggers are shared as `Rc<dyn IFernflowerLogger>`.

use std::cell::Cell;
use std::rc::Rc;

use crate::rt::{Caught, Shared};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Severity {
    Trace,
    Info,
    Warn,
    Error,
}

impl Severity {
    pub fn name(self) -> &'static str {
        match self {
            Severity::Trace => "TRACE",
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::Error => "ERROR",
        }
    }

    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

pub trait IFernflowerLogger {
    /// The abstract class's `severity` field (initially `Severity::Info`).
    fn severity(&self) -> &Cell<Severity>;

    fn accepts(&self, severity: Severity) -> bool {
        severity.ordinal() >= self.severity().get().ordinal()
    }

    fn set_severity(&self, severity: Severity) {
        self.severity().set(severity)
    }

    fn write_message(&self, message: &str, severity: Severity);

    /// Java: writeMessage(String, Severity, Throwable)
    fn write_message_ex(&self, message: &str, severity: Severity, t: &Caught);

    /// Java: writeMessage(String, Throwable)
    fn write_message_throwable(&self, message: &str, t: &Caught) {
        self.write_message_ex(message, Severity::Error, t)
    }

    fn start_processing_class(&self, _class_name: &str) {}

    fn end_processing_class(&self) {}

    fn start_reading_class(&self, _class_name: &str) {}

    fn end_reading_class(&self) {}

    fn start_class(&self, _class_name: &str) {}

    fn end_class(&self) {}

    fn start_method(&self, _method_name: &str) {}

    fn end_method(&self) {}

    fn start_write_class(&self, _class_name: &str) {}

    fn end_write_class(&self) {}
}

impl dyn IFernflowerLogger {
    #[allow(non_snake_case)]
    pub fn NO_OP() -> Rc<dyn IFernflowerLogger> {
        Rc::new(NoOpLogger::new())
    }
}

/// The anonymous `IFernflowerLogger.NO_OP` class.
pub struct NoOpLogger {
    severity: Cell<Severity>,
}

impl NoOpLogger {
    #[allow(clippy::new_without_default)]
    pub fn new() -> NoOpLogger {
        NoOpLogger {
            severity: Cell::new(Severity::Info),
        }
    }
}

impl IFernflowerLogger for NoOpLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, _message: &str, _severity: Severity) {}

    fn write_message_ex(&self, _message: &str, _severity: Severity, _t: &Caught) {}
}

/// One accepted message of a [`CollectingLogger`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogRecord {
    pub severity: Severity,
    pub message: String,
    pub throwable: Option<String>,
}

/// A logger for tests: keeps every accepted message instead of printing it.
pub struct CollectingLogger {
    severity: Cell<Severity>,
    records: Shared<Vec<LogRecord>>,
}

impl CollectingLogger {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CollectingLogger {
        CollectingLogger {
            severity: Cell::new(Severity::Info),
            records: Shared::new(Vec::new()),
        }
    }

    /// The record list; it aliases the logger's own.
    pub fn records(&self) -> Shared<Vec<LogRecord>> {
        self.records.clone()
    }

    pub fn messages(&self) -> Vec<String> {
        self.records
            .borrow()
            .iter()
            .map(|r| r.message.clone())
            .collect()
    }
}

impl IFernflowerLogger for CollectingLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, message: &str, severity: Severity) {
        if self.accepts(severity) {
            self.records.borrow_mut().push(LogRecord {
                severity,
                message: message.to_string(),
                throwable: None,
            });
        }
    }

    fn write_message_ex(&self, message: &str, severity: Severity, t: &Caught) {
        if self.accepts(severity) {
            self.records.borrow_mut().push(LogRecord {
                severity,
                message: message.to_string(),
                throwable: Some(t.message.clone()),
            });
        }
    }
}
