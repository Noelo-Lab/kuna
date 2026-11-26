//! (kuna #1978) What a narrowed project export's `.asm` covers.
//!
//! A `decompile-project` run given `--functions`/`--addr` writes a `.c` of just
//! those functions, so its `.asm` sweeps just their code: each function's
//! inventory extent (its `size`) and every instruction its decompile decoded
//! (its `ranges`), which reach past that extent when the flow does. The spans
//! are clipped to the CODE sections a whole-image sweep walks, so a narrowed
//! `.asm` never disassembles bytes the whole-image one would not. Its data tail
//! labels the named globals that `.c` names, not every one in the image.

use std::collections::BTreeSet;

use crate::engine::ConsoleProgram;
use crate::project::FuncResult;
use crate::project_stream::{code_sections, is_arm, normalize};

/// The byte spans a narrowed `.asm` sweeps for `results`: the union of every
/// result's `[entry, entry + size)` and its decoded `ranges`, merged where they
/// touch and intersected with each CODE section. Ascending and disjoint within
/// a section.
pub fn selection_spans(prog: &ConsoleProgram, results: &[FuncResult]) -> Vec<(u64, u64)> {
    let arm = is_arm(prog);
    let unit = prog.code_unit_bytes();
    let mut wanted: Vec<(u64, u64)> = Vec::new();
    for r in results {
        let entry = normalize(arm, r.byte_address);
        if r.size > 0 {
            wanted.push((entry, entry.saturating_add(r.size as u64)));
        }
        wanted.extend(
            r.ranges.iter().map(|&(start, end)| (start.saturating_mul(unit), end.saturating_mul(unit))),
        );
    }
    let wanted = crate::kuna_coverage::joined(wanted);
    let mut sections = code_sections(prog);
    sections.sort_unstable();
    let mut spans: Vec<(u64, u64)> = Vec::new();
    for (vma, size) in sections {
        let end = vma.saturating_add(size);
        let first = wanted.partition_point(|&(_, stop)| stop <= vma);
        spans.extend(
            wanted[first..]
                .iter()
                .take_while(|&&(start, _)| start < end)
                .map(|&(start, stop)| (start.max(vma), stop.min(end))),
        );
    }
    spans
}

/// The names a narrowed data tail keeps a named global for: every global the
/// selected functions' C declares (`FuncResult::globals`) and every name its
/// text spells, the way the tail already finds the `dat_<hex>` it labels. A
/// token is a run of the characters a symbol name can carry, kept whole and
/// split at `.` and `:` as well, so `obj.field` and `ns::obj` find `obj`.
pub fn referenced_names(results: &[FuncResult], names: &mut BTreeSet<String>) {
    for r in results {
        names.extend(r.globals.iter().map(|g| g.name.clone()));
        let Some(code) = &r.code else { continue };
        for token in code.split(|c: char| !(c.is_ascii_alphanumeric() || "_.$@:?".contains(c))) {
            if token.is_empty() {
                continue;
            }
            for part in token.split(['.', ':']).filter(|part| !part.is_empty() && *part != token) {
                if !names.contains(part) {
                    names.insert(part.to_string());
                }
            }
            if !names.contains(token) {
                names.insert(token.to_string());
            }
        }
    }
}
