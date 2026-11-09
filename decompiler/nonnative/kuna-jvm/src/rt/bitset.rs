//! `java.util.BitSet`.

#[derive(Clone, Default)]
pub struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    pub const fn new() -> Self {
        BitSet { words: Vec::new() }
    }

    fn trim(&mut self) {
        while self.words.last() == Some(&0) {
            self.words.pop();
        }
    }

    pub fn get(&self, bit: usize) -> bool {
        self.words
            .get(bit / 64)
            .is_some_and(|w| w & (1u64 << (bit % 64)) != 0)
    }

    pub fn set(&mut self, bit: usize) {
        let w = bit / 64;
        if w >= self.words.len() {
            self.words.resize(w + 1, 0);
        }
        self.words[w] |= 1u64 << (bit % 64);
    }

    /// `set(bit, value)`.
    pub fn set_to(&mut self, bit: usize, value: bool) {
        if value {
            self.set(bit)
        } else {
            self.clear(bit)
        }
    }

    /// `set(from, to)`: the half-open range.
    pub fn set_range(&mut self, from: usize, to: usize) {
        for bit in from..to {
            self.set(bit);
        }
    }

    pub fn clear(&mut self, bit: usize) {
        if let Some(w) = self.words.get_mut(bit / 64) {
            *w &= !(1u64 << (bit % 64));
            self.trim();
        }
    }

    pub fn clear_all(&mut self) {
        self.words.clear();
    }

    pub fn or(&mut self, other: &BitSet) {
        if other.words.len() > self.words.len() {
            self.words.resize(other.words.len(), 0);
        }
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a |= *b;
        }
    }

    pub fn and(&mut self, other: &BitSet) {
        self.words.truncate(other.words.len());
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a &= *b;
        }
        self.trim();
    }

    pub fn and_not(&mut self, other: &BitSet) {
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a &= !*b;
        }
        self.trim();
    }

    pub fn xor(&mut self, other: &BitSet) {
        if other.words.len() > self.words.len() {
            self.words.resize(other.words.len(), 0);
        }
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a ^= *b;
        }
        self.trim();
    }

    pub fn intersects(&self, other: &BitSet) -> bool {
        self.words.iter().zip(&other.words).any(|(a, b)| a & b != 0)
    }

    pub fn cardinality(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|w| *w == 0)
    }

    /// One past the highest set bit; 0 when empty.
    pub fn length(&self) -> usize {
        match self.words.iter().rposition(|w| *w != 0) {
            Some(i) => i * 64 + 64 - self.words[i].leading_zeros() as usize,
            None => 0,
        }
    }

    /// `nextSetBit(from)`: -1 when there is none.
    pub fn next_set_bit(&self, from: usize) -> i32 {
        let mut w = from / 64;
        if w >= self.words.len() {
            return -1;
        }
        let mut word = self.words[w] & (u64::MAX << (from % 64));
        loop {
            if word != 0 {
                return (w * 64 + word.trailing_zeros() as usize) as i32;
            }
            w += 1;
            if w == self.words.len() {
                return -1;
            }
            word = self.words[w];
        }
    }

    /// `nextClearBit(from)`.
    pub fn next_clear_bit(&self, from: usize) -> usize {
        let mut bit = from;
        while self.get(bit) {
            bit += 1;
        }
        bit
    }

    /// Set bits in ascending order.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.words.iter().enumerate().flat_map(|(wi, &word)| {
            let mut rest = word;
            std::iter::from_fn(move || {
                if rest == 0 {
                    return None;
                }
                let bit = rest.trailing_zeros() as usize;
                rest &= rest - 1;
                Some(wi * 64 + bit)
            })
        })
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        let n = self.words.len().max(other.words.len());
        (0..n).all(|i| {
            self.words.get(i).copied().unwrap_or(0) == other.words.get(i).copied().unwrap_or(0)
        })
    }
}
impl Eq for BitSet {}

/// `BitSet.toString()`: `{1, 5, 9}`.
impl std::fmt::Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        for (n, bit) in self.iter().enumerate() {
            if n > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{bit}")?;
        }
        f.write_str("}")
    }
}

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
