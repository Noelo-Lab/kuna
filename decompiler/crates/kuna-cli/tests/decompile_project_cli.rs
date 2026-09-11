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

/// The README's entry point is reported through the inventory: an ARM
/// `e_entry` carrying the Thumb mode bit (`0x100d7`) prints at the even
/// address every other artifact uses for that function.
#[test]
fn arm_thumb_readme_entry_point_is_the_even_inventory_address() {
    let Some(dir) = project("arm_thumb_linked_le32", "thumb_entry") else { return };
    let (_c, _h, _asm, readme) = artifacts(&dir, "arm_thumb_linked_le32");
    let readme_text = std::fs::read_to_string(&readme).unwrap();
    assert!(
        readme_text.contains("| Entry point | `0x100d6` |"),
        "README entry point must be the even inventory address:\n{readme_text}"
    );
    assert!(
        readme_text.contains("| `.text` | `0x"),
        "README must list the loader's named sections:\n{readme_text}"
    );
}

/// A relocatable object declares no entry and has no load addresses of its
/// own: the README says so, and lists the laid-out sections at the synthetic
/// layout the rest of the export uses, rather than every section at file
/// offset zero.
#[test]
fn relocatable_readme_has_no_entry_and_lists_laid_out_sections() {
    let Some(dir) = project("entry_selectors_x86_64.o", "reloc_readme") else { return };
    let (_c, _h, _asm, readme) = artifacts(&dir, "entry_selectors_x86_64.o");
    let readme_text = std::fs::read_to_string(&readme).unwrap();
    assert!(
        readme_text.contains("| Entry point | unavailable |"),
        "a relocatable object declares no entry:\n{readme_text}"
    );
    assert!(
        readme_text.contains("| `.text.selector_a` | `0x400000` |"),
        "sections must be listed at their synthetic load addresses:\n{readme_text}"
    );
    assert!(
        !readme_text.contains("| `.symtab` |") && !readme_text.contains("| `0x0` |"),
        "link-time-only sections and file-offset-zero rows must not appear:\n{readme_text}"
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
    // The export preserves the recovered prototype token-for-token, including
    // a possibly non-standard signature for `main`. Remap that reserved C
    // identifier so this test checks the header's syntax rather than asking the
    // host compiler to validate prototype recovery.
    let stub = dir.join("hcheck.c");
    std::fs::write(
        &stub,
        format!(
            "#define main kuna_recovered_main\n#include \"{}\"\nint stub_entry(void){{return 0;}}\n",
            h.display()
        ),
    )
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

/// `pdb_prog.exe` ships its matching `pdb_prog.pdb` beside it, and the shipped
/// `pdb` default (DIV-129) inventories every function that PDB names -- the hidden
/// leaf this test expects only fast discovery to find included. Both runs turn it
/// off so the discovery under test is the only one operating.
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
        "--option",
        "pdb",
        "off",
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
        "--option",
        "pdb",
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

/// A named global whose declared datatype is larger than the load image's
/// 512-byte read window took the whole export down: the loader answered every
/// read out of that window and indexed past its end, so `decompile-project`
/// panicked before it created the output directory and wrote nothing at all.
/// `regglobal_fmt_x86_64` carries a 40,000-byte `unused_word_type`, which is
/// dumped whole -- the loader serves the read the declared type asks for.
#[test]
fn a_global_larger_than_the_load_window_is_dumped_whole() {
    let Some(dir) = project("regglobal_fmt_x86_64", "big_global") else { return };
    let (_c, _h, asm, _readme) = artifacts(&dir, "regglobal_fmt_x86_64");
    let asm_text = std::fs::read_to_string(&asm).unwrap();
    assert!(
        asm_text.contains("\nunused_word_type:"),
        "the 40,000-byte global is missing from the data tail"
    );
    let rows = asm_text
        .lines()
        .skip_while(|l| !l.starts_with("unused_word_type:"))
        .skip(1)
        .take_while(|l| l.starts_with("  "))
        .count();
    assert_eq!(rows, 40_000 / 16, "the declared 40,000 bytes, 16 to a row");
}

/// The same defect on the shape that made it reachable everywhere: a plain
/// string. `tests/bug-repro/sort` carries a 604-byte usage string at `0x16600`,
/// and reading it took the export down before `create_dir_all` ran, so the run
/// wrote nothing at all -- the same failure `/bin/ls` and 52 of the 219 ELF
/// binaries in `/usr/bin` hit. `--addr` selects one function; the `.asm` data
/// tail, where the read happens, is emitted for the whole image regardless.
#[test]
fn a_binary_with_a_string_over_the_load_window_still_exports() {
    let bin = repo_root().join("tests/bug-repro/sort");
    let dir = out_dir("big_string");
    let (_stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        bin.to_str().unwrap(),
        "-o",
        dir.to_str().unwrap(),
        "--addr",
        "0x3000",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("a_binary_with_a_string_over_the_load_window_still_exports: skipping: {stderr}");
            return;
        }
        panic!("kuna decompile-project failed on tests/bug-repro/sort: {stderr}");
    }
    let (c, h, asm, readme) = artifacts(&dir, "sort");
    for f in [&c, &h, &asm, &readme] {
        assert!(f.exists(), "missing artifact {}", f.display());
    }
    let asm_text = std::fs::read_to_string(&asm).unwrap();
    let rows: Vec<&str> = asm_text
        .lines()
        .skip_while(|l| !l.starts_with("s_16600:"))
        .skip(1)
        .take_while(|l| l.starts_with("  "))
        .collect();
    assert_eq!(rows.len(), 38, "604 bytes, 16 to a row:\n{}", rows.join("\n"));
    assert!(rows[0].contains("KEYDEF is F[.C]"), "first row: {:?}", rows[0]);
    assert!(rows[37].starts_with("  00016850: 20 73 75 66 66 69 78 65 73"), "last row: {:?}", rows[37]);
}

/// Every artifact, not just the `.c`.  The `.h` is the one that could plausibly
/// move: its type block renders the type factory AFTER the loop, and a decompile
/// can intern a type into it, so a sharded run has one factory per worker where a
/// serial run has one.  The block therefore travels back from the workers, and
/// this is what holds it to the serial rendering.
#[test]
fn jobs_project_artifacts_are_byte_identical_to_serial() {
    let bin = fixture("dwarfstructs_x86_64");
    let serial = out_dir("jobs_serial");
    let (_, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        serial.to_str().unwrap(),
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("jobs project: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("serial project export failed: {stderr}");
    }
    let names = [
        "dwarfstructs_x86_64.c",
        "dwarfstructs_x86_64.h",
        "dwarfstructs_x86_64.asm",
        "README.md",
    ];
    let want: Vec<Vec<u8>> =
        names.iter().map(|n| std::fs::read(serial.join(n)).expect(n)).collect();
    // The .h really does carry recovered aggregates, or this proves nothing.
    let header = String::from_utf8_lossy(&want[1]).into_owned();
    assert!(
        header.contains("struct Nest {"),
        "the fixture stopped exercising the type block:\n{header}"
    );

    for (jobs, chunk) in [("2", "1"), ("4", "3"), ("8", "100")] {
        let dir = out_dir(&format!("jobs_{jobs}_{chunk}"));
        let (_, stderr, ok) = run_kuna(&[
            "decompile-project",
            &bin,
            "-o",
            dir.to_str().unwrap(),
            "--max-fn-seconds",
            "0",
            "--jobs",
            jobs,
            "--jobs-chunk",
            chunk,
            "--sleighpath",
            &specs(),
        ]);
        assert!(ok, "--jobs {jobs} project export failed: {stderr}");
        for (name, want) in names.iter().zip(&want) {
            let got = std::fs::read(dir.join(name)).expect(name);
            assert!(&got == want, "--jobs {jobs} (chunk {chunk}) moved {name}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
    let _ = std::fs::remove_dir_all(&serial);
}

// --- `--stream` ---------------------------------------------------------------

/// Run a streamed export into a fresh temp dir; `None` on a specs-less skip.
fn stream_project(fixture_name: &str, tag: &str, extra: &[&str]) -> Option<PathBuf> {
    let bin = fixture(fixture_name);
    let dir = out_dir(tag);
    let specs = specs();
    let mut args: Vec<&str> = vec![
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--stream",
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        &specs,
    ];
    args.extend_from_slice(extra);
    let (_stdout, stderr, ok) = run_kuna(&args);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("stream project: skipping (no `.sla`; run `make specs`): {stderr}");
            return None;
        }
        panic!("streamed project export failed on {fixture_name}: {stderr}");
    }
    Some(dir)
}

/// The raw token a compact-JSON field carries (`"name":"main"` -> `"main"`).
fn json_field<'a>(line: &'a str, key: &str) -> &'a str {
    let needle = format!("\"{key}\":");
    let at = line
        .find(&needle)
        .unwrap_or_else(|| panic!("index line has no {key}: {line}"))
        + needle.len();
    let rest = &line[at..];
    let end = match rest.strip_prefix('"') {
        Some(body) => body.find('"').expect("unterminated string") + 2,
        None => rest.find([',', '}']).expect("unterminated value"),
    };
    &rest[..end]
}

/// The `// Function:` blocks of a `.c`, as a sorted multiset.
fn c_blocks(text: &str) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    for (i, part) in text.split("// Function: ").enumerate() {
        if i == 0 {
            continue;
        }
        blocks.push(format!("// Function: {part}"));
    }
    blocks.sort();
    blocks
}

