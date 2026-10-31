//! (kuna) `option bejoin callers`: how a function's callers read its pair is
//! evidence the pair is one value.
//!
//! A `long long` built from a 64-bit sum or one `umull` compiles to the bytes
//! of the `int` that returns its high word (`addc 4,4,6; adde 3,3,5` is both
//! `a + b` and `(a + b) >> 32`), so the function alone cannot tell them apart.
//! Its callers can. A call clobbers the second return register, so a caller
//! compiled against `int f()` never reads it after the call; one that does
//! reads the value's low word.
//!
//! A function is proven by its callers when it starts a symbol of the image's
//! own symbol table (its extent runs to the next one), its instructions reach
//! a RETURN or a branch into another function, it writes the second register
//! (itself, through a direct callee that does, or by an indirect or unknown
//! call, which the ABI lets clobber it), and some direct call to it, inside
//! another symbol's extent, is followed on some path by an instruction that
//! reads the second register before anything writes it or another call
//! clobbers it. A callee that never writes the register lets its caller keep
//! a value of its own there across the call (gcc's IPA-RA does), so a read
//! then says nothing. In a function with a register window, only a write of a
//! register its `restore` moves into the second register counts, and calls
//! do not. A RETURN the register reaches untouched (`return f();` in a
//! function that keeps a frame) proves the callee when that caller is proven
//! by its own callers.
//!
//! A call (an argument the callee may not take), a RETURN, a branch, a
//! register window's copy of every register, a register XORed with or
//! subtracted from itself, and an instruction nothing of which depends on
//! the register's value (PowerPC's `subfe 4,4,4`, which makes `xer_ca - 1`)
//! do not read it, nor, once the path took a branch,
//! a store of several registers or a register pair (gcc's merged `std %o0`
//! or `stm`, whose second word can be dead at a join), nor the delay slot of
//! a conditional branch, which may come from the branch target. The walk
//! after a call keeps, on each path, what the branches it took decided about
//! the flags, so the instructions a condition skips are skipped together
//! (ARM's `ldrne r1,..; orrnes r2,r1,r0` never reads a stale `r1`). A
//! function whose first instruction stores the second register (one that
//! saves the argument registers to give them back, as `__gnu_mcount_nc`
//! does) says nothing.
//!
//! A call kuna does not know never returns lays unrelated code out after it
//! (`bl f; __builtin_unreachable()` is followed by the block a `bne`
//! reaches, which reads `r4` for its own value), so a site says nothing when
//! its fall-through reaches, before any branch, an instruction a branch of
//! the caller targets, when a branch after it tests a flag nothing set since
//! the call (an ARM literal pool's words decode as `andeq r0,r1,..`), or when
//! the caller jumps through a register, whose table may reach that code. Nor
//! does one whose walk reaches a word the caller loads as data (a Thumb
//! literal pool's `0x0008` decodes as `movs r0,r1`), or, before any branch,
//! the target of a direct call (a function whose symbol `strip -x` removed),
//! nor any in a part of a function gcc's hot/cold partition names `f.cold`,
//! whose blocks the hot part enters. A C++ landing pad, which the unwinder
//! enters with the selector in the second register, ends a site the same way
//! on any path; the pads come from the image's exception tables
//! ([`landing`]), and a caller whose tables do not decode, or an image whose
//! tables cannot be placed, gives no evidence.
//!
//! The call sites come from one scan of the executable bytes for the
//! direct-call encodings (PowerPC `bl`, MIPS `jal` and `bal`, SPARC `call`,
//! ARM `bl` and `blx` and their Thumb forms) whose target starts a symbol, each
//! decoded again before it is read, and the extents from the image's own
//! symbol table, so a verdict is the same whichever function a run decompiles,
//! in whatever order. An image without symbols proves nothing this way.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::{decode, overlaps, Entry, Insn, Key};
use crate::architecture::Architecture;

/// How many call sites of one function are read.
pub(super) const MAX_SITES: usize = 16;

/// How many instructions the walk after one call site visits.
const MAX_SITE_INSNS: usize = 256;

/// How many instructions the scan of one function visits.
const MAX_SCAN_INSNS: usize = 16_384;

/// How many functions the search for a write of the second register visits.
const MAX_CLOBBER_FUNCS: usize = 32;

/// How many bytes of code one read of the image takes.
const CHUNK: usize = 1 << 20;

/// The symbols that say an image unwinds C++ exceptions.
const EH_NAMES: [&[u8]; 7] = [
    b"__gxx_personality_v0",
    b"__gcc_personality_v0",
    b"__gxx_personality_sj0",
    b"__gcc_personality_sj0",
    b"_Unwind_Resume",
    b"__cxa_begin_catch",
    b"__cxa_end_cleanup",
];

const M32: u64 = 0xffff_ffff;

/// The instruction sets whose direct calls the scan recognizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Isa {
    Ppc,
    Mips,
    Sparc,
    Arm { little: bool },
}

impl Isa {
    /// The set a language id names, for 32-bit big-endian data.
    pub(super) fn of(archid: &str) -> Option<Isa> {
        let mut parts = archid.split(':');
        let (proc, endian, bits) = (parts.next()?, parts.next()?, parts.next()?);
        if bits != "32" {
            return None;
        }
        match (proc, endian) {
            ("PowerPC", "BE") => Some(Isa::Ppc),
            ("MIPS", "BE") => Some(Isa::Mips),
            ("sparc", "BE") => Some(Isa::Sparc),
            ("ARM", "BE") => Some(Isa::Arm { little: false }),
            ("ARM", "LEBE") => Some(Isa::Arm { little: true }),
            _ => None,
        }
    }

