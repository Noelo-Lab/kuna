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
//! function that keeps a frame) proves the callee when that caller is proven.
//!
//! A call (an argument the callee may not take), a RETURN, a branch, a
//! register window's copy of every register, and a register XORed with or
//! subtracted from itself do not read it. The walk after a call keeps, on each
//! path, what the branches it took decided about the flags, so the
//! instructions a condition skips are skipped together (ARM's `ldrne r1,..;
//! orrnes r2,r1,r0` never reads a stale `r1`). A function whose first
//! instruction stores the second register (one that saves the argument
//! registers to give them back, as `__gnu_mcount_nc` does) says nothing.
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
const MAX_SITES: usize = 256;

/// How many instructions the walk after one call site visits.
const MAX_SITE_INSNS: usize = 2048;

/// How many instructions the scan of one function visits.
const MAX_SCAN_INSNS: usize = 16_384;

/// How many functions the search for a write of the second register visits.
const MAX_CLOBBER_FUNCS: usize = 32;

/// How many bytes of code one read of the image takes.
const CHUNK: usize = 1 << 20;

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
    /// `[start, end)` of each symbol, sorted.
    funcs: Vec<(u64, u64)>,
    /// `(target, site)` of each direct call to a symbol, sorted.
    calls: Vec<(u64, u64)>,
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
        {
            let l = loader.borrow();
            l.open_symbols();
            let mut rec = kuna_sleigh::loadimage::LoadImageFunc::default();
            while l.get_next_symbol(&mut rec) {
                if rec.address.get_space().is_some_and(|s| Rc::ptr_eq(s, &code)) {
                    starts.push(rec.address.get_offset() & mask);
                }
            }
            l.close_symbols();
        }
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
        calls.retain(|(t, _)| funcs.binary_search_by_key(t, |f| f.0).is_ok());
        calls.sort_unstable();
        calls.dedup();
        Some(Index { code, funcs, calls })
    }

    fn start(&self, entry: u64) -> Option<(u64, u64)> {
        self.funcs.binary_search_by_key(&entry, |f| f.0).ok().map(|i| self.funcs[i])
    }

    fn containing(&self, at: u64) -> Option<(u64, u64)> {
        let i = self.funcs.partition_point(|f| f.0 <= at);
        let f = *self.funcs.get(i.checked_sub(1)?)?;
        (at < f.1).then_some(f)
    }

    fn sites(&self, target: u64) -> impl Iterator<Item = u64> + '_ {
        let lo = self.calls.partition_point(|c| c.0 < target);
        self.calls[lo..].iter().take_while(move |c| c.0 == target).map(|c| c.1)
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
}

