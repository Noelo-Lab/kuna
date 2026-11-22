//! valuewitness end-to-end gate: name and prototype a stripped Swift
//! binary's value witness functions from their tables, and render a generic
//! function's witness calls through its metadata argument as named calls.
//!
//! Fixture: `tests/fixtures/swift_witness_elf` -- an x86-64 ELF executable
//! built from `swift_witness_elf.swift` with swiftc 6.4.2 (`-O -module-name
//! witfix`) and `llvm-strip --strip-all`. The expected functions are the
//! unstripped build's value-witness symbols (`$s6witfix4PairVwxx` at 0x2970,
//! `destroy value witness for witfix.Pair`, ...); `witfix.swapIn<A>` is at
//! 0x3580.
//!
//! Outlined helpers: `tests/fixtures/swift_outlined_macho` -- an arm64 iOS
//! dylib built from `swift_outlined_macho.swift` with swiftc 6.4.2 (`-O
//! -module-name witfix -emit-library -target arm64-apple-ios17.0`) and
//! `llvm-strip -x`. From the unstripped build: `outlined destroy of
//! witfix.Slot` at 0x53f8, `merged outlined init with copy of witfix.Stamp`
//! at 0x4918 and `merged outlined init with take of witfix.Stamp` at 0x4970;
//! `outlined init with copy of Foundation.URL?` at 0x4a40 fetches its
//! metadata through the instantiation helper and is left alone.
//!
//! Two-state: with `valuewitness` off none of it happens.

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, bootstrap_from_object};
use kuna_console::ifacedecomp::{
    DECOMPILE_MODULE,
    IfaceDecompData,
    execute,
    register_decomp_commands,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// `(VMA, name)` from the unstripped symbol table: a static struct's, an
/// enum's (with the three enum witnesses) and a generic struct's, whose
/// table is in its metadata pattern.
const WITNESSES: &[(u64, &str)] = &[
    (
        0x2960,
        "witfix.Pair.initializeBufferWithCopyOfBuffer_value_witness",
    ),
    (0x2970, "witfix.Pair.destroy_value_witness"),
    (0x3160, "witfix.Pair.initializeWithCopy_value_witness"),
    (0x29d0, "witfix.Pair.assignWithCopy_value_witness"),
    (0x2a40, "witfix.Pair.assignWithTake_value_witness"),
    (0x2a80, "witfix.Pair.getEnumTagSinglePayload_value_witness"),
    (
        0x2ac0,
        "witfix.Pair.storeEnumTagSinglePayload_value_witness",
    ),
    (0x2b10, "witfix.Shape.destroy_value_witness"),
    (0x2cc0, "witfix.Shape.getEnumTag_value_witness"),
    (
        0x2cd0,
        "witfix.Shape.destructiveProjectEnumData_value_witness",
    ),
    (
        0x2ce0,
        "witfix.Shape.destructiveInjectEnumTag_value_witness",
    ),
    (0x2e10, "witfix.Holder.destroy_value_witness"),
    (0x2ec0, "witfix.Holder.initializeWithTake_value_witness"),
];

fn load(on: bool) -> ConsoleProgram {
    load_fixture("swift_witness_elf", on)
}

fn load_fixture(fixture: &str, on: bool) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("swiftmetadata", "on")
        .expect("swiftmetadata flips on");
    if !on {
        prog.arch_mut()
            .set_kuna_option("valuewitness", "off")
            .expect("valuewitness flips off");
    }
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    prog
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .expect("a default code space")
        .clone();
    Address::new(space, vma)
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let addr = code_addr(prog, vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// The C of the function at `vma`.
fn decompile(mut prog: ConsoleProgram, vma: u64) -> String {
    let addr = code_addr(&prog, vma);
    let name = name_at(&prog, vma);
    let fd = kuna_decomp::decompile_drive::build_and_follow_flow(
        prog.arch_mut(),
        &name,
        addr,
        kuna_console::engine::UNBOUNDED_SIZE,
    )
    .expect("the function builds");
    let mut status =
        ConsoleCommands::into_status(vec!["decompile".to_string(), "print C".to_string()]);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    execute(&mut status);
    execute(&mut status);
    status.optr.clone()
}

#[test]
fn value_witnesses_are_named_from_their_tables() {
    let prog = load(true);
    for &(vma, want) in WITNESSES {
        assert_eq!(name_at(&prog, vma), want, "0x{vma:x}");
    }
}

#[test]
fn a_value_witness_has_its_runtime_prototype() {
    let c = decompile(load(true), 0x2970);
    assert!(
        c.contains("witfix.Pair.destroy_value_witness(void *object,swift.type_metadata *self)"),
        "{c}"
    );
}

#[test]
fn witness_calls_through_metadata_are_named() {
    let c = decompile(load(true), 0x3580);
    assert!(c.contains("->initializeWithTake)(v2,a0,a2);"), "{c}");
    assert!(c.contains("->initializeWithCopy)(a0,a1,a2);"), "{c}");
    assert!(c.contains("(void *a0,void *a1,swift.type_metadata *a2)"), "{c}");
}

#[test]
fn a_generic_arguments_table_is_the_common_table_read_off_a_byte_offset() {
    let c = decompile(load(true), 0x2f60);
    assert!(c.contains("swift.value_witness_table *v1;"), "{c}");
    assert!(c.contains("v1->extraInhabitantCount"), "{c}");
    assert!(!c.contains("enum_value_witness_table"), "{c}");
    assert!(c.contains("*)self)[2] + -8)"), "{c}");
    assert!(!c.contains("->kind"), "{c}");
}

#[test]
fn valuewitness_off_names_and_types_nothing() {
    let prog = load(false);
    for &(vma, name) in WITNESSES {
        assert_ne!(name_at(&prog, vma), name, "0x{vma:x}");
    }
    let c = decompile(prog, 0x3580);
    assert!(!c.contains("->initializeWithCopy"), "{c}");
}

/// `(VMA, name)` of the outlined helpers the arm names.
const HELPERS: &[(u64, &str)] = &[
    (0x53f8, "witfix.Slot.outlined_destroy"),
    (0x4918, "outlined_init_with_copy.merged"),
    (0x4970, "outlined_init_with_take.merged"),
];

#[test]
fn outlined_helpers_are_named_from_their_bodies() {
    let prog = load_fixture("swift_outlined_macho", true);
    for &(vma, want) in HELPERS {
        assert_eq!(name_at(&prog, vma), want, "0x{vma:x}");
    }
    assert!(!name_at(&prog, 0x4a40).contains("outlined"));
}

#[test]
fn an_outlined_helper_has_the_prototype_its_body_states() {
    let c = decompile(load_fixture("swift_outlined_macho", true), 0x4918);
    assert!(
        c.contains("void * outlined_init_with_copy.merged(void *src,void *dest,swift.type_metadata *(*accessor)("),
        "{c}"
    );
    let c = decompile(load_fixture("swift_outlined_macho", true), 0x53f8);
    assert!(c.contains("void * witfix.Slot.outlined_destroy(void *value)"), "{c}");
}

#[test]
fn valuewitness_off_names_no_outlined_helper() {
    let prog = load_fixture("swift_outlined_macho", false);
    for &(vma, name) in HELPERS {
        assert_ne!(name_at(&prog, vma), name, "0x{vma:x}");
    }
}