fn header_halves(text: &str) -> (String, Vec<String>) {
    let (types, protos) = text
        .split_once("/* function prototypes */")
        .unwrap_or_else(|| panic!(".h has no prototype section:\n{text}"));
    let mut lines: Vec<String> = protos.lines().map(str::to_string).collect();
    lines.sort();
    (types.to_string(), lines)
}

/// Everything a streamed export promises about its artifacts, against the
/// non-stream export of the same binary: the same function SET, the same
/// prototypes, the same disassembly, and an `index.jsonl` that slices the `.c`.
fn assert_stream_matches_serial(
    stream: &std::path::Path,
    serial: &std::path::Path,
    file_name: &str,
    exact_types: bool,
) {
    assert!(!stream.join(".streaming").exists(), ".streaming outlived a successful export");

    let c = std::fs::read_to_string(stream.join(format!("{file_name}.c"))).unwrap();
    let plain_c = std::fs::read_to_string(serial.join(format!("{file_name}.c"))).unwrap();
    assert_eq!(c_blocks(&c), c_blocks(&plain_c), "the streamed .c is not the same function set");

    let index = std::fs::read_to_string(stream.join("index.jsonl")).unwrap();
    let lines: Vec<&str> = index.lines().collect();
    assert_eq!(lines.len(), c_blocks(&c).len(), "one index line per function");
    for line in &lines {
        let offset: usize = json_field(line, "c_offset").parse().unwrap();
        let len: usize = json_field(line, "c_len").parse().unwrap();
        let name = json_field(line, "name").trim_matches('"');
        let addr = json_field(line, "addr").trim_matches('"');
        let block = &c[offset..offset + len];
        assert!(
            block.starts_with(&format!("// Function: {name} @ {addr}")),
            "index line does not slice its own block: {line}\n{block:?}"
        );
    }

    let (types, protos) = header_halves(
        &std::fs::read_to_string(stream.join(format!("{file_name}.h"))).unwrap(),
    );
    let (plain_types, plain_protos) = header_halves(
        &std::fs::read_to_string(serial.join(format!("{file_name}.h"))).unwrap(),
    );
    assert_eq!(protos, plain_protos, "the streamed .h declares a different prototype set");
    if exact_types {
        assert_eq!(types, plain_types, "the serial streamed .h type block moved");
    } else {
        let mut got: Vec<&str> = types.lines().collect();
        let mut want: Vec<&str> = plain_types.lines().collect();
        got.sort_unstable();
        want.sort_unstable();
        assert_eq!(got, want, "the sharded streamed .h type block is not the serial one's union");
    }

    let asm = std::fs::read_to_string(stream.join(format!("{file_name}.asm"))).unwrap();
    let plain_asm = std::fs::read_to_string(serial.join(format!("{file_name}.asm"))).unwrap();
    let (sweep, tails) = asm.split_once("\n; --- variables ---\n").expect("no variables section");
    let (plain_sweep, plain_tail) =
        plain_asm.split_once("\n; --- data ---\n").expect("no data tail");
    let stripped: String = plain_sweep
        .lines()
        .filter(|l| !l.starts_with("; arg:") && !l.starts_with("; stack:"))
        .map(|l| format!("{l}\n"))
        .collect();
    assert_eq!(
        sweep,
        stripped,
        "the streamed sweep is not the non-stream disassembly minus its variable comments"
    );
    let (vars, tail) = tails.split_once("\n; --- data ---\n").expect("no data tail");
    assert_eq!(tail, plain_tail, "the streamed data tail moved");
    let moved: Vec<&str> =
        vars.lines().filter(|l| l.starts_with("; arg:") || l.starts_with("; stack:")).collect();
    let removed: Vec<&str> = plain_sweep
        .lines()
        .filter(|l| l.starts_with("; arg:") || l.starts_with("; stack:"))
        .collect();
    assert_eq!(moved, removed, "the variables section is not what the labels lost");
}

