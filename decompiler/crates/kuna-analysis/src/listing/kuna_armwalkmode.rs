//! (kuna `armwalkmode`) The ARM decode mode the Listing walk carries along the
//! control flow it follows from code whose mode the image states, on an image
//! whose metadata paints no mode.
//!
//! A `blx imm` commits `TMode` at its target, and the context database fills
//! that value up to the next point where the mode was set. A stripped image has
//! no such point, so every address above a Thumb helper read as Thumb, and an
//! A32 function placed there decoded as Thumb even when an A32 `bl` called it.
//!
//! The walk starts carrying a mode at an even `e_entry` and at each even
//! function symbol, which the ELF for the ARM architecture makes A32. From an
//! instruction decoded in a carried mode, a branch target and a fall-through
//! keep the mode, and a direct call target takes the mode the call commits
//! there (`blx imm` switches) or else keeps it (`bl`). Every other function
//! entry, and every byte the walk does not reach, reads the database as before,
//! the walk's own `blx` writes included, so code found only through a pointer
//! or a prologue pattern keeps the mode it had.
//!
//! The instruction after a call, or after a user-defined p-code operation such
//! as `svc` or `bkpt`, is where a carried mode can run into another function:
//! one that never comes back is followed by whatever the linker placed next.
//! The walk holds each such address back until the functions its calls reach
//! are walked, so a call target decodes in its call's mode before a
//! fall-through can claim it. An address still undecoded then carries the mode
//! on where the database agrees with it, or where its callee is shown to reach
//! a return (an operation or an indirect call is assumed to come back); a
//! callee never shown to return leaves the address to the database.
//!
//! When the walk ends, each instruction decoded in a carried mode whose span
//! the database holds in another mode is painted with that mode, and the
//! address after each painted run is marked as set, so the decompiler reads the
//! instructions the walk found and a later `blx` write at a run stops at its end.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::{ContextCommitRecord, Translate};
use object::{Object, ObjectKind, ObjectSymbol, SymbolKind};

use super::context::ContextPainter;
use super::kuna_insnstore::InstructionStore;

const TMODE: &[u8] = b"TMode";

/// The most instructions one return search visits before it assumes a return.
const RETURN_SEARCH_LIMIT: usize = 1 << 16;

/// The `TMode` field of the language and the A32 roots, for an image the pass
/// applies to.
pub(super) struct ArmWalkMode {
    word: usize,
    shift: u32,
    mask: u32,
    roots: BTreeSet<u64>,
}

impl ArmWalkMode {
    /// `Some` for a linked ARM ELF with an even `e_entry`, build attributes that
    /// allow A32, no metadata `TMode` paint (mapping symbol, Thumb function
    /// symbol, Cortex-M vector table, A32 function extent) and no `--isa`.
    pub(super) fn for_object(
        file: &object::File<'_>,
        arch: &Architecture,
        painter: &ContextPainter,
    ) -> Option<ArmWalkMode> {
        if !arch.analysis_armwalkmode
            || arch.input_arm_isa_override
            || file.format() != object::BinaryFormat::Elf
            || file.architecture() != object::Architecture::Arm
            || file.kind() == ObjectKind::Relocatable
            || file.entry() & 1 != 0
            || painter.paints("TMode")
            || crate::loader::kuna_armfloatabi::thumb_only(file)
        {
            return None;
        }
        let roots: BTreeSet<u64> = file
            .symbols()
            .chain(file.dynamic_symbols())
            .filter(|sym| sym.kind() == SymbolKind::Text && sym.is_definition())
            .map(|sym| sym.address())
            .chain(std::iter::once(file.entry()))
            .filter(|&at| at != 0 && at & 1 == 0)
            .collect();
        let range = arch.with_context_db_mut(|db| db.get_variable(TMODE)).ok()?;
        Some(ArmWalkMode {
            word: usize::try_from(range.get_word()).ok()?,
            shift: u32::try_from(range.get_shift()).ok()?,
            mask: range.get_mask(),
            roots,
        })
    }

    fn bits(&self) -> u32 {
        self.mask << self.shift
    }

    /// The per-walk state.
    pub(super) fn begin(&self) -> WalkModes<'_> {
        WalkModes {
            mode: self,
            calls: HashMap::new(),
            decoded: Vec::new(),
            held: Vec::new(),
            returns: HashSet::new(),
        }
    }
}

