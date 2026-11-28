//! (kuna) `unreadtable` — the computed jumps whose jump table the walk did not
//! read, so an answer that is missing their targets can say so.
//!
//! [`super::kuna_switchtable`] reads a switch's table only when it can prove the
//! table's base and length. Where it cannot (a base kept in a caller-saved
//! register across a call, a base loaded from the GOT or a stack slot, a range
//! check it does not model, an unchecked float-derived index), the dispatch has
//! no successor and the case bodies are never decoded: every reference they
//! make is missing, and nothing in the answer said so.
//!
//! A computed jump counts when the code feeding it, folded by the table
//! reader's own lattice, branches to an entry read out of a table: a case
//! address (a delta or compact-table entry added back to its base), or a
//! pointer read at an index scaled by the pointer width. Each try leaves one
//! value unknown as the index (everything read from above the straight run,
//! one register read from above, or what one read of memory yields) and gives
//! every other register read from above, and every other read of a fixed
//! address, a stand-in base no image maps: a base hoisted out of reach, MIPS
//! and PowerPC PIC's base loaded from the GOT, a base spilled to the stack. A
//! table at a constant address must also read as one, and a pointer table
//! needs a constant address or a register's stand-in ([`indexes_table`]).
//!
//! So a jump through a pointer does not count: `jmp *0x10(%rax)` folds to a
//! "table" at `0x10` off an unknown, unscaled, which is a field of whatever
//! `%rax` points to; a PLT stub's `ldr x17, [x16, #off]; br x17`, MIPS PIC's
//! `lw t9, %call16(f)(gp); jr t9` and i386's `jmp *0xc(%ebx)` are the same
//! shape, and a returned function pointer reads nothing. A forwarding veneer's
//! `jmp [slot]` branches on memory rather than a register, and a return is not
//! a computed jump at all.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{
    falls_through, fold, invalidate, is_constant, key_of, keys_overlap, leaves_registers_unknown, previous, survives,
    targets, value_of, Key, Val, MAX_TRACKED, MIN_ENTRIES,
};
use super::xrefs::{decode, in_range, looks_like_address, FullCapture, FullOp};

/// How far back from the jump the slice may reach.
const MAX_SLICE_INSNS: usize = 64;

/// How many instructions the slice steps back over without one that computes
/// what it still wants, once it holds a load and before: what the chain still
/// reads from further up is a stand-in base, so a jump through a pointer that
/// arrived in a register costs a few decodes rather than the whole run. MIPS
/// schedules a table's load well above its `jr`, so the wait before a load is
/// the longer.
const MAX_IDLE_LOADED: usize = 3;

/// [`MAX_IDLE_LOADED`] before the slice holds a load.
const MAX_IDLE_UNLOADED: usize = 32;

/// The most registers the slice may read from above it and still be tried one
/// unknown at a time.
const MAX_FREE: usize = 8;

/// How many copies, extensions and masks the index may pass through between
/// its scaling and the table-address sum.
const MAX_SCALE_STEPS: usize = 8;

/// What a classification reads besides the code: the read-only image the
/// table reader reads (`pool`) and every section with file content (`image`),
/// the executable ranges, the registers the calling convention preserves, the
/// mapped ranges and the stack pointer.
pub(super) struct Reach<'a> {
    pub(super) pool: &'a PoolImage<'a>,
    pub(super) image: &'a PoolImage<'a>,
    pub(super) exec: &'a [(u64, u64)],
    pub(super) preserved: &'a [(u64, u64)],
    pub(super) mapped: &'a [(u64, u64)],
    pub(super) sp: Option<Key>,
}

