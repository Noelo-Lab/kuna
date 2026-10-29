//! Resolve secure-PLT PIC entries from bounded caller control flow. A stub needs
//! the same validated slot at every direct incoming edge; table metadata and
//! linker labels never supply a register value. Callers without a disjoint text
//! symbol extent, unknown transfers, and address-taken stubs are refused.
use super::elf_plt::PltSym;
use object::{Object, ObjectSection, ObjectSymbol, SectionFlags, SectionKind, SymbolKind};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Value {
    #[default]
    Unknown,
    Constant(u32),
    Return,
    Stack(u32),
}

impl Value {
    fn add(self, offset: u32) -> Self {
        match self {
            Self::Constant(value) => Self::Constant(value.wrapping_add(offset)),
            Self::Stack(value) => Self::Stack(value.wrapping_add(offset)),
            other if offset == 0 => other,
            _ => Self::Unknown,
        }
    }

    fn constant(self) -> Option<u32> {
        if let Self::Constant(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct State {
    regs: [Value; 32],
    lr: Value,
    saved_return: Option<u32>,
}

impl State {
    fn entry() -> Self {
        let mut state = Self {
            lr: Value::Return,
            ..Self::default()
        };
        state.regs[1] = Value::Stack(0);
        state
    }

    fn meet(self, other: Self) -> Self {
        Self {
            regs: std::array::from_fn(|i| {
                if self.regs[i] == other.regs[i] {
                    self.regs[i]
                } else {
                    Value::Unknown
                }
            }),
            lr: if self.lr == other.lr {
                self.lr
            } else {
                Value::Unknown
            },
            saved_return: self.saved_return.filter(|v| Some(*v) == other.saved_return),
        }
    }

    fn base(self, register: usize) -> Value {
        if register == 0 {
            Value::Constant(0)
        } else {
            self.regs[register]
        }
    }

    fn store(&mut self, address: Value, width: u32, value: Value) {
        if let Some(saved) = self.saved_return {
            let disjoint = matches!(address, Value::Stack(at)
                if saved.wrapping_sub(at) >= width && at.wrapping_sub(saved) >= 4);
            if !disjoint {
                self.saved_return = None;
            }
        }
        if width == 4 && value == Value::Return {
            if let Value::Stack(at) = address {
                self.saved_return = Some(at);
            }
        }
    }

    fn after_import(&mut self) {
        if self.saved_return != Some(4)
            || !matches!(self.regs[1], Value::Stack(at) if (at as i32) <= -16)
            || self.regs[3..11]
                .iter()
                .any(|v| matches!(v, Value::Stack(_)))
        {
            self.saved_return = None;
        }
        self.regs[0] = Value::Unknown;
        self.regs[2..14].fill(Value::Unknown);
        self.lr = Value::Unknown;
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

fn signed(word: u32) -> u32 {
    word as u16 as i16 as u32
}

/// Track constants separately from the entry LR token and its stack spill.
/// Unsupported effects discard evidence; unknown values are never returns.
fn transfer(word: u32, mut state: State) -> State {
    let rt = ((word >> 21) & 31) as usize;
    let ra = ((word >> 16) & 31) as usize;
    let rb = ((word >> 11) & 31) as usize;
    let opcode = word >> 26;
    let address = state.base(ra).add(signed(word));
    match opcode {
        14 | 15 => {
            state.regs[rt] = state.base(ra).add(if opcode == 15 {
                signed(word) << 16
            } else {
                signed(word)
            });
        }
        10 | 11 | 48 | 50 => {}
        24 | 25 => {
            let mask = if opcode == 25 {
                word << 16
            } else {
                word & 0xffff
            };
            state.regs[ra] = if mask == 0 {
                state.regs[rt]
            } else {
                state.regs[rt]
                    .constant()
                    .map_or(Value::Unknown, |v| Value::Constant(v | mask))
            };
        }
        20..=29 => state.regs[ra] = Value::Unknown,
        32..=35 | 40..=43 => {
            state.regs[rt] = if opcode == 32
                && matches!(address, Value::Stack(at) if Some(at) == state.saved_return)
            {
                Value::Return
            } else {
                Value::Unknown
            };
            if opcode & 1 != 0 {
                state.regs[ra] = if ra == rt { Value::Unknown } else { address };
            }
        }
        36..=39 | 44..=45 | 47 | 52..=55 => {
            let width = match opcode {
                38 | 39 => 1,
                44 | 45 => 2,
                47 => 4 * (32 - rt as u32),
                54 | 55 => 8,
                _ => 4,
            };
            let value = if matches!(opcode, 36 | 37) {
                state.regs[rt]
            } else {
                Value::Unknown
            };
            state.store(address, width, value);
            if matches!(opcode, 37 | 39 | 45 | 53 | 55) {
                state.regs[ra] = address;
            }
        }
        49 | 51 => state.regs[ra] = address,
        46 => state.regs[rt..].fill(Value::Unknown),
        31 if word & 0xfc1f_ffff == 0x7c08_02a6 => state.regs[rt] = state.lr,
        31 if word & 0xfc1f_ffff == 0x7c08_03a6 => state.lr = state.regs[rt],
        31 if word & 0xfc1f_ffff == 0x7c09_03a6 => {}
        31 if (word >> 1) & 1023 == 444 && rt == rb => state.regs[ra] = state.regs[rt],
        _ => state = State::default(),
    }
    state
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

fn slot(
    pc: u32,
    state: State,
    stubs: &BTreeMap<u32, u32>,
    slots: &BTreeMap<u64, Vec<u8>>,
) -> Option<u32> {
    let addr = state.regs[30].constant()?.wrapping_add(*stubs.get(&pc)?);
    slots.contains_key(&u64::from(addr)).then_some(addr)
}

fn terminal_import(
    pc: u32,
    state: State,
    stubs: &BTreeMap<u32, u32>,
    slots: &BTreeMap<u64, Vec<u8>>,
) -> bool {
    slot(pc, state, stubs, slots)
        .and_then(|addr| slots.get(&u64::from(addr)))
        .and_then(|name| std::str::from_utf8(name).ok())
        .is_some_and(kuna_decomp::kuna_noreturnextern::matches_noreturn_extern_name)
}

/// One symbol-bounded caller. Its entry register values are unknown, joins keep
/// only equal constants, and only relocation-proven import calls preserve r30.
fn caller(
    start: u32,
    code: &[u32],
    stubs: &BTreeMap<u32, u32>,
    slots: &BTreeMap<u64, Vec<u8>>,
    absolute: &BTreeSet<u32>,
) -> Option<Vec<Option<State>>> {
    if code.is_empty() || code.len() > 1_000_000 {
        return None;
    }
    let mut states = vec![None; code.len()];
    let mut queue = VecDeque::from([0]);
    states[0] = Some(State::entry());
    let mut steps = 0;
    while let Some(i) = queue.pop_front() {
        steps += 1;
        if steps > code.len().saturating_mul(8) {
            return None;
        }
        let pc = start + i as u32 * 4;
        let word = code[i];
        let mut state = states[i]?;
        let mut targets = [None, None];
        let mut terminal = None;
        if let Some(b) = branch(pc, word) {
            if terminal_import(b.target, state, stubs, slots) {
                terminal = Some(b.target);
            }
            if b.link && b.target == pc + 4 {
                state.lr = Value::Constant(pc + 4);
                targets[0] = Some(pc + 4);
            } else if b.link {
                if absolute.contains(&b.target) || slot(b.target, state, stubs, slots).is_some() {
                    state.after_import();
                } else {
                    state = State::default();
                }
                if terminal.is_none() || b.conditional {
                    targets[0] = Some(pc + 4);
                }
            } else {
                targets[0] = Some(b.target);
                if b.conditional {
                    targets[1] = Some(pc + 4);
                }
            }
        } else if word >> 26 == 19 && (word >> 1) & 1023 == 16 {
            if state.lr != Value::Return || word & 1 != 0 {
                return None;
            }
            if (word >> 21) & 20 != 20 {
                targets[0] = Some(pc + 4);
            }
        } else if word >> 26 == 19 || word >> 26 == 17 {
            return None;
        } else {
            state = transfer(word, state);
            targets[0] = Some(pc + 4);
        }
        for target in targets.into_iter().flatten() {
            if !(start..start + code.len() as u32 * 4).contains(&target)
                && state.lr != Value::Return
                && terminal != Some(target)
            {
                return None;
            }
            let Some(offset) = target.checked_sub(start) else {
                continue;
            };
            let next = offset as usize / 4;
            if offset & 3 != 0 || next >= code.len() {
                continue;
            }
            let merged = states[next].map_or(state, |previous| previous.meet(state));
            if states[next] != Some(merged) {
                states[next] = Some(merged);
                queue.push_back(next);
            }
        }
    }
    Some(states)
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
        if code.iter().any(|&(pc, word)| {
            branch(pc, word)
                .is_some_and(|b| b.target > start && b.target <= start + 52 && pc != start + 8)
        }) {
            continue;
        }
        exits.insert(start + 52);
    }
    exits
}

pub(super) fn resolve(
    file: &object::File<'_>,
    slots: &BTreeMap<u64, Vec<u8>>,
    imports: &[PltSym],
) -> Vec<PltSym> {
    let mut code = Vec::new();
    let mut stubs = BTreeMap::new();
    for section in file.sections() {
        if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6)
            || section.address() & 3 != 0
            || section.address().saturating_add(section.size()) > u32::MAX as u64
        {
            continue;
        }
        let Ok(bytes) = section.data() else { continue };
        let words: Vec<_> = bytes
            .chunks_exact(4)
            .map(|b| super::kuna_ppc32_imports::word(file, b))
            .collect();
        for (i, &word) in words.iter().enumerate() {
            let pc = section.address() as u32 + i as u32 * 4;
            code.push((pc, word));
        }
        for (i, words) in words.windows(4).enumerate() {
            if let Some(delta) = pic_delta(words) {
                stubs.insert(section.address() as u32 + i as u32 * 4, delta);
            }
        }
    }
    if stubs.is_empty() {
        return Vec::new();
    }
    code.sort_unstable_by_key(|&(pc, _)| pc);
    if code.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Vec::new();
    }
    let word_at = |pc: u32| {
        code.binary_search_by_key(&pc, |&(addr, _)| addr)
            .ok()
            .map(|i| code[i].1)
    };
    let mut functions: BTreeMap<u32, u32> = BTreeMap::new();
    let mut blocked = BTreeSet::new();
    let mut tainted = BTreeSet::new();
    let mut pointers = BTreeSet::new();
    let mut entries = BTreeSet::new();
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if symbol.is_undefined() {
            continue;
        }
        let addr = symbol.address();
        if addr > u32::MAX as u64 {
            continue;
        }
        entries.insert(addr as u32);
        if let Some((&stub, _)) = stubs.range(..=addr as u32).next_back() {
            if addr < u64::from(stub) + 16 && (addr != u64::from(stub) || symbol.is_global()) {
                blocked.insert(stub);
            }
        }
        if symbol.kind() == SymbolKind::Text
            && symbol.size() != 0
            && addr & 3 == 0
            && symbol.size() & 3 == 0
            && addr.saturating_add(symbol.size()) <= u32::MAX as u64
        {
            let end = (addr + symbol.size()) as u32;
            functions
                .entry(addr as u32)
                .and_modify(|old| {
                    if *old != end {
                        tainted.insert(addr as u32);
                        *old = (*old).max(end);
                    }
                })
                .or_insert(end);
        }
    }
    if let Ok(entry) = u32::try_from(file.entry()) {
        entries.insert(entry);
        if let Some((&stub, _)) = stubs.range(..=entry).next_back() {
            if entry < stub + 16 {
                blocked.insert(stub);
            }
        }
    }
    for section in file.sections() {
        if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 2)
            || !matches!(
                section.kind(),
                SectionKind::Data | SectionKind::ReadOnlyData | SectionKind::Tls
            )
        {
            continue;
        }
        if let Ok(bytes) = section.data() {
            for word in bytes.chunks_exact(4) {
                let addr = super::kuna_ppc32_imports::word(file, word);
                pointers.insert(addr);
                if let Some((&stub, _)) = stubs.range(..=addr).next_back() {
                    if addr < stub + 16 {
                        blocked.insert(stub);
                    }
                }
            }
        }
    }
    let mut sites: BTreeMap<u32, Vec<(u32, bool)>> = BTreeMap::new();
    let mut invalidate = |address: u64| {
        let Ok(address) = u32::try_from(address) else {
            return;
        };
        pointers.insert(address);
        if let Some((&stub, _)) = stubs.range(..=address).next_back() {
            if address < stub + 16 {
                blocked.insert(stub);
            }
        }
        if let Some((&start, &end)) = functions.range(..=address).next_back() {
            if address < end {
                tainted.insert(start);
            }
        }
    };
    let dynamic_addresses: BTreeMap<_, _> = file
        .dynamic_symbols()
        .filter(|symbol| !symbol.is_undefined())
        .map(|symbol| (symbol.index().0, symbol.address()))
        .collect();
    if let Some(relocations) = file.dynamic_relocations() {
        for (address, relocation) in relocations {
            let width = u64::from(relocation.size()).div_ceil(8).max(4);
            let first = code.partition_point(|&(pc, _)| u64::from(pc) + 4 <= address);
            if code
                .get(first)
                .is_some_and(|&(pc, _)| u64::from(pc) < address.saturating_add(width))
            {
                // Runtime writes can change both callee clobbers and the edge census.
                return Vec::new();
            }
            invalidate(address);
            invalidate(address.saturating_add(width - 1));
            if let object::RelocationTarget::Symbol(index) = relocation.target() {
                if let Some(&value) = dynamic_addresses.get(&index.0) {
                    invalidate((value as u32).wrapping_add(relocation.addend() as u32) as u64);
                }
            }
            if matches!(
                relocation.flags(),
                object::RelocationFlags::Elf {
                    r_type: object::elf::R_PPC_RELATIVE
                }
            ) {
                invalidate(relocation.addend() as u32 as u64);
            }
        }
    }
    for &address in &pointers {
        if let Some((&start, &end)) = functions.range(..=address).next_back() {
            if address > start && address < end {
                tainted.insert(start);
            }
        }
    }
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if symbol.is_undefined() || !(symbol.is_global() || symbol.kind() == SymbolKind::Text) {
            continue;
        }
        if let Some((&start, &end)) = functions.range(..=symbol.address() as u32).next_back() {
            if start < symbol.address() as u32 && symbol.address() < u64::from(end) {
                tainted.insert(start);
            }
        }
    }
    if let Some((&start, &end)) = functions.range(..=file.entry() as u32).next_back() {
        if start < file.entry() as u32 && file.entry() < u64::from(end) {
            tainted.insert(start);
        }
    }
    let mut branch_targets = BTreeSet::new();
    for &(pc, word) in &code {
        let Some(b) = branch(pc, word) else { continue };
        if b.link && b.target == pc + 4 {
            continue;
        }
        branch_targets.insert(b.target);
        if stubs.contains_key(&b.target) {
            sites.entry(b.target).or_default().push((pc, false));
        } else if let Some((&stub, _)) = stubs.range(..=b.target).next_back() {
            if b.target < stub + 16 {
                blocked.insert(stub);
            }
        }
        if let Some((&start, &end)) = functions.range(..=b.target).next_back() {
            if b.target > start && b.target < end && !(start..end).contains(&pc) {
                tainted.insert(start);
            }
        }
    }
    for &stub in stubs.keys() {
        let mut pc = stub.wrapping_sub(4);
        let mut padding = 0;
        while word_at(pc) == Some(0x60000000) && padding < 16 {
            if branch_targets.contains(&pc) || pointers.contains(&pc) || entries.contains(&pc) {
                blocked.insert(stub);
            }
            pc = pc.wrapping_sub(4);
            padding += 1;
        }
        let Some(word) = word_at(pc) else { continue };
        let terminal = branch(pc, word).is_some_and(|b| !b.link && !b.conditional)
            || matches!(word, 0x4e800020 | 0x4e800420);
        if !terminal {
            sites.entry(stub).or_default().push((pc, true));
        }
    }
    if !blocked.is_empty() {
        return Vec::new();
    }
    let absolute: BTreeSet<_> = imports.iter().map(|s| s.addr as u32).collect();
    let mut trusted_indirect = resolver_exit(file, &code);
    trusted_indirect.retain(|&exit| {
        pointers.range((exit - 52 + 1)..=exit).next().is_none()
            && entries.range((exit - 52 + 1)..=exit).next().is_none()
    });
    let resolver_starts: BTreeSet<_> = trusted_indirect.iter().map(|&exit| exit - 52).collect();
    let mut covered: BTreeSet<_> = resolver_starts
        .iter()
        .flat_map(|&start| (start..start + 56).step_by(4))
        .collect();
    for &stub in stubs.keys().chain(absolute.iter()) {
        if word_at(stub).is_none() {
            continue;
        }
        covered.extend((stub..stub + 16).step_by(4));
        let exit = if word_at(stub + 8) == Some(0x4e800420) {
            stub + 8
        } else {
            stub + 12
        };
        trusted_indirect.insert(exit);
    }
    let mut evidence = BTreeMap::new();
    let required: BTreeSet<_> = sites.values().flatten().map(|&(pc, _)| pc).collect();
    let ranges: Vec<_> = functions.into_iter().collect();
    let mut prior_end = 0;
    for (index, &(start, end)) in ranges.iter().enumerate() {
        let overlaps_prior = start < prior_end;
        prior_end = prior_end.max(end);
        if end <= start
            || tainted.contains(&start)
            || overlaps_prior
            || ranges.get(index + 1).is_some_and(|&(next, _)| next < end)
        {
            return Vec::new();
        }

        if stubs.contains_key(&start)
            || absolute.contains(&start)
            || resolver_starts.contains(&start)
        {
            continue;
        }
        let body: Option<Vec<_>> = (start..end).step_by(4).map(|pc| word_at(pc)).collect();
        let Some(body) = body else { return Vec::new() };
        let Some(states) = caller(start, &body, &stubs, slots, &absolute) else {
            return Vec::new();
        };
        covered.extend((start..end).step_by(4));
        for (i, state) in states.into_iter().enumerate() {
            let pc = start + i as u32 * 4;
            if word_at(pc).is_some_and(|w| w >> 26 == 19 && matches!((w >> 1) & 1023, 16 | 528)) {
                if state.is_none()
                    || state.is_some_and(|s| s.lr == Value::Return)
                        && word_at(pc).is_some_and(|w| (w >> 1) & 1023 == 16 && w & 1 == 0)
                {
                    trusted_indirect.insert(pc);
                }
            }
            if required.contains(&pc) {
                if let Some(state) = state {
                    evidence.insert(pc, state);
                }
            }
        }
    }
    if code.iter().any(|&(pc, word)| {
        (word >> 26 == 19
            && matches!((word >> 1) & 1023, 16 | 528)
            && !trusted_indirect.contains(&pc))
            || (!covered.contains(&pc)
                && (word & 0xfc1f_ffff == 0x7c08_03a6 || branch(pc, word).is_some_and(|b| b.link)))
    }) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (stub, callers) in sites {
        if blocked.contains(&stub) {
            continue;
        }
        let mut resolved = None;
        let consistent = callers.into_iter().all(|(pc, fallthrough)| {
            let state = evidence.get(&pc).copied().map(|mut state| {
                if fallthrough {
                    let word = word_at(pc).unwrap();
                    if let Some(b) = branch(pc, word) {
                        if b.link
                            && !(absolute.contains(&b.target)
                                || slot(b.target, state, &stubs, slots).is_some())
                        {
                            state = State::default();
                        }
                    } else {
                        state = transfer(word, state);
                    }
                }
                state
            });
            let found = state.and_then(|state| slot(stub, state, &stubs, slots));
            if found.is_none() || resolved.is_some() && found != resolved {
                return false;
            }
            resolved = found;
            true
        });
        if consistent {
            if let Some(addr) = resolved {
                out.push(PltSym {
                    addr: u64::from(stub),
                    name: slots[&u64::from(addr)].clone(),
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_call(words: &[u32], at: usize) -> Option<u32> {
        let stubs = BTreeMap::from([(0x1800, 0x800)]);
        let slots = BTreeMap::from([(0x3000, b"first".to_vec()), (0x3004, b"second".to_vec())]);
        let mut body = words.to_vec();
        body.push(0x48000000); // Keep partial instruction probes inside their extent.
        caller(0x1000, &body, &stubs, &slots, &BTreeSet::new())?[at]?.regs[30].constant()
    }

    #[test]
    fn caller_joins_and_clobbers() {
        // li r30,0x2800; cmpwi; beq join; nop/li/lwz; join: bl stub.
        for (word, expected) in [
            (0x60000000, Some(0x2800)),
            (0x3bc02800, Some(0x2800)),
            (0x3bc02804, None),
            (0x83c10008, None),
            (0xbb810000, None), // lmw r28,0(r1) also writes r30.
            (0x97de0004, None), // stwu r30,4(r30) updates the base.
            (0x7fde2214, None), // add r30,r30,r4: unsupported, never retained.
        ] {
            assert_eq!(
                at_call(&[0x3bc02800, 0x2c030000, 0x41820008, word, 0x480007f1], 4),
                expected
            );
        }
    }

    #[test]
    fn pc_capture_and_signed_high_low() {
        for (high, low, expected) in [(0, 0x17fc, 0x2800), (2, 0xeffc, 0x20000)] {
            let code = [
                0x429f0005,
                0x7fc802a6,
                0x3fde0000 | high,
                0x3bde0000 | low,
                0x480007f1,
            ];
            assert_eq!(at_call(&code, 4), Some(expected));
        }
        assert_eq!(at_call(&[0x7fc802a6, 0x3bde2800, 0x480007f9], 2), None);
    }

    #[test]
    fn only_proven_import_calls_preserve_the_base() {
        let code = [0x3bc02800, 0x480007fd, 0x480007f9];
        assert_eq!(at_call(&code, 2), Some(0x2800));
        let code = [0x3bc02800, 0x48000ffd, 0x480007f9];
        assert_eq!(at_call(&code, 2), None);
        let code = [0x3bc027fc, 0x480007fd, 0x480007f9];
        assert_eq!(at_call(&code, 2), None);
    }

    #[test]
    fn back_edges_reach_a_fixed_point() {
        let code = [0x3bc02800, 0x480007fd, 0x3bde0004, 0x4082fff8, 0x480007f1];
        assert_eq!(at_call(&code, 1), None);
        assert_eq!(at_call(&code, 4), None);
        let code = [0x3bc02800, 0x480007fd, 0x60000000, 0x4082fff8, 0x480007f1];
        assert_eq!(at_call(&code, 4), Some(0x2800));
    }

    #[test]
    fn unknown_indirect_jump_rejects_the_entire_caller() {
        let code = [0x3bc02800, 0x480007fd, 0x4e800420];
        assert_eq!(at_call(&code, 1), None);
        let captured_lr = [0x429f0005, 0x7fc802a6, 0x3bde17fc, 0x4e800020];
        assert_eq!(at_call(&captured_lr, 2), None);
    }
    #[test]
    fn only_the_saved_entry_lr_proves_a_return() {
        let good = [
            0x7c0802a6, 0x9421fff0, 0x90010014, 0x3bc02800, 0x480007f1, 0x80010014, 0x38210010,
            0x7c0803a6, 0x4e800020,
        ];
        assert_eq!(at_call(&good, 4), Some(0x2800));
        let mut computed = good;
        computed[0] = 0x38001008;
        assert_eq!(at_call(&computed, 4), None);
        let mut clobbered = good;
        clobbered[5] = 0x7c002214;
        assert_eq!(at_call(&clobbered, 4), None);
        let mut unknown = good;
        unknown[5] = 0x80010018;
        assert_eq!(at_call(&unknown, 4), None);
        assert_eq!(at_call(&[0x3bc02800, 0x480007fd, 0x4e800020], 1), None);
        assert!(caller(
            0x1000,
            &[0x38001008, 0x7c0803a6],
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeSet::new(),
        )
        .is_none());
    }

    #[test]
    fn a_conflicting_computed_lr_join_is_not_a_return() {
        let code = [
            0x3bc02800, 0x2c030000, 0x41820008, 0x480007f5, 0x2c040000, 0x41820014, 0x3bc0100c,
            0x7fc803a6, 0x48000014, 0x60000000, 0x3bc01008, 0x7fc803a6, 0x60000000, 0x3bc02804,
            0x4e800020,
        ];
        assert_eq!(at_call(&code, 3), None);
    }

    #[test]
    fn overwriting_or_exposing_the_saved_lr_discards_return_evidence() {
        let prefix = [0x7c0802a6, 0x9421fff0, 0x90010014];
        for (write, valid) in [(0x90610008, true), (0x98610015, false), (0x90640000, false)] {
            let mut code = prefix.to_vec();
            code.extend([
                write, 0x3bc02800, 0x480007ed, 0x80010014, 0x7c0803a6, 0x4e800020,
            ]);
            assert_eq!(at_call(&code, 5), valid.then_some(0x2800));
        }
        let mut exposed = prefix.to_vec();
        exposed.extend([
            0x38610014, 0x3bc02800, 0x480007ed, 0x80010014, 0x7c0803a6, 0x4e800020,
        ]);
        assert_eq!(at_call(&exposed, 5), None);
        // A callee can overwrite its incoming linkage area and unallocated stack.
        for (frame, spill) in [(0x60000000, 4), (0x9421fff0, 4), (0x9421fff0, 0xfffc)] {
            let code = [
                0x7c0802a6,
                frame,
                0x90010000 | spill,
                0x3bc02800,
                0x480007f1,
                0x80010000 | spill,
                0x7c0803a6,
                0x4e800020,
            ];
            assert_eq!(at_call(&code, 4), None);
        }
    }
}
