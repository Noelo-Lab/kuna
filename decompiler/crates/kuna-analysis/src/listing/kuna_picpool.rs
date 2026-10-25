//! (kuna) `picpool` — compose a literal-pool word with the PC that adds it, so a
//! position-independent string reference has an owning function.
//!
//! # The gap
//!
//! [`super::kuna_poolref`] follows a pool word as an **absolute** pointer. In a
//! position-independent ARM image the pool holds no pointer at all: it holds the
//! signed distance from the `add` that consumes it to the datum, and the address
//! only exists once the two are put together.
//!
//! ```text
//! 0x660  ldr r0,[0x6a0]     ; r0 = the word at 0x6a0 = 0xfffffe3f
//! 0x664  add r0,pc,r0       ; r0 = 0x66c + (-0x1c1) = 0x4ab, "Benar! Flag: ..."
//! 0x668  bl  printf
//! ...
//! 0x6a0  3f fe ff ff
//! ```
//!
//! `poolref` correctly declines 0xfffffe3f — it is not an address and lands in no
//! section — so nothing in the program references 0x4ab, `kuna xrefs --to` it
//! answers zero and `kuna strings --json` reports it with no owning function, on
//! a binary whose disassembly plainly shows which function prints it. The idiom
//! is not one site either: the filing image forms every one of its four string
//! references this way.
//!
//! This is [`super::kuna_picbase`]'s defect with a per-site base instead of a
//! module-wide one. There the PIC base is a register a prologue establishes once
//! and the whole function inherits; here each reference carries its own base —
//! the PC of its own `add` — so there is nothing to detect once and nothing to
//! scope.
//!
//! # The rule
//!
//! Two decode-time constants, composed, and nothing else. A pool word is admitted
//! as a *displacement* only when
//!
//!  * it comes out of the same read-only, pointer-sized, pointer-aligned slot
//!    [`PoolImage`] already vouches for — the image's copy of it is what runs;
//!  * whatever the word itself looks like: in an image mapped low (a non-PIE
//!    ARM executable at 0x10000) a displacement often lands in a mapped section
//!    by chance, and only the PC fold below decides that it is one;
//!  * a later instruction in the same straight-line run folds it, through
//!    `COPY`/`INT_ADD`/`INT_SUB` alone, together with a constant this
//!    instruction materialized from its **own address** (SLEIGH spells a PC read
//!    that way: `inst_start + 8` on A32, `+ 4` on Thumb, the fall-through on
//!    x86);
//!  * and the folded value lands in a **register** and in a mapped section.
//!
//! Then the composed address is filed as [`XrefKind::Data`](super::xrefs::XrefKind::Data)
//! from the instruction that completed it — the address-taken case, which is
//! exactly what the `add` did, and attributing it to an instruction rather than
//! to the pool word is what gives the reference an owning function.
//!
//! `ScalarOperandAnalyzer.checkOperands`' "below 4096 could be a number" floor
//! is deliberately **not** applied to the composed value. That floor asks whether
//! an *immediate* is an address or an integer; here the question is already
//! settled by the PC, and applying it would discard every reference in an image
//! mapped low — which the filing Android PIE is, its whole layout living under
//! 0x2828.
//!
//! # GOT bases
//!
//! ARM `-fPIC` code makes the GOT address once per function with the same pair
//! (`ldr r4,[pc,#a]; add r4,pc,r4`), keeps it in a callee-saved register, and
//! reads slots through it (`ldr r2,[pc,#b]; ldr r2,[r4,r2]`). A composed value
//! that lands in `.got` is therefore tracked as a base along every edge of the
//! body, and across calls when its register is preserved, until the register is
//! written. A load through base plus a pool word is filed as a
//! [`XrefKind::Read`] of the slot, and the slot's file word, when it is a mapped
//! address, as a [`XrefKind::Data`] reference: in a non-PIE executable the
//! linker writes a local function's address there with no relocation.
//!
//! # Joins
//!
//! A tracked value holds at an instruction only if every predecessor in the body
//! hands it the same one; the function entry's caller hands it nothing. The walk
//! cannot apply that rule as it goes, since it decodes a join before it has seen
//! every edge into it, so [`PicPool::finish`] applies it to the recorded body
//! and answers only what survives.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use object::read::{Object, ObjectSection};

use super::kuna_poolref::PoolImage;
use super::xrefs::{in_range, looks_like_address, FullOp, XrefKind};

/// The file bytes of `.got`, which [`PoolImage`] leaves out because the section
/// is writable. A slot for a local function holds its address in a non-PIE
/// executable and the same word as an in-place `R_ARM_RELATIVE` addend in a PIE;
/// an import's slot holds 0 and names nothing.
pub(super) struct GotImage<'a> {
    lo: u64,
    hi: u64,
    data: &'a [u8],
    ptr: u32,
    little_endian: bool,
}

impl<'a> GotImage<'a> {
    pub(super) fn new(file: &'a object::File<'a>) -> Option<Self> {
        let sec = file.section_by_name(".got")?;
        let lo = sec.address();
        let data = sec.data().ok()?;
        if lo == 0 || data.is_empty() {
            return None;
        }
        let hi = lo.saturating_add(sec.size().min(data.len() as u64));
        Some(GotImage { lo, hi, data, ptr: if file.is_64() { 8 } else { 4 }, little_endian: file.is_little_endian() })
    }

    fn contains(&self, vma: u64) -> bool {
        vma >= self.lo && vma < self.hi
    }

    /// The pointer-sized, pointer-aligned slot word at `vma`.
    fn word_at(&self, vma: u64) -> Option<u64> {
        if !self.contains(vma) || vma % u64::from(self.ptr) != 0 {
            return None;
        }
        let off = usize::try_from(vma - self.lo).ok()?;
        let b = self.data.get(off..off.checked_add(self.ptr as usize)?)?;
        Some(match (self.ptr, self.little_endian) {
            (8, true) => u64::from_le_bytes(b.try_into().ok()?),
            (8, false) => u64::from_be_bytes(b.try_into().ok()?),
            (_, true) => u64::from(u32::from_le_bytes(b.try_into().ok()?)),
            (_, false) => u64::from(u32::from_be_bytes(b.try_into().ok()?)),
        })
    }
}

