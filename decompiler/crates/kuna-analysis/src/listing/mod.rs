//! The Listing/xref tier — a post-disassembly recursive-descent disassembler +
//! instruction/xref/function model that reuses the ported SLEIGH decoder.
//!
//! This is **scope-B**: an optional, default-OFF subsystem (the canonical spec
//! is `docs/listing-tier-design.md`). It performs program-wide recursive-descent
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
pub mod decode;
pub mod model;
pub mod walk;

use std::collections::BTreeMap;
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
    #[allow(dead_code)] // gap = exec_ranges - covered: queried in PR3/PR7.
    covered: RangeList,
    /// The coverage universe for the partition / gap walk.
    #[allow(dead_code)] // partition / gap queries are PR3/PR7.
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
        // The executable-range universe (design §2.4 / §3.4 out-of-bounds gate).
        let exec_ranges: Vec<(u64, u64)> = crate::s1_entry::executable_sections(file)
            .into_iter()
            .map(|(lo, hi, _data)| (lo, hi))
            .collect();

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

        let st = walk::walk(translate, &code_space, &exec_ranges, seeds, &seed_funcs);

        Listing {
            insns: st.insns,
            refs_to: st.refs_to,
            refs_from: st.refs_from,
            funcs: st.funcs,
            covered: st.covered,
            exec_ranges,
        }
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

    // ---- xref model (read-only) ----

    /// Incoming references to `to` (callers / branch sources).
    pub fn refs_to(&self, to: u64) -> &[Reference] {
        self.refs_to.get(&to).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Outgoing references from `from`.
    pub fn refs_from(&self, from: u64) -> &[Reference] {
        self.refs_from.get(&from).map(Vec::as_slice).unwrap_or(&[])
    }

    // ---- function model (ordered) ----

    /// The function entry at exactly `vma`, if any.
    pub fn function_at(&self, vma: u64) -> Option<&DiscoveredFunction> {
        self.funcs.get(&vma)
    }

    /// The next function strictly after `vma`, by address (ordered query).
    pub fn next_function_after(&self, vma: u64) -> Option<&DiscoveredFunction> {
        self.funcs.range(vma + 1..).next().map(|(_, f)| f)
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
