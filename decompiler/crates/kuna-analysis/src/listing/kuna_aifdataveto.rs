//! (kuna `aif`) The data read/write veto on a gap entry.
//!
//! A speculative gap entry whose first instruction the decoded code reads or
//! writes as data is a table, a literal or a variable, not a function, however
//! well its bytes decode. [`data_targets`] collects every address the Listing's
//! instructions use as data: a `LOAD`/`STORE` through a constant or a constant
//! plus something (the constant), and every direct data-space operand. A
//! register holding a constant stays known over a short straight-line run, so
//! an AArch64 `adrp`/`add`/`ldrsw` read of an inline jump table counts. The
//! `aif` consumer drops a gap entry whose first instruction spans one of them.
//! An ARM PC-relative literal load is not counted: the word it reads is a
//! literal-pool word, which `poolentry` and `aifpoolslide` own, and its target
//! can be the first word of a real function the Listing never decoded.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use super::xrefs::{decode, FullCapture};
use super::Listing;
use crate::aif::kuna_poolentry::{literal_target, DecodeMode};

/// Every address the decoded code reads or writes as data.
///
/// A register a constant was put in stays known across the next
/// [`CARRY_INSNS`] straight-line instructions until something else writes it,
/// so an AArch64 `adrp xT, P; add xT, xT, #o; ldrsw xD, [xT, xI, lsl #2]`
/// reads the jump table at `P + o`.
pub fn data_targets(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data_space: &Rc<AddrSpace>,
    mode: &DecodeMode,
) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    let is_data = |vn: &VarnodeData| vn.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, data_space));
    let space_type = |vn: &VarnodeData| vn.space.as_ref().map(|s| s.get_type());
    let same_space =
        |a: &VarnodeData, b: &VarnodeData| a.space.as_ref().zip(b.space.as_ref()).is_some_and(|(x, y)| Rc::ptr_eq(x, y));
    let overlaps = |a: &VarnodeData, b: &VarnodeData| {
        same_space(a, b)
            && a.offset < b.offset.saturating_add(u64::from(b.size))
            && b.offset < a.offset.saturating_add(u64::from(a.size))
    };
    let register = |v: &VarnodeData| {
        space_type(v) == Some(spacetype::IPTR_PROCESSOR)
            && !is_data(v)
            && !v.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, code_space))
    };
    let mut cap = FullCapture::default();
    let mut bases: Vec<(VarnodeData, u64, u32)> = Vec::new();
    let mut next_vma: Option<u64> = None;
    for (&vma, insn) in listing.instructions() {
        if next_vma != Some(vma) {
            bases.clear();
        }
        next_vma = vma.checked_add(u64::from(insn.len.max(1)));
        if literal_target(vma, &insn.operands, mode).is_some() && mode.pc_delta(vma).is_some() {
            bases.clear();
            continue;
        }
        if decode(translate, vma, code_space, &mut cap).is_none() {
            bases.clear();
            continue;
        }
        bases.retain_mut(|(_, _, age)| {
            *age += 1;
            *age <= CARRY_INSNS
        });
        let mut ends_run = false;
        for op in cap.ops() {
            let flow = matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL);
            ends_run |= matches!(
                op.opcode,
                OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_CALL
                    | OpCode::CPUI_CALLIND
                    | OpCode::CPUI_CALLOTHER
                    | OpCode::CPUI_RETURN
            );
            for vn in op.ins.iter().skip(usize::from(flow)).chain(op.out.iter()) {
                if is_data(vn) {
                    out.insert(vn.offset);
                }
            }
            let base = |vn: &VarnodeData, bases: &[(VarnodeData, u64, u32)]| {
                if space_type(vn) == Some(spacetype::IPTR_CONSTANT) {
                    return Some(vn.offset);
                }
                bases
                    .iter()
                    .rev()
                    .find(|(b, _, _)| b.offset == vn.offset && b.size == vn.size && same_space(b, vn))
                    .map(|&(_, c, _)| c)
            };
            let value = match op.opcode {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    if let Some(c) = op.ins.get(1).and_then(|p| base(p, &bases)) {
                        out.insert(c);
                    }
                    None
                }
                OpCode::CPUI_COPY => op.ins.first().and_then(|v| base(v, &bases)),
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => {
                    let a = op.ins.first().and_then(|v| base(v, &bases));
                    let b = op.ins.get(1).and_then(|v| base(v, &bases));
                    match (a, b) {
                        (Some(a), Some(b)) if op.opcode == OpCode::CPUI_INT_ADD => Some(a.wrapping_add(b)),
                        (Some(c), _) | (None, Some(c)) => Some(c),
                        (None, None) => None,
                    }
                }
                _ => None,
            };
            if let Some(o) = op.out.as_ref() {
                bases.retain(|(b, _, _)| !overlaps(b, o));
                if let Some(c) = value {
                    bases.push((o.clone(), c, 0));
                }
            }
        }
        if ends_run {
            bases.clear();
            continue;
        }
        bases.retain(|(v, _, _)| register(v));
    }
    out
}

/// How many straight-line instructions a register keeps a constant for in
/// [`data_targets`].
const CARRY_INSNS: u32 = 8;

/// The entries of `entries` whose first instruction (`[e, e + len)`) spans no
/// data target.
pub fn without_data_entries(listing: &Listing, targets: &BTreeSet<u64>, entries: &mut Vec<u64>) {
    entries.retain(|&e| {
        let span = listing.instruction_at(e).map_or(1, |i| u64::from(i.len).max(1));
        targets.range(e..e.saturating_add(span)).next().is_none()
    });
}
