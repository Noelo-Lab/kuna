//! JVM inputs: recognizing them, and holding one input's class universe as
//! shared read-only bytes that several engine instances can read at once.
//!
//! [`sniff`] decides whether a path goes to the JVM engine at all; it runs on
//! every `kuna` invocation, so it costs one `stat` and an 8-byte read for a
//! native binary. [`classify`] names the input kind. [`load`] reads an input
//! into an [`InputSet`] (`Send + Sync`); each engine thread then asks it for
//! its own [`IContextSource`] views ([`InputSet::sources`]). The views list
//! exactly what `JarContextSource`, `DirectoryContextSource` and
//! `SingleFileContextSource` list for the same path, in the same order, so a
//! run over them decompiles exactly what a run over the files would.
//! Multi-release entries keep the jar's naming (`META-INF/versions/<n>/…`
//! entries are variants the engine skips) and prefixed entries such as
//! `BOOT-INF/classes/a/B.class` stay as they are: the engine places them by
//! their class name, as Vineflower does.
//!
//! [`SharedRuntime`] is the same idea for a JDK's class library.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::rc::Rc;
use std::sync::Arc;

use crate::context_source::{
    java_absolute_path, java_child, java_file_path, java_name, read_class_info, Entries, Entry,
    IContextSource, CLASS_SUFFIX,
};
use crate::jdk_runtime::{JavaRuntimeContextSource, JdkRuntime};
use crate::jthrow;
use crate::rt::JLinkedHashSet;
use crate::zip_reader::ZipReader;

/// What an input path holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputKind {
    /// A class file: `CA FE BA BE` and a major version in 45..=255.
    ClassFile,
    /// A zip (jar, war, ear) with at least one `.class` entry.
    Archive,
    /// A directory (searched for class files and archives).
    Directory,
    /// A `.jmod` (`JM 01 00`); recognized, not supported yet.
    Jmod,
    /// A jimage (`lib/modules`, `DA DA FE CA`); recognized, not supported yet.
    JImage,
    /// A DEX file (`dex\n`): Android bytecode, not JVM bytecode.
    Dex,
    /// A zip without `.class` entries (an APK holds DEX instead).
    NoJvmClasses,
    /// Anything else, including fat Mach-O binaries (also `CA FE BA BE`).
    Other,
}

impl InputKind {
    /// Whether the JVM path should handle the input (possibly with a clean error).
    pub fn is_jvm(self) -> bool {
        self != InputKind::Other
    }

    /// Whether [`load`] can decompile it.
    pub fn is_supported(self) -> bool {
        matches!(
            self,
            InputKind::ClassFile | InputKind::Archive | InputKind::Directory
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            InputKind::ClassFile => "class",
            InputKind::Archive => "jar",
            InputKind::Directory => "directory",
            InputKind::Jmod => "jmod",
            InputKind::JImage => "jimage",
            InputKind::Dex => "dex",
            InputKind::NoJvmClasses => "zip without JVM classes",
            InputKind::Other => "other",
        }
    }
}

const ZIP_LOCAL: [u8; 4] = *b"PK\x03\x04";
const ZIP_EMPTY: [u8; 4] = *b"PK\x05\x06";
const JMOD_MAGIC: [u8; 4] = *b"JM\x01\x00";
const JIMAGE_MAGIC: [u8; 4] = [0xDA, 0xDA, 0xFE, 0xCA];
const DEX_MAGIC: [u8; 4] = *b"dex\n";
const ZIP_EXTENSIONS: [&str; 4] = ["jar", "war", "ear", "zip"];

/// Whether `head` (the first bytes of a file) starts a class file rather than
/// a fat Mach-O: both begin `CA FE BA BE`, but where a class file has its major
/// version (45 and up) a fat header has the low half of `nfat_arch` (a handful).
pub fn is_class_header(head: &[u8]) -> bool {
    head.len() >= 8
        && head[..4] == [0xCA, 0xFE, 0xBA, 0xBE]
        && (45..=255).contains(&u16::from_be_bytes([head[6], head[7]]))
}

