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
//!  * `poolref` **declined** it, so the two mechanisms never claim one word: a
//!    value that is already a mapped address is a pointer, not a displacement;
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

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::kuna_poolref::PoolImage;
use super::xrefs::{in_range, looks_like_address, FullOp};

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
}

/// The pool values one function's walk is carrying, keyed by the address of the
/// instruction they reach.
///
/// Keyed by address rather than held as "the previous instruction's state"
/// because [`super::xrefs::build_with_focus`] walks a function breadth-first: at
/// a conditional branch the queue interleaves the two successors, so "the
/// instruction decoded before this one" is not the instruction that precedes it.
#[derive(Default)]
pub(super) struct PicPool {
    at: HashMap<u64, Vec<Pending>>,
}

impl PicPool {
    /// Consume one decoded instruction, and answer the addresses it composed.
    ///
    /// Called for every instruction of the function in whatever order the walk
    /// reaches them; the answer is empty for all but the `add` of a PIC pair.
    pub(super) fn step(
        &mut self,
        vma: u64,
        len: u32,
        ops: &[FullOp],
        pool: &PoolImage,
        mapped: &[(u64, u64)],
        data_space: Option<&Rc<AddrSpace>>,
    ) -> Vec<u64> {
        // Hashing every instruction's address would be this walk's whole cost on
        // an architecture that never forms one of these pairs; nothing is in
        // flight until a pool load puts it there.
        let pending =
            if self.at.is_empty() { Vec::new() } else { self.at.remove(&vma).unwrap_or_default() };
        let (mut refs, mut carry) = if pending.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            evaluate(vma, ops, &pending, mapped, data_space)
        };

        // A load out of the pool starts (or restarts) the value its register
        // holds, so the pair `ldr; add` needs no separate entry point.
        if let Some(fresh) = seed(ops, pool, mapped, data_space) {
            carry.retain(|p| !overlaps(p, &fresh));
            carry.push(fresh);
        }

        if !carry.is_empty() && !leaves_the_run(ops) {
            let next = vma.wrapping_add(u64::from(len));
            let slot = self.at.entry(next).or_default();
            for mut p in carry {
                p.hops += 1;
                if p.hops <= MAX_HOPS && slot.len() < MAX_TRACKED {
                    slot.push(p);
                }
            }
            if slot.is_empty() {
                self.at.remove(&next);
            }
        }

        refs.sort_unstable();
        refs.dedup();
        refs
    }
}

/// The composed addresses this instruction forms, and the tracked values that
/// survive it.
fn evaluate(
    vma: u64,
    ops: &[FullOp],
    pending: &[Pending],
    mapped: &[(u64, u64)],
    data: Option<&Rc<AddrSpace>>,
) -> (Vec<u64>, Vec<Pending>) {
    let mut vals: Vec<(Key, Known)> =
        pending.iter().map(|p| (Key::Reg(p.off, p.size), Known::seed(p.word))).collect();
    let mut written: Vec<(u64, u32)> = Vec::new();
    let mut found: Vec<u64> = Vec::new();

    for op in ops {
        let folded = fold(op, vma, &vals, data);
        let Some(out) = &op.out else { continue };
        let key = key_of(out, data);
        if let Some(Key::Reg(off, size)) = key {
            written.push((off, size));
        }
        invalidate(&mut vals, key);
        let (Some(key), Some(mut val)) = (key, folded) else { continue };
        val.v &= mask(out.size);
        if matches!(key, Key::Reg(..)) && val.seed && val.pc && in_range(mapped, val.v) {
            found.push(val.v);
        }
        vals.push((key, val));
    }

    let survivors = pending
        .iter()
        .filter(|p| {
            !written.iter().any(|&(off, size)| {
                spans_overlap(p.off, p.size, off, size)
            })
        })
        .copied()
        .collect();
    (found, survivors)
}

