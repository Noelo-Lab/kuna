//! Replaces main/extern/IContextSource + struct/*ContextSource.
//!
//! The reading half of `IContextSource` (the output-sink half is replaced by
//! kuna's own output) and its three file-system implementations:
//! `JarContextSource`, `DirectoryContextSource` and `SingleFileContextSource`.
//! The legacy `IBytecodeProvider` constructor argument is gone (it was always
//! `null` outside deprecated embedder APIs). Every checked `IOException` is
//! thrown as `jthrow!(IOException; …)`, whatever subclass the JDK would use.
//!
//! `File.listFiles()` order is unspecified in Java; `DirectoryContextSource`
//! visits each directory's children sorted by name (UTF-16 order, like
//! `File.compareTo`), so its listings are deterministic.

use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::path::{Path, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use std::rc::Rc;

use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_logger::Severity;
use crate::jthrow;
use crate::rt::jstring::decode_modified_utf8;
use crate::rt::{JLinkedHashSet, JavaStr};
use crate::zip_reader::{ZipEntry, ZipReader};

/// `IContextSource.CLASS_SUFFIX`.
pub const CLASS_SUFFIX: &str = ".class";

/// `org.jetbrains.java.decompiler.main.extern.IContextSource`, reading side.
pub trait IContextSource {
    fn get_name(&self) -> String;

    fn get_entries(&self) -> Entries;

    fn is_lazy(&self) -> bool {
        false
    }

    fn has_class(&self, class_name: &str) -> bool {
        self.get_class_bytes(class_name).is_some()
    }

    fn get_class_bytes(&self, class_name: &str) -> Option<Vec<u8>> {
        self.get_input_stream(&format!("{class_name}{CLASS_SUFFIX}"))
    }

    /// [`IContextSource::get_class_bytes`] as bytes a class may keep (kuna).
    fn get_class_bytes_shared(&self, class_name: &str) -> Option<std::sync::Arc<[u8]>> {
        self.get_class_bytes(class_name).map(std::sync::Arc::from)
    }

    /// Java: getInputStream(Entry)
    fn get_input_stream_entry(&self, resource: &Entry) -> Option<Vec<u8>> {
        self.get_input_stream(&resource.path())
    }

    /// Java: getInputStream(String). The whole stream is read; `None` is a `null` stream.
    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>>;

    /// `AutoCloseable.close()` for the sources that implement it; a no-op otherwise.
    fn close(&self) {}
}

/// `IContextSource.Entries`.
#[derive(Clone)]
pub struct Entries {
    classes: Vec<Entry>,
    directories: Vec<String>,
    others: Vec<Entry>,
    child_contexts: Vec<Rc<dyn IContextSource>>,
}

impl Entries {
    #[allow(non_snake_case)]
    pub fn EMPTY() -> Entries {
        Entries::new(Vec::new(), Vec::new(), Vec::new())
    }

    /// Java: Entries(List, List, List)
    pub fn new(classes: Vec<Entry>, directories: Vec<String>, others: Vec<Entry>) -> Entries {
        Entries::new_with_children(classes, directories, others, Vec::new())
    }

    /// Java: Entries(List, List, List, List)
    pub fn new_with_children(
        classes: Vec<Entry>,
        directories: Vec<String>,
        others: Vec<Entry>,
        child_contexts: Vec<Rc<dyn IContextSource>>,
    ) -> Entries {
        Entries {
            classes,
            directories,
            others,
            child_contexts,
        }
    }

    pub fn classes(&self) -> &[Entry] {
        &self.classes
    }

    pub fn directories(&self) -> &[String] {
        &self.directories
    }

    pub fn others(&self) -> &[Entry] {
        &self.others
    }

    pub fn child_contexts(&self) -> &[Rc<dyn IContextSource>] {
        &self.child_contexts
    }
}

impl fmt::Debug for Entries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let children: Vec<String> = self.child_contexts.iter().map(|c| c.get_name()).collect();
        f.debug_struct("Entries")
            .field("classes", &self.classes)
            .field("directories", &self.directories)
            .field("others", &self.others)
            .field("child_contexts", &children)
            .finish()
    }
}

