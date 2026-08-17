//! End-to-end gate for the libstdc++ `std::__throw_*` no-return family (GH-273,
//! kuna DIV-78).
//!
//! Fixture: `cxxthrow_noreturn_x86_64` (`+.cpp`), a non-PIE x86-64 ELF built with
//! the local **clang++** at `-O1`. It declares two libstdc++ throw helpers itself,
//! deliberately WITHOUT `__attribute__((__noreturn__))`, so the compiler emits the
//! code after each call:
//!
//! ```c
//! long append_bound(long n) {
//!   if (n <= 100) return n * 3;
//!   std::__throw_length_error("basic_string::append");
//!   return n * 7;                      // dead in fact: the helper always throws
//! }
//! ```
//!
//! At the decompiler's boundary no attribute survives — the callee is an undefined
//! `.dynsym` import (`_ZSt20__throw_length_errorPKc`) reached through a PLT stub —
//! so the only thing that can prove the call cannot return is the shipped
//! no-return name list. Upstream Ghidra's list omits the whole family, which is
//! the bug: every clang/g++ C++ binary that touches `std::string` / `std::vector`
//! emits the dead tail as if it ran.
//!
//! The two passes:
//!
//!  - **default** (`noreturn_known` on, DIV-78 list entries present): the call is
//!    marked no-return, the fall-through is never followed, and the `n * 7` tail is
//!    gone.
//!  - **`option noreturn_known off` + `option noreturn_externmatch off`** (both
//!    consumers of the list disabled): the pre-fix rendering — the dead `7`
//!    multiplier is back in the emitted C.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cxxthrow_noreturn_x86_64")
}

/// Whether the two list consumers are left at their defaults or turned off.
enum Mode {
    /// Default: `noreturn_known` + `noreturn_externmatch` on (the fix).
    Default,
    /// Both list consumers off — the pre-DIV-78 rendering.
    ListOff,
}

/// Bootstrap the fixture, apply the mode, decompile `func`, return the captured C
/// (`None` ⇒ specs-less skip).
fn decompile(func: &str, mode: Mode) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_cxx_throw_noreturn: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: `option` lines precede `read symbols` (the deferred
    // analysis commit), so the flags are set before the pass would run.
    if let Mode::ListOff = mode {
        prog.arch_mut().set_kuna_option("noreturn_known", "off").expect("noreturn_known flips off");
        prog.arch_mut()
            .set_kuna_option("noreturn_externmatch", "off")
            .expect("noreturn_externmatch flips off");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");

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

/// THE PAYOFF: by default the `std::__throw_length_error` call is no-return, so the
/// `n * 7` tail — reachable only by falling through a call that always throws — is
/// gone. With the list consumers off it is emitted, which is the GH-273 bug.
#[test]
fn throw_length_error_eliminates_dead_tail() {
    let Some(fixed) = decompile("append_bound", Mode::Default) else {
        return; // specs-less skip
    };
    let buggy =
        decompile("append_bound", Mode::ListOff).expect("second bootstrap succeeds if the first did");

    eprintln!("---- append_bound (default / fixed) ----\n{fixed}");
    eprintln!("---- append_bound (list off / pre-fix) ----\n{buggy}");

    assert_ne!(
        fixed, buggy,
        "the throw-family list entry must change append_bound's decompilation:\n\
         --- default ---\n{fixed}\n--- list off ---\n{buggy}"
    );
    // Default: the call survives, marked no-return, and the dead multiplier is gone.
    assert!(
        fixed.contains("__throw_length_error"),
        "default output must still show the throw call:\n{fixed}"
    );
    assert!(
        fixed.contains("// no-return"),
        "default output must mark the throw call no-return:\n{fixed}"
    );
    assert!(
        !fixed.contains('7'),
        "default output must not emit the dead `* 7` tail:\n{fixed}"
    );
    // List off: the pre-fix rendering, with the dead multiplier present.
    assert!(
        !buggy.contains("// no-return"),
        "list-off output must not mark the throw call no-return:\n{buggy}"
    );
    assert!(
        buggy.contains('7'),
        "list-off output must emit the dead `* 7` tail (the GH-273 bug):\n{buggy}"
    );
}

/// A second family member on the same binary, so the fix is not proven on one
/// symbol: `std::__throw_out_of_range` likewise drops its `* 11` dead tail.
#[test]
fn throw_out_of_range_eliminates_dead_tail() {
    let Some(fixed) = decompile("at_bound", Mode::Default) else {
        return; // specs-less skip
    };
    let buggy =
        decompile("at_bound", Mode::ListOff).expect("second bootstrap succeeds if the first did");

    eprintln!("---- at_bound (default / fixed) ----\n{fixed}");
    eprintln!("---- at_bound (list off / pre-fix) ----\n{buggy}");

    assert_ne!(fixed, buggy, "the throw-family list entry must change at_bound too");
    assert!(
        fixed.contains("__throw_out_of_range") && fixed.contains("// no-return"),
        "default output must show the throw call marked no-return:\n{fixed}"
    );
    // 0xb is the dead `* 11` multiplier the fall-through carried.
    assert!(!fixed.contains("0xb"), "default output must not emit the dead `* 11` tail:\n{fixed}");
    assert!(buggy.contains("0xb"), "list-off output must emit the dead `* 11` tail:\n{buggy}");
}
