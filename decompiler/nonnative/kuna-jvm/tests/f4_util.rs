//! Shared fixtures of the F4 tests: golden-file decoding, an in-memory class
//! source and a minimal installed decompilation context.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_jvm::classes_processor::ClassesProcessor;
use kuna_jvm::context_source::{Entries, Entry, IContextSource};
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_decompiled_data::IDecompiledData;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::JHashMap;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;

pub fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

pub fn golden(name: &str) -> Vec<String> {
    std::fs::read_to_string(data_dir().join(name))
        .unwrap_or_else(|e| panic!("{name}: {e}"))
        .lines()
        .map(str::to_string)
        .collect()
}

/// Inverse of the generator's `esc`: `\N` is null.
pub fn unesc(s: &str) -> Option<String> {
    if s == "\\N" {
        return None;
    }
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            other => panic!("bad escape {other:?} in {s}"),
        }
    }
    Some(out)
}

/// Inverse of the generator's `hex`: `x` + four hex digits per UTF-16 unit, `N` is null.
pub fn unhex(s: &str) -> Option<String> {
    if s == "N" {
        return None;
    }
    let hex = s.strip_prefix('x').expect("hex token");
    let units: Vec<u16> = (0..hex.len() / 4)
        .map(|i| u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).unwrap())
        .collect();
    Some(from_utf16(&units))
}

pub struct NoDecompiledData;

impl IDecompiledData for NoDecompiledData {
    fn get_class_entry_name(&self, _cl: StructClass, _entryname: &str) -> Option<String> {
        None
    }

    fn process_class(&self, _cl: StructClass) {}

    fn get_class_content(&self, _cl: StructClass) -> Option<String> {
        None
    }
}

/// A root context over an empty class universe, installed on this thread.
pub fn install_context(logger: Rc<dyn IFernflowerLogger>) -> DecompilerContext {
    install_context_with(logger, IFernflowerPreferences::DEFAULTS())
}

pub fn install_context_with(
    logger: Rc<dyn IFernflowerLogger>,
    properties: JHashMap<String, String>,
) -> DecompilerContext {
    let struct_context = StructContext::new(Rc::new(NoDecompiledData));
    let context = DecompilerContext::new(
        properties,
        logger,
        struct_context,
        ClassesProcessor::new(struct_context),
        None,
    );
    DecompilerContext::set_current_context(Some(context.clone()));
    context
}

pub fn class_bytes(path: &str) -> Vec<u8> {
    std::fs::read(data_dir().join("f4_classes").join(format!("{path}.class")))
        .unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The generator's `MemorySource`: classes and other files held in memory.
pub struct MemorySource {
    pub name: String,
    pub lazy: bool,
    pub classes: Vec<Entry>,
    pub others: Vec<Entry>,
    pub data: RefCell<Vec<(String, Vec<u8>)>>,
}

impl MemorySource {
    pub fn new(name: &str, lazy: bool) -> MemorySource {
        MemorySource {
            name: name.to_string(),
            lazy,
            classes: Vec::new(),
            others: Vec::new(),
            data: RefCell::new(Vec::new()),
        }
    }

    pub fn cls(mut self, entry: &str, multirelease: i32, file: &str) -> MemorySource {
        self.classes
            .push(Entry::new(entry.to_string(), multirelease));
        if multirelease == Entry::BASE_VERSION {
            self.data
                .borrow_mut()
                .push((format!("{entry}.class"), class_bytes(file)));
        }
        self
    }

    pub fn other(mut self, path: &str, content: &str) -> MemorySource {
        self.others.push(Entry::at_base(path));
        self.data
            .borrow_mut()
            .push((path.to_string(), content.as_bytes().to_vec()));
        self
    }
}

impl IContextSource for MemorySource {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_entries(&self) -> Entries {
        Entries::new(self.classes.clone(), Vec::new(), self.others.clone())
    }

    fn is_lazy(&self) -> bool {
        self.lazy
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        self.data
            .borrow()
            .iter()
            .find(|(path, _)| path == resource)
            .map(|(_, bytes)| bytes.clone())
    }
}
