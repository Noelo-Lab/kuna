use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use kuna_base::xml::xml_tree;

use super::index::{Index, split_path};

const MAX_INDEX_BYTES: u64 = 64 * 1024 * 1024;
const MAX_PROPERTY_BYTES: u64 = 1024 * 1024;

pub(super) struct Database {
    pub path: PathBuf,
    pub name: String,
    pub version: Option<i32>,
}

pub(super) type Candidate = (String, Result<Database, String>);

pub(super) fn discover(root: &Path) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let properties = root.join("project.prp");
    let local_versioned = match read_properties(&properties) {
        Ok(props) if props.contains_key("SERVER") => {
            candidates.push(("versioned".into(), Err(
                "this project uses a shared Ghidra repository; only local checkout bytes are in the snapshot".into(),
            )));
            false
        }
        Ok(_) => true,
        Err(error) => {
            candidates.push(("project.prp".into(), Err(error)));
            false
        }
    };

    let mut selected = BTreeMap::new();
    // GhidraFileData.getFolderItem(DEFAULT_VERSION) prefers the private item
    // (including a checkout or a hijacked file) over the same logical path in
    // the versioned namespace. Independent version counters are not compared.
    for area in ["versioned", "idata"] {
        if area == "versioned" && !local_versioned {
            continue;
        }
        let dir = root.join(area);
        match read_index(&dir) {
            Ok(index) => {
                for (path, id) in index.items {
                    selected.insert(path, (area, id));
                }
            }
            Err(error) => {
                if area == "idata" {
                    // The unreadable private namespace may shadow any versioned item.
                    selected.clear();
                }
                candidates.push((area.into(), Err(error)));
            }
        }
    }
    for (path, (area, id)) in selected {
        let item = locate_database(&root.join(area), &path, &id);
        candidates.push((path, item));
    }
    candidates
}

fn read_index(root: &Path) -> Result<Index, String> {
    check_kind(root, Kind::Directory)?;
    let path = root.join("~index.dat");
    let text = read_text(&path, MAX_INDEX_BYTES)?;
    let mut index = Index::read(&text).map_err(|e| at(&path, e))?;
    let path = root.join("~journal.dat");
    if optional_kind(&path, Kind::File)? {
        let journal = read_text(&path, MAX_INDEX_BYTES)?;
        index.replay(&journal).map_err(|e| at(&path, e))?;
    }
    Ok(index)
}

fn read_text(path: &Path, limit: u64) -> Result<String, String> {
    check_kind(path, Kind::File)?;
    let file = fs::File::open(path).map_err(|e| at(path, e))?;
    let metadata = file.metadata().map_err(|e| at(path, e))?;
    if !metadata.is_file() || metadata.len() > limit {
        return Err(at(path, "project metadata is not a regular file within the size limit"));
    }
    let mut text = String::new();
    file.take(limit + 1).read_to_string(&mut text).map_err(|e| at(path, e))?;
    if text.len() as u64 > limit {
        return Err(at(path, "project metadata exceeds the size limit"));
    }
    Ok(text)
}

fn locate_database(root: &Path, path: &str, id: &str) -> Result<Database, String> {
    let (parent, name) = split_path(path)?;
    if name.starts_with(".ghidra.") {
        return Err("unsupported temporary Ghidra item; use a closed, complete project snapshot".into());
    }
    // IndexedLocalFileSystem.getStorageDir uses the third- and second-last
    // hex digits. LocalFolderItem stores properties beside a hidden item DB.
    let bucket = root.join(&id[id.len() - 3..id.len() - 1]);
    check_kind(&bucket, Kind::Directory)?;
    let property = bucket.join(format!("{id}.prp"));
    let props = read_properties(&property)?;
    if props.get("NAME").map(String::as_str) != Some(name)
        || props.get("PARENT").map(String::as_str) != Some(parent)
    {
        return Err(at(&property, format!("NAME/PARENT do not match indexed item {path:?}")));
    }
    let content_type = props.get("CONTENT_TYPE").map_or("<missing>", String::as_str);
    let file_type = props.get("FILE_TYPE").map_or("<missing>", String::as_str);
    if content_type != "Program" || file_type != "0" {
        return Err(at(&property, format!(
            "unsupported item: CONTENT_TYPE={content_type:?}, FILE_TYPE={file_type:?}; expected a Program database",
        )));
    }

    let dir = bucket.join(format!("~{id}.db"));
    let (version, path) = latest_buffer_file(&dir)?;
    Ok(Database { path, name: name.to_owned(), version: Some(version) })
}

