//! Resolve secure-PLT PIC entries from caller register evidence. A stub is named
//! when every transfer into it selects the same validated slot through r30.
//!
//! Each function entry is explored once with symbolic entry registers. Direct
//! branches, fallthrough, resolved jump tables and LSDA landing pads are followed;
//! calls continue at the return address with the registers the callee provably
//! keeps (imports and calls through registers keep the SysV nonvolatiles, local
//! callees are summarized from their own exploration, straight-line helpers are
//! executed in place). An indirect jump the walk cannot resolve meets its state
//! into every instruction of its function, so only evidence reached from it is
//! lost. Stubs are entered only by branch fields the linker wrote: a stub that a
//! data word, symbol, entry, landing pad or unresolved function scope reaches is
//! never named. Table metadata and linker labels never supply a register value.
use super::elf_plt::PltSym;
use kuna_decomp::kuna_fasthash::{FxHashMap, FxHashSet};
use object::{Object, ObjectSection, ObjectSymbol, SectionFlags, SectionKind, SymbolKind};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Value {
    #[default]
    Unknown,
    Constant(u32),
    Entry(u8),
    Return,
    Stack(u32),
    Import,
    Mark(u32),
    Bounded(u16),
    Index(u16),
    Element(u32, u16),
    Table(u32, u16),
    Case(u32, u16),
}

/// No compare bounds the index: the table is read to the end of its scope.
const UNBOUNDED: u16 = u16::MAX;

impl Value {
    fn add(self, offset: u32) -> Self {
        match self {
            Self::Constant(v) => Self::Constant(v.wrapping_add(offset)),
            Self::Stack(v) => Self::Stack(v.wrapping_add(offset)),
            other if offset == 0 => other,
            _ => Self::Unknown,
        }
    }

    fn constant(self) -> Option<u32> {
        if let Self::Constant(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn sum(self, other: Self) -> Self {
        match (self, other) {
            (Self::Constant(t), Self::Index(n)) | (Self::Index(n), Self::Constant(t)) => {
                Self::Element(t, n)
            }
            (Self::Constant(a), b) => b.add(a),
            (a, Self::Constant(b)) => a.add(b),
            _ => Self::Unknown,
        }
    }
}

/// r1, r2 and r13-r31: the registers a SysV PPC32 callee returns unchanged.
const NONVOLATILE: u32 = 0xffff_e006;

/// Imports the C, POSIX and Itanium C++ ABIs define as never returning.
const NORETURN: &[&str] = &[
    "abort",
    "exit",
    "_exit",
    "_Exit",
    "quick_exit",
    "pthread_exit",
    "longjmp",
    "_longjmp",
    "siglongjmp",
    "__longjmp_chk",
    "__stack_chk_fail",
    "__assert_fail",
    "__fortify_fail",
    "__chk_fail",
    "__cxa_throw",
    "__cxa_rethrow",
    "__cxa_bad_cast",
    "__cxa_bad_typeid",
    "__cxa_throw_bad_array_new_length",
    "_Unwind_Resume",
    "_ZSt9terminatev",
];
const SLOTS: usize = 8;
const UNSAVED: u32 = u32::MAX;

/// Registers, LR and CTR, plus the stack words that hold an entry value: the
/// save offset of each entry nonvolatile register, and a few return-address
/// and back-chain words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    regs: [Value; 32],
    lr: Value,
    ctr: Value,
    saved: [u32; 32],
    slots: [Option<(u32, Value)>; SLOTS],
    compare: Option<(u8, u8, u16, Value)>,
}

impl State {
    fn top() -> Self {
        Self {
            regs: [Value::Unknown; 32],
            lr: Value::Unknown,
            ctr: Value::Unknown,
            saved: [UNSAVED; 32],
            slots: [None; SLOTS],
            compare: None,
        }
    }

    fn entry() -> Self {
        let mut state = Self::top();
        for r in 0..32 {
            if NONVOLATILE & (1 << r) != 0 {
                state.regs[r] = Value::Entry(r as u8);
            }
        }
        state.regs[1] = Value::Stack(0);
        state.lr = Value::Return;
        state
    }

    fn meet(&self, other: &Self) -> Self {
        let pick = |a: Value, b: Value| if a == b { a } else { Value::Unknown };
        let mut slots = [None; SLOTS];
        for (out, slot) in slots.iter_mut().zip(self.slots) {
            *out = slot.filter(|s| other.slots.contains(&Some(*s)));
        }
        Self {
            regs: std::array::from_fn(|i| pick(self.regs[i], other.regs[i])),
            lr: pick(self.lr, other.lr),
            ctr: pick(self.ctr, other.ctr),
            saved: std::array::from_fn(|i| {
                if self.saved[i] == other.saved[i] {
                    self.saved[i]
                } else {
                    UNSAVED
                }
            }),
            slots,
            compare: self.compare.filter(|c| other.compare == Some(*c)),
        }
    }

    /// On the edge where the last unsigned compare `cmplwi crF,rX,N` shows
    /// rX <= N, rX becomes `Bounded(N)`. The compare marks rX with its own
    /// address, so any write to rX in between breaks the match.
    fn refine(mut self, word: u32, taken: bool) -> Self {
        let bo = (word >> 21) & 31;
        let bi = (word >> 16) & 31;
        if let Some((field, reg, bound, value)) = self.compare {
            let within = match bo & 0x1c {
                0x0c => !taken,
                0x04 => taken,
                _ => return self,
            };
            if bi == u32::from(field) * 4 + 1 && within && self.regs[reg as usize] == value {
                self.regs[reg as usize] = Value::Bounded(bound);
            }
        }
        self
    }

    fn base(&self, register: usize) -> Value {
        if register == 0 {
            Value::Constant(0)
        } else {
            self.regs[register]
        }
    }

    fn load(&self, image: &Image<'_>, address: Value) -> Value {
        match address {
            Value::Stack(at) => {
                if let Some(r) = self.saved.iter().position(|&offset| offset == at) {
                    return Value::Entry(r as u8);
                }
                self.slots
                    .iter()
                    .flatten()
                    .find(|(offset, _)| *offset == at)
                    .map_or(Value::Unknown, |(_, v)| *v)
            }
            Value::Constant(at) => image.read_word(at),
            Value::Element(t, n) => Value::Table(t, n),
            _ => Value::Unknown,
        }
    }

    /// Only stores at a known offset from the entry stack pointer can reach a
    /// frame's register save words; other pointers address program objects.
    fn store(&mut self, address: Value, width: u32, value: Value) {
        let Value::Stack(at) = address else { return };
        let hit = |offset: u32| offset.wrapping_sub(at) < width || at.wrapping_sub(offset) < 4;
        for offset in self.saved.iter_mut() {
            if *offset != UNSAVED && hit(*offset) {
                *offset = UNSAVED;
            }
        }
        for slot in self.slots.iter_mut() {
            if slot.is_some_and(|(offset, _)| hit(offset)) {
                *slot = None;
            }
        }
        if width != 4 {
            return;
        }
        match value {
            Value::Entry(r) => self.saved[r as usize] = at,
            Value::Return | Value::Stack(_) => {
                let free = self.slots.iter().position(|s| s.is_none()).or_else(|| {
                    (0..SLOTS)
                        .filter(|&i| self.slots[i].is_some_and(|(o, _)| (o as i32) < at as i32))
                        .min_by_key(|&i| self.slots[i].map_or(0, |(o, _)| o as i32))
                });
                if let Some(i) = free {
                    self.slots[i] = Some((at, value));
                }
            }
            _ => {}
        }
    }

    fn keep(mut self, mask: u32) -> Self {
        for (r, value) in self.regs.iter_mut().enumerate() {
            if mask & (1 << r) == 0 {
                *value = Value::Unknown;
            }
        }
        self.lr = Value::Unknown;
        self.ctr = Value::Unknown;
        self.compare = None;
        self
    }

