//! (kuna `armframes`) Follow an A32 jump table in the walk, so the case blocks of
//! a switch belong to the function that dispatches to them.
//!
//! GCC lowers an A32 switch to a predicated computed jump right after the range
//! check, with its table immediately after the default branch:
//!
//! ```text
//! cmp    r3,#21
//! ldrls  pc,[pc,r3,lsl #2]    ; a table of 22 case addresses at inst + 8
//! b      default
//! .word  case0, case1, ...
//! ```
//!
//! or, for a table of branches, `addls pc,pc,r3,lsl #2` followed by
//! `b default` and one `b caseN` per case. The walk gives a computed jump no
//! successor, so every case block stays undefined. Frame recovery then exposes
//! those blocks to the AIF gap walk, whose fingerprint histogram, now drawn
//! from the whole program, accepts them as function entries and cuts the
//! dispatcher apart.
//!
//! [`A32Tables::targets`] folds the dispatch's p-code with the
//! [`super::kuna_switchtable`] lattice and answers only for these two
//! PC-relative shapes, whose table starts exactly 8 bytes past the dispatch.
//! The case count comes from the `cmp index,#n` before it, read the way
//! [`super::kuna_thumbswitch`] reads a `tbb` guard, with the dispatch's own
//! condition as the range branch when it is predicated. Without that guard
//! nothing is read. Word tables are confined to the dispatcher's executable
//! section; branch-table slots are the instructions themselves.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{
    fold, invalidate, is_constant, key_of, value_of, Key, Val, MAX_TRACKED, MIN_ENTRIES,
};
use super::xrefs::{decode, in_range, FullCapture, FullOp};

/// The A32 PC reads 8 bytes past the instruction, which is where both table
/// forms begin.
const A32_PC_OFFSET: u64 = 8;

/// The read-only image an A32 word table is read from.
pub(crate) struct A32Tables<'a> {
    pool: PoolImage<'a>,
    data_space: Option<Rc<AddrSpace>>,
    max_entries: usize,
}

impl<'a> A32Tables<'a> {
    /// `None` unless `armframes` recovery is on for a linked ARM image.
    pub(crate) fn new(
        file: &'a object::File<'a>,
        arch: &kuna_decomp::architecture::Architecture,
    ) -> Option<Self> {
        use object::Object;
        if !arch.analysis_armframes
            || file.architecture() != object::Architecture::Arm
            || file.kind() == object::ObjectKind::Relocatable
        {
            return None;
        }
        let max_entries = match arch.max_jumptable_size as usize {
            0 => super::kuna_switchtable::DEFAULT_MAX_ENTRIES,
            n => n,
        };
        Some(A32Tables {
            pool: PoolImage::new(file)?,
            data_space: arch.manage().get_default_data_space().cloned(),
            max_entries,
        })
    }

    /// The case blocks of the computed jump at `from` when it is a guarded A32
    /// `ldr pc,[pc,rN,lsl #2]` or `add pc,pc,rN,lsl #2`; nothing otherwise.
    pub(crate) fn targets(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        exec: &[(u64, u64)],
        decoded: &dyn Fn(u64) -> bool,
        from: u64,
    ) -> Vec<u64> {
        let mut cap = FullCapture::default();
        if decode(translate, from, code_space, &mut cap) != Some(4) {
            return Vec::new();
        }
        let data = self.data_space.as_ref();
        let base = from + A32_PC_OFFSET;
        let mut vals: Vec<(Key, Val)> = Vec::new();
        let mut index: Option<Key> = None;
        let mut predicated = false;
        for op in cap.ops() {
            if op.opcode == OpCode::CPUI_CBRANCH {
                predicated = true;
            }
            if op.opcode == OpCode::CPUI_BRANCHIND {
                let shape = op.ins.first().and_then(|vn| value_of(vn, &vals, data));
                let words = match shape {
                    Some(Val::Entry { table, width: 4 }) if table == base => true,
                    Some(Val::Indexed(table)) if table == base => false,
                    _ => return Vec::new(),
                };
                let Some(index) = index else { return Vec::new() };
                let Some(cases) = super::kuna_thumbswitch::guard_cases(
                    translate, code_space, data, decoded, from, index, predicated,
                )
                .filter(|&n| n >= MIN_ENTRIES)
                else {
                    return Vec::new();
                };
                let cases = cases.min(self.max_entries);
                return if words {
                    super::kuna_switchtable::targets(base, from, &self.pool, exec, Some(cases), cases)
                } else {
                    branch_slots(base, cases, from, exec)
                };
            }
            if index.is_none()
                && matches!(op.opcode, OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_ADD)
            {
                index = op
                    .ins
                    .iter()
                    .filter(|vn| value_of(vn, &vals, data).is_none())
                    .find_map(|vn| key_of(vn, data).filter(|k| matches!(k, Key::Reg(..))));
            }
            let folded = interworking(op, &vals, data).or_else(|| fold(op, &vals, data, &self.pool));
            let Some(out) = &op.out else { continue };
            let key = key_of(out, data);
            invalidate(&mut vals, key);
            if let (Some(key), Some(val)) = (key, folded) {
                if vals.len() < MAX_TRACKED {
                    vals.push((key, val));
                }
            }
        }
        Vec::new()
    }
}

/// An A32 write to the PC clears bit 0 of an interworking target, which leaves
/// a table entry or an indexed address as it was.
fn interworking(op: &FullOp, vals: &[(Key, Val)], data: Option<&Rc<AddrSpace>>) -> Option<Val> {
    if op.opcode != OpCode::CPUI_INT_AND {
        return None;
    }
    let (value, mask) = (op.ins.first()?, op.ins.get(1)?);
    if !is_constant(mask) || mask.offset != 0xffff_fffe {
        return None;
    }
    match value_of(value, vals, data)? {
        v @ (Val::Entry { .. } | Val::Indexed(_)) => Some(v),
        _ => None,
    }
}

/// The `cases` branch slots of an `add pc,pc,rN,lsl #2` table at `base`, all
/// inside the dispatcher's executable section.
fn branch_slots(base: u64, cases: usize, from: u64, exec: &[(u64, u64)]) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let slots: Vec<u64> = (0..cases as u64).map(|i| base + 4 * i).collect();
    if slots.iter().all(|&at| in_range(&[section], at)) {
        slots
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_slots_follow_the_default_branch() {
        assert_eq!(branch_slots(0x1008, 3, 0x1000, &[(0x1000, 0x1100)]), vec![0x1008, 0x100c, 0x1010]);
    }

    #[test]
    fn branch_slots_past_the_section_read_nothing() {
        assert!(branch_slots(0x10f8, 4, 0x1000, &[(0x1000, 0x1100)]).is_empty());
    }
}
