//! End-to-end gate for the Listing/xref tier: the Aggressive Instruction Finder
//! gap-walk (`AggressiveInstructionFinderAnalyzer`), the third Listing consumer —
//! THE PAYOFF.
//!
//! Fixture: `aif_gap_x86_64` (`+.c`), a STRIPPED PIE x86-64 ELF with a function
//! reachable ONLY through an indirect/data path:
//!
//! ```c
//! // 24 directly-called handlers h0..h23, all sharing an identical prologue:
//! __attribute__((noinline)) int hN(int x) { int y = x + k; y = y * 2; return y + N; }
//! __attribute__((noinline)) int hidden_handler(int x) { int y=x+64; y=y*2; return y+9; }
//! static handler_fn const table[2] = { h0, hidden_handler };  // hidden_handler ONLY here
//! int main(void){ int s=sel; int r=table[s&1](s); /* CALLIND */ r += h0(s); ... h23(s); }
//! ```
//!
//! The 24 `h0..h23` are all called DIRECTLY from `main`, so the recursive-descent
//! Listing walk reaches them — clearing Ghidra's `MINIMUM_FUNCTION_COUNT` (20) — and
//! their identical `push rbp; mov rsp,rbp; ...` prologue stocks the fingerprint
//! histogram far past the acceptance threshold (4).
//!
//! `hidden_handler` (VMA `0x13ae`) is:
//!  - in NO symbol table (the binary is stripped);
//!  - NOT covered by an `.eh_frame` FDE (built `-fno-asynchronous-unwind-tables`);
//!  - NEVER the target of a static CALL — its address lives ONLY in the const
//!    `.rodata` function-pointer table at `0x3df0` (slot 1 = `0x3df8`), and
//!    `main` calls it via `call *reg` with a volatile (unfoldable) index.
//!
//! So entry discovery + funcsyms + the recursive-descent Listing walk all MISS it:
//! `main` (`sub_13c9`) renders the call as `(**(code **)(...0x3df0))(...)` (the
//! target unresolved), and `0x13ae` stays an UNDEFINED gap between the last
//! directly-called handler and `main` (`0x13c9`).
//!
//! AIF's gap-walk finds it: `hidden_handler`'s prologue
//! (`push rbp; mov rsp,rbp; mov edi,-0x14(rbp); ...`) is byte-for-byte the SAME
//! mnemonic shape as `h0..h23`, so it matches a function-start fingerprint shared by
//! >= 4 discovered functions; and it disassembles into a valid subroutine (a clean
//! `ret`, 11 instructions). It is emitted as a discovered `entries` fact →
//! registered as `sub_13ae`.
//!
//! The assertions (the before/after):
//!  - **flags ON** (`--option listing on --option aif on`): the gap-walk runs over
//!    the built Listing, accepts `0x13ae`, and the commit registers `sub_13ae`,
//!    so it is decompilable BY NAME (`load function sub_13ae`).
//!  - **flags OFF** (default): the gap-walk never runs (and the Listing is never
//!    built), so `0x13ae` is in no symbol table — `sub_13ae` is not registered.
//!    The default output is byte-identical to today.
//!
//! ## The build-timing fix this proves
//!
//! The live CLI emits `option listing on` AFTER `load file` but the analysis driver
//! runs at load — so the Listing (and the AIF gap-walk over it) must run at the
//! DEFERRED commit point (`read symbols` / `commit_pending_analysis`), when the flag
//! is known. This test mirrors that ordering: it sets the options on the live arch
//! and THEN calls `commit_pending_analysis()`.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and returns
//! early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64")
}

/// The hidden function recovered only by the AIF gap-walk (VMA pinned by the
/// fixture; see the fixtures README).
const HIDDEN_FN: &str = "sub_13ae";

/// Whether to enable the AIF gap-walk (and the Listing it reads).
enum Mode {
    /// Default: no flags (the today baseline).
    Off,
    /// `--option listing on --option aif on`, set BEFORE the deferred commit — the
    /// live-CLI ordering the build-timing fix targets.
    On,
}

