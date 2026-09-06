//! CLI end-to-end gate for `kuna decompile-project` — drives the built `kuna`
//! binary over the vendored `fauxware` (x86-64) and `arm_thumb_linked_le32`
//! (ARM Thumb) ELF fixtures and asserts the four project artifacts
//! (`.c` / `.h` / `.asm` / `README.md`) are written, cross-referenced, and
//! (for the `.h`) syntactically valid C.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` / `ARM` `.sla` under `specs/` (gitignored;
//! `make specs`).  When it is absent the command fails to build an architecture;
//! the affected test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// A unique per-test output directory under the system temp dir.
fn out_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "kuna_decompile_project_{tag}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn run_kuna(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// Run `decompile-project` on `fixture_name` into a fresh temp dir; returns
/// `Some(out_dir)` on success or `None` on a specs-less skip (panics on any
/// other failure).
fn project(fixture_name: &str, tag: &str) -> Option<PathBuf> {
    let bin = fixture(fixture_name);
    let dir = out_dir(tag);
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_project_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return None;
        }
        panic!("kuna decompile-project failed on {fixture_name}: {stderr}");
    }
    Some(dir)
}

/// The four artifact paths for a `<file_name>` project export.
fn artifacts(dir: &std::path::Path, file_name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    (
        dir.join(format!("{file_name}.c")),
        dir.join(format!("{file_name}.h")),
        dir.join(format!("{file_name}.asm")),
        dir.join("README.md"),
    )
}

#[test]
fn project_folder_written_for_fauxware() {
    let Some(dir) = project("fauxware", "written") else { return };
    let (c, h, asm, readme) = artifacts(&dir, "fauxware");
    for f in [&c, &h, &asm, &readme] {
        assert!(f.exists(), "missing artifact {}", f.display());
    }
    let c_text = std::fs::read_to_string(&c).unwrap();
    assert!(c_text.contains("#include \"fauxware.h\""), ".c missing header include:\n{c_text}");
    assert!(c_text.contains("// Function: main"), ".c missing main function header:\n{c_text}");

    let h_text = std::fs::read_to_string(&h).unwrap();
    assert!(
        h_text.contains("typedef unsigned int undefined4;"),
        ".h missing the undefined4 typedef:\n{h_text}"
    );
    // The `main` prototype line ends in `;` (token-identical to the .c
    // definition's signature line).
    let main_proto = h_text
        .lines()
        .find(|l| l.contains("main(") && !l.contains("//"))
        .unwrap_or_else(|| panic!(".h missing a main( prototype:\n{h_text}"));
    assert!(main_proto.trim_end().ends_with(';'), "main prototype must end in `;`: {main_proto:?}");

    let readme_text = std::fs::read_to_string(&readme).unwrap();
    assert!(readme_text.contains("x86:LE:64"), "README missing arch id:\n{readme_text}");
    assert!(readme_text.contains("Entry point"), "README missing entry point:\n{readme_text}");
    assert!(readme_text.contains("## Sections"), "README missing sections table:\n{readme_text}");
}

/// The README's entry-point row was `object`'s raw `entry()`, which on a Mach-O
/// `LC_MAIN` image is a `__TEXT`-relative file offset -- so the export claimed an
/// entry at `0x5b0` for a program whose `main` is at `0x1000005b0`.
#[test]
fn macho_readme_entry_point_is_a_vma() {
    let Some(dir) = project("macho_stripped_main", "macho_entry") else { return };
    let (_c, _h, _asm, readme) = artifacts(&dir, "macho_stripped_main");
    let readme_text = std::fs::read_to_string(&readme).unwrap();
    assert!(
        readme_text.contains("| Entry point | `0x1000005b0` |"),
        "README entry point must be the VMA:\n{readme_text}"
    );
}

#[test]
fn asm_labels_match_c_function_names() {
    let Some(dir) = project("fauxware", "labels") else { return };
    let (c, _h, asm, _r) = artifacts(&dir, "fauxware");
    let c_text = std::fs::read_to_string(&c).unwrap();
    let asm_text = std::fs::read_to_string(&asm).unwrap();
    // Every `// Function: <name>` in the .c must have a `<name>:` label in the .asm.
    let mut checked = 0;
    for line in c_text.lines() {
        if let Some(rest) = line.strip_prefix("// Function: ") {
            let name = rest.split_whitespace().next().unwrap();
            assert!(
                asm_text.lines().any(|l| l.starts_with(&format!("{name}:"))),
                "asm has no `{name}:` label for the .c function {name:?}"
            );
            checked += 1;
        }
    }
    assert!(checked > 1, "expected several functions cross-checked, got {checked}");
}

