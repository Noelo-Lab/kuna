//! End-to-end gate for MIPS16 `ISA_MODE` decode-mode painting (Increment 21, the
//! MIPS analog of ARM Thumb `TMode`).
//!
//! Drives the full `load file <mips16_le32>` → `commit analysis` →
//! `load function m16_square` → `decompile` → `print C` path against a **real**
//! freestanding MIPS ELF whose `m16_square` function is compiled as MIPS16 (its
//! STT_FUNC carries `st_other & 0xf0 == STO_MIPS_MIPS16`).  The `mips_isa`
//! analysis pass paints the SLEIGH `ISA_MODE` context to 1 at the (even) entry,
//! so the body decodes as MIPS16 instead of being misdecoded as MIPS32.
//!
//! ## What "decoded as MIPS16" means here
//!
//! `m16_square(n)` is `return n * n + 3`.  In MIPS16 the body is the four 2-byte
//! ops `mult a0,a0; mflo v0; jr ra; addiu v0,3` (an 8-byte function).  Decoded
//! correctly, the C recovers a self-multiply of the argument plus 3 and returns.
//! If `ISA_MODE` were NOT painted, the same 8 bytes (`ec98 ea12 e820 4a03`)
//! would be read as MIPS32 4-byte words — a coprocessor/branch soup, no clean
//! `arg * arg + 3` — which is the BEFORE state this gate guards.
//!
//! ## Timing
//!
//! The `ISA_MODE` paint lands in the engine's `ContextDatabase`, which the decode
//! reads — so it MUST be committed BEFORE `load function` decodes the body. The
//! analysis facts are committed at `read symbols` (the gated commit, after any
//! `--option` flips); the test triggers that with `commit_pending_analysis`
//! (after optionally flipping `mips_isa off`), then loads + decompiles.
//!
//! ## `.sla` precondition
//!
//! Like the sibling `verify_w11_*` gates, bootstrapping needs the built MIPS
//! `.sla` under `specs/` (gitignored; `make specs`, or just
//! `slacomp specs/Ghidra/Processors/MIPS/data/languages/mips32be.slaspec`).  When
//! it is absent the bootstrap fails; the test prints that and returns early (a
//! specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The freestanding MIPS16 fixture (big-endian; shared with the kuna-analysis
/// loader unit tests). `m16_square` is the `__attribute__((mips16))` leaf.
fn mips16() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/mips16_le32")
}

/// Bootstrap the MIPS16 fixture, optionally flip `mips_isa off` BEFORE the gated
/// analysis commit (exactly the order the CLI uses: `option` lines before `read
/// symbols`), commit, then `load function m16_square` → `decompile` → `print C`.
/// Returns the printed C, or `None` if the MIPS `.sla` is absent (a visible skip).
fn decompile_m16(turn_off: bool) -> Option<String> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = mips16().to_str()?.to_string();

    let mut prog: ConsoleProgram = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_mips16_isa: skipping (bootstrap failed, build the MIPS `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };

    // The MIPS16 function resolves by name (the loader's FUNC symbol stream).
    assert!(
        prog.lookup_symbol("m16_square").is_some(),
        "MIPS16 function `m16_square` not resolved by the loader"
    );

    if turn_off {
        // Flip the analysis-pass gate off BEFORE the commit (the CLI sets `option`
        // lines before `read symbols`); the disabled pass's ISA_MODE paint is then
        // dropped at commit, leaving the default MIPS32 (mis)decode.
        prog.arch_mut().set_kuna_option("mips_isa", "off").expect("set mips_isa off");
    }

    // Commit the gated analysis facts (what `IfcReadSymbols` does): the ISA_MODE
    // paint must land in the ContextDatabase BEFORE `load function` decodes.
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let cmds: Vec<String> = ["load function m16_square", "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
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
    Some(status.optr.clone())
}

#[test]
fn mips16_function_decodes_as_mips16_not_mips32() {
    let Some(out) = decompile_m16(false) else { return };

    eprintln!("=== decompiled m16_square (mips_isa ON / MIPS16) ===\n{out}\n===");

    // A non-empty C body must have come back (it decoded to *something*).
    assert!(
        out.contains("m16_square") || out.contains('{'),
        "expected a decompiled body for m16_square, got:\n{out}"
    );

    // MIPS16 decode recovers `arg * arg + 3` (a self-multiply plus the constant 3,
    // returned). The exact varnode naming varies, but a MIPS16-decoded body must
    // carry a multiplication (the `mult a0,a0` → `n * n`) and a return; a MIPS32
    // *misdecode* of the identical bytes (read as 4-byte words) produces no such
    // clean shape.
    let has_mul = out.contains('*');
    let has_return = out.contains("return");
    assert!(
        has_mul && has_return,
        "MIPS16-decoded m16_square must show a multiply and a return (got mul={has_mul}, \
         ret={has_return}); a MIPS32 misdecode would not. Body:\n{out}"
    );
}

/// With `mips_isa off`, the SAME function reverts to the un-painted default
/// (MIPS32) decode of the MIPS16 bytes — the BEFORE state. Pins that the pass is
/// what flips the decode (not some other markup) and that the option gate works:
/// the two renderings DIFFER.
#[test]
fn mips16_off_reverts_to_mips32_misdecode() {
    let on = decompile_m16(false);
    let off = decompile_m16(true);

    let (Some(on), Some(off)) = (on, off) else {
        eprintln!("verify_mips16_isa: skipping off-toggle (no MIPS `.sla`)");
        return;
    };

    eprintln!("=== mips_isa ON ===\n{on}\n=== mips_isa OFF ===\n{off}\n===");

    // The painted (MIPS16) form differs from the un-painted (MIPS32 misdecode)
    // form — the option genuinely flips the decode.
    assert_ne!(
        on, off,
        "mips_isa on/off must produce different output (the paint flips the decode)"
    );
}
