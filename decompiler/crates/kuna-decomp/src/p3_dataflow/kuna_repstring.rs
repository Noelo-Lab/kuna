//! (kuna `repstring`) An x86 `rep movs`, `rep stos` or `repz cmpsb` becomes a
//! copy, fill or compare builtin.
//!
//! A compiler that inlines a copy, fill or compare (gcc and clang at `-Os`,
//! MSVC's `/Oi` intrinsics, gcc's `-minline-all-stringops`) emits
//!
//! ```text
//!     mov  rcx, n          ; count, in elements
//!     rep  movsq           ; *rdi++ = *rsi++, rcx times        (memcpy)
//!     rep  stosq           ; *rdi++ = rax, rcx times           (memset)
//!     repz cmpsb           ; while (rcx-- && *rsi++ == *rdi++) (memcmp)
//! ```
//!
//! and SLEIGH lifts the repeat prefix as a loop inside the one instruction, so
//! the C shows `while (n) { *p = *q; n -= 1; p = p + 1; q = q + 1; }`.
//!
//! # Copies and fills
//!
//! On SSA, after the main loop has settled, the pass looks for the two blocks
//! the instruction lifts to: a head `H` and a body `B` whose ops all sit at one
//! instruction address, where `H` is entered from one block outside and from
//! `B`, `B` is entered only from `H` and leaves only to `H`, and `H`'s other
//! successor `E` starts at another address. It declines unless:
//!
//! * `H` holds, besides its branch, only the phis of the count, the destination
//!   and, for a copy, the source;
//! * `H` leaves to `E` exactly when the count is zero;
//! * `B` steps the count down by one and each pointer up by the element width
//!   `w` (1, 2, 4 or 8; a set direction flag steps them down and is declined),
//!   stores one element through the destination, and computes nothing else:
//!   a copy loads the element through the source, a fill stores a value
//!   defined before the loop;
//! * nothing outside the loop reads a value the body computes;
//! * each pointer is as wide as an address or a compiler-spec pointer (a
//!   compare's too): an x86-64 address-size override steps EDI/ESI, which wrap
//!   at 4 GiB as no builtin can.
//!
//! A destination in the function's own frame (the stack pointer plus a
//! constant) is declined: the builtin is placed after heritage, so the frame
//! locations it writes would have no definition at it, and the merge could fold
//! a value stored into the buffer after the copy into one held before it. A
//! frame location heritage already guards the element store with (an INDIRECT on
//! the store and a phi in the head, for an address-taken local the destination
//! might alias) is given an INDIRECT on the builtin instead, so a read of it
//! after the loop still sees the write.
//!
//! `builtin_memcpy(dst, src, n * w)` is placed in `H` before its branch, where
//! `dst`, `src` and `n` are the values entering the loop. A fill whose value
//! has the same byte throughout (`stosb` always, a constant such as `0` or
//! `0x2020202020202020`) is `builtin_memset(dst, c, n * w)`; any other fill of
//! 2-, 4- or 8-byte elements is `builtin_memset16`/`32`/`64(dst, value, n)`,
//! whose count is in elements. The builtin keeps the call flag `CALLOTHER`
//! carries, so it stays ordered like a call against the loads, stores and calls
//! around it. Every read after the loop of the count becomes `0`, and of a
//! pointer, the pointer plus `n * w`. The edge into `B` is then removed, so `H`
//! runs once and `B`, now unreachable, is deleted with the rest of the dead
//! code.
//!
//! `rep movs` is defined for a destination that overlaps its source ahead of
//! it (an LZ77 decoder's match copy replicates a pattern that way), and
//! `memcpy` is not; the rewrite cannot tell the two apart, which is what the
//! option is for.
//!
//! # Compares
//!
//! A `repz cmpsb` has the same head, and a body that leaves two ways: back to
//! `H` while the two bytes it loaded are equal, and out when they differ. The
//! body must step the count down by one and both pointers up by one, load one
//! byte through each, and otherwise compute only pure values of those two
//! bytes; that it loops exactly while they are equal is checked for all 65536
//! byte pairs. `r = builtin_memcmp(a, b, n)` is placed in `H`, whose branch
//! now leaves on `r == 0` instead of on a zero count, and the body's back edge
//! is removed, so the body is what runs when the blocks differ.
//!
//! What the code after the loop reads is rebuilt from `r`. A value the body
//! computes is read after it only for the first pair that differs, `x` from `a`
//! and `y` from `b`; over all such pairs it must be a constant (the zero flag
//! is 0), `x < y` (the carry flag, `r < 0`) or `x > y` (`0 < r`). A flag the
//! head merges is read after it when the count ran out: its value for an equal
//! pair, which must be one constant, provided the count enters as a non-zero
//! constant or the flag enters the loop holding that same constant (gcc
//! precedes the instruction with `cmp eax, eax` for this). A loop whose final
//! count or pointers are read afterwards, or any other value, keeps its loop.