    fn preserved(&self) -> u32 {
        let mut mask = 0;
        for r in 0..32 {
            let entry = if r == 1 {
                Value::Stack(0)
            } else {
                Value::Entry(r as u8)
            };
            if NONVOLATILE & (1 << r) != 0 && self.regs[r] == entry {
                mask |= 1 << r;
            }
        }
        mask
    }
}

#[derive(Clone, Copy)]
struct Branch {
    target: u32,
    link: bool,
    conditional: bool,
}

fn branch(pc: u32, word: u32) -> Option<Branch> {
    let (delta, conditional) = match word >> 26 {
        18 => (((word & 0x03ff_fffc) as i32) << 6 >> 6, false),
        16 => ((word as u16 as i16 as i32) & !3, (word >> 21) & 20 != 20),
        _ => return None,
    };
    Some(Branch {
        target: (if word & 2 == 0 { pc } else { 0 }).wrapping_add(delta as u32),
        link: word & 1 != 0,
        conditional,
    })
}

/// Instructions that leave every condition-register field unchanged.
fn keeps_cr(word: u32) -> bool {
    match word >> 26 {
        2 | 3 | 7 | 8 | 12 | 14 | 15 | 24..=27 | 32..=55 => true,
        20 | 21 | 23 => word & 1 == 0,
        31 => word & 1 == 0 && !matches!((word >> 1) & 1023, 0 | 32 | 144 | 512),
        _ => false,
    }
}

/// A branch-and-link that transfers to a callee: the only instructions that
/// unwind into a landing pad.
fn calls(pc: u32, word: u32) -> bool {
    match branch(pc, word) {
        Some(b) => b.link && b.target != pc.wrapping_add(4) && !pc_capture(word),
        None => word >> 26 == 19 && word & 1 != 0 && matches!((word >> 1) & 1023, 16 | 528),
    }
}

/// `tw 31,..` / `twi 31,..`: traps unconditionally, so nothing follows it, as
/// nothing follows `sc` with r0 = 1 or 234 (Linux `exit`, `exit_group`).
fn trap(word: u32) -> bool {
    (word >> 21) & 31 == 31 && (word >> 26 == 3 || word >> 26 == 31 && (word >> 1) & 1023 == 4)
}

/// `bcl 20,31,target`: the ISA's form for reading the program counter, not a call.
fn pc_capture(word: u32) -> bool {
    word & 0xffff_0003 == 0x429f_0001
}

fn signed(word: u32) -> u32 {
    word as u16 as i16 as u32
}

fn rotate_mask(begin: u32, end: u32) -> u32 {
    let from = u32::MAX >> begin;
    let to = u32::MAX << (31 - end);
    if begin <= end {
        from & to
    } else {
        from | to
    }
}

fn pic_delta(words: &[u32]) -> Option<u32> {
    if words[0] & 0xffff_0000 == 0x817e_0000
        && words[1..] == [0x7d69_03a6, 0x4e80_0420, 0x6000_0000]
    {
        Some(signed(words[0]))
    } else if words[0] & 0xffff_0000 == 0x3d7e_0000
        && words[1] & 0xffff_0000 == 0x816b_0000
        && words[2..] == [0x7d69_03a6, 0x4e80_0420]
    {
        Some((words[0] << 16).wrapping_add(signed(words[1])))
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum Reloc {
    Address(u32),
    Import,
    Other,
}

struct DataSection<'a> {
    start: u32,
    bytes: &'a [u8],
    stable: bool,
    pointers: bool,
}

/// The loaded image as the walk reads it: code words, relocated data words, the
/// stubs and the function scopes an unresolved indirect jump may target.
#[derive(Default)]
struct Image<'a> {
    code: Vec<(u32, Vec<u32>)>,
    data: Vec<DataSection<'a>>,
    little: bool,
    relocs: FxHashMap<u32, Reloc>,
    stubs: BTreeMap<u32, u32>,
    slots: BTreeMap<u64, Vec<u8>>,
    absolute: BTreeSet<u32>,
    resolvers: Vec<(u32, u32)>,
    seeds: FxHashSet<u32>,
    sites: FxHashSet<u32>,
    scopes: Vec<(u32, u32)>,
    ends: FxHashSet<u32>,
    targets: FxHashSet<u32>,
    bounds: BTreeSet<u32>,
    pads: Vec<(u32, u32, u32)>,
    opaque: Vec<(u32, u32)>,
}

impl<'a> Image<'a> {
    fn word_at(&self, pc: u32) -> Option<u32> {
        let i = self.code.partition_point(|(start, _)| *start <= pc).checked_sub(1)?;
        let (start, words) = &self.code[i];
        let offset = pc.wrapping_sub(*start);
        if offset & 3 != 0 {
            return None;
        }
        words.get(offset as usize / 4).copied()
    }

    fn is_code(&self, pc: u32) -> bool {
        self.word_at(pc).is_some()
    }

    fn read(&self, bytes: &[u8]) -> u32 {
        let bytes = bytes[..4].try_into().unwrap();
        if self.little {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        }
    }

    /// A word whose runtime value is fixed by the image: code, a relocated
    /// word, or a word of a section nothing but the loader writes.
    fn read_word(&self, at: u32) -> Value {
        if at & 3 != 0 {
            return Value::Unknown;
        }
        if let Some(word) = self.word_at(at) {
            return Value::Constant(word);
        }
        match self.relocs.get(&at) {
            Some(Reloc::Address(v)) => return Value::Constant(*v),
            Some(Reloc::Import) => return Value::Import,
            Some(Reloc::Other) => return Value::Unknown,
            None => {}
        }
        let i = self.data.partition_point(|d| d.start <= at);
        let Some(section) = i.checked_sub(1).map(|i| &self.data[i]) else {
            return Value::Unknown;
        };
        let offset = (at - section.start) as usize;
        match section.bytes.get(offset..offset + 4) {
            Some(bytes) if section.stable => Value::Constant(self.read(bytes)),
            _ => Value::Unknown,
        }
    }

    fn slot(&self, stub: u32, state: &State) -> Option<u32> {
        state.regs[30]
            .constant()
            .map(|base| base.wrapping_add(self.stubs[&stub]))
            .filter(|slot| self.slots.contains_key(&u64::from(*slot)))
    }

    /// The stub's proven slot names a function its ABI defines as not returning.
    fn terminal(&self, stub: u32, state: &State) -> bool {
        self.slot(stub, state)
            .and_then(|slot| std::str::from_utf8(&self.slots[&u64::from(slot)]).ok())
            .is_some_and(|name| {
                NORETURN.contains(&name)
                    || name.starts_with("_ZSt") && name.contains("__throw_")
            })
    }

    fn stub_at(&self, pc: u32) -> Option<u32> {
        self.stubs
            .range(..=pc)
            .next_back()
            .filter(|(&start, _)| pc < start.wrapping_add(16))
            .map(|(&start, _)| start)
    }

    fn import_exit(&self, pc: u32) -> bool {
        self.absolute.contains(&pc) || self.resolvers.iter().any(|&(s, e)| (s..e).contains(&pc))
    }

    /// Code from `pc` to its first `blr` with no other control transfer; a
    /// called helper must also leave LR alone.
    fn straight(&self, pc: u32, tail: bool) -> bool {
        (0..32).find_map(|i| {
            let word = self.word_at(pc.wrapping_add(4 * i))?;
            if word == 0x4e80_0020 {
                return Some(true);
            }
            let op = word >> 26;
            let xo = (word >> 1) & 1023;
            let spr = ((word >> 16) & 31) | ((word >> 6) & 0x3e0);
            if matches!(op, 16..=18)
                || op == 19 && matches!(xo, 16 | 18 | 50 | 528)
                || !tail && op == 31 && xo == 467 && spr == 8
            {
                return Some(false);
            }
            None
        }) == Some(true)
    }

    /// The function an unresolved jump at `pc` may target: its FDE range or
    /// sized symbol, else the code between neighbouring entries and stubs.
    fn scope(&self, pc: u32) -> (u32, u32) {
        let i = self.scopes.partition_point(|&(start, _)| start <= pc);
        if let Some(&(start, end)) =
            self.scopes[..i].iter().rev().take(8).find(|&&(_, end)| pc < end)
        {
            return (start, end);
        }
        let start = self.bounds.range(..=pc).next_back().copied().unwrap_or(0);
        let end = self.bounds.range(pc + 1..).next().copied().unwrap_or(u32::MAX);
        (start, end)
    }

    fn pad(&self, pc: u32) -> Option<u32> {
        let i = self.pads.partition_point(|&(start, _, _)| start <= pc);
        self.pads[..i]
            .iter()
            .rev()
            .take(8)
            .find(|&&(_, end, _)| pc < end)
            .map(|&(_, _, pad)| pad)
    }

    fn opaque(&self, pc: u32) -> bool {
        let i = self.opaque.partition_point(|&(start, _)| start <= pc);
        self.opaque[..i].iter().rev().take(8).any(|&(_, end)| pc < end)
    }

