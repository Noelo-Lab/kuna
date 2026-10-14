//! (kuna `strlenscan`) An inline `repnz scasb` string length becomes `strlen`.
//!
//! gcc up to version 9 (at -O1 and above) and older MSVC expand `strlen(s)`
//! inline as
//!
//! ```text
//!     or   rcx, -1          ; count: as large as it gets
//!     xor  eax, eax         ; AL: the byte to look for, the terminator
//!     repnz scasb           ; step RDI and RCX until [RDI] == AL
//!     not  rcx              ; ~RCX = strlen(s) + 1
//! ```
//!
//! SLEIGH lifts the repeat prefix as a loop inside the one instruction, so the
//! C shows a `do { if (!n) break; n -= 1; ... } while (c);` over the string and
//! `~n - 1` arithmetic wherever the length is used.
//!
//! # What is proved
//!
//! On SSA, after the main loop has settled, the pass looks for the two blocks
//! the instruction lifts to: a head `H` and a body `B` whose ops all sit at one
//! instruction address, where `H` is entered from one block outside and from
//! `B`, `B` is entered only from `H`, and both leave to the same exit `E`.  It
//! declines unless all of these hold:
//!
//! * `H` holds exactly two phis besides its branch: the count, which enters the
//!   loop as the all-ones constant of its width, and the pointer;
//! * `H` leaves to `E` exactly when the count is zero, and otherwise to `B`;
//! * `B` steps the count down by one and the pointer up by one (so the direction
//!   flag is clear), reads one byte through the pointer, and computes nothing
//!   but values derived from that byte, the count, the pointer and constants;
//! * `B` returns to `H` exactly when the byte it read is not zero, checked for
//!   all 256 byte values, which also proves the byte searched for is zero;
//! * nothing reads the count after the loop except through a bitwise NOT, the
//!   first step of every length computation built on it.
//!
//! The count starts at all ones and loses one per byte, so it reaches zero only
//! after the whole address space has been read: the exit on a zero count is never
//! taken, and the loop ends after `strlen(s) + 1` bytes with the terminator read.
//!
//! # The rewrite
//!
//! `len = strlen(s)` (a typed builtin user-op) is placed at the top of `B`, where
//! `s` is the pointer entering the loop, and every value `B` hands to `E` is given
//! its final-iteration value: the count `~(len + 1)`, which the NOT after the loop
//! folds back to `len + 1`; the pointer `s + len + 1`; the byte read, zero, from
//! which the flags follow.  The count-is-zero edge and the back edge are then
//! removed, so `H` and `B` run once and the next pass of the main loop folds the
//! rest.

use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode::{self, *};

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;
use crate::userop::BUILTIN_STRLEN;

/// The ops the body may compute besides the LOAD and its branch.
const PURE: &[OpCode] = &[
    CPUI_COPY,
    CPUI_INT_ADD,
    CPUI_INT_SUB,
    CPUI_INT_MULT,
    CPUI_INT_AND,
    CPUI_INT_OR,
    CPUI_INT_XOR,
    CPUI_INT_LEFT,
    CPUI_INT_RIGHT,
    CPUI_INT_SRIGHT,
    CPUI_INT_ZEXT,
    CPUI_INT_SEXT,
    CPUI_INT_NEGATE,
    CPUI_INT_2COMP,
    CPUI_SUBPIECE,
    CPUI_PIECE,
    CPUI_INT_EQUAL,
    CPUI_INT_NOTEQUAL,
    CPUI_INT_LESS,
    CPUI_INT_LESSEQUAL,
    CPUI_INT_SLESS,
    CPUI_INT_SLESSEQUAL,
    CPUI_INT_CARRY,
    CPUI_INT_SCARRY,
    CPUI_INT_SBORROW,
    CPUI_BOOL_NEGATE,
    CPUI_BOOL_AND,
    CPUI_BOOL_OR,
    CPUI_BOOL_XOR,
    CPUI_POPCOUNT,
    CPUI_PTRADD,
];

/// (kuna) `ActionStrlenScan` — collapse an inline `repnz scasb` string length
/// into `strlen` (option `strlenscan`).
pub struct ActionStrlenScan {
    base: ActionBase,
    /// Unit-test override of the `strlen_scan` gate; the scheduled instance
    /// passes `false` and reads the live flag.
    enabled: bool,
}

