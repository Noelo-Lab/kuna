//! The kernel collections another collection was linked against.
//!
//! A System or Auxiliary collection points into the boot collection (and an
//! Auxiliary one into the System collection) with kernel-cache chained fixups
//! whose target is an offset from the other file's base. The collection's
//! `__PRELINK_INFO` plist names each file it needs by UUID (`_BootKCID`,
//! `_PageableKCID`); [`resolve`] finds the file carrying that UUID, rebases the
//! targets against its base and names the functions they reach from its symbol
//! tables. Gated by the `kernelcache-boot` option ([`KernelcacheBoot`]).

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::{Path, PathBuf};

use kuna_decomp::kuna_kernelcacheboot::KernelcacheBoot;

use super::fileset::{self, rd_u64};

/// Directory entries listed, and collections read out of them, when a
/// directory is searched.
const MAX_LISTED: usize = 4096;
const MAX_CANDIDATES: usize = 16;

const LC_UUID: u32 = 0x1b;
const N_STAB: u8 = 0xe0;
const N_TYPE: u8 = 0x0e;
const N_SECT: u8 = 0x0e;

/// Where the linked collections of the image being loaded may be read from.
pub struct LinkSearch {
    input: Option<PathBuf>,
    choice: KernelcacheBoot,
}

impl LinkSearch {
    /// Read no other file.
    pub fn none() -> Self {
        LinkSearch { input: None, choice: KernelcacheBoot::Off }
    }

    /// Search per `choice` for the collections `input` links.
    pub fn new(input: &Path, choice: KernelcacheBoot) -> Self {
        LinkSearch { input: Some(input.to_path_buf()), choice }
    }

    /// The files to try, in name order, and the place searched as a note names it.
    /// The input itself is never one of them.
    fn candidates(&self) -> (Vec<PathBuf>, String) {
        let real = |path: &Path| std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let input = self.input.as_deref().map(real);
        let not_input = |path: &Path| input.as_ref().is_none_or(|input| real(path) != *input);
        match &self.choice {
            KernelcacheBoot::Off => (Vec::new(), String::new()),
            KernelcacheBoot::Path(path) if !path.is_dir() => {
                (Some(path.clone()).filter(|p| not_input(p)).into_iter().collect(), format!("in {}", path.display()))
            }
            KernelcacheBoot::Path(dir) => (collections_in(dir, not_input), format!("in {}", dir.display())),
            KernelcacheBoot::Auto => {
                let dir = self.input.as_deref().and_then(Path::parent).filter(|d| !d.as_os_str().is_empty());
                (collections_in(dir.unwrap_or(Path::new(".")), not_input), "beside the input".to_string())
            }
        }
    }
}

/// The `MH_FILESET` regular files directly inside `dir` that `keep` accepts,
/// told by their first 16 bytes.
fn collections_in(dir: &Path, keep: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let Ok(listing) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut paths: Vec<PathBuf> = listing.take(MAX_LISTED).filter_map(|e| Some(e.ok()?.path())).collect();
    paths.sort();
    paths.retain(|path| {
        let mut head = [0u8; 16];
        path.is_file()
            && std::fs::File::open(path).and_then(|mut f| f.read_exact(&mut head)).is_ok()
            && fileset::is_fileset(&head)
            && keep(path)
    });
    paths.truncate(MAX_CANDIDATES);
    paths
}

/// A candidate's bytes, a firmware container unwrapped; `None` unless it is a
/// regular file and a 64-bit Mach-O within the decode limit.
fn read_collection(path: &Path) -> Option<Vec<u8>> {
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > super::DECODED_LIMIT as u64 {
        return None;
    }
    let mut bytes = Vec::new();
    std::fs::File::open(path).ok()?.take(meta.len()).read_to_end(&mut bytes).ok()?;
    let bytes = if super::is_container(&bytes) { super::unwrap_payload(&bytes).ok()?.0 } else { bytes };
    fileset::load_commands(&bytes, 0).map(|_| bytes)
}