/// `IContextSource.Entry`: a path, possibly a multi-release variant.
#[derive(Clone, Debug)]
pub struct Entry {
    base_path: String,
    multirelease: i32,
}

impl Entry {
    pub const BASE_VERSION: i32 = -1;
    const MULTIRELEASE_PREFIX: &'static str = "META-INF/versions/";

    /// Parse a raw jar path; `META-INF/versions/<n>/x` keeps the `/x` tail as its base path.
    pub fn parse(path: &str) -> Entry {
        if let Some(rest) = path.strip_prefix(Self::MULTIRELEASE_PREFIX) {
            let Some(rel) = rest.find('/') else {
                return Entry::new(path.to_string(), Self::BASE_VERSION);
            };
            let next_slash = Self::MULTIRELEASE_PREFIX.len() + rel;
            let version = &path[Self::MULTIRELEASE_PREFIX.len()..next_slash];
            if let Some(v) = java_parse_int(version) {
                return Entry::new(path[next_slash..].to_string(), v);
            }
        }
        Entry::new(path.to_string(), Self::BASE_VERSION)
    }

    pub fn at_base(path: &str) -> Entry {
        Entry::new(path.to_string(), Self::BASE_VERSION)
    }

    pub fn new(base_path: String, multirelease: i32) -> Entry {
        if multirelease != -1 && multirelease < 9 {
            jthrow!(IllegalArgumentException; "A multirelease variant must target a Java runtime >= 9");
        }
        Entry {
            base_path,
            multirelease,
        }
    }

    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    pub fn multirelease(&self) -> i32 {
        self.multirelease
    }

    pub fn path(&self) -> String {
        if self.multirelease == Self::BASE_VERSION {
            self.base_path.clone()
        } else {
            format!(
                "{}{}/{}",
                Self::MULTIRELEASE_PREFIX,
                self.multirelease,
                self.base_path
            )
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E[{}]", self.base_path)
    }
}

/// `struct.JarContextSource`: a jar or zip archive.
pub struct JarContextSource {
    relative_path: String,
    jar_file: String,
    file: ZipReader,
}

impl JarContextSource {
    /// Java: JarContextSource(IBytecodeProvider, File)
    pub fn new(archive: &Path) -> JarContextSource {
        JarContextSource::new_relative(archive, "")
    }

    /// Java: JarContextSource(IBytecodeProvider, File, String)
    pub fn new_relative(archive: &Path, relative_path: &str) -> JarContextSource {
        match JarContextSource::open(archive, relative_path) {
            Ok(source) => source,
            Err(e) => jthrow!(IOException; "{}: {}", java_file_path(archive), e),
        }
    }

    /// The constructor with its `IOException` as a `Result`.
    pub fn open(archive: &Path, relative_path: &str) -> io::Result<JarContextSource> {
        let jar_file = java_file_path(archive);
        let file = ZipReader::open(Path::new(&jar_file))?;
        Ok(JarContextSource {
            relative_path: relative_path.to_string(),
            jar_file,
            file,
        })
    }

    /// The archive's folder relative to the enclosing directory source (`""` at top level).
    pub fn get_relative_path(&self) -> &str {
        &self.relative_path
    }

    pub fn get_jar_file(&self) -> &Path {
        Path::new(&self.jar_file)
    }

    pub fn get_zip(&self) -> &ZipReader {
        &self.file
    }

    fn add_directories(entry: &ZipEntry, directories: &mut JLinkedHashSet<String>) {
        let name = &entry.name;
        let mut segment_index = name.find('/');
        while let Some(index) = segment_index {
            directories.add(name[..index].to_string());
            segment_index = name[index + 1..].find('/').map(|r| index + 1 + r);
        }
        if entry.is_directory() {
            directories.add(name.clone());
        }
    }

