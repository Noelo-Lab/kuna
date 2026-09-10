//! The `entrythumbflow` walk on the object path: a PE whose machine word
//! leaves the ARM mode to the entry bit (`IMAGE_FILE_MACHINE_ARM`) and whose
//! `AddressOfEntryPoint` carries it gets the same entry-reachable Thumb
//! context a TE with that shape gets. The fixture is the vendored Thumb PE
//! with its machine word rewritten from `THUMB` (painted wholly Thumb by the
//! Windows reading) to `ARM`.

mod common;

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object_with_isa, ArmIsa, ConsoleProgram};
use kuna_console::project::decompile_targets;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn specs() -> Vec<String> {
    vec![std::env::var("KUNA_SPECS")
        .unwrap_or_else(|_| repo_root().join("specs").to_string_lossy().into_owned())]
}

/// The vendored Thumb PE with machine `ARM` (`0x1c0`) instead of `THUMB`.
fn arm_machine_pe() -> PathBuf {
    arm_machine_pe_with(None)
}

/// [`arm_machine_pe`], optionally with its 0x200-byte `.text` rewritten to
/// `code` (and its `VirtualSize` widened to match).
fn arm_machine_pe_with(code: Option<&[u8]>) -> PathBuf {
    let mut bytes = std::fs::read(
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe"),
    )
    .unwrap();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    assert_eq!(&bytes[pe..pe + 4], b"PE\0\0");
    assert_eq!(bytes[pe + 4..pe + 6], object::pe::IMAGE_FILE_MACHINE_THUMB.to_le_bytes());
    bytes[pe + 4..pe + 6].copy_from_slice(&object::pe::IMAGE_FILE_MACHINE_ARM.to_le_bytes());
    if let Some(code) = code {
        let optional = u16::from_le_bytes(bytes[pe + 20..pe + 22].try_into().unwrap()) as usize;
        let text = pe + 24 + optional;
        assert_eq!(&bytes[text..text + 5], b".text");
        let raw = u32::from_le_bytes(bytes[text + 20..text + 24].try_into().unwrap()) as usize;
        bytes[text + 8..text + 12].copy_from_slice(&(code.len() as u32).to_le_bytes());
        bytes[raw..raw + code.len()].copy_from_slice(code);
    }
    let path = common::scratch_file("arm-machine-pe", "exe");
    std::fs::write(&path, bytes).unwrap();
    path
}

/// Add one defined COFF function symbol to the synthetic PE's code section.
fn pe_with_callee(code: &[u8], name: &[u8], offset: u32) -> PathBuf {
    assert!(name.len() <= 8);
    let path = arm_machine_pe_with(Some(code));
    let mut bytes = std::fs::read(&path).unwrap();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    let symbols = bytes.len() as u32;
    bytes[pe + 12..pe + 16].copy_from_slice(&symbols.to_le_bytes());
    bytes[pe + 16..pe + 20].copy_from_slice(&1u32.to_le_bytes());
    let mut symbol = [0u8; 18];
    symbol[..name.len()].copy_from_slice(name);
    symbol[8..12].copy_from_slice(&offset.to_le_bytes());
    symbol[12..14].copy_from_slice(&1u16.to_le_bytes());
    symbol[14..16].copy_from_slice(&0x20u16.to_le_bytes());
    symbol[16] = object::pe::IMAGE_SYM_CLASS_EXTERNAL;
    bytes.extend_from_slice(&symbol);
    bytes.extend_from_slice(&4u32.to_le_bytes());
    std::fs::write(&path, bytes).unwrap();
    path
}

fn tmode_at(program: &ConsoleProgram, offset: u64) -> u32 {
    let space = program.arch().manage().get_default_code_space().cloned().unwrap();
    program
        .arch()
        .with_context_db_mut(|db| db.get_variable_value(b"TMode", &Address::new(space, offset)))
        .unwrap()
}

fn load(path: &str, options: &[(&str, &str)]) -> Option<ConsoleProgram> {
    load_with_decoder(path, "", None, options)
}

fn load_with_decoder(
    path: &str,
    target: &str,
    isa: Option<ArmIsa>,
    options: &[(&str, &str)],
) -> Option<ConsoleProgram> {
    let mut program = match bootstrap_from_object_with_isa(path, target, &specs(), isa) {
        Ok(program) => program,
        Err(error) if error.explain().contains("No sleigh specification") => {
            eprintln!("verify_entrythumbflow_pe: skipping (build the ARM `.sla`): {}", error.explain());
            return None;
        }
        Err(error) => panic!("PE bootstrap failed: {}", error.explain()),
    };
    for (name, value) in options {
        program.arch_mut().set_kuna_option(name, value).unwrap();
    }
    program.commit_pending_analysis().unwrap();
    Some(program)
}

