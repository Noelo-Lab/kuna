//! Relocatable-object entry selectors retain and resolve object coordinates.

use std::path::PathBuf;

use kuna_base::{address::Address, xml::DocumentStorage};
use kuna_console::engine::{
    bootstrap_from_file, bootstrap_from_object, bootstrap_from_root, ConsoleProgram,
    EntryLookupError, EntryProvenance, EntrySelector,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn boot_fixture(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let fixture = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name);
    let roots = vec![root.join("specs").to_string_lossy().into_owned()];
    match bootstrap_from_object(fixture.to_str().unwrap(), "", &roots) {
        Ok(program) => Some(program),
        Err(error) => {
            eprintln!(
                "verify_entry_selectors: skipping (bootstrap failed; build `.sla` with make specs): {}",
                error.explain()
            );
            None
        }
    }
}

fn boot_xml_fixture(name: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let fixture = root.join("tests/datatests").join(name);
    let roots = vec![root.join("specs").to_string_lossy().into_owned()];
    match bootstrap_from_file(fixture.to_str().unwrap(), "", &roots) {
        Ok(program) => Some(program),
        Err(error) => {
            eprintln!(
                "verify_entry_selectors: skipping XML bootstrap (build `.sla` with make specs): {}",
                error.explain()
            );
            None
        }
    }
}

#[test]
fn numeric_selectors_probe_sectionless_xml_mappings() {
    let Some(program) = boot_xml_fixture("condmulti.xml") else {
        return;
    };
    assert!(program.sections().is_empty());

    let entry = program
        .resolve_entry(&EntrySelector::Numeric(0x100825))
        .expect("XML symbol address is backed by its bytechunk");
    assert_eq!(entry.name, "condconst_multi");
    assert_eq!(entry.addr.get_offset(), 0x100825);

    let interior = program
        .resolve_entry(&EntrySelector::Numeric(0x100826))
        .expect("mapped XML byte remains directly selectable");
    assert_eq!(interior.addr.get_offset(), 0x100826);

    assert!(matches!(
        program.resolve_entry(&EntrySelector::Numeric(0x100824)),
        Err(EntryLookupError::Unmapped { .. })
    ));
}

#[test]
fn exact_address_selection_preserves_nondefault_space() {
    let root = repo_root();
    let specs = vec![root.join("specs").to_string_lossy().into_owned()];
    let mut store = DocumentStorage::new();
    let document = store
        .parse_document(
            br#"<binaryimage arch="8051:BE:16:default:default">
<bytechunk space="CODE" offset="0x100">22</bytechunk>
<bytechunk space="INTMEM" offset="0x100">22</bytechunk>
<symbol space="CODE" offset="0x100" name="code_entry"/>
<symbol space="INTMEM" offset="0x100" name="intmem_entry"/>
</binaryimage>"#,
        )
        .expect("parse synthetic multi-space XML");
    let program = match bootstrap_from_root(&document.get_root().clone(), &specs) {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "verify_entry_selectors: skipping multi-space bootstrap (build `.sla` with make specs): {}",
                error.explain()
            );
            return;
        }
    };
    let intmem = program
        .arch()
        .manage()
        .get_space_by_name("INTMEM")
        .expect("8051 INTMEM space");
    let requested = Address::new(std::rc::Rc::clone(intmem), 0x100);
    let selected = program
        .resolve_address(&requested)
        .expect("mapped non-default address must resolve in its parsed space");
    assert_eq!(selected.name, "intmem_entry");
    assert_eq!(selected.addr, requested);
}

#[test]
fn name_section_and_unique_raw_offset_resolve_the_same_definition() {
    let Some(program) = boot_fixture("ptx.o") else {
        return;
    };

    let by_name = program
        .resolve_entry(&EntrySelector::Name("fix_output_parameters".into()))
        .expect("resolve by name");
    let by_section = program
        .resolve_entry(&EntrySelector::parse(".text+0x660"))
        .expect("resolve by section coordinate");
    let by_raw_offset = program
        .resolve_entry(&EntrySelector::Numeric(0x660))
        .expect("resolve unique raw symbol offset");

    assert_eq!(by_name.addr.get_offset(), 0x400660);
    assert_eq!(by_section.addr.get_offset(), by_name.addr.get_offset());
    assert_eq!(by_raw_offset.addr.get_offset(), by_name.addr.get_offset());
    assert_eq!(by_name.provenance, EntryProvenance::DefinedObject);
    let location = by_name.object_location.expect("object location");
    assert_eq!(location.section, ".text");
    assert_eq!(location.section_index, 1);
    assert_eq!(location.offset, 0x660);
}

#[test]
fn duplicate_raw_offsets_are_reported_instead_of_guessed() {
    let Some(program) = boot_fixture("ptx.o") else {
        return;
    };

    let error = program
        .resolve_entry(&EntrySelector::Numeric(0))
        .expect_err(".text+0 and .text.startup+0 must be ambiguous");
    let EntryLookupError::Ambiguous { candidates, .. } = error else {
        panic!("expected ambiguity, got {error}");
    };
    assert!(candidates.iter().any(|entry| entry.name == "to_uchar"));
    assert!(candidates.iter().any(|entry| entry.name == "main"));
    let message = EntryLookupError::Ambiguous {
        selector: "0x0".into(),
        candidates,
    }
    .to_string();
    assert!(message.contains(".text+0x0"), "{message}");
    assert!(message.contains(".text.startup+0x0"), "{message}");
    assert!(message.contains("synthetic 0x"), "{message}");
    assert!(
        message.contains("local") || message.contains("global"),
        "{message}"
    );
}

