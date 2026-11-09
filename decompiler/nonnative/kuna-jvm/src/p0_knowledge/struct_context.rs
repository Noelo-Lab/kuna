//! Port of `org/jetbrains/java/decompiler/struct/StructContext.java`.
//!
//! The class universe of a session. The Java caches failed lookups under a
//! sentinel `StructClass`; here the cache holds `None` for them. The result
//! saver is replaced by the class callback handed to [`StructContext::save_context`].

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::rc::Rc;

use crate::context_source::{
    DirectoryContextSource, IContextSource, JarContextSource, SingleFileContextSource,
};
use crate::context_unit::ContextUnit;
use crate::decompiler_context::DecompilerContext;
use crate::fernflower::DecompiledClass;
use crate::generic_main::GenericMain;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::i_decompiled_data::IDecompiledData;
use crate::i_fernflower_logger::Severity;
use crate::plugin_context::PluginContext;
use crate::rt::{rethrow, try_catch, Caught, JHashMap, JavaException};
use crate::struct_class::StructClass;
use crate::{arena_identity, arena_object, jthrow};

pub struct StructContextData {
    pub decompiled_data: Rc<dyn IDecompiledData>,
    pub units: Vec<ContextUnit>,
    pub lazy_units: Vec<ContextUnit>,
    pub classes: JHashMap<String, Option<StructClass>>,
    pub badly_placed_classes: JHashMap<String, String>,
    pub units_by_class_name: JHashMap<String, ContextUnit>,
    pub abstract_names: JHashMap<String, Vec<String>>,
    pub plugin_context: PluginContext,
}

arena_object!(pub StructContext => StructContextData, Session);
arena_identity!(StructContext);

impl StructContext {
    /// Java: StructContext(IResultSaver, IDecompiledData)
    pub fn new(decompiled_data: Rc<dyn IDecompiledData>) -> StructContext {
        StructContext::alloc(StructContextData {
            decompiled_data,
            units: Vec::new(),
            lazy_units: Vec::new(),
            classes: JHashMap::new(),
            badly_placed_classes: JHashMap::new(),
            units_by_class_name: JHashMap::new(),
            abstract_names: JHashMap::new(),
            plugin_context: PluginContext::new(),
        })
    }

    pub fn get_class(self, name: &str) -> Option<StructClass> {
        let key = name.to_string();
        let cached = self.with(|d| d.classes.get(&key).copied());
        let ret = match cached {
            Some(ret) => ret,
            None => {
                let loaded = self.load_class(&key);
                self.with_mut(|d| *d.classes.compute_if_absent(key.clone(), || loaded))
            }
        };
        let ret = ret?;
        let corrected_name = self.with_mut(|d| d.badly_placed_classes.remove(&key));
        if let Some(corrected_name) = corrected_name {
            self.with_mut(|d| {
                d.classes.put(corrected_name, Some(ret));
                d.classes.remove(&key);
            });
        }
        Some(ret)
    }

    /// The `computeIfAbsent` mapping function of `getClass`.
    fn load_class(self, key: &str) -> Option<StructClass> {
        let unit_for_class = self.with(|d| d.units_by_class_name.get(&key.to_string()).copied());
        if let Some(unit_for_class) = unit_for_class {
            if let Some(clazz) = self.try_load_class(unit_for_class, key) {
                return Some(clazz);
            }
        }
        let lazy_units = self.with(|d| d.lazy_units.clone());
        for unit in lazy_units {
            if let Some(clazz) = self.try_load_class(unit, key) {
                return Some(clazz);
            }
        }
        None
    }

    fn try_load_class(self, unit_for_class: ContextUnit, key: &str) -> Option<StructClass> {
        let result = try_catch(|| {
            DecompilerContext::get_logger().write_message(
                &format!("Loading Class: {} from {}", key, unit_for_class.get_name()),
                Severity::Info,
            );
            let clazz = unit_for_class.try_load_class(key)?;

            let qualified_name = qualified_name_of(clazz);
            if key != qualified_name {
                self.with_mut(|d| {
                    d.units_by_class_name
                        .put(qualified_name.clone(), unit_for_class);
                    d.badly_placed_classes.put(key.to_string(), qualified_name);
                });
            }
            Some(clazz)
        });
        match result {
            Ok(clazz) => clazz,
            Err(ex) if is_io_exception(&ex) => {
                DecompilerContext::get_logger().write_message_ex(
                    &format!(
                        "Failed to read class {} from {}",
                        key,
                        unit_for_class.get_name()
                    ),
                    Severity::Error,
                    &ex,
                );
                None
            }
            Err(ex) => rethrow(ex),
        }
    }

