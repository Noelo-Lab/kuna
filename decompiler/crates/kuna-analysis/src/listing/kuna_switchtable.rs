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
//! [`Jump`](super::xrefs::XrefKind::Jump) and not a `Data`, so a veneer is never
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
use kuna_decomp::architecture::Architecture;
use kuna_decomp::fspec::effect_type;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::xrefs::{decode, in_range, FullCapture, FullOp};

/// Below this an accepted run is a coincidence rather than a table: a compiler
/// does not lower a one-case switch through a jump table.
pub(super) const MIN_ENTRIES: usize = 2;

/// The largest table read when the caller names no ceiling: the engine's own
/// default (`Architecture::max_jumptable_size`, 1024). The walk passes the live
/// `jumptablemax` instead, so raising it for a giant switch reaches this tier too.
pub(super) const DEFAULT_MAX_ENTRIES: usize = 1024;

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
    max_entries: usize,
) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let stride = u64::from(pool.ptr_width());
    let mut out = Vec::new();
    for i in 0..cases.unwrap_or(max_entries).min(max_entries) as u64 {
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
    max_entries: usize,
) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for i in 0..cases.unwrap_or(max_entries).min(max_entries) as u64 {
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

/// How far back the dispatch chain may reach. gcc and MSVC usually establish
/// the base one to five instructions ahead of the jump, but gcc -O2 hoists it
/// out of a loop into a callee-saved register, as far as the prologue: diff's
/// `main` loads its option table with `lea jt(%rip),%r15` right after the
/// pushes and dispatches on `getopt_long`'s result some sixty instructions
/// and nine calls later. The slice stops long before this wherever the chain
/// is satisfied, a call clobbers what it still wants, or the code above stops
/// falling through.
const MAX_DISPATCH_INSNS: usize = 96;

/// How far back the range check may sit. It precedes the whole dispatch chain,
/// not just the part the slice keeps: gcc -O0 spends seven instructions between
/// its `ja` and the jump (`mov %eax,%eax; lea 0(,%rax,4),%rdx; lea jt,%rax; mov
/// (%rdx,%rax,1),%eax; cltq; lea jt,%rdx; add %rdx,%rax`), and gcc -O2 schedules
/// the callee-saved pushes and argument moves of the case bodies in between.
const MAX_GUARD_INSNS: usize = 16;

/// The widest instruction any vendored processor encodes, which bounds how far
/// back a predecessor's first byte can lie.
const MAX_INSN_BYTES: u64 = 16;

/// How far the register-chain read follows a dispatch before it gives up, and
/// the largest table it reads.
///
/// `far` is the `switchreach` option (on by default): the 96-instruction slice
/// that steps over calls through the registers in `preserved`, the multiply by
/// one that is an unscaled index, the 16-instruction search for the range check,
/// the masked index, and the re-read once the code above a dispatch is decoded
/// (`TableRead::gap`). Off, the slice and the search stop at eight instructions
/// and at the first call, which is how far the read reached before.
#[derive(Debug, Clone, Copy)]
pub(super) struct Reach<'a> {
    pub max_entries: usize,
    pub preserved: &'a [(u64, u64)],
    pub far: bool,
}

impl Reach<'_> {
    fn slice_insns(&self) -> usize {
        if self.far { MAX_DISPATCH_INSNS } else { NEAR_INSNS }
    }

    fn guard_insns(&self) -> usize {
        if self.far { MAX_GUARD_INSNS } else { NEAR_INSNS }
    }
}

/// How far both searches reach with `switchreach` off.
const NEAR_INSNS: usize = 8;

