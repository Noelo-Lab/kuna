//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/DomTracer.java`.
//!
//! The dot-file collection (`COLLECT_DOTS`) goes through the dropped
//! `DotExporter`, so the per-node colour maps are not built.

use crate::statement::Statement;
use crate::struct_method::StructMethod;

const COLLECT_STRINGS: bool = false;
const COLLECT_DOTS: bool = false;

#[allow(dead_code)]
pub struct DomTracer {
    file_prefix: String,
    struct_method: StructMethod,
    string: String,
    counter: i32,
}

impl DomTracer {
    pub fn new(file_prefix: &str, struct_method: StructMethod) -> DomTracer {
        DomTracer {
            file_prefix: file_prefix.to_string(),
            struct_method,
            string: String::new(),
            counter: 0,
        }
    }

    fn add(&mut self, gen: Statement, s: &str) {
        let mut sb = String::new();
        if COLLECT_DOTS && COLLECT_STRINGS {
            sb.push_str(&format!("(g{}) ", self.counter));
        }

        if COLLECT_STRINGS {
            sb.push_str(&format!("[{gen}] {s}"));

            self.string.push_str(&sb);
            self.string.push('\n');
        }

        self.counter += 1;
    }

    pub fn error(&mut self, stat: Statement, s: &str) {
        self.add(stat, s);
    }

    pub fn warn(&mut self, stat: Statement, s: &str) {
        self.add(stat, s);
    }

    pub fn info(&mut self, stat: Statement, s: &str) {
        self.add(stat, s);
    }

    pub fn success(&mut self, stat: Statement, s: &str) {
        self.add(stat, s);
    }

    pub fn success_created(&mut self, stat: Statement, s: &str, _new_stat: Statement) {
        self.add(stat, s);
    }
}

impl std::fmt::Display for DomTracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string)
    }
}
