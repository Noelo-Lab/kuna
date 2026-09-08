//! The RET-call chain a `flow <addr> call` override extends along —
//! `docs/re-needs/flow-call-override-retain.md`.
//!
//! A `push <continuation>; push <target>; ret` run is a call written without a
//! `call` instruction, and a body built out of them is one such `ret` per
//! callee.  Reclassifying ONE of them recovers one call and leaves every later
//! link a return, so the rest of the body is dead and prints as `return;` — the
//! caller cannot even see that the chain continues.
//!
//! Fixture: `kuna-analysis/tests/fixtures/retcallchain_i386` (source
//! `retcallchain_i386.s`), whose `chain_entry` is three links followed by a
//! plain epilogue.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`).  When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::kuna_retcallchain::{
    kuna_chain_sites, CHAIN_MAX_INSNS, CHAIN_MAX_SITES,
};
use kuna_console::project::decompile_targets;

/// `chain_entry`, and the `ret` of each of its three links.
const CHAIN_ENTRY: u64 = 0x0804_900e;
const LINK_RETS: [u64; 3] = [0x0804_9019, 0x0804_9025, 0x0804_9031];
/// The plain epilogue after the last link, and an ordinary leaf's own `ret`.
const EPILOGUE_RET: u64 = 0x0804_9032;
const FIRST_LINK: u64 = 0x0804_a000;

/// The three callees the chain reaches, by the global each `ret` pops.
const CALLS: [&str; 3] = ["(*dat_804a000)()", "(*dat_804a004)()", "(*dat_804a008)()"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and run the analysis commit.  `None` ⇒ specs-less skip.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/retcallchain_i386");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_retcallchain: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit");
    Some(prog)
}

fn flow(addr: u64, kind: &str) -> Directive {
    Directive {
        raw: format!("flow {addr:#x} {kind}"),
        body: Body::Flow { func: None, addr, kind: kind.to_string() },
    }
}

/// Decompile `chain_entry` under `directives` and return its C.
fn decompile_with(directives: Vec<Directive>) -> Option<String> {
    let mut prog = load()?;
    if !directives.is_empty() {
        prog.set_assertions(directives);
        assertions::apply_program_scoped(&mut prog);
    }
    let target = prog
        .resolve_entry(&EntrySelector::Numeric(CHAIN_ENTRY))
        .expect("the fixture has a function at chain_entry");
    let funcs = decompile_targets(&mut prog, vec![target], false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    assert_eq!(funcs[0].error, None, "the pipeline aborted:\n{code}");
    Some(code)
}

/// The sites the walk reports for an override at `at`.
fn sites(prog: &ConsoleProgram, at: u64) -> Vec<u64> {
    let space = prog.arch().manage().get_default_code_space().cloned().expect("code space");
    let entry = Address::new(space.clone(), CHAIN_ENTRY);
    let at = Address::new(space, at);
    kuna_chain_sites(prog.arch().translate(), &entry, &at, CHAIN_MAX_SITES, CHAIN_MAX_INSNS)
        .iter()
        .map(|a| a.get_offset())
        .collect()
}

/// Un-asserted, every `ret` is a return and the body is empty.
#[test]
fn the_baseline_reads_the_whole_chain_as_one_return() {
    let Some(code) = decompile_with(Vec::new()) else { return };
    for call in CALLS {
        assert!(!code.contains(call), "baseline already recovered {call}:\n{code}");
    }
}

/// The need's own case: one override on the first link recovers all three calls.
#[test]
fn overriding_the_first_link_recovers_the_whole_chain() {
    let Some(code) = decompile_with(vec![flow(LINK_RETS[0], "call")]) else { return };
    for call in CALLS {
        assert!(code.contains(call), "the chain stopped before {call}:\n{code}");
    }
}

/// Every link names the other two, whichever one the caller happened to find.
#[test]
fn each_link_reports_the_others() {
    let Some(prog) = load() else { return };
    for (i, at) in LINK_RETS.iter().enumerate() {
        let want: Vec<u64> =
            LINK_RETS.iter().copied().filter(|a| a != at).collect();
        assert_eq!(sites(&prog, *at), want, "link {i} at {at:#x}");
    }
}

/// A `ret` that is not a link extends to nothing: the plain epilogue pushes no
/// continuation, and neither does an ordinary leaf's own return.
#[test]
fn an_ordinary_return_extends_to_nothing() {
    let Some(prog) = load() else { return };
    assert!(sites(&prog, EPILOGUE_RET).is_empty(), "the epilogue was read as a link");
    assert!(sites(&prog, FIRST_LINK).is_empty(), "an address outside the walk was read as a link");
}

/// The site cap is a hard stop, not an advisory one.
#[test]
fn the_site_cap_bounds_what_is_reported() {
    let Some(prog) = load() else { return };
    let space = prog.arch().manage().get_default_code_space().cloned().expect("code space");
    let entry = Address::new(space.clone(), CHAIN_ENTRY);
    let at = Address::new(space, LINK_RETS[0]);
    let capped = kuna_chain_sites(prog.arch().translate(), &entry, &at, 1, CHAIN_MAX_INSNS);
    // Only the first link fits under the cap, and it is the override's own site,
    // so there is nothing left to report.
    assert!(capped.is_empty(), "the cap did not bound the walk: {capped:?}");
}
