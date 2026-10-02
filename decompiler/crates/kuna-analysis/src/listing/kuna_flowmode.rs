//! (kuna `flowmode`) The ARM decode mode of every walked instruction follows
//! control flow, the way Ghidra's disassembler carries its flowing context.
//!
//! `TMode` selects the instruction set an address decodes in. An interworking
//! call (`blx imm`) runs a SLEIGH `globalset` that writes the callee's mode into
//! the per-address `ContextDatabase` from the target up to the next point where
//! the mode was set explicitly. An image with no mapping symbols has no such
//! point, so the write reaches every address above the target: once a caller of
//! a Thumb helper is decoded, an A32 function placed after the helper decodes as
//! Thumb too, whatever calls it.
//!
//! The walk decodes every instruction it has mode evidence for in that mode,
//! through a context read override, so the database is not what decides it:
//!
//! * a function symbol's address decodes in the mode its address bit gives,
//!   and so does an `e_entry` no symbol names: a call to it decodes in that
//!   mode, and a fall-through or branch that reaches it in the other mode (say,
//!   past a call that never returns) stops there instead of decoding the
//!   function in that mode;
//! * a branch target, fall-through or direct call target of evidenced code
//!   decodes in the mode its instruction committed for that address, or else in
//!   that instruction's own mode, so `bl` keeps the caller's mode and `blx`
//!   switches only its target;
//! * a Thumb `bx pc` in evidenced code continues in A32 at the next word, which
//!   is how the linker's Thumb-to-A32 veneers (`bx pc; nop; b target`) reach
//!   their A32 half;
//! * an evidenced A32 function that is one of the linker's interworking stubs,
//!   `ldr pc, [pc, #k]`, `ldr rX, [pc, #k]; bx rX` or `ldr rX, [pc, #k]; add
//!   rX, rX, pc; bx rX` through a read-only literal to a Thumb address (low bit
//!   set), makes that address a Thumb function, as the branch itself selects
//!   Thumb there at run time.
//!
//! Every evidenced entry is walked before any entry without evidence, so a
//! call with evidence reaches its target before a guess does. An entry without
//! evidence, and everything reached from it, decodes in the mode the database
//! holds for each address when the walk reaches it, exactly as the plain walk
//! decodes. SLEIGH's own context writes land in the database as before, and
//! the walk itself paints nothing.
//!
//! When the evidenced instructions span both modes, or one of them decoded in
//! a mode the database does not hold there, the walk hands every evidenced
//! instruction run to the analysis commit ([`super::Listing::decode_mode_paints`]),
//! which paints each with its mode after every other decode-mode paint. The
//! decompiler then reads the modes the Listing used for that code, and its own
//! `globalset` at a call target stops at the next painted run instead of
//! flowing across it. Code decoded without evidence is never painted: it keeps
//! whatever the metadata paints and the walk's own writes leave there, as it
//! would without this walk.
//!
//! Only an ARM image without mapping symbols takes this walk; mapping symbols
//! already delimit every mode run. An ARM image whose odd `e_entry` is its only
//! Thumb evidence keeps the plain walk, as does one whose build attributes rule
//! out A32 code, and one decoded with an explicit `--isa`. MIPS `ISA_MODE` keeps
//! the plain walk too: its MIPS16 function-symbol paints run to the next change
//! point, and the `jalx` writes that walk makes are what end them at the MIPS32
//! code after them.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::{ContextCommitRecord, Translate};
use object::{Object, ObjectKind, ObjectSymbol};

use super::kuna_poolref::PoolImage;
use super::model::{DiscoveredFunction, Insn};
use super::walk::{
    discovered, in_exec, step, CallbackEvidence, RefBuckets, StepCtx, Successors, WalkState,
};

const TMODE: &[u8] = b"TMode";

/// The context variable that selects the instruction set.
pub(super) struct FlowMode<'a> {
    word: usize,
    shift: u32,
    mask: u32,
    /// The even address and address-bit mode of every function symbol, and of
    /// an `e_entry` no function symbol names.
    functions: Vec<(u64, u32)>,
    /// The read-only image of a little-endian linked ARM ELF, where an
    /// interworking stub's instructions and literal are read.
    image: Option<PoolImage<'a>>,
}