use kuna_base::error::{KunaError, KunaResult};
use std::rc::Rc;

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode::*;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_strlenscan::{eval, PURE};
use crate::op::PcodeOp;
use crate::userop::{
    BUILTIN_MEMCMP, BUILTIN_MEMCPY, BUILTIN_MEMSET, BUILTIN_MEMSET16, BUILTIN_MEMSET32, BUILTIN_MEMSET64,
};

/// (kuna) `ActionRepString` — collapse an x86 `rep movs`/`rep stos`/`repz cmpsb`
/// loop into a copy, fill or compare builtin (option `repstring`).
pub struct ActionRepString {
    base: ActionBase,
}

impl ActionRepString {
    pub fn new(g: impl Into<String>) -> Self {
        Self { base: ActionBase::new(0, "repstring", g) }
    }
}

impl Action for ActionRepString {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(Self { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().rep_string {
            return 0;
        }
        let heads: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
        let mut changed = 0;
        for head in heads {
            let done = if let Some(rep) = match_rep(data, head) {
                rewrite(data, &rep)
            } else if let Some(compare) = match_compare(data, head) {
                rewrite_compare(data, &compare)
            } else {
                continue;
            };
            if done.is_err() {
                break;
            }
            changed += 1;
        }
        self.base.count += changed;
        (changed != 0) as ApplyResult
    }
}

/// What the element store writes: a copied element (the source pointer's phi
/// output and its entering value), a fill byte, constant or not, or a fill
/// element whose bytes differ.
enum Element {
    Copy { src_out: VarnodeId, src_in: VarnodeId },
    FillConst(u64),
    FillByte(VarnodeId),
    FillElement(VarnodeId),
}

/// One matched repeat loop: the head, the edge into the body, the values
/// entering the loop and the element width.
struct Rep {
    head: BlockId,
    body: BlockId,
    to_body: i32,
    width: i32,
    count_out: VarnodeId,
    count_in: VarnodeId,
    dst_out: VarnodeId,
    dst_in: VarnodeId,
    element: Element,
    /// The head phis of the frame locations the element store may write, which
    /// heritage carries through the body with an INDIRECT on the store.
    memory: Vec<VarnodeId>,
}

fn op(data: &Funcdata, id: OpId) -> Option<&PcodeOp> {
    data.obank().get(id)
}

fn size(data: &Funcdata, v: VarnodeId) -> Option<i32> {
    data.vbank().get(v).map(|n| n.get_size())
}

fn constant(data: &Funcdata, v: VarnodeId) -> Option<u64> {
    let n = data.vbank().get(v)?;
    n.is_constant().then(|| n.get_offset())
}

fn mask(width: i32) -> u64 {
    if width >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * width)) - 1
    }
}

/// Is `v` as wide as an address of the default data space, or as a pointer of
/// the compiler spec? A narrower pointer (`rep stos` with an address-size
/// override, through EDI on x86-64) wraps within its own width, which no C
/// copy, fill or compare can express. 32-bit code on the 64-bit model
/// (`x86:LE:64:compat32`) steps the program's own 4-byte pointers. With no
/// default data space to compare against, the rewrite is refused.
fn full_width(data: &Funcdata, v: VarnodeId) -> bool {
    let arch = data.get_arch();
    let Some(space) = arch.manage().get_default_data_space().map(|s| s.get_addr_size() as i32) else { return false };
    let pointer = arch.types().map(|t| t.get_size_of_pointer()).filter(|&p| p > 0);
    size(data, v).is_some_and(|width| width >= space || pointer.is_some_and(|p| width >= p))
}

fn def_block(data: &Funcdata, v: VarnodeId) -> Option<BlockId> {
    op(data, data.vbank().get(v)?.get_def()?)?.get_parent()
}

/// `v` through the COPYs that define it.
fn copied(data: &Funcdata, mut v: VarnodeId) -> VarnodeId {
    for _ in 0..4 {
        let Some(d) = data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| op(data, d)) else { break };
        if d.code() != CPUI_COPY {
            break;
        }
        let Some(input) = d.get_in(0) else { break };
        v = input;
    }
    v
}

/// The constant `inc` adds to `v` (INT_ADD either way round, INT_SUB, or a
/// PTRADD of a constant index), or `None`.
fn step(data: &Funcdata, inc: &PcodeOp, v: VarnodeId) -> Option<u64> {
    let width = size(data, v)?;
    let (a, b) = (inc.get_in(0)?, inc.get_in(1)?);
    let k = match inc.code() {
        CPUI_INT_ADD if a == v => constant(data, b)?,
        CPUI_INT_ADD if b == v => constant(data, a)?,
        CPUI_INT_SUB if a == v => constant(data, b)?.wrapping_neg(),
        CPUI_PTRADD if a == v => constant(data, b)?.wrapping_mul(constant(data, inc.get_in(2)?)?),
        _ => return None,
    };
    Some(k & mask(width))
}

