//! (kuna #1288) The MS-DOS interrupts that end a program.
//!
//! `INT 20h` and `INT 27h` terminate a DOS program, and so does `INT 21h` with
//! AH = 00h (terminate), 31h (terminate and stay resident) or 4Ch (terminate
//! with a return code). Control never comes back from one, but x86 SLEIGH
//! lowers every `INT n` to a call through the `swi` userop that falls through,
//! so the decompiler followed the flow past `mov ah,4ch; int 21h` into whatever
//! the bytes after it decode as. [`analyze`] finds the sites, which the
//! decompiler's flow ends at, and the functions every path of which reaches one
//! (Turbo C's `_exit`, then `exit` that calls it), which are no-return to their
//! callers.
//!
//! AH is the constant the code writes on the one path into the interrupt:
//! walking back along fall-through edges from the `INT 21h`, the nearest
//! instruction that writes AH decides (`mov ah,4ch`, `mov ax,4c00h`, `xor
//! ah,ah`), and the walk gives up at a call or interrupt, at a write it cannot
//! evaluate, at an instruction a branch, a call, a switch table or a second
//! fall-through enters, and after [`MAX_BACK`] instructions. Only the x86
//! real-mode language has DOS below it, so every other language returns nothing.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::listing::Listing;
use crate::pass::NoReturnFact;

/// The instructions the walk back from an `INT 21h` reads before it gives up.
pub const MAX_BACK: usize = 16;

/// The longest x86 instruction, which bounds where a fall-through into an
/// address can start.
const MAX_INSN_LEN: u64 = 15;

/// The DOS exits [`analyze`] found.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DosExits {
    /// The address of every exiting interrupt, sorted.
    pub sites: Vec<u64>,
    /// The functions no path of which returns once those interrupts end theirs.
    pub noreturn: Vec<NoReturnFact>,
}

/// Is `archid` the x86 real-mode language, the one a DOS program runs in?
pub fn is_real_mode(archid: &str) -> bool {
    archid.starts_with("x86:LE:16:Real Mode")
}

/// The DOS exits of the code `listing` decoded. The no-return functions follow
/// the `noreturn_propagate` gate, and its `noreturn_reach` rule when that is
/// on; the sites are a fact of the program and are always found.
pub fn analyze(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    listing: &Listing,
) -> DosExits {
    if !is_real_mode(&arch.archid) {
        return DosExits::default();
    }
    let sites = exit_sites(arch, translate, code_space, listing);
    let noreturn = if sites.is_empty() || !arch.analysis_noreturn_propagate {
        Vec::new()
    } else {
        crate::analyzers::noreturn_propagate::propagate_exits(
            listing,
            &sites,
            arch.analysis_noreturn_reach,
        )
    };
    DosExits { sites, noreturn }
}

/// The address of every interrupt in `listing` that ends the program.
fn exit_sites(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    listing: &Listing,
) -> Vec<u64> {
    let Some(swi) = arch.userops.get_op_by_name(b"swi").map(|op| op.get_index() as u64) else {
        return Vec::new();
    };
    let Ok(ah) = translate.get_register_data("AH") else {
        return Vec::new();
    };
    let mut entered: Option<BTreeSet<u64>> = None;
    let mut sites = Vec::new();
    for (&vma, insn) in listing.instructions() {
        if !(insn.flow.is_call && insn.flow.is_computed) {
            continue;
        }
        let Some(ops) = decode(translate, code_space, vma) else {
            continue;
        };
        let exits = match interrupt_vector(&ops, swi) {
            Some(0x20 | 0x27) => true,
            Some(0x21) => {
                let entered =
                    entered.get_or_insert_with(|| entered_addresses(arch, translate, code_space, listing));
                matches!(
                    ah_at(translate, code_space, listing, entered, &ah, vma),
                    Some(0x00 | 0x31 | 0x4c)
                )
            }
            _ => false,
        };
        if exits {
            sites.push(vma);
        }
    }
    sites
}

