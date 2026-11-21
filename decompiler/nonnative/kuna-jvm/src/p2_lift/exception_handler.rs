//! Port of `org/jetbrains/java/decompiler/code/ExceptionHandler.java`.
//!
//! A Java record: `equals`/`hashCode` over the components (the JDK's
//! `31 * h + hash(component)` from 0).

use crate::decompiler_context::DecompilerContext;
use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExceptionHandler {
    pub from: i32,
    pub to: i32,
    pub handler: i32,
    pub exception_class: Option<String>,
}

impl ExceptionHandler {
    pub fn new(from: i32, to: i32, handler: i32, exception_class: Option<String>) -> Self {
        ExceptionHandler {
            from,
            to,
            handler,
            exception_class,
        }
    }

    pub fn from(&self) -> i32 {
        self.from
    }

    pub fn to(&self) -> i32 {
        self.to
    }

    pub fn handler(&self) -> i32 {
        self.handler
    }

    pub fn exception_class(&self) -> Option<String> {
        self.exception_class.clone()
    }
}

impl JavaEq for ExceptionHandler {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for ExceptionHandler {
    fn java_hash(&self) -> i32 {
        let mut h: i32 = 0;
        h = h.wrapping_mul(31).wrapping_add(self.from);
        h = h.wrapping_mul(31).wrapping_add(self.to);
        h = h.wrapping_mul(31).wrapping_add(self.handler);
        h.wrapping_mul(31)
            .wrapping_add(self.exception_class.java_hash())
    }
}

impl std::fmt::Display for ExceptionHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nl = DecompilerContext::get_new_line_separator();
        write!(
            f,
            "from instr: {} to instr: {} handler instr: {}{}exception class: {}{}",
            self.from,
            self.to,
            self.handler,
            nl,
            self.exception_class.as_deref().unwrap_or("null"),
            nl
        )
    }
}