/// The case bodies the computed jump at `from` dispatches to when its target
/// register — not the instruction's own operand — carries the table.
///
/// Answers the two shapes in the module header plus the absolute table read
/// through a register (`jmp *(%rdx,%rax,8)`), and nothing else: the fold reports
/// a table only where the `BRANCHIND`'s input is a word drawn from a
/// constant-based array, so a jump through a plain pointer, a virtual call and a
/// returned function pointer all decline before any memory is read.
///
/// The second half of the answer is the case count the range check states,
/// which can exceed `max_entries`: the table is then read only that far, and the
/// caller reports the switch as over the cap.
#[allow(clippy::too_many_arguments)]
pub(super) fn register_targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data_space: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    branch: &VarnodeData,
    pool: &PoolImage,
    exec: &[(u64, u64)],
    reach: &Reach,
) -> TableRead {
    let max_entries = reach.max_entries;
    // Nothing is read for a jump whose target is not in a register or a
    // temporary: `jmp qword ptr [__imp_X]` branches on a data-space varnode and
    // is a forwarding veneer, which is the shape this must never walk back from.
    let Some(want) = key_of(branch, data_space) else { return TableRead::default() };
    let (slice, gap) = dispatch_slice(translate, code_space, data_space, decoded, from, want, reach);

    let mut vals: Vec<(Key, Val)> = Vec::new();
    let mut dispatch = None;
    let mut index: Option<(u64, usize, Key)> = None;
    'slice: for (vma, ops) in &slice {
        for (i, op) in ops.iter().enumerate() {
            if op.opcode == OpCode::CPUI_BRANCHIND && *vma == from {
                dispatch = op.ins.first().and_then(|vn| value_of(vn, &vals, data_space));
                break 'slice;
            }
            if leaves_registers_unknown(op.opcode) {
                vals.retain(|(k, _)| survives(*k, reach.preserved));
            }
            let folded = fold(op, &vals, data_space, pool);
            if op.opcode == OpCode::CPUI_INT_ADD && matches!(folded, Some(Val::Indexed(_))) {
                let unknown = op.ins.iter().find(|vn| value_of(vn, &vals, data_space).is_none());
                if let Some(key) = unknown.and_then(|vn| key_of(vn, data_space)) {
                    index = Some((*vma, i, key));
                }
            }
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
        _ => return TableRead { gap: gap.filter(|_| reach.far), ..TableRead::default() },
    };
    // Read only now, on a jump that really does index a table: without the
    // switch's own case count there is nothing to stop the scan at the end of
    // it, and a delta table's neighbour reads as more cases.
    let read = |limit: usize| match table {
        (table, Some(base)) => delta_targets(table, base, from, pool, exec, Some(limit), max_entries),
        (table, None) => targets(table, from, pool, exec, Some(limit), max_entries),
    };
    // A masked index bounds the table whatever the nearest check compares:
    // clang guards `switch (x & 3)` under `if (y <= 200)` with the `cmp $0xc8`,
    // and a mask tighter than the check is the table's size.
    let masked = index.filter(|_| reach.far).and_then(|at| {
        mask_limit(translate, code_space, data_space, decoded, from, at, max_entries)
    });
    match (guard_limit(translate, code_space, data_space, decoded, from, reach), masked) {
        (Ok(limit), masked) => {
            let limit = masked.map_or(limit, |n| n.min(limit));
            TableRead { cases: read(limit), stated: Some(limit), gap: None }
        }
        (Err(gap), masked) => match masked.map(|n| (n, read(n))) {
            Some((n, cases)) if cases.len() == n => TableRead { cases, stated: Some(n), gap: None },
            _ => TableRead { gap: gap.filter(|_| reach.far), ..TableRead::default() },
        },
    }
}

/// The op index of the conditional branch that leaves `ops` (the instruction at
/// `at`, `len` bytes long) for another instruction, if any. A `cmov`'s or a
/// `rep` prefix's `CBRANCH` only skips the rest of its own instruction (it goes
/// to `inst_next`, or to an op of the same instruction), so it is no guard.
fn leaving_branch(ops: &[FullOp], at: u64, len: u32) -> Option<usize> {
    let next = at.wrapping_add(u64::from(len));
    ops.iter().rposition(|op| {
        op.opcode == OpCode::CPUI_CBRANCH
            && op.ins.first().is_some_and(|dest| !is_constant(dest) && !(at..=next).contains(&dest.offset))
    })
}

/// How many entries a table needs whose index is masked rather than checked,
/// or `None` unless that is provable here.
///
/// bash's readline dispatches on `and $0x7,%eax` under a guard that only
/// bounds the selector from below (`cmp $0x7,%eax; jbe`), and clang's Duff's
/// device in `internal_malloc` on `and $0x7,%esi` under a `cmp $0x20,%rdx; ja`
/// on the byte count, so the mask is the table's size. From the addend `at`
/// (instruction, op, varnode) of the table-address sum the fold could not
/// value, the index is followed back over the code that falls into the
/// dispatch, through scaling, copies and zero extensions, to an `and` with a
/// constant. The caller takes the smaller of this and the range check, and with
/// no range check requires every entry to be a case.
fn mask_limit(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    at: (u64, usize, Key),
    max_entries: usize,
) -> Option<usize> {
    let (site, upto, mut key) = at;
    let mut cap = FullCapture::default();
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    let mut reached = false;
    for step in 0..MAX_GUARD_INSNS {
        let len = decode(translate, cur, code_space, &mut cap)?;
        if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) {
            return None;
        }
        let ops = cap.ops();
        if step > 0 && !falls_through(ops) {
            return None;
        }
        reached |= cur == site;
        let ops = if cur == site { &ops[..upto.min(ops.len())] } else { ops };
        for op in ops.iter().rev().filter(|_| reached) {
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
            if !keys_overlap(out, key) {
                continue;
            }
            let input = op.ins.first().and_then(|vn| key_of(vn, data));
            if out != key {
                // `rax = zext(eax)` leaves the `eax` being followed as it was.
                if op.opcode == OpCode::CPUI_INT_ZEXT && input == Some(key) {
                    continue;
                }
                return None;
            }
            let constant = |i: usize| op.ins.get(i).filter(|vn| is_constant(vn)).map(|vn| vn.offset);
            key = match op.opcode {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => input?,
                OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_LEFT if constant(1).is_some() => input?,
                OpCode::CPUI_INT_AND => {
                    let mask = constant(1)?;
                    return usize::try_from(mask.checked_add(1)?).ok().filter(|&n| n <= max_entries);
                }
                _ => return None,
            };
        }
        ends_at = Some(cur);
        cur = previous(decoded, cur)?;
    }
    None
}

