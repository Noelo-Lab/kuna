//! (kuna) `pdbinterior` — never start a function inside a procedure the PDB
//! describes (P1 code/data partition).
//!
//! The PDB half of [`crate::entry::kuna_pdatainterior`], for the functions the
//! exception table cannot vouch for. A kuna `FunctionSymbol` is an entry address
//! with no extent, so the commit boundary cannot answer "is this candidate already
//! inside a known function?". `.pdata` answers it for every function that needs
//! unwind data, but a frameless leaf has no `RUNTIME_FUNCTION` at all. When MSVC
//! also inlined that leaf into its only caller, the out-of-line copy has no caller
//! either, so recursive descent never decodes it and the PDB is the only record of
//! where it starts and ends.
//!
//! ## What goes wrong without it
//!
//! The reporter's MSVC `/O2` testbed (`ab_o2.exe`) has `cascade_switch` at
//! `0x1400010e0`, a 246-byte leaf of exactly that kind. The aggressive gap walk
//! (`aif`) probes its undecoded bytes and accepts `sub_140001178` (the middle of a
//! 6-byte `cmp`) and `sub_1400011bc` (a real instruction, but a fall-through inside
//! the switch). `funcboundflow` then truncates `cascade_switch` at the first of
//! them, and the `0xf043` case disappears from the emitted C.
//!
//! ## What makes a procedure extent usable
//!
//! Every `S_GPROC32`/`S_LPROC32` record carries its code length, which is the
//! extent the symbol table never had. `[start, start + len)` is used only when
//!
//! 1. the PDB gives that start one end (no second record disagrees), the code
//!    maps to one contiguous range of the image, and the range lies inside one
//!    executable section, so a corrupt, overflowing or OMAP-split length is never
//!    read as a body;
//! 2. no named function start (image symbol or PDB public) and no other
//!    procedure's start lies strictly inside it, and it does not overlap the range
//!    kept before it (the `.pdata` test, shared). A `.pdata` `BeginAddress` inside
//!    a procedure does not disqualify it: MSVC splits one function's unwind data
//!    across chained records, and the procedure is the authority on where that
//!    function ends; and
//! 3. a function is committed at `start` ([`suppress_owned_interior_entries`],
//!    checked at the commit). The `pdb` pass names functions from the publics,
//!    and a `static` function has an `S_LPROC32` record but no public: 756 of the
//!    3,268 procedures in each testbed image. Dropping an entry inside one of
//!    those whose start nothing else admits would leave its code with no function
//!    at all, so such a procedure is left alone.
//!
//! ## Which entries inside it are dropped
//!
//! Only the ones the procedure's own function decodes. At the commit the walk
//! from `start` follows fall-through and jumps without leaving the procedure or
//! entering a callee, and it does not fall through a call it cannot prove returns:
//! one to a target the decompiler treats as no-return, or one with no resolvable
//! target at all (an indirect `call [__imp_ExitProcess]` bound no-return by
//! `peimportcall`, or the `int 0x29` fastfail). An interior entry is dropped when
//! it lies inside an instruction some used body's walk decoded (the truncating
//! fall-through on `cascade_switch`, or the middle of its `cmp`) and no used body
//! calls it. An entry in code only a jump table, an exception handler or a call
//! reaches is the only function that code has: on a 32-bit MSVC image a `__finally`
//! block, a `catch` block and a switch's case bodies sit inside the parent's
//! procedure, and hand-written math routines call labels inside their own
//! procedure, so those entries stay. An entry a caller in another procedure still
//! calls stays too, so the call site keeps naming it.
//!
//! ## Scope
//!
//! A PE whose fingerprint-matched `.pdb` is found (the `pdb` pass's search and
//! GUID/age gate, reused), on x86, x64 and ARM64. 32-bit ARM abstains: a Thumb
//! entry can carry the interworking bit, which would place an entry that IS a
//! procedure start one byte inside it. The commit applies the bodies only while
//! `pdb` is also on, so switching the PDB off removes every trace of it.

use std::collections::{BTreeMap, BTreeSet};

use object::read::{Object, ObjectSection};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::walk::PdbProcedure;
use super::{codeview, locate, open_fingerprint_matched, pe_image_base, walk};

/// (kuna) The PDB-procedure-interior entry-suppression pass (`pdbinterior`).
///
/// Reports the usable procedure extents on [`AnalysisOutput::pdb_bodies`]; the
/// commit applies them with [`suppress_owned_interior_entries`].
pub struct PdbInteriorPass;

impl AnalysisPass for PdbInteriorPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "pdbinterior"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput {
            pdb_bodies: eligible_pdb_bodies(ctx.file, ctx.bytes, ctx.image_path),
            ..AnalysisOutput::default()
        }
    }
}

