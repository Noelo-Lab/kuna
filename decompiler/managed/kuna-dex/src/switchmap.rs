//! javac's `switch` on an enum reads `$SwitchMap$...[e.ordinal()]`, an
//! array its synthetic class fills in `<clinit>`. d8's release build changes
//! both sides: a small `switch` becomes a chain of `if-eq`/`if-ne` tests of
//! the value against hoisted constants, and `<clinit>` fills the first entry
//! through the register it stored the new array from. javac writes such an
//! array only for a `switch` on an enum and reads it only to switch on the
//! value, so the chain is the `switch` again: its first test becomes a
//! `sparse-switch` with the chain's cases and an explicit default target,
//! and the tests after it become `nop`s. And in `<clinit>`, when an `aput`
//! fills the array through the register a `sput-object` just stored to such
//! a field, that register reads the field right after the store (the same
//! reference), as javac's `getstatic` before the first entry does.

use std::collections::{BTreeMap, BTreeSet};

use crate::insn::{packed_switch, sparse_switch, Insn};
use crate::Dex;

/// Inserts the field read of a `<clinit>` that fills a `$SwitchMap$` array
/// through the register it stored it from, so the read is pushed before the
/// first entry's index as javac's is: after the `sput-object`, and inside
/// the try item (`tries`) that covers the entry's `aput` when that starts
/// later, since javac's read is inside the entry's `try`. A class that
/// switches on several enums has one array per enum, each stored and filled
/// this way.
pub fn refill(dex: &Dex, insns: &mut Vec<Insn>, tries: &[(u32, u32)]) {
    let stores: Vec<usize> = (0..insns.len()).filter(|&k| insns[k].op == 0x69 && switch_map(dex, insns[k].idx)).collect();
    for k in stores.into_iter().rev() {
        let Some(fill) = filled_through(insns, k) else { continue };
        let start = tries.iter().find(|&&(s, e)| s <= insns[fill].addr && insns[fill].addr < e).map_or(0, |t| t.0);
        let at = (k + 1..=fill).find(|&i| insns[i].addr >= start).unwrap_or(fill);
        let mut read = insns[k].clone();
        read.op = 0x62;
        read.addr = insns[at].addr;
        read.size = 0;
        insns.insert(at, read);
    }
}

/// The `aput` that fills the array through the register the `sput-object`
/// at `k` stored it from, when nothing before it writes the register or
/// branches.
fn filled_through(insns: &[Insn], k: usize) -> Option<usize> {
    let r = insns[k].a;
    for (j, x) in insns.iter().enumerate().skip(k + 1) {
        if x.op == 0x4b && x.b == r {
            return Some(j);
        }
        if writes(x) == Some(r) || matches!(x.op, 0x0e..=0x11 | 0x27..=0x2c | 0x32..=0x3d) {
            return None;
        }
    }
    None
}

fn switch_map(dex: &Dex, field: u32) -> bool {
    dex.field(field).and_then(|f| dex.string(f.name)).is_ok_and(|n| n.starts_with(b"$SwitchMap$"))
}

/// The addresses branches, switches and `handlers` reach in `insns`.
pub fn entered(insns: &[Insn], code: &[u8], handlers: &[u32]) -> BTreeSet<u32> {
    let mut entered: BTreeSet<u32> = handlers.iter().copied().collect();
    for x in insns {
        match x.op {
            0x28..=0x2a | 0x32..=0x3d => entered.extend(u32::try_from(x.target())),
            0x2b => entered.extend(packed_switch(code, x.target()).map(|p| p.1).unwrap_or_default().iter().map(|&r| (i64::from(x.addr) + i64::from(r)) as u32)),
            0x2c => entered.extend(sparse_switch(code, x.target()).unwrap_or_default().iter().map(|&(_, r)| (i64::from(x.addr) + i64::from(r)) as u32)),
            _ => {}
        }
    }
    entered
}

/// A rewritten chain's cases (key, absolute target) and default target.
pub type Chain = (Vec<(i32, u32)>, u32);

/// The registers an instruction reads and writes.
pub type Regs<'r> = &'r dyn Fn(&Insn) -> (Vec<u32>, Vec<u32>);

