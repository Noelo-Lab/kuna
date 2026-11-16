//! One weak import keeps one logical symbol, entered at its synthetic extern
//! address, across its import slot and direct relocations.
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_base::address::Address;
use kuna_console::{
    assertions, assertsyntax,
    engine::{bootstrap_from_object, EntrySelector},
};
use std::path::PathBuf;

#[test]
fn mixed_and_got_only_imports_keep_one_identity_and_share_edited_prototypes() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for abi in ["n32", "64"] {
        for endian in ["be", "le"] {
            for form in ["mixed", "got-only"] {
                let path = root.join(format!(
                    "tests/fixtures/mips-weak-imports/identity-{abi}-{endian}-{form}.o"
                ));
                let target = match (abi, endian) {
                    ("n32", "le") => "MIPS:LE:64:64-32addr:n32",
                    (_, "le") => "MIPS:LE:64:default",
                    _ => "",
                };
                let bytes = std::fs::read(&path).unwrap();
                let image =
                    ObjectLoadImage::from_bytes_with_target(path.to_str().unwrap(), &bytes, target)
                        .unwrap();
                let aliases = image.function_address_aliases().to_vec();
                assert_eq!(aliases.len(), 1, "{path:?}: the import slot");
                let mut prog = bootstrap_from_object(
                    path.to_str().unwrap(),
                    target,
                    &[root.join("specs").to_string_lossy().into_owned()],
                )
                .unwrap();
                prog.commit_pending_analysis().unwrap();
                let entries = prog.function_entries_canonical();
                assert_eq!(
                    entries.iter().filter(|e| e.name == "optional").count(),
                    1,
                    "{path:?}"
                );
                let entry = prog
                    .resolve_entry(&EntrySelector::Name("optional".into()))
                    .unwrap();
                assert!(!prog.is_import_slot(entry.addr.get_offset()));
                assert!(!prog.vma_bytes_mapped(entry.addr.get_offset()));
                assert!(!prog
                    .function_entries_executable()
                    .iter()
                    .any(|e| e.name == "optional"));
                let mut addresses = vec![entry.addr.clone()];
                for &(alias, canonical) in &aliases {
                    assert_eq!(canonical, entry.addr.get_offset());
                    assert!(prog.is_import_slot(alias));
                    assert_eq!(
                        prog.resolve_entry(&EntrySelector::Numeric(alias))
                            .unwrap()
                            .addr,
                        entry.addr
                    );
                    addresses.push(Address::new(entry.addr.get_space().unwrap().clone(), alias));
                }
                let (symbol, _) = prog
                    .arch()
                    .symboltab
                    .find_function_across_scopes(&entry.addr)
                    .unwrap();
                for address in &addresses {
                    assert_eq!(
                        prog.arch()
                            .symboltab
                            .find_function_across_scopes(address)
                            .unwrap()
                            .0,
                        symbol
                    );
                }
                for (selector, declaration, count) in [
                    (
                        "optional".to_string(),
                        "unsigned int optional(unsigned int x)",
                        1,
                    ),
                    (
                        format!("0x{:x}", addresses.last().unwrap().get_offset()),
                        "unsigned int optional(unsigned int x,unsigned int y)",
                        2,
                    ),
                ] {
                    prog.set_assertions(vec![assertsyntax::parse_directive(&format!(
                        "prototype {selector} {declaration}"
                    ))
                    .unwrap()]);
                    assertions::apply_program_scoped(&mut prog);
                    assert!(
                        prog.assertion_outcomes()
                            .iter()
                            .all(|o| o.status == "applied"),
                        "{:?}",
                        prog.assertion_outcomes()
                    );
                    let context = prog.arch().build_arch_handle();
                    for address in &addresses {
                        let (name, _, proto) = context
                            .query_function(address)
                            .expect("shared import lookup");
                        assert_eq!(name, "optional");
                        assert_eq!(proto.num_params(), count, "{path:?}: {address:?}");
                    }
                }
            }
        }
    }
}
