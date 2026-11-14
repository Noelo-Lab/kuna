//! (kuna #1288) Where a function of an x86 real-mode raw image ends.
//!
//! The inventory measures a function as the clip from its entry to the next
//! entry or the end of its CODE section (`kuna-console/src/funcextent.rs`).
//! That is an honest bound where the section table says which bytes are code.
//! An MS-DOS executable and a `--raw-image` real-mode program have no section
//! table: the loader publishes the whole load module as one CODE range, its
//! DGROUP and far data segments included, so a function that data follows ran
//! on to the next entry. On the unpacked DAZZLE.EXE of issue #1288 a 28-byte
//! function in front of DGROUP measured 12,174 bytes, and `kuna disassemble`
//! listed DGROUP as its instructions.
//!
//! [`measure`] ends each entry where its flow does, over the descent the raw
//! discovery already decoded: at the end of the last instruction its flow
//! reaches between the entry and the next entry, so the span is still
//! contiguous from the entry and can only be shorter than the clip. The flow
//! is followed wherever it goes short of another entry, since a path can leave
//! the span and come back into it, and only what it reaches in the span
//! counts. Calls are not entered (the callee is another function) and fall
//! through unless the callee is no-return; a DOS exit
//! ([`super::kuna_dosexit`]) has no successor. A branch through a `cs:` table
//! that lies in the span, which the descent leaves unresolved, reaches the
//! table's words that name an address in the span, decoded here when the
//! descent did not decode them; a branch through a fixed word (`jmp
//! [0x2ef2]`) leaves the function, as the decompiler's tail call does; any
//! other computed branch has targets nothing names, so its entry is left
//! unmeasured and keeps the clip. An address a path reaches that does not
//! decode counts by its first byte. Bytes between reached instructions stay
//! in the span (a switch table in the body); a table after the last
//! instruction does not.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_fasthash::FxHashSet;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::listing::kuna_dosexit::{self, DosExits, Op};
use crate::listing::Listing;

/// The exclusive end of every measured entry's flow, keyed by entry, or `None`
/// when the language is not x86 real mode.
///
/// `entries` is the whole inventory, ascending.
pub fn measure(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    listing: &Listing,
    entries: &[u64],
    exits: &DosExits,
) -> Option<BTreeMap<u64, u64>> {
    if !kuna_dosexit::is_real_mode(&arch.archid) {
        return None;
    }
    let walk = Walk {
        arch,
        translate,
        code_space,
        listing,
        entries,
        exits,
        noreturn: exits.noreturn.iter().map(|fact| fact.addr).collect(),
        segment: arch.userops.get_op_by_name(b"segment").map(|op| op.get_index() as u64),
        cs: translate.get_register_data("CS").ok(),
    };
    let ranges = listing.exec_ranges();
    Some(crate::loader::kuna_realmodecs::without_commits(arch, translate, || {
        let mut ends = BTreeMap::new();
        let mut seen = FxHashSet::default();
        let mut stack = Vec::new();
        for (i, &entry) in entries.iter().enumerate() {
            let Some(&(lo, hi)) = ranges.iter().find(|&&(lo, hi)| lo <= entry && entry < hi) else {
                continue;
            };
            let bound = entries.get(i + 1).copied().filter(|&next| next > entry && next < hi).unwrap_or(hi);
            if let Some(end) = walk.end(entry, bound, (lo, hi), &mut seen, &mut stack) {
                ends.insert(entry, end);
            }
        }
        ends
    }))
}

/// The most instructions one entry's walk visits before it gives up and the
/// entry keeps its clip.
const MAX_VISITS: usize = 1 << 14;

/// The most words read from one table, as [`super::kuna_dosexit`] reads.
const MAX_TABLE: u64 = 256;