#[test]
fn conditional_noreturn_blx_preserves_the_not_taken_path() {
    let fixture = pe_with_callee(&[
        0x00, 0x28, 0x08, 0xbf, // cmp r0,#0; it eq
        0x00, 0xf0, 0x04, 0xe8, // blxeq 0x401010
        0x07, 0x20, 0x70, 0x47, // movs r0,#7; bx lr
        0x00, 0x00, 0x00, 0x00,
        0xfe, 0xff, 0xff, 0xea, // A32: b 0x401010
    ], b"exit", 0x10);
    let path = fixture.to_string_lossy();
    let Some(mut program) = load_with_decoder(&path, "ARM:LE:32:v8:default", None, &[]) else {
        return;
    };
    assert_eq!(tmode_at(&program, 0x401008), 1, "the condition-false successor is Thumb");
    assert_eq!(tmode_at(&program, 0x40100a), 1, "the reachable return is Thumb");
    assert_eq!(tmode_at(&program, 0x401010), 0, "the BLX callee stays A32");
    let entry = program.find_entry_at(0x401000).unwrap();
    let results = decompile_targets(&mut program, vec![entry], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    let code = results[0].code.as_deref().unwrap();
    assert!(code.contains("if (") && code.contains("exit(") && code.contains("return 7;"), "{code}");
    let _ = std::fs::remove_file(fixture);
}

#[test]
fn unconditional_noreturn_blx_still_terminates_the_path() {
    let fixture = pe_with_callee(&[
        0x00, 0xf0, 0x06, 0xe8, // blx 0x401010
        0x07, 0x20, 0x70, 0x47,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xfe, 0xff, 0xff, 0xea,
    ], b"exit", 0x10);
    let path = fixture.to_string_lossy();
    let Some(program) = load_with_decoder(&path, "ARM:LE:32:v8:default", None, &[]) else {
        return;
    };
    assert_eq!(tmode_at(&program, 0x401000), 1);
    assert_eq!(tmode_at(&program, 0x401004), 0, "no fall-through from the unconditional call");
    assert_eq!(tmode_at(&program, 0x401010), 0);
    let _ = std::fs::remove_file(fixture);
}

#[test]
fn disabled_noreturn_known_does_not_cut_off_thumb_flow() {
    let fixture = pe_with_callee(&[
        0x00, 0xf0, 0x06, 0xe8, // blx 0x401010
        0x07, 0x20, 0x70, 0x47, // movs r0,#7; bx lr
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x1e, 0xff, 0x2f, 0xe1, // A32: bx lr (despite the no-return name)
    ], b"fastfail", 0x10);
    let path = fixture.to_string_lossy();
    let Some(enabled) = load_with_decoder(&path, "ARM:LE:32:v8:default", None, &[]) else {
        return;
    };
    assert_eq!(tmode_at(&enabled, 0x401004), 0, "the enabled name heuristic terminates the path");
    for listing in ["off", "on"] {
        let mut outputs = Vec::new();
        for isa in [None, Some(ArmIsa::Thumb)] {
            let Some(mut program) = load_with_decoder(
                &path,
                "ARM:LE:32:v8:default",
                isa,
                &[("noreturn_known", "off"), ("listing", listing)],
            ) else {
                return;
            };
            assert_eq!(tmode_at(&program, 0x401004), 1, "disabled facts must not stop the walk");
            let entry = program.find_entry_at(0x401000).unwrap();
            let results = decompile_targets(&mut program, vec![entry], true, false, false);
            assert!(results[0].error.is_none(), "{:?}", results[0].error);
            let code = results[0].code.clone().unwrap();
            assert!(code.contains("fastfail(") && code.contains("return 7;"), "{code}");
            outputs.push(code);
        }
        assert_eq!(outputs[0], outputs[1], "automatic and explicit Thumb agree with Listing {listing}");
    }
    let _ = std::fs::remove_file(fixture);
}

#[test]
fn non_arm_override_preserves_odd_pe_entries() {
    for machine in [
        object::pe::IMAGE_FILE_MACHINE_ARM,
        object::pe::IMAGE_FILE_MACHINE_THUMB,
        object::pe::IMAGE_FILE_MACHINE_ARMNT,
    ] {
        for header_entry in [false, true] {
            let code = [0xcc, 0xb8, 0x07, 0x00, 0x00, 0x00, 0xc3];
            let fixture = arm_machine_pe_with(Some(&code));
            let mut bytes = std::fs::read(&fixture).unwrap();
            let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
            bytes[pe + 4..pe + 6].copy_from_slice(&machine.to_le_bytes());
            let entry = if header_entry {
                let optional = pe + 24;
                let headers = u32::from_le_bytes(bytes[optional + 60..optional + 64].try_into().unwrap());
                let offset = headers as usize - code.len() - 1;
                bytes[offset..offset + code.len()].copy_from_slice(&code);
                let rva = offset as u32 + 1;
                bytes[optional + 16..optional + 20].copy_from_slice(&rva.to_le_bytes());
                0x400000 + u64::from(rva)
            } else {
                0x401001
            };
            assert_eq!(entry & 1, 1);
            std::fs::write(&fixture, bytes).unwrap();
            let path = fixture.to_string_lossy();
            for listing in ["off", "on"] {
                let Some(mut program) = load_with_decoder(
                    &path, "x86:LE:32:default:gcc", None, &[("listing", listing)],
                ) else {
                    return;
                };
                let entries = program.function_entries_canonical();
                assert_eq!(entries.len(), 1, "machine {machine:x}, Listing {listing}");
                assert_eq!(entries[0].addr.get_offset(), entry, "selected x86 entry remains odd");
                assert_eq!(entries[0].name, format!("sub_{entry:x}"));
                assert_eq!(program.image_metadata().unwrap().entry, Some(entry));
                let results = decompile_targets(&mut program, entries, true, false, false);
                assert!(results[0].error.is_none(), "{:?}", results[0].error);
                assert!(results[0].code.as_deref().unwrap().contains("return 7;"), "{:?}", results[0].code);
            }
            let _ = std::fs::remove_file(fixture);
        }
    }
}

#[test]
fn arm_override_normalizes_the_pe_entry_before_naming_it() {
    let fixture = arm_machine_pe_with(Some(&[0x07, 0x20, 0x70, 0x47]));
    let mut bytes = std::fs::read(&fixture).unwrap();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    bytes[pe + 4..pe + 6].copy_from_slice(&object::pe::IMAGE_FILE_MACHINE_I386.to_le_bytes());
    std::fs::write(&fixture, bytes).unwrap();
    let path = fixture.to_string_lossy();
    let Some(mut program) = load_with_decoder(
        &path, "ARM:LE:32:v8:default", Some(ArmIsa::Thumb), &[],
    ) else {
        return;
    };
    let entries = program.function_entries_canonical();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].addr.get_offset(), 0x401000);
    assert_eq!(entries[0].name, "sub_401000");
    assert_eq!(program.image_metadata().unwrap().entry, Some(0x401000));
    let results = decompile_targets(&mut program, entries, true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(results[0].code.as_deref().unwrap().contains("return 7;"), "{:?}", results[0].code);
    let _ = std::fs::remove_file(fixture);
}

