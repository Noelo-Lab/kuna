//! (kuna `flowmode`) Decode-mode paints that follow control flow from code
//! whose ARM mode the image itself states, over a stripped image whose only
//! mode changes come from the walk's own `blx` writes.
//!
//! `TMode` selects the instruction set an address decodes in. An interworking
//! call (`blx imm`) runs a SLEIGH `globalset` that writes the callee's mode
//! into the per-address `ContextDatabase` from the target up to the next point
//! where the mode was set explicitly. An image with no mapping symbols and no
//! Thumb function symbol has no such point, so the write reaches every address
//! above the target: an A32 function placed after a Thumb helper decodes as
//! Thumb, whatever calls it.
//!
//! The plain walk runs unchanged and its result is kept as it is. Afterwards,
//! this pass decodes again, without writing the mode into the database, the
//! code the image proves the mode of, and reports every instruction of that
//! code whose mode the database disagrees with. The analysis commit paints
//! those instructions, and nothing else, after every other decode-mode paint.
//!
//! Proof starts at an even `e_entry` and at each even function symbol, all
//! A32 by the ELF for the ARM architecture. From proven code it carries on
//! where the instruction set decides the mode:
//!
//! * a branch target and a fall-through keep the mode;
//! * a direct call target takes the mode the call commits there (`blx imm`
//!   switches) or else keeps the caller's (`bl`);
//! * the instruction after an unconditional call is proven only once the
//!   callee is proven to return: some return instruction is reachable from its
//!   entry through proven code. A call whose callee never returns, or that
//!   reaches an import or a computed target, is not followed, because the
//!   bytes after it may be a literal pool or another function.
//!
//! The result does not depend on visiting order. Proof is a least fixpoint,
//! and when two proven paths give one address different modes, or two proven
//! instructions of different modes overlap, or a proven instruction fails to
//! decode where the database holds the other mode, nothing is painted at all.
//!
//! The pass runs only on an ARM ELF that is not relocatable, has an even
//! `e_entry` or an even function symbol, no load-time `TMode` paint at all (no
//! mapping symbol, no Thumb function symbol, no Cortex-M vector table), and
//! build attributes that allow A32 code, and only when the input selects no
//! instruction set and the database holds Thumb somewhere after the walk.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::{ContextCommitRecord, Translate};
use object::{Object, ObjectKind, ObjectSymbol, SymbolKind};

use super::classify::classify;
use super::context::ContextPainter;
use super::decode::decode_one;
use super::model::FlowKind;
use super::walk::{in_exec, StepCtx};

const TMODE: &[u8] = b"TMode";

/// The context variable that selects the instruction set, and the addresses
/// the image states are A32 code.
pub(super) struct FlowMode {
    word: usize,
    shift: u32,
    mask: u32,
    seeds: Vec<u64>,
}

impl FlowMode {
    /// The proof seeds of `file` when the pass applies to it (see the module
    /// docs), else `None`.
    pub(super) fn for_object(
        file: &object::File<'_>,
        arch: &Architecture,
        painter: &ContextPainter,
    ) -> Option<FlowMode> {
        if !arch.analysis_flowmode
            || arch.input_arm_isa_override
            || file.format() != object::BinaryFormat::Elf
            || file.architecture() != object::Architecture::Arm
            || file.kind() == ObjectKind::Relocatable
            || painter.paints("TMode")
            || crate::loader::kuna_armfloatabi::thumb_only(file)
        {
            return None;
        }
        let entry = file.entry();
        if entry & 1 != 0 {
            return None;
        }
        let mut seeds: Vec<u64> = file
            .symbols()
            .chain(file.dynamic_symbols())
            .filter(|sym| sym.kind() == SymbolKind::Text && sym.is_definition())
            .map(|sym| sym.address())
            .chain(std::iter::once(entry))
            .filter(|&at| at != 0 && at & 1 == 0)
            .collect();
        seeds.sort_unstable();
        seeds.dedup();
        if seeds.is_empty() {
            return None;
        }
        let range = arch.with_context_db_mut(|db| db.get_variable(TMODE)).ok()?;
        Some(FlowMode {
            word: usize::try_from(range.get_word()).ok()?,
            shift: u32::try_from(range.get_shift()).ok()?,
            mask: range.get_mask(),
            seeds,
        })
    }

