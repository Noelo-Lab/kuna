//! End-to-end two-pass gate for `unwindstarts` — Mach-O `__TEXT,__unwind_info`
//! function-start discovery (P1 code/data partition).
//!
//! Fixture: `macho_unwind_only_arm64`, a linked arm64 Mach-O whose
//! `LC_FUNCTION_STARTS` was stripped (`-no_function_starts`) and whose local
//! symbols were stripped (`ld64.lld -x`). Its four `uw_*` functions are kept in
//! the image but have no direct caller and no symbol, so the entry-discovery
//! oracles (`LC_MAIN`, exports, `__mod_init_func`, symtab) never reach them and
//! the arm64 walk cannot either — only `__unwind_info` records their starts. Each
//! uses a different stack size so the linker does not fold the runs, leaving four
//! distinct second-level entries. See `macho_unwind_only_arm64.c`.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** the inventory is `__mh_execute_header` + `_main`;
//!   the four `uw_*` bodies are never discovered.
//! * **option ON (the fix):** the four `__unwind_info` starts are recovered as
//!   `sub_<addr>` entries.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The four function starts `__TEXT,__unwind_info` records, each reachable by no
/// other oracle in this stripped image.
const UNWIND_ONLY: [&str; 4] =
    ["sub_1000002e0", "sub_100000328", "sub_100000370", "sub_1000003bc"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn bootstrap(unwindstarts: bool) -> ConsoleProgram {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join("macho_unwind_only_arm64");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("unwindstarts", if unwindstarts { "on" } else { "off" })
        .expect("unwindstarts flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

/// BEFORE (`--option unwindstarts off`): the stripped image's only discovered
/// functions are its program entry and the Mach header; the `__unwind_info`-only
/// bodies are invisible.
#[test]
fn the_unwind_only_functions_are_absent_with_the_option_off() {
    let prog = bootstrap(false);
    assert!(
        prog.lookup_symbol("_main").is_some(),
        "the LC_MAIN entry is still discovered, named by its own symbol"
    );
    for name in UNWIND_ONLY {
        assert!(
            prog.lookup_symbol(name).is_none(),
            "with unwindstarts off nothing may discover {name} — the option must restore \
             the previous inventory exactly"
        );
    }
}

/// default + flip (`--option unwindstarts on`): each `__unwind_info` second-level
/// entry becomes a discovered `sub_<addr>`.
#[test]
fn the_unwind_only_functions_are_recovered_with_the_option_on() {
    let prog = bootstrap(true);
    for name in UNWIND_ONLY {
        assert!(
            prog.lookup_symbol(name).is_some(),
            "with unwindstarts on each __unwind_info function start must be discovered; \
             {name} missing"
        );
    }
}