fn head_kind(head: &[u8]) -> Option<InputKind> {
    if is_class_header(head) {
        return Some(InputKind::ClassFile);
    }
    let magic = head.get(..4)?;
    if magic == ZIP_LOCAL || magic == ZIP_EMPTY {
        Some(InputKind::Archive)
    } else if magic == JMOD_MAGIC {
        Some(InputKind::Jmod)
    } else if magic == JIMAGE_MAGIC {
        Some(InputKind::JImage)
    } else if magic == DEX_MAGIC {
        Some(InputKind::Dex)
    } else {
        None
    }
}

fn read_head(path: &Path) -> io::Result<Vec<u8>> {
    let mut head = Vec::with_capacity(8);
    File::open(path)?.take(8).read_to_end(&mut head)?;
    Ok(head)
}

fn has_zip_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| ZIP_EXTENSIONS.iter().any(|z| e.eq_ignore_ascii_case(z)))
}

/// An end-of-central-directory record in the last KiB (the scan of
/// `StructContext.isJarFile`), for archives with a stub in front.
fn has_end_record(path: &Path) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    if size < 22 {
        return Ok(false);
    }
    let window = (size & !3).min(1024) as usize;
    file.seek(SeekFrom::Start(size - window as u64))?;
    let mut tail = vec![0u8; window];
    file.read_exact(&mut tail)?;
    Ok((0..=window - 22).rev().any(|p| tail[p..p + 4] == ZIP_EMPTY))
}

/// Whether `path` goes to the JVM engine: a directory, or a file whose first
/// eight bytes are a class, zip, jmod, jimage or DEX header, or a `.jar`,
/// `.war`, `.ear` or `.zip` file with a zip end record (a launcher-script
/// jar). Never true for a fat Mach-O.
pub fn sniff(path: &Path) -> bool {
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };
    if meta.is_dir() {
        return true;
    }
    if !meta.is_file() {
        return false;
    }
    match read_head(path) {
        Ok(head) if head_kind(&head).is_some() => true,
        Ok(_) => has_zip_extension(path) && has_end_record(path).unwrap_or(false),
        Err(_) => false,
    }
}

/// The kind of input at `path`; reads a zip's central directory to tell a
/// jar from an archive without class files.
pub fn classify(path: &Path) -> io::Result<InputKind> {
    let meta = std::fs::metadata(path)?;
    if meta.is_dir() {
        return Ok(InputKind::Directory);
    }
    let head = read_head(path)?;
    let kind = match head_kind(&head) {
        Some(kind) => kind,
        None if has_zip_extension(path) && has_end_record(path)? => InputKind::Archive,
        None => return Ok(InputKind::Other),
    };
    if kind != InputKind::Archive {
        return Ok(kind);
    }
    let zip = ZipReader::open(path)?;
    let has_classes = zip
        .entries()
        .iter()
        .any(|e| !e.is_directory() && e.name.ends_with(CLASS_SUFFIX));
    Ok(if has_classes {
        InputKind::Archive
    } else {
        InputKind::NoJvmClasses
    })
}

/// Why an input cannot be loaded.
#[derive(Debug)]
pub enum InputError {
    Io(PathBuf, io::Error),
    /// Recognized but not decompiled (`Jmod`, `JImage`, `Dex`, `NoJvmClasses`) or not JVM at all.
    Unsupported(PathBuf, InputKind),
    /// A class file the engine could not read the header of.
    BadClass(PathBuf, String),
    /// A directory without class files.
    NoClasses(PathBuf),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::Io(p, e) => write!(f, "{}: {e}", p.display()),
            InputError::Unsupported(p, InputKind::Dex | InputKind::NoJvmClasses) => write!(
                f,
                "{}: no JVM classes (DEX bytecode is not JVM bytecode)",
                p.display()
            ),
            InputError::Unsupported(p, InputKind::Other) => {
                write!(f, "{}: not a JVM input", p.display())
            }
            InputError::Unsupported(p, kind) => {
                write!(
                    f,
                    "{}: {} inputs are not supported yet",
                    p.display(),
                    kind.name()
                )
            }
            InputError::BadClass(p, e) => write!(f, "{}: unreadable class file: {e}", p.display()),
            InputError::NoClasses(p) => write!(f, "no .class files under {}", p.display()),
        }
    }
}

impl std::error::Error for InputError {}