/// Rewrites every such chain in `insns` (which branches and handlers reach
/// at `entered`, and try items cover at `tries`); returns them by the
/// address of the `sparse-switch` that replaces the first test.
pub fn rewrite(dex: &Dex, insns: &mut [Insn], code: &[u8], entered: &BTreeSet<u32>, tries: &[(u32, u32)], handlers: &[u32], regs: Regs) -> BTreeMap<u32, Chain> {
    let mut out = BTreeMap::new();
    let mut maps: BTreeSet<u32> = BTreeSet::new();
    let mut known: Option<Known> = None;
    let mut k = 0;
    while k < insns.len() {
        let x = &insns[k];
        if x.op == 0x44 && maps.contains(&x.b) {
            let consts = known.get_or_insert_with(|| Known::new(insns, code, entered, tries, handlers, regs)).after(k, regs);
            if let Some((first, chain, end)) = chain(insns, k, consts, entered, tries, regs) {
                let v = insns[k].a;
                for n in &mut insns[first + 1..end] {
                    n.op = 0x00;
                }
                let s = &mut insns[first];
                s.op = 0x2c;
                s.a = v;
                out.insert(s.addr, chain);
                k = end;
                continue;
            }
        }
        let x = &insns[k];
        if x.op == 0x62 && switch_map(dex, x.idx) {
            maps.insert(x.a);
        } else if let Some(w) = writes(x) {
            maps.remove(&w);
        }
        k += 1;
    }
    out
}

/// The register an instruction writes, for those that can stand between the
/// array's read and its use.
fn writes(x: &Insn) -> Option<u32> {
    match x.op {
        0x01..=0x0d | 0x12..=0x1c | 0x1f..=0x23 | 0x2d..=0x31 | 0x44..=0x4a | 0x52..=0x58 | 0x60..=0x66 | 0x7b..=0xe2 => Some(x.a),
        _ => None,
    }
}

/// The chain of tests on the value `aget` `k` defines (`consts`: the
/// registers holding a 32-bit constant on every path to it, d8 hoisting
/// them anywhere above): hoisted constants, then consecutive `if-eq`/`if-ne` of that value against
/// constants, which nothing else enters; an `if-ne` ends the chain (its
/// fall-through is its case, its target the default). A constant loaded
/// between two tests vanishes with them, so its register must be dead at
/// every target.
fn chain(insns: &[Insn], k: usize, mut consts: BTreeMap<u32, i32>, entered: &BTreeSet<u32>, tries: &[(u32, u32)], regs: Regs) -> Option<(usize, Chain, usize)> {
    let v = insns[k].a;
    consts.remove(&v);
    let mut j = k + 1;
    while let Some(x) = insns.get(j) {
        match x.op {
            0x12..=0x15 if x.a != v => {
                consts.insert(x.a, x.lit as i32);
            }
            0x00 => {}
            _ => break,
        }
        j += 1;
    }
    let first = j;
    let mut cases: Vec<(i32, u32)> = Vec::new();
    let mut default = None;
    let mut end = j;
    while let Some(x) = insns.get(j) {
        if j > first && entered.contains(&x.addr) {
            break;
        }
        if matches!(x.op, 0x12..=0x15) && x.a != v && j > first {
            consts.insert(x.a, x.lit as i32);
            j += 1;
            continue;
        }
        if !matches!(x.op, 0x32 | 0x33) {
            break;
        }
        let key = match (x.a == v, x.b == v) {
            (true, false) => consts.get(&x.b),
            (false, true) => consts.get(&x.a),
            _ => None,
        };
        let Some(&key) = key else { break };
        let target = u32::try_from(x.target()).ok()?;
        let (case, rest) = if x.op == 0x32 { (target, x.next()) } else { (x.next(), target) };
        if !cases.iter().any(|c| c.0 == key) {
            cases.push((key, case));
        }
        default = Some(rest);
        j += 1;
        end = j;
        if x.op == 0x33 {
            break;
        }
    }
    let default = default?;
    let late: Vec<u32> = insns[first..end].iter().filter(|x| matches!(x.op, 0x12..=0x15)).map(|x| x.a).collect();
    let targets: Vec<u32> = cases.iter().map(|c| c.1).chain([default]).collect();
    if !late.iter().all(|&r| targets.iter().all(|&t| dead(insns, t, r, tries, regs))) {
        return None;
    }
    cases.sort_unstable();
    Some((first, (cases, default), end))
}

/// Whether register `r` is written before it is read on every path from
/// `addr`, as far as a short walk over code no try item covers can tell.
fn dead(insns: &[Insn], addr: u32, r: u32, tries: &[(u32, u32)], regs: Regs) -> bool {
    let at = |a: u32| insns.binary_search_by_key(&a, |x| x.addr).ok();
    let mut work = vec![addr];
    let mut seen = BTreeSet::new();
    while let Some(a) = work.pop() {
        if !seen.insert(a) {
            continue;
        }
        if seen.len() > 256 {
            return false;
        }
        let Some(mut k) = at(a) else { return false };
        loop {
            let x = &insns[k];
            if tries.iter().any(|&(s, e)| s <= x.addr && x.addr < e) {
                return false;
            }
            let (reads, writes) = regs(x);
            if reads.contains(&r) {
                return false;
            }
            if writes.contains(&r) {
                break;
            }
            match x.op {
                0x0e..=0x11 | 0x27 => break,
                0x28..=0x2a => {
                    let Ok(t) = u32::try_from(x.target()) else { return false };
                    work.push(t);
                    break;
                }
                0x32..=0x3d => {
                    let Ok(t) = u32::try_from(x.target()) else { return false };
                    work.push(t);
                }
                0x2b | 0x2c => return false,
                _ => {}
            }
            k += 1;
            if k >= insns.len() {
                return false;
            }
        }
    }
    true
}