/// The PDB procedure extents that describe exactly one function, sorted and
/// disjoint. Empty unless `file` is an x86/x64/ARM64 PE whose fingerprint-matched
/// `.pdb` can be located from `image_path`.
pub fn eligible_pdb_bodies(
    file: &object::File,
    bytes: &[u8],
    image_path: Option<&std::path::Path>,
) -> Vec<(u64, u64)> {
    if file.format() != object::BinaryFormat::Pe
        || !matches!(
            file.architecture(),
            object::Architecture::I386
                | object::Architecture::X86_64
                | object::Architecture::Aarch64
        )
    {
        return Vec::new();
    }
    let Some(cv) = codeview::extract_codeview(bytes) else {
        return Vec::new();
    };
    let candidates = locate::pdb_candidates(image_path, &cv);
    let (Some(image_base), false) = (pe_image_base(bytes), candidates.is_empty()) else {
        return Vec::new();
    };
    let Some(mut pdb) = open_fingerprint_matched(&candidates, &cv) else {
        return Vec::new();
    };
    let procs = walk::walk_procedures(&mut pdb, image_base);
    if procs.is_empty() {
        return Vec::new();
    }
    let mut starts = crate::entry::existing_function_addrs(file, bytes);
    starts.extend(
        walk::walk_functions(&mut pdb, image_base)
            .into_iter()
            .map(|f| f.vma),
    );
    starts.extend(procs.iter().map(|p| p.start));
    starts.sort_unstable();
    starts.dedup();
    let extents = vouched_extents(&procs, &executable_ranges(file));
    crate::entry::kuna_pdatainterior::eligible(&extents, &starts)
}

/// One decoded instruction, as the walk inside a procedure reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InsnFlow {
    /// The instruction's byte length.
    pub len: u32,
    /// The next instruction, when this one can fall through.
    pub fall_through: Option<u64>,
    /// Static jump targets.
    pub jumps: Vec<u64>,
    /// Static call targets.
    pub calls: Vec<u64>,
    /// Whether this instruction is a call (including a computed/indirect call and
    /// an `int`-userop call, which carry no [`Self::calls`] target). A call the
    /// walk cannot resolve is one it cannot prove returns, so it does not fall
    /// through: the decompiler stops at the no-return imports (`peimportcall`) and
    /// the `int 0x29` fastfail these shapes take, and the walk must not claim the
    /// code past such a call as the procedure's own.
    pub is_call: bool,
}

/// Drop every entry strictly inside one of `bodies` that the body's own function
/// already decodes, and return the dropped entries.
///
/// A body is used only when its first address is a function the commit installs
/// (one of `entries` itself, or an address `is_function` accepts), and only
/// bodies disjoint from the ones used before it, so no used body's start is inside
/// another and the entry that admitted a body is never dropped. Every used body is
/// walked from its start with `decode`, following fall-through and jumps but
/// staying inside `[start, end)`, never entering a callee, and never falling
/// through a call it cannot prove returns (one `no_return` accepts, or one with no
/// resolvable target). An interior entry is dropped only when
///
/// * it lies inside an instruction some used body's own walk decoded (at its first
///   byte or in its middle), and
/// * no used body's walk calls it.
///
/// Everything else is kept: an entry in code only a jump table, an exception
/// handler or a call reaches is the only function that code has, and an entry a
/// caller (even one in another procedure) still calls stays a named function so
/// the call site keeps naming it.
pub fn suppress_owned_interior_entries(
    entries: &mut Vec<u64>,
    bodies: &[(u64, u64)],
    is_function: impl Fn(u64) -> bool,
    no_return: impl Fn(u64) -> bool,
    mut decode: impl FnMut(u64) -> Option<InsnFlow>,
) -> Vec<u64> {
    if bodies.is_empty() || entries.is_empty() {
        return Vec::new();
    }
    let mut admitted = entries.clone();
    admitted.sort_unstable();
    admitted.dedup();
    let mut sorted = bodies.to_vec();
    sorted.sort_unstable();

    // Walk every used body once. Each contributes its decoded reach (for the drop
    // test) and the addresses it calls (which no body may drop, wherever they land).
    let mut used_end: Option<u64> = None;
    let mut used: Vec<ProcedureWalk> = Vec::new();
    let mut called: BTreeSet<u64> = BTreeSet::new();
    for (start, end) in sorted {
        let owned = start < end
            && used_end.map_or(true, |prev| prev <= start)
            && (admitted.binary_search(&start).is_ok() || is_function(start));
        if !owned {
            continue;
        }
        used_end = Some(end);
        let walk = walk_procedure(start, end, &no_return, &mut decode);
        called.extend(walk.calls.iter().copied());
        used.push(walk);
    }

    let mut drop: BTreeSet<u64> = BTreeSet::new();
    for walk in &used {
        let lo = admitted.partition_point(|&a| a <= walk.start);
        let hi = admitted.partition_point(|&a| a < walk.end);
        drop.extend(
            admitted[lo..hi]
                .iter()
                .copied()
                .filter(|&e| !called.contains(&e) && walk.covers(e)),
        );
    }
    let mut dropped = Vec::new();
    entries.retain(|vma| {
        let keep = !drop.contains(vma);
        if !keep {
            dropped.push(*vma);
        }
        keep
    });
    dropped
}

