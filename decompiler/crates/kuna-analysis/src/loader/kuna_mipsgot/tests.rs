use super::*;

fn ranges(name: &str) -> Vec<(u64, u64)> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("fixture");
    const_ranges(&object::File::parse(&*bytes).expect("parses"))
}

/// The local entries after the module pointer and the global entries of the
/// four defined symbols are constant; the lazy resolver, the module pointer
/// and the `puts` import's stub slot are not reported here.
#[test]
fn local_and_defined_global_entries_are_constant() {
    assert_eq!(ranges("mipsgot_gcc_be.so"), vec![(0x10428, 0x10443)]);
    assert_eq!(ranges("mipsgot_clang_el.so"), vec![(0x10438, 0x10453)]);
    assert_eq!(ranges("mipsgot_gcc_be64.so"), vec![(0x105c0, 0x105f7)]);
}

/// A global entry holding anything but its symbol's value stays variable.
#[test]
fn a_slot_that_does_not_hold_its_symbols_value_is_skipped() {
    let path = format!("{}/tests/fixtures/mipsgot_gcc_be.so", env!("CARGO_MANIFEST_DIR"));
    let mut bytes = std::fs::read(&path).expect("fixture");
    let file = object::File::parse(&*bytes).expect("parses");
    let got = file.section_by_name(".got").expect(".got");
    let at = (got.file_range().unwrap().0 + (0x1043c - got.address())) as usize;
    drop(file);
    bytes[at..at + 4].copy_from_slice(&0x1234u32.to_be_bytes());
    let file = object::File::parse(&*bytes).expect("parses");
    assert_eq!(const_ranges(&file), vec![(0x10428, 0x1043b), (0x10440, 0x10443)]);
}

/// Off MIPS, and for a relocatable object, nothing is reported.
#[test]
fn other_images_report_nothing() {
    assert!(ranges("selfplt_x86_64.so").is_empty());
    assert!(ranges("bejoin_mips32_be.o").is_empty());
}

/// A `DT_MIPS_LOCAL_GOTNO` far past the GOT's section reads no slot outside
/// that section, and the walk ends at the section's end instead of counting on.
#[test]
fn a_local_count_past_the_got_section_stops_at_its_end() {
    let path = format!("{}/tests/fixtures/mipsgot_gcc_be.so", env!("CARGO_MANIFEST_DIR"));
    let mut bytes = std::fs::read(&path).expect("fixture");
    let file = object::File::parse(&*bytes).expect("parses");
    let dynamic = file.section_by_name(".dynamic").expect(".dynamic");
    let (dyn_off, dyn_size) = dynamic.file_range().unwrap();
    let got = file.section_by_name(".got").expect(".got");
    let (got_lo, got_hi) = (got.address(), got.address() + got.size() - 1);
    drop(file);
    let local = (dyn_off as usize..(dyn_off + dyn_size) as usize)
        .step_by(8)
        .find(|&at| bytes[at..at + 4] == 0x7000_000au32.to_be_bytes())
        .expect("DT_MIPS_LOCAL_GOTNO");
    bytes[local + 4..local + 8].copy_from_slice(&u32::MAX.to_be_bytes());
    let file = object::File::parse(&*bytes).expect("parses");
    let ranges = const_ranges(&file);
    assert!(!ranges.is_empty());
    assert!(ranges.iter().all(|&(lo, hi)| got_lo <= lo && hi <= got_hi), "{ranges:x?}");
}
