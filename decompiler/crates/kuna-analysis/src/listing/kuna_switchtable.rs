//! (kuna) `switchtable` — read a computed jump's target table out of the image,
//! so the case bodies of a switch belong to the function that dispatches to them.
//!
//! # The gap
//!
//! [`super::xrefs`]'s walk is a recursive descent over [`super::classify`]'s
//! static successors, and a `BRANCHIND` contributes none (`classify.rs`: "no
//! static target, deferred jump-table resolution"). So the walk dies at every
//! switch dispatch: on an MSVC i386 window procedure whose handler is
//!
//! ```text
//! 0x401435  JA   0x40172a                        ; default
//! 0x40143b  JMP  dword ptr [EAX*0x4 + 0x4017c4]  ; ten cases
//! ...
//! 0x4016ef  PUSH 0x403288                        ; "Product Already Registered"
//! ```
//!
//! the whole case-body region between the dispatch and the epilogue is
//! undecoded, every reference it forms is missing, and `kuna strings` reports a
//! literal the disassembly plainly pushes with `xrefs_count 0` and no owning
//! function. The engine's own switch recovery (`p2_lift/jumptable.rs`) does
//! reach that body, but it runs over a decompiled function; this tier has only
//! the bytes.
//!
//! # The rule
//!
//! One table read, of content that cannot change, bounded by the image's own
//! partition. The instruction must be a computed **jump** with no static target,
//! and the table base is a constant the instruction itself materializes — the
//! [`XrefKind::Data`](super::xrefs::XrefKind::Data) reference the constant scan
//! already files for it. From that base the entries are read in order while each
//! one is
//!
//!  * a pointer-sized, pointer-aligned word of **allocated, non-writable** memory
//!    with file content (the same [`PoolImage`] dereference `poolref` uses), and
//!  * an address inside the **same executable section** as the dispatching
//!    instruction — a case body is code, and it is the dispatcher's own code.
//!
//! The first word that is not both ends the table, which is what stops the scan
//! at the `cc cc cc cc` padding after the last case on the example above. A run
//! of fewer than [`MIN_ENTRIES`] is not a table: one plausible word after a
//! constant is a coincidence, a switch is not.
//!
//! A `ram` *varnode* base is deliberately not a candidate. `jmp dword ptr
//! [__imp_X]` — a PE import veneer, an ELF PLT entry — encodes its slot as a
//! direct data-space operand, which the constant scan files as a
//! [`Read`](super::xrefs::XrefKind::Read) and not a `Data`, so a veneer is never
//! read as a one-entry table of whatever its unrelocated slot happens to hold.
//!
//! # The shape real compilers emit
//!
//! Neither half of that rule holds on an x86-64 image. The base is materialized
//! by an instruction of its own, one to a few before the jump, and the entries
//! are signed 32-bit **displacements** off a base that is generally not the
//! table:
//!
//! ```text
//! gcc -O2      lea    jt(%rip),%rdx      ; the base, on its own instruction
//!              movslq (%rdx,%rax,4),%rax ; a 32-bit delta, not a pointer
//!              add    %rdx,%rax
//!              jmp    *%rax
//!
//! MSVC x64     LEA RDX,[0x140000000]     ; __ImageBase: base != table
//!              MOV ECX,[RDX + RBX*4 + 0x2d84]
//!              ADD RCX,RDX
//!              JMP RCX
//! ```
//!
//! So the dispatching instruction materializes no constant at all and the walk
//! files no `Data` reference for it, which is why a whole switch's worth of
//! direct callees is missing from `kuna decompile-graph` on a stock MSVC binary.
//!
//! [`register_targets`] reads that shape by asking what the branch register
//! holds, rather than by searching for a constant that might be a table. From
//! the dispatch it steps back over at most [`MAX_DISPATCH_INSNS`] already-decoded
//! instructions, folds them forward through a four-value lattice — a constant,
//! a constant plus an unknown index, a table entry, and an entry composed with
//! its base — and answers only when the `BRANCHIND`'s own input is the last of
//! those. Nothing runs anywhere but at a computed jump, and the fold sees no
//! instruction the walk did not already decode.

use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::xrefs::{decode, in_range, FullCapture, FullOp};

