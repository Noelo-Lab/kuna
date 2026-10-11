//! Instruction records in bounded chunks, with a small ordered address index.
//! Tree nodes hold row numbers rather than the large instruction payload, so
//! spare node slots do not reserve another full record. Chunks never relocate
//! existing records or grow a second whole-image buffer during the walk.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ops::RangeBounds;

use super::model::Insn;

const CHUNK_ROWS: usize = 1024;

#[derive(Default)]
pub(super) struct InstructionStore {
    index: BTreeMap<u64, usize>,
    chunks: Vec<Vec<Insn>>,
}

impl Clone for InstructionStore {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            chunks: self
                .chunks
                .iter()
                .map(|chunk| {
                    let mut copy = Vec::with_capacity(CHUNK_ROWS);
                    copy.extend_from_slice(chunk);
                    copy
                })
                .collect(),
        }
    }
}

impl InstructionStore {
    pub fn insert(&mut self, address: u64, insn: Insn) -> Option<Insn> {
        match self.index.entry(address) {
            Entry::Occupied(entry) => {
                let slot = *entry.get();
                Some(std::mem::replace(
                    &mut self.chunks[slot / CHUNK_ROWS][slot % CHUNK_ROWS],
                    insn,
                ))
            }
            Entry::Vacant(entry) => {
                let slot = self.chunks.last().map_or(0, |chunk| {
                    (self.chunks.len() - 1) * CHUNK_ROWS + chunk.len()
                });
                if slot % CHUNK_ROWS == 0 {
                    self.chunks.push(Vec::with_capacity(CHUNK_ROWS));
                }
                self.chunks.last_mut().unwrap().push(insn);
                entry.insert(slot);
                None
            }
        }
    }

    pub fn get(&self, address: &u64) -> Option<&Insn> {
        let &slot = self.index.get(address)?;
        Some(&self.chunks[slot / CHUNK_ROWS][slot % CHUNK_ROWS])
    }

    pub fn get_mut(&mut self, address: &u64) -> Option<&mut Insn> {
        let &slot = self.index.get(address)?;
        Some(&mut self.chunks[slot / CHUNK_ROWS][slot % CHUNK_ROWS])
    }

    pub fn contains_key(&self, address: &u64) -> bool {
        self.index.contains_key(address)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &u64> {
        self.index.keys()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&u64, &Insn)> {
        self.range(..)
    }

    pub fn range(
        &self,
        bounds: impl RangeBounds<u64>,
    ) -> impl DoubleEndedIterator<Item = (&u64, &Insn)> {
        self.index
            .range(bounds)
            .map(|(address, &slot)| (address, &self.chunks[slot / CHUNK_ROWS][slot % CHUNK_ROWS]))
    }
}

impl FromIterator<(u64, Insn)> for InstructionStore {
    fn from_iter<T: IntoIterator<Item = (u64, Insn)>>(iter: T) -> Self {
        let mut store = Self::default();
        for (address, insn) in iter {
            store.insert(address, insn);
        }
        store
    }
}

#[cfg(test)]
mod tests {
    use super::super::model::{FlowType, RawOp};
    use super::*;
    use kuna_num::opcodes::OpCode;

    fn insn(addr: u64) -> Insn {
        Insn {
            addr,
            len: 3,
            fall_through: addr.checked_add(3),
            flow: FlowType {
                has_fallthrough: true,
                ..FlowType::default()
            },
            flows: vec![addr.wrapping_add(17)],
            mnemonic: format!("op_{addr}"),
            operands: format!("target_{addr}"),
            pcode: Some(vec![RawOp {
                opcode: OpCode::CPUI_RETURN,
                in0: None,
            }]),
        }
    }

    fn same(actual: &Insn, expected: &Insn) {
        assert_eq!(actual.addr, expected.addr);
        assert_eq!(actual.len, expected.len);
        assert_eq!(actual.fall_through, expected.fall_through);
        assert_eq!(actual.flow, expected.flow);
        assert_eq!(actual.flows, expected.flows);
        assert_eq!(actual.mnemonic, expected.mnemonic);
        assert_eq!(actual.operands, expected.operands);
        assert_eq!(
            actual.pcode.as_ref().unwrap()[0].opcode,
            expected.pcode.as_ref().unwrap()[0].opcode
        );
    }

    #[test]
    fn unordered_inserts_replacements_and_ranges_match_the_instruction_map() {
        let addresses: Vec<_> = (0..CHUNK_ROWS * 3 + 17)
            .map(|n| n as u64 * 7 + 3)
            .chain([0, u64::MAX])
            .collect();
        let mut store = InstructionStore::default();
        let mut expected = BTreeMap::new();
        for &addr in addresses.iter().rev() {
            store.insert(addr, insn(addr));
            expected.insert(addr, insn(addr));
        }
        for &addr in &[0, addresses[CHUNK_ROWS], u64::MAX] {
            let mut replacement = insn(addr);
            replacement.len = 5;
            same(
                &store.insert(addr, replacement.clone()).unwrap(),
                &expected.insert(addr, replacement).unwrap(),
            );
        }
        assert_eq!(store.len(), expected.len());
        assert_eq!(
            store.keys().collect::<Vec<_>>(),
            expected.keys().collect::<Vec<_>>()
        );
        for (&addr, insn) in &expected {
            assert!(store.contains_key(&addr));
            same(store.get(&addr).unwrap(), insn);
        }
        assert!(store.get(&1).is_none());
        for (start, end) in [
            (0, 0),
            (0, 100),
            (9, 5000),
            (7000, 20000),
            (u64::MAX, u64::MAX),
        ] {
            let got: Vec<_> = store.range(start..=end).collect();
            let want: Vec<_> = expected.range(start..=end).collect();
            assert_eq!(got.len(), want.len());
            for ((a, actual), (b, expected)) in got.into_iter().zip(want) {
                assert_eq!(a, b);
                same(actual, expected);
            }
        }
        assert_eq!(
            store.range(..u64::MAX).next_back().unwrap().0,
            expected.range(..u64::MAX).next_back().unwrap().0
        );
        for ((a, actual), (b, expected)) in store.iter().rev().zip(expected.iter().rev()) {
            assert_eq!(a, b);
            same(actual, expected);
        }
        assert_eq!(
            store.chunks.iter().map(Vec::len).sum::<usize>(),
            store.len()
        );
        assert!(store
            .chunks
            .iter()
            .all(|chunk| chunk.capacity() == CHUNK_ROWS));
    }

    #[test]
    fn a_cloned_partial_chunk_can_extend_without_changing_the_prior_listing() {
        let prior: InstructionStore = (0..CHUNK_ROWS + 7)
            .map(|n| (n as u64, insn(n as u64)))
            .collect();
        let mut next = prior.clone();
        next.get_mut(&1).unwrap().mnemonic = "changed".into();
        next.get_mut(&1).unwrap().flows.push(9000);
        for n in CHUNK_ROWS + 7..CHUNK_ROWS * 3 + 5 {
            next.insert(n as u64, insn(n as u64));
        }
        same(prior.get(&1).unwrap(), &insn(1));
        assert_eq!(prior.len(), CHUNK_ROWS + 7);
        assert_eq!(next.len(), CHUNK_ROWS * 3 + 5);
        assert!(prior.get(&(CHUNK_ROWS as u64 * 3)).is_none());
        same(
            next.get(&(CHUNK_ROWS as u64 * 3)).unwrap(),
            &insn(CHUNK_ROWS as u64 * 3),
        );
        let capacity: usize = next.chunks.iter().map(Vec::capacity).sum();
        assert!(capacity - next.len() < CHUNK_ROWS);
    }
}