    /// Targets of the jump table at `base` dispatched by the indirect jump at
    /// `pc`: relative entries add the base. Real targets are labels of the
    /// jump's scope that start a block, never the dispatch sequence itself.
    /// A bounded index reads its entries and keeps those; otherwise the table
    /// is read until its first target outside the scope.
    fn table(&self, pc: u32, base: u32, relative: bool, bound: u16) -> Vec<u32> {
        let scope = self.scope(pc);
        let mut dispatch = pc;
        while dispatch > scope.0 && !self.targets.contains(&dispatch) {
            match self.word_at(dispatch.wrapping_sub(4)) {
                Some(w) if !matches!(w >> 26, 16..=19) => dispatch = dispatch.wrapping_sub(4),
                _ => break,
            }
        }
        let mut out = Vec::new();
        let count = if bound == UNBOUNDED {
            4096
        } else {
            u32::from(bound) + 1
        };
        for i in 0..count {
            let Value::Constant(word) = self.read_word(base.wrapping_add(4 * i)) else {
                return if bound == UNBOUNDED { out } else { Vec::new() };
            };
            let target = if relative { base.wrapping_add(word) } else { word };
            if !(scope.0..scope.1).contains(&target) || !self.is_code(target) {
                if bound == UNBOUNDED {
                    break;
                }
                continue;
            }
            if !(dispatch.wrapping_add(4)..=pc).contains(&target) {
                out.push(target);
            }
        }
        out
    }
}

#[derive(Default)]
struct Evidence {
    events: Vec<(u32, Option<u32>)>,
    reached: Vec<u32>,
    blocked: Vec<u32>,
}

impl Evidence {
    fn enter(&mut self, image: &Image<'_>, stub: u32, state: &State) {
        let slot = image.slot(stub, state);
        self.events.push((stub, slot));
    }

    fn escape(&mut self, image: &Image<'_>, value: Value) {
        if let Some(stub) = value.constant().and_then(|v| image.stub_at(v)) {
            self.blocked.push(stub);
        }
    }
}

/// What a call to a local function keeps: the nonvolatile registers every
/// return restores, and whether any path returns at all.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Summary {
    kept: u32,
    returns: bool,
}

const STRONGEST: Summary = Summary {
    kept: NONVOLATILE,
    returns: false,
};

/// An ABI callee: returns with every nonvolatile register restored.
const CALLEE: Summary = Summary {
    kept: NONVOLATILE,
    returns: true,
};

struct Walk {
    seed: u32,
    states: FxHashMap<u32, State>,
    queue: VecDeque<u32>,
    flooded: bool,
    exits: Summary,
    evidence: Evidence,
}

impl Walk {
    fn exit(&mut self, state: &State, after: Summary) {
        if after.returns && !self.flooded {
            self.exits.kept &= state.preserved() & after.kept;
            self.exits.returns = true;
        }
    }
}

struct Analysis<'i, 'a> {
    image: &'i Image<'a>,
    summaries: FxHashMap<u32, Summary>,
    dependents: FxHashMap<u32, FxHashSet<u32>>,
    active: FxHashSet<u32>,
    stale: FxHashSet<u32>,
    dirty: VecDeque<u32>,
    queued: FxHashSet<u32>,
    evidence: FxHashMap<u32, Evidence>,
    events: FxHashMap<u32, Option<u32>>,
    reached: FxHashSet<u32>,
    blocked: FxHashSet<u32>,
    budget: u64,
}

const DEPTH: usize = 48;

impl Analysis<'_, '_> {
    /// What `callee` keeps, as currently known. Summaries start from the
    /// strongest claim and only weaken; a caller is re-explored whenever a
    /// summary it read changes, so the fixed point is the greatest one. That is
    /// sound for recursion by induction on the depth of a returning call.
    fn summary(&mut self, caller: u32, callee: u32) -> Summary {
        self.dependents.entry(callee).or_default().insert(caller);
        if let Some(&summary) = self.summaries.get(&callee) {
            return summary;
        }
        if self.active.contains(&callee) || self.active.len() >= DEPTH || self.budget == 0 {
            self.mark(callee);
            return STRONGEST;
        }
        self.run(callee);
        self.summaries.get(&callee).copied().unwrap_or(STRONGEST)
    }

    /// Schedule `entry` to be explored again; one still being explored is
    /// rescheduled when it finishes.
    fn mark(&mut self, entry: u32) {
        if self.active.contains(&entry) {
            self.stale.insert(entry);
        } else if self.queued.insert(entry) {
            self.dirty.push_back(entry);
        }
    }

    fn run(&mut self, entry: u32) {
        self.active.insert(entry);
        let walk = self.explore(entry);
        self.active.remove(&entry);
        let old = self.summaries.insert(entry, walk.exits).unwrap_or(STRONGEST);
        self.evidence.insert(entry, walk.evidence);
        if self.stale.remove(&entry) {
            self.mark(entry);
        }
        if old != walk.exits {
            let dependents: Vec<_> = self
                .dependents
                .get(&entry)
                .map(|d| d.iter().copied().collect())
                .unwrap_or_default();
            for dependent in dependents {
                self.mark(dependent);
            }
        }
    }

    fn solve(&mut self, seeds: &[u32]) {
        for &seed in seeds {
            if !self.summaries.contains_key(&seed) {
                self.run(seed);
            }
        }
        while let Some(entry) = self.dirty.pop_front() {
            self.queued.remove(&entry);
            if self.budget == 0 {
                return;
            }
            self.run(entry);
        }
        for (_, evidence) in std::mem::take(&mut self.evidence) {
            self.commit(evidence);
        }
    }

    fn commit(&mut self, evidence: Evidence) {
        for (stub, slot) in evidence.events {
            self.events
                .entry(stub)
                .and_modify(|seen| {
                    if *seen != slot {
                        *seen = None;
                    }
                })
                .or_insert(slot);
        }
        self.reached.extend(evidence.reached);
        self.blocked.extend(evidence.blocked);
    }

    fn explore(&mut self, seed: u32) -> Walk {
        let mut walk = Walk {
            seed,
            states: FxHashMap::default(),
            queue: VecDeque::new(),
            flooded: false,
            exits: STRONGEST,
            evidence: Evidence::default(),
        };
        self.flow(&mut walk, seed, State::entry());
        while let Some(pc) = walk.queue.pop_front() {
            if self.budget == 0 {
                break;
            }
            self.budget -= 1;
            let state = walk.states[&pc];
            self.visit(&mut walk, pc, state);
        }
        walk
    }

    fn flow(&mut self, walk: &mut Walk, target: u32, state: State) {
        let image = self.image;
        if let Some(stub) = image.stub_at(target) {
            if stub == target {
                walk.evidence.enter(image, stub, &state);
                if !image.terminal(stub, &state) {
                    self.leave(walk, state, CALLEE);
                }
            } else {
                walk.evidence.blocked.push(stub);
            }
            return;
        }
        if image.import_exit(target) {
            self.leave(walk, state, CALLEE);
            return;
        }
        if !image.is_code(target) {
            return;
        }
        if target != walk.seed && image.seeds.contains(&target) && !image.straight(target, true) {
            let after = self.summary(walk.seed, target);
            self.leave(walk, state, after);
            return;
        }
        if walk.flooded {
            return;
        }
        if image.opaque(target) {
            self.unresolved(walk, target);
            return;
        }
        match walk.states.get_mut(&target) {
            Some(old) if *old == state => {}
            Some(old) => {
                let merged = old.meet(&state);
                if merged != *old {
                    *old = merged;
                    walk.queue.push_back(target);
                }
            }
            None => {
                walk.states.insert(target, state);
                walk.queue.push_back(target);
            }
        }
    }

    /// A transfer to another function, which returns through LR: to this
    /// walk's caller for the entry return address, else to a known address
    /// of this code.
    fn leave(&mut self, walk: &mut Walk, state: State, after: Summary) {
        match state.lr {
            Value::Constant(back) => {
                if after.returns {
                    self.flow(walk, back, state.keep(after.kept));
                }
            }
            _ => walk.exit(&state, after),
        }
    }