/// What [`register_targets`] read at one computed jump.
#[derive(Debug, Default)]
pub(super) struct TableRead {
    /// The case bodies, in table order.
    pub cases: Vec<u64>,
    /// The case count the range check states.
    pub stated: Option<usize>,
    /// Where the read stopped for want of a decoded instruction ending at this
    /// address: the walk reached the dispatch before the code falling into it.
    pub gap: Option<u64>,
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
/// The guard is the nearest conditional branch above the dispatch, and the
/// bound is a comparison feeding that branch's condition: in the branch itself,
/// or in the nearest instruction above it that writes what the condition reads.
/// Instructions that leave the condition alone may sit in between (gcc -O2
/// schedules `movb $0x3f,0x34(%rsp)` between `cmpb $0x56,(%rbx)` and its `ja`);
/// a comparison that does not reach the condition (the carry of a `cmp` ahead
/// of a `je`) is not a bound. Which side of the comparison the dispatch is on
/// comes from evaluating the condition both ways ([`guard_cases`]).
fn guard_limit(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    reach: &Reach,
) -> Result<usize, Option<u64>> {
    let max_entries = reach.max_entries;
    let mut cap = FullCapture::default();
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    let mut guard: Option<(Vec<FullOp>, usize, Vec<Key>)> = None;
    for step in 0..reach.guard_insns() {
        let len = decode(translate, cur, code_space, &mut cap).ok_or(None)?;
        if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) {
            return Err(ends_at);
        }
        if step > 0 && !falls_through(cap.ops()) {
            return Err(None);
        }
        let ops = cap.ops();
        match &guard {
            None => {
                if let Some(at) = leaving_branch(ops, cur, len) {
                    let want: Vec<Key> = ops[at].ins.get(1).and_then(|vn| key_of(vn, data)).into_iter().collect();
                    let (reads, feeding) = feeders(&ops[..at], want, data);
                    if let Some(n) = guard_cases(&[], &feeding, ops, at, data, max_entries) {
                        return Ok(n);
                    }
                    let reads = reads.into_iter().filter(|k| matches!(k, Key::Reg(..))).collect();
                    guard = Some((ops.to_vec(), at, reads));
                }
            }
            Some((branch, at, reads)) => {
                let writes = ops.iter().filter_map(|op| op.out.as_ref().and_then(|vn| key_of(vn, data)));
                let written: Vec<Key> = writes.filter(|w| reads.iter().any(|r| keys_overlap(*r, *w))).collect();
                if !written.is_empty() {
                    let (_, feeding) = feeders(ops, written, data);
                    return guard_cases(ops, &feeding, branch, *at, data, max_entries).ok_or(None);
                }
            }
        }
        ends_at = Some(cur);
        cur = previous(decoded, cur).ok_or(Some(cur))?;
    }
    Err(None)
}

/// The case count a comparison among `writer[feeding]` (or, with an empty
/// `writer`, among `branch[feeding]`) states for the dispatch on the guard's
/// fall-through side: the out-of-range values must go to the branch target and
/// the rest down to the dispatch.
///
/// The comparison is a selector against a constant `c` in any of the four
/// spellings (`sel < c` on x86, `c <= sel` on AArch64, signed or not), and the
/// condition is evaluated with the comparison as it comes out for a selector
/// above `c` and below it, every other flag the comparison sets read as clear.
/// `ja` after `cmp $c,sel` branches for the large selector, so the dispatch
/// sees `sel <= c`: `c + 1` cases. `jb` branches for the small one, so the
/// dispatch sees `sel >= c`, which an unsigned comparison of a `w`-byte selector
/// bounds at `2^(8w) - c` — gcc's `cmp $0xfffffffb,%r12d; jb default` ahead of a
/// five-case table indexed by `sel + 5`. Read as `c + 1`, that one ran on into
/// the next table. A lower bound that leaves more than `max_entries` values is
/// no bound at all: it is the first of two checks, not a table's size.
fn guard_cases(
    writer: &[FullOp],
    feeding: &[usize],
    branch: &[FullOp],
    at: usize,
    data: Option<&Rc<AddrSpace>>,
    max_entries: usize,
) -> Option<usize> {
    let ops = if writer.is_empty() { branch } else { writer };
    let mut best: Option<u64> = None;
    for &i in feeding {
        let op = &ops[i];
        let signed = match op.opcode {
            OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => false,
            OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => true,
            _ => continue,
        };
        let (Some(a), Some(b)) = (op.ins.first(), op.ins.get(1)) else { continue };
        let (sel, c, above) = match (constant_in(&ops[..i], a, data), constant_in(&ops[..i], b, data)) {
            (None, Some(c)) => (a, c, 0),
            (Some(c), None) => (b, c, 1),
            _ => continue,
        };
        if c == 0 {
            continue;
        }
        let taken = |x: u64| condition_when(writer, i, branch, at, x, data);
        let n = match (taken(above), taken(1 - above)) {
            (Some(true), Some(false)) => c.checked_add(1),
            (Some(false), Some(true)) if !signed && sel.size < 8 => (1u64 << (8 * sel.size))
                .checked_sub(c)
                .filter(|&n| n <= max_entries as u64),
            _ => None,
        };
        best = best.max(n);
    }
    usize::try_from(best?).ok()
}

