//! The Listing/xref tier — a post-disassembly recursive-descent disassembler +
//! instruction/xref/function model that reuses the ported SLEIGH decoder.
//!
//! This is **scope-B**: an optional, default-OFF subsystem (the canonical spec
//! is `docs/history/listing-tier-design.md`). It performs program-wide recursive-descent
//! disassembly over loadimage bytes — reusing [`Translate::one_instruction`] and
//! a *lifted copy* of the S2 flow classifier ([`classify`]) — to build three
//! sub-models behind one [`Listing`] facade:
//!
//!  - the **instruction model** ([`model::Insn`]),
//!  - the **cross-reference model** ([`model::Reference`], Call/Code edges), and
//!  - the **discovered-function model** ([`model::DiscoveredFunction`]).
//!
//! # PR0 scope (this module)
//!
//! The keystone core: [`model`], [`decode`], [`classify`], [`walk`], and the
//! [`Listing`] facade. [`Listing::build`] is assembled but **not invoked from
//! the engine** (PR2) and has **no `--option` flag** (PR1) and **no
//! `AnalysisCtx` change** (PR1). `context.rs` (ARM/MIPS decode-context paint) is
//! PR5; x86-64 (the PR0 target) needs no context. The CodeUnit partition
//! *queries* are PR3, but the [`model::CodeUnit`] type and the
//! `covered`/`exec_ranges` fields are defined here now.

pub mod classify;
pub mod context;
pub mod decode;
pub mod kuna_tailcallentry;
mod kuna_picbase;
mod kuna_picpool;
mod kuna_poolref;
mod kuna_ppclocalentry;
mod kuna_switchtable;
mod kuna_unmappedentry;
pub mod model;
pub mod walk;
// (kuna) The read-only cross-reference query behind `kuna xrefs` -- a consumer of
// the same decode this tier performs, not a pass; nothing commits its output.
pub mod xrefs;

use std::collections::BTreeMap;
use std::ops::Bound;
use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

use crate::loadimage_object::ObjectLoadImage;

pub use model::{
    CodeUnit, DiscoveredFunction, FlowKind, FlowType, Insn, RawOp, RefKind, Reference,
};

/// The Listing facade: three sub-models sharing one decode pass (design §2.5).
pub struct Listing {
    /// Instruction model, keyed by VMA.
    insns: BTreeMap<u64, Insn>,
    /// Incoming xref edges (callers / branch sources), keyed by target VMA.
    refs_to: BTreeMap<u64, Vec<Reference>>,
    /// Outgoing xref edges, keyed by source VMA.
    refs_from: BTreeMap<u64, Vec<Reference>>,
    /// Discovered/seeded functions, keyed by entry VMA (ordered).
    funcs: BTreeMap<u64, DiscoveredFunction>,
    /// Instruction-byte coverage (`[vma, vma+len-1]` per decoded insn).
    #[allow(dead_code)] // gap = exec_ranges - covered: a PR7 (AIF) consumer.
    covered: RangeList,
    /// The coverage universe for the partition / gap walk (sorted, disjoint).
    exec_ranges: Vec<(u64, u64)>,
}

impl Listing {
    /// Build the Listing by recursive-descent over the loadimage, seeded from
    /// `seeds` (design §3). `file` supplies the executable-range universe;
    /// `image` and `arch` are part of the keystone contract (the partition uses
    /// the exec ranges; the decode reads through the engine loader). `translate`
    /// drives the SLEIGH decoder.
    ///
    /// `seed_names` is an optional `(addr, name)` overlay for the seed functions
    /// (e.g. from `existing_function_addrs` / entry-name overlays). `from_symbol`
    /// is recorded for every seed listed in `funcsym_seeds`.
    pub fn build(
        file: &object::File,
        _image: &ObjectLoadImage,
        arch: &Architecture,
        translate: &dyn Translate,
        seeds: &[u64],
    ) -> Listing {
        Self::build_with_meta(file, _image, arch, translate, seeds, &[], &[])
    }

