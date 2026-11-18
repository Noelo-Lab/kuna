//! End-to-end gate for `aif precise` (GH-313): AIF's missing second
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
//! **The bug** (`aif strict`, the `aggressive` preset's setting): `U` opens with the
//! count-20 fingerprint and corroborates nothing, and AIF takes it anyway.
//!
//! **The fix** (`--option aif precise`, the same aligned cursor plus the test): `U`
//! is refused — `20 < 50` and there is no call and no jump into discovered code.
//!
//! The two controls are what keep that assertion from being vacuous. `V` has the
//! SAME count-20 prologue but calls `H1`, discovered code (upstream: "calls always
//! add info"), so it survives both passes: same count, opposite verdict, which is
//! only possible if `precise` tests corroboration rather than raising the count
//! floor. `W` is as uncorroborated as `U` but opens the count-50 fingerprint, and
//! `50 >= 50` keeps it in both passes — the guard is a disjunction, not a ban on
//! self-contained routines.
//!
//! The fourth assertion is the cursor pairing. `U`'s interior at `0x08000392` is
//! itself a count-50 `movs ; adds` prologue whose three remaining instructions still
//! reach `bx lr`, so it would be accepted on the count branch if anything handed the
//! cursor back to `U`'s own body. An uncorroborated candidate is refused as an entry
//! but still CONSUMES its body, and the aligned cursor never probes a 2-mod-4
//! interior byte, so it is not. Without the pairing the guard backfires: on the
//! 3.4 MB PE witness it turned a 361-entry mid-body cut into a 222-entry mid-body
//! rise. The byte cursor, which no `aif` value combines with the test, still runs
//! that pairing against the reference walk in `kuna_analysis`'s `gap_tests`.

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

/// Bootstrap the fixture with the discovery set `kuna functions` / `decompile-all` inject
/// on non-x86-64 (`listing` + `funcstart_patterns` + `aif`), with `aif` at `strict` for
/// the bug pass and `precise` for the fix pass, so the two differ only by the
/// corroboration test — the live-CLI ordering, where the `option` lines precede the
/// deferred `read symbols` commit.
fn bootstrap(aif: &str) -> kuna_console::engine::ConsoleProgram {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str().expect("UTF-8 fixture path").to_string();
    let mut prog = bootstrap_from_object(&bin, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    prog.arch_mut().set_kuna_option("aif", aif).expect("aif takes its value");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

/// Pass 1 — THE BUG (`aif strict`). AIF accepts a routine that corroborates nothing,
/// on four other functions sharing its two-mnemonic prologue.
#[test]
fn default_accepts_an_uncorroborated_routine() {
    let off = bootstrap("strict");
    assert!(
        off.lookup_symbol(U).is_some(),
        "`aif strict` must show the defect: {U} calls nothing, \
         jumps nowhere known and only reaches a `bx lr`, yet a 20-function prologue \
         fingerprint is enough to make it a function"
    );
    assert!(off.lookup_symbol(V).is_some(), "the corroborated routine {V} is always found");
    assert!(off.lookup_symbol(W).is_some(), "the count-50 routine {W} is always found");
}

/// Pass 2 — THE FIX (`aif precise`). Upstream's second fingerprint test is
/// applied, and only the uncorroborated below-threshold candidate loses.
#[test]
fn on_refuses_only_the_uncorroborated_below_threshold_candidate() {
    let on = bootstrap("precise");
    assert!(
        on.lookup_symbol(U).is_none(),
        "`aif precise` must refuse {U}: `startCount` is 20, below the \
         threshold of 50, and the routine adds no information"
    );
    assert!(
        on.lookup_symbol(V).is_some(),
        "{V} has the SAME count-20 prologue as {U} but calls discovered code, so \
         upstream's `addsInfo` holds and it must survive — `precise` tests \
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
/// same `bx lr`, so it WOULD be accepted on the count branch if the cursor reached it.
#[test]
fn a_refusal_does_not_leak_the_cursor_into_the_refused_body() {
    let off = bootstrap("strict");
    assert!(
        off.lookup_symbol(U_INTERIOR).is_none(),
        "under `strict`, {U} is ACCEPTED and its body consumed, so its interior \
         is never probed"
    );
    let on = bootstrap("precise");
    assert!(
        on.lookup_symbol(U_INTERIOR).is_none(),
        "and under `precise`, {U} is REFUSED but still consumes its body: \
         {U_INTERIOR} is a count-50 prologue reaching the same `bx lr`, so a refusal \
         that released the cursor would replace one bad entry with a worse one"
    );
}
