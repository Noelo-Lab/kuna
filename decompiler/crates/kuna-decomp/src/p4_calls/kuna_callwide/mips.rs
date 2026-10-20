//! Bounded MIPS32-BE direct-call closure with private-frame return-token proof.
//! Object, mode, relocation and effective-byte admission is the frontend's job.
//! The whitelist follows `mips32Instructions.sinc` in legacy delay-slot mode.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::CodeRegion;

const MAX_BYTES: usize = 65_536;
const MAX_WORDS: usize = MAX_BYTES / 4;
const MAX_FUNCTION_WORDS: u64 = 512;
const SP: usize = 29;
const FP: usize = 30;
const RA: usize = 31;
const SAVED: [usize; 9] = [16, 17, 18, 19, 20, 21, 22, 23, 28];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Unknown,
    Zero,
    EntryRa,
    EntryFp,
    EntrySaved(usize),
    Link(u64),
    Stack(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Insn {
    Move(usize, usize),
    Addiu(usize, usize, i16),
    Numeric(usize),
    Load(usize, usize, i16),
    Store(usize, usize, i16),
    Call(u64),
    Return,
}

fn decode(word: u32, pc: u64) -> Option<Insn> {
    let rs = ((word >> 21) & 31) as usize;
    let rt = ((word >> 16) & 31) as usize;
    let rd = ((word >> 11) & 31) as usize;
    let shift = (word >> 6) & 31;
    let imm = word as i16;
    Some(match word >> 26 {
        0 => match word & 63 {
            0 if word == 0x0000_0140 => return None,
            0 | 2 | 3 if rs == 0 => {
                if shift == 0 {
                    Insn::Move(rd, rt)
                } else {
                    Insn::Numeric(rd)
                }
            }
            4 | 6 | 7 if shift == 0 => {
                if rs == 0 {
                    Insn::Move(rd, rt)
                } else {
                    Insn::Numeric(rd)
                }
            }
            0x21 | 0x25 | 0x26 if shift == 0 && rt == 0 => Insn::Move(rd, rs),
            0x21 | 0x25 | 0x26 if shift == 0 && rs == 0 => Insn::Move(rd, rt),
            0x23 if shift == 0 && rt == 0 => Insn::Move(rd, rs),
            0x21 | 0x23..=0x27 | 0x2a | 0x2b if shift == 0 => Insn::Numeric(rd),
            8 if word == 0x03e0_0008 => Insn::Return,
            _ => return None,
        },
        3 => Insn::Call(
            ((pc.checked_add(4)? & 0xf000_0000) | (u64::from(word & 0x03ff_ffff) << 2))
                & 0xffff_ffff,
        ),
        9 => Insn::Addiu(rt, rs, imm),
        0x0a..=0x0e => Insn::Numeric(rt),
        0x0f if rs == 0 => Insn::Numeric(rt),
        0x23 => Insn::Load(rt, rs, imm),
        0x2b => Insn::Store(rt, rs, imm),
        _ => return None,
    })
}

fn ordinary(insn: Insn) -> bool {
    !matches!(insn, Insn::Call(_) | Insn::Return)
}

struct Frame {
    regs: [Value; 32],
    base: Option<i32>,
    cells: BTreeMap<i32, Value>,
    saved_ra: Option<i32>,
}

impl Frame {
    fn new() -> Self {
        let mut regs = [Value::Unknown; 32];
        regs[0] = Value::Zero;
        regs[SP] = Value::Stack(0);
        regs[FP] = Value::EntryFp;
        regs[RA] = Value::EntryRa;
        for reg in SAVED {
            regs[reg] = Value::EntrySaved(reg);
        }
        Self {
            regs,
            base: None,
            cells: BTreeMap::new(),
            saved_ra: None,
        }
    }

    fn write(&mut self, dst: usize, value: Value, restore_ra: bool) -> Option<()> {
        if dst == 0 {
            return Some(());
        }
        if matches!(
            value,
            Value::EntryRa | Value::EntryFp | Value::Link(_) | Value::Stack(_)
        ) && !matches!(dst, SP | FP | RA)
        {
            return None;
        }
        if dst == RA && !(restore_ra && value == Value::EntryRa) {
            return None;
        }
        if dst == SP {
            let Value::Stack(next) = value else {
                return None;
            };
            if let Some(base) = self.base {
                if next != base && next != 0 {
                    return None;
                }
            } else if self.regs[SP] == Value::Stack(0) && next < 0 && next % 8 == 0 {
                self.base = Some(next);
            } else if next != 0 {
                return None;
            }
        }
        self.regs[dst] = value;
        Some(())
    }

    fn cell(&self, base: usize, off: i16) -> Option<i32> {
        let frame = self.base?;
        if self.regs[SP] != Value::Stack(frame) {
            return None;
        }
        let Value::Stack(at) = self.regs[base] else {
            return None;
        };
        let at = at.checked_add(i32::from(off))?;
        (at >= frame && at.checked_add(4)? <= 0 && at % 4 == 0).then_some(at)
    }

    fn step(&mut self, insn: Insn) -> Option<()> {
        match insn {
            Insn::Move(dst, src) => self.write(dst, self.regs[src], false),
            Insn::Addiu(dst, src, imm) => {
                let value = match self.regs[src] {
                    Value::Stack(at) => Value::Stack(at.checked_add(i32::from(imm))?),
                    value if imm == 0 => value,
                    _ => Value::Unknown,
                };
                self.write(dst, value, false)
            }
            Insn::Numeric(dst) => self.write(dst, Value::Unknown, false),
            Insn::Load(dst, base, off) => {
                let cell = self.cell(base, off)?;
                let value = *self.cells.get(&cell)?;
                if dst == RA && Some(cell) != self.saved_ra {
                    return None;
                }
                if matches!(
                    value,
                    Value::EntryRa | Value::EntryFp | Value::Link(_) | Value::Stack(_)
                ) && !matches!(dst, SP | FP | RA)
                {
                    return None;
                }
                self.write(dst, value, dst == RA)
            }
            Insn::Store(src, base, off) => {
                let at = self.cell(base, off)?;
                if self.saved_ra == Some(at) {
                    return None;
                }
                let value = self.regs[src];
                if value == Value::EntryRa {
                    if self.saved_ra.is_some() {
                        return None;
                    }
                    self.saved_ra = Some(at);
                }
                self.cells.insert(at, value);
                Some(())
            }
            Insn::Call(_) | Insn::Return => None,
        }
    }

    fn call(&mut self, link: u64) -> Option<()> {
        if self.regs[SP] != Value::Stack(self.base.unwrap_or(0))
            || self.regs[RA] != Value::Link(link)
        {
            return None;
        }
        let before = self.regs;
        self.regs = [Value::Unknown; 32];
        self.regs[0] = Value::Zero;
        self.regs[SP] = before[SP];
        self.regs[FP] = before[FP];
        for reg in SAVED {
            self.regs[reg] = before[reg];
        }
        self.regs[RA] = Value::Link(link);
        Some(())
    }
}

fn layout(
    start: u64,
    end: u64,
    words: &BTreeMap<u64, u32>,
    entries: &BTreeMap<u64, u64>,
    calls: &mut BTreeSet<(u64, u64)>,
) -> Option<BTreeSet<u64>> {
    let mut at = start;
    let mut returned = false;
    let mut targets = BTreeSet::new();
    while at < end {
        let word = *words.get(&at)?;
        if returned {
            if word != 0 {
                return None;
            }
            at += 4;
            continue;
        }
        match decode(word, at)? {
            Insn::Call(target) => {
                if !entries.contains_key(&target) {
                    return None;
                }
                targets.insert(target);
                calls.insert((at, target));
            }
            Insn::Return => returned = true,
            _ => {
                at += 4;
                continue;
            }
        }
        let delay = at.checked_add(4)?;
        if delay >= end || !ordinary(decode(*words.get(&delay)?, delay)?) {
            return None;
        }
        at += 8;
    }
    returned.then_some(targets)
}

fn prove_frame(start: u64, end: u64, words: &BTreeMap<u64, u32>) -> Option<()> {
    let mut frame = Frame::new();
    let mut at = start;
    while at < end {
        match decode(*words.get(&at)?, at)? {
            Insn::Call(_) => {
                let link = at.checked_add(8)? & 0xffff_ffff;
                frame.regs[RA] = Value::Link(link);
                frame.step(decode(*words.get(&(at + 4))?, at + 4)?)?;
                frame.call(link)?;
                at += 8;
            }
            Insn::Return => {
                if frame.regs[RA] != Value::EntryRa {
                    return None;
                }
                frame.step(decode(*words.get(&(at + 4))?, at + 4)?)?;
                return (frame.regs[SP] == Value::Stack(0)
                    && frame.regs[FP] == Value::EntryFp
                    && SAVED
                        .iter()
                        .all(|&reg| frame.regs[reg] == Value::EntrySaved(reg)))
                .then_some(());
            }
            insn => {
                frame.step(insn)?;
                at += 4;
            }
        }
    }
    None
}

/// Prove executed transfer closure from one sealed process entry. Its exact
/// JAL/NOP/J-self/NOP stub never returns outside the image. Ordinary functions
/// retain only direct calls and proved ABI returns; integer values are not targets.
pub fn closed_image(
    entry: u64,
    regions: &[CodeRegion],
    entries: &[(u64, u64)],
    observed_calls: &BTreeSet<(u64, u64)>,
) -> Option<BTreeMap<u64, Vec<u64>>> {
    closed(Some(entry), regions, entries, observed_calls)
}

fn sealed_root(
    entry: u64,
    words: &BTreeMap<u64, u32>,
    functions: &BTreeMap<u64, u64>,
) -> Option<u64> {
    if functions.get(&entry).copied() != entry.checked_add(16) {
        return None;
    }
    let Insn::Call(target) = decode(*words.get(&entry)?, entry)? else {
        return None;
    };
    let spin = entry.checked_add(8)?;
    let jump = *words.get(&spin)?;
    let destination =
        ((spin.checked_add(4)? & 0xf000_0000) | (u64::from(jump & 0x03ff_ffff) << 2)) & 0xffff_ffff;
    if target == entry
        || !functions.contains_key(&target)
        || *words.get(&entry.checked_add(4)?)? != 0
        || jump >> 26 != 2
        || destination != spin
        || *words.get(&entry.checked_add(12)?)? != 0
    {
        return None;
    }
    Some(target)
}

fn closed(
    root: Option<u64>,
    regions: &[CodeRegion],
    entries: &[(u64, u64)],
    observed_calls: &BTreeSet<(u64, u64)>,
) -> Option<BTreeMap<u64, Vec<u64>>> {
    if regions.is_empty()
        || entries.is_empty()
        || regions.len() > MAX_WORDS
        || entries.len() > MAX_WORDS
        || observed_calls.len() > MAX_WORDS
    {
        return None;
    }
    let mut bytes = 0usize;
    let mut words = BTreeMap::new();
    for region in regions {
        bytes = bytes.checked_add(region.bytes.len())?;
        let end = region.address.checked_add(region.bytes.len() as u64)?;
        if bytes > MAX_BYTES
            || region.bytes.is_empty()
            || region.address % 4 != 0
            || region.bytes.len() % 4 != 0
            || end > 0x1_0000_0000
        {
            return None;
        }
        for (index, chunk) in region.bytes.chunks_exact(4).enumerate() {
            if words
                .insert(
                    region.address + index as u64 * 4,
                    u32::from_be_bytes(chunk.try_into().ok()?),
                )
                .is_some()
            {
                return None;
            }
        }
    }
    let mut functions = BTreeMap::new();
    let mut owned = BTreeSet::new();
    for &(start, size) in entries {
        let end = start.checked_add(size)?;
        if start % 4 != 0
            || size == 0
            || size % 4 != 0
            || size / 4 > MAX_FUNCTION_WORDS
            || end > 0x1_0000_0000
            || functions.insert(start, end).is_some()
        {
            return None;
        }
        for at in (start..end).step_by(4) {
            if !words.contains_key(&at) || !owned.insert(at) {
                return None;
            }
        }
    }
    if words
        .iter()
        .any(|(at, word)| !owned.contains(at) && *word != 0)
    {
        return None;
    }
    if let Some(entry) = root {
        let end = entry.checked_add(16)?;
        if !regions.iter().any(|region| {
            region.address <= entry
                && region
                    .address
                    .checked_add(region.bytes.len() as u64)
                    .is_some_and(|last| end <= last)
        }) {
            return None;
        }
    }
    let mut calls = BTreeSet::new();
    let mut dependencies = BTreeMap::new();
    let mut callers: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for (&start, &end) in &functions {
        let targets = if root == Some(start) {
            let target = sealed_root(start, &words, &functions)?;
            calls.insert((start, target));
            BTreeSet::from([target])
        } else {
            layout(start, end, &words, &functions, &mut calls)?
        };
        if targets.iter().any(|target| Some(*target) == root) {
            return None;
        }
        for &target in &targets {
            callers.entry(target).or_default().push(start);
        }
        dependencies.insert(start, targets.len());
    }
    if &calls != observed_calls {
        return None;
    }
    if root.is_some_and(|entry| !functions.contains_key(&entry)) {
        return None;
    }
    let mut ready: VecDeque<u64> = dependencies
        .iter()
        .filter_map(|(&entry, &n)| (n == 0).then_some(entry))
        .collect();
    let mut proved = 0usize;
    while let Some(entry) = ready.pop_front() {
        if Some(entry) != root {
            prove_frame(entry, functions[&entry], &words)?;
        }
        proved += 1;
        for caller in callers.get(&entry).into_iter().flatten() {
            let count = dependencies.get_mut(caller)?;
            *count = count.checked_sub(1)?;
            if *count == 0 {
                ready.push_back(*caller);
            }
        }
    }
    if proved != functions.len() {
        return None;
    }
    let mut sites: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for (at, target) in calls {
        sites.entry(target).or_default().push(at);
    }
    Some(sites)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed_functions(
        regions: &[CodeRegion],
        entries: &[(u64, u64)],
        observed_calls: &BTreeSet<(u64, u64)>,
    ) -> Option<BTreeMap<u64, Vec<u64>>> {
        closed(None, regions, entries, observed_calls)
    }

    const BASE: u64 = 0x100000;
    const JR: u32 = 0x03e0_0008;

    fn i(op: u32, rt: u32, rs: u32, imm: i16) -> u32 {
        op << 26 | rs << 21 | rt << 16 | u32::from(imm as u16)
    }

    fn mov(dst: u32, src: u32) -> u32 {
        src << 21 | dst << 11 | 0x25
    }

    fn jal(target: u64) -> u32 {
        3 << 26 | (target as u32 >> 2 & 0x03ff_ffff)
    }

    fn image(code: &[u32]) -> CodeRegion {
        CodeRegion {
            address: BASE,
            bytes: code.iter().flat_map(|word| word.to_be_bytes()).collect(),
        }
    }

    fn leaf() -> Vec<u32> {
        vec![0x00c4_0821, 0x00e5_1821, 0x0067_102b, JR, 0x0022_1021]
    }

    fn caller() -> Vec<u32> {
        vec![
            i(9, 29, 29, -24),
            i(0x2b, 31, 29, 20),
            i(0x2b, 30, 29, 16),
            mov(30, 29),
            jal(BASE),
            mov(4, 2),
            mov(29, 30),
            i(0x23, 30, 29, 16),
            i(0x23, 31, 29, 20),
            JR,
            i(9, 29, 29, 24),
        ]
    }

    fn pair(code: &[u32]) -> Option<BTreeMap<u64, Vec<u64>>> {
        let mut all = leaf();
        all.extend(code);
        closed_functions(
            &[image(&all)],
            &[(BASE, 20), (BASE + 20, code.len() as u64 * 4)],
            &BTreeSet::from([(BASE + 36, BASE)]),
        )
    }

    fn process(extra: &[u32]) -> (u64, CodeRegion, Vec<(u64, u64)>, BTreeSet<(u64, u64)>) {
        let mut code = leaf();
        code.extend(caller());
        let mut entries = vec![(BASE, 20), (BASE + 20, 44)];
        if !extra.is_empty() {
            entries.push((BASE + code.len() as u64 * 4, extra.len() as u64 * 4));
            code.extend(extra);
        }
        let root = BASE + code.len() as u64 * 4;
        code.extend([
            jal(BASE + 20),
            0,
            2 << 26 | ((root + 8) as u32 >> 2 & 0x03ff_ffff),
            0,
        ]);
        entries.push((root, 16));
        (
            root,
            image(&code),
            entries,
            BTreeSet::from([(BASE + 36, BASE), (root, BASE + 20)]),
        )
    }

    #[test]
    fn sealed_entry_calls_an_ordinary_coordinator_then_only_spins() {
        let (root, region, entries, refs) = process(&[]);
        let result = closed_image(root, &[region], &entries, &refs).unwrap();
        assert_eq!(result[&BASE], vec![BASE + 36]);
        assert_eq!(result[&(BASE + 20)], vec![root]);
        assert!(!result.contains_key(&root));
    }

    #[test]
    fn an_unknown_external_return_boundary_is_not_a_closed_process() {
        let (root, region, entries, refs) = process(&[]);
        for entry in [BASE, BASE + 20, root + 4, root + 8] {
            assert!(closed_image(entry, &[region.clone()], &entries, &refs).is_none());
        }
    }

    #[test]
    fn every_root_word_and_observed_edge_is_exact() {
        let (root, region, entries, refs) = process(&[]);
        for (slot, replacement) in [
            (0, jal(root)),
            (0, jal(root + 4)),
            (0, jal(BASE + 4)),
            (1, mov(2, 4)),
            (2, JR),
            (2, 2 << 26 | (BASE as u32 >> 2)),
            (3, i(9, 2, 0, 1)),
        ] {
            let mut altered = region.clone();
            let index = (root - BASE) as usize + slot * 4;
            altered.bytes[index..index + 4].copy_from_slice(&replacement.to_be_bytes());
            assert!(closed_image(root, &[altered], &entries, &refs).is_none());
        }
        let mut missing = refs.clone();
        missing.remove(&(root, BASE + 20));
        assert!(closed_image(root, &[region], &entries, &missing).is_none());
    }

    #[test]
    fn ordinary_functions_cannot_call_back_into_any_root_word() {
        let (root, region, entries, refs) = process(&[]);
        for target in [root, root + 4, root + 8, root + 12] {
            let mut altered = region.clone();
            altered.bytes[36..40].copy_from_slice(&jal(target).to_be_bytes());
            let mut changed = refs.clone();
            changed.remove(&(BASE + 36, BASE));
            changed.insert((BASE + 36, target));
            assert!(closed_image(root, &[altered], &entries, &changed).is_none());
        }
    }

    #[test]
    fn a_root_cannot_jump_across_the_pseudodirect_region_boundary() {
        let entry = 0x0fff_fff4;
        let mut code = leaf();
        code.extend(caller());
        let root = CodeRegion {
            address: entry,
            bytes: [
                jal(BASE + 20),
                0,
                2 << 26 | ((entry + 8) as u32 >> 2 & 0x03ff_ffff),
                0,
            ]
            .iter()
            .flat_map(|word| word.to_be_bytes())
            .collect(),
        };
        assert!(closed_image(
            entry,
            &[image(&code), root],
            &[(BASE, 20), (BASE + 20, 44), (entry, 16)],
            &BTreeSet::from([(BASE + 36, BASE), (entry, BASE + 20)])
        )
        .is_none());
    }

    #[test]
    fn all_root_bytes_must_come_from_one_immutable_region() {
        let (root, mut region, entries, refs) = process(&[]);
        let bytes = region.bytes.split_off((root - BASE) as usize);
        let first = CodeRegion {
            address: root,
            bytes: bytes[..8].to_vec(),
        };
        let second = CodeRegion {
            address: root + 8,
            bytes: bytes[8..].to_vec(),
        };
        assert!(closed_image(root, &[region, first, second], &entries, &refs).is_none());
    }

    #[test]
    fn numeric_code_addresses_do_not_create_executed_indirect_edges() {
        for code in [
            vec![0x3c02_0010, 0x3442_0000, JR, 0],
            vec![0x0080_4021, 0x0104_1023, 0x3842_0010, 0x0002_1400, JR, 0],
            vec![0x0004_102b, 0x0002_1500, JR, 0],
            vec![0x2c02_0001, 0x0002_1500, JR, 0],
        ] {
            let (root, region, entries, refs) = process(&code);
            assert!(closed_image(root, &[region.clone()], &entries, &refs).is_some());
            assert!(closed_image(BASE + 20, &[region], &entries, &refs).is_none());
        }
    }

    #[test]
    fn computed_leaf_and_return_delay_word_are_admitted() {
        assert_eq!(
            closed_functions(&[image(&leaf())], &[(BASE, 20)], &BTreeSet::new()),
            Some(BTreeMap::new())
        );
    }

    #[test]
    fn callee_saved_gprs_must_retain_their_entry_values() {
        for reg in SAVED {
            let mut code = leaf();
            code[0] = mov(reg as u32, 4);
            assert!(closed_functions(&[image(&code)], &[(BASE, 20)], &BTreeSet::new()).is_none());
        }
        let mut code = caller();
        code[2] = i(0x2b, 16, 29, 16);
        code[3] = mov(16, 4);
        code[6] = 0;
        code[7] = i(0x23, 16, 29, 16);
        assert!(pair(&code).is_some());
    }

    #[test]
    fn frame_control_tokens_do_not_become_scalar_values() {
        for source in [SP, FP, RA] {
            let code = [mov(2, source as u32), JR, 0];
            assert!(closed_functions(&[image(&code)], &[(BASE, 12)], &BTreeSet::new()).is_none());
        }
    }

    #[test]
    fn caller_restores_entry_ra_fp_and_sp_after_delay() {
        assert_eq!(
            pair(&caller()),
            Some(BTreeMap::from([(BASE, vec![BASE + 36])]))
        );
    }

    #[test]
    fn arbitrary_ra_moves_and_constants_are_not_returns() {
        for word in [
            mov(31, 4),
            mov(31, 31),
            i(9, 31, 0, 4),
            i(0x0f, 31, 0, 0x10),
        ] {
            assert!(
                closed_functions(&[image(&[word, JR, 0])], &[(BASE, 12)], &BTreeSet::new())
                    .is_none()
            );
        }
    }

    #[test]
    fn wrong_restore_cell_and_saved_token_clobbers_refuse() {
        let mut code = caller();
        code[8] = i(0x23, 31, 29, 12);
        assert!(pair(&code).is_none());
        for store in [
            i(0x2b, 2, 29, 20),
            i(0x2b, 2, 29, 18),
            i(0x2b, 2, 4, 0),
            i(0x2b, 2, 29, 24),
            i(0x29, 2, 29, 20),
        ] {
            let mut code = caller();
            code[5] = store;
            assert!(pair(&code).is_none());
        }
    }

    #[test]
    fn every_load_requires_an_initialized_private_frame_cell() {
        for base in [4, 30, 31] {
            let code = [i(0x23, 2, base, 0), JR, 0];
            assert!(closed_functions(&[image(&code)], &[(BASE, 12)], &BTreeSet::new()).is_none());
        }
        for offset in [-4, 0, 20, 24, 1000] {
            let mut code = caller();
            code[5] = i(0x23, 2, 29, offset);
            assert!(pair(&code).is_none());
        }
        let mut code = caller();
        code[2] = i(0x2b, 4, 29, 16);
        code[3] = 0;
        code[5] = i(0x23, 2, 29, 16);
        code[6] = 0;
        code[7] = 0;
        assert!(pair(&code).is_some());
    }

    #[test]
    fn call_link_cannot_replace_saved_entry_token() {
        let mut code = caller();
        code[1] = 0;
        code[5] = i(0x2b, 31, 29, 20);
        assert!(pair(&code).is_none());
    }

    #[test]
    fn call_delay_cannot_replace_the_new_link_with_the_entry_token() {
        let mut code = caller();
        code[5] = i(0x23, 31, 29, 20);
        assert!(pair(&code).is_none());
    }

    #[test]
    fn callee_cannot_enter_above_the_live_private_frame() {
        for raised_by in [8, 24] {
            let mut code = caller();
            code[5] = i(9, 29, 29, raised_by);
            assert!(pair(&code).is_none());
        }
    }

    #[test]
    fn opaque_pause_refuses_in_bodies_and_both_delay_kinds() {
        let mut code = leaf();
        code[0] = 0x0000_0140;
        assert!(closed_functions(&[image(&code)], &[(BASE, 20)], &BTreeSet::new()).is_none());
        let mut code = leaf();
        code[4] = 0x0000_0140;
        assert!(closed_functions(&[image(&code)], &[(BASE, 20)], &BTreeSet::new()).is_none());
        let mut code = caller();
        code[5] = 0x0000_0140;
        assert!(pair(&code).is_none());
    }

    #[test]
    fn sp_and_fp_restoration_are_proved_instead_of_assumed() {
        let mut direct_sp = caller();
        direct_sp[6] = 0;
        assert!(pair(&direct_sp).is_some());
        for slot in [7, 10] {
            let mut code = caller();
            code[slot] = if slot == 10 { i(9, 29, 29, 16) } else { 0 };
            assert!(pair(&code).is_none());
        }
        let mut code = caller();
        code[6] = mov(29, 4);
        assert!(pair(&code).is_none());
        let mut all = leaf();
        all[0] = mov(30, 4);
        all.extend(caller());
        assert!(closed_functions(
            &[image(&all)],
            &[(BASE, 20), (BASE + 20, 44)],
            &BTreeSet::from([(BASE + 36, BASE)])
        )
        .is_none());
    }

    #[test]
    fn indirect_opaque_branch_and_bad_reserved_bits_refuse() {
        for word in [
            0x0080_f809,
            0x0000_000c,
            0x1000_0000,
            0x0804_0000,
            0x7fff_ffff,
            0x03e0_0048,
        ] {
            assert!(
                closed_functions(&[image(&[word, JR, 0])], &[(BASE, 12)], &BTreeSet::new())
                    .is_none()
            );
        }
    }

    #[test]
    fn incomplete_and_nested_delay_bundles_refuse() {
        assert!(closed_functions(&[image(&[JR])], &[(BASE, 4)], &BTreeSet::new()).is_none());
        assert!(closed_functions(
            &[image(&[JR, jal(BASE)])],
            &[(BASE, 8)],
            &BTreeSet::from([(BASE + 4, BASE)])
        )
        .is_none());
        let code = [JR, 0, jal(BASE)];
        assert!(closed_functions(
            &[image(&code)],
            &[(BASE, 8), (BASE + 8, 4)],
            &BTreeSet::from([(BASE + 8, BASE)])
        )
        .is_none());
    }

    #[test]
    fn all_executable_words_and_reference_identities_are_required() {
        let mut code = leaf();
        code.extend([0, 0]);
        assert!(closed_functions(&[image(&code)], &[(BASE, 20)], &BTreeSet::new()).is_some());
        code[6] = mov(2, 4);
        assert!(closed_functions(&[image(&code)], &[(BASE, 20)], &BTreeSet::new()).is_none());
        let mut all = leaf();
        all.extend(caller());
        for refs in [
            BTreeSet::new(),
            BTreeSet::from([(BASE + 40, BASE)]),
            BTreeSet::from([(BASE + 36, BASE + 4)]),
        ] {
            assert!(
                closed_functions(&[image(&all)], &[(BASE, 20), (BASE + 20, 44)], &refs).is_none()
            );
        }
    }

    #[test]
    fn recursive_calls_and_unknown_direct_targets_refuse() {
        for target in [BASE + 20, BASE + 4, BASE + 0x100] {
            let mut code = caller();
            code[4] = jal(target);
            let mut all = leaf();
            all.extend(&code);
            assert!(closed_functions(
                &[image(&all)],
                &[(BASE, 20), (BASE + 20, 44)],
                &BTreeSet::from([(BASE + 36, target)])
            )
            .is_none());
        }
    }

    #[test]
    fn region_entry_and_work_bounds_refuse_the_whole_image() {
        let r = image(&leaf());
        assert!(
            closed_functions(&[r.clone(), r.clone()], &[(BASE, 20)], &BTreeSet::new()).is_none()
        );
        for entries in [
            vec![(BASE, 20), (BASE + 4, 8)],
            vec![(BASE + 1, 16)],
            vec![(BASE, 24)],
            vec![(BASE, 0)],
        ] {
            assert!(closed_functions(&[r.clone()], &entries, &BTreeSet::new()).is_none());
        }
        let huge = CodeRegion {
            address: BASE,
            bytes: vec![0; MAX_BYTES + 4],
        };
        assert!(closed_functions(&[huge], &[(BASE, 20)], &BTreeSet::new()).is_none());
        let wrapped = CodeRegion {
            address: 0xffff_fffc,
            bytes: vec![0; 8],
        };
        assert!(closed_functions(&[wrapped], &[(0xffff_fffc, 8)], &BTreeSet::new()).is_none());
        let mut partial = r;
        partial.bytes.pop();
        assert!(closed_functions(&[partial], &[(BASE, 20)], &BTreeSet::new()).is_none());
    }

    #[test]
    fn more_than_thirty_two_or_two_hundred_fifty_six_sites_are_not_sampled() {
        let mut code = leaf();
        let mut c = vec![i(9, 29, 29, -24), i(0x2b, 31, 29, 20)];
        let mut refs = BTreeSet::new();
        for _ in 0..200 {
            refs.insert((BASE + (code.len() + c.len()) as u64 * 4, BASE));
            c.extend([jal(BASE), 0]);
        }
        c.extend([i(0x23, 31, 29, 20), JR, i(9, 29, 29, 24)]);
        let first_len = c.len();
        code.extend(c);
        let next = BASE + code.len() as u64 * 4;
        let mut second = vec![i(9, 29, 29, -24), i(0x2b, 31, 29, 20)];
        for _ in 0..100 {
            refs.insert((BASE + (code.len() + second.len()) as u64 * 4, BASE));
            second.extend([jal(BASE), 0]);
        }
        second.extend([i(0x23, 31, 29, 20), JR, i(9, 29, 29, 24)]);
        let second_len = second.len();
        code.extend(second);
        let sites = closed_functions(
            &[image(&code)],
            &[
                (BASE, 20),
                (BASE + 20, first_len as u64 * 4),
                (next, second_len as u64 * 4),
            ],
            &refs,
        )
        .unwrap();
        assert_eq!(sites[&BASE].len(), 300);
    }
}