    /// A Listing assembled from a partition someone else already decoded.
    ///
    /// The reference walk ([`xrefs::build`]) is a recursive descent over the same
    /// loadimage and leaves behind the same two facts the AIF gap-walk consumes —
    /// which bytes are instructions, and which addresses are functions — so it can
    /// hand them here instead of paying for a second decode of the program. Only
    /// the partition is populated: `mnemonic` is filled for the prologues the
    /// fingerprint histogram reads and empty everywhere else, and the reference
    /// model is empty (the gap-walk reads neither).
    pub fn from_partition(
        insns: BTreeMap<u64, Insn>,
        funcs: BTreeMap<u64, DiscoveredFunction>,
        exec_ranges: Vec<(u64, u64)>,
    ) -> Listing {
        Listing {
            insns,
            refs_to: BTreeMap::new(),
            refs_from: BTreeMap::new(),
            funcs,
            covered: RangeList::default(),
            exec_ranges,
        }
    }

    /// Like [`Listing::build`], but with seed metadata: `funcsym_seeds` is the
    /// subset of `seeds` that came from a real funcsym (sets `from_symbol`), and
    /// `seed_names` is an `(addr, name)` overlay for naming seed functions.
    pub fn build_with_meta(
        file: &object::File,
        _image: &ObjectLoadImage,
        arch: &Architecture,
        translate: &dyn Translate,
        seeds: &[u64],
        funcsym_seeds: &[u64],
        seed_names: &[(u64, String)],
    ) -> Listing {
        // The executable-range universe (design §2.4 / §3.4 out-of-bounds gate),
        // sorted by low VMA so the partition / gap queries can binary-search it.
        let mut exec_ranges: Vec<(u64, u64)> = crate::entry::executable_sections(file)
            .into_iter()
            .map(|(lo, hi, _data)| (lo, hi))
            .collect();
        exec_ranges.sort_unstable();

        // The code space the Addresses are built in.
        let code_space = match arch.manage().get_default_code_space() {
            Some(s) => Rc::clone(s),
            None => {
                // No code space ⇒ nothing decodable; return an empty Listing.
                return Listing {
                    insns: BTreeMap::new(),
                    refs_to: BTreeMap::new(),
                    refs_from: BTreeMap::new(),
                    funcs: BTreeMap::new(),
                    covered: RangeList::new(),
                    exec_ranges,
                };
            }
        };

        // Build the seed metadata map.
        let name_of: BTreeMap<u64, String> =
            seed_names.iter().map(|(a, n)| (*a, n.clone())).collect();
        let mut seed_funcs: BTreeMap<u64, DiscoveredFunction> = BTreeMap::new();
        for &entry in seeds {
            seed_funcs.insert(
                entry,
                DiscoveredFunction {
                    entry,
                    name: name_of.get(&entry).cloned(),
                    from_symbol: funcsym_seeds.contains(&entry),
                    has_no_return: false,
                    call_fixup: None,
                },
            );
        }

        // Resolve the decode-mode context (ARM TMode / MIPS ISA_MODE) by reusing
        // the marker logic (design §4.2 / PR5). On x86-64 (and any language with no
        // decode-mode context) this painter is empty and the walk decodes exactly
        // as before; on ARM/MIPS it paints Thumb/MIPS16 mode so alt-ISA functions
        // decode correctly instead of as A32/MIPS32 garbage.
        let painter = context::ContextPainter::new(file);

        // The PPC64 ELFv2 local-entry fold (`ppclocalentry`): an intra-module `bl`
        // targets `st_value + <localentry>`, which is a point inside the callee,
        // not a function of its own. Empty on every other architecture and
        // whenever the option is off (see `kuna_ppclocalentry`).
        let local_entries = kuna_ppclocalentry::fold_map(arch, file, seeds);

        let st = walk::walk(
            translate,
            arch,
            &code_space,
            &exec_ranges,
            seeds,
            &seed_funcs,
            &painter,
            &local_entries,
        );

        let mut refs_to = st.refs_to;
        let mut refs_from = st.refs_from;
        // Lock the xref read-API ordering/dedup semantics (design §6 / PR4): each
        // bucket is sorted (refs_to by source VMA then kind, refs_from by target
        // VMA then kind) and de-duplicated on `(from, to, kind)`, so a target
        // referenced twice from the same call site contributes one edge and
        // `ref_count_to` equals the number of distinct referencing sites.
        finalize_refs(&mut refs_to, /* by_source = */ true);
        finalize_refs(&mut refs_from, /* by_source = */ false);

        Listing {
            insns: st.insns,
            refs_to,
            refs_from,
            funcs: st.funcs,
            covered: st.covered,
            exec_ranges,
        }
    }