impl<'a> FlowMode<'a> {
    /// [`FlowMode::resolve`] for an ARM image without mapping symbols, with
    /// the mode evidence of a linked ARM ELF: none for an ARM image whose
    /// build attributes rule out A32 code, whose odd `e_entry` is its only
    /// Thumb evidence, or whose instruction set the input selected.
    pub(super) fn for_object(
        file: &'a object::File<'a>,
        arch: &Architecture,
    ) -> Option<FlowMode<'a>> {
        if arch.input_arm_isa_override
            || crate::loader::kuna_armfloatabi::thumb_only(file)
            || crate::loader::arm_markers::unmarked_thumb_entry(file)
            || crate::loader::arm_markers::has_mapping_symbols(file)
        {
            return None;
        }
        let mut mode = FlowMode::resolve(arch)?;
        if file.format() == object::BinaryFormat::Elf
            && file.architecture() == object::Architecture::Arm
            && file.kind() != ObjectKind::Relocatable
        {
            let symbols: Vec<u64> = file
                .symbols()
                .chain(file.dynamic_symbols())
                .filter(|sym| sym.kind() == object::SymbolKind::Text && sym.is_definition())
                .map(|sym| sym.address())
                .collect();
            let entry = file.entry();
            let unnamed_entry =
                (entry != 0 && !symbols.iter().any(|&at| at & !1 == entry & !1)).then_some(entry);
            mode.functions = function_modes(symbols.into_iter().chain(unnamed_entry));
            if file.is_little_endian() {
                mode.image = PoolImage::new(file);
            }
        }
        Some(mode)
    }

    /// The language's ARM `TMode` variable, if it registers one.
    fn resolve(arch: &Architecture) -> Option<FlowMode<'a>> {
        let range = arch.with_context_db_mut(|db| db.get_variable(TMODE)).ok()?;
        Some(FlowMode {
            word: usize::try_from(range.get_word()).ok()?,
            shift: u32::try_from(range.get_shift()).ok()?,
            mask: range.get_mask(),
            functions: Vec::new(),
            image: None,
        })
    }

    /// The address an A32 interworking stub at `entry` branches to, low bit
    /// included: `ldr pc, [pc, #k]`, `ldr rX, [pc, #k]; bx rX`, or the
    /// position-independent `ldr rX, [pc, #k]; add rX, rX, pc; bx rX`, with
    /// the literal in read-only memory.
    fn stub_target(&self, entry: u64) -> Option<u64> {
        let image = self.image.as_ref()?;
        let word = |at: u64| image.word32_at(at).map(|w| w as u32);
        let load = word(entry)?;
        if load & 0xff7f_0000 != 0xe51f_0000 {
            return None;
        }
        let base = entry.checked_add(8)?;
        let offset = u64::from(load & 0xfff);
        let literal = if load & 0x0080_0000 != 0 {
            base.checked_add(offset)?
        } else {
            base.checked_sub(offset)?
        };
        let value = word(literal)?;
        let register = (load >> 12) & 0xf;
        if register == 15 {
            return Some(u64::from(value));
        }
        let bx = 0xe12f_ff10 | register;
        let next = word(entry.checked_add(4)?)?;
        if next == bx {
            return Some(u64::from(value));
        }
        let add_pc = [
            0xe080_000f | register << 16 | register << 12,
            0xe08f_0000 | register << 12 | register,
        ];
        if add_pc.contains(&next) && word(entry.checked_add(8)?)? == bx {
            return Some(u64::from(
                value.wrapping_add((entry as u32).wrapping_add(12)),
            ));
        }
        None
    }

    /// The A32 address a Thumb `bx pc` at `at` branches to: the instruction
    /// word after it, `(at + 4) & !3`. The linker's Thumb-to-A32 veneers
    /// (`bx pc; nop; b target`) have this form.
    fn bx_pc_target(&self, at: u64) -> Option<u64> {
        let word = self.image.as_ref()?.word32_at(at & !3)? as u32;
        let half = if at & 2 != 0 {
            word >> 16
        } else {
            word & 0xffff
        };
        (half == 0x4778).then(|| at.wrapping_add(4) & !3)
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

/// The even address and mode of each function symbol `address`, read off its
/// low bit; an address two symbols give different modes is left out.
fn function_modes(addresses: impl Iterator<Item = u64>) -> Vec<(u64, u32)> {
    let mut modes: BTreeMap<u64, Option<u32>> = BTreeMap::new();
    for address in addresses {
        let mode = u32::from(address & 1 != 0);
        modes
            .entry(address & !1)
            .and_modify(|known| {
                if *known != Some(mode) {
                    *known = None;
                }
            })
            .or_insert(Some(mode));
    }
    modes
        .into_iter()
        .filter_map(|(entry, mode)| Some((entry, mode?)))
        .collect()
}

/// The decode mode the translator reads, set through its context read override
/// and restored when the walk ends.
struct ModeOverride<'a> {
    translate: &'a dyn Translate,
    mode: &'a FlowMode<'a>,
    current: Option<u32>,
    saved: (u32, u32),
}

