//! (kuna pushan) The abstract machine state the devirtualizer emulates over.
//!
//! Registers, SLEIGH temporaries and memory are byte-granular stores of
//! `(expression, byte index)` cells, so overlapping x86 sub-registers and
//! mixed-width stack accesses compose and slice exactly.  Memory is split into
//! regions by the symbolic base of the address (`None` for concrete
//! addresses): an access at `base + offset` touches only its own region, which
//! is the "pointers to distinct symbolic bases never alias" assumption the
//! paper's stack handling relies on.  Unwritten concrete memory reads the
//! program image where its bytes cannot have changed (not a writable data
//! section); unwritten symbolic memory, writable data and registers read a
//! stable symbolic variable, so the same location is the same symbol on every
//! path.

use std::collections::BTreeMap;
use std::rc::Rc;

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
    /// Whether the bytes at `addr` may differ from the file's when the
    /// function runs: writable memory, other than what the loader fills in.
    fn is_mutable(&self, addr: u64) -> bool {
        self.is_writable(addr)
    }
}

/// The image as the file holds it, writable memory included: what a fetch
/// through an in-function interpreter's virtual PC reads.
pub struct FileImage<'a>(pub &'a dyn PushanImage);

impl PushanImage for FileImage<'_> {
    fn read_bytes(&self, addr: u64, buf: &mut [u8]) -> bool {
        self.0.read_bytes(addr, buf)
    }
    fn is_writable(&self, addr: u64) -> bool {
        self.0.is_writable(addr)
    }
    fn is_mutable(&self, _addr: u64) -> bool {
        false
    }
}

/// `len` bytes of image memory at `src` copied into `[off, off + len)` of
/// the region of `base` by an external copy routine: an unwritten byte of the
/// destination reads what the source held when it was copied (at external
/// call epoch `epoch`), so a large copy costs one record, not one cell per byte.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CopyRegion {
    pub base: Option<VarId>,
    pub off: i128,
    pub len: u64,
    pub src: u64,
    pub epoch: Option<u64>,
}

impl CopyRegion {
    fn end(&self) -> i128 {
        self.off + self.len as i128
    }
}

/// Bytes per page of a [`ByteStore`].
const PAGE: usize = 16;
/// The `idx` of an empty cell of a page.
const HOLE: u8 = u8::MAX;
const EMPTY_CELL: ByteVal = ByteVal { expr: 0, idx: HOLE };

/// [`PAGE`] consecutive cells of a store, shared by the states cloned from
/// one another until one of them writes into it.
#[derive(Debug)]
struct Page {
    cells: [ByteVal; PAGE],
    n: u8,
}

thread_local! {
    /// Pages alive on this thread ([`live_pages`]).
    static LIVE_PAGES: std::cell::Cell<i64> = const { std::cell::Cell::new(0) };
}

/// The number of store pages alive on this thread: a deterministic measure
/// of the memory the states of an exploration hold, shared pages counted once.
pub fn live_pages() -> i64 {
    LIVE_PAGES.with(|n| n.get())
}

/// Bytes one page holds, with its reference count and its entry in a store.
pub const PAGE_BYTES: usize = std::mem::size_of::<Page>() + 2 * std::mem::size_of::<usize>() + 40;

impl Page {
    fn empty() -> Self {
        LIVE_PAGES.with(|n| n.set(n.get() + 1));
        Page { cells: [EMPTY_CELL; PAGE], n: 0 }
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        LIVE_PAGES.with(|n| n.set(n.get() + 1));
        Page { cells: self.cells, n: self.n }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        LIVE_PAGES.with(|n| n.set(n.get() - 1));
    }
}

#[inline]
fn page_of(off: i128) -> (i128, usize) {
    (off.div_euclid(PAGE as i128) * PAGE as i128, off.rem_euclid(PAGE as i128) as usize)
}

