//! Port of `org/vineflower/kotlin/KotlinImportCollector.java`.
//!
//! `KotlinImportCollector extends ImportCollector` adds no field: it is an
//! `ImportCollector` with `kotlin` set, and the two methods it overrides
//! branch on that flag.

use crate::import_collector::ImportCollector;
use crate::text_buffer::TextBuffer;
use crate::unported;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct KotlinImportCollector(pub ImportCollector);

impl std::ops::Deref for KotlinImportCollector {
    type Target = ImportCollector;
    fn deref(&self) -> &ImportCollector {
        &self.0
    }
}

impl From<KotlinImportCollector> for ImportCollector {
    fn from(collector: KotlinImportCollector) -> ImportCollector {
        collector.0
    }
}

impl KotlinImportCollector {
    pub const AUTO_KOTLIN_IMPORTS: [&'static str; 8] = [
        "annotation",
        "collections",
        "comparisons",
        "io",
        "jvm",
        "ranges",
        "sequences",
        "text",
    ];

    /// Java: KotlinImportCollector(ImportCollector)
    pub fn new(_parent: ImportCollector) -> KotlinImportCollector {
        unported!("KotlinImportCollector.<init>")
    }

    pub fn keep_import(self, _ent: (&str, &str)) -> bool {
        unported!("KotlinImportCollector.keepImport")
    }

    pub fn write_imports(self, _buffer: &mut TextBuffer, _add_separator: bool) {
        unported!("KotlinImportCollector.writeImports")
    }
}

impl ImportCollector {
    /// Java `instanceof KotlinImportCollector`.
    pub fn is_kotlin(self) -> bool {
        self.with(|d| d.kotlin)
    }

    /// Makes this collector a `KotlinImportCollector` (the constructor's subclass step).
    pub fn set_kotlin(self, kotlin: bool) {
        self.with_mut(|d| d.kotlin = kotlin)
    }

    pub fn try_as_kotlin(self) -> Option<KotlinImportCollector> {
        self.is_kotlin().then_some(KotlinImportCollector(self))
    }
}
