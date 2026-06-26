//! Unit tests for the DWARF `.debug_line` source-line parse ([`DwarfLinesPass`]).
//!
//! `cet_pie_x86_64` carries a `.debug_line` program for `debug_symbol.c`. The
//! objdump-decoded line table pins these PC→line rows (the values this test
//! asserts the gimli row-walk recovers):
//!
//! ```text
//! debug_symbol.c:122  0x1357   <- elaborate_debug_symbol entry (DW_AT_low_pc)
//! debug_symbol.c:124  0x1367
//! debug_symbol.c:125  0x1381
//! ...
//! (end_sequence)      0x1464   <- past-the-end, line `-`, MUST be skipped
//! ```

use super::*;
use std::collections::BTreeMap;

/// Parse the `.debug_line` of a fixture and return the recovered
/// `(addr -> "file:line")` comment map, keyed by instruction VMA.
///
/// Drives the pass's pure row-walk helpers (`func_ranges` + `collect_line_comments`)
/// directly — the `run` body's only extra work is building/loading the gimli
/// sections (done here) and stable-sorting (irrelevant for a by-address map).
fn line_comments(path: &str) -> BTreeMap<u64, String> {
    let bytes = std::fs::read(path).expect("read fixture");
    let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
    let endian = if file.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
        Ok(super::dwarf_section_data(&file, id).unwrap_or_default())
    };
    let sections = gimli::DwarfSections::load(load).expect("load dwarf");
    let dwarf = sections.borrow(|b| gimli::EndianSlice::new(b, endian));

    let mut out = Vec::new();
    let mut units = dwarf.units();
    while let Ok(Some(header)) = units.next() {
        let unit = dwarf.unit(header).expect("unit");
        let ranges = func_ranges(&dwarf, &unit);
        collect_line_comments(&dwarf, &unit, &ranges, &mut out);
    }
    out.into_iter().map(|c| (c.addr, c.text)).collect()
}

#[test]
fn cet_pie_debug_line_maps_pc_to_source_line() {
    // The headline assertion: 0x1357 (elaborate_debug_symbol's entry) maps to
    // debug_symbol.c:122, and the next rows follow the objdump-decoded table.
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cet_pie_x86_64");
    let by_addr = line_comments(path);

    assert_eq!(
        by_addr.get(&0x1357).map(String::as_str),
        Some("debug_symbol.c:122"),
        "0x1357 (elaborate_debug_symbol entry) should map to debug_symbol.c:122; got {:?}",
        by_addr.get(&0x1357)
    );
    assert_eq!(
        by_addr.get(&0x1367).map(String::as_str),
        Some("debug_symbol.c:124"),
        "0x1367 should map to debug_symbol.c:124; got {:?}",
        by_addr.get(&0x1367)
    );
    assert_eq!(
        by_addr.get(&0x1381).map(String::as_str),
        Some("debug_symbol.c:125"),
        "0x1381 should map to debug_symbol.c:125; got {:?}",
        by_addr.get(&0x1381)
    );
}

#[test]
fn end_sequence_row_is_skipped() {
    // 0x1464 is the line table's `end_sequence` row (objdump line `-`): a
    // past-the-end marker, not a statement. It must NOT produce a comment.
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cet_pie_x86_64");
    let by_addr = line_comments(path);
    assert!(
        !by_addr.contains_key(&0x1464),
        "the end_sequence row at 0x1464 must be skipped, got: {:?}",
        by_addr.get(&0x1464)
    );
}

#[test]
fn comments_attach_to_owning_function_entry() {
    // Each row is bucketed onto the subprogram whose [low_pc,high_pc) contains it,
    // so the printer's per-function `comments_for(funcaddr)` retrieves it. Confirm
    // elaborate_debug_symbol's range starts at its DW_AT_low_pc (0x1357).
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/cet_pie_x86_64");
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let endian = gimli::RunTimeEndian::Little;
    let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
        Ok(super::dwarf_section_data(&file, id).unwrap_or_default())
    };
    let sections = gimli::DwarfSections::load(load).unwrap();
    let dwarf = sections.borrow(|b| gimli::EndianSlice::new(b, endian));
    let mut units = dwarf.units();
    let mut ranges = Vec::new();
    while let Ok(Some(header)) = units.next() {
        let unit = dwarf.unit(header).unwrap();
        ranges.extend(func_ranges(&dwarf, &unit));
    }
    // elaborate_debug_symbol's range starts at 0x1357 (its DW_AT_low_pc).
    assert!(
        ranges.iter().any(|r| r.low == 0x1357 && r.high > 0x1357),
        "expected a subprogram range starting at 0x1357, got {} ranges",
        ranges.len()
    );
}
