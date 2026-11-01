use super::*;

#[test]
fn elf_alias_names_survive_both_symbol_tables_and_rebasing() {
    for linked in [false, true] {
        for veneer in [false, true] {
            for reverse in [false, true] {
                let bytes = alias_test_fixture::image(veneer, reverse, linked);
                let image = ObjectLoadImage::from_bytes("synthetic-aliases", &bytes).unwrap();
                let primary = image.func_symbols();
                let aliases = image.func_symbol_aliases();
                let tag = format!("linked={linked} veneer={veneer} reverse={reverse}");
                let definition = primary
                    .iter()
                    .find(|(_, name)| name == "__answer_from_arm")
                    .unwrap_or_else(|| panic!("{tag}: the first name keeps the address"))
                    .0;
                assert_eq!(
                    primary.iter().filter(|(addr, _)| *addr == definition).count(),
                    1,
                    "{tag}: one reported name per address"
                );
                for name in ["answer", "answer_alias"] {
                    assert_eq!(
                        aliases.iter().filter(|(a, n)| *a == definition && n == name).count(),
                        1,
                        "{tag}: {name}"
                    );
                }
                assert_eq!(
                    primary.len() + aliases.len(),
                    3 + usize::from(veneer) + usize::from(linked),
                    "{tag}: .dynsym repeats collapse onto the .symtab names"
                );
                if veneer {
                    let (thumb, _) =
                        primary.iter().find(|(_, name)| name == "__real_answer").unwrap();
                    assert_eq!(*thumb, definition + 17, "{tag}: the Thumb bit stays raw");
                }
                if linked {
                    let provenance = image.elf_function_provenance();
                    assert!(provenance.definitions.contains(&0x1000));
                    assert!(provenance.imports.contains(&0x2000));
                    assert!(primary.iter().any(|(addr, name)| *addr == 0x2000 && name == "answer"));
                    let mut shifted = image;
                    shifted.attach_to_space(Rc::clone(
                        super::tests::manager().get_default_code_space().unwrap(),
                    ));
                    shifted.adjust_vma(0x10000);
                    let moved: Vec<_> =
                        aliases.iter().map(|(addr, name)| (addr + 0x10000, name.clone())).collect();
                    assert_eq!(shifted.func_symbol_aliases(), moved, "{tag}: aliases follow the shift");
                }
            }
        }
    }
}

/// (GH #1357) A Mach-O or PE name the address dedup drops is an alias, as on
/// ELF: the second `nlist` label at an entry, local or external, and a MinGW
/// COFF pair. The export stream's C spelling of a Mach-O name is not a second
/// name.
#[test]
fn macho_and_pe_alias_names_survive() {
    let load = |name: &str| {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        ObjectLoadImage::from_bytes(&path, &bytes).unwrap()
    };
    let macho = load("macho_alias_names_arm64");
    let mut primary = macho.func_symbols();
    primary.sort_unstable();
    let named = |rows: &[(u64, &str)]| -> Vec<(u64, String)> {
        rows.iter().map(|&(addr, name)| (addr, name.to_string())).collect()
    };
    assert_eq!(
        primary,
        named(&[
            (0x240, "_audit_export_a"),
            (0x248, "_audit_local_a"),
            (0x250, "_audit_caller"),
            (0x254, "_audit_first"),
            (0x25c, "_audit_shared"),
            (0x264, "_audit_second"),
        ])
    );
    let mut aliases = macho.func_symbol_aliases();
    aliases.sort_unstable();
    assert_eq!(
        aliases,
        named(&[
            (0x240, "_audit_export_b"),
            (0x248, "_audit_local_b"),
            (0x254, "_audit_both"),
            (0x254, "_audit_shared"),
            (0x264, "_audit_both"),
        ])
    );

    let pe = load("pe_imports.exe");
    let aliases = pe.func_symbol_aliases();
    for want in [
        (0x140001890, "fpreset"),
        (0x140007850, "_get_invalid_parameter_handler"),
        (0x140007860, "_set_invalid_parameter_handler"),
    ] {
        assert!(aliases.contains(&(want.0, want.1.to_string())), "{want:?}: {aliases:?}");
    }
}
