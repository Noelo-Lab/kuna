//! Scalar / operand reference markup — the kuna analog of Ghidra's
//! `ScalarOperandAnalyzer` / `ElfScalarOperandAnalyzer` (and, for the
//! listing-cosmetic half, `OperandReferenceAnalyzer` /
//! `DataOperandReferenceAnalyzer`).
//!
//! ## What this is (the salvageable, faithful subset)
//!
//! Ghidra's operand/reference markup family walks every disassembled instruction
//! operand and creates *listing references* — string / pointer / address-table /
//! subroutine references. Most of that output is **listing cosmetics** (a
//! `ReferenceManager` xref that never reaches a decompiler) or already delivered
//! elsewhere in kuna (subroutine refs → `entry`; jump tables → the engine's
//! `JumpTable::recoverAddresses` switch recovery / `addrtable`; PLT/GOT names →
//! `loader::elf_plt`). See the "covered-elsewhere" map below.
//!
//! The **one** product with genuine decompiler relevance is the
//! `ScalarOperandAnalyzer` idea: a scalar immediate operand that points into an
//! allocated **read-only** data section is an address, so the data it targets
//! should render as a typed object (a string literal) rather than a bare integer.
//! This pass ports that faithful subset:
//!
//! 1. linear-decode every instruction in the executable sections (driving the
//!    same ported SLEIGH engine the [`crate::listing`] tier uses — design §4),
//! 2. for each **constant-space** p-code input (the kuna analog of Ghidra's
//!    `Instruction.getOpObjects(i)` `Scalar` operands — see [`ScalarCapture`]),
//!    apply `ScalarOperandAnalyzer.checkOperands`'s value filter (reject `< 4096`
//!    and the byte-mask values like `0xffff`/`0xff00`),
//! 3. accept the scalar as an address only when it lands inside an **allocated,
//!    read-only** section (`SHF_ALLOC` and **not** `SHF_WRITE` — the `.rodata`
//!    case the decompiler can use; mirrors `program.getMemory().contains(addr)` +
//!    the readonly partition),
//! 4. apply the `ElfScalarOperandAnalyzer` `.got`/`.plt` exclusion (those targets
//!    are already named by [`crate::loader::elf_plt`], so a scalar pointing at
//!    them is never a data reference),
//! 5. emit a [`crate::pass::StringFact`] (a typed `char[N]`) when the target is a
//!    NUL-terminated printable run, plus a `readonly` range over it — reusing the
//!    **existing** strings/readonly commit arms, so the printer's
//!    pointer-to-readonly-char-array literal route (Increment 12) renders the
//!    reference as the string literal.
//!
//! ## Why DEFAULT-OFF (the buildplan §1.2 net-negative)
//!
//! `docs/analysis-port-buildplan.md` §1.2 gives this family the verdict
//! **never-for-an-ELF-decompiler (as producing passes)**, for reasons that all
//! still hold and are why this pass ships gated off behind `--option operand_refs
//! on` (default off):
//!
//! - **ELF-default-off upstream.** `ScalarOperandAnalyzer.getDefaultEnablement`
//!   returns `!ElfLoader.isElf(program)` — Ghidra ships the producing analyzer
//!   **disabled for every ELF**. `ElfScalarOperandAnalyzer` exists only to *remove*
//!   the bad `.got`/`.plt` references its parent would create — a correction of a
//!   bug kuna never has (kuna names `.plt`/`.got` correctly via `elf_plt`).
//! - **The one useful product is already covered.** A `.rodata` string ≥ 5 chars
//!   is already planted as a `char[N]` by the always-on [`crate::strings`] pass,
//!   and the printer renders it as the literal via the SPACEBASE route (Increment
//!   12). Library-call argument typing already types a `char *` argument
//!   ([`crate::protos`] / S5 usage inference). So this pass only adds output for
//!   the *residual* case: a short (< 5 char) or otherwise `strings`-missed
//!   read-only printable run pointed at by a bare immediate whose consuming call
//!   has no prototype — a narrow, low-payoff slice.
//! - **Over-acceptance risk.** A per-instruction immediate scan that types any
//!   in-`.rodata` constant as a pointer over-accepts (a coincidental constant that
//!   happens to land in `.rodata` is not necessarily an address), the same
//!   false-positive shape that keeps [`crate::addrtable`] off by default.
//!
//! So the pass is **ported + flippable** (it exists, is registered, and is
//! exercised by tests + a console e2e gate) but **off by default** — exactly the
//! posture the buildplan prescribes. `--option operand_refs on` enables it.
//!
//! ## Empirical render (verified) + the deferred-run requirement
//!
//! The scalar→string-literal render **does** work (the Increment-12 printer change
//! removed the old shadowing wall): with `--option operand_refs on`, a `movabs
//! $0x402004,%rax` that materializes a `.rodata` string address renders the
//! consuming call as `mystery("hi")` instead of `mystery(0x402004)` (proven by
//! `kuna-console/tests/verify_operand_refs.rs`). It fires only when the address
//! **appears directly in code** as a bare immediate (the `movabs` / large-code-model
//! case) — for a RIP-relative `lea 0xNNN(%rip)` (gcc `-O0` default) the absolute
//! address is a `pc + displacement` computation, not a bare immediate, so no scalar
//! is captured — faithful to Ghidra's `ADDRESSES_DO_NOT_APPEAR_DIRECTLY_IN_CODE`
//! gate (`getDefaultEnablement2`).
//!
//! Like the [`crate::listing`] tier (the PR6 build-timing fix), this pass runs
//! **deferred** — at the commit point (`read symbols`), NOT in the load-time pass
//! list — because it decodes through the engine `Translate` whose program loadimage
//! is only attached (`set_loader`) *after* the load-time passes run. A load-time
//! decode finds no bytes (every `one_instruction` fails). It is therefore driven
//! from `passes::run_operand_refs`, called from the console's
//! `commit_pending_analysis` gated on `analysis_operand_refs`.
//!
//! ## The listing-cosmetic / covered-elsewhere half (documented, not built)
//!
//! `OperandReferenceAnalyzer` / `DataOperandReferenceAnalyzer` additionally create
//! **subroutine** references (it even runs a `PseudoDisassembler` to detect a
//! function start) and **address-table** references. kuna has no commit arm for a
//! bare xref, and these products are delivered by other passes:
//!
//! | Ghidra operand-ref product | kuna source (covered elsewhere) |
//! |---|---|
//! | subroutine reference → create function | [`crate::entry`] (entry discovery) |
//! | jump/address-table reference | engine `JumpTable::recoverAddresses` (S2) + [`crate::addrtable`] |
//! | string reference | [`crate::strings`] + the printer literal route |
//! | `.plt`/`.got` reference (corrective) | [`crate::loader::elf_plt`] names them directly |
//!
//! So those halves are **documented as covered-elsewhere** rather than built as a
//! no-op (faithful to the buildplan: "faithfully DOCUMENT it as covered-elsewhere
//! … rather than building a no-op").
//!
//! ## Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/analysis/{ScalarOperandAnalyzer,ElfScalarOperandAnalyzer,OperandReferenceAnalyzer,DataOperandReferenceAnalyzer}.java`
//! — `ScalarOperandAnalyzer.checkOperands()` (the per-operand `Scalar` loop +
//! value filter), `addReference()` (the in-memory acceptance), `getDefaultEnablement`
//! (`!isElf`); `ElfScalarOperandAnalyzer.addReference()` (the `.got`/`.plt`
//! exclusion).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};
use object::read::{Object, ObjectSection};
use object::SectionKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, StringFact};