/// Bootstrap the fixture, (optionally) enable AIF, commit the analysis, and return
/// the live program so the caller can inspect the symbol table / decompile a
/// discovered function. `None` ⇒ specs-less skip.
fn bootstrap(mode: Mode) -> Option<kuna_console::engine::ConsoleProgram> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_aif: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The live-CLI ordering: the `option` lines precede `read symbols` (the deferred
    // commit). Set the flags on the live arch BEFORE committing so the deferred
    // Listing build + AIF gap-walk see them on.
    if let Mode::On = mode {
        prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
        prog.arch_mut().set_kuna_option("aif", "on").expect("aif flips on");
    }
    // `read symbols`: build the Listing (deferred) and run the consumers, gated.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Decompile `func` in the already-bootstrapped program and return the captured C.
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

/// THE PAYOFF: with AIF on, the hidden function — reachable only through the
/// `.rodata` function-pointer table — is recovered as `sub_13ae`, registered in
/// the symbol table, and decompilable by name. With AIF off it is in no symbol
/// table (the default is unchanged).
#[test]
fn aif_recovers_function_reachable_only_via_data_path() {
    let Some(off) = bootstrap(Mode::Off) else {
        return; // specs-less skip
    };
    // OFF (default): the hidden, indirect-only function is NOT discovered (the
    // gap-walk never runs and no oracle / static CALL reaches 0x13ae).
    assert!(
        off.lookup_symbol(HIDDEN_FN).is_none(),
        "hidden function {HIDDEN_FN} must NOT be discovered with AIF off (default \
         parity): it is reachable only through the .rodata function-pointer table"
    );

    let Some(on) = bootstrap(Mode::On) else {
        return; // specs-less skip (unreachable if the first bootstrap succeeded)
    };
    // ON: the AIF gap-walk accepts 0x13ae (fingerprint match + valid subroutine)
    // and the commit registers it.
    assert!(
        on.lookup_symbol(HIDDEN_FN).is_some(),
        "AIF gap-walk must discover the indirect-only function {HIDDEN_FN} (it did \
         not): the fingerprint match + valid-subroutine check should accept it"
    );

    // ...and it is decompilable by name, producing its real body (`x + 64) * 2 + 9`).
    let body = decompile(on, HIDDEN_FN);
    eprintln!("---- {HIDDEN_FN} (AIF on) ----\n{body}");
    assert!(
        body.contains(HIDDEN_FN),
        "expected a decompiled body for the AIF-discovered {HIDDEN_FN}, got:\n{body}"
    );
    assert!(
        body.contains("return"),
        "expected a C function body (with a return) for {HIDDEN_FN}, got:\n{body}"
    );
}

/// The default path is unchanged by AIF: with NO flags the hidden function is not
/// registered (the deferred Listing build + gap-walk never fire by default). This
/// is the parity guarantee — default-off ⇒ byte-identical to today.
#[test]
fn flags_off_does_not_discover_the_gap_function() {
    let Some(off) = bootstrap(Mode::Off) else {
        return; // specs-less skip
    };
    assert!(
        off.lookup_symbol(HIDDEN_FN).is_none(),
        "default (AIF off) must not register the gap function {HIDDEN_FN}"
    );
    // The directly-called handlers are still discovered (entry/CALL reachability),
    // and `main` (sub_13c9) still decompiles, with the indirect call unresolved.
    let main_body = decompile(off, "sub_13c9");
    eprintln!("---- main / sub_13c9 (AIF off / default) ----\n{main_body}");
    assert!(
        main_body.contains("sub_13c9"),
        "default main body must name sub_13c9:\n{main_body}"
    );
    // The hidden function is NOT named in the default main body (the indirect call
    // resolves to a `(code **)` deref, not a `sub_13ae(` call).
    assert!(
        !main_body.contains("sub_13ae("),
        "default main must NOT resolve the indirect call to sub_13ae():\n{main_body}"
    );
}
