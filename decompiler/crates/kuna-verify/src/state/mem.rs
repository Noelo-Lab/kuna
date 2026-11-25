//! Memory by 64-byte copy-on-write pages, temporaries by value, and the hasher both use.

use super::*;

/// The frame bytes a callee run for real reads.
pub type FrameReads = HashSet<u64, BuildHasherDefault<Fast>>;

/// Memory this side wrote, in 64-byte pages each with a mask of the bytes set.
/// Every reader that needs an order collects into a sorted map, so the pages
/// need none.
#[derive(Clone, Default)]
pub(super) struct Mem {
    /// Shared until written: cloning a state to run a callee for real copies
    /// the page pointers, not the pages.
    pub(super) pages: HashMap<(i32, u64), Rc<Page>, BuildHasherDefault<Fast>>,
}

#[derive(Clone)]
pub(super) struct Page {
    pub(super) bytes: [u8; 64],
    pub(super) set: u64,
}

impl Mem {
    pub(super) fn get(&self, sp: i32, off: u64) -> Option<u8> {
        let p = self.pages.get(&(sp, off >> 6))?;
        let i = off & 63;
        (p.set >> i & 1 == 1).then(|| p.bytes[i as usize])
    }

    pub(super) fn contains(&self, sp: i32, off: u64) -> bool {
        self.get(sp, off).is_some()
    }

    pub(super) fn insert(&mut self, sp: i32, off: u64, b: u8) {
        let p = Rc::make_mut(self.pages.entry((sp, off >> 6)).or_insert_with(|| Rc::new(Page { bytes: [0; 64], set: 0 })));
        let i = off & 63;
        p.bytes[i as usize] = b;
        p.set |= 1 << i;
    }

    /// Every byte, in no order.
    pub(super) fn iter(&self) -> impl Iterator<Item = ((i32, u64), u8)> + '_ {
        self.pages.iter().flat_map(|(&(sp, page), p)| {
            (0..64u64).filter(move |i| p.set >> i & 1 == 1).map(move |i| ((sp, page << 6 | i), p.bytes[i as usize]))
        })
    }

    /// The bytes of `sp` in `[start, end)`, in no order: the range's pages
    /// looked up, or every page scanned when there are fewer of those.
    pub(super) fn range(&self, sp: i32, start: u64, end: u64) -> Vec<(u64, u8)> {
        if end <= start {
            return Vec::new();
        }
        let (first, last) = (start >> 6, (end - 1) >> 6);
        let inside = |off: u64| off >= start && off < end;
        if last - first < self.pages.len() as u64 {
            (first..=last)
                .filter_map(|page| self.pages.get(&(sp, page)).map(|p| (page, p)))
                .flat_map(|(page, p)| (0..64u64).filter(move |i| p.set >> i & 1 == 1).map(move |i| (page << 6 | i, p.bytes[i as usize])))
                .filter(|&(off, _)| inside(off))
                .collect()
        } else {
            self.iter().filter(|&((s, off), _)| s == sp && inside(off)).map(|((_, off), b)| (off, b)).collect()
        }
    }

    pub(super) fn retain(&mut self, mut keep: impl FnMut(i32, u64) -> bool) {
        self.pages.retain(|&(sp, page), p| {
            let gone = (0..64u64).filter(|&i| p.set >> i & 1 == 1 && !keep(sp, page << 6 | i)).fold(0u64, |m, i| m | 1 << i);
            if gone != 0 {
                Rc::make_mut(p).set &= !gone;
            }
            p.set != 0
        });
    }
}

/// The temporaries of the current instruction, one entry per value written:
/// p-code reads a temporary as it wrote it, so a read is usually one lookup.
#[derive(Clone, Default)]
pub(super) struct Temps {
    pub(super) entries: Vec<(u64, u32, u128, bool)>,
}

impl Temps {
    pub(super) fn exact(&self, off: u64, size: u32) -> Option<u128> {
        self.entries.iter().rev().find(|e| e.0 == off && e.1 == size).map(|e| e.2)
    }

    pub(super) fn byte(&self, off: u64) -> Option<u8> {
        self.entries.iter().rev().find(|e| off >= e.0 && off - e.0 < u64::from(e.1)).map(|&(at, size, v, big)| {
            let i = off - at;
            let shift = if big { u64::from(size) - 1 - i } else { i };
            (v >> (shift * 8)) as u8
        })
    }

    pub(super) fn set(&mut self, off: u64, size: u32, v: u128, big: bool) {
        let end = off + u64::from(size);
        if self.entries.iter().any(|e| e.0 < end && off < e.0 + u64::from(e.1)) {
            let old = std::mem::take(&mut self.entries);
            for (at, n, val, b) in old {
                if at < end && off < at + u64::from(n) {
                    for i in 0..u64::from(n) {
                        let o = at + i;
                        if o < off || o >= end {
                            let shift = if b { u64::from(n) - 1 - i } else { i };
                            self.entries.push((o, 1, (val >> (shift * 8)) & 0xff, b));
                        }
                    }
                } else {
                    self.entries.push((at, n, val, b));
                }
            }
        }
        self.entries.push((off, size, v & crate::value::mask(size), big));
    }
}

/// A multiplicative hasher for the temporaries' byte offsets: they are
/// cleared every instruction, so the map keeps its capacity and its hashing
/// costs one multiply.
#[derive(Default, Clone, Copy)]
pub struct Fast(u64);

impl std::hash::Hasher for Fast {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x51_7cc1_b727_220a_95);
        }
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7cc1_b727_220a_95);
    }
}