// --- Upstream constants (ScalarOperandAnalyzer.java) ---------------------------

/// `ScalarOperandAnalyzer.checkOperands`: a scalar value `< 4096` "could be a
/// number, even if it is in the address space" — rejected as an address. (Same
/// 4096 floor the address-table scanner uses, `AddressTable.MINIMUM_SAFE_ADDRESS`.)
const MIN_ADDRESS_VALUE: u64 = 4096;

/// `ScalarOperandAnalyzer.checkOperands`: the explicit byte-mask values that are
/// "even if in the address space" never addresses (`0xffff`, `0xff00`, …). Ported
/// verbatim from the Java `value == 0xffff || value == 0xff00 || …` guard (plus
/// `0xff`, which the < 4096 floor already rejects but is kept for clarity).
const MASK_VALUES: [u64; 10] = [
    0xffff, 0xff00, 0xffffff, 0xff0000, 0xff00ff, 0xffffffff, 0xffffff00, 0xffff0000, 0xff000000,
    0xff,
];

/// ELF section-header flag `SHF_ALLOC` (the section occupies memory at runtime).
const SHF_ALLOC: u64 = 0x2;
/// ELF section-header flag `SHF_WRITE` (the section is writable at runtime).
const SHF_WRITE: u64 = 0x1;
/// ELF section-header flag `SHF_EXECINSTR` (the section holds executable code).
const SHF_EXECINSTR: u64 = 0x4;