/// The streamed export writes the same export, in a different order, and says
/// where everything landed.
#[test]
fn streamed_artifacts_match_the_non_stream_export() {
    let Some(stream) = stream_project("dwarfstructs_x86_64", "stream_serial", &[]) else {
        return;
    };
    let serial = out_dir("stream_reference");
    let (_, stderr, ok) = run_kuna(&[
        "decompile-project",
        &fixture("dwarfstructs_x86_64"),
        "-o",
        serial.to_str().unwrap(),
        "--max-fn-seconds",
        "0",
        "--sleighpath",
        &specs(),
    ]);
    assert!(ok, "the reference project export failed: {stderr}");
    // The fixture really does carry recovered aggregates, or the type-block
    // comparison proves nothing.
    let header =
        std::fs::read_to_string(serial.join("dwarfstructs_x86_64.h")).unwrap();
    assert!(header.contains("struct Nest {"), "the fixture stopped exercising the type block");

    assert_stream_matches_serial(&stream, &serial, "dwarfstructs_x86_64", true);

    let Some(pooled) =
        stream_project("dwarfstructs_x86_64", "stream_j3", &["--jobs", "3", "--jobs-chunk", "1"])
    else {
        for dir in [stream, serial] {
            let _ = std::fs::remove_dir_all(dir);
        }
        return;
    };
    assert_stream_matches_serial(&pooled, &serial, "dwarfstructs_x86_64", false);

    for dir in [stream, serial, pooled] {
        let _ = std::fs::remove_dir_all(dir);
    }
}

