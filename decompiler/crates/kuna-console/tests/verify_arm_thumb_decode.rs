//! End-to-end gate for ARM/Thumb decode-mode painting — the deferred
//! Increment-8/17 e2e (`arm_markers` on a LINKED ARM exe).
//!
//! Increments 8 and 17 ported the ARM `$t`/`$a` mapping-symbol decode-mode
//! painting (`TMode` context bit) and the Thumb-FUNC re-home (a LSB-set
//! STT_FUNC also surfaced at the even address) into
//! `kuna-analysis/src/s1_loader/arm_markers.rs`. Those were proven only by a
//! bare ARM `.o` UNIT test — the linked-executable Thumb-decode e2e was blocked
//! because the build host had no ARM linker. The kuna-dev container now provides
//! `arm-linux-gnueabihf-gcc`, so the fixture is a genuine ET_EXEC and this gate
//! drives the full `bootstrap_from_elf` → `load function compute` → `decompile`
//! → `print C` path against it.
//!
//! It proves the analysis-tier paint actually drives a CORRECT Thumb decode
//! through the whole pipeline: the `arm_markers` pass paints `TMode=1` at
//! `compute`'s even entry (from both the `$t` mapping symbol and the
//! odd-address STT_FUNC convention) and re-homes the Thumb FUNC to the even
//! address, the commit seam (`commit_pending_analysis`, fired by `load
//! function`) applies it to the engine's `ContextDatabase` BEFORE the decode,
//! and the SLEIGH translator decodes Thumb (not A32). A correct Thumb decode of
//! `compute` computes `x*3 + 7`; an ARM-mode misdecode of the same bytes yields
//! nonsense — so asserting the arithmetic IS the Thumb-mode proof.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_elf_*` gates, bootstrapping needs the built ARM
//! `.sla` under `specs/` (gitignored; `make specs`, or just
//! `slacomp specs/Ghidra/Processors/ARM/data/languages/ARM8_le.slaspec`). When
//! it is absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_elf;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored LINKED ARM Thumb fixture (built in the kuna-dev container;
/// `arm-linux-gnueabihf-gcc -mthumb -static -nostdlib -e _start`). ET_EXEC with
/// a PT_LOAD R E segment (`ObjectLoadImage` reads only segments).
fn arm_thumb_linked() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_linked_le32")
}

// Load-bearing VMAs, read from `arm-linux-gnueabihf-objdump -d` / `readelf
// --syms` at fixture-build time (do not re-derive — addresses shift across
// toolchains):
//   $t          @ 0x100b8 (NOTYPE LOCAL mapping symbol, start of the Thumb run)
//   compute     @ 0x100b9 (FUNC, LSB-set Thumb odd-address) -> even decode 0x100b8
//   _start      @ 0x100d7 (FUNC, LSB-set Thumb odd-address) -> even decode 0x100d6
//
// `compute`'s ODD loader-symbol address (`entry|1`, the Thumb convention) — what
// the raw ELF `.symtab` records and what `read_loader_symbols` (so
// `ConsoleProgram::lookup_symbol`, which reads the loader-symbol list) returns.
// `load function compute` resolving here still decodes correctly: the engine
// decodes at the LSB-masked even address and `TMode=1` is painted there.
const COMPUTE_ODD_LOADER_ENTRY: u64 = 0x100b9;
/// `compute`'s normalized (even) entry — the address the `TMode=1` paint and the
/// engine decode use, and where the Thumb-FUNC re-home installs the FunctionSymbol
/// in the engine symbol table (so a CALL landing here resolves to the name).
const COMPUTE_EVEN_ENTRY: u64 = 0x100b8;

