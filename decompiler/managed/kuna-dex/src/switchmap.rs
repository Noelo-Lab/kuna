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
/// through the register it stored it from: right after the `sput-object`,
/// so the read is pushed before the first entry's index as javac's is.
pub fn refill(dex: &Dex, insns: &mut Vec<Insn>) {
    let Some(k) = insns.iter().position(|x| x.op == 0x69 && switch_map(dex, x.idx)) else { return };
    let r = insns[k].a;
    let Some(next) = insns.get(k + 1).map(|x| x.addr) else { return };
    for x in &insns[k + 1..] {
        if x.op == 0x4b && x.b == r {
            let mut read = insns[k].clone();
            read.op = 0x62;
            read.addr = next;
            read.size = 0;
            insns.insert(k + 1, read);
            return;
        }
        if writes(x) == Some(r) || matches!(x.op, 0x0e..=0x11 | 0x27..=0x2c | 0x32..=0x3d) {
            return;
        }
    }
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
pub fn rewrite(dex: &Dex, insns: &mut [Insn], entered: &BTreeSet<u32>, tries: &[(u32, u32)], regs: Regs) -> BTreeMap<u32, Chain> {
    let mut out = BTreeMap::new();
    let mut maps: BTreeSet<u32> = BTreeSet::new();
    let mut k = 0;
    while k < insns.len() {
        let x = &insns[k];
        if x.op == 0x44 && maps.contains(&x.b) {
            if let Some((first, chain, end)) = chain(insns, k, entered, tries, regs) {
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

/// The chain of tests on the value `aget` `k` defines: hoisted 32-bit
/// constants, then consecutive `if-eq`/`if-ne` of that value against
/// constants, which nothing else enters; an `if-ne` ends the chain (its
/// fall-through is its case, its target the default). A constant loaded
/// between two tests vanishes with them, so its register must be dead at
/// every target.
fn chain(insns: &[Insn], k: usize, entered: &BTreeSet<u32>, tries: &[(u32, u32)], regs: Regs) -> Option<(usize, Chain, usize)> {
    let v = insns[k].a;
    let mut consts: BTreeMap<u32, i32> = BTreeMap::new();
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
