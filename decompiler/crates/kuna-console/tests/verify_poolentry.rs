//! End-to-end gate for `poolentry` — ARM PC-relative literal-pool inference
//! (P1 code/data partition, ARM discovery).
//!
//! ## Why this is a Rust gate and not a `tests/stages/*.xml` two-pass case
//!
//! The pass keys on the **Listing tier** and on AIF's gap walk over it, and both
//! are built only from a real `ObjectLoadImage`. The datatest/stages harness embeds
//! raw `<bytechunk>`s with explicit `<symbol>` definitions and never constructs a
//! load image (`tests/stages/kuna-listing-flag.xml` says so in its own comment), so
//! the option is structurally unobservable there — `fdeinterior`/DIV-61 shipped an
//! entry-*removing* default-ON flip at 675/675 PARITY OK with no per-test opt-out
//! for the same reason. Every sibling analysis-tier discovery flag
//! (`funcstart_patterns`, `aif`, `eh_frame_full`, `cortexmvectors`, `ptrentry`,
//! `tailcallentry`) is gated by a cross-crate `verify_*` test over a real object.
//! This file is the two-pass case: the same image, option off (the bug) then option
//! on (the fix).
//!
//! ## Fixture
//!
//! `cortexm_poolentry_le32` (built by the committed `cortexm_poolentry_le32.py`,
//! see the fixtures README): a stripped bare-metal Cortex-M image whose vector
//! table the **shipped** oracle already confirms, so nothing here depends on
//! `cortexmvectors`. Twenty `movs ; movs ; movs ; bx lr` helpers, all `BL`-reached
//! from the reset vector, clear AIF's two floors at once — its 20-discovered-
//! function minimum and its 4-occurrence prologue-fingerprint threshold. On top of
//! that the image carries the three shapes this option must tell apart:
//!
//!  - **PHANTOM** (`A` @ `0x08000140` → `POOL1` @ `0x08000148` → `B` @
//!    `0x0800014c`). `A` loads the pool word `0x20001000`, the STM32 SRAM constant
//!    of the real defect: its HIGH halfword `0x2000` decodes as `movs r0,#0`, which
//!    matches the helpers' fingerprint, so AIF's byte-granular cursor accepts
//!    `0x0800014a` — one halfword before `B` — and then jumps past `B`'s body, so
//!    `B` itself is never probed. `B` never writes `r0`, so the phantom's dead
//!    leading `movs r0,#0` is visible in the C: `sub_800014a` returns a fabricated
//!    constant where the true entry returns nothing.
//!  - **UNPAIRED** (`C` → `POOL2` @ `0x0800015c` → `D` @ `0x08000160`). The same
//!    shape, except `D` opens `movs ; adds` — a fingerprint no discovered function
//!    shares — so the additive half refuses to emit an entry at the pool end. With
//!    no replacement the pairing invariant forbids the removal and the phantom at
//!    `0x0800015e` is KEPT.
//!  - **SPLIT** (`G` @ `0x08000168` → `F` @ `0x08000170`). `G`'s literal reference
//!    resolves onto `F`'s own FIRST WORD, which the Listing never decoded, so the
//!    inference reads those four bytes as a pool and moves the entry four bytes
//!    into a real function.
//!
//! ## The two passes
//!
//! **Off (default)** — AIF's accept list is exactly what it is today: `sub_800014a`
//! and `sub_800015e` (both phantoms, both one halfword below a real function) and
//! `sub_8000170` (`F`, correct). `B` produces no C at any address of its own.
//!
//! **On** — `sub_800014a` is replaced by `sub_800014c`, and `sub_800015e` survives
//! untouched because nothing replaces it.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_poolentry_le32")
}

