//! What a single-function study view needs beyond the batch record: the token
//! source map of the rendered C ([`FuncDetail`]), the function's instruction
//! listing ([`function_rows`]), its references ([`function_xrefs`]) and the
//! image's section table with file offsets ([`section_rows`]).

use std::collections::BTreeMap;

use kuna_decomp::decompile_drive::{CodeProvenance, VarInfo};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_srcmap::{self, CodeToken};
use kuna_decomp::prettyprint::EmitToken;
use kuna_sleigh::loadimage::section_flags;

use crate::disasm::{self, mapped_run_end, Row, Window};
use crate::engine::ConsoleProgram;

/// How many rows [`function_rows`] lists before it reports a truncated listing.
pub const FUNCTION_ROW_CAP: usize = 4096;

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
/// where it publishes one, else its unnamed section spans. A section is
/// writable when the segment mapping it is (the loader leaves the READONLY bit
/// off the loader-owned tables on purpose), else when its own flags say so.
pub fn section_rows(prog: &ConsoleProgram) -> Vec<SectionRow> {
    let segments = prog.segments();
    let row = |name: String, address: u64, size: u64, file_offset, flags: u32| {
        let mapping = segments
            .iter()
            .find(|&&(start, len, _)| address >= start && address - start < len)
            .map(|&(_, _, seg)| seg);
        SectionRow {
            name,
            address,
            size,
            file_offset,
            executable: flags & section_flags::CODE != 0,
            writable: mapping.unwrap_or(flags) & section_flags::READONLY == 0,
        }
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

/// The instruction listing of the function at `entry` spanning `size` bytes:
/// the CLI's `disassemble --follow` walk (branch targets start rows, literal
/// pools fold, undecodable bytes are `.byte` rows), at most `cap` rows. A
/// `size` of 0 (no known extent) lists forward to the end of the mapped run.
/// Returns the rows and whether the listing stopped short of the extent.
pub fn function_rows(prog: &ConsoleProgram, entry: u64, size: u64, cap: usize) -> (Vec<Row>, bool) {
    if !prog.vma_bytes_mapped(entry) {
        return (Vec::new(), false);
    }
    let window = Window {
        start: entry,
        end: (size > 0).then(|| entry.saturating_add(size)),
        derived: false,
        mapped_end: mapped_run_end(prog, entry).map(|(end, _)| end),
    };
    let walk = disasm::walk(prog, &window, Some(cap), true);
    let listed_to = walk.rows.last().map_or(entry, |r| r.addr.saturating_add(r.size));
    let short = window.stop().is_none_or(|stop| listed_to < stop) && walk.rows.len() >= cap;
    (walk.rows, walk.truncated || short)
}

/// One reference row: the other end's name and address, the instruction that
/// makes the reference, its kind (`call`, `jump`, `data`, `read`, `write`), and
/// that instruction's text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrefRow {
    /// The caller for a caller row, the target for a callee or data row.
    pub name: Option<String>,
    pub address: u64,
    /// The referencing instruction.
    pub site: u64,
    pub kind: &'static str,
    pub instruction: String,
}

/// Who calls or jumps to a function, what it calls or jumps to, and the data it
/// reads, writes or takes the address of.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionXrefs {
    pub callers: Vec<XrefRow>,
    pub callees: Vec<XrefRow>,
    pub data_refs: Vec<XrefRow>,
}

/// The references of the function at `entry`, from the `kuna xrefs` walk
/// (`kuna_analysis::listing::xrefs`) seeded with the program's inventory and
/// focused on `entry`. Callers are named by the function holding the call
/// site; targets by the function or global data symbol at them; a jump that
/// stays inside the function is control flow, not a callee. Addresses are the
/// program's byte addresses.
pub fn function_xrefs(prog: &ConsoleProgram, file: &object::File, entry: u64) -> FunctionXrefs {
    use kuna_analysis::listing::xrefs::{self as xr, XrefKind};
    let entries = prog.function_entries_canonical();
    let names: BTreeMap<u64, String> =
        entries.iter().map(|e| (e.addr.get_offset(), e.name.clone())).collect();
    let data: BTreeMap<u64, String> =
        prog.global_data_symbols().into_iter().map(|(name, vma, _)| (vma, name)).collect();
    let inventory: Vec<u64> = names.keys().copied().collect();
    let seeds = xr::discovery_seeds(file, &inventory, prog.arch().analysis_funcstart_patterns);
    let index = xr::build_with_focus(file, prog.arch(), prog.arch().translate(), &seeds, &[entry]);
    let name_of = |vma: u64| names.get(&vma).or_else(|| data.get(&vma)).cloned();
    let mut out = FunctionXrefs::default();
    for r in index.refs_to_unified(entry) {
        let caller = index.function_containing(r.from);
        out.callers.push(XrefRow {
            name: caller.and_then(name_of),
            address: caller.unwrap_or(r.from),
            site: r.from,
            kind: r.kind.as_str(),
            instruction: r.instruction.clone(),
        });
    }
    for r in index.refs_from_function(entry) {
        if r.kind == XrefKind::Jump && index.function_containing(r.to) == Some(entry) {
            continue;
        }
        let row = XrefRow {
            name: name_of(r.to),
            address: r.to,
            site: r.from,
            kind: r.kind.as_str(),
            instruction: r.instruction.clone(),
        };
        match r.kind {
            XrefKind::Call | XrefKind::Jump => out.callees.push(row),
            XrefKind::Data | XrefKind::Read | XrefKind::Write => out.data_refs.push(row),
        }
    }
    out.callees.sort_by_key(|r| (r.site, r.address));
    out.data_refs.sort_by_key(|r| (r.site, r.address));
    out
}