/// What a walk from a procedure's start decodes inside the procedure.
struct ProcedureWalk {
    /// The procedure's `[start, end)`.
    start: u64,
    end: u64,
    /// `(vma, running maximum of vma + len)` over the decoded instructions, by vma.
    reach: Vec<(u64, u64)>,
    /// Every address this walk calls (a static call target, wherever it lands).
    calls: BTreeSet<u64>,
}

impl ProcedureWalk {
    /// Does a decoded instruction's byte range contain `vma`?
    fn covers(&self, vma: u64) -> bool {
        let below = self.reach.partition_point(|&(start, _)| start <= vma);
        below > 0 && self.reach[below - 1].1 > vma
    }
}

/// Decode from `start`, following fall-through and jump targets inside
/// `[start, end)`, at most once per address and at most `end - start` instructions.
/// A call the walk cannot prove returns — one to a `no_return` target, or one with
/// no resolvable target at all — does not fall through.
fn walk_procedure(
    start: u64,
    end: u64,
    no_return: &impl Fn(u64) -> bool,
    decode: &mut impl FnMut(u64) -> Option<InsnFlow>,
) -> ProcedureWalk {
    let budget = usize::try_from(end - start).unwrap_or(usize::MAX);
    let mut insns: BTreeMap<u64, u64> = BTreeMap::new();
    let mut calls = BTreeSet::new();
    let mut work = vec![start];
    while let Some(vma) = work.pop() {
        if vma < start || vma >= end || insns.contains_key(&vma) || insns.len() >= budget {
            continue;
        }
        let Some(insn) = decode(vma).filter(|i| i.len != 0) else {
            continue;
        };
        insns.insert(vma, vma.saturating_add(u64::from(insn.len)));
        calls.extend(insn.calls.iter().copied());
        work.extend(insn.jumps.iter().copied());
        let call_stops =
            insn.is_call && (insn.calls.is_empty() || insn.calls.iter().any(|&t| no_return(t)));
        if !call_stops {
            work.extend(insn.fall_through);
        }
    }
    let reach = insns
        .into_iter()
        .scan(0u64, |hi, (vma, insn_end)| {
            *hi = (*hi).max(insn_end);
            Some((vma, *hi))
        })
        .collect();
    ProcedureWalk { start, end, reach, calls }
}

/// The procedure extents the PDB and the image agree on, sorted: a start whose
/// records all carry the same end, lying inside one executable section.
fn vouched_extents(procs: &[PdbProcedure], exec: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut sorted = procs.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut out = Vec::new();
    for group in sorted.chunk_by(|a, b| a.start == b.start) {
        let [PdbProcedure {
            start,
            end: Some(end),
        }] = *group
        else {
            continue;
        };
        if inside_one_range(exec, start, end) {
            out.push((start, end));
        }
    }
    out
}

/// `[address, address + size)` of every section the image maps executable.
fn executable_ranges(file: &object::File) -> Vec<(u64, u64)> {
    file.sections()
        .filter(|s| match s.flags() {
            object::SectionFlags::Coff { characteristics } => {
                characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
            }
            _ => false,
        })
        .filter_map(|s| Some((s.address(), s.address().checked_add(s.size())?)))
        .collect()
}