/// The successor the CBRANCH ending `b` takes when its condition is true, and
/// the other one.
fn taken(data: &Funcdata, b: BlockId) -> Option<(BlockId, BlockId)> {
    let fb = data.bblocks_ref().block(b);
    let tail = op(data, data.bb_op_tail(b)?)?;
    if fb.size_out() != 2 || tail.code() != CPUI_CBRANCH {
        return None;
    }
    let (t, f) = (fb.get_true_out(), fb.get_false_out());
    Some(if tail.is_boolean_flip() { (f, t) } else { (t, f) })
}

/// The counter `cond` compares with zero, and whether `cond` holds when it is
/// zero (`n == 0`) or when it is not (`n != 0`).
fn zero_test(data: &Funcdata, cond: VarnodeId) -> Option<(VarnodeId, bool)> {
    let c = op(data, data.vbank().get(cond)?.get_def()?)?;
    let (a, b) = (c.get_in(0)?, c.get_in(1)?);
    let n = match (constant(data, a), constant(data, b)) {
        (None, Some(0)) => a,
        (Some(0), None) => b,
        _ => return None,
    };
    match c.code() {
        CPUI_INT_EQUAL => Some((n, true)),
        CPUI_INT_NOTEQUAL => Some((n, false)),
        _ => None,
    }
}

fn match_rep(data: &Funcdata, head: BlockId) -> Option<Rep> {
    let hb = data.bblocks_ref().block(head);
    if hb.size_in() != 2 || hb.size_out() != 2 {
        return None;
    }
    let cond = op(data, data.bb_op_tail(head)?)?.get_in(1)?;
    let (counter, when_zero) = zero_test(data, cond)?;
    let (on_true, on_false) = taken(data, head)?;
    let (exit, body) = if when_zero { (on_true, on_false) } else { (on_false, on_true) };
    let bb = data.bblocks_ref().block(body);
    if body == head || body == exit || bb.size_in() != 1 || bb.size_out() != 1 || bb.get_out(0) != head {
        return None;
    }
    let back = hb.get_in_index(body);
    let to_body = hb.get_out_index(body);
    if back < 0 || to_body < 0 {
        return None;
    }
    let (head_ops, body_ops) = (data.bb_ops(head), data.bb_ops(body));
    let at = op(data, *head_ops.first()?)?.get_addr().clone();
    if !head_ops.iter().chain(body_ops.iter()).all(|&o| op(data, o).is_some_and(|p| *p.get_addr() == at)) {
        return None;
    }
    if data.bb_ops(exit).first().and_then(|&o| op(data, o)).is_none_or(|p| *p.get_addr() == at) {
        return None;
    }
    // The head: phis (out, entering value, value from the body), then the branch.
    let mut phis: Vec<(VarnodeId, VarnodeId, VarnodeId)> = Vec::new();
    for &o in &head_ops {
        let p = op(data, o)?;
        match p.code() {
            CPUI_MULTIEQUAL => phis.push((p.get_out()?, p.get_in(1 - back)?, p.get_in(back)?)),
            CPUI_CBRANCH => {}
            _ if p.get_out() == Some(cond) => {}
            _ => return None,
        }
    }
    let defined_in = |v: VarnodeId, b: BlockId| def_block(data, v) == Some(b);
    // The body's steps, by the phi they step.
    let mut count = None;
    let mut ptrs: Vec<(VarnodeId, VarnodeId, u64)> = Vec::new();
    let mut memory: Vec<VarnodeId> = Vec::new();
    let mut indirects: Vec<OpId> = Vec::new();
    for &(out, enter, next) in &phis {
        if defined_in(enter, head) || defined_in(enter, body) || !defined_in(next, body) {
            return None;
        }
        let def = data.vbank().get(copied(data, next))?.get_def()?;
        let inc = op(data, def)?;
        if inc.code() == CPUI_INDIRECT && inc.get_in(0) == Some(out) {
            memory.push(out);
            indirects.push(def);
            continue;
        }
        let k = step(data, inc, out)?;
        if out == counter && k == mask(size(data, out)?) {
            count = Some((out, enter));
        } else {
            ptrs.push((out, enter, k));
        }
    }
    let (count_out, count_in) = count?;
    // The element store, and for a copy the element load.
    let (mut store, mut load) = (None, None);
    for &o in &body_ops {
        let p = op(data, o)?;
        match p.code() {
            CPUI_STORE if store.is_none() => store = Some(o),
            CPUI_LOAD if load.is_none() => load = Some(o),
            CPUI_COPY | CPUI_INT_ADD | CPUI_INT_SUB | CPUI_PTRADD | CPUI_BRANCH => {}
            CPUI_INDIRECT if indirects.contains(&o) => {}
            _ => return None,
        }
    }
    let store = store?;
    let on_store = |o: OpId| {
        op(data, o)
            .and_then(|p| p.get_in(1))
            .and_then(|v| data.vbank().get(v))
            .is_some_and(|v| crate::funcdata_varnode::op_iop_decode(v.get_offset()) == store)
    };
    if !indirects.iter().all(|&o| on_store(o)) {
        return None;
    }
    let st = op(data, store)?;
    let st_ptr = copied(data, st.get_in(1)?);
    let stored = st.get_in(2)?;
    let width = size(data, stored)?;
    let dst = ptrs.iter().find(|(out, _, _)| *out == st_ptr)?;
    if in_frame(data, dst.1) || !full_width(data, dst.0) {
        return None;
    }
    if dst.2 != width as u64 {
        return None;
    }
    let element = match load {
        Some(l) => {
            let ld = op(data, l)?;
            if ptrs.len() != 2 || copied(data, stored) != ld.get_out()? {
                return None;
            }
            let ld_ptr = copied(data, ld.get_in(1)?);
            let src = ptrs.iter().find(|(out, _, _)| *out == ld_ptr)?;
            if src.2 != width as u64 || src.0 == dst.0 || !full_width(data, src.0) {
                return None;
            }
            Element::Copy { src_out: src.0, src_in: src.1 }
        }
        None => {
            if ptrs.len() != 1 || def_block(data, stored).is_some_and(|b| b == head || b == body) {
                return None;
            }
            match constant(data, stored) {
                Some(k) if (0..width).all(|i| (k >> (8 * i)) & 0xff == k & 0xff) => Element::FillConst(k & 0xff),
                None if width == 1 => Element::FillByte(stored),
                _ if matches!(width, 2 | 4 | 8) => Element::FillElement(stored),
                _ => return None,
            }
        }
    };
    // Nothing outside the loop reads what the body computes.
    for &o in &body_ops {
        let Some(out) = op(data, o)?.get_out() else { continue };
        for u in data.vbank().get(out)?.descend_iter() {
            if !op(data, u)?.get_parent().is_some_and(|b| b == head || b == body) {
                return None;
            }
        }
    }
    Some(Rep { head, body, to_body, width, count_out, count_in, dst_out: dst.0, dst_in: dst.1, element, memory })
}