impl Cache {
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
        let facts = self.facts.entry(key.1).or_insert_with(|| facts(arch, index, lo, key.1)).clone();
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
fn facts(arch: &Architecture, index: &Index, lo: Key, callee: u64) -> Facts {
    let mut out = Facts::default();
    if crate::p4_calls::kuna_calleenoreturn::is_no_return(arch, &index.at(callee)) {
        return out;
    }
    for site in index.sites(callee).take(MAX_SITES) {
        let Some(caller) = index.containing(site) else { continue };
        match site_use(arch, index, lo, callee, site, caller) {
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

/// Is op `opc` over `ins` in `insn` one that reads the second register `lo`?
fn reads(insn: &Insn, opc: OpCode, out: &Option<VarnodeData>, ins: &[VarnodeData], lo: &Key) -> bool {
    if matches!(
        opc,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND
    ) {
        return false;
    }
    if insn.window && opc == OpCode::CPUI_COPY && out.as_ref().is_some_and(is_reg) && ins.first().is_some_and(is_reg) {
        return false;
    }
    if matches!(opc, OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB) && ins.len() == 2 && ins[0] == ins[1] {
        return false;
    }
    let ins = if opc == OpCode::CPUI_CBRANCH { &ins[1..] } else { ins };
    ins.iter().any(|v| !is_const(v) && key(v).is_some_and(|k| overlaps(&k, lo)))
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

/// Where a path through one instruction's operations from op `from` goes,
/// the literals `known` holds at its start: each operation some path reaches,
/// the code addresses a reached branch may go to, and whether a path falls
/// off the end. `stops` ends a path at a call.
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
}

/// What one path learned from the branches it took: the values of one-byte
/// registers (the flags a conditional instruction tests) and the outcome of
/// each condition it decided, until a write of a register they read.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct PathFacts {
    regs: BTreeMap<Key, u64>,
    conds: BTreeMap<Cond, bool>,
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

/// Walk the caller at `caller` from the call to `callee` at `site`: does an
/// instruction read the second register `lo` before anything writes it or
/// another call clobbers it, or does it reach the caller's RETURN untouched?
/// Each path keeps what its branches decided about the flags, so a
/// conditional instruction that skips a write of `lo` skips the later reads
/// under the same condition too (`ldrne r1,[sp,#0x1c]; orrnes r2,r1,r0`). A
/// site that does not decode to that call says nothing.
fn site_use(arch: &Architecture, index: &Index, lo: Key, callee: u64, site: u64, caller: (u64, u64)) -> Use {
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
        }
    }
    let mut facts = entry.clone();
    let mut deriv = BTreeMap::new();
    for (opc, out, ins) in &insn.ops[..k] {
        derive(*opc, out, ins, &mut deriv, &mut facts);
    }
    type Item = (u64, Rc<Insn>, usize, PathFacts, BTreeMap<Key, Cond>);
    let mut todo: Vec<Item> = vec![(site, Rc::new(insn), k + 1, facts, deriv)];
    let mut seen: BTreeSet<(u64, PathFacts)> = BTreeSet::new();
    let mut result = Use::Dead;
    let visit = |t: u64, f: PathFacts, seen: &mut BTreeSet<(u64, PathFacts)>, todo: &mut Vec<Item>| {
        if t > caller.0 && t < caller.1 && seen.len() < MAX_SITE_INSNS && seen.insert((t, f.clone())) {
            if let Some(i) = decode(arch, &index.at(t)) {
                todo.push((t, Rc::new(i), 0, f, BTreeMap::new()));
            }
        }
    };
    while let Some((at, insn, mut op, mut facts, mut deriv)) = todo.pop() {
        let mut fall = true;
        while op < insn.ops.len() {
            let (opc, out, ins) = &insn.ops[op];
            op += 1;
            if reads(&insn, *opc, out, ins, &lo) {
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
                            todo.push((at, Rc::clone(&insn), to as usize, there, deriv.clone()));
                        }
                        continue;
                    }
                    if let Some(t) = index.target(dest) {
                        visit(t, there, &mut seen, &mut todo);
                    }
                    if taken == Some(true) {
                        fall = false;
                        break;
                    }
                }
                _ => derive(*opc, out, ins, &mut deriv, &mut facts),
            }
        }
        if fall {
            visit(at + insn.len, facts, &mut seen, &mut todo);
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
        out.windowed |= insn.window;
        if at == start {
            out.saves = insn.ops.iter().any(|(opc, _, ins)| *opc == OpCode::CPUI_STORE && ins.get(2).and_then(key).is_some_and(|k| overlaps(&k, &lo)));
        }
        let (ops, targets, falls) = reached(&insn, &base, index, &noreturn);
        for i in ops {
            let (opc, o, ins) = &insn.ops[i];
            if let Some(k) = o.as_ref().and_then(key) {
                if insn.window && *opc == OpCode::CPUI_COPY && ins.first().is_some_and(is_reg) && o.as_ref().is_some_and(is_reg) {
                    if overlaps(&k, &lo) {
                        restored.extend(ins.first().and_then(key));
                    }
                } else {
                    if overlaps(&k, &lo) {
                        direct = true;
                        in_window |= insn.window;
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
    }
    out.writes = if out.windowed { in_window || written.iter().any(|k| restored.iter().any(|r| overlaps(k, r))) } else { direct };
    out.calls.sort_unstable();
    out.calls.dedup();
    out
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

#[cfg(test)]
#[path = "callers_tests.rs"]
mod tests;
