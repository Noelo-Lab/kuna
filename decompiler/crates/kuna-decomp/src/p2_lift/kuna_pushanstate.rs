//! (kuna pushan) The abstract machine state the devirtualizer emulates over.
//!
//! Registers, SLEIGH temporaries and memory are byte-granular stores of
//! `(expression, byte index)` cells, so overlapping x86 sub-registers and
//! mixed-width stack accesses compose and slice exactly.  Memory is split into
//! regions by the symbolic base of the address (`None` for concrete
//! addresses): an access at `base + offset` touches only its own region, which
//! is the "pointers to distinct symbolic bases never alias" assumption the
//! paper's stack handling relies on.  Unwritten concrete memory reads the
//! program image; unwritten symbolic memory and registers read a stable
//! symbolic variable, so the same location is the same symbol on every path.

use std::collections::BTreeMap;

use super::kuna_pushanexpr::{ExprArena, ExprId, VarId, VarKind, MAX_SIZE};

/// Byte `idx` of value `expr`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ByteVal {
    pub expr: ExprId,
    pub idx: u8,
}

/// Where the emulator reads the bytes of the binary.
pub trait PushanImage {
    /// Fill `buf` from `addr`; false when the first byte is unmapped.
    fn read_bytes(&self, addr: u64, buf: &mut [u8]) -> bool;
    /// Whether `addr` lies in a section the program may write at run time.
    fn is_writable(&self, addr: u64) -> bool;
}

/// A byte-granular store (one register file, one temporary file, or one memory region).
#[derive(Clone, Default, Debug)]
pub struct ByteStore {
    bytes: BTreeMap<i128, ByteVal>,
}

/// A maximal run of consecutive bytes that slice one value, or a gap.
struct Run {
    off: i128,
    len: u8,
    src: Option<(ExprId, u8)>,
}