    fn ensure_open(&self) {
        if self.file.is_closed() {
            jthrow!(IllegalStateException; "zip file closed");
        }
    }
}

impl IContextSource for JarContextSource {
    fn get_name(&self) -> String {
        format!("archive {}", java_absolute_path(&self.jar_file))
    }

    fn get_entries(&self) -> Entries {
        self.ensure_open();
        let mut classes = Vec::new();
        let mut directories = JLinkedHashSet::new();
        let mut others = Vec::new();
        for entry in self.file.entries() {
            let name = &entry.name;
            JarContextSource::add_directories(entry, &mut directories);
            if !entry.is_directory() {
                if name.ends_with(CLASS_SUFFIX) {
                    classes.push(Entry::parse(&name[..name.len() - CLASS_SUFFIX.len()]));
                } else {
                    others.push(Entry::parse(name));
                }
            }
        }
        Entries::new_with_children(classes, directories.to_vec(), others, Vec::new())
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        self.ensure_open();
        let Some(entry) = self.file.get_entry(resource) else {
            jthrow!(NullPointerException; "entry");
        };
        match self.file.read_entry(entry) {
            Ok(bytes) => Some(bytes),
            Err(e) => jthrow!(IOException; "{}", e),
        }
    }

    fn close(&self) {
        self.file.close();
    }
}

/// `struct.DirectoryContextSource`: a directory tree, recursively.
pub struct DirectoryContextSource {
    base_directory: String,
}

impl DirectoryContextSource {
    pub fn new(base_directory: &Path) -> DirectoryContextSource {
        DirectoryContextSource {
            base_directory: java_file_path(base_directory),
        }
    }

    pub fn get_base_directory(&self) -> &Path {
        Path::new(&self.base_directory)
    }

    /// `current` is an absolute path string, as `File.getAbsolutePath()` gives it.
    #[allow(clippy::only_used_in_recursion)]
    pub fn collect_entries(
        &self,
        base: &str,
        current: &str,
        classes: &mut Vec<Entry>,
        directories: &mut Vec<String>,
        others: &mut Vec<Entry>,
        jar_children: &mut Vec<Rc<dyn IContextSource>>,
    ) {
        let relative_path = Self::relativize(base, current);
        if Path::new(current).is_dir() {
            directories.push(relative_path);
            let Some(children) = list_files(current) else {
                jthrow!(NullPointerException; "listFiles() returned null for {}", current);
            };
            for child in children {
                self.collect_entries(base, &child, classes, directories, others, jar_children);
            }
        } else if relative_path.ends_with(CLASS_SUFFIX) {
            classes.push(Self::sanitize(
                &relative_path[..relative_path.len() - CLASS_SUFFIX.len()],
            ));
        } else if relative_path.ends_with(".jar") || relative_path.ends_with(".zip") {
            let parent = java_parent(current);
            let relative_to = parent.get(base.len()..).unwrap_or("").to_string();
            match JarContextSource::open(Path::new(current), &relative_to) {
                Ok(source) => jar_children.push(Rc::new(source)),
                Err(_) => {
                    let message = format!("Invalid archive {current}");
                    DecompilerContext::get_logger().write_message(&message, Severity::Error);
                    jthrow!(UncheckedIOException; "{}", message);
                }
            }
        } else {
            others.push(Self::sanitize(&relative_path));
        }
    }

    fn relativize(base: &str, current: &str) -> String {
        let relative_path = current.get(base.len()..).unwrap_or("");
        match relative_path.strip_prefix(['/', '\\']) {
            Some(rest) => rest.to_string(),
            None => relative_path.to_string(),
        }
    }

