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

/// (RE-need `string-owner-function-name`) A placeholder name kuna PRINTS is a
/// name kuna ACCEPTS.
///
/// `tailcallframe_x86_64` renders `sub_1170(a0)` inside `sub_11b0` — a recovered
/// tail call to an address the canonical inventory does not hold as an entry —
/// and the by-name selector answered `no function matches "sub_1170"` while
/// `--addr 0x1170` decompiled it. Both name lookups now read the placeholder as
/// the address it spells and land on exactly the entry the numeric selector does.
#[test]
fn a_generated_placeholder_name_resolves_to_the_address_it_spells() {
    let Some(program) = boot_fixture("tailcallframe_x86_64") else {
        return;
    };
    assert!(
        program.find_entry_at(0x1170).is_none(),
        "fixture no longer reproduces: 0x1170 is a discovered entry"
    );

    let by_addr = program
        .resolve_entry(&EntrySelector::Numeric(0x1170))
        .expect("the numeric selector always reached this function");
    let by_name = program
        .resolve_entry(&EntrySelector::Name("sub_1170".into()))
        .expect("the printed placeholder name resolves");
    assert_eq!(by_name.addr.get_offset(), by_addr.addr.get_offset());
    assert_eq!(by_name.name, "sub_1170");
    assert_eq!(
        program.find_entry_by_name("sub_1170").map(|e| e.addr.get_offset()),
        Some(0x1170),
        "the yes/no lookup answers the same name the same way"
    );
}

/// (RE-need `changing-namestyle-invalidates-discovered`) A placeholder resolves
/// in EVERY naming style, not only the active one.
///
/// `option namestyle` decides how a synthesized name is PRINTED; a selector must
/// not depend on that choice, or the styles become disjoint name spaces. The
/// tester hit it the other way round: `kuna functions` reports `sub_15dc` and
/// ignores namestyle, so feeding that name back with `--option namestyle ghidra`
/// answered `no function matches` for a function the same run decompiles under
/// `func_0x000015dc`.
#[test]
fn a_placeholder_resolves_in_every_naming_style() {
    let Some(program) = boot_fixture("tailcallframe_x86_64") else {
        return;
    };
    // The default (angr) style is active, so the other two vocabularies' names
    // are the ones a style flip would put in an agent's hands.
    for name in ["sub_1170", "func_0x00001170", "FUN_00001170"] {
        let entry = program
            .resolve_entry(&EntrySelector::Name(name.into()))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(entry.addr.get_offset(), 0x1170, "{name}");
        assert_eq!(
            program.find_entry_by_name(name).map(|e| e.addr.get_offset()),
            Some(0x1170),
            "{name}: the yes/no lookup answers the same name the same way"
        );
    }
}

/// The tester's exact round trip: a name taken from the default-style
/// enumeration is fed back to a run whose naming style has been flipped.
///
/// `option namestyle ghidra` clears `name_style_angr`, which is what used to
/// take `sub_1170` out of the resolvable set entirely.
#[test]
fn a_default_style_name_survives_a_namestyle_flip() {
    let Some(mut program) = boot_fixture("tailcallframe_x86_64") else {
        return;
    };
    let default_style_name = "sub_1170";
    program.arch_mut().name_style_angr = false;
    assert_eq!(
        program.arch().name_function(
            &program.resolve_entry(&EntrySelector::Numeric(0x1170)).unwrap().addr
        ),
        "func_0x00001170",
        "the flip must actually change what this build mints, or the test is vacuous"
    );
    let entry = program
        .resolve_entry(&EntrySelector::Name(default_style_name.into()))
        .expect("a name the default style printed still selects its function");
    assert_eq!(entry.addr.get_offset(), 0x1170);
}

/// The reading is minted, not parsed: only a name SOME naming style would print
/// at a MAPPED address is read as one, so a hex-tailed name that is not a
/// placeholder, a placeholder misspelled for every style, and one spelling an
/// address with no bytes behind it all still miss.
#[test]
fn a_placeholder_name_is_not_read_as_an_address_unless_this_build_would_mint_it() {
    let Some(program) = boot_fixture("tailcallframe_x86_64") else {
        return;
    };
    // `func_00001170` misses on its `0x`: the upstream style prints
    // `func_0x00001170`, and `FUN_` is the only style without the prefix.
    for miss in ["sub_deadbeef", "FUN_1170", "func_00001170", "handler_1170", "sub_"] {
        let error = program
            .resolve_entry(&EntrySelector::Name(miss.into()))
            .expect_err("not a name this program mints at a mapped address");
        assert!(
            matches!(error, EntryLookupError::NotFound { .. }),
            "{miss}: {error}"
        );
        assert!(program.find_entry_by_name(miss).is_none(), "{miss}");
    }
}

/// (RE-need `mach-o-import-data`) An import name spelled on both a `__stubs`
/// veneer and the `__la_symbol_ptr` slot that veneer reads resolves to the
/// veneer, and BOTH entries stay in the inventory.
///
/// Dropping the data row would close the ambiguity too, and would delete the
/// only name a slot with no veneer (`__DATA,__got`, `__DATA,__nl_symbol_ptr`)
/// ever carries — so the narrowing is at selection, not in the enumeration.
#[test]
fn a_macho_import_name_resolves_to_its_stub_without_losing_the_slot() {
    let Some(program) = boot_fixture("macho_imports") else {
        return;
    };

    let entry = program
        .resolve_entry(&EntrySelector::Name("printf".into()))
        .expect("the __stubs veneer is the one executable candidate");
    assert_eq!(entry.addr.get_offset(), 0x1000005cc);

    let rows: Vec<u64> = program
        .function_entries_canonical()
        .into_iter()
        .filter(|e| e.name == "printf")
        .map(|e| e.addr.get_offset())
        .collect();
    assert_eq!(rows, vec![0x1000005cc, 0x100003000], "both rows survive");
    assert!(
        !program.any_executable_entry(&[program
            .find_entry_at(0x100003000)
            .expect("the slot row")]),
        "the slot is what makes the veneer the lone executable candidate"
    );
}

/// The narrowing needs EXACTLY one executable candidate: two definitions in
/// different code sections of one object are both executable and stay
/// ambiguous, and a sectionless XML image reads every address as executable, so
/// neither can be silently guessed.
#[test]
fn same_named_code_definitions_are_still_ambiguous() {
    let Some(program) = boot_fixture("entry_selectors_x86_64.o") else {
        return;
    };
    let error = program
        .resolve_entry(&EntrySelector::Name("duplicate_local".into()))
        .expect_err("two .text definitions are both executable");
    assert!(matches!(error, EntryLookupError::Ambiguous { .. }), "{error}");
}