/// Does the computed jump at `from`, whose p-code is `jump`, index a table?
///
/// A delta or compact table (an entry added back to a base) is a switch's
/// shape and nothing else's. A pointer table is a structure's array of
/// function pointers called in tail position as often as a switch, so it
/// counts only at a constant address whose first [`MIN_ENTRIES`] entries, read
/// out of `image` (every section with file content, so a computed goto's labels
/// in `.data.rel.ro` count), land in the executable section holding the jump
/// (`exec`), as the table reader requires of its own; or indexed off a
/// register read from above (a base gcc `-fipa-ra` keeps in MIPS `t0`) with the
/// frame still standing. One indexed off what a load reads, or reached after
/// the epilogue (`mov 0x1f18(%r14,%rax,8),%rax; pop ...; jmp *%rax`), is not.
pub(super) fn indexes_table(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &dyn Fn(u64) -> bool,
    from: u64,
    jump: &[FullOp],
    reach: &Reach,
) -> bool {
    let Reach { pool, image, exec, preserved, mapped, sp } = *reach;
    let branch = jump.iter().find(|op| op.opcode == OpCode::CPUI_BRANCHIND).and_then(|op| op.ins.first());
    let Some(want) = branch.and_then(|vn| key_of(vn, data)) else { return false };
    let (slice, teardown) = straight_slice(translate, code_space, data, decoded, from, jump, want, preserved, sp);
    let ops = || slice.iter().flat_map(|(_, ops)| ops);
    if !ops().any(|op| op.opcode == OpCode::CPUI_LOAD) || !shaped(&slice, data, pool.ptr_width()) {
        return false;
    }
    let loads = ops().filter(|op| reads_memory(op, data)).count();
    let free = free_keys(&slice, data);
    let tried = if free.len() <= MAX_FREE { free.len() } else { 0 };
    let ptr = pool.ptr_width();
    let (held, loaded, span) = if ptr == 8 {
        (0x4b00_0000_0000_0000, 0x5b00_0000_0000_0000, 1u64 << 31)
    } else {
        (0x9b00_0000, 0xbb00_0000, 1u64 << 27)
    };
    let near = |v: u64, stand_in: u64| v.wrapping_sub(stand_in - span) < 2 * span;
    let standin = |v: u64| near(v, held) || near(v, loaded);
    let real = |v: u64| looks_like_address(v) && in_range(mapped, v);
    let big = data.is_some_and(|d| d.is_big_endian());
    let mut unknowns = std::iter::once(Unknown::Above)
        .chain((0..tried).map(Unknown::Register))
        .chain((0..loads).filter(|_| tried == free.len()).map(Unknown::Load));
    unknowns.any(|unknown| {
        let given: Vec<(Key, u64)> = match unknown {
            Unknown::Above => Vec::new(),
            Unknown::Register(u) => {
                free.iter().enumerate().filter(|&(i, _)| i != u).map(|(_, &k)| (k, held)).collect()
            }
            Unknown::Load(_) => free.iter().map(|&k| (k, held)).collect(),
        };
        let keep = match unknown {
            Unknown::Load(j) => Some(j),
            _ => None,
        };
        let slice = propagate(&slice, given, keep, loaded, data, big, preserved);
        let (dispatch, index) = fold_all(&slice, from, data, pool, preserved);
        match dispatch {
            Some(Val::Case { table, base } | Val::Relative { table, base, .. }) => {
                real(table) || standin(table) || standin(base)
            }
            Some(Val::Entry { table, width, .. }) if width == ptr && (real(table) || near(table, held)) => {
                let read = if real(table) {
                    !targets(table, from, image, exec, Some(MIN_ENTRIES), MIN_ENTRIES).is_empty()
                } else {
                    !teardown
                };
                read && index.is_some_and(|(at, op, key)| scale(&slice, at, op, key, data) == Some(u64::from(ptr)))
            }
            _ => false,
        }
    })
}

/// Every allocated section of `file` with content in the file, as the image a
/// table at a constant address is read out of.
pub(super) fn image<'a>(file: &'a object::File<'a>) -> Option<PoolImage<'a>> {
    use object::{Object, ObjectSection};
    let sections = file.sections().filter(|s| s.address() != 0 && s.size() != 0).filter_map(|s| {
        let data = s.data().ok().filter(|d| !d.is_empty())?;
        Some((s.address(), s.address().saturating_add(s.size().min(data.len() as u64)), data))
    });
    PoolImage::from_ranges(sections.collect(), if file.is_64() { 8 } else { 4 }, file.is_little_endian())
}

/// Which value a try leaves unknown, as the table index.
#[derive(Clone, Copy, Debug)]
enum Unknown {
    /// Everything read from above the slice.
    Above,
    /// The `n`th register read from above; the rest are stand-in bases.
    Register(usize),
    /// What the `n`th read of memory reads; every register read from above is
    /// a stand-in base.
    Load(usize),
}

