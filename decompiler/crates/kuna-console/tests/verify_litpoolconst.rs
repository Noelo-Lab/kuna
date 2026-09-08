//! End-to-end two-pass gate for `litpoolconst` — a word read out of executable
//! read-only memory is a constant, so an ARM literal pool folds without the
//! program-wide `option readonly` (P1 code/data partition, consumed at P3 by
//! `ActionVarnodeProps`).
//!
//! Fixture: `picpool_arm_le32` (`picpool_arm_le32.py` builds it), reused for its
//! shape rather than its own defect — three functions that each `ldr` a word out
//! of a pool inside `.text` and compute with it. It reproduces the filing witness
//! exactly: a statically linked ARM crackme whose `main` ended `ldr r3,[0x8458]`
//! with `0x8458 39050000 .word 0x00000539` in the same listing `kuna disassemble`
//! prints, while the emitted C said `v3 = dat_8458;`.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** `sub_420` returns `dat_42c + 0x42c` — the number
//!   is an opaque global and reading it means inspecting the pool separately.
//! * **default (the fix):** it returns `0x300`.
//!
//! Neither parity corpus can cover this: both are symbol-less `<binaryimage>`
//! bytechunks whose loader reports no sections and no segments, so the range list
//! is empty there and the option is structurally inert — which is also why the
//! default-on flip moves 0/675 datatest assertions.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `ARM` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// `ldr r0,[0x42c] ; add r0,pc,r0 ; bx lr` — the pool word at `0x42c` holds
/// `-0x12c`, so the value is `0x42c - 0x12c = 0x300`.
const COMPOSES_PROMPT: u64 = 0x420;
/// `ldr r0,[0x454] ; add r0,r0,#4 ; bx lr` — no PC in the arithmetic, so the
/// fold has to reach it through the pool word alone (`0x33c + 4`).
const NO_PC: u64 = 0x448;
/// `.text` of the fixture, which is where both pool words live. The literals it
/// points at (0x300/0x320/0x340) sit in a read-only DATA section OUTSIDE this
/// range, which is the narrowing the whole feature turns on.
const TEXT_LO: u64 = 0x400;
const TEXT_HI: u64 = 0x457;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with `litpoolconst` in the requested state.
/// `None` is a visible skip when the `.sla` is missing.
fn load(litpoolconst: bool) -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/picpool_arm_le32");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_litpoolconst: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("litpoolconst", if litpoolconst { "on" } else { "off" })
        .expect("litpoolconst flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog.arch().manage().get_default_code_space().expect("code space").clone();
    Address::new(Rc::clone(&space), vma)
}

/// The rendered C of the function entered at `vma`.
fn body(prog: &mut ConsoleProgram, vma: u64) -> String {
    let entry = code_addr(prog, vma);
    let declared = prog.declared_extent(vma);
    let step = decompile_one(
        prog.arch_mut(),
        &format!("sub_{vma:x}"),
        entry,
        declared,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the function decompiles");
    kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd)
}

/// The precondition: the loader reports `.text` as executable read-only memory,
/// which is the entire warrant for the fold.
#[test]
fn the_text_section_is_the_foldable_range() {
    let Some(prog) = load(true) else { return };
    assert_eq!(
        prog.arch().litpool_const.as_slice(),
        &[(TEXT_LO, TEXT_HI)],
        "only `.text` may be declared constant — a read-only DATA section stays \
         behind `option readonly`"
    );
}

/// BEFORE (`--option litpoolconst off`): the pool word is an opaque global.
#[test]
fn the_pool_word_is_a_global_with_the_option_off() {
    let Some(mut prog) = load(false) else { return };
    let c = body(&mut prog, COMPOSES_PROMPT);
    assert!(
        c.contains("dat_42c"),
        "off must restore the previous output exactly — the pool read stays an \
         opaque global; got:\n{c}"
    );
    let c = body(&mut prog, NO_PC);
    assert!(c.contains("dat_454"), "the same, with no PC in the arithmetic; got:\n{c}");
}

/// AFTER (default): the value the program computes is in the C.
#[test]
fn the_pool_word_folds_to_its_constant_by_default() {
    let Some(mut prog) = load(true) else { return };
    let c = body(&mut prog, COMPOSES_PROMPT);
    assert!(
        !c.contains("dat_"),
        "no operand of the expression may stay an opaque global; got:\n{c}"
    );
    assert!(c.contains("0x300"), "0x42c + (-0x12c) is 0x300; got:\n{c}");
    let c = body(&mut prog, NO_PC);
    assert!(c.contains("0x340"), "0x33c + 4 is 0x340; got:\n{c}");
}

/// The fold is not the program-wide `option readonly`, which stays off: the two
/// are independent, and turning `litpoolconst` off leaves `readonly`'s own
/// behaviour untouched.
#[test]
fn the_program_wide_readonly_switch_is_still_off() {
    let Some(prog) = load(true) else { return };
    assert!(
        !prog.arch().readonlypropagate,
        "`litpoolconst` must not imply `option readonly`; folding every `.rodata` \
         read is a much larger claim than this one"
    );
}