    /// An indirect jump the walk cannot resolve: it may return, tail-call, or
    /// reach any instruction of its function. Its state would reach all of
    /// them, including the GOT-pointer setup, so the walk forgets everything:
    /// every stub entered from what it reaches gets no evidence, and nothing is
    /// kept for callers.
    fn unresolved(&mut self, walk: &mut Walk, pc: u32) {
        walk.exits = Summary {
            kept: 0,
            returns: true,
        };
        if walk.flooded {
            return;
        }
        walk.flooded = true;
        walk.queue.clear();
        let image = self.image;
        let mut work: Vec<u32> = walk.states.drain().map(|(pc, _)| pc).collect();
        let mut seen = FxHashSet::default();
        let mut scopes = Vec::new();
        let scope = image.scope(pc);
        scopes.push(scope);
        walk.evidence
            .blocked
            .extend(image.stubs.range(scope.0..scope.1).map(|(&s, _)| s));
        work.extend((scope.0..scope.1).step_by(4));
        work.push(pc);
        while let Some(pc) = work.pop() {
            if self.budget == 0 {
                return;
            }
            if !seen.insert(pc) {
                continue;
            }
            self.budget -= 1;
            if let Some(stub) = image.stub_at(pc) {
                if stub == pc {
                    walk.evidence.events.push((stub, None));
                } else {
                    walk.evidence.blocked.push(stub);
                }
                continue;
            }
            let Some(word) = image.word_at(pc) else { continue };
            if image.import_exit(pc) || pc != walk.seed && image.seeds.contains(&pc) {
                continue;
            }
            if image.sites.contains(&pc) {
                walk.evidence.reached.push(pc);
            }
            let next = pc.wrapping_add(4);
            if calls(pc, word) {
                work.extend(image.pad(pc));
                if let Some(stub) = branch(pc, word).and_then(|b| image.stub_at(b.target)) {
                    work.push(stub);
                }
                if !image.ends.contains(&next) {
                    work.push(next);
                }
                continue;
            }
            if let Some(b) = branch(pc, word) {
                work.push(b.target);
                if b.conditional {
                    work.push(next);
                }
                continue;
            }
            if word >> 26 == 19 && matches!((word >> 1) & 1023, 16 | 18 | 50 | 528) {
                let scope = image.scope(pc);
                if !scopes.contains(&scope) {
                    scopes.push(scope);
                    walk.evidence
                        .blocked
                        .extend(image.stubs.range(scope.0..scope.1).map(|(&s, _)| s));
                    work.extend((scope.0..scope.1).step_by(4));
                }
                if (word >> 21) & 20 != 20 {
                    work.push(next);
                }
                continue;
            }
            if !trap(word) {
                work.push(next);
            }
        }
    }

    fn visit(&mut self, walk: &mut Walk, pc: u32, state: State) {
        let image = self.image;
        let Some(word) = image.word_at(pc) else { return };
        if image.sites.contains(&pc) {
            walk.evidence.reached.push(pc);
        }
        if calls(pc, word) {
            if let Some(pad) = image.pad(pc) {
                self.flow(walk, pad, state.keep(NONVOLATILE));
            }
        }
        let next = pc.wrapping_add(4);
        if let Some(b) = branch(pc, word) {
            if b.link {
                let mut linked = state;
                linked.lr = Value::Constant(next);
                if b.target == next || pc_capture(word) {
                    self.flow(walk, b.target, linked);
                    return;
                }
                if b.conditional {
                    self.flow(walk, next, linked);
                }
                let after = self.call(walk, b.target, state);
                if let Some(after) = after.filter(|_| !image.ends.contains(&next)) {
                    self.flow(walk, next, after);
                }
            } else if b.conditional && word >> 26 == 16 {
                self.flow(walk, b.target, state.refine(word, true));
                self.flow(walk, next, state.refine(word, false));
            } else {
                self.flow(walk, b.target, state);
            }
            return;
        }
        if word >> 26 == 19 {
            match (word >> 1) & 1023 {
                16 | 528 => self.indirect(walk, pc, word, state),
                18 | 50 => self.unresolved(walk, pc),
                _ => {
                    let mut state = state;
                    state.compare = None;
                    self.flow(walk, next, state);
                }
            }
            return;
        }
        if trap(word) || word == 0x4400_0002 && matches!(state.regs[0], Value::Constant(1 | 234)) {
            return;
        }
        let after = step(image, &mut walk.evidence, pc, word, state);
        self.flow(walk, next, after);
    }

    /// The state at the return address, or `None` when the callee never returns.
    fn call(&mut self, walk: &mut Walk, target: u32, state: State) -> Option<State> {
        let image = self.image;
        for r in 3..=10 {
            walk.evidence.escape(image, state.regs[r]);
        }
        if let Some(stub) = image.stub_at(target) {
            if stub == target {
                walk.evidence.enter(image, stub, &state);
                if image.terminal(stub, &state) {
                    return None;
                }
            } else {
                walk.evidence.blocked.push(stub);
            }
            return Some(state.keep(NONVOLATILE));
        }
        if image.import_exit(target) || !image.is_code(target) {
            return Some(state.keep(NONVOLATILE));
        }
        if image.straight(target, false) {
            let mut inline = state;
            inline.lr = Value::Unknown;
            let mut pc = target;
            while let Some(word) = image.word_at(pc).filter(|&w| w != 0x4e80_0020) {
                inline = step(image, &mut walk.evidence, pc, word, inline);
                pc = pc.wrapping_add(4);
            }
            inline.lr = Value::Unknown;
            return Some(inline);
        }
        let summary = self.summary(walk.seed, target);
        summary.returns.then(|| state.keep(summary.kept))
    }

    fn indirect(&mut self, walk: &mut Walk, pc: u32, word: u32, state: State) {
        let link = word & 1 != 0;
        let target = if (word >> 1) & 1023 == 528 {
            state.ctr
        } else {
            state.lr
        };
        let next = pc.wrapping_add(4);
        if (word >> 21) & 20 != 20 {
            let mut skipped = state;
            if link {
                skipped.lr = Value::Constant(next);
            }
            self.flow(walk, next, skipped);
        }
        if link {
            let after = match target {
                Value::Constant(callee) => self.call(walk, callee, state),
                _ => Some(state.keep(NONVOLATILE)),
            };
            if let Some(after) = after.filter(|_| !self.image.ends.contains(&next)) {
                self.flow(walk, next, after);
            }
            return;
        }
        match target {
            Value::Constant(to) => self.flow(walk, to, state),
            Value::Case(base, bound) | Value::Table(base, bound) => {
                let relative = matches!(target, Value::Case(..));
                let targets = self.image.table(pc, base, relative, bound);
                if targets.is_empty() {
                    self.unresolved(walk, pc);
                }
                for to in targets {
                    self.flow(walk, to, state);
                }
            }
            Value::Return => walk.exit(&state, CALLEE),
            Value::Import => self.leave(walk, state, CALLEE),
            _ => self.unresolved(walk, pc),
        }
    }
}

