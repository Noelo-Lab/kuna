//! Slot storage with compact, inlineable live-id validation in every build.
//!
//! Each slot stores its value followed by a full 64-bit key word. An occupied
//! slot's word is the FFI encoding of its live id, so a lookup compares one word
//! that sits beside the value's trailing fields instead of reading a separate
//! array. A vacant slot's word holds an even version and the next free index;
//! that index never names the slot itself, so no id can match a vacancy. The
//! sentinel slot's word is zero, which no id encodes. Allocation, free-list
//! order and versions replicate `slotmap::SlotMap`, so ids are unchanged.

use slotmap::{Key, KeyData};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

#[repr(C)]
struct Slot<V> {
    value: MaybeUninit<V>,
    key: u64,
}

impl<V> Slot<V> {
    #[inline(always)]
    fn occupied(&self) -> bool {
        (self.key >> 32) & 1 == 1
    }
}

pub(crate) struct Arena<K: Key, V> {
    slots: Vec<Slot<V>>,
    free_head: u32,
    _k: PhantomData<fn(K) -> K>,
}

#[inline(always)]
fn encode(version: u32, low: u32) -> u64 {
    (u64::from(version) << 32) | u64::from(low)
}

impl<K: Key, V> Arena<K, V> {
    pub(crate) fn with_key() -> Self {
        Self {
            slots: vec![Slot {
                value: MaybeUninit::uninit(),
                key: 0,
            }],
            free_head: 1,
            _k: PhantomData,
        }
    }

    pub(crate) fn insert(&mut self, value: V) -> K {
        let idx = self.free_head;
        if let Some(slot) = self.slots.get_mut(idx as usize) {
            let next = slot.key as u32;
            let key = encode((slot.key >> 32) as u32 | 1, idx);
            slot.value.write(value);
            slot.key = key;
            self.free_head = next;
            return KeyData::from_ffi(key).into();
        }
        assert!(self.slots.len() < u32::MAX as usize, "SlotMap is full");
        let idx = self.slots.len() as u32;
        let key = encode(1, idx);
        self.slots.push(Slot {
            value: MaybeUninit::new(value),
            key,
        });
        self.free_head = idx + 1;
        KeyData::from_ffi(key).into()
    }

    fn take(&mut self, idx: usize) -> V {
        let free_head = self.free_head;
        let slot = &mut self.slots[idx];
        debug_assert!(slot.occupied());
        let value = unsafe { slot.value.assume_init_read() };
        slot.key = encode(((slot.key >> 32) as u32).wrapping_add(1), free_head);
        self.free_head = idx as u32;
        value
    }

    pub(crate) fn remove(&mut self, id: K) -> Option<V> {
        let encoded = id.data().as_ffi();
        let idx = encoded as u32 as usize;
        if self.slots.get(idx)?.key != encoded {
            return None;
        }
        Some(self.take(idx))
    }

    pub(crate) fn clear(&mut self) {
        for idx in 1..self.slots.len() {
            if self.slots[idx].occupied() {
                drop(self.take(idx));
            }
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, id: K) -> Option<&V> {
        let encoded = id.data().as_ffi();
        let slot = self.slots.get(encoded as u32 as usize)?;
        if slot.key != encoded {
            return None;
        }
        Some(unsafe { slot.value.assume_init_ref() })
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, id: K) -> Option<&mut V> {
        let encoded = id.data().as_ffi();
        let slot = self.slots.get_mut(encoded as u32 as usize)?;
        if slot.key != encoded {
            return None;
        }
        Some(unsafe { slot.value.assume_init_mut() })
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.slots
            .iter()
            .filter(|slot| slot.occupied())
            .map(|slot| {
                (KeyData::from_ffi(slot.key).into(), unsafe {
                    slot.value.assume_init_ref()
                })
            })
    }
}

impl<K: Key, V> Drop for Arena<K, V> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<V>() {
            for slot in &mut self.slots {
                if slot.occupied() {
                    unsafe { slot.value.assume_init_drop() };
                }
            }
        }
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
