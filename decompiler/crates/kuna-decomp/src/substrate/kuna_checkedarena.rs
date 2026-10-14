//! SlotMap storage with compact, inlineable live-id validation in every build.
//!
//! Only this wrapper changes slot membership. Cached full FFI encodings identify
//! live keys; removal and clear invalidate them before changing storage. The
//! vacancy marker's index is never allocated, so it cannot match an in-bounds key.
//! SlotMap additionally checks the original generation on reads in debug builds.

use slotmap::{Key, SlotMap};
use std::ops::{Index, IndexMut};

const VACANT: u64 = u64::MAX;

pub(crate) struct Arena<K: Key, V> {
    values: SlotMap<K, V>,
    keys: Vec<u64>,
}

/// The locked SlotMap version encodes the slot index in the low 32 bits.
#[inline(always)]
fn index(encoded: u64) -> usize {
    encoded as u32 as usize
}

impl<K: Key, V> Arena<K, V> {
    pub(crate) fn with_key() -> Self {
        Self {
            values: SlotMap::with_key(),
            keys: Vec::new(),
        }
    }

    pub(crate) fn insert(&mut self, value: V) -> K {
        let id = self.values.insert(value);
        let encoded = id.data().as_ffi();
        let slot = index(encoded);
        assert!(slot < u32::MAX as usize, "SlotMap key encoding changed");
        if self.keys.len() <= slot {
            self.keys.resize(slot + 1, VACANT);
        }
        assert_eq!(self.keys[slot], VACANT, "SlotMap key encoding changed");
        self.keys[slot] = encoded;
        id
    }

    pub(crate) fn remove(&mut self, id: K) -> Option<V> {
        let encoded = id.data().as_ffi();
        let key = self.keys.get_mut(index(encoded))?;
        if *key != encoded {
            return None;
        }
        *key = VACANT;
        self.values.remove(id)
    }

    pub(crate) fn clear(&mut self) {
        self.keys.fill(VACANT);
        self.values.clear();
    }

    #[inline(always)]
    pub(crate) fn get(&self, id: K) -> Option<&V> {
        let encoded = id.data().as_ffi();
        if self.keys.get(index(encoded)) != Some(&encoded) {
            return None;
        }
        Some(unsafe { self.values.get_unchecked(id) })
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, id: K) -> Option<&mut V> {
        let encoded = id.data().as_ffi();
        if self.keys.get(index(encoded)) != Some(&encoded) {
            return None;
        }
        Some(unsafe { self.values.get_unchecked_mut(id) })
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.values.iter()
    }
}

impl<K: Key, V> Index<K> for Arena<K, V> {
    type Output = V;

    #[inline(always)]
    fn index(&self, id: K) -> &V {
        self.get(id).expect("invalid SlotMap key used")
    }
}

impl<K: Key, V> IndexMut<K> for Arena<K, V> {
    #[inline(always)]
    fn index_mut(&mut self, id: K) -> &mut V {
        self.get_mut(id).expect("invalid SlotMap key used")
    }
}

#[cfg(test)]
mod tests;
