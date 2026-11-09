//! Port of `org/jetbrains/java/decompiler/util/collections/PackedMap.java`.

use crate::jthrow;
use crate::rt::{JHashMap, JavaEq, JavaHash};

/// Keys mapped to packed `(high, low)` int pairs, for `FastSparseSetFactory`.
pub struct PackedMap<K> {
    values: Vec<i64>,
    keys: Vec<Option<K>>,
    map_keys: JHashMap<K, i32>,
    size: i32,
}

impl<K: JavaHash + JavaEq + Clone> Default for PackedMap<K> {
    fn default() -> Self {
        PackedMap::new()
    }
}

impl<K: JavaHash + JavaEq + Clone> PackedMap<K> {
    pub fn new() -> Self {
        PackedMap {
            values: vec![0; 16],
            keys: vec![None; 16],
            map_keys: JHashMap::new(),
            size: 0,
        }
    }

    /// Java: `putWithKey(int high, int low, K key)`; returns the packed value.
    pub fn put_with_key_packed(&mut self, high: i32, low: i32, key: K) -> i64 {
        let packed = PackedMap::<K>::pack(high, low);
        self.put_with_key(packed, key);
        packed
    }

    pub fn put_with_key(&mut self, element: i64, key: K) {
        match self.map_keys.get(&key).copied() {
            Some(index) => self.values[index as usize] = element,
            None => {
                let index = self.size;
                if index as usize >= self.values.len() {
                    self.resize();
                }
                self.values[index as usize] = element;
                self.keys[index as usize] = Some(key.clone());
                self.map_keys.put(key, index);
                self.size += 1;
            }
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map_keys.contains_key(key)
    }

    pub fn get_with_key(&self, key: &K) -> i64 {
        match self.map_keys.get(key) {
            Some(&index) => self.values[index as usize],
            None => jthrow!(IllegalArgumentException;
                "Key not found in key map. Use containsKey to check if it exists first!"),
        }
    }

    pub fn get(&self, index: i32) -> i64 {
        self.values[index as usize]
    }

    pub fn get_key(&self, index: i32) -> Option<K> {
        self.keys[index as usize].clone()
    }

    fn resize(&mut self) {
        let size = self.size as usize;
        let new_len = size + size / 2;
        self.values.truncate(size);
        self.values.resize(new_len, 0);
        self.keys.truncate(size);
        self.keys.resize(new_len, None);
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

impl<K> PackedMap<K> {
    pub fn pack(high: i32, low: i32) -> i64 {
        (low as i64 & 0xffff_ffff) | ((high as i64 & 0xffff_ffff) << 32)
    }

    pub fn unpack_low(packed: i64) -> i32 {
        (packed & 0xffff_ffff) as i32
    }

    pub fn unpack_high(packed: i64) -> i32 {
        (((packed as u64) >> 32) & 0xffff_ffff) as i32
    }
}
