//! What a single-function study view needs beyond the batch record: the token
//! source map of the rendered C ([`FuncDetail`]) and the image's section table
//! with file offsets ([`section_rows`]).

use kuna_decomp::decompile_drive::{CodeProvenance, VarInfo};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_srcmap::{self, CodeToken};
use kuna_decomp::prettyprint::EmitToken;
use kuna_sleigh::loadimage::section_flags;

use crate::engine::ConsoleProgram;

/// The token source map of one rendered function: the tokens when they rebuild
/// the code exactly, else none and the reason.
#[derive(Debug, Clone, Default)]
pub struct FuncDetail {
    pub tokens: Vec<CodeToken>,
    pub tokens_error: Option<String>,
}

impl FuncDetail {
    /// Resolve and verify the captured tokens of `code`, reporting addresses in
    /// the program's output units.
    pub fn resolve(
        prog: &ConsoleProgram,
        fd: &Funcdata,
        provenance: &CodeProvenance,
        raw: &[EmitToken],
        untrimmed: &str,
        code: &str,
        variables: &[VarInfo],
    ) -> Self {
        let mut tokens = kuna_srcmap::resolve(fd, provenance, raw, untrimmed, variables);
        if let Err(error) = kuna_srcmap::verify(&tokens, code) {
            return FuncDetail { tokens: Vec::new(), tokens_error: Some(error) };
        }
        for t in &mut tokens {
            t.address = t.address.map(|a| prog.output_code_offset(a));
            t.callee = t.callee.map(|a| prog.output_code_offset(a));
        }
        FuncDetail { tokens, tokens_error: None }
    }
}

/// One loaded section, for mapping an address back to the input file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionRow {
    pub name: String,
    pub address: u64,
    pub size: u64,
    /// Where the section's bytes start in the input file, when they come from it.
    pub file_offset: Option<u64>,
    pub executable: bool,
    pub writable: bool,
}

/// The program's loaded sections in address order: the loader's named table
/// where it publishes one, else its unnamed section spans.
pub fn section_rows(prog: &ConsoleProgram) -> Vec<SectionRow> {
    let row = |name: String, address: u64, size: u64, file_offset, flags: u32| SectionRow {
        name,
        address,
        size,
        file_offset,
        executable: flags & section_flags::CODE != 0,
        writable: flags & section_flags::READONLY == 0,
    };
    let mut rows: Vec<SectionRow> = match prog.image_metadata() {
        Some(meta) if !meta.sections.is_empty() => meta
            .sections
            .iter()
            .filter(|s| s.flags & section_flags::UNALLOC == 0)
            .map(|s| row(s.name.clone(), s.vma, s.size, s.file_offset, s.flags))
            .collect(),
        _ => prog
            .sections()
            .into_iter()
            .filter(|&(_, _, flags)| flags & section_flags::UNALLOC == 0)
            .map(|(vma, size, flags)| row(String::new(), vma, size, None, flags))
            .collect(),
    };
    rows.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));
    rows
}