struct Walk<'a> {
    arch: &'a Architecture,
    translate: &'a dyn Translate,
    code_space: &'a Rc<AddrSpace>,
    listing: &'a Listing,
    entries: &'a [u64],
    exits: &'a DosExits,
    noreturn: BTreeSet<u64>,
    segment: Option<u64>,
    cs: Option<VarnodeData>,
}

/// One instruction's length and successors, from the Listing or a fresh decode.
struct Step<'a> {
    len: u32,
    is_call: bool,
    unresolved_jump: bool,
    flows: Cow<'a, [u64]>,
    fall_through: Option<u64>,
}

/// Where a computed branch the Listing left without targets goes.
enum Through {
    /// The words of a `cs:` table at this segment offset.
    Table(u64),
    /// A fixed word: another function.
    Word,
    /// Nowhere this pass can name.
    Unknown,
}

impl Walk<'_> {
    /// The end of the flow from `entry` inside `[entry, bound)`, or `None` when
    /// the descent never decoded the entry, a computed branch on the way cannot
    /// be followed, or the walk outgrows [`MAX_VISITS`]. `range` is the
    /// executable range holding the entry.
    fn end(
        &self,
        entry: u64,
        bound: u64,
        range: (u64, u64),
        seen: &mut FxHashSet<u64>,
        stack: &mut Vec<u64>,
    ) -> Option<u64> {
        self.listing.instruction_at(entry)?;
        seen.clear();
        stack.clear();
        stack.push(entry);
        let mut end = entry;
        while let Some(vma) = stack.pop() {
            if vma < range.0 || vma >= range.1 || (vma != entry && self.entries.binary_search(&vma).is_ok()) {
                continue;
            }
            if !seen.insert(vma) {
                continue;
            }
            if seen.len() > MAX_VISITS {
                return None;
            }
            let inside = (entry..bound).contains(&vma);
            let Some(step) = self.step(vma) else {
                if inside {
                    end = end.max(vma + 1);
                }
                continue;
            };
            if inside {
                end = end.max(vma + u64::from(step.len));
            }
            if self.exits.sites.binary_search(&vma).is_ok() {
                continue;
            }
            if step.is_call {
                if !step.flows.iter().any(|target| self.noreturn.contains(target)) {
                    stack.extend(step.fall_through);
                }
                continue;
            }
            stack.extend(step.flows.iter().copied());
            stack.extend(step.fall_through);
            if step.unresolved_jump {
                match self.through(vma) {
                    Through::Table(offset) => stack.extend(self.table(vma, offset, entry, bound, range)?),
                    Through::Word => {}
                    Through::Unknown => return None,
                }
            }
        }
        Some(end.min(bound))
    }

    fn step(&self, vma: u64) -> Option<Step<'_>> {
        if let Some(insn) = self.listing.instruction_at(vma) {
            return Some(Step {
                len: insn.len,
                is_call: insn.flow.is_call,
                unresolved_jump: insn.flow.is_computed && insn.flow.is_jump && insn.flows.is_empty(),
                flows: Cow::Borrowed(&insn.flows),
                fall_through: insn.fall_through,
            });
        }
        let decoded = decode_one(self.translate, vma, self.code_space, false, false).ok()?;
        if decoded.len == 0 {
            return None;
        }
        let c = classify(&decoded.ops, vma, decoded.len);
        Some(Step {
            len: decoded.len,
            is_call: c.flow.is_call,
            unresolved_jump: c.flow.is_computed && c.flow.is_jump && c.flows.is_empty(),
            flows: Cow::Owned(c.flows),
            fall_through: c.fall_through,
        })
    }

    /// Read the computed branch at `vma`: `jmp cs:[reg+table]` names a table,
    /// `jmp [word]` a fixed word.
    fn through(&self, vma: u64) -> Through {
        let (Some(segment), Some(cs)) = (self.segment, self.cs.as_ref()) else {
            return Through::Unknown;
        };
        let Some(ops) = kuna_dosexit::decode(self.translate, self.code_space, vma) else {
            return Through::Unknown;
        };
        through(&ops, segment, cs)
    }

    /// The case targets inside `[entry, bound)` of the `cs:` table the branch
    /// at `vma` reads at `offset`: its words before `bound`, read while they
    /// name an address in `range` (at most [`MAX_TABLE`]). `None` unless the
    /// table itself lies in `[entry, bound)` and names an address there:
    /// otherwise the constant is a displacement off a pointer (`jmp
    /// cs:[bx+0eh]` after a value search), not a table.
    fn table(&self, vma: u64, offset: u64, entry: u64, bound: u64, range: (u64, u64)) -> Option<Vec<u64>> {
        let at = Address::new(Rc::clone(self.code_space), vma);
        let cs = self.arch.with_context_db(|db| db.get_variable_value(b"realCS", &at)).unwrap_or(0);
        let base = u64::from(cs) << 4;
        let offset = offset & 0xffff;
        let table = base + offset;
        if !(entry..bound).contains(&table) {
            return None;
        }
        let mut words = vec![0u8; (bound - table).min(2 * MAX_TABLE).min(0x1_0000 - offset) as usize];
        let loader = self.arch.translate().loader_rc();
        let mut loader = loader.try_borrow_mut().ok()?;
        loader.load_fill(&mut words, &Address::new(Rc::clone(self.code_space), table)).ok()?;
        let targets: Vec<u64> = words
            .chunks_exact(2)
            .map(|word| base + u64::from(u16::from_le_bytes([word[0], word[1]])))
            .take_while(|target| (range.0..range.1).contains(target))
            .filter(|target| (entry..bound).contains(target))
            .collect();
        (!targets.is_empty()).then_some(targets)
    }
}

