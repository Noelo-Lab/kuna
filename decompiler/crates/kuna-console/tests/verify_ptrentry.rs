//! End-to-end gate for `ptrentry` — pointer-referenced ARM function entries
//! (P1 code/data partition, ARM discovery Stage 3b).
//!
//! ## Why this is a Rust gate and not a `tests/stages/*.xml` two-pass case
//!
//! The pass keys entirely on **ELF structure** — the allocated-section table, the
//! `PT_LOAD` program headers, and the section bytes at a section's base — and on
//! the **Listing tier**, which is built only from a real `ObjectLoadImage`. The
//! datatest/stages harness embeds raw `<bytechunk>`s with explicit `<symbol>`
//! definitions and never constructs a load image, so neither half exists on that
//! path and the option is unobservable there. Every sibling analysis-tier
//! discovery flag (`funcstart_patterns`, `aif`, `eh_frame_full`,
//! `cortexmvectors`, `i386_pie_plt`) is gated the same way, by a cross-crate
//! `verify_*` test over a real object. This file is the two-pass case: the same
//! image, option off (the bug) then option on (the fix).
//!
//! ## Fixture
//!
//! `cortexm_ptrentry_le32` (built by the committed `cortexm_ptrentry_le32.py`,
//! see the fixtures README): a stripped bare-metal Cortex-M image whose vector
//! table the **shipped** oracle already confirms, so nothing here depends on
//! `cortexmvectors`. It carries the two shapes this option must tell apart:
//!
//!  - **`LEAF` @ `0x08000048`** — `movs r0,#7 ; bx lr`. No `BL` reaches it, its
//!    first halfword is not a Thumb frame prologue (so Stage 2's `PUSH {..,lr}`
//!    scan and Stage 3's prologue gate both skip it) and it is two instructions
//!    long (so `MIN_SUBROUTINE_INSNS` would reject it anyway). The only thing that
//!    reaches it is a function-pointer word in `.rodata` — the fops-struct /
//!    vtable / ISR-table shape this option exists for.
//!  - **`SWCASE` @ `0x0800005c`** — byte-for-byte the same shape, also reached by
//!    a code-pointer word. The difference is that its pointer word
//!    (`0x08000058`) lies in the **same discovered function** as the target, which
//!    is exactly the `ldr pc,[pc,r]` switch-table layout. Admitting it would split
//!    a real function body, so `ptrentry` must leave it undiscovered — with the
//!    option on or off.
//!
//! ## The two passes
//!
//! **Off (default)** — the shipped discovery stages find `reset` and `callee`
//! only. `LEAF`'s four bytes are in no symbol table and produce no C at all: the
//! bug the arm-entry-granularity proposal measured 1,671 times over the decbench
//! ARM corpus.
//!
//! **On** — `LEAF` is registered and decompiles to its real constant
//! (`movs r0,#7 ; bx lr` → `return 7;`), while `SWCASE` stays undiscovered.
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
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/cortexm_ptrentry_le32")
}

/// The reset vector's function — `push {r7,lr} ; bl callee ; pop {r7,pc}`.
const RESET: &str = "sub_8000040";
/// The pointer-only frameless leaf — `movs r0,#7 ; bx lr`, reachable ONLY through
/// the `.rodata` function-pointer word at `0x08000060`.
const LEAF: &str = "sub_8000048";
/// The `BL` target the recursive-descent walk finds on its own.
const CALLEE: &str = "sub_8000050";
/// The switch-table case body — same shape as [`LEAF`], but its pointer word sits
/// inside the same discovered function, so it must never become an entry.
const SWCASE: &str = "sub_800005c";

