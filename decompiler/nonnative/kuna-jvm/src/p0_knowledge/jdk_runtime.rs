//! Replaces util/JrtFinder + ModuleBasedContextSource.
//!
//! Vineflower reads the Java standard library through the running JVM's `jrt:`
//! file system. Without a JVM, [`JdkRuntime`] locates a JDK home and reads its
//! modules directly: from the `lib/modules` jimage container ([`JImage`]), else
//! from `jmods/*.jmod`, else (Java 8) from the jars in `jre/lib` and `lib`.
//! What gets registered mirrors `JrtFinder.addRuntime`: one root
//! [`JavaRuntimeContextSource`] whose child contexts are one
//! [`JavaRuntimeModuleContextSource`] per module, each listing its entries in
//! the order `Files.walk` over `jrt:/modules/<module>` produces and reading
//! class bytes on demand. Like the originals, the module sources are not
//! `is_lazy()`.
//!
//! That walk order is the image's stored (sorted) order, except that the root
//! reads `module-info.class` before the walk, which makes the `jrt:` reader
//! list it first; a JDK whose `lib/jrt-fs.jar` predates the duplicate guard in
//! `ImageReader.SharedImageReader.handleModulesSubTree` (JDK 17 and older)
//! lists it a second time at its sorted place. Vineflower reads an explicit
//! home through that home's own `jrt-fs.jar`, so the port checks the same jar.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::context_source::{
    java_absolute_path, java_file_path, read_class_info, Entries, Entry, IContextSource,
    JarContextSource, CLASS_SUFFIX,
};
use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_logger::Severity;
use crate::jthrow;
use crate::rt::jstring::decode_modified_utf8;
use crate::struct_context::StructContext;
use crate::zip_reader::{inflate, ZipReader, MAX_ARRAY_SIZE};

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn log(message: &str, severity: Severity) {
    DecompilerContext::get_logger().write_message(message, severity);
}

/// Locates a JDK and registers its modules with a `StructContext`.
pub struct JdkRuntime;

impl JdkRuntime {
    /// `JrtFinder.CURRENT`.
    pub const CURRENT: &'static str = "current";

    /// The JDK home an `include-runtime` value names: `"1"`/`"current"` search
    /// `JAVA_HOME`, then `java` on `PATH`, then well-known install directories
    /// (newest first); a path names a home; `""` and `"0"` name none.
    pub fn locate(spec: &str) -> Option<PathBuf> {
        if spec.is_empty() || spec.eq_ignore_ascii_case("0") {
            return None;
        }
        if spec.eq_ignore_ascii_case(Self::CURRENT) || spec.eq_ignore_ascii_case("1") {
            return Self::locate_current();
        }
        let home = PathBuf::from(spec);
        Self::is_java_home(&home).then_some(home)
    }

    /// Whether `home` holds a readable runtime (`lib/modules`, `jmods/`, or an `rt.jar`).
    pub fn is_java_home(home: &Path) -> bool {
        home.join("lib/modules").is_file()
            || home.join("jmods/java.base.jmod").is_file()
            || home.join("lib/rt.jar").is_file()
            || home.join("jre/lib/rt.jar").is_file()
    }

    /// What `Fernflower`'s constructor does with the `include-runtime` option.
    pub fn add_runtime(struct_context: StructContext, spec: &str) -> bool {
        if spec.is_empty() || spec.eq_ignore_ascii_case("0") {
            return false;
        }
        if !(spec.eq_ignore_ascii_case(Self::CURRENT) || spec.eq_ignore_ascii_case("1")) {
            return Self::add_runtime_home(struct_context, Path::new(spec));
        }
        let Some(home) = Self::locate(spec) else {
            log(
                "Failed to open current java runtime for inspection",
                Severity::Error,
            );
            return false;
        };
        if !Self::is_modular_home(&home) {
            if Self::add_legacy(struct_context, &home) {
                return true;
            }
            log(
                "Failed to open current java runtime for inspection",
                Severity::Error,
            );
            return false;
        }
        match JavaRuntimeContextSource::open(&home, Self::CURRENT) {
            Ok(source) => {
                struct_context.add_space(Rc::new(source), false);
                true
            }
            Err(e) => {
                log(
                    &format!("Failed to open current java runtime for inspection: {e}"),
                    Severity::Error,
                );
                false
            }
        }
    }

    /// Java: JrtFinder.addRuntime(StructContext, File)
    pub fn add_runtime_home(struct_context: StructContext, java_home: &Path) -> bool {
        let display = java_file_path(java_home);
        if Self::is_modular_home(java_home) {
            return match JavaRuntimeContextSource::open(java_home, &java_absolute_path(&display)) {
                Ok(source) => {
                    struct_context.add_space(Rc::new(source), false);
                    true
                }
                Err(e) => {
                    log(
                        &format!("Failed to open java runtime at {display}: {e}"),
                        Severity::Error,
                    );
                    false
                }
            };
        } else if java_home.exists() && Self::add_legacy(struct_context, java_home) {
            return true;
        }
        log(
            &format!("Unable to detect a java runtime at {display}"),
            Severity::Error,
        );
        false
    }

