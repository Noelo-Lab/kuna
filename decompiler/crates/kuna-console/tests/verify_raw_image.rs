//! Headerless raw-image bootstrap and address-translation regressions.

mod common;

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::{bootstrap_from_raw, ArmIsa};
use kuna_console::project::decompile_targets;

struct RawFixture(PathBuf);

impl RawFixture {
    fn thumb_return_7() -> Self {
        Self::new("raw-thumb", &[0x07, 0x20, 0x70, 0x47])
    }

    fn new(stem: &str, bytes: &[u8]) -> Self {
        let path = common::scratch_file(stem, "bin");
        std::fs::write(&path, bytes).unwrap();
        Self(path)
    }

    fn thumb_return_7_with_spaces() -> Self {
        let path = common::scratch_file("raw thumb image", "bin");
        std::fs::write(&path, [0x07, 0x20, 0x70, 0x47]).unwrap();
        Self(path)
    }
}

impl Drop for RawFixture {
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

fn specs() -> Vec<String> {
    vec![std::env::var("KUNA_SPECS")
        .unwrap_or_else(|_| repo_root().join("specs").to_string_lossy().into_owned())]
}

fn run_console(script: &str, isa: Option<&str>) -> std::process::Output {
    let roots = specs();
    let mut command = Command::new(env!("CARGO_BIN_EXE_decomp_dbg"));
    command
        .args(["-sleighpath", &roots[0]])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    match isa {
        Some(isa) => {
            command.env(kuna_console::engine::ARM_ISA_ENV, isa);
        }
        None => {
            command.env_remove(kuna_console::engine::ARM_ISA_ENV);
        }
    }
    let mut child = command.spawn().expect("spawn decomp_dbg");
    child
        .stdin
        .as_mut()
        .expect("console stdin")
        .write_all(script.as_bytes())
        .expect("write console commands");
    child.wait_with_output().expect("wait for decomp_dbg")
}

#[test]
fn raw_thumb_maps_base_zero_and_nonzero() {
    let fixture = RawFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    for (base, selected) in [(0, 0), (0x4000, 0x4001)] {
        let mut program = match bootstrap_from_raw(
            &path,
            "ARM:LE:32:v4t:default",
            base,
            &[base, selected],
            Some(ArmIsa::Thumb),
            &specs(),
        ) {
            Ok(program) => program,
            Err(error) => {
                eprintln!(
                    "verify_raw_image: skipping (build the ARM `.sla`): {}",
                    error.explain()
                );
                return;
            }
        };
        program.commit_pending_analysis().unwrap();
        assert_eq!(program.sections(), vec![(base, 4, 4)]);
        assert_eq!(program.num_symbols(), 1);
        let entry = program
            .find_entry_at(selected)
            .expect("seeded raw entry must resolve");
        assert_eq!(entry.addr.get_offset(), base);
        assert!(program.entry_bytes_mapped(&entry.addr));

        let results = decompile_targets(&mut program, vec![entry], true, false, false);
        assert_eq!(results.len(), 1);
        assert!(results[0].error.is_none(), "{:?}", results[0].error);
        let code = results[0].code.as_deref().unwrap();
        assert!(
            code.contains("return 7;"),
            "unexpected raw Thumb body:\n{code}"
        );
    }
}

#[test]
fn word_addressed_targets_scale_base_and_entries_to_byte_offsets() {
    let fixture = RawFixture::new("raw-avr-nops", &[0, 0, 0, 0]);
    let path = fixture.0.to_string_lossy();

    let mut at_nonzero_base =
        match bootstrap_from_raw(&path, "avr8:LE:16:default", 0x100, &[0x100], None, &specs()) {
            Ok(program) => program,
            Err(error) => {
                eprintln!(
                    "verify_raw_image: skipping (build the AVR8 `.sla`): {}",
                    error.explain()
                );
                return;
            }
        };
    at_nonzero_base.commit_pending_analysis().unwrap();
    assert_eq!(at_nonzero_base.sections(), vec![(0x200, 4, 4)]);
    let entry = at_nonzero_base
        .find_entry_at(0x200)
        .expect("word-addressed base must become a byte VMA");
    assert!(at_nonzero_base.entry_bytes_mapped(&entry.addr));
    assert_eq!(at_nonzero_base.output_code_offset(entry.addr.get_offset()), 0x100);
    assert_eq!(at_nonzero_base.output_code_end_offset(0x201), 0x101);

    let mut at_second_word = bootstrap_from_raw(&path, "avr8:LE:16:default", 0, &[1], None, &specs())
        .expect("same built AVR8 specification");
    at_second_word.commit_pending_analysis().unwrap();
    let entry = at_second_word
        .find_entry_at(2)
        .expect("entry word 1 must select file byte offset 2");
    assert_eq!(entry.addr.get_offset(), 2);
    assert_eq!(at_second_word.output_code_offset(entry.addr.get_offset()), 1);
    assert!(at_second_word.entry_bytes_mapped(&entry.addr));
}

#[test]
fn raw_console_reports_word_addresses_in_target_units() {
    let fixture = RawFixture::new("raw-word-report", &[0, 0, 0x08, 0x95]);
    let roots = specs();
    if !PathBuf::from(&roots[0])
        .join("Ghidra/Processors/Atmel/data/languages/avr8.sla")
        .exists()
    {
        eprintln!("verify_raw_image: skipping console report test (no AVR8 `.sla`)");
        return;
    }
    let script = format!(
        "load raw avr8:LE:16:default 0x100 0x101 \"{}\"\n\
         read symbols\n\
         functions\n\
         load addr 0x101\n\
         decompile\n\
         region blocks\n\
         region tree\n\
         region walk\n\
         quit\n",
        fixture.0.display()
    );
    let output = run_console(&script, None);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    for expected in ["0x101 sub_101", "[0x101]", "region head=0x101", "walk 0x101"] {
        assert!(stdout.contains(expected), "missing {expected:?}:\n{stdout}\n{stderr}");
    }
    for byte_address in ["0x202 sub_101", "[0x202]", "region head=0x202", "walk 0x202"] {
        assert!(
            !stdout.contains(byte_address),
            "leaked byte address {byte_address:?}:\n{stdout}\n{stderr}"
        );
    }
}

#[test]
fn raw_map_function_normalizes_an_odd_thumb_pointer() {
    let fixture = RawFixture::thumb_return_7();
    let roots = specs();
    if !PathBuf::from(&roots[0])
        .join("Ghidra/Processors/ARM/data/languages/ARM8_le.sla")
        .exists()
    {
        eprintln!("verify_raw_image: skipping map function test (no ARM `.sla`)");
        return;
    }
    let script = format!(
        "load raw ARM:LE:32:v4t:default 0x4000 0x4000 \"{}\"\n\
         map function 0x4001 mapped_thumb\n\
         decompile\n\
         print C\n\
         quit\n",
        fixture.0.display()
    );
    let output = run_console(&script, Some("thumb"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    assert!(stdout.contains("mapped_thumb"), "{stdout}\n{stderr}");
    assert!(stdout.contains("return 7;"), "{stdout}\n{stderr}");
}

#[test]
fn console_load_raw_preserves_whitespace_in_final_filename() {
    let fixture = RawFixture::thumb_return_7_with_spaces();
    let roots = specs();
    if !PathBuf::from(&roots[0])
        .join("Ghidra/Processors/ARM/data/languages/ARM8_le.sla")
        .exists()
    {
        eprintln!("verify_raw_image: skipping whitespace path test (no ARM `.sla`)");
        return;
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_decomp_dbg"))
        .args(["-sleighpath", &roots[0]])
        .env(kuna_console::engine::ARM_ISA_ENV, "thumb")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn decomp_dbg");
    writeln!(
        child.stdin.as_mut().expect("console stdin"),
        "load raw ARM:LE:32:v4t:default 0x4000 0x4001 \"{}\"\nquit",
        fixture.0.display()
    )
    .expect("write console commands");
    let output = child.wait_with_output().expect("wait for decomp_dbg");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    assert!(stdout.contains("successfully loaded"), "{stdout}\n{stderr}");
}

#[test]
fn raw_input_rejects_missing_metadata_and_unmapped_entries() {
    let fixture = RawFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();

    let missing_target = bootstrap_from_raw(&path, "", 0, &[0], None, &specs())
        .err()
        .expect("missing target must fail")
        .explain()
        .to_string();
    assert!(missing_target.contains("requires --target"));

    let error = match bootstrap_from_raw(
        &path,
        "ARM:LE:32:v4t:default",
        0x4000,
        &[0x4000],
        None,
        &specs(),
    ) {
        Ok(_) => panic!("raw ARM input without --isa unexpectedly loaded"),
        Err(error) => error.explain().to_string(),
    };
    if error.contains("No sleigh specification") {
        eprintln!("verify_raw_image: skipping (build the ARM `.sla`): {error}");
        return;
    }
    assert!(
        error.contains("requires --isa"),
        "unexpected error: {error}"
    );
}

#[test]
fn raw_input_rejects_entry_at_end_of_mapping() {
    let fixture = RawFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    let error = match bootstrap_from_raw(
        &path,
        "ARM:LE:32:v4t:default",
        0x4000,
        &[0x4004],
        Some(ArmIsa::Thumb),
        &specs(),
    ) {
        Ok(_) => panic!("out-of-range raw entry unexpectedly loaded"),
        Err(error) => error.explain().to_string(),
    };
    if error.contains("No sleigh specification") {
        eprintln!("verify_raw_image: skipping (build the ARM `.sla`): {error}");
        return;
    }
    assert!(
        error.contains("outside mapped range"),
        "unexpected error: {error}"
    );
}

#[test]
fn word_addressed_mapping_errors_report_target_units() {
    let fixture = RawFixture::new("raw-word-bounds", &[0, 0, 0, 0]);
    let path = fixture.0.to_string_lossy();
    let error = match bootstrap_from_raw(
        &path,
        "avr8:LE:16:default",
        0x100,
        &[0x103],
        None,
        &specs(),
    ) {
        Ok(_) => panic!("out-of-range word-addressed entry unexpectedly loaded"),
        Err(error) => error.explain().to_string(),
    };
    if error.contains("No sleigh specification") {
        eprintln!("verify_raw_image: skipping target-unit bounds test (no AVR8 `.sla`): {error}");
        return;
    }
    assert!(
        error.contains("raw entry 0x103 is outside mapped range 0x100..0x102"),
        "unexpected error: {error}"
    );
    assert!(!error.contains("0x200..0x204"), "byte bounds leaked: {error}");
}

#[test]
fn raw_input_rejects_empty_images() {
    let fixture = RawFixture::new("raw-empty", &[]);
    let path = fixture.0.to_string_lossy();
    let error = match bootstrap_from_raw(
        &path,
        "ARM:LE:32:v4t:default",
        0,
        &[0],
        Some(ArmIsa::Thumb),
        &specs(),
    ) {
        Ok(_) => panic!("empty raw image unexpectedly loaded"),
        Err(error) => error.explain().to_string(),
    };
    if error.contains("No sleigh specification") {
        eprintln!("verify_raw_image: skipping (build the ARM `.sla`): {error}");
        return;
    }
    assert!(
        error.contains("raw image is empty"),
        "unexpected error: {error}"
    );
}

#[test]
fn raw_input_rejects_mapping_overflow() {
    let fixture = RawFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    let error = match bootstrap_from_raw(
        &path,
        "ARM:LE:32:v4t:default",
        u64::MAX,
        &[u64::MAX],
        Some(ArmIsa::Thumb),
        &specs(),
    ) {
        Ok(_) => panic!("overflowing raw mapping unexpectedly loaded"),
        Err(error) => error.explain().to_string(),
    };
    if error.contains("No sleigh specification") {
        eprintln!("verify_raw_image: skipping (build the ARM `.sla`): {error}");
        return;
    }
    assert!(error.contains("overflows"), "unexpected error: {error}");
}
