//! Port of `decompiler/cpp/filemanage.{hh,cc}` (W1): generic class for
//! searching files and managing paths — the path scanning / directory
//! listing used to find SLEIGH spec files (`sleigh_arch.cc`,
//! `slgh_compile.cc`, `consolemain.cc`, `test.cc`).
//!
//! Only the POSIX branch of the C++ `#ifdef _WINDOWS` pairs is ported (kuna
//! targets Linux); the Windows branches (`FindFirstFileA`,
//! `GetCurrentDirectoryA`, the `'\\'` separator) are intentionally dropped.
//! Implementation is `std::fs` based but keeps the C++ string-typed API
//! surface, including the out-parameter accumulation style (`res` vectors
//! are appended to across calls) and the "empty string means not found"
//! convention of [`FileManage::find_file`].
//!
//! Known deviations from the POSIX C++ (all unobservable to the in-tree
//! callers):
//! - `std::fs::read_dir` never yields the `.` and `..` entries that
//!   `readdir(3)` does.  `directoryList` filtered them explicitly anyway;
//!   `matchListDir` did not, so a C++ caller using `allowdot=true` could in
//!   principle match them (no in-tree caller passes `allowdot=true`).
//! - Directory entries whose names are not valid UTF-8 are skipped (C++
//!   strings carry raw bytes).  Spec trees are ASCII.
//! - `addCurrentDir` uses `std::env::current_dir()`, dropping the C++
//!   256-byte `getcwd` buffer cap.

use std::fs;
use std::fs::File;

/// Path name separator (POSIX branch; C++ `FileManage::separator`).
const SEPARATOR: char = '/';

/// A list of paths to search for files, plus static path utilities
/// (C++ `FileManage`).
#[derive(Debug, Clone, Default)]
pub struct FileManage {
    /// List of paths to search for files
    pathlist: Vec<String>,
}

impl FileManage {
    /// Add a directory to the search path; a terminating separator is
    /// appended if missing.  An empty path is ignored.  (C++ `addDir2Path`.)
    pub fn add_dir_2_path(&mut self, path: &str) {
        if !path.is_empty() {
            let mut entry = path.to_string();
            if !Self::is_separator(path.chars().next_back().unwrap()) {
                entry.push(SEPARATOR);
            }
            self.pathlist.push(entry);
        }
    }

    /// Add current working directory to path (C++ `addCurrentDir`).
    ///
    /// C++ calls `getcwd` into a 256-byte buffer and silently adds nothing on
    /// failure; the port does the same on `current_dir()` failure (or a
    /// non-UTF-8 cwd).
    pub fn add_current_dir(&mut self) {
        if let Ok(dir) = std::env::current_dir() {
            if let Some(filename) = dir.to_str() {
                self.add_dir_2_path(filename);
            }
        }
    }

    /// Search through paths to find a file with the given name, resolving the
    /// full pathname into `res`.  An absolute `name` is only checked
    /// verbatim.  On failure `res` is cleared: "can't find it, return empty
    /// string".  (C++ `findFile`.)
    pub fn find_file(&self, res: &mut String, name: &str) {
        // C++ tests name[0]; on an empty string C++11 operator[] yields '\0',
        // which is not a separator, so the else branch runs (and matches the
        // None case here).
        if name.chars().next().is_some_and(Self::is_separator) {
            *res = name.to_string();
            // C++ opens an ifstream purely as an existence/readability test
            if File::open(&*res).is_ok() {
                return;
            }
        } else {
            for path in &self.pathlist {
                *res = format!("{}{}", path, name);
                if File::open(&*res).is_ok() {
                    return;
                }
            }
        }
        res.clear(); // Can't find it, return empty string
    }