/// e2e: a LINKED ARM Thumb exe decompiles `compute` correctly **because** the
/// analysis-tier `arm_markers` pass painted `TMode=1` (Thumb) at its even entry
/// (the commit seam applies it before the decode) and re-homed the Thumb FUNC to
/// that even address (so a CALL to it resolves by name).
///
/// Two proofs in one drive:
///  1. **TMode paint** — `load function compute` Thumb-decodes to `a0 * 3 + 7`.
///     An ARM-mode (A32) misdecode of the same byte stream is garbage, so the
///     arithmetic IS the Thumb-mode proof.
///  2. **Thumb-FUNC re-home** — `load function _start` renders the CALL to
///     `compute`'s EVEN entry (`0x100b8`, where the `bl` actually lands) by name
///     as `compute(...)`. Without the even-address SymFact the engine symbol
///     table would only know `compute` at the odd `0x100b9`, so the CALL would
///     render an anonymous `sub_100b8` placeholder.
#[test]
fn arm_thumb_compute_decodes_in_thumb_mode() {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = arm_thumb_linked();
    let bin = match bin.to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let prog = match bootstrap_from_elf(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_arm_thumb_decode: skipping (bootstrap failed, build the ARM `.sla` with \
                 `make specs` or `slacomp .../ARM8_le.slaspec`): {}",
                e.explain()
            );
            return;
        }
    };

    // The loader-symbol list (read_loader_symbols) keeps the raw `.symtab`
    // ODD address (`entry|1`) — the Thumb convention — so `lookup_symbol`
    // returns the odd entry, and `load function compute` resolves there. The
    // decode still lands on the even bytes (LSB masked) with `TMode=1` painted.
    let compute_addr = prog.lookup_symbol("compute").expect("`compute` not resolved by the loader");
    assert_eq!(
        compute_addr.get_offset(),
        COMPUTE_ODD_LOADER_ENTRY,
        "loader symbol keeps the odd Thumb `entry|1` address"
    );

    // --- Proof 1: TMode paint drives a correct Thumb decode of `compute`. -----
    // `load function compute` fires `commit_pending_analysis`, which paints the
    // stashed `TMode=1` facts over the ContextDatabase BEFORE the decode (the
    // timing Thumb mode needs).
    let out = drive(prog, "compute");
    // The correct Thumb decode of `compute(x)` computes `x*3 + 7`. SLEIGH folds
    // the `lsls #1; add; adds #7` Thumb sequence to `a0 * 3 + 7`. An ARM-mode
    // (A32) misdecode of the same byte stream would NOT produce this arithmetic
    // (different, garbage instructions), so this assertion is the Thumb proof.
    assert!(
        out.contains("* 3 + 7"),
        "expected the Thumb-decoded `x*3 + 7` arithmetic in `compute`, got:\n{out}"
    );
    assert!(out.contains("return"), "expected a `return` of the computed value, got:\n{out}");

    // --- Proof 2: the Thumb-FUNC re-home makes the CALL resolve by name. ------
    // Re-bootstrap (the drive above consumed `prog`), load `_start`, and confirm
    // its `bl` to `compute`'s EVEN entry (0x100b8) renders `compute(`. The
    // re-home installed the FunctionSymbol at the even address; without it the
    // CALL target would be anonymous.
    let prog2 =
        bootstrap_from_elf(&bin, "", &spec_roots).expect("re-bootstrap for the _start drive");
    let out_start = drive(prog2, "_start");
    assert!(
        out_start.contains("compute("),
        "the `bl` to compute's even entry ({:#x}) must resolve to the name `compute` \
         (the Thumb-FUNC re-home), got:\n{out_start}",
        COMPUTE_EVEN_ENTRY
    );
    assert!(
        !out_start.contains("sub_100b8") && !out_start.contains("sub_0x100b8"),
        "the even-entry call must NOT render as an anonymous `sub_<addr>` placeholder, \
         got:\n{out_start}"
    );
}

/// Drive `load function <name>` → `decompile` → `print C` on a bootstrapped
/// program and return the captured console output. Mirrors the
/// `verify_w11_elf_plt_names` drive harness.
fn drive(prog: kuna_console::engine::ConsoleProgram, func: &str) -> String {
    let cmds: Vec<String> = [format!("load function {func}"), "decompile".into(), "print C".into()]
        .into_iter()
        .collect();
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
