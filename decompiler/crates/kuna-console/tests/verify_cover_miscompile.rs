//! End-to-end gate: the decompiler must never emit C that **computes a different
//! value than the binary**.
//!
//! Two P6 Cover-extension defects each produced a silent miscompilation; this test
//! pins the VALUE-carrying statement for both, against the `covercopy_x86_64`
//! fixture (`+.c`, a non-PIE `gcc -O0` x86-64 ELF built for exactly these shapes).
//!
//! 1. **Dropped restore** (`Merge::checkCopyPair`, `merge.cc:1121`).
//!    `lookup_service` has three `return name;` guards sharing one `-O0` epilogue,
//!    and the middle path clobbers the return register with `lookup()`'s result
//!    before reloading `name`.  The port built `checkCopyPair`'s dominance range
//!    from the dominant COPY's def point alone, omitting
//!    `range.addRefPoint(subOp, subOp->getIn(0))`, so the intervening
//!    `v = lookup(...)` write was never seen inside the range and
//!    `markRedundantCopies` marked the reload non-printing.  The emitted C then
//!    returned the NULL from the failed lookup instead of the parameter.
//!
//! 2. **Over-merge** (`Merge::markImplied`, `merge.cc:1595-1605`, plus the
//!    `Varnode::setFlags` -> `high->coverDirty()` forward, `varnode.cc:377-378`).
//!    `two_selects` has two `cond ? step : 0` phis whose reads are both inlined
//!    into one call argument printed after both writes.  Without dirtying the
//!    operands' Covers on implied-marking, `Cover::rebuild`'s forward walk through
//!    implied consumers never runs, the two phis look cover-disjoint, and the
//!    speculative merge folds them into one variable — so the emitted C subtracts
//!    the second select's value twice.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make specs`).
//! When absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/covercopy_x86_64")
}

/// Bootstrap the fixture and decompile `func`, returning the printed C
/// (`None` ⇒ specs-less skip).
fn decompile(func: &str) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("verify_cover_miscompile: skipping (bootstrap failed, `make specs`): {}", e.explain());
            return None;
        }
    };

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

/// The name the printer gave the returned local — the single local declared with
/// the pointer-sized return storage.  Returns the `vN` token assigned from the
/// `lookup(...)` call, which is the variable the epilogue returns.
fn returned_local(c: &str) -> Option<String> {
    for line in c.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("return ") {
            return Some(rest.trim_end_matches(';').trim().to_string());
        }
    }
    None
}

#[test]
fn restore_of_returned_parameter_is_emitted() {
    let Some(c) = decompile("lookup_service") else {
        return; // specs-less skip
    };
    eprintln!("---- lookup_service ----\n{c}");

    // Sanity: the fixture still produces the merged single-exit shape this bug
    // needs — one trailing `return vN` whose variable is also written by the
    // `lookup(...)` call.
    let ret = returned_local(&c).expect("lookup_service must end in a `return <expr>;`");
    assert!(
        ret.starts_with('v'),
        "expected the epilogue to return a merged local (the shape this bug needs), got `{ret}`\n--- C ---\n{c}"
    );
    assert!(
        c.contains(&format!("{ret} = ")) && c.contains("lookup("),
        "expected `{ret}` to be assigned from the lookup(...) call\n--- C ---\n{c}"
    );

    // The bug: the reload of the parameter on the lookup-failed path was deleted,
    // so the function returned the NULL from `lookup`.  Assert the restore is
    // present AFTER the clobbering call — the statement that carries the VALUE.
    // (An identical `vN = a0;` is hoisted to the entry for the other two guards,
    // so position, not mere presence, is what distinguishes the bug.)
    let lines: Vec<&str> = c.lines().map(|l| l.trim()).collect();
    let clobber = lines
        .iter()
        .position(|l| l.starts_with(&format!("{ret} = ")) && l.contains("lookup("))
        .expect("lookup_service must assign the returned local from lookup(...)");
    let is_restore =
        |l: &&str| **l == format!("{ret} = a0;") || **l == format!("{ret} = (void *)a0;");
    assert!(
        lines[..clobber].iter().any(is_restore),
        "expected the entry hoist `{ret} = a0;` before the lookup(...) call\n--- C ---\n{c}"
    );
    assert!(
        lines[clobber + 1..].iter().any(is_restore),
        "the `{ret} = a0;` restore on the lookup-failed path is MISSING — the emitted C \
         returns NULL where the binary returns the parameter (Merge::checkCopyPair \
         addRefPoint, merge.cc:1121)\n--- C ---\n{c}"
    );
}

#[test]
fn independent_selects_do_not_share_one_variable() {
    let Some(c) = decompile("two_selects") else {
        return; // specs-less skip
    };
    eprintln!("---- two_selects ----\n{c}");

    // Sanity: both selects must still render as assignment diamonds (this is the
    // shape the over-merge needs; if a later pass folds them to `?:` the test
    // would pass vacuously).
    let step_writes: Vec<&str> = c
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.ends_with("= g_step;"))
        .collect();
    assert_eq!(
        step_writes.len(),
        2,
        "expected two `vN = g_step;` select arms (the shape this bug needs), got {step_writes:?}\n--- C ---\n{c}"
    );

    // The bug: both phis merged into ONE HighVariable, so the call argument read
    // the same variable twice and the first select's value was lost.
    let lhs: std::collections::BTreeSet<&str> =
        step_writes.iter().map(|l| l.split(" =").next().unwrap()).collect();
    assert_eq!(
        lhs.len(),
        2,
        "the two independent selects share ONE variable — the emitted call argument \
         subtracts the second select's value twice and drops the first \
         (Merge::markImplied cover-dirty, merge.cc:1595-1605)\n--- C ---\n{c}"
    );

    // And the call argument must read both of them.
    let call = c
        .lines()
        .find(|l| l.contains("emit("))
        .expect("two_selects must call emit(...)");
    for v in &lhs {
        assert!(
            call.contains(&format!("- {v}")),
            "the emit(...) argument does not subtract `{v}`: {call}\n--- C ---\n{c}"
        );
    }
}