    /// The context sources of the runtime at `home`, without the root: one per
    /// module, or one per jar of a Java 8 runtime.
    pub fn sources(home: &Path) -> Vec<Rc<dyn IContextSource>> {
        if Self::is_modular_home(home) {
            let identifier = java_absolute_path(&java_file_path(home));
            return match JavaRuntimeContextSource::open(home, &identifier) {
                Ok(root) => root.get_entries().child_contexts().to_vec(),
                Err(_) => Vec::new(),
            };
        }
        Self::legacy_jars(home)
            .iter()
            .filter_map(|jar| JarContextSource::open(jar, "").ok())
            .map(|source| Rc::new(source) as Rc<dyn IContextSource>)
            .collect()
    }

    fn is_modular_home(home: &Path) -> bool {
        home.join("lib/jrt-fs.jar").is_file()
            || home.join("lib/modules").is_file()
            || home.join("jmods").is_dir()
    }

    fn legacy_jars(home: &Path) -> Vec<PathBuf> {
        let mut jars = Vec::new();
        for dir in ["jre/lib", "lib"] {
            for file in sorted_children(&home.join(dir)) {
                let is_jar = file
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().ends_with(".jar"));
                if is_jar && file.is_file() {
                    jars.push(file);
                }
            }
        }
        jars
    }

    fn add_legacy(struct_context: StructContext, home: &Path) -> bool {
        let mut any_added = false;
        for lib in Self::legacy_jars(home) {
            match JarContextSource::open(&lib, "") {
                Ok(source) => struct_context.add_space(Rc::new(source), false),
                Err(_) => {
                    let message = format!("Invalid archive {}", java_file_path(&lib));
                    log(&message, Severity::Error);
                    jthrow!(UncheckedIOException; "{}", message);
                }
            }
            any_added = true;
        }
        any_added
    }

    fn locate_current() -> Option<PathBuf> {
        if let Some(home) = std::env::var_os("JAVA_HOME").filter(|h| !h.is_empty()) {
            let home = PathBuf::from(home);
            if Self::is_java_home(&home) {
                return Some(home);
            }
        }
        if let Some(home) = Self::home_of_java_on_path() {
            return Some(home);
        }
        let mut best: Option<(Vec<u64>, PathBuf)> = None;
        for home in Self::well_known_homes() {
            let key = (version_key(&home), home);
            if best.as_ref().is_none_or(|b| key > *b) {
                best = Some(key);
            }
        }
        best.map(|(_, home)| home)
    }

    fn home_of_java_on_path() -> Option<PathBuf> {
        let exe = if cfg!(windows) { "java.exe" } else { "java" };
        let path = std::env::var_os("PATH")?;
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(exe);
            if !candidate.is_file() {
                continue;
            }
            let Ok(real) = std::fs::canonicalize(&candidate) else {
                continue;
            };
            if let Some(home) = real.parent().and_then(Path::parent) {
                if Self::is_java_home(home) {
                    return Some(home.to_path_buf());
                }
            }
        }
        None
    }

    fn well_known_homes() -> BTreeSet<PathBuf> {
        let mut roots: Vec<PathBuf> = [
            "/usr/lib/jvm",
            "/usr/lib64/jvm",
            "/usr/java",
            "/opt/java",
            "/Library/Java/JavaVirtualMachines",
        ]
        .iter()
        .map(PathBuf::from)
        .collect();
        if let Some(user) = std::env::var_os("HOME") {
            let user = PathBuf::from(user);
            roots.push(user.join("Library/Java/JavaVirtualMachines"));
            roots.push(user.join(".sdkman/candidates/java"));
        }
        let mut homes = BTreeSet::new();
        for root in roots {
            for dir in sorted_children(&root) {
                for candidate in [dir.join("Contents/Home"), dir] {
                    if Self::is_java_home(&candidate) {
                        homes.insert(std::fs::canonicalize(&candidate).unwrap_or(candidate));
                        break;
                    }
                }
            }
        }
        homes
    }
}

fn sorted_children(dir: &Path) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut children: Vec<PathBuf> = read.filter_map(|e| e.ok().map(|e| e.path())).collect();
    children.sort();
    children
}

fn version_key(home: &Path) -> Vec<u64> {
    let release = std::fs::read_to_string(home.join("release")).unwrap_or_default();
    let text = release
        .lines()
        .find_map(|l| l.strip_prefix("JAVA_VERSION="))
        .map(|v| v.trim_matches('"').to_string())
        .unwrap_or_else(|| {
            let named = if home.ends_with("Contents/Home") {
                home.parent().and_then(Path::parent).unwrap_or(home)
            } else {
                home
            };
            named
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        });
    let mut parts: Vec<u64> = text
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap_or(u64::MAX))
        .collect();
    if parts.len() > 1 && parts[0] == 1 {
        parts.remove(0);
    }
    parts
}

/// `JrtFinder.JavaRuntimeContextSource`: the root source of a modular runtime.
pub struct JavaRuntimeContextSource {
    identifier: String,
    image: Rc<RuntimeImage>,
}