/// Every address control reaches other than by falling through: function
/// entries, static branch and call targets, and every word of a table an
/// unresolved computed branch reads (the Listing does not resolve a real-mode
/// switch, which the decompiler does).
fn entered_addresses(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    listing: &Listing,
) -> BTreeSet<u64> {
    let mut entered: BTreeSet<u64> = listing.functions().map(|(&entry, _)| entry).collect();
    for (_, insn) in listing.instructions() {
        entered.extend(insn.flows.iter().copied());
    }
    let Some(segment) = arch.userops.get_op_by_name(b"segment").map(|op| op.get_index() as u64) else {
        return entered;
    };
    let (Some(lo), Some(hi)) = (
        listing.instructions().next().map(|(&vma, _)| vma),
        listing.instructions_in_range(0, None).next_back().map(|(&vma, _)| vma),
    ) else {
        return entered;
    };
    for (&vma, insn) in listing.instructions() {
        if !insn.flow.is_computed || !insn.flows.is_empty() {
            continue;
        }
        let Some(offset) = decode(translate, code_space, vma).and_then(|ops| table_offset(&ops, segment))
        else {
            continue;
        };
        let at = Address::new(Rc::clone(code_space), vma);
        let cs = arch.with_context_db(|db| db.get_variable_value(b"realCS", &at)).unwrap_or(0);
        entered.extend(table_targets(arch, code_space, u64::from(cs) << 4, offset, lo, hi));
    }
    entered
}

/// The most words read from one table.
const MAX_TABLE: u64 = 256;

/// The segment offset of the memory a computed branch's p-code `ops` loads its
/// target from: the constant in the offset operand of the `segment` op its
/// `LOAD` addresses through (`jmp cs:[bx+table]`).
fn table_offset(ops: &[Op], segment: u64) -> Option<u64> {
    let load = ops.iter().find(|op| op.opcode == OpCode::CPUI_LOAD && op.ins.len() == 2)?;
    let at = ops.iter().position(|op| {
        op.opcode == OpCode::CPUI_CALLOTHER
            && op.ins.len() == 3
            && is_const(&op.ins[0])
            && op.ins[0].offset == segment
            && op.out.as_ref().is_some_and(|out| same(out, &load.ins[1]))
    })?;
    let offset = &ops[at].ins[2];
    if is_const(offset) {
        return Some(offset.offset);
    }
    if !is_unique(offset) {
        return None;
    }
    let def = ops[..at].iter().rev().find(|op| op.out.as_ref().is_some_and(|out| same(out, offset)))?;
    match def.opcode {
        OpCode::CPUI_COPY => def.ins.first().filter(|v| is_const(v)).map(|v| v.offset),
        OpCode::CPUI_INT_ADD => def.ins.iter().find(|v| is_const(v)).map(|v| v.offset),
        _ => None,
    }
}

/// The code addresses the words of the table at `base:offset` name, read until
/// one falls outside the decoded code `[lo, hi]`.
fn table_targets(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    base: u64,
    offset: u64,
    lo: u64,
    hi: u64,
) -> Vec<u64> {
    let loader = arch.translate().loader_rc();
    let Ok(mut loader) = loader.try_borrow_mut() else {
        return Vec::new();
    };
    let mut targets = Vec::new();
    for i in 0..MAX_TABLE {
        let mut word = [0u8; 2];
        let at = Address::new(Rc::clone(code_space), base + (offset.wrapping_add(2 * i) & 0xffff));
        if loader.load_fill(&mut word, &at).is_err() {
            break;
        }
        let target = base + u64::from(u16::from_le_bytes(word));
        if target < lo || target > hi {
            break;
        }
        targets.push(target);
    }
    targets
}

/// The vector of the `INT n` whose p-code `ops` is, when it is one: a `swi`
/// CALLOTHER on a one-byte constant, then the CALLIND through its result.
fn interrupt_vector(ops: &[Op], swi: u64) -> Option<u64> {
    let at = ops.iter().position(|op| {
        op.opcode == OpCode::CPUI_CALLOTHER
            && op.ins.len() == 2
            && is_const(&op.ins[0])
            && op.ins[0].offset == swi
            && is_const(&op.ins[1])
            && op.ins[1].size == 1
    })?;
    let out = ops[at].out.as_ref()?;
    ops[at + 1..]
        .iter()
        .any(|op| op.opcode == OpCode::CPUI_CALLIND && op.ins.first().is_some_and(|t| same(t, out)))
        .then_some(ops[at].ins[1].offset)
}

