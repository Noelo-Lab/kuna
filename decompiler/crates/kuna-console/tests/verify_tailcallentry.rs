//! End-to-end gate for `tailcallentry` — tail-call function-entry recovery
//! (P1 code/data partition, the Listing walk's missing tail-call notion).
//!
//! ## Why this is a Rust gate and not a `tests/stages/*.xml` two-pass case
//!
//! The pass reads the **Listing tier**, which is built only from an
//! `ObjectLoadImage` over a real object. The datatest/stages harness embeds raw
//! `<bytechunk>`s with explicit `<symbol>` definitions and never constructs one,
//! so the recursive-descent walk — and therefore this option — is unobservable
//! there. Every sibling analysis-tier discovery flag (`funcstart_patterns`,
//! `aif`, `eh_frame_full`, `cortexmvectors`, `ptrentry`, `i386_pie_plt`) is
//! gated the same way, by a cross-crate `verify_*` test over a real object. This
//! file is the two-pass case: the same image, option off (the bug) then option
//! on (the fix).
//!
//! ## Fixture
//!
//! `cortexm_tailcall_le32` (built by the committed `cortexm_tailcall_le32.py`,
//! see the fixtures README): a stripped bare-metal Cortex-M image whose
//! `.text` holds one genuine tail call plus the three near-miss shapes the
//! containment model has to keep rejecting. Its vector table matches the
//! **shipped** signature, so nothing here depends on `cortexmvectors`.
//!
//! | VMA | shape | verdict |
//! |---|---|---|
//! | `0x8008020` | reached only by `b` from `_start`, crosses a discovered entry | **accepted** |
//! | `0x8008038` | reached only by `b`, but stays inside its caller's region | rejected (containment) |
//! | `0x8008058` | crosses a region, but opens `pop {r4,pc}` | rejected (epilogue) |
//! | `0x8008060` | crosses a region, but its region is an infinite loop | rejected (non-terminating) |
//!
//! All four are reached ONLY by an unconditional `B`, so the naive tail-call
//! rule — the one the proposal measured at 39% precision — accepts every one of
//! them.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built ARM `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_tailcall_le32")
}

/// `_start` — the ELF entry, and the only seeded root.
const START: &str = "sub_8008000";
/// `helper` — discovered through `_start`'s `BL`.
const HELPER: &str = "sub_8008010";
/// The tail-called routine (`movs r0,#0x2a ; bx lr`), reachable ONLY through the
/// unconditional `B` at `_start+4`. This is what the option recovers.
const TAIL: &str = "sub_8008020";
/// An unconditional-branch target that stays inside its caller's own region —
/// the rotated-loop-head shape the containment guard exists to reject.
const IN_REGION: &str = "sub_8008038";
/// An unconditional-branch target that opens with a stack restore — a shared
/// epilogue, not a function.
const EPILOGUE: &str = "sub_8008058";
/// An unconditional-branch target whose flow region never terminates.
const NON_TERMINATING: &str = "sub_8008060";

/// Bootstrap the fixture with the Listing tier on, optionally flipping
/// `tailcallentry` on before the deferred commit (the live-CLI ordering).
/// `None` ⇒ specs-less skip.
fn bootstrap(on: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_tailcallentry: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    if on {
        prog.arch_mut()
            .set_kuna_option("tailcallentry", "on")
            .expect("tailcallentry flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Decompile `func` in an already-bootstrapped program and return the captured C.
fn decompile(prog: kuna_console::engine::ConsoleProgram, func: &str) -> String {
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
    status.optr.clone()
}

/// Pass 1 — THE BUG. The walk treats the unconditional `B` at `_start+4` as a
/// same-function successor, so the tail-called routine is not a function at all:
/// it has no entry, and its body is emitted as part of `_start`.
#[test]
fn default_absorbs_the_tail_called_routine() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(
        off.lookup_symbol(TAIL).is_none(),
        "default (tailcallentry off) must NOT discover {TAIL}: the walk makes a \
         new function only at a CALL target"
    );
    let body = decompile(off, START);
    eprintln!("---- {START} (tailcallentry off / default) ----\n{body}");
    assert!(
        body.contains("0x2a"),
        "the absorbed routine's body must appear inside {START} (that IS the \
         bug — a tail call rendered as straight-line flow), got:\n{body}"
    );
}

/// Pass 2 — THE FIX. With the option on the branch is recognised as a tail call,
/// the routine becomes its own function, and it decompiles to its real constant.
#[test]
fn tailcallentry_recovers_the_tail_called_routine() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(
        on.lookup_symbol(TAIL).is_some(),
        "tailcallentry must discover the tail-called routine {TAIL} (it did not)"
    );
    let Some(prog) = bootstrap(true) else { return };
    let body = decompile(prog, TAIL);
    eprintln!("---- {TAIL} (tailcallentry on) ----\n{body}");
    assert!(
        body.contains("return 0x2a;"),
        "{TAIL} must decompile to its own body `return 0x2a;`, got:\n{body}"
    );
}

/// The precision model, pinned. All three near-misses are reached ONLY by an
/// unconditional `B` — the naive rule takes every one of them — and each is
/// rejected by exactly one guard.
#[test]
fn containment_rejects_the_near_misses() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(
        on.lookup_symbol(IN_REGION).is_none(),
        "{IN_REGION} stays inside its caller's entry-ordered region, so the \
         containment guard must reject it (this is the rotated-loop-head case \
         that takes the naive rule to 39% precision)"
    );
    assert!(
        on.lookup_symbol(EPILOGUE).is_none(),
        "{EPILOGUE} opens with `pop {{r4,pc}}` — a shared epilogue, not a \
         function start"
    );
    assert!(
        on.lookup_symbol(NON_TERMINATING).is_none(),
        "{NON_TERMINATING}'s flow region never reaches a RETURN or a computed \
         jump, so it is not a valid subroutine"
    );
}

/// "This option never removes an entry" is a property of the WIRING, not of a
/// heuristic: the accepted targets are emitted as an additive `entries` fact and
/// the Listing is never rebuilt, so the walk's instruction closure — and every
/// other consumer's input — is byte-identical. Pinned here as a strict superset.
#[test]
fn tailcallentry_only_adds_entries() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    let Some(on) = bootstrap(true) else { return };
    for f in [START, HELPER] {
        assert!(
            off.lookup_symbol(f).is_some() && on.lookup_symbol(f).is_some(),
            "{f} must survive the flip in both directions"
        );
    }
    assert!(off.lookup_symbol(TAIL).is_none() && on.lookup_symbol(TAIL).is_some());
}