/// One non-branch instruction. Unrecognized encodings forget every register
/// and stack word they could write.
fn step(image: &Image<'_>, evidence: &mut Evidence, pc: u32, word: u32, mut s: State) -> State {
    let op = word >> 26;
    if !keeps_cr(word) {
        s.compare = None;
    }
    let rt = ((word >> 21) & 31) as usize;
    let ra = ((word >> 16) & 31) as usize;
    let imm = signed(word);
    let ea = s.base(ra).add(imm);
    match op {
        14 => s.regs[rt] = ea,
        15 => s.regs[rt] = s.base(ra).add(imm << 16),
        12 | 13 => s.regs[rt] = s.regs[ra].add(imm),
        7 | 8 => s.regs[rt] = Value::Unknown,
        10 if word & 0x0060_0000 == 0 && ra != 0 => {
            let bound = (word & 0xffff) as u16;
            if bound != UNBOUNDED && matches!(s.regs[ra], Value::Unknown | Value::Bounded(_) | Value::Mark(_)) {
                s.regs[ra] = Value::Mark(pc);
                s.compare = Some(((word >> 23) as u8 & 7, ra as u8, bound, Value::Mark(pc)));
            } else {
                s.compare = None;
            }
        }
        2 | 3 | 10 | 11 | 59 | 63 => {}
        24..=29 => {
            let k = if op & 1 == 1 {
                (word & 0xffff) << 16
            } else {
                word & 0xffff
            };
            s.regs[ra] = match (op, s.regs[rt]) {
                (24 | 25 | 26 | 27, v) if k == 0 => v,
                (24 | 25, Value::Constant(v)) => Value::Constant(v | k),
                (26 | 27, Value::Constant(v)) => Value::Constant(v ^ k),
                (28 | 29, Value::Constant(v)) => Value::Constant(v & k),
                _ => Value::Unknown,
            };
        }
        20 | 21 | 23 => {
            let (sh, mb, me) = ((word >> 11) & 31, (word >> 6) & 31, (word >> 1) & 31);
            s.regs[ra] = match (op, s.regs[rt]) {
                (21, v) if sh == 2 && me == 29 && mb <= 29 => {
                    let masked = if mb > 15 { (1u16 << (30 - mb)) - 1 } else { UNBOUNDED };
                    match v {
                        Value::Bounded(n) => Value::Index(n.min(masked)),
                        _ => Value::Index(masked),
                    }
                }
                (21, Value::Constant(v)) => Value::Constant(v.rotate_left(sh) & rotate_mask(mb, me)),
                _ => Value::Unknown,
            };
        }
        32..=35 | 40..=43 => {
            let value = if op == 32 {
                s.load(image, ea)
            } else {
                Value::Unknown
            };
            if op & 1 == 1 {
                if ra == 0 || ra == rt {
                    s.regs[ra] = Value::Unknown;
                    s.regs[rt] = Value::Unknown;
                    return s;
                }
                s.regs[ra] = ea;
            }
            s.regs[rt] = value;
        }
        36..=39 | 44 | 45 => {
            let width = match op {
                36 | 37 => 4,
                38 | 39 => 1,
                _ => 2,
            };
            let value = if width == 4 {
                s.regs[rt]
            } else {
                Value::Unknown
            };
            evidence.escape(image, value);
            s.store(ea, width, value);
            if op & 1 == 1 {
                s.regs[ra] = ea;
            }
        }
        46 => {
            if (rt..32).contains(&ra) {
                s.regs[rt..].fill(Value::Unknown);
            } else {
                for r in rt..32 {
                    s.regs[r] = s.load(image, ea.add(4 * (r - rt) as u32));
                }
            }
        }
        47 => {
            for r in rt..32 {
                evidence.escape(image, s.regs[r]);
                let value = s.regs[r];
                s.store(ea.add(4 * (r - rt) as u32), 4, value);
            }
        }
        48..=51 => {
            if op & 1 == 1 {
                s.regs[ra] = ea;
            }
        }
        52..=55 => {
            s.store(ea, if op >= 54 { 8 } else { 4 }, Value::Unknown);
            if op & 1 == 1 {
                s.regs[ra] = ea;
            }
        }
        17 => {
            s.regs[0] = Value::Unknown;
            s.regs[3..=12].fill(Value::Unknown);
            s.ctr = Value::Unknown;
        }
        31 => return indexed(image, evidence, word, s),
        _ => {
            s.regs = [Value::Unknown; 32];
            s.saved = [UNSAVED; 32];
            s.slots = [None; SLOTS];
        }
    }
    s
}

fn indexed(image: &Image<'_>, evidence: &mut Evidence, word: u32, mut s: State) -> State {
    let rt = ((word >> 21) & 31) as usize;
    let ra = ((word >> 16) & 31) as usize;
    let rb = ((word >> 11) & 31) as usize;
    let xo = (word >> 1) & 1023;
    let spr = ((word >> 16) & 31) | ((word >> 6) & 0x3e0);
    let ea = s.base(ra).sum(s.regs[rb]);
    let store = |s: &mut State, evidence: &mut Evidence, width: u32, gpr: bool| {
        let value = if gpr && width == 4 {
            s.regs[rt]
        } else {
            Value::Unknown
        };
        evidence.escape(image, value);
        let at = match ea {
            Value::Stack(at) if !gpr => Value::Stack(at & !(width - 1)),
            other => other,
        };
        s.store(at, width, value);
    };
    match xo {
        0 | 4 | 6 | 7 | 32 | 38 | 39 | 54 | 71 | 86 | 103 | 131 | 144 | 146 | 163 | 246 | 278
        | 342 | 359 | 374 | 470 | 512 | 598 | 822 | 854 | 982 => {}
        467 => {
            evidence.escape(image, s.regs[rt]);
            match spr {
                8 => s.lr = s.regs[rt],
                9 => s.ctr = s.regs[rt],
                _ => {}
            }
        }
        339 => {
            s.regs[rt] = match spr {
                8 => s.lr,
                9 => s.ctr,
                _ => Value::Unknown,
            }
        }
        23 => s.regs[rt] = s.load(image, ea),
        19 | 20 | 83 | 87 | 279 | 343 | 371 | 534 | 595 | 659 | 790 => {
            s.regs[rt] = Value::Unknown
        }
        55 | 119 | 311 | 375 => {
            if ra == 0 || ra == rt {
                s.regs[ra] = Value::Unknown;
            } else {
                s.regs[ra] = ea;
            }
            s.regs[rt] = Value::Unknown;
        }
        535 | 599 => {}
        567 | 631 => s.regs[ra] = ea,
        150 | 151 | 662 => store(&mut s, evidence, 4, true),
        215 => store(&mut s, evidence, 1, true),
        407 | 918 => store(&mut s, evidence, 2, true),
        663 | 983 | 199 => store(&mut s, evidence, 4, false),
        135 => store(&mut s, evidence, 1, false),
        167 => store(&mut s, evidence, 2, false),
        727 => store(&mut s, evidence, 8, false),
        231 | 487 => store(&mut s, evidence, 16, false),
        183 | 247 | 439 | 695 | 759 => {
            let (width, gpr) = match xo {
                183 => (4, true),
                247 => (1, true),
                439 => (2, true),
                695 => (4, false),
                _ => (8, false),
            };
            store(&mut s, evidence, width, gpr);
            s.regs[ra] = ea;
        }
        758 | 1014 => s.store(ea.add(0u32.wrapping_sub(128)), 256, Value::Unknown),
        725 | 661 => {
            let at = if xo == 725 { s.base(ra) } else { ea };
            s.store(at, 128, Value::Unknown);
        }
        533 | 597 => s.regs = [Value::Unknown; 32],
        444 => {
            s.regs[ra] = match (s.regs[rt], s.regs[rb]) {
                (v, _) if rt == rb => v,
                (Value::Constant(a), Value::Constant(b)) => Value::Constant(a | b),
                _ => Value::Unknown,
            }
        }
        24 | 26 | 28 | 60 | 122 | 124 | 154 | 284 | 316 | 378 | 412 | 476 | 508 | 536 | 792
        | 824 | 922 | 954 => s.regs[ra] = Value::Unknown,
        _ => match xo & 511 {
            266 => {
                s.regs[rt] = match (s.regs[ra], s.regs[rb]) {
                    (Value::Table(t, n), Value::Constant(c))
                    | (Value::Constant(c), Value::Table(t, n))
                        if t == c =>
                    {
                        Value::Case(t, n)
                    }
                    (a, b) => a.sum(b),
                }
            }
            40 => {
                s.regs[rt] = match (s.regs[ra], s.regs[rb]) {
                    (Value::Constant(a), b) => b.add(a.wrapping_neg()),
                    _ => Value::Unknown,
                }
            }
            8 | 10 | 11 | 75 | 104 | 136 | 138 | 200 | 202 | 232 | 234 | 235 | 459 | 491 => {
                s.regs[rt] = Value::Unknown
            }
            _ if (word >> 1) & 31 == 15 => s.regs[rt] = Value::Unknown,
            _ => {
                s.regs[rt] = Value::Unknown;
                s.regs[ra] = Value::Unknown;
                s.saved = [UNSAVED; 32];
                s.slots = [None; SLOTS];
            }
        },
    }
    s
}

