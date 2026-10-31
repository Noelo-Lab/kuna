//! Tests for the exception-table reader `option bejoin callers` uses.

use super::*;

fn image(parts: &[(u64, Vec<u8>)]) -> impl FnMut(u64, &mut [u8]) -> bool + '_ {
    move |at, buf| {
        for (i, b) in buf.iter_mut().enumerate() {
            let a = at + i as u64;
            let Some((base, bytes)) = parts.iter().find(|(base, bytes)| a >= *base && a < base + bytes.len() as u64) else {
                return false;
            };
            *b = bytes[(a - base) as usize];
        }
        true
    }
}

fn code(a: u64) -> bool {
    (0x2000..0x2100).contains(&a)
}

/// A `zPLR` CIE at 0x1000 and one FDE for 0x2000 whose LSDA at 0x3000 names
/// pads at +0x10 and +0x30.
fn dwarf(pad2: u8) -> Vec<(u64, Vec<u8>)> {
    let mut eh = vec![0, 0, 0, 24, 0, 0, 0, 0, 1, b'z', b'P', b'L', b'R', 0, 4, 0x7c, 0x41, 7, 0, 0, 0, 0, 0, 0, 0x1b, 0, 0, 0];
    eh.extend([0, 0, 0, 20, 0, 0, 0, 0x20, 0, 0, 0x0f, 0xdc, 0, 0, 0, 0x40, 4, 0, 0, 0x30, 0, 0, 0, 0]);
    eh.extend([0, 0, 0, 0]);
    let lsda = vec![0xff, 0xff, 0x01, 12, 0x04, 0x08, 0x10, 0, 0x10, 4, 0, 0, 0x20, 4, pad2, 1];
    vec![(0x1000, eh), (0x3000, lsda)]
}

fn sections(names: &[(&str, u64, u64)]) -> Vec<(String, u64, u64)> {
    names.iter().map(|&(n, a, s)| (n.to_string(), a, s)).collect()
}