/// What [`resolve`] did with a collection's pointers into other collections.
#[derive(Default)]
pub(super) struct Linked {
    /// `(slot, address)` of every pointer a linked file gave a base to.
    pub(super) rebases: Vec<(u64, u64)>,
    /// `(address, symbol name)` of each function among those addresses.
    pub(super) names: Vec<(u64, Vec<u8>)>,
    /// One clause per cache level, for the load note.
    pub(super) notes: Vec<String>,
}

fn level_name(level: u8) -> String {
    match level {
        0 => "the boot collection".to_string(),
        1 => "the System collection".to_string(),
        n => format!("cache level {n}"),
    }
}

/// Resolve `targets`, the `(slot, cache level, offset)` rebases of `collection`
/// that point into another collection, against the files `search` finds.
pub(super) fn resolve(collection: &[u8], targets: &[(u64, u8, u64)], search: &LinkSearch) -> Linked {
    let mut out = Linked::default();
    let mut counts: BTreeMap<u8, usize> = BTreeMap::new();
    for &(_, level, _) in targets {
        *counts.entry(level).or_default() += 1;
    }
    let info = prelink_info(collection).unwrap_or_default();
    let mut wanted: Vec<(u8, [u8; 16])> = Vec::new();
    let mut notes = Vec::new();
    let unresolved = |level: u8, why: String| {
        format!("{} pointers into {} left unresolved ({why})", counts[&level], level_name(level))
    };
    for &level in counts.keys() {
        let key = match level {
            0 => "_BootKCID",
            1 => "_PageableKCID",
            _ => {
                notes.push(unresolved(level, "no file is linked at that level".into()));
                continue;
            }
        };
        match kc_id(info, key) {
            Some(id) => wanted.push((level, id)),
            None => notes.push(unresolved(level, format!("the collection's plist carries no {key}"))),
        }
    }
    if search.choice == KernelcacheBoot::Off {
        for (level, _) in wanted.drain(..) {
            notes.push(unresolved(level, "kernelcache-boot is off".into()));
        }
    }
    let (candidates, place) = if wanted.is_empty() { Default::default() } else { search.candidates() };
    let mut found: BTreeMap<u8, (PathBuf, Vec<u8>)> = BTreeMap::new();
    for path in candidates {
        if found.len() == wanted.len() {
            break;
        }
        let Some(bytes) = read_collection(&path) else { continue };
        let ids = collection_ids(&bytes);
        let hit = wanted.iter().find(|(level, id)| ids.contains(id) && !found.contains_key(level));
        if let Some(&(level, _)) = hit {
            found.insert(level, (path, bytes));
        }
    }
    for (level, id) in wanted {
        let Some((path, bytes)) = found.get(&level) else {
            notes.push(unresolved(
                level,
                format!("no collection with UUID {} {place}; --option kernelcache-boot PATH names one", uuid_text(&id)),
            ));
            continue;
        };
        let Some(base) = fileset::image_base(bytes) else {
            notes.push(unresolved(level, format!("{} maps no segment", path.display())));
            continue;
        };
        let mut addresses = BTreeSet::new();
        for &(slot, _, offset) in targets.iter().filter(|t| t.1 == level) {
            let address = base.wrapping_add(offset);
            out.rebases.push((slot, address));
            addresses.insert(address);
        }
        let names = name_functions(bytes, &addresses);
        notes.push(format!(
            "{} pointers into {} rebased from {} ({} functions named)",
            counts[&level],
            level_name(level),
            path.file_name().unwrap_or(path.as_os_str()).to_string_lossy(),
            names.len()
        ));
        out.names.extend(names);
    }
    out.notes = notes;
    out
}

/// The bytes of a collection's `__PRELINK_INFO` segment.
fn prelink_info(bytes: &[u8]) -> Option<&[u8]> {
    fileset::load_commands(bytes, 0)?.into_iter().find_map(|(cmd, p)| {
        if cmd != fileset::LC_SEGMENT_64 || !bytes.get(p + 8..p + 24)?.starts_with(b"__PRELINK_INFO\0") {
            return None;
        }
        let off = usize::try_from(rd_u64(bytes, p + 40)?).ok()?;
        bytes.get(off..off.checked_add(usize::try_from(rd_u64(bytes, p + 48)?).ok()?)?)
    })
}

