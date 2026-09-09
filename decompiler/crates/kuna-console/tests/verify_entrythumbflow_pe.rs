//! The `entrythumbflow` walk on the object path: a PE whose machine word
//! leaves the ARM mode to the entry bit (`IMAGE_FILE_MACHINE_ARM`) and whose
//! `AddressOfEntryPoint` carries it gets the same entry-reachable Thumb
//! context a TE with that shape gets. The fixture is the vendored Thumb PE
//! with its machine word rewritten from `THUMB` (painted wholly Thumb by the
//! Windows reading) to `ARM`.

mod common;

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object_with_isa, ConsoleProgram};

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

fn tmode_at(program: &ConsoleProgram, offset: u64) -> u32 {
    let space = program.arch().manage().get_default_code_space().cloned().unwrap();
    program
        .arch()
        .with_context_db_mut(|db| db.get_variable_value(b"TMode", &Address::new(space, offset)))
        .unwrap()
}

fn load(path: &str, options: &[(&str, &str)]) -> Option<ConsoleProgram> {
    let mut program = match bootstrap_from_object_with_isa(path, "", &specs(), None) {
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