/// Classify a computed branch's p-code `ops`: the address its target is
/// loaded from is `segment(seg, offset)`, a constant offset is a fixed word,
/// and a constant added to a register under `cs` is a table.
fn through(ops: &[Op], segment: u64, cs: &VarnodeData) -> Through {
    let Some(branch) = ops.iter().rev().find(|op| op.opcode == OpCode::CPUI_BRANCHIND) else {
        return Through::Unknown;
    };
    let Some(mut target) = branch.ins.first() else {
        return Through::Unknown;
    };
    if let Some(seg) = defining(ops, target).filter(|op| is_segment(op, segment)) {
        target = &seg.ins[2];
    }
    let Some(load) = defining(ops, target).filter(|op| op.opcode == OpCode::CPUI_LOAD && op.ins.len() == 2) else {
        return Through::Unknown;
    };
    let Some(seg) = defining(ops, &load.ins[1]).filter(|op| is_segment(op, segment)) else {
        return Through::Unknown;
    };
    let offset = &seg.ins[2];
    if kuna_dosexit::is_const(offset) {
        return Through::Word;
    }
    if !kuna_dosexit::same(&seg.ins[1], cs) {
        return Through::Unknown;
    }
    match defining(ops, offset) {
        Some(add) if add.opcode == OpCode::CPUI_INT_ADD && add.ins.len() == 2 => {
            match add.ins.iter().find(|v| kuna_dosexit::is_const(v)) {
                Some(table) => Through::Table(table.offset),
                None => Through::Unknown,
            }
        }
        _ => Through::Unknown,
    }
}

/// The last op of `ops` that writes the temporary `v`.
fn defining<'o>(ops: &'o [Op], v: &VarnodeData) -> Option<&'o Op> {
    if !kuna_dosexit::is_unique(v) {
        return None;
    }
    ops.iter().rev().find(|op| op.out.as_ref().is_some_and(|out| kuna_dosexit::same(out, v)))
}

fn is_segment(op: &Op, segment: u64) -> bool {
    op.opcode == OpCode::CPUI_CALLOTHER
        && op.ins.len() == 3
        && kuna_dosexit::is_const(&op.ins[0])
        && op.ins[0].offset == segment
}