impl JavaRuntimeContextSource {
    /// Java: JavaRuntimeContextSource(File); `None` is the current runtime.
    pub fn new(java_home: Option<&Path>) -> JavaRuntimeContextSource {
        let opened = match java_home {
            None => match JdkRuntime::locate(JdkRuntime::CURRENT) {
                Some(home) => Self::open(&home, JdkRuntime::CURRENT),
                None => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "no Java runtime found",
                )),
            },
            Some(home) => Self::open(home, &java_absolute_path(&java_file_path(home))),
        };
        match opened {
            Ok(source) => source,
            Err(e) => jthrow!(IOException; "{}", e),
        }
    }

    /// Open the runtime at `home`; `identifier` is what `get_name` reports.
    pub fn open(home: &Path, identifier: &str) -> io::Result<JavaRuntimeContextSource> {
        Ok(JavaRuntimeContextSource {
            identifier: identifier.to_string(),
            image: Rc::new(RuntimeImage::open(home)?),
        })
    }

    /// Whether the modules come from `lib/modules` (else from `jmods/`).
    pub fn is_jimage(&self) -> bool {
        matches!(self.image.store, Store::Image(_))
    }
}

impl IContextSource for JavaRuntimeContextSource {
    fn get_name(&self) -> String {
        format!("Java runtime {}", self.identifier)
    }

    fn get_entries(&self) -> Entries {
        let modules = match self.image.module_names() {
            Ok(modules) => modules,
            Err(_) => {
                log(
                    &format!("Failed to read modules from runtime {}", self.identifier),
                    Severity::Error,
                );
                return Entries::EMPTY();
            }
        };
        let mut children: Vec<Rc<dyn IContextSource>> = Vec::new();
        for module in modules {
            let Ok(bytes) = self.image.read(&module, "module-info.class") else {
                continue;
            };
            let Ok(info) = read_class_info(&bytes) else {
                continue;
            };
            let Some((name, version)) = info.module else {
                continue;
            };
            let name_and_version = match version {
                Some(v) => format!("{name}@{v}"),
                None => name,
            };
            children.push(Rc::new(JavaRuntimeModuleContextSource {
                image: self.image.clone(),
                module,
                name_and_version,
            }));
        }
        Entries::new_with_children(Vec::new(), Vec::new(), Vec::new(), children)
    }

    fn get_input_stream(&self, _resource: &str) -> Option<Vec<u8>> {
        None
    }

    fn close(&self) {
        self.image.close();
    }
}

/// `JrtFinder.JavaRuntimeModuleContextSource` over `ModuleBasedContextSource`.
pub struct JavaRuntimeModuleContextSource {
    image: Rc<RuntimeImage>,
    module: String,
    name_and_version: String,
}

impl JavaRuntimeModuleContextSource {
    /// The module's directory name in the runtime image.
    pub fn get_module(&self) -> &str {
        &self.module
    }

    /// `entryNames()`: the module's paths relative to its root, in walk order.
    pub fn entry_names(&self) -> io::Result<Vec<String>> {
        self.image.walk(&self.module)
    }
}

impl IContextSource for JavaRuntimeModuleContextSource {
    fn get_name(&self) -> String {
        format!("module {}", self.name_and_version)
    }

    fn get_entries(&self) -> Entries {
        let mut class_names = Vec::new();
        let mut directory_names = Vec::new();
        let mut other_entries = Vec::new();
        match self.entry_names() {
            Ok(names) => {
                for name in names {
                    if let Some(dir) = name.strip_suffix('/') {
                        directory_names.push(dir.to_string());
                    } else if let Some(class) = name.strip_suffix(CLASS_SUFFIX) {
                        class_names.push(Entry::at_base(class));
                    } else {
                        other_entries.push(Entry::at_base(&name));
                    }
                }
            }
            Err(_) => log(
                &format!("Failed to list contents of {}", self.get_name()),
                Severity::Error,
            ),
        }
        Entries::new(class_names, directory_names, other_entries)
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        match self.image.read(&self.module, resource) {
            Ok(bytes) => Some(bytes),
            Err(e) => jthrow!(IOException; "{}", e),
        }
    }
}

impl std::fmt::Debug for JavaRuntimeContextSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

impl std::fmt::Debug for JavaRuntimeModuleContextSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}

enum Store {
    Image(JImage),
    Jmods(Vec<Jmod>),
}

struct Jmod {
    module: String,
    zip: ZipReader,
}

struct RuntimeImage {
    store: Store,
    closed: Cell<bool>,
    module_info_twice: bool,
}

const JMOD_CLASSES: &str = "classes/";