impl<'a> ModeOverride<'a> {
    fn new(translate: &'a dyn Translate, mode: &'a FlowMode<'a>) -> Self {
        let saved = translate.set_context_read_override(mode.word, 0, 0);
        ModeOverride {
            translate,
            mode,
            current: None,
            saved,
        }
    }

    /// Decode in `value`, or read the database when it is `None`.
    fn set(&mut self, value: Option<u32>) {
        if self.current != value {
            let (bits, value_bits) = match value {
                Some(value) => (self.mode.bits(), value << self.mode.shift),
                None => (0, 0),
            };
            self.translate
                .set_context_read_override(self.mode.word, bits, value_bits);
            self.current = value;
        }
    }
}

impl Drop for ModeOverride<'_> {
    fn drop(&mut self) {
        self.translate
            .set_context_read_override(self.mode.word, self.saved.0, self.saved.1);
    }
}

/// The walk's worklists. An evidenced entry or instruction carries the mode it
/// decodes in; one without evidence carries `None` and reads the database.
struct ModedWorklists<'a> {
    translate: &'a dyn Translate,
    mode: &'a FlowMode<'a>,
    symbol_modes: &'a BTreeMap<u64, u32>,
    insns: Vec<(u64, Option<u32>)>,
    evidenced: &'a mut Vec<(u64, u32)>,
    unevidenced: &'a mut Vec<u64>,
    current: Option<u32>,
    committed: Option<Vec<(u64, u32)>>,
}

impl ModedWorklists<'_> {
    fn mode_for(&mut self, target: u64, current: u32) -> u32 {
        let (mode, translate) = (self.mode, self.translate);
        self.committed
            .get_or_insert_with(|| mode.committed(&translate.last_context_commits()))
            .iter()
            .rev()
            .find(|&&(addr, _)| addr == target)
            .map_or(current, |&(_, value)| value)
    }
}

impl Successors for ModedWorklists<'_> {
    fn insn(&mut self, vma: u64) {
        let Some(current) = self.current else {
            self.insns.push((vma, None));
            return;
        };
        let mode = self.mode_for(vma, current);
        if self
            .symbol_modes
            .get(&vma)
            .is_some_and(|&symbol| symbol != mode)
        {
            return;
        }
        self.insns.push((vma, Some(mode)));
    }
    fn func(&mut self, entry: u64) {
        let current = self.current;
        let mode = match self.symbol_modes.get(&entry) {
            Some(&symbol) => Some(symbol),
            None => current.map(|current| self.mode_for(entry, current)),
        };
        match mode {
            Some(mode) => self.evidenced.push((entry, mode)),
            None => self.unevidenced.push(entry),
        }
    }
}

/// The decoded instructions as maximal same-mode runs. `decoded` is the
/// address-ordered `(start, end, mode)` of every evidenced instruction.
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

