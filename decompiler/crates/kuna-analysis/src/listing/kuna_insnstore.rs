//! Instruction records in bounded chunks, with a small ordered address index.
//! Tree nodes hold row numbers rather than the large instruction payload, so
//! spare node slots do not reserve another full record. Chunks never relocate
//! existing records or grow a second whole-image buffer during the walk.
//!
//! Store snapshots share immutable instruction chunks. A write to a shared
//! chunk clones just that chunk; appending through a partial tail detaches that
//! tail once, while untouched full chunks remain shared.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ops::RangeBounds;
use std::rc::Rc;

use super::model::Insn;

const CHUNK_ROWS: usize = 1024;

#[derive(Default, Clone)]
pub(super) struct InstructionStore {
    index: BTreeMap<u64, usize>,
    chunks: Vec<Rc<InstructionChunk>>,
}

/// A bounded block of instruction records, shared between store snapshots.
struct InstructionChunk {
    rows: Vec<Insn>,
}

impl Clone for InstructionChunk {
    fn clone(&self) -> Self {
        let mut rows = Vec::with_capacity(CHUNK_ROWS);
        rows.extend_from_slice(&self.rows);
        Self { rows }
    }
}

impl InstructionStore {
    pub fn insert(&mut self, address: u64, insn: Insn) -> Option<Insn> {
        match self.index.entry(address) {
            Entry::Occupied(entry) => {
                let slot = *entry.get();
                Some(std::mem::replace(
                    &mut Rc::make_mut(&mut self.chunks[slot / CHUNK_ROWS]).rows[slot % CHUNK_ROWS],
                    insn,
                ))
            }
            Entry::Vacant(entry) => {
                let slot = self.chunks.last().map_or(0, |chunk| {
                    (self.chunks.len() - 1) * CHUNK_ROWS + chunk.rows.len()
                });
                if slot % CHUNK_ROWS == 0 {
                    self.chunks.push(Rc::new(InstructionChunk {
                        rows: Vec::with_capacity(CHUNK_ROWS),
                    }));
                }
                Rc::make_mut(self.chunks.last_mut().unwrap())
                    .rows
                    .push(insn);
                entry.insert(slot);
                None
            }
        }
    }

    pub fn get(&self, address: &u64) -> Option<&Insn> {
        let &slot = self.index.get(address)?;
        Some(&self.chunks[slot / CHUNK_ROWS].rows[slot % CHUNK_ROWS])
    }