/// How many instructions a pool value may travel before the `add` that composes
/// it. GCC and Clang emit the pair adjacent (the displacement is measured from
/// the `add`'s own label, so the two are one template), but instruction
/// scheduling separates them when several references are formed at once. Bounded
/// because every hop is one more instruction over which the tracked register is
/// believed.
const MAX_HOPS: u8 = 8;

/// How far past an instruction's own address a constant may lie and still be its
/// materialized PC. A32 reads `pc` as `inst_start + 8`, Thumb as `+ 4`, x86 `rip`
/// as the fall-through; nothing needs more slack than that.
const PC_WINDOW: u64 = 16;

/// How many pool values may be tracked into one instruction. A join point can be
/// reached from several predecessors, each carrying its own; the cap keeps that
/// from accumulating on a pathological CFG.
const MAX_TRACKED: usize = 8;

/// A pool word in flight: the register holding it, its value, and how far it has
/// travelled from the load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pending {
    /// Register-space offset and size of the varnode holding the word.
    off: u64,
    size: u32,
    /// The word, zero-extended out of the image. The composition is a wrapping
    /// add masked to the register width, so a negative displacement needs no
    /// separate sign extension.
    word: u64,
    hops: u8,
    /// The pool load the word came from; 0 for a GOT base.
    origin: u64,
}

/// The pool values one function's walk is carrying, keyed by the address of the
/// instruction they reach.
///
/// Keyed by address rather than held as "the previous instruction's state"
/// because [`super::xrefs::build_with_focus`] walks a function breadth-first: at
/// a conditional branch the queue interleaves the two successors, so "the
/// instruction decoded before this one" is not the instruction that precedes it.
///
/// The same order means an instruction can be decoded before every edge into it
/// is known, so what the walk carries is only a candidate. Each instruction that
/// sees a tracked value is recorded as a [`Node`], every edge as it is taken, and
/// [`PicPool::finish`] keeps a value at an instruction only where every
/// predecessor delivers the same one (a meet), before it answers any address.
#[derive(Default)]
pub(super) struct PicPool {
    at: HashMap<u64, Vec<Pending>>,
    /// GOT bases, keyed by the instruction they reach like `at`, but carried
    /// along every edge with no hop bound.
    bases_at: HashMap<u64, Vec<Pending>>,
    nodes: Vec<Node>,
    /// `(from, to, fall_through)`.
    edges: Vec<(u64, u64, bool)>,
}

/// One instruction's effect on the values it was handed: `ins` are the values,
/// and every value it passes on and every address it forms names, as a bit mask
/// over `ins`, the values it was computed from.
struct Node {
    vma: u64,
    ins: Vec<Fact>,
    outs: Vec<(Fact, u32)>,
    refs: Vec<(u64, XrefKind, u32, u64)>,
}

/// A tracked value. A GOT base travels every edge; a pool word only the
/// fall-through one.
#[derive(Debug, Clone, Copy)]
struct Fact {
    p: Pending,
    base: bool,
}

impl Fact {
    fn same(&self, o: &Fact) -> bool {
        self.base == o.base && self.p.off == o.p.off && self.p.size == o.p.size && self.p.word == o.p.word
    }
}

/// What [`PicPool::finish`] answers for one body.
#[derive(Default)]
pub(super) struct Verdict {
    /// `(from, to, kind)`.
    pub refs: Vec<(u64, u64, XrefKind)>,
    /// Pool loads whose word a PC fold consumed: a displacement, never a pointer.
    pub displaced: HashSet<u64>,
}

