//! Port of `org/vineflower/kotlin/KotlinImportCollector.java`.
//!
//! `KotlinImportCollector extends ImportCollector` adds no field: it is an
//! `ImportCollector` with `kotlin` set, and the two methods it overrides
//! branch on that flag.

use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::import_collector::ImportCollector;
use crate::k_types::KTypes;
use crate::text_buffer::TextBuffer;

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
    /// Java: KotlinImportCollector(ImportCollector). The JDK iterates the
    /// `KTypes` tables in a per-run order; the order cannot reach the output:
    /// every simple name in them is distinct, so the map ends up the same, and
    /// the map is only read by lookups and by `packImports`, which sorts.
    pub fn new(parent: ImportCollector) -> KotlinImportCollector {
        let collector = ImportCollector::new_copy(parent);
        collector.set_kotlin(true);

        for (class_name, _) in KTypes::KOTLIN_TO_JAVA_LANG {
            put_kotlin_name(collector, class_name);
        }

        for (class_name, _) in KTypes::KOTLIN_TO_JAVA_UTIL {
            put_kotlin_name(collector, class_name);
        }

        KotlinImportCollector(collector)
    }

    pub fn keep_import(self, ent: (&str, &str)) -> bool {
        if !ImportCollector::keep_import_base(self.0, ent) {
            return false;
        }
        let (_, value) = ent;
        if value == "kotlin" {
            return false;
        }
        for auto_import in KotlinImportCollector::AUTO_KOTLIN_IMPORTS {
            if value.strip_prefix("kotlin.") == Some(auto_import) {
                return false;
            }
        }
        true
    }

    pub fn write_imports(self, buffer: &mut TextBuffer, add_separator: bool) {
        if DecompilerContext::get_option(IFernflowerPreferences::REMOVE_IMPORTS) {
            return;
        }

        let imports = self.0.pack_imports();
        for imp in &imports {
            buffer.append("import ").append(imp).append_line_separator();
        }
        if add_separator && !imports.is_empty() {
            buffer.append_line_separator();
        }
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

/// One step of the constructor's loops: `className`'s simple name maps to its
/// package unless the name is taken.
fn put_kotlin_name(collector: ImportCollector, class_name: &str) {
    let slash = class_name.rfind('/').expect("a qualified name");
    let simple_name = class_name[slash + 1..].to_string();
    let package_name = class_name[..slash].replace('/', ".");
    collector.with_mut(|d| {
        if !d.map_simple_names.contains_key(&simple_name) {
            d.map_simple_names.put(simple_name, package_name);
        }
    });
}
