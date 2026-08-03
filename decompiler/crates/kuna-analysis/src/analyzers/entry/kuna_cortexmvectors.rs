//! (kuna) `cortexmvectors` — the widened ARM Cortex-M hardware vector-table
//! signature (P1 code/data partition, oracle 6).
//!
//! The shipped signature ([`super::cortexm_vector_table_shipped`]) confirms a table
//! only when it starts a section the *loader* maps executable, its word 0 lands in
//! the architectural SRAM block (`0x2000_0000..=0x3FFF_FFFF`), and its word 1 equals
//! `e_entry`. Measured over the decbench ARM Cortex-M corpus, that signature
//! silently no-ops on the four projects that contribute most of the entry-recall
//! gap:
//!
//! | project | table | why the shipped signature rejects it |
//! |---|---|---|
//! | cleanflight | `.isr_vector` @ `0x0800_0000`, `A`-only, read-only `PT_LOAD` | not loaded executable **and** stack word `0x1000_fff0` (STM32F4 CCM RAM) |
//! | betaflight | `.isr_vector` @ `0x2000_0000` (linked into RAM for runtime relocation), `A`-only, read-only `PT_LOAD` | same two |
//! | crazyflie | `.isr_vector` @ `0x0800_4000` | reset word `0x0801_a811` != `e_entry` `0x0800_41c0` |
//! | nuttx | `.text` @ `0x0800_0000` | reset word `0x0800_0a95` != `e_entry` `0x0800_0189` |
//!
//! All three predicates are category errors. The table is **data the CPU
//! reads** — what has to be executable is what its slots point at, which the
//! harvest already checks — so an `A`-only section in a read-only load is a
//! perfectly ordinary place to link it. STM32F4 (CCM) and `-M7` (TCM) parts put
//! the initial stack at `0x1000_0000`, below the architectural SRAM block. And
//! `e_entry` is the *ELF*'s idea of the start symbol, which a bare-metal link
//! script is free to point somewhere other than the reset vector (nuttx points it
//! at `__start`, crazyflie at the `.text` base).
//!
//! So the widened signature drops all three and leans on the one thing that is
//! genuinely load-bearing: a Cortex-M table is a **run of Thumb handler
//! pointers**. A candidate is confirmed when
//!
//! 1. it starts an *allocated* section (any `SHF_ALLOC`; executable or not),
//! 2. word 0 is a plausible initial stack pointer — SRAM, CCM or TCM, i.e.
//!    `0x1000_0000..=0x3FFF_FFFF`, and
//! 3. the slots from word 1 on yield at least [`MIN_CONFORMING_HANDLERS`] Thumb
//!    handler pointers (odd, inside an executable section) before the run ends.
//!
//! Requirement 3 is what replaces `e_entry`: two conforming words can occur by
//! chance in a `.data` struct, three consecutive ones essentially cannot. It is
//! computed by the *same* harvest loop the oracle then seeds from
//! ([`super::harvest_vector_slots`]), so confirmation and harvest can never
//! disagree.
//!
//! **Monotonic by construction.** [`vector_table`] tries the shipped signature
//! first and only falls back to the widened scan, so every image that resolved a
//! table before resolves the same section, at the same address, with the same
//! harvest. The option can add discovered entries; it can never remove one.
//!
//! ## Wiring, and why the Listing tier is deliberately left alone
//!
//! The widened table ships as its own [`CortexMVectorsPass`] (`id() ==
//! "cortexmvectors"`), NOT as a widening of the always-on `entry_disc` pass: a
//! load-time pass runs before `--option` is applied, so a flag read inside
//! `EntryDiscoveryPass::run` would silently do nothing. The stash-at-load,
//! gate-at-commit shape (`engine.rs::analysis_pass_enabled`) is what makes an
//! output-changing discovery flag observable at all — exactly how `eh_frame_full`
//! and `funcstart_patterns` are wired.
//!
//! The pass emits **entry facts and context paints only**. It deliberately does
//! NOT feed the widened handlers to the Listing tier's recursive-descent walk
//! (`passes::listing_seeds`) nor mirror its region paint into the walk's own
//! `ContextPainter`. Both were tried and measured over the 16-binary Cortex-M
//! corpus (`docs/features/cortexmvectors/analysis.md` §4):
//!
//! | wiring | ground-truth entries recovered | entries LOST |
//! |---|---|---|
//! | entries + paints only | **+96** | **0** |
//! | + the walk's region paint | +86 | 10 |
//! | + walk seeds as well | +44 | 54 |
//!
//! The walk treats an unconditional `B` as same-function flow, so seeding an ISR
//! stub that tail-calls a shared handler makes the walk absorb that handler; its
//! own entry then fails `code_pointer_table_seeds`' `is_undefined` guard and the
//! AIF gap scan, and is dropped. Handing the walk more seeds therefore costs more
//! than it gains until the tail-call split lands. Keeping the pass purely
//! additive also makes "this option never removes an entry" a property of the
//! wiring, not of a heuristic.

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{
    executable_sections, harvest_vector_slots, in_executable_section, read_u32, CORTEXM_SRAM_HI,
    CORTEXM_SRAM_LO,
};

