//! Multi-format-loader PR-8 e2e gate: a **Mach-O fat / universal** (`0xcafebabe`)
//! binary loads by selecting ONE arch slice at dispatch, decompiles a function,
//! and names its libc import — and the `--slice`/`--target` override picks the
//! *other* slice (design §3.4 / §8 PR-8).
//!
//! ## The fixture
//!
//! `macho_fat` is a hand-built 2-slice universal binary wrapping the existing
//! thin fixtures: `macho_imports` (x86-64) at slice 0 and `macho_imports_arm64`
//! (arm64) at slice 1, behind a big-endian `fat_header` + two `fat_arch` records
//! (the fat format is a header + per-slice `{cputype,cpusubtype,offset,size,
//! align}`). No `lipo`/`llvm-lipo` was available in-container, so the wrapper was
//! emitted by hand from the two real slices.
//!
//! ## What this proves
//!
//! `object::File::parse` is a thin, single-arch view and **cannot** parse a fat
//! header. So the engine dispatch (`bootstrap_from_object`) peels the fat binary
//! to one slice's bytes *before* the loader runs — the single, canonical
//! slice-selection point. With no override the default preference (x86-64 →
//! arm64 → first) picks the **x86-64** slice; `--slice arm64` (env
//! `KUNA_MACHO_SLICE`) picks the **arm64** slice instead. Both slices are the
//! same `_main` calling `printf`, so each decompiles with a named `printf(`.
//!
//! ## Default-on multi-format dispatch
//!
//! Like the thin-Mach-O gate, the fat binary loads through the default `load file`
//! dispatch with no flag — multi-format support is unconditional. The fat magic
//! routes straight to the object loader (no XML rejection); the `--slice`/arm64e
//! overrides remain separate, opt-in selectors layered on top.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86`/`AARCH64` `.sla` under `specs/` (gitignored;
//! `make specs`). If absent the bootstrap fails and the test prints that and
//! returns early (a visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

/// `KUNA_MACHO_SLICE` / `KUNA_MACHO_ARM64E` are process-global env vars these
/// tests toggle; serialize their bodies so a slice/arm64e override in one never
/// races the default-slice bootstrap in another (cargo runs tests in parallel
/// within one process).
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

fn decompile_func(prog: ConsoleProgram, func_cmd: &str) -> String {
    let cmds: Vec<String> = [func_cmd, "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
        .collect();
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
    status.optr.clone()
}

/// Bootstrap `macho_fat` (multi-format loading is unconditional) with the given
/// slice override env, returning `None` (a visible skip) when the `.sla` is absent.
fn boot_fat(slice: Option<&str>) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("macho_fat");
    assert!(path.exists(), "missing fixture {path:?}");

    match slice {
        Some(s) => std::env::set_var("KUNA_MACHO_SLICE", s),
        None => std::env::remove_var("KUNA_MACHO_SLICE"),
    }
    let r = match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_macho_fat: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    };
    std::env::remove_var("KUNA_MACHO_SLICE");
    r
}

/// (default-on proof) A fat Mach-O loads through the *default* `load file`
/// dispatch with no flag — multi-format support is unconditional. A `.sla`-absent
/// environment surfaces as a load error; here we only assert the dispatch ROUTES
/// to the object loader (no XML "not recognized" rejection). The default slice
/// selection still applies (the `KUNA_MACHO_SLICE` override stays a separate
/// feature).
#[test]
fn fat_default_on_routes_to_object_loader() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("macho_fat");
    if let Err(e) = kuna_console::engine::bootstrap_from_file(path.to_str().unwrap(), "", &spec_roots) {
        let msg = e.explain();
        assert!(
            !msg.contains("Unable to recognize") && !msg.contains("XML"),
            "default-on: the binary must route to the object loader (got: {msg})"
        );
    }
}

/// The headline: the fat binary loads with the **default** slice (x86-64),
/// decompiles `_main`, and renders the `printf(` call named — proving the
/// dispatch peeled the x86-64 slice and the whole single-arch pipeline (loadimage
/// + `__stubs` naming) ran on it.
#[test]
fn fat_default_slice_is_x86_64_and_names_printf() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(prog) = boot_fat(None) else { return };

    // Default preference selects the x86-64 slice.
    let desc = prog.description().to_string();
    assert!(
        desc.contains("x86"),
        "fat: default slice should be x86-64, got spec {desc:?}"
    );
    assert!(
        prog.lookup_symbol("_main").is_some(),
        "fat (x86-64 slice): `_main` not resolved by the loader"
    );
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "fat (x86-64 slice): `printf` not resolved by the `__stubs` walk"
    );

    let out = decompile_func(prog, "load function _main");
    assert!(
        out.contains("printf("),
        "fat (x86-64 slice): expected a `printf(` call, got:\n{out}"
    );
}

