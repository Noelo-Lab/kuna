//! `java.util.HashMap`/`HashSet`/`LinkedHashMap`/`LinkedHashSet` with the JDK's
//! exact iteration order.
//!
//! Vineflower iterates hash collections in places where the order reaches the
//! output, so the port reproduces the JDK 21 table: spread hash, power-of-two
//! buckets, tail insertion, order-preserving split on resize, the early resize
//! a long chain forces below 64 buckets, and capacity retained across `clear`.
//! Treeified bins are not modelled (a treeified bin only reorders nine or more
//! keys colliding in one bucket of a table of 64 or more).

use std::rc::Rc;

/// Java `hashCode()`.
pub trait JavaHash {
    fn java_hash(&self) -> i32;
}

/// Java `equals()`.
pub trait JavaEq {
    fn java_equals(&self, other: &Self) -> bool;
}

macro_rules! int_hash {
    ($($t:ty),*) => {$(
        impl JavaHash for $t {
            fn java_hash(&self) -> i32 { *self as i32 }
        }
        impl JavaEq for $t {
            fn java_equals(&self, other: &Self) -> bool { self == other }
        }
    )*};
}
int_hash!(i8, i16, i32, u8, u16, u32, usize);

impl JavaHash for i64 {
    fn java_hash(&self) -> i32 {
        (*self ^ ((*self as u64) >> 32) as i64) as i32
    }
}
impl JavaEq for i64 {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for bool {
    fn java_hash(&self) -> i32 {
        if *self {
            1231
        } else {
            1237
        }
    }
}
impl JavaEq for bool {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for char {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}
impl JavaEq for char {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for str {
    fn java_hash(&self) -> i32 {
        super::jstring::java_string_hash(self)
    }
}
impl JavaEq for str {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
impl JavaHash for String {
    fn java_hash(&self) -> i32 {
        super::jstring::java_string_hash(self)
    }
}
impl JavaEq for String {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
impl JavaHash for &str {
    fn java_hash(&self) -> i32 {
        super::jstring::java_string_hash(self)
    }
}
impl JavaEq for &str {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
impl JavaHash for Rc<str> {
    fn java_hash(&self) -> i32 {
        super::jstring::java_string_hash(self)
    }
}
impl JavaEq for Rc<str> {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T: JavaHash> JavaHash for Option<T> {
    fn java_hash(&self) -> i32 {
        self.as_ref().map_or(0, JavaHash::java_hash)
    }
}
impl<T: JavaEq> JavaEq for Option<T> {
    fn java_equals(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.java_equals(b),
            _ => false,
        }
    }
}

/// `List.hashCode()`.
impl<T: JavaHash> JavaHash for Vec<T> {
    fn java_hash(&self) -> i32 {
        self.iter()
            .fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(e.java_hash()))
    }
}
impl<T: JavaEq> JavaEq for Vec<T> {
    fn java_equals(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other).all(|(a, b)| a.java_equals(b))
    }
}

impl<A: JavaHash, B: JavaHash> JavaHash for (A, B) {
    fn java_hash(&self) -> i32 {
        objects_hash(&[self.0.java_hash(), self.1.java_hash()])
    }
}
impl<A: JavaEq, B: JavaEq> JavaEq for (A, B) {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.java_equals(&other.0) && self.1.java_equals(&other.1)
    }
}

/// `Objects.hash(a, b, ...)` over already-computed element hashes.
pub fn objects_hash(hashes: &[i32]) -> i32 {
    hashes
        .iter()
        .fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(*e))
}

/// `Objects.equals(a, b)`.
pub fn objects_equals<T: JavaEq>(a: &Option<T>, b: &Option<T>) -> bool {
    a.java_equals(b)
}

const NIL: u32 = u32::MAX;
const TREEIFY_THRESHOLD: usize = 8;
const MIN_TREEIFY_CAPACITY: usize = 64;

#[derive(Clone)]
struct Node<K, V> {
    hash: i32,
    key: K,
    value: V,
    next: u32,
    before: u32,
    after: u32,
}

#[derive(Clone)]
struct Core<K, V> {
    table: Vec<u32>,
    nodes: Vec<Option<Node<K, V>>>,
    free: Vec<u32>,
    size: usize,
    threshold: usize,
    head: u32,
    tail: u32,
}

fn spread(h: i32) -> i32 {
    h ^ ((h as u32) >> 16) as i32
}

fn table_size_for(cap: usize) -> usize {
    cap.max(1).next_power_of_two()
}

impl<K: JavaHash + JavaEq, V> Core<K, V> {
    const fn new() -> Self {
        Core {
            table: Vec::new(),
            nodes: Vec::new(),
            free: Vec::new(),
            size: 0,
            threshold: 0,
            head: NIL,
            tail: NIL,
        }
    }