    /// Every `(target, site)` a direct call in `bytes`, loaded at `base`,
    /// encodes, for sites below `base + limit`.
    pub(super) fn calls(self, base: u64, bytes: &[u8], limit: usize, out: &mut Vec<(u64, u64)>) {
        let little = matches!(self, Isa::Arm { little: true });
        let word = |i: usize| -> Option<u32> {
            let b: [u8; 4] = bytes.get(i..i + 4)?.try_into().ok()?;
            Some(if little { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) })
        };
        let half = |i: usize| -> Option<u16> {
            let b: [u8; 2] = bytes.get(i..i + 2)?.try_into().ok()?;
            Some(if little { u16::from_le_bytes(b) } else { u16::from_be_bytes(b) })
        };
        let first = (base.wrapping_neg() & 3) as usize;
        let mut i = first;
        while i < limit {
            let site = base.wrapping_add(i as u64) & M32;
            if let Some(w) = word(i) {
                let target = match self {
                    Isa::Ppc => ppc(site, w),
                    Isa::Mips => mips(site, w),
                    Isa::Sparc => sparc(site, w),
                    Isa::Arm { .. } => a32(site, w),
                };
                out.extend(target.map(|t| (t, site)));
            }
            i += 4;
        }
        if matches!(self, Isa::Arm { .. }) {
            let mut i = (base & 1) as usize;
            while i < limit {
                if let (Some(h1), Some(h2)) = (half(i), half(i + 2)) {
                    let site = base.wrapping_add(i as u64) & M32;
                    out.extend(t32(site, h1, h2).map(|t| (t, site)));
                }
                i += 2;
            }
        }
    }
}

fn sext(v: u64, bits: u32) -> i64 {
    ((v << (64 - bits)) as i64) >> (64 - bits)
}

/// PowerPC `bl` and `bla`.
pub(super) fn ppc(site: u64, w: u32) -> Option<u64> {
    let li = sext((w & 0x03ff_fffc) as u64, 26);
    match w & 0xfc00_0003 {
        0x4800_0001 => Some((site as i64).wrapping_add(li) as u64 & M32),
        0x4800_0003 => Some(li as u64 & M32),
        _ => None,
    }
}

/// MIPS `jal` and `bal`.
pub(super) fn mips(site: u64, w: u32) -> Option<u64> {
    if w >> 26 == 3 {
        return Some((site.wrapping_add(4) & 0xf000_0000) | (((w & 0x03ff_ffff) as u64) << 2));
    }
    if w & 0xffff_0000 == 0x0411_0000 {
        return Some((site as i64 + 4 + sext(((w & 0xffff) as u64) << 2, 18)) as u64 & M32);
    }
    None
}

/// SPARC `call`.
pub(super) fn sparc(site: u64, w: u32) -> Option<u64> {
    (w >> 30 == 1).then(|| (site as i64).wrapping_add(sext((w as u64) << 2, 32)) as u64 & M32)
}

/// ARM `bl` and `blx` (immediate).
pub(super) fn a32(site: u64, w: u32) -> Option<u64> {
    let off = sext(((w & 0x00ff_ffff) as u64) << 2, 26);
    if w >> 28 != 0xf && (w >> 24) & 0xf == 0xb {
        return Some((site as i64 + 8 + off) as u64 & M32);
    }
    if w >> 25 == 0x7d {
        let h = (((w >> 24) & 1) << 1) as i64;
        return Some((site as i64 + 8 + off + h) as u64 & M32);
    }
    None
}

/// Thumb `bl` and `blx` (immediate), the pair of halfwords `h1`, `h2`.
pub(super) fn t32(site: u64, h1: u16, h2: u16) -> Option<u64> {
    if h1 & 0xf800 != 0xf000 {
        return None;
    }
    let blx = match (h2 & 0xd000 == 0xd000, h2 & 0xd001 == 0xc000) {
        (true, _) => false,
        (_, true) => true,
        _ => return None,
    };
    let s = ((h1 >> 10) & 1) as u64;
    let (j1, j2) = (((h2 >> 13) & 1) as u64, ((h2 >> 11) & 1) as u64);
    let (i1, i2) = (!(j1 ^ s) & 1, !(j2 ^ s) & 1);
    let imm = (s << 24) | (i1 << 23) | (i2 << 22) | (((h1 & 0x3ff) as u64) << 12) | (((h2 & 0x7ff) as u64) << 1);
    let pc = site + 4;
    let pc = if blx { pc & !3 } else { pc };
    Some((pc as i64 + sext(imm, 25)) as u64 & M32)
}

/// What the walk after one call site found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Use {
    /// An instruction read the second register.
    Read,
    /// The register reached the caller's RETURN untouched.
    Returned,
    /// Written, clobbered, or out of reach on every path.
    Dead,
}

/// What one function's scan found.
#[derive(Clone, Debug, Default)]
struct Scan {
    /// A RETURN, or a branch into another function.
    ends: bool,
    /// It writes the second register as its caller sees it.
    writes: bool,
    /// An indirect call, or a call to an address no symbol starts.
    unknown: bool,
    /// It holds a register window's `save` or `restore`.
    windowed: bool,
    /// Its first instruction stores the second register: it saves the
    /// argument registers to give them back (`push {r0-r3,lr}`).
    saves: bool,
    /// The functions it calls or branches into.
    calls: Vec<u64>,
}

/// Each callee's callers: does one read the second register, and which
/// callers hand the pair back untouched.
#[derive(Clone, Debug, Default)]
struct Facts {
    read: bool,
    returned: Vec<u64>,
}

/// The call sites and function extents of one image.
pub(super) struct Index {
    code: Rc<AddrSpace>,
    /// Does the instruction set have register windows (SPARC)?
    windows: bool,
    /// `[start, end)` of each symbol, sorted.
    funcs: Vec<(u64, u64)>,
    /// `(target, site)` of each direct call to a symbol, sorted.
    calls: Vec<(u64, u64)>,
    /// The target of every direct call into the executable ranges other than
    /// itself (an object's unrelocated `bl .`), sorted: a function's entry,
    /// whether or not a symbol starts it.
    entries: Vec<u64>,
    /// The symbols gcc's hot/cold partition names `f.cold`, sorted.
    cold: Vec<u64>,
    /// The landing pads its exception tables name.
    eh: landing::Landing,
}