/// The serial walk with the decode mode carried along flow (see the module
/// docs). The same [`step`] as the plain walk decodes every instruction.
pub(super) fn walk(
    ctx: &StepCtx<'_>,
    arch: &Architecture,
    mode: &FlowMode<'_>,
    seeds: &[u64],
    seed_funcs: &BTreeMap<u64, DiscoveredFunction>,
) -> WalkState {
    let space = ctx.code_space;
    let mut insns: BTreeMap<u64, Insn> = BTreeMap::new();
    let mut funcs: BTreeMap<u64, DiscoveredFunction> = BTreeMap::new();
    let mut refs = RefBuckets::new(ctx.detail.refs);
    let mut callbacks = CallbackEvidence::default();
    let symbol_modes: BTreeMap<u64, u32> = mode.functions.iter().copied().collect();
    let mut evidenced: Vec<(u64, u32)> = Vec::new();
    let mut unevidenced: Vec<u64> = Vec::new();
    for &entry in seeds {
        match symbol_modes.get(&entry).copied() {
            Some(entry_mode) => evidenced.push((entry, entry_mode)),
            None => unevidenced.push(entry),
        }
    }
    let mut visited_funcs: BTreeSet<u64> = BTreeSet::new();
    for &entry in seeds {
        let df = seed_funcs
            .get(&entry)
            .cloned()
            .unwrap_or_else(|| discovered(entry));
        funcs.entry(entry).or_insert(df);
    }

    let mut decode_mode = ModeOverride::new(ctx.translate, mode);
    let mut decided: Vec<(u64, u32)> = Vec::new();
    loop {
        let (entry, entry_mode) = match evidenced.pop() {
            Some((entry, entry_mode)) => (entry, Some(entry_mode)),
            None => match unevidenced.pop() {
                Some(entry) => (entry, None),
                None => break,
            },
        };
        if !visited_funcs.insert(entry) {
            continue;
        }
        if let Some(thumb) = (entry_mode == Some(0))
            .then(|| mode.stub_target(entry))
            .flatten()
        {
            let target = thumb & !1;
            if thumb & 1 != 0
                && in_exec(ctx.exec_ranges, target)
                && symbol_modes.get(&target).is_none_or(|&symbol| symbol == 1)
            {
                funcs.entry(target).or_insert_with(|| discovered(target));
                evidenced.push((target, 1));
            }
        }
        let mut work = ModedWorklists {
            translate: ctx.translate,
            mode,
            symbol_modes: &symbol_modes,
            insns: vec![(entry, entry_mode)],
            evidenced: &mut evidenced,
            unevidenced: &mut unevidenced,
            current: entry_mode,
            committed: None,
        };
        while let Some((vma, vma_mode)) = work.insns.pop() {
            if insns.contains_key(&vma) || !in_exec(ctx.exec_ranges, vma) {
                continue;
            }
            decode_mode.set(vma_mode);
            work.current = vma_mode;
            work.committed = None;
            if step(
                ctx,
                vma,
                &mut insns,
                &mut funcs,
                &mut refs,
                &mut callbacks,
                &mut work,
            ) {
                if let Some(vma_mode) = vma_mode {
                    decided.push((vma, vma_mode));
                    if let Some(target) = (vma_mode == 1).then(|| mode.bx_pc_target(vma)).flatten()
                    {
                        if symbol_modes.get(&target).is_none_or(|&symbol| symbol == 0) {
                            work.insns.push((target, Some(0)));
                        }
                    }
                }
            }
        }
    }
    drop(decode_mode);
    decided.sort_unstable();
    let both = decided.iter().any(|&(_, m)| m == 0) && decided.iter().any(|&(_, m)| m == 1);
    let mode_runs = if both
        || decided
            .iter()
            .any(|&(at, m)| mode.value_at(arch, space, at) != m)
    {
        mode_runs(decided.iter().map(|&(at, run_mode)| {
            let len = insns.get(&at).map_or(0, |insn| u64::from(insn.len));
            (at, at.saturating_add(len), run_mode)
        }))
    } else {
        Vec::new()
    };

    let (refs_to, refs_from) = refs.into_parts();
    WalkState {
        insns,
        refs_to,
        refs_from,
        funcs,
        stack_callback_refs: callbacks.into_refs(),
        mode_runs,
    }
}

#[cfg(test)]
mod tests {
    use super::{function_modes, mode_runs};

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

    #[test]
    fn a_function_symbols_low_bit_is_its_mode_unless_two_symbols_disagree() {
        let modes = function_modes([0x41, 0x80, 0x41, 0x100, 0x101].into_iter());
        assert_eq!(modes, vec![(0x40, 1), (0x80, 0)]);
    }
}