impl RuntimeImage {
    fn open(home: &Path) -> io::Result<RuntimeImage> {
        let module_info_twice = Self::jrt_lists_module_info_twice(home);
        let modules = home.join("lib/modules");
        let jimage_error = if modules.is_file() {
            match Self::open_jimage(&modules) {
                Ok(image) => {
                    return Ok(RuntimeImage {
                        store: Store::Image(image),
                        closed: Cell::new(false),
                        module_info_twice,
                    })
                }
                Err(e) => Some(e),
            }
        } else {
            None
        };
        let jmods = match Self::open_jmods(&home.join("jmods")) {
            Ok(jmods) => jmods,
            Err(e) => return Err(jimage_error.unwrap_or(e)),
        };
        if jmods.is_empty() {
            return Err(jimage_error.unwrap_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{}: no lib/modules or jmods", home.display()),
                )
            }));
        }
        Ok(RuntimeImage {
            store: Store::Jmods(jmods),
            closed: Cell::new(false),
            module_info_twice,
        })
    }

    /// Whether the home's `jrt-fs.jar` lacks the `nodes.containsKey` guard.
    fn jrt_lists_module_info_twice(home: &Path) -> bool {
        let Ok(jar) = ZipReader::open(&home.join("lib/jrt-fs.jar")) else {
            return false;
        };
        match jar.read("jdk/internal/jimage/ImageReader$SharedImageReader.class") {
            Ok(Some(class)) => !class.windows(11).any(|w| w == b"containsKey"),
            _ => false,
        }
    }

    /// Opens the image and reads `java.base/module-info.class`, so an image
    /// whose compression is unsupported fails here, by name.
    fn open_jimage(path: &Path) -> io::Result<JImage> {
        let image = JImage::open(path)?;
        if let Some(loc) = image.find_location("/java.base/module-info.class")? {
            image.get_resource(&loc)?;
        }
        Ok(image)
    }

    fn open_jmods(dir: &Path) -> io::Result<Vec<Jmod>> {
        let mut jmods = Vec::new();
        for path in sorted_children(dir) {
            if path.extension().is_none_or(|e| e != "jmod") || !path.is_file() {
                continue;
            }
            let zip = ZipReader::open(&path)?;
            let Some(entry) = zip.get_entry_exact("classes/module-info.class") else {
                continue;
            };
            let info = read_class_info(&zip.read_entry(entry)?)?;
            if let Some((module, _)) = info.module {
                jmods.push(Jmod { module, zip });
            }
        }
        jmods.sort_by(|a, b| utf16_cmp(&a.module, &b.module));
        Ok(jmods)
    }

    fn ensure_open(&self) {
        if self.closed.get() {
            jthrow!(ClosedFileSystemException; "");
        }
    }

    fn close(&self) {
        self.closed.set(true);
        if let Store::Image(image) = &self.store {
            image.close();
        }
    }

    fn module_names(&self) -> io::Result<Vec<String>> {
        self.ensure_open();
        match &self.store {
            Store::Image(image) => image.module_names(),
            Store::Jmods(jmods) => Ok(jmods.iter().map(|j| j.module.clone()).collect()),
        }
    }

    fn read(&self, module: &str, resource: &str) -> io::Result<Vec<u8>> {
        self.ensure_open();
        let missing = || {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("/modules/{module}/{resource}"),
            )
        };
        let resource = normalize_resource(resource).ok_or_else(missing)?;
        match &self.store {
            Store::Image(image) => image
                .read(&format!("/{module}/{resource}"))?
                .ok_or_else(missing),
            Store::Jmods(jmods) => {
                let jmod = jmods
                    .iter()
                    .find(|j| j.module == module)
                    .ok_or_else(missing)?;
                match jmod
                    .zip
                    .get_entry_exact(&format!("{JMOD_CLASSES}{resource}"))
                {
                    Some(entry) if !entry.is_directory() => jmod.zip.read_entry(entry),
                    _ => Err(missing()),
                }
            }
        }
    }

    /// The `jrt:` walk: stored order with `module-info.class` opened first.
    fn walk(&self, module: &str) -> io::Result<Vec<String>> {
        let mut names = self.stored_walk(module)?;
        if let Some(p) = names.iter().position(|n| n == "module-info.class") {
            let first = if self.module_info_twice {
                names[p].clone()
            } else {
                names.remove(p)
            };
            names.insert(1, first);
        }
        Ok(names)
    }

    fn stored_walk(&self, module: &str) -> io::Result<Vec<String>> {
        self.ensure_open();
        match &self.store {
            Store::Image(image) => Ok(image
                .walk_module(module)?
                .into_iter()
                .map(|(name, _)| name)
                .collect()),
            Store::Jmods(jmods) => {
                let jmod = jmods
                    .iter()
                    .find(|j| j.module == module)
                    .ok_or_else(|| invalid(format!("no module {module}")))?;
                let mut root = Tree::default();
                for entry in jmod.zip.entries() {
                    if let Some(rel) = entry.name.strip_prefix(JMOD_CLASSES) {
                        root.insert(rel, entry.is_directory());
                    }
                }
                let mut out = vec![String::new()];
                root.walk("", &mut out);
                Ok(out)
            }
        }
    }
}

fn normalize_resource(resource: &str) -> Option<String> {
    if resource.starts_with('/') {
        return None;
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in resource.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            p => parts.push(p),
        }
    }
    Some(parts.join("/"))
}