/// The registers holding the same 32-bit constant on every path to each
/// block of the method as d8 wrote it (a handler is entered knowing none,
/// since an exception leaves from the middle of a block).
struct Known {
    insns: Vec<Insn>,
    starts: Vec<usize>,
    entry: Vec<Option<BTreeMap<u32, i32>>>,
}

impl Known {
    fn new(insns: &[Insn], code: &[u8], entered: &BTreeSet<u32>, tries: &[(u32, u32)], handlers: &[u32], regs: Regs) -> Known {
        let insns = insns.to_vec();
        let at = |a: u32| insns.binary_search_by_key(&a, |x| x.addr).ok();
        let mut lead: BTreeSet<usize> = BTreeSet::from([0]);
        for (k, x) in insns.iter().enumerate() {
            if matches!(x.op, 0x0e..=0x11 | 0x27..=0x2c | 0x32..=0x3d) {
                lead.insert(k + 1);
            }
        }
        for &a in entered.iter().chain(tries.iter().flat_map(|t| [&t.0, &t.1])) {
            lead.extend(at(a));
        }
        lead.retain(|&k| k < insns.len());
        let starts: Vec<usize> = lead.into_iter().collect();
        let block = |k: usize| starts.partition_point(|&s| s <= k) - 1;
        let mut entry: Vec<Option<BTreeMap<u32, i32>>> = vec![None; starts.len()];
        let mut work: Vec<usize> = vec![0];
        entry[0] = Some(BTreeMap::new());
        for &a in handlers {
            if let Some(k) = at(a) {
                entry[block(k)] = Some(BTreeMap::new());
                work.push(block(k));
            }
        }
        while let Some(b) = work.pop() {
            let mut state = entry[b].clone().unwrap_or_default();
            let end = starts.get(b + 1).copied().unwrap_or(insns.len());
            for x in &insns[starts[b]..end] {
                step(&mut state, x, regs);
            }
            let last = &insns[end - 1];
            let mut succ: Vec<usize> = Vec::new();
            match last.op {
                0x0e..=0x11 | 0x27 => {}
                0x28..=0x2a => succ.extend(u32::try_from(last.target()).ok().and_then(at)),
                0x32..=0x3d => {
                    succ.extend(u32::try_from(last.target()).ok().and_then(at));
                    succ.push(end);
                }
                0x2b => {
                    succ.extend(packed_switch(code, last.target()).map(|p| p.1).unwrap_or_default().iter().filter_map(|&r| at((i64::from(last.addr) + i64::from(r)) as u32)));
                    succ.push(end);
                }
                0x2c => {
                    succ.extend(sparse_switch(code, last.target()).unwrap_or_default().iter().filter_map(|&(_, r)| at((i64::from(last.addr) + i64::from(r)) as u32)));
                    succ.push(end);
                }
                _ => succ.push(end),
            }
            for s in succ.into_iter().filter(|&s| s < insns.len()) {
                let t = block(s);
                let merged = match &entry[t] {
                    None => state.clone(),
                    Some(old) => old.iter().filter(|(r, v)| state.get(r) == Some(v)).map(|(&r, &v)| (r, v)).collect(),
                };
                if entry[t].as_ref() != Some(&merged) {
                    entry[t] = Some(merged);
                    work.push(t);
                }
            }
        }
        Known { insns, starts, entry }
    }

    /// The known constants right after instruction `k`.
    fn after(&self, k: usize, regs: Regs) -> BTreeMap<u32, i32> {
        let b = self.starts.partition_point(|&s| s <= k) - 1;
        let mut state = self.entry[b].clone().unwrap_or_default();
        for x in &self.insns[self.starts[b]..=k] {
            step(&mut state, x, regs);
        }
        state
    }
}

fn step(state: &mut BTreeMap<u32, i32>, x: &Insn, regs: Regs) {
    match x.op {
        0x12..=0x15 => {
            state.insert(x.a, x.lit as i32);
        }
        0x01..=0x03 => match state.get(&x.b).copied() {
            Some(v) => {
                state.insert(x.a, v);
            }
            None => {
                state.remove(&x.a);
            }
        },
        _ => {
            for r in regs(x).1 {
                state.remove(&r);
            }
        }
    }
}