/// The constant AH holds when the instruction at `vma` runs, by the walk the
/// module header describes.
fn ah_at(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    listing: &Listing,
    entered: &BTreeSet<u64>,
    ah: &VarnodeData,
    vma: u64,
) -> Option<u64> {
    let mut cur = vma;
    for _ in 0..MAX_BACK {
        if entered.contains(&cur) {
            return None;
        }
        let mut into = listing
            .instructions_in_range(cur.saturating_sub(MAX_INSN_LEN), Some(cur))
            .filter(|(_, insn)| insn.fall_through == Some(cur));
        let (&prev, insn) = into.next()?;
        if into.next().is_some() || insn.flow.is_call {
            return None;
        }
        match ah_write(&decode(translate, code_space, prev)?, ah) {
            Write::Untouched => cur = prev,
            Write::Const(value) => return Some(value),
            Write::Unknown => return None,
        }
    }
    None
}

/// What one instruction does to AH.
enum Write {
    Untouched,
    Const(u64),
    Unknown,
}

/// The last write the p-code `ops` makes to AH, evaluated over the constants
/// the instruction itself moves through its temporaries.
fn ah_write(ops: &[Op], ah: &VarnodeData) -> Write {
    let mut temps: Vec<(VarnodeData, u64)> = Vec::new();
    let mut result = Write::Untouched;
    for op in ops {
        let Some(out) = &op.out else {
            continue;
        };
        let value = evaluate(op, &temps);
        if is_unique(out) {
            temps.retain(|(t, _)| !same(t, out));
            if let Some(value) = value {
                temps.push((out.clone(), value));
            }
            continue;
        }
        if !overlaps(out, ah) {
            continue;
        }
        result = match value {
            Some(value) if out.offset <= ah.offset => Write::Const((value >> (8 * (ah.offset - out.offset))) & 0xff),
            _ => Write::Unknown,
        };
    }
    result
}

/// The constant `op` produces, when its inputs are constants or temporaries
/// already holding one, or it is the `x ^ x` / `x - x` idiom for zero.
fn evaluate(op: &Op, temps: &[(VarnodeData, u64)]) -> Option<u64> {
    let input = |i: usize| -> Option<u64> {
        let v = op.ins.get(i)?;
        if is_const(v) {
            return Some(v.offset);
        }
        temps.iter().find(|(t, _)| same(t, v)).map(|(_, value)| *value)
    };
    match op.opcode {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => input(0),
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB
            if op.ins.len() == 2 && same(&op.ins[0], &op.ins[1]) =>
        {
            Some(0)
        }
        _ => None,
    }
}

fn is_const(v: &VarnodeData) -> bool {
    v.space.as_ref().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
}

fn is_unique(v: &VarnodeData) -> bool {
    v.space.as_ref().is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
}

fn same_space(a: &VarnodeData, b: &VarnodeData) -> bool {
    match (&a.space, &b.space) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        _ => false,
    }
}

fn same(a: &VarnodeData, b: &VarnodeData) -> bool {
    same_space(a, b) && a.offset == b.offset && a.size == b.size
}

fn overlaps(a: &VarnodeData, b: &VarnodeData) -> bool {
    same_space(a, b)
        && a.offset < b.offset + u64::from(b.size)
        && b.offset < a.offset + u64::from(a.size)
}

/// One p-code op with its output, which the Listing's capture does not keep.
struct Op {
    opcode: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

#[derive(Default)]
struct Capture {
    ops: Vec<Op>,
}

impl PcodeEmit for Capture {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push(Op { opcode: opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

fn decode(translate: &dyn Translate, code_space: &Rc<AddrSpace>, vma: u64) -> Option<Vec<Op>> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = Capture::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        translate.one_instruction(&mut cap, &addr)
    }));
    match decoded {
        Ok(Ok(len)) if len > 0 => Some(cap.ops),
        _ => None,
    }
}