/// How [`load`] reads an input.
#[derive(Clone, Debug)]
pub struct LoadOptions {
    /// For a single `.class`, also load `Name$*.class` from its directory (as
    /// Vineflower's test tooling and IDE integration group a class with its
    /// nested classes), each as its own source after the class itself.
    pub siblings: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions { siblings: true }
    }
}

/// One input's class universe, shareable across threads (a clone shares the bytes).
#[derive(Clone, Debug)]
pub struct InputSet {
    path: PathBuf,
    kind: InputKind,
    units: Vec<Arc<Unit>>,
}

/// One class file of an [`InputSet`].
#[derive(Clone, Copy, Debug)]
pub struct ClassBytes<'a> {
    /// The entry path without `.class` (`a/B`, `BOOT-INF/classes/a/B`).
    pub entry: &'a str,
    pub bytes: &'a [u8],
}

#[derive(Debug)]
enum Store {
    Zip {
        path: PathBuf,
        raw: Vec<(String, bool)>,
    },
    Directory {
        base: String,
    },
    File {
        contents: Vec<u8>,
    },
}

#[derive(Debug)]
struct Unit {
    name: String,
    store: Store,
    classes: Vec<Entry>,
    directories: Vec<String>,
    others: Vec<Entry>,
    /// Resource path (with `.class`) to bytes or the read error.
    bytes: BTreeMap<String, Result<Arc<[u8]>, String>>,
    children: Vec<Arc<Unit>>,
}

/// Reads the input at `path`.
pub fn load(path: &Path, options: &LoadOptions) -> Result<InputSet, InputError> {
    let io = |e| InputError::Io(path.to_path_buf(), e);
    let kind = classify(path).map_err(io)?;
    let units = match kind {
        InputKind::Archive => vec![Arc::new(zip_unit(path).map_err(io)?)],
        InputKind::Directory => {
            let unit = directory_unit(path).map_err(io)?;
            if !unit.has_classes() {
                return Err(InputError::NoClasses(path.to_path_buf()));
            }
            vec![Arc::new(unit)]
        }
        InputKind::ClassFile => {
            let mut units = vec![Arc::new(file_unit(path)?)];
            if options.siblings {
                for sibling in siblings(path) {
                    units.push(Arc::new(file_unit(&sibling)?));
                }
            }
            units
        }
        other => return Err(InputError::Unsupported(path.to_path_buf(), other)),
    };
    Ok(InputSet {
        path: path.to_path_buf(),
        kind,
        units,
    })
}

impl InputSet {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn kind(&self) -> InputKind {
        self.kind
    }

    /// Fresh context sources over the shared bytes for this thread, in the
    /// order they are added to an engine (`Fernflower::add_source`).
    pub fn sources(&self) -> Vec<Rc<dyn IContextSource>> {
        self.units
            .iter()
            .map(|u| Rc::new(View::new(u.clone())) as Rc<dyn IContextSource>)
            .collect()
    }

    /// Every base-version class file, in source order (units, then entries;
    /// a directory's archives after its own files).
    pub fn classes(&self) -> Vec<ClassBytes<'_>> {
        let mut out = Vec::new();
        for unit in &self.units {
            unit.collect_classes(&mut out);
        }
        out
    }

    /// Total size of the class files, for sizing engine instances.
    pub fn class_bytes(&self) -> usize {
        self.classes().iter().map(|c| c.bytes.len()).sum()
    }
}

impl Unit {
    fn has_classes(&self) -> bool {
        !self.classes.is_empty() || self.children.iter().any(|c| c.has_classes())
    }

    fn collect_classes<'a>(&'a self, out: &mut Vec<ClassBytes<'a>>) {
        for entry in &self.classes {
            if entry.multirelease() != Entry::BASE_VERSION {
                continue;
            }
            let resource = format!("{}{CLASS_SUFFIX}", entry.base_path());
            if let Some(Ok(bytes)) = self.bytes.get(&resource) {
                out.push(ClassBytes {
                    entry: entry.base_path(),
                    bytes,
                });
            } else if let Store::File { contents, .. } = &self.store {
                out.push(ClassBytes {
                    entry: entry.base_path(),
                    bytes: contents,
                });
            }
        }
        for child in &self.children {
            child.collect_classes(out);
        }
    }
}

