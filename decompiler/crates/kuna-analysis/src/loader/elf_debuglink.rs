//! An ELF's separate debug information: the companion file its build-id or
//! `.gnu_debuglink` names (option `debuglink`).
//!
//! A distribution strips its binaries and ships the symbol table and DWARF in a
//! separate `.debug` file (`debuginfod`, `-dbg`/`-debuginfo` packages). The image
//! names that file two ways, searched the way gdb searches them:
//!
//! - the GNU build-id note: `<dir>/.build-id/<xx>/<rest>.debug`, the first byte
//!   of the id as two hex digits and the rest after it;
//! - `.gnu_debuglink` (a file name plus a CRC32 of the file): next to the image,
//!   in its `.debug/` directory, and under `<dir>/<image's directory>/`.
//!
//! `<dir>` is each entry of `KUNA_DEBUG_DIRS` (colon-separated), then
//! `/usr/lib/debug`. A candidate is accepted only if its build-id matches, or
//! (for a debuglink) its CRC32 does.

use std::path::{Path, PathBuf};

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

/// A companion larger than this is not read.
const MAX_COMPANION_BYTES: u64 = 1 << 30;

fn debug_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var("KUNA_DEBUG_DIRS")
        .map(|v| v.split(':').filter(|d| !d.is_empty()).map(PathBuf::from).collect())
        .unwrap_or_default();
    dirs.push(PathBuf::from("/usr/lib/debug"));
    dirs
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// How a candidate proves it belongs to the image.
#[derive(Debug, Clone)]
enum Check {
    BuildId(Vec<u8>),
    Crc(u32),
}

/// Where the image's build-id and debuglink say its debug file may be, in
/// search order, each with the check it must pass.
fn candidates(file: &object::File, image: &Path) -> Vec<(PathBuf, Check)> {
    let mut out = Vec::new();
    if let Ok(Some(id)) = file.build_id() {
        if id.len() >= 2 {
            for dir in debug_dirs() {
                let path = dir.join(".build-id").join(hex(&id[..1])).join(format!("{}.debug", hex(&id[1..])));
                out.push((path, Check::BuildId(id.to_vec())));
            }
        }
    }
    if let Ok(Some((name, crc))) = file.gnu_debuglink() {
        let name = String::from_utf8_lossy(name).into_owned();
        let mut parts = Path::new(&name).components();
        if !matches!((parts.next(), parts.next()), (Some(std::path::Component::Normal(_)), None)) {
            return out;
        }
        let parent = image.parent().unwrap_or(Path::new("."));
        let absolute = std::fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
        out.push((parent.join(&name), Check::Crc(crc)));
        out.push((parent.join(".debug").join(&name), Check::Crc(crc)));
        for dir in debug_dirs() {
            let relative = absolute.strip_prefix("/").unwrap_or(&absolute);
            out.push((dir.join(relative).join(&name), Check::Crc(crc)));
        }
    }
    out
}

/// The first candidate that exists and passes its check, with its bytes.
pub fn find_companion(file: &object::File, image: &Path) -> Option<(PathBuf, Vec<u8>)> {
    for (path, check) in candidates(file, image) {
        if path == image || !std::fs::metadata(&path).is_ok_and(|m| m.is_file() && m.len() <= MAX_COMPANION_BYTES) {
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let ok = match &check {
            Check::BuildId(id) => object::File::parse(&*bytes)
                .ok()
                .and_then(|companion| companion.build_id().ok().flatten().map(|other| other == id.as_slice()))
                .unwrap_or(false),
            Check::Crc(crc) => {
                let mut sum = flate2::Crc::new();
                sum.update(&bytes);
                sum.sum() == *crc
            }
        };
        if ok {
            return Some((path, bytes));
        }
    }
    None
}

/// The defined function symbols of an ELF given as bytes, as `(addr, name)`.
fn function_symbols(bytes: &[u8]) -> Vec<(u64, Vec<u8>)> {
    let Ok(file) = object::File::parse(bytes) else { return Vec::new() };
    file.symbols()
        .filter(|s| s.kind() == SymbolKind::Text && !s.is_undefined() && s.address() != 0)
        .filter_map(|s| {
            let name = s.name_bytes().ok()?;
            (!name.is_empty()).then(|| (s.address(), name.to_vec()))
        })
        .collect()
}

/// The function symbols a stripped image's debug companion supplies, and the
/// companion's path. Empty when the image keeps its own `.symtab`.
pub fn companion_symbols(file: &object::File, image: Option<&Path>) -> (Vec<(u64, Vec<u8>)>, Option<String>) {
    if file.format() != object::BinaryFormat::Elf || file.section_by_name(".symtab").is_some() {
        return (Vec::new(), None);
    }
    match image.and_then(|image| find_companion(file, image)) {
        Some((path, bytes)) => (function_symbols(&bytes), Some(path.display().to_string())),
        None => (Vec::new(), None),
    }
}