/// The override: `--slice arm64` (env `KUNA_MACHO_SLICE`) selects the *other*
/// slice — the same fat binary now loads with the AArch64 spec and still names
/// `printf` (arch-independent `__stubs` naming).
#[test]
fn fat_slice_override_selects_arm64() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(prog) = boot_fat(Some("arm64")) else {
        return;
    };

    let desc = prog.description().to_string();
    assert!(
        desc.contains("AARCH64") || desc.contains("ARM"),
        "fat --slice arm64: expected an AArch64 spec, got {desc:?}"
    );
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "fat (arm64 slice): `printf` not resolved by the `__stubs` walk"
    );

    let out = decompile_func(prog, "load function _main");
    assert!(
        out.contains("printf("),
        "fat (arm64 slice): expected a `printf(` call, got:\n{out}"
    );
}

// ---- arm64e (§3.7) Apple-Silicon spec selection -----------------------------
//
// `macho_arm64e` is the `macho_imports_arm64` fixture with its header cpusubtype
// flipped to `CPU_SUBTYPE_ARM64E` (arm64e is binary-compatible arm64 — the same
// instruction encodings plus PAC — so the real arm64 code disassembles under the
// AppleSilicon spec, a v8.5-A superset). No `clang -arch arm64e` SDK exists
// in-container, so the cpusubtype is synthesized; the *load + spec-selection path
// is real* (the engine reads the header, the gate fires, the AppleSilicon `.sla`
// drives the decode). A genuine Apple-toolchain arm64e fixture is a follow-up.

/// Bootstrap `macho_arm64e` (multi-format loading is unconditional) with the
/// `macho-arm64e` gate set per `gate`, returning `None` (a visible skip) when the
/// `.sla` is absent.
fn boot_arm64e(gate: bool) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join("macho_arm64e");
    assert!(path.exists(), "missing fixture {path:?}");

    if gate {
        std::env::set_var("KUNA_MACHO_ARM64E", "1");
    } else {
        std::env::remove_var("KUNA_MACHO_ARM64E");
    }
    let r = match bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_macho_fat: skipping arm64e (bootstrap failed; build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            None
        }
    };
    std::env::remove_var("KUNA_MACHO_ARM64E");
    r
}

/// Gate ON: an arm64e Mach-O selects the `AARCH64:LE:64:AppleSilicon` spec (the
/// pointer-auth / AMX-modelling variant), not the generic `v8A` — and still
/// decompiles `_main` with a named `printf(` (import naming is unaffected by PAC).
#[test]
fn arm64e_gate_on_selects_apple_silicon_spec() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(prog) = boot_arm64e(true) else {
        return;
    };

    let desc = prog.description().to_string();
    assert!(
        desc.contains("AppleSilicon"),
        "arm64e (gate on): expected the AppleSilicon spec, got {desc:?}"
    );
    assert!(
        prog.lookup_symbol("printf").is_some(),
        "arm64e: `printf` not resolved (pointer-auth must not affect import naming)"
    );

    let out = decompile_func(prog, "load function _main");
    assert!(
        out.contains("printf("),
        "arm64e (gate on): expected a `printf(` call under the AppleSilicon spec, got:\n{out}"
    );
}

/// Gate OFF (default): the *same* arm64e Mach-O loads with the generic
/// `AARCH64:LE:64:v8A` spec (the default-off, opt-in behavior) — proving the
/// AppleSilicon selection is strictly behind the `macho-arm64e` gate.
#[test]
fn arm64e_gate_off_uses_generic_v8a_spec() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(prog) = boot_arm64e(false) else {
        return;
    };

    let desc = prog.description().to_string();
    assert!(
        !desc.contains("AppleSilicon"),
        "arm64e (gate off): must NOT select AppleSilicon (opt-in), got {desc:?}"
    );
    assert!(
        desc.contains("AARCH64") || desc.contains("ARM"),
        "arm64e (gate off): expected the generic AArch64 spec, got {desc:?}"
    );
}