/// The minimum visible run length to plant a `char[N]`. The always-on
/// [`crate::strings`] pass uses 5; this pass is the value-add for the residual
/// shorter / missed runs, so it requires only `>= 1` visible char before the NUL.
const STRING_MIN_LEN: usize = 1;

/// A recovered scalar-operand reference: the instruction at `from` carries a
/// scalar immediate whose value `to` is an address in read-only data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScalarRef {
    /// The referencing instruction's VMA.
    pub from: u64,
    /// The referenced read-only-data address (the scalar's value).
    pub to: u64,
}

/// `[start, end)` half-open section range plus its ELF flags (for the readonly /
/// `.got`/`.plt` partition).
#[derive(Clone, Debug)]
struct SecRange {
    lo: u64,
    hi: u64,
    /// ELF `sh_flags` (or 0 for a non-ELF section; the readonly arms key off the
    /// neutral `SectionKind` then).
    elf_flags: u64,
    kind: SectionKind,
    /// `.got` / `.plt` (and `.got.plt`, `.plt.sec`): the `ElfScalarOperandAnalyzer`
    /// exclusion sections.
    is_got_or_plt: bool,
}

impl SecRange {
    fn contains(&self, a: u64) -> bool {
        a >= self.lo && a < self.hi
    }

    /// Is this an allocated, read-only data section — the `.rodata` partition a
    /// scalar may point at? `SHF_ALLOC` set, `SHF_WRITE` clear, not executable.
    fn is_readonly_data(&self) -> bool {
        if self.elf_flags != 0 {
            // ELF: the authoritative flags. Allocated, not writable, not code.
            return self.elf_flags & SHF_ALLOC != 0
                && self.elf_flags & SHF_WRITE == 0
                && self.elf_flags & SHF_EXECINSTR == 0;
        }
        // Non-ELF fallback: the neutral section kind.
        matches!(self.kind, SectionKind::ReadOnlyData | SectionKind::ReadOnlyString)
    }
}

/// Build the section partition once: every section's `[lo, hi)` + the flags the
/// readonly / `.got`/`.plt` arms need. Mirrors `program.getMemory()` block
/// enumeration in `ScalarOperandAnalyzer.addReference`.
fn section_ranges(file: &object::File) -> Vec<SecRange> {
    let mut out = Vec::new();
    for sec in file.sections() {
        let lo = sec.address();
        let sz = sec.size();
        if sz == 0 {
            continue;
        }
        let elf_flags = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags,
            _ => 0,
        };
        let name = sec.name().unwrap_or("");
        // `.got`, `.got.plt`, `.plt`, `.plt.sec`, `.plt.got`: the
        // `ElfScalarOperandAnalyzer` exclusion set (a scalar that lands here is not
        // a data reference — those targets are already named by `elf_plt`).
        let is_got_or_plt = name.starts_with(".got") || name.starts_with(".plt");
        out.push(SecRange { lo, hi: lo.saturating_add(sz), elf_flags, kind: sec.kind(), is_got_or_plt });
    }
    out
}