    fn bits(&self) -> u32 {
        self.mask << self.shift
    }

    fn value_at(&self, arch: &Architecture, space: &Rc<AddrSpace>, vma: u64) -> u32 {
        let addr = Address::new(Rc::clone(space), vma);
        arch.with_context_db_mut(|db| {
            db.get_context(&addr)
                .get(self.word)
                .map_or(0, |w| (w >> self.shift) & self.mask)
        })
    }

    /// Whether the database holds `value` anywhere inside `ranges`.
    fn holds_anywhere(
        &self,
        arch: &Architecture,
        space: &Rc<AddrSpace>,
        ranges: &[(u64, u64)],
        value: u32,
    ) -> bool {
        ranges.iter().any(|&(lo, hi)| {
            let mut at = lo;
            while at < hi {
                let addr = Address::new(Rc::clone(space), at);
                let (held, last) = arch.with_context_db_mut(|db| {
                    let (blob, _, last) = db.get_context_bounds(&addr);
                    let held = blob
                        .get(self.word)
                        .map_or(0, |w| (w >> self.shift) & self.mask);
                    (held, last)
                });
                if held == value {
                    return true;
                }
                match last.checked_add(1) {
                    Some(next) if next > at => at = next,
                    _ => break,
                }
            }
            false
        })
    }

    /// The `(address, mode)` pairs among `commits` that write this variable.
    fn committed(&self, commits: &[ContextCommitRecord]) -> Vec<(u64, u32)> {
        commits
            .iter()
            .filter(|c| usize::try_from(c.word).ok() == Some(self.word))
            .filter(|c| c.mask & self.bits() == self.bits())
            .map(|c| (c.addr.get_offset(), (c.value >> self.shift) & self.mask))
            .collect()
    }
}

/// While alive, every decode reads the mode last [`set`](DecodeMode::set)
/// whatever the database holds, and no decode writes the mode into it.
struct DecodeMode<'a> {
    translate: &'a dyn Translate,
    mode: &'a FlowMode,
    current: Option<u32>,
    saved_read: (u32, u32),
    saved_write: u32,
}

impl<'a> DecodeMode<'a> {
    fn new(translate: &'a dyn Translate, mode: &'a FlowMode) -> Self {
        let saved_write = translate.set_context_write_mask(mode.word, !mode.bits());
        let saved_read = translate.set_context_read_override(mode.word, 0, 0);
        DecodeMode {
            translate,
            mode,
            current: None,
            saved_read,
            saved_write,
        }
    }

    fn set(&mut self, value: u32) {
        if self.current != Some(value) {
            self.translate.set_context_read_override(
                self.mode.word,
                self.mode.bits(),
                value << self.mode.shift,
            );
            self.current = Some(value);
        }
    }
}

impl Drop for DecodeMode<'_> {
    fn drop(&mut self) {
        self.translate
            .set_context_read_override(self.mode.word, self.saved_read.0, self.saved_read.1);
        self.translate
            .set_context_write_mask(self.mode.word, self.saved_write);
    }
}

/// One proven instruction.
struct Proven {
    len: u32,
    mode: u32,
    returns: bool,
    calls: Vec<(u64, u32)>,
    branches: Vec<u64>,
    fall_through: Option<u64>,
    after_call: bool,
}

/// A function entry the proof reached, and what its own walk has visited.
struct Entry {
    mode: u32,
    visited: BTreeSet<u64>,
    returns: bool,
}

