//! Stock Ghidra 12.1.2 `IndexedLocalFileSystem` / `IndexedV1LocalFileSystem`.
//! The UTF-8 index starts at `/`, lists absolute logical folder paths and
//! indented `storage:name[:file-id]` entries, then ends with NEXT-ID and MD5.
//! Version 0 lacks the VERSION header and file ID column. Logical pathnames
//! are never physical path components: storage IDs alone locate item files.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Included, Unbounded};

#[derive(Default)]
pub(super) struct Index {
    folders: BTreeSet<String>,
    pub(super) items: BTreeMap<String, String>,
}

impl Index {
    pub(super) fn read(text: &str) -> Result<Self, String> {
        let mut lines = text.lines().enumerate();
        let Some((_, first)) = lines.next() else {
            return Err("empty filesystem index".into());
        };
        let versioned = match first {
            "VERSION=1" => true,
            "/" => false,
            _ => return Err(format!("unsupported filesystem index header {first:?}")),
        };
        let mut lines = (!versioned).then_some((0, first)).into_iter().chain(lines);
        let mut index = Self::default();
        let mut folder = None;
        let mut highest_storage = None;
        for (line_number, line) in lines.by_ref() {
            let bad_line = |why: &str| format!("line {}: {why}: {line:?}", line_number + 1);
            if let Some(id) = line.strip_prefix("NEXT-ID:") {
                let next_storage = hex(id).ok_or_else(|| bad_line("invalid NEXT-ID"))?;
                if highest_storage.is_some_and(|id| id >= next_storage) {
                    return Err(bad_line("NEXT-ID does not follow allocated storage IDs"));
                }
                if !index.folders.contains("/") {
                    return Err("missing root folder in filesystem index".into());
                }
                // The pinned Ghidra implementation calls MessageDigest.digest(bytes)
                // (which finalizes/resets) for each index row. Its footer is MD5(empty),
                // not a checksum of the index. Enforce the actual stock trailer.
                if !matches!(lines.next(), Some((_, "MD5:d41d8cd98f00b204e9800998ecf8427e")))
                    || lines.next().is_some()
                {
                    return Err("invalid stock filesystem index MD5 footer".into());
                }
                index.validate_storage_ids()?;
                return Ok(index);
            }
            if line.starts_with('/') {
                if line != "/" {
                    let (parent, _) = split_path(line).map_err(bad_line)?;
                    if !index.folders.contains(parent) {
                        return Err(bad_line("folder parent is absent"));
                    }
                }
                if !index.folders.insert(line.to_owned()) {
                    return Err(bad_line("duplicate folder path"));
                }
                folder = Some(line);
                continue;
            }

            let parent = folder.ok_or_else(|| bad_line("item precedes its folder"))?;
            let (storage, fields) = line
                .strip_prefix("  ")
                .and_then(|entry| entry.split_once(':'))
                .ok_or_else(|| bad_line("expected an indented item entry"))?;
            let id = storage_id(storage).ok_or_else(|| bad_line("invalid item storage ID"))?;
            let name = if let Some((name, file_id)) = fields.split_once(':') {
                if !versioned || !valid_file_id(file_id) {
                    return Err(bad_line("unexpected or invalid file ID"));
                }
                name
            } else {
                fields
            };
            if !valid_name(name) {
                return Err(bad_line("invalid item name"));
            }
            let path = format!("{}/{name}", parent.trim_end_matches('/'));
            if index.items.insert(path, storage.to_owned()).is_some() {
                return Err(bad_line("duplicate item path"));
            }
            highest_storage = highest_storage.max(Some(id));
        }
        Err("missing NEXT-ID and MD5 footer in filesystem index".into())
    }