/// The executable `[lo, hi)` ranges to linear-decode (`SHF_EXECINSTR` / the neutral
/// `SectionKind::Text`). Mirrors [`crate::listing`]'s exec-range gate.
fn exec_ranges(file: &object::File) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for sec in file.sections() {
        let is_exec = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
            _ => matches!(sec.kind(), SectionKind::Text),
        };
        if !is_exec {
            continue;
        }
        let lo = sec.address();
        let sz = sec.size();
        if sz == 0 {
            continue;
        }
        out.push((lo, lo.saturating_add(sz)));
    }
    out
}

/// A capturing [`PcodeEmit`] that records every **constant-space** input varnode's
/// value for one instruction. These are the scalar immediates the
/// `ScalarOperandAnalyzer` reads as `Instruction.getOpObjects(i)` `Scalar`s — kuna
/// has no separate operand model at this tier, so the constant inputs of the
/// instruction's p-code are the faithful projection (every literal an operand
/// contributes appears as a constant-space varnode in the emitted ops).
#[derive(Default)]
struct ScalarCapture {
    consts: Vec<u64>,
}

impl PcodeEmit for ScalarCapture {
    fn dump(
        &mut self,
        _addr: &Address,
        _opc: OpCode,
        _outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        for v in vars {
            if let Some(space) = &v.space {
                if space.get_type() == spacetype::IPTR_CONSTANT {
                    self.consts.push(v.offset);
                }
            }
        }
    }
}

/// Decode the instruction at `vma` and return `(len, constant_inputs)`. `None` on
/// an undecodable address (the caller's policy is to skip to the next aligned
/// address — a conservative linear sweep, unlike the listing tier's flow-following
/// recursive descent, because this pass needs no flow, only the operand scalars).
fn decode_scalars(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
) -> Option<(u32, Vec<u64>)> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = ScalarCapture::default();
    let len = translate.one_instruction(&mut cap, &addr).ok()?;
    if len <= 0 {
        return None;
    }
    Some((len as u32, cap.consts))
}

/// `ScalarOperandAnalyzer.checkOperands` value filter: a scalar is a candidate
/// address unless it is `< 4096` or one of the byte-mask values.
fn looks_like_address(value: u64) -> bool {
    value >= MIN_ADDRESS_VALUE && !MASK_VALUES.contains(&value)
}

/// If `addr` is the start of a NUL-terminated printable run in `file`'s data,
/// return the byte length **including** the NUL (`char[N]` length), else `None`.
/// Reuses the [`crate::strings`] printable-char recognizer so a planted symbol
/// is shaped identically. Used to emit a [`StringFact`] for the residual short /
/// `strings`-missed strings this pass is the value-add for.
fn readonly_string_at(file: &object::File, addr: u64) -> Option<u32> {
    for sec in file.sections() {
        let lo = sec.address();
        let sz = sec.size();
        if sz == 0 || addr < lo || addr >= lo.saturating_add(sz) {
            continue;
        }
        let data = sec.data().ok()?;
        let off = (addr - lo) as usize;
        let mut len = 0usize;
        let mut i = off;
        while i < data.len() {
            let b = data[i];
            if b == 0 {
                // NUL terminator: a string iff at least one visible char preceded.
                if len >= STRING_MIN_LEN {
                    return Some((len + 1) as u32); // + the trailing NUL
                }
                return None;
            }
            if !is_printable_string_byte(b) {
                return None; // a non-printable, non-NUL byte: not a string
            }
            len += 1;
            i += 1;
        }
        return None; // ran off the section without a NUL
    }
    None
}

/// Mirror of `strings::is_string_char` (`AsciiCharSetRecognizer.contains`):
/// printable ASCII + CR/LF/TAB.
fn is_printable_string_byte(b: u8) -> bool {
    (0x20..=0x7e).contains(&b) || b == 0x0d || b == 0x0a || b == 0x09
}