impl PicPool {
    /// Consume one decoded instruction, and answer the pool word it loaded and
    /// carries on, whose [`super::kuna_poolref`] pointer edge waits for
    /// [`Verdict::displaced`].
    ///
    /// Called for every instruction of the function in whatever order the walk
    /// reaches them, with its static successors.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn step(
        &mut self,
        vma: u64,
        len: u32,
        ops: &[FullOp],
        pool: &PoolImage,
        mapped: &[(u64, u64)],
        data_space: Option<&Rc<AddrSpace>>,
        got: Option<&GotImage>,
        preserved: &[(u64, u64)],
        branch_targets: &[u64],
    ) -> Option<u64> {
        let falls = !ops.iter().any(|op| {
            matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN)
        });
        let next = vma.wrapping_add(u64::from(len));
        if falls {
            self.edges.push((vma, next, true));
        }
        // A predicated instruction branches to its own fall-through, which is
        // still the fall-through edge.
        self.edges.extend(
            branch_targets.iter().filter(|&&to| !(falls && to == next)).map(|&to| (vma, to, false)),
        );

        // Hashing every instruction's address would be this walk's whole cost on
        // an architecture that never forms one of these pairs; nothing is in
        // flight until a pool load puts it there.
        let pending =
            if self.at.is_empty() { Vec::new() } else { self.at.remove(&vma).unwrap_or_default() };
        let bases =
            if self.bases_at.is_empty() { Vec::new() } else { self.bases_at.remove(&vma).unwrap_or_default() };
        let fresh = seed(vma, ops, pool, data_space);
        if pending.is_empty() && bases.is_empty() && fresh.is_none() {
            return None;
        }
        let ins: Vec<Fact> = bases
            .iter()
            .map(|&p| Fact { p, base: true })
            .chain(pending.iter().map(|&p| Fact { p, base: false }))
            .collect();
        let ev = evaluate(vma, ops, &ins, mapped, data_space, got);
        let calls = ops.iter().any(|op| matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND));
        let kept = |f: &Fact| {
            !ev.written.iter().any(|&(off, size)| spans_overlap(f.p.off, f.p.size, off, size))
                && (!calls || survives_calls(&f.p, preserved))
        };

        let mut outs: Vec<(Fact, u32)> = Vec::new();
        for (i, f) in ins.iter().enumerate().filter(|(_, f)| f.base) {
            if kept(f) {
                outs.push((*f, 1 << i));
            }
        }
        for &(p, deps) in &ev.new_bases {
            let f = Fact { p, base: true };
            if !calls || survives_calls(&p, preserved) {
                outs.retain(|(o, _)| !(o.base && spans_overlap(o.p.off, o.p.size, p.off, p.size)));
                outs.push((f, deps));
            }
        }
        if !leaves_the_run(ops) {
            let mut carry: Vec<(Fact, u32)> = Vec::new();
            for (i, f) in ins.iter().enumerate().filter(|(_, f)| !f.base) {
                if kept(f) {
                    carry.push((*f, 1 << i));
                }
            }
            if let Some(p) = fresh.filter(|p| !calls || survives_calls(p, preserved)) {
                carry.retain(|(o, _)| !overlaps(&o.p, &p));
                carry.push((Fact { p, base: false }, 0));
            }
            for (mut f, deps) in carry {
                f.p.hops += 1;
                if f.p.hops <= MAX_HOPS && outs.iter().filter(|(o, _)| !o.base).count() < MAX_TRACKED {
                    outs.push((f, deps));
                }
            }
        }

        let carried: Vec<Pending> = outs.iter().filter(|(f, _)| !f.base).map(|(f, _)| f.p).collect();
        if falls && !carried.is_empty() {
            self.at.entry(next).or_default().extend(carried);
        }
        let based: Vec<Pending> =
            outs.iter().filter(|(f, _)| f.base).map(|(f, _)| f.p).take(MAX_TRACKED).collect();
        if !based.is_empty() {
            for to in falls.then_some(next).into_iter().chain(branch_targets.iter().copied()) {
                match self.bases_at.get_mut(&to) {
                    Some(have) => have.retain(|h| based.contains(h)),
                    None => {
                        self.bases_at.insert(to, based.clone());
                    }
                }
            }
        }

        let seeded = fresh.and_then(|p| {
            outs.iter().any(|(f, deps)| *deps == 0 && !f.base && f.p.origin == p.origin).then_some(p.word)
        });
        self.nodes.push(Node { vma, ins, outs, refs: ev.found });
        seeded
    }

    /// Record the case edges of a switch dispatch at `vma`, which the walk reads
    /// after the dispatch's own [`PicPool::step`].
    pub(super) fn cases(&mut self, vma: u64, targets: &[u64]) {
        if !self.nodes.is_empty() {
            self.edges.extend(targets.iter().map(|&to| (vma, to, false)));
        }
    }

    /// The addresses the body formed, once the whole body is known.
    ///
    /// Starting from what the walk carried, a value is dropped at an instruction
    /// unless every recorded predecessor passes on the same one (`entry` has the
    /// caller as a predecessor, which passes nothing), and everything computed
    /// from a dropped value goes with it, until nothing changes. An address is
    /// answered only when all the values it was formed from survived.
    pub(super) fn finish(self, entry: u64) -> Verdict {
        let mut verdict = Verdict::default();
        if self.nodes.iter().all(|n| n.refs.is_empty()) {
            return verdict;
        }
        let index: HashMap<u64, usize> = self.nodes.iter().enumerate().map(|(i, n)| (n.vma, i)).collect();
        let mut preds: HashMap<u64, Vec<(u64, bool)>> = HashMap::new();
        for &(from, to, fall) in &self.edges {
            if index.contains_key(&to) {
                preds.entry(to).or_default().push((from, fall));
            }
        }
        let mut valid: Vec<u32> = self.nodes.iter().map(|n| all(n.ins.len())).collect();
        let delivers = |valid: &[u32], from: u64, fall: bool, f: &Fact| {
            index.get(&from).is_some_and(|&i| {
                self.nodes[i].outs.iter().any(|(o, deps)| {
                    deps & !valid[i] == 0 && (o.base || fall) && o.same(f)
                })
            })
        };
        let mut changed = true;
        while changed {
            changed = false;
            for (i, n) in self.nodes.iter().enumerate() {
                let mut keep = valid[i];
                let ps = preds.get(&n.vma).map_or(&[][..], Vec::as_slice);
                for (j, f) in n.ins.iter().enumerate() {
                    if keep & (1 << j) == 0 {
                        continue;
                    }
                    if n.vma == entry || ps.is_empty() || !ps.iter().all(|&(from, fall)| delivers(&valid, from, fall, f)) {
                        keep &= !(1 << j);
                    }
                }
                if keep != valid[i] {
                    valid[i] = keep;
                    changed = true;
                }
            }
        }
        for (i, n) in self.nodes.iter().enumerate() {
            for &(to, kind, deps, origin) in &n.refs {
                if deps & !valid[i] == 0 {
                    verdict.refs.push((n.vma, to, kind));
                    if origin != 0 {
                        verdict.displaced.insert(origin);
                    }
                }
            }
        }
        verdict.refs.sort_unstable();
        verdict.refs.dedup();
        verdict
    }
}

/// The mask with the low `n` bits set.
fn all(n: usize) -> u32 {
    if n >= 32 { u32::MAX } else { (1u32 << n) - 1 }
}

/// What one instruction did to the tracked values.
struct Eval {
    /// `(to, kind, deps, origin)`.
    found: Vec<(u64, XrefKind, u32, u64)>,
    written: Vec<(u64, u32)>,
    new_bases: Vec<(Pending, u32)>,
}