/// Is the non-empty `[start, end)` wholly inside one of `ranges`?
fn inside_one_range(ranges: &[(u64, u64)], start: u64, end: u64) -> bool {
    start < end && ranges.iter().any(|&(lo, hi)| lo <= start && end <= hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEAF: u64 = 0x1400011c0;
    const LEAF_END: u64 = 0x1400011f8;
    const PHANTOM: u64 = 0x1400011d0;
    const PICK_09: u64 = 0x1400011a0;
    const STATIC_LEAF: u64 = 0x140001340;
    const STATIC_LEAF_END: u64 = 0x140001378;
    const STATIC_PHANTOM: u64 = 0x140001350;
    const HANDLER_LEAF: u64 = 0x140001380;
    const UNREACHED: u64 = 0x1400013a0;
    const CALLER_LEAF: u64 = 0x1400013c0;
    const INNER: u64 = 0x1400013d0;
    const ABORT: u64 = 0x1400013f0;
    const NORETURN_LEAF: u64 = 0x140001400;
    const PAST_ABORT: u64 = 0x140001410;
    const ABSURD_LENGTH: u64 = 0x140001540;
    const PAST_TEXT: u64 = 0x140001548;
    const IN_RDATA: u64 = 0x140002008;

    fn fixture_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    fn bodies_for(exe: &std::path::Path) -> Vec<(u64, u64)> {
        let bytes = std::fs::read(exe).unwrap_or_else(|e| panic!("read {exe:?}: {e}"));
        let file = object::File::parse(&*bytes).expect("the fixture parses");
        eligible_pdb_bodies(&file, &bytes, Some(exe))
    }

    fn scratch_copy(tag: &str, with_pdb: bool) -> (std::path::PathBuf, std::path::PathBuf) {
        let dir =
            std::env::temp_dir().join(format!("kuna-pdbinterior-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");
        for ext in if with_pdb {
            &["exe", "pdb"][..]
        } else {
            &["exe"][..]
        } {
            let name = format!("pe_pdbinterior_x86_64.{ext}");
            std::fs::copy(fixture_dir().join(&name), dir.join(&name)).expect("copy");
        }
        (dir.join("pe_pdbinterior_x86_64.exe"), dir)
    }

    /// The fixture's `.text`, decoded for the encodings the generator emits in its
    /// leaves (`cmp ecx,imm32`, `jg`/`jz rel8`, `mov eax|edx,imm32`,
    /// `cmovnz eax,edx`, `call rel32`, `ret`, `int3`); anything else is
    /// undecodable.
    fn fixture_decoder() -> impl FnMut(u64) -> Option<InsnFlow> {
        let bytes = std::fs::read(fixture_dir().join("pe_pdbinterior_x86_64.exe")).expect("read");
        let file = object::File::parse(&*bytes).expect("parse");
        let text = file.section_by_name(".text").expect(".text");
        let (base, data) = (text.address(), text.data().expect("data").to_vec());
        move |vma| {
            let at = usize::try_from(vma.checked_sub(base)?).ok()?;
            let next = |len: u64| vma + len;
            match data.get(at..)? {
                [0x81, 0xF9, ..] => Some(InsnFlow {
                    len: 6,
                    fall_through: Some(next(6)),
                    ..Default::default()
                }),
                [0x7F | 0x74, rel, ..] => Some(InsnFlow {
                    len: 2,
                    fall_through: Some(next(2)),
                    jumps: vec![next(2).wrapping_add_signed(i64::from(*rel as i8))],
                    ..Default::default()
                }),
                [0xB8 | 0xBA, ..] => Some(InsnFlow {
                    len: 5,
                    fall_through: Some(next(5)),
                    ..Default::default()
                }),
                [0x0F, 0x45, 0xC2, ..] => Some(InsnFlow {
                    len: 3,
                    fall_through: Some(next(3)),
                    ..Default::default()
                }),
                [0xE8, r0, r1, r2, r3, ..] => Some(InsnFlow {
                    len: 5,
                    fall_through: Some(next(5)),
                    calls: vec![next(5)
                        .wrapping_add_signed(i64::from(i32::from_le_bytes([*r0, *r1, *r2, *r3])))],
                    is_call: true,
                    ..Default::default()
                }),
                // FF 15 disp32: an indirect call through a memory slot -- a call
                // with no static target (the s1b IAT no-return shape).
                [0xFF, 0x15, ..] => Some(InsnFlow { len: 6, fall_through: Some(next(6)), is_call: true, ..Default::default() }),
                // CD 29: int 0x29, the fastfail userop-call, also targetless.
                [0xCD, 0x29, ..] => Some(InsnFlow { len: 2, fall_through: Some(next(2)), is_call: true, ..Default::default() }),
                [0xC3, ..] => Some(InsnFlow {
                    len: 1,
                    ..Default::default()
                }),
                [0xCC, ..] => Some(InsnFlow {
                    len: 1,
                    fall_through: Some(next(1)),
                    ..Default::default()
                }),
                _ => None,
            }
        }
    }

    /// The sidecar's `S_GPROC32` length bounds the leaf, and the gap walk's start
    /// inside it, a fall-through of the leaf's own flow, is what the suppression
    /// removes.
    #[test]
    fn the_leaf_extent_comes_from_the_pdb() {
        let bodies = bodies_for(&fixture_dir().join("pe_pdbinterior_x86_64.exe"));
        assert_eq!(
            bodies.len(),
            30,
            "one body per usable procedure, got {bodies:#x?}"
        );
        assert!(
            bodies.contains(&(LEAF, LEAF_END)),
            "leaf extent missing: {bodies:#x?}"
        );
        let mut entries = vec![PICK_09, LEAF, PHANTOM, LEAF_END];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &bodies,
            |_| false,
            |_| false,
            fixture_decoder(),
        );
        assert_eq!(dropped, vec![PHANTOM]);
        assert_eq!(entries, vec![PICK_09, LEAF, LEAF_END]);
    }

    /// A `static` function's `S_LPROC32` is an extent like any other, but it
    /// comes with no public: with no entry at its start, the entry inside it is
    /// the only function its code has, and it stays.
    #[test]
    fn a_local_procedure_whose_start_nothing_admits_keeps_its_only_entry() {
        let bodies = bodies_for(&fixture_dir().join("pe_pdbinterior_x86_64.exe"));
        assert!(
            bodies.contains(&(STATIC_LEAF, STATIC_LEAF_END)),
            "static extent: {bodies:#x?}"
        );
        let mut entries = vec![PICK_09, STATIC_PHANTOM];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &bodies,
            |_| false,
            |_| false,
            fixture_decoder(),
        );
        assert!(dropped.is_empty());
        assert_eq!(entries, vec![PICK_09, STATIC_PHANTOM]);

        let mut entries = vec![STATIC_PHANTOM];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &bodies,
            |a| a == STATIC_LEAF,
            |_| false,
            fixture_decoder(),
        );
        assert_eq!((dropped, entries), (vec![STATIC_PHANTOM], vec![]));
    }

    /// The unreached block after `handler_leaf`'s last `ret`, the routine
    /// `caller_leaf` only calls and the block after `noreturn_leaf`'s call to
    /// `abort` are inside public procedures, but no procedure's own flow decodes
    /// them, so their entries stay. Were `abort` to return, the last one would be
    /// a fall-through and go.
    #[test]
    fn entries_the_owner_does_not_decode_stay_in_the_fixture() {
        let bodies = bodies_for(&fixture_dir().join("pe_pdbinterior_x86_64.exe"));
        let all = vec![
            HANDLER_LEAF,
            UNREACHED,
            CALLER_LEAF,
            INNER,
            NORETURN_LEAF,
            PAST_ABORT,
        ];
        let mut entries = all.clone();
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &bodies,
            |_| false,
            |t| t == ABORT,
            fixture_decoder(),
        );
        assert!(dropped.is_empty(), "dropped {dropped:#x?}");
        assert_eq!(entries, all);

        let mut entries = all.clone();
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &bodies,
            |_| false,
            |_| false,
            fixture_decoder(),
        );
        assert_eq!(dropped, vec![PAST_ABORT]);
    }

    /// Records whose extent cannot be a body in this image are never used: a
    /// length that overflows the 32-bit RVA space, one that runs past the end of
    /// `.text`, and one placed in the non-executable `.rdata`.
    #[test]
    fn extents_outside_one_executable_section_are_rejected() {
        let bodies = bodies_for(&fixture_dir().join("pe_pdbinterior_x86_64.exe"));
        for start in [ABSURD_LENGTH, PAST_TEXT, IN_RDATA] {
            assert!(
                bodies.iter().all(|&(s, _)| s != start),
                "{start:#x} must not yield a body: {bodies:#x?}"
            );
        }
    }

    /// With no `.pdb` beside the image there is nothing to vouch for a body.
    #[test]
    fn no_sidecar_abstains() {
        let (exe, dir) = scratch_copy("nosidecar", false);
        let bodies = bodies_for(&exe);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(bodies.is_empty(), "no sidecar, yet {bodies:#x?}");
    }

    /// A sidecar whose age or GUID disagrees with the image's CodeView record is
    /// another build's PDB, and none of its extents may be read.
    #[test]
    fn a_fingerprint_mismatched_pdb_abstains() {
        for (tag, field) in [("age", 20usize), ("guid", 4)] {
            let (exe, dir) = scratch_copy(tag, true);
            let mut bytes = std::fs::read(&exe).expect("read copy");
            let rsds = bytes
                .windows(4)
                .position(|w| w == b"RSDS")
                .expect("RSDS record");
            bytes[rsds + field] ^= 0x5a;
            std::fs::write(&exe, &bytes).expect("patch copy");
            let bodies = bodies_for(&exe);
            let _ = std::fs::remove_dir_all(&dir);
            assert!(bodies.is_empty(), "{tag} mismatch, yet {bodies:#x?}");
        }
    }

    /// A 32-bit ARM image abstains before the PDB is opened.
    #[test]
    fn arm32_abstains() {
        let exe = fixture_dir().join("pe_pdbinterior_x86_64.exe");
        let mut bytes = std::fs::read(&exe).expect("read fixture");
        let lfanew = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        bytes[lfanew + 4..lfanew + 6]
            .copy_from_slice(&object::pe::IMAGE_FILE_MACHINE_ARMNT.to_le_bytes());
        let file = object::File::parse(&*bytes).expect("an ARMNT PE still parses");
        assert!(eligible_pdb_bodies(&file, &bytes, Some(&exe)).is_empty());
    }

    fn proc_at(start: u64, end: Option<u64>) -> PdbProcedure {
        PdbProcedure { start, end }
    }

    /// The executable-range guard: an extent is usable only inside one section.
    #[test]
    fn vouched_extents_need_one_executable_section() {
        let exec = [(0x1000, 0x2000), (0x2000, 0x3000)];
        let procs = [
            proc_at(0x1000, Some(0x1010)),
            proc_at(0x1ff0, Some(0x2010)),
            proc_at(0x2f00, Some(0x3001)),
            proc_at(0x4000, Some(0x4010)),
            proc_at(0x0ff0, Some(0x1008)),
            proc_at(0x2800, None),
            proc_at(0x2900, Some(0x2900)),
        ];
        assert_eq!(vouched_extents(&procs, &exec), vec![(0x1000, 0x1010)]);
        assert!(vouched_extents(&procs, &[]).is_empty());
    }

    /// Two records that disagree about where one procedure ends describe no
    /// single body, and neither is used; an exact duplicate is one record.
    #[test]
    fn conflicting_records_for_one_start_are_not_used() {
        let exec = [(0x1000, 0x9000)];
        let procs = [
            proc_at(0x1000, Some(0x1040)),
            proc_at(0x1000, Some(0x1080)),
            proc_at(0x2000, Some(0x2040)),
            proc_at(0x2000, Some(0x2040)),
            proc_at(0x3000, None),
            proc_at(0x3000, Some(0x3040)),
        ];
        assert_eq!(vouched_extents(&procs, &exec), vec![(0x2000, 0x2040)]);
    }

    #[test]
    fn inside_one_range_rejects_empty_and_straddling_extents() {
        let ranges = [(0x1000, 0x2000)];
        assert!(inside_one_range(&ranges, 0x1000, 0x2000));
        assert!(!inside_one_range(&ranges, 0x1800, 0x1800));
        assert!(!inside_one_range(&ranges, 0x1800, 0x2001));
        assert!(!inside_one_range(&ranges, u64::MAX - 1, u64::MAX));
    }

    /// A synthetic procedure: each tuple is `(vma, len, falls through, jumps, calls)`.
    fn program(insns: &[(u64, u32, bool, &[u64], &[u64])]) -> impl FnMut(u64) -> Option<InsnFlow> {
        let map: BTreeMap<u64, InsnFlow> = insns
            .iter()
            .map(|&(vma, len, falls, jumps, calls)| {
                let flow = InsnFlow {
                    len,
                    fall_through: falls.then_some(vma + u64::from(len)),
                    jumps: jumps.to_vec(),
                    calls: calls.to_vec(),
                    is_call: !calls.is_empty(),
                };
                (vma, flow)
            })
            .collect();
        move |vma| map.get(&vma).cloned()
    }

    /// `[0x1000, 0x1040)`: a compare-and-branch head, a fall-through arm ending in
    /// a `ret`, a branch arm that calls an internal routine, a `ret`, then code no
    /// flow from the start reaches (a jump-table case or a handler), then the
    /// internal routine only the call reaches.
    fn shaped() -> impl FnMut(u64) -> Option<InsnFlow> {
        program(&[
            (0x1000, 6, true, &[], &[]),
            (0x1006, 2, true, &[0x1018], &[]),
            (0x1008, 5, true, &[], &[]),
            (0x100d, 1, false, &[], &[]),
            (0x1018, 5, true, &[], &[0x1030]),
            (0x101d, 1, false, &[], &[]),
            (0x1020, 4, true, &[], &[]),
            (0x1024, 1, false, &[], &[]),
            (0x1030, 3, true, &[], &[]),
            (0x1033, 1, false, &[], &[]),
        ])
    }

    /// Only entries inside code the start's own flow decodes are dropped: a
    /// fall-through instruction, a jump target and the middle of an instruction.
    /// An entry only an unfollowed path reaches, a call target, and padding the
    /// flow never decodes all stay.
    #[test]
    fn only_entries_the_owner_decodes_are_dropped() {
        let bodies = [(0x1000, 0x1040)];
        let mut entries = vec![0x1000, 0x1008, 0x1018, 0x101a, 0x1020, 0x1030, 0x1034];
        let dropped =
            suppress_owned_interior_entries(&mut entries, &bodies, |_| false, |_| false, shaped());
        assert_eq!(dropped, vec![0x1008, 0x1018, 0x101a]);
        assert_eq!(entries, vec![0x1000, 0x1020, 0x1030, 0x1034]);
    }

    /// A call target the flow also falls into is still a function the code
    /// calls, and it stays.
    #[test]
    fn a_call_target_on_the_flow_is_kept() {
        let decode = program(&[
            (0x1000, 5, true, &[], &[0x1005]),
            (0x1005, 1, false, &[], &[]),
        ]);
        let mut entries = vec![0x1000, 0x1005];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1010)],
            |_| false,
            |_| false,
            decode,
        );
        assert!(dropped.is_empty());
        assert_eq!(entries, vec![0x1000, 0x1005]);
    }

    /// A call the decompiler treats as no-return ends the flow there: code after
    /// it is not the procedure's to claim, while the same call to a returning
    /// target falls through as usual.
    #[test]
    fn a_no_return_call_does_not_fall_through() {
        let decode = || {
            program(&[
                (0x1000, 5, true, &[], &[0x5000]),
                (0x1005, 4, true, &[], &[]),
                (0x1009, 1, false, &[], &[]),
            ])
        };
        let mut entries = vec![0x1000, 0x1005];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1010)],
            |_| false,
            |t| t == 0x5000,
            decode(),
        );
        assert_eq!((dropped, entries), (vec![], vec![0x1000, 0x1005]));
        let mut entries = vec![0x1000, 0x1005];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1010)],
            |_| false,
            |_| false,
            decode(),
        );
        assert_eq!((dropped, entries), (vec![0x1005], vec![0x1000]));
    }

    /// A call the walk cannot resolve (an indirect `call [mem]` or an `int`-userop
    /// call: `is_call` with no static target) does not fall through, so an entry in
    /// the block after it stays — the decompiler stops at the no-return imports and
    /// the fastfail these shapes take, and nothing else renders that block.
    #[test]
    fn a_targetless_call_does_not_fall_through() {
        let targetless_call = |vma: u64| {
            let insn = |len: u32, is_call: bool, fall: bool| InsnFlow {
                len,
                fall_through: fall.then_some(vma + u64::from(len)),
                is_call,
                ..Default::default()
            };
            match vma {
                0x1000 => Some(insn(6, false, true)),  // cmp
                0x1006 => Some(insn(6, true, true)),   // indirect/int call, no target
                0x100c => Some(insn(5, false, true)),  // interior entry
                0x1011 => Some(insn(1, false, false)), // ret
                _ => None,
            }
        };
        let mut entries = vec![0x1000, 0x100c];
        let dropped =
            suppress_owned_interior_entries(&mut entries, &[(0x1000, 0x1020)], |_| false, |_| false, targetless_call);
        assert_eq!((dropped, entries), (vec![], vec![0x1000, 0x100c]));
        // With the call spelled as a plain fall-through instruction (not a call),
        // the walk reaches the interior and drops it — proving the guard is what
        // keeps it above.
        let plain = |vma: u64| match vma {
            0x1000 => Some(InsnFlow { len: 6, fall_through: Some(0x1006), ..Default::default() }),
            0x1006 => Some(InsnFlow { len: 6, fall_through: Some(0x100c), ..Default::default() }),
            0x100c => Some(InsnFlow { len: 5, fall_through: Some(0x1011), ..Default::default() }),
            0x1011 => Some(InsnFlow { len: 1, ..Default::default() }),
            _ => None,
        };
        let mut entries = vec![0x1000, 0x100c];
        let dropped =
            suppress_owned_interior_entries(&mut entries, &[(0x1000, 0x1020)], |_| false, |_| false, plain);
        assert_eq!((dropped, entries), (vec![0x100c], vec![0x1000]));
    }

    /// An entry a caller in ANOTHER procedure calls is a real function and stays,
    /// even though its own body's owner decodes over it.
    #[test]
    fn a_cross_procedure_call_target_is_kept() {
        // `caller` [0x2000,0x2010) calls 0x1008, which is interior to `owner`
        // [0x1000,0x1040) and covered by owner's fall-through decode.
        let decode = program(&[
            (0x1000, 8, true, &[], &[]),
            (0x1008, 4, true, &[], &[]),
            (0x100c, 1, false, &[], &[]),
            (0x2000, 5, true, &[], &[0x1008]),
            (0x2005, 1, false, &[], &[]),
        ]);
        let mut entries = vec![0x1000, 0x1008, 0x2000];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1040), (0x2000, 0x2010)],
            |_| false,
            |_| false,
            decode,
        );
        assert_eq!((dropped, entries), (vec![], vec![0x1000, 0x1008, 0x2000]));
    }

    /// The ownership rule: with nothing at the start, or a start that does not
    /// decode, nothing inside is dropped. A body overlapping one already used is
    /// ignored: its admitting entry `0x1020` is dropped by the first body, so the
    /// second may not drop `0x1050` on its behalf.
    #[test]
    fn only_owned_bodies_reject_their_interior() {
        let mut entries = vec![0x1008, 0x1018];
        assert!(suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1040)],
            |_| false,
            |_| false,
            shaped()
        )
        .is_empty());
        let mut entries = vec![0x1008, 0x1018];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1040)],
            |a| a == 0x1000,
            |_| false,
            shaped(),
        );
        assert_eq!((dropped, entries), (vec![0x1008, 0x1018], vec![]));

        let mut entries = vec![0x2000, 0x2004];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x2000, 0x2010)],
            |_| false,
            |_| false,
            |_| None,
        );
        assert_eq!((dropped, entries), (vec![], vec![0x2000, 0x2004]));

        let linear: Vec<(u64, u32, bool, &[u64], &[u64])> = (0x1000..0x1060)
            .step_by(4)
            .map(|v| (v, 4, true, &[][..], &[][..]))
            .collect();
        let mut entries = vec![0x1000, 0x1020, 0x1030, 0x1050];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1040), (0x1020, 0x1060)],
            |_| false,
            |_| false,
            program(&linear),
        );
        assert_eq!(
            (dropped, entries),
            (vec![0x1020, 0x1030], vec![0x1000, 0x1050])
        );
    }

    /// The walk never asks for an address outside the procedure, however the
    /// code branches or calls.
    #[test]
    fn the_walk_stays_inside_the_procedure() {
        let mut inner = program(&[
            (0x1000, 2, true, &[0x0f00, 0x2000], &[0x3000]),
            (0x1002, 2, true, &[], &[]),
            (0x1004, 4, true, &[], &[]),
        ]);
        let mut entries = vec![0x1000, 0x1006];
        let dropped = suppress_owned_interior_entries(
            &mut entries,
            &[(0x1000, 0x1008)],
            |_| false,
            |_| false,
            |v| {
                assert!(
                    (0x1000..0x1008).contains(&v),
                    "decoded {v:#x} outside the procedure"
                );
                inner(v)
            },
        );
        assert_eq!(dropped, vec![0x1006]);
    }

    /// Over many random procedures: every dropped entry was inside a body whose
    /// start is still admitted, inside an instruction that start's walk decoded,
    /// and was not a call target; nothing else is ever dropped.
    #[test]
    fn a_dropped_entry_always_lies_in_its_owners_decoded_code() {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = |n: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % n
        };
        for _ in 0..2000 {
            let mut bodies = Vec::new();
            let mut insns: BTreeMap<u64, InsnFlow> = BTreeMap::new();
            let mut at = 0x1000;
            for _ in 0..1 + next(4) {
                at += next(0x20);
                let len = 4 + next(0x40);
                let (lo, hi) = (at, at + len);
                let mut vma = lo;
                while vma < hi {
                    let l = 1 + next(6) as u32;
                    let target = lo + next(len + 8);
                    let flow = match next(6) {
                        0 => InsnFlow {
                            len: l,
                            ..Default::default()
                        },
                        1 => InsnFlow {
                            len: l,
                            jumps: vec![target],
                            ..Default::default()
                        },
                        2 => InsnFlow {
                            len: l,
                            fall_through: Some(vma + u64::from(l)),
                            calls: vec![target],
                            ..Default::default()
                        },
                        3 => InsnFlow {
                            len: l,
                            fall_through: Some(vma + u64::from(l)),
                            jumps: vec![target],
                            ..Default::default()
                        },
                        _ => InsnFlow {
                            len: l,
                            fall_through: Some(vma + u64::from(l)),
                            ..Default::default()
                        },
                    };
                    insns.insert(vma, flow);
                    vma += 1 + next(u64::from(l) + 2);
                }
                bodies.push((lo, hi));
                at = hi;
            }
            let symbols: Vec<u64> = bodies
                .iter()
                .filter(|_| next(3) == 0)
                .map(|b| b.0)
                .collect();
            let mut entries: Vec<u64> = (0..next(16)).map(|_| 0x1000 + next(0x180)).collect();
            entries.sort_unstable();
            entries.dedup();
            let before = entries.clone();
            let decoded = insns.clone();
            let dropped = suppress_owned_interior_entries(
                &mut entries,
                &bodies,
                |a| symbols.contains(&a),
                |t| t % 7 == 0,
                |v| decoded.get(&v).cloned(),
            );
            assert_eq!(entries.len() + dropped.len(), before.len());
            for &d in &dropped {
                let &(s, e) = bodies
                    .iter()
                    .find(|&&(s, e)| s < d && d < e)
                    .expect("inside a body");
                assert!(
                    entries.contains(&s) || symbols.contains(&s),
                    "{d:#x}: owner {s:#x} gone"
                );
                let mut walk_decode = |v: u64| insns.get(&v).cloned();
                let walk = walk_procedure(s, e, &|t| t % 7 == 0, &mut walk_decode);
                assert!(
                    walk.covers(d) && !walk.calls.contains(&d),
                    "{d:#x} not in {s:#x}'s decoded code"
                );
            }
        }
    }
}