impl ActionStrlenScan {
    pub fn new(enabled: bool, g: impl Into<String>) -> Self {
        Self { base: ActionBase::new(0, "strlenscan", g), enabled }
    }
}

impl Action for ActionStrlenScan {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(Self { base: self.base.clone(), enabled: self.enabled }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !self.enabled && !data.get_arch().strlen_scan {
            return 0;
        }
        let heads: Vec<BlockId> = (0..data.bblocks_get_size())
            .map(|i| data.bblocks_get_block(i))
            .filter(|&h| match_scan(data, h).is_some())
            .collect();
        let mut changed = 0;
        for head in heads {
            let Some(scan) = match_scan(data, head) else { continue };
            if rewrite(data, &scan).is_err() {
                break;
            }
            changed += 1;
        }
        self.base.count += changed;
        (changed != 0) as ApplyResult
    }
}

/// One matched scan: the blocks, the loop-carried values and the body's ops.
struct Scan {
    head: BlockId,
    body: BlockId,
    exit: BlockId,
    width: i32,
    start: VarnodeId,
    count_dec: OpId,
    ptr_inc: OpId,
    load: OpId,
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

/// `v`'s value when it is a constant or a chain of COPYs of one.
fn copied_constant(data: &Funcdata, mut v: VarnodeId) -> Option<u64> {
    for _ in 0..8 {
        if let Some(k) = constant(data, v) {
            return Some(k);
        }
        let d = op(data, data.vbank().get(v)?.get_def()?)?;
        if d.code() != CPUI_COPY {
            return None;
        }
        v = d.get_in(0)?;
    }
    None
}

fn mask(width: i32) -> u64 {
    if width >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * width)) - 1
    }
}

fn def_block(data: &Funcdata, v: VarnodeId) -> Option<BlockId> {
    let d = data.vbank().get(v)?.get_def()?;
    op(data, d)?.get_parent()
}

/// The successors a CBRANCH block takes when its condition is true and false.
fn taken(data: &Funcdata, b: BlockId) -> Option<(BlockId, BlockId)> {
    let fb = data.bblocks_ref().block(b);
    if fb.size_out() != 2 {
        return None;
    }
    let tail = op(data, data.bb_op_tail(b)?)?;
    if tail.code() != CPUI_CBRANCH {
        return None;
    }
    let (t, f) = (fb.get_true_out(), fb.get_false_out());
    Some(if tail.is_boolean_flip() { (f, t) } else { (t, f) })
}