/// Below this an accepted run is a coincidence rather than a table: a compiler
/// does not lower a one-case switch through a jump table.
const MIN_ENTRIES: usize = 2;

/// The largest table read, the same ceiling the engine's own switch recovery
/// takes (`Architecture::max_jumptable_size`, 1024). A scan that runs this far
/// has stopped describing a switch and is walking whatever follows it.
const MAX_ENTRIES: usize = 1024;

/// The case bodies the computed jump at `from` dispatches to through a table at
/// `base`, or empty when `base` does not front one.
///
/// `exec` is the executable partition; the entries are confined to the range of
/// it that contains `from`, so a table cannot carry the walk into a different
/// section's code.
pub(super) fn targets(
    base: u64,
    from: u64,
    pool: &PoolImage,
    exec: &[(u64, u64)],
    cases: Option<usize>,
) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let stride = u64::from(pool.ptr_width());
    let mut out = Vec::new();
    for i in 0..cases.unwrap_or(MAX_ENTRIES).min(MAX_ENTRIES) as u64 {
        let Some(at) = base.checked_add(i * stride) else { break };
        let Some(word) = pool.word_at(at) else { break };
        if !in_range(&[section], word) {
            break;
        }
        out.push(word);
    }
    if out.len() < MIN_ENTRIES {
        out.clear();
    }
    out
}

/// The case bodies a **delta** table dispatches to: a run of 4-byte signed
/// displacements at `table`, each measured from `base`.
///
/// `base == table` is the gcc form and `base == __ImageBase` the MSVC one; the
/// two differ only in which constant the code adds back, so one read serves
/// both. Every other clause is [`targets`]': the entries stop at the first word
/// that is not an address in the dispatch's own executable section, and a run
/// shorter than [`MIN_ENTRIES`] is not a table. `cases` is the switch's own case
/// count, which is what keeps the scan from running out of one table and into
/// the next — two delta tables sit back to back in a `-O2` gnulib object, and
/// the second one's entries are perfectly good code addresses measured from the
/// wrong base.
pub(super) fn delta_targets(
    table: u64,
    base: u64,
    from: u64,
    pool: &PoolImage,
    exec: &[(u64, u64)],
    cases: Option<usize>,
) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for i in 0..cases.unwrap_or(MAX_ENTRIES).min(MAX_ENTRIES) as u64 {
        let Some(at) = table.checked_add(i * 4) else { break };
        let Some(delta) = pool.word32_at(at) else { break };
        let word = base.wrapping_add(delta as i64 as u64);
        if !in_range(&[section], word) {
            break;
        }
        out.push(word);
    }
    if out.len() < MIN_ENTRIES {
        out.clear();
    }
    out
}

/// How far back the dispatch chain may reach. gcc and MSVC both establish the
/// base one to five instructions ahead of the jump; the slack covers a scheduler
/// that hoists it further, and the slice below usually stops long before this.
const MAX_DISPATCH_INSNS: usize = 8;

/// The widest instruction any vendored processor encodes, which bounds how far
/// back a predecessor's first byte can lie.
const MAX_INSN_BYTES: u64 = 16;

/// The case bodies the computed jump at `from` dispatches to when its target
/// register — not the instruction's own operand — carries the table.
///
/// Answers the two shapes in the module header plus the absolute table read
/// through a register (`jmp *(%rdx,%rax,8)`), and nothing else: the fold reports
/// a table only where the `BRANCHIND`'s input is a word drawn from a
/// constant-based array, so a jump through a plain pointer, a virtual call and a
/// returned function pointer all decline before any memory is read.
pub(super) fn register_targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data_space: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    branch: &VarnodeData,
    pool: &PoolImage,
    exec: &[(u64, u64)],
) -> Vec<u64> {
    // Nothing is read for a jump whose target is not in a register or a
    // temporary: `jmp qword ptr [__imp_X]` branches on a data-space varnode and
    // is a forwarding veneer, which is the shape this must never walk back from.
    let Some(want) = key_of(branch, data_space) else { return Vec::new() };
    let slice = dispatch_slice(translate, code_space, data_space, decoded, from, want);

    let mut vals: Vec<(Key, Val)> = Vec::new();
    let mut dispatch = None;
    'slice: for (vma, ops) in &slice {
        for op in ops {
            if op.opcode == OpCode::CPUI_BRANCHIND && *vma == from {
                dispatch = op.ins.first().and_then(|vn| value_of(vn, &vals, data_space));
                break 'slice;
            }
            let folded = fold(op, &vals, data_space, pool);
            let Some(out) = &op.out else { continue };
            let key = key_of(out, data_space);
            invalidate(&mut vals, key);
            if let (Some(key), Some(val)) = (key, folded) {
                if vals.len() < MAX_TRACKED {
                    vals.push((key, val));
                }
            }
        }
    }
    let table = match dispatch {
        Some(Val::Case { table, base }) => (table, Some(base)),
        Some(Val::Entry { table, width }) if width == pool.ptr_width() => (table, None),
        _ => return Vec::new(),
    };
    // Read only now, on a jump that really does index a table: without the
    // switch's own case count there is nothing to stop the scan at the end of
    // it, and a delta table's neighbour reads as more cases.
    let Some(limit) = guard_limit(translate, code_space, decoded, from) else {
        return Vec::new();
    };
    match table {
        (table, Some(base)) => delta_targets(table, base, from, pool, exec, Some(limit)),
        (table, None) => targets(table, from, pool, exec, Some(limit)),
    }
}