impl ByteStore {
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn write(&mut self, off: i128, e: ExprId, size: u8, big_endian: bool) {
        for i in 0..size {
            let idx = if big_endian { size - 1 - i } else { i };
            self.bytes.insert(off + i as i128, ByteVal { expr: e, idx });
        }
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn remove(&mut self, off: i128, size: u8) {
        for i in 0..size {
            self.bytes.remove(&(off + i as i128));
        }
    }

    pub fn has_any(&self, off: i128, size: u8) -> bool {
        (0..size).any(|i| self.bytes.contains_key(&(off + i as i128)))
    }

    fn runs(&self, off: i128, size: u8, big_endian: bool) -> Vec<Run> {
        let mut runs: Vec<Run> = Vec::new();
        for i in 0..size {
            let o = off + i as i128;
            let cell = self.bytes.get(&o).copied();
            if let Some(last) = runs.last_mut() {
                let extends = match (last.src, cell) {
                    (None, None) => true,
                    (Some((e, idx)), Some(c)) if c.expr == e => {
                        let expect = if big_endian { idx.checked_sub(last.len) } else { idx.checked_add(last.len) };
                        expect == Some(c.idx)
                    }
                    _ => false,
                };
                if extends {
                    last.len += 1;
                    continue;
                }
            }
            runs.push(Run { off: o, len: 1, src: cell.map(|c| (c.expr, c.idx)) });
        }
        runs
    }

    /// Read `size` bytes at `off`, composing the stored slices.  Gaps are
    /// filled by `fill(offset, length)`, whose value is stored so later reads
    /// of the same bytes agree.
    pub fn read_with(
        &mut self,
        arena: &mut ExprArena,
        off: i128,
        size: u8,
        big_endian: bool,
        fill: &mut dyn FnMut(&mut ExprArena, i128, u8) -> ExprId,
    ) -> ExprId {
        let runs = self.runs(off, size, big_endian);
        if runs.len() == 1 {
            if let Some((e, idx)) = runs[0].src {
                let start = if big_endian { idx + 1 - size } else { idx };
                if start == 0 && arena.size(e) == size {
                    return e;
                }
                return arena.mk_subpiece(e, start, size);
            }
        }
        let mut pieces: Vec<ExprId> = Vec::with_capacity(runs.len());
        for r in &runs {
            let piece = match r.src {
                Some((e, idx)) => {
                    let start = if big_endian { idx + 1 - r.len } else { idx };
                    arena.mk_subpiece(e, start, r.len)
                }
                None => {
                    let v = fill(arena, r.off, r.len);
                    self.write(r.off, v, r.len, big_endian);
                    v
                }
            };
            pieces.push(piece);
        }
        // Little-endian: the lowest offset is the least significant piece.
        let mut acc = if big_endian { pieces[0] } else { pieces[0] };
        if big_endian {
            for p in &pieces[1..] {
                acc = arena.mk_piece(acc, *p);
            }
        } else {
            for p in &pieces[1..] {
                acc = arena.mk_piece(*p, acc);
            }
        }
        acc
    }

    /// Byte offsets where the two stores disagree, as ranges.
    pub fn diff_ranges(&self, other: &ByteStore) -> Vec<(i128, u8)> {
        let mut offs: Vec<i128> = Vec::new();
        for (o, v) in &self.bytes {
            if other.bytes.get(o) != Some(v) {
                offs.push(*o);
            }
        }
        for o in other.bytes.keys() {
            if !self.bytes.contains_key(o) {
                offs.push(*o);
            }
        }
        offs.sort_unstable();
        offs.dedup();
        let mut out: Vec<(i128, u8)> = Vec::new();
        for o in offs {
            if let Some(last) = out.last_mut() {
                if last.0 + last.1 as i128 == o && last.1 < MAX_SIZE {
                    last.1 += 1;
                    continue;
                }
            }
            out.push((o, 1));
        }
        out
    }

    pub fn offsets(&self) -> impl Iterator<Item = &i128> {
        self.bytes.keys()
    }
}

/// A location a merge symbolized.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Loc {
    Reg { off: u64 },
    Mem { base: Option<VarId>, off: i128 },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SymLoc {
    pub loc: Loc,
    pub size: u8,
}

#[derive(Clone, Debug)]
pub struct State {
    pub regs: ByteStore,
    pub uniq: ByteStore,
    pub mem: BTreeMap<Option<VarId>, ByteStore>,
    pub big_endian: bool,
    /// Register-space range holding one-byte boolean flags (x86: `0x200..0x216`).
    pub flag_regs: (u64, u64),
    /// Memory bytes the state wrote at a concrete address (never re-read from the image).
    pub unknown_stores: u32,
    pub unknown_loads: u32,
}

impl State {
    pub fn new(big_endian: bool, flag_regs: (u64, u64)) -> State {
        State {
            regs: ByteStore::default(),
            uniq: ByteStore::default(),
            mem: BTreeMap::new(),
            big_endian,
            flag_regs,
            unknown_stores: 0,
            unknown_loads: 0,
        }
    }

    pub fn read_reg(&mut self, arena: &mut ExprArena, off: u64, size: u8) -> ExprId {
        let flags = self.flag_regs;
        let be = self.big_endian;
        self.regs.read_with(arena, off as i128, size, be, &mut |a, o, len| {
            let boolean = len == 1 && (o as u64) >= flags.0 && (o as u64) < flags.1;
            a.var_of(VarKind::InitReg { offset: o as u64 }, len, boolean)
        })
    }

    pub fn write_reg(&mut self, off: u64, e: ExprId, size: u8) {
        self.regs.write(off as i128, e, size, self.big_endian);
    }

    pub fn read_unique(&mut self, arena: &mut ExprArena, off: u64, size: u8) -> ExprId {
        let be = self.big_endian;
        self.uniq.read_with(arena, off as i128, size, be, &mut |a, _o, len| a.unknown(len))
    }

    pub fn write_unique(&mut self, off: u64, e: ExprId, size: u8) {
        self.uniq.write(off as i128, e, size, self.big_endian);
    }

    pub fn clear_uniques(&mut self) {
        self.uniq.clear();
    }