#[test]
fn an_fde_lsda_names_its_landing_pads() {
    let parts = dwarf(0x30);
    let secs = sections(&[(".eh_frame", 0x1000, 56), (".gcc_except_table", 0x3000, 16)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert_eq!(got, Landing { pads: vec![0x2010, 0x2030], unknown: vec![], blind: false });
}

#[test]
fn a_pad_outside_the_code_leaves_its_function_unknown() {
    let parts = dwarf(0x7f);
    let secs = sections(&[(".eh_frame", 0x1000, 56), (".gcc_except_table", 0x3000, 16)]);
    let mut big = parts.clone();
    big[1].1[14] = 0xff;
    big[1].1.insert(15, 0x7f);
    big[1].1[3] = 13;
    let got = read(Some(&secs), false, false, &code, &mut image(&big));
    assert_eq!(got, Landing { pads: vec![], unknown: vec![0x2000], blind: false });
}

#[test]
fn an_fde_for_no_code_blinds_an_image_with_tables() {
    let mut parts = dwarf(0x30);
    parts[0].1[39] = 0x00;
    parts[0].1[38] = 0x00;
    let secs = sections(&[(".eh_frame", 0x1000, 56), (".gcc_except_table", 0x3000, 16)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert!(got.blind, "{got:?}");
}

#[test]
fn arm_exidx_reads_the_lsda_after_the_unwind_opcodes() {
    let rel = |from: u64, to: u64| (to.wrapping_sub(from) & 0x7fff_ffff) as u32;
    let mut exidx = Vec::new();
    for (e, func, data) in [(0x5000u64, 0x2000u64, rel(0x5004, 0x6000)), (0x5008, 0x2080, 0x80b0_b0b0), (0x5010, 0x20c0, rel(0x5014, 0x6020))] {
        exidx.extend(rel(e, func).to_be_bytes());
        exidx.extend(data.to_be_bytes());
    }
    let mut extab = Vec::new();
    extab.extend(0x0000_0100u32.to_be_bytes());
    extab.extend(0x00b0_b0b0u32.to_be_bytes());
    extab.extend([0xff, 0xff, 0x01, 4, 0x04, 0x08, 0x20, 0x00]);
    extab.resize(0x20, 0);
    extab.extend(0x8101_b0b0u32.to_be_bytes());
    extab.extend(0xb0b0_b0b0u32.to_be_bytes());
    extab.extend(0u32.to_be_bytes());
    let parts = vec![(0x5000, exidx), (0x6000, extab)];
    let secs = sections(&[(".ARM.exidx", 0x5000, 24), (".ARM.extab", 0x6000, 0x2c)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert_eq!(got, Landing { pads: vec![0x2020], unknown: vec![], blind: false });
}

#[test]
fn a_front_end_without_sections_is_blind_only_to_a_cxx_image() {
    let none = |_: u64, _: &mut [u8]| false;
    assert_eq!(read(None, false, false, &code, &mut { none }), Landing::default());
    assert!(read(None, true, false, &code, &mut { none }).blind);
    let secs = sections(&[]);
    assert!(read(Some(&secs), false, true, &code, &mut { none }).blind, "setjmp unwinding names no pads");
    assert!(!read(Some(&secs), false, false, &code, &mut { none }).blind);
}

/// A `zLR` CIE at `at` (absolute LSDA pointers, PC-relative FDE addresses).
fn cie_at() -> Vec<u8> {
    vec![0, 0, 0, 16, 0, 0, 0, 0, 1, b'z', b'L', b'R', 0, 4, 0x7c, 0x41, 2, 0x00, 0x1b, 0]
}

/// An FDE at `at` for the function at `func`, under the CIE at `cie`, whose
/// LSDA is at `lsda`.
fn fde_at(at: u64, cie: u64, func: u64, lsda: u32) -> Vec<u8> {
    let mut v = vec![0, 0, 0, 20];
    v.extend((((at + 4) - cie) as u32).to_be_bytes());
    v.extend((func.wrapping_sub(at + 8) as u32).to_be_bytes());
    v.extend(0x40u32.to_be_bytes());
    v.push(4);
    v.extend(lsda.to_be_bytes());
    v.extend([0, 0, 0]);
    v
}

#[test]
fn an_fde_past_a_zero_terminator_is_still_read() {
    let mut eh = cie_at();
    eh.extend([0, 0, 0, 0]);
    let at = 0x1000 + eh.len() as u64;
    eh.extend(fde_at(at, 0x1000, 0x2000, 0x3000));
    let lsda = vec![0xff, 0xff, 0x01, 4, 0x04, 0x08, 0x10, 0];
    let parts = vec![(0x1000, eh.clone()), (0x3000, lsda)];
    let secs = sections(&[(".eh_frame", 0x1000, eh.len() as u64), (".gcc_except_table", 0x3000, 8)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert_eq!(got, Landing { pads: vec![0x2010], unknown: vec![], blind: false });
}

#[test]
fn call_site_tables_longer_than_their_sections_blind_the_image() {
    let mut eh = cie_at();
    let mut table = vec![0u8; 0x10000];
    for i in 0..64u64 {
        let at = 0x1000 + eh.len() as u64;
        eh.extend(fde_at(at, 0x1000, 0x2000, 0x10000 + 16 * i as u32));
        table[16 * i as usize..16 * i as usize + 6].copy_from_slice(&[0xff, 0xff, 0x01, 0x80, 0x80, 0x04]);
    }
    let parts = vec![(0x1000, eh.clone()), (0x10000, table)];
    let secs = sections(&[(".eh_frame", 0x1000, eh.len() as u64), (".gcc_except_table", 0x10000, 0x10000)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert!(got.blind, "{got:?}");
}

#[test]
fn exception_data_without_its_tables_is_blind() {
    let none = |_: u64, _: &mut [u8]| false;
    let secs = sections(&[(".gcc_except_table", 0x3000, 16)]);
    assert!(read(Some(&secs), false, false, &code, &mut { none }).blind);
    assert!(read(Some(&sections(&[])), true, false, &code, &mut { none }).blind, "a C++ runtime with no table at all");
}

#[test]
fn a_compact_entry_with_descriptors_leaves_its_function_unknown() {
    let rel = |from: u64, to: u64| (to.wrapping_sub(from) & 0x7fff_ffff) as u32;
    let mut exidx = Vec::new();
    exidx.extend(rel(0x5000, 0x2000).to_be_bytes());
    exidx.extend(rel(0x5004, 0x6000).to_be_bytes());
    let mut extab = Vec::new();
    extab.extend(0x80b0_b0b0u32.to_be_bytes());
    extab.extend(0x0004_0000u32.to_be_bytes());
    extab.extend(0x0000_0021u32.to_be_bytes());
    let parts = vec![(0x5000, exidx), (0x6000, extab)];
    let secs = sections(&[(".ARM.exidx", 0x5000, 8), (".ARM.extab", 0x6000, 12)]);
    let got = read(Some(&secs), false, false, &code, &mut image(&parts));
    assert_eq!(got, Landing { pads: vec![], unknown: vec![0x2000], blind: false });
}