/// `JarContextSource.getEntries` over the raw entry list.
fn zip_listing(raw: &[(String, bool)]) -> (Vec<Entry>, Vec<String>, Vec<Entry>) {
    let mut classes = Vec::new();
    let mut directories = JLinkedHashSet::new();
    let mut others = Vec::new();
    for (name, is_directory) in raw {
        let mut segment_index = name.find('/');
        while let Some(index) = segment_index {
            directories.add(name[..index].to_string());
            segment_index = name[index + 1..].find('/').map(|r| index + 1 + r);
        }
        if *is_directory {
            directories.add(name.clone());
        } else if let Some(class) = name.strip_suffix(CLASS_SUFFIX) {
            classes.push(Entry::parse(class));
        } else {
            others.push(Entry::parse(name));
        }
    }
    (classes, directories.to_vec(), others)
}

fn zip_unit(path: &Path) -> io::Result<Unit> {
    let zip = ZipReader::open(Path::new(&java_file_path(path)))?;
    let raw: Vec<(String, bool)> = zip
        .entries()
        .iter()
        .map(|e| (e.name.clone(), e.is_directory()))
        .collect();
    let listing = crate::rt::try_catch(|| zip_listing(&raw));
    let mut names: Vec<&String> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (name, is_directory) in &raw {
        if *is_directory || !name.ends_with(CLASS_SUFFIX) || name.starts_with("META-INF/versions/")
        {
            continue;
        }
        if seen.insert(name) {
            names.push(name);
        }
    }
    let mut bytes = BTreeMap::new();
    for (name, read) in read_entries(&zip, path, &names) {
        if let Some(read) = read {
            bytes.insert(name.clone(), read);
        }
    }
    let (classes, directories, others) = listing.unwrap_or_default();
    Ok(Unit {
        name: format!("archive {}", java_absolute_path(&java_file_path(path))),
        store: Store::Zip {
            path: PathBuf::from(java_file_path(path)),
            raw,
        },
        classes,
        directories,
        others,
        bytes,
        children: Vec::new(),
    })
}

type EntryRead = Option<Result<Arc<[u8]>, String>>;

/// `zip.get_entry(name)` read for every name (`None`: no such entry), in
/// order; large archives are inflated on up to 8 threads, each with its own reader.
fn read_entries<'a>(
    zip: &ZipReader,
    path: &Path,
    names: &[&'a String],
) -> Vec<(&'a String, EntryRead)> {
    let read = |zip: &ZipReader, name: &String| -> EntryRead {
        zip.get_entry(name).map(|entry| {
            zip.read_entry(entry)
                .map(Arc::from)
                .map_err(|e| e.to_string())
        })
    };
    let threads = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .clamp(1, 8)
        .min(names.len().div_ceil(256).max(1));
    if threads == 1 {
        return names.iter().map(|&name| (name, read(zip, name))).collect();
    }
    let chunk = names.len().div_ceil(threads);
    let parts: Vec<Option<Vec<EntryRead>>> = std::thread::scope(|scope| {
        let handles: Vec<_> = names
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    let own = ZipReader::open(Path::new(&java_file_path(path))).ok()?;
                    Some(
                        part.iter()
                            .map(|&name| read(&own, name))
                            .collect::<Vec<_>>(),
                    )
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().ok().flatten())
            .collect()
    });
    let mut out = Vec::with_capacity(names.len());
    for (part, reads) in names.chunks(chunk).zip(parts) {
        match reads {
            Some(reads) => out.extend(part.iter().copied().zip(reads)),
            None => out.extend(part.iter().map(|&name| (name, read(zip, name)))),
        }
    }
    out
}

fn utf16_sorted_children(dir: &str) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        names.push(entry?.file_name().to_string_lossy().into_owned());
    }
    names.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
    Ok(names
        .iter()
        .map(|n| crate::context_source::java_resolve(dir, n))
        .collect())
}

fn relativize(base: &str, current: &str) -> String {
    let relative = current.get(base.len()..).unwrap_or("");
    match relative.strip_prefix(['/', '\\']) {
        Some(rest) => rest.to_string(),
        None => relative.to_string(),
    }
}

