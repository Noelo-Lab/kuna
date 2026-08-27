//! End-to-end gate for `aifcorroborate` (GH-313): AIF's missing second
//! fingerprint test.
//!
//! Upstream Ghidra rejects a gap candidate on TWO tests, not one:
//!
//! ```text
//! AggressiveInstructionFinderAnalyzer.java:278   if (startCount < 4) { continue; }
//! AggressiveInstructionFinderAnalyzer.java:367   if (numInstr <= 2 || (!addsInfo && startCount < 50)) { continue; }
//! ```
//!
//! kuna ported the first and the `numInstr` half of the second, so a routine that
//! calls nothing, jumps nowhere known and merely reaches a `bx lr` becomes a
//! function on four other functions sharing its two-mnemonic prologue.
//!
//! Fixture: the vendored `cortexm_aifcorroborate_le32` (`+.py` builder), a stripped
//! bare-metal Cortex-M image whose `.text` tail is three shapes in an undefined gap,
//! against a histogram stocked with two deliberately different counts — `movs ; movs`
//! shared by 20 discovered functions, `movs ; adds` shared by exactly 50:
//!
//! ```text
//!   0x08000390  U   movs r0,#1 ; movs r1,#2 ; adds r2,#3 ; adds r3,#4 ; bx lr
//!   0x0800039c  V   movs r0,#5 ; movs r1,#6 ; bl H1 ; bx lr
//!   0x080003a8  W   movs r0,#7 ; adds r1,#8 ; adds r2,#9 ; bx lr
//! ```
//!
//! **The bug** (default, `aifcorroborate off`): `U` opens with the count-20
//! fingerprint and corroborates nothing, and AIF takes it anyway.
//!
//! **The fix** (`--option aifcorroborate on`): `U` is refused — `20 < 50` and there
//! is no call and no jump into discovered code.
//!
//! The two controls are what keep that assertion from being vacuous. `V` has the
//! SAME count-20 prologue but calls `H1`, discovered code (upstream: "calls always
//! add info"), so it survives both passes: same count, opposite verdict, which is
//! only possible if the option tests corroboration rather than raising the count
//! floor. `W` is as uncorroborated as `U` but opens the count-50 fingerprint, and
//! `50 >= 50` keeps it in both passes — the guard is a disjunction, not a ban on
//! self-contained routines.
//!
//! The fourth assertion is the cursor pairing. `U`'s interior at `0x08000392` is
//! itself a count-50 `movs ; adds` prologue whose three remaining instructions still
//! reach `bx lr`, so it would be accepted on the count branch if refusing `U` handed
//! the cursor back to `U`'s own body. An uncorroborated candidate is refused as an
//! entry but still CONSUMES its body, so it does not. Without that pairing the guard
//! backfires: on the 3.4 MB PE witness it turned a 361-entry mid-body cut into a
//! 222-entry mid-body rise.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_aifcorroborate_le32")
}

/// THE DEFECT: uncorroborated, `startCount == 20`.
const U: &str = "sub_8000390";
/// One halfword inside `U`, itself a `startCount == 50` prologue — the address the
/// cursor must never fall back onto after refusing `U`.
const U_INTERIOR: &str = "sub_8000392";
/// THE CORROBORATED CONTROL: the same count-20 prologue, but it calls `H1`.
const V: &str = "sub_800039c";
/// THE COUNT CONTROL: uncorroborated like `U`, but `startCount == 50`.
const W: &str = "sub_80003a8";

/// Bootstrap the fixture with the discovery set `kuna functions` / `decompile-all`
/// inject on non-x86-64 (`listing` + `funcstart_patterns` + `aif`), flipping
/// `aifcorroborate` on for the fix pass — the live-CLI ordering, where the `option`
/// lines precede the deferred `read symbols` commit. `None` ⇒ specs-less skip.
///
/// `aifstrict` is left at its default (off) so the byte-granular cursor is live:
/// that is what makes the `U_INTERIOR` probe reachable at all, and therefore what
/// makes the cursor-pairing assertion mean something.
fn bootstrap(corroborate: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_aifcorroborate: skipping (bootstrap failed, build `.sla` with \
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
    if corroborate {
        prog.arch_mut()
            .set_kuna_option("aifcorroborate", "on")
            .expect("aifcorroborate flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Pass 1 — THE BUG (the default). AIF accepts a routine that corroborates nothing,
/// on four other functions sharing its two-mnemonic prologue.
#[test]
fn default_accepts_an_uncorroborated_routine() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(
        off.lookup_symbol(U).is_some(),
        "the default (`aifcorroborate off`) must show the defect: {U} calls nothing, \
         jumps nowhere known and only reaches a `bx lr`, yet a 20-function prologue \
         fingerprint is enough to make it a function"
    );
    assert!(off.lookup_symbol(V).is_some(), "the corroborated routine {V} is always found");
    assert!(off.lookup_symbol(W).is_some(), "the count-50 routine {W} is always found");
}

/// Pass 2 — THE FIX (`aifcorroborate on`). Upstream's second fingerprint test is
/// applied, and only the uncorroborated below-threshold candidate loses.
#[test]
fn on_refuses_only_the_uncorroborated_below_threshold_candidate() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(
        on.lookup_symbol(U).is_none(),
        "`aifcorroborate on` must refuse {U}: `startCount` is 20, below the \
         threshold of 50, and the routine adds no information"
    );
    assert!(
        on.lookup_symbol(V).is_some(),
        "{V} has the SAME count-20 prologue as {U} but calls discovered code, so \
         upstream's `addsInfo` holds and it must survive — the option tests \
         corroboration, it does not raise the fingerprint floor"
    );
    assert!(
        on.lookup_symbol(W).is_some(),
        "{W} corroborates nothing either, but its prologue is shared by 50 \
         discovered functions, so the disjunction's second arm keeps it"
    );
}

/// The cursor pairing: refusing a candidate must not hand the cursor back to that
/// candidate's own interior. `U_INTERIOR` is a count-50 prologue that reaches the
/// same `bx lr`, so it WOULD be accepted on the count branch if the refusal leaked.
#[test]
fn a_refusal_does_not_leak_the_cursor_into_the_refused_body() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(
        off.lookup_symbol(U_INTERIOR).is_none(),
        "with the option off, {U} is ACCEPTED and its body consumed, so its interior \
         is never probed"
    );
    let Some(on) = bootstrap(true) else { return };
    assert!(
        on.lookup_symbol(U_INTERIOR).is_none(),
        "and with the option on, {U} is REFUSED but still consumes its body: \
         {U_INTERIOR} is a count-50 prologue reaching the same `bx lr`, so a refusal \
         that released the cursor would replace one bad entry with a worse one"
    );
}

/// The option is inert without `aif`: with the gap-walk off, both passes produce
/// exactly the same discovery set.
#[test]
fn aifcorroborate_is_inert_without_aif() {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str().unwrap().to_string();
    let mut sets = Vec::new();
    for corroborate in [false, true] {
        let Ok(mut prog) = bootstrap_from_object(&bin, "", &spec_roots) else {
            return; // specs-less skip
        };
        prog.arch_mut().set_kuna_option("listing", "on").unwrap();
        prog.arch_mut().set_kuna_option("funcstart_patterns", "on").unwrap();
        if corroborate {
            prog.arch_mut().set_kuna_option("aifcorroborate", "on").unwrap();
        }
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        sets.push([U, U_INTERIOR, V, W].map(|s| prog.lookup_symbol(s).is_some()));
    }
    assert_eq!(sets[0], sets[1], "with `aif` off, `aifcorroborate` must change nothing");
}