    /// Accumulate into `res` the files in every search path matching
    /// `match_` (as suffix or prefix, per `is_suffix`), excluding dot files.
    /// (C++ `matchList`.)
    pub fn match_list(&self, res: &mut Vec<String>, match_: &str, is_suffix: bool) {
        for path in &self.pathlist {
            Self::match_list_dir(res, match_, is_suffix, path, false);
        }
    }

    /// Is the character a path separator?  (POSIX branch of C++
    /// `isSeparator`; the Windows branch also accepted `'/'` and `'\\'`.)
    pub fn is_separator(c: char) -> bool {
        c == SEPARATOR
    }

    /// Is the path an existing directory?  (POSIX branch of C++
    /// `isDirectory`: `stat(2)`, so symlinks are followed; any error is
    /// "not a directory".)
    pub fn is_directory(path: &str) -> bool {
        match fs::metadata(path) {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        }
    }

    /// Look through files in a directory for those matching `match_`,
    /// appending `dirfinal + name` for each hit.  (POSIX branch of C++
    /// `matchListDir`.)
    ///
    /// * `match_` is compared as a suffix when `is_suffix`, else as a prefix
    /// * names shorter than `match_` never match
    /// * names beginning with `'.'` are skipped unless `allowdot`
    /// * an unreadable directory contributes nothing
    pub fn match_list_dir(
        res: &mut Vec<String>,
        match_: &str,
        is_suffix: bool,
        dirname: &str,
        allowdot: bool,
    ) {
        let mut dirfinal = dirname.to_string();
        if !dirfinal.ends_with(SEPARATOR) {
            dirfinal.push(SEPARATOR);
        }
        let dir = match fs::read_dir(&dirfinal) {
            Ok(d) => d,
            Err(_) => return, // opendir failed
        };
        for entry in dir.flatten() {
            let name_os = entry.file_name();
            let fullname = match name_os.to_str() {
                Some(s) => s,
                None => continue, // non-UTF-8 name; see module docs
            };
            if match_.len() <= fullname.len() && (allowdot || !fullname.starts_with('.')) {
                if is_suffix {
                    if fullname.ends_with(match_) {
                        res.push(format!("{}{}", dirfinal, fullname));
                    }
                } else if fullname.starts_with(match_) {
                    res.push(format!("{}{}", dirfinal, fullname));
                }
            }
        }
    }

    /// List full pathnames of all directories under the directory `dirname`
    /// (POSIX branch of C++ `directoryList`; the C++ declares
    /// `allowdot=false` as a default argument).
    ///
    /// Symlinks (and `DT_UNKNOWN` entries) are resolved with `stat(2)`, so a
    /// symlink to a directory counts; `.`/`..` are excluded; dot-directories
    /// are excluded unless `allowdot`.
    pub fn directory_list(res: &mut Vec<String>, dirname: &str, allowdot: bool) {
        let mut dirfinal = dirname.to_string();
        if !dirfinal.ends_with(SEPARATOR) {
            dirfinal.push(SEPARATOR);
        }
        let dir = match fs::read_dir(&dirfinal) {
            Ok(d) => d,
            Err(_) => return, // opendir failed
        };
        for entry in dir.flatten() {
            // C++: d_type == DT_DIR is a directory; through DT_UNKNOWN or
            // DT_LNK it stat()s the full path (following symlinks).
            // DirEntry::file_type() already resolves DT_UNKNOWN via lstat, so
            // the symlink branch below is the only one needing a second look.
            let is_directory = match entry.file_type() {
                Ok(ft) => {
                    if ft.is_dir() {
                        true
                    } else if ft.is_symlink() {
                        // C++ ignores a stat failure, leaving stbuf garbage;
                        // practically a dangling link is "not a directory"
                        fs::metadata(entry.path()).map(|m| m.is_dir()).unwrap_or(false)
                    } else {
                        false
                    }
                }
                Err(_) => false,
            };
            if is_directory {
                let name_os = entry.file_name();
                let fullname = match name_os.to_str() {
                    Some(s) => s,
                    None => continue, // non-UTF-8 name; see module docs
                };
                // (read_dir never yields the "."/".." entries C++ filters here)
                if allowdot || !fullname.starts_with('.') {
                    res.push(format!("{}{}", dirfinal, fullname));
                }
            }
        }
    }