/// `vn`'s value when it is a constant, or a temporary `ops` copied one into
/// (AArch64 SLEIGH spells `cmp x8, #6` as `tmp = 6; tmpCY = tmp <= x8`).
fn constant_in(ops: &[FullOp], vn: &VarnodeData, data: Option<&Rc<AddrSpace>>) -> Option<u64> {
    if is_constant(vn) {
        return Some(vn.offset);
    }
    let key = key_of(vn, data).filter(|k| matches!(k, Key::Tmp(..)))?;
    let def = ops.iter().rev().find(|op| op.out.as_ref().and_then(|o| key_of(o, data)) == Some(key))?;
    let src = def.ins.first().filter(|_| def.opcode == OpCode::CPUI_COPY)?;
    is_constant(src).then_some(src.offset)
}

/// The guard's condition when the op at `set` (in `writer`, or in `branch`
/// when `writer` is empty) yields `x`: `writer` runs first and `branch` up to
/// its `CBRANCH` at `at` after it, and a register `branch` reads that nothing
/// evaluated is a clear flag. `None` when the condition does not follow.
fn condition_when(
    writer: &[FullOp],
    set: usize,
    branch: &[FullOp],
    at: usize,
    x: u64,
    data: Option<&Rc<AddrSpace>>,
) -> Option<bool> {
    let mut env: Vec<(Key, u64)> = Vec::new();
    let in_branch = writer.is_empty();
    let phases: [(&[FullOp], bool, Option<usize>); 2] =
        [(writer, false, Some(set)), (&branch[..at], true, in_branch.then_some(set))];
    for (ops, clear, set) in phases {
        for (i, op) in ops.iter().enumerate() {
            let Some(out) = op.out.as_ref() else { continue };
            let Some(key) = key_of(out, data) else { continue };
            let val = if set == Some(i) { Some(x) } else { evaluate(op, &env, clear, data) };
            env.retain(|(k, _)| !keys_overlap(*k, key));
            if let Some(v) = val {
                env.push((key, v & mask(out.size)));
            }
        }
    }
    let cond = branch.get(at)?.ins.get(1)?;
    known(cond, &env, true, data).map(|v| v != 0)
}

/// `vn`'s value in `env`; with `clear`, a register nothing set reads as zero.
fn known(vn: &VarnodeData, env: &[(Key, u64)], clear: bool, data: Option<&Rc<AddrSpace>>) -> Option<u64> {
    if is_constant(vn) {
        return Some(vn.offset & mask(vn.size));
    }
    let key = key_of(vn, data)?;
    match env.iter().rev().find(|(k, _)| *k == key) {
        Some(&(_, v)) => Some(v),
        None if clear && matches!(key, Key::Reg(..)) => Some(0),
        None => None,
    }
}

/// The boolean and bitwise ops a branch condition is spelled in.
fn evaluate(op: &FullOp, env: &[(Key, u64)], clear: bool, data: Option<&Rc<AddrSpace>>) -> Option<u64> {
    let get = |i: usize| known(op.ins.get(i)?, env, clear, data);
    Some(match op.opcode {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => get(0)?,
        OpCode::CPUI_BOOL_NEGATE => u64::from(get(0)? == 0),
        OpCode::CPUI_BOOL_AND => u64::from(get(0)? != 0 && get(1)? != 0),
        OpCode::CPUI_BOOL_OR => u64::from(get(0)? != 0 || get(1)? != 0),
        OpCode::CPUI_BOOL_XOR => u64::from((get(0)? != 0) != (get(1)? != 0)),
        OpCode::CPUI_INT_AND => get(0)? & get(1)?,
        OpCode::CPUI_INT_OR => get(0)? | get(1)?,
        OpCode::CPUI_INT_XOR => get(0)? ^ get(1)?,
        OpCode::CPUI_INT_EQUAL => u64::from(get(0)? == get(1)?),
        OpCode::CPUI_INT_NOTEQUAL => u64::from(get(0)? != get(1)?),
        _ => return None,
    })
}

