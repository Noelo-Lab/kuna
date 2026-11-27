//! javac's layout of protected regions. d8 keeps a try item only over
//! instructions that can throw, places blocks in its own order (a handler
//! among the code it protects, a `throw` after the method's return), and
//! cannot list a typed handler after a catch-all, so a handler's protection
//! arrives in pieces and an outer handler does not cover the inner `finally`
//! it encloses, neither of which the engine's structuring matches against
//! javac's shape. Here each handler (with its catch type) gets the blocks it
//! protects plus the blocks of every region whose handler's first throwing
//! code it protects (the nested ones), laid out together, innermost first: a block among them that
//! cannot throw stays and is covered, any other block moves after them, and
//! a `synchronized` region also covers the code between the `monitor-enter`
//! and its first protected block. Each region is then one exception-table
//! entry, innermost first, and a handler protecting its own code gets a
//! second one, as javac writes them. The emitter checks that every block
//! still meets its own handlers in its own order.

use std::collections::{BTreeMap, BTreeSet};

use crate::code::{Kind, Method};

/// A handler's region: the first and last block of its run in the layout,
/// and the blocks of its own handler code that it protects as well.
#[derive(Clone, Debug)]
pub struct Region {
    pub handler: u32,
    pub ty: Option<u32>,
    pub first: u32,
    pub last: u32,
    pub own: Vec<u32>,
}

pub fn throws(m: &Method, b: u32) -> bool {
    let blk = &m.blocks[b as usize];
    m.ops[blk.start..blk.end].iter().any(|o| o.throws && o.kind != Kind::Absorbed)
}

type Key = (u32, Option<u32>);

/// Rearranges `order` and returns the regions in exception-table order.
pub fn arrange(m: &Method, order: &mut Vec<u32>) -> Vec<Region> {
    let n = m.blocks.len();
    let mut preds: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut entry = vec![false; n];
    let mut protects: BTreeMap<Key, BTreeSet<u32>> = BTreeMap::new();
    let mut rank: BTreeMap<Key, usize> = BTreeMap::new();
    for (b, blk) in m.blocks.iter().enumerate().filter(|(_, b)| b.reachable) {
        for &s in &blk.succs {
            preds[s as usize].push(b as u32);
        }
        for (i, &(ty, h)) in blk.handlers.iter().enumerate() {
            entry[h as usize] = true;
            protects.entry((h, ty)).or_default().insert(b as u32);
            rank.entry((h, ty)).or_insert(i);
        }
    }
    let mut chains: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for &(h, _) in protects.keys() {
        let mut chain = BTreeSet::from([h]);
        let mut work = vec![h];
        while let Some(b) = work.pop() {
            for &s in &m.blocks[b as usize].succs {
                if !chain.contains(&s) && !entry[s as usize] && preds[s as usize].iter().all(|p| chain.contains(p)) {
                    chain.insert(s);
                    work.push(s);
                }
            }
        }
        chains.insert(h, chain);
    }
    let keys: Vec<Key> = protects.keys().copied().collect();
    let mut inside = protects.clone();
    for _ in 0..8 {
        let mut changed = false;
        for &k in &keys {
            let nested: Vec<Key> = keys
                .iter()
                .copied()
                .filter(|&g| g.0 != k.0 && first_throw(m, g.0).is_some_and(|b| inside[&k].contains(&b)))
                .collect();
            for g in nested {
                let add: Vec<u32> = inside[&g].iter().copied().filter(|b| !inside[&k].contains(b)).collect();
                changed |= !add.is_empty();
                inside.get_mut(&k).expect("region").extend(add);
            }
        }
        if !changed {
            break;
        }
    }
    let mut sorted = keys;
    sorted.sort_by_key(|k| (inside[k].len(), rank[k]));
    let mut out = Vec::new();
    for k in sorted {
        let monitor = if k.1.is_none() { releases(m, k.0) } else { None };
        if k.1.is_some() || !rethrows(m, k.0) {
            continue;
        }
        let mut body: BTreeSet<u32> = inside[&k].difference(&chains[&k.0]).copied().collect();
        if let Some(r) = monitor {
            let exits: Vec<u32> = (0..n as u32)
                .filter(|&b| {
                    let o = &m.ops[m.blocks[b as usize].start];
                    m.blocks[b as usize].reachable
                        && o.insn.op == 0x1e
                        && o.insn.a == r
                        && !chains[&k.0].contains(&b)
                        && !preds[b as usize].is_empty()
                        && preds[b as usize].iter().all(|p| body.contains(p))
                })
                .collect();
            body.extend(exits);
        }
        let at: Vec<usize> = order.iter().enumerate().filter(|(_, b)| body.contains(b)).map(|(i, _)| i).collect();
        let (Some(&i0), Some(&i1)) = (at.first(), at.last()) else { continue };
        let mut held = body.clone();
        loop {
            let more: Vec<u32> = order[i0..=i1]
                .iter()
                .copied()
                .filter(|&b| !held.contains(&b) && !throws(m, b) && !entry[b as usize] && preds[b as usize].iter().all(|p| held.contains(p)))
                .collect();
            if more.is_empty() {
                break;
            }
            held.extend(more);
        }
        let (keep, moved): (Vec<u32>, Vec<u32>) = order[i0..=i1].iter().partition(|&&b| held.contains(&b));
        let last = *keep.last().expect("region block");
        let tail = order.split_off(i1 + 1);
        order.truncate(i0);
        order.extend(keep);
        order.extend(moved);
        order.extend(tail);
        let mut start = i0;
        while monitor.is_some() && start > 0 && !throws(m, order[start - 1]) && !entry[order[start - 1] as usize] {
            start -= 1;
        }
        let mut own: Vec<u32> = chains[&k.0].iter().copied().filter(|b| protects[&k].contains(b)).collect();
        if monitor.is_some() && !own.is_empty() {
            own.push(k.0);
        }
        out.push(Region { handler: k.0, ty: k.1, first: order[start], last, own });
    }
    out
}

