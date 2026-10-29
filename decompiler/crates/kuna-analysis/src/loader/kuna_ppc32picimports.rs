//! Resolve secure-PLT PIC entries from bounded caller control flow. A stub needs
//! the same validated slot at every direct incoming edge; table metadata and
//! linker labels never supply a register value. Callers without a disjoint text
//! symbol extent, unknown transfers, and address-taken stubs are refused.
use super::elf_plt::PltSym;
use object::{Object, ObjectSection, ObjectSymbol, SectionFlags, SectionKind, SymbolKind};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct State {
    r30: Option<u32>,
    lr: Option<u32>,
}

impl State {
    fn meet(self, other: Self) -> Self {
        Self {
            r30: self.r30.filter(|v| Some(*v) == other.r30),
            lr: self.lr.filter(|v| Some(*v) == other.lr),
        }
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

/// Interpret only instructions whose effects on r30 and LR are known. Other
/// arithmetic conservatively kills both facts; indirect jumps reject the caller.
fn transfer(word: u32, mut state: State) -> State {
    let rt = (word >> 21) & 31;
    let ra = (word >> 16) & 31;
    match word >> 26 {
        14 | 15 if rt == 30 => {
            let base = match ra {
                0 => Some(0),
                30 => state.r30,
                _ => None,
            };
            let delta = if word >> 26 == 15 {
                signed(word) << 16
            } else {
                signed(word)
            };
            state.r30 = base.map(|v| v.wrapping_add(delta));
        }
        14 | 15 | 10 | 11 | 36 | 38 | 44 | 47 | 48 | 50 | 52 | 54 => {}
        24 | 25 if ra == 30 => {
            let mask = if word >> 26 == 25 {
                word << 16
            } else {
                word & 0xffff
            };
            state.r30 = if rt == 30 {
                state.r30.map(|v| v | mask)
            } else {
                None
            };
        }
        20..=29 => {
            if ra == 30 {
                state.r30 = None;
            }
        }
        32 | 34 | 40 | 42 => {
            if rt == 30 {
                state.r30 = None;
            }
        }
        33 | 35 | 41 | 43 => {
            if rt == 30 || ra == 30 {
                state.r30 = None;
            }
        }
        37 | 39 | 45 | 49 | 51 | 53 | 55 => {
            if ra == 30 {
                state.r30 = None;
            }
        }
        46 => {
            if rt <= 30 {
                state.r30 = None;
            }
        }
        31 if word & 0xfc1f_ffff == 0x7c08_02a6 => {
            if rt == 30 {
                state.r30 = state.lr;
            }
        }
        31 if word & 0xfc1f_ffff == 0x7c08_03a6 => {
            state.lr = if rt == 30 { state.r30 } else { None };
        }
        31 if word & 0xfc1f_ffff == 0x7c09_03a6 => {}
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
    let addr = state.r30?.wrapping_add(*stubs.get(&pc)?);
    slots.contains_key(&u64::from(addr)).then_some(addr)
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
    states[0] = Some(State::default());
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
        if let Some(b) = branch(pc, word) {
            if b.link && b.target == pc + 4 {
                state.lr = Some(pc + 4);
                targets[0] = Some(pc + 4);
            } else if b.link {
                state = State {
                    r30: if absolute.contains(&b.target)
                        || slot(b.target, state, stubs, slots).is_some()
                    {
                        state.r30
                    } else {
                        None
                    },
                    lr: None,
                };
                targets[0] = Some(pc + 4);
            } else {
                targets[0] = Some(b.target);
                if b.conditional {
                    targets[1] = Some(pc + 4);
                }
            }
        } else if word >> 26 == 19 && (word >> 1) & 1023 == 16 {
            // A known LR is a computed jump, not the ABI return address.
            if state.lr.is_some() {
                return None;
            }
            if word & 1 != 0 {
                state = State::default();
                targets[0] = Some(pc + 4);
            } else if (word >> 21) & 20 != 20 {
                targets[0] = Some(pc + 4);
            }
        } else if word >> 26 == 19 || word >> 26 == 17 {
            return None;
        } else {
            state = transfer(word, state);
            targets[0] = Some(pc + 4);
        }
        for target in targets.into_iter().flatten() {
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
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if symbol.is_undefined() {
            continue;
        }
        let addr = symbol.address();
        if addr > u32::MAX as u64 {
            continue;
        }
        if stubs.contains_key(&(addr as u32)) && symbol.is_global() {
            blocked.insert(addr as u32);
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
        blocked.insert(entry);
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
            invalidate(address);
            let width = u64::from(relocation.size()).div_ceil(8).max(4);
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
    for &(pc, word) in &code {
        let Some(b) = branch(pc, word) else { continue };
        if b.link && b.target == pc + 4 {
            continue;
        }
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
    let absolute: BTreeSet<_> = imports.iter().map(|s| s.addr as u32).collect();
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
            continue;
        }
        if required.range(start..end).next().is_none() {
            continue;
        }
        let body: Option<Vec<_>> = (start..end).step_by(4).map(|pc| word_at(pc)).collect();
        let Some(body) = body else { continue };
        let Some(states) = caller(start, &body, &stubs, slots, &absolute) else {
            continue;
        };
        for (i, state) in states.into_iter().enumerate() {
            let pc = start + i as u32 * 4;
            if required.contains(&pc) {
                if let Some(state) = state {
                    evidence.insert(pc, state);
                }
            }
        }
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
        caller(0x1000, words, &stubs, &slots, &BTreeSet::new())?[at]?.r30
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
}