/// The pool word this instruction loads into a register, when it loads one that
/// [`super::kuna_poolref`] declined to follow as a pointer.
///
/// The first such load, which on every architecture that forms these pairs is
/// also the only one: a literal load's address is a decode-time constant, and a
/// multi-register load (`ldm`) computes its addresses from a base register.
fn seed(
    ops: &[FullOp],
    pool: &PoolImage,
    mapped: &[(u64, u64)],
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
        if looks_like_address(word) && in_range(mapped, word) {
            continue; // a pointer, which is `poolref`'s to follow
        }
        return Some(Pending { off: out.offset, size: out.size, word, hops: 0 });
    }
    None
}

/// Does control leave the straight-line run here, so the fall-through is not the
/// next thing executed (or a call has clobbered the registers in between)?
///
/// A `CBRANCH` is deliberately not in the set: its fall-through IS a successor,
/// it writes nothing, and ARM spells a predicated instruction that way.
fn leaves_the_run(ops: &[FullOp]) -> bool {
    ops.iter().any(|op| {
        matches!(
            op.opcode,
            OpCode::CPUI_BRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
        )
    })
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
}

impl Known {
    fn seed(v: u64) -> Self {
        Known { v, seed: true, pc: false }
    }
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
    let get = |i: usize| -> Option<Known> {
        let vn = op.ins.get(i)?;
        if is_constant(vn) {
            return Some(Known {
                v: vn.offset & mask(vn.size),
                seed: false,
                pc: is_own_pc(vn.offset, vma),
            });
        }
        let key = key_of(vn, data)?;
        vals.iter().rev().find(|(k, _)| *k == key).map(|(_, v)| *v)
    };
    let a = get(0)?;
    match op.opcode {
        OpCode::CPUI_COPY => Some(a),
        OpCode::CPUI_INT_ADD => {
            let b = get(1)?;
            Some(Known { v: a.v.wrapping_add(b.v), seed: a.seed || b.seed, pc: a.pc || b.pc })
        }
        OpCode::CPUI_INT_SUB => {
            let b = get(1)?;
            Some(Known { v: a.v.wrapping_sub(b.v), seed: a.seed || b.seed, pc: a.pc || b.pc })
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

    /// The defect: the address of the literal is in no instruction and in no pool
    /// word — only in the two put together.
    #[test]
    fn a_pool_displacement_added_to_the_pc_is_the_address_it_forms() {
        let s = spaces();
        let mut pp = PicPool::default();
        assert!(pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram)).is_empty());
        assert_eq!(
            pp.step(0x664, 4, &add_pc(&s, 0x20, 0x664), &image(), &MAPPED, Some(&s.ram)),
            vec![0x4ab],
            "0x66c + (-0x1c1)"
        );
    }