    /// Read memory at `base + off`; concrete gaps come from the image.
    pub fn read_mem(
        &mut self,
        arena: &mut ExprArena,
        image: &dyn PushanImage,
        base: Option<VarId>,
        off: i128,
        size: u8,
    ) -> ExprId {
        let be = self.big_endian;
        let region = self.mem.entry(base).or_default();
        region.read_with(arena, off, size, be, &mut |a, o, len| {
            if base.is_none() {
                let addr = o as u64;
                let mut buf = vec![0u8; len as usize];
                if (o as u64 as i128) == o && image.read_bytes(addr, &mut buf) {
                    let mut v: u128 = 0;
                    if be {
                        for b in &buf {
                            v = (v << 8) | *b as u128;
                        }
                    } else {
                        for b in buf.iter().rev() {
                            v = (v << 8) | *b as u128;
                        }
                    }
                    return a.cst(len, v);
                }
            }
            a.var_of(VarKind::InitMem { base, offset: o }, len, false)
        })
    }

    pub fn write_mem(&mut self, base: Option<VarId>, off: i128, e: ExprId, size: u8) {
        let be = self.big_endian;
        self.mem.entry(base).or_default().write(off, e, size, be);
    }

    pub fn has_mem(&self, base: Option<VarId>, off: i128, size: u8) -> bool {
        self.mem.get(&base).is_some_and(|r| r.has_any(off, size))
    }

    /// Locations where `self` (the recorded entry state) and `other` (a new
    /// incoming state) disagree and at least one side is a constant.
    pub fn diff_constants(&mut self, other: &mut State, arena: &mut ExprArena, image: &dyn PushanImage) -> Vec<SymLoc> {
        let mut out = Vec::new();
        for (o, len) in self.regs.diff_ranges(&other.regs) {
            let a = self.read_reg(arena, o as u64, len);
            let b = other.read_reg(arena, o as u64, len);
            if a != b && (arena.is_const(a).is_some() || arena.is_const(b).is_some()) {
                out.push(SymLoc { loc: Loc::Reg { off: o as u64 }, size: len });
            }
        }
        let bases: Vec<Option<VarId>> = self.mem.keys().chain(other.mem.keys()).copied().collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        let empty = ByteStore::default();
        for base in bases {
            let ranges = self.mem.get(&base).unwrap_or(&empty).diff_ranges(other.mem.get(&base).unwrap_or(&empty));
            for (o, len) in ranges {
                let a = self.read_mem(arena, image, base, o, len);
                let b = other.read_mem(arena, image, base, o, len);
                if a != b && (arena.is_const(a).is_some() || arena.is_const(b).is_some()) {
                    out.push(SymLoc { loc: Loc::Mem { base, off: o }, size: len });
                }
            }
        }
        out
    }

    /// Replace each location by a stable fresh symbol.
    pub fn symbolize(&mut self, arena: &mut ExprArena, node_tag: u64, locs: &[SymLoc]) {
        for (i, l) in locs.iter().enumerate() {
            let tag = node_tag.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(i as u64);
            let v = arena.var_of(VarKind::Merged { node: (node_tag & 0xffff_ffff) as u32, tag }, l.size, false);
            match l.loc {
                Loc::Reg { off } => self.write_reg(off, v, l.size),
                Loc::Mem { base, off } => self.write_mem(base, off, v, l.size),
            }
        }
    }

    /// Rewrite every stored value through `f` (used to pin a boolean atom).
    pub fn map_values(&mut self, arena: &mut ExprArena, f: &mut dyn FnMut(&mut ExprArena, ExprId) -> ExprId) {
        let mut rewrite = |store: &mut ByteStore, arena: &mut ExprArena| {
            let mut cache: BTreeMap<ExprId, ExprId> = BTreeMap::new();
            for v in store.bytes.values_mut() {
                let ne = match cache.get(&v.expr) {
                    Some(n) => *n,
                    None => {
                        let n = f(arena, v.expr);
                        cache.insert(v.expr, n);
                        n
                    }
                };
                if ne != v.expr {
                    // A value may shrink when a select collapses; keep the slice valid.
                    if arena.size(ne) == arena.size(v.expr) {
                        v.expr = ne;
                    }
                }
            }
        };
        rewrite(&mut self.regs, arena);
        rewrite(&mut self.uniq, arena);
        for region in self.mem.values_mut() {
            rewrite(region, arena);
        }
    }

    pub fn mem_bytes(&self) -> usize {
        self.mem.values().map(|r| r.len()).sum()
    }
}

#[cfg(test)]
mod tests;
