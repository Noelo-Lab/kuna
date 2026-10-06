//! Bounded query-local reuse of context-free decodes over an immutable image.
use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};
use kuna_base::address::Address;
use crate::translate::PcodeEmit;

#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct Key(pub i32, pub u64, pub Vec<u32>);

pub(crate) struct Record {
    pub len: i32,
    pub bytes: [u8; crate::sleigh::MAX_INSTRUCTION_LEN as usize],
    pub ops: RefCell<Option<crate::sleigh::CachedPcode>>,
    pub text: RefCell<Option<(String, String)>>,
}
impl Record {
    pub fn emit(&self, addr: &Address, emit: &mut dyn PcodeEmit) {
        self.ops.borrow().as_ref().unwrap().emit(addr, emit);
    }
    fn bytes(&self, key: &Key) -> usize {
        std::mem::size_of::<Self>() + 2 * std::mem::size_of::<Key>() + 8 * key.2.capacity()
            + self.ops.borrow().as_ref().map_or(0, |ops| ops.bytes())
            + self.text.borrow().as_ref().map_or(0, |(a,b)| a.capacity() + b.capacity())
    }
}

pub(crate) struct Cache {
    limit: usize,
    bytes: usize,
    entries: HashMap<Key, Rc<Record>>,
    fifo: VecDeque<Key>,
}
impl Cache {
    fn new(limit: usize) -> Self {
        Self { limit, bytes: 0, entries: HashMap::new(), fifo: VecDeque::new() }
    }
    pub fn get(&self, key: &Key) -> Option<Rc<Record>> { self.entries.get(key).cloned() }
    pub fn set_text(&mut self, key: &Key, text: (String, String)) {
        let Some(record) = self.get(key) else { return };
        if record.text.borrow().is_some() { return; }
        let extra = text.0.capacity() + text.1.capacity();
        if record.bytes(key) + extra > self.limit { return; }
        while self.bytes + extra > self.limit {
            let old = self.fifo.pop_front().unwrap();
            let record = self.entries.remove(&old).unwrap();
            self.bytes -= record.bytes(&old);
        }
        if self.entries.contains_key(key) {
            record.text.replace(Some(text));
            self.bytes += extra;
        }
    }
    pub fn insert(&mut self, key: Key, record: Record) {
        if let Some(existing) = self.get(&key) {
            if existing.ops.borrow().is_none() {
                if let Some(ops) = record.ops.into_inner() {
                    let extra = ops.bytes();
                    if existing.bytes(&key) + extra > self.limit { return; }
                    while self.bytes + extra > self.limit {
                        let old = self.fifo.pop_front().unwrap();
                        let record = self.entries.remove(&old).unwrap();
                        self.bytes -= record.bytes(&old);
                    }
                    if self.entries.contains_key(&key) {
                        existing.ops.replace(Some(ops));
                        self.bytes += extra;
                    }
                }
            }
            return;
        }
        let bytes = record.bytes(&key);
        if bytes > self.limit { return; }
        while self.bytes + bytes > self.limit {
            let old = self.fifo.pop_front().unwrap();
            let record = self.entries.remove(&old).unwrap();
            self.bytes -= record.bytes(&old);
        }
        self.bytes += bytes;
        self.fifo.push_back(key.clone());
        self.entries.insert(key, Rc::new(record));
    }
}

/// The image and processor specification must remain unchanged while this scope
/// is active. Dropping it releases the cache; nested scopes restore their parent.
pub struct DecodeReuseScope<'a> {
    cache: &'a RefCell<Option<Cache>>,
    saved: Option<Cache>,
}
impl<'a> DecodeReuseScope<'a> {
    pub(crate) fn new(cache: &'a RefCell<Option<Cache>>, limit: usize) -> Self {
        Self { saved: cache.replace(Some(Cache::new(limit))), cache }
    }
}
impl Drop for DecodeReuseScope<'_> {
    fn drop(&mut self) { self.cache.replace(self.saved.take()); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_evicts_oldest_and_never_exceeds_payload_limit() {
        let record = || Record { len: 4, bytes: [0; crate::sleigh::MAX_INSTRUCTION_LEN as usize], ops: RefCell::new(Some(crate::sleigh::CachedPcode::default())), text: RefCell::new(None) };
        let key = |offset| Key(1, offset, vec![0]);
        let limit = 2 * record().bytes(&key(0));
        let mut cache = Cache::new(limit);
        for offset in 0..1000 {
            cache.insert(key(offset), record());
            assert!(cache.bytes <= limit);
            assert!(cache.entries.len() <= 2);
            assert_eq!(cache.entries.len(), cache.fifo.len());
        }
        assert!(cache.get(&key(997)).is_none());
        assert!(cache.get(&key(998)).is_some());
        assert!(cache.get(&key(999)).is_some());
        cache.insert(key(999), record());
        assert_eq!(cache.entries.len(), 2);
        assert_eq!(cache.bytes, limit);
        cache.set_text(&key(999), ("x".repeat(32), "y".repeat(32)));
        assert!(cache.bytes <= limit);
        assert!(cache.get(&key(998)).is_none());
        assert!(cache.get(&key(999)).unwrap().text.borrow().is_some());
    }
}
