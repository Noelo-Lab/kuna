//! End-to-end gate for `pdatachained` (GH-403): a PE `.pdata` record whose
//! `UNWIND_INFO` carries `UNW_FLAG_CHAININFO` is a separated code chunk of the
//! function before it, not a function start.
//!
//! MSVC emits one such record per shrink-wrapped prologue or out-of-line cold
//! block. Claiming a function at its `BeginAddress` puts a known entry in the
//! middle of a body, and `funcboundflow` then truncates the real function at its
//! own chunk — which is how the reported `sub_140002650` ended after four
//! statements. The three x86-64 fixtures reproduce the three shapes that
//! truncation takes, all from the same defect:
//!
//! * `pe_chainedunwind_x86_64.exe` — the chunk begins at the fall-through of a
//!   conditional branch, and the truncated function emits `} while ;`, which is
//!   not C at all;
//! * `pe_chainedunwind_loop_x86_64.exe` — the same, with the branch a loop latch,
//!   and the decompile of the whole function fails outright;
//! * `pe_chainedunwind_plainft_x86_64.exe` — the chunk begins at an ordinary
//!   fall-through, and the function silently loses its second half.
//!
//! The gate is a process-global env var (the entry oracles run inside `load
//! file`, upstream of every per-function `option`), so both arms are driven
//! through it under one mutex.
//!
//! The parity corpora cannot see any of this: both are symbol-less bytechunks
//! that never construct an `ObjectLoadImage`, so a `.pdata` change is invisible
//! to `make test` and `make test-stages`.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` / `AARCH64` `.sla` under `specs/`
//! (gitignored; `make specs`). When absent the bootstrap fails, and the test
//! prints that and returns early — a visible skip, never a false green.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_pdatachained::PDATACHAINED_ENV;

/// Serializes the env-var flip + load across the concurrently-run tests: the gate
/// is a process-global env var and every object load reads it.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixtures() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

/// Load a fixture with the gate forced on/off, or left unset (`None`) to observe
/// the shipped default, and commit the analysis facts so the discovered entries
/// become visible symbols. `None` back means the `.sla` is absent (a visible skip).
fn boot(name: &str, gate: Option<bool>) -> Option<ConsoleProgram> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = fixtures().join(name);
    assert!(path.exists(), "missing fixture {path:?}");

    match gate {
        Some(on) => std::env::set_var(PDATACHAINED_ENV, if on { "on" } else { "off" }),
        None => std::env::remove_var(PDATACHAINED_ENV),
    }
    let booted = bootstrap_from_object(path.to_str().unwrap(), "", &spec_roots);
    std::env::remove_var(PDATACHAINED_ENV);

    let mut prog = match booted {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_pdatachained: skipping {name} (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Drive `load function <name>` → `decompile` → `print C` and return the C.
fn decompile_func(prog: ConsoleProgram, name: &str) -> String {
    let cmds: Vec<String> = [format!("load function {name}"), "decompile".into(), "print C".into()]
        .into_iter()
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

/// The shipped default: the chained chunk at `0x140001020` is not a function, and
/// the primary at `0x140001000` decompiles whole. With the gate off it is a
/// function, and the primary is truncated into the reported `} while ;`.
#[test]
fn chained_chunk_is_not_an_entry_and_the_primary_decompiles_whole() {
    let Some(prog) = boot("pe_chainedunwind_x86_64.exe", None) else { return };
    assert!(
        prog.lookup_symbol("sub_140001000").is_some(),
        "the primary must still be discovered from its own .pdata record"
    );
    assert!(
        prog.lookup_symbol("sub_140001040").is_some(),
        "the entry point must still be discovered"
    );
    assert!(
        prog.lookup_symbol("sub_140001020").is_none(),
        "the chained chunk 0x140001020 must not be claimed as a function"
    );

    let out = decompile_func(prog, "sub_140001000");
    assert!(
        !out.contains("funcboundflow"),
        "the primary must not be truncated at its own chunk, got:\n{out}"
    );
    assert!(!out.contains("while ;"), "expected valid C, got:\n{out}");
    // The chunk's `add eax,7` is inside the recovered body: 0x16 = 1+2+3+4+5+7.
    assert!(out.contains("0x16"), "expected the chunk's arithmetic in the body, got:\n{out}");
}

/// Gate off restores the pre-fix inventory and the pre-fix output exactly.
#[test]
fn gate_off_restores_the_bogus_entry_and_the_invalid_c() {
    let Some(prog) = boot("pe_chainedunwind_x86_64.exe", Some(false)) else { return };
    assert!(
        prog.lookup_symbol("sub_140001020").is_some(),
        "gate off must restore the chained chunk as a function"
    );
    let out = decompile_func(prog, "sub_140001000");
    assert!(out.contains("funcboundflow"), "gate off must still truncate, got:\n{out}");
    assert!(out.contains("while ;"), "gate off must still emit the invalid C, got:\n{out}");
}

/// The same defect where the truncated branch is a loop latch: the decompile of
/// the whole function fails. The default recovers the loop.
#[test]
fn loop_shape_decompiles_at_the_default() {
    let Some(prog) = boot("pe_chainedunwind_loop_x86_64.exe", None) else { return };
    assert!(
        prog.lookup_symbol("sub_140001020").is_none(),
        "the chained chunk must not be claimed as a function"
    );
    let out = decompile_func(prog, "sub_140001000");
    assert!(
        !out.contains("decompilation failed") && !out.contains("LOSS-131"),
        "the loop shape must decompile at the default, got:\n{out}"
    );
    assert!(out.contains("while"), "expected the recovered loop, got:\n{out}");
}

/// A chunk reached by an ordinary fall-through rather than a branch: the
/// truncation is quiet, and the second half of the function simply disappears.
#[test]
fn plain_fallthrough_shape_keeps_its_second_half() {
    let Some(prog) = boot("pe_chainedunwind_plainft_x86_64.exe", None) else { return };
    let out = decompile_func(prog, "sub_140001000");
    assert!(!out.contains("funcboundflow"), "must not be truncated, got:\n{out}");
    // 0x1c = 1+2+3+4+5+6+7, the sum only the whole body computes.
    assert!(out.contains("0x1c"), "expected the whole body, got:\n{out}");
}

/// ARM64 `.pdata` uses the 8-byte `{BeginAddress, UnwindData}` record. Read at
/// the x64 stride of 12 the probe's four functions come back as two; at the right
/// stride all four are discovered. Not gated — the stride is not a judgement call.
#[test]
fn arm64_pdata_stride_discovers_every_record() {
    let Some(prog) = boot("pe_pdata_arm64.exe", None) else { return };
    for name in
        ["sub_140001000", "sub_140001010", "sub_140001020", "sub_140001030"]
    {
        assert!(prog.lookup_symbol(name).is_some(), "{name} missing from the ARM64 inventory");
    }
}
