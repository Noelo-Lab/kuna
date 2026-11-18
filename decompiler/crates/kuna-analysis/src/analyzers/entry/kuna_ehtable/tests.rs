//! [`scan`] over the three `ehcatch.cc` builds: an `.eh_frame` LSDA, an ARM
//! `.ARM.extab` LSDA, and a stripped Thumb image whose hidden class is named
//! only by its `type_info` name string.

use super::*;

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn sites(name: &str) -> Vec<EhCallSite> {
    let bytes = fixture(name);
    scan(&object::File::parse(&*bytes).expect("parse"))
}

fn site(start: u64, end: u64, landing_pad: u64, catches: &[&str]) -> EhCallSite {
    EhCallSite { start, end, landing_pad, catches: catches.iter().map(|c| c.to_string()).collect() }
}

#[test]
fn x86_64_lsda_names_a_builtin_type_a_hidden_class_and_a_catch_all() {
    assert_eq!(
        sites("ehcatch_x86_64.so"),
        [site(0x1607, 0x160c, 0x1610, &["int"]), site(0x1637, 0x163c, 0x1641, &["Odd", "..."])]
    );
}

#[test]
fn arm_extab_lsda_resolves_target2_type_entries() {
    assert_eq!(
        sites("ehcatch_arm.so"),
        [site(0x103b8, 0x103bc, 0x103c4, &["int"]), site(0x103e0, 0x103e4, 0x103f0, &["Odd", "..."])]
    );
}

#[test]
fn stripped_thumb_image_counts_from_the_even_function_start() {
    assert_eq!(
        sites("ehcatch_thumb_stripped.so"),
        [site(0x103b2, 0x103b6, 0x103ba, &["int"]), site(0x103cc, 0x103d0, 0x103d8, &["Odd", "..."])]
    );
}

#[test]
fn an_image_without_an_lsda_has_no_sites() {
    assert!(sites("kernel_bugtable_x86_64.ko").is_empty());
    assert!(sites("adrnext_arm").is_empty());
}

/// An LSDA whose call-site table runs past its section contributes nothing;
/// the next function's still decodes.
#[test]
fn a_truncated_lsda_yields_nothing() {
    let mut bytes = fixture("ehcatch_x86_64.so");
    let file = object::File::parse(&*bytes).expect("parse");
    let table = file.section_by_name(".gcc_except_table").expect("table");
    let (off, _) = table.file_range().expect("range");
    drop(file);
    bytes[off as usize + 4] = 0xff;
    bytes[off as usize + 5] = 0x7f;
    let sites = scan(&object::File::parse(&*bytes).expect("parse"));
    assert_eq!(sites, [site(0x1637, 0x163c, 0x1641, &["Odd", "..."])]);
}