/// A byte-granular store (one register file, one temporary file, or one
/// memory region), in pages shared copy-on-write: cloning a state copies one
/// pointer per page, and a write copies only the page it lands in.
#[derive(Clone, Default, Debug)]
pub struct ByteStore {
    pages: BTreeMap<i128, Rc<Page>>,
    n: usize,
}

/// A maximal run of consecutive bytes that slice one value, or a gap.
struct Run {
    off: i128,
    len: u8,
    src: Option<(ExprId, u8)>,
}

impl ByteStore {
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn get(&self, off: i128) -> Option<ByteVal> {
        let (b, i) = page_of(off);
        let c = self.pages.get(&b)?.cells[i];
        (c.idx != HOLE).then_some(c)
    }

    pub fn contains(&self, off: i128) -> bool {
        self.get(off).is_some()
    }

    pub fn insert(&mut self, off: i128, v: ByteVal) {
        let (b, i) = page_of(off);
        let p = self.pages.entry(b).or_insert_with(|| Rc::new(Page::empty()));
        if p.cells[i] == v {
            return;
        }
        let page = Rc::make_mut(p);
        if page.cells[i].idx == HOLE {
            page.n += 1;
            self.n += 1;
        }
        page.cells[i] = v;
    }

    fn remove_at(&mut self, off: i128) {
        let (b, i) = page_of(off);
        let Some(p) = self.pages.get_mut(&b) else { return };
        if p.cells[i].idx == HOLE {
            return;
        }
        let page = Rc::make_mut(p);
        page.cells[i] = EMPTY_CELL;
        page.n -= 1;
        self.n -= 1;
        if page.n == 0 {
            self.pages.remove(&b);
        }
    }