/// The reset vector's function — `push {r7,lr} ; bl ×23 ; pop {r7,pc}`.
const RESET: &str = "sub_8000040";
/// `A`, the function whose `ldr r0,[pc,#4]` defines `POOL1`.
const A: &str = "sub_8000140";
/// The PHANTOM: AIF's accept one halfword into `POOL1`, decoding the pool word's
/// high halfword as a dead `movs r0,#0` and falling through into `B`.
const PHANTOM: &str = "sub_800014a";
/// `B`, the real function `POOL1` precedes — reachable at its own address only
/// once the pool is known.
const B: &str = "sub_800014c";
/// The UNPAIRED phantom: the same shape at `POOL2`, whose pool end (`D`) carries
/// no replacement entry, so the removal is forbidden.
const UNPAIRED: &str = "sub_800015e";
/// `F`, whose own first word a literal reference lands on — the disclosed split.
const F: &str = "sub_8000170";
/// Four bytes into `F`: where the additive half plants its entry because the
/// Listing never decoded `F`'s first word.
const F_PLUS_4: &str = "sub_8000174";

/// Bootstrap the fixture with the discovery set `decompile-all` / `kuna functions`
/// inject on non-x86-64 plus `aif` (the pass `poolentry` reads), optionally
/// flipping `poolentry` on before the deferred commit — the live-CLI ordering.
/// `None` ⇒ specs-less skip.
fn bootstrap(on: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_poolentry: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    prog.arch_mut().set_kuna_option("aif", "on").expect("aif flips on");
    if on {
        prog.arch_mut().set_kuna_option("poolentry", "on").expect("poolentry flips on");
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

/// Pass 1 — THE BUG. AIF's cursor is byte-granular with no instruction-alignment
/// filter, so it probes the interior of a literal pool and accepts the halfword
/// that decodes as `movs r0,#0`. The real function one halfword later is never
/// probed, and the entry that exists in its place carries a dead instruction the
/// decompiler then folds through the body.
#[test]
fn default_plants_the_entry_inside_the_literal_pool() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(off.lookup_symbol(RESET).is_some(), "the reset vector is always found");
    assert!(off.lookup_symbol(A).is_some(), "the walk always follows the BL to A");
    assert!(
        off.lookup_symbol(PHANTOM).is_some(),
        "default (poolentry off) must show the defect: AIF accepts {PHANTOM}, one \
         halfword into POOL1, because the pool word's high halfword decodes as \
         `movs r0,#0` and matches the helpers' prologue fingerprint"
    );
    assert!(
        off.lookup_symbol(B).is_none(),
        "and the real function {B} is never probed, because AIF advances past the \
         body it just claimed"
    );
    // The user-visible form of the bug: the dead leading `movs r0,#0` becomes a
    // fabricated return value.
    let body = decompile(off, PHANTOM);
    eprintln!("---- {PHANTOM} (poolentry off / default) ----\n{body}");
    assert!(
        body.contains("return 0x600000000"),
        "the phantom must fold its dead `movs r0,#0` into the return value, got:\n{body}"
    );
}

/// Pass 2 — THE FIX. The pool is recovered from the literal reference that already
/// exists, the entry is emitted at the pool end, and the phantom is dropped — a
/// MOVE, not a delete.
#[test]
fn poolentry_moves_the_entry_to_the_pool_end() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(
        on.lookup_symbol(B).is_some(),
        "poolentry must emit an entry at the end of POOL1, which is {B}"
    );
    assert!(
        on.lookup_symbol(PHANTOM).is_none(),
        "and must drop the {PHANTOM} accept, since the pool it sits in now carries \
         a replacement entry at its end"
    );
    let Some(prog) = bootstrap(true) else { return };
    let body = decompile(prog, B);
    eprintln!("---- {B} (poolentry on) ----\n{body}");
    assert!(
        !body.contains("return 0x600000000"),
        "{B} must not carry the phantom's fabricated return value, got:\n{body}"
    );
}