/// The complete GNU/LLD lazy resolver loads its branch target from GOT[1].
/// This is a loader-defined dispatch, not an arbitrary application indirect edge.
fn resolver_exit(file: &object::File<'_>, code: &[(u32, u32)]) -> BTreeSet<u32> {
    let mut exits = BTreeSet::new();
    let Some(dynamic) = file.section_by_name(".dynamic") else {
        return exits;
    };
    let Ok(bytes) = dynamic.data() else {
        return exits;
    };
    let mut got = None;
    for tag in bytes.chunks_exact(8) {
        let key = super::kuna_ppc32_imports::word(file, tag);
        if key == 0 {
            break;
        }
        if i64::from(key) == object::elf::DT_PPC_GOT {
            if got.is_some() {
                return exits;
            }
            got = Some(super::kuna_ppc32_imports::word(file, &tag[4..]));
        }
    }
    let Some(got) = got else { return exits };
    if got & 3 != 0
        || !file.sections().any(|section| {
            matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 2)
                && u64::from(got) >= section.address()
                && u64::from(got) + 12 <= section.address().saturating_add(section.size())
                && section
                    .data()
                    .ok()
                    .and_then(|data| data.get((u64::from(got) - section.address()) as usize..))
                    .is_some_and(|data| {
                        data.len() >= 12
                            && u64::from(super::kuna_ppc32_imports::word(file, data))
                                == dynamic.address()
                    })
        })
    {
        return exits;
    }
    let mut sources = None;
    for window in code.windows(14) {
        let start = window[0].0;
        if window[13].0 != start + 52 {
            continue;
        }
        let w: [u32; 14] = std::array::from_fn(|i| window[i].1);
        if w[0] & 0xffff0000 != 0x3d6b0000
            || w[1] != 0x7c0802a6
            || w[2] != 0x429f0005
            || w[3] & 0xffff0000 != 0x396b0000
            || w[4..7] != [0x7d8802a6, 0x7c0803a6, 0x7d6c5850]
            || w[7] & 0xffff0000 != 0x3d8c0000
            || w[8] & 0xffff0000 != 0x800c0000
            || w[9] & 0xffff0000 != 0x818c0000
            || w[10..] != [0x7c0903a6, 0x7c0b5a14, 0x7d605a14, 0x4e800420]
        {
            continue;
        }
        let base = (start + 12).wrapping_add(w[7] << 16);
        if base.wrapping_add(signed(w[8])) != got.wrapping_add(4)
            || base.wrapping_add(signed(w[9])) != got.wrapping_add(8)
        {
            continue;
        }
        // An entry bypassing the load invalidates the dispatch certificate.
        let sources = sources.get_or_insert_with(|| {
            let mut sources: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
            for &(pc, word) in code {
                if let Some(b) = branch(pc, word) {
                    sources.entry(b.target).or_default().push(pc);
                }
            }
            sources
        });
        if sources
            .range(start + 1..=start + 52)
            .any(|(_, pcs)| pcs.iter().any(|&pc| pc != start + 8))
        {
            continue;
        }
        exits.insert(start + 52);
    }
    exits
}

/// [`prove`], remembered for the last few images: `format::resolve_imports` runs
/// about twenty times per load. The proof reads only the image, so the key hashes
/// every byte of it and its length.
pub(super) fn resolve(
    file: &object::File<'_>,
    slots: &BTreeMap<u64, Vec<u8>>,
    imports: &[PltSym],
) -> Vec<PltSym> {
    type Entry = ((usize, u64, u64), Vec<(u64, Vec<u8>)>);
    static CACHE: Mutex<Vec<Entry>> = Mutex::new(Vec::new());
    let object::File::Elf32(elf) = file else {
        return prove(file, slots, imports);
    };
    let key = fingerprint(elf.data());
    let unpack = |found: &[(u64, Vec<u8>)]| {
        found
            .iter()
            .map(|(addr, name)| PltSym {
                addr: *addr,
                name: name.clone(),
            })
            .collect()
    };
    if let Some((_, hit)) = CACHE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|e| e.0 == key)
    {
        return unpack(hit);
    }
    let found: Vec<_> = prove(file, slots, imports)
        .into_iter()
        .map(|s| (s.addr, s.name))
        .collect();
    let out = unpack(&found);
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if cache.len() == 4 {
        cache.remove(0);
    }
    cache.push((key, found));
    out
}

fn fingerprint(data: &[u8]) -> (usize, u64, u64) {
    const K: u64 = 0x9E37_79B9_7F4A_7C15;
    let mix = |h: u64, v: u64| (h ^ v).wrapping_mul(K).rotate_left(29);
    let (mut a, mut b) = (0x243F_6A88_85A3_08D3u64, 0x1319_8A2E_0370_7344u64);
    let mut chunks = data.chunks_exact(16);
    for c in &mut chunks {
        a = mix(a, u64::from_le_bytes(c[..8].try_into().unwrap()));
        b = mix(b, u64::from_le_bytes(c[8..].try_into().unwrap()));
    }
    for &byte in chunks.remainder() {
        a = mix(a, u64::from(byte));
        b = mix(b, u64::from(byte).rotate_left(17));
    }
    (data.len(), a, b)
}

