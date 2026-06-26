//! DWARF `.debug_line` source-line mapping — the kuna analog of Ghidra's
//! `DWARFLineInfoCommentScript` (the line-number side of the DWARF analyzer).
//!
//! The sibling [`DwarfPass`](super::DwarfPass) recovers names + typed signatures
//! + stack locals from the `.debug_info`/`.debug_abbrev`/`.debug_str*` sections.
//! This pass reads the **`.debug_line`** program — the PC→`file:line` mapping the
//! compiler emits under `-g` — and surfaces each instruction's source location
//! as a comment on the decompiled output.
//!
//! ## Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `ghidra_scripts/DWARFLineInfoCommentScript.java` — `addSourceLineInfo()`:
//!   for each compilation unit, walk `cu.getLine().getAllSourceFileAddrInfo(cu)`
//!   (the line-table rows) and, per row, `appendComment(addr, EOL,
//!   "%s:%d".formatted(fileName, lineNum))`. This pass is that script's commit,
//!   adapted to kuna's analysis-tier→engine boundary: instead of writing the
//!   program's CommentDB inline, it emits [`CommentFact`]s the console commit
//!   seam installs into `Architecture::commentdb` (the same `Comment::user2`
//!   instruction-comment type the C printer emits as a `/* … */` line).
//! - `ghidra.app.util.bin.format.dwarf.line.DWARFLine.getAllSourceFileAddrInfo`
//!   iterates the line-program state-machine rows (`DWARFLineProgramExecutor`,
//!   the standard DWARF address-advance / `is_stmt` / `end_sequence` opcodes) and
//!   yields one `SourceFileAddr{address, fileName, lineNum, isEndSequence}` per
//!   row. gimli's `LineRows`/`LineRow` is the same state machine; we read it
//!   through gimli's high-level `Unit::line_program` (a dependency-substitution
//!   LOSS identical to the `.debug_info` side — see [`super`] docs / losses.md).
//!
//! ## Faithful divergences (DOC)
//!
//! - **`end_sequence` rows are skipped.** A DWARF `end_sequence` row marks the
//!   address ONE PAST the last instruction of a contiguous code range; its line
//!   number is not a real statement (objdump renders it as `-`). The Ghidra
//!   script keeps these rows (it emits a `file:line` for them too); kuna drops
//!   them so no spurious comment lands at a function's past-the-end address. This
//!   is the only behavioral difference from the script's row set.
//! - **Each row is attached to its OWNING function** by VMA. The kuna comment DB
//!   keys comments by `(funcaddr, instr_addr)` and the C printer only retrieves a
//!   function's comments via `comments_for(funcaddr)` — so a row's `func_addr`
//!   must be the entry VMA of the function whose body contains the row. We gather
//!   the subprogram `[low_pc, high_pc)` ranges from `.debug_info` (the same DIEs
//!   the names pass reads) and assign each row to the range that contains it; a
//!   row outside every known function range is dropped (it has no function to
//!   render against). Ghidra has a flat program-wide CommentDB so it needs no
//!   such bucketing.
//! - **Same-address de-duplication.** Several rows can share one address (a
//!   `view`-split statement, or two source lines folded onto one PC). We keep the
//!   FIRST row seen per `(func, addr)` so the comment is the statement that
//!   begins at that PC, and to keep the output stable.

use object::Object;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, CommentFact, Stage};

use super::{dwarf_section_data, Reader};

/// Port of `DWARFLineInfoCommentScript`: install DWARF `.debug_line` source-line
/// `file:line` annotations as instruction comments. Gated on `dwarf_lines`
/// (default-off — it changes the decompiled output).
pub struct DwarfLinesPass;

/// One subprogram's half-open VMA range `[low, high)` and its entry (= `low`),
/// the bucket a line row is assigned to. `low` doubles as the comment's
/// `func_addr` key.
#[derive(Clone, Copy)]
struct FuncRange {
    low: u64,
    high: u64,
}

impl AnalysisPass for DwarfLinesPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "dwarf_lines"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // No `.debug_line` (or no `.debug_info` to bucket against) => nothing to do.
        // (`DWARFProgram.hasLineInfo` / the script's early return.)
        if dwarf_section_data(ctx.file, gimli::SectionId::DebugLine).is_none() {
            return out;
        }

        let endian = if ctx.file.is_little_endian() {
            gimli::RunTimeEndian::Little
        } else {
            gimli::RunTimeEndian::Big
        };
        let load = |id: gimli::SectionId| -> Result<Vec<u8>, gimli::Error> {
            Ok(dwarf_section_data(ctx.file, id).unwrap_or_default())
        };
        let Ok(sections) = gimli::DwarfSections::load(load) else {
            return out;
        };
        let dwarf = sections.borrow(|bytes| gimli::EndianSlice::new(bytes, endian));

        let mut units = dwarf.units();
        while let Ok(Some(header)) = units.next() {
            let Ok(unit) = dwarf.unit(header) else { continue };
            // 1. The subprogram ranges in this CU (the buckets the rows hang on).
            let ranges = func_ranges(&dwarf, &unit);
            if ranges.is_empty() {
                continue;
            }
            // 2. Walk the line-program rows and emit a `file:line` comment per
            //    statement row, attached to its owning function range.
            collect_line_comments(&dwarf, &unit, &ranges, &mut out.comments);
        }

        // Stable output: sort by (func, addr); a single comment per address (keep
        // the first), so the printed annotation set is deterministic regardless of
        // CU / row iteration order.
        out.comments.sort_by(|a, b| (a.func_addr, a.addr).cmp(&(b.func_addr, b.addr)));
        out.comments.dedup_by(|a, b| a.func_addr == b.func_addr && a.addr == b.addr);
        out
    }
}