    /// Seed the discovered-function model's `has_no_return` / `call_fixup` flags
    /// from the load-time Known passes (the `noreturn_known` / `callfixup` pass
    /// outputs, by entry VMA), so a Listing consumer can skip already-modeled
    /// callees and treat a Known-no-return callee as terminal evidence.
    ///
    /// Consumes and returns `self` (a builder-style post-build refinement): the
    /// keystone walk does not know the Known/fixup facts (they come from sibling
    /// passes), so they are layered on after `build`. A seed VMA with no matching
    /// `DiscoveredFunction` is ignored (additive, never fails).
    #[must_use]
    pub fn with_noreturn_seeds(mut self, noreturn: &[u64], callfixup: &[u64]) -> Listing {
        for &addr in noreturn {
            if let Some(f) = self.funcs.get_mut(&addr) {
                f.has_no_return = true;
            }
        }
        for &addr in callfixup {
            if let Some(f) = self.funcs.get_mut(&addr) {
                // A nonempty marker is all the consumer reads (`call_fixup.is_some()`).
                if f.call_fixup.is_none() {
                    f.call_fixup = Some(String::new());
                }
            }
        }
        self
    }

    // ---- instruction model ----

    /// The instruction starting exactly at `vma`, if one was decoded.
    pub fn instruction_at(&self, vma: u64) -> Option<&Insn> {
        self.insns.get(&vma)
    }

    /// True iff a decoded instruction starts exactly at `vma`.
    pub fn is_instruction_start(&self, vma: u64) -> bool {
        self.insns.contains_key(&vma)
    }

    /// The number of decoded instructions.
    pub fn num_instructions(&self) -> usize {
        self.insns.len()
    }

    /// Iterate the decoded instructions in address order.
    pub fn instructions(&self) -> impl Iterator<Item = (&u64, &Insn)> {
        self.insns.iter()
    }

    /// Iterate decoded instructions in `[start, end)`, or from `start` onward
    /// when `end` is `None`.
    pub fn instructions_in_range(
        &self,
        start: u64,
        end: Option<u64>,
    ) -> impl Iterator<Item = (&u64, &Insn)> {
        let end = end.map(|end| end.max(start));
        self.insns.range((
            Bound::Included(start),
            end.map_or(Bound::Unbounded, Bound::Excluded),
        ))
    }

    /// The VMA of the first decoded instruction that starts strictly after `vma`
    /// (the upper bound of an undefined gap that begins at `vma`). `None` if no
    /// decoded instruction starts after `vma` (the gap runs to the end of the
    /// executable image). Used by the AIF gap-walk to scope the "must-decode-here"
    /// interior of a gap (`aif`).
    pub fn next_instruction_start_after(&self, vma: u64) -> Option<u64> {
        self.insns.range(vma.checked_add(1)?..).next().map(|(&a, _)| a)
    }

    /// The executable-range universe `[lo, hi)` the Listing covers (sorted,
    /// disjoint). The AIF gap-walk (`aif`) needs it to bound its speculative
    /// gap decode (`memory.contains` / executable-block guard).
    pub fn exec_ranges(&self) -> &[(u64, u64)] {
        &self.exec_ranges
    }

    /// The decoded instruction whose `[addr, addr+len)` byte span contains `vma`
    /// (its start *or* interior), if any (ordered interior lookup).
    fn instruction_covering(&self, vma: u64) -> Option<&Insn> {
        let (_, insn) = self.insns.range(..=vma).next_back()?;
        let end = insn.addr.wrapping_add(insn.len as u64);
        (vma >= insn.addr && vma < end).then_some(insn)
    }

    // ---- code-unit partition (design §2.4 / §6) ----

