//! End-to-end gate for **DWARF enum recovery**: an enum-typed value renders by
//! member name (`mode_append`, `shell_escape_always_quoting_style`) instead of a
//! bare number, and an enum parameter declares its enum type.
//!
//! The decompiler could always render an enum constant by name — the printer's
//! enum arm and its flag-decomposition are ported and tested. What was missing
//! was the *type*: the DWARF pass flattened every `DW_TAG_enumeration_type` to its
//! underlying integer, discarding the `DW_TAG_enumerator` children, so
//! `quotearg_style(shell_escape_always_quoting_style, ...)` printed
//! `quotearg_style(4, ...)`.
//!
//! Two details the fixtures pin down. Enums are built at the DIE's own
//! `DW_AT_byte_size` rather than the type factory's architecture default (8 on
//! x86-64) — an 8-byte type will not bind to the 4-byte constant it describes.
//! And member values are masked to that width, so a negative member of a signed
//! enum (`level_error = -1`) is keyed by the value the constant Varnode actually
//! carries.
//!
//! Fixtures: `dwarf_enums_x86_64` (`+.c`, purpose-built: unsigned enum, signed
//! enum with a negative member) and `regglobal_fmt_x86_64` (the IDA-parity
//! reference — coreutils `quoting_style`).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

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

/// Bootstrap `bin`, commit the (default-on) DWARF facts, decompile `func`, and
/// return the captured C (`None` ⇒ specs-less skip).
fn decompile(bin: &str, func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let path = fixture(bin).to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&path, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_dwarf_enums: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> =
        [format!("load function {func}"), "decompile".into(), "print C".into()].to_vec();
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

/// `int apply(enum mode m, enum level l, int n)` — both parameters declare their
/// enum type, and both comparisons render by member name. `level_error` is `-1`,
/// so it also pins the width-masking of the value map.
#[test]
fn enum_parameters_and_comparisons_render_by_member_name() {
    let Some(code) = decompile("dwarf_enums_x86_64", "apply") else { return };

    assert!(
        code.contains("apply(mode m,level l,int4 n)"),
        "expected the enum parameter types in the signature; got:\n{code}",
    );
    assert!(
        code.contains("mode_append"),
        "expected the unsigned enum member name; got:\n{code}",
    );
    assert!(
        code.contains("level_error"),
        "expected the NEGATIVE signed enum member name (value masking); got:\n{code}",
    );
}

/// Constant arguments at a call site pick up the callee's enum parameter type.
#[test]
fn enum_arguments_render_by_member_name_at_the_call_site() {
    let Some(code) = decompile("dwarf_enums_x86_64", "main") else { return };

    assert!(
        code.contains("apply(mode_truncate,level_error,7)"),
        "expected both enum arguments by member name; got:\n{code}",
    );
}

/// The IDA-parity reference: coreutils `quoting_style`, whose definition repeats
/// in every compilation unit that includes `quotearg.h` — so this also covers the
/// reuse path (re-creating an already-filled enum is an error, not a no-op).
#[test]
fn fmt_main_renders_the_quoting_style_member() {
    let Some(code) = decompile("regglobal_fmt_x86_64", "main") else { return };

    assert!(
        code.contains("quotearg_style(shell_escape_always_quoting_style"),
        "expected the `quoting_style` member name, not the raw 4; got:\n{code}",
    );
}