/// [`super::kuna_switchtable::fold_slice`] over `slice` without its cap on the
/// values carried, which the long chains a table read missing its base makes
/// reach: the value the jump at `from` branches on, and the addend of the
/// table-address sum the fold could not value.
fn fold_all(
    slice: &[(u64, Vec<FullOp>)],
    from: u64,
    data: Option<&Rc<AddrSpace>>,
    pool: &PoolImage,
    preserved: &[(u64, u64)],
) -> (Option<Val>, Option<(u64, usize, Key)>) {
    let mut vals: Vec<(Key, Val)> = Vec::new();
    let mut index = None;
    for (vma, ops) in slice {
        for (i, op) in ops.iter().enumerate() {
            if op.opcode == OpCode::CPUI_BRANCHIND && *vma == from {
                return (op.ins.first().and_then(|vn| value_of(vn, &vals, data)), index);
            }
            if leaves_registers_unknown(op.opcode) {
                vals.retain(|(k, _)| survives(*k, preserved));
            }
            let folded = fold(op, &vals, data, pool).or_else(|| self_relative(op, &vals, data));
            if op.opcode == OpCode::CPUI_INT_ADD && matches!(folded, Some(Val::Indexed(_))) {
                let unknown = op.ins.iter().find(|vn| value_of(vn, &vals, data).is_none());
                if let Some(key) = unknown.and_then(|vn| key_of(vn, data)) {
                    index = Some((*vma, i, key));
                }
            }
            let Some(out) = &op.out else { continue };
            let key = key_of(out, data);
            invalidate(&mut vals, key);
            if let (Some(key), Some(val)) = (key, folded) {
                vals.push((key, val));
            }
        }
    }
    (None, index)
}

/// An entry added to its own address: the case address of a self-relative
/// table (`add x6, x6, x14, lsl #2; ldr w7, [x6]; add x6, x6, w7, sxtw`).
fn self_relative(op: &FullOp, vals: &[(Key, Val)], data: Option<&Rc<AddrSpace>>) -> Option<Val> {
    if op.opcode != OpCode::CPUI_INT_ADD {
        return None;
    }
    let get = |i: usize| value_of(op.ins.get(i)?, vals, data);
    match (get(0)?, get(1)?) {
        (Val::Indexed(at), Val::Entry { table, width: 4, .. }) | (Val::Entry { table, width: 4, .. }, Val::Indexed(at))
            if at == table =>
        {
            Some(Val::Case { table, base: table })
        }
        _ => None,
    }
}

/// The ops that feed the jump at `from`, in execution order with the
/// instruction each is in: the straight run above the jump, stepped back over
/// while an instruction falls into the next, as far as a call that clobbers
/// what the chain still wants, keeping only the ops whose output the jump's
/// target is computed from (and the calls between), or until
/// [`MAX_IDLE_LOADED`] / [`MAX_IDLE_UNLOADED`] go by without one. What the
/// chain reads from above where it stops is read as it stands at the jump.
///
/// It follows more than the table reader's own slice (a mask, a rotate's
/// halves, any scaling), since it only classifies: MIPS's `jr` and ARM's `bx`
/// clear the ISA bit of their target, and PowerPC's `slwi` is a rotate under a
/// mask. The flag says whether an instruction on the run pops the frame: adds
/// a positive constant to the stack pointer `sp`.
#[allow(clippy::too_many_arguments)]
fn straight_slice(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &dyn Fn(u64) -> bool,
    from: u64,
    jump: &[FullOp],
    want: Key,
    preserved: &[(u64, u64)],
    sp: Option<Key>,
) -> (Vec<(u64, Vec<FullOp>)>, bool) {
    let mut cap = FullCapture::default();
    let mut need = vec![want];
    let mut out = Vec::new();
    let mut teardown = false;
    let (mut holds_load, mut idle) = (false, 0);
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    for step in 0..MAX_SLICE_INSNS {
        let insn = if step == 0 {
            jump
        } else {
            let Some(len) = decode(translate, cur, code_space, &mut cap) else { break };
            if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) || !falls_through(cap.ops()) {
                break;
            }
            cap.ops()
        };
        teardown |= sp.is_some_and(|sp| insn.iter().any(|op| pops(op, sp, data)));
        let mut kept = Vec::new();
        let mut call = false;
        for op in insn.iter().rev() {
            if op.opcode == OpCode::CPUI_CALLOTHER && op.out.is_none() {
                continue;
            }
            if matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
                call = true;
                kept.push(op.clone());
                continue;
            }
            if step == 0 && op.opcode == OpCode::CPUI_BRANCHIND {
                kept.push(op.clone());
                continue;
            }
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
            if !need.iter().any(|n| keys_overlap(*n, out)) {
                continue;
            }
            need.retain(|&n| !covers(out, n));
            holds_load |= op.opcode == OpCode::CPUI_LOAD;
            kept.push(op.clone());
            for k in op.ins.iter().filter(|vn| !is_constant(vn)).filter_map(|vn| key_of(vn, data)) {
                if !need.contains(&k) && need.len() < MAX_TRACKED {
                    need.push(k);
                }
            }
        }
        let clobbered = call && !need.iter().all(|k| survives(*k, preserved));
        if clobbered {
            kept.retain(|op| !matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER));
        }
        let relevant = kept.iter().any(|op| !matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER));
        idle = if relevant { 0 } else { idle + 1 };
        if !kept.is_empty() {
            kept.reverse();
            out.push((cur, kept));
        }
        let patience = if holds_load { MAX_IDLE_LOADED } else { MAX_IDLE_UNLOADED };
        if need.is_empty() || clobbered || idle >= patience {
            break;
        }
        let Some(prev) = previous(decoded, cur) else { break };
        ends_at = Some(cur);
        cur = prev;
    }
    out.reverse();
    (out, teardown)
}