    // An up-to-date closed project usually has no journal. A consistent
    // snapshot may have one: replay it in order, in memory only. Neither
    // backups nor storage files absent from the resulting index are candidates.
    pub(super) fn replay(&mut self, text: &str) -> Result<(), String> {
        if !text.is_empty() && !text.ends_with('\n') {
            return Err("index journal has an unterminated last record; use a closed snapshot".into());
        }
        for (line_number, line) in text.lines().enumerate() {
            self.apply_journal_line(line)
                .map_err(|why| format!("line {}: {why}: {line:?}", line_number + 1))?;
        }
        self.validate_storage_ids()
    }

    fn apply_journal_line(&mut self, line: &str) -> Result<(), &'static str> {
        let mut fields = line.split(':');
        let op = fields.next().unwrap_or_default();
        let a = fields.next().ok_or("missing journal operand")?;
        let b = fields.next();
        if fields.next().is_some() {
            return Err("too many journal operands");
        }
        match (op, b) {
            ("FADD", None) => {
                let (parent, _) = split_path(a)?;
                self.ensure_folders(parent);
                if !self.folders.insert(a.to_owned()) {
                    return Err("folder already exists");
                }
            }
            ("FDEL", None) => {
                split_path(a)?;
                if !self.folders.remove(a) {
                    return Err("folder does not exist");
                }
                let prefix = format!("{a}/");
                let range = (Included(prefix.as_str()), Unbounded);
                for path in paths_below(self.folders.range::<str, _>(range), &prefix) {
                    self.folders.remove(&path);
                }
                for path in paths_below(self.items.range::<str, _>(range).map(|(path, _)| path), &prefix) {
                    self.items.remove(&path);
                }
            }
            ("FMV", Some(dest)) => {
                split_path(a)?;
                let (parent, _) = split_path(dest)?;
                let prefix = format!("{a}/");
                if dest.starts_with(&prefix) || self.folders.contains(dest) {
                    return Err("invalid destination folder");
                }
                if !self.folders.remove(a) {
                    return Err("source folder does not exist");
                }
                self.ensure_folders(parent);
                self.folders.insert(dest.to_owned());
                let range = (Included(prefix.as_str()), Unbounded);
                for path in paths_below(self.folders.range::<str, _>(range), &prefix) {
                    self.folders.remove(&path);
                    self.folders.insert(format!("{dest}{}", &path[a.len()..]));
                }
                for path in paths_below(self.items.range::<str, _>(range).map(|(path, _)| path), &prefix) {
                    let storage = self.items.remove(&path).ok_or("source item does not exist")?;
                    self.items.insert(format!("{dest}{}", &path[a.len()..]), storage);
                }
            }
            ("IADD", Some(path)) => {
                let (parent, _) = split_path(path)?;
                storage_id(a).ok_or("invalid item storage ID")?;
                self.ensure_folders(parent);
                if self.items.insert(path.to_owned(), a.to_owned()).is_some() {
                    return Err("item already exists");
                }
            }
            ("IDEL", None) => {
                split_path(a)?;
                self.items.remove(a).ok_or("item does not exist")?;
            }
            ("IMV", Some(dest)) => {
                split_path(a)?;
                let (parent, _) = split_path(dest)?;
                if self.items.contains_key(dest) {
                    return Err("destination item already exists");
                }
                let storage = self.items.remove(a).ok_or("source item does not exist")?;
                self.ensure_folders(parent);
                self.items.insert(dest.to_owned(), storage);
            }
            ("IDSET", Some(id)) => {
                split_path(a)?;
                if !valid_file_id(id) || !self.items.contains_key(a) {
                    return Err("invalid file ID or missing target item");
                }
                // File IDs are not storage IDs and do not change path selection.
            }
            _ => return Err("unsupported journal record"),
        }
        Ok(())
    }

    fn ensure_folders(&mut self, mut path: &str) {
        while !self.folders.contains(path) {
            self.folders.insert(path.to_owned());
            let (parent, _) = path.rsplit_once('/').expect("validated absolute project pathname");
            path = if parent.is_empty() { "/" } else { parent };
        }
    }

    fn validate_storage_ids(&self) -> Result<(), String> {
        let mut ids = BTreeSet::new();
        for (path, storage) in &self.items {
            if !ids.insert(storage) {
                return Err(format!("storage ID {storage:?} is indexed more than once (at {path:?})"));
            }
        }
        Ok(())
    }
}