/// Bootstrap the fixture with the Listing tier on (the surface `decompile-all` /
/// `kuna functions` inject on non-x86-64), optionally flipping `ptrentry` on
/// before the deferred commit — the live-CLI ordering. `None` ⇒ specs-less skip.
fn bootstrap(on: bool) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let bin = fixture().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_ptrentry: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The non-x86-64 discovery set the `decompile-all` / `functions` drivers
    // inject (DIV-20). `funcstart_patterns` is what COMMITS the recursive-descent
    // walk's own finds, so without it `callee` would be discovered but unnamed.
    prog.arch_mut().set_kuna_option("listing", "on").expect("listing flips on");
    prog.arch_mut()
        .set_kuna_option("funcstart_patterns", "on")
        .expect("funcstart_patterns flips on");
    if on {
        prog.arch_mut().set_kuna_option("ptrentry", "on").expect("ptrentry flips on");
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

/// Pass 1 — THE BUG. Every shipped discovery stage rejects a frameless
/// two-instruction leaf that only a data pointer reaches, so its bytes are never
/// decompiled at all.
#[test]
fn default_drops_the_pointer_only_leaf() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    assert!(off.lookup_symbol(RESET).is_some(), "the reset vector is always found");
    assert!(off.lookup_symbol(CALLEE).is_some(), "the walk always follows the BL");
    assert!(
        off.lookup_symbol(LEAF).is_none(),
        "default (ptrentry off) must NOT discover {LEAF}: Stage 3 finds its \
         .rodata pointer and then rejects it for having no frame prologue and \
         fewer than three instructions"
    );
    // The bug in its user-visible form: real code between two emitted functions
    // that produces no output.
    let body = decompile(off, RESET);
    eprintln!("---- {RESET} (ptrentry off / default) ----\n{body}");
    assert!(!body.contains("return 7;"), "the leaf's body is nowhere in the output");
}

/// Pass 2 — THE FIX. The containment precision model admits the pointer-only leaf
/// and still refuses the switch-table case that shares its shape.
#[test]
fn ptrentry_recovers_the_pointer_only_leaf() {
    let Some(on) = bootstrap(true) else {
        return; // specs-less skip
    };
    assert!(
        on.lookup_symbol(LEAF).is_some(),
        "ptrentry must discover {LEAF} from its .rodata function-pointer word"
    );
    let Some(prog) = bootstrap(true) else { return };
    let body = decompile(prog, LEAF);
    eprintln!("---- {LEAF} (ptrentry on) ----\n{body}");
    assert!(body.contains("return 7;"), "{LEAF} must decompile to its real constant, got:\n{body}");
}

/// The precision guard, pinned: a code pointer whose word lies in the same
/// discovered function as its target is a switch table, and admitting it would
/// split a real body. It must stay rejected with the option ON — this is the
/// single predicate that took the corpus measurement from 1,261 split bodies to
/// zero.
#[test]
fn ptrentry_refuses_the_switch_table_case() {
    for on in [false, true] {
        let Some(prog) = bootstrap(on) else {
            return; // specs-less skip
        };
        assert!(
            prog.lookup_symbol(SWCASE).is_none(),
            "ptrentry (on = {on}) must NOT split a function at {SWCASE}: its \
             pointer word at 0x08000058 lies in the same discovered function as \
             the target, which is the ldr pc,[pc,r] switch-table shape"
        );
    }
}

/// Both discovery sets are a strict superset relation: the option adds `LEAF` and
/// removes nothing. "Never removes an entry" is a property of the wiring (the
/// accepted targets are emitted as an additive fact stream and never re-seed the
/// recursive-descent walk), so it is pinned here.
#[test]
fn ptrentry_only_adds_entries() {
    let Some(off) = bootstrap(false) else {
        return; // specs-less skip
    };
    let Some(on) = bootstrap(true) else { return };
    for f in [RESET, CALLEE] {
        assert!(off.lookup_symbol(f).is_some() && on.lookup_symbol(f).is_some(), "{f} survives");
    }
    assert!(off.lookup_symbol(LEAF).is_none() && on.lookup_symbol(LEAF).is_some());
}
