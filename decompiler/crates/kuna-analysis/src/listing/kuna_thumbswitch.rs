//! (kuna) `thumbswitch` — read a Thumb `tbb`/`tbh` switch table, so the walk
//! decodes the case bodies of the function that dispatches to them (GH-299).
//!
//! # The gap
//!
//! [`super::walk`] gives a `BRANCHIND` no successor, so the walk stops at every
//! switch dispatch and leaves the case bodies undecoded. On Cortex-M firmware
//! the dispatch is almost always `tbb [pc,rN]` or `tbh [pc,rN,lsl #1]`, a jump
//! through a table of byte or halfword offsets that sits right after the
//! instruction:
//!
//! ```text
//! cmp   r0,#0x3
//! bhi   default
//! tbb   [pc,r0]          ; pc = inst_next + 2 * table[r0]
//! .byte 2, 5, 9, 11      ; the table, at inst_next
//! case0: ...
//! ```
//!
//! The holes that leaves are what the AIF gap walk (`aif`) probes, and a case
//! body that opens with a common prologue and ends in its own return becomes a
//! `sub_<addr>` in the middle of the switch. Over 33 Cortex-M decbench images
//! those case bodies were the largest single source of mid-body AIF entries.
//!
//! # The rule
//!
//! [`HalfwordTables::targets`] folds the dispatch's own p-code with the
//! [`super::kuna_switchtable`] lattice, which answers only for the PC-relative
//! form: a byte or halfword read at `inst_next + index`, doubled and added back
//! to the same `inst_next`. The table states no length, so the count comes from
//! the range check right before it: a `cmp index,#n` (lifted to
//! `INT_LESSEQUAL(n, index)`) whose carry a conditional branch to the default
//! case reads, with nothing between them writing `index`; the table then has
//! `n + 1` entries behind a `bhi`/`bls` and `n` behind a `bcs`/`bcc`. A
//! dispatch with no such check, or behind a branch on any other flags (a
//! signed `bge`, which a negative index passes), reads nothing. Over the
//! jump-table ceiling (`jumptablemax`) the first entries are read and the
//! xrefs walk records the switch as truncated, as it does any other table. Every entry is
//! a forward offset, and the cases follow the table, so a table whose entries
//! land inside itself or outside the dispatcher's section is not read at all.
//!
//! The entries become same-function successors of the dispatch, as a branch
//! target is, and a code reference from it.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{
    fold, invalidate, is_constant, key_of, keys_overlap, value_of, Key, Val, DEFAULT_MAX_ENTRIES,
    MAX_TRACKED, MIN_ENTRIES,
};
use super::xrefs::{decode, in_range, FullCapture};

/// How far back from a `tbb`/`tbh` its range check may sit: the conditional
/// branch and the `cmp`, with room for one instruction scheduled between them.
const MAX_GUARD_INSNS: usize = 3;

/// The read-only image a Thumb `tbb`/`tbh` reads its offsets from.
pub(crate) struct HalfwordTables<'a> {
    pool: PoolImage<'a>,
    data_space: Option<Rc<AddrSpace>>,
    max_entries: usize,
}

impl<'a> HalfwordTables<'a> {
    /// `None` for an image with no ARM code or no read-only content, and for a
    /// relocatable object, whose sections are not where the code runs.
    pub(crate) fn new(
        file: &'a object::File<'a>,
        data_space: Option<Rc<AddrSpace>>,
        max_entries: usize,
    ) -> Option<Self> {
        use object::Object;
        if file.architecture() != object::Architecture::Arm
            || file.kind() == object::ObjectKind::Relocatable
        {
            return None;
        }
        let max_entries = if max_entries == 0 { DEFAULT_MAX_ENTRIES } else { max_entries };
        Some(HalfwordTables { pool: PoolImage::new(file)?, data_space, max_entries })
    }

