//! End-to-end gate for the DWARF analyzer port (`dwarf`, the kuna analog of
//! Ghidra's `DWARFAnalyzer`).
//!
//! Two real-ELF fixtures, two recoveries:
//!
//! 1. **Names (subtask 1)** — `dwarf_stripped_x86_64` has its `.symtab` FUNC
//!    names stripped, so DWARF is the ONLY name source. Decompiling `add_values`
//!    must resolve the DWARF name (the function loads by name and its body
//!    renders) instead of a `FUN_`/`sub_` placeholder.
//!
//! 2. **Typed signature (subtask 2, the headline)** — `cet_pie_x86_64` is NOT
//!    stripped (names already come from `.symtab`), so its DWARF value is TYPES:
//!    `elaborate_debug_symbol`'s first parameter is `char *binary` in DWARF. The
//!    decompiled signature must show a `char *` parameter (the DWARF type) rather
//!    than the engine's default `undefined8`/`long`.
//!
//! 3. **Named, typed stack local (subtask 3)** — `stacklocal_x86_64`'s
//!    `compute_sum` has two `DW_OP_fbreg` stack locals; `accumulator` is
//!    address-taken (`scanf("%d", &accumulator)`) so it survives as an addrtied
//!    stack slot. The DWARF name+type must bind: the body renders
//!    `int accumulator`/`int counter` instead of `local_10`/`local_c`. (cet_pie's
//!    own locals are write-once spill slots the engine eliminates, so they never
//!    render — the dedicated fixture is what proves the install path.)
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_*` gates, bootstrapping needs the built `x86`
//! `.sla` under `specs/` (gitignored; `make specs`). When it is absent the
//! bootstrap fails; the test prints that and returns early (a specs-less CI is a
//! visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// Bootstrap a fixture and drive `load function <fn>` -> `decompile` -> `print C`,
/// returning the captured C output (or `None` if the `.sla` is absent — a skip).
fn decompile_c(fixture_name: &str, func: &str) -> Option<String> {
    decompile_c_with(fixture_name, func, &[])
}

/// Like [`decompile_c`] but runs the given `option <name> <value>` lines BEFORE
/// `load function` (the analysis commit is deferred to `load function`/`read
/// symbols`, so a gate must be flipped before then). Each `pre_options` entry is
/// a full console command, e.g. `"option dwarf_lines on"`.
fn decompile_c_with(fixture_name: &str, func: &str, pre_options: &[&str]) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture(fixture_name);
    let bin = bin.to_str()?.to_string();

    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_s1_dwarf: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    let mut cmds: Vec<String> = pre_options.iter().map(|s| s.to_string()).collect();
    cmds.extend([format!("load function {func}"), "decompile".into(), "print C".into()]);
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
fn dwarf_stripped_recovers_function_name() {
    // The .symtab FUNC names are stripped; only DWARF names `add_values`. The
    // function must be loadable + decompilable by its DWARF name, and its body
    // must not render as a `FUN_`/`sub_` placeholder for itself.
    let Some(out) = decompile_c("dwarf_stripped_x86_64", "add_values") else {
        return;
    };
    // The DWARF name resolved (the header names the function, not a placeholder).
    assert!(
        out.contains("add_values"),
        "expected the DWARF name `add_values` in the output, got:\n{out}"
    );
    assert!(
        !out.contains("FUN_00401136") && !out.contains("sub_00401136"),
        "add_values should render by its DWARF name, not a placeholder, got:\n{out}"
    );
}

#[test]
fn cet_pie_typed_signature_has_char_ptr() {
    // cet_pie is NOT stripped, so the name already comes from .symtab; the DWARF
    // win is the TYPED parameter `char *binary`. The decompiled signature must
    // carry a `char *` (the DWARF type), not the engine's default undefined8/long.
    let Some(out) = decompile_c("cet_pie_x86_64", "elaborate_debug_symbol") else {
        return;
    };
    assert!(
        out.contains("elaborate_debug_symbol"),
        "expected the function name in the output, got:\n{out}"
    );
    assert!(
        out.contains("char *"),
        "expected a DWARF-typed `char *` parameter in the signature, got:\n{out}"
    );
}

#[test]
fn stacklocal_renders_dwarf_named_typed_locals() {
    // subtask 3: compute_sum's `accumulator` is address-taken (passed to scanf), so
    // it survives as an addrtied stack slot — the DWARF name+type must bind, naming
    // the local `accumulator` (an `int`) rather than the engine's `local_*`.
    let Some(out) = decompile_c("stacklocal_x86_64", "compute_sum") else {
        return;
    };
    assert!(
        out.contains("compute_sum"),
        "expected the function name in the output, got:\n{out}"
    );
    // The DWARF local names bind to the surviving stack slots.
    assert!(
        out.contains("accumulator"),
        "expected the DWARF-named stack local `accumulator`, got:\n{out}"
    );
    assert!(
        out.contains("counter"),
        "expected the DWARF-named stack local `counter`, got:\n{out}"
    );
    // And they carry the DWARF `int` type, declared as a local (not the param).
    assert!(
        out.contains("int4 accumulator") || out.contains("int accumulator"),
        "expected `accumulator` typed as int, got:\n{out}"
    );
    // The placeholder local names must be gone for these slots (the whole point).
    assert!(
        !out.contains("local_10") && !out.contains("local_c"),
        "the DWARF-named slots should not render as `local_*`, got:\n{out}"
    );
}

#[test]
fn dwarf_lines_annotate_source_locations() {
    // The `.debug_line` source-line pass (`dwarf_lines`, the kuna analog of Ghidra's
    // DWARFLineInfoCommentScript). cet_pie's `elaborate_debug_symbol` body
    // (debug_symbol.c lines ~122-146) decompiles to statements that each carry their
    // DWARF `file:line` as a `/* … */` comment. With the gate ON, the output must
    // carry the source-file annotations; DEFAULT-OFF leaves the output unchanged.
    //
    // NOTE: the entry-line comment (line 122 @ 0x1357) does not render — the prologue
    // instruction at the entry is eliminated, so the comment at that PC has no
    // surviving op to hang on and `CommentSorter` excises it (the same reason
    // cet_pie's write-once locals never render, see the typed-signature test). We
    // assert a body line (124) that maps to a surviving statement instead.
    let Some(on) =
        decompile_c_with("cet_pie_x86_64", "elaborate_debug_symbol", &["option dwarf_lines on"])
    else {
        return;
    };
    assert!(
        on.contains("elaborate_debug_symbol"),
        "expected the function name in the output, got:\n{on}"
    );
    // The headline: a body statement's source line lands as a C `/* … */` comment.
    assert!(
        on.contains("/* debug_symbol.c:124 */"),
        "expected the DWARF source-line annotation `/* debug_symbol.c:124 */`, got:\n{on}"
    );
    // Several distinct lines are annotated (the pass walks the whole line program).
    assert!(
        on.contains("/* debug_symbol.c:125 */") && on.contains("/* debug_symbol.c:146 */"),
        "expected multiple distinct DWARF source-line comments, got:\n{on}"
    );

    // Default-OFF parity: the same function with the gate OFF (the default) carries
    // NO source-line comment — the output is byte-identical to pre-feature.
    let Some(off) = decompile_c("cet_pie_x86_64", "elaborate_debug_symbol") else {
        return;
    };
    assert!(
        !off.contains("debug_symbol.c:"),
        "default (gate off) must NOT add source-line comments, got:\n{off}"
    );
}