impl Index {
    /// Scan the image `arch` decodes; `None` without an instruction set the
    /// scan knows, executable ranges, or symbols.
    pub(super) fn build(arch: &Architecture) -> Option<Index> {
        let isa = Isa::of(&arch.archid)?;
        let code = Rc::clone(arch.manage().get_default_code_space()?);
        let ranges: &[(u64, u64)] = &arch.litpool_const;
        if ranges.is_empty() {
            return None;
        }
        let mask = if matches!(isa, Isa::Arm { .. }) { !1u64 } else { !0u64 };
        let loader = arch.translate().loader_rc();
        let mut starts = Vec::new();
        let mut cold = Vec::new();
        let (mut named, mut sjlj) = (false, false);
        {
            let l = loader.borrow();
            l.open_symbols();
            let mut rec = kuna_sleigh::loadimage::LoadImageFunc::default();
            while l.get_next_symbol(&mut rec) {
                named |= EH_NAMES.contains(&rec.name.as_slice());
                sjlj |= rec.name.starts_with(b"_Unwind_SjLj_");
                if rec.address.get_space().is_some_and(|s| Rc::ptr_eq(s, &code)) {
                    starts.push(rec.address.get_offset() & mask);
                    if rec.name.ends_with(b".cold") || rec.name.windows(6).any(|w| w == b".cold.") {
                        cold.push(rec.address.get_offset() & mask);
                    }
                }
            }
            l.close_symbols();
        }
        cold.sort_unstable();
        cold.dedup();
        let range_of = |a: u64| ranges.iter().find(|&&(lo, hi)| lo <= a && a <= hi).copied();
        starts.retain(|&s| range_of(s).is_some());
        starts.sort_unstable();
        starts.dedup();
        if starts.is_empty() {
            return None;
        }
        let funcs: Vec<(u64, u64)> = starts
            .iter()
            .enumerate()
            .filter_map(|(i, &s)| {
                let (_, hi) = range_of(s)?;
                let next = starts.get(i + 1).copied().unwrap_or(u64::MAX);
                Some((s, next.min(hi.saturating_add(1))))
            })
            .collect();
        let mut calls = Vec::new();
        for &(lo, hi) in ranges {
            let mut at = lo;
            while at <= hi {
                let limit = ((hi - at) as usize).saturating_add(1).min(CHUNK);
                let mut buf = vec![0u8; limit + 4];
                let read = (limit + 4).min(((hi - at) as usize).saturating_add(1));
                if loader.borrow_mut().load_fill(&mut buf[..read], &Address::new(Rc::clone(&code), at)).is_err() {
                    break;
                }
                buf.truncate(read);
                isa.calls(at, &buf, limit, &mut calls);
                match at.checked_add(limit as u64) {
                    Some(n) => at = n,
                    None => break,
                }
            }
        }
        let mut entries: Vec<u64> = calls.iter().filter(|c| c.0 != c.1 && range_of(c.0).is_some()).map(|c| c.0).collect();
        entries.sort_unstable();
        entries.dedup();
        calls.retain(|(t, _)| funcs.binary_search_by_key(t, |f| f.0).is_ok());
        calls.sort_unstable();
        calls.dedup();
        let eh = landing::read(arch.eh_sections.as_deref().map(Vec::as_slice), named, sjlj, &|a| range_of(a).is_some(), &mut |a, buf| {
            loader.borrow_mut().load_fill(buf, &Address::new(Rc::clone(&code), a)).is_ok()
        });
        Some(Index { code, windows: isa == Isa::Sparc, funcs, calls, entries, cold, eh })
    }

    fn start(&self, entry: u64) -> Option<(u64, u64)> {
        self.funcs.binary_search_by_key(&entry, |f| f.0).ok().map(|i| self.funcs[i])
    }

    fn containing(&self, at: u64) -> Option<(u64, u64)> {
        let i = self.funcs.partition_point(|f| f.0 <= at);
        let f = *self.funcs.get(i.checked_sub(1)?)?;
        (at < f.1).then_some(f)
    }

    /// How many call sites of `target` [`facts`] reads.
    pub(super) fn site_count(&self, target: u64) -> usize {
        self.sites(target).take(MAX_SITES).count()
    }

    fn sites(&self, target: u64) -> impl Iterator<Item = u64> + '_ {
        let lo = self.calls.partition_point(|c| c.0 < target);
        self.calls[lo..].iter().take_while(move |c| c.0 == target).map(|c| c.1)
    }

    /// Does a direct call target `at`?
    fn entry(&self, at: u64) -> bool {
        self.entries.binary_search(&at).is_ok()
    }

    fn at(&self, off: u64) -> Address {
        Address::new(Rc::clone(&self.code), off)
    }

    /// The code address a branch or call names, when it names one.
    fn target(&self, v: &VarnodeData) -> Option<u64> {
        let sp = v.space.as_ref()?;
        Rc::ptr_eq(sp, &self.code).then_some(v.offset)
    }
}

/// What `option bejoin callers` keeps per image.
#[derive(Default)]
pub(super) struct Cache {
    index: Option<Option<Index>>,
    facts: BTreeMap<u64, Facts>,
    scans: BTreeMap<u64, Scan>,
    clobbers: BTreeMap<u64, bool>,
    layouts: BTreeMap<u64, Option<Layout>>,
}

impl Cache {
    /// How many call sites of the function at `key` its caller evidence reads.
    pub(super) fn site_count(&mut self, arch: &Architecture, key: Entry) -> usize {
        if self.index.is_none() {
            self.index = Some(Index::build(arch));
        }
        match &self.index {
            Some(Some(index)) if index.code.get_index() == key.0 => index.site_count(key.1),
            _ => 0,
        }
    }

    /// What the callers of the function at `key` say, for the second register
    /// `lo`: whether one reads it after a call (the function then is proven),
    /// and the callers whose RETURN hands the pair back (it is proven when one
    /// of them is). Nothing for a function no symbol starts, that never ends
    /// or never writes the register.
    pub(super) fn evidence(&mut self, arch: &Architecture, lo: Key, key: Entry) -> (bool, Vec<Entry>) {
        if self.index.is_none() {
            self.index = Some(Index::build(arch));
        }
        let Some(Some(index)) = &self.index else { return (false, Vec::new()) };
        if index.code.get_index() != key.0 || index.start(key.1).is_none() {
            return (false, Vec::new());
        }
        let facts = match self.facts.get(&key.1) {
            Some(f) => f.clone(),
            None => {
                let f = facts(arch, index, lo, key.1, &mut self.layouts);
                self.facts.insert(key.1, f.clone());
                f
            }
        };
        if !facts.read && facts.returned.is_empty() {
            return (false, Vec::new());
        }
        if !scan(arch, index, lo, key.1, &mut self.scans).ends {
            return (false, Vec::new());
        }
        let clobbers = match self.clobbers.get(&key.1) {
            Some(&c) => c,
            None => {
                let c = clobbers(arch, index, lo, key.1, &mut self.scans);
                self.clobbers.insert(key.1, c);
                c
            }
        };
        if !clobbers {
            return (false, Vec::new());
        }
        (facts.read, facts.returned.iter().map(|&c| (key.0, c)).collect())
    }
}

