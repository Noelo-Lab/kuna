//! (kuna) SLEIGH spec roots that are either directories or trees held in memory.
//!
//! A [`MemorySpecs`] is a flat table of files, each keyed by its `/`-separated
//! path relative to the spec root (`Ghidra/Processors/x86/data/languages/x86-64.sla`):
//! the tree `make specs` leaves on disk, minus whatever the caller chose not to
//! embed. A [`SpecRoot`] names either kind of root. A bootstrap takes a list of
//! anything that converts into one, so the two kinds mix in one list.
//!
//! The table travels by value: [`LanguageDatabase::add_spec_root`] keeps it in
//! the database, which hands each mounted tree an internal root,
//! `kuna-memory://<n>`, unique within that database. Nothing is registered
//! process-wide. The database reaches spec files only through the functions
//! here; each answers a mounted path from its table and passes any other path
//! to [`FileManage`]/`std::fs`.
//!
//! [`LanguageDatabase::add_spec_root`]: crate::sleigh_arch::LanguageDatabase::add_spec_root

use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kuna_base::filemanage::FileManage;

const MEMORY_ROOT_SCHEME: &str = "kuna-memory://";

type Bytes = Box<dyn AsRef<[u8]> + Send + Sync>;
type Files = [(Cow<'static, str>, Bytes)];

/// A compiled SLEIGH spec tree in memory: (`/`-separated path relative to the
/// spec root, contents) for each file, in any order. Cloning shares the table.
///
/// The tree need not follow Ghidra's layout: a flat table holding one `.ldefs`
/// and the `.sla`, `.pspec` and `.cspec` it names is a complete root.
#[derive(Clone)]
pub struct MemorySpecs {
    files: Arc<Files>,
}

impl MemorySpecs {
    /// A tree over a table compiled into the program; no contents are copied.
    pub fn new(files: &'static [(&'static str, &'static [u8])]) -> MemorySpecs {
        MemorySpecs::from_files(files.iter().copied())
    }

    /// A tree over paths (`&'static str` or `String`) and contents in any
    /// container the caller owns or holds for `'static`: `Vec<u8>`,
    /// `Arc<[u8]>`, `&'static [u8]`, and so on.
    pub fn from_files<P, B>(files: impl IntoIterator<Item = (P, B)>) -> MemorySpecs
    where
        P: Into<Cow<'static, str>>,
        B: AsRef<[u8]> + Send + Sync + 'static,
    {
        MemorySpecs {
            files: files.into_iter().map(|(p, b)| (p.into(), Box::new(b) as Bytes)).collect(),
        }
    }

    /// The contents of the file at `rel`, relative to the spec root.
    pub fn get(&self, rel: &str) -> Option<&[u8]> {
        self.files.iter().find(|(path, _)| path == rel).map(|(_, bytes)| (**bytes).as_ref())
    }
}

impl fmt::Debug for MemorySpecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemorySpecs").field("files", &self.files.len()).finish()
    }
}

/// Where a SLEIGH spec tree lives.
#[derive(Clone, Debug)]
pub enum SpecRoot {
    /// A directory holding `Ghidra/Processors/...` (or the language directories
    /// themselves).
    Dir(PathBuf),
    /// A tree held in memory.
    Memory(MemorySpecs),
}

impl From<String> for SpecRoot {
    fn from(path: String) -> SpecRoot {
        SpecRoot::Dir(path.into())
    }
}

impl From<&String> for SpecRoot {
    fn from(path: &String) -> SpecRoot {
        SpecRoot::Dir(path.into())
    }
}

impl From<&str> for SpecRoot {
    fn from(path: &str) -> SpecRoot {
        SpecRoot::Dir(path.into())
    }
}

impl From<PathBuf> for SpecRoot {
    fn from(path: PathBuf) -> SpecRoot {
        SpecRoot::Dir(path)
    }
}

impl From<&Path> for SpecRoot {
    fn from(path: &Path) -> SpecRoot {
        SpecRoot::Dir(path.into())
    }
}

impl From<MemorySpecs> for SpecRoot {
    fn from(specs: MemorySpecs) -> SpecRoot {
        SpecRoot::Memory(specs)
    }
}

/// Anything a bootstrap accepts in its spec-root list: a path, a
/// [`MemorySpecs`], or a [`SpecRoot`].
pub trait IntoSpecRoot {
    fn to_spec_root(&self) -> SpecRoot;
}

impl<T: Into<SpecRoot> + Clone> IntoSpecRoot for T {
    fn to_spec_root(&self) -> SpecRoot {
        self.clone().into()
    }
}

/// The internal root naming the `index`th tree a database mounted.
pub(crate) fn memory_root(index: usize) -> String {
    format!("{MEMORY_ROOT_SCHEME}{index}")
}

