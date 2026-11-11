//! (kuna) The load-time fixups a byte view overlaps, and where its bytes came
//! from (GH #1379).
//!
//! `kuna read` shows the load image, the bytes every other surface reads: a
//! Mach-O chained-fixup rebase or an ELF `RELATIVE` slot holds the pointer the
//! loader stores, not the chain word or zero the file holds, and a bind holds a
//! zero where the loader writes another image's address
//! (`kuna_analysis::loader::kuna_dynrelocs`). That is the right view of a
//! pointer and the wrong one to take for the file's bytes, and nothing in the
//! output said which a slot was. So every fixup slot the listed span overlaps is
//! reported with its kind, the symbol it binds, the file's own bytes and what
//! the listed bytes are there: the file's, the loader's statically known value,
//! or kuna's zero placeholder.

use kuna_analysis::loader::kuna_dataptrs::Coverage;
use kuna_analysis::loader::kuna_dynrelocs::{FixupKind, FixupSite};
use kuna_console::disasm::hex;
use kuna_console::engine::ConsoleProgram;
use object::read::{Object, ObjectSegment};

use crate::jsonfmt::Json;

/// How many slots the note names before it counts the rest.
const NOTE_SLOTS: usize = 3;

/// What the listed bytes hold at a fixup slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Returned {
    /// The file's own bytes: kuna rewrote nothing here.
    File,
    /// The value the loader stores, which this image determines.
    Applied,
    /// A zero standing in for an address the loader looks up at run time.
    Placeholder,
}

impl Returned {
    fn as_str(self) -> &'static str {
        match self {
            Returned::File => "file",
            Returned::Applied => "applied",
            Returned::Placeholder => "placeholder",
        }
    }
}

/// One fixup slot the span overlaps.
struct Slot {
    site: FixupSite,
    /// The file's bytes at the slot; `None` where the segment's file content
    /// ends before it (zero fill).
    file: Option<Vec<u8>>,
    image: Vec<u8>,
    returned: Returned,
    /// The listed bytes already are `target`, the value the loader stores.
    holds_target: bool,
}

/// The fixup slots one byte view overlaps, and the metadata that listed them.
pub(super) struct Fixups {
    coverage: Coverage,
    slots: Vec<Slot>,
}

/// Every fixup slot of the image that `[start, end)` overlaps, whole. `view` is
/// the image's bytes and object view.
///
/// An image whose object view cannot be read again lists none, under
/// `coverage` `none`, as does one whose fixups kuna does not read.
pub(super) fn overlapping(view: Option<(&[u8], &object::File)>, prog: &ConsoleProgram, start: u64, end: u64) -> Fixups {
    let none = Fixups { coverage: Coverage::None, slots: Vec::new() };
    if end <= start {
        return none;
    }
    let Some((bytes, file)) = view else { return none };
    let listed = kuna_analysis::loader::kuna_dynrelocs::fixup_sites(file, bytes);
    let le = file.is_little_endian();
    let mut slots = Vec::new();
    for site in listed.sites {
        let width = u64::from(site.width);
        if site.vma >= end || site.vma.saturating_add(width) <= start {
            continue;
        }
        let Some(image) = prog.read_bytes(site.vma, usize::from(site.width)) else { continue };
        let file_bytes = file_bytes(file, site.vma, width);
        let unchanged = file_bytes.as_deref().unwrap_or(&vec![0; image.len()])[..] == image[..];
        let returned = if unchanged {
            Returned::File
        } else if site.kind == FixupKind::Bind && site.target.is_none() && image.iter().all(|&b| b == 0) {
            Returned::Placeholder
        } else {
            Returned::Applied
        };
        let holds_target = site.target.is_some_and(|t| encode(t, image.len(), le) == image);
        slots.push(Slot { site, file: file_bytes, image, returned, holds_target });
    }
    Fixups { coverage: listed.coverage, slots }
}

/// `value` as a `width`-byte word in the image's byte order.
fn encode(value: u64, width: usize, le: bool) -> Vec<u8> {
    let width = width.min(8);
    if le { value.to_le_bytes()[..width].to_vec() } else { value.to_be_bytes()[8 - width..].to_vec() }
}