    /// Recursively scan `rootpath` for directories whose base name is exactly
    /// `matchname`, descending at most `maxdepth` levels.  A matching
    /// directory is recorded and NOT descended into.  (C++
    /// `scanDirectoryRecursive`.)
    pub fn scan_directory_recursive(
        res: &mut Vec<String>,
        matchname: &str,
        rootpath: &str,
        maxdepth: i32,
    ) {
        if maxdepth == 0 {
            return;
        }
        let mut subdir: Vec<String> = Vec::new();
        Self::directory_list(&mut subdir, rootpath, false);
        for curpath in &subdir {
            // string::find_last_of(separatorClass): byte position of the last
            // separator ('/' is single-byte, so byte indexing is UTF-8 safe)
            let pos = match curpath.rfind(SEPARATOR) {
                Some(p) => p + 1,
                None => 0,
            };
            if &curpath[pos..] == matchname {
                res.push(curpath.clone());
            } else {
                Self::scan_directory_recursive(res, matchname, curpath, maxdepth - 1);
            }
        }
    }

    /// Split path string `full` into its `base`-name and `path` (relative or
    /// absolute).  If there is no path, i.e. only a basename in `full`, then
    /// `path` will return as an empty string; otherwise `path` will be
    /// non-empty and end in a separator character.  (C++ `splitPath`.)
    pub fn split_path(full: &str, path: &mut String, base: &mut String) {
        let bytes = full.as_bytes();
        if bytes.is_empty() {
            // C++ would read full[size()-1] with a wrapped unsigned index
            // here; the observable outcome on the only call path
            // (discoverGhidraRoot's loop) is base == full == "" and an empty
            // path, which is what falls out of npos handling below.
            base.clear();
            path.clear();
            return;
        }
        // For full == "/" the C++ size_type wraps to a huge value;
        // find_last_of clamps the search to the whole string and the substr
        // count clamps to the string end.  `search_end`/`base_end` encode the
        // clamped values directly.
        let last_is_sep = Self::is_separator(bytes[bytes.len() - 1] as char);
        let (search_end, base_end) = if last_is_sep {
            if bytes.len() == 1 {
                (bytes.len() - 1, bytes.len()) // wrapped end, clamped
            } else {
                (bytes.len() - 2, bytes.len() - 1)
            }
        } else {
            (bytes.len() - 1, bytes.len())
        };
        let pos = bytes[..=search_end].iter().rposition(|&c| Self::is_separator(c as char));
        match pos {
            None => {
                // Didn't find any separator
                *base = full.to_string();
                path.clear();
            }
            Some(pos) => {
                *base = full[pos + 1..base_end].to_string();
                *path = full[..pos + 1].to_string();
            }
        }
    }

    /// Is the path absolute (begins with the separator)?  An empty string is
    /// not.  (C++ `isAbsolutePath`.)
    pub fn is_absolute_path(full: &str) -> bool {
        if full.is_empty() {
            return false;
        }
        Self::is_separator(full.as_bytes()[0] as char)
    }

    /// Build an absolute path using elements from `pathels`, in reverse
    /// order, up to and including `pathels[level]`.  (C++ `buildPath`.)
    fn build_path(pathels: &[String], level: i32) -> String {
        let mut s = String::new();
        let mut i = pathels.len() as i32 - 1;
        while i >= level {
            s.push(SEPARATOR);
            s.push_str(&pathels[i as usize]);
            i -= 1;
        }
        s
    }