/// The pure core: linear-decode the executable sections, and for every scalar
/// immediate that is a valid read-only-data address (passing the value filter, the
/// readonly partition test, and the `.got`/`.plt` exclusion), record a
/// [`ScalarRef`]. Shared with the unit tests.
///
/// `translate` + `code_space` drive the SLEIGH decoder; on x86-64 the linear sweep
/// at 1-byte stride realigns after a bad decode (CISC instructions vary in length).
pub fn scan_scalar_refs(
    file: &object::File,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
) -> Vec<ScalarRef> {
    let secs = section_ranges(file);
    let exec = exec_ranges(file);
    let mut out = Vec::new();

    for &(lo, hi) in &exec {
        let mut vma = lo;
        // The (vma, value) pairs already emitted for the CURRENT instruction — a
        // single immediate often appears in several of an instruction's captured
        // p-code ops. Dedup is per-instruction (each `vma` is visited once in the
        // linear sweep), so this stays O(consts) per instruction — no global set.
        let mut emitted_for_insn: Vec<u64> = Vec::new();
        while vma < hi {
            let (len, consts) = match decode_scalars(translate, vma, code_space) {
                Some(r) => r,
                None => {
                    // Undecodable: realign by one byte (CISC linear sweep).
                    vma += 1;
                    continue;
                }
            };
            emitted_for_insn.clear();
            for &value in &consts {
                if !looks_like_address(value) {
                    continue;
                }
                // Must land in an allocated read-only data section — and NOT in the
                // `.got`/`.plt` exclusion (ElfScalarOperandAnalyzer).
                let in_ro = secs
                    .iter()
                    .any(|s| s.contains(value) && s.is_readonly_data() && !s.is_got_or_plt);
                if !in_ro {
                    continue;
                }
                // Dedup an immediate that appears in several captured ops of one insn.
                if emitted_for_insn.contains(&value) {
                    continue;
                }
                emitted_for_insn.push(value);
                out.push(ScalarRef { from: vma, to: value });
            }
            vma += len.max(1) as u64;
        }
    }
    out
}

/// Turn the recovered scalar references into [`AnalysisOutput`] facts: for each
/// referenced read-only address that begins a NUL-terminated printable run, emit a
/// [`StringFact`] (a typed `char[N]`, via the **existing** strings commit arm) + a
/// `readonly` range over it — so the printer renders the reference as the string
/// literal. Targets that are not printable runs are skipped (no type to plant).
/// Pure — the unit tests assert it directly.
fn emit_facts(file: &object::File, refs: &[ScalarRef]) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    let mut planted: Vec<u64> = Vec::new();
    for r in refs {
        if planted.contains(&r.to) {
            continue; // one fact per target address
        }
        if let Some(len) = readonly_string_at(file, r.to) {
            out.strings.push(StringFact { addr: r.to, len });
            out.readonly.push((r.to, r.to + len as u64));
            planted.push(r.to);
        }
    }
    out
}

/// Port of the salvageable `ScalarOperandAnalyzer` / `ElfScalarOperandAnalyzer`
/// subset: type a scalar immediate that points into read-only data as a string
/// literal. **Disabled by default** (see the module docs: ELF-default-off
/// upstream, covered-elsewhere, over-acceptance-prone) — gated behind
/// `--option operand_refs on`.
pub struct OperandRefsPass;