#[test]
fn asm_has_stack_comment_for_main() {
    let Some(dir) = project("fauxware", "stack") else { return };
    let (_c, _h, asm, _r) = artifacts(&dir, "fauxware");
    let asm_text = std::fs::read_to_string(&asm).unwrap();
    // Find `main:` and assert a `; stack:` or `; arg:` line appears in its
    // header block (before the first instruction / blank separator).
    let mut lines = asm_text.lines();
    let found = lines.by_ref().any(|l| l.starts_with("main:"));
    assert!(found, "no main: label in asm:\n{asm_text}");
    let header: Vec<&str> = lines.take_while(|l| l.starts_with(';')).collect();
    assert!(
        header.iter().any(|l| l.starts_with("; stack:") || l.starts_with("; arg:")),
        "main: has no stack/arg comment block:\n{}",
        header.join("\n")
    );
}

#[test]
fn dat_labels_cross_referenced() {
    let Some(dir) = project("fauxware", "dat") else { return };
    let (c, _h, asm, _r) = artifacts(&dir, "fauxware");
    let c_text = std::fs::read_to_string(&c).unwrap();
    let asm_text = std::fs::read_to_string(&asm).unwrap();

    // Collect every `dat_<hex>` token in the .c (same policy as the emitter:
    // preceding char must not be an identifier char).
    let bytes = c_text.as_bytes();
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut tokens = std::collections::BTreeSet::new();
    let mut i = 0;
    while let Some(pos) = c_text[i..].find("dat_") {
        let start = i + pos;
        let hs = start + 4;
        let mut he = hs;
        while he < bytes.len() && matches!(bytes[he], b'0'..=b'9' | b'a'..=b'f') {
            he += 1;
        }
        let prev_ok = start == 0 || !is_ident(bytes[start - 1]);
        let next_ok = he == bytes.len() || !is_ident(bytes[he]);
        if prev_ok && next_ok && he > hs {
            tokens.insert(c_text[start..he].to_string());
        }
        i = he.max(hs);
    }
    assert!(!tokens.is_empty(), "fauxware .c should reference some dat_<hex> data:\n{c_text}");

    // Label policy (documented in decompile_project.rs): a bare `dat_<hex>`
    // gets its own `dat_<hex>:` label line; when a NAMED global covers the
    // address the symbol name is the label and the `dat_` spelling is appended
    // as `= dat_<hex>`.  Either form satisfies the cross-reference.
    for t in &tokens {
        let has_label = asm_text.lines().any(|l| l.starts_with(&format!("{t}:")))
            || asm_text.contains(&format!("= {t}"));
        assert!(has_label, "dat token {t:?} from the .c has no label/alias in the .asm");
    }
}

