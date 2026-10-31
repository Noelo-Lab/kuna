use super::*;

fn bound(name: &str) -> Vec<(u64, u64)> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("fixture");
    let file = object::File::parse(&*bytes).expect("parses");
    bind(&file, &crate::loader::elf_plt::resolve_plt_imports(&file, &bytes))
}

/// Each stub is joined to the definition its slot's `JUMP_SLOT` names.
#[test]
fn each_stub_is_bound_to_the_definition_its_relocation_names() {
    assert_eq!(bound("pltlocal_aarch64.so"), vec![(0x10380, 0x10344), (0x10390, 0x10354)]);
    assert_eq!(bound("selfplt_x86_64.so"), vec![(0x1050, 0x1120)]);
}

/// A stub whose relocation names an undefined symbol is an import and binds
/// nothing.
#[test]
fn an_import_binds_nothing() {
    assert!(bound("plt_aarch64").is_empty());
}