/// The mounted table and the path inside it, or `None` for a filesystem path.
/// An unknown mount resolves to an empty table, so it holds nothing.
fn resolve<'a, 'p>(mounts: &'a [MemorySpecs], path: &'p str) -> Option<(&'a Files, &'p str)> {
    let rest = path.strip_prefix(MEMORY_ROOT_SCHEME)?;
    let (index, inner) = rest.split_once('/').unwrap_or((rest, ""));
    let files = index
        .parse::<usize>()
        .ok()
        .and_then(|i| mounts.get(i))
        .map_or(&[][..], |m| &*m.files);
    Some((files, inner))
}

/// The files and subdirectory names directly inside `dir`, a path inside a table.
fn children<'a>(files: &'a Files, dir: &str) -> (Vec<&'a str>, Vec<&'a str>) {
    let prefix = dir.trim_end_matches('/');
    let mut leaves = Vec::new();
    let mut subdirs = Vec::new();
    for (path, _) in files {
        let rest = if prefix.is_empty() {
            &**path
        } else {
            match path.strip_prefix(prefix).and_then(|r| r.strip_prefix('/')) {
                Some(rest) => rest,
                None => continue,
            }
        };
        match rest.split_once('/') {
            Some((sub, _)) => subdirs.push(sub),
            None => leaves.push(rest),
        }
    }
    leaves.sort_unstable();
    leaves.dedup();
    subdirs.sort_unstable();
    subdirs.dedup();
    (leaves, subdirs)
}

fn with_separator(dirname: &str) -> String {
    let mut dirfinal = dirname.to_string();
    if !dirfinal.ends_with('/') {
        dirfinal.push('/');
    }
    dirfinal
}

/// The contents of the spec file at `path`.
pub(crate) fn read<'a>(mounts: &'a [MemorySpecs], path: &str) -> io::Result<Cow<'a, [u8]>> {
    match resolve(mounts, path) {
        Some((files, inner)) => files
            .iter()
            .find(|(p, _)| p == inner)
            .map(|(_, bytes)| Cow::Borrowed((**bytes).as_ref()))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("{path}: not in the mounted specs"))),
        None => std::fs::read(path).map(Cow::Owned),
    }
}

/// Whether a spec file exists at `path` (C++ `findFile`'s open test).
fn exists(mounts: &[MemorySpecs], path: &str) -> bool {
    match resolve(mounts, path) {
        Some((files, inner)) => files.iter().any(|(p, _)| p == inner),
        None => File::open(path).is_ok(),
    }
}

/// [`FileManage::directory_list`] without dot directories, over either kind of path.
pub(crate) fn directory_list(mounts: &[MemorySpecs], res: &mut Vec<String>, dirname: &str) {
    let Some((files, inner)) = resolve(mounts, dirname) else {
        FileManage::directory_list(res, dirname, false);
        return;
    };
    let dirfinal = with_separator(dirname);
    for sub in children(files, inner).1 {
        if !sub.starts_with('.') {
            res.push(format!("{dirfinal}{sub}"));
        }
    }
}

/// [`FileManage::match_list_dir`] for a suffix, without dot files, over either
/// kind of path.
pub(crate) fn match_list_dir(mounts: &[MemorySpecs], res: &mut Vec<String>, suffix: &str, dirname: &str) {
    let Some((files, inner)) = resolve(mounts, dirname) else {
        FileManage::match_list_dir(res, suffix, true, dirname, false);
        return;
    };
    let dirfinal = with_separator(dirname);
    for leaf in children(files, inner).0 {
        if suffix.len() <= leaf.len() && !leaf.starts_with('.') && leaf.ends_with(suffix) {
            res.push(format!("{dirfinal}{leaf}"));
        }
    }
}

/// [`FileManage::scan_directory_recursive`] over either kind of path.
pub(crate) fn scan_directory_recursive(
    mounts: &[MemorySpecs],
    res: &mut Vec<String>,
    matchname: &str,
    rootpath: &str,
    maxdepth: i32,
) {
    if resolve(mounts, rootpath).is_none() {
        FileManage::scan_directory_recursive(res, matchname, rootpath, maxdepth);
        return;
    }
    if maxdepth == 0 {
        return;
    }
    let mut subdir = Vec::new();
    directory_list(mounts, &mut subdir, rootpath);
    for curpath in &subdir {
        let base = curpath.rsplit('/').next().unwrap_or(curpath);
        if base == matchname {
            res.push(curpath.clone());
        } else {
            scan_directory_recursive(mounts, res, matchname, curpath, maxdepth - 1);
        }
    }
}