/// The monitor register a handler releases first thing after its
/// `move-exception`.
fn releases(m: &Method, h: u32) -> Option<u32> {
    let (mut b, mut k) = (h as usize, m.blocks[h as usize].start + 1);
    for _ in 0..4 {
        if k >= m.blocks[b].end {
            match m.blocks[b].succs[..] {
                [s] => (b, k) = (s as usize, m.blocks[s as usize].start),
                _ => return None,
            }
            continue;
        }
        match m.ops[k].insn.op {
            0x28..=0x2a => match m.blocks[b].succs[..] {
                [s] => (b, k) = (s as usize, m.blocks[s as usize].start),
                _ => return None,
            },
            0x1e => return Some(m.ops[k].insn.a),
            _ => return None,
        }
    }
    None
}

/// The first block of a handler's code that can throw, following it from its
/// entry through blocks with a single successor.
fn first_throw(m: &Method, h: u32) -> Option<u32> {
    let mut b = h;
    for _ in 0..8 {
        if throws(m, b) {
            return Some(b);
        }
        match m.blocks[b as usize].succs[..] {
            [s] => b = s,
            _ => return None,
        }
    }
    None
}

/// Whether a catch-all handler is a `finally` (or `synchronized`) one: its
/// code throws the exception it caught again. One that does not is javac's
/// `catch (Throwable t)`, which DEX writes as a catch-all.
pub fn rethrows(m: &Method, h: u32) -> bool {
    let blk = &m.blocks[h as usize];
    let Some(caught) = m.ops.get(blk.start).filter(|o| o.insn.op == 0x0d).and_then(|o| o.def.as_ref()).map(|d| d.id) else {
        return false;
    };
    let mut seen = BTreeSet::from([h]);
    let mut work = vec![h];
    while let Some(b) = work.pop() {
        let blk = &m.blocks[b as usize];
        if m.ops[blk.start..blk.end].iter().any(|o| o.insn.op == 0x27 && o.uses.first().is_some_and(|u| u.val == caught)) {
            return true;
        }
        for &s in &blk.succs {
            if seen.len() < 64 && seen.insert(s) {
                work.push(s);
            }
        }
    }
    false
}
