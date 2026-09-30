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
                }
            }
        }
    }
}
