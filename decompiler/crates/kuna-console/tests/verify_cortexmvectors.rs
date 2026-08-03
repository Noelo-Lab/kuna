//! End-to-end gate for `cortexmvectors` — the widened ARM Cortex-M hardware
//! vector-table signature (P1 entry discovery, oracle 6).
//!
//! ## Why this is a Rust gate and not a `tests/stages/*.xml` two-pass case
//!
//! The oracle keys entirely on **ELF structure**: section `sh_flags`, the
//! `PT_LOAD` program headers, `e_entry`, and the section bytes at a section's
//! base. The datatest/stages harness embeds raw `<bytechunk>`s with explicit
//! `<symbol>` definitions and never constructs an `ObjectLoadImage`, so none of
//! those facts exist on that path and the option is unobservable there. Every
//! sibling analysis-tier discovery flag (`funcstart_patterns`, `aif`,
//! `eh_frame_full`, `i386_pie_plt`) is gated the same way, by a cross-crate
//! `verify_*` test over a real object. This file is the two-pass case: the same
//! image, option off (the bug) then option on (the fix).
//!
//! ## Fixture
//!
//! `cortexm_ccm_vectors_le32` (built by the committed
//! `cortexm_ccm_vectors_le32.py`, see the fixtures README): a stripped bare-metal
//! Cortex-M image that fails **all three** shipped predicates at once, the way
//! the four biggest decbench ARM projects do —
//!
//!  - `.isr_vector` @ `0x08000000` is `SHF_ALLOC` only and sits in a **read-only**
//!    `PT_LOAD`, so it is neither an `SHF_EXECINSTR` section nor inside a `PF_X`
//!    load (cleanflight / betaflight);
//!  - its word 0 is `0x1000fff0`, a stack in STM32F4 CCM RAM, outside the
//!    architectural SRAM window `0x20000000..0x3FFFFFFF` (cleanflight /
//!    betaflight);
//!  - its word 1 (`Reset_Handler|1` = `0x08008001`) is **not** `e_entry`, which
//!    the link script points at `_start` (`0x08008011`) instead (crazyflie /
//!    nuttx).
//!
//! `.text` @ `0x08008000` holds five two-instruction Thumb bodies
//! (`movs r0,#k ; bx lr`), four of them reachable ONLY through a vector slot.
//!
//! ## The two passes
//!
//! **Off (default)** — no table is confirmed, so (a) the four handlers are in no
//! symbol table, and (b) nothing paints `TMode=1`, so the language default
//! (`ARM:LE:32:v8`, A32) misdecodes even the one function `e_entry` does find and
//! it produces no C at all. One entry, zero output: the bug.
//!
//! **On** — the widened signature confirms the table, seeds all four handlers,
//! and region-paints `TMode=1` across `.text`. Five entries, and every one
//! decompiles to its real constant (`movs r0,#7 ; bx lr` → `return 7;`). Decoding
//! those bytes as A32 does not produce that, so the arithmetic IS the Thumb-paint
//! proof.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built ARM `.sla` under `specs/` (gitignored; `make
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_ccm_vectors_le32")
}

/// `Reset_Handler`, vector slot 1 — `movs r0,#1 ; bx lr`.
const RESET_HANDLER: &str = "sub_8008000";
/// `NMI_Handler`, vector slot 2 — `movs r0,#7 ; bx lr`. Reachable ONLY through
/// the vector table (no symbol, no call, no `e_entry` relation).
const NMI_HANDLER: &str = "sub_8008004";
/// `HardFault_Handler`, vector slot 3 — `movs r0,#11 ; bx lr`.
const HARDFAULT_HANDLER: &str = "sub_8008008";
/// `SysTick_Handler`, vector slot 4 — `movs r0,#21 ; bx lr`.
const SYSTICK_HANDLER: &str = "sub_800800c";
/// `_start` — the ONE function `e_entry` finds with or without the option
/// (`movs r0,#0 ; bx lr`). Its C body is the Thumb-paint witness.
const START: &str = "sub_8008010";

/// Bootstrap the fixture, optionally flip `cortexmvectors` on (before the
/// deferred commit, the live-CLI ordering), commit, and hand back the program.
/// `None` ⇒ specs-less skip.
fn bootstrap(on: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_cortexmvectors: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    if on {
        prog.arch_mut()
            .set_kuna_option("cortexmvectors", "on")
            .expect("cortexmvectors flips on");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// Decompile `func` in an already-bootstrapped program and return the captured C.
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

/// Pass 1 — THE BUG. With the option off (the default) the shipped signature
/// rejects the table on all three counts, so none of the four vector handlers is
/// registered and the whole firmware is one undecodable function.
#[test]
fn default_misses_the_ccm_relocated_vector_table() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    for h in [RESET_HANDLER, NMI_HANDLER, HARDFAULT_HANDLER, SYSTICK_HANDLER] {
        assert!(
            off.lookup_symbol(h).is_none(),
            "default (cortexmvectors off) must NOT discover {h}: the shipped \
             signature rejects an A-only .isr_vector in a read-only PT_LOAD with a \
             CCM stack word and a reset word that is not e_entry"
        );
    }
    // `e_entry` still finds `_start`, but with no table there is no Thumb region
    // paint, so its two Thumb halfwords are decoded as A32 and produce no C.
    assert!(off.lookup_symbol(START).is_some(), "e_entry always finds {START}");
    let body = decompile(off, START);
    eprintln!("---- {START} (cortexmvectors off / default) ----\n{body}");
    assert!(
        !body.contains("return 0;"),
        "default must NOT recover {START}'s Thumb body (nothing painted TMode=1), \
         got:\n{body}"
    );
}

/// Pass 2 — THE FIX. With the option on the widened signature confirms the table,
/// every vector handler is registered, and the whole-image Thumb region paint
/// makes each of them decompile to its real constant.
#[test]
fn cortexmvectors_recovers_the_handlers_and_paints_thumb() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    for (h, want) in [
        (RESET_HANDLER, "return 1;"),
        (NMI_HANDLER, "return 7;"),
        (HARDFAULT_HANDLER, "return 0xb;"),
        (SYSTICK_HANDLER, "return 0x15;"),
    ] {
        assert!(
            on.lookup_symbol(h).is_some(),
            "cortexmvectors must discover the vector handler {h} (it did not)"
        );
        let Some(prog) = bootstrap(true) else { return };
        let body = decompile(prog, h);
        eprintln!("---- {h} (cortexmvectors on) ----\n{body}");
        assert!(
            body.contains(want),
            "{h} must Thumb-decode to `{want}` (an A32 read of the same bytes does \
             not), got:\n{body}"
        );
    }
    // The pre-existing `e_entry` function now decodes too — the region paint is
    // what a per-entry paint cannot supply.
    let Some(prog) = bootstrap(true) else { return };
    let body = decompile(prog, START);
    eprintln!("---- {START} (cortexmvectors on) ----\n{body}");
    assert!(
        body.contains("return 0;"),
        "the confirmed table's TMode=1 region paint must make {START} decode as \
         Thumb, got:\n{body}"
    );
}