/// The value of `v` with each `(leaf, value)` of `env` substituted, following
/// only ops defined in `region`; `None` when `v` reads anything else.
fn eval(data: &Funcdata, v: VarnodeId, env: &[(VarnodeId, u64)], region: &[BlockId], depth: u32) -> Option<u64> {
    if let Some(&(_, x)) = env.iter().find(|(leaf, _)| *leaf == v) {
        return Some(x);
    }
    let width = size(data, v)?;
    if width > 8 || depth > 32 {
        return None;
    }
    if let Some(k) = constant(data, v) {
        return Some(k & mask(width));
    }
    let d = data.vbank().get(v)?.get_def()?;
    let o = op(data, d)?;
    if !region.contains(&o.get_parent()?) {
        return None;
    }
    let arg = |i: i32| -> Option<(u64, i32)> {
        let a = o.get_in(i)?;
        Some((eval(data, a, env, region, depth + 1)?, size(data, a)?))
    };
    let sext = |x: u64, w: i32| -> i64 {
        let s = 64 - 8 * w.min(8) as u32;
        ((x << s) as i64) >> s
    };
    let m = mask(width);
    let r = match o.code() {
        CPUI_COPY => arg(0)?.0,
        CPUI_INT_ZEXT => arg(0)?.0,
        CPUI_INT_SEXT => {
            let (x, w) = arg(0)?;
            sext(x, w) as u64
        }
        CPUI_INT_NEGATE => !arg(0)?.0,
        CPUI_INT_2COMP => arg(0)?.0.wrapping_neg(),
        CPUI_BOOL_NEGATE => (arg(0)?.0 == 0) as u64,
        CPUI_POPCOUNT => arg(0)?.0.count_ones() as u64,
        CPUI_SUBPIECE => {
            let (x, _) = arg(0)?;
            let k = constant(data, o.get_in(1)?)?;
            if k >= 8 {
                0
            } else {
                x >> (8 * k)
            }
        }
        code => {
            let ((a, wa), (b, _)) = (arg(0)?, arg(1)?);
            match code {
                CPUI_INT_ADD => a.wrapping_add(b),
                CPUI_INT_SUB => a.wrapping_sub(b),
                CPUI_INT_MULT => a.wrapping_mul(b),
                CPUI_INT_AND | CPUI_BOOL_AND => a & b,
                CPUI_INT_OR | CPUI_BOOL_OR => a | b,
                CPUI_INT_XOR | CPUI_BOOL_XOR => a ^ b,
                CPUI_INT_LEFT => a.checked_shl(b as u32).unwrap_or(0),
                CPUI_INT_RIGHT => a.checked_shr(b as u32).unwrap_or(0),
                CPUI_INT_SRIGHT => (sext(a, wa) >> b.min(63)) as u64,
                CPUI_INT_EQUAL => (a == b) as u64,
                CPUI_INT_NOTEQUAL => (a != b) as u64,
                CPUI_INT_LESS => (a < b) as u64,
                CPUI_INT_LESSEQUAL => (a <= b) as u64,
                CPUI_INT_SLESS => (sext(a, wa) < sext(b, wa)) as u64,
                CPUI_INT_SLESSEQUAL => (sext(a, wa) <= sext(b, wa)) as u64,
                CPUI_INT_CARRY => (a.wrapping_add(b) & mask(wa) < a) as u64,
                CPUI_INT_SCARRY => {
                    let s = sext(a, wa).wrapping_add(sext(b, wa));
                    (sext(s as u64 & mask(wa), wa) != s) as u64
                }
                CPUI_INT_SBORROW => {
                    let s = sext(a, wa).wrapping_sub(sext(b, wa));
                    (sext(s as u64 & mask(wa), wa) != s) as u64
                }
                CPUI_PIECE => {
                    let lo = size(data, o.get_in(1)?)?;
                    if lo >= 8 {
                        return None;
                    }
                    (a << (8 * lo)) | b
                }
                _ => return None,
            }
        }
    };
    Some(r & m)
}

/// `inc` is `v + step` (INT_ADD with either operand order, or a byte PTRADD).
fn steps(data: &Funcdata, inc: &PcodeOp, v: VarnodeId, step: u64, width: i32) -> bool {
    let step = step & mask(width);
    match inc.code() {
        CPUI_INT_ADD => {
            let (Some(a), Some(b)) = (inc.get_in(0), inc.get_in(1)) else { return false };
            (a == v && constant(data, b) == Some(step)) || (b == v && constant(data, a) == Some(step))
        }
        CPUI_INT_SUB => {
            let (Some(a), Some(b)) = (inc.get_in(0), inc.get_in(1)) else { return false };
            a == v && constant(data, b) == Some(step.wrapping_neg() & mask(width))
        }
        CPUI_PTRADD => {
            let (Some(a), Some(i), Some(e)) = (inc.get_in(0), inc.get_in(1), inc.get_in(2)) else { return false };
            a == v && constant(data, e) == Some(1) && constant(data, i) == Some(step)
        }
        _ => false,
    }
}