/// The proof walk's state; `None` from any step means two proofs disagree.
struct Proof<'c, 'a> {
    ctx: &'c StepCtx<'a>,
    arch: &'c Architecture,
    mode: &'c FlowMode,
    decoded: BTreeMap<u64, Proven>,
    entries: BTreeMap<u64, Entry>,
    /// Instructions after a call, waiting for the callee to return.
    waiting: BTreeMap<u64, Vec<(u64, u64)>>,
    work: Vec<(u64, u64)>,
}

impl Proof<'_, '_> {
    fn enter(&mut self, entry: u64, mode: u32) -> Option<()> {
        if let Some(known) = self.entries.get(&entry) {
            return (known.mode == mode).then_some(());
        }
        self.entries.insert(
            entry,
            Entry {
                mode,
                visited: BTreeSet::new(),
                returns: false,
            },
        );
        self.work.push((entry, entry));
        Some(())
    }

    fn mark_returns(&mut self, entry: u64) {
        let Some(known) = self.entries.get_mut(&entry) else {
            return;
        };
        if known.returns {
            return;
        }
        known.returns = true;
        if let Some(after) = self.waiting.remove(&entry) {
            self.work.extend(after);
        }
    }

    /// Decode `at` in `mode` unless it already is; `Ok(false)` when it does
    /// not decode.
    fn decode(&mut self, at: u64, mode: u32, decode_mode: &mut DecodeMode<'_>) -> Option<bool> {
        if let Some(known) = self.decoded.get(&at) {
            return (known.mode == mode).then_some(true);
        }
        if !in_exec(self.ctx.exec_ranges, at) {
            return Some(false);
        }
        decode_mode.set(mode);
        let decoded = decode_one(self.ctx.translate, at, self.ctx.code_space, false, false)
            .ok()
            .filter(|decoded| decoded.len != 0);
        let Some(decoded) = decoded else {
            let held = self.mode.value_at(self.arch, self.ctx.code_space, at);
            return (held == mode).then_some(false);
        };
        let commits = self.mode.committed(&self.ctx.translate.last_context_commits());
        let class = classify(&decoded.ops, at, decoded.len);
        let mode_at = |target: u64| {
            commits
                .iter()
                .rev()
                .find(|&&(addr, _)| addr == target)
                .map_or(mode, |&(_, value)| value)
        };
        let targets = class
            .flows
            .iter()
            .copied()
            .filter(|&target| Some(target) != class.fall_through);
        let (calls, branches) = if class.flow.is_call {
            (targets.map(|t| (t, mode_at(t))).collect(), Vec::new())
        } else {
            (Vec::new(), targets.collect())
        };
        self.decoded.insert(
            at,
            Proven {
                len: decoded.len,
                mode,
                returns: class.flow.kind == FlowKind::Return || class.flow.is_terminal,
                calls,
                branches,
                fall_through: class.fall_through,
                after_call: class.flow.is_call && !class.flow.is_conditional,
            },
        );
        Some(true)
    }

    /// Walk `at` as part of the function at `entry`.
    fn visit(&mut self, entry: u64, at: u64, decode_mode: &mut DecodeMode<'_>) -> Option<()> {
        let mode = self.entries.get(&entry)?.mode;
        if !self.entries.get_mut(&entry)?.visited.insert(at) {
            return Some(());
        }
        if !self.decode(at, mode, decode_mode)? {
            return Some(());
        }
        let insn = &self.decoded[&at];
        let (returns, calls, branches) = (insn.returns, insn.calls.clone(), insn.branches.clone());
        let (fall_through, after_call) = (insn.fall_through, insn.after_call);
        if returns {
            self.mark_returns(entry);
        }
        for &(target, target_mode) in &calls {
            if in_exec(self.ctx.exec_ranges, target) {
                self.enter(target, target_mode)?;
            }
        }
        for &target in &branches {
            self.work.push((entry, target));
        }
        let Some(next) = fall_through else {
            return Some(());
        };
        if !after_call {
            self.work.push((entry, next));
            return Some(());
        }
        if let [(callee, _)] = calls[..] {
            match self.entries.get(&callee) {
                Some(known) if known.returns => self.work.push((entry, next)),
                Some(_) => self.waiting.entry(callee).or_default().push((entry, next)),
                None => {}
            }
        }
        Some(())
    }