/// [`FileManage::find_file`] over a search list that may hold mounted
/// directories: the first directory holding `name` wins, and `res` is cleared
/// when none does.
pub(crate) fn find_file(mounts: &[MemorySpecs], paths: &FileManage, res: &mut String, name: &str) {
    if FileManage::is_absolute_path(name) {
        *res = name.to_string();
        if exists(mounts, res) {
            return;
        }
    } else {
        for path in paths.path_list() {
            *res = format!("{path}{name}");
            if exists(mounts, res) {
                return;
            }
        }
    }
    res.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TREE: &[(&str, &[u8])] = &[
        ("Ghidra/Processors/toy/data/languages/toy.ldefs", b"<language_definitions/>"),
        ("Ghidra/Processors/toy/data/languages/toy.sla", b"sla"),
        ("Ghidra/Processors/toy/data/languages/.hidden.ldefs", b""),
        ("Ghidra/Processors/toy/data/languages/old/toy_old.ldefs", b"old"),
        ("Ghidra/Processors/toy/data/patterns/p.xml", b"p"),
        ("README.md", b"readme"),
    ];

    fn mounted() -> (Vec<MemorySpecs>, String) {
        (vec![MemorySpecs::new(TREE)], memory_root(0))
    }

    #[test]
    fn a_mounted_table_reads_back() {
        let (mounts, root) = mounted();
        let sla = format!("{root}/Ghidra/Processors/toy/data/languages/toy.sla");
        assert!(exists(&mounts, &sla));
        assert_eq!(&*read(&mounts, &sla).unwrap(), b"sla");
        assert_eq!(&*read(&mounts, &format!("{root}/README.md")).unwrap(), b"readme");
        let missing = read(&mounts, &format!("{root}/nope.sla")).unwrap_err();
        assert_eq!(missing.kind(), io::ErrorKind::NotFound);
        assert!(!exists(&mounts, &format!("{root}/Ghidra/Processors")), "a directory is not a file");
    }

    #[test]
    fn owned_and_shared_contents_read_like_static_ones() {
        let owned = MemorySpecs::from_files([(String::from("a/b.sla"), vec![1u8, 2, 3])]);
        assert_eq!(owned.get("a/b.sla"), Some(&[1u8, 2, 3][..]));
        let shared: Arc<[u8]> = Arc::from(&b"pspec"[..]);
        let shared = MemorySpecs::from_files([("a/b.pspec", shared)]);
        let mounts = [owned, shared];
        assert_eq!(&*read(&mounts, &format!("{}/a/b.sla", memory_root(0))).unwrap(), [1, 2, 3]);
        assert_eq!(&*read(&mounts, &format!("{}/a/b.pspec", memory_root(1))).unwrap(), b"pspec");
    }

    #[test]
    fn an_unmounted_index_holds_nothing() {
        let (mounts, _) = mounted();
        let root = memory_root(1);
        assert!(!exists(&mounts, &format!("{root}/README.md")));
        let mut dirs = Vec::new();
        directory_list(&mounts, &mut dirs, &root);
        assert!(dirs.is_empty());
    }

    #[test]
    fn listings_see_only_direct_children() {
        let (mounts, root) = mounted();
        let mut dirs = Vec::new();
        directory_list(&mounts, &mut dirs, &format!("{root}/Ghidra/Processors/toy/data/"));
        let languages = format!("{root}/Ghidra/Processors/toy/data/languages");
        assert_eq!(dirs, [languages.clone(), format!("{root}/Ghidra/Processors/toy/data/patterns")]);

        let mut ldefs = Vec::new();
        match_list_dir(&mounts, &mut ldefs, ".ldefs", &languages);
        assert_eq!(ldefs, [format!("{languages}/toy.ldefs")]);
    }

    #[test]
    fn the_sleigh_layout_scan_finds_language_directories() {
        let (mounts, root) = mounted();
        let mut ghidra = Vec::new();
        scan_directory_recursive(&mounts, &mut ghidra, "Ghidra", &root, 2);
        assert_eq!(ghidra, [format!("{root}/Ghidra")]);
        let mut languages = Vec::new();
        scan_directory_recursive(&mounts, &mut languages, "languages", &format!("{root}/Ghidra/Processors/toy"), 2);
        assert_eq!(languages, [format!("{root}/Ghidra/Processors/toy/data/languages")]);
    }

    #[test]
    fn find_file_takes_the_first_directory_in_order() {
        let (mounts, root) = mounted();
        let mut paths = FileManage::default();
        paths.add_dir_2_path("/nonexistent/kuna/specs");
        paths.add_dir_2_path(&format!("{root}/Ghidra/Processors/toy/data/languages/old"));
        paths.add_dir_2_path(&format!("{root}/Ghidra/Processors/toy/data/languages"));
        let mut res = String::new();
        find_file(&mounts, &paths, &mut res, "toy.sla");
        assert_eq!(res, format!("{root}/Ghidra/Processors/toy/data/languages/toy.sla"));
        find_file(&mounts, &paths, &mut res, "absent.sla");
        assert!(res.is_empty());
    }
}
