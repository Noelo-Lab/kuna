//! (kuna `panicloc`) Rust panic `Location` records, typed and named.
//!
//! Every panic site passes `&'static core::panic::Location { file: &str, line:
//! u32, col: u32 }`. rustc emits one per site in read-only data, so a call reads
//! `panic_bounds_check(a2, a1, 0x51150)` with the source position hidden behind
//! the address. A record is recognized by its shape: the `file` pointer reaches
//! read-only text of `1..=0x1000` bytes that ends in `.rs` (rustc 1.89 counts a
//! trailing NUL in the length), `1 <= line <= 0xffffff` and `1 <= col <= 0xfff`;
//! columns are 1-based, so a zero there is the high half of a following pointer
//! or length (a `&str` table, a `(&str, usize)` pair, a `log` record's
//! padding). On a PIE the pointer half is an `R_*_RELATIVE` slot, so its value
//! is the relocation addend.
//!
//! Each record becomes a typed data object named after its position
//! (`panic_loc_src_main_rs_10_5`), and its file name a bounded string.

use std::collections::HashMap;
use std::rc::Rc;

use object::read::Object;
use object::RelocationFlags;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};

use super::kuna_strrecords::{is_text, read_word, ro_regions, RoRegion};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, StringFact, TypedDataFact};

/// One recognized `Location`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PanicLocation {
    pub addr: u64,
    pub file_addr: u64,
    pub file: String,
    pub line: u32,
    pub col: u32,
}

/// Every `Location`-shaped record in the image's read-only data.
pub fn scan(file: &object::File) -> Vec<PanicLocation> {
    let regions = ro_regions(file);
    let width: u64 = if file.is_64() { 8 } else { 4 };
    let relative = relative_addends(file);
    let mut out = Vec::new();
    for region in &regions {
        let n = region.data.len() as u64;
        let mut off = (width - region.vma % width) % width;
        while off + 2 * width + 8 <= n {
            let at = region.vma + off;
            off += width;
            if let Some(loc) = record_at(&regions, region, at, width, &relative) {
                out.push(loc);
                off += width + 8;
            }
        }
    }
    out
}

fn record_at(
    regions: &[RoRegion],
    region: &RoRegion,
    at: u64,
    width: u64,
    relative: &HashMap<u64, u64>,
) -> Option<PanicLocation> {
    let raw = region.bytes(at, 2 * width + 8)?;
    let w = width as usize;
    let ptr = relative.get(&at).copied().unwrap_or_else(|| read_word(&raw[..w]));
    let len = read_word(&raw[w..2 * w]);
    let line = u32::from_le_bytes(raw[2 * w..2 * w + 4].try_into().ok()?);
    let col = u32::from_le_bytes(raw[2 * w + 4..2 * w + 8].try_into().ok()?);
    if !(1..=0x1000).contains(&len) || !(1..=0xff_ffff).contains(&line) || !(1..=0xfff).contains(&col) {
        return None;
    }
    let text = regions.iter().find(|r| r.contains(ptr))?.bytes(ptr, len)?;
    let text = text.strip_suffix(b"\0").unwrap_or(text);
    if !text.ends_with(b".rs") || !is_text(text) {
        return None;
    }
    Some(PanicLocation { addr: at, file_addr: ptr, file: String::from_utf8_lossy(text).into_owned(), line, col })
}

/// `slot -> addend` for every `R_*_RELATIVE` dynamic relocation.
fn relative_addends(file: &object::File) -> HashMap<u64, u64> {
    let Some((r_relative, _, _)) = crate::loader::kuna_dynrelocs::reloc_triple(file.architecture()) else {
        return HashMap::new();
    };
    file.dynamic_relocations()
        .into_iter()
        .flatten()
        .filter(|(_, r)| matches!(r.flags(), RelocationFlags::Elf { r_type } if r_type == r_relative))
        .map(|(off, r)| (off, r.addend() as u64))
        .collect()
}