    fn with_capacity(initial_capacity: usize) -> Self {
        let mut c = Self::new();
        c.threshold = table_size_for(initial_capacity);
        c
    }

    fn node(&self, idx: u32) -> &Node<K, V> {
        self.nodes[idx as usize].as_ref().expect("live node")
    }

    fn node_mut(&mut self, idx: u32) -> &mut Node<K, V> {
        self.nodes[idx as usize].as_mut().expect("live node")
    }

    fn bucket_of(&self, hash: i32) -> usize {
        (self.table.len() - 1) & (hash as u32 as usize)
    }

    fn find(&self, key: &K) -> Option<u32> {
        if self.table.is_empty() {
            return None;
        }
        let hash = spread(key.java_hash());
        let mut idx = self.table[self.bucket_of(hash)];
        while idx != NIL {
            let n = self.node(idx);
            if n.hash == hash && n.key.java_equals(key) {
                return Some(idx);
            }
            idx = n.next;
        }
        None
    }

    fn resize(&mut self) {
        let old_cap = self.table.len();
        let old_thr = self.threshold;
        let new_cap;
        let mut new_thr = 0;
        if old_cap > 0 {
            new_cap = old_cap * 2;
            if old_cap >= 16 {
                new_thr = old_thr * 2;
            }
        } else if old_thr > 0 {
            new_cap = old_thr;
        } else {
            new_cap = 16;
            new_thr = 12;
        }
        if new_thr == 0 {
            new_thr = (new_cap as f32 * 0.75) as usize;
        }
        self.threshold = new_thr;
        let old = std::mem::replace(&mut self.table, vec![NIL; new_cap]);
        for (j, &first) in old.iter().enumerate() {
            let (mut lo_head, mut lo_tail, mut hi_head, mut hi_tail) = (NIL, NIL, NIL, NIL);
            let mut idx = first;
            while idx != NIL {
                let n = self.node(idx);
                let next = n.next;
                let hi = (n.hash as u32 as usize) & old_cap != 0;
                let (head, tail) = if hi {
                    (&mut hi_head, &mut hi_tail)
                } else {
                    (&mut lo_head, &mut lo_tail)
                };
                if *tail == NIL {
                    *head = idx;
                } else {
                    let t = *tail;
                    self.node_mut(t).next = idx;
                }
                *tail = idx;
                idx = next;
            }
            if lo_tail != NIL {
                self.node_mut(lo_tail).next = NIL;
                self.table[j] = lo_head;
            }
            if hi_tail != NIL {
                self.node_mut(hi_tail).next = NIL;
                self.table[j + old_cap] = hi_head;
            }
        }
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        if let Some(idx) = self.find(&key) {
            return Some(std::mem::replace(&mut self.node_mut(idx).value, value));
        }
        self.insert_new(key, value);
        None
    }

    /// Append a key known to be absent; returns its node index.
    fn insert_new(&mut self, key: K, value: V) -> u32 {
        if self.table.is_empty() {
            self.resize();
        }
        let hash = spread(key.java_hash());
        let node = Node {
            hash,
            key,
            value,
            next: NIL,
            before: self.tail,
            after: NIL,
        };
        let idx = match self.free.pop() {
            Some(i) => {
                self.nodes[i as usize] = Some(node);
                i
            }
            None => {
                self.nodes.push(Some(node));
                (self.nodes.len() - 1) as u32
            }
        };
        if self.tail == NIL {
            self.head = idx;
        } else {
            let tail = self.tail;
            self.node_mut(tail).after = idx;
        }
        self.tail = idx;
        let bucket = self.bucket_of(hash);
        let mut chain_before = 0;
        let mut last = self.table[bucket];
        if last == NIL {
            self.table[bucket] = idx;
        } else {
            loop {
                chain_before += 1;
                let next = self.node(last).next;
                if next == NIL {
                    break;
                }
                last = next;
            }
            self.node_mut(last).next = idx;
        }
        if chain_before >= TREEIFY_THRESHOLD && self.table.len() < MIN_TREEIFY_CAPACITY {
            self.resize();
        }
        self.size += 1;
        if self.size > self.threshold {
            self.resize();
        }
        idx
    }