/// Read every call site of `callee` ([`site_use`]).
fn facts(arch: &Architecture, index: &Index, lo: Key, callee: u64, layouts: &mut BTreeMap<u64, Option<Layout>>) -> Facts {
    let mut out = Facts::default();
    if crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, &index.at(callee)) {
        return out;
    }
    for site in index.sites(callee).take(MAX_SITES) {
        let Some(caller) = index.containing(site) else { continue };
        let Some(layout) = layouts.entry(caller.0).or_insert_with(|| layout_of(arch, index, caller)) else { continue };
        match site_use(arch, index, lo, callee, site, caller, layout) {
            Use::Read => {
                out.read = true;
                out.returned.clear();
                return out;
            }
            Use::Returned => out.returned.push(caller.0),
            Use::Dead => {}
        }
    }
    out.returned.sort_unstable();
    out.returned.dedup();
    out
}

fn is_reg(v: &VarnodeData) -> bool {
    v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR)
}

fn is_const(v: &VarnodeData) -> bool {
    v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}

fn key(v: &VarnodeData) -> Option<Key> {
    Some((v.space.as_ref()?.get_index(), v.offset, v.size))
}

/// Does `insn` copy three or more registers `size` bytes wide into others at
/// once (a register window's `save` or `restore`)? ARM's flag-setting
/// instructions copy four one-byte flags and are no window.
fn window(insn: &Insn, size: u32) -> bool {
    insn.window
        && insn
            .ops
            .iter()
            .filter(|(opc, out, ins)| {
                *opc == OpCode::CPUI_COPY
                    && out.as_ref().is_some_and(|o| is_reg(o) && o.size == size)
                    && ins.first().is_some_and(|i| is_reg(i) && i.size == size)
            })
            .count()
            >= 3
}

/// Does `insn` store two or more registers, or a register pair?
fn stores_many(insn: &Insn, size: u32) -> bool {
    let stored = || insn.ops.iter().filter(|(opc, _, _)| *opc == OpCode::CPUI_STORE).filter_map(|(_, _, ins)| ins.get(2));
    stored().any(|v| is_reg(v) && v.size > size) || stored().filter(|v| is_reg(v)).count() >= 2
}

/// Is op `opc` over `ins` in `insn` one that reads the second register `lo`,
/// on a path that took a branch since the call (`branched`)? A store of two
/// or more registers or of a register pair is not, past a branch: gcc merges
/// two stores into SPARC's `std %o0` or ARM's `stm` at a join where the
/// second word is dead on the path from the call.
fn reads(insn: &Insn, opc: OpCode, out: &Option<VarnodeData>, ins: &[VarnodeData], lo: &Key, branched: bool) -> bool {
    if matches!(
        opc,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND
    ) {
        return false;
    }
    if opc == OpCode::CPUI_COPY && out.as_ref().is_some_and(is_reg) && ins.first().is_some_and(is_reg) && window(insn, lo.2) {
        return false;
    }
    if matches!(opc, OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB) && ins.len() == 2 && ins[0] == ins[1] {
        return false;
    }
    let merged = opc == OpCode::CPUI_STORE && branched && stores_many(insn, lo.2);
    let ins = if opc == OpCode::CPUI_CBRANCH { &ins[1..] } else { ins };
    ins.iter().enumerate().any(|(i, v)| !(is_const(v) || (merged && i == 2)) && key(v).is_some_and(|k| overlaps(&k, lo)))
}

/// The bytes `k` covers.
fn bytes(k: Key) -> impl Iterator<Item = (i32, u64)> {
    (0..u64::from(k.2)).map(move |i| (k.0, k.1.wrapping_add(i)))
}

/// How a value inside one instruction depends on the second register: not
/// at all, as `c` times it plus something that does not, or otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Taint {
    Clean,
    Lin(i64),
    Dirty,
}