/// The closest decoded address below `cur`: the walk's partition does not
/// overlap, so anything further back either ends before `cur` or is not an
/// instruction start at all.
fn previous(decoded: &HashSet<u64>, cur: u64) -> Option<u64> {
    (1..=MAX_INSN_BYTES).filter_map(|d| cur.checked_sub(d)).find(|at| decoded.contains(at))
}

/// Does control reach the next instruction from this one? The walk's own rule
/// (`classify`): it does unless the last op is a `BRANCH`, `BRANCHIND` or
/// `RETURN`. Stepping back over an instruction that does not is leaving the
/// block for code that never runs ahead of the dispatch.
fn falls_through(ops: &[FullOp]) -> bool {
    !ops.last().is_some_and(|op| {
        matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN)
    })
}

/// Within one instruction's `ops`, the ops that compute any of `want`, by index,
/// and the varnodes that computation reads from outside the instruction.
fn feeders(ops: &[FullOp], mut want: Vec<Key>, data: Option<&Rc<AddrSpace>>) -> (Vec<Key>, Vec<usize>) {
    let mut feeding = Vec::new();
    for (i, op) in ops.iter().enumerate().rev() {
        let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
        if !want.iter().any(|w| keys_overlap(*w, out)) {
            continue;
        }
        want.retain(|w| !keys_overlap(*w, out));
        feeding.push(i);
        for k in op.ins.iter().filter_map(|vn| key_of(vn, data)) {
            if !want.contains(&k) && want.len() < MAX_TRACKED {
                want.push(k);
            }
        }
    }
    (want, feeding)
}

pub(super) fn is_constant(vn: &VarnodeData) -> bool {
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
    reach: &Reach,
) -> (Vec<(u64, Vec<FullOp>)>, Option<u64>) {
    let mut cap = FullCapture::default();
    let mut need: Vec<Key> = vec![want];
    let mut out: Vec<(u64, Vec<FullOp>)> = Vec::new();
    let mut cur = from;
    let mut ends_at: Option<u64> = None;
    let mut gap = None;
    for step in 0..reach.slice_insns() {
        let Some(len) = decode(translate, cur, code_space, &mut cap) else { break };
        // The step back trusted the closest decoded address below the previous
        // instruction; this is where that is confirmed, at no cost, by requiring
        // the two to be contiguous.
        if ends_at.is_some_and(|end| cur.wrapping_add(u64::from(len)) != end) {
            gap = ends_at;
            break;
        }
        if step > 0 && !falls_through(cap.ops()) {
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
            if !models(op, reach.far) {
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
        if stop && !need.is_empty() && need.iter().all(|k| survives(*k, reach.preserved)) {
            if !relevant {
                out.push((cur, cap.ops().to_vec()));
            }
            stop = false;
        }
        if need.is_empty() || stop {
            break;
        }
        let Some(prev) = previous(decoded, cur) else {
            gap = Some(cur);
            break;
        };
        ends_at = Some(cur);
        cur = prev;
    }
    out.reverse();
    (out, gap)
}

/// Does the fold give this op's output a value derived from its inputs? The
/// slice follows only these, which is what prunes the switch selector's own
/// chain: a scaled index reaches the table address through `INT_MULT`, and
/// nothing before that multiply can change what the table is. A multiply by
/// one is the exception: it is how x86 SLEIGH spells an unscaled index, and
/// gcc -O0 puts the table base in that slot (`mov (%rdx,%rax,1),%eax`).
fn models(op: &FullOp, far: bool) -> bool {
    match op.opcode {
        OpCode::CPUI_COPY
        | OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT
        | OpCode::CPUI_LOAD => true,
        OpCode::CPUI_INT_MULT => far && op.ins.iter().any(|vn| is_constant(vn) && vn.offset == 1),
        _ => false,
    }
}

/// How many values the fold carries at once. A dispatch chain needs two (the
/// base and the entry); the cap keeps a long window of constant materializations
/// from accumulating.
pub(super) const MAX_TRACKED: usize = 16;

/// What the fold knows about a value. Only [`Val::Case`] and a pointer-wide
/// [`Val::Entry`] are answers; the other two exist to get there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Val {
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
    /// A byte or halfword entry doubled: a Thumb `tbb`/`tbh` branch offset.
    Halves { table: u64, width: u32 },
    /// `base` plus a doubled entry: the case address of a `tbb`/`tbh`.
    Short { table: u64, width: u32, base: u64 },
}

/// A varnode's identity, in the two classes the fold models — the same split
/// [`super::kuna_picpool`] makes, since `register` and `ram` are both processor
/// spaces and only the default data space tells them apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Key {
    Reg(u64, u32),
    Tmp(u64, u32),
}

pub(super) fn key_of(vn: &VarnodeData, data: Option<&Rc<AddrSpace>>) -> Option<Key> {
    let space = vn.space.as_ref()?;
    match space.get_type() {
        spacetype::IPTR_INTERNAL => Some(Key::Tmp(vn.offset, vn.size)),
        spacetype::IPTR_PROCESSOR if !matches!(data, Some(d) if Rc::ptr_eq(space, d)) => {
            Some(Key::Reg(vn.offset, vn.size))
        }
        _ => None,
    }
}