    fn remove(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self.find(key)?;
        Some(self.remove_idx(idx))
    }

    fn remove_idx(&mut self, idx: u32) -> (K, V) {
        let bucket = self.bucket_of(self.node(idx).hash);
        let next = self.node(idx).next;
        if self.table[bucket] == idx {
            self.table[bucket] = next;
        } else {
            let mut prev = self.table[bucket];
            while self.node(prev).next != idx {
                prev = self.node(prev).next;
            }
            self.node_mut(prev).next = next;
        }
        let node = self.nodes[idx as usize].take().expect("live node");
        if node.before == NIL {
            self.head = node.after;
        } else {
            self.node_mut(node.before).after = node.after;
        }
        if node.after == NIL {
            self.tail = node.before;
        } else {
            self.node_mut(node.after).before = node.before;
        }
        self.free.push(idx);
        self.size -= 1;
        (node.key, node.value)
    }

    fn clear(&mut self) {
        self.table.fill(NIL);
        self.nodes.clear();
        self.free.clear();
        self.size = 0;
        self.head = NIL;
        self.tail = NIL;
    }

    /// `putMapEntries` sizing ahead of a bulk insert of `incoming` entries.
    fn presize(&mut self, incoming: usize) {
        if incoming == 0 {
            return;
        }
        if self.table.is_empty() {
            let t = (incoming as f64 / 0.75).ceil() as usize;
            if t > self.threshold {
                self.threshold = table_size_for(t);
            }
        } else {
            while incoming > self.threshold {
                self.resize();
            }
        }
    }

    fn hash_order(&self) -> OrderIter<'_, K, V> {
        OrderIter {
            core: self,
            bucket: 0,
            cur: NIL,
            linked: false,
        }
    }

    fn link_order(&self) -> OrderIter<'_, K, V> {
        OrderIter {
            core: self,
            bucket: 0,
            cur: self.head,
            linked: true,
        }
    }

    fn order_vec(&self, linked: bool) -> Vec<u32> {
        if linked {
            self.link_order().collect()
        } else {
            self.hash_order().collect()
        }
    }
}

struct OrderIter<'a, K, V> {
    core: &'a Core<K, V>,
    bucket: usize,
    cur: u32,
    linked: bool,
}

impl<K, V> Iterator for OrderIter<'_, K, V> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.linked {
            if self.cur == NIL {
                return None;
            }
            let idx = self.cur;
            self.cur = self.core.nodes[idx as usize]
                .as_ref()
                .expect("live node")
                .after;
            return Some(idx);
        }
        loop {
            if self.cur != NIL {
                let idx = self.cur;
                self.cur = self.core.nodes[idx as usize]
                    .as_ref()
                    .expect("live node")
                    .next;
                return Some(idx);
            }
            if self.bucket >= self.core.table.len() {
                return None;
            }
            self.cur = self.core.table[self.bucket];
            self.bucket += 1;
        }
    }
}

