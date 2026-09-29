use super::*;

#[test]
fn elf_alias_names_survive_both_symbol_tables_and_rebasing() {
    for linked in [false, true] {
        for veneer in [false, true] {
            for reverse in [false, true] {
                let bytes = alias_test_fixture::image(veneer, reverse, linked);
                let image = ObjectLoadImage::from_bytes("synthetic-aliases", &bytes).unwrap();
                let definition = image
                    .funcsyms
                    .iter()
                    .find(|s| s.name == b"__answer_from_arm")
                    .unwrap()
                    .addr;
                for name in [b"answer".as_slice(), b"answer_alias", b"__answer_from_arm"] {
                    assert_eq!(
                        image
                            .funcsyms
                            .iter()
                            .filter(|s| s.addr == definition && s.name == name)
                            .count(),
                        1,
                        "{linked} {veneer} {reverse}: {name:?}"
                    );
                }
                assert_eq!(
                    image.funcsyms.len(),
                    3 + usize::from(veneer) + usize::from(linked)
                );
                if veneer {
                    let thumb = image
                        .funcsyms
                        .iter()
                        .find(|s| s.name == b"__real_answer")
                        .unwrap();
                    assert_eq!(
                        thumb.addr,
                        definition + 17,
                        "preserve the encoded Thumb state"
                    );
                }
                if linked {
                    assert!(image
                        .elf_function_provenance()
                        .definitions
                        .contains(&0x1000));
                    assert!(image.elf_function_provenance().imports.contains(&0x2000));
                    assert!(image
                        .funcsyms
                        .iter()
                        .any(|s| s.addr == 0x2000 && s.name == b"answer"));
                }
            }
        }
    }
}