/// `DirectoryContextSource.collectEntries`, reading class files as it goes.
fn directory_unit(path: &Path) -> io::Result<Unit> {
    let base_directory = java_file_path(path);
    let base = java_absolute_path(&base_directory);
    let mut unit = Unit {
        name: format!("directory {base}"),
        store: Store::Directory {
            base: base_directory.clone(),
        },
        classes: Vec::new(),
        directories: Vec::new(),
        others: Vec::new(),
        bytes: BTreeMap::new(),
        children: Vec::new(),
    };
    let mut stack = vec![base.clone()];
    while let Some(current) = stack.pop() {
        let relative_path = relativize(&base, &current);
        if Path::new(&current).is_dir() {
            unit.directories.push(relative_path);
            let children = utf16_sorted_children(&current)?;
            stack.extend(children.into_iter().rev());
        } else if let Some(class) = relative_path.strip_suffix(CLASS_SUFFIX) {
            let class = class.replace(MAIN_SEPARATOR, "/");
            let resource = format!("{class}{CLASS_SUFFIX}");
            let read = std::fs::read(java_child(&base_directory, &resource))
                .map(Arc::from)
                .map_err(|e| e.to_string());
            unit.bytes.insert(resource, read);
            unit.classes.push(Entry::at_base(&class));
        } else if relative_path.ends_with(".jar") || relative_path.ends_with(".zip") {
            unit.children.push(Arc::new(zip_unit(Path::new(&current))?));
        } else {
            unit.others
                .push(Entry::at_base(&relative_path.replace(MAIN_SEPARATOR, "/")));
        }
    }
    Ok(unit)
}

fn file_unit(path: &Path) -> Result<Unit, InputError> {
    let file = java_file_path(path);
    let contents = std::fs::read(&file).map_err(|e| InputError::Io(path.to_path_buf(), e))?;
    let class_name = if java_name(&file).ends_with(CLASS_SUFFIX) {
        let info = read_class_info(&contents)
            .map_err(|e| InputError::BadClass(path.to_path_buf(), e.to_string()))?;
        Some(info.this_class)
    } else {
        None
    };
    let (classes, others) = match &class_name {
        Some(name) => (vec![Entry::at_base(name)], Vec::new()),
        None => (Vec::new(), vec![Entry::at_base(java_name(&file))]),
    };
    Ok(Unit {
        name: format!("file {file}"),
        store: Store::File { contents },
        classes,
        directories: Vec::new(),
        others,
        bytes: BTreeMap::new(),
        children: Vec::new(),
    })
}

/// `Name$*.class` next to `Name.class`, sorted by name.
fn siblings(class_file: &Path) -> Vec<PathBuf> {
    let (Some(dir), Some(stem)) = (
        class_file.parent(),
        class_file.file_stem().and_then(|s| s.to_str()),
    ) else {
        return Vec::new();
    };
    let dir = if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    };
    let prefix = format!("{stem}$");
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<String> = read
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| {
            n.starts_with(&prefix) && n.ends_with(CLASS_SUFFIX) && n.len() > prefix.len() + 6
        })
        .collect();
    names.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
    names.into_iter().map(|n| dir.join(n)).collect()
}

/// A per-thread [`IContextSource`] over a shared [`Unit`].
struct View {
    unit: Arc<Unit>,
    zip: RefCell<Option<ZipReader>>,
}

impl View {
    fn new(unit: Arc<Unit>) -> View {
        View {
            unit,
            zip: RefCell::new(None),
        }
    }

    fn read_zip(&self, path: &Path, resource: &str) -> Option<Vec<u8>> {
        if self.zip.borrow().is_none() {
            match ZipReader::open(path) {
                Ok(zip) => *self.zip.borrow_mut() = Some(zip),
                Err(e) => jthrow!(IOException; "{}: {}", path.display(), e),
            }
        }
        let guard = self.zip.borrow();
        let zip = guard.as_ref().expect("opened");
        let Some(entry) = zip.get_entry(resource) else {
            jthrow!(NullPointerException; "entry");
        };
        match zip.read_entry(entry) {
            Ok(bytes) => Some(bytes),
            Err(e) => jthrow!(IOException; "{}", e),
        }
    }
}