/// The file's `width` bytes at `vma`, from the segment holding them.
fn file_bytes(file: &object::File, vma: u64, width: u64) -> Option<Vec<u8>> {
    file.segments().find_map(|seg| seg.data_range(vma, width).ok().flatten().map(<[u8]>::to_vec))
}

impl Fixups {
    /// The stderr / `notes` line for a span that overlaps a slot whose bytes are
    /// not simply the file's pointer: anything but a rebase whose file bytes
    /// already hold its target.
    pub(super) fn note(&self) -> Option<String> {
        let flagged: Vec<&Slot> = self
            .slots
            .iter()
            .filter(|s| s.returned != Returned::File || s.site.kind != FixupKind::Rebase || !s.holds_target)
            .collect();
        if flagged.is_empty() {
            return None;
        }
        let mut parts: Vec<String> = flagged.iter().take(NOTE_SLOTS).map(|s| s.describe()).collect();
        if flagged.len() > NOTE_SLOTS {
            parts.push(format!("and {} more", flagged.len() - NOTE_SLOTS));
        }
        let what = if flagged.len() == 1 { "a load-time fixup slot" } else { "load-time fixup slots" };
        Some(format!(
            "these bytes overlap {what}: {} -- `--json` lists each in `fixups`",
            parts.join("; ")
        ))
    }

    /// `fixups_coverage` and `fixups`, for the byte view's JSON document.
    pub(super) fn json_fields(&self) -> Vec<(String, Json)> {
        let number = |v: u64| Json::Number(v.to_string());
        let rows = self
            .slots
            .iter()
            .map(|s| {
                Json::Object(vec![
                    ("address".into(), number(s.site.vma)),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", s.site.vma))),
                    ("size".into(), number(u64::from(s.site.width))),
                    ("kind".into(), Json::Str(s.site.kind.as_str().into())),
                    ("symbol".into(), s.site.symbol.clone().map_or(Json::Null, Json::Str)),
                    ("addend".into(), s.site.addend.map_or(Json::Null, |a| Json::Number(a.to_string()))),
                    ("target".into(), s.site.target.map_or(Json::Null, number)),
                    ("target_hex".into(), s.site.target.map_or(Json::Null, |t| Json::Str(format!("0x{t:x}")))),
                    ("file_bytes".into(), s.file.as_deref().map_or(Json::Null, |b| Json::Str(hex(b)))),
                    ("image_bytes".into(), Json::Str(hex(&s.image))),
                    ("returned".into(), Json::Str(s.returned.as_str().into())),
                ])
            })
            .collect();
        vec![
            ("fixups_coverage".into(), Json::Str(self.coverage.as_str().into())),
            ("fixups".into(), Json::Array(rows)),
        ]
    }
}

impl Slot {
    /// `0x4000 bind _malloc: kuna's zero placeholder, the file holds ..`.
    fn describe(&self) -> String {
        let site = &self.site;
        let mut head = format!("0x{:x} {}", site.vma, site.kind.as_str());
        if let Some(symbol) = &site.symbol {
            head = format!("{head} {symbol}");
        }
        let file = self.file.as_deref().map_or("zero fill".to_string(), hex);
        let what = match (self.returned, site.target) {
            (Returned::Placeholder, _) => {
                format!("kuna's zero placeholder for an address the loader looks up at run time; the file holds {file}")
            }
            (Returned::Applied, Some(t)) => format!("the pointer the loader stores, 0x{t:x}; the file holds {file}"),
            (Returned::Applied, None) => format!("the value the loader stores; the file holds {file}"),
            (Returned::File, Some(_)) if self.holds_target => {
                "the file's bytes, already the value the loader stores".to_string()
            }
            (Returned::File, Some(t)) => format!("the file's bytes; the loader stores 0x{t:x}"),
            (Returned::File, None) => "the file's bytes; the loader rewrites them at run time".to_string(),
        };
        format!("{head} ({what})")
    }
}