/// THE PAIRING INVARIANT, pinned. A phantom whose pool end carries no replacement
/// entry must be KEPT: without this clause the same predicate removes 5,069 AIF
/// accepts corpus-wide and 531 real functions stop being decompiled at any address
/// at all, which the entry-address bar scores as zero harm. Here `D` opens
/// `movs ; adds`, a fingerprint no discovered function shares, so the additive half
/// declines the pool end and the removal is forbidden.
#[test]
fn poolentry_keeps_an_unpaired_phantom() {
    for on in [false, true] {
        let Some(prog) = bootstrap(on) else {
            return; // specs-less skip
        };
        assert!(
            prog.lookup_symbol(UNPAIRED).is_some(),
            "poolentry (on = {on}) must KEEP {UNPAIRED}: nothing replaces it at the \
             end of POOL2, so dropping it would delete the only entry from which \
             D's body is decompiled"
        );
    }
}

/// THE DISCLOSED RESIDUE, pinned as current behaviour rather than hidden. `G`'s
/// literal reference resolves onto `F`'s own first word — which the Listing never
/// decoded, so the inference cannot tell it from a pool word — and the entry moves
/// four bytes into a real function, losing its first two instructions.
///
/// This is the single split in the 98-image / 48,553-address corpus measurement
/// (betaflight O2-noinline, `SCSI_RequestSense` + 4). It is disclosed rather than
/// gated away because the only guard that removes it — refusing to emit at a known
/// branch target — costs 108 of the 189 recovered ground-truth entries. If a future
/// change closes it, this test is the one to update, and the corpus split count
/// should go from 1 to 0.
#[test]
fn poolentry_split_residue_is_pinned() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(off.lookup_symbol(F).is_some(), "AIF finds F at its real entry today");
    assert!(off.lookup_symbol(F_PLUS_4).is_none());
    let body = decompile(off, F);
    eprintln!("---- {F} (poolentry off / default) ----\n{body}");
    assert!(
        body.contains("return 0x800000007"),
        "F's real body loads r0 = 7 and r1 = 8, got:\n{body}"
    );

    let Some(on) = bootstrap(true) else { return };
    assert!(
        on.lookup_symbol(F_PLUS_4).is_some(),
        "KNOWN RESIDUE: the entry moves to {F_PLUS_4}, four bytes into F"
    );
    assert!(on.lookup_symbol(F).is_none(), "KNOWN RESIDUE: F's own entry is dropped");
    let split = decompile(on, F_PLUS_4);
    eprintln!("---- {F_PLUS_4} (poolentry on / the disclosed split) ----\n{split}");
    assert!(
        !split.contains("return 0x800000007"),
        "the split entry has lost F's first two instructions, which is the harm \
         being disclosed, got:\n{split}"
    );
}

/// `poolentry` is computed inside the AIF block, so with `aif` off it is a
/// structural no-op: nothing is added and nothing is removed. Pinned because the
/// suppression half is the only entry-*removing* thing in the ARM entry sequence.
#[test]
fn poolentry_is_inert_without_aif() {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let Some(bin) = fixture().to_str().map(str::to_string) else { return };
    let mut sets = Vec::new();
    for on in [false, true] {
        let Ok(mut prog) = bootstrap_from_object(&bin, "", &spec_roots) else {
            return; // specs-less skip
        };
        prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
        prog.arch_mut()
            .set_kuna_option("funcstart_patterns", "on")
            .expect("funcstart_patterns flips on");
        if on {
            prog.arch_mut().set_kuna_option("poolentry", "on").expect("poolentry flips on");
        }
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        sets.push([PHANTOM, B, UNPAIRED, F, F_PLUS_4].map(|s| prog.lookup_symbol(s).is_some()));
    }
    assert_eq!(sets[0], sets[1], "with `aif` off, poolentry must change nothing");
    assert_eq!(
        sets[0],
        [false, false, false, false, false],
        "and with `aif` off none of the gap-discovered entries exist at all"
    );
}
