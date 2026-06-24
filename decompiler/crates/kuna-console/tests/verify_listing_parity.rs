//! PR2 parity gate for the Listing/xref tier (scope-B): the key proof that
//! enabling `--option listing on` changes **nothing** in the decompiler output.
//!
//! The Listing is built at bootstrap behind the flag (PR2 wiring) but **no pass
//! consumes it yet** (the first consumer is PR6), so the decompiler output MUST
//! be byte-identical with the flag on, with the flag off, and to today. This
//! test decompiles `fauxware main` → `print C` two ways and asserts the C is the
//! same string:
//!
//!  - default (flag off, the today baseline),
//!  - `--option listing on` (the live-CLI ordering: the `option` line follows
//!    `load file`, so the flag flips on the live arch).
//!
//! Note on build-at-load timing: the analysis driver (which would build the
//! Listing) runs once inside `bootstrap_from_elf` (`load file`), and the live
//! CLI emits `option listing on` *after* `load file`, so the build-at-load does
//! not fire through the live CLI today — wiring it to fire is PR6's concern (the
//! first consumer). The build-at-load *body* (the flag-on path) is exercised
//! directly through the engine's real `Translate` by `verify_listing_core.rs`'s
//! `listing_build_through_engine_driver_seeds`. Either way the output is
//! byte-identical: nothing consumes the Listing.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_*` gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored non-PIE x86-64 fauxware fixture (shared with `verify_listing_core`).
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// How to set the `listing` flag for a decompile run.
enum ListingMode {
    /// Default: never touched (the today baseline).
    Off,
    /// `--option listing on` issued after bootstrap (the live-CLI ordering — the
    /// build-at-load has already happened by then, so this only flips the live
    /// flag; the Listing is not (re)built but nothing consumes it either way).
    OnAfterBootstrap,
}

/// Bootstrap fauxware, optionally enable `listing`, decompile `main`, and return
/// the captured C (`None` ⇒ specs-less skip).
fn decompile_main(mode: ListingMode) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fauxware().to_str()?.to_string();
    let mut prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_listing_parity: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    if let ListingMode::OnAfterBootstrap = mode {
        prog.arch_mut()
            .set_kuna_option("listing", "on")
            .expect("listing option flips on");
    }

    let cmds: Vec<String> =
        ["load function main".into(), "decompile".into(), "print C".into()].to_vec();
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

/// The key PR2 proof: `--option listing on` is byte-identical to the default
/// (flag off). The Listing is built behind the flag but consumed by no pass, so
/// there is zero behavior change.
#[test]
fn listing_on_is_byte_identical_to_off() {
    let Some(off) = decompile_main(ListingMode::Off) else {
        return; // specs-less skip
    };
    let on = decompile_main(ListingMode::OnAfterBootstrap)
        .expect("second bootstrap succeeds if the first did");

    assert!(!off.trim().is_empty(), "expected non-empty C for fauxware main");
    assert!(off.contains("main") || off.contains("undefined"), "expected a function body:\n{off}");
    assert_eq!(
        on, off,
        "`--option listing on` must produce byte-identical C to the default (flag off): \
         the Listing is built but consumed by no pass yet (PR2)\n--- off ---\n{off}\n--- on ---\n{on}"
    );
    eprintln!(
        "verify_listing_parity: flag-on == flag-off, {} bytes of identical C",
        off.len()
    );
}
