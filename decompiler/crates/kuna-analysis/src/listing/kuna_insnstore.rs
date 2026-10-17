//! Instruction records in bounded chunks, with an ordered page address index.
//! Tree nodes hold page buckets rather than large instruction payloads; each
//! bucket stores sorted addresses and row numbers. Chunks never relocate records
//! or grow a second whole-image buffer during the walk.
//!
//! Store snapshots share immutable instruction chunks. A write to a shared
//! chunk clones just that chunk; appending through a partial tail detaches that
//! tail once, while untouched full chunks remain shared.

use std::ops::RangeBounds;
use std::rc::Rc;

use super::kuna_insnindex::InstructionIndex;
use super::model::{Insn, Mnemonic, MnemonicPool};

const CHUNK_ROWS: usize = 1024;

#[derive(Default, Clone)]
pub(super) struct InstructionStore {
    index: InstructionIndex,
    chunks: Vec<Rc<InstructionChunk>>,
    mnemonics: MnemonicPool,
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
    pub(super) fn intern_mnemonic(&mut self, text: &str) -> Mnemonic {
        self.mnemonics.intern(text)
    }

    pub fn insert(&mut self, address: u64, insn: Insn) -> Option<Insn> {
        let Self { index, chunks, .. } = self;
        let mut pending = Some(insn);
        let replaced = index.insert_with(address, || {
            let slot = chunks
                .last()
                .map_or(0, |chunk| (chunks.len() - 1) * CHUNK_ROWS + chunk.rows.len());
            if slot % CHUNK_ROWS == 0 {
                chunks.push(Rc::new(InstructionChunk {
                    rows: Vec::with_capacity(CHUNK_ROWS),
                }));
            }
            Rc::make_mut(chunks.last_mut().unwrap())
                .rows
                .push(pending.take().unwrap());
            slot
        });
        replaced.map(|slot| {
            std::mem::replace(
                &mut Rc::make_mut(&mut chunks[slot / CHUNK_ROWS]).rows[slot % CHUNK_ROWS],
                pending.take().unwrap(),
            )
        })
    }

    pub fn get(&self, address: &u64) -> Option<&Insn> {
        let slot = self.index.get(*address)?;
        Some(&self.chunks[slot / CHUNK_ROWS].rows[slot % CHUNK_ROWS])
    }

    pub fn get_mut(&mut self, address: &u64) -> Option<&mut Insn> {
        let slot = self.index.get(*address)?;
        Some(&mut Rc::make_mut(&mut self.chunks[slot / CHUNK_ROWS]).rows[slot % CHUNK_ROWS])
    }