macro_rules! define_map {
    ($(#[$meta:meta])* $name:ident, $order:ident, $linked:expr) => {
        $(#[$meta])*
        pub struct $name<K, V> {
            core: Core<K, V>,
        }

        /// `new HashMap<>(other)`: re-inserted in iteration order, not a table copy.
        impl<K: JavaHash + JavaEq + Clone, V: Clone> Clone for $name<K, V> {
            fn clone(&self) -> Self {
                Self::from_entries(self.entry_vec())
            }
        }

        impl<K: JavaHash + JavaEq + ::std::fmt::Debug, V: ::std::fmt::Debug> ::std::fmt::Debug for $name<K, V> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_map().entries(self.iter()).finish()
            }
        }

        impl<K: JavaHash + JavaEq, V> Default for $name<K, V> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<K: JavaHash + JavaEq, V> $name<K, V> {
            pub const fn new() -> Self {
                $name { core: Core::new() }
            }

            /// `new HashMap<>(initialCapacity)`.
            pub fn with_capacity(initial_capacity: usize) -> Self {
                $name { core: Core::with_capacity(initial_capacity) }
            }

            pub fn len(&self) -> usize {
                self.core.size
            }

            pub fn is_empty(&self) -> bool {
                self.core.size == 0
            }

            pub fn contains_key(&self, key: &K) -> bool {
                self.core.find(key).is_some()
            }

            pub fn get(&self, key: &K) -> Option<&V> {
                self.core.find(key).map(|i| &self.core.node(i).value)
            }

            pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
                self.core.find(key).map(|i| &mut self.core.node_mut(i).value)
            }

            /// `put`: returns the previous value. Re-putting a key keeps its position.
            pub fn put(&mut self, key: K, value: V) -> Option<V> {
                self.core.put(key, value)
            }

            pub fn put_if_absent(&mut self, key: K, value: V) -> bool {
                if self.core.find(&key).is_some() {
                    return false;
                }
                self.core.put(key, value);
                true
            }

            /// `computeIfAbsent`.
            pub fn compute_if_absent(&mut self, key: K, make: impl FnOnce() -> V) -> &mut V {
                let idx = match self.core.find(&key) {
                    Some(i) => i,
                    None => self.core.insert_new(key, make()),
                };
                &mut self.core.node_mut(idx).value
            }

            pub fn remove(&mut self, key: &K) -> Option<V> {
                self.core.remove(key).map(|(_, v)| v)
            }

            pub fn clear(&mut self) {
                self.core.clear()
            }

            pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
                self.core.$order().map(move |i| {
                    let n = self.core.node(i);
                    (&n.key, &n.value)
                })
            }

            pub fn keys(&self) -> impl Iterator<Item = &K> + '_ {
                self.iter().map(|(k, _)| k)
            }

            pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
                self.iter().map(|(_, v)| v)
            }

            /// Visit every value mutably, in iteration order.
            pub fn for_each_value_mut(&mut self, mut f: impl FnMut(&K, &mut V)) {
                for i in self.core.order_vec($linked) {
                    let n = self.core.node_mut(i);
                    f(&n.key, &mut n.value);
                }
            }

            /// Keep only the entries `keep` accepts (an iterator-`remove()` loop).
            pub fn retain(&mut self, mut keep: impl FnMut(&K, &mut V) -> bool) {
                for i in self.core.order_vec($linked) {
                    let n = self.core.node_mut(i);
                    if !keep(&n.key, &mut n.value) {
                        self.core.remove_idx(i);
                    }
                }
            }
        }

        impl<K: JavaHash + JavaEq + Clone, V: Clone> $name<K, V> {
            /// `new HashMap<>(other)` over any entry sequence of known length.
            pub fn from_entries<I: IntoIterator<Item = (K, V)>>(entries: I) -> Self
            where
                I::IntoIter: ExactSizeIterator,
            {
                let mut m = Self::new();
                m.put_all(entries);
                m
            }

            /// `putAll`.
            pub fn put_all<I: IntoIterator<Item = (K, V)>>(&mut self, entries: I)
            where
                I::IntoIter: ExactSizeIterator,
            {
                let it = entries.into_iter();
                self.core.presize(it.len());
                for (k, v) in it {
                    self.core.put(k, v);
                }
            }

            pub fn key_vec(&self) -> Vec<K> {
                self.keys().cloned().collect()
            }

            /// An exact copy of the table -- the state the same sequence of
            /// operations would have built -- unlike `clone`, which is Java's
            /// re-inserting copy constructor.
            pub fn duplicate(&self) -> Self {
                $name { core: self.core.clone() }
            }

            pub fn value_vec(&self) -> Vec<V> {
                self.values().cloned().collect()
            }

            pub fn entry_vec(&self) -> Vec<(K, V)> {
                self.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
            }

            pub fn get_cloned(&self, key: &K) -> Option<V> {
                self.get(key).cloned()
            }

            pub fn get_or_default(&self, key: &K, default: V) -> V {
                self.get(key).cloned().unwrap_or(default)
            }
        }
    };
}

define_map!(
    /// `java.util.HashMap`.
    JHashMap,
    hash_order,
    false
);
define_map!(
    /// `java.util.LinkedHashMap` (insertion order).
    JLinkedHashMap,
    link_order,
    true
);