    /// Given `pathels[level]` is "Ghidra", determine if this is a Ghidra
    /// development layout, returning the root through `root`.  (C++
    /// `testDevelopmentPath`.)
    fn test_development_path(pathels: &[String], level: i32, root: &mut String) -> bool {
        // C++: if (level + 2 >= pathels.size()) — int promoted to size_t in
        // the comparison; level is a nonnegative loop index in the only caller
        if (level + 2) as usize >= pathels.len() {
            return false;
        }
        let parent = &pathels[(level + 1) as usize];
        if parent.len() < 11 {
            return false;
        }
        if !parent.as_bytes().starts_with(b"ghidra.") {
            return false;
        }
        if !parent.as_bytes().ends_with(b".git") {
            return false;
        }
        *root = Self::build_path(pathels, level + 2);
        let mut testpaths1: Vec<String> = Vec::new();
        let mut testpaths2: Vec<String> = Vec::new();
        Self::scan_directory_recursive(&mut testpaths1, "ghidra.git", root, 1);
        if testpaths1.len() != 1 {
            return false;
        }
        Self::scan_directory_recursive(&mut testpaths2, "Ghidra", &testpaths1[0], 1);
        testpaths2.len() == 1
    }

    /// Determine if `pathels[level]` ("Ghidra") sits inside an installed
    /// Ghidra layout, returning the root through `root`.  (C++
    /// `testInstallPath`.)
    fn test_install_path(pathels: &[String], level: i32, root: &mut String) -> bool {
        // mixed signed/unsigned comparison as above
        if (level + 1) as usize >= pathels.len() {
            return false;
        }
        *root = Self::build_path(pathels, level + 1);
        let mut testpaths1: Vec<String> = Vec::new();
        let mut testpaths2: Vec<String> = Vec::new();
        Self::scan_directory_recursive(&mut testpaths1, "server", root, 1);
        if testpaths1.len() != 1 {
            return false;
        }
        Self::scan_directory_recursive(&mut testpaths2, "server.conf", &testpaths1[0], 1);
        testpaths2.len() == 1
    }

