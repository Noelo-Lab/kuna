//! End-to-end gate for the Listing/xref tier decode-mode context paint (scope-B
//! PR5): the recursive-descent walker paints the right decode mode (ARM `TMode` /
//! MIPS `ISA_MODE`) into the engine's `ContextDatabase` BEFORE it decodes, so an
//! alternate-ISA (Thumb / MIPS16) function decodes correctly instead of being
//! misread as A32 / MIPS32 garbage.
//!
//! ## What this proves (and how it is a real proof)
//!
//! `Listing::build`'s walker drives `Translate::one_instruction` directly —
//! *outside* the decompiler's normal `commit_analysis_output` paint path — so it
//! must paint the decode mode itself (`listing/context.rs`, reusing the
//! `arm_markers` / `mips_markers` marker logic). Without that paint, the same byte
//! stream misdecodes:
//!
//!  - **ARM Thumb.** `compute`'s entry (`0x100b8`) is the 2-byte Thumb op
//!    `push {r7}` (`80 b4`). Decoded in Thumb mode it has length **2**; an A32
//!    misdecode reads a fixed **4**-byte word (`b0 83 80 b4` LE) — a different,
//!    garbage instruction. The width alone is decisive (A32 is *always* 4 bytes),
//!    and we also confirm a Thumb mnemonic and the multiply/shift body.
//!  - **MIPS16.** `m16_square`'s entry (`0x400130`) is the 2-byte MIPS16 op
//!    `mult a0,a0` (`ec 98`). Decoded as MIPS16 it has length **2**; a MIPS32
//!    misdecode reads a fixed **4**-byte word. Same width-based proof.
//!
//! Each case also runs a **control**: the SAME seed decoded through a fresh
//! bootstrap's raw `Translate` (no Listing, no paint) gets the un-painted default
//! decode — and we assert the Listing's painted decode DIFFERS from it (so the
//! paint, not some incidental default, is what flips the ISA). x86-64 needs no
//! decode-mode context, so the `verify_listing_core` gate (which never paints)
//! still passes unchanged — this gate adds the ARM/MIPS half.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built ARM (and MIPS) `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_analysis::listing::Listing;
use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn spec_roots() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

/// The vendored LINKED ARM Thumb fixture (`compute` is `x*3 + 7`, a Thumb FUNC at
/// the odd `entry|1` address; `$t` marks the Thumb run start). Shared with
/// `verify_arm_thumb_decode`.
fn arm_thumb_linked() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_linked_le32")
}

/// The vendored freestanding MIPS16 fixture (`m16_square` is `n*n + 3`, marked
/// MIPS16 via `st_other & 0xf0 == STO_MIPS_MIPS16`). Shared with
/// `verify_mips16_isa`.
fn mips16() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/mips16_le32")
}

// ARM Thumb load-bearing VMAs (`readelf --syms` / `.text` dump at fixture-build
// time): `$t` + `compute`'s even decode entry @ 0x100b8 (Thumb), `_start`'s even
// entry @ 0x100d6. `compute` starts with the 2-byte Thumb `push {r7}` (`80 b4`).
const COMPUTE_EVEN_ENTRY: u64 = 0x100b8;

// MIPS16 load-bearing VMAs: `m16_square`'s entry @ 0x400130 (MIPS16, 8-byte body
// `ec98 ea12 e820 4a03`), starting with the 2-byte MIPS16 `mult a0,a0` (`ec 98`).
const M16_SQUARE_ENTRY: u64 = 0x400130;

/// Bootstrap a fixture for a real `Translate`, or return `None` (a visible skip)
/// if the `.sla` is absent.
fn boot(bin: &str, who: &str) -> Option<ConsoleProgram> {
    match bootstrap_from_object(bin, "", &spec_roots()) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("{who}: skipping (bootstrap failed, build the `.sla` with `make specs`): {}", e.explain());
            None
        }
    }
}

/// Decode one instruction at `vma` through the engine's raw `Translate` (no
/// Listing, no context paint) — the un-painted DEFAULT-mode decode used as the
/// "before"/control. Returns `Some((len, mnemonic))`, or `None` if the default
/// ISA cannot decode the bytes at all (itself proof the bytes are not valid in
/// the default mode — e.g. MIPS16 bytes read as MIPS32).
fn raw_decode(prog: &ConsoleProgram, vma: u64) -> Option<(u32, String)> {
    let arch = prog.arch();
    let translate = arch.translate();
    let code_space = Rc::clone(arch.manage().get_default_code_space().expect("code space"));
    let addr = Address::new(code_space, vma);

    struct Sink;
    impl PcodeEmit for Sink {
        fn dump(
            &mut self,
            _a: &Address,
            _o: kuna_num::opcodes::OpCode,
            _ov: Option<&kuna_num::pcoderaw::VarnodeData>,
            _v: &[kuna_num::pcoderaw::VarnodeData],
        ) {
        }
    }
    #[derive(Default)]
    struct Asm {
        m: String,
    }
    impl AssemblyEmit for Asm {
        fn dump(&mut self, _a: &Address, mnem: &str, _b: &str) {
            self.m = mnem.to_string();
        }
    }

    let mut sink = Sink;
    // The default-ISA decode may legitimately FAIL on alt-ISA bytes (e.g. MIPS16
    // bytes are not a valid MIPS32 word) — that is a `None`, the strongest control.
    let len = translate.one_instruction(&mut sink, &addr).ok()?;
    let mut asm = Asm::default();
    let _ = translate.print_assembly(&mut asm, &addr);
    Some((len.max(0) as u32, asm.m))
}