    /// The pair need not be adjacent: instruction scheduling separates the load
    /// from the `add` when a function forms several references at once.
    #[test]
    fn the_add_may_sit_a_few_instructions_past_the_load() {
        let s = spaces();
        let mut pp = PicPool::default();
        let nop = vec![op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&s.reg, 0x30, 4)),
            vec![vn(&s.reg, 0x30, 4), vn(&s.konst, 4, 4)],
        )];
        pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram));
        pp.step(0x664, 4, &nop, &image(), &MAPPED, Some(&s.ram));
        assert_eq!(
            pp.step(0x668, 4, &add_pc(&s, 0x20, 0x668), &image(), &MAPPED, Some(&s.ram)),
            vec![0x4af]
        );
    }

    /// Overwriting the register between the load and the `add` ends the value:
    /// what the `add` composes is whatever was written last.
    #[test]
    fn a_write_to_the_register_ends_the_tracked_word() {
        let s = spaces();
        let mut pp = PicPool::default();
        let clobber =
            vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, 0x20, 4)), vec![vn(&s.konst, 0, 4)])];
        pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram));
        pp.step(0x664, 4, &clobber, &image(), &MAPPED, Some(&s.ram));
        assert!(pp
            .step(0x668, 4, &add_pc(&s, 0x20, 0x668), &image(), &MAPPED, Some(&s.ram))
            .is_empty());
    }

    /// A call clobbers the caller-saved registers, so nothing survives it.
    #[test]
    fn a_call_ends_the_tracked_word() {
        let s = spaces();
        let mut pp = PicPool::default();
        let mut call = ldr(&s, 0x20, 0x6a0);
        call.push(op(OpCode::CPUI_CALL, None, vec![vn(&s.ram, 0x700, 4)]));
        pp.step(0x660, 4, &call, &image(), &MAPPED, Some(&s.ram));
        assert!(pp
            .step(0x664, 4, &add_pc(&s, 0x20, 0x664), &image(), &MAPPED, Some(&s.ram))
            .is_empty());
    }

    /// The three refusals, each of which would be a fabricated reference.
    #[test]
    fn arithmetic_without_the_pc_a_pointer_word_and_an_unmapped_sum_are_declined() {
        let s = spaces();

        // `add r0,r0,#8` — the pool word used as a number, not as a base.
        let mut pp = PicPool::default();
        pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram));
        let no_pc = vec![op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&s.reg, 0x20, 4)),
            vec![vn(&s.reg, 0x20, 4), vn(&s.konst, 8, 4)],
        )];
        assert!(pp.step(0x664, 4, &no_pc, &image(), &MAPPED, Some(&s.ram)).is_empty());

        // The word at 0x6a4 IS a mapped address, so it is `poolref`'s pointer to
        // follow and never a displacement.
        let mut pp = PicPool::default();
        pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a4), &image(), &MAPPED, Some(&s.ram));
        assert!(pp
            .step(0x664, 4, &add_pc(&s, 0x20, 0x664), &image(), &MAPPED, Some(&s.ram))
            .is_empty());

        // A sum landing in no section is not an address, whatever formed it.
        let mut pp = PicPool::default();
        pp.step(0x660, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram));
        assert!(pp
            .step(0x664, 4, &add_pc(&s, 0x20, 0x664), &image(), &[(0x9000, 0x9100)], Some(&s.ram))
            .is_empty());
    }

    /// The GOT idiom `ldr r1,[pc,#imm]; ldr r1,[pc,r1]` composes its address into
    /// a temporary and dereferences it. The address is never held in a register,
    /// so nothing is filed — the double indirection is not this mechanism's.
    #[test]
    fn a_composed_address_that_stays_in_a_temporary_files_nothing() {
        let s = spaces();
        let mut pp = PicPool::default();
        pp.step(0x660, 4, &ldr(&s, 0x24, 0x6a0), &image(), &MAPPED, Some(&s.ram));
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
        assert!(pp.step(0x664, 4, &indirect, &image(), &MAPPED, Some(&s.ram)).is_empty());
    }

    /// A narrow or unaligned pool read is not a slot at all, which is
    /// [`PoolImage`]'s rule and is inherited here.
    #[test]
    fn a_narrow_pool_read_is_not_a_displacement() {
        let s = spaces();
        let mut pp = PicPool::default();
        let narrow = vec![op(
            OpCode::CPUI_LOAD,
            Some(vn(&s.reg, 0x20, 2)),
            vec![vn(&s.konst, ram_id(&s), 8), vn(&s.konst, 0x6a0, 4)],
        )];
        pp.step(0x660, 4, &narrow, &image(), &MAPPED, Some(&s.ram));
        assert!(pp
            .step(0x664, 4, &add_pc(&s, 0x20, 0x664), &image(), &MAPPED, Some(&s.ram))
            .is_empty());
    }

    /// Nothing is tracked past the hop bound.
    #[test]
    fn a_word_is_not_believed_forever() {
        let s = spaces();
        let mut pp = PicPool::default();
        let nothing: Vec<FullOp> = Vec::new();
        pp.step(0x600, 4, &ldr(&s, 0x20, 0x6a0), &image(), &MAPPED, Some(&s.ram));
        let mut vma = 0x604;
        for _ in 0..MAX_HOPS {
            pp.step(vma, 4, &nothing, &image(), &MAPPED, Some(&s.ram));
            vma += 4;
        }
        assert!(pp
            .step(vma, 4, &add_pc(&s, 0x20, vma), &image(), &MAPPED, Some(&s.ram))
            .is_empty());
    }
}