    /// Find the root of the Ghidra distribution based on the current working
    /// directory and the passed-in executable path.  Returns an empty string
    /// when no root is discovered.  (C++ `discoverGhidraRoot`.)
    pub fn discover_ghidra_root(argv0: &str) -> String {
        let mut pathels: Vec<String> = Vec::new();
        let mut cur: String = argv0.to_string();
        let mut base = String::new();
        let mut skiplevel: i32 = 0;
        let is_abs = Self::is_absolute_path(&cur);

        loop {
            let sizebefore = cur.len();
            // C++ calls splitPath(cur,cur,base) with full/path aliased; base
            // is assigned before path is touched, so a snapshot reproduces it
            let full = cur.clone();
            Self::split_path(&full, &mut cur, &mut base);
            if cur.len() == sizebefore {
                break;
            }
            if base == "." {
                skiplevel += 1;
            } else if base == ".." {
                skiplevel += 2;
            }
            if skiplevel > 0 {
                skiplevel -= 1;
            } else {
                pathels.push(base.clone());
            }
        }
        if !is_abs {
            let mut curdir = FileManage::default();
            curdir.add_current_dir();
            // C++ indexes pathlist[0] unconditionally; an empty pathlist
            // (getcwd failure) is UB there and panics here (internal
            // invariant violation)
            cur = curdir.pathlist[0].clone();
            loop {
                let sizebefore = cur.len();
                let full = cur.clone();
                Self::split_path(&full, &mut cur, &mut base);
                if cur.len() == sizebefore {
                    break;
                }
                pathels.push(base.clone());
            }
        }

        // loop index is nonnegative
        for (i, el) in pathels.iter().enumerate() {
            if el != "Ghidra" {
                continue;
            }
            let mut root = String::new();
            if Self::test_development_path(&pathels, i as i32, &mut root) {
                return root;
            }
            if Self::test_install_path(&pathels, i as i32, &mut root) {
                return root;
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Unique scratch directory per test; removed by `TempTree::drop`.
    struct TempTree {
        root: PathBuf,
    }

    impl TempTree {
        fn new(tag: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("kuna_filemanage_{}_{}", tag, std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).unwrap();
            TempTree { root }
        }

        fn path(&self) -> &str {
            self.root.to_str().unwrap()
        }

        fn mkdir(&self, rel: &str) {
            fs::create_dir_all(self.root.join(rel)).unwrap();
        }

        fn mkfile(&self, rel: &str) {
            fs::write(self.root.join(rel), b"x").unwrap();
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn test_filemanage_match_list_dir_suffix_and_prefix() {
        let t = TempTree::new("matchdir");
        t.mkfile("x86.sla");
        t.mkfile("arm.sla");
        t.mkfile("notes.txt");
        t.mkfile(".hidden.sla");
        t.mkdir("sub");
        t.mkfile("sub/inner.sla");

        // suffix match, no dot files, no recursion into sub/
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, ".sla", true, t.path(), false);
        assert_eq!(
            sorted(res),
            vec![format!("{}/arm.sla", t.path()), format!("{}/x86.sla", t.path())]
        );

        // allowdot picks up the dot file
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, ".sla", true, t.path(), true);
        assert_eq!(
            sorted(res),
            vec![
                format!("{}/.hidden.sla", t.path()),
                format!("{}/arm.sla", t.path()),
                format!("{}/x86.sla", t.path())
            ]
        );

        // prefix match; directory entries participate like any dirent in C++
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, "x86", false, t.path(), false);
        assert_eq!(res, vec![format!("{}/x86.sla", t.path())]);
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, "sub", false, t.path(), false);
        assert_eq!(res, vec![format!("{}/sub", t.path())]);

        // a trailing separator on dirname is not doubled
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, "notes.txt", false, &format!("{}/", t.path()), false);
        assert_eq!(res, vec![format!("{}/notes.txt", t.path())]);

        // unreadable directory contributes nothing
        let mut res: Vec<String> = Vec::new();
        FileManage::match_list_dir(&mut res, ".sla", true, &format!("{}/nosuch", t.path()), false);
        assert!(res.is_empty());
    }

    #[test]
    fn test_filemanage_match_list_accumulates_across_paths() {
        let t = TempTree::new("matchlist");
        t.mkdir("a");
        t.mkdir("b");
        t.mkfile("a/one.ldefs");
        t.mkfile("b/two.ldefs");
        t.mkfile("b/skip.txt");

        let mut fm = FileManage::default();
        fm.add_dir_2_path(&format!("{}/a", t.path()));
        fm.add_dir_2_path(&format!("{}/b", t.path()));
        let mut res: Vec<String> = Vec::new();
        fm.match_list(&mut res, ".ldefs", true);
        assert_eq!(
            sorted(res),
            vec![format!("{}/a/one.ldefs", t.path()), format!("{}/b/two.ldefs", t.path())]
        );
    }