/// Is `v` an address in the function's own frame: the stack pointer plus a
/// constant, through copies?
fn in_frame(data: &Funcdata, mut v: VarnodeId) -> bool {
    for _ in 0..8 {
        let Some(n) = data.vbank().get(v) else { return false };
        if n.is_spacebase() {
            return true;
        }
        let Some(d) = n.get_def().and_then(|d| op(data, d)) else { return false };
        let next = match d.code() {
            CPUI_COPY | CPUI_CAST => d.get_in(0),
            CPUI_INT_ADD | CPUI_PTRSUB | CPUI_PTRADD | CPUI_INT_SUB => d.get_in(0),
            _ => None,
        };
        match next {
            Some(n) => v = n,
            None => return false,
        }
    }
    false
}

/// A new op of `code` over `inputs`, placed before `before`, with a fresh
/// output of `size` bytes when `size` is positive.
fn insert(
    data: &mut Funcdata,
    code: kuna_num::opcodes::OpCode,
    inputs: &[VarnodeId],
    size: i32,
    before: OpId,
) -> KunaResult<(OpId, Option<VarnodeId>)> {
    let at = op(data, before).map(|p| p.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("repstring: stale op"))?;
    let new = data.new_op(inputs.len() as i32, at);
    data.op_set_opcode_code(new, code);
    for (slot, &v) in inputs.iter().enumerate() {
        data.op_set_input(new, v, slot as i32)?;
    }
    let out = if size > 0 { Some(data.new_unique_out(size, new)?) } else { None };
    data.op_insert_before(new, before);
    Ok((new, out))
}

/// Point every read of `from` outside the loop at `to`, or at a new zero of
/// `from`'s width when `to` is `None`.
fn redirect(data: &mut Funcdata, from: VarnodeId, to: Option<VarnodeId>, inside: [BlockId; 2]) -> KunaResult<()> {
    let width = size(data, from).unwrap_or(8);
    let readers: Vec<OpId> = data.vbank().get(from).map(|v| v.descend_iter().collect()).unwrap_or_default();
    for reader in readers {
        let Some(p) = op(data, reader) else { continue };
        if p.get_parent().is_some_and(|b| inside.contains(&b)) {
            continue;
        }
        let slots: Vec<i32> = (0..p.num_input()).filter(|&i| p.get_in(i) == Some(from)).collect();
        for slot in slots {
            let value = match to {
                Some(v) => v,
                None => data.new_constant(width, 0),
            };
            data.op_set_input(reader, value, slot)?;
        }
    }
    Ok(())
}

