//! ARM PE input-context regression coverage.

mod common;

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object_with_isa, ArmIsa};
use kuna_console::project::decompile_targets;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe")
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

#[test]
fn thumb_machine_pe_normalizes_entry_and_decompiles() {
    let mut program = match bootstrap_from_object_with_isa(&fixture(), "", &specs(), None) {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "verify_arm_pe_context: skipping (build the ARM `.sla`): {}",
                error.explain()
            );
            return;
        }
    };
    program.commit_pending_analysis().unwrap();

    let odd = program
        .find_entry_at(0x401001)
        .expect("odd PE entry must resolve");
    let even = program
        .find_entry_at(0x401000)
        .expect("even PE entry must resolve");
    assert_eq!(odd.addr.get_offset(), 0x401000);
    assert_eq!(even.addr.get_offset(), 0x401000);

    let result = decompile_targets(&mut program, vec![odd], true, false, false);
    assert_eq!(result.len(), 1);
    assert!(result[0].error.is_none(), "{:?}", result[0].error);
    let code = result[0].code.as_deref().unwrap();
    assert!(
        code.contains("return 7;"),
        "expected Thumb body, got:\n{code}"
    );
}

#[test]
fn explicit_thumb_mode_uses_container_mapping() {
    let mut program = match bootstrap_from_object_with_isa(
        &fixture(),
        "ARM:LE:32:v4t:default",
        &specs(),
        Some(ArmIsa::Thumb),
    ) {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "verify_arm_pe_context: skipping (build the ARM `.sla`): {}",
                error.explain()
            );
            return;
        }
    };
    program.commit_pending_analysis().unwrap();
    let entry = program.find_entry_at(0x401001).unwrap();
    let result = decompile_targets(&mut program, vec![entry], true, false, false);
    assert!(result[0].code.as_deref().unwrap().contains("return 7;"));
}

#[test]
fn unmarked_trivial_decode_requires_explicit_isa() {
    let source = std::fs::read(fixture()).unwrap();
    let mut bytes = source.clone();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    bytes[pe + 4..pe + 6].copy_from_slice(&0x01c0u16.to_le_bytes());
    let path = common::scratch_file("arm-mode", "exe");
    std::fs::write(&path, bytes).unwrap();
    let path_text = path.to_str().unwrap();
    let trivial = "void sub_401000(void)\n{\n  return;\n}";

    let mut automatic =
        bootstrap_from_object_with_isa(path_text, "ARM:LE:32:v4t:default", &specs(), None).unwrap();
    automatic.commit_pending_analysis().unwrap();
    let diagnostic = automatic
        .arm_isa_diagnostic_for_output(0x401000, trivial)
        .expect("alternate Thumb return must make a trivial A32 decode ambiguous");
    assert!(diagnostic.contains("--isa thumb"));

    let mut explicit_arm = bootstrap_from_object_with_isa(
        path_text,
        "ARM:LE:32:v4t:default",
        &specs(),
        Some(ArmIsa::Arm),
    )
    .unwrap();
    explicit_arm.commit_pending_analysis().unwrap();
    assert!(
        explicit_arm
            .arm_isa_diagnostic_for_output(0x401000, trivial)
            .is_none(),
        "explicit A32 context must suppress automatic switching"
    );

    let mut arm_bytes = source;
    let pe = u32::from_le_bytes(arm_bytes[0x3c..0x40].try_into().unwrap()) as usize;
    arm_bytes[pe + 4..pe + 6].copy_from_slice(&0x01c0u16.to_le_bytes());
    arm_bytes[pe + 40..pe + 44].copy_from_slice(&0x1000u32.to_le_bytes());
    arm_bytes[0x200..0x204].copy_from_slice(&[0x1e, 0xff, 0x2f, 0xe1]);
    let arm_path = common::scratch_file("empty-arm", "exe");
    std::fs::write(&arm_path, arm_bytes).unwrap();
    let mut arm = bootstrap_from_object_with_isa(
        arm_path.to_str().unwrap(),
        "ARM:LE:32:v4t:default",
        &specs(),
        None,
    )
    .unwrap();
    arm.commit_pending_analysis().unwrap();
    let entry = arm.find_entry_at(0x401000).unwrap();
    let result = decompile_targets(&mut arm, vec![entry], true, false, false);
    assert!(result[0].error.is_none(), "genuine A32 return was rejected");
    assert!(result[0].code.as_deref().unwrap().contains("return;"));

    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(arm_path).unwrap();
}
