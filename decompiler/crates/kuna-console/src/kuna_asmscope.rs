//! (kuna #1978) What a narrowed project export's `.asm` covers.
//!
//! A `decompile-project` run given `--functions`/`--addr` writes a `.c` of just
//! those functions, so its `.asm` sweeps just their code: each function's
//! inventory extent (its `size`) and every instruction its decompile decoded
//! (its `ranges`), which reach past that extent when the flow does. The spans
//! are clipped to the CODE sections a whole-image sweep walks, so a narrowed
//! `.asm` never disassembles bytes the whole-image one would not. Its data tail
//! labels the named globals that `.c` names or that the swept code's operands
//! point into, not every one in the image.

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

/// What a narrowed data tail keeps besides its `dat_<hex>` labels.
#[derive(Debug, Default, Clone)]
pub struct TailScope {
    /// The names the selected functions' C spells ([`referenced_names`]).
    pub names: BTreeSet<String>,
    /// Every constant the swept code's operands print ([`operand_constants`]).
    pub operands: BTreeSet<u64>,
}

impl TailScope {
    /// How a narrowed tail keeps the named global `name`, `size` bytes from
    /// `starts` (its displayed and its byte coordinate): 2 when the C names it,
    /// 1 when an operand of the swept code points into it, 0 when it is left out.
    pub(crate) fn rank(&self, name: &str, starts: [u64; 2], size: u64) -> u8 {
        if self.names.contains(name) {
            return 2;
        }
        let end = |start: u64| start.saturating_add(size.max(1));
        u8::from(starts.into_iter().any(|start| self.operands.range(start..end(start)).next().is_some()))
    }
}

/// Add every `0x<hex>` constant one instruction's operand text spells to `out`:
/// the addresses a reader of the `.asm` follows into its data tail.
pub(crate) fn operand_constants(operands: &str, out: &mut BTreeSet<u64>) {
    let mut rest = operands;
    while let Some(at) = rest.find("0x") {
        rest = &rest[at + 2..];
        let digits = rest.bytes().take_while(u8::is_ascii_hexdigit).count();
        if let Ok(value) = u64::from_str_radix(&rest[..digits], 16) {
            out.insert(value);
        }
        rest = &rest[digits..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operand_constants_are_every_hex_number_the_text_spells() {
        let mut found = BTreeSet::new();
        operand_constants("RDX,qword ptr [0x601048]", &mut found);
        operand_constants("r0,[pc,#0x20] ; 0x0", &mut found);
        operand_constants("byte ptr [RBP + -0x18],0x0 0x", &mut found);
        operand_constants("EAX,0x10000000000000000", &mut found);
        assert_eq!(found.into_iter().collect::<Vec<_>>(), [0x0, 0x18, 0x20, 0x601048]);
    }

    #[test]
    fn a_tail_keeps_what_the_c_names_first_and_what_an_operand_points_into() {
        let scope = TailScope {
            names: BTreeSet::from(["sneaky".to_string()]),
            operands: BTreeSet::from([0x400915, 0x601054]),
        };
        assert_eq!(scope.rank("sneaky", [0x9000, 0x9000], 8), 2);
        assert_eq!(scope.rank("s_400915", [0x400915, 0x400915], 11), 1);
        assert_eq!(scope.rank("table", [0x601050, 0x601050], 8), 1);
        assert_eq!(scope.rank("table", [0x601050, 0x601050], 4), 0);
        assert_eq!(scope.rank("unsized", [0x400915, 0x400915], 0), 1);
        assert_eq!(scope.rank("elsewhere", [0x1000, 0x400915], 1), 1);
    }
}