/// Does anything `insn` writes outside its temporaries, stores, loads from or
/// branches on depend on the value of `lo`? Dependence follows every
/// operation, except that an operation over one varnode twice (`x - x`,
/// `x < x`) depends on nothing and sums keep `lo`'s multiple, so terms that
/// cancel depend on nothing: PowerPC's `subfe 4,4,4` names `r4` and makes
/// `xer_ca - 1`.
fn depends(insn: &Insn, lo: &Key) -> bool {
    let mut vals: BTreeMap<Key, Taint> = BTreeMap::from([(*lo, Taint::Lin(1))]);
    let mut dirty: BTreeSet<(i32, u64)> = (0..u64::from(lo.2)).map(|i| (lo.0, lo.1 + i)).collect();
    let get = |v: &VarnodeData, vals: &BTreeMap<Key, Taint>, dirty: &BTreeSet<(i32, u64)>| -> Taint {
        let Some(k) = key(v).filter(|_| !is_const(v)) else { return Taint::Clean };
        match vals.get(&k) {
            Some(&t) => t,
            None if bytes(k).any(|b| dirty.contains(&b)) => Taint::Dirty,
            None => Taint::Clean,
        }
    };
    let temp = |v: &VarnodeData| v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_INTERNAL);
    let mask = |c: i64, size: u32| if size >= 8 { c } else { c & ((1i64 << (8 * size)) - 1) };
    for (opc, out, ins) in &insn.ops {
        let t: Vec<Taint> = ins.iter().map(|v| get(v, &vals, &dirty)).collect();
        let any = t.iter().any(|&x| x != Taint::Clean);
        let same = ins.len() == 2 && ins[0] == ins[1] && !is_const(&ins[0]);
        let lin = |x: Taint| match x {
            Taint::Clean => Some(0),
            Taint::Lin(c) => Some(c),
            Taint::Dirty => None,
        };
        let size = out.as_ref().map_or(0, |o| o.size);
        let r = match opc {
            OpCode::CPUI_STORE | OpCode::CPUI_LOAD | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN
                if any =>
            {
                return true;
            }
            OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_XOR
                if same =>
            {
                Taint::Clean
            }
            _ if !any => Taint::Clean,
            OpCode::CPUI_COPY => t[0],
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => match (lin(t[0]), lin(t[1])) {
                (Some(a), Some(b)) => {
                    let c = mask(if *opc == OpCode::CPUI_INT_ADD { a.wrapping_add(b) } else { a.wrapping_sub(b) }, size);
                    if c == 0 { Taint::Clean } else { Taint::Lin(c) }
                }
                _ => Taint::Dirty,
            },
            OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_NEGATE => match lin(t[0]) {
                Some(a) => Taint::Lin(mask(a.wrapping_neg(), size)),
                None => Taint::Dirty,
            },
            _ => Taint::Dirty,
        };
        if let Some(o) = out.as_ref() {
            let Some(k) = key(o) else { return true };
            vals.retain(|v, _| !overlaps(v, &k));
            vals.insert(k, r);
            for b in bytes(k) {
                if r == Taint::Clean {
                    dirty.remove(&b);
                } else {
                    dirty.insert(b);
                }
            }
            if r != Taint::Clean && !temp(o) {
                return true;
            }
        } else if any && !matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CALL) {
            return true;
        }
    }
    false
}

/// The literal `v` holds inside one instruction, from the literals its
/// earlier operations wrote (`known`).
fn literal(v: &VarnodeData, known: &BTreeMap<Key, u64>) -> Option<u64> {
    if is_const(v) {
        return Some(v.offset);
    }
    known.get(&key(v)?).copied()
}

/// Record what op `opc` writes into `known` when its inputs are literals.
fn track(opc: OpCode, out: &Option<VarnodeData>, ins: &[VarnodeData], known: &mut BTreeMap<Key, u64>) {
    let Some(k) = out.as_ref().and_then(key) else { return };
    let a = ins.first().and_then(|v| literal(v, known));
    let b = ins.get(1).and_then(|v| literal(v, known));
    let v = match (opc, a, b) {
        (OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT, Some(a), _) => Some(a),
        (OpCode::CPUI_INT_EQUAL, Some(a), Some(b)) => Some((a == b) as u64),
        (OpCode::CPUI_INT_NOTEQUAL, Some(a), Some(b)) => Some((a != b) as u64),
        (OpCode::CPUI_BOOL_NEGATE, Some(a), _) => Some((a == 0) as u64),
        _ => None,
    };
    match v {
        Some(v) => {
            known.insert(k, v);
        }
        None => {
            known.remove(&k);
        }
    }
}

/// The literals the context holds for the registers it tracks at `entry`
/// (SPARC's `DECOMPILE_MODE`, which its window macros branch on).
fn tracked(arch: &Architecture, index: &Index, entry: u64) -> BTreeMap<Key, u64> {
    arch.with_context_db(|db| db.get_tracked_set(&index.at(entry)).clone())
        .iter()
        .filter_map(|t| Some(((t.loc.space.as_ref()?.get_index(), t.loc.offset, t.loc.size), t.val)))
        .collect()
}

/// Where the paths through one instruction's operations go, given the
/// literals `known` holds at its start: each operation some path reaches, the
/// code addresses a reached branch may go to, and whether a path falls off
/// the end. `stops` ends a path at a call.
fn reached(insn: &Insn, known: &BTreeMap<Key, u64>, index: &Index, stops: &dyn Fn(u64) -> bool) -> (Vec<usize>, Vec<u64>, bool) {
    let n = insn.ops.len();
    let mut seen = vec![false; n];
    let (mut ops, mut targets, mut falls) = (Vec::new(), Vec::new(), false);
    let mut stack = vec![(0usize, known.clone())];
    while let Some((mut op, mut known)) = stack.pop() {
        loop {
            if op >= n {
                falls = true;
                break;
            }
            if seen[op] {
                break;
            }
            seen[op] = true;
            ops.push(op);
            let (opc, out, ins) = &insn.ops[op];
            match opc {
                OpCode::CPUI_RETURN | OpCode::CPUI_BRANCHIND => break,
                OpCode::CPUI_CALL if ins.first().and_then(|v| index.target(v)).is_some_and(stops) => break,
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let taken = if *opc == OpCode::CPUI_CBRANCH {
                        ins.get(1).and_then(|c| literal(c, &known)).map(|c| c != 0)
                    } else {
                        Some(true)
                    };
                    let dest = &ins[0];
                    if is_const(dest) {
                        let to = op as i64 + sext(dest.offset, 8 * dest.size.clamp(1, 8));
                        if to < 0 || to > n as i64 {
                            break;
                        }
                        match taken {
                            Some(true) => op = to as usize,
                            Some(false) => op += 1,
                            None => {
                                stack.push((to as usize, known.clone()));
                                op += 1;
                            }
                        }
                        continue;
                    }
                    if taken != Some(false) {
                        targets.extend(index.target(dest));
                    }
                    if taken == Some(true) {
                        break;
                    }
                    op += 1;
                }
                _ => {
                    track(*opc, out, ins, &mut known);
                    op += 1;
                }
            }
        }
    }
    ops.sort_unstable();
    (ops, targets, falls)
}

/// A branch condition over registers, as one instruction computes it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cond {
    Reg(Key),
    Lit(u64),
    Not(Box<Cond>),
    And(Box<Cond>, Box<Cond>),
    Or(Box<Cond>, Box<Cond>),
    Eq(Box<Cond>, Box<Cond>),
    Unknown,
}

impl Cond {
    fn mentions(&self, k: &Key) -> bool {
        match self {
            Cond::Reg(r) => overlaps(r, k),
            Cond::Lit(_) | Cond::Unknown => false,
            Cond::Not(a) => a.mentions(k),
            Cond::And(a, b) | Cond::Or(a, b) | Cond::Eq(a, b) => a.mentions(k) || b.mentions(k),
        }
    }