/// Could `slice` read a table at all, before any fold: does it scale a
/// value by a constant (an index into wider entries), read memory narrower
/// than a pointer (a byte, halfword or delta entry), or add a loaded value to
/// another (an entry and its base)? A jump through a pointer fetched at a
/// register plus a constant does none of these, and is settled here.
fn shaped(slice: &[(u64, Vec<FullOp>)], data: Option<&Rc<AddrSpace>>, ptr: u32) -> bool {
    let mut loaded: Vec<Key> = Vec::new();
    for op in slice.iter().flat_map(|(_, ops)| ops) {
        let constant = |i: usize| op.ins.get(i).is_some_and(is_constant);
        let reads_loaded = op.ins.iter().filter_map(|vn| key_of(vn, data)).any(|k| loaded.contains(&k));
        let out = op.out.as_ref().and_then(|vn| key_of(vn, data));
        match op.opcode {
            OpCode::CPUI_INT_MULT if constant(0) != constant(1) => return true,
            OpCode::CPUI_INT_LEFT if !constant(0) => return true,
            OpCode::CPUI_INT_ADD if reads_loaded => return true,
            OpCode::CPUI_LOAD if op.out.as_ref().is_some_and(|o| o.size < ptr) => return true,
            OpCode::CPUI_LOAD => loaded.extend(out),
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT if reads_loaded => loaded.extend(out),
            _ => {}
        }
    }
    false
}

/// Does `op` add a positive constant to the stack pointer `sp`?
fn pops(op: &FullOp, sp: Key, data: Option<&Rc<AddrSpace>>) -> bool {
    let is_sp = |vn: &VarnodeData| key_of(vn, data) == Some(sp);
    op.opcode == OpCode::CPUI_INT_ADD
        && op.out.as_ref().is_some_and(is_sp)
        && op.ins.first().is_some_and(is_sp)
        && op.ins.get(1).is_some_and(|c| is_constant(c) && c.offset != 0 && c.offset >> (8 * c.size.clamp(1, 8) - 1) & 1 == 0)
}

/// Does a write to `out` replace all of `key`?
fn covers(out: Key, key: Key) -> bool {
    match (out, key) {
        (Key::Reg(o, os), Key::Reg(k, ks)) | (Key::Tmp(o, os), Key::Tmp(k, ks)) => {
            o <= k && k.saturating_add(u64::from(ks)) <= o.saturating_add(u64::from(os))
        }
        _ => false,
    }
}