fn paths_below<'a>(paths: impl Iterator<Item = &'a String>, prefix: &str) -> Vec<String> {
    paths.take_while(|path| path.starts_with(prefix)).cloned().collect()
}

/// Return the logical folder and basename, without filesystem normalization.
pub(super) fn split_path(path: &str) -> Result<(&str, &str), &'static str> {
    let rest = path.strip_prefix('/').ok_or("project pathname must be absolute")?;
    if !rest.split('/').all(valid_name) {
        return Err("invalid project pathname component");
    }
    let (parent, name) = path.rsplit_once('/').ok_or("empty project pathname")?;
    Ok((if parent.is_empty() { "/" } else { parent }, name))
}

fn valid_name(name: &str) -> bool {
    !matches!(name, "" | "." | "..")
        && !name.chars().any(|ch| ch.is_control() || matches!(ch, '/' | '\\' | ':'))
}

fn valid_file_id(id: &str) -> bool {
    !id.is_empty() && !id.chars().any(|ch| ch.is_control() || ch == ':')
}

fn hex(s: &str) -> Option<i64> {
    if s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        i64::from_str_radix(s, 16).ok()
    } else {
        None
    }
}

fn storage_id(s: &str) -> Option<i64> {
    if s.len() < 8 || (s.len() > 8 && s.starts_with('0')) {
        return None;
    }
    hex(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOOTER: &str = "MD5:d41d8cd98f00b204e9800998ecf8427e\n";

    #[test]
    fn reads_v0_and_v1_names_and_paths() {
        for (header, id) in [("", ""), ("VERSION=1\n", ":7f002ad2d52541218497978")] {
            let index = Index::read(&format!(
                "{header}/\n  00000000:main{id}\n/nested\n  000001fa:résumé{id}\nNEXT-ID:1fb\n{FOOTER}"
            )).unwrap();
            assert_eq!(index.items["/main"], "00000000");
            assert_eq!(index.items["/nested/résumé"], "000001fa");
        }
    }

    #[test]
    fn rejects_invalid_paths_ids_and_footer() {
        for body in [
            "/\n/../bad\nNEXT-ID:1\n",
            "/\n  ../00000:escape\nNEXT-ID:1\n",
            "/\n  00000000:one\n  00000000:two\nNEXT-ID:1\n",
            "/\n  00000001:one\nNEXT-ID:1\n",
            "/\n  00000000:one\n",
        ] {
            assert!(Index::read(&format!("VERSION=1\n{body}{FOOTER}")).is_err(), "{body}");
        }
        assert!(Index::read("VERSION=1\n/\nNEXT-ID:0\nMD5:invalid\n").is_err());
    }

    #[test]
    fn reconstructs_final_journal_namespace_before_resolving_items() {
        let mut index = Index::read(&format!(
            "VERSION=1\n/\n/old\n  00000003:first\n/old/sub\n  00000004:second\nNEXT-ID:5\n{FOOTER}"
        )).unwrap();
        index.replay(concat!(
            "FMV:/old:/fresh/path\n",
            "IMV:/fresh/path/first:/first\n",
            "IDSET:/first:ignored-id\n",
            "FDEL:/fresh/path\n",
            "FADD:/new\n",
            "IADD:00000005:/new/third\n",
            "IADD:00000006:/new/fourth\n",
            "IDEL:/new/third\n",
        )).unwrap();
        assert_eq!(index.items, BTreeMap::from([
            ("/first".to_owned(), "00000003".to_owned()),
            ("/new/fourth".to_owned(), "00000006".to_owned()),
        ]));
        assert!(index.replay("IADD:00000007:/bad/../file\n").is_err());
    }
}