impl AnalysisPass for OperandRefsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "operand_refs"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        // The decode needs the default code space; absent it (no decodable image)
        // the pass is a no-op.
        let code_space = match ctx.arch.manage().get_default_code_space() {
            Some(s) => Rc::clone(s),
            None => return AnalysisOutput::default(),
        };
        let refs = scan_scalar_refs(ctx.file, ctx.arch.translate(), &code_space);
        emit_facts(ctx.file, &refs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_filter_matches_ghidra() {
        // < 4096 is "could be a number" — rejected.
        assert!(!looks_like_address(0));
        assert!(!looks_like_address(1024));
        assert!(!looks_like_address(4095));
        // The explicit byte-mask values are rejected even though > 4096.
        assert!(!looks_like_address(0xffff));
        assert!(!looks_like_address(0xff00));
        assert!(!looks_like_address(0xffffffff));
        assert!(!looks_like_address(0xff0000));
        // A plausible .rodata address is accepted.
        assert!(looks_like_address(0x402010));
        assert!(looks_like_address(0x4096));
    }

    #[test]
    fn readonly_partition_classifies_sections() {
        // .rodata: ALLOC, not WRITE, not EXEC -> readonly data.
        let ro = SecRange {
            lo: 0x402000,
            hi: 0x403000,
            elf_flags: SHF_ALLOC,
            kind: SectionKind::ReadOnlyData,
            is_got_or_plt: false,
        };
        assert!(ro.is_readonly_data());
        // .data: ALLOC + WRITE -> not readonly.
        let rw = SecRange {
            lo: 0x404000,
            hi: 0x405000,
            elf_flags: SHF_ALLOC | SHF_WRITE,
            kind: SectionKind::Data,
            is_got_or_plt: false,
        };
        assert!(!rw.is_readonly_data());
        // .text: ALLOC + EXEC -> not readonly data.
        let code = SecRange {
            lo: 0x401000,
            hi: 0x402000,
            elf_flags: SHF_ALLOC | SHF_EXECINSTR,
            kind: SectionKind::Text,
            is_got_or_plt: false,
        };
        assert!(!code.is_readonly_data());
    }

    #[test]
    fn got_plt_exclusion_is_recognized() {
        // The ElfScalarOperandAnalyzer exclusion: a `.got`/`.plt` section is flagged
        // so a scalar landing there is rejected (those targets are `elf_plt`-named).
        let names_excluded = [".got", ".got.plt", ".plt", ".plt.sec", ".plt.got"];
        for n in names_excluded {
            assert!(
                n.starts_with(".got") || n.starts_with(".plt"),
                "{n} must match the .got/.plt exclusion predicate"
            );
        }
        // .rodata / .data are NOT excluded.
        for n in [".rodata", ".data", ".text", ".bss"] {
            assert!(
                !(n.starts_with(".got") || n.starts_with(".plt")),
                "{n} must not be excluded"
            );
        }
    }

    #[test]
    fn readonly_string_recognizer() {
        // The printable / NUL recognizer (the per-section walk is covered by the
        // e2e fixture gate; here we exercise the byte predicate).
        assert!(is_printable_string_byte(b'h'));
        assert!(is_printable_string_byte(b' '));
        assert!(is_printable_string_byte(b'\t'));
        assert!(!is_printable_string_byte(0));
        assert!(!is_printable_string_byte(0x01));
        assert!(!is_printable_string_byte(0x80));
    }

    #[test]
    fn scan_over_fauxware_finds_only_readonly_targets() {
        // Real x86-64 ELF: the section partition must classify .rodata vs .text/.data
        // distinctly. Building a live Translate needs the SLEIGH specs (not loaded in
        // the kuna-analysis unit tests), so this asserts the pure partition half over
        // the real fixture; the decode+render path is proven by the console e2e gate
        // `verify_operand_refs.rs`.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let secs = section_ranges(&file);
        // fauxware has a .rodata (readonly) and a .data (writable) — the partition
        // must classify them distinctly.
        assert!(
            secs.iter().any(|s| s.is_readonly_data()),
            "fauxware must have at least one readonly-data section"
        );
        // The "Username: " literal @ 0x400915 lands in a readonly section.
        assert!(
            secs.iter().any(|s| s.contains(0x400915) && s.is_readonly_data()),
            "0x400915 (\"Username: \") must be in a readonly-data section"
        );
        // A .text address must NOT classify as readonly data.
        assert!(
            !secs.iter().any(|s| s.contains(0x400720) && s.is_readonly_data() && !s.is_got_or_plt),
            "a .text address must not be a readonly-data target"
        );
    }
}