/// What the fold knows about `vn`: its own tracked value, else the low part of
/// the wider register that holds it. x86-64 writes `rax = zext(eax)` after
/// every 32-bit load, and that write is what retires `eax`'s own entry, so
/// the `cltq` reading `eax` back would otherwise see nothing.
pub(super) fn value_of(vn: &VarnodeData, vals: &[(Key, Val)], data: Option<&Rc<AddrSpace>>) -> Option<Val> {
    let space = vn.space.as_ref()?;
    if space.get_type() == spacetype::IPTR_CONSTANT {
        return Some(Val::Const(vn.offset & mask(vn.size)));
    }
    let key = key_of(vn, data)?;
    if let Some((_, v)) = vals.iter().rev().find(|(k, _)| *k == key) {
        return Some(*v);
    }
    let big = data.is_some_and(|d| d.is_big_endian());
    let (class, off, size) = split(key);
    vals.iter().rev().find_map(|&(k, v)| {
        let (c, o, s) = split(k);
        let low = if big { o.checked_add(u64::from(s - size)) == Some(off) } else { o == off };
        if c != class || s <= size || !low {
            return None;
        }
        match v {
            Val::Const(c) => Some(Val::Const(c & mask(size))),
            Val::Entry { width, .. } if width <= size => Some(v),
            _ => None,
        }
    })
}

/// Does this op make every tracked register unbelievable? A call clobbers the
/// caller-saved ones; what the calling convention preserves survives it
/// ([`survives`]).
fn leaves_registers_unknown(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
    )
}

/// Is `key` a register the calling convention preserves across a call? gcc -O2
/// keeps a table base in `%rbx`/`%r13` across the calls between a loop's head
/// and its dispatch (`lea jt(%rip),%r13; ... call f; cmp $5,%eax; ja; mov
/// %eax,%eax; movslq 0(%r13,%rax,4),%rax`), and the compiler relied on the
/// callee keeping it.
fn survives(key: Key, preserved: &[(u64, u64)]) -> bool {
    let Key::Reg(off, size) = key else { return false };
    let end = off.saturating_add(u64::from(size));
    preserved.iter().any(|&(lo, hi)| off >= lo && end <= hi)
}

/// The register ranges `[lo, hi)` the default calling convention leaves
/// unaffected by a call.
pub(super) fn preserved_registers(arch: &Architecture) -> Vec<(u64, u64)> {
    let Some(model) = arch.default_fp() else { return Vec::new() };
    let data = arch.manage().get_default_data_space();
    model
        .effect_list()
        .iter()
        .filter(|e| e.get_type() == effect_type::UNAFFECTED && e.get_size() > 0)
        .filter_map(|e| {
            let addr = e.get_address();
            let space = addr.get_space()?;
            let register = space.get_type() == spacetype::IPTR_PROCESSOR
                && !data.is_some_and(|d| Rc::ptr_eq(d, space));
            let lo = addr.get_offset();
            register.then(|| (lo, lo.saturating_add(e.get_size() as u64)))
        })
        .collect()
}