/// The point of the feature: the entry point and what it reaches are written
/// first, ahead of functions that come earlier in address order.
#[test]
fn streamed_blocks_follow_the_entry_point_not_the_address_order() {
    let Some(dir) = stream_project("fauxware", "stream_order", &[]) else { return };
    let index = std::fs::read_to_string(dir.join("index.jsonl")).unwrap();
    let order: Vec<String> = index
        .lines()
        .map(|l| json_field(l, "name").trim_matches('"').to_string())
        .collect();
    let at = |name: &str| {
        order
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} missing from the streamed order: {order:?}"))
    };
    assert_eq!(order[0], "_start", "the image entry point is written first: {order:?}");
    // `_init` and `sub_400500` are the two lowest addresses in the binary and
    // the entry point reaches neither, so address order would put them first.
    for reached in ["main", "authenticate", "accepted", "rejected"] {
        for unreached in ["_init", "sub_400500"] {
            assert!(
                at(reached) < at(unreached),
                "{reached} is reachable from the entry and must precede {unreached}: {order:?}"
            );
        }
    }
    let _ = std::fs::remove_dir_all(dir);
}

/// A selected export stays selected: a callee hint that names a function the
/// run did not ask for is scheduling noise, not a target.
#[test]
fn streamed_selected_project_does_not_expand_to_callees() {
    let bin = fixture("pdb_prog.exe");
    let dir = out_dir("stream_selected");
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--stream",
        "--addr",
        "0x140001010",
        "--mode",
        "fast",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("streamed_selected_project_does_not_expand_to_callees: skipping: {stderr}");
            return;
        }
        panic!("streamed selected project failed: {stderr}");
    }
    let c = std::fs::read_to_string(dir.join("pdb_prog.exe.c")).unwrap();
    assert!(c.contains("@ 0x140001010"), "selected function missing:\n{c}");
    assert!(!c.contains("@ 0x140001000"), "selector expanded to an unrequested callee:\n{c}");
    assert_eq!(
        std::fs::read_to_string(dir.join("index.jsonl")).unwrap().lines().count(),
        1,
        "one target, one index line"
    );
    assert!(stdout.contains("functions: 1 ok, 0 failed"), "unexpected project summary: {stdout}");
    let _ = std::fs::remove_dir_all(dir);
}