    /// Run to the fixpoint; `None` when two proofs disagree.
    fn run(&mut self) -> Option<()> {
        let mut decode_mode = DecodeMode::new(self.ctx.translate, self.mode);
        for seed in self.mode.seeds.clone() {
            if in_exec(self.ctx.exec_ranges, seed) {
                self.enter(seed, 0)?;
            }
        }
        while let Some((entry, at)) = self.work.pop() {
            self.visit(entry, at, &mut decode_mode)?;
        }
        let mut reach: Option<(u64, u32)> = None;
        for (&at, insn) in &self.decoded {
            let end = at.saturating_add(u64::from(insn.len));
            if reach.is_some_and(|(far, far_mode)| at < far && far_mode != insn.mode) {
                return None;
            }
            if reach.is_none_or(|(far, _)| end > far) {
                reach = Some((end, insn.mode));
            }
        }
        Some(())
    }
}

/// The `(start, end, mode)` runs of proven code whose mode the database does
/// not hold after the walk; empty when there is none, or when proofs disagree.
pub(super) fn disagreeing_runs(
    ctx: &StepCtx<'_>,
    arch: &Architecture,
    mode: &FlowMode,
) -> Vec<(u64, u64, u32)> {
    let space = ctx.code_space;
    if !mode.holds_anywhere(arch, space, ctx.exec_ranges, 1) {
        return Vec::new();
    }
    let mut proof = Proof {
        ctx,
        arch,
        mode,
        decoded: BTreeMap::new(),
        entries: BTreeMap::new(),
        waiting: BTreeMap::new(),
        work: Vec::new(),
    };
    if proof.run().is_none() {
        return Vec::new();
    }
    mode_runs(proof.decoded.iter().filter_map(|(&at, insn)| {
        (mode.value_at(arch, space, at) != insn.mode)
            .then(|| (at, at.saturating_add(u64::from(insn.len)), insn.mode))
    }))
}

/// Address-ordered `(start, end, mode)` instructions as maximal same-mode
/// runs of adjacent instructions.
fn mode_runs(decoded: impl Iterator<Item = (u64, u64, u32)>) -> Vec<(u64, u64, u32)> {
    let mut runs: Vec<(u64, u64, u32)> = Vec::new();
    for (start, end, mode) in decoded {
        match runs.last_mut() {
            Some(last) if last.2 == mode && start <= last.1 => last.1 = last.1.max(end),
            _ => runs.push((start, end, mode)),
        }
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::mode_runs;

    #[test]
    fn same_mode_neighbours_merge_and_a_mode_change_splits() {
        let decoded = [
            (0x00, 0x04, 0),
            (0x04, 0x08, 0),
            (0x40, 0x42, 1),
            (0x42, 0x44, 1),
            (0x80, 0x84, 0),
            (0x84, 0x88, 0),
        ];
        assert_eq!(
            mode_runs(decoded.into_iter()),
            vec![(0x00, 0x08, 0), (0x40, 0x44, 1), (0x80, 0x88, 0)]
        );
    }

    #[test]
    fn a_gap_splits_a_run_even_in_one_mode() {
        let decoded = [(0x00, 0x04, 0), (0x08, 0x0c, 0), (0x0c, 0x0e, 1)];
        assert_eq!(
            mode_runs(decoded.into_iter()),
            vec![(0x00, 0x04, 0), (0x08, 0x0c, 0), (0x0c, 0x0e, 1)]
        );
    }
}