    fn sanitize(path: &str) -> Entry {
        Entry::at_base(&path.replace(MAIN_SEPARATOR, "/"))
    }
}

impl IContextSource for DirectoryContextSource {
    fn get_name(&self) -> String {
        format!("directory {}", java_absolute_path(&self.base_directory))
    }

    fn get_entries(&self) -> Entries {
        let mut classes = Vec::new();
        let mut directories = Vec::new();
        let mut others = Vec::new();
        let mut jar_children = Vec::new();
        let base = java_absolute_path(&self.base_directory);
        self.collect_entries(
            &base,
            &base,
            &mut classes,
            &mut directories,
            &mut others,
            &mut jar_children,
        );
        Entries::new_with_children(classes, directories, others, jar_children)
    }

    fn get_input_stream(&self, resource: &str) -> Option<Vec<u8>> {
        let target_file = java_child(&self.base_directory, resource);
        match std::fs::read(&target_file) {
            Ok(bytes) => Some(bytes),
            Err(e) => jthrow!(IOException; "{} ({})", target_file, e),
        }
    }
}

/// `struct.SingleFileContextSource`: one file, usually a lone `.class`.
pub struct SingleFileContextSource {
    file: String,
    qualified_name: Option<String>,
    contents: Option<Vec<u8>>,
}

impl SingleFileContextSource {
    pub fn new(single_file: &Path) -> SingleFileContextSource {
        match SingleFileContextSource::open(single_file) {
            Ok(source) => source,
            Err(e) => jthrow!(IOException; "{}: {}", java_file_path(single_file), e),
        }
    }

    /// The constructor with its `IOException` as a `Result`.
    pub fn open(single_file: &Path) -> io::Result<SingleFileContextSource> {
        let file = java_file_path(single_file);
        if !Path::new(&file).is_file() {
            return Ok(SingleFileContextSource {
                file,
                qualified_name: None,
                contents: None,
            });
        }
        let contents = std::fs::read(&file)?;
        let qualified_name = if java_name(&file).ends_with(CLASS_SUFFIX) {
            Some(read_class_info(&contents)?.this_class)
        } else {
            None
        };
        Ok(SingleFileContextSource {
            file,
            qualified_name,
            contents: Some(contents),
        })
    }

    pub fn get_file(&self) -> &Path {
        Path::new(&self.file)
    }
}

impl IContextSource for SingleFileContextSource {
    fn get_name(&self) -> String {
        format!("file {}", self.file)
    }

    fn get_entries(&self) -> Entries {
        if self.contents.is_none() {
            Entries::EMPTY()
        } else if java_name(&self.file).ends_with(CLASS_SUFFIX) {
            let name = self.qualified_name.as_deref().expect("qualifiedName");
            Entries::new(vec![Entry::at_base(name)], Vec::new(), Vec::new())
        } else {
            Entries::new(
                Vec::new(),
                Vec::new(),
                vec![Entry::at_base(java_name(&self.file))],
            )
        }
    }

    fn get_input_stream(&self, _resource: &str) -> Option<Vec<u8>> {
        match &self.contents {
            Some(contents) => Some(contents.clone()),
            None => jthrow!(NullPointerException; "contents"),
        }
    }
}

macro_rules! debug_by_name {
    ($($t:ty),*) => {$(
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.get_name())
            }
        }
    )*};
}
debug_by_name!(
    JarContextSource,
    DirectoryContextSource,
    SingleFileContextSource
);

const BMP_DIGIT_ZEROS: [u16; 37] = [
    0x0030, 0x0660, 0x06F0, 0x07C0, 0x0966, 0x09E6, 0x0A66, 0x0AE6, 0x0B66, 0x0BE6, 0x0C66, 0x0CE6,
    0x0D66, 0x0DE6, 0x0E50, 0x0ED0, 0x0F20, 0x1040, 0x1090, 0x17E0, 0x1810, 0x1946, 0x19D0, 0x1A80,
    0x1A90, 0x1B50, 0x1BB0, 0x1C40, 0x1C50, 0xA620, 0xA8D0, 0xA900, 0xA9D0, 0xA9F0, 0xAA50, 0xABF0,
    0xFF10,
];

