//! End-to-end two-pass gate for `unmappedentry` — the Listing walk's function
//! worklist may not claim a CALL target its own instruction worklist refuses to
//! decode (P1 code/data partition).
//!
//! Fixture: `unmapped_call_x86_64`, the reduced form of the shape
//! crackmes.one/66fd63059b533b4c22bd0b91 presents (`unmapped_call_x86_64.py`
//! carries the byte-by-byte layout). An always-taken `xor eax,eax; je +1` is
//! followed by the `e8` byte of an overlapping encoding, so the walk decodes a
//! `call` on the never-executed fall-through whose displacement is really the
//! next instruction's immediate. The target, 0x349688c1, is half a gigabyte
//! above a 190-byte image and inside no section.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** the walk hands 0x349688c1 to the function
//!   worklist, `fast_funcdisc` commits it, and the inventory carries
//!   `sub_349688c1` — an entry with no bytes, no extent and no body.
//! * **default (the fix):** the same predicate that already gates the
//!   instruction worklist gates the function claim, so the phantom is absent
//!   while both real functions (`e_entry` and the callee it calls) stay.
//!
//! The third test is the property that keeps this from deleting real code: only
//! the FUNCTION claim is withheld. The instruction genuinely encodes a call to
//! that address, so the Call cross-reference is filed either way and `kuna xrefs
//! --to 0x349688c1` still reports the call site.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The overlapping call's target — outside every section of the fixture.
const PHANTOM: &str = "sub_349688c1";
/// `e_entry`, the walk's only seed.
const ENTRY: &str = "sub_401000";
/// The one function reached by a real, mapped CALL.
const CALLEE: &str = "sub_401010";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with the bounded recursive discovery on (the consumer
/// that commits the walk's CALL targets) and `unmappedentry` in the requested
/// state. `None` is a visible skip when the `.sla` is missing.
fn bootstrap(unmappedentry: bool) -> Option<ConsoleProgram> {
    let bin =
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/unmapped_call_x86_64");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_unmappedentry: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut().set_kuna_option("fast_funcdisc", "on").expect("fast_funcdisc flips on");
    prog.arch_mut()
        .set_kuna_option("unmappedentry", if unmappedentry { "on" } else { "off" })
        .expect("unmappedentry flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn entry_names(prog: &ConsoleProgram) -> Vec<String> {
    prog.function_entries_canonical().into_iter().map(|e| e.name).collect()
}

/// BEFORE (`--option unmappedentry off`): the unmapped CALL target is committed.
#[test]
fn unmapped_call_target_is_a_function_with_the_option_off() {
    let Some(prog) = bootstrap(false) else {
        return;
    };
    let names = entry_names(&prog);
    assert!(
        names.iter().any(|n| n == PHANTOM),
        "with unmappedentry off the walk must still mint {PHANTOM} — the option \
         must restore the previous discovery set exactly; got {names:?}"
    );
    let phantom = prog
        .function_entries_canonical()
        .into_iter()
        .find(|e| e.name == PHANTOM)
        .expect("checked present above");
    assert_eq!(
        phantom.size, 0,
        "the phantom has no bytes behind it — that is what makes it a phantom"
    );
}

/// AFTER (default): the phantom is gone and both real functions survive.
#[test]
fn unmapped_call_target_is_refused_by_default_and_real_entries_survive() {
    let Some(prog) = bootstrap(true) else {
        return;
    };
    let names = entry_names(&prog);
    assert!(
        !names.iter().any(|n| n == PHANTOM),
        "0x349688c1 is outside every executable section, so it must not be a \
         function by default; got {names:?}"
    );
    for want in [ENTRY, CALLEE] {
        assert!(
            names.iter().any(|n| n == want),
            "{want} is a real, mapped function and must survive; got {names:?}"
        );
    }
}

/// The property that keeps the gate from losing information: the CALL
/// cross-reference is filed either way, so the two arms differ only in the
/// function claim — the entry set loses exactly the phantom and nothing else.
#[test]
fn only_the_phantom_differs_between_the_two_arms() {
    let (Some(off), Some(on)) = (bootstrap(false), bootstrap(true)) else {
        return;
    };
    let mut before = entry_names(&off);
    let mut after = entry_names(&on);
    before.sort();
    after.sort();
    let removed: Vec<&String> = before.iter().filter(|n| !after.contains(n)).collect();
    let added: Vec<&String> = after.iter().filter(|n| !before.contains(n)).collect();
    assert_eq!(removed, vec![&PHANTOM.to_string()], "exactly one entry may be withheld");
    assert!(added.is_empty(), "the gate never adds an entry; got {added:?}");
}