fn rewrite(data: &mut Funcdata, rep: &Rep) -> KunaResult<()> {
    let tail = data.bb_op_tail(rep.head).ok_or_else(|| KunaError::lowlevel("repstring: empty head"))?;
    let ptr_size = size(data, rep.dst_out).unwrap_or(8);
    let count_size = size(data, rep.count_in).unwrap_or(ptr_size);
    let bytes = if rep.width == 1 {
        rep.count_in
    } else {
        let w = data.new_constant(count_size, rep.width as uintb);
        insert(data, CPUI_INT_MULT, &[rep.count_in, w], count_size, tail)?
            .1
            .ok_or_else(|| KunaError::lowlevel("repstring: no size"))?
    };
    let (id, second, length, src) = match rep.element {
        Element::Copy { src_out, src_in } => (BUILTIN_MEMCPY, src_in, bytes, Some(src_out)),
        Element::FillConst(byte) => (BUILTIN_MEMSET, data.new_constant(4, byte as uintb), bytes, None),
        Element::FillByte(byte) => (BUILTIN_MEMSET, byte, bytes, None),
        Element::FillElement(value) => {
            let id = match rep.width {
                2 => BUILTIN_MEMSET16,
                4 => BUILTIN_MEMSET32,
                _ => BUILTIN_MEMSET64,
            };
            let value = match constant(data, value) {
                Some(k) => data.new_constant(rep.width, k),
                None => value,
            };
            (id, value, rep.count_in, None)
        }
    };
    let id = data.new_constant(4, id as uintb);
    let (call, _) = insert(data, CPUI_CALLOTHER, &[id, rep.dst_in, second, length], 0, tail)?;
    for &location in &rep.memory {
        let (addr, width) = match data.vbank().get(location) {
            Some(v) => (v.get_addr().clone(), v.get_size()),
            None => continue,
        };
        let at = op(data, call).map(|p| p.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("repstring: stale call"))?;
        let ind = data.new_op(2, at);
        data.op_set_opcode_code(ind, CPUI_INDIRECT);
        data.op_set_input(ind, location, 0)?;
        let iop = data.new_varnode_iop(call);
        data.op_set_input(ind, iop, 1)?;
        let after = data.new_varnode_out(width, &addr, ind)?;
        data.op_insert_before(ind, call);
        redirect(data, location, Some(after), [rep.head, rep.body])?;
    }
    let (_, dst_end) = insert(data, CPUI_INT_ADD, &[rep.dst_out, bytes], ptr_size, tail)?;
    redirect(data, rep.dst_out, dst_end, [rep.head, rep.body])?;
    if let Some(src_out) = src {
        let (_, src_end) = insert(data, CPUI_INT_ADD, &[src_out, bytes], ptr_size, tail)?;
        redirect(data, src_out, src_end, [rep.head, rep.body])?;
    }
    redirect(data, rep.count_out, None, [rep.head, rep.body])?;
    data.remove_branch(rep.head, rep.to_body)
}

/// What a value of a compare loop is once the loop is a `builtin_memcmp`
/// returning `r`: a constant, `r < 0` or `0 < r`.
#[derive(Clone, Copy, PartialEq)]
enum After {
    Constant(u64),
    Less,
    Greater,
}

/// One matched compare loop: the blocks, the edge back to the head, the
/// values entering the loop, whether the head's branch condition holds on a
/// zero count, and what each value read after the loop becomes.
struct Compare {
    head: BlockId,
    body: BlockId,
    to_head: i32,
    to_body: i32,
    exit_when_true: bool,
    count_in: VarnodeId,
    first_in: VarnodeId,
    second_in: VarnodeId,
    after: Vec<(VarnodeId, After)>,
    /// When both ways out of the loop reach one block and each of its phis
    /// is a value of the result: the phi, its slot from the head and the value.
    joins: Option<Vec<(OpId, i32, Join)>>,
}

/// A value merged from the two ways out of a compare loop, as a value of the
/// `builtin_memcmp` result `r`: a constant, or `r` compared with a constant
/// (`r` first, or the constant first).
#[derive(Clone, Copy)]
enum Join {
    Constant(u64),
    Result(kuna_num::opcodes::OpCode, u64),
    Reversed(kuna_num::opcodes::OpCode, u64),
}

/// The join of `equal`, a value's constant when the blocks are equal, with
/// what it is when they differ.
fn join(equal: u64, differ: After) -> Option<Join> {
    Some(match (equal, differ) {
        (c, After::Constant(d)) if c == d => Join::Constant(c),
        (1, After::Constant(0)) => Join::Result(CPUI_INT_EQUAL, 0),
        (0, After::Constant(1)) => Join::Result(CPUI_INT_NOTEQUAL, 0),
        (0, After::Less) => Join::Result(CPUI_INT_SLESS, 0),
        (1, After::Less) => Join::Result(CPUI_INT_SLESS, 1),
        (0, After::Greater) => Join::Reversed(CPUI_INT_SLESS, 0),
        (1, After::Greater) => Join::Reversed(CPUI_INT_SLESSEQUAL, 0),
        _ => return None,
    })
}

/// Is `v` read by an op outside `inside`?
fn read_outside(data: &Funcdata, v: VarnodeId, inside: [BlockId; 2]) -> bool {
    data.vbank().get(v).is_some_and(|n| {
        n.descend_iter().any(|u| !op(data, u).and_then(|p| p.get_parent()).is_some_and(|b| inside.contains(&b)))
    })
}