/// FDE ranges and LSDA call-site landing pads. `None` when exception tables
/// exist but cannot be read, since unread landing pads are unmodeled entries.
#[allow(clippy::type_complexity)]
fn frames(
    file: &object::File<'_>,
    image: &Image<'_>,
) -> Option<(Vec<(u32, u32)>, Vec<(u32, u32, u32)>, Vec<(u32, u32)>)> {
    use gimli::UnwindSection;
    let mut ranges = Vec::new();
    let mut pads = Vec::new();
    let mut opaque = Vec::new();
    let Some(section) = file.section_by_name(".eh_frame") else {
        return Some((ranges, pads, opaque));
    };
    let data = section.data().ok()?;
    let endian = if image.little {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let mut eh = gimli::EhFrame::new(data, endian);
    eh.set_address_size(4);
    let mut bases = gimli::BaseAddresses::default().set_eh_frame(section.address());
    if let Some(text) = file.section_by_name(".text") {
        bases = bases.set_text(text.address());
    }
    if let Some(got) = file.section_by_name(".got") {
        bases = bases.set_got(got.address());
    }
    let mut entries = eh.entries(&bases);
    loop {
        let entry = entries.next().ok()?;
        let Some(entry) = entry else { break };
        let gimli::CieOrFde::Fde(partial) = entry else {
            continue;
        };
        let fde = partial.parse(|_, bases, o| eh.cie_from_offset(bases, o)).ok()?;
        let start = u32::try_from(fde.initial_address()).ok()?;
        let end = start.checked_add(u32::try_from(fde.len()).ok()?)?;
        if end > start {
            ranges.push((start, end));
        }
        match fde.lsda() {
            None => {}
            Some(gimli::Pointer::Direct(lsda)) => {
                match u32::try_from(lsda).ok().and_then(|at| call_sites(file, image, at, start)) {
                    Some(sites) => pads.extend(sites),
                    None => opaque.push((start, end)),
                }
            }
            Some(gimli::Pointer::Indirect(_)) => opaque.push((start, end)),
        }
    }
    ranges.sort_unstable();
    pads.sort_unstable();
    opaque.sort_unstable();
    Some((ranges, pads, opaque))
}

fn call_sites(
    file: &object::File<'_>,
    image: &Image<'_>,
    at: u32,
    function: u32,
) -> Option<Vec<(u32, u32, u32)>> {
    let section = file.sections().find(|s| {
        (s.address()..s.address().saturating_add(s.size())).contains(&u64::from(at))
    })?;
    let bytes = section.data().ok()?;
    let mut p = (u64::from(at) - section.address()) as usize;
    let byte = |p: &mut usize| -> Option<u8> {
        let b = *bytes.get(*p)?;
        *p += 1;
        Some(b)
    };
    let uleb = |p: &mut usize| -> Option<u32> {
        let mut value = 0u64;
        for shift in (0..35).step_by(7) {
            let b = *bytes.get(*p)?;
            *p += 1;
            value |= u64::from(b & 0x7f) << shift;
            if b & 0x80 == 0 {
                return u32::try_from(value).ok();
            }
        }
        None
    };
    let encoded = |p: &mut usize, enc: u8| -> Option<u32> {
        let field = section.address() as u32 + *p as u32;
        let raw = match enc & 0x0f {
            0x00 | 0x03 | 0x0b => {
                let v = image.read(bytes.get(*p..*p + 4)?);
                *p += 4;
                v
            }
            0x01 => uleb(p)?,
            0x02 | 0x0a => {
                let b = bytes.get(*p..*p + 2)?;
                *p += 2;
                let v = if image.little {
                    u16::from_le_bytes([b[0], b[1]])
                } else {
                    u16::from_be_bytes([b[0], b[1]])
                };
                if enc & 0x0f == 0x0a {
                    v as i16 as i32 as u32
                } else {
                    u32::from(v)
                }
            }
            _ => return None,
        };
        match enc & 0x70 {
            0x00 => Some(raw),
            0x10 => Some(field.wrapping_add(raw)),
            _ => None,
        }
    };
    let lp_enc = byte(&mut p)?;
    let lp_start = if lp_enc == 0xff {
        function
    } else {
        encoded(&mut p, lp_enc)?
    };
    if byte(&mut p)? != 0xff {
        uleb(&mut p)?;
    }
    let cs_enc = byte(&mut p)?;
    let length = uleb(&mut p)? as usize;
    let end = p.checked_add(length)?;
    if end > bytes.len() {
        return None;
    }
    let mut out = Vec::new();
    while p < end {
        let start = encoded(&mut p, cs_enc)?;
        let len = encoded(&mut p, cs_enc)?;
        let pad = encoded(&mut p, cs_enc)?;
        uleb(&mut p)?;
        if pad != 0 {
            let first = lp_start.wrapping_add(start);
            out.push((first, first.wrapping_add(len), lp_start.wrapping_add(pad)));
        }
    }
    (p == end).then_some(out)
}

fn prove(
    file: &object::File<'_>,
    slots: &BTreeMap<u64, Vec<u8>>,
    imports: &[PltSym],
) -> Vec<PltSym> {
    let mut image = Image {
        little: file.is_little_endian(),
        slots: slots.clone(),
        ..Image::default()
    };
    let mut code = Vec::new();
    for section in file.sections() {
        let SectionFlags::Elf { sh_flags } = section.flags() else {
            continue;
        };
        if sh_flags & 2 == 0 || section.address().saturating_add(section.size()) > u32::MAX as u64 {
            continue;
        }
        let Ok(bytes) = section.data() else { continue };
        let start = section.address() as u32;
        if sh_flags & 4 == 0 {
            if !bytes.is_empty() {
                let name = section.name().unwrap_or("");
                image.data.push(DataSection {
                    start,
                    bytes,
                    stable: sh_flags & 1 == 0
                        || name == ".got2" && file.kind() == object::ObjectKind::Executable,
                    pointers: matches!(
                        section.kind(),
                        SectionKind::Data | SectionKind::ReadOnlyData | SectionKind::Tls
                    ),
                });
            }
            continue;
        }
        if start & 3 != 0 {
            continue;
        }
        let words: Vec<_> = bytes.chunks_exact(4).map(|b| image.read(b)).collect();
        for (i, window) in words.windows(4).enumerate() {
            if let Some(delta) = pic_delta(window) {
                image.stubs.insert(start + i as u32 * 4, delta);
            }
        }
        code.extend(words.iter().enumerate().map(|(i, &w)| (start + i as u32 * 4, w)));
        image.bounds.insert(start);
        image.bounds.insert(start.wrapping_add(words.len() as u32 * 4));
        image.code.push((start, words));
    }
    if image.stubs.is_empty() {
        return Vec::new();
    }
    image.code.sort_unstable_by_key(|(start, _)| *start);
    image.data.sort_unstable_by_key(|d| d.start);
    code.sort_unstable_by_key(|&(pc, _)| pc);
    if code.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Vec::new();
    }
    let mut blocked = BTreeSet::new();
    let mut pointers = BTreeSet::new();
    let block = |blocked: &mut BTreeSet<u32>, image: &Image<'_>, at: u64| {
        if let Some(stub) = u32::try_from(at).ok().and_then(|at| image.stub_at(at)) {
            blocked.insert(stub);
        }
    };
    let defined: BTreeMap<_, _> = file
        .dynamic_symbols()
        .filter(|symbol| !symbol.is_undefined())
        .map(|symbol| (symbol.index().0, symbol.address()))
        .collect();
    if let Some(relocations) = file.dynamic_relocations() {
        for (address, relocation) in relocations {
            let Ok(at) = u32::try_from(address) else {
                continue;
            };
            let width = u64::from(relocation.size()).div_ceil(8).max(4) as u32;
            if (0..width).any(|i| image.is_code(at.wrapping_add(i) & !3)) {
                // Runtime writes can change both callee clobbers and the edge census.
                return Vec::new();
            }
            let r_type = match relocation.flags() {
                object::RelocationFlags::Elf { r_type } => r_type,
                _ => 0,
            };
            let addend = relocation.addend() as u32;
            let value = match relocation.target() {
                object::RelocationTarget::Symbol(index) => match defined.get(&index.0) {
                    Some(&value) => Reloc::Address((value as u32).wrapping_add(addend)),
                    None => Reloc::Import,
                },
                _ if r_type == object::elf::R_PPC_RELATIVE => Reloc::Address(addend),
                _ => Reloc::Other,
            };
            let value = match (r_type, value) {
                (
                    object::elf::R_PPC_ADDR32
                    | object::elf::R_PPC_GLOB_DAT
                    | object::elf::R_PPC_JMP_SLOT
                    | object::elf::R_PPC_RELATIVE,
                    v,
                ) => v,
                _ => Reloc::Other,
            };
            if let Reloc::Address(v) = value {
                pointers.insert(v);
            }
            image.relocs.insert(at, value);
        }
    }
    for section in image.data.iter().filter(|d| d.pointers) {
        for word in section.bytes.chunks_exact(4) {
            let value = image.read(word);
            block(&mut blocked, &image, u64::from(value));
            if file.kind() == object::ObjectKind::Executable {
                pointers.insert(value);
            }
        }
    }
    for &v in &pointers {
        block(&mut blocked, &image, u64::from(v));
    }
    let mut entries = BTreeSet::new();
    let mut functions = Vec::new();
    let mut exported = BTreeSet::new();
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if symbol.is_undefined() || symbol.address() > u64::from(u32::MAX) {
            continue;
        }
        let addr = symbol.address() as u32;
        if let Some(stub) = image.stub_at(addr) {
            if addr != stub || symbol.is_global() {
                blocked.insert(stub);
            }
            continue;
        }
        entries.insert(addr);
        if symbol.is_global() {
            exported.insert(addr);
        }
        if symbol.kind() == SymbolKind::Text && symbol.size() != 0 {
            if let Some(end) = u32::try_from(symbol.address() + symbol.size()).ok() {
                functions.push((addr, end));
            }
        }
    }
    if let Ok(entry) = u32::try_from(file.entry()) {
        block(&mut blocked, &image, u64::from(entry));
        entries.insert(entry);
        exported.insert(entry);
    }
    if let Some(dynamic) = file.section_by_name(".dynamic").and_then(|s| s.data().ok()) {
        for tag in dynamic.chunks_exact(8) {
            if matches!(image.read(tag), 12 | 13) {
                let value = image.read(&tag[4..]);
                block(&mut blocked, &image, u64::from(value));
                entries.insert(value);
                exported.insert(value);
            }
        }
    }
    // Landing pads are entries; without a readable table only images that
    // cannot have one are analyzed.
    let Some((ranges, pads, opaque)) = frames(file, &image).or_else(|| {
        file.section_by_name(".gcc_except_table")
            .is_none()
            .then(|| (Vec::new(), Vec::new(), Vec::new()))
    }) else {
        return Vec::new();
    };
    for &(_, _, pad) in &pads {
        block(&mut blocked, &image, u64::from(pad));
    }
    let absolute: BTreeSet<_> = imports.iter().map(|s| s.addr as u32).collect();
    let mut resolvers = resolver_exit(file, &code);
    resolvers.retain(|&exit| {
        pointers.range((exit - 52 + 1)..=exit).next().is_none()
            && entries.range((exit - 52 + 1)..=exit).next().is_none()
    });
    image.resolvers = resolvers.iter().map(|&exit| (exit - 52, exit + 4)).collect();
    image.absolute = absolute;
    let mut calls = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut outside = BTreeSet::new();
    let mut sites: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for &(pc, word) in &code {
        let Some(b) = branch(pc, word) else { continue };
        if b.link && b.target == pc.wrapping_add(4) {
            continue;
        }
        targets.insert(b.target);
        if b.link && !pc_capture(word) {
            calls.insert(b.target);
        }
        if let Some(stub) = image.stub_at(b.target) {
            if stub == b.target {
                sites.entry(stub).or_default().push(pc);
            } else {
                blocked.insert(stub);
            }
        }
        if !b.link {
            let home = ranges.partition_point(|&(s, _)| s <= pc);
            let inside = home > 0 && (ranges[home - 1].0..ranges[home - 1].1).contains(&pc);
            if !inside || !(ranges[home - 1].0..ranges[home - 1].1).contains(&b.target) {
                outside.insert(b.target);
            }
        }
    }
    for &stub in image.stubs.keys() {
        let mut pc = stub.wrapping_sub(4);
        for _ in 0..16 {
            if image.word_at(pc) != Some(0x6000_0000) {
                break;
            }
            if targets.contains(&pc)
                || pointers.contains(&pc)
                || entries.contains(&pc)
                || pads.iter().any(|&(_, _, pad)| pad == pc)
            {
                blocked.insert(stub);
            }
            pc = pc.wrapping_sub(4);
        }
        let Some(word) = image.word_at(pc) else { continue };
        let terminal = branch(pc, word).is_some_and(|b| !b.link && !b.conditional)
            || matches!(word, 0x4e80_0020 | 0x4e80_0420);
        if !terminal && image.stub_at(pc).is_none() {
            sites.entry(stub).or_default().push(pc);
        }
    }
    // A function part that only other functions branch to (a `.cold` part)
    // is entered with their state, not as a function of its own.
    let part = |s: &u32| outside.contains(s) && !calls.contains(s) && !pointers.contains(s);
    let mut seeds: FxHashSet<u32> = calls.iter().chain(&pointers).copied().collect();
    seeds.extend(entries.iter().copied().filter(|s| exported.contains(s) || !part(s)));
    seeds.extend(ranges.iter().map(|&(s, _)| s).filter(|s| !part(s)));
    seeds.retain(|&s| image.is_code(s) && image.stub_at(s).is_none() && !image.import_exit(s));
    let mut scopes: Vec<(u32, u32)> = ranges.clone();
    scopes.extend(functions.iter().copied().filter(|&(s, e)| {
        e > s && !ranges.iter().any(|&(rs, re)| rs < e && s < re)
    }));
    scopes.sort_unstable();
    image.bounds.extend(seeds.iter().copied());
    for &(s, e) in scopes.iter() {
        image.bounds.insert(s);
        image.bounds.insert(e);
    }
    for &stub in image.stubs.keys() {
        image.bounds.insert(stub);
        image.bounds.insert(stub.wrapping_add(16));
    }
    for &(s, e) in &image.resolvers {
        image.bounds.insert(s);
        image.bounds.insert(e);
    }
    image.ends = scopes.iter().map(|&(_, e)| e).collect();
    image.targets = targets.iter().copied().collect();
    image.sites = sites.values().flatten().copied().collect();
    image.seeds = seeds;
    image.scopes = scopes;
    image.pads = pads;
    image.opaque = opaque;
    let mut order: Vec<_> = image.seeds.iter().copied().collect();
    order.sort_unstable();
    let words: u64 = image.code.iter().map(|(_, w)| w.len() as u64).sum();
    let mut analysis = Analysis {
        image: &image,
        summaries: FxHashMap::default(),
        dependents: FxHashMap::default(),
        active: FxHashSet::default(),
        stale: FxHashSet::default(),
        dirty: VecDeque::new(),
        queued: FxHashSet::default(),
        evidence: FxHashMap::default(),
        events: FxHashMap::default(),
        reached: FxHashSet::default(),
        blocked: FxHashSet::default(),
        budget: words.saturating_mul(64).max(1 << 20),
    };
    analysis.solve(&order);
    if analysis.budget == 0 {
        return Vec::new();
    }
    blocked.extend(analysis.blocked.iter().copied());
    let mut out = Vec::new();
    for (stub, callers) in sites {
        if blocked.contains(&stub) || !callers.iter().all(|pc| analysis.reached.contains(pc)) {
            continue;
        }
        if let Some(Some(slot)) = analysis.events.get(&stub) {
            out.push(PltSym {
                addr: u64::from(stub),
                name: image.slots[&u64::from(*slot)].clone(),
                slot: None,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[u32], state: State) -> State {
        let image = Image::default();
        let mut evidence = Evidence::default();
        words.iter().fold(state, |s, &w| step(&image, &mut evidence, 0x1000, w, s))
    }

    #[test]
    fn saved_r30_survives_pointer_stores_and_calls_but_not_its_own_slot() {
        // stwu r1,-16(r1); stw r30,8(r1); stw r3,8(r9); then a call.
        let saved = run(&[0x9421fff0, 0x93c10008, 0x90690008], State::entry()).keep(NONVOLATILE);
        // li r30,0; lwz r30,8(r1) restores the caller's value.
        assert_eq!(run(&[0x3bc00000, 0x83c10008], saved).regs[30], Value::Entry(30));
        // stw r4,8(r1) overwrites the save word.
        assert_eq!(run(&[0x90810008, 0x83c10008], saved).regs[30], Value::Unknown);
        // A byte store into the save word also clears it.
        assert_eq!(run(&[0x9881000b, 0x83c10008], saved).regs[30], Value::Unknown);
        // lwz r1,0(r1) follows the back chain; addi r1,r1,16 too.
        assert_eq!(run(&[0x80210000], saved).regs[1], Value::Stack(0));
        assert_eq!(run(&[0x38210010], saved).preserved() & 2, 2);
    }

    #[test]
    fn a_compare_bounds_only_the_register_it_tested() {
        let bgt_not_taken = |s: State| s.refine(0x41810010, false).regs[9];
        // cmplwi r9,5
        let compared = run(&[0x28090005], State::top());
        assert_eq!(bgt_not_taken(compared), Value::Bounded(5));
        // The taken edge (r9 > 5) learns nothing.
        assert_eq!(compared.refine(0x41810010, true).regs[9], Value::Mark(0x1000));
        // ble's taken edge is the bounded one.
        assert_eq!(compared.refine(0x40810010, true).regs[9], Value::Bounded(5));
        // lwz r9,0(r3) between compare and branch: a new value, unbounded.
        assert_ne!(bgt_not_taken(run(&[0x28090005, 0x81230000], State::top())), Value::Bounded(5));
        // cmpwi r10,0 rewrites cr0 before the branch.
        assert_ne!(bgt_not_taken(run(&[0x28090005, 0x2c0a0000], State::top())), Value::Bounded(5));
        // add. r3,r3,r4 writes cr0; add r3,r3,r4 does not.
        assert_ne!(bgt_not_taken(run(&[0x28090005, 0x7c632215], State::top())), Value::Bounded(5));
        assert_eq!(bgt_not_taken(run(&[0x28090005, 0x7c632214], State::top())), Value::Bounded(5));
        // A different field is a different test.
        assert_ne!(compared.refine(0x41850010, false).regs[9], Value::Bounded(5));
    }

    #[test]
    fn table_indices_carry_their_bound() {
        let mut s = State::top();
        s.regs[9] = Value::Bounded(3);
        // slwi r10,r9,2
        assert_eq!(run(&[0x552a103a], s).regs[10], Value::Index(3));
        // rlwinm r10,r9,2,27,29 masks to eight entries.
        assert_eq!(run(&[0x552a16fa], State::top()).regs[10], Value::Index(7));
        // A bounded mask keeps the tighter bound.
        s.regs[9] = Value::Bounded(100);
        assert_eq!(run(&[0x552a16fa], s).regs[10], Value::Index(7));
        // slwi of an unknown index is unbounded; any other shift is no index.
        assert_eq!(run(&[0x552a103a], State::top()).regs[10], Value::Index(UNBOUNDED));
        assert_eq!(run(&[0x552a183a], State::top()).regs[10], Value::Unknown);
    }

    #[test]
    fn floating_and_vector_stores_never_track_a_gpr() {
        // stw r30 then stfs f30 over it: the slot now holds a float.
        let s = run(&[0x9421fff0, 0x93c10008, 0xd3c10008, 0x83c10008], State::entry());
        assert_eq!(s.regs[30], Value::Unknown);
        // stfsx f30,r1,r9 with r9 = 8 (li r9,8).
        let s = run(
            &[0x9421fff0, 0x93c10008, 0x39200008, 0x7fc14d2e, 0x83c10008],
            State::entry(),
        );
        assert_eq!(s.regs[30], Value::Unknown);
    }

    #[test]
    fn tables_read_their_bounded_entries_inside_the_scope() {
        let image = Image {
            code: vec![(0x1000, vec![0x60000000, 0x4e800420, 8, 0xc, 0x10, 0x7fff0000])],
            scopes: vec![(0x1000, 0x1018)],
            ..Image::default()
        };
        assert_eq!(image.table(0x1004, 0x1008, true, 1), vec![0x1010, 0x1014]);
        // A bounded entry outside the scope is not a target; unbounded reads
        // stop at the first one.
        assert_eq!(image.table(0x1004, 0x1008, true, 3), vec![0x1010, 0x1014]);
        assert_eq!(image.table(0x1004, 0x1008, true, UNBOUNDED), vec![0x1010, 0x1014]);
    }

    #[test]
    fn classifies_traps_pc_capture_and_calls() {
        assert!(trap(0x7fe00008));
        assert!(trap(0x0fe00000));
        assert!(!trap(0x7c832008));
        assert!(pc_capture(0x429f0005));
        assert!(pc_capture(0x429f0009));
        assert!(!pc_capture(0x48000005));
        assert!(calls(0x1000, 0x48000101));
        assert!(!calls(0x1000, 0x48000005));
        assert!(!calls(0x1000, 0x429f0009));
        assert!(calls(0x1000, 0x4e800421));
        assert!(!calls(0x1000, 0x4e800420));
        assert!(keeps_cr(0x7c632214));
        assert!(!keeps_cr(0x7c632215));
        assert!(!keeps_cr(0x2c030000));
    }
}
