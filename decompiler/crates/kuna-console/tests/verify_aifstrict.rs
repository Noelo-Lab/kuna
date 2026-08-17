//! End-to-end gate for `aifstrict` (GH-299): the AIF gap cursor's aligned slide.
//!
//! Fixture: the vendored `cortexm_aifstrict_le32` (`+.py` builder), a stripped
//! bare-metal Cortex-M image whose `.text` carries the defect in its minimal form:
//!
//! ```text
//!   0x08000140  A      push {r7,lr} ; ldr r0,[pc,#4] ; movs r1,#1 ; pop {r7,pc}
//!   0x08000148  POOL   .word 0x20001000          <- A's literal (SRAM constant)
//!   0x0800014c  B      movs r1,#6 ; movs r2,#7 ; movs r3,#8 ; bx lr
//! ```
//!
//! `A` is reached by a direct `BL` from the reset vector, so the Listing walk
//! decodes it and stops at its `pop {r7,pc}`. `POOL` is data, so it is an undefined
//! hole, and `B` is called by nobody — the hole runs from `POOL` straight through
//! `B`'s body. Twenty `movs ; movs` helpers clear AIF's 20-function floor and stock
//! its fingerprint histogram past the acceptance threshold of 4.
//!
//! **The bug** (default, `aifstrict off`): the cursor slides one byte at a time, so
//! after rejecting `POOL` it probes `POOL + 2`. The pool word `0x20001000` is
//! little-endian, so the halfword at `POOL + 2` is `0x2000` — `movs r0,#0` — and the
//! next halfword is `B`'s own `movs r1,#6`. That synthetic `movs ; movs` pair is
//! byte-for-byte the helpers' fingerprint, so AIF accepts `0x0800014a`: an entry one
//! halfword *inside* a literal pool, whose body is `B` with a dead `movs r0,#0`
//! glued to its front. On accept the cursor jumps past the accepted body, so the
//! real function at `0x0800014c` is never probed at all — the false entry does not
//! merely accompany the true one, it *replaces* it.
//!
//! **The fix** (`--option aifstrict on`, which `--mode aggressive` carries):
//! `0x0800014a` is 2-mod-4 and is not the first byte of its hole, so the aligned
//! slide never probes it. The cursor advances from `POOL` to
//! `POOL + 4`, which is `B`'s real entry, and accepts that instead. One phantom
//! removed, one real function recovered, from the same walk.
//!
//! The second shape in the fixture (`C` / `POOL2` / `D`) is the control that keeps
//! the assertion honest: `D` opens `movs ; adds`, a fingerprint no discovered
//! function shares, so the aligned probe at `POOL2 + 4` is *rejected*. The option
//! must not invent an entry there — it only stops probing addresses that cannot be
//! instruction boundaries; it never lowers the acceptance bar.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built ARM `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_aifstrict_le32")
}

/// `A`, the function whose `ldr r0,[pc,#4]` defines the pool.
const A: &str = "sub_8000140";
/// THE PHANTOM: the byte-slide's accept one halfword into the pool.
const PHANTOM: &str = "sub_800014a";
/// `B`, the real function the pool precedes.
const B: &str = "sub_800014c";
/// The second pool's phantom (same shape, unshared successor fingerprint).
const PHANTOM2: &str = "sub_800015e";
/// `D`, the second pool's successor — `movs ; adds`, a fingerprint no discovered
/// function shares, so AIF must accept it in NEITHER pass.
const D: &str = "sub_8000160";

/// Bootstrap the fixture with the discovery set `kuna functions` / `decompile-all`
/// inject on non-x86-64 (`listing` + `funcstart_patterns` + `aif`), flipping
/// `aifstrict` on for the fix pass — the live-CLI ordering, where the `option` lines
/// precede the deferred `read symbols` commit. `None` ⇒ specs-less skip.
fn bootstrap(strict: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_aifstrict: skipping (bootstrap failed, build `.sla` with \
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
    if strict {
        prog.arch_mut().set_kuna_option("aifstrict", "on").expect("aifstrict flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Pass 1 — THE BUG (the default). With the byte-granular cursor, AIF plants an
/// entry one halfword inside the literal pool and the real function behind it is
/// lost.
#[test]
fn default_plants_a_phantom_inside_the_literal_pool() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(off.lookup_symbol(A).is_some(), "the pool's own loader is always found");
    assert!(
        off.lookup_symbol(PHANTOM).is_some(),
        "the default (`aifstrict off`) must show the defect: the byte-slide probes \
         {PHANTOM}, one halfword into the pool, where the pool word's high halfword \
         decodes as \
         `movs r0,#0` and completes the helpers' `movs ; movs` fingerprint"
    );
    assert!(
        off.lookup_symbol(B).is_none(),
        "and the accept must consume the real function {B}: on accept the cursor \
         jumps past the accepted body, so {B}'s own entry is never probed"
    );
    assert!(
        off.lookup_symbol(PHANTOM2).is_some(),
        "the second pool shows the same defect at {PHANTOM2}"
    );
}

/// Pass 2 — THE FIX (`aifstrict on`). The aligned slide never probes the 2-mod-4
/// interior byte, so the phantom is gone and the cursor's next candidate is the real
/// entry.
#[test]
fn on_slides_past_the_pool_and_finds_the_real_function() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(on.lookup_symbol(A).is_some(), "the pool's own loader is always found");
    assert!(
        on.lookup_symbol(PHANTOM).is_none(),
        "`aifstrict on` must not probe {PHANTOM}: it is 2-mod-4 and is not the first \
         byte of its hole, so it cannot be an instruction boundary the walk stopped at"
    );
    assert!(
        on.lookup_symbol(B).is_some(),
        "and the cursor's next aligned candidate IS the real function {B}, which the \
         byte-slide's premature accept had consumed"
    );
    assert!(
        on.lookup_symbol(PHANTOM2).is_none(),
        "the second pool's phantom {PHANTOM2} is suppressed by the same slide"
    );
}

/// The acceptance bar is untouched: the option only declines to *probe* addresses
/// that cannot be instruction boundaries. `D` sits at an aligned pool end but opens
/// with a fingerprint no discovered function shares, so it is accepted in neither
/// pass — a recovered `B` is evidence the fingerprint gate still ran, not evidence
/// it was relaxed.
#[test]
fn the_acceptance_bar_is_unchanged() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(off.lookup_symbol(D).is_none(), "{D}'s fingerprint is unshared (bug pass)");
    let Some(on) = bootstrap(true) else { return };
    assert!(
        on.lookup_symbol(D).is_none(),
        "{D} sits at an aligned pool end but its `movs ; adds` prologue matches no \
         discovered function, so the aligned slide must NOT accept it"
    );
}

/// The option is inert without `aif`: with the gap-walk off, both passes produce
/// exactly the same discovery set.
#[test]
fn aifstrict_is_inert_without_aif() {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str().unwrap().to_string();
    let mut sets = Vec::new();
    for strict in [false, true] {
        let Ok(mut prog) = bootstrap_from_object(&bin, "", &spec_roots) else {
            return; // specs-less skip
        };
        prog.arch_mut().set_kuna_option("listing", "on").unwrap();
        prog.arch_mut().set_kuna_option("funcstart_patterns", "on").unwrap();
        if strict {
            prog.arch_mut().set_kuna_option("aifstrict", "on").unwrap();
        }
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        sets.push([A, PHANTOM, B, PHANTOM2, D].map(|s| prog.lookup_symbol(s).is_some()));
    }
    assert_eq!(sets[0], sets[1], "with `aif` off, `aifstrict` must change nothing");
}