/// Collect every defined subprogram's `[low_pc, high_pc)` range in `unit`
/// (`DW_TAG_subprogram` with a `DW_AT_low_pc`; `DW_AT_high_pc` is the
/// DWARF-standard offset-from-low or absolute form). A declaration-only DIE (no
/// low_pc) contributes no range. Mirrors `DWARFFunction.getFuncBodyRanges` — the
/// body extent the importer uses, here used only to bucket line rows.
fn func_ranges(dwarf: &gimli::Dwarf<Reader<'_>>, unit: &gimli::Unit<Reader<'_>>) -> Vec<FuncRange> {
    let mut ranges = Vec::new();
    let mut entries = unit.entries();
    while let Ok(Some(entry)) = entries.next_dfs() {
        if entry.tag() != gimli::DW_TAG_subprogram {
            continue;
        }
        let Some(low) = attr_addr(dwarf, unit, entry, gimli::DW_AT_low_pc) else {
            continue;
        };
        // DW_AT_high_pc: a `DW_FORM_addr` is an absolute end VMA; any constant form
        // is the *size* (offset from low_pc) — the DWARF-4+ convention.
        let high = match entry.attr_value(gimli::DW_AT_high_pc) {
            Some(gimli::AttributeValue::Addr(a)) => a,
            Some(v) => match v.udata_value() {
                Some(size) => low.saturating_add(size),
                None => low.saturating_add(1),
            },
            None => low.saturating_add(1),
        };
        if high > low {
            ranges.push(FuncRange { low, high });
        }
    }
    ranges
}

/// Resolve a `DW_AT_low_pc` (or any address-form attribute) to its VMA, going
/// through gimli's high-level `attr_address` so DWARF-5 `addrx`/`.debug_addr`
/// indirections resolve (the same accessor the names pass uses).
fn attr_addr(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    entry: &gimli::DebuggingInformationEntry<Reader<'_>>,
    name: gimli::DwAt,
) -> Option<u64> {
    match entry.attr_value(name)? {
        gimli::AttributeValue::Addr(a) => Some(a),
        v @ gimli::AttributeValue::DebugAddrIndex(_) => dwarf.attr_address(unit, v).ok().flatten(),
        _ => None,
    }
}

/// Walk the line-program rows for `unit` and append one `file:line` [`CommentFact`]
/// per statement row that falls inside a known function range
/// (`DWARFLineInfoCommentScript.addSourceLineInfo`'s per-row `appendComment`).
fn collect_line_comments(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    ranges: &[FuncRange],
    out: &mut Vec<CommentFact>,
) {
    let Some(program) = unit.line_program.clone() else {
        return;
    };
    let mut rows = program.rows();
    // next_row drives the state machine: (header, row) per emitted matrix row.
    while let Ok(Some((header, row))) = rows.next_row() {
        // end_sequence rows mark the past-the-end address of a code range; not a
        // real statement (see module docs — the one divergence from the script).
        if row.end_sequence() {
            continue;
        }
        let addr = row.address();
        // Bucket the row into the function whose body contains its address. A row
        // outside every known range has no function to render against — drop it.
        let Some(func) = ranges.iter().find(|r| addr >= r.low && addr < r.high) else {
            continue;
        };
        // The source line; a row with line 0 (`DW_LNE_set_address` with no real
        // statement, or compiler-elided) carries no useful location — skip it.
        let Some(line) = row.line() else {
            continue;
        };
        let file = file_name(dwarf, unit, header, row.file_index());
        let text = format!("{file}:{}", line.get());
        out.push(CommentFact { func_addr: func.low, addr, text });
    }
}

/// Resolve a line-row's file index to a printable file name
/// (`DWARFFile.getPathName` / the script's `sfa.fileName()`), the BASENAME only
/// (the comment is `file:line`; the full comp-dir path is noise in a one-line
/// annotation). Falls back to `"<unknown>"` if the index has no file entry.
fn file_name(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    header: &gimli::LineProgramHeader<Reader<'_>>,
    file_index: u64,
) -> String {
    let Some(entry) = header.file(file_index) else {
        return "<unknown>".to_string();
    };
    match dwarf.attr_string(unit, entry.path_name()) {
        Ok(s) => {
            let full = String::from_utf8_lossy(s.slice());
            // Basename: the part after the last '/' (paths are POSIX in DWARF).
            full.rsplit('/').next().unwrap_or(&full).to_string()
        }
        Err(_) => "<unknown>".to_string(),
    }
}

#[cfg(test)]
mod tests;