/// An unqualified `--assert` directive binds to "the function under decompile",
/// which a streamed whole-binary run would apply to every function in turn.
#[test]
fn stream_refuses_an_assertion_and_is_not_a_whole_binary_flag() {
    let bin = fixture("fauxware");
    let dir = out_dir("stream_assert");
    let (_, stderr, ok) = run_kuna(&[
        "decompile-project",
        &bin,
        "-o",
        dir.to_str().unwrap(),
        "--stream",
        "--assert",
        "name main authenticated",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "--stream with --assert must be refused");
    assert!(
        stderr.contains("--assert and --stream are exclusive"),
        "unexpected refusal: {stderr}"
    );
    assert!(!dir.exists(), "a refused run must not create the output folder");

    for cmd in ["decompile-all", "decompile-graph"] {
        let (_, stderr, ok) = run_kuna(&[cmd, &bin, "--stream", "--sleighpath", &specs()]);
        assert!(!ok, "{cmd} must not accept --stream");
        assert!(stderr.contains("unknown option --stream"), "{cmd}: {stderr}");
    }
}

/// A failed load is reported in `.streaming`, and leaves whatever a previous
/// export wrote into that folder alone — the whole reason nothing is truncated
/// before the program loads, README.md included.
#[test]
fn a_failed_load_reports_itself_and_spares_a_previous_export() {
    let Some(dir) = project("fauxware", "stream_failed") else { return };
    let (c, h, asm, readme) = artifacts(&dir, "fauxware");
    let before: Vec<Vec<u8>> =
        [&c, &h, &asm, &readme].iter().map(|f| std::fs::read(f).unwrap()).collect();

    // NAMED after the fixture and outside the folder, so a run that did truncate
    // its artifacts would truncate exactly the ones this test hashes.
    let junk_dir = out_dir("stream_failed_input");
    std::fs::create_dir_all(&junk_dir).unwrap();
    let junk = junk_dir.join("fauxware");
    std::fs::write(&junk, b"this is not an object file\n").unwrap();
    let (_, stderr, ok) = run_kuna(&[
        "decompile-project",
        junk.to_str().unwrap(),
        "-o",
        dir.to_str().unwrap(),
        "--stream",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "a load failure must exit nonzero: {stderr}");

    let status = std::fs::read_to_string(dir.join(".streaming")).unwrap();
    assert!(status.contains("\"phase\":\"failed\""), "no failed phase: {status}");
    assert!(status.contains("\"schema\":1"), "no schema: {status}");
    assert!(!status.contains("\"error\":null"), "a failed run must say why: {status}");
    let after: Vec<Vec<u8>> =
        [&c, &h, &asm, &readme].iter().map(|f| std::fs::read(f).unwrap()).collect();
    assert_eq!(before, after, "a failed load overwrote a previous export's artifacts");
    for dir in [dir, junk_dir] {
        let _ = std::fs::remove_dir_all(dir);
    }
}

/// With no previous export in the folder there is nothing to put back, so the
/// failure is reported in the README instead — and a failed export does not
/// claim to be still streaming.
#[test]
fn a_failed_load_into_an_empty_folder_leaves_a_failed_readme() {
    let junk_dir = out_dir("stream_failed_fresh_input");
    std::fs::create_dir_all(&junk_dir).unwrap();
    let junk = junk_dir.join("notabinary");
    std::fs::write(&junk, b"this is not an object file\n").unwrap();
    let dir = out_dir("stream_failed_fresh");
    let (_, stderr, ok) = run_kuna(&[
        "decompile-project",
        junk.to_str().unwrap(),
        "-o",
        dir.to_str().unwrap(),
        "--stream",
        "--sleighpath",
        &specs(),
    ]);
    assert!(!ok, "a load failure must exit nonzero: {stderr}");
    let readme = std::fs::read_to_string(dir.join("README.md")).unwrap();
    assert!(readme.contains("| Phase | failed |"), "the README must report the failure: {readme}");
    assert!(!readme.contains("still streaming"), "a failed export is not streaming: {readme}");
    assert!(!dir.join("notabinary.c").exists(), "a failed load must write no .c");
    for dir in [dir, junk_dir] {
        let _ = std::fs::remove_dir_all(dir);
    }
}

/// A path that does not exist is refused before anything is created, the way
/// the non-stream export refuses it — an explicit `--mode` skips the file stat
/// the argument parser would otherwise do.
#[test]
fn a_missing_binary_creates_no_folder() {
    let dir = out_dir("stream_missing");
    let missing = dir.join("nope.bin");
    let specs = specs();
    for extra in [vec![], vec!["-o", dir.to_str().unwrap()]] {
        let mut args = vec![
            "decompile-project",
            missing.to_str().unwrap(),
            "--stream",
            "--mode",
            "fast",
            "--sleighpath",
            &specs,
        ];
        args.extend(extra);
        let (_, stderr, ok) = run_kuna(&args);
        assert!(!ok, "a missing binary must fail: {stderr}");
        assert!(stderr.contains("binary not found"), "unexpected error: {stderr}");
        assert!(!dir.exists(), "a missing binary must not create a folder: {stderr}");
        assert!(
            !PathBuf::from("nope.bin.kuna").exists(),
            "a missing binary must not create a folder in the cwd"
        );
    }
}

/// The writer owns every artifact a reader polls, so when it cannot write one
/// the run stops there and reports ITS error — rather than decompiling the rest
/// of the binary into a channel nobody is reading.
#[test]
fn a_dead_writer_stops_the_run_and_reports_its_own_error() {
    let bin = fixture("mcount_x86_64");
    let dir = out_dir("stream_writer_death");
    std::fs::create_dir_all(&dir).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-project",
            &bin,
            "-o",
            dir.to_str().unwrap(),
            "--stream",
            "--max-fn-seconds",
            "0",
            "--sleighpath",
            &specs(),
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn the kuna binary");

    // One index line proves the artifacts exist and the writer is running.  Then
    // the `.h` it rewrites on its clock becomes a directory, so its next atomic
    // replace fails the way a full disk or a lost mount would.
    let header = dir.join("mcount_x86_64.h");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
    let mut injected = None;
    while std::time::Instant::now() < deadline {
        let lines =
            std::fs::read_to_string(dir.join("index.jsonl")).map_or(0, |t| t.lines().count());
        if lines >= 1 {
            std::fs::remove_file(&header).unwrap();
            std::fs::create_dir(&header).unwrap();
            injected = Some(std::time::Instant::now());
            break;
        }
        if child.try_wait().unwrap().is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let Some(injected) = injected else {
        let out = child.wait_with_output().unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        if is_specs_skip(&stderr) {
            eprintln!("a_dead_writer_stops_the_run: skipping (no `.sla`): {stderr}");
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }
        panic!("the export never wrote an index line: {stderr}");
    };

    let out = child.wait_with_output().unwrap();
    let stopped_after = injected.elapsed();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "a dead writer must fail the run: {stderr}");
    assert!(
        stderr.contains("mcount_x86_64.h") && stderr.contains("Is a directory"),
        "the run must report the writer's own error, not a sentinel: {stderr}"
    );

    let status = std::fs::read_to_string(dir.join(".streaming")).unwrap();
    assert!(status.contains("\"phase\":\"failed\""), "no failed phase: {status}");
    assert!(
        status.contains("mcount_x86_64.h") && status.contains("Is a directory"),
        ".streaming must carry the real path and errno: {status}"
    );
    let done: usize = json_field(&status, "functions_done").parse().unwrap();
    let total: usize = json_field(&status, "functions_total").parse().unwrap();
    assert!(done < total, "the run wrote {done} of {total} and must not have finished");
    // The whole export is ~97 s serially; stopping is the point of the signal.
    assert!(
        stopped_after < std::time::Duration::from_secs(60),
        "the run took {stopped_after:?} to stop after its writer died"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// Strip an ELF's section headers (`e_shoff`/`e_shnum`/`e_shstrndx`), which
/// kuna loads through its segment fallback.  Such an image publishes no CODE
/// section, so there is nothing to sweep.
fn section_header_stripped(fixture_name: &str, tag: &str) -> (PathBuf, PathBuf) {
    let dir = out_dir(tag);
    std::fs::create_dir_all(&dir).unwrap();
    let mut bytes = std::fs::read(fixture(fixture_name)).unwrap();
    assert_eq!(&bytes[..4], b"\x7fELF", "{fixture_name} is not an ELF");
    assert_eq!(bytes[4], 2, "{fixture_name} is not ELF64");
    bytes[0x28..0x30].fill(0); // e_shoff
    bytes[0x3c..0x40].fill(0); // e_shnum, e_shstrndx
    let path = dir.join(fixture_name);
    std::fs::write(&path, &bytes).unwrap();
    (dir, path)
}

/// An image with no CODE section has nothing to sweep, so its `.asm` is final
/// before the first function is decompiled — and `.streaming` has to say
/// `complete` rather than `sweeping` for the whole run, because an agent that
/// waits for `complete` before reading the `.asm` would otherwise wait for the
/// export it did not need.
#[test]
fn a_sectionless_image_never_reports_a_sweeping_asm_at_jobs_1() {
    let (junk_dir, bin) = section_header_stripped("fauxware", "stream_sectionless_input");
    let dir = out_dir("stream_sectionless");
    std::fs::create_dir_all(&dir).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-project",
            bin.to_str().unwrap(),
            "-o",
            dir.to_str().unwrap(),
            "--stream",
            "--jobs",
            "1",
            "--max-fn-seconds",
            "0",
            "--sleighpath",
            &specs(),
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn the kuna binary");

    let mut seen: Vec<String> = Vec::new();
    while child.try_wait().unwrap().is_none() {
        if let Ok(status) = std::fs::read_to_string(dir.join(".streaming")) {
            if seen.last().map(String::as_str) != Some(status.trim()) {
                seen.push(status.trim().to_string());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let out = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        if is_specs_skip(&stderr) {
            eprintln!("a_sectionless_image: skipping (no `.sla`): {stderr}");
            for dir in [dir, junk_dir] {
                let _ = std::fs::remove_dir_all(dir);
            }
            return;
        }
        panic!("the sectionless streamed export failed: {stderr}");
    }
    assert!(!seen.is_empty(), "the status file was never observed");
    for status in &seen {
        assert!(
            !status.contains("\"asm\":\"sweeping\""),
            "there is no CODE section to sweep: {status}"
        );
        if status.contains("\"phase\":\"decompiling\"") || status.contains("\"phase\":\"finalizing\"")
        {
            assert!(
                status.contains("\"asm\":\"complete\""),
                "the .asm was final before the first function: {status}"
            );
        }
    }
    assert!(!dir.join(".streaming").exists(), ".streaming outlived a successful export");
    let asm = std::fs::read_to_string(dir.join("fauxware.asm")).unwrap();
    assert!(asm.contains("\n; --- variables ---\n"), "the .asm is missing its variables tail");
    assert!(
        std::fs::read_to_string(dir.join("index.jsonl")).unwrap().lines().count() > 0,
        "the export decompiled nothing"
    );
    for dir in [dir, junk_dir] {
        let _ = std::fs::remove_dir_all(dir);
    }
}