    /// The case bodies of the computed jump at `from` when it is a PC-relative
    /// `tbb`/`tbh` behind a range check, and the case count that check states;
    /// nothing for every other computed jump. `decoded` answers which addresses
    /// the walk has decoded, which is where the range check is looked for. A
    /// table longer than the jump-table ceiling yields its first entries only.
    pub(crate) fn targets(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        exec: &[(u64, u64)],
        decoded: &dyn Fn(u64) -> bool,
        from: u64,
    ) -> (Vec<u64>, Option<usize>) {
        let mut cap = FullCapture::default();
        if decode(translate, from, code_space, &mut cap).is_none() {
            return (Vec::new(), None);
        }
        let data = self.data_space.as_ref();
        let mut vals: Vec<(Key, Val)> = Vec::new();
        let mut index: Option<Key> = None;
        for op in cap.ops() {
            if op.opcode == OpCode::CPUI_BRANCHIND {
                let Some(Val::Short { table, width, base }) =
                    op.ins.first().and_then(|vn| value_of(vn, &vals, data))
                else {
                    return (Vec::new(), None);
                };
                let Some(index) = index.filter(|_| table == base) else { return (Vec::new(), None) };
                let Some(cases) = guard_cases(translate, code_space, data, decoded, from, index)
                    .filter(|&n| n >= MIN_ENTRIES)
                else {
                    return (Vec::new(), None);
                };
                let read = offset_targets(table, width, cases, self.max_entries, from, &self.pool, exec);
                return if read.is_empty() { (read, None) } else { (read, Some(cases)) };
            }
            if index.is_none()
                && matches!(op.opcode, OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT)
            {
                index = op
                    .ins
                    .iter()
                    .filter(|vn| value_of(vn, &vals, data).is_none())
                    .find_map(|vn| key_of(vn, data).filter(|k| matches!(k, Key::Reg(..))));
            }
            let folded = fold(op, &vals, data, &self.pool);
            let Some(out) = &op.out else { continue };
            let key = key_of(out, data);
            invalidate(&mut vals, key);
            if let (Some(key), Some(val)) = (key, folded) {
                if vals.len() < MAX_TRACKED {
                    vals.push((key, val));
                }
            }
        }
        (Vec::new(), None)
    }
}

/// The case count the range check ahead of the dispatch at `from` admits, or
/// `None` when the instructions just before it do not compare `index` with a
/// constant and then branch, or write `index` after the comparison.
///
/// `cmp index,#n` sets the carry when `n <= index`. The branch on it reads
/// exactly the carry and the zero flag when it is `bhi`/`bls`, which keeps
/// `index == n` in range (`n + 1` cases), and the carry alone when it is
/// `bcs`/`bcc`, which does not (`n` cases). A branch on any other flags (a
/// signed `bge`/`bgt`, which a negative index passes) bounds nothing.
fn guard_cases(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &dyn Fn(u64) -> bool,
    from: u64,
    index: Key,
) -> Option<usize> {
    let mut cap = FullCapture::default();
    let mut cur = from;
    let mut inclusive: Option<bool> = None;
    for _ in 0..MAX_GUARD_INSNS {
        let prev = [2, 4].iter().filter_map(|&d| cur.checked_sub(d)).find(|&at| decoded(at))?;
        let len = decode(translate, prev, code_space, &mut cap)?;
        if prev + u64::from(len) != cur {
            return None;
        }
        let ops = cap.ops();
        if let Some(inclusive) = inclusive {
            let bound = ops.iter().find_map(|op| {
                let (n, reg) = (op.ins.first()?, op.ins.get(1)?);
                (op.opcode == OpCode::CPUI_INT_LESSEQUAL
                    && is_constant(n)
                    && key_of(reg, data) == Some(index))
                .then_some(n.offset)
            });
            if let Some(n) = bound {
                return usize::try_from(n).ok()?.checked_add(usize::from(inclusive));
            }
        }
        let writes_index = ops.iter().any(|op| {
            op.out.as_ref().and_then(|o| key_of(o, data)).is_some_and(|k| keys_overlap(k, index))
        });
        if writes_index {
            return None;
        }
        if inclusive.is_none() && ops.iter().any(|op| op.opcode == OpCode::CPUI_CBRANCH) {
            let mut flags: Vec<String> = ops
                .iter()
                .flat_map(|op| op.ins.iter())
                .filter(|vn| vn.size == 1 && matches!(key_of(vn, data), Some(Key::Reg(..))))
                .filter_map(|vn| {
                    Some(translate.get_register_name(vn.space.as_ref()?, vn.offset, 1))
                })
                .collect();
            flags.sort_unstable();
            flags.dedup();
            inclusive = Some(match flags.iter().map(String::as_str).collect::<Vec<_>>()[..] {
                ["CY", "ZR"] => true,
                ["CY"] => false,
                _ => return None,
            });
        }
        cur = prev;
    }
    None
}