/// The addresses this instruction forms from the values it was handed, the
/// registers it writes, and the GOT bases it establishes.
fn evaluate(
    vma: u64,
    ops: &[FullOp],
    ins: &[Fact],
    mapped: &[(u64, u64)],
    data: Option<&Rc<AddrSpace>>,
    got: Option<&GotImage>,
) -> Eval {
    let mut vals: Vec<(Key, Known)> = ins
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let k = if f.base {
                Known { v: f.p.word, seed: true, pc: true, got: true, origin: 0, deps: 1 << i }
            } else {
                Known { deps: 1 << i, ..Known::seed(f.p.word, f.p.origin) }
            };
            (Key::Reg(f.p.off, f.p.size), k)
        })
        .collect();
    let mut written: Vec<(u64, u32)> = Vec::new();
    let mut found: Vec<(u64, XrefKind, u32, u64)> = Vec::new();
    let mut new_bases: Vec<(Pending, u32)> = Vec::new();

    for op in ops {
        if op.opcode == OpCode::CPUI_LOAD {
            if let (Some(g), Some(addr)) = (got, op.ins.get(1).and_then(|vn| value_of(vn, vma, &vals, data))) {
                let slot = addr.v & mask(op.ins[1].size);
                if addr.got && addr.seed && addr.pc && g.contains(slot) {
                    found.push((slot, XrefKind::Read, addr.deps, addr.origin));
                    if let Some(word) = g.word_at(slot) {
                        if looks_like_address(word) && in_range(mapped, word) {
                            found.push((word, XrefKind::Data, addr.deps, addr.origin));
                        }
                    }
                }
            }
        }
        let folded = fold(op, vma, &vals, data);
        let Some(out) = &op.out else { continue };
        let key = key_of(out, data);
        if let Some(Key::Reg(off, size)) = key {
            written.push((off, size));
        }
        invalidate(&mut vals, key);
        let (Some(key), Some(mut val)) = (key, folded) else { continue };
        val.v &= mask(out.size);
        if let Key::Reg(off, size) = key {
            if val.seed && val.pc && in_range(mapped, val.v) {
                found.push((val.v, XrefKind::Data, val.deps, val.origin));
                if !val.got && got.is_some_and(|g| g.contains(val.v)) {
                    new_bases.retain(|(b, _)| !spans_overlap(b.off, b.size, off, size));
                    new_bases.push((Pending { off, size, word: val.v, hops: 0, origin: 0 }, val.deps));
                }
            }
        }
        vals.push((key, val));
    }
    Eval { found, written, new_bases }
}

/// The pool word this instruction loads into a register.
///
/// The first such load, which on every architecture that forms these pairs is
/// also the only one: a literal load's address is a decode-time constant, and a
/// multi-register load (`ldm`) computes its addresses from a base register.
fn seed(
    vma: u64,
    ops: &[FullOp],
    pool: &PoolImage,
    data: Option<&Rc<AddrSpace>>,
) -> Option<Pending> {
    let data_index = data.map(|d| d.get_index());
    for op in ops {
        if op.opcode != OpCode::CPUI_LOAD {
            continue;
        }
        let Some(out) = &op.out else { continue };
        if !matches!(key_of(out, data), Some(Key::Reg(..))) || out.size != pool.ptr_width() {
            continue;
        }
        // `in0` is the space id: a load through some other space is not a read of
        // the image.
        let through_image = op
            .ins
            .first()
            .zip(data_index)
            .is_some_and(|(vn, idx)| vn.offset == u64::try_from(idx).unwrap_or(u64::MAX));
        if !through_image {
            continue;
        }
        let Some(addr) = op.ins.get(1).filter(|vn| is_constant(vn)) else { continue };
        let Some(word) = pool.word_at(addr.offset) else { continue };
        return Some(Pending { off: out.offset, size: out.size, word, hops: 0, origin: vma });
    }
    None
}

/// Does control leave the straight-line run here, so the fall-through is not the
/// next thing executed?
///
/// A `CBRANCH` is deliberately not in the set: its fall-through IS a successor,
/// it writes nothing, and ARM spells a predicated instruction that way. A call is
/// not either: it ends only the values the calling convention does not preserve
/// (`survives_calls`), since GCC schedules `ldr r4,[pc,#..]` before a call and
/// its `add r4,pc,r4` after it.
fn leaves_the_run(ops: &[FullOp]) -> bool {
    ops.iter().any(|op| {
        matches!(
            op.opcode,
            OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN
        )
    })
}

/// Is the value's whole register callee-saved?
fn survives_calls(p: &Pending, preserved: &[(u64, u64)]) -> bool {
    preserved.iter().any(|&(lo, hi)| p.off >= lo && p.off + u64::from(p.size) <= hi)
}

// --- the one-instruction constant fold ---------------------------------------

/// A value the fold knows, and where it came from. Both provenance bits must be
/// set for the result to be a PIC address: `seed` alone is the pool word being
/// used as a number, `pc` alone is any PC-relative arithmetic.
#[derive(Debug, Clone, Copy)]
struct Known {
    v: u64,
    /// A pool word contributed to this value.
    seed: bool,
    /// This instruction's own PC contributed to this value.
    pc: bool,
    /// A GOT base contributed to this value.
    got: bool,
    /// The pool load of the word that contributed, or 0.
    origin: u64,
    /// The tracked values that contributed, as a mask over the node's `ins`.
    deps: u32,
}

impl Known {
    fn seed(v: u64, origin: u64) -> Self {
        Known { v, seed: true, pc: false, got: false, origin, deps: 0 }
    }
}

/// The value a varnode holds, as [`fold`] reads its inputs.
fn value_of(vn: &VarnodeData, vma: u64, vals: &[(Key, Known)], data: Option<&Rc<AddrSpace>>) -> Option<Known> {
    if is_constant(vn) {
        return Some(Known { v: vn.offset & mask(vn.size), seed: false, pc: is_own_pc(vn.offset, vma), got: false, origin: 0, deps: 0 });
    }
    let key = key_of(vn, data)?;
    vals.iter().rev().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// A varnode's identity, in the two classes the fold models. `register` and `ram`
/// are both processor spaces, so the space type alone cannot tell them apart —
/// the default data space is what separates them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Reg(u64, u32),
    Tmp(u64, u32),
}