/// (kuna) The widened Cortex-M vector-table discovery pass (`cortexmvectors`).
///
/// Emits the reset/exception/IRQ handler pointers of a table the shipped
/// signature rejects, plus the whole-image Thumb (`TMode=1`) region paint that a
/// confirmed table unlocks. Like every other analysis pass it RUNS at load; the
/// **commit** is gated by `--option cortexmvectors on`
/// (`engine.rs::analysis_pass_enabled` → `arch.analysis_cortexmvectors`), so a
/// default run computes the facts and commits nothing, leaving the discovery set
/// byte-identical to the shipped oracle.
///
/// Emits nothing at all when the shipped signature already confirmed the table
/// (its facts would duplicate `entry_disc`'s), on a non-ARM object, and on any
/// ARM object without the widened signature — so it is a strict no-op outside
/// bare-metal Cortex-M firmware whose table the shipped oracle misses.
pub struct CortexMVectorsPass;

impl AnalysisPass for CortexMVectorsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "cortexmvectors"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let file = ctx.file;
        // Nothing to add when the shipped signature already found the table (the
        // always-on `entry_disc` pass emitted these exact facts), nor when the
        // widened scan finds none either.
        if super::cortexm_vector_table_shipped(file).is_some() || widened_scan(file).is_none() {
            return out;
        }
        // The same exec-section / funcsym-skip filtering `collect_entries` applies
        // to every oracle, so this pass cannot introduce an entry the always-on
        // path would have rejected.
        let execs = executable_sections(file);
        let funcsyms = super::existing_function_addrs(file, ctx.bytes);
        let mut entries: Vec<u64> = super::cortexm_vector_entries(file, true)
            .into_iter()
            .filter(|&vma| {
                vma != 0
                    && in_executable_section(&execs, vma)
                    && funcsyms.binary_search(&vma).is_err()
            })
            .collect();
        entries.sort_unstable();
        entries.dedup();
        out.entries = entries;
        out.context_paints = super::cortexm_thumb_paints(file, true);
        out
    }
}

/// Cortex-M tightly-coupled / core-coupled RAM: the `0x1000_0000` block STM32F4
/// (CCM) and the `-M7` TCM parts place a stack in. Contiguous with the
/// architectural SRAM block below it, so together the two cover
/// `0x1000_0000..=0x3FFF_FFFF`.
const CORTEXM_TCM_LO: u64 = 0x1000_0000;
const CORTEXM_TCM_HI: u64 = 0x1FFF_FFFF;

/// How many Thumb handler pointers a candidate must yield before it counts as a
/// vector table. Three consecutive `{zero | odd in-exec}` slots is the smallest
/// run that a `.data` structure does not reach by accident (measured: no
/// allocated section in the 16-binary Cortex-M corpus reaches it except the real
/// table).
const MIN_CONFORMING_HANDLERS: usize = 3;

/// ELF section header flag: SHF_ALLOC (the section occupies memory at run time).
const SHF_ALLOC: u64 = 0x2;