    pub fn has_class(self, name: &str) -> bool {
        if self.with(|d| d.units_by_class_name.contains_key(&name.to_string())) {
            return true;
        }

        let lazy_units = self.with(|d| d.lazy_units.clone());
        for unit in lazy_units {
            match try_catch(|| unit.has_class(name)) {
                Ok(true) => return true,
                Ok(false) => {}
                Err(ex) if is_io_exception(&ex) => {
                    DecompilerContext::get_logger().write_message_ex(
                        &format!(
                            "Failed to check if class {} exists in {}",
                            name,
                            unit.get_name()
                        ),
                        Severity::Error,
                        &ex,
                    );
                }
                Err(ex) => rethrow(ex),
            }
        }

        false
    }

    pub fn get_own_classes(self) -> Vec<StructClass> {
        let units = self.with(|d| d.units.clone());
        let mut classes = Vec::new();
        for unit in units {
            if !unit.is_own() {
                continue;
            }
            for name in unit.get_class_names() {
                match self.get_class(&name) {
                    Some(cl) => classes.push(cl),
                    None => jthrow!(NullPointerException; "Could not find class {}", name),
                }
            }
        }
        classes
    }

    pub fn reload_context(self) {
        let units = self.with_mut(|d| {
            d.classes.clear();
            d.units_by_class_name.clear();
            d.abstract_names.clear();

            let units = d.units.clone();
            d.units.clear();
            d.lazy_units.clear();
            units
        });
        for unit in units {
            if unit.is_root() {
                unit.clear();
                self.with_mut(|d| d.units.push(unit));
                if unit.is_lazy() {
                    self.with_mut(|d| d.lazy_units.push(unit));
                }
                self.init_unit(unit);
            }
        }
    }

    /// Decompiles every own unit; each finished class goes to `sink` in place
    /// of the Java's `IResultSaver`.
    pub fn save_context(self, sink: &mut dyn FnMut(DecompiledClass)) {
        let units = self.with(|d| d.units.clone());
        for unit in units {
            if unit.is_own() {
                let result =
                    try_catch(|| unit.save(&|name: &str| self.get_class(name), &mut *sink));
                match result {
                    Ok(()) => {}
                    Err(ex) if is_io_exception(&ex) => {
                        DecompilerContext::get_logger().write_message_ex(
                            &format!("Failed to save data for context unit{}", unit.get_name()),
                            Severity::Error,
                            &ex,
                        );
                    }
                    Err(ex) => rethrow(ex),
                }
            }
        }
    }

    pub fn is_jar_file(file: &Path) -> bool {
        if !file.is_file() {
            return false;
        }
        let name = file
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if name.ends_with(".jar") || name.ends_with(".zip") {
            return true;
        }
        if name.ends_with(".class") {
            return false;
        }
        let scan = || -> std::io::Result<bool> {
            let mut channel = std::fs::File::open(file)?;
            let size = channel.metadata()?.len();
            if size < 22 {
                return Ok(false);
            }
            let buffer_size = std::cmp::min(size & !3, 1024) as usize;
            channel.seek(SeekFrom::Start(size - buffer_size as u64))?;
            let mut buffer = vec![0u8; buffer_size];
            channel.read_exact(&mut buffer)?;
            let mut pos = buffer_size as i64 - 22;
            while pos >= 0 {
                let p = pos as usize;
                let word =
                    u32::from_le_bytes([buffer[p], buffer[p + 1], buffer[p + 2], buffer[p + 3]]);
                if word == 0x06054b50 {
                    return Ok(true);
                }
                pos -= 1;
            }
            Ok(false)
        };
        match scan() {
            Ok(found) => found,
            Err(e) => {
                DecompilerContext::get_logger().write_message_ex(
                    &format!(
                        "Could not determine if {} contains a JAR file",
                        file.display()
                    ),
                    Severity::Warn,
                    &io_caught(&e),
                );
                false
            }
        }
    }