/// `slice` with every value it computes from constants, or from the
/// registers `known` holds, written as that constant, and every read of a
/// constant address but the `keep`th ([`reads_memory`]) as `stand_in`: the
/// slot holds a base, not a table entry. A mask that clears, or an `or` that sets, no more
/// than the low three bits (an ISA bit, Thumb's `orr r2, r2, #1`) is a copy,
/// and so is a shift by nothing (AArch64's `sxtw`). A call forgets what the
/// calling convention does not preserve.
fn propagate(
    slice: &[(u64, Vec<FullOp>)],
    mut known: Vec<(Key, u64)>,
    keep: Option<usize>,
    stand_in: u64,
    data: Option<&Rc<AddrSpace>>,
    big: bool,
    preserved: &[(u64, u64)],
) -> Vec<(u64, Vec<FullOp>)> {
    let Some(space) =
        slice.iter().flat_map(|(_, ops)| ops).flat_map(|op| &op.ins).find(|vn| is_constant(vn)).map(|vn| vn.space.clone())
    else {
        return slice.to_vec();
    };
    let constant = |v: u64, size: u32| VarnodeData { space: space.clone(), offset: v & mask(size), size };
    let mut out = Vec::with_capacity(slice.len());
    let mut load = 0;
    for (vma, ops) in slice {
        let mut rewritten = Vec::with_capacity(ops.len());
        for op in ops {
            let read = reads_memory(op, data);
            let kept = read && keep == Some(load);
            load += usize::from(read);
            if matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
                known.retain(|(k, _)| survives(*k, preserved));
            }
            let mut op = op.clone();
            let skip = usize::from(op.opcode == OpCode::CPUI_LOAD);
            for vn in op.ins.iter_mut().skip(skip) {
                if let Some(v) = key_of(vn, data).and_then(|k| value(&known, k, big)) {
                    *vn = constant(v, vn.size);
                }
            }
            let Some(dest) = op.out.clone() else {
                rewritten.push(op);
                continue;
            };
            let key = key_of(&dest, data);
            known.retain(|(k, _)| !key.is_some_and(|key| keys_overlap(*k, key)));
            let a = op.ins.first().filter(|vn| is_constant(vn)).map(|vn| vn.offset);
            let b = op.ins.get(1).filter(|vn| is_constant(vn)).map(|vn| vn.offset);
            let m = mask(dest.size);
            let result = match (op.opcode, a, b) {
                (OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT, Some(a), _) => Some(a),
                (OpCode::CPUI_INT_SEXT, Some(a), _) => {
                    let bits = 8 * op.ins[0].size;
                    Some(if bits < 64 && a >> (bits - 1) & 1 == 1 { a | (u64::MAX << bits) } else { a })
                }
                (OpCode::CPUI_INT_2COMP, Some(a), _) => Some(a.wrapping_neg()),
                (OpCode::CPUI_INT_NEGATE, Some(a), _) => Some(!a),
                (OpCode::CPUI_INT_ADD, Some(a), Some(b)) => Some(a.wrapping_add(b)),
                (OpCode::CPUI_INT_SUB, Some(a), Some(b)) => Some(a.wrapping_sub(b)),
                (OpCode::CPUI_INT_MULT, Some(a), Some(b)) => Some(a.wrapping_mul(b)),
                (OpCode::CPUI_INT_LEFT, Some(a), Some(b)) => Some(if b < 64 { a << b } else { 0 }),
                (OpCode::CPUI_INT_RIGHT, Some(a), Some(b)) => {
                    Some(if b < 64 { (a & mask(op.ins[0].size)) >> b } else { 0 })
                }
                (OpCode::CPUI_INT_OR, Some(a), Some(b)) => Some(a | b),
                (OpCode::CPUI_INT_XOR, Some(a), Some(b)) => Some(a ^ b),
                (OpCode::CPUI_INT_AND, Some(a), Some(b)) => Some(a & b),
                (OpCode::CPUI_LOAD, _, Some(_)) if !kept => Some(stand_in),
                (OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, None, _) if read && !kept => {
                    Some(stand_in)
                }
                _ => None,
            };
            let copy = |vn: VarnodeData| FullOp { opcode: OpCode::CPUI_COPY, out: Some(dest.clone()), ins: vec![vn] };
            let op = match (result, op.opcode, a, b) {
                (Some(v), ..) => {
                    known.extend(key.map(|k| (k, v & m)));
                    copy(constant(v, dest.size))
                }
                (None, OpCode::CPUI_INT_AND, None, Some(c)) if !c & m < 8 => copy(op.ins[0].clone()),
                (None, OpCode::CPUI_INT_AND, Some(c), None) if !c & m < 8 => copy(op.ins[1].clone()),
                (None, OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT, None, Some(0)) => {
                    copy(op.ins[0].clone())
                }
                (None, OpCode::CPUI_INT_OR, None, Some(c)) if c < 8 => copy(op.ins[0].clone()),
                (None, OpCode::CPUI_INT_OR, Some(c), None) if c < 8 => copy(op.ins[1].clone()),
                _ => op,
            };
            rewritten.push(op);
        }
        out.push((*vma, rewritten));
    }
    out
}

