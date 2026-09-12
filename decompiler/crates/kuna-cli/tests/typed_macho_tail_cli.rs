//! End-to-end regression for an import veneer whose whole body is
//! `jmp qword ptr [slot]`: the slot's callable type/prototype must seed the
//! CALLIND before parameter recovery, without treating arbitrary data as code.

use std::path::PathBuf;
use std::process::Command;

const STUB: &str = "0x1000005cc";
const SLOT: &str = "0x100003000";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/macho_imports")
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

fn run(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(args)
        .output()
        .expect("run kuna");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

#[test]
fn explicit_function_pointer_prototype_types_the_tail_call() {
    let bin = fixture();
    let sp = specs();
    let prototype = format!("prototype {STUB} int printf(char *fmt,int value)");
    let data = format!("data {SLOT} int (*printf_ptr)(char *,int)");
    let (stdout, stderr, ok) = run(&[
        "decompile",
        &bin,
        "printf",
        "--sleighpath",
        &sp,
        "--assert",
        &prototype,
        "--assert",
        &data,
        "--assert-strict",
    ]);
    if !ok && is_specs_skip(&stderr) {
        eprintln!("typed_macho_tail_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(ok, "explicit typed decompile failed: {stderr}");
    assert!(
        stdout.contains("int printf(char *fmt,int value)")
            && stdout.contains("(*printf_ptr)(fmt,value)"),
        "the function-pointer prototype did not reach the CALLIND:\n{stdout}"
    );
    assert!(
        !stdout.contains("(*printf_ptr)()"),
        "the typed call stayed empty:\n{stdout}"
    );
}

#[test]
fn non_callable_slot_declarations_do_not_clone_the_current_function_prototype() {
    let bin = fixture();
    let sp = specs();
    let prototype = format!("prototype {STUB} int printf(char *fmt,int value)");
    for declaration in [
        format!("data {SLOT} long printf_slot"),
        format!("data {SLOT} long *printf_slot"),
        format!("data {SLOT} code printf_slot"),
        format!("data {SLOT} code *printf_slot"),
    ] {
        let (stdout, stderr, ok) = run(&[
            "decompile",
            &bin,
            "printf",
            "--sleighpath",
            &sp,
            "--assert",
            &prototype,
            "--assert",
            &declaration,
            "--assert-strict",
        ]);
        if !ok && is_specs_skip(&stderr) {
            eprintln!("typed_macho_tail_cli: skipping (no `.sla`): {stderr}");
            return;
        }
        assert!(ok, "negative control rejected {declaration:?}: {stderr}");
        assert!(
            stdout.contains("int printf(char *fmt,int value)"),
            "the current prototype was not installed:\n{stdout}"
        );
        assert!(
            stdout.contains(")(); // jump-as-call"),
            "a non-callable slot declaration supplied arguments ({declaration:?}):\n{stdout}"
        );
        assert!(
            !stdout.contains("(fmt,value); // jump-as-call"),
            "the resolver cloned the current function prototype ({declaration:?}):\n{stdout}"
        );
    }
}

#[test]
fn default_single_batch_and_project_surfaces_keep_the_import_argument() {
    let bin = fixture();
    let sp = specs();

    let (single, stderr, ok) = run(&["decompile", &bin, "printf", "--sleighpath", &sp]);
    if !ok && is_specs_skip(&stderr) {
        eprintln!("typed_macho_tail_cli: skipping (no `.sla`): {stderr}");
        return;
    }
    assert!(ok, "single decompile failed: {stderr}");
    assert!(
        single.contains("(*dat_100003000)(a0)"),
        "single call lost a0:\n{single}"
    );

    let (batch, stderr, ok) = run(&[
        "decompile-all",
        &bin,
        "--filter",
        "^printf$",
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "batch decompile failed: {stderr}");
    assert!(
        batch.contains("(*dat_100003000)(a0)"),
        "batch call lost a0:\n{batch}"
    );

    let out_dir = std::env::temp_dir().join(format!(
        "kuna_typed_macho_tail_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let (_stdout, stderr, ok) = run(&[
        "decompile-project",
        &bin,
        "--functions",
        "printf",
        "-o",
        out_dir.to_str().unwrap(),
        "--sleighpath",
        &sp,
    ]);
    assert!(ok, "project decompile failed: {stderr}");
    let c = std::fs::read_to_string(out_dir.join("macho_imports.c")).expect("project C artifact");
    let _ = std::fs::remove_dir_all(&out_dir);
    assert!(
        c.contains("(*dat_100003000)(a0)"),
        "project call lost a0:\n{c}"
    );
}