    /// Does the condition test a flag (a one-byte register) nothing in `set`
    /// wrote?
    fn stale(&self, set: &BTreeSet<Key>) -> bool {
        match self {
            Cond::Reg(r) => r.2 == 1 && !set.iter().any(|w| overlaps(w, r)),
            Cond::Lit(_) | Cond::Unknown => false,
            Cond::Not(a) => a.stale(set),
            Cond::And(a, b) | Cond::Or(a, b) | Cond::Eq(a, b) => a.stale(set) || b.stale(set),
        }
    }
}

/// What one path learned from the branches it took: the values of one-byte
/// registers (the flags a conditional instruction tests) and the outcome of
/// each condition it decided, until a write of a register they read, and the
/// one-byte registers written since the call (`set`).
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct PathFacts {
    regs: BTreeMap<Key, u64>,
    conds: BTreeMap<Cond, bool>,
    set: BTreeSet<Key>,
}

impl PathFacts {
    fn value(&self, c: &Cond) -> Option<u64> {
        match c {
            Cond::Lit(v) => Some(*v),
            Cond::Reg(k) => self.regs.get(k).copied(),
            _ => self.truth(c).map(u64::from),
        }
    }

    fn truth(&self, c: &Cond) -> Option<bool> {
        if let Some(&v) = self.conds.get(c) {
            return Some(v);
        }
        match c {
            Cond::Unknown => None,
            Cond::Lit(_) | Cond::Reg(_) => self.value(c).map(|v| v != 0),
            Cond::Not(a) => self.truth(a).map(|v| !v),
            Cond::And(a, b) => match (self.truth(a), self.truth(b)) {
                (Some(false), _) | (_, Some(false)) => Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            },
            Cond::Or(a, b) => match (self.truth(a), self.truth(b)) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            },
            Cond::Eq(a, b) => match (self.value(a), self.value(b)) {
                (Some(x), Some(y)) => Some(x == y),
                _ => self.conds.get(&Cond::Eq(b.clone(), a.clone())).copied(),
            },
        }
    }

    /// Record that `c` came out `v` on this path.
    fn learn(&mut self, c: &Cond, v: bool) {
        if matches!(c, Cond::Unknown) {
            return;
        }
        self.conds.insert(c.clone(), v);
        match c {
            Cond::Reg(k) if k.2 == 1 => {
                self.regs.insert(*k, v as u64);
            }
            Cond::Not(a) => self.learn(a, !v),
            Cond::And(a, b) if v => {
                self.learn(a, true);
                self.learn(b, true);
            }
            Cond::Or(a, b) if !v => {
                self.learn(a, false);
                self.learn(b, false);
            }
            Cond::Eq(a, b) if v => {
                for (r, other) in [(a, b), (b, a)] {
                    if let (Cond::Reg(k), Some(x)) = (r.as_ref(), self.value(other)) {
                        if k.2 == 1 && self.value(r).is_none() {
                            self.regs.insert(*k, x);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Forget what a write of `k` changes.
    fn forget(&mut self, k: &Key) {
        self.regs.retain(|r, _| !overlaps(r, k));
        self.conds.retain(|c, _| !c.mentions(k));
    }
}

/// The condition `v` holds inside one instruction: what its earlier
/// operations derived it from (`deriv`), down to registers and literals.
fn cond_of(v: &VarnodeData, deriv: &BTreeMap<Key, Cond>) -> Cond {
    if is_const(v) {
        return Cond::Lit(v.offset);
    }
    let Some(k) = key(v) else { return Cond::Unknown };
    if let Some(c) = deriv.get(&k) {
        return c.clone();
    }
    if is_reg(v) {
        Cond::Reg(k)
    } else {
        Cond::Unknown
    }
}

/// Record in `deriv` the condition op `opc` writes into a temporary, and in
/// `facts` that a register it writes no longer holds what the path learned.
fn derive(opc: OpCode, out: &Option<VarnodeData>, ins: &[VarnodeData], deriv: &mut BTreeMap<Key, Cond>, facts: &mut PathFacts) {
    let Some(o) = out.as_ref() else { return };
    let Some(k) = key(o) else { return };
    let arg = |i: usize| ins.get(i).map_or(Cond::Unknown, |v| cond_of(v, deriv));
    let c = match opc {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => arg(0),
        OpCode::CPUI_BOOL_NEGATE => Cond::Not(Box::new(arg(0))),
        OpCode::CPUI_BOOL_AND => Cond::And(Box::new(arg(0)), Box::new(arg(1))),
        OpCode::CPUI_BOOL_OR => Cond::Or(Box::new(arg(0)), Box::new(arg(1))),
        OpCode::CPUI_INT_EQUAL => Cond::Eq(Box::new(arg(0)), Box::new(arg(1))),
        OpCode::CPUI_INT_NOTEQUAL => Cond::Not(Box::new(Cond::Eq(Box::new(arg(0)), Box::new(arg(1))))),
        _ => Cond::Unknown,
    };
    if is_reg(o) {
        facts.forget(&k);
        if k.2 == 1 {
            facts.set.insert(k);
        }
        deriv.remove(&k);
        if let (Cond::Lit(v), 1) = (&c, k.2) {
            facts.regs.insert(k, *v);
        }
    } else if matches!(c, Cond::Unknown) {
        deriv.remove(&k);
    } else {
        deriv.insert(k, c);
    }
}

/// The delay slot of a conditional branch at `at` (SPARC, MIPS), decoded on
/// its own. A delay slot filler may move an instruction there from the branch
/// target, so what it reads holds on that path only (gcc fills one with the
/// target's `std %o0`, whose `%o1` is dead on the path that falls through).
fn delay_slot(arch: &Architecture, index: &Index, at: u64, insn: &Insn) -> Option<Insn> {
    let conditional = insn.ops.iter().any(|(opc, _, ins)| *opc == OpCode::CPUI_CBRANCH && ins.first().is_some_and(|d| !is_const(d)));
    if !conditional || insn.len <= 4 {
        return None;
    }
    decode(arch, &index.at(at + insn.len - 4))
}

/// Walk the caller at `caller` from the call to `callee` at `site`: does an
/// instruction read the second register `lo` before anything writes it or
/// another call clobbers it, or does it reach the caller's RETURN untouched?
/// Each path keeps what its branches decided about the flags, so a
/// conditional instruction that skips a write of `lo` skips the later reads
/// under the same condition too (`ldrne r1,[sp,#0x1c]; orrnes r2,r1,r0`). A
/// site that does not decode to that call says nothing, nor does one whose
/// fall-through reaches, before any branch, an instruction another branch of
/// the caller targets or a direct call's target ([`Layout`]), one whose walk
/// reaches a word the caller loads as data, or one after which a branch tests
/// a flag nothing set since the call: the call need not return there. A path
/// that branches to a direct call's target is a tail call and ends there.
fn site_use(arch: &Architecture, index: &Index, lo: Key, callee: u64, site: u64, caller: (u64, u64), layout: &Layout) -> Use {
    let Some(insn) = decode(arch, &index.at(site)) else { return Use::Dead };
    let Some(k) = insn
        .ops
        .iter()
        .position(|(opc, _, ins)| *opc == OpCode::CPUI_CALL && ins.first().and_then(|v| index.target(v)) == Some(callee))
    else {
        return Use::Dead;
    };
    let mut entry = PathFacts::default();
    for (key, v) in tracked(arch, index, caller.0) {
        if key.2 == 1 {
            entry.regs.insert(key, v);
            entry.set.insert(key);
        }
    }
    let mut facts = entry.clone();
    let mut deriv = BTreeMap::new();
    for (opc, out, ins) in &insn.ops[..k] {
        derive(*opc, out, ins, &mut deriv, &mut facts);
    }
    type Item = (u64, Rc<Insn>, usize, PathFacts, BTreeMap<Key, Cond>, bool);
    let mut todo: Vec<Item> = vec![(site, Rc::new(insn), k + 1, facts, deriv, false)];
    let mut seen: BTreeSet<(u64, bool, PathFacts)> = BTreeSet::new();
    let mut insns: BTreeMap<u64, Option<Rc<Insn>>> = BTreeMap::new();
    let mut result = Use::Dead;
    let mut visit = |t: u64, f: PathFacts, branched: bool, seen: &mut BTreeSet<(u64, bool, PathFacts)>, todo: &mut Vec<Item>| {
        if t > caller.0 && t < caller.1 && seen.len() < MAX_SITE_INSNS && seen.insert((t, branched, f.clone())) {
            if let Some(i) = insns.entry(t).or_insert_with(|| decode(arch, &index.at(t)).map(Rc::new)) {
                todo.push((t, Rc::clone(i), 0, f, BTreeMap::new(), branched));
            }
        }
    };
    while let Some((at, insn, mut op, mut facts, mut deriv, branched)) = todo.pop() {
        if at != site && op == 0 {
            let end = at + insn.len.max(1);
            if layout.data.range(at..end).next().is_some() || index.eh.pad_in(at, end) || (!branched && layout.joins.contains(&at)) {
                return Use::Dead;
            }
            if index.entry(at) {
                if !branched {
                    return Use::Dead;
                }
                continue;
            }
        }
        let mut fall = true;
        let mut forked = branched;
        let counts = |i: &Insn| i.ops.iter().filter(|(opc, out, ins)| reads(i, *opc, out, ins, &lo, branched)).count();
        let slot = delay_slot(arch, index, at, &insn).map_or(0, |d| counts(&d));
        let genuine = slot == 0 || counts(&insn) > slot;
        let mut dep = None;
        while op < insn.ops.len() {
            let (opc, out, ins) = &insn.ops[op];
            op += 1;
            if genuine && reads(&insn, *opc, out, ins, &lo, branched) && (at == site || *dep.get_or_insert_with(|| depends(&insn, &lo))) {
                return Use::Read;
            }
            if matches!(opc, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                || out.as_ref().and_then(key).is_some_and(|o| overlaps(&o, &lo))
            {
                fall = false;
                break;
            }
            match opc {
                OpCode::CPUI_RETURN => {
                    result = Use::Returned;
                    fall = false;
                    break;
                }
                OpCode::CPUI_BRANCHIND => {
                    fall = false;
                    break;
                }
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let c = if *opc == OpCode::CPUI_CBRANCH {
                        ins.get(1).map_or(Cond::Unknown, |v| cond_of(v, &deriv))
                    } else {
                        Cond::Lit(1)
                    };
                    if c.stale(&facts.set) {
                        return Use::Dead;
                    }
                    let taken = facts.truth(&c);
                    if taken == Some(false) {
                        continue;
                    }
                    let mut there = facts.clone();
                    if taken.is_none() {
                        there.learn(&c, true);
                        facts.learn(&c, false);
                    }
                    let dest = &ins[0];
                    if is_const(dest) {
                        let to = (op - 1) as i64 + sext(dest.offset, 8 * dest.size.clamp(1, 8));
                        if to < 0 || to > insn.ops.len() as i64 {
                            fall = false;
                            break;
                        }
                        if taken == Some(true) {
                            op = to as usize;
                        } else {
                            todo.push((at, Rc::clone(&insn), to as usize, there, deriv.clone(), branched));
                        }
                        continue;
                    }
                    let next = at + insn.len;
                    if let Some(t) = index.target(dest) {
                        visit(t, there, branched || t != next, &mut seen, &mut todo);
                    }
                    if taken == Some(true) {
                        fall = false;
                        break;
                    }
                    forked |= index.target(dest) != Some(next);
                }
                _ => derive(*opc, out, ins, &mut deriv, &mut facts),
            }
        }
        if fall {
            visit(at + insn.len, facts, forked, &mut seen, &mut todo);
        }
    }
    result
}

/// Scan the function at `entry` once per image (see [`Scan`]).
fn scan<'a>(arch: &Architecture, index: &Index, lo: Key, entry: u64, cache: &'a mut BTreeMap<u64, Scan>) -> &'a Scan {
    cache.entry(entry).or_insert_with(|| scan_function(arch, index, lo, entry))
}

fn scan_function(arch: &Architecture, index: &Index, lo: Key, entry: u64) -> Scan {
    let mut out = Scan::default();
    let Some((start, end)) = index.start(entry) else { return out };
    let base = tracked(arch, index, start);
    let noreturn = |t: u64| crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, &index.at(t));
    let mut todo = vec![start];
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let (mut direct, mut in_window) = (false, false);
    let mut restored: Vec<Key> = Vec::new();
    let mut written: Vec<Key> = Vec::new();
    while let Some(at) = todo.pop() {
        if seen.len() >= MAX_SCAN_INSNS || !seen.insert(at) {
            continue;
        }
        let Some(insn) = decode(arch, &index.at(at)) else { continue };
        let win = window(&insn, lo.2);
        out.windowed |= win;
        if at == start {
            out.saves = insn.ops.iter().any(|(opc, _, ins)| *opc == OpCode::CPUI_STORE && ins.get(2).and_then(key).is_some_and(|k| overlaps(&k, &lo)));
        }
        let (ops, targets, falls) = reached(&insn, &base, index, &noreturn);
        for i in ops {
            let (opc, o, ins) = &insn.ops[i];
            if let Some(k) = o.as_ref().and_then(key) {
                if win && *opc == OpCode::CPUI_COPY && ins.first().is_some_and(is_reg) && o.as_ref().is_some_and(is_reg) {
                    if overlaps(&k, &lo) {
                        restored.extend(ins.first().and_then(key));
                    }
                } else {
                    if overlaps(&k, &lo) {
                        direct = true;
                        in_window |= win;
                    }
                    if o.as_ref().is_some_and(is_reg) && !written.contains(&k) {
                        written.push(k);
                    }
                }
            }
            match opc {
                OpCode::CPUI_CALL => match ins.first().and_then(|v| index.target(v)) {
                    Some(t) if index.start(t).is_some() => out.calls.push(t),
                    _ => out.unknown = true,
                },
                OpCode::CPUI_CALLIND => out.unknown = true,
                OpCode::CPUI_RETURN => out.ends = true,
                _ => {}
            }
        }
        for t in targets {
            if t > start && t < end {
                todo.push(t);
            } else if t != start && index.start(t).is_some() {
                out.ends = true;
                out.calls.push(t);
            }
        }
        if falls && at + insn.len < end {
            todo.push(at + insn.len);
        }
        if !index.windows && out.ends && direct {
            break;
        }
    }
    out.writes = if out.windowed { in_window || written.iter().any(|k| restored.iter().any(|r| overlaps(k, r))) } else { direct };
    out.calls.sort_unstable();
    out.calls.dedup();
    out
}

/// What a walk after a call in one function must not take for its
/// continuation ([`layout_of`]).
#[derive(Debug, Default)]
struct Layout {
    /// The instructions a branch of the function targets.
    joins: BTreeSet<u64>,
    /// The bytes inside the function it reads as data (a literal pool).
    data: BTreeSet<u64>,
}

/// The instructions of the function `func` that a branch of its own targets,
/// other than the next instruction (a conditional instruction's skip), and
/// the bytes of its own extent it reads (an ARM literal pool's words); `None`
/// when it jumps through a register (a jump table, whose targets the walk
/// does not know), when it is the `.cold` part of a function (whose blocks
/// the hot part enters), when its landing pads are not known (its exception
/// data does not decode, or the image's cannot be placed), or when it is too
/// long to walk.
fn layout_of(arch: &Architecture, index: &Index, func: (u64, u64)) -> Option<Layout> {
    let (start, end) = func;
    if index.cold.binary_search(&start).is_ok() || index.eh.blind || index.eh.unknown_in(start, end) {
        return None;
    }
    let base = tracked(arch, index, start);
    let noreturn = |t: u64| crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, &index.at(t));
    let mut todo = vec![start];
    let mut seen: BTreeSet<u64> = BTreeSet::new();
    let mut out = Layout::default();
    while let Some(at) = todo.pop() {
        if !seen.insert(at) {
            continue;
        }
        if seen.len() > MAX_SCAN_INSNS {
            return None;
        }
        let Some(insn) = decode(arch, &index.at(at)) else { continue };
        let (ops, targets, falls) = reached(&insn, &base, index, &noreturn);
        if ops.iter().any(|&i| insn.ops[i].0 == OpCode::CPUI_BRANCHIND) {
            return None;
        }
        for &i in &ops {
            let (opc, _, ins) = &insn.ops[i];
            let skip = usize::from(matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL));
            for v in ins.iter().skip(skip) {
                if let Some(a) = index.target(v).filter(|&a| a >= start && a < end) {
                    out.data.extend(a..a.saturating_add(u64::from(v.size)).min(end));
                }
            }
        }
        let next = at + insn.len;
        for t in targets {
            if t > start && t < end {
                if t != next {
                    out.joins.insert(t);
                }
                todo.push(t);
            }
        }
        if falls && next < end {
            todo.push(next);
        }
    }
    Some(out)
}

/// Does the function at `entry` write the second register as its caller sees
/// it: itself, by an indirect or unknown call, or through a direct callee or
/// a function it branches into that does? A function with a register window
/// only by itself, and none that saves the register on entry.
fn clobbers(arch: &Architecture, index: &Index, lo: Key, entry: u64, cache: &mut BTreeMap<u64, Scan>) -> bool {
    let mut queue = VecDeque::from([entry]);
    let mut seen: BTreeSet<u64> = BTreeSet::from([entry]);
    while let Some(f) = queue.pop_front() {
        let s = scan(arch, index, lo, f, cache);
        if s.saves {
            if f == entry {
                return false;
            }
            continue;
        }
        if s.writes || (!s.windowed && s.unknown) {
            return true;
        }
        if s.windowed {
            continue;
        }
        for &c in &s.calls.clone() {
            if seen.insert(c) {
                if seen.len() > MAX_CLOBBER_FUNCS {
                    return false;
                }
                queue.push_back(c);
            }
        }
    }
    false
}

#[path = "landing.rs"]
mod landing;

#[cfg(test)]
#[path = "callers_tests.rs"]
mod tests;