/// Does `op` read memory: a `LOAD`, or a copy of a fixed address the
/// instruction names (`mov rax, [rip+slot]`, an ARM literal-pool `ldr`)?
fn reads_memory(op: &FullOp, data: Option<&Rc<AddrSpace>>) -> bool {
    match op.opcode {
        OpCode::CPUI_LOAD => true,
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            op.ins.first().is_some_and(|vn| !is_constant(vn) && key_of(vn, data).is_none())
        }
        _ => false,
    }
}

/// What `known` says `key` holds: its own value, or the low part of a wider
/// value at the same place.
fn value(known: &[(Key, u64)], key: Key, big: bool) -> Option<u64> {
    let parts = |k: Key| match k {
        Key::Reg(o, s) => (0, o, s),
        Key::Tmp(o, s) => (1, o, s),
    };
    let (class, off, size) = parts(key);
    known.iter().rev().find_map(|&(k, v)| {
        let (c, o, s) = parts(k);
        let low = if big { o.checked_add(u64::from(s.checked_sub(size)?)) == Some(off) } else { o == off };
        (c == class && s >= size && low).then(|| v & mask(size))
    })
}

fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 { u64::MAX } else { (1u64 << (8 * size)) - 1 }
}

/// The registers and temporaries `slice` reads before anything in it writes
/// them, in first-read order.
fn free_keys(slice: &[(u64, Vec<FullOp>)], data: Option<&Rc<AddrSpace>>) -> Vec<Key> {
    let mut written: Vec<Key> = Vec::new();
    let mut free: Vec<Key> = Vec::new();
    for op in slice.iter().flat_map(|(_, ops)| ops) {
        for k in op.ins.iter().filter(|vn| !is_constant(vn)).filter_map(|vn| key_of(vn, data)) {
            if !written.iter().any(|w| covers(*w, k)) && !free.contains(&k) {
                free.push(k);
            }
        }
        if let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) {
            written.push(out);
        }
    }
    free
}

/// The factor the value `key` read by op `op` of the instruction at `at`
/// carries: the constant it was multiplied by or the power of two it was
/// shifted left by, followed back through copies, extensions, masks and the
/// halves of a rotate. `None` when the slice does not scale it.
fn scale(slice: &[(u64, Vec<FullOp>)], at: u64, op: usize, key: Key, data: Option<&Rc<AddrSpace>>) -> Option<u64> {
    let ops: Vec<&FullOp> = slice
        .iter()
        .take_while(|(vma, _)| *vma <= at)
        .flat_map(|(vma, ops)| ops.iter().take(if *vma == at { op } else { ops.len() }))
        .collect();
    factor(&ops, ops.len(), key, data, MAX_SCALE_STEPS)
}

fn factor(ops: &[&FullOp], before: usize, key: Key, data: Option<&Rc<AddrSpace>>, steps: usize) -> Option<u64> {
    let steps = steps.checked_sub(1)?;
    let at = ops[..before].iter().rposition(|o| o.out.as_ref().and_then(|vn| key_of(vn, data)) == Some(key))?;
    let def = ops[at];
    let constant = |i: usize| def.ins.get(i).filter(|vn| is_constant(vn)).map(|vn| vn.offset);
    let variable = |i: usize| def.ins.get(i).is_some_and(|vn| !is_constant(vn));
    let follow = |i: usize| factor(ops, at, def.ins.get(i).filter(|vn| !is_constant(vn)).and_then(|vn| key_of(vn, data))?, data, steps);
    match def.opcode {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => follow(0),
        OpCode::CPUI_INT_AND if constant(1).is_some() => follow(0),
        OpCode::CPUI_INT_AND if constant(0).is_some() => follow(1),
        OpCode::CPUI_INT_OR => follow(0).or_else(|| follow(1)),
        OpCode::CPUI_INT_MULT => match (constant(0), constant(1)) {
            (_, Some(1)) if variable(0) => follow(0),
            (Some(1), _) if variable(1) => follow(1),
            (_, Some(c)) if variable(0) => Some(c),
            (Some(c), _) if variable(1) => Some(c),
            _ => None,
        },
        OpCode::CPUI_INT_LEFT if variable(0) => constant(1).filter(|&c| c < 8).map(|c| 1 << c),
        _ => None,
    }
}