fn match_scan(data: &Funcdata, head: BlockId) -> Option<Scan> {
    let hb = data.bblocks_ref().block(head);
    if hb.size_in() != 2 || hb.size_out() != 2 {
        return None;
    }
    let (h_true, h_false) = taken(data, head)?;
    let body = [h_true, h_false].into_iter().find(|&b| {
        let fb = data.bblocks_ref().block(b);
        b != head && fb.size_in() == 1 && fb.size_out() == 2 && fb.get_out_index(head) >= 0
    })?;
    let exit = if h_true == body { h_false } else { h_true };
    let bb = data.bblocks_ref().block(body);
    if bb.get_out_index(exit) < 0 || exit == head || exit == body {
        return None;
    }
    let back = hb.get_in_index(body);
    let (head_ops, body_ops) = (data.bb_ops(head), data.bb_ops(body));
    let at = op(data, *head_ops.first()?)?.get_addr().clone();
    if !head_ops.iter().chain(body_ops.iter()).all(|&o| op(data, o).is_some_and(|p| *p.get_addr() == at)) {
        return None;
    }
    if data.bb_ops(exit).first().and_then(|&o| op(data, o)).is_none_or(|p| *p.get_addr() == at) {
        return None;
    }
    let phis: Vec<OpId> = head_ops.iter().copied().filter(|&o| op(data, o).is_some_and(|p| p.code() == CPUI_MULTIEQUAL)).collect();
    if phis.len() != 2 {
        return None;
    }
    let outside = 1 - back;
    let phi_val = |o: OpId| -> Option<(VarnodeId, VarnodeId, VarnodeId)> {
        let p = op(data, o)?;
        Some((p.get_out()?, p.get_in(outside)?, p.get_in(back)?))
    };
    let (a, b) = (phi_val(phis[0])?, phi_val(phis[1])?);
    let width_of = |t: &(VarnodeId, VarnodeId, VarnodeId)| size(data, t.0).unwrap_or(0);
    let is_ones = |t: &(VarnodeId, VarnodeId, VarnodeId)| copied_constant(data, t.1) == Some(mask(width_of(t)));
    let ((count, _, count_next), (ptr, start, ptr_next)) = match (is_ones(&a), is_ones(&b)) {
        (true, false) => (a, b),
        (false, true) => (b, a),
        _ => return None,
    };
    let width = size(data, count)?;
    if !(width == 4 || width == 8) || size(data, ptr)? != width {
        return None;
    }
    if def_block(data, start).is_some_and(|d| d == head || d == body) {
        return None;
    }
    // The head: its phis, then a pure test of the count alone.
    let head_cond = op(data, data.bb_op_tail(head)?)?.get_in(1)?;
    for &o in &head_ops {
        let p = op(data, o)?;
        if p.code() != CPUI_MULTIEQUAL && p.code() != CPUI_CBRANCH && !PURE.contains(&p.code()) {
            return None;
        }
    }
    let ones = mask(width);
    for x in [0, 1, 2, 0x7f, 0x80, 0xff, 0x100, ones, ones - 1, ones >> 1, (ones >> 1) + 1] {
        let c = eval(data, head_cond, &[(count, x)], &[head], 0)?;
        let to = if c != 0 { h_true } else { h_false };
        if (to == exit) != (x == 0) {
            return None;
        }
    }
    // The body: one byte read through the pointer, the two steps, pure rest.
    let (mut load, mut count_dec, mut ptr_inc) = (None, None, None);
    for &o in &body_ops {
        let p = op(data, o)?;
        match p.code() {
            CPUI_CBRANCH => {}
            CPUI_LOAD => {
                if load.is_some() || p.get_in(1)? != ptr || size(data, p.get_out()?)? != 1 {
                    return None;
                }
                load = Some(o);
            }
            code if PURE.contains(&code) => {
                let out = p.get_out()?;
                if data.vbank().get(out).is_some_and(|n| n.is_persist()) {
                    return None;
                }
                if out == count_next {
                    if !steps(data, p, count, ones, width) {
                        return None;
                    }
                    count_dec = Some(o);
                } else if out == ptr_next {
                    if !steps(data, p, ptr, 1, width) {
                        return None;
                    }
                    ptr_inc = Some(o);
                }
            }
            _ => return None,
        }
    }
    let (load, count_dec, ptr_inc) = (load?, count_dec?, ptr_inc?);
    let byte = op(data, load)?.get_out()?;
    let (b_true, b_false) = taken(data, body)?;
    let body_cond = op(data, data.bb_op_tail(body)?)?.get_in(1)?;
    for x in 0..=255u64 {
        let c = eval(data, body_cond, &[(byte, x)], &[body], 0)?;
        let to = if c != 0 { b_true } else { b_false };
        if (to == head) != (x != 0) {
            return None;
        }
    }
    // Every op in the loop reads only loop values, its own results, constants
    // and values defined before the loop.
    for &o in head_ops.iter().chain(body_ops.iter()) {
        let p = op(data, o)?;
        if p.code() == CPUI_MULTIEQUAL {
            continue;
        }
        let first = if p.code() == CPUI_CBRANCH { 1 } else { 0 };
        for i in first..p.num_input() {
            let v = p.get_in(i)?;
            if v == count || v == ptr || data.vbank().get(v).is_some_and(|n| n.is_constant()) {
                continue;
            }
            if !def_block(data, v).is_some_and(|d| d == body || d == head) {
                return None;
            }
        }
    }
    // Nothing outside the loop reads a loop value except through the exit's
    // phis, and the count only ever through a NOT.
    let exit_phis: Vec<OpId> = data
        .bb_ops(exit)
        .into_iter()
        .filter(|&o| op(data, o).is_some_and(|p| p.code() == CPUI_MULTIEQUAL))
        .collect();
    for &o in head_ops.iter().chain(body_ops.iter()) {
        let Some(out) = op(data, o)?.get_out() else { continue };
        for u in data.vbank().get(out)?.descend_iter() {
            let up = op(data, u)?;
            let blk = up.get_parent()?;
            if blk == head || blk == body {
                continue;
            }
            if !exit_phis.contains(&u) {
                return None;
            }
            if out == count || out == count_next {
                let merged = up.get_out()?;
                if data.vbank().get(merged)?.descend_iter().any(|r| op(data, r).is_none_or(|q| q.code() != CPUI_INT_NEGATE)) {
                    return None;
                }
                for k in 0..up.num_input() {
                    let v = up.get_in(k)?;
                    if v != count && v != count_next {
                        return None;
                    }
                }
            }
        }
    }
    Some(Scan { head, body, exit, width, start, count_dec, ptr_inc, load })
}

