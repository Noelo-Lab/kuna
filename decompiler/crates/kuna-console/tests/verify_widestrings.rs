//! End-to-end gate for the 2-byte width of the string-literal pass
//! (`widestrings` — `kuna_widestrings::scan_wide_strings`).
//!
//! Both arms run against the vendored `widestrings_x86_64.exe`, a synthesized
//! PE32+ whose entry passes a UTF-16LE `L"ntdll.dll"` to `LoadLibraryW` and an
//! ASCII `"NtQueryInformationProcess"` to `GetProcAddress` — the shape
//! crackmes.one/65acadf3eef082e477ff5ede presents, reduced to the smallest image
//! that reproduces it.
//!
//! * **on** (default): the wide literal commits a typelocked `wchar2[10]` at its
//!   address and the call renders `LoadLibraryW(L"ntdll.dll")`.
//! * **off**: nothing is marked up at 2-byte width, the literal is read at 1-byte
//!   width as the one-character string behind the first NUL, and the same call
//!   renders `LoadLibraryW("n")` — the recorded defect. The `"n"` spelling needs
//!   `operand_refs` (a `--mode aggressive` member, and `auto` picks aggressive for
//!   anything under 500 KiB, so it is what the `kuna` binary runs by default):
//!   its run test accepts a SINGLE visible character, so at the wide literal it
//!   reads the first unit plus its high-byte NUL as a complete `char[2]`. With
//!   `operand_refs` off too, nothing types the constant at all and the call
//!   renders the bare `LoadLibraryW(0x140002100)`. Both off-arms are asserted.
//!
//! The ASCII argument is the control: it must render identically on both arms, so
//! the width the pass already had is untouched.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_*` gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/widestrings_x86_64.exe")
}

/// Bootstrap the fixture, set `widestrings`, commit the analysis facts and return
/// the C of the entry function — `None` when the `.sla` is not built.
fn decompile_entry(widestrings: bool) -> Option<String> {
    decompile_entry_with(widestrings, false)
}

/// The same, with the `operand_refs` scalar-markup pass forced on — the shape a
/// default `kuna decompile` runs (`auto` -> `aggressive`).
fn decompile_entry_with(widestrings: bool, operand_refs: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str()?.to_string()];
    let bin = fixture().to_str()?.to_string();

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_widestrings: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().analysis_widestrings = widestrings;
    prog.arch_mut().analysis_operand_refs = operand_refs;
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        vec!["load function sub_140001000".into(), "decompile".into(), "print C".into()];
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}

#[test]
fn wide_literal_renders_whole_with_the_width_on() {
    let Some(out) = decompile_entry(true) else { return };
    assert!(
        out.contains(r#"LoadLibraryW(L"ntdll.dll")"#),
        "expected the whole wide literal `LoadLibraryW(L\"ntdll.dll\")`, got:\n{out}"
    );
    assert!(
        !out.contains(r#"LoadLibraryW("n")"#),
        "the one-character 1-byte read must not survive the 2-byte width, got:\n{out}"
    );
    // The control: the ASCII width still owns its own literal.
    assert!(
        out.contains(r#""NtQueryInformationProcess""#),
        "the 1-byte width must still render its own literal, got:\n{out}"
    );
}

#[test]
fn wide_literal_is_unmarked_with_the_width_off() {
    let Some(out) = decompile_entry(false) else { return };
    assert!(
        !out.contains("ntdll.dll"),
        "off must mark up nothing at 2-byte width, got:\n{out}"
    );
    assert!(
        out.contains("LoadLibraryW(0x140002100)"),
        "off must leave the wide literal's address untyped, got:\n{out}"
    );
    // The control: the ASCII width is untouched by the gate.
    assert!(
        out.contains(r#""NtQueryInformationProcess""#),
        "the 1-byte width must be unaffected by the gate, got:\n{out}"
    );
}

/// The recorded defect verbatim, on the pass set a default `kuna decompile` runs:
/// with `operand_refs` on and the 2-byte width off, the wide literal renders as
/// its own first character. With the width on it renders whole — and `widestrings`
/// wins the shared commit stream, which is the ordering half of the fix.
#[test]
fn the_width_outranks_the_scalar_markup_that_read_one_character() {
    let Some(off) = decompile_entry_with(false, true) else { return };
    assert!(
        off.contains(r#"LoadLibraryW("n")"#),
        "expected the recorded defect `LoadLibraryW(\"n\")` with operand_refs on \
         and the width off, got:\n{off}"
    );
    let Some(on) = decompile_entry_with(true, true) else { return };
    assert!(
        on.contains(r#"LoadLibraryW(L"ntdll.dll")"#),
        "the 2-byte width must outrank the scalar markup's one-character read, \
         got:\n{on}"
    );
}

/// The markup itself, not the rendering: the commit plants a typelocked data
/// symbol at the wide literal's address, and only with the width on.
#[test]
fn the_commit_plants_a_symbol_at_the_wide_literal() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str().unwrap().to_string();

    for (on, want) in [(true, true), (false, false)] {
        let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
            Ok(p) => p,
            Err(_) => return, // no `.sla`: the sibling tests already report the skip
        };
        prog.arch_mut().analysis_widestrings = on;
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        let arch = prog.arch();
        let global = arch.symboltab.get_global_scope().expect("global scope");
        let found = !arch.symboltab.query_by_name(global, "s_140002100").is_empty();
        assert_eq!(
            found, want,
            "widestrings={on}: expected the wide literal's data symbol \
             `s_140002100` to be {}",
            if want { "planted" } else { "absent" }
        );
    }
}
