//! (kuna `aif`) The data read/write veto on a gap entry.
//!
//! A speculative gap entry whose first instruction the decoded code reads or
//! writes as data is a table, a literal or a variable, not a function, however
//! well its bytes decode. [`data_targets`] collects every address the Listing's
//! instructions use as data: a `LOAD`/`STORE` through a constant or a constant
//! plus something (the constant), and every direct data-space operand. The
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
pub fn data_targets(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data_space: &Rc<AddrSpace>,
    mode: &DecodeMode,
) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    let is_data = |vn: &VarnodeData| vn.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, data_space));
    let is_const =
        |vn: &VarnodeData| vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT);
    let same = |a: &VarnodeData, b: &VarnodeData| {
        a.offset == b.offset
            && a.size == b.size
            && a.space.as_ref().zip(b.space.as_ref()).is_some_and(|(x, y)| Rc::ptr_eq(x, y))
    };
    let mut cap = FullCapture::default();
    for (&vma, insn) in listing.instructions() {
        if literal_target(vma, &insn.operands, mode).is_some() && mode.pc_delta(vma).is_some() {
            continue;
        }
        if decode(translate, vma, code_space, &mut cap).is_none() {
            continue;
        }
        // temporaries holding a constant, or a constant plus something
        let mut bases: Vec<(&VarnodeData, u64)> = Vec::new();
        for op in cap.ops() {
            let flow = matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL);
            for vn in op.ins.iter().skip(usize::from(flow)).chain(op.out.iter()) {
                if is_data(vn) {
                    out.insert(vn.offset);
                }
            }
            let base = |vn: &VarnodeData, bases: &[(&VarnodeData, u64)]| {
                if is_const(vn) {
                    return Some(vn.offset);
                }
                bases.iter().rev().find(|(b, _)| same(b, vn)).map(|&(_, c)| c)
            };
            match op.opcode {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    if let Some(c) = op.ins.get(1).and_then(|p| base(p, &bases)) {
                        out.insert(c);
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => {
                    let Some(o) = op.out.as_ref() else { continue };
                    let take = if op.opcode == OpCode::CPUI_COPY { 1 } else { 2 };
                    if let Some(c) = op.ins.iter().take(take).find_map(|v| base(v, &bases)) {
                        bases.push((o, c));
                    }
                }
                _ => {}
            }
        }
    }
    out
}

/// The entries of `entries` whose first instruction (`[e, e + len)`) spans no
/// data target.
pub fn without_data_entries(listing: &Listing, targets: &BTreeSet<u64>, entries: &mut Vec<u64>) {
    entries.retain(|&e| {
        let span = listing.instruction_at(e).map_or(1, |i| u64::from(i.len).max(1));
        targets.range(e..e.saturating_add(span)).next().is_none()
    });
}