fn latest_buffer_file(dir: &Path) -> Result<(i32, PathBuf), String> {
    check_kind(dir, Kind::Directory)?;
    // db.Database.scanFiles selects the highest numeric version of the full
    // db.N.gbf files. ver.N.gbf and change.N.gbf are historical delta files.
    let mut latest = None;
    for entry in fs::read_dir(dir).map_err(|e| at(dir, e))? {
        let entry = entry.map_err(|e| at(dir, e))?;
        let filename = entry.file_name();
        let raw = filename.as_encoded_bytes();
        let Some(digits) = raw.strip_prefix(b"db.").and_then(|s| s.strip_suffix(b".gbf")) else {
            continue;
        };
        let version = std::str::from_utf8(digits).ok()
            .filter(|s| !s.starts_with('0') && s.bytes().all(|b| b.is_ascii_digit()))
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or_else(|| at(&entry.path(), "invalid Ghidra database version filename"))?;
        if latest.as_ref().is_none_or(|(previous, _)| version > *previous) {
            latest = Some((version, entry.path()));
        }
    }
    let (version, path) = latest.ok_or_else(|| at(dir, "missing saved database (db.N.gbf)"))?;
    check_kind(&path, Kind::File)?;
    Ok((version, path))
}

fn read_properties(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = read_text(path, MAX_PROPERTY_BYTES)?;
    let doc = xml_tree(text.as_bytes()).map_err(|e| at(path, e.explain()))?;
    let root = doc.get_root();
    let sections = root.get_children();
    if root.get_name() != "FILE_INFO" || sections.len() != 1 || sections[0].get_name() != "BASIC_INFO" {
        return Err(at(path, "expected a Ghidra FILE_INFO / BASIC_INFO property file"));
    }
    let mut props = BTreeMap::new();
    for state in sections[0].get_children() {
        if state.get_name() != "STATE" || !state.get_children().is_empty() {
            return Err(at(path, "expected only STATE properties in BASIC_INFO"));
        }
        let attr = |key| {
            let value = state.get_attribute_value(key).map_err(|e| at(path, e.explain()))?;
            std::str::from_utf8(value).map_err(|e| at(path, e))
        };
        let key = attr("NAME")?;
        let value = attr("VALUE")?;
        let property_type = attr("TYPE")?;
        let required_type = match key {
            "NAME" | "PARENT" | "CONTENT_TYPE" | "SERVER" => Some("string"),
            "FILE_TYPE" => Some("int"),
            _ => None,
        };
        if required_type.is_some_and(|expected| expected != property_type) {
            return Err(at(path, format!("invalid type {property_type:?} for property {key:?}")));
        }
        if props.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(at(path, format!("duplicate property {key:?}")));
        }
    }
    Ok(props)
}

#[derive(Clone, Copy)]
pub(super) enum Kind {
    File,
    Directory,
}

pub(super) fn check_kind(path: &Path, kind: Kind) -> Result<(), String> {
    if optional_kind(path, kind)? {
        Ok(())
    } else {
        Err(at(path, "missing from snapshot"))
    }
}