fn utf16_cmp(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

#[derive(PartialEq, Eq)]
struct JKey(String);

impl Ord for JKey {
    fn cmp(&self, other: &Self) -> Ordering {
        utf16_cmp(&self.0, &other.0)
    }
}

impl PartialOrd for JKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A jmod's `classes/` section as the sorted directory tree a jimage stores.
#[derive(Default)]
struct Tree {
    children: BTreeMap<JKey, Option<Tree>>,
}

impl Tree {
    fn insert(&mut self, path: &str, is_dir: bool) {
        let mut node = self;
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        for (i, part) in parts.iter().enumerate() {
            let leaf = i + 1 == parts.len() && !is_dir;
            let slot = node.children.entry(JKey(part.to_string())).or_insert(None);
            if leaf {
                return;
            }
            node = slot.get_or_insert_with(Tree::default);
        }
    }

    fn walk(&self, prefix: &str, out: &mut Vec<String>) {
        for (key, sub) in &self.children {
            let name = if prefix.is_empty() {
                key.0.clone()
            } else {
                format!("{prefix}/{}", key.0)
            };
            out.push(name.clone());
            if let Some(sub) = sub {
                sub.walk(&name, out);
            }
        }
    }
}

const IMAGE_MAGIC: u32 = 0xCAFE_DADA;
const IMAGE_HEADER_SIZE: u64 = 28;
const HASH_MULTIPLIER: i32 = 0x0100_0193;
const ATTRIBUTE_MODULE: usize = 1;
const ATTRIBUTE_PARENT: usize = 2;
const ATTRIBUTE_BASE: usize = 3;
const ATTRIBUTE_EXTENSION: usize = 4;
const ATTRIBUTE_OFFSET: usize = 5;
const ATTRIBUTE_COMPRESSED: usize = 6;
const ATTRIBUTE_UNCOMPRESSED: usize = 7;
const ATTRIBUTE_COUNT: usize = 8;
const COMPRESSED_HEADER_SIZE: usize = 29;
const COMPRESSED_MAGIC: u32 = 0xCAFE_FAFA;
const MAX_COMPRESSION_LAYERS: usize = 16;

/// One decoded `ImageLocation`.
#[derive(Clone, Copy, Debug)]
pub struct ImageLocation {
    attributes: [u64; ATTRIBUTE_COUNT],
}

impl ImageLocation {
    pub fn get_content_offset(&self) -> u64 {
        self.attributes[ATTRIBUTE_OFFSET]
    }

    pub fn get_compressed_size(&self) -> u64 {
        self.attributes[ATTRIBUTE_COMPRESSED]
    }

    pub fn get_uncompressed_size(&self) -> u64 {
        self.attributes[ATTRIBUTE_UNCOMPRESSED]
    }
}

/// A reader for the JDK 9+ `lib/modules` jimage container (`jdk.internal.jimage`).
///
/// The index (header, redirect, offsets, locations, strings) is read once;
/// resources are read on demand. Uncompressed resources and the JDK's two
/// compression plugins (`zip`, `compact-cp`) are supported; any other plugin
/// is reported as an error naming it.
pub struct JImage {
    path: PathBuf,
    file: RefCell<Option<File>>,
    file_len: u64,
    big_endian: bool,
    redirect: Vec<i32>,
    offsets: Vec<u32>,
    locations: Vec<u8>,
    strings: Vec<u8>,
    index_size: u64,
}

fn read_exact_at(file: &File, buf: &mut [u8], pos: u64) -> io::Result<()> {
    let mut f = file;
    f.seek(SeekFrom::Start(pos))?;
    f.read_exact(buf)
}

/// `ImageStringsReader.hashCode(name, seed)`.
fn image_hash(name: &str, seed: i32) -> i32 {
    let mut seed = seed;
    for unit in name.encode_utf16() {
        let mut uch = unit as i32;
        if uch & !0x7F != 0 {
            let mut buffer = [0u8; 8];
            let mut mask: i32 = !0x3F;
            let mut n = 0usize;
            loop {
                buffer[n] = (0x80 | (uch & 0x3F)) as u8;
                n += 1;
                uch >>= 6;
                mask >>= 1;
                if uch & mask == 0 {
                    break;
                }
            }
            buffer[n] = ((mask << 1) | uch) as u8;
            loop {
                seed = seed.wrapping_mul(HASH_MULTIPLIER) ^ (buffer[n] as i32);
                if n == 0 {
                    break;
                }
                n -= 1;
            }
        } else {
            seed = seed.wrapping_mul(HASH_MULTIPLIER) ^ uch;
        }
    }
    seed & 0x7FFF_FFFF
}

impl JImage {
    pub fn open(path: &Path) -> io::Result<JImage> {
        let file = File::open(path)?;
        let file_len = file.metadata()?.len();
        let not_image = || invalid(format!("\"{}\" is not an image file", path.display()));
        if file_len < IMAGE_HEADER_SIZE {
            return Err(not_image());
        }
        let mut header = [0u8; IMAGE_HEADER_SIZE as usize];
        read_exact_at(&file, &mut header, 0)?;
        let word = |i: usize, be: bool| {
            let b = [
                header[i * 4],
                header[i * 4 + 1],
                header[i * 4 + 2],
                header[i * 4 + 3],
            ];
            if be {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            }
        };
        let big_endian = if word(0, false) == IMAGE_MAGIC {
            false
        } else if word(0, true) == IMAGE_MAGIC {
            true
        } else {
            return Err(not_image());
        };
        let version = word(1, big_endian);
        if version >> 16 != 1 || version & 0xFFFF != 0 {
            return Err(invalid(format!(
                "The image file \"{}\" is not the correct version. Major: {}. Minor: {}",
                path.display(),
                version >> 16,
                version & 0xFFFF
            )));
        }
        let table_length = word(4, big_endian) as u64;
        let locations_size = word(5, big_endian) as u64;
        let strings_size = word(6, big_endian) as u64;
        let index_size = IMAGE_HEADER_SIZE + table_length * 8 + locations_size + strings_size;
        if index_size > file_len {
            return Err(invalid(format!(
                "\"{}\" has a truncated index",
                path.display()
            )));
        }
        let mut index = vec![0u8; (index_size - IMAGE_HEADER_SIZE) as usize];
        read_exact_at(&file, &mut index, IMAGE_HEADER_SIZE)?;
        let table = table_length as usize;
        let u32_at = |off: usize| {
            let b = [index[off], index[off + 1], index[off + 2], index[off + 3]];
            if big_endian {
                u32::from_be_bytes(b)
            } else {
                u32::from_le_bytes(b)
            }
        };
        let redirect = (0..table).map(|i| u32_at(i * 4) as i32).collect();
        let offsets = (0..table).map(|i| u32_at(table * 4 + i * 4)).collect();
        let loc_start = table * 8;
        let str_start = loc_start + locations_size as usize;
        let locations = index[loc_start..str_start].to_vec();
        let strings = index[str_start..].to_vec();
        Ok(JImage {
            path: path.to_path_buf(),
            file: RefCell::new(Some(file)),
            file_len,
            big_endian,
            redirect,
            offsets,
            locations,
            strings,
            index_size,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_big_endian(&self) -> bool {
        self.big_endian
    }

    /// The number of locations (resources and directories) in the index.
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn close(&self) {
        self.file.borrow_mut().take();
    }

    fn order_u32(&self, b: &[u8]) -> u32 {
        let a = [b[0], b[1], b[2], b[3]];
        if self.big_endian {
            u32::from_be_bytes(a)
        } else {
            u32::from_le_bytes(a)
        }
    }

    fn order_u64(&self, b: &[u8]) -> u64 {
        let mut a = [0u8; 8];
        a.copy_from_slice(&b[..8]);
        if self.big_endian {
            u64::from_be_bytes(a)
        } else {
            u64::from_le_bytes(a)
        }
    }

    fn string_bytes(&self, offset: u64) -> io::Result<&[u8]> {
        let bad = || invalid(format!("bad jimage string offset {offset}"));
        let start = usize::try_from(offset).map_err(|_| bad())?;
        let tail = self.strings.get(start..).ok_or_else(bad)?;
        let end = tail.iter().position(|&b| b == 0).ok_or_else(bad)?;
        Ok(&tail[..end])
    }

    /// `ImageStringsReader.get(offset)`.
    pub fn get_string(&self, offset: u64) -> io::Result<Cow<'_, str>> {
        let bytes = self.string_bytes(offset)?;
        if bytes.is_ascii() {
            Ok(Cow::Borrowed(
                std::str::from_utf8(bytes).unwrap_or_default(),
            ))
        } else {
            Ok(Cow::Owned(decode_modified_utf8(bytes)))
        }
    }

    /// `ImageLocation.readFrom(reader, offset)`: decode the attributes at a locations offset.
    pub fn get_location(&self, offset: u32) -> io::Result<ImageLocation> {
        let mut attributes = [0u64; ATTRIBUTE_COUNT];
        let b = &self.locations;
        let mut pos = offset as usize;
        while pos < b.len() {
            let data = b[pos];
            pos += 1;
            if data <= 0x7 {
                break;
            }
            let kind = (data >> 3) as usize;
            if kind >= ATTRIBUTE_COUNT {
                return Err(invalid(format!("Invalid jimage attribute kind: {kind}")));
            }
            let length = (data & 0x7) as usize + 1;
            let bytes = b
                .get(pos..pos + length)
                .ok_or_else(|| invalid("truncated jimage location"))?;
            attributes[kind] = bytes.iter().fold(0u64, |v, &x| (v << 8) | x as u64);
            pos += length;
        }
        Ok(ImageLocation { attributes })
    }

    /// `ImageLocation.getFullName(false)`.
    pub fn full_name(&self, loc: &ImageLocation) -> io::Result<String> {
        let mut builder = String::new();
        if loc.attributes[ATTRIBUTE_MODULE] != 0 {
            builder.push('/');
            builder.push_str(&self.get_string(loc.attributes[ATTRIBUTE_MODULE])?);
            builder.push('/');
        }
        if loc.attributes[ATTRIBUTE_PARENT] != 0 {
            builder.push_str(&self.get_string(loc.attributes[ATTRIBUTE_PARENT])?);
            builder.push('/');
        }
        builder.push_str(&self.get_string(loc.attributes[ATTRIBUTE_BASE])?);
        if loc.attributes[ATTRIBUTE_EXTENSION] != 0 {
            builder.push('.');
            builder.push_str(&self.get_string(loc.attributes[ATTRIBUTE_EXTENSION])?);
        }
        Ok(builder)
    }

    /// `BasicImageReader.findLocation(name)`: `/module/path` for a resource,
    /// `/modules/module/path` for a directory.
    pub fn find_location(&self, name: &str) -> io::Result<Option<ImageLocation>> {
        let count = self.redirect.len();
        if count == 0 {
            return Ok(None);
        }
        let slot = self.redirect[image_hash(name, HASH_MULTIPLIER) as usize % count];
        let index = match slot.cmp(&0) {
            Ordering::Less => (-(slot as i64) - 1) as usize,
            Ordering::Greater => image_hash(name, slot) as usize % count,
            Ordering::Equal => return Ok(None),
        };
        let Some(&offset) = self.offsets.get(index) else {
            return Ok(None);
        };
        let loc = self.get_location(offset)?;
        Ok((self.full_name(&loc)? == name).then_some(loc))
    }

    fn read_at(&self, offset: u64, size: u64) -> io::Result<Vec<u8>> {
        let guard = self.file.borrow();
        let Some(file) = guard.as_ref() else {
            return Err(io::Error::other("jimage closed"));
        };
        let end = offset
            .checked_add(size)
            .filter(|&e| e <= self.file_len)
            .ok_or_else(|| invalid("jimage resource outside the file"))?;
        let mut buf = vec![0u8; (end - offset) as usize];
        read_exact_at(file, &mut buf, offset)?;
        Ok(buf)
    }

    /// `BasicImageReader.getResource(loc)`, decompressed.
    pub fn get_resource(&self, loc: &ImageLocation) -> io::Result<Vec<u8>> {
        let offset = self
            .index_size
            .checked_add(loc.get_content_offset())
            .ok_or_else(|| invalid("bad jimage resource offset"))?;
        let compressed = loc.get_compressed_size();
        let uncompressed = loc.get_uncompressed_size();
        if compressed > i32::MAX as u64 {
            return Err(invalid("Bad compressed size"));
        }
        if uncompressed > i32::MAX as u64 {
            return Err(invalid("Bad uncompressed size"));
        }
        if compressed == 0 {
            self.read_at(offset, uncompressed)
        } else {
            self.decompress_resource(self.read_at(offset, compressed)?)
        }
    }

    /// The bytes of the resource `/module/path`; `None` when it does not exist.
    pub fn read(&self, name: &str) -> io::Result<Option<Vec<u8>>> {
        match self.find_location(name)? {
            Some(loc) => self.get_resource(&loc).map(Some),
            None => Ok(None),
        }
    }

    /// `Decompressor.decompressResource`.
    fn decompress_resource(&self, mut content: Vec<u8>) -> io::Result<Vec<u8>> {
        for _ in 0..MAX_COMPRESSION_LAYERS {
            if content.len() < COMPRESSED_HEADER_SIZE
                || self.order_u32(&content) != COMPRESSED_MAGIC
            {
                return Ok(content);
            }
            let uncompressed = self.order_u64(&content[12..]);
            let name_offset = self.order_u32(&content[20..]) as u64;
            let plugin = self.get_string(name_offset)?.into_owned();
            content = match plugin.as_str() {
                "zip" => self.zip_decompress(&content, uncompressed)?,
                "compact-cp" => self.string_sharing_normalize(&content)?,
                other => {
                    return Err(invalid(format!(
                        "jimage {}: unsupported resource compression plugin '{other}'",
                        self.path.display()
                    )))
                }
            };
        }
        Err(invalid("too many jimage compression layers"))
    }

    /// `ZipDecompressor.decompress`.
    fn zip_decompress(&self, content: &[u8], original_size: u64) -> io::Result<Vec<u8>> {
        if original_size > MAX_ARRAY_SIZE {
            return Err(invalid("Required array size too large"));
        }
        let out = inflate(
            &content[COMPRESSED_HEADER_SIZE..],
            true,
            original_size,
            true,
        )?;
        if out.len() as u64 != original_size {
            return Err(invalid("Resource content size mismatch"));
        }
        Ok(out)
    }

    /// `StringSharingDecompressor.normalize`.
    fn string_sharing_normalize(&self, transformed: &[u8]) -> io::Result<Vec<u8>> {
        let mut stream = SharingCursor {
            bytes: &transformed[COMPRESSED_HEADER_SIZE..],
            pos: 0,
        };
        let mut out = Vec::with_capacity(transformed.len() * 2);
        out.extend_from_slice(stream.take(8)?);
        let count = stream.u2()?;
        out.extend_from_slice(&count.to_be_bytes());
        let mut i = 1;
        while i < count {
            let tag = stream.u1()?;
            match tag {
                1 => {
                    out.push(tag);
                    let len = stream.u2()?;
                    out.extend_from_slice(&len.to_be_bytes());
                    out.extend_from_slice(stream.take(len as usize)?);
                }
                23 => {
                    let index = stream.compressed_int()?;
                    let orig = self.string_bytes(index as u64)?;
                    out.push(1);
                    write_utf(&mut out, orig)?;
                }
                25 => {
                    let orig = self.reconstruct(&mut stream)?;
                    out.push(1);
                    write_utf(&mut out, &orig)?;
                }
                _ => {
                    let size = match tag {
                        3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => 4,
                        5 | 6 => {
                            i += 1;
                            8
                        }
                        7 | 8 | 16 | 19 | 20 => 2,
                        15 => 3,
                        _ => return Err(invalid(format!("bad constant pool tag {tag}"))),
                    };
                    out.push(tag);
                    out.extend_from_slice(stream.take(size)?);
                }
            }
            i += 1;
        }
        out.extend_from_slice(&stream.bytes[stream.pos..]);
        Ok(out)
    }

    /// `StringSharingDecompressor.reconstruct`, as modified UTF-8 bytes.
    fn reconstruct(&self, stream: &mut SharingCursor<'_>) -> io::Result<Vec<u8>> {
        let desc_index = stream.compressed_int()?;
        let encoded_desc = self.string_bytes(desc_index as u64)?;
        let indexes_length = stream.compressed_int()?;
        let bytes =
            stream.take(usize::try_from(indexes_length).map_err(|_| invalid("bad length"))?)?;
        let indices = decompress_flow(bytes)?;
        let mut buffer = Vec::with_capacity(encoded_desc.len() * 2);
        let mut arg_index = 0;
        let next = |arg_index: &mut usize| -> io::Result<u64> {
            let v = *indices
                .get(*arg_index)
                .ok_or_else(|| invalid("compact-cp descriptor index out of range"))?;
            *arg_index += 1;
            Ok(v as u64)
        };
        for &c in encoded_desc {
            buffer.push(c);
            if c == b'L' {
                let pkg = self.string_bytes(next(&mut arg_index)?)?;
                if !pkg.is_empty() {
                    buffer.extend_from_slice(pkg);
                    buffer.push(b'/');
                }
                buffer.extend_from_slice(self.string_bytes(next(&mut arg_index)?)?);
            }
        }
        Ok(buffer)
    }

    /// The module directories under `/modules`, in stored (sorted) order.
    pub fn module_names(&self) -> io::Result<Vec<String>> {
        let root = self
            .find_location("/modules")?
            .ok_or_else(|| invalid("jimage has no /modules directory"))?;
        let mut names = Vec::new();
        for child in self.children(&root)? {
            if let Some(module) = self.full_name(&child)?.strip_prefix("/modules/") {
                names.push(module.to_string());
            }
        }
        Ok(names)
    }

    fn children(&self, dir: &ImageLocation) -> io::Result<Vec<ImageLocation>> {
        let content = self.get_resource(dir)?;
        content
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| self.get_location(self.order_u32(c)))
            .collect()
    }

    /// `(path, is_directory)` for the module's tree relative to its root:
    /// `""` first, then pre-order in the stored (sorted) child order.
    pub fn walk_module(&self, module: &str) -> io::Result<Vec<(String, bool)>> {
        let root = self
            .find_location(&format!("/modules/{module}"))?
            .ok_or_else(|| invalid(format!("no module {module}")))?;
        let mut out = vec![(String::new(), true)];
        let dir_prefix = format!("/modules/{module}/");
        let res_prefix = format!("/{module}/");
        self.walk_dir(&root, &dir_prefix, &res_prefix, &mut out)?;
        Ok(out)
    }

    fn walk_dir(
        &self,
        dir: &ImageLocation,
        dir_prefix: &str,
        res_prefix: &str,
        out: &mut Vec<(String, bool)>,
    ) -> io::Result<()> {
        let mut kids = Vec::new();
        for child in self.children(dir)? {
            let full = self.full_name(&child)?;
            let is_dir = full.starts_with("/modules");
            let prefix = if is_dir { dir_prefix } else { res_prefix };
            let rel = full
                .strip_prefix(prefix)
                .ok_or_else(|| invalid(format!("misplaced jimage entry {full}")))?
                .to_string();
            kids.push((rel, is_dir, child));
        }
        for (rel, is_dir, child) in kids {
            out.push((rel, is_dir));
            if is_dir {
                self.walk_dir(&child, dir_prefix, res_prefix, out)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for JImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JImage({}, {} locations)",
            self.path.display(),
            self.offsets.len()
        )
    }
}

fn write_utf(out: &mut Vec<u8>, mutf8: &[u8]) -> io::Result<()> {
    let len = u16::try_from(mutf8.len()).map_err(|_| invalid("encoded string too long"))?;
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(mutf8);
    Ok(())
}

struct SharingCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> SharingCursor<'a> {
    fn take(&mut self, n: usize) -> io::Result<&'a [u8]> {
        if self.bytes.len() - self.pos < n {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOFException"));
        }
        let s = &self.bytes[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn u1(&mut self) -> io::Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn u2(&mut self) -> io::Result<u16> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }

    /// `CompressIndexes.readInt`.
    fn compressed_int(&mut self) -> io::Result<i32> {
        let header = self.u1()?;
        let size = header_length(header);
        let mut result = header_value(header);
        for _ in 1..size {
            result = (result << 8) | self.u1()? as i32;
        }
        Ok(result)
    }
}

const COMPRESSED_FLAG: u8 = 0x80;

fn header_length(b: u8) -> usize {
    if b & COMPRESSED_FLAG != 0 {
        ((b >> 5) & 3) as usize
    } else {
        4
    }
}

fn header_value(b: u8) -> i32 {
    if b & COMPRESSED_FLAG != 0 {
        (b & 0x1F) as i32
    } else {
        b as i32
    }
}

/// `CompressIndexes.decompressFlow`.
fn decompress_flow(values: &[u8]) -> io::Result<Vec<i32>> {
    let mut list = Vec::new();
    let mut i = 0;
    while i < values.len() {
        let size = header_length(values[i]);
        if size == 0 {
            return Err(invalid("bad compact-cp index header"));
        }
        let bytes = values
            .get(i..i + size)
            .ok_or_else(|| invalid("truncated compact-cp index"))?;
        let mut result = header_value(bytes[0]);
        for &b in &bytes[1..] {
            result = (result << 8) | b as i32;
        }
        list.push(result);
        i += size;
    }
    Ok(list)
}