    /// The code/data partition class of `vma` (design §2.4): the start-or-interior
    /// of a decoded instruction is [`CodeUnit::Instruction`]; any other VMA inside
    /// an executable range is [`CodeUnit::Undefined`]; everything outside the
    /// executable ranges is conservatively [`CodeUnit::Data`].
    ///
    /// This is the partition the walk leaves behind (`partition_code_units` in the
    /// design): the keystone has no per-byte symbol-typed-data model, so "Data"
    /// here means "not in an executable range" — the conservative split the two
    /// relevant consumers need (A's "fell into data" is `!is_instruction_start`;
    /// AIF's gap walk is `first_undefined_after`).
    pub fn code_unit_at(&self, vma: u64) -> CodeUnit {
        if let Some(insn) = self.instruction_covering(vma) {
            return CodeUnit::Instruction(insn.addr);
        }
        if self.in_exec(vma) {
            CodeUnit::Undefined
        } else {
            CodeUnit::Data(vma, 0)
        }
    }

    /// True iff `code_unit_at(vma)` is [`CodeUnit::Data`] (A's "fell into data").
    pub fn is_data(&self, vma: u64) -> bool {
        matches!(self.code_unit_at(vma), CodeUnit::Data(..))
    }

    /// True iff `code_unit_at(vma)` is [`CodeUnit::Undefined`] — an executable-range
    /// VMA that is neither code nor typed data.
    pub fn is_undefined(&self, vma: u64) -> bool {
        matches!(self.code_unit_at(vma), CodeUnit::Undefined)
    }

    /// The first [`CodeUnit::Undefined`] VMA strictly after `vma` (AIF gap walk).
    ///
    /// Scans forward from `vma + 1`, skipping over each decoded instruction's byte
    /// span; returns the first executable-range address that is not covered by an
    /// instruction, or `None` if every executable byte after `vma` is covered.
    pub fn first_undefined_after(&self, vma: u64) -> Option<u64> {
        let mut probe = vma.checked_add(1)?;
        loop {
            // Advance past any executable range we have already fallen off the end
            // of, snapping `probe` up to the next range's low bound.
            let (lo, hi) = self.range_containing_or_after(probe)?;
            if probe < lo {
                probe = lo;
            }
            // Within [lo, hi): if covered by an instruction, jump to its end and
            // retry; otherwise this is the first undefined byte.
            match self.instruction_covering(probe) {
                Some(insn) => {
                    let end = insn.addr.wrapping_add(insn.len as u64);
                    if end <= probe {
                        return Some(probe); // degenerate; avoid a non-advancing loop
                    }
                    probe = end;
                    if probe >= hi {
                        // Fell off this range; continue with the next range.
                        probe = hi;
                    }
                }
                None => return Some(probe),
            }
        }
    }

    /// True iff `vma` lands inside any executable range `[lo, hi)`.
    fn in_exec(&self, vma: u64) -> bool {
        self.exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
    }

    /// The executable range containing `vma`, else the first range starting at-or
    /// -after `vma` (exec ranges are sorted by low bound).
    fn range_containing_or_after(&self, vma: u64) -> Option<(u64, u64)> {
        self.exec_ranges.iter().copied().find(|&(_, hi)| vma < hi)
    }

    // ---- xref model (read-only, design §6 / PR4) ----

