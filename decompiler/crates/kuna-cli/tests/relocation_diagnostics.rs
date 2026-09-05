//! Process-level proof that malformed/unsupported relocation volume cannot
//! produce unbounded stderr, including for machine-readable output.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

use object::write;
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind, SymbolScope,
};

const RELOCATION_COUNT: usize = 4_000;

struct SyntheticFixture(PathBuf);

impl SyntheticFixture {
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for SyntheticFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn unsupported_arm_relocations() -> SyntheticFixture {
    let mut object = write::Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.section_id(write::StandardSection::Text);
    let mut code = Vec::with_capacity(RELOCATION_COUNT * 4);
    for _ in 0..RELOCATION_COUNT {
        code.extend_from_slice(&0xe12f_ff1eu32.to_le_bytes()); // bx lr
    }
    object.append_section_data(text, &code, 4);
    let function = object.add_symbol(write::Symbol {
        name: b"synthetic_entry".to_vec(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Linkage,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    for offset in (0..code.len()).step_by(4) {
        object
            .add_relocation(
                text,
                write::Relocation {
                    offset: offset as u64,
                    symbol: function,
                    addend: 0,
                    // Deliberately outside the implemented inventory. The
                    // diagnostic must aggregate all identical entries.
                    flags: RelocationFlags::Elf {
                        r_type: object::elf::R_ARM_MOVW_ABS_NC,
                    },
                },
            )
            .unwrap();
    }

    let path = common::scratch_file("synthetic-relocation-diagnostics", "o");
    std::fs::write(&path, object.write().expect("write synthetic ET_REL"))
        .expect("store synthetic ET_REL");
    SyntheticFixture(path)
}

#[test]
fn json_functions_reports_exact_relocation_count_with_fixed_stderr_ceiling() {
    let root = repo_root();
    let specs = root.join("specs");
    if !specs
        .join("Ghidra/Processors/ARM/data/languages/ARM8_le.sla")
        .exists()
    {
        eprintln!("relocation_diagnostics: skipping (no built ARM8_le.sla)");
        return;
    }
    let fixture = unsupported_arm_relocations();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "functions",
            fixture.path().to_str().unwrap(),
            "--json",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--mode",
            "fast",
        ])
        .output()
        .expect("spawn kuna");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "functions failed: {stderr}");
    assert!(stderr.contains("4000 relocation(s) skipped"), "{stderr}");
    assert!(stderr.contains("count=4000"), "{stderr}");
    assert!(stderr.contains("sample(s)-suppressed=3997"), "{stderr}");
    assert!(
        stderr.len() < 2_048,
        "stderr was {} bytes: {stderr}",
        stderr.len()
    );
    assert_eq!(
        stderr.matches("[kuna ET_REL loader]").count(),
        2,
        "{stderr}"
    );
}