#[test]
fn only_undefined_symbols_have_external_provenance() {
    let Some(program) = boot_fixture("ptx.o") else {
        return;
    };

    let external = program
        .resolve_entry(&EntrySelector::Name("strlen".into()))
        .expect("fixture references strlen");
    assert_eq!(external.provenance, EntryProvenance::UndefinedExternal);
    assert!(external.object_location.is_none());

    let error = program
        .resolve_entry(&EntrySelector::Numeric(0x12345))
        .expect_err("arbitrary unmapped numeric address must fail");
    assert!(matches!(error, EntryLookupError::Unmapped { .. }));
    let message = error.to_string();
    assert!(!message.contains("external symbol"), "{message}");
    assert!(message.contains("synthetic load VMAs"), "{message}");
    assert!(message.contains(".section+0xOFFSET"), "{message}");
    assert!(message.contains("SECTION_INDEX:0xOFFSET"), "{message}");
}

#[test]
fn duplicate_local_names_are_ambiguous_and_section_selectors_are_exact() {
    let Some(program) = boot_fixture("entry_selectors_x86_64.o") else {
        return;
    };

    let error = program
        .resolve_entry(&EntrySelector::Name("duplicate_local".into()))
        .expect_err("duplicate local names must not select the first symbol");
    let EntryLookupError::Ambiguous { candidates, .. } = error else {
        panic!("expected ambiguity, got {error}");
    };
    assert_eq!(candidates.len(), 2);
    let message = EntryLookupError::Ambiguous {
        selector: "duplicate_local".into(),
        candidates,
    }
    .to_string();
    assert!(message.contains(".text.selector_a+0x0"), "{message}");
    assert!(message.contains(".text.selector_b+0x0"), "{message}");
    assert!(message.contains("local"), "{message}");

    let a = program
        .resolve_entry(&EntrySelector::parse(".text.selector_a+0x0"))
        .expect("section A selector");
    let b = program
        .resolve_entry(&EntrySelector::parse(".text.selector_b+0x0"))
        .expect("section B selector");
    assert_ne!(a.addr.get_offset(), b.addr.get_offset());
    assert_eq!(
        a.object_location
            .as_ref()
            .map(|location| location.section.as_str()),
        Some(".text.selector_a")
    );
    assert_eq!(
        b.object_location
            .as_ref()
            .map(|location| location.section.as_str()),
        Some(".text.selector_b")
    );
}

#[test]
fn section_index_selectors_are_exact_and_enforce_section_bounds() {
    let Some(program) = boot_fixture("entry_selectors_x86_64.o") else {
        return;
    };

    let a = program
        .resolve_entry(&EntrySelector::parse("4:0x0"))
        .expect("section-index selector A");
    let b = program
        .resolve_entry(&EntrySelector::parse("6:0x0"))
        .expect("section-index selector B");
    assert_ne!(a.addr.get_offset(), b.addr.get_offset());
    assert_eq!(
        a.object_location
            .as_ref()
            .map(|location| location.section_index),
        Some(4)
    );
    assert_eq!(
        b.object_location
            .as_ref()
            .map(|location| location.section_index),
        Some(6)
    );
    assert_eq!(
        a.addr.get_offset(),
        program
            .resolve_entry(&EntrySelector::parse(".text.selector_a+0x0"))
            .unwrap()
            .addr
            .get_offset()
    );
    assert_eq!(
        b.addr.get_offset(),
        program
            .resolve_entry(&EntrySelector::parse(".text.selector_b+0x0"))
            .unwrap()
            .addr
            .get_offset()
    );

    for selector in ["4:0x6", "6:0x6", "99:0x0"] {
        let error = program
            .resolve_entry(&EntrySelector::parse(selector))
            .expect_err("out-of-range or unknown section index must fail");
        assert!(matches!(error, EntryLookupError::NotFound { .. }));
        assert!(error.to_string().contains(selector), "{error}");
    }
}

#[test]
fn arm_thumb_symbols_keep_raw_object_coordinates_and_normalized_entry_vmas() {
    let Some(program) = boot_fixture("arm_thumb_le32.o") else {
        return;
    };

    let by_name = program
        .resolve_entry(&EntrySelector::Name("thumb_add".into()))
        .expect("ARM Thumb symbol by name");
    let by_section = program
        .resolve_entry(&EntrySelector::parse(".text+0x0"))
        .expect("ARM Thumb symbol by section coordinate");
    let by_index = program
        .resolve_entry(&EntrySelector::parse("2:0x0"))
        .expect("ARM Thumb symbol by section index");

    assert_eq!(by_name.addr.get_offset(), 0x400000);
    assert_eq!(by_section.addr.get_offset(), by_name.addr.get_offset());
    assert_eq!(by_index.addr.get_offset(), by_name.addr.get_offset());
    // ELF stores the Thumb-state bit in st_value. Preserve that raw object
    // coordinate as provenance while presenting the normalized code entry VMA.
    assert_eq!(
        by_name.object_location,
        Some(kuna_console::engine::ObjectLocation {
            section_index: 2,
            section: ".text".into(),
            offset: 1,
        })
    );
    assert_eq!(by_name.provenance, EntryProvenance::DefinedObject);
    assert_eq!(by_name.binding.as_deref(), Some("global"));
}