    /// Incoming references to `to` (callers / branch sources), sorted by source
    /// VMA then kind, de-duplicated on `(from, to, kind)`.
    pub fn refs_to(&self, to: u64) -> &[Reference] {
        self.refs_to.get(&to).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Outgoing references from `from`, sorted by target VMA then kind,
    /// de-duplicated on `(from, to, kind)`.
    pub fn refs_from(&self, from: u64) -> &[Reference] {
        self.refs_from.get(&from).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Iterate every distinct reference *source* VMA in address order (the
    /// call-site / reference worklist a consumer drives).
    pub fn ref_source_iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.refs_from.keys().copied()
    }

    /// True iff anything references `to`.
    pub fn has_refs_to(&self, to: u64) -> bool {
        self.refs_to.get(&to).is_some_and(|v| !v.is_empty())
    }

    /// The number of (distinct) references to `to` — equal to the number of
    /// distinct referencing sites after dedup.
    pub fn ref_count_to(&self, to: u64) -> usize {
        self.refs_to.get(&to).map_or(0, Vec::len)
    }

    // ---- function model (ordered, design §6 / PR3) ----

    /// The function entry at exactly `vma`, if any.
    pub fn function_at(&self, vma: u64) -> Option<&DiscoveredFunction> {
        self.funcs.get(&vma)
    }

    /// The function whose entry is the greatest `<= vma` — the function *containing*
    /// `vma` (ordered interval lookup, `range(..=vma).next_back()`, the faithful
    /// analog of Ghidra's ordered FunctionManager).
    ///
    /// Note this is an entry-ordered containment: it returns the nearest preceding
    /// function entry; it does not bound `vma` by that function's extent (the
    /// keystone tracks entries, not function bodies).
    pub fn function_containing(&self, vma: u64) -> Option<&DiscoveredFunction> {
        self.funcs.range(..=vma).next_back().map(|(_, f)| f)
    }

    /// The next function strictly after `vma`, by address (ordered query).
    pub fn next_function_after(&self, vma: u64) -> Option<&DiscoveredFunction> {
        self.funcs.range(vma.checked_add(1)?..).next().map(|(_, f)| f)
    }

    /// The number of discovered/seeded functions.
    pub fn function_count(&self) -> usize {
        self.funcs.len()
    }

    /// Iterate the functions in address order.
    pub fn functions(&self) -> impl Iterator<Item = (&u64, &DiscoveredFunction)> {
        self.funcs.iter()
    }
}

/// Numeric rank of a [`RefKind`] for a stable secondary sort key.
fn ref_kind_ord(k: RefKind) -> u8 {
    match k {
        RefKind::Call => 0,
        RefKind::Code => 1,
        RefKind::Data => 2,
        RefKind::Read => 3,
        RefKind::Write => 4,
    }
}

/// Sort + dedup one direction of the xref multimap, locking the read-API ordering
/// (design §6 / PR4). `by_source` sorts each bucket by source VMA (the `refs_to`
/// direction, where the bucket key is the target); otherwise by target VMA (the
/// `refs_from` direction, where the bucket key is the source). Kind is the
/// secondary key. Dedup is on the full `(from, to, kind)` triple.
fn finalize_refs(map: &mut BTreeMap<u64, Vec<Reference>>, by_source: bool) {
    for refs in map.values_mut() {
        refs.sort_by(|a, b| {
            let pa = if by_source { a.from } else { a.to };
            let pb = if by_source { b.from } else { b.to };
            pa.cmp(&pb).then_with(|| ref_kind_ord(a.kind).cmp(&ref_kind_ord(b.kind)))
        });
        refs.dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn insn(addr: u64) -> Insn {
        Insn {
            addr,
            len: 1,
            fall_through: addr.checked_add(1),
            flow: Default::default(),
            flows: Vec::new(),
            mnemonic: String::new(),
            operands: String::new(),
            pcode: None,
        }
    }

    #[test]
    fn instruction_ranges_are_start_inclusive_and_end_exclusive() {
        let insns = [0x10, 0x20, u64::MAX]
            .into_iter()
            .map(|addr| (addr, insn(addr)))
            .collect();
        let listing = Listing {
            insns,
            refs_to: BTreeMap::new(),
            refs_from: BTreeMap::new(),
            funcs: BTreeMap::new(),
            covered: RangeList::new(),
            exec_ranges: Vec::new(),
        };
        let addrs = |start, end| {
            listing
                .instructions_in_range(start, end)
                .map(|(&addr, _)| addr)
                .collect::<Vec<_>>()
        };

        assert_eq!(addrs(0x10, Some(0x20)), vec![0x10]);
        assert_eq!(addrs(0x15, Some(u64::MAX)), vec![0x20]);
        assert_eq!(addrs(0x20, None), vec![0x20, u64::MAX]);
        assert!(addrs(0x20, Some(0x20)).is_empty());
        assert!(addrs(0x20, Some(0x10)).is_empty());
    }
}