/// How many cases the range check ahead of the dispatch admits, or `None` when
/// there is no range check to read.
///
/// A compiler never lets an unchecked index reach a jump table: the selector is
/// compared against the last case and the out-of-range arm branches away, which
/// is `cmp $0xa,%r11d; ja default` on x86 and lifts to `INT_LESS(sel, 0xa)`
/// feeding a `CBRANCH`. That constant is the table's length, and it is the only
/// bound in the image that is: the entries themselves say nothing about where
/// they end, and a second table laid out immediately after the first — which is
/// what gnulib's `quotearg_buffer_restyled` has — continues the run with
/// addresses that are real code and are measured from the wrong base.
///
/// The search stops one instruction past the guard branch, so an unrelated
/// comparison further up the block cannot widen the bound.
fn guard_limit(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    decoded: &HashSet<u64>,
    from: u64,
) -> Option<usize> {
    let mut cap = FullCapture::default();
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    let mut bound: Option<u64> = None;
    let mut after_branch = false;
    for _ in 0..MAX_DISPATCH_INSNS {
        let len = decode(translate, cur, code_space, &mut cap)?;
        if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) {
            break;
        }
        let mut branches = false;
        for op in cap.ops() {
            branches |= op.opcode == OpCode::CPUI_CBRANCH;
            if !matches!(op.opcode, OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS) {
                continue;
            }
            let (Some(a), Some(b)) = (op.ins.first(), op.ins.get(1)) else { continue };
            if is_constant(a) || !is_constant(b) || b.offset == 0 {
                continue;
            }
            bound = Some(bound.unwrap_or(0).max(b.offset));
        }
        if after_branch {
            break;
        }
        after_branch = branches;
        let prev = (1..=MAX_INSN_BYTES)
            .filter_map(|d| cur.checked_sub(d))
            .find(|at| decoded.contains(at))?;
        ends_at = Some(cur);
        cur = prev;
    }
    let bound = bound?;
    usize::try_from(bound.checked_add(1)?).ok()
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}