/// Is every read of the head's phi `v` outside the loop one that sees it
/// only when the head leaves to `exit`: a phi of `exit` on the edge from the
/// head, or an op of `exit` when nothing else enters it?
fn read_leaving_head(data: &Funcdata, v: VarnodeId, inside: [BlockId; 2], exit: BlockId) -> bool {
    let eb = data.bblocks_ref().block(exit);
    let from_head = eb.get_in_index(inside[0]);
    data.vbank().get(v).is_some_and(|n| {
        n.descend_iter().all(|u| {
            let Some(p) = op(data, u) else { return false };
            match p.get_parent() {
                Some(b) if inside.contains(&b) => true,
                Some(b) if b == exit && p.code() == CPUI_MULTIEQUAL => {
                    (0..p.num_input()).all(|i| p.get_in(i) != Some(v) || i == from_head)
                }
                Some(b) if b == exit => eb.size_in() == 1,
                _ => false,
            }
        })
    })
}

/// `v` as a function of the byte pair `(x, y)` the body loaded, over the
/// pairs that differ: one constant, `x < y` or `x > y`.
fn after_difference(data: &Funcdata, v: VarnodeId, x: VarnodeId, y: VarnodeId, body: BlockId) -> Option<After> {
    let at = |a: u64, b: u64| eval(data, v, &[(x, a), (y, b)], &[body], 0);
    let first = at(0, 1)?;
    let candidates = [After::Constant(first), After::Less, After::Greater];
    let mut alive = [true, size(data, v)? == 1, size(data, v)? == 1];
    for a in 0..=255u64 {
        for b in 0..=255u64 {
            if a == b {
                continue;
            }
            let got = at(a, b)?;
            for (k, candidate) in candidates.iter().enumerate() {
                let want = match candidate {
                    After::Constant(c) => *c,
                    After::Less => (a < b) as u64,
                    After::Greater => (a > b) as u64,
                };
                alive[k] &= got == want;
            }
            if !alive.contains(&true) {
                return None;
            }
        }
    }
    candidates.into_iter().zip(alive).find(|&(_, ok)| ok).map(|(c, _)| c)
}

/// `v`'s value for every equal byte pair, when it is the same for all of them.
fn after_equal(data: &Funcdata, v: VarnodeId, x: VarnodeId, y: VarnodeId, body: BlockId) -> Option<u64> {
    let first = eval(data, v, &[(x, 0), (y, 0)], &[body], 0)?;
    (1..=255u64).all(|a| eval(data, v, &[(x, a), (y, a)], &[body], 0) == Some(first)).then_some(first)
}

/// `v` through COPYs, when that ends in a constant.
fn copied_constant(data: &Funcdata, v: VarnodeId) -> Option<u64> {
    constant(data, copied(data, v))
}