/// The 16-byte `<data>` value a prelink plist gives `key`.
fn kc_id(info: &[u8], key: &str) -> Option<[u8; 16]> {
    let key = format!("<key>{key}</key>");
    let at = info.windows(key.len()).position(|w| w == key.as_bytes())? + key.len();
    let rest = &info[at..];
    let rest = &rest[rest.iter().position(|b| !b.is_ascii_whitespace())?..];
    let rest = rest.strip_prefix(b"<data>")?;
    let text = &rest[..rest.iter().take(64).position(|&b| b == b'<')?];
    let mut out = Vec::with_capacity(18);
    let (mut acc, mut bits) = (0u32, 0);
    for &c in text.iter().filter(|c| !c.is_ascii_whitespace() && **c != b'=') {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    out.try_into().ok()
}

/// The UUIDs a collection answers to: its `LC_UUID` and its plist's
/// `_PrelinkKCID`.
fn collection_ids(bytes: &[u8]) -> Vec<[u8; 16]> {
    let uuid = fileset::load_commands(bytes, 0).into_iter().flatten().find_map(|(cmd, p)| {
        let id: [u8; 16] = bytes.get(p + 8..p + 24)?.try_into().ok()?;
        (cmd == LC_UUID).then_some(id)
    });
    let kcid = prelink_info(bytes).and_then(|info| kc_id(info, "_PrelinkKCID"));
    uuid.into_iter().chain(kcid).collect()
}

fn uuid_text(id: &[u8; 16]) -> String {
    let hex = |range: std::ops::Range<usize>| id[range].iter().map(|b| format!("{b:02X}")).collect::<String>();
    format!("{}-{}-{}-{}-{}", hex(0..4), hex(4..6), hex(6..8), hex(8..10), hex(10..16))
}

/// The symbol of each address in `wanted` that a section symbol of `bytes`
/// defines inside an instruction-bearing section: the kernel's symbol first,
/// then the first kext's. Each distinct entry header is read once, within the
/// [`fileset::bind_work`] budget.
fn name_functions(bytes: &[u8], wanted: &BTreeSet<u64>) -> Vec<(u64, Vec<u8>)> {
    let mut headers: Vec<(bool, usize)> = match fileset::entries(bytes) {
        Some(all) => all.iter().map(|e| (e.filetype != fileset::MH_EXECUTE, e.fileoff as usize)).collect(),
        None => vec![(false, 0)],
    };
    headers.sort();
    headers.dedup_by_key(|h| h.1);
    let mut work = fileset::bind_work(bytes);
    let mut found: BTreeMap<u64, Vec<u8>> = BTreeMap::new();
    for (_, at) in headers {
        if work == 0 || found.len() == wanted.len() {
            break;
        }
        let Some(cmds) = fileset::load_commands(bytes, at) else { continue };
        work = work.saturating_sub(cmds.len());
        let (Some(syms), Some(code)) = (fileset::symbols(bytes, &cmds), fileset::code_ranges(bytes, &cmds, &mut work))
        else {
            continue;
        };
        for i in 0..syms.count {
            if work == 0 {
                break;
            }
            work -= 1;
            let at = syms.symoff + i * fileset::NLIST_LEN;
            let Some(&n_type) = bytes.get(at + 4) else { break };
            if n_type & N_STAB != 0 || n_type & N_TYPE != N_SECT {
                continue;
            }
            let Some(addr) = rd_u64(bytes, at + 8) else { break };
            if !wanted.contains(&addr) || found.contains_key(&addr) || !code.iter().any(|r| r.contains(&addr)) {
                continue;
            }
            if let Some(name) = fileset::nlist_name(bytes, &syms, i, &mut work) {
                found.insert(addr, name.to_vec());
            }
        }
    }
    found.into_iter().collect()
}