/// Build a Listing seeded at `seed` over `bin` (already bootstrapped as `prog`).
fn build_listing(prog: &ConsoleProgram, bin: &str, seed: u64) -> Listing {
    let bytes = std::fs::read(bin).expect("read fixture bytes");
    let file = object::File::parse(&*bytes).expect("parse fixture ELF");
    let arch = prog.arch();
    let translate = arch.translate();
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(bin, &bytes)
        .expect("open loadimage");
    Listing::build(&file, &image, arch, translate, &[seed])
}

/// ARM Thumb: the walker paints `TMode=1` at `compute`'s even entry (reusing the
/// `arm_markers` logic), so the Listing decodes it as Thumb. Proof: the seed
/// instruction has Thumb width (2 bytes, not A32's fixed 4), a Thumb mnemonic, and
/// the painted decode DIFFERS from the un-painted A32 control.
#[test]
fn arm_thumb_seed_decodes_in_thumb_mode_in_listing() {
    let bin = match arm_thumb_linked().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };
    let Some(prog) = boot(&bin, "verify_listing_context(arm)") else { return };

    // Control: the un-painted (default A32) decode of `compute`'s entry. Built on a
    // FRESH bootstrap so no Listing paint has touched its ContextDatabase. The
    // Thumb `push {r7}` bytes form a valid (but different) 4-byte A32 word.
    let prog_ctrl = boot(&bin, "verify_listing_context(arm-control)").expect("re-bootstrap");
    let (a32_len, a32_mnem) = raw_decode(&prog_ctrl, COMPUTE_EVEN_ENTRY)
        .expect("the A32 control decodes the bytes as a 4-byte word");
    // A32 is always a 4-byte word.
    assert_eq!(a32_len, 4, "the un-painted control must decode `compute` as a fixed 4-byte A32 word");

    // The Listing builds with the context paint: `compute` decodes as Thumb.
    let listing = build_listing(&prog, &bin, COMPUTE_EVEN_ENTRY);

    let seed = listing
        .instruction_at(COMPUTE_EVEN_ENTRY)
        .unwrap_or_else(|| panic!("the Listing decoded no instruction at compute ({COMPUTE_EVEN_ENTRY:#x})"));

    eprintln!(
        "verify_listing_context(arm): seed {:#x} thumb=`{}`(len {}) vs control a32=`{}`(len {})",
        COMPUTE_EVEN_ENTRY, seed.mnemonic, seed.len, a32_mnem, a32_len
    );

    // (1) Width proof: a Thumb op at the entry is 2 bytes; A32 is fixed 4.
    assert_eq!(
        seed.len, 2,
        "the painted Thumb decode of `compute`'s `push {{r7}}` must be 2 bytes (Thumb), not {} \
         (an A32 misdecode reads a fixed 4-byte word)",
        seed.len
    );

    // (2) The painted decode DIFFERS from the un-painted A32 control — the paint
    //     (not some incidental default) is what flips the ISA.
    assert!(
        seed.len != a32_len || seed.mnemonic != a32_mnem,
        "the painted Thumb decode must differ from the un-painted A32 control \
         (thumb len={}/mnem=`{}` vs a32 len={}/mnem=`{}`)",
        seed.len, seed.mnemonic, a32_len, a32_mnem
    );

    // (3) Sane Thumb mnemonic: `compute` opens with `push {r7}`.
    let m = seed.mnemonic.to_lowercase();
    assert!(
        m.contains("push"),
        "the Thumb-decoded entry of `compute` must be a `push` (got `{}`)",
        seed.mnemonic
    );

    // (4) The whole small function decodes cleanly as Thumb: every instruction in
    //     `compute`'s body is 2 or 4 bytes (the Thumb widths), and the body carries
    //     the `*3` shift/add — recover at least one shift mnemonic (`lsl`).
    let body: Vec<_> = listing
        .instructions()
        .filter(|(&a, _)| (COMPUTE_EVEN_ENTRY..COMPUTE_EVEN_ENTRY + 30).contains(&a))
        .collect();
    assert!(body.len() >= 3, "expected several Thumb instructions in `compute`, got {}", body.len());
    for (&a, insn) in &body {
        assert!(
            insn.len == 2 || insn.len == 4,
            "every Thumb instruction is 2 or 4 bytes; insn at {a:#x} has len {}",
            insn.len
        );
    }
    let has_shift = body.iter().any(|(_, i)| {
        let m = i.mnemonic.to_lowercase();
        m.contains("lsl") || m.contains("lsr") || m.contains("asr")
    });
    assert!(
        has_shift,
        "the Thumb decode of `x*3+7` must include a shift (the `*3 = (x<<1)+x`); body = {:#x?}",
        body.iter().map(|(a, i)| (**a, i.mnemonic.clone())).collect::<Vec<_>>()
    );
}