/// `Character.digit(ch, 10)` (JDK 21 tables).
fn java_digit(unit: u16) -> i32 {
    BMP_DIGIT_ZEROS
        .iter()
        .find(|&&zero| unit >= zero && unit < zero + 10)
        .map_or(-1, |&zero| (unit - zero) as i32)
}

/// `Integer.parseInt(s)`; `None` is a `NumberFormatException`.
pub fn java_parse_int(s: &str) -> Option<i32> {
    let units = s.utf16();
    let len = units.len();
    if len == 0 {
        return None;
    }
    let mut i = 0;
    let mut negative = false;
    let mut limit = -i32::MAX;
    let first = units[0];
    if first < b'0' as u16 {
        if first == b'-' as u16 {
            negative = true;
            limit = i32::MIN;
        } else if first != b'+' as u16 {
            return None;
        }
        if len == 1 {
            return None;
        }
        i += 1;
    }
    let multmin = limit / 10;
    let mut result: i32 = 0;
    while i < len {
        let digit = java_digit(units[i]);
        i += 1;
        if digit < 0 || result < multmin {
            return None;
        }
        result *= 10;
        if result < limit + digit {
            return None;
        }
        result -= digit;
    }
    Some(if negative { result } else { -result })
}

/// `new File(path).getPath()`: separators unified, repeated and trailing ones removed.
pub fn java_normalize(path: &str) -> String {
    let path = if MAIN_SEPARATOR == '/' {
        path.to_string()
    } else {
        path.replace('/', MAIN_SEPARATOR_STR)
    };
    let mut out = String::with_capacity(path.len());
    let mut prev_sep = false;
    for c in path.chars() {
        if c == MAIN_SEPARATOR {
            if !prev_sep {
                out.push(c);
            }
            prev_sep = true;
        } else {
            out.push(c);
            prev_sep = false;
        }
    }
    if out.len() > 1 && out.ends_with(MAIN_SEPARATOR) {
        out.pop();
    }
    out
}

/// `File.getPath()` of `new File(path)`.
pub fn java_file_path(path: &Path) -> String {
    java_normalize(&path.to_string_lossy())
}

/// `UnixFileSystem.resolve(parent, child)` for normalized strings.
pub fn java_resolve(parent: &str, child: &str) -> String {
    let sep = MAIN_SEPARATOR;
    let root = parent.len() == sep.len_utf8() && parent.starts_with(sep);
    if child.is_empty() {
        parent.to_string()
    } else if child.starts_with(sep) {
        if root {
            child.to_string()
        } else {
            format!("{parent}{child}")
        }
    } else if root {
        format!("{parent}{child}")
    } else {
        format!("{parent}{sep}{child}")
    }
}

/// `new File(new File(parent), child).getPath()`.
pub fn java_child(parent: &str, child: &str) -> String {
    let child = java_normalize(child);
    if parent.is_empty() {
        java_resolve(MAIN_SEPARATOR_STR, &child)
    } else {
        java_resolve(parent, &child)
    }
}

/// `File.getAbsolutePath()` of a normalized path string.
pub fn java_absolute_path(path: &str) -> String {
    if Path::new(path).is_absolute() {
        return path.to_string();
    }
    let cwd = std::env::current_dir()
        .map(|p| java_file_path(&p))
        .unwrap_or_default();
    java_resolve(&cwd, path)
}