/// The instructions that actually feed the dispatch, in address order, each with
/// the p-code it emitted.
///
/// A backward slice rather than a fixed window, because a computed jump is
/// usually not a switch at all — an indirect tail call, a virtual dispatch, a
/// returned function pointer — and the whole cost of this feature is the
/// instructions it decodes at one. Stepping back from `from`, an instruction
/// joins the slice only where it defines something the chain still wants, and
/// the walk stops the moment the chain wants nothing: `jmp *%rdx` fed by `mov
/// 0x8(%rax),%rdx` is settled one instruction back, because a value that reaches
/// the branch through an operation the fold does not model — a multiply, a mask,
/// a call's return — is unknown and asks for nothing further.
fn dispatch_slice(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    want: Key,
) -> Vec<(u64, Vec<FullOp>)> {
    let mut cap = FullCapture::default();
    let mut need: Vec<Key> = vec![want];
    let mut out: Vec<(u64, Vec<FullOp>)> = Vec::new();
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    for step in 0..MAX_DISPATCH_INSNS {
        let Some(len) = decode(translate, cur, code_space, &mut cap) else { break };
        // The step back trusted the closest decoded address below the previous
        // instruction; this is where that is confirmed, at no cost, by requiring
        // the two to be contiguous.
        if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) {
            break;
        }
        let mut relevant = step == 0;
        let mut stop = false;
        for op in cap.ops().iter().rev() {
            stop |= leaves_registers_unknown(op.opcode);
            let Some(vn) = &op.out else { continue };
            let Some(key) = key_of(vn, data) else { continue };
            if !need.iter().any(|n| keys_overlap(*n, key)) {
                continue;
            }
            relevant = true;
            need.retain(|n| !keys_overlap(*n, key));
            if !models(op.opcode) {
                continue;
            }
            for input in &op.ins {
                let Some(k) = key_of(input, data) else { continue };
                if !need.contains(&k) && need.len() < MAX_TRACKED {
                    need.push(k);
                }
            }
        }
        if relevant {
            out.push((cur, cap.ops().to_vec()));
        }
        if need.is_empty() || stop {
            break;
        }
        // The closest decoded address below `cur` is the only candidate: the
        // walk's partition does not overlap, so anything further back either
        // ends before `cur` or is not an instruction start at all.
        let Some(prev) = (1..=MAX_INSN_BYTES)
            .filter_map(|d| cur.checked_sub(d))
            .find(|at| decoded.contains(at))
        else {
            break;
        };
        ends_at = Some(cur);
        cur = prev;
    }
    out.reverse();
    out
}

/// Does the fold give this op's output a value derived from its inputs? The
/// slice follows only these, which is what prunes the switch selector's own
/// chain: a scaled index reaches the table address through `INT_MULT`, and
/// nothing before that multiply can change what the table is.
fn models(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_LOAD
    )
}

/// How many values the fold carries at once. A dispatch chain needs two (the
/// base and the entry); the cap keeps a long window of constant materializations
/// from accumulating.
const MAX_TRACKED: usize = 16;

/// What the fold knows about a value. Only [`Val::Case`] and a pointer-wide
/// [`Val::Entry`] are answers; the other two exist to get there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Val {
    /// Exactly this constant.
    Const(u64),
    /// This constant plus an unknown — an array address whose index is the
    /// switch selector.
    Indexed(u64),
    /// A `width`-byte word read out of the array at `table`.
    Entry { table: u64, width: u32 },
    /// A table entry added back to the base it is measured from: the case
    /// address itself.
    Case { table: u64, base: u64 },
}

/// A varnode's identity, in the two classes the fold models — the same split
/// [`super::kuna_picpool`] makes, since `register` and `ram` are both processor
/// spaces and only the default data space tells them apart.
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

fn value_of(vn: &VarnodeData, vals: &[(Key, Val)], data: Option<&Rc<AddrSpace>>) -> Option<Val> {
    let space = vn.space.as_ref()?;
    if space.get_type() == spacetype::IPTR_CONSTANT {
        return Some(Val::Const(vn.offset & mask(vn.size)));
    }
    let key = key_of(vn, data)?;
    vals.iter().rev().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// Does this op make every tracked register unbelievable? A call clobbers the
/// caller-saved ones, and nothing about a dispatch chain survives one.
fn leaves_registers_unknown(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
    )
}