/// The value this op produces, or `None` where the fold does not model it.
pub(super) fn fold(
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
            v @ (Val::Entry { .. } | Val::Case { .. } | Val::Halves { .. }) => Some(v),
            Val::Indexed(_) | Val::Short { .. } => None,
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
                (Some(Val::Halves { table, width }), Some(Val::Const(base)))
                | (Some(Val::Const(base)), Some(Val::Halves { table, width })) => {
                    Some(Val::Short { table, width, base })
                }
                (Some(Val::Const(a) | Val::Indexed(a)), Some(Val::Entry { width: 1 | 2, .. }))
                | (Some(Val::Entry { width: 1 | 2, .. }), Some(Val::Const(a) | Val::Indexed(a))) => {
                    Some(Val::Indexed(a))
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
        OpCode::CPUI_INT_MULT => match (get(0)?, get(1)?) {
            (v, Val::Const(1)) | (Val::Const(1), v) => Some(v),
            (Val::Entry { table, width: width @ (1 | 2) }, Val::Const(2))
            | (Val::Const(2), Val::Entry { table, width: width @ (1 | 2) }) => {
                Some(Val::Halves { table, width })
            }
            _ => None,
        },
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
                Val::Indexed(table) | Val::Short { base: table, .. }
                    if matches!(width, 1 | 2 | 4) || width == pool.ptr_width() =>
                {
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
pub(super) fn invalidate(vals: &mut Vec<(Key, Val)>, key: Option<Key>) {
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
pub(super) fn keys_overlap(a: Key, b: Key) -> bool {
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
            targets(0x2000, 0x1008, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES),
            vec![0x1010, 0x1020, 0x1030]
        );
    }

    /// The stop rule: the first word that is not a code address ends the table,
    /// so the entry past the padding is not taken up.
    #[test]
    fn the_scan_stops_at_the_first_word_that_is_not_code() {
        let t = targets(0x2000, 0x1008, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES);
        assert!(!t.contains(&0x1040));
    }

    /// A single plausible word after a constant is a coincidence, not a switch.
    #[test]
    fn a_run_shorter_than_two_entries_is_not_a_table() {
        assert!(targets(0x2008, 0x1008, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES).is_empty());
    }

    /// A case body is the dispatcher's own code: a word landing in another
    /// executable range ends the table rather than extending it.
    #[test]
    fn a_word_outside_the_dispatchers_own_section_ends_the_table() {
        let exec = [(0x1000, 0x1015), (0x1020, 0x1100)];
        assert!(targets(0x2000, 0x1008, &image(), &exec, None, DEFAULT_MAX_ENTRIES).is_empty());
    }

    /// Nothing is read for an instruction the executable partition does not
    /// contain (a relocatable object, whose sections are not the runtime ones).
    #[test]
    fn a_dispatch_in_no_executable_section_reads_no_table() {
        assert!(targets(0x2000, 0x9000, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES).is_empty());
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
            delta_targets(0x3000, 0x3000, 0x2008, &delta_image(), &DELTA_EXEC, Some(3), DEFAULT_MAX_ENTRIES),
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
            delta_targets(0x5000, 0x1000, 0x3020, &image, &[(0x3000, 0x3100)], Some(3), DEFAULT_MAX_ENTRIES),
            vec![0x3010, 0x3020, 0x3030]
        );
    }

    /// The bound is the switch's own case count: without it the run continues
    /// into whatever follows the table, which on a real image is another table
    /// whose entries are equally good code addresses.
    #[test]
    fn the_case_count_stops_the_scan_at_the_end_of_the_table() {
        let t = delta_targets(0x3000, 0x3000, 0x2008, &delta_image(), &DELTA_EXEC, Some(3), DEFAULT_MAX_ENTRIES);
        assert!(!t.contains(&0x2040));
        assert_eq!(t.len(), 3);
    }

    /// The table must be read-only image content: an unaligned or unmapped base
    /// is not one.
    #[test]
    fn a_base_that_is_not_a_readable_word_reads_no_table() {
        assert!(targets(0x2001, 0x1008, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES).is_empty());
        assert!(targets(0x8000, 0x1008, &image(), &EXEC, None, DEFAULT_MAX_ENTRIES).is_empty());
    }

    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>, Rc<AddrSpace>) {
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
        )
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    /// x86-64 writes `rax = zext(eax)` after a 32-bit load, which retires the
    /// tracked `eax`; the `cltq` reading it back still sees the table entry.
    #[test]
    fn a_register_reads_as_the_low_part_of_the_wider_one_holding_it() {
        let (reg, _, _) = spaces();
        let entry = Val::Entry { table: 0x2000, width: 4 };
        let vals = vec![(Key::Reg(0, 8), entry)];
        assert_eq!(value_of(&vn(&reg, 0, 4), &vals, None), Some(entry));
        assert_eq!(value_of(&vn(&reg, 4, 4), &vals, None), None, "the high half is not the entry");
        let wide = vec![(Key::Reg(0, 8), Val::Const(0x1_0000_0002))];
        assert_eq!(value_of(&vn(&reg, 0, 4), &wide, None), Some(Val::Const(2)));
    }

    /// `cmp $c,sel` as x86 SLEIGH lifts it: CF and ZF for the guard to read.
    fn x86_cmp(reg: &Rc<AddrSpace>, uniq: &Rc<AddrSpace>, cst: &Rc<AddrSpace>, c: u64) -> Vec<FullOp> {
        let (eax, cf, zf, diff) = (vn(reg, 0, 4), vn(reg, 0x200, 1), vn(reg, 0x206, 1), vn(uniq, 0x100, 4));
        vec![
            op(OpCode::CPUI_INT_LESS, Some(cf), vec![eax.clone(), vn(cst, c, 4)]),
            op(OpCode::CPUI_INT_SUB, Some(diff.clone()), vec![eax, vn(cst, c, 4)]),
            op(OpCode::CPUI_INT_EQUAL, Some(zf), vec![diff, vn(cst, 0, 4)]),
        ]
    }

    /// The case count `guard_limit` reads for a compare `writer` and a branch
    /// `branch` whose last op is the `CBRANCH`.
    fn guarded(writer: &[FullOp], branch: &[FullOp], max: usize) -> Option<usize> {
        let at = branch.len() - 1;
        let want: Vec<Key> = branch[at].ins.get(1).and_then(|v| key_of(v, None)).into_iter().collect();
        let (reads, _) = feeders(&branch[..at], want, None);
        let reads: Vec<Key> = reads.into_iter().filter(|k| matches!(k, Key::Reg(..))).collect();
        let written: Vec<Key> = writer
            .iter()
            .filter_map(|o| o.out.as_ref().and_then(|v| key_of(v, None)))
            .filter(|w| reads.iter().any(|r| keys_overlap(*r, *w)))
            .collect();
        let (_, feeding) = feeders(writer, written, None);
        guard_cases(writer, &feeding, branch, at, None, max)
    }

    /// `ja` sends the selectors above `c` away: the dispatch sees `c + 1`.
    #[test]
    fn a_ja_guard_bounds_the_table_from_above() {
        let (reg, uniq, cst) = spaces();
        let (cf, zf, t) = (vn(&reg, 0x200, 1), vn(&reg, 0x206, 1), |o| vn(&uniq, o, 1));
        let ja = vec![
            op(OpCode::CPUI_BOOL_NEGATE, Some(t(0x10)), vec![cf]),
            op(OpCode::CPUI_BOOL_NEGATE, Some(t(0x11)), vec![zf]),
            op(OpCode::CPUI_BOOL_AND, Some(t(0x12)), vec![t(0x10), t(0x11)]),
            op(OpCode::CPUI_CBRANCH, None, vec![vn(&reg, 0x9000, 8), t(0x12)]),
        ];
        assert_eq!(guarded(&x86_cmp(&reg, &uniq, &cst, 5), &ja, 1024), Some(6));
    }

    /// `jb` sends the selectors below `c` away, so the dispatch sees `sel >= c`:
    /// five values of a 32-bit selector above `0xfffffffb`, and no bound at all
    /// above a small `c`, where the check is only the first of two.
    #[test]
    fn a_jb_guard_bounds_the_table_from_below() {
        let (reg, uniq, cst) = spaces();
        let jb = vec![op(OpCode::CPUI_CBRANCH, None, vec![vn(&reg, 0x9000, 8), vn(&reg, 0x200, 1)])];
        assert_eq!(guarded(&x86_cmp(&reg, &uniq, &cst, 0xffff_fffb), &jb, 1024), Some(5));
        assert_eq!(guarded(&x86_cmp(&reg, &uniq, &cst, 5), &jb, 1024), None);
    }

    /// `je` reads only ZF: the carry of the same `cmp` is no bound.
    #[test]
    fn a_compare_that_does_not_reach_the_condition_is_no_bound() {
        let (reg, uniq, cst) = spaces();
        let je = vec![op(OpCode::CPUI_CBRANCH, None, vec![vn(&reg, 0x9000, 8), vn(&reg, 0x206, 1)])];
        assert_eq!(guarded(&x86_cmp(&reg, &uniq, &cst, 5), &je, 1024), None);
    }

    /// AArch64 spells `cmp x8, #6; b.hi` as `tmp = 6; tmpCY = tmp <= x8; CY =
    /// tmpCY` and `CY && !ZR`.
    #[test]
    fn an_aarch64_guard_reads_the_constant_on_the_left() {
        let (reg, uniq, cst) = spaces();
        let (x8, tmp_cy, cy, zr) = (vn(&reg, 0x4040, 8), vn(&reg, 0x105, 1), vn(&reg, 0x102, 1), vn(&reg, 0x101, 1));
        let six = vn(&uniq, 0x20, 8);
        let subs = vec![
            op(OpCode::CPUI_COPY, Some(six.clone()), vec![vn(&cst, 6, 8)]),
            op(OpCode::CPUI_INT_LESSEQUAL, Some(tmp_cy.clone()), vec![six, x8]),
            op(OpCode::CPUI_COPY, Some(cy.clone()), vec![tmp_cy]),
        ];
        let hi = vec![
            op(OpCode::CPUI_BOOL_NEGATE, Some(vn(&uniq, 0x30, 1)), vec![zr]),
            op(OpCode::CPUI_BOOL_AND, Some(vn(&uniq, 0x31, 1)), vec![cy, vn(&uniq, 0x30, 1)]),
            op(OpCode::CPUI_CBRANCH, None, vec![vn(&reg, 0x9000, 8), vn(&uniq, 0x31, 1)]),
        ];
        assert_eq!(guarded(&subs, &hi, 1024), Some(7));
    }

    /// A `cmov` branches only to the end of its own instruction and a `rep`
    /// prefix to an op inside it; a `jcc` leaves for another instruction.
    #[test]
    fn only_a_branch_out_of_its_own_instruction_is_a_guard() {
        let (reg, _, cst) = spaces();
        let cbranch = |dest: VarnodeData| op(OpCode::CPUI_CBRANCH, None, vec![dest, vn(&reg, 0x200, 1)]);
        assert_eq!(leaving_branch(&[cbranch(vn(&reg, 0x1004, 8))], 0x1000, 4), None, "cmov");
        assert_eq!(leaving_branch(&[cbranch(vn(&cst, 2, 8))], 0x1000, 4), None, "rep");
        assert_eq!(leaving_branch(&[cbranch(vn(&reg, 0x2000, 8))], 0x1000, 4), Some(0), "jcc");
    }
}