fn rewrite(data: &mut Funcdata, s: &Scan) -> kuna_base::error::KunaResult<()> {
    use kuna_base::error::KunaError;
    let at = op(data, s.count_dec).map(|p| p.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("strlenscan: stale op"))?;
    let first = *data.bb_ops(s.body).first().ok_or_else(|| KunaError::lowlevel("strlenscan: empty body"))?;
    let to_exit = data.bblocks_ref().block(s.head).get_out_index(s.exit);
    let to_head = data.bblocks_ref().block(s.body).get_out_index(s.head);
    if to_exit < 0 || to_head < 0 {
        return Err(KunaError::lowlevel("strlenscan: missing loop edge"));
    }

    let call = data.new_op(2, at.clone());
    data.op_set_opcode_code(call, CPUI_CALLOTHER);
    let id = data.new_constant(4, BUILTIN_STRLEN as uintb);
    data.op_set_input(call, id, 0)?;
    data.op_set_input(call, s.start, 1)?;
    let len = data.new_unique_out(s.width, call)?;
    let uint = data.get_arch().types().and_then(|t| t.get_base(s.width, crate::dtype::type_metatype::TYPE_UINT).ok());
    if let (Some(t), Some(n)) = (uint, data.vbank_mut().get_mut(len)) {
        n.update_type(t);
    }
    data.op_insert_before(call, first);

    let plus = data.new_op(2, at);
    data.op_set_opcode_code(plus, CPUI_INT_ADD);
    data.op_set_input(plus, len, 0)?;
    let one = data.new_constant(s.width, 1);
    data.op_set_input(plus, one, 1)?;
    let past = data.new_unique_out(s.width, plus)?;
    data.op_insert_after(plus, call);

    while op(data, s.count_dec).map_or(0, |p| p.num_input()) > 1 {
        let last = op(data, s.count_dec).map_or(0, |p| p.num_input()) - 1;
        data.op_remove_input(s.count_dec, last);
    }
    data.op_set_opcode_code(s.count_dec, CPUI_INT_NEGATE);
    data.op_set_input(s.count_dec, past, 0)?;

    while op(data, s.ptr_inc).map_or(0, |p| p.num_input()) > 2 {
        let last = op(data, s.ptr_inc).map_or(0, |p| p.num_input()) - 1;
        data.op_remove_input(s.ptr_inc, last);
    }
    data.op_set_opcode_code(s.ptr_inc, CPUI_INT_ADD);
    data.op_set_input(s.ptr_inc, s.start, 0)?;
    data.op_set_input(s.ptr_inc, past, 1)?;

    data.op_remove_input(s.load, 1);
    data.op_set_opcode_code(s.load, CPUI_COPY);
    let zero = data.new_constant(1, 0);
    data.op_set_input(s.load, zero, 0)?;

    data.remove_branch(s.head, to_exit)?;
    data.remove_branch(s.body, to_head)?;
    Ok(())
}

#[cfg(test)]
#[path = "kuna_strlenscan/tests.rs"]
mod tests;