#[test]
fn header_syntax_checks_with_cc() {
    // Gated: only run when a C compiler is available.
    if Command::new("cc").arg("--version").output().map(|o| !o.status.success()).unwrap_or(true) {
        eprintln!("header_syntax_checks_with_cc: skipping (no working `cc`)");
        return;
    }
    let Some(dir) = project("fauxware", "cc") else { return };
    let (_c, h, _asm, _r) = artifacts(&dir, "fauxware");
    // A stub that includes the .h; it must NOT define `main` (the recovered
    // header declares `void main(void)`, so an `int main` would legitimately
    // conflict — we are asserting the .h itself is valid, not the decompiled C).
    let stub = dir.join("hcheck.c");
    std::fs::write(&stub, format!("#include \"{}\"\nint stub_entry(void){{return 0;}}\n", h.display()))
        .unwrap();
    let out = Command::new("cc")
        .args(["-std=c99", "-fsyntax-only", stub.to_str().unwrap()])
        .output()
        .expect("spawn cc");
    assert!(
        out.status.success(),
        "generated .h failed cc -fsyntax-only:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn arm_thumb_project_smoke() {
    let Some(dir) = project("arm_thumb_linked_le32", "arm") else { return };
    let (_c, _h, asm, _r) = artifacts(&dir, "arm_thumb_linked_le32");
    assert!(asm.exists(), "arm project missing .asm");
    let asm_text = std::fs::read_to_string(&asm).unwrap();
    assert!(!asm_text.trim().is_empty(), "arm .asm is empty");
    // The non-x86 sweep + ARM Thumb odd-address normalization must still label
    // functions (at least one `<name>:  ; 0x` line).
    assert!(
        asm_text.lines().any(|l| l.contains(":  ; 0x")),
        "arm .asm has no function labels:\n{asm_text}"
    );
}

#[test]
fn fast_project_discovers_real_function_bodies() {
    let bin = fixture("pdb_prog.exe");
    let dir = out_dir("fast_bodies");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--mode",
        "fast",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("fast_project_discovers_real_function_bodies: skipping: {stderr}");
            return;
        }
        panic!("fast project failed: {stderr}");
    }
    let c = std::fs::read_to_string(dir.join("pdb_prog.exe.c")).unwrap();
    assert!(c.contains("@ 0x140001010"), "entry function missing:\n{c}");
    assert!(c.contains("@ 0x140001000"), "direct-call target missing:\n{c}");
    assert!(c.contains("return a1 * 7 + a0 * 3;"), "discovered function has no real body:\n{c}");
    assert!(stdout.contains("functions: 2 ok, 0 failed"), "unexpected project summary: {stdout}");

    let control = out_dir("fast_bodies_off");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        control.to_str().unwrap(),
        "--mode",
        "fast",
        "--option",
        "fast_funcdisc",
        "off",
        "--sleighpath",
        &specs(),
    ]);
    assert!(ok, "fast discovery control failed: {stderr}");
    let control_c = std::fs::read_to_string(control.join("pdb_prog.exe.c")).unwrap();
    assert!(
        !control_c.contains("@ 0x140001000"),
        "disabled fast discovery unexpectedly found the hidden leaf:\n{control_c}"
    );
}

#[test]
fn fast_project_discovers_indirect_pointer_target() {
    let bin = fixture("aif_gap_x86_64");
    let dir = out_dir("fast_pointer");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--mode",
        "fast",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("fast_project_discovers_indirect_pointer_target: skipping: {stderr}");
            return;
        }
        panic!("fast pointer project failed: {stderr}");
    }
    let c = std::fs::read_to_string(dir.join("aif_gap_x86_64.c")).unwrap();
    assert!(c.contains("@ 0x13ae"), "pointer-only function missing:\n{c}");
    assert!(
        c.contains("return (a0 + 0x40) * 2 + 9;"),
        "pointer-only function has no real body:\n{c}"
    );

    let control = out_dir("fast_pointer_off");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        control.to_str().unwrap(),
        "--mode",
        "fast",
        "--option",
        "fast_funcdisc",
        "off",
        "--sleighpath",
        &specs(),
    ]);
    assert!(ok, "fast pointer discovery control failed: {stderr}");
    let control_c = std::fs::read_to_string(control.join("aif_gap_x86_64.c")).unwrap();
    assert!(
        !control_c.contains("@ 0x13ae"),
        "disabled fast discovery unexpectedly found the pointer-only target:\n{control_c}"
    );
}

#[test]
fn fast_project_does_not_promote_switch_case_labels() {
    let bin = fixture("switchtab_x86_64");
    let dir = out_dir("fast_switch_labels");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--mode",
        "fast",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("fast_project_does_not_promote_switch_case_labels: skipping: {stderr}");
            return;
        }
        panic!("fast switch-table project failed: {stderr}");
    }
    let c = std::fs::read_to_string(dir.join("switchtab_x86_64.c")).unwrap();
    for case in [
        0x401119_u64,
        0x40111f,
        0x401125,
        0x40112b,
        0x401131,
        0x401137,
        0x40113d,
        0x401149,
    ] {
        assert!(
            !c.contains(&format!("@ 0x{case:x}")),
            "switch case 0x{case:x} was promoted to a function:\n{c}"
        );
    }
}

#[test]
fn fast_selected_project_does_not_expand_to_callees() {
    let bin = fixture("pdb_prog.exe");
    let dir = out_dir("fast_selected");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--addr",
        "0x140001010",
        "--mode",
        "fast",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("fast_selected_project_does_not_expand_to_callees: skipping: {stderr}");
            return;
        }
        panic!("fast selected project failed: {stderr}");
    }
    let c = std::fs::read_to_string(dir.join("pdb_prog.exe.c")).unwrap();
    assert!(c.contains("@ 0x140001010"), "selected function missing:\n{c}");
    assert!(!c.contains("@ 0x140001000"), "selector expanded to an unrequested callee:\n{c}");
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "unexpected project summary: {stdout}");
}
