use crate::common;
use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture() -> Option<PathBuf> {
    process::optional_output(Command::new("clang").arg("--version"))?;
    let source = common::scratch_file("project-assertions", "s");
    let binary = source.with_extension("dll");
    std::fs::write(&source, r#".text
.globl _sample, _adjust
.def _sample; .scl 2; .type 32; .endef
_sample:
 push 4(%esp)
 call _adjust
 add $4,%esp
 ret
.def _adjust; .scl 2; .type 32; .endef
_adjust:
 mov 4(%esp),%eax
 add $9,%eax
 ret
"#).unwrap();
    let output = Command::new("clang")
        .args(["--target=i686-w64-windows-gnu", "-shared", "-nostdlib",
            "-Wl,--entry,0", "-Wl,--export-all-symbols"])
        .arg(&source).arg("-o").arg(&binary).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    Some(binary)
}

fn export(binary: &Path, directive: &str, strict: bool) -> std::process::Output {
    let directory = common::scratch_file("project-assertion-output", "dir");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.arg("decompile-project").arg(binary)
        .args(["--functions", "_sample", "--assert",
            "prototype _sample int __cdecl _sample(int value)", "--assert", directive])
        .arg("-o").arg(&directory);
    if strict { command.arg("--assert-strict"); }
    let output = command.output().unwrap();
    let name = binary.file_name().unwrap().to_str().unwrap();
    for suffix in ["c", "h", "asm"] {
        assert!(directory.join(format!("{name}.{suffix}")).is_file(), "{output:?}");
    }
    assert!(directory.join("README.md").is_file(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("wrote "), "{output:?}");
    std::fs::remove_dir_all(directory).unwrap();
    output
}

#[test]
fn rejected_callee_contract_is_reported_and_strict_export_fails() {
    let Some(binary) = fixture() else { return };
    for strict in [false, true] {
        let output = export(&binary,
            "prototype _adjust void missing_model _adjust(int value)", strict);
        assert_eq!(output.status.code(), Some(i32::from(strict)), "{output:?}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("rejected") && error.contains("_adjust"), "{error}");
    }
}

#[test]
fn rejected_local_directive_is_reported_and_strict_export_fails() {
    let Some(binary) = fixture() else { return };
    for strict in [false, true] {
        let output = export(&binary, "type _sample::no_such_local int", strict);
        assert_eq!(output.status.code(), Some(i32::from(strict)), "{output:?}");
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("rejected") && error.contains("no_such_local"), "{error}");
    }
}

#[test]
fn applied_contracts_keep_strict_export_successful() {
    let Some(binary) = fixture() else { return };
    let output = export(&binary,
        "prototype _adjust int __cdecl _adjust(int value)", true);
    assert!(output.status.success(), "{output:?}");
    assert!(!String::from_utf8_lossy(&output.stderr).contains("rejected"), "{output:?}");
}