    /// Every stored byte, by ascending offset.
    pub fn iter(&self) -> impl Iterator<Item = (i128, ByteVal)> + '_ {
        self.pages.iter().flat_map(|(b, p)| p.cells.iter().enumerate().filter(|(_, c)| c.idx != HOLE).map(move |(i, c)| (b + i as i128, *c)))
    }

    /// The stored bytes in `[lo, hi)`, by ascending offset.
    pub fn range(&self, lo: i128, hi: i128) -> impl Iterator<Item = (i128, ByteVal)> + '_ {
        let first = page_of(lo).0;
        self.pages
            .range(first..hi.max(first))
            .flat_map(|(b, p)| p.cells.iter().enumerate().filter(|(_, c)| c.idx != HOLE).map(move |(i, c)| (b + i as i128, *c)))
            .filter(move |(o, _)| *o >= lo && *o < hi)
    }

    /// Keep the stored bytes whose offset satisfies `keep`.
    pub fn retain(&mut self, keep: impl Fn(i128) -> bool) {
        let mut n = 0usize;
        self.pages.retain(|b, p| {
            let gone: Vec<usize> = (0..PAGE).filter(|&i| p.cells[i].idx != HOLE && !keep(b + i as i128)).collect();
            if !gone.is_empty() {
                let page = Rc::make_mut(p);
                for i in gone {
                    page.cells[i] = EMPTY_CELL;
                    page.n -= 1;
                }
            }
            n += p.n as usize;
            p.n > 0
        });
        self.n = n;
    }

    pub fn write(&mut self, off: i128, e: ExprId, size: u8, big_endian: bool) {
        for i in 0..size {
            let idx = if big_endian { size - 1 - i } else { i };
            self.insert(off + i as i128, ByteVal { expr: e, idx });
        }
    }

    pub fn clear(&mut self) {
        self.pages.clear();
        self.n = 0;
    }

    pub fn remove(&mut self, off: i128, size: u8) {
        for i in 0..size {
            self.remove_at(off + i as i128);
        }
    }

    /// Drop every stored byte whose offset satisfies `f`.
    pub fn remove_where(&mut self, f: impl Fn(i128) -> bool) {
        self.retain(|o| !f(o));
    }

    pub fn has_any(&self, off: i128, size: u8) -> bool {
        (0..size).any(|i| self.contains(off + i as i128))
    }

    /// Rewrite every stored value through `f`, called once per distinct
    /// value in ascending offset order; a value that would change size is
    /// kept (a select that collapsed), so every slice stays valid.
    pub fn map_exprs(&mut self, arena: &mut ExprArena, f: &mut dyn FnMut(&mut ExprArena, ExprId) -> ExprId) {
        let mut cache: BTreeMap<ExprId, ExprId> = BTreeMap::new();
        for p in self.pages.values_mut() {
            let mut changed: Vec<(usize, ExprId)> = Vec::new();
            for (i, c) in p.cells.iter().enumerate().filter(|(_, c)| c.idx != HOLE) {
                let ne = match cache.get(&c.expr) {
                    Some(n) => *n,
                    None => {
                        let n = f(arena, c.expr);
                        cache.insert(c.expr, n);
                        n
                    }
                };
                if ne != c.expr && arena.size(ne) == arena.size(c.expr) {
                    changed.push((i, ne));
                }
            }
            if !changed.is_empty() {
                let page = Rc::make_mut(p);
                for (i, ne) in changed {
                    page.cells[i].expr = ne;
                }
            }
        }
    }

    fn runs(&self, off: i128, size: u8, big_endian: bool) -> Vec<Run> {
        let mut runs: Vec<Run> = Vec::new();
        for i in 0..size {
            let o = off + i as i128;
            let cell = self.get(o);
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

    /// Offsets where the two stores disagree (a byte stored in one only,
    /// or different in both), ascending; a page they share is skipped whole.
    fn differing(&self, other: &ByteStore) -> Vec<i128> {
        let mut out = Vec::new();
        let (mut a, mut b) = (self.pages.iter().peekable(), other.pages.iter().peekable());
        loop {
            let (base, pa, pb) = match (a.peek(), b.peek()) {
                (None, None) => break,
                (Some((ka, _)), Some((kb, _))) if ka == kb => {
                    let (k, x) = a.next().unwrap();
                    let (_, y) = b.next().unwrap();
                    if Rc::ptr_eq(x, y) {
                        continue;
                    }
                    (*k, Some(x), Some(y))
                }
                (Some((ka, _)), Some((kb, _))) if ka > kb => {
                    let (k, y) = b.next().unwrap();
                    (*k, None, Some(y))
                }
                (Some(_), _) => {
                    let (k, x) = a.next().unwrap();
                    (*k, Some(x), None)
                }
                (None, Some(_)) => {
                    let (k, y) = b.next().unwrap();
                    (*k, None, Some(y))
                }
            };
            for i in 0..PAGE {
                let ca = pa.map_or(EMPTY_CELL, |p| p.cells[i]);
                let cb = pb.map_or(EMPTY_CELL, |p| p.cells[i]);
                if ca != cb {
                    out.push(base + i as i128);
                }
            }
        }
        out
    }

    /// Byte offsets where the two stores disagree, as maximal runs.
    pub fn diff_bytes(&self, other: &ByteStore) -> Vec<(i128, u8)> {
        let mut out: Vec<(i128, u8)> = Vec::new();
        for o in self.differing(other) {
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
        // A differing byte marks its whole 8-byte-aligned slot (one register,
        // one stack word), so a location is always compared as a whole and a
        // code address in it is seen as such.
        let mut slots: Vec<i128> = self.differing(other).iter().map(|o| o.div_euclid(8) * 8).collect();
        slots.dedup();
        slots.into_iter().map(|s| (s, 8)).collect()
    }

    pub fn offsets(&self) -> impl Iterator<Item = i128> + '_ {
        self.iter().map(|(o, _)| o)
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
    /// The last external call emulated on this path (a number unique to that
    /// emulation): unwritten mutable memory holds what that call left there.
    pub global_epoch: Option<u64>,
    /// Copies of image memory made by external copy routines, oldest first.
    pub copies: Vec<CopyRegion>,
    /// Besides mutable image memory, what the last external call may have
    /// written ([`State::clobber_call`]): every region under a symbolic base
    /// other than the frame base, and the frame bytes in these ranges.
    pub call_frame: Option<(VarId, Vec<(i128, i128)>)>,
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
            global_epoch: None,
            copies: Vec::new(),
            call_frame: None,
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

    /// Read memory at `base + off`; concrete gaps come from the image.  An
    /// unwritten byte of mutable memory is not the file's: it reads the
    /// stable symbol of its value at entry, or after the last external call.
    pub fn read_mem(
        &mut self,
        arena: &mut ExprArena,
        image: &dyn PushanImage,
        base: Option<VarId>,
        off: i128,
        size: u8,
    ) -> ExprId {
        let be = self.big_endian;
        let epoch = self.global_epoch;
        let State { mem, copies, call_frame, .. } = self;
        let copies: &[CopyRegion] = copies;
        let frame = call_frame.as_ref();
        let region = mem.entry(base).or_default();
        region.read_with(arena, off, size, be, &mut |a, o, len| fill_mem(a, image, be, base, o, len, epoch, copies, frame))
    }

    /// An external call may have written any mutable memory: the stored
    /// bytes there are dropped and later reads see the output of the call
    /// numbered `epoch`.
    pub fn clobber_mutable(&mut self, image: &dyn PushanImage, epoch: u64) {
        if let Some(region) = self.mem.get_mut(&None) {
            region.retain(|o| u64::try_from(o).is_ok_and(|a| !image.is_mutable(a)));
        }
        self.copies.retain(|c| c.base.is_some() || (0..c.len).step_by(0x1000).chain([c.len - 1]).all(|i| !image.is_mutable((c.off as u64).wrapping_add(i))));
        self.global_epoch = Some(epoch);
    }

    /// An external call (`epoch`) may have written everything its caller
    /// could have handed it a pointer to: all mutable image memory, the
    /// region under every symbolic base but the frame base `frame` (a
    /// pointer argument, or one reachable from it, may point anywhere into
    /// any of them), and the frame bytes in `ranges` (those an escaped frame
    /// address reaches).  Their stored bytes are dropped and their unwritten
    /// bytes read as that call's output.
    pub fn clobber_call(&mut self, image: &dyn PushanImage, epoch: u64, frame: VarId, ranges: Vec<(i128, i128)>) {
        self.clobber_mutable(image, epoch);
        let symbolic: Vec<VarId> = self.mem.keys().filter_map(|b| b.filter(|v| *v != frame)).collect();
        for b in symbolic {
            self.forget_copies(Some(b), i128::MIN, i128::MAX);
            self.mem.remove(&Some(b));
        }
        for &(lo, hi) in &ranges {
            self.forget_copies(Some(frame), lo, hi);
            if let Some(region) = self.mem.get_mut(&Some(frame)) {
                region.remove_where(|o| o >= lo && o < hi);
            }
        }
        self.call_frame = Some((frame, ranges));
    }

    /// Model `memcpy(base + off, src, len)` from concrete image memory: the
    /// destination's stored bytes are replaced by what the source holds now
    /// (the bytes this path wrote there, copied; the rest read lazily through
    /// a [`CopyRegion`]). False, with nothing changed, when the source
    /// overlaps another copy's concrete destination.
    pub fn copy_in(&mut self, base: Option<VarId>, off: i128, src: u64, len: u64) -> bool {
        let (s0, s1) = (src as i128, src as i128 + len as i128);
        if len == 0 || self.copies.iter().any(|c| c.base.is_none() && c.off < s1 && s0 < c.end()) {
            return false;
        }
        let written: Vec<(i128, ByteVal)> = self.mem.get(&None).map(|r| r.range(s0, s1).map(|(o, v)| (o - s0 + off, v)).collect()).unwrap_or_default();
        self.forget_copies(base, off, off + len as i128);
        let region = self.mem.entry(base).or_default();
        region.retain(|o| o < off || o >= off + len as i128);
        for (o, v) in written {
            region.insert(o, v);
        }
        self.copies.push(CopyRegion { base, off, len, src, epoch: self.global_epoch });
        true
    }

    /// Drop the copies' claim on `[lo, hi)` of the region of `base`, whose
    /// contents are being forgotten (a copy partly inside keeps the rest).
    pub fn forget_copies(&mut self, base: Option<VarId>, lo: i128, hi: i128) {
        if !self.copies.iter().any(|c| c.base == base && c.off < hi && lo < c.end()) {
            return;
        }
        let mut kept = Vec::with_capacity(self.copies.len() + 1);
        for c in self.copies.drain(..) {
            if c.base != base || c.end() <= lo || hi <= c.off {
                kept.push(c);
                continue;
            }
            if c.off < lo {
                kept.push(CopyRegion { len: (lo - c.off) as u64, ..c });
            }
            if hi < c.end() {
                kept.push(CopyRegion { off: hi, len: (c.end() - hi) as u64, src: c.src + (hi - c.off) as u64, ..c });
            }
        }
        self.copies = kept;
    }

    /// Read memory as the file holds it, writable memory included: a byte the
    /// path wrote reads what it wrote, and an unwritten byte reads the file
    /// (directly or through a copy), even where an earlier read cached the
    /// symbol of its value at entry.
    pub fn read_mem_file(&mut self, arena: &mut ExprArena, image: &dyn PushanImage, base: Option<VarId>, off: i128, size: u8) -> ExprId {
        let stale: Vec<i128> = (off..off + size as i128).filter(|o| self.cached_fill(arena, base, *o)).collect();
        if let Some(region) = self.mem.get_mut(&base) {
            for o in stale {
                region.remove(o, 1);
            }
        }
        self.read_mem(arena, &FileImage(image), base, off, size)
    }

    /// The byte at `base + o` holds only the cached symbol of an unwritten
    /// image byte's value (its own address, or its copy's source).
    fn cached_fill(&self, arena: &ExprArena, base: Option<VarId>, o: i128) -> bool {
        let Some(bv) = self.mem.get(&base).and_then(|r| r.get(o)) else { return false };
        let at = match arena.var_kind(bv.expr) {
            Some(VarKind::InitMem { base: None, offset }) | Some(VarKind::CallOutputAt { base: None, offset, .. }) => offset,
            _ => return false,
        };
        let size = arena.size(bv.expr);
        let byte = at + if self.big_endian { (size - 1 - bv.idx) as i128 } else { bv.idx as i128 };
        match base {
            None => byte == o,
            Some(_) => self.copy_source(base, o).is_some_and(|s| s as i128 == byte),
        }
    }

    /// The image address the unwritten byte at `base + off` was copied from.
    pub fn copy_source(&self, base: Option<VarId>, off: i128) -> Option<u64> {
        self.copies.iter().rev().find(|c| c.base == base && c.off <= off && off < c.end()).map(|c| c.src + (off - c.off) as u64)
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
    /// neighbour does not take the neighbour with it.  A slot whose two
    /// values are the same by [`same_value`] (each path rebuilt it through a
    /// different identity) is no difference.
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
                if same_value(arena, a, b) {
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
                    if same_value(arena, a, b) {
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
        self.regs.map_exprs(arena, f);
        self.uniq.map_exprs(arena, f);
        for region in self.mem.values_mut() {
            region.map_exprs(arena, f);
        }
    }

    /// Whether any of `[off, off + size)` was written in the region of `base`.
    pub fn mem_written(&self, base: Option<VarId>, off: i128, size: u8) -> bool {
        self.mem.get(&base).is_some_and(|r| (off..off + size as i128).any(|o| r.contains(o)))
    }

    /// The offsets with a stored byte in the region of `base`.
    pub fn mem_offsets(&self, base: Option<VarId>) -> Vec<i128> {
        self.mem.get(&base).map(|r| r.offsets().collect()).unwrap_or_default()
    }

    pub fn mem_bytes(&self) -> usize {
        self.mem.values().map(|r| r.len()).sum()
    }
}

/// The value of `len` unwritten bytes at `base + o`: a copy's source as it
/// was when copied, the image where its bytes cannot have changed, what the
/// last external call left in memory it may have written (mutable image
/// memory, symbolic regions and the reachable frame, `frame`), and
/// otherwise the stable symbol of their entry value.
#[allow(clippy::too_many_arguments)]
fn fill_mem(
    a: &mut ExprArena,
    image: &dyn PushanImage,
    be: bool,
    base: Option<VarId>,
    o: i128,
    len: u8,
    epoch: Option<u64>,
    copies: &[CopyRegion],
    frame: Option<&(VarId, Vec<(i128, i128)>)>,
) -> ExprId {
    if let Some(c) = copies.iter().rev().find(|c| c.base == base && c.off < o + len as i128 && o < c.end()) {
        if c.off <= o && o + len as i128 <= c.end() {
            let src = c.src as i128 + (o - c.off);
            return fill_mem(a, image, be, None, src, len, c.epoch, &[], None);
        }
        let bytes: Vec<ExprId> = (0..len as i128).map(|i| fill_mem(a, image, be, base, o + i, 1, epoch, copies, frame)).collect();
        let mut acc = bytes[0];
        for b in &bytes[1..] {
            acc = if be { a.mk_piece(acc, *b) } else { a.mk_piece(*b, acc) };
        }
        return acc;
    }
    if base.is_none() {
        let addr = o as u64;
        let concrete = (o as u64 as i128) == o;
        if concrete && (image.is_mutable(addr) || image.is_mutable(addr.wrapping_add(len as u64 - 1))) {
            if let Some(site) = epoch {
                return a.var_of(VarKind::CallOutputAt { epoch: site, base, offset: o }, len, false);
            }
            return a.var_of(VarKind::InitMem { base, offset: o }, len, false);
        }
        let mut buf = vec![0u8; len as usize];
        if concrete && image.read_bytes(addr, &mut buf) {
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
    if let (Some(b), Some(site), Some((f, ranges))) = (base, epoch, frame) {
        let end = o + len as i128;
        if b != *f || ranges.iter().any(|&(lo, hi)| lo <= o && end <= hi) {
            return a.var_of(VarKind::CallOutputAt { epoch: site, base, offset: o }, len, false);
        }
        if len > 1 && ranges.iter().any(|&(lo, hi)| lo < end && o < hi) {
            let bytes: Vec<ExprId> = (0..len as i128).map(|i| fill_mem(a, image, be, base, o + i, 1, epoch, copies, frame)).collect();
            let mut acc = bytes[0];
            for x in &bytes[1..] {
                acc = if be { a.mk_piece(acc, *x) } else { a.mk_piece(*x, acc) };
            }
            return acc;
        }
    }
    a.var_of(VarKind::InitMem { base, offset: o }, len, false)
}

fn loc_tag(l: &Loc) -> u64 {
    match l {
        Loc::Reg { off } => off.wrapping_mul(0x100000001b3),
        Loc::Mem { base, off } => (*off as u64).wrapping_mul(0x9E3779B97F4A7C15) ^ base.map(|b| b as u64 + 1).unwrap_or(0).rotate_left(40),
    }
}

/// `a` and `b` are the same value whatever the inputs: the same node, or two
/// expressions whose difference the identity test finds to be the constant
/// 0 ([`ExprArena::identity_probe`]).
pub fn same_value(arena: &mut ExprArena, a: ExprId, b: ExprId) -> bool {
    if a == b {
        return true;
    }
    if arena.is_const(a).is_some() && arena.is_const(b).is_some() {
        return false;
    }
    let size = arena.size(a);
    let w = size.max(2);
    let x = arena.resize(a, w);
    let y = arena.resize(b, w);
    let d = arena.mk_bin(kuna_num::opcodes::OpCode::CPUI_INT_SUB, x, y, w);
    arena.is_const(d) == Some(0) || arena.identity_probe(d) == Some((None, 0))
}

#[cfg(test)]
mod tests;
