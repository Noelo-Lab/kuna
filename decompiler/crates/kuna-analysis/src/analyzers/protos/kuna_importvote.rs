//! (kuna) `importvote` — the direct call sites of every PLT import (P1).
//!
//! A byte scan of the executable sections of an x86-64 ELF for `call rel32`
//! into a PLT stub. The sites are committed to the architecture; the engine
//! reads them only when a function tail-calls an import no declaration reached
//! ([`kuna_decomp::kuna_importvote`]), so the scan is all a run pays up front.

use std::collections::BTreeMap;

use object::read::Object;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// The most call sites kept per import.
const MAX_SITES: usize = 32;

/// (kuna) The import call-site pass (`importvote`), gated at the commit.
pub struct ImportVotePass;

impl AnalysisPass for ImportVotePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "importvote"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let file = ctx.file;
        if !matches!(file.format(), object::BinaryFormat::Elf) || file.architecture() != object::Architecture::X86_64 {
            return AnalysisOutput::default();
        }
        let mut stubs: Vec<u64> = crate::loader::elf_plt::resolve_plt_imports(file, ctx.bytes).iter().map(|s| s.addr).collect();
        stubs.sort_unstable();
        if stubs.is_empty() {
            return AnalysisOutput::default();
        }
        let mut sites: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
        for (vma, _, data) in crate::entry::executable_sections(file) {
            for i in 0..data.len().saturating_sub(4) {
                if data[i] != 0xE8 {
                    continue;
                }
                let rel = i32::from_le_bytes([data[i + 1], data[i + 2], data[i + 3], data[i + 4]]) as i64;
                let site = vma + i as u64;
                let target = (site + 5).wrapping_add_signed(rel);
                if stubs.binary_search(&target).is_ok() {
                    let v = sites.entry(target).or_default();
                    if v.len() < MAX_SITES {
                        v.push(site);
                    }
                }
            }
        }
        AnalysisOutput { import_call_sites: sites.into_iter().collect(), ..AnalysisOutput::default() }
    }
}