/// The walk's paints reach the context database before the deferred Listing
/// consumers decode, so on a PE the Listing sees the entry as Thumb and finds
/// the callee its `bl` reaches; stashed only, it would have decoded the same
/// bytes as A32 and found nothing.
#[test]
fn the_listing_decodes_the_walked_entry_as_thumb() {
    let fixture = arm_machine_pe_with(Some(&[
        0x00, 0xf0, 0x02, 0xf8, // 0x401000 bl 0x401008
        0x70, 0x47, 0x00, 0xbf, // bx lr; nop
        0x07, 0x20, 0x70, 0x47, // 0x401008 movs r0,#7; bx lr
    ]));
    let path = fixture.to_string_lossy().into_owned();
    // The whole-binary drivers' non-x86-64 bundle: the Listing, and the
    // recursive-descent commit that promotes its followed CALL targets.
    let Some(program) = load(&path, &[("listing", "on"), ("funcstart_patterns", "on")]) else {
        return;
    };
    assert_eq!(tmode_at(&program, 0x401000), 1);
    assert_eq!(tmode_at(&program, 0x401008), 1, "the BL target is walked as Thumb");
    assert!(
        program.find_entry_at(0x401008).is_some(),
        "the Listing must discover the Thumb callee: {:?}",
        program
            .function_entries_canonical()
            .iter()
            .map(|entry| entry.addr.get_offset())
            .collect::<Vec<_>>()
    );
    let _ = std::fs::remove_file(fixture);
}

#[test]
fn an_arm_machine_pe_with_a_thumb_entry_gets_the_entry_walk() {
    let fixture = arm_machine_pe();
    let path = fixture.to_string_lossy().into_owned();
    let Some(program) = load(&path, &[]) else {
        return;
    };
    assert!(program.description().starts_with("ARM:LE:32"), "{}", program.description());
    let entry = program.find_entry_at(0x401001).expect("the odd PE entry resolves");
    assert_eq!(entry.addr.get_offset(), 0x401000);
    assert_eq!(entry.name, "sub_401000", "named at the even address it lives at");
    assert_eq!(tmode_at(&program, 0x401000), 1, "the walk paints the entry Thumb");

    let Some(off) = load(&path, &[("entrythumbflow", "off")]) else {
        return;
    };
    assert_eq!(tmode_at(&off, 0x401000), 0, "off leaves the language default");
    let _ = std::fs::remove_file(fixture);
}