impl IContextSource for View {
    fn get_name(&self) -> String {
        self.unit.name.clone()
    }

    fn get_entries(&self) -> Entries {
        let unit = &self.unit;
        let (classes, directories, others) = match &unit.store {
            Store::Zip { raw, .. } => zip_listing(raw),
            _ => (
                unit.classes.clone(),
                unit.directories.clone(),
                unit.others.clone(),
            ),
        };
        let children = unit
            .children
            .iter()
            .map(|c| Rc::new(View::new(c.clone())) as Rc<dyn IContextSource>)
            .collect();
        Entries::new_with_children(classes, directories, others, children)
    }

    fn get_class_bytes_shared(&self, class_name: &str) -> Option<Arc<[u8]>> {
        match self.unit.bytes.get(&format!("{class_name}{CLASS_SUFFIX}")) {
            Some(Ok(bytes)) => Some(bytes.clone()),
            _ => self.get_class_bytes(class_name).map(Arc::from),
        }
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        if let Some(read) = self.unit.bytes.get(resource) {
            return match read {
                Ok(bytes) => Some(bytes.to_vec()),
                Err(e) => jthrow!(IOException; "{}", e),
            };
        }
        match &self.unit.store {
            Store::Zip { path, .. } => self.read_zip(path, resource),
            Store::Directory { base } => {
                let target = java_child(base, resource);
                match std::fs::read(&target) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => jthrow!(IOException; "{} ({})", target, e),
                }
            }
            Store::File { contents, .. } => Some(contents.clone()),
        }
    }
}

/// A located JDK whose class library engine threads read through their own views.
#[derive(Clone, Debug)]
pub struct SharedRuntime {
    home: PathBuf,
    identifier: String,
    modular: bool,
    version: Option<String>,
}

impl SharedRuntime {
    /// The JDK an `include-runtime` value names (`1`/`current`: `JAVA_HOME`,
    /// then `java` on `PATH`, then well-known install directories; else a home path).
    pub fn locate(spec: &str) -> Option<PathBuf> {
        JdkRuntime::locate(spec)
    }

    /// Locates and opens the runtime `spec` names, as `Fernflower`'s
    /// constructor would for `include-runtime`; the views of
    /// [`SharedRuntime::sources`] are added with `add_library` in its place.
    pub fn open(spec: &str) -> Result<SharedRuntime, String> {
        let current = spec.eq_ignore_ascii_case(JdkRuntime::CURRENT) || spec == "1";
        let home = if current {
            JdkRuntime::locate(spec)
                .ok_or_else(|| "Failed to open current java runtime for inspection".to_string())?
        } else {
            PathBuf::from(spec)
        };
        let modular = home.join("lib/jrt-fs.jar").is_file()
            || home.join("lib/modules").is_file()
            || home.join("jmods").is_dir();
        let identifier = if current {
            JdkRuntime::CURRENT.to_string()
        } else {
            java_absolute_path(&java_file_path(&home))
        };
        if modular {
            JavaRuntimeContextSource::open(&home, &identifier)
                .map_err(|e| format!("Failed to open java runtime at {}: {e}", home.display()))?;
        } else if !home.exists() || JdkRuntime::sources(&home).is_empty() {
            return Err(format!(
                "Unable to detect a java runtime at {}",
                java_file_path(&home)
            ));
        }
        let version = std::fs::read_to_string(home.join("release"))
            .ok()
            .and_then(|release| {
                release
                    .lines()
                    .find_map(|l| l.strip_prefix("JAVA_VERSION="))
                    .map(|v| v.trim_matches('"').to_string())
            });
        Ok(SharedRuntime {
            home,
            identifier,
            modular,
            version,
        })
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    /// `JAVA_VERSION` from the home's `release` file.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// This thread's sources for the runtime: one root source with a child per
    /// module, or one per jar of a Java 8 runtime.
    pub fn sources(&self) -> Vec<Rc<dyn IContextSource>> {
        if self.modular {
            match JavaRuntimeContextSource::open(&self.home, &self.identifier) {
                Ok(source) => vec![Rc::new(source)],
                Err(_) => Vec::new(),
            }
        } else {
            JdkRuntime::sources(&self.home)
        }
    }
}