/// The value this op produces, or `None` where the fold does not model it.
fn fold(
    op: &FullOp,
    vals: &[(Key, Val)],
    data: Option<&Rc<AddrSpace>>,
    pool: &PoolImage,
) -> Option<Val> {
    let get = |i: usize| -> Option<Val> { value_of(op.ins.get(i)?, vals, data) };
    match op.opcode {
        OpCode::CPUI_COPY => get(0),
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => match get(0)? {
            Val::Const(c) => {
                let width = op.ins.first()?.size;
                Some(Val::Const(if op.opcode == OpCode::CPUI_INT_SEXT {
                    sign_extend(c, width)
                } else {
                    c & mask(width)
                }))
            }
            v @ (Val::Entry { .. } | Val::Case { .. }) => Some(v),
            Val::Indexed(_) => None,
        },
        OpCode::CPUI_INT_ADD => {
            let out = op.out.as_ref()?.size;
            match (get(0), get(1)) {
                (Some(Val::Const(a)), Some(Val::Const(b))) => {
                    Some(Val::Const(a.wrapping_add(b) & mask(out)))
                }
                (Some(Val::Const(a)), Some(Val::Indexed(b)))
                | (Some(Val::Indexed(a)), Some(Val::Const(b))) => {
                    Some(Val::Indexed(a.wrapping_add(b)))
                }
                // The dispatch's own composition: an entry plus the base it is
                // measured from. Only a 32-bit entry, which is the width every
                // delta table is emitted at.
                (Some(Val::Entry { table, width: 4 }), Some(Val::Const(base)))
                | (Some(Val::Const(base)), Some(Val::Entry { table, width: 4 })) => {
                    Some(Val::Case { table, base })
                }
                // A constant plus something the fold does not know is the array
                // address: the unknown is the selector, scaled.
                (Some(Val::Const(a)), None) | (None, Some(Val::Const(a))) => Some(Val::Indexed(a)),
                (Some(Val::Indexed(a)), None) | (None, Some(Val::Indexed(a))) => {
                    Some(Val::Indexed(a))
                }
                _ => None,
            }
        }
        OpCode::CPUI_INT_SUB => match (get(0)?, get(1)?) {
            (Val::Const(a), Val::Const(b)) => {
                Some(Val::Const(a.wrapping_sub(b) & mask(op.out.as_ref()?.size)))
            }
            _ => None,
        },
        // A read of the image through a computed, constant-based address: the
        // table entry. `in0` is the space id, and a load through anything but the
        // default data space is not a read of the image at all.
        OpCode::CPUI_LOAD => {
            let idx = data.map(|d| d.get_index())?;
            let space = op.ins.first()?;
            if space.offset != u64::try_from(idx).ok()? {
                return None;
            }
            let width = op.out.as_ref()?.size;
            match get(1)? {
                Val::Indexed(table) if width == 4 || width == pool.ptr_width() => {
                    Some(Val::Entry { table, width })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Drop every tracked value whose bytes overlap the write to `key`, so a write to
/// `eax` cannot leave a stale `rax` behind.
fn invalidate(vals: &mut Vec<(Key, Val)>, key: Option<Key>) {
    let Some(key) = key else { return };
    vals.retain(|(k, _)| !keys_overlap(*k, key));
}

fn split(key: Key) -> (u8, u64, u32) {
    match key {
        Key::Reg(off, size) => (0, off, size),
        Key::Tmp(off, size) => (1, off, size),
    }
}

/// Do two keys name overlapping bytes of the same class — `eax` and `rax`, but
/// never a register and a temporary at the same offset?
fn keys_overlap(a: Key, b: Key) -> bool {
    let (ac, ao, as_) = split(a);
    let (bc, bo, bs) = split(b);
    ac == bc && spans_overlap(ao, as_, bo, bs)
}

fn spans_overlap(a_off: u64, a_size: u32, b_off: u64, b_size: u32) -> bool {
    let a_end = a_off.saturating_add(u64::from(a_size));
    let b_end = b_off.saturating_add(u64::from(b_size));
    a_off < b_end && b_off < a_end
}

fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

fn sign_extend(v: u64, size: u32) -> u64 {
    if size == 0 || size >= 8 {
        return v;
    }
    let bits = 8 * size;
    let v = v & mask(size);
    if v >> (bits - 1) == 1 {
        v | !mask(size)
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A three-entry table at 0x2000 into code at 0x1000..0x1100, then a word of
    /// `0xcccccccc` padding — the shape MSVC emits.
    const TABLE: [u8; 20] = [
        0x10, 0x10, 0x00, 0x00, // 0x2000 -> 0x1010
        0x20, 0x10, 0x00, 0x00, // 0x2004 -> 0x1020
        0x30, 0x10, 0x00, 0x00, // 0x2008 -> 0x1030
        0xcc, 0xcc, 0xcc, 0xcc, // 0x200c -> padding
        0x40, 0x10, 0x00, 0x00, // 0x2010 -> 0x1040 (past the padding)
    ];
    const EXEC: [(u64, u64); 1] = [(0x1000, 0x1100)];

    fn image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x2000, 0x2014, &TABLE[..])], 4, true).unwrap()
    }

    #[test]
    fn a_table_of_code_addresses_yields_every_case_body() {
        assert_eq!(
            targets(0x2000, 0x1008, &image(), &EXEC, None),
            vec![0x1010, 0x1020, 0x1030]
        );
    }

    /// The stop rule: the first word that is not a code address ends the table,
    /// so the entry past the padding is not taken up.
    #[test]
    fn the_scan_stops_at_the_first_word_that_is_not_code() {
        let t = targets(0x2000, 0x1008, &image(), &EXEC, None);
        assert!(!t.contains(&0x1040));
    }

    /// A single plausible word after a constant is a coincidence, not a switch.
    #[test]
    fn a_run_shorter_than_two_entries_is_not_a_table() {
        assert!(targets(0x2008, 0x1008, &image(), &EXEC, None).is_empty());
    }

    /// A case body is the dispatcher's own code: a word landing in another
    /// executable range ends the table rather than extending it.
    #[test]
    fn a_word_outside_the_dispatchers_own_section_ends_the_table() {
        let exec = [(0x1000, 0x1015), (0x1020, 0x1100)];
        assert!(targets(0x2000, 0x1008, &image(), &exec, None).is_empty());
    }

    /// Nothing is read for an instruction the executable partition does not
    /// contain (a relocatable object, whose sections are not the runtime ones).
    #[test]
    fn a_dispatch_in_no_executable_section_reads_no_table() {
        assert!(targets(0x2000, 0x9000, &image(), &EXEC, None).is_empty());
    }

    /// The gcc form: 4-byte displacements measured from the table itself.
    const DELTA: [u8; 20] = [
        0x10, 0xf0, 0xff, 0xff, // 0x3000 -> 0x3000 + (-0xff0) = 0x2010
        0x20, 0xf0, 0xff, 0xff, // 0x3004 -> 0x2020
        0x30, 0xf0, 0xff, 0xff, // 0x3008 -> 0x2030
        0x40, 0xf0, 0xff, 0xff, // 0x300c -> 0x2040, past the switch's own bound
        0xcc, 0xcc, 0xcc, 0xcc,
    ];
    const DELTA_EXEC: [(u64, u64); 1] = [(0x2000, 0x2100)];

    fn delta_image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x3000, 0x3014, &DELTA[..])], 8, true).unwrap()
    }

    #[test]
    fn a_delta_table_composes_every_entry_with_its_base() {
        assert_eq!(
            delta_targets(0x3000, 0x3000, 0x2008, &delta_image(), &DELTA_EXEC, Some(3)),
            vec![0x2010, 0x2020, 0x2030]
        );
    }

    /// The MSVC form: the entries are RVAs and the base is the image base, so the
    /// table is not the base and is not even reachable from it by a fixed offset
    /// the read knows about.
    #[test]
    fn a_delta_table_reads_from_a_base_that_is_not_the_table() {
        const RVA: [u8; 12] = [
            0x10, 0x20, 0x00, 0x00, // -> 0x1000 + 0x2010
            0x20, 0x20, 0x00, 0x00,
            0x30, 0x20, 0x00, 0x00,
        ];
        let image = PoolImage::from_ranges(vec![(0x5000, 0x500c, &RVA[..])], 8, true).unwrap();
        assert_eq!(
            delta_targets(0x5000, 0x1000, 0x3020, &image, &[(0x3000, 0x3100)], Some(3)),
            vec![0x3010, 0x3020, 0x3030]
        );
    }

    /// The bound is the switch's own case count: without it the run continues
    /// into whatever follows the table, which on a real image is another table
    /// whose entries are equally good code addresses.
    #[test]
    fn the_case_count_stops_the_scan_at_the_end_of_the_table() {
        let t = delta_targets(0x3000, 0x3000, 0x2008, &delta_image(), &DELTA_EXEC, Some(3));
        assert!(!t.contains(&0x2040));
        assert_eq!(t.len(), 3);
    }

    /// The table must be read-only image content: an unaligned or unmapped base
    /// is not one.
    #[test]
    fn a_base_that_is_not_a_readable_word_reads_no_table() {
        assert!(targets(0x2001, 0x1008, &image(), &EXEC, None).is_empty());
        assert!(targets(0x8000, 0x1008, &image(), &EXEC, None).is_empty());
    }
}