fn match_compare(data: &Funcdata, head: BlockId) -> Option<Compare> {
    let hb = data.bblocks_ref().block(head);
    if hb.size_in() != 2 || hb.size_out() != 2 {
        return None;
    }
    let tail = op(data, data.bb_op_tail(head)?)?;
    let cond = tail.get_in(1)?;
    let (counter, when_zero) = zero_test(data, cond)?;
    let (on_true, on_false) = taken(data, head)?;
    let (exit, body) = if when_zero { (on_true, on_false) } else { (on_false, on_true) };
    let bb = data.bblocks_ref().block(body);
    if body == head || body == exit || bb.size_in() != 1 || bb.size_out() != 2 {
        return None;
    }
    let (back, to_head) = (hb.get_in_index(body), bb.get_out_index(head));
    if back < 0 || to_head < 0 {
        return None;
    }
    let (head_ops, body_ops) = (data.bb_ops(head), data.bb_ops(body));
    let at = op(data, *head_ops.first()?)?.get_addr().clone();
    if !head_ops.iter().chain(body_ops.iter()).all(|&o| op(data, o).is_some_and(|p| *p.get_addr() == at)) {
        return None;
    }
    let inside = [head, body];
    let mut phis: Vec<(VarnodeId, VarnodeId, VarnodeId)> = Vec::new();
    for &o in &head_ops {
        let p = op(data, o)?;
        match p.code() {
            CPUI_MULTIEQUAL => {
                let (enter, next) = (p.get_in(1 - back)?, p.get_in(back)?);
                if def_block(data, enter).is_some_and(|b| inside.contains(&b)) || def_block(data, next) != Some(body) {
                    return None;
                }
                phis.push((p.get_out()?, enter, next));
            }
            CPUI_CBRANCH => {}
            _ if p.get_out() == Some(cond) => {}
            _ => return None,
        }
    }
    // The body: two byte loads, pure values, and the branch.
    let mut loads: Vec<(VarnodeId, VarnodeId)> = Vec::new();
    for &o in &body_ops {
        let p = op(data, o)?;
        match p.code() {
            CPUI_CBRANCH => {}
            CPUI_LOAD => {
                let out = p.get_out()?;
                if size(data, out)? != 1 {
                    return None;
                }
                loads.push((copied(data, p.get_in(1)?), out));
            }
            code if PURE.contains(&code) => {
                if data.vbank().get(p.get_out()?).is_some_and(|n| n.is_persist()) {
                    return None;
                }
            }
            _ => return None,
        }
    }
    let [(p0, b0), (p1, b1)] = loads[..] else { return None };
    if p0 == p1 || !full_width(data, p0) || !full_width(data, p1) {
        return None;
    }
    // The count and the two pointers; every other phi is a flag.
    let (mut count_in, mut ptr_in, mut flags) = (None, [None, None], Vec::new());
    for &(out, enter, next) in &phis {
        let step = op(data, data.vbank().get(copied(data, next))?.get_def()?).and_then(|inc| step(data, inc, out));
        if out == counter {
            if step != Some(mask(size(data, out)?)) || read_outside(data, out, inside) {
                return None;
            }
            count_in = Some(enter);
        } else if out == p0 || out == p1 {
            if step != Some(1) || read_outside(data, out, inside) {
                return None;
            }
            ptr_in[(out == p1) as usize] = Some(enter);
        } else {
            flags.push((out, enter, next));
        }
    }
    let (count_in, in0, in1) = (count_in?, ptr_in[0]?, ptr_in[1]?);
    // The first operand is the one a subtraction or ordered compare of the two
    // bytes reads first, else the one an equality does.
    let reads_both = |p: &PcodeOp| {
        let (a, b) = (p.get_in(0), p.get_in(1));
        (a == Some(b0) && b == Some(b1)) || (a == Some(b1) && b == Some(b0))
    };
    let ordered = |p: &PcodeOp| matches!(p.code(), CPUI_INT_SUB | CPUI_INT_LESS | CPUI_INT_SBORROW);
    let users: Vec<&PcodeOp> = body_ops.iter().filter_map(|&o| op(data, o)).filter(|p| reads_both(p)).collect();
    let lead = users.iter().find(|p| ordered(p)).or(users.first())?;
    let swap = lead.get_in(0) == Some(b1);
    let ((x, first_in), (y, second_in)) = if swap { ((b1, in1), (b0, in0)) } else { ((b0, in0), (b1, in1)) };
    // The body returns to the head exactly while the bytes are equal.
    let (b_true, b_false) = taken(data, body)?;
    let body_cond = op(data, data.bb_op_tail(body)?)?.get_in(1)?;
    for a in 0..=255u64 {
        for b in 0..=255u64 {
            let c = eval(data, body_cond, &[(x, a), (y, b)], &[body], 0)?;
            let to = if c != 0 { b_true } else { b_false };
            if (to == head) != (a == b) {
                return None;
            }
        }
    }
    // What is read after the loop.
    let nonzero_count = copied_constant(data, count_in).is_some_and(|n| n != 0);
    let mut after = Vec::new();
    for &(out, enter, next) in &flags {
        if !read_outside(data, out, inside) {
            continue;
        }
        let equal = after_equal(data, next, x, y, body)?;
        if copied_constant(data, enter) != Some(equal) && !(nonzero_count && read_leaving_head(data, out, inside, exit)) {
            return None;
        }
        after.push((out, After::Constant(equal)));
    }
    for &o in &body_ops {
        let Some(out) = op(data, o)?.get_out() else { continue };
        if read_outside(data, out, inside) {
            after.push((out, after_difference(data, out, x, y, body)?));
        }
    }
    let other = bb.get_out(1 - to_head);
    let known = |v: VarnodeId| match constant(data, v) {
        Some(k) => Some(After::Constant(k)),
        None => after.iter().find(|(value, _)| *value == v).map(|&(_, a)| a),
    };
    let joins = (other == exit).then(|| {
        let eb = data.bblocks_ref().block(exit);
        let (from_head, from_body) = (eb.get_in_index(head), eb.get_in_index(body));
        let mut joins = Vec::new();
        for o in data.bb_ops(exit) {
            let p = op(data, o)?;
            if p.code() != CPUI_MULTIEQUAL {
                continue;
            }
            let (h, b) = (p.get_in(from_head)?, p.get_in(from_body)?);
            if h == b {
                continue;
            }
            let After::Constant(equal) = known(h)? else { return None };
            joins.push((o, from_head, join(equal, known(b)?)?));
        }
        Some(joins)
    });
    let to_body = hb.get_out_index(body);
    Some(Compare {
        head,
        body,
        to_head,
        to_body,
        exit_when_true: when_zero,
        count_in,
        first_in,
        second_in,
        after,
        joins: joins.flatten(),
    })
}