/// An instruction after a call or an operation, held back until the walk's
/// other work is done.
struct Held {
    at: u64,
    mode: u32,
    callee: Option<u64>,
}

/// The modes one walk has decoded with and the evidence it has collected.
pub(super) struct WalkModes<'a> {
    mode: &'a ArmWalkMode,
    calls: HashMap<u64, u32>,
    decoded: Vec<(u64, u32, u32)>,
    held: Vec<Held>,
    returns: HashSet<u64>,
}

impl WalkModes<'_> {
    /// The carried mode of a function entry: A32 at a root, else its call's.
    pub fn entry(&self, at: u64) -> Option<u32> {
        if self.mode.roots.contains(&at) {
            return Some(0);
        }
        self.calls.get(&at).copied()
    }

    /// The mode `at`, a successor of an instruction decoded in `mode`, takes.
    pub fn successor(&self, at: u64, mode: u32, commits: &[ContextCommitRecord]) -> u32 {
        let bits = self.mode.bits();
        commits
            .iter()
            .rev()
            .find(|c| {
                c.addr.get_offset() == at
                    && usize::try_from(c.word).ok() == Some(self.mode.word)
                    && c.mask & bits == bits
            })
            .map_or(mode, |c| (c.value >> self.mode.shift) & self.mode.mask)
    }

    /// Queue a direct call's mode for its target, the first call winning.
    pub fn called(&mut self, target: u64, mode: u32) {
        self.calls.entry(target).or_insert(mode);
    }

    /// Note that `[at, at + len)` decoded in the carried `mode`.
    pub fn decoded(&mut self, at: u64, len: u32, mode: u32) {
        self.decoded.push((at, len, mode));
    }

    /// Hold the instruction at `at` after a call to `callee` (`None` for an
    /// operation or an indirect call), carried in `mode`.
    pub fn hold(&mut self, at: u64, mode: u32, callee: Option<u64>) {
        self.held.push(Held { at, mode, callee });
    }

    /// The held instructions the walk resumes now, in carried mode: those still
    /// undecoded where the database agrees, after an operation or an indirect
    /// call, or after a callee that reaches a return. When none qualifies, all
    /// of them resume under the database's mode.
    pub fn resume(
        &mut self,
        arch: &Architecture,
        space: &Rc<AddrSpace>,
        insns: &InstructionStore,
    ) -> Vec<(u64, Option<u32>)> {
        let held = std::mem::take(&mut self.held);
        let mut resumed = Vec::new();
        let mut no_return = HashSet::new();
        for item in held {
            if insns.contains_key(&item.at) {
                continue;
            }
            let carried = self.database_mode(arch, space, item.at) == item.mode
                || item.callee.is_none_or(|callee| {
                    !no_return.contains(&callee)
                        && (self.reaches_return(callee, insns) || !no_return.insert(callee))
                });
            if carried {
                resumed.push((item.at, Some(item.mode)));
            } else {
                self.held.push(item);
            }
        }
        if resumed.is_empty() {
            resumed = std::mem::take(&mut self.held)
                .into_iter()
                .map(|item| (item.at, None))
                .collect();
        }
        resumed
    }

    fn database_mode(&self, arch: &Architecture, space: &Rc<AddrSpace>, at: u64) -> u32 {
        let word = arch.with_context_db_mut(|db| {
            db.get_context(&Address::new(Rc::clone(space), at))
                .get(self.mode.word)
                .copied()
                .unwrap_or(0)
        });
        (word >> self.mode.shift) & self.mode.mask
    }

    /// Whether some return is reachable from `entry` through decoded code,
    /// following branches and the instruction after each call; an indirect
    /// branch may leave for code that returns, so it counts as one.
    fn reaches_return(&mut self, entry: u64, insns: &InstructionStore) -> bool {
        if self.returns.contains(&entry) {
            return true;
        }
        let mut seen = HashSet::new();
        let mut stack = vec![entry];
        let mut returns = false;
        while let Some(at) = stack.pop() {
            if seen.len() > RETURN_SEARCH_LIMIT {
                returns = true;
                break;
            }
            if !seen.insert(at) {
                continue;
            }
            let Some(insn) = insns.get(&at) else { continue };
            if insn.flow.is_terminal || (insn.flow.is_computed && !insn.flow.is_call) {
                returns = true;
                break;
            }
            if !insn.flow.is_call {
                stack.extend(insn.flows.iter().copied());
            }
            stack.extend(insn.fall_through);
        }
        if returns {
            self.returns.insert(entry);
        }
        returns
    }

    /// Whether every carried decode is in `mode` and one stretch of the
    /// database holding `mode` covers them all, so there is nothing to paint.
    fn holds_only(&self, arch: &Architecture, space: &Rc<AddrSpace>, mode: u32) -> bool {
        if self.decoded.iter().any(|&(_, _, decoded)| decoded != mode) {
            return false;
        }
        let low = self.decoded.iter().map(|&(at, _, _)| at).min().unwrap_or(0);
        let high = self
            .decoded
            .iter()
            .map(|&(at, len, _)| at.saturating_add(u64::from(len)))
            .max()
            .unwrap_or(0);
        arch.with_context_db_mut(|db| {
            let (words, _, last) = db.get_context_bounds(&Address::new(Rc::clone(space), low));
            let word = words.get(self.mode.word).copied().unwrap_or(0);
            (word >> self.mode.shift) & self.mode.mask == mode && high.saturating_sub(1) <= last
        })
    }

    /// The read override that makes decodes read a carried mode, until it drops.
    pub fn read_override<'t>(&self, translate: &'t dyn Translate) -> ModeOverride<'t> {
        let saved = translate.set_context_read_override(self.mode.word, 0, 0);
        translate.set_context_read_override(self.mode.word, saved.0, saved.1);
        ModeOverride {
            translate,
            word: self.mode.word,
            bits: self.mode.bits(),
            shift: self.mode.shift,
            saved,
            current: None,
        }
    }

    /// Paint every span decoded in a carried mode the database holds otherwise,
    /// and mark the mode after each painted run as set, so a later `blx` write
    /// at the run stops at its end.
    pub fn finish(mut self, arch: &Architecture, space: &Rc<AddrSpace>) {
        if self.holds_only(arch, space, 0) {
            return;
        }
        self.decoded.sort_unstable();
        let mut runs: Vec<(u64, u64, u32)> = Vec::new();
        let mut range: Option<(u64, u64, u32)> = None;
        arch.with_context_db_mut(|db| {
            for &(at, len, mode) in &self.decoded {
                let end = at.saturating_add(u64::from(len));
                let (_, last, value) = match range {
                    Some(bounds) if bounds.0 <= at && at <= bounds.1 => bounds,
                    _ => {
                        let (words, first, last) =
                            db.get_context_bounds(&Address::new(Rc::clone(space), at));
                        let word = words.get(self.mode.word).copied().unwrap_or(0);
                        let bounds = (first, last, (word >> self.mode.shift) & self.mode.mask);
                        range = Some(bounds);
                        bounds
                    }
                };
                if value == mode && end.saturating_sub(1) <= last {
                    continue;
                }
                match runs.last_mut() {
                    Some(run) if run.1 == at && run.2 == mode => run.1 = end,
                    _ => runs.push((at, end, mode)),
                }
            }
        });
        let at = |offset: u64| Address::new(Rc::clone(space), offset);
        for (start, end, mode) in runs {
            arch.with_context_db_mut(|db| {
                let _ = db.set_variable_region(TMODE, &at(start), &at(end), mode);
                let (words, _, last) = db.get_context_bounds(&at(end));
                let word = words.get(self.mode.word).copied().unwrap_or(0);
                let after = (word >> self.mode.shift) & self.mode.mask;
                if let Some(next) = last.checked_add(1) {
                    let _ = db.set_variable_region(TMODE, &at(end), &at(next), after);
                }
            });
        }
    }
}

/// Decodes read the mode last [`set`](ModeOverride::set), or the database's
/// for `None`; dropping it restores the read override it replaced.
pub(super) struct ModeOverride<'t> {
    translate: &'t dyn Translate,
    word: usize,
    bits: u32,
    shift: u32,
    saved: (u32, u32),
    current: Option<u32>,
}

impl ModeOverride<'_> {
    pub fn set(&mut self, mode: Option<u32>) {
        if mode == self.current {
            return;
        }
        let (mask, value) = match mode {
            Some(mode) => (
                self.saved.0 | self.bits,
                (self.saved.1 & !self.bits) | (mode << self.shift),
            ),
            None => self.saved,
        };
        self.translate
            .set_context_read_override(self.word, mask, value);
        self.current = mode;
    }
}

impl Drop for ModeOverride<'_> {
    fn drop(&mut self) {
        self.translate
            .set_context_read_override(self.word, self.saved.0, self.saved.1);
    }
}