/// The symbol a record is installed under: its position as an identifier.
pub fn location_name(loc: &PanicLocation) -> String {
    let tail: String = loc
        .file
        .rsplit(['/', '\\'])
        .take(2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("panic_loc_{tail}_{}_{}", loc.line, loc.col)
}

/// `struct core::panic::Location { &str file; uint32 line; uint32 col; }`.
fn location_type(tf: &dyn TypeFactory, width: i32) -> Option<Rc<Datatype>> {
    if let Some(t) = tf.find_by_name("core::panic::Location").ok().flatten() {
        return Some(t);
    }
    let usize_t = tf.get_base(width, type_metatype::TYPE_UINT).ok()?;
    let char_t = tf.get_type_char(1).ok()?;
    let data_t = tf.get_type_pointer(width, char_t, 1).ok()?;
    let str_t = match tf.find_by_name("&str").ok().flatten() {
        Some(t) => t,
        None => {
            let shell = tf.get_type_struct("&str").ok()?;
            let fields = vec![
                TypeField::new(0, 0, "data_ptr", data_t),
                TypeField::new(1, width, "length", usize_t),
            ];
            tf.set_fields_struct_raw(&shell, fields, Vec::new(), 2 * width, width, 0).ok()?
        }
    };
    let u32_t = tf.get_base(4, type_metatype::TYPE_UINT).ok()?;
    let shell = tf.get_type_struct("core::panic::Location").ok()?;
    let fields = vec![
        TypeField::new(0, 0, "file", str_t),
        TypeField::new(1, 2 * width, "line", Rc::clone(&u32_t)),
        TypeField::new(2, 2 * width + 4, "col", u32_t),
    ];
    tf.set_fields_struct_raw(&shell, fields, Vec::new(), 2 * width + 8, width, 0).ok()
}

/// (kuna `panicloc`) Registered for rustc-detected images; committed under
/// `--option panicloc`.
pub struct PanicLocationPass;

impl AnalysisPass for PanicLocationPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "panicloc"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let locs = scan(ctx.file);
        if locs.is_empty() {
            return AnalysisOutput::default();
        }
        let width = if ctx.file.is_64() { 8 } else { 4 };
        let mut out = AnalysisOutput::default();
        let ty = location_type(ctx.arch.types(), width);
        for loc in &locs {
            out.bounded_strings.push(StringFact { addr: loc.file_addr, len: loc.file.len() as u32 });
            if let Some(t) = &ty {
                out.typed_data.push(TypedDataFact { addr: loc.addr, name: location_name(loc), type_: Rc::clone(t) });
            }
        }
        out.bounded_strings.sort_by_key(|f| f.addr);
        out.bounded_strings.dedup_by_key(|f| f.addr);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_keep_the_last_two_path_components() {
        let loc = PanicLocation { addr: 0, file_addr: 0, file: "/rustc/abc/library/core/src/str/mod.rs".into(), line: 68, col: 5 };
        assert_eq!(location_name(&loc), "panic_loc_str_mod_rs_68_5");
        let loc = PanicLocation { addr: 0, file_addr: 0, file: "p.rs".into(), line: 3, col: 39 };
        assert_eq!(location_name(&loc), "panic_loc_p_rs_3_39");
    }

    #[test]
    fn record_shape_tests() {
        let text = b"src/main.rs\0";
        let mut data = vec![0u8; 0x40];
        data[..8].copy_from_slice(&0x2000u64.to_le_bytes());
        data[8..16].copy_from_slice(&11u64.to_le_bytes());
        data[16..20].copy_from_slice(&10u32.to_le_bytes());
        data[20..24].copy_from_slice(&5u32.to_le_bytes());
        let regions = vec![RoRegion { vma: 0x1000, data: &data }, RoRegion { vma: 0x2000, data: text }];
        let loc = record_at(&regions, &regions[0], 0x1000, 8, &HashMap::new()).expect("location");
        assert_eq!((loc.file.as_str(), loc.line, loc.col), ("src/main.rs", 10, 5));
        assert!(record_at(&regions, &regions[0], 0x1008, 8, &HashMap::new()).is_none());
    }

    #[test]
    fn a_zero_column_is_not_a_location_and_a_counted_nul_is_dropped() {
        let text = b"src/main.rs\0";
        let mut data = vec![0u8; 0x40];
        data[..8].copy_from_slice(&0x2000u64.to_le_bytes());
        data[8..16].copy_from_slice(&11u64.to_le_bytes());
        data[16..20].copy_from_slice(&0x2010u32.to_le_bytes());
        data[24..32].copy_from_slice(&0x2000u64.to_le_bytes());
        data[32..40].copy_from_slice(&12u64.to_le_bytes());
        data[40..44].copy_from_slice(&7u32.to_le_bytes());
        data[44..48].copy_from_slice(&3u32.to_le_bytes());
        let regions = vec![RoRegion { vma: 0x1000, data: &data }, RoRegion { vma: 0x2000, data: text }];
        assert!(record_at(&regions, &regions[0], 0x1000, 8, &HashMap::new()).is_none());
        let loc = record_at(&regions, &regions[0], 0x1018, 8, &HashMap::new()).expect("location");
        assert_eq!((loc.file.as_str(), loc.line, loc.col), ("src/main.rs", 7, 3));
    }
}
