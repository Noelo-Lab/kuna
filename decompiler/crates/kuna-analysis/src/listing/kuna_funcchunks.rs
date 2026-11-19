//! Function chunks (`funcchunks`): a per-function body made of address runs,
//! with tails that several functions can own.
//!
//! The Listing keys a function by its entry; every consumer that needs a body
//! clips `[entry, next_entry)`. That clip mis-assigns the two shapes compilers
//! produce most often: a shared tail (an epilogue or error path several
//! functions jump into) and an outlined part placed after another function.
//!
//! This model is built after the walk from the decoded flow. A function's body is the flow
//! closure of its entry: fall-through and branch targets, never call targets,
//! and never another function's entry (a jump there is a tail call). The body
//! is cut into address runs; the run holding the entry is the main chunk and
//! every other run is a tail. An instruction reached from several entries is
//! shared, and [`ChunkModel::owners`] lists every function that reaches it.

use std::collections::{BTreeMap, BTreeSet};

use super::Listing;

/// Instruction cap on one function's closure (a runaway walk stays bounded).
const MAX_BODY_INSNS: usize = 200_000;

/// Undecoded bytes allowed between two runs before they count as separate
/// chunks (inter-instruction alignment padding).
const MAX_PAD_GAP: u64 = 16;

/// Every function's chunks and the owners of shared instructions.
#[derive(Debug, Default, Clone)]
pub struct ChunkModel {
    chunks: BTreeMap<u64, Vec<(u64, u64)>>,
    shared: BTreeMap<u64, Vec<u64>>,
}

impl ChunkModel {
    /// Build the model over every function the Listing knows.
    pub fn build(listing: &Listing) -> ChunkModel {
        let entries: BTreeSet<u64> = listing.functions().map(|(&e, _)| e).collect();
        let mut model = ChunkModel::default();
        let mut owners: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
        for &entry in &entries {
            let body = closure(listing, &entries, entry);
            for &a in &body {
                owners.entry(a).or_default().push(entry);
            }
            model.chunks.insert(entry, runs(listing, entry, &body));
        }
        model.shared = owners.into_iter().filter(|(_, o)| o.len() > 1).collect();
        model
    }

    /// The `[start, end)` chunks of the function at `entry`, main chunk first,
    /// tails after it in address order.
    pub fn chunks(&self, entry: u64) -> Option<&[(u64, u64)]> {
        self.chunks.get(&entry).map(Vec::as_slice)
    }

    /// The functions whose body holds the instruction at `vma`.
    pub fn owners(&self, vma: u64) -> Vec<u64> {
        if let Some(o) = self.shared.get(&vma) {
            return o.clone();
        }
        self.chunks
            .iter()
            .filter(|(_, cs)| cs.iter().any(|&(s, e)| s <= vma && vma < e))
            .map(|(&f, _)| f)
            .collect()
    }

    /// Instructions reached from more than one entry, with their owners.
    pub fn shared(&self) -> &BTreeMap<u64, Vec<u64>> {
        &self.shared
    }

    /// Number of tail chunks (chunks that do not hold their function's entry).
    pub fn tail_count(&self) -> usize {
        self.chunks.values().map(|c| c.len().saturating_sub(1)).sum()
    }
}

/// The flow closure of `entry`: every decoded instruction reachable by
/// fall-through or branch without entering another function's entry.
fn closure(listing: &Listing, entries: &BTreeSet<u64>, entry: u64) -> BTreeSet<u64> {
    let mut seen = BTreeSet::new();
    let mut work = vec![entry];
    while let Some(vma) = work.pop() {
        if seen.len() >= MAX_BODY_INSNS || (vma != entry && entries.contains(&vma)) {
            continue;
        }
        let Some(i) = listing.instruction_at(vma) else { continue };
        if !seen.insert(vma) {
            continue;
        }
        if !i.flow.is_call {
            work.extend(i.flows.iter().copied());
        }
        work.extend(i.fall_through);
    }
    seen
}

/// Cut a body into address runs, main chunk (the one holding `entry`) first.
fn runs(listing: &Listing, entry: u64, body: &BTreeSet<u64>) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    for &a in body {
        let end = a + listing.instruction_at(a).map_or(1, |i| u64::from(i.len.max(1)));
        match out.last_mut() {
            Some(last) if a >= last.1 && a - last.1 <= MAX_PAD_GAP && !decoded_between(listing, last.1, a) => {
                last.1 = last.1.max(end);
            }
            _ => out.push((a, end)),
        }
    }
    if let Some(i) = out.iter().position(|&(s, e)| s <= entry && entry < e) {
        let main = out.remove(i);
        out.insert(0, main);
    }
    out
}

/// True iff a decoded instruction starts in `[lo, hi)`: a gap holding foreign
/// code splits two runs even when it is short.
fn decoded_between(listing: &Listing, lo: u64, hi: u64) -> bool {
    lo < hi && listing.instructions_in_range(lo, Some(hi)).next().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::model::{FlowKind, FlowType, Insn};

    fn insn(addr: u64, kind: FlowKind, flows: Vec<u64>) -> Insn {
        let terminal = matches!(kind, FlowKind::Return | FlowKind::UnconditionalBranch);
        Insn {
            addr,
            len: 4,
            fall_through: (!terminal).then_some(addr + 4),
            flow: FlowType {
                kind,
                is_call: kind == FlowKind::Call,
                is_jump: matches!(kind, FlowKind::UnconditionalBranch | FlowKind::ConditionalBranch),
                is_terminal: kind == FlowKind::Return,
                is_conditional: kind == FlowKind::ConditionalBranch,
                has_fallthrough: !terminal,
                ..Default::default()
            },
            flows,
            mnemonic: "".into(),
            operands: String::new(),
            pcode: None,
        }
    }

    /// f@0x100 and g@0x200 both jump into the shared tail at 0x300, which
    /// returns; f also calls h@0x400, which must not join f's body.
    fn listing() -> Listing {
        use FlowKind::*;
        let insns = vec![
            insn(0x100, Call, vec![0x400]),
            insn(0x104, ConditionalBranch, vec![0x300]),
            insn(0x108, Return, vec![]),
            insn(0x200, Fallthrough, vec![]),
            insn(0x204, UnconditionalBranch, vec![0x300]),
            insn(0x300, Fallthrough, vec![]),
            insn(0x304, Return, vec![]),
            insn(0x400, Return, vec![]),
        ];
        Listing::from_parts_for_test(insns, &[0x100, 0x200, 0x400])
    }

    #[test]
    fn shared_tail_has_both_owners() {
        let m = ChunkModel::build(&listing());
        assert_eq!(m.chunks(0x100).unwrap(), &[(0x100, 0x10c), (0x300, 0x308)]);
        assert_eq!(m.chunks(0x200).unwrap(), &[(0x200, 0x208), (0x300, 0x308)]);
        assert_eq!(m.chunks(0x400).unwrap(), &[(0x400, 0x404)]);
        assert_eq!(m.owners(0x304), vec![0x100, 0x200]);
        assert_eq!(m.owners(0x104), vec![0x100]);
        assert_eq!(m.tail_count(), 2);
    }
}