/// MIPS16: the walker paints `ISA_MODE=1` at `m16_square`'s entry (reusing the
/// `mips_markers` ISA-mode logic), so the Listing decodes it as MIPS16. Proof: the
/// seed instruction has MIPS16 width (2 bytes, not MIPS32's fixed 4) and the
/// painted decode DIFFERS from the un-painted MIPS32 control.
#[test]
fn mips16_seed_decodes_in_mips16_mode_in_listing() {
    let bin = match mips16().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };
    let Some(prog) = boot(&bin, "verify_listing_context(mips16)") else { return };

    // Control: the un-painted (default MIPS32) decode of `m16_square`'s entry. The
    // MIPS16 bytes are not a valid MIPS32 word, so this may legitimately fail to
    // resolve a constructor (`None`) — the strongest control: the bytes simply are
    // not MIPS32. If it does resolve, it reads a fixed 4-byte word.
    let prog_ctrl = boot(&bin, "verify_listing_context(mips16-control)").expect("re-bootstrap");
    let m32 = raw_decode(&prog_ctrl, M16_SQUARE_ENTRY);

    // The Listing builds with the context paint: `m16_square` decodes as MIPS16.
    let listing = build_listing(&prog, &bin, M16_SQUARE_ENTRY);

    let seed = listing
        .instruction_at(M16_SQUARE_ENTRY)
        .unwrap_or_else(|| panic!("the Listing decoded no instruction at m16_square ({M16_SQUARE_ENTRY:#x})"));

    eprintln!(
        "verify_listing_context(mips16): seed {:#x} mips16=`{}`(len {}) vs control mips32={:?}",
        M16_SQUARE_ENTRY, seed.mnemonic, seed.len, m32
    );

    // (1) Width proof: a MIPS16 op at the entry is 2 bytes; MIPS32 is fixed 4.
    assert_eq!(
        seed.len, 2,
        "the painted MIPS16 decode of `m16_square`'s `mult a0,a0` must be 2 bytes (MIPS16), not {} \
         (a MIPS32 misdecode reads a fixed 4-byte word, or fails to decode at all)",
        seed.len
    );

    // (2) The painted MIPS16 decode DIFFERS from the un-painted MIPS32 control: the
    //     control either fails to decode the bytes (the bytes are not MIPS32) or
    //     yields a different (4-byte) word. Either way the paint flipped the ISA.
    match m32 {
        None => { /* MIPS32 can't decode these bytes at all — decisive. */ }
        Some((m32_len, m32_mnem)) => assert!(
            seed.len != m32_len || seed.mnemonic != m32_mnem,
            "the painted MIPS16 decode must differ from the un-painted MIPS32 control \
             (mips16 len={}/mnem=`{}` vs mips32 len={}/mnem=`{}`)",
            seed.len, seed.mnemonic, m32_len, m32_mnem
        ),
    }

    // (3) The 8-byte MIPS16 body decodes as several MIPS16 ops (the hallmark of the
    //     alternate ISA; MIPS32 would read it as two 4-byte words or fail). The
    //     entry `mult` and the `mflo` are 2-byte MIPS16 ops; the `jr ra` is 4 bytes
    //     because MIPS16 folds its delay slot (`addiu v0,3`) into the length
    //     (design §4.3 gotcha 3) — both are MIPS16 widths, never a stray odd or
    //     >4-byte length.
    let body: Vec<_> = listing
        .instructions()
        .filter(|(&a, _)| (M16_SQUARE_ENTRY..M16_SQUARE_ENTRY + 8).contains(&a))
        .collect();
    assert!(
        body.iter().all(|(_, i)| i.len == 2 || i.len == 4),
        "MIPS16 ops are 2 bytes (4 with a folded delay slot); `m16_square`'s body = {:#x?}",
        body.iter().map(|(a, i)| (**a, i.len, i.mnemonic.clone())).collect::<Vec<_>>()
    );
    // The seed op is decisively a 2-byte MIPS16 `mult` (asserted above); the body
    // begins with a self-multiply (the `n*n`), a MIPS16-only shape.
    assert!(
        body.iter().any(|(_, i)| i.mnemonic.to_lowercase().contains("mult")),
        "the MIPS16 decode of `n*n+3` must include a `mult`; body = {:#x?}",
        body.iter().map(|(a, i)| (**a, i.mnemonic.clone())).collect::<Vec<_>>()
    );
}