    pub fn contains_key(&self, address: &u64) -> bool {
        self.index.contains_key(*address)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &u64> {
        self.index.keys()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&u64, &Insn)> {
        let chunks = &self.chunks;
        self.index
            .iter()
            .map(move |(address, &slot)| (address, &chunks[slot / CHUNK_ROWS].rows[slot % CHUNK_ROWS]))
    }

    pub fn range(
        &self,
        bounds: impl RangeBounds<u64>,
    ) -> impl DoubleEndedIterator<Item = (&u64, &Insn)> {
        let chunks = &self.chunks;
        self.index
            .range(bounds)
            .map(move |(address, &slot)| (address, &chunks[slot / CHUNK_ROWS].rows[slot % CHUNK_ROWS]))
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
    use std::collections::BTreeMap;
    use std::ops::Bound;

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
            mnemonic: format!("op_{addr}").into(),
            operands: format!("target_{addr}"),
            pcode: Some(Box::new(vec![RawOp {
                opcode: OpCode::CPUI_RETURN,
                in0: None,
            }])),
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

    #[test]
    fn generated_sparse_page_queries_match_a_btree_map_oracle() {
        let mut addresses: Vec<_> = (0..160u64)
            .map(|n| ((n * 73 % 193) << 12) | (n * 997 % 4096))
            .collect();
        addresses.extend([0, 4095, 4096, 8191, 8192, u64::MAX - 1, u64::MAX]);

        let mut store = InstructionStore::default();
        let mut expected = BTreeMap::new();
        for (ordinal, &address) in addresses.iter().rev().enumerate() {
            let mut row = insn(address);
            row.len = (ordinal % 11 + 1) as u32;
            store.insert(address, row.clone());
            expected.insert(address, row);
        }

        // Replacements must preserve one slot and return the prior record.
        for &address in addresses.iter().step_by(9) {
            let mut replacement = insn(address);
            replacement.len = 77;
            same(
                &store.insert(address, replacement.clone()).unwrap(),
                &expected.insert(address, replacement).unwrap(),
            );
        }

        assert_eq!(store.len(), expected.len());
        assert_eq!(
            store.keys().collect::<Vec<_>>(),
            expected.keys().collect::<Vec<_>>()
        );
        for (&address, expected_row) in &expected {
            assert!(store.contains_key(&address));
            same(store.get(&address).unwrap(), expected_row);
        }
        assert!(!store.contains_key(&1));

        let alternate: InstructionStore = expected
            .iter()
            .rev()
            .map(|(&address, row)| (address, row.clone()))
            .collect();
        for ((actual_address, actual), (alternate_address, alternate_row)) in
            store.iter().zip(alternate.iter())
        {
            assert_eq!(actual_address, alternate_address);
            same(actual, alternate_row);
        }

        // Bounds come from the data and mix inclusive/exclusive edges across
        // ordinary, page-boundary, sparse, and extreme addresses.
        let probes: Vec<_> = addresses.iter().step_by(7).copied().collect();
        for (i, &left) in probes.iter().enumerate() {
            for &right in probes.iter().skip(i) {
                let (low, high) = (left.min(right), left.max(right));
                for start in [Bound::Included(low), Bound::Excluded(low)] {
                    for end in [Bound::Included(high), Bound::Excluded(high)] {
                        if low == high
                            && matches!(start, Bound::Excluded(_))
                            && matches!(end, Bound::Excluded(_))
                        {
                            continue; // BTreeMap::range also rejects this pair.
                        }
                        let got: Vec<_> = store.range((start, end)).collect();
                        let want: Vec<_> = expected.range((start, end)).collect();
                        assert_eq!(got.len(), want.len());
                        for ((actual_addr, actual), (want_addr, want)) in got.into_iter().zip(want)
                        {
                            assert_eq!(actual_addr, want_addr);
                            same(actual, want);
                        }
                    }
                }
            }
        }

        for (start, end) in [
            (Bound::Unbounded, Bound::Included(u64::MAX)),
            (Bound::Excluded(0), Bound::Unbounded),
            (Bound::Unbounded, Bound::Excluded(0)),
            (Bound::Included(4096), Bound::Excluded(4096)),
        ] {
            let got: Vec<_> = store.range((start, end)).collect();
            let want: Vec<_> = expected.range((start, end)).collect();
            assert_eq!(got.len(), want.len());
            for ((actual_addr, actual), (want_addr, want)) in got.into_iter().zip(want) {
                assert_eq!(actual_addr, want_addr);
                same(actual, want);
            }
        }
        assert_eq!(
            store
                .range(..)
                .rev()
                .map(|(address, _)| address)
                .collect::<Vec<_>>(),
            expected.keys().rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn mutable_instruction_address_does_not_change_its_indexed_key() {
        let mut store: InstructionStore = [4095, 4096, 8192]
            .into_iter()
            .map(|address| (address, insn(address)))
            .collect();

        store.get_mut(&4096).unwrap().addr = u64::MAX;

        assert_eq!(store.get(&4096).unwrap().addr, u64::MAX);
        assert!(store.get(&u64::MAX).is_none());
        assert_eq!(
            store.keys().copied().collect::<Vec<_>>(),
            [4095, 4096, 8192]
        );
        assert_eq!(
            store
                .range(..)
                .map(|(address, _)| *address)
                .collect::<Vec<_>>(),
            [4095, 4096, 8192]
        );
    }

    #[test]
    fn bounded_reverse_range_crosses_page_boundaries() {
        let addresses = [4094, 4095, 4096, 8191, 8192, 8193];
        let store: InstructionStore = addresses
            .iter()
            .rev()
            .map(|&address| (address, insn(address)))
            .collect();
        let expected: BTreeMap<_, _> = addresses
            .iter()
            .map(|&address| (address, insn(address)))
            .collect();
        let bounds = (Bound::Included(4095), Bound::Excluded(8193));

        let got = store
            .range(bounds)
            .rev()
            .map(|(address, _)| *address)
            .collect::<Vec<_>>();
        let want = expected
            .range(bounds)
            .rev()
            .map(|(&address, _)| address)
            .collect::<Vec<_>>();

        assert_eq!(got, want);
        assert_eq!(got, [8192, 8191, 4096, 4095]);
    }

    #[test]
    fn range_bound_validation_matches_btree_map_on_empty_and_populated_stores() {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        let invalid_bounds = [
            (Bound::Included(8), Bound::Included(3)),
            (Bound::Included(8), Bound::Excluded(3)),
            (Bound::Excluded(8), Bound::Included(3)),
            (Bound::Excluded(8), Bound::Excluded(3)),
            (Bound::Excluded(5), Bound::Excluded(5)),
        ];
        let valid_empty_bounds = [
            (Bound::Included(5), Bound::Excluded(5)),
            (Bound::Excluded(5), Bound::Included(5)),
        ];

        for populated in [false, true] {
            let mut store = InstructionStore::default();
            let mut expected = BTreeMap::new();
            if populated {
                for address in [1, 5, 9] {
                    store.insert(address, insn(address));
                    expected.insert(address, insn(address));
                }
            }

            for bounds in invalid_bounds {
                let btree_panics = catch_unwind(AssertUnwindSafe(|| {
                    let _ = expected.range(bounds).next();
                }))
                .is_err();
                let store_panics = catch_unwind(AssertUnwindSafe(|| {
                    let _ = store.range(bounds).next();
                }))
                .is_err();
                assert_eq!(
                    store_panics, btree_panics,
                    "range behavior differs for populated={populated}, bounds={bounds:?}"
                );
            }

            for bounds in valid_empty_bounds {
                assert!(expected.range(bounds).next().is_none());
                assert!(store.range(bounds).next().is_none());
            }
        }
    }
}