/// `File.getName()`.
pub fn java_name(path: &str) -> &str {
    match path.rfind(MAIN_SEPARATOR) {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

fn java_parent(path: &str) -> &str {
    match path.rfind(MAIN_SEPARATOR) {
        Some(0) => &path[..1],
        Some(i) => &path[..i],
        None => "",
    }
}

fn utf16_cmp(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// `File.listFiles()` as absolute path strings, sorted by name; `None` on an I/O error.
fn list_files(dir: &str) -> Option<Vec<String>> {
    let mut names = Vec::new();
    for entry in std::fs::read_dir(dir).ok()? {
        names.push(entry.ok()?.file_name().to_string_lossy().into_owned());
    }
    names.sort_by(|a, b| utf16_cmp(a, b));
    Some(names.iter().map(|n| java_resolve(dir, n)).collect())
}

/// What the decompiler needs from a class file before `StructClass` exists.
pub struct ClassFileInfo {
    pub this_class: String,
    /// The `Module` attribute's module name and version.
    pub module: Option<(String, Option<String>)>,
}

#[derive(Clone)]
enum Pooled {
    None,
    Utf8(String),
    Ref(u16),
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
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

    fn u4(&mut self) -> io::Result<u32> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn skip_members(&mut self) -> io::Result<()> {
        let count = self.u2()?;
        for _ in 0..count {
            self.take(6)?;
            self.skip_attributes()?;
        }
        Ok(())
    }

    fn skip_attributes(&mut self) -> io::Result<()> {
        let count = self.u2()?;
        for _ in 0..count {
            self.take(2)?;
            let len = self.u4()? as usize;
            self.take(len)?;
        }
        Ok(())
    }
}

fn pool_string(pool: &[Pooled], index: u16) -> io::Result<String> {
    let bad = || io::Error::new(io::ErrorKind::InvalidData, format!("bad constant #{index}"));
    match pool.get(index as usize) {
        Some(Pooled::Utf8(s)) => Ok(s.clone()),
        Some(Pooled::Ref(target)) => match pool.get(*target as usize) {
            Some(Pooled::Utf8(s)) => Ok(s.clone()),
            _ => Err(bad()),
        },
        _ => Err(bad()),
    }
}

/// The structural walk of `StructClass.create` that yields `qualifiedName` and
/// the `Module` attribute; a truncated class fails here as it would there.
pub fn read_class_info(bytes: &[u8]) -> io::Result<ClassFileInfo> {
    let mut c = Cursor { bytes, pos: 0 };
    c.take(8)?;
    let count = c.u2()? as usize;
    let mut pool = vec![Pooled::None; count.max(1)];
    let mut i = 1;
    while i < count {
        let tag = c.u1()?;
        match tag {
            1 => {
                let len = c.u2()? as usize;
                pool[i] = Pooled::Utf8(decode_modified_utf8(c.take(len)?));
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => {
                c.take(4)?;
            }
            5 | 6 => {
                c.take(8)?;
                i += 1;
            }
            7 | 8 | 16 | 19 | 20 => pool[i] = Pooled::Ref(c.u2()?),
            15 => {
                c.take(3)?;
            }
            _ => jthrow!("Invalid Constant Pool entry #{} Type: {}", i, tag as i8),
        }
        i += 1;
    }
    c.take(2)?;
    let this_class = pool_string(&pool, c.u2()?)?;
    c.take(2)?;
    let interfaces = c.u2()? as usize;
    c.take(interfaces * 2)?;
    c.skip_members()?;
    c.skip_members()?;
    let mut module = None;
    let attributes = c.u2()?;
    for _ in 0..attributes {
        let name = c.u2()?;
        let len = c.u4()? as usize;
        let data = c.take(len)?;
        if matches!(pool.get(name as usize), Some(Pooled::Utf8(s)) if s == "Module") {
            let mut a = Cursor {
                bytes: data,
                pos: 0,
            };
            let module_name = pool_string(&pool, a.u2()?)?;
            a.u2()?;
            let version = match a.u2()? {
                0 => None,
                v => Some(pool_string(&pool, v)?),
            };
            module = Some((module_name, version));
        }
    }
    Ok(ClassFileInfo { this_class, module })
}