fn key_of(vn: &VarnodeData, data: Option<&Rc<AddrSpace>>) -> Option<Key> {
    let space = vn.space.as_ref()?;
    match space.get_type() {
        spacetype::IPTR_INTERNAL => Some(Key::Tmp(vn.offset, vn.size)),
        spacetype::IPTR_PROCESSOR if !matches!(data, Some(d) if Rc::ptr_eq(space, d)) => {
            Some(Key::Reg(vn.offset, vn.size))
        }
        _ => None,
    }
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}

/// The value this op produces, or `None` when the fold does not model it.
fn fold(
    op: &FullOp,
    vma: u64,
    vals: &[(Key, Known)],
    data: Option<&Rc<AddrSpace>>,
) -> Option<Known> {
    let get = |i: usize| -> Option<Known> { value_of(op.ins.get(i)?, vma, vals, data) };
    let a = get(0)?;
    match op.opcode {
        OpCode::CPUI_COPY => Some(a),
        OpCode::CPUI_INT_ADD => {
            let b = get(1)?;
            Some(Known {
                v: a.v.wrapping_add(b.v),
                seed: a.seed || b.seed,
                pc: a.pc || b.pc,
                got: a.got || b.got,
                origin: if a.origin != 0 { a.origin } else { b.origin },
                deps: a.deps | b.deps,
            })
        }
        OpCode::CPUI_INT_SUB => {
            let b = get(1)?;
            Some(Known {
                v: a.v.wrapping_sub(b.v),
                seed: a.seed || b.seed,
                pc: a.pc || b.pc,
                got: a.got || b.got,
                origin: if a.origin != 0 { a.origin } else { b.origin },
                deps: a.deps | b.deps,
            })
        }
        _ => None,
    }
}

/// Is `c` this instruction materializing its own program counter?
fn is_own_pc(c: u64, vma: u64) -> bool {
    c >= vma && c <= vma.wrapping_add(PC_WINDOW)
}

/// Drop every tracked value whose bytes overlap the write to `key`. Keying on the
/// exact triple would let a write to `r0` leave a stale two-byte view of it
/// behind, which is the one way this fold could hand out a value the program does
/// not hold.
fn invalidate(vals: &mut Vec<(Key, Known)>, key: Option<Key>) {
    let Some(key) = key else { return };
    let (cls, off, size) = split(key);
    vals.retain(|(k, _)| {
        let (c, o, s) = split(*k);
        c != cls || !spans_overlap(o, s, off, size)
    });
}

fn split(key: Key) -> (u8, u64, u32) {
    match key {
        Key::Reg(off, size) => (0, off, size),
        Key::Tmp(off, size) => (1, off, size),
    }
}

fn spans_overlap(a_off: u64, a_size: u32, b_off: u64, b_size: u32) -> bool {
    let a_end = a_off.saturating_add(u64::from(a_size));
    let b_end = b_off.saturating_add(u64::from(b_size));
    a_off < b_end && b_off < a_end
}

fn overlaps(a: &Pending, b: &Pending) -> bool {
    spans_overlap(a.off, a.size, b.off, b.size)
}

