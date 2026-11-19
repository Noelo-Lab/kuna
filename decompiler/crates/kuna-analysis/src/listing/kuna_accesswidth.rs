//! (kuna `globalwidth`) The width a program reads and writes each sized data
//! object with.
//!
//! A symbol-table data object (`STT_OBJECT`) has a size but no type, so it is
//! mapped as `undefined<size>`: a `short g[8]` is an opaque 16-byte blob, and
//! every element access renders as `*(short *)&g[i * 2]`. The instructions
//! that use the object say how wide its elements are. This reads that evidence
//! from the decoded Listing: every `LOAD` or `STORE` whose address is a constant or a
//! constant plus something, and every direct data-space operand, that lands
//! inside an object, at the width of the value moved. An object every access of
//! which uses one width `w` that divides its size gets that width; the commit
//! then maps it as `undefined<w>` or an array of `size / w` of them. An object
//! some instruction moves through a floating-point register is left alone: its
//! elements are floats, and an integer element type would hide that.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_floatglobals::FloatRegisters;
use super::xrefs::{decode, FullCapture};
use super::Listing;

/// `(object address, width)` for every object in `objects` (`(addr, size)`)
/// that the decoded code only ever reads or writes at one width dividing its
/// size, and (for a width short of the size) that some access indexes into or
/// lands past the first element of: one `(int)g` read of a `long g` is a
/// narrowing, not an array. `family` is the processor family of the language
/// id (`x86`, `AARCH64`), which names the float registers.
pub fn access_widths(
    listing: &Listing,
    translate: &dyn Translate,
    family: &str,
    code_space: &Rc<AddrSpace>,
    data_space: &Rc<AddrSpace>,
    objects: &[(u64, u64)],
) -> Vec<(u64, u32)> {
    let mut sorted: Vec<(u64, u64)> = objects.iter().copied().filter(|&(_, s)| s > 0).collect();
    sorted.sort_unstable();
    if sorted.is_empty() {
        return Vec::new();
    }
    let owner = |a: u64| -> Option<u64> {
        let i = sorted.partition_point(|&(s, _)| s <= a);
        let (s, size) = *sorted.get(i.checked_sub(1)?)?;
        (a < s + size).then_some(s)
    };
    // width per object (0 = conflicting widths seen), and whether any access
    // indexes into it or lands past its first element
    let mut seen: BTreeMap<u64, (u32, bool)> = BTreeMap::new();
    let floats = FloatRegisters::new(translate, family);
    let mut note = |a: u64, w: u32, indexed: bool, float: bool| {
        if let Some(o) = owner(a) {
            let e = seen.entry(o).or_insert((w, false));
            if e.0 != w || float {
                e.0 = 0;
            }
            e.1 |= indexed || a != o;
        }
    };
    let is_data = |vn: &kuna_num::pcoderaw::VarnodeData| vn.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, data_space));
    let is_const = |vn: &kuna_num::pcoderaw::VarnodeData| {
        vn.space.as_ref().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
    };
    let mut cap = FullCapture::default();
    for (&vma, _) in listing.instructions() {
        if decode(translate, vma, code_space, &mut cap).is_none() {
            continue;
        }
        let ops = cap.ops();
        let float = ops.iter().any(|op| op.ins.iter().chain(op.out.iter()).any(|vn| floats.contains(vn)));
        // temporaries holding a constant, or a constant plus something
        let mut bases: Vec<(&kuna_num::pcoderaw::VarnodeData, u64, bool)> = Vec::new();
        let base_of = |vn: &kuna_num::pcoderaw::VarnodeData, bases: &[(&kuna_num::pcoderaw::VarnodeData, u64, bool)]| -> Option<(u64, bool)> {
            if is_const(vn) {
                return Some((vn.offset, false));
            }
            bases.iter().rev().find(|(b, _, _)| b.space.as_ref().zip(vn.space.as_ref()).is_some_and(|(x, y)| Rc::ptr_eq(x, y)) && b.offset == vn.offset && b.size == vn.size).map(|&(_, c, i)| (c, i))
        };
        for op in ops {
            let flow = matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL);
            for vn in op.ins.iter().skip(usize::from(flow)).chain(op.out.iter()) {
                if is_data(vn) {
                    note(vn.offset, vn.size, false, float);
                }
            }
            match op.opcode {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    let Some(ptr) = op.ins.get(1) else { continue };
                    let width = if op.opcode == OpCode::CPUI_LOAD {
                        op.out.as_ref().map(|o| o.size)
                    } else {
                        op.ins.get(2).map(|v| v.size)
                    };
                    if let (Some((c, indexed)), Some(w)) = (base_of(ptr, &bases), width) {
                        note(c, w, indexed, float);
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => {
                    let Some(out) = op.out.as_ref() else { continue };
                    let c = if op.opcode == OpCode::CPUI_COPY {
                        op.ins.first().and_then(|v| base_of(v, &bases))
                    } else {
                        let known: Vec<(u64, bool)> = op.ins.iter().take(2).filter_map(|v| base_of(v, &bases)).collect();
                        match known.as_slice() {
                            [(a, i), (b, j)] => Some((a.wrapping_add(*b), *i || *j)),
                            [(a, _)] => Some((*a, true)),
                            _ => None,
                        }
                    };
                    if let Some((c, indexed)) = c {
                        bases.push((out, c, indexed));
                    }
                }
                _ => {}
            }
        }
    }
    sorted
        .iter()
        .filter_map(|&(a, size)| {
            let (w, spread) = *seen.get(&a)?;
            let fits = w > 0 && u64::from(w) <= size && size % u64::from(w) == 0;
            (fits && (spread || u64::from(w) == size)).then_some((a, w))
        })
        .collect()
}