/// The vector-table candidate for `file`, widened per this module's doc when
/// `widen` is set.
///
/// `widen == false` is the shipped signature verbatim. `widen == true` keeps that
/// result when it exists and otherwise runs the widened scan, so the widened
/// answer is always a superset.
pub(super) fn vector_table(file: &object::File, widen: bool) -> Option<(u64, u64, Vec<u8>)> {
    let shipped = super::cortexm_vector_table_shipped(file);
    if !widen || shipped.is_some() {
        return shipped;
    }
    widened_scan(file)
}

/// The widened scan: every allocated section, in the shipped candidate order
/// (`SHF_EXECINSTR` first, then the program-header-executable delta, then the
/// remaining allocated sections), confirmed by [`is_vector_table`].
///
/// ARM-gated, like the shipped signature — the whole oracle is Cortex-M
/// firmware discovery and must stay a strict no-op on every other architecture.
fn widened_scan(file: &object::File) -> Option<(u64, u64, Vec<u8>)> {
    use object::read::Object;
    if file.architecture() != object::Architecture::Arm {
        return None;
    }
    let le = file.is_little_endian();
    let execs = executable_sections(file);
    let in_exec = |vma: u64| in_executable_section(&execs, vma);
    super::executable_sections(file)
        .into_iter()
        .chain(super::phdr_executable_sections(file))
        .chain(other_allocated_sections(file))
        .find(|(addr, _, data)| is_vector_table(*addr, data, le, &in_exec))
}

/// The allocated sections neither [`super::executable_sections`] nor
/// [`super::phdr_executable_sections`] returns — the `A`-only sections in a
/// non-executable load, which is where cleanflight and betaflight put
/// `.isr_vector`.
///
/// ELF-only by construction (a PE/Mach-O `SectionFlags` never matches).
fn other_allocated_sections(file: &object::File) -> Vec<(u64, u64, Vec<u8>)> {
    use object::read::{Object, ObjectSection};
    let already: Vec<u64> = super::executable_sections(file)
        .into_iter()
        .chain(super::phdr_executable_sections(file))
        .map(|(addr, _, _)| addr)
        .collect();
    let mut out = Vec::new();
    for sec in file.sections() {
        let object::SectionFlags::Elf { sh_flags } = sec.flags() else {
            continue;
        };
        if sh_flags & SHF_ALLOC == 0 {
            continue;
        }
        let addr = sec.address();
        let size = sec.size();
        if size == 0 || already.contains(&addr) {
            continue;
        }
        let Ok(data) = sec.data() else {
            continue;
        };
        out.push((addr, addr.saturating_add(size), data.to_vec()));
    }
    out
}