/// The byte mask of an `n`-byte value.
fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reduction of the filing image: a read-only pool at 0x6a0 holding the
    /// displacement -0x1c1, and `.rodata` at 0x400.
    const POOL: [u8; 8] = [
        0x3f, 0xfe, 0xff, 0xff, // 0x6a0 -> 0xfffffe3f (= -0x1c1)
        0x00, 0x05, 0x00, 0x00, // 0x6a4 -> 0x500 (a mapped address)
    ];
    const MAPPED: [(u64, u64); 2] = [(0x400, 0x4c3), (0x4c4, 0x6b0)];

    fn image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x6a0, 0x6a8, &POOL[..])], 4, true).unwrap()
    }

    /// A throwaway space set: `ram` stands in for the default data space,
    /// `register` for any other processor space, as the two are told apart in
    /// the engine.
    struct Spaces {
        konst: Rc<AddrSpace>,
        ram: Rc<AddrSpace>,
        reg: Rc<AddrSpace>,
        uniq: Rc<AddrSpace>,
    }

    fn spaces() -> Spaces {
        Spaces {
            konst: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
            ram: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            reg: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            uniq: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
        }
    }

    /// The space-id constant a `LOAD` through `ram` carries in `in0`.
    fn ram_id(s: &Spaces) -> u64 {
        u64::try_from(s.ram.get_index()).unwrap()
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    /// `ldr r0,[pc,#imm]` — SLEIGH resolves the PC-relative address at decode
    /// time, so the load reads a constant location.
    fn ldr(s: &Spaces, reg: u64, at: u64) -> Vec<FullOp> {
        vec![op(
            OpCode::CPUI_LOAD,
            Some(vn(&s.reg, reg, 4)),
            vec![vn(&s.konst, ram_id(s), 8), vn(&s.konst, at, 4)],
        )]
    }

    /// `add rN,pc,rN` at `vma`, exactly as the ARM SLEIGH lifts it: the PC is
    /// `inst_start + 8`, formed in a temporary first.
    fn add_pc(s: &Spaces, reg: u64, vma: u64) -> Vec<FullOp> {
        vec![
            op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&s.uniq, 0x2f00, 4)),
                vec![vn(&s.konst, vma, 4), vn(&s.konst, 8, 4)],
            ),
            op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&s.reg, reg, 4)),
                vec![vn(&s.uniq, 0x2f00, 4), vn(&s.reg, reg, 4)],
            ),
        ]
    }

    /// One body's walk, driven in the order the test lists the instructions.
    struct Walk<'a> {
        pp: PicPool,
        pool: &'a PoolImage<'a>,
        mapped: &'a [(u64, u64)],
        got: Option<&'a GotImage<'a>>,
        preserved: &'a [(u64, u64)],
        s: &'a Spaces,
    }

    impl<'a> Walk<'a> {
        fn new(s: &'a Spaces, pool: &'a PoolImage<'a>, mapped: &'a [(u64, u64)]) -> Self {
            Walk { pp: PicPool::default(), pool, mapped, got: None, preserved: &[], s }
        }

        fn step(&mut self, vma: u64, ops: &[FullOp], branches: &[u64]) -> Option<u64> {
            self.pp.step(
                vma,
                4,
                ops,
                self.pool,
                self.mapped,
                Some(&self.s.ram),
                self.got,
                self.preserved,
                branches,
            )
        }

        fn finish(self, entry: u64) -> Verdict {
            self.pp.finish(entry)
        }
    }

    fn refs(v: &Verdict) -> Vec<(u64, u64, XrefKind)> {
        v.refs.clone()
    }

    /// The defect: the address of the literal is in no instruction and in no pool
    /// word — only in the two put together.
    #[test]
    fn a_pool_displacement_added_to_the_pc_is_the_address_it_forms() {
        let (s, pool) = (spaces(), image());
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &add_pc(&s, 0x20, 0x664), &[]);
        assert_eq!(refs(&w.finish(0x660)), vec![(0x664, 0x4ab, XrefKind::Data)], "0x66c + (-0x1c1)");
    }

    /// The pair need not be adjacent: instruction scheduling separates the load
    /// from the `add` when a function forms several references at once.
    #[test]
    fn the_add_may_sit_a_few_instructions_past_the_load() {
        let (s, pool) = (spaces(), image());
        let nop = vec![op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&s.reg, 0x30, 4)),
            vec![vn(&s.reg, 0x30, 4), vn(&s.konst, 4, 4)],
        )];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &nop, &[]);
        w.step(0x668, &add_pc(&s, 0x20, 0x668), &[]);
        assert_eq!(refs(&w.finish(0x660)), vec![(0x668, 0x4af, XrefKind::Data)]);
    }

    /// Overwriting the register between the load and the `add` ends the value:
    /// what the `add` composes is whatever was written last.
    #[test]
    fn a_write_to_the_register_ends_the_tracked_word() {
        let (s, pool) = (spaces(), image());
        let clobber =
            vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, 0x20, 4)), vec![vn(&s.konst, 0, 4)])];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &clobber, &[]);
        w.step(0x668, &add_pc(&s, 0x20, 0x668), &[]);
        assert!(w.finish(0x660).refs.is_empty());
    }

    /// A call clobbers the caller-saved registers, so nothing in them survives it.
    #[test]
    fn a_call_ends_the_tracked_word() {
        let (s, pool) = (spaces(), image());
        let mut call = ldr(&s, 0x20, 0x6a0);
        call.push(op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x700, 4)]));
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &call, &[]);
        w.step(0x664, &add_pc(&s, 0x20, 0x664), &[]);
        assert!(w.finish(0x660).refs.is_empty());
    }

    /// A word in a callee-saved register survives the call between its load and
    /// its `add`, as GCC schedules the GOT base's pair around a call.
    #[test]
    fn a_call_keeps_a_word_in_a_callee_saved_register() {
        let (s, pool) = (spaces(), image());
        let mut call = ldr(&s, 0x30, 0x6a0);
        call.push(op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x700, 4)]));
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.preserved = &[(0x30, 0x34)];
        w.step(0x660, &call, &[]);
        w.step(0x664, &add_pc(&s, 0x30, 0x664), &[]);
        assert_eq!(refs(&w.finish(0x660)), vec![(0x664, 0x4ab, XrefKind::Data)]);
    }

    /// The two refusals, each of which would be a fabricated reference.
    #[test]
    fn arithmetic_without_the_pc_and_an_unmapped_sum_are_declined() {
        let (s, pool) = (spaces(), image());

        // `add r0,r0,#8` — the pool word used as a number, not as a base.
        let no_pc = vec![op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&s.reg, 0x20, 4)),
            vec![vn(&s.reg, 0x20, 4), vn(&s.konst, 8, 4)],
        )];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &no_pc, &[]);
        assert!(w.finish(0x660).refs.is_empty());

        // A sum landing in no section is not an address, whatever formed it.
        let mut w = Walk::new(&s, &pool, &[(0x9000, 0x9100)]);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &add_pc(&s, 0x20, 0x664), &[]);
        assert!(w.finish(0x660).refs.is_empty());
    }

    /// `ldr r1,[pc,#imm]; ldr r1,[pc,r1]` composes its address into a temporary
    /// and dereferences it. The address is never held in a register and is no
    /// GOT base, so nothing is filed.
    #[test]
    fn a_composed_address_that_stays_in_a_temporary_files_nothing() {
        let (s, pool) = (spaces(), image());
        let indirect = vec![
            op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&s.uniq, 0x2f00, 4)),
                vec![vn(&s.konst, 0x664, 4), vn(&s.konst, 8, 4)],
            ),
            op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&s.uniq, 0x10900, 4)),
                vec![vn(&s.uniq, 0x2f00, 4), vn(&s.reg, 0x24, 4)],
            ),
            op(
                OpCode::CPUI_LOAD,
                Some(vn(&s.reg, 0x24, 4)),
                vec![vn(&s.konst, ram_id(&s), 8), vn(&s.uniq, 0x10900, 4)],
            ),
        ];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x24, 0x6a0), &[]);
        w.step(0x664, &indirect, &[]);
        assert!(w.finish(0x660).refs.is_empty());
    }

    /// A narrow or unaligned pool read is not a slot at all, which is
    /// [`PoolImage`]'s rule and is inherited here.
    #[test]
    fn a_narrow_pool_read_is_not_a_displacement() {
        let (s, pool) = (spaces(), image());
        let narrow = vec![op(
            OpCode::CPUI_LOAD,
            Some(vn(&s.reg, 0x20, 2)),
            vec![vn(&s.konst, ram_id(&s), 8), vn(&s.konst, 0x6a0, 4)],
        )];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &narrow, &[]);
        w.step(0x664, &add_pc(&s, 0x20, 0x664), &[]);
        assert!(w.finish(0x660).refs.is_empty());
    }

    /// A displacement that happens to be a mapped address itself, as in a
    /// non-PIE image mapped low, is still a displacement once the PC is added,
    /// and `poolref` must not also file it as a pointer.
    #[test]
    fn a_displacement_that_is_also_a_mapped_address_still_composes() {
        const P: [u8; 4] = [0x00, 0x05, 0x00, 0x00]; // 0x6a0 -> 0x500, mapped
        const MAP: [(u64, u64); 1] = [(0x400, 0x1000)];
        let pool = PoolImage::from_ranges(vec![(0x6a0, 0x6a4, &P[..])], 4, true).unwrap();
        let s = spaces();
        let mut w = Walk::new(&s, &pool, &MAP);
        assert_eq!(w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]), Some(0x500));
        w.step(0x664, &add_pc(&s, 0x20, 0x664), &[]);
        let v = w.finish(0x660);
        assert_eq!(refs(&v), vec![(0x664, 0xb6c, XrefKind::Data)]);
        assert!(v.displaced.contains(&0x660));
    }

    /// A mapped word that no PC fold consumes stays `poolref`'s pointer.
    #[test]
    fn a_mapped_word_used_as_a_number_is_not_displaced() {
        let (s, pool) = (spaces(), image());
        let no_pc = vec![op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&s.reg, 0x20, 4)),
            vec![vn(&s.reg, 0x20, 4), vn(&s.konst, 8, 4)],
        )];
        let mut w = Walk::new(&s, &pool, &MAPPED);
        assert_eq!(w.step(0x660, &ldr(&s, 0x20, 0x6a4), &[]), Some(0x500));
        w.step(0x664, &no_pc, &[]);
        let v = w.finish(0x660);
        assert!(v.refs.is_empty() && v.displaced.is_empty());
    }

    /// Nothing is tracked past the hop bound.
    #[test]
    fn a_word_is_not_believed_forever() {
        let (s, pool) = (spaces(), image());
        let nothing: Vec<FullOp> = Vec::new();
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x600, &ldr(&s, 0x20, 0x6a0), &[]);
        let mut vma = 0x604;
        for _ in 0..MAX_HOPS {
            w.step(vma, &nothing, &[]);
            vma += 4;
        }
        w.step(vma, &add_pc(&s, 0x20, vma), &[]);
        assert!(w.finish(0x600).refs.is_empty());
    }

    // --- GOT bases -----------------------------------------------------------

    /// Pool words for a GOT at 0x800: 0x6a0 holds the base's displacement from
    /// an `add` at 0x604 (0x60c + 0x1f4 = 0x800), 0x6a4 the GOT offset 0x10 of
    /// the slot at 0x810, which holds the function address 0x1450.
    const GOT_POOL: [u8; 8] = [
        0xf4, 0x01, 0x00, 0x00, // 0x6a0 -> 0x1f4
        0x10, 0x00, 0x00, 0x00, // 0x6a4 -> 0x10
    ];
    const GOT_BYTES: [u8; 0x20] = {
        let mut g = [0u8; 0x20];
        g[0x10] = 0x50;
        g[0x11] = 0x14;
        g
    };
    const GOT_MAP: [(u64, u64); 1] = [(0x400, 0x2000)];
    const R4: u64 = 0x30;
    const R2: u64 = 0x28;

    fn got_pool() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x6a0, 0x6a8, &GOT_POOL[..])], 4, true).unwrap()
    }

    fn got_image() -> GotImage<'static> {
        GotImage { lo: 0x800, hi: 0x820, data: &GOT_BYTES[..], ptr: 4, little_endian: true }
    }

    /// `ldr r2,[r4,r2]`.
    fn slot_load(s: &Spaces) -> Vec<FullOp> {
        vec![
            op(OpCode::CPUI_INT_ADD, Some(vn(&s.uniq, 0x2f00, 4)), vec![vn(&s.reg, R4, 4), vn(&s.reg, R2, 4)]),
            op(OpCode::CPUI_LOAD, Some(vn(&s.reg, R2, 4)), vec![vn(&s.konst, ram_id(&s), 8), vn(&s.uniq, 0x2f00, 4)]),
        ]
    }

    fn jump(s: &Spaces, to: u64) -> Vec<FullOp> {
        vec![op(OpCode::CPUI_BRANCH, None, vec![vn(&s.ram, to, 4)])]
    }

    fn cbranch(s: &Spaces, to: u64) -> Vec<FullOp> {
        vec![op(OpCode::CPUI_CBRANCH, None, vec![vn(&s.ram, to, 4), vn(&s.reg, 0x100, 1)])]
    }

    const SLOT_REFS: [(u64, XrefKind); 2] = [(0x810, XrefKind::Read), (0x1450, XrefKind::Data)];

    fn slot_refs(v: &Verdict, at: u64) -> Vec<(u64, XrefKind)> {
        v.refs.iter().filter(|r| r.0 == at).map(|r| (r.1, r.2)).collect()
    }

    /// The ARM GOT idiom of a `-fPIC` body: `ldr r4,[pc,#a]; add r4,pc,r4`
    /// makes r4 the GOT, and `ldr r2,[pc,#b]; ldr r2,[r4,r2]` reads a slot. The
    /// base survives a call (r4 is callee-saved) and a branch, and the slot's
    /// word, a function address, is filed as the address the body takes.
    #[test]
    fn a_load_through_a_pc_composed_got_base_files_the_slot_and_its_word() {
        let (s, pool, got) = (spaces(), got_pool(), got_image());
        let mut w = Walk::new(&s, &pool, &GOT_MAP);
        w.got = Some(&got);
        w.preserved = &[(R4, R4 + 4)];
        let call = vec![op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x400, 4)])];
        w.step(0x600, &ldr(&s, R4, 0x6a0), &[]);
        w.step(0x604, &add_pc(&s, R4, 0x604), &[]);
        w.step(0x608, &call, &[]);
        w.step(0x60c, &jump(&s, 0x700), &[0x700]);
        w.step(0x700, &ldr(&s, R2, 0x6a4), &[]);
        w.step(0x704, &slot_load(&s), &[]);
        let v = w.finish(0x600);
        assert_eq!(slot_refs(&v, 0x604), vec![(0x800, XrefKind::Data)]);
        assert_eq!(slot_refs(&v, 0x704), SLOT_REFS.to_vec());
    }

    /// The join rule. 0x620 is reached from 0x608, where r4 is the GOT, and from
    /// 0x610, after 0x60c overwrote r4. The breadth-first walk decodes 0x620
    /// before 0x610, so only the meet over both edges, taken once the body is
    /// known, can tell that r4 is not the GOT there.
    #[test]
    fn a_base_that_one_path_overwrites_is_not_believed_at_the_join() {
        let (s, pool, got) = (spaces(), got_pool(), got_image());
        let clobber = vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, R4, 4)), vec![vn(&s.konst, 0x810, 4)])];
        let mut w = Walk::new(&s, &pool, &GOT_MAP);
        w.got = Some(&got);
        w.step(0x600, &ldr(&s, R4, 0x6a0), &[]);
        w.step(0x604, &add_pc(&s, R4, 0x604), &[]);
        w.step(0x608, &cbranch(&s, 0x620), &[0x620]);
        w.step(0x60c, &clobber, &[]);
        w.step(0x620, &ldr(&s, R2, 0x6a4), &[]);
        w.step(0x610, &jump(&s, 0x620), &[0x620]);
        w.step(0x624, &slot_load(&s), &[]);
        let v = w.finish(0x600);
        assert!(slot_refs(&v, 0x624).is_empty(), "got {:?}", v.refs);
    }

    /// The same join with both paths keeping r4: the base is equal on every
    /// edge, so the meet keeps it.
    #[test]
    fn a_base_both_paths_keep_survives_the_join() {
        let (s, pool, got) = (spaces(), got_pool(), got_image());
        let other = vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, 0x20, 4)), vec![vn(&s.konst, 0, 4)])];
        let mut w = Walk::new(&s, &pool, &GOT_MAP);
        w.got = Some(&got);
        w.step(0x600, &ldr(&s, R4, 0x6a0), &[]);
        w.step(0x604, &add_pc(&s, R4, 0x604), &[]);
        w.step(0x608, &cbranch(&s, 0x620), &[0x620]);
        w.step(0x60c, &other, &[]);
        w.step(0x620, &ldr(&s, R2, 0x6a4), &[]);
        w.step(0x610, &jump(&s, 0x620), &[0x620]);
        w.step(0x624, &slot_load(&s), &[]);
        assert_eq!(slot_refs(&w.finish(0x600), 0x624), SLOT_REFS.to_vec());
    }

    /// A predicated instruction (`ldrne`) spells its condition as a branch to its
    /// own fall-through; that is not a join, and the word carries past it.
    #[test]
    fn a_predicated_instruction_between_the_load_and_the_add_keeps_the_word() {
        let (s, pool) = (spaces(), image());
        let mut w = Walk::new(&s, &pool, &MAPPED);
        w.step(0x660, &ldr(&s, 0x20, 0x6a0), &[]);
        w.step(0x664, &cbranch(&s, 0x668), &[0x668]);
        w.step(0x668, &add_pc(&s, 0x20, 0x668), &[]);
        assert_eq!(refs(&w.finish(0x660)), vec![(0x668, 0x4af, XrefKind::Data)]);
    }

    /// A loop whose body does not write r4 keeps the base across its back edge.
    #[test]
    fn a_base_survives_a_loop_back_edge() {
        let (s, pool, got) = (spaces(), got_pool(), got_image());
        let mut w = Walk::new(&s, &pool, &GOT_MAP);
        w.got = Some(&got);
        w.step(0x600, &ldr(&s, R4, 0x6a0), &[]);
        w.step(0x604, &add_pc(&s, R4, 0x604), &[]);
        w.step(0x608, &ldr(&s, R2, 0x6a4), &[]);
        w.step(0x60c, &slot_load(&s), &[]);
        w.step(0x610, &cbranch(&s, 0x608), &[0x608]);
        assert_eq!(slot_refs(&w.finish(0x600), 0x60c), SLOT_REFS.to_vec());
    }

    /// The entry has the caller as a predecessor, which hands it nothing: a
    /// branch back to the entry cannot make a base hold there.
    #[test]
    fn nothing_holds_at_the_entry() {
        let (s, pool, got) = (spaces(), got_pool(), got_image());
        let mut w = Walk::new(&s, &pool, &GOT_MAP);
        w.got = Some(&got);
        w.step(0x5f8, &ldr(&s, R2, 0x6a4), &[]);
        w.step(0x5fc, &slot_load(&s), &[]);
        w.step(0x600, &ldr(&s, R4, 0x6a0), &[]);
        w.step(0x604, &add_pc(&s, R4, 0x604), &[]);
        w.step(0x608, &jump(&s, 0x5f8), &[0x5f8]);
        assert!(slot_refs(&w.finish(0x5f8), 0x5fc).is_empty());
    }
}