fn optional_kind(path: &Path, kind: Kind) -> Result<bool, String> {
    match fs::symlink_metadata(path) {
        Ok(meta) => match kind {
            Kind::File if meta.is_file() => Ok(true),
            Kind::Directory if meta.is_dir() => Ok(true),
            _ => Err(at(path, "unexpected file type (symlinks are not supported in snapshots)")),
        },
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(at(path, err)),
    }
}

pub(super) fn at(path: &Path, error: impl std::fmt::Display) -> String {
    format!("{}: {error}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn property(path: &Path, values: &[(&str, &str, &str)]) {
        let mut xml = String::from("<FILE_INFO><BASIC_INFO>");
        for (name, kind, value) in values {
            xml.push_str(&format!(r#"<STATE NAME="{name}" TYPE="{kind}" VALUE="{value}"/>"#));
        }
        xml.push_str("</BASIC_INFO></FILE_INFO>");
        fs::write(path, xml).unwrap();
    }

    fn index(dir: &Path, rows: &str) {
        fs::create_dir_all(dir).unwrap();
        fs::write(dir.join("~index.dat"), format!(
            "VERSION=1\n/\n{rows}NEXT-ID:100\nMD5:d41d8cd98f00b204e9800998ecf8427e\n",
        )).unwrap();
    }

    #[test]
    fn latest_indexed_checkout_overlays_versioned_and_invalid_items_are_reported() {
        let scratch = tempfile::tempdir().unwrap();
        let root = scratch.path();
        property(&root.join("project.prp"), &[]);
        let idata = root.join("idata");
        index(&idata, "  00000000:missing\n  00000001:unsupported\n/nested\n  000000fa:elf\n");
        index(&root.join("versioned"), "/nested\n  00000001:elf\n");
        fs::create_dir(idata.join("00")).unwrap();
        property(&idata.join("00/00000001.prp"), &[
            ("CONTENT_TYPE", "string", "Archive"), ("NAME", "string", "unsupported"),
            ("PARENT", "string", "/"), ("FILE_TYPE", "int", "0"),
        ]);
        let db = idata.join("0f/~000000fa.db");
        fs::create_dir_all(&db).unwrap();
        property(&idata.join("0f/000000fa.prp"), &[
            ("CONTENT_TYPE", "string", "Program"), ("NAME", "string", "elf"),
            ("PARENT", "string", "/nested"), ("FILE_TYPE", "int", "0"),
        ]);
        for filename in ["db.2.gbf", "db.10.gbf", "ver.11.gbf", "change.12.gbf"] {
            fs::write(db.join(filename), []).unwrap();
        }
        let candidates = discover(root);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0].0, "/missing");
        assert!(candidates[0].1.as_ref().is_err_and(|e| e.contains("missing from snapshot")));
        assert_eq!(candidates[1].0, "/nested/elf");
        let selected = candidates[1].1.as_ref().ok().unwrap();
        assert_eq!(selected.version, Some(10));
        assert_eq!(selected.path, db.join("db.10.gbf"));
        assert_eq!(candidates[2].0, "/unsupported");
        assert!(candidates[2].1.as_ref().is_err_and(|e| e.contains("unsupported item")));
    }

    #[test]
    fn unreadable_private_index_withholds_versioned_items() {
        let scratch = tempfile::tempdir().unwrap();
        let root = scratch.path();
        property(&root.join("project.prp"), &[]);
        index(&root.join("versioned"), "  00000000:potentially_shadowed\n");
        fs::create_dir(root.join("idata")).unwrap();
        fs::write(root.join("idata/~index.dat"), "invalid\n").unwrap();
        let candidates = discover(root);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0, "idata");
        assert!(candidates[0].1.is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_for_selected_version_without_using_old_bytes() {
        let scratch = tempfile::tempdir().unwrap();
        let dir = scratch.path();
        fs::write(dir.join("db.1.gbf"), []).unwrap();
        std::os::unix::fs::symlink(dir.join("other"), dir.join("db.2.gbf")).unwrap();
        assert!(latest_buffer_file(dir).is_err_and(|e| e.contains("symlinks")));
    }
}