/// The widened signature over one candidate's bytes: word 0 is a plausible
/// initial stack pointer (SRAM, CCM or TCM), and the harvest starting at word 1
/// yields at least [`MIN_CONFORMING_HANDLERS`] Thumb handler pointers.
///
/// Reusing [`super::harvest_vector_slots`] for the confirmation is deliberate:
/// the words that confirm the table are exactly the words the oracle then seeds
/// from, so a candidate can never be confirmed on evidence the harvest rejects.
/// The count is over accepted *slots*, not distinct addresses — a bare-metal
/// table points most of its vectors at one shared `Default_Handler` (nuttx's
/// first fourteen slots resolve to two addresses), so counting addresses would
/// reject the very tables this exists to find.
pub(super) fn is_vector_table(
    sec_addr: u64,
    data: &[u8],
    le: bool,
    in_exec: &dyn Fn(u64) -> bool,
) -> bool {
    if data.len() < 8 {
        return false;
    }
    let sp = read_u32(&data[0..], le) as u64;
    let plausible_sp = (CORTEXM_TCM_LO..=CORTEXM_TCM_HI).contains(&sp)
        || (CORTEXM_SRAM_LO..=CORTEXM_SRAM_HI).contains(&sp);
    if !plausible_sp {
        return false;
    }
    harvest_vector_slots(sec_addr, data, le, in_exec, true).len() >= MIN_CONFORMING_HANDLERS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pass::ContextPaint;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    /// Hand-assemble a bare-metal Cortex-M ELF32: a `.text` at `text_vma` holding
    /// `n_bodies` two-halfword Thumb bodies, and a table section (`table_flags`
    /// `sh_flags`) at `table_vma` carrying `words`, inside two `PT_LOAD`s whose
    /// flags are `table_pf` / `PF_R|PF_X`. `e_entry` and `machine` are explicit so
    /// the `word[1] == e_entry` and ARM-gate predicates can each be steered.
    ///
    /// Assembled here because no cross toolchain on this host emits a bare-metal
    /// STM32 link layout; mirrors `tests/fixtures/cortexm_ccm_vectors_le32.py`.
    #[allow(clippy::too_many_arguments)]
    fn build(
        table_vma: u32,
        table_flags: u32,
        table_pf: u32,
        words: &[u32],
        text_vma: u32,
        n_bodies: usize,
        e_entry: u32,
        machine: u16,
    ) -> Vec<u8> {
        const EHDR: u32 = 52;
        const PHDR: u32 = 32;
        const SHDR: u32 = 40;
        let table: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        // `movs r0,#k ; bx lr` per body (Thumb-1: 0x20kk, 0x4770).
        let text: Vec<u8> = (0..n_bodies).flat_map(|k| [k as u8, 0x20u8, 0x70, 0x47]).collect();
        let ph_off = EHDR;
        let table_off = ph_off + PHDR * 2;
        let text_off = table_off + table.len() as u32;

        let mut shstr = vec![0u8];
        let mut name_of = |n: &str| {
            let off = shstr.len() as u32;
            shstr.extend_from_slice(n.as_bytes());
            shstr.push(0);
            off
        };
        let n_shstrtab = name_of(".shstrtab");
        let n_table = name_of(".isr_vector");
        let n_text = name_of(".text");
        let shstr_off = text_off + text.len() as u32;
        let sh_off = shstr_off + shstr.len() as u32;

        let mut b: Vec<u8> = Vec::new();
        b.extend_from_slice(&[0x7f, b'E', b'L', b'F', 1, 1, 1]);
        b.extend_from_slice(&[0u8; 9]);
        b.extend_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
        b.extend_from_slice(&machine.to_le_bytes());
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&e_entry.to_le_bytes());
        b.extend_from_slice(&ph_off.to_le_bytes());
        b.extend_from_slice(&sh_off.to_le_bytes());
        b.extend_from_slice(&0x0500_0200u32.to_le_bytes()); // e_flags: EABI5
        b.extend_from_slice(&(EHDR as u16).to_le_bytes());
        b.extend_from_slice(&(PHDR as u16).to_le_bytes());
        b.extend_from_slice(&2u16.to_le_bytes()); // e_phnum
        b.extend_from_slice(&(SHDR as u16).to_le_bytes());
        b.extend_from_slice(&4u16.to_le_bytes()); // e_shnum
        b.extend_from_slice(&3u16.to_le_bytes()); // e_shstrndx

        let phdr = |b: &mut Vec<u8>, off: u32, vaddr: u32, size: u32, flags: u32| {
            for v in [1u32, off, vaddr, vaddr, size, size, flags, 4] {
                b.extend_from_slice(&v.to_le_bytes());
            }
        };
        phdr(&mut b, table_off, table_vma, table.len() as u32, table_pf);
        phdr(&mut b, text_off, text_vma, text.len() as u32, 0x4 | 0x1);
        assert_eq!(b.len() as u32, table_off);
        b.extend_from_slice(&table);
        b.extend_from_slice(&text);
        b.extend_from_slice(&shstr);

        let shdr = |b: &mut Vec<u8>, name: u32, ty: u32, fl: u32, addr: u32, off: u32, sz: u32| {
            for v in [name, ty, fl, addr, off, sz, 0, 0, 4, 0] {
                b.extend_from_slice(&v.to_le_bytes());
            }
        };
        assert_eq!(b.len() as u32, sh_off);
        shdr(&mut b, 0, 0, 0, 0, 0, 0);
        shdr(&mut b, n_table, 1, table_flags, table_vma, table_off, table.len() as u32);
        shdr(&mut b, n_text, 1, 0x2 | 0x4, text_vma, text_off, text.len() as u32);
        shdr(&mut b, n_shstrtab, 3, 0, 0, shstr_off, shstr.len() as u32);
        b
    }

    /// The shape every corpus table opens with: an initial SP, a reset vector and
    /// three handler pointers into `text_vma`, then reserved zeros.
    fn handler_words(sp: u32, text_vma: u32) -> Vec<u32> {
        vec![sp, text_vma | 1, (text_vma + 4) | 1, (text_vma + 8) | 1, (text_vma + 12) | 1, 0, 0, 0]
    }

    const EM_ARM: u16 = 40;
    const EM_X86_64: u16 = 62;

    /// cleanflight / betaflight: `.isr_vector` is `A`-only inside a **read-only**
    /// `PT_LOAD` and its stack word is STM32F4 CCM RAM (`0x1000fff0`), so the
    /// shipped signature cannot even see the section, let alone accept word 0. The
    /// widened scan finds it, harvests all four handlers, and arms the Thumb paint.
    #[test]
    fn ccm_stack_in_a_readonly_load_is_confirmed_only_when_widened() {
        let bytes = build(
            0x0800_0000,
            0x2, // SHF_ALLOC only
            0x4, // PF_R: not an executable load
            &handler_words(0x1000_fff0, 0x0800_8000),
            0x0800_8000,
            5,
            0x0800_8011, // e_entry is `_start`, not the reset vector
            EM_ARM,
        );
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert!(
            super::super::cortexm_vector_table_shipped(&file).is_none(),
            "THE BUG: the shipped signature must reject this image"
        );
        assert_eq!(
            vector_table(&file, false).map(|t| t.0),
            None,
            "option off is the shipped call verbatim"
        );
        assert_eq!(
            vector_table(&file, true).map(|t| t.0),
            Some(0x0800_0000),
            "THE FIX: the widened scan confirms the A-only .isr_vector"
        );
        assert_eq!(
            super::super::cortexm_vector_entries(&file, true),
            vec![0x0800_8000, 0x0800_8004, 0x0800_8008, 0x0800_800c],
            "all four Thumb handlers are harvested, masked/sorted/deduped"
        );
        assert_eq!(
            super::super::cortexm_thumb_paints(&file, true),
            vec![ContextPaint {
                addr: 0x0800_8000,
                end: Some(0x0800_8014),
                var: "TMode",
                value: 1
            }],
            "a confirmed table region-paints TMode=1 over the executable sections"
        );
    }

    /// betaflight: the table is linked into RAM at `0x20000000` while its handlers
    /// live in flash BELOW it. The shipped harvest's "stop once the scan reaches
    /// the lowest handler (the start of code)" rule then fires on the second slot
    /// and the table looks one word long; the widened harvest requires the lowest
    /// handler to be at or above the table base before that stop can apply.
    #[test]
    fn ram_relocated_table_below_its_handlers_still_harvests() {
        let words = handler_words(0x1000_fff0, 0x0800_8000);
        let bytes = build(0x2000_0000, 0x2, 0x4, &words, 0x0800_8000, 5, 0x0800_8001, EM_ARM);
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        let execs = executable_sections(&file);
        let in_exec = |vma: u64| in_executable_section(&execs, vma);
        let data: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        assert_eq!(
            harvest_vector_slots(0x2000_0000, &data, true, &in_exec, false).len(),
            1,
            "THE BUG: the unrelaxed start-of-code stop truncates the table to one slot"
        );
        assert_eq!(
            harvest_vector_slots(0x2000_0000, &data, true, &in_exec, true).len(),
            4,
            "THE FIX: a table above the code it points at is walked to its real end"
        );
        assert_eq!(vector_table(&file, true).map(|t| t.0), Some(0x2000_0000));
    }

    /// crazyflie / nuttx: the table sits in an ordinary executable section with an
    /// ordinary SRAM stack word, and is rejected ONLY because the link script
    /// points `e_entry` somewhere other than the reset vector.
    #[test]
    fn reset_word_need_not_equal_e_entry() {
        let bytes = build(
            0x0800_0000,
            0x2 | 0x4, // AX: the shipped candidate set can see it
            0x4 | 0x1,
            &handler_words(0x2001_ffff, 0x0800_8000),
            0x0800_8000,
            5,
            0x0800_8011, // != word[1]
            EM_ARM,
        );
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert!(super::super::cortexm_vector_table_shipped(&file).is_none(), "THE BUG");
        assert_eq!(vector_table(&file, true).map(|t| t.0), Some(0x0800_0000), "THE FIX");
    }

    /// Precision floor: a `.data`-shaped section whose first word happens to land
    /// in the SRAM window is NOT a table — the run of Thumb handler pointers is
    /// what confirms one, and two conforming slots are not a run.
    #[test]
    fn sram_first_word_alone_is_not_a_table() {
        // A plausible `.data` head: a self-referential SRAM pointer, a zero, then
        // even (non-Thumb) pointers — the run ends at slot 2 with 0 handlers.
        let datalike = vec![0x2000_0004u32, 0, 0x2000_2530, 0x2000_2598, 0, 0, 0, 0];
        let bytes =
            build(0x2000_0000, 0x2 | 0x1, 0x4 | 0x2, &datalike, 0x0800_8000, 5, 0x0800_8001, EM_ARM);
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert_eq!(vector_table(&file, true), None, "a .data struct must not arm the oracle");

        let two = vec![0x2000_4000u32, 0x0800_8001, 0x0800_8005, 0x1234_5678, 0, 0, 0, 0];
        let bytes = build(0x0800_0000, 0x2, 0x4, &two, 0x0800_8000, 5, 0x0800_8001, EM_ARM);
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert_eq!(vector_table(&file, true), None, "two conforming slots is not a run");

        let three = vec![0x2000_4000u32, 0x0800_8001, 0x0800_8005, 0x0800_8009, 0x1234_5678, 0, 0, 0];
        let bytes = build(0x0800_0000, 0x2, 0x4, &three, 0x0800_8000, 5, 0x0800_8001, EM_ARM);
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert_eq!(
            vector_table(&file, true).map(|t| t.0),
            Some(0x0800_0000),
            "three is the threshold"
        );
    }

    /// Strict no-op off ARM: the same byte pattern on an x86-64 object confirms
    /// nothing, with the flag on or off.
    #[test]
    fn non_arm_is_a_strict_no_op() {
        let bytes = build(
            0x0800_0000,
            0x2,
            0x4,
            &handler_words(0x1000_fff0, 0x0800_8000),
            0x0800_8000,
            5,
            0x0800_8001,
            EM_X86_64,
        );
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        assert_eq!(vector_table(&file, true), None);
        assert!(super::super::cortexm_vector_entries(&file, true).is_empty());
        assert!(super::super::cortexm_thumb_paints(&file, true).is_empty());
    }

    /// Monotonicity: when the shipped signature already matches, the widened call
    /// returns that same section with the same harvest — the option can add a
    /// table, never move or drop one.
    #[test]
    fn shipped_match_wins_over_the_widened_scan() {
        let bytes = build(
            0x0800_0000,
            0x2 | 0x4,
            0x4 | 0x1,
            &handler_words(0x2000_4000, 0x0800_8000),
            0x0800_8000,
            5,
            0x0800_8001, // == word[1]: the shipped signature matches
            EM_ARM,
        );
        let file = object::File::parse(bytes.as_slice()).expect("parse");
        let shipped = super::super::cortexm_vector_table_shipped(&file).map(|t| t.0);
        assert_eq!(shipped, Some(0x0800_0000), "precondition: the shipped signature matches");
        assert_eq!(vector_table(&file, true).map(|t| t.0), shipped, "same section either way");
        assert_eq!(
            super::super::cortexm_vector_entries(&file, true),
            super::super::cortexm_vector_entries(&file, false),
            "same harvest either way"
        );
    }

    /// The committed fixture the console e2e drives is exactly the widened shape:
    /// rejected by the shipped signature, confirmed and fully harvested by the
    /// widened one. Pins the fixture so a regenerated file cannot silently drift.
    #[test]
    fn committed_fixture_is_the_widened_shape() {
        let bytes = fixture("cortexm_ccm_vectors_le32");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        assert!(super::super::cortexm_vector_table_shipped(&file).is_none());
        assert_eq!(vector_table(&file, true).map(|t| t.0), Some(0x0800_0000));
        assert_eq!(
            super::super::cortexm_vector_entries(&file, true),
            vec![0x0800_8000, 0x0800_8004, 0x0800_8008, 0x0800_800c]
        );
    }
}