    #[test]
    fn test_filemanage_directory_list_dirs_only() {
        let t = TempTree::new("dirlist");
        t.mkdir("Adir");
        t.mkdir("Bdir");
        t.mkdir(".git");
        t.mkfile("plain.txt");

        let mut res: Vec<String> = Vec::new();
        FileManage::directory_list(&mut res, t.path(), false);
        assert_eq!(
            sorted(res),
            vec![format!("{}/Adir", t.path()), format!("{}/Bdir", t.path())]
        );

        // allowdot includes the dot directory
        let mut res: Vec<String> = Vec::new();
        FileManage::directory_list(&mut res, t.path(), true);
        assert_eq!(
            sorted(res),
            vec![
                format!("{}/.git", t.path()),
                format!("{}/Adir", t.path()),
                format!("{}/Bdir", t.path())
            ]
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_filemanage_directory_list_follows_symlinks() {
        let t = TempTree::new("dirlink");
        t.mkdir("real");
        t.mkfile("real/f");
        std::os::unix::fs::symlink(t.root.join("real"), t.root.join("link")).unwrap();
        std::os::unix::fs::symlink(t.root.join("real/f"), t.root.join("filelink")).unwrap();
        std::os::unix::fs::symlink(t.root.join("gone"), t.root.join("dangling")).unwrap();

        let mut res: Vec<String> = Vec::new();
        FileManage::directory_list(&mut res, t.path(), false);
        // symlink-to-dir counts (C++ stats through DT_LNK); symlink-to-file
        // and dangling links do not
        assert_eq!(
            sorted(res),
            vec![format!("{}/link", t.path()), format!("{}/real", t.path())]
        );
    }

    #[test]
    fn test_filemanage_scan_directory_recursive_depth_and_no_descend() {
        let t = TempTree::new("scan");
        t.mkdir("data/languages");
        t.mkdir("other/data");
        t.mkdir("data/data"); // a match nested under a match: not descended into

        // maxdepth 0: nothing scanned
        let mut res: Vec<String> = Vec::new();
        FileManage::scan_directory_recursive(&mut res, "data", t.path(), 0);
        assert!(res.is_empty());

        // depth 1: only direct children
        let mut res: Vec<String> = Vec::new();
        FileManage::scan_directory_recursive(&mut res, "data", t.path(), 1);
        assert_eq!(res, vec![format!("{}/data", t.path())]);

        // depth 2: other/data appears; data/data does NOT (matched parent is
        // recorded, not recursed into)
        let mut res: Vec<String> = Vec::new();
        FileManage::scan_directory_recursive(&mut res, "data", t.path(), 2);
        assert_eq!(
            sorted(res),
            vec![format!("{}/data", t.path()), format!("{}/other/data", t.path())]
        );

        let mut res: Vec<String> = Vec::new();
        FileManage::scan_directory_recursive(&mut res, "languages", t.path(), 2);
        assert_eq!(res, vec![format!("{}/data/languages", t.path())]);
    }

    #[test]
    fn test_filemanage_find_file_search_order_and_absolute() {
        let t = TempTree::new("findfile");
        t.mkdir("p1");
        t.mkdir("p2");
        t.mkfile("p2/target.spec");
        t.mkfile("p1/both.spec");
        t.mkfile("p2/both.spec");

        let mut fm = FileManage::default();
        fm.add_dir_2_path(&format!("{}/p1", t.path()));
        fm.add_dir_2_path(&format!("{}/p2", t.path()));

        let mut res = String::new();
        fm.find_file(&mut res, "target.spec");
        assert_eq!(res, format!("{}/p2/target.spec", t.path()));

        // first path in the list wins
        fm.find_file(&mut res, "both.spec");
        assert_eq!(res, format!("{}/p1/both.spec", t.path()));

        // not found: res cleared even if previously set
        res = "stale".to_string();
        fm.find_file(&mut res, "missing.spec");
        assert_eq!(res, "");

        // absolute name is used verbatim, ignoring the path list
        let abs = format!("{}/p2/target.spec", t.path());
        fm.find_file(&mut res, &abs);
        assert_eq!(res, abs);
        fm.find_file(&mut res, &format!("{}/p2/nope.spec", t.path()));
        assert_eq!(res, "");
    }

    #[test]
    fn test_filemanage_split_path_vectors() {
        let mut path = String::new();
        let mut base = String::new();

        FileManage::split_path("/a/b/c", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("/a/b/", "c"));

        // terminating separator is taken into account
        FileManage::split_path("/a/b/", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("/a/", "b"));

        // no separator: everything is the base, path empty
        FileManage::split_path("name", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("", "name"));

        // bare root: C++'s wrapped `end` clamps to an empty base
        FileManage::split_path("/", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("/", ""));

        // separator-terminated relative path with no other separator: the
        // C++ bounded find_last_of misses it, so base keeps the separator
        FileManage::split_path("a/", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("", "a/"));

        FileManage::split_path("", &mut path, &mut base);
        assert_eq!((path.as_str(), base.as_str()), ("", ""));
    }

    #[test]
    fn test_filemanage_is_absolute_path_and_separator() {
        assert!(!FileManage::is_absolute_path(""));
        assert!(FileManage::is_absolute_path("/x"));
        assert!(FileManage::is_absolute_path("/"));
        assert!(!FileManage::is_absolute_path("x/y"));
        assert!(FileManage::is_separator('/'));
        assert!(!FileManage::is_separator('\\')); // POSIX branch only
    }

    #[test]
    fn test_filemanage_is_directory() {
        let t = TempTree::new("isdir");
        t.mkdir("d");
        t.mkfile("f");
        assert!(FileManage::is_directory(t.path()));
        assert!(FileManage::is_directory(&format!("{}/d", t.path())));
        assert!(!FileManage::is_directory(&format!("{}/f", t.path())));
        assert!(!FileManage::is_directory(&format!("{}/nosuch", t.path())));
    }

    #[test]
    fn test_filemanage_add_dir_2_path_separator_handling() {
        let t = TempTree::new("adddir");
        t.mkfile("here.txt");
        let mut fm = FileManage::default();
        fm.add_dir_2_path(""); // ignored
        fm.add_dir_2_path(t.path()); // separator appended
        let mut res = String::new();
        fm.find_file(&mut res, "here.txt");
        assert_eq!(res, format!("{}/here.txt", t.path()));

        // already-terminated path is not double-separated
        let mut fm2 = FileManage::default();
        fm2.add_dir_2_path(&format!("{}/", t.path()));
        fm2.find_file(&mut res, "here.txt");
        assert_eq!(res, format!("{}/here.txt", t.path()));
    }

    #[test]
    fn test_filemanage_discover_ghidra_root_development_layout() {
        let t = TempTree::new("devroot");
        // argv0 path components: .../devroot/ghidra.master.git/Ghidra/bin/decomp
        t.mkdir("devroot/ghidra.master.git/Ghidra/bin");
        // the development witness tree: devroot/ghidra.git/Ghidra
        t.mkdir("devroot/ghidra.git/Ghidra");

        let argv0 = format!("{}/devroot/ghidra.master.git/Ghidra/bin/decomp", t.path());
        assert_eq!(
            FileManage::discover_ghidra_root(&argv0),
            format!("{}/devroot", t.path())
        );

        // "." path elements are skipped while walking argv0
        let dotted = format!("{}/devroot/ghidra.master.git/./Ghidra/bin/decomp", t.path());
        assert_eq!(
            FileManage::discover_ghidra_root(&dotted),
            format!("{}/devroot", t.path())
        );
    }

    #[test]
    fn test_filemanage_discover_ghidra_root_install_layout() {
        let t = TempTree::new("instroot");
        // argv0 path components: .../inst/Ghidra/bin/decomp ; install witness
        // is inst/server/server.conf (a directory, per the C++ scan)
        t.mkdir("inst/Ghidra/bin");
        t.mkdir("inst/server/server.conf");

        let argv0 = format!("{}/inst/Ghidra/bin/decomp", t.path());
        assert_eq!(FileManage::discover_ghidra_root(&argv0), format!("{}/inst", t.path()));
    }

    #[test]
    fn test_filemanage_discover_ghidra_root_not_found() {
        let t = TempTree::new("noroot");
        t.mkdir("plain/Ghidra/bin");
        let argv0 = format!("{}/plain/Ghidra/bin/decomp", t.path());
        assert_eq!(FileManage::discover_ghidra_root(&argv0), "");
        // no "Ghidra" element at all
        assert_eq!(FileManage::discover_ghidra_root("/usr/bin/decomp"), "");
    }
}