    /// Java: addSpace(File, boolean)
    pub fn add_space_file(self, file: &Path, is_own: bool) {
        if file.is_dir() {
            self.add_space(Rc::new(DirectoryContextSource::new(file)), is_own);
        } else if Self::is_jar_file(file) {
            let result = try_catch(|| {
                self.add_space(Rc::new(JarContextSource::new(file)), is_own);
            });
            if let Err(ex) = result {
                Self::invalid_file(ex, format!("Invalid archive {}", file.display()));
            }
        } else {
            let result = try_catch(|| {
                self.add_space(Rc::new(SingleFileContextSource::new(file)), is_own);
            });
            if let Err(ex) = result {
                Self::invalid_file(ex, format!("Invalid file {}", file.display()));
            }
        }
    }

    fn invalid_file(ex: Caught, message: String) -> ! {
        if !is_io_exception(&ex) {
            rethrow(ex);
        }
        DecompilerContext::get_logger().write_message_ex(&message, Severity::Error, &ex);
        jthrow!(UncheckedIOException; "{}", message)
    }

    /// Java: addSpace(IContextSource, boolean)
    pub fn add_space(self, source: Rc<dyn IContextSource>, is_own: bool) {
        self.add_space_root(source, is_own, true);
    }

    /// Java: addSpace(IContextSource, boolean, boolean)
    fn add_space_root(self, source: Rc<dyn IContextSource>, is_own: bool, is_root: bool) {
        let decompiled_data = self.with(|d| d.decompiled_data.clone());
        let unit = ContextUnit::new(source, is_own, is_root, decompiled_data);
        self.with_mut(|d| d.units.push(unit));
        if unit.is_lazy() {
            self.with_mut(|d| d.lazy_units.push(unit));
        }
        self.init_unit(unit);
    }

    pub fn get_plugin_context(self) -> PluginContext {
        self.with(|d| d.plugin_context)
    }

    fn init_unit(self, unit: ContextUnit) {
        DecompilerContext::get_logger().write_message(
            &format!("Scanning classes from {}", unit.get_name()),
            Severity::Info,
        );
        let is_own = unit.is_own();
        for clazz in unit.get_class_names() {
            let existing = self.with_mut(|d| {
                let existing = d.units_by_class_name.get(&clazz).copied();
                if existing.is_none() {
                    d.units_by_class_name.put(clazz.clone(), unit);
                }
                existing
            });
            if let Some(existing) = existing {
                if !is_own || existing.is_own() {
                    continue;
                }

                let replaced = self.with_mut(|d| {
                    if d.units_by_class_name.get(&clazz).copied() == Some(existing) {
                        d.units_by_class_name.put(clazz.clone(), unit);
                        true
                    } else {
                        false
                    }
                });
                if !replaced {
                    continue;
                }
            }

            DecompilerContext::get_logger().write_message(&format!("    {clazz}"), Severity::Trace);
            if is_own {
                self.get_class(&clazz);
            }
        }

        for child in unit.get_child_contexts() {
            self.add_space_root(child, is_own, false);
        }
    }

    /// `valclass instanceof refclass`.
    pub fn instance_of(self, valclass: &str, refclass: &str) -> bool {
        if valclass == refclass {
            return true;
        }

        let cl = self.get_class(valclass);
        let super_class = cl.and_then(super_class_name_of);
        if (cl.is_none() || super_class.is_none()) && refclass == "java/lang/Object" {
            return true;
        }

        let Some(cl) = cl else {
            return false;
        };

        if let Some(super_class) = &super_class {
            if self.instance_of(super_class, refclass) {
                return true;
            }
        }

        for iface in cl.get_interfaces() {
            let name = cl.get_pool().get_primitive_constant(iface).get_string();
            if self.instance_of(&name, refclass) {
                return true;
            }
        }

        false
    }