/// The first `limit` of the `cases` targets of a `width`-byte offset table at
/// `table`, each `table + 2 * entry`; empty when any of them lands before the
/// end of the whole table or outside the executable range holding `from`.
fn offset_targets(
    table: u64,
    width: u32,
    cases: usize,
    limit: usize,
    from: u64,
    pool: &PoolImage,
    exec: &[(u64, u64)],
) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let step = u64::from(width);
    let Some(end) = (cases as u64).checked_mul(step).and_then(|n| table.checked_add(n)) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(cases.min(limit));
    for at in (table..end).step_by(step as usize).take(limit) {
        let Some(entry) = pool.uword_at(at, width) else { return Vec::new() };
        let target = table.wrapping_add(entry * 2);
        if target < end || !in_range(&[section], target) {
            return Vec::new();
        }
        out.push(target);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `tbb` at 0x1000; its table at 0x1004: four byte offsets, then code.
    const BYTES: [u8; 8] = [0x02, 0x05, 0x09, 0x0b, 0x0a, 0x20, 0x01, 0x30];
    const EXEC: [(u64, u64); 1] = [(0x1000, 0x1100)];

    fn image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x1004, 0x100c, &BYTES[..])], 4, true).unwrap()
    }

    #[test]
    fn a_byte_table_yields_each_case_after_it() {
        assert_eq!(
            offset_targets(0x1004, 1, 4, 4, 0x1000, &image(), &EXEC),
            vec![0x1008, 0x100e, 0x1016, 0x101a]
        );
    }

    #[test]
    fn a_halfword_table_reads_little_endian_pairs() {
        let exec = [(0x1000, 0x3000)];
        assert_eq!(offset_targets(0x1004, 2, 2, 2, 0x1000, &image(), &exec), vec![0x1a08, 0x2616]);
    }

    /// A case outside the dispatcher's own executable range is not a case.
    #[test]
    fn a_target_outside_the_section_refuses_the_read() {
        assert!(offset_targets(0x1004, 2, 2, 2, 0x1000, &image(), &EXEC).is_empty());
    }

    /// One more entry than the table has reads the first case's code as an
    /// offset back into the table: the whole read is refused.
    #[test]
    fn an_entry_inside_the_table_refuses_the_read() {
        let bytes = [0x02, 0x01, 0x09, 0x0b];
        let pool = PoolImage::from_ranges(vec![(0x1004, 0x1008, &bytes[..])], 4, true).unwrap();
        assert!(offset_targets(0x1004, 1, 4, 4, 0x1000, &pool, &EXEC).is_empty());
    }

    /// Over the jump-table ceiling only the first entries are taken, and they are
    /// still checked against the end of the whole table.
    #[test]
    fn a_ceiling_reads_the_first_entries() {
        assert_eq!(offset_targets(0x1004, 1, 4, 2, 0x1000, &image(), &EXEC), vec![0x1008, 0x100e]);
        let bytes = [0x01, 0x05, 0x09, 0x0b];
        let pool = PoolImage::from_ranges(vec![(0x1004, 0x1008, &bytes[..])], 4, true).unwrap();
        assert!(offset_targets(0x1004, 1, 4, 2, 0x1000, &pool, &EXEC).is_empty());
    }

    #[test]
    fn a_table_past_the_read_only_image_reads_nothing() {
        assert!(offset_targets(0x1004, 1, 12, 12, 0x1000, &image(), &[(0x1000, 0x2000)]).is_empty());
    }
}