fn rewrite_compare(data: &mut Funcdata, c: &Compare) -> KunaResult<()> {
    let tail = data.bb_op_tail(c.head).ok_or_else(|| KunaError::lowlevel("repstring: empty head"))?;
    let id = data.new_constant(4, BUILTIN_MEMCMP as uintb);
    let (_, result) = insert(data, CPUI_CALLOTHER, &[id, c.first_in, c.second_in, c.count_in], 4, tail)?;
    let result = result.ok_or_else(|| KunaError::lowlevel("repstring: no result"))?;
    let int = data.get_arch().types().and_then(|t| t.get_base(4, type_metatype::TYPE_INT).ok());
    if let (Some(t), Some(n)) = (int, data.vbank_mut().get_mut(result)) {
        n.update_type(t);
    }
    let inside = [c.head, c.body];
    let (mut less, mut greater) = (None, None);
    for &(value, after) in &c.after {
        let to = match after {
            After::Constant(k) => {
                let width = size(data, value).unwrap_or(1);
                Some(data.new_constant(width, k))
            }
            After::Less => {
                if less.is_none() {
                    let zero = data.new_constant(4, 0);
                    less = insert(data, CPUI_INT_SLESS, &[result, zero], 1, tail)?.1;
                }
                less
            }
            After::Greater => {
                if greater.is_none() {
                    let zero = data.new_constant(4, 0);
                    greater = insert(data, CPUI_INT_SLESS, &[zero, result], 1, tail)?.1;
                }
                greater
            }
        };
        redirect(data, value, to, inside)?;
    }
    if let Some(joins) = &c.joins {
        for &(phi, slot, value) in joins {
            let width = op(data, phi).and_then(|p| p.get_out()).and_then(|v| size(data, v)).unwrap_or(1);
            let merged = match value {
                Join::Constant(k) => Some(data.new_constant(width, k)),
                Join::Result(code, k) => {
                    let k = data.new_constant(4, k);
                    insert(data, code, &[result, k], width, tail)?.1
                }
                Join::Reversed(code, k) => {
                    let k = data.new_constant(4, k);
                    insert(data, code, &[k, result], width, tail)?.1
                }
            };
            data.op_set_input(phi, merged.ok_or_else(|| KunaError::lowlevel("repstring: no join"))?, slot)?;
        }
        return data.remove_branch(c.head, c.to_body);
    }
    let zero = data.new_constant(4, 0);
    let code = if c.exit_when_true { CPUI_INT_EQUAL } else { CPUI_INT_NOTEQUAL };
    let (_, same) = insert(data, code, &[result, zero], 1, tail)?;
    data.op_set_input(tail, same.ok_or_else(|| KunaError::lowlevel("repstring: no test"))?, 1)?;
    data.remove_branch(c.body, c.to_head)
}

/// The builtin a CALLOTHER names, when it is one this pass places.
fn builtin(data: &Funcdata, op_id: OpId) -> Option<u32> {
    let id = constant(data, op(data, op_id)?.get_in(0)?)? as u32;
    [BUILTIN_MEMCPY, BUILTIN_MEMSET, BUILTIN_MEMSET16, BUILTIN_MEMSET32, BUILTIN_MEMSET64, BUILTIN_MEMCMP]
        .contains(&id)
        .then_some(id)
}

/// The type of a pointer operand of a copy, fill or compare builtin: `void *`,
/// a pointer to the element of a `builtin_memset16`/`32`/`64`, or `char *` for
/// a compare, whose constant operand is then printed as the string it is.
pub(crate) fn input_type(data: &Funcdata, op_id: OpId, slot: int4) -> Option<Rc<Datatype>> {
    let id = builtin(data, op_id)?;
    let both = id == BUILTIN_MEMCPY || id == BUILTIN_MEMCMP;
    if slot != 1 && !(slot == 2 && both) {
        return None;
    }
    let arch = data.get_arch();
    let types = arch.types()?;
    let element = match id {
        BUILTIN_MEMSET16 => types.get_base(2, type_metatype::TYPE_UINT).ok()?,
        BUILTIN_MEMSET32 => types.get_base(4, type_metatype::TYPE_UINT).ok()?,
        BUILTIN_MEMSET64 => types.get_base(8, type_metatype::TYPE_UINT).ok()?,
        BUILTIN_MEMCMP => types.get_type_char(1).ok()?,
        _ => types.get_type_void().ok()?,
    };
    let word = arch.manage().get_default_data_space().map(|s| s.get_word_size()).unwrap_or(1);
    types.get_type_pointer(size(data, op(data, op_id)?.get_in(slot)?)?, element, word).ok()
}

/// The type of `builtin_memcmp`'s result: a signed integer.
pub(crate) fn output_type(data: &Funcdata, op_id: OpId) -> Option<Rc<Datatype>> {
    if builtin(data, op_id)? != BUILTIN_MEMCMP {
        return None;
    }
    let width = size(data, op(data, op_id)?.get_out()?)?;
    data.get_arch().types()?.get_base(width, type_metatype::TYPE_INT).ok()
}