    /// The join of both classes on the lattice: a common ancestor other than `java/lang/Object`.
    pub fn find_common_ancestor(self, class_a: &str, class_b: &str) -> Option<StructClass> {
        let a = self.get_class(class_a);
        let b = self.get_class(class_b);

        let (Some(a), Some(_)) = (a, b) else {
            return None;
        };

        if self.instance_of(class_b, class_a) {
            return Some(a);
        }

        if let Some(super_class) = super_class_name_of(a) {
            let cl = self.find_common_ancestor(&super_class, class_b);
            if let Some(cl) = cl {
                if qualified_name_of(cl) != "java/lang/Object" {
                    return Some(cl);
                }
            }
        }

        for iface in a.get_interfaces() {
            let name = a.get_pool().get_primitive_constant(iface).get_string();
            let cl = self.find_common_ancestor(&name, class_b);
            if let Some(cl) = cl {
                if qualified_name_of(cl) != "java/lang/Object" {
                    return Some(cl);
                }
            }
        }

        None
    }

    pub fn load_abstract_metadata(self, string: &str) {
        for line in java_split(string, '\n') {
            let pts = java_split(line, ' ');
            if pts.len() < 4 {
                continue;
            }
            let desc = GenericMain::parse_method_signature(pts[2]);
            let mut params = Vec::new();
            for x in 0..pts.len() - 3 {
                for _ in 0..param_stack_size(&desc, x) {
                    params.push(pts[x + 3].to_string());
                }
            }
            self.with_mut(|d| {
                d.abstract_names
                    .put(format!("{} {} {}", pts[0], pts[1], pts[2]), params)
            });
        }
    }

    pub fn rename_abstract_parameter(
        self,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
        index: i32,
        default: Option<&str>,
    ) -> Option<String> {
        let key = format!("{class_name} {method_name} {descriptor}");
        let params = self.with(|d| d.abstract_names.get_cloned(&key));
        match params {
            Some(params) if index < params.len() as i32 => Some(params[index as usize].clone()),
            _ => default.map(str::to_string),
        }
    }

    pub fn clear(self) {
        self.get_plugin_context().close();

        let units = self.with(|d| d.units.clone());
        for unit in units {
            if let Err(ex) = try_catch(|| unit.close()) {
                DecompilerContext::get_logger().write_message_ex(
                    &format!("Failed to close context unit {}", unit.get_name()),
                    Severity::Error,
                    &ex,
                );
            }
        }
        self.with_mut(|d| {
            d.units.clear();
            d.units_by_class_name.clear();
            d.classes.clear();
        });
    }

    pub fn get_units(self) -> Vec<ContextUnit> {
        self.with(|d| d.units.clone())
    }
}

fn param_stack_size(desc: &GenericMethodDescriptor, x: usize) -> i32 {
    desc.parameter_types[x].stack_size
}

/// `cl.qualifiedName`.
pub(crate) fn qualified_name_of(cl: StructClass) -> String {
    cl.with(|d| d.qualified_name.clone())
}

/// `cl.superClass == null ? null : cl.superClass.getString()`.
pub(crate) fn super_class_name_of(cl: StructClass) -> Option<String> {
    cl.with(|d| d.super_class.clone()).map(|s| s.get_string())
}

const IO_EXCEPTIONS: &[&str] = &[
    "IOException",
    "EOFException",
    "UTFDataFormatException",
    "FileNotFoundException",
    "NoSuchFileException",
    "AccessDeniedException",
    "FileSystemException",
    "ZipException",
    "CharacterCodingException",
    "MalformedInputException",
];

/// `catch (IOException e)`: the class or one of its JDK subclasses.
pub(crate) fn is_io_exception(caught: &Caught) -> bool {
    IO_EXCEPTIONS.iter().any(|class| caught.is(class))
}

/// A `java.io.IOException` standing for a Rust I/O error.
pub(crate) fn io_caught(e: &std::io::Error) -> Caught {
    let exception = JavaException {
        class: "IOException",
        message: e.to_string(),
    };
    Caught {
        message: exception.to_string(),
        exception: Some(exception),
        location: String::new(),
    }
}

/// `String.split` on a single literal character: trailing empty strings are
/// dropped, and a string without the separator splits to itself.
pub(crate) fn java_split(s: &str, separator: char) -> Vec<&str> {
    if !s.contains(separator) {
        return vec![s];
    }
    let mut parts: Vec<&str> = s.split(separator).collect();
    while parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    parts
}