    pub fn get_mut(&mut self, address: &u64) -> Option<&mut Insn> {
        let &slot = self.index.get(address)?;
        Some(&mut Rc::make_mut(&mut self.chunks[slot / CHUNK_ROWS]).rows[slot % CHUNK_ROWS])
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
        self.index.range(bounds).map(|(address, &slot)| {
            (
                address,
                &self.chunks[slot / CHUNK_ROWS].rows[slot % CHUNK_ROWS],
            )
        })
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
            store
                .chunks
                .iter()
                .map(|chunk| chunk.rows.len())
                .sum::<usize>(),
            store.len()
        );
        assert!(store
            .chunks
            .iter()
            .all(|chunk| chunk.rows.capacity() == CHUNK_ROWS));
    }

    #[test]
    fn cloned_chunks_detach_independently_for_mutation_and_replacement() {
        let mut prior: InstructionStore = (0..CHUNK_ROWS * 2 + 2)
            .map(|n| (n as u64, insn(n as u64)))
            .collect();
        let mut next = prior.clone();

        assert!(Rc::ptr_eq(&prior.chunks[0], &next.chunks[0]));
        assert!(Rc::ptr_eq(&prior.chunks[1], &next.chunks[1]));
        assert!(Rc::ptr_eq(&prior.chunks[2], &next.chunks[2]));

        // Mutating the clone at the last row of a full chunk detaches that
        // chunk. Its vectors and p-code remain independent of the prior.
        {
            let row = next.get_mut(&((CHUNK_ROWS - 1) as u64)).unwrap();
            row.flows.push(9000);
            row.operands.push_str("_changed");
            row.pcode.as_mut().unwrap().push(RawOp {
                opcode: OpCode::CPUI_COPY,
                in0: None,
            });
        }
        assert!(!Rc::ptr_eq(&prior.chunks[0], &next.chunks[0]));
        assert!(Rc::ptr_eq(&prior.chunks[1], &next.chunks[1]));
        assert!(Rc::ptr_eq(&prior.chunks[2], &next.chunks[2]));
        same(
            prior.get(&((CHUNK_ROWS - 1) as u64)).unwrap(),
            &insn((CHUNK_ROWS - 1) as u64),
        );
        assert_eq!(
            prior
                .get(&((CHUNK_ROWS - 1) as u64))
                .unwrap()
                .pcode
                .as_ref()
                .unwrap()
                .len(),
            1
        );
        let changed = next.get(&((CHUNK_ROWS - 1) as u64)).unwrap();
        assert_eq!(changed.flows, [CHUNK_ROWS as u64 - 1 + 17, 9000]);
        assert_eq!(
            changed.operands,
            format!("target_{}_changed", CHUNK_ROWS - 1)
        );
        assert_eq!(changed.pcode.as_ref().unwrap().len(), 2);

        // Mutating the prior snapshot independently detaches its partial tail.
        {
            let row = prior.get_mut(&((CHUNK_ROWS * 2) as u64)).unwrap();
            row.flows.push(8000);
            row.operands.push_str("_prior");
            row.pcode.as_mut().unwrap().pop();
        }
        assert!(Rc::ptr_eq(&prior.chunks[1], &next.chunks[1]));
        assert!(!Rc::ptr_eq(&prior.chunks[2], &next.chunks[2]));
        assert_eq!(
            prior.get(&((CHUNK_ROWS * 2) as u64)).unwrap().flows.len(),
            2
        );
        assert_eq!(next.get(&((CHUNK_ROWS * 2) as u64)).unwrap().flows.len(), 1);
        assert_eq!(
            prior.get(&((CHUNK_ROWS * 2) as u64)).unwrap().operands,
            format!("target_{}_prior", CHUNK_ROWS * 2)
        );
        assert_eq!(
            next.get(&((CHUNK_ROWS * 2) as u64)).unwrap().operands,
            format!("target_{}", CHUNK_ROWS * 2)
        );
        assert!(prior
            .get(&((CHUNK_ROWS * 2) as u64))
            .unwrap()
            .pcode
            .as_ref()
            .unwrap()
            .is_empty());
        assert_eq!(
            next.get(&((CHUNK_ROWS * 2) as u64))
                .unwrap()
                .pcode
                .as_ref()
                .unwrap()
                .len(),
            1
        );

        // Replacement in the clone detaches an untouched shared chunk, returns
        // its old row, and cannot alter the prior snapshot.
        let mut replacement = insn(CHUNK_ROWS as u64);
        replacement.operands = "replacement".to_owned();
        let mut removed = next.insert(CHUNK_ROWS as u64, replacement).unwrap();
        same(&removed, &insn(CHUNK_ROWS as u64));
        removed.flows.push(7000);
        removed.operands.push_str("_removed");
        assert!(!Rc::ptr_eq(&prior.chunks[1], &next.chunks[1]));
        assert_eq!(
            prior.get(&(CHUNK_ROWS as u64)).unwrap().operands,
            format!("target_{}", CHUNK_ROWS)
        );
        assert_eq!(
            next.get(&(CHUNK_ROWS as u64)).unwrap().operands,
            "replacement"
        );
        assert_eq!(prior.get(&(CHUNK_ROWS as u64)).unwrap().flows.len(), 1);
        assert_eq!(next.get(&(CHUNK_ROWS as u64)).unwrap().flows.len(), 1);
    }

    #[test]
    fn appending_to_a_nested_clone_detaches_only_the_partial_tail() {
        let prior: InstructionStore = (0..CHUNK_ROWS + 7)
            .map(|n| (n as u64, insn(n as u64)))
            .collect();
        let next = prior.clone();
        let mut newest = next.clone();

        assert!(Rc::ptr_eq(&prior.chunks[0], &next.chunks[0]));
        assert!(Rc::ptr_eq(&next.chunks[0], &newest.chunks[0]));
        assert!(Rc::ptr_eq(&prior.chunks[1], &newest.chunks[1]));

        for n in CHUNK_ROWS + 7..CHUNK_ROWS * 3 + 5 {
            newest.insert(n as u64, insn(n as u64));
        }

        assert!(Rc::ptr_eq(&prior.chunks[0], &newest.chunks[0]));
        assert!(!Rc::ptr_eq(&prior.chunks[1], &newest.chunks[1]));
        same(prior.get(&1).unwrap(), &insn(1));
        same(next.get(&1).unwrap(), &insn(1));
        assert_eq!(prior.len(), CHUNK_ROWS + 7);
        assert_eq!(next.len(), CHUNK_ROWS + 7);
        assert_eq!(newest.len(), CHUNK_ROWS * 3 + 5);
        assert!(prior.get(&((CHUNK_ROWS * 3) as u64)).is_none());
        same(
            newest.get(&((CHUNK_ROWS * 3) as u64)).unwrap(),
            &insn((CHUNK_ROWS * 3) as u64),
        );
        let capacity: usize = newest
            .chunks
            .iter()
            .map(|chunk| chunk.rows.capacity())
            .sum();
        assert!(capacity - newest.len() < CHUNK_ROWS);
    }
}
