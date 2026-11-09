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
    /// filled by `fill(offset, length)`.  A symbolic fill is stored so later
    /// reads of the same bytes, at any width, agree; a constant one (an image
    /// byte) is the same on every read and is not, which keeps the bytecode a
    /// path fetched out of every later state.
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
                    if arena.is_const(v).is_none() {
                        self.write(r.off, v, r.len, big_endian);
                    }
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

    /// Byte offsets where the two stores disagree, as maximal runs.
    pub fn diff_bytes(&self, other: &ByteStore) -> Vec<(i128, u8)> {
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
        // A differing byte marks its whole 8-byte-aligned slot (one register,
        // one stack word), so a location is always compared as a whole and a
        // code address in it is seen as such.
        let mut slots: Vec<i128> = offs.iter().map(|o| o.div_euclid(8) * 8).collect();
        slots.dedup();
        slots.into_iter().map(|s| (s, 8)).collect()
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

/// A differing location found when a path re-reaches a node.
#[derive(Clone, Copy, Debug)]
pub struct Diff {
    pub loc: SymLoc,
    /// The two constants when both sides were constant.
    pub values: Option<(u128, u128)>,
    /// The recorded entry value, when constant.
    pub entry_const: Option<u128>,
    /// The incoming value, when constant.
    pub incoming_const: Option<u128>,
    /// Bits both sides agree on: (known-zero mask, known-one mask).
    pub known: (u128, u128),
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
    /// The 4-byte granules of the entry-stack region the program stored to
    /// since the last external call, sorted: the words a call's arguments can be.
    pub stack_stores: Vec<i32>,
}

/// Most granules [`State::stack_stores`] records between two calls.
const MAX_STACK_STORES: usize = 4096;

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
            stack_stores: Vec::new(),
        }
    }

    /// Record a store of `size` bytes at entry-stack offset `off`.
    pub fn note_store(&mut self, off: i128, size: u8) {
        for g in off.div_euclid(4)..=(off + size.max(1) as i128 - 1).div_euclid(4) {
            let Ok(g) = i32::try_from(g) else { continue };
            if let Err(i) = self.stack_stores.binary_search(&g) {
                if self.stack_stores.len() < MAX_STACK_STORES {
                    self.stack_stores.insert(i, g);
                }
            }
        }
    }

    /// Whether the program stored to the 4-byte granule holding entry-stack
    /// offset `off` since the last external call.
    pub fn stored_since_call(&self, off: i128) -> bool {
        i32::try_from(off.div_euclid(4)).is_ok_and(|g| self.stack_stores.binary_search(&g).is_ok())
    }

    /// Union in the stores `other` recorded (the two sides of a merged select).
    pub fn merge_stores(&mut self, other: &State) {
        for &g in &other.stack_stores {
            if let Err(i) = self.stack_stores.binary_search(&g) {
                if self.stack_stores.len() < MAX_STACK_STORES {
                    self.stack_stores.insert(i, g);
                }
            }
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

    /// Every location whose value differs between `self` (the recorded
    /// entry state) and `other` (a new incoming state), as angr's merge
    /// reports it.  A location is an aligned slot of `word` bytes (the
    /// target's register and stack-word size); a slot symbolic on both sides
    /// whose difference sits in one half only is narrowed to that half (down
    /// to a byte), so a narrow variable sharing a slot with an unchanged
    /// neighbour does not take the neighbour with it.
    pub fn diff_constants(&mut self, other: &mut State, arena: &mut ExprArena, image: &dyn PushanImage, word: u8) -> Vec<Diff> {
        let mut out = Vec::new();
        let w = word.max(1) as i128;
        let slots_of = |runs: Vec<(i128, u8)>| {
            let mut slots: Vec<i128> = Vec::new();
            for (o, len) in runs {
                let mut s = o.div_euclid(w) * w;
                while s < o + len as i128 {
                    if !slots.contains(&s) {
                        slots.push(s);
                    }
                    s += w;
                }
            }
            slots.sort_unstable();
            slots
        };
        let record = |arena: &ExprArena, loc: SymLoc, a: ExprId, b: ExprId| {
            let (za, oa) = arena.known_bits(a);
            let (zb, ob) = arena.known_bits(b);
            Diff {
                loc,
                values: arena.is_const(a).zip(arena.is_const(b)),
                entry_const: arena.is_const(a),
                incoming_const: arena.is_const(b),
                known: (za & zb, oa & ob),
            }
        };
        for slot in slots_of(self.regs.diff_bytes(&other.regs)) {
            let mut todo = vec![(slot, word)];
            while let Some((off, size)) = todo.pop() {
                let a = self.read_reg(arena, off as u64, size);
                let b = other.read_reg(arena, off as u64, size);
                if a == b {
                    continue;
                }
                let h = size / 2;
                let split = size > 1 && arena.is_const(a).is_none() && arena.is_const(b).is_none() && {
                    let lo_same = self.read_reg(arena, off as u64, h) == other.read_reg(arena, off as u64, h);
                    let hi_same = self.read_reg(arena, (off + h as i128) as u64, h) == other.read_reg(arena, (off + h as i128) as u64, h);
                    lo_same || hi_same
                };
                if split {
                    todo.push((off + h as i128, h));
                    todo.push((off, h));
                } else {
                    out.push(record(arena, SymLoc { loc: Loc::Reg { off: off as u64 }, size }, a, b));
                }
            }
        }
        let bases: Vec<Option<VarId>> = self.mem.keys().chain(other.mem.keys()).copied().collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        let empty = ByteStore::default();
        for base in bases {
            let runs = self.mem.get(&base).unwrap_or(&empty).diff_bytes(other.mem.get(&base).unwrap_or(&empty));
            for slot in slots_of(runs) {
                let mut todo = vec![(slot, word)];
                while let Some((off, size)) = todo.pop() {
                    let a = self.read_mem(arena, image, base, off, size);
                    let b = other.read_mem(arena, image, base, off, size);
                    if a == b {
                        continue;
                    }
                    let h = size / 2;
                    let split = size > 1 && arena.is_const(a).is_none() && arena.is_const(b).is_none() && {
                        let lo_same = self.read_mem(arena, image, base, off, h) == other.read_mem(arena, image, base, off, h);
                        let hi_same =
                            self.read_mem(arena, image, base, off + h as i128, h) == other.read_mem(arena, image, base, off + h as i128, h);
                        lo_same || hi_same
                    };
                    if split {
                        todo.push((off + h as i128, h));
                        todo.push((off, h));
                    } else {
                        out.push(record(arena, SymLoc { loc: Loc::Mem { base, off }, size }, a, b));
                    }
                }
            }
        }
        out
    }

    /// Replace each location by a stable fresh symbol; returns the symbols.
    pub fn symbolize(&mut self, arena: &mut ExprArena, node_tag: u64, locs: &[SymLoc]) -> Vec<(SymLoc, ExprId)> {
        let with: Vec<(SymLoc, (u128, u128))> = locs.iter().map(|l| (*l, (0, 0))).collect();
        self.symbolize_known(arena, node_tag, &with)
    }

    /// [`symbolize`](Self::symbolize) with the bits every observed value
    /// agreed on, so fixed bits (a flag word's reserved and system bits, an
    /// aligned pointer's low bits) still fold through the symbol.
    pub fn symbolize_known(&mut self, arena: &mut ExprArena, node_tag: u64, locs: &[(SymLoc, (u128, u128))]) -> Vec<(SymLoc, ExprId)> {
        let mut out = Vec::with_capacity(locs.len());
        for (l, known) in locs.iter() {
            let known_tag = ((known.0 ^ (known.1 << 1)) as u64).wrapping_mul(0xff51afd7ed558ccd);
            let tag = node_tag.wrapping_mul(0x9E3779B97F4A7C15) ^ loc_tag(&l.loc).rotate_left(7) ^ l.size as u64 ^ known_tag;
            let node = (node_tag & 0xffff_ffff) as u32;
            let boolean = l.size == 1 && matches!(l.loc, Loc::Reg { off } if off >= self.flag_regs.0 && off < self.flag_regs.1);
            let v = arena.var_of_known(VarKind::Merged { node, tag }, l.size, boolean, *known);
            match l.loc {
                Loc::Reg { off } => self.write_reg(off, v, l.size),
                Loc::Mem { base, off } => self.write_mem(base, off, v, l.size),
            }
            out.push((*l, v));
        }
        out
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

    /// Whether any of `[off, off + size)` was written in the region of `base`.
    pub fn mem_written(&self, base: Option<VarId>, off: i128, size: u8) -> bool {
        self.mem.get(&base).is_some_and(|r| (off..off + size as i128).any(|o| r.bytes.contains_key(&o)))
    }

    /// The offsets with a stored byte in the region of `base`.
    pub fn mem_offsets(&self, base: Option<VarId>) -> Vec<i128> {
        self.mem.get(&base).map(|r| r.offsets().copied().collect()).unwrap_or_default()
    }

    pub fn mem_bytes(&self) -> usize {
        self.mem.values().map(|r| r.len()).sum()
    }
}

fn loc_tag(l: &Loc) -> u64 {
    match l {
        Loc::Reg { off } => off.wrapping_mul(0x100000001b3),
        Loc::Mem { base, off } => (*off as u64).wrapping_mul(0x9E3779B97F4A7C15) ^ base.map(|b| b as u64 + 1).unwrap_or(0).rotate_left(40),
    }
}

#[cfg(test)]
mod tests;