macro_rules! define_set {
    ($(#[$meta:meta])* $name:ident, $order:ident, $linked:expr) => {
        $(#[$meta])*
        pub struct $name<T> {
            core: Core<T, ()>,
        }

        /// `new HashSet<>(other)`: re-inserted in iteration order, not a table copy.
        impl<T: JavaHash + JavaEq + Clone> Clone for $name<T> {
            fn clone(&self) -> Self {
                Self::from_collection(self.to_vec())
            }
        }

        impl<T: JavaHash + JavaEq + ::std::fmt::Debug> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_set().entries(self.iter()).finish()
            }
        }

        impl<T: JavaHash + JavaEq> Default for $name<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<T: JavaHash + JavaEq> $name<T> {
            pub const fn new() -> Self {
                $name { core: Core::new() }
            }

            /// `new HashSet<>(initialCapacity)`.
            pub fn with_capacity(initial_capacity: usize) -> Self {
                $name { core: Core::with_capacity(initial_capacity) }
            }

            /// `new HashSet<>(collection)`.
            pub fn from_collection<I: IntoIterator<Item = T>>(items: I) -> Self
            where
                I::IntoIter: ExactSizeIterator,
            {
                let it = items.into_iter();
                let n = it.len();
                let cap = (n.max(12) as f64 / 0.75).ceil() as usize;
                let mut s = Self::with_capacity(cap);
                for item in it {
                    s.core.put(item, ());
                }
                s
            }

            pub fn len(&self) -> usize {
                self.core.size
            }

            pub fn is_empty(&self) -> bool {
                self.core.size == 0
            }

            pub fn contains(&self, item: &T) -> bool {
                self.core.find(item).is_some()
            }

            /// `add`: true when the item was not present.
            pub fn add(&mut self, item: T) -> bool {
                if self.core.find(&item).is_some() {
                    return false;
                }
                self.core.put(item, ());
                true
            }

            pub fn remove(&mut self, item: &T) -> bool {
                self.core.remove(item).is_some()
            }

            pub fn clear(&mut self) {
                self.core.clear()
            }

            pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
                self.core.$order().map(move |i| &self.core.node(i).key)
            }

            /// Bucket count of the backing table (0 before the first insert).
            pub fn table_len(&self) -> usize {
                self.core.table.len()
            }

            /// `addAll`: true when the set changed.
            pub fn add_all<I: IntoIterator<Item = T>>(&mut self, items: I) -> bool {
                let mut changed = false;
                for item in items {
                    changed |= self.add(item);
                }
                changed
            }

            /// Keep only the items `keep` accepts.
            pub fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
                for i in self.core.order_vec($linked) {
                    if !keep(&self.core.node(i).key) {
                        self.core.remove_idx(i);
                    }
                }
            }

            /// `retainAll(other)`.
            pub fn retain_all(&mut self, other: &Self) -> bool {
                let before = self.len();
                self.retain(|x| other.contains(x));
                before != self.len()
            }

            /// `removeAll(other)`.
            pub fn remove_all(&mut self, other: &Self) -> bool {
                let before = self.len();
                self.retain(|x| !other.contains(x));
                before != self.len()
            }

            pub fn contains_all(&self, other: &Self) -> bool {
                other.iter().all(|x| self.contains(x))
            }
        }

        impl<T: JavaHash + JavaEq + Clone> $name<T> {
            pub fn to_vec(&self) -> Vec<T> {
                self.iter().cloned().collect()
            }

            /// `HashSet.clone()`, whose table sizing differs from the copy constructor's.
            pub fn java_clone(&self) -> Self {
                let mut s = Self::new();
                s.core.presize(self.len());
                for item in self.to_vec() {
                    s.core.put(item, ());
                }
                s
            }
        }

        impl<T: JavaHash + JavaEq> JavaEq for $name<T> {
            fn java_equals(&self, other: &Self) -> bool {
                self.len() == other.len() && self.contains_all(other)
            }
        }
    };
}

define_set!(
    /// `java.util.HashSet`.
    JHashSet,
    hash_order,
    false
);
define_set!(
    /// `java.util.LinkedHashSet` (insertion order).
    JLinkedHashSet,
    link_order,
    true
);
