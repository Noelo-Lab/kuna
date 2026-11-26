//! (kuna `rangetest`) Print a merged unsigned range test as the two
//! comparisons it stands for.
//!
//! An optimizing compiler folds `A <= x && x <= C` into one unsigned
//! comparison of `x - A` against `C - A`, and `x < A || C < x` into its
//! negation; the printer shows that merged form, `(unsigned int)(x - A) <= B`.
//! `split` prints `A <= x && x <= A + B` (or `x < A || A + B < x`) wherever the
//! two comparisons give the merged test's truth value for every `x` in the
//! types the C prints; `merged` keeps the one comparison; `auto` splits except
//! in a loop's condition and in a stand-alone condition whose range holds at
//! most 16 values, where the merged form is the source's own more often (a
//! `switch` case list sharing one body, an `ISDIGIT`-style macro).
//!
//! Exactness: the merged test holds when the `n`-bit value of `x` lies in
//! `[A, A + B]`, which is one interval only when `A + B < 2^n`. Two C
//! comparisons against `A` and `A + B` read `x`'s printed value: equal to the
//! `n`-bit unsigned value when the printed type is unsigned and `n` bits wide,
//! and equal to it on the whole interval, with every other value falling
//! outside, when `A + B < 2^(n-1)` whatever the signedness. A same-size cast
//! around `x` is looked through (its spelling may be elided where `x` prints),
//! as is a zero extension when `A + B` fits the narrower value and a sign
//! extension when it fits its non-negative half. `x` prints twice, so it must
//! be a variable or a pure expression the printer already prints in place; the
//! bound check of a `switch` keeps its form.

use std::collections::HashSet;

use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype::{TYPE_INT, TYPE_UINT, TYPE_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;
use crate::printlanguage::modifiers;

/// Ops a twice-printed expression may contain.
const PURE_BUDGET: u32 = 8;
/// Data-flow steps searched from the tested value for a `switch`.
const SWITCH_DEPTH: u32 = 10;
/// `auto`: the fewest values a stand-alone condition's range splits at.
pub const STANDALONE_MIN_VALUES: u64 = 17;

/// The `rangetest` values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RangeTestMode {
    /// The merged comparison everywhere.
    Merged,
    /// The two comparisons wherever they are exact.
    Split,
    /// `split`, except loop conditions and stand-alone conditions of at most
    /// 16 values.
    #[default]
    Auto,
}

impl RangeTestMode {
    pub fn as_str(self) -> &'static str {
        match self {
            RangeTestMode::Merged => "merged",
            RangeTestMode::Split => "split",
            RangeTestMode::Auto => "auto",
        }
    }
}

/// The `rangetest` option parser.
pub struct OptionRangeTest;

impl OptionRangeTest {
    pub const NAME: &'static str = "rangetest";

    pub fn apply(&self, p1: &str) -> KunaResult<(RangeTestMode, String)> {
        let mode = match p1 {
            "merged" => RangeTestMode::Merged,
            "split" => RangeTestMode::Split,
            "auto" | "" => RangeTestMode::Auto,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown rangetest value: {other} (expected merged|split|auto)"
                )))
            }
        };
        Ok((mode, format!("Range test rendering set to {}", mode.as_str())))
    }
}

/// A merged test reoriented as `t <= hi` (`inside`) or `hi < t` (outside),
/// from the printed comparison `token` with the constant `c` on the left or
/// right. `None` for a test that is constant.
pub fn orient(token: &str, const_left: bool, c: u64, mask: u64) -> Option<(bool, u64)> {
    let token = match (const_left, token) {
        (false, t) => t,
        (true, "<") => ">",
        (true, "<=") => ">=",
        (true, ">") => "<",
        (true, ">=") => "<=",
        _ => return None,
    };
    let (inside, hi) = match token {
        "<" => (true, c.checked_sub(1)?),
        "<=" => (true, c),
        ">" => (false, c),
        ">=" => (false, c.checked_sub(1)?),
        _ => return None,
    };
    (hi < mask).then_some((inside, hi))
}

/// The interval `[a, a + h]` that `(x - a) mod 2^bits <= h` selects, when it
/// does not wrap past `2^bits - 1`.
pub fn interval(a: u64, h: u64, bits: u32) -> Option<(u64, u64)> {
    let top = a.checked_add(h)?;
    (a != 0 && (bits >= 64 || top >> bits == 0)).then_some((a, top))
}

/// Whether `lo <= x && x <= hi` reads the same as the `bits`-bit unsigned
/// value of `x` on every input, `x` printing in a type of `bits` bits that is
/// unsigned when `unsigned`.
pub fn exact(hi: u64, bits: u32, unsigned: bool) -> bool {
    let limit_bits = if unsigned { bits } else { bits - 1 };
    limit_bits >= 64 || hi >> limit_bits == 0
}

/// A merged range test the printer splits: `lo <= x && x <= hi`, or
/// `x < lo || hi < x` when not `inside`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RangeSplit {
    pub x: VarnodeId,
    pub reader: OpId,
    pub lo: u64,
    pub hi: u64,
    pub size: i32,
    pub inside: bool,
}

impl PrintC {
    /// The split of the unsigned comparison `op`, printing as `token`, when the
    /// `rangetest` value and the site allow it and the two comparisons are
    /// exact.
    pub(crate) fn range_split(&self, fd: &Funcdata, arch: &Architecture, op: OpId, token: &str) -> Option<RangeSplit> {
        let mode = arch.range_test;
        if mode == RangeTestMode::Merged {
            return None;
        }
        let o = fd.obank().get(op)?;
        if !matches!(o.code(), OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL) {
            return None;
        }
        let (in0, in1) = (o.get_in(0)?, o.get_in(1)?);
        let constant = |vn: VarnodeId| fd.vbank().get(vn).filter(|v| v.is_constant()).map(|v| v.get_offset());
        let (c, mut t, const_left) = match (constant(in0), constant(in1)) {
            (Some(c), None) => (c, in1, true),
            (None, Some(c)) => (c, in0, false),
            _ => return None,
        };
        let size = fd.vbank().get(t)?.get_size();
        if !(1..=8).contains(&size) {
            return None;
        }
        let bits = 8 * size as u32;
        let mask = u64::MAX >> (64 - bits);
        let (inside, h) = orient(token, const_left, c, mask)?;
        while let Some(def) = implied_def(fd, t, OpCode::CPUI_CAST) {
            t = fd.obank().get(def)?.get_in(0)?;
        }
        let add = implied_def(fd, t, OpCode::CPUI_INT_SUB).or_else(|| implied_def(fd, t, OpCode::CPUI_INT_ADD))?;
        let add_op = fd.obank().get(add)?;
        let (x0, k) = (add_op.get_in(0)?, constant(add_op.get_in(1)?)?);
        if fd.vbank().get(x0)?.get_size() != size || fd.vbank().get(t)?.get_size() != size {
            return None;
        }
        let a = if add_op.code() == OpCode::CPUI_INT_SUB { k } else { k.wrapping_neg() } & mask;
        let (lo, hi) = interval(a, h, bits)?;
        if mode == RangeTestMode::Auto && !self.range_split_site(op, h) {
            return None;
        }
        let (x, reader) = self.range_operand(fd, x0, add, hi)?;
        if !printable_twice(fd, x, &mut PURE_BUDGET.clone()) || branches_to_switch(fd, op) || feeds_switch(fd, &[t, x0, x]) {
            return None;
        }
        Some(RangeSplit { x, reader, lo, hi, size: fd.vbank().get(x)?.get_size(), inside })
    }

    /// `auto`: whether the comparison `op`, whose range holds `h + 1` values,
    /// sits where splitting pays: not in a loop's condition, and as a
    /// stand-alone condition (not an operand of `&&`/`||`) only for a range of
    /// more than 16 values. A short run of values is more often a `switch` case
    /// list sharing one body, or a macro, than two comparisons.
    fn range_split_site(&self, op: OpId, h: u64) -> bool {
        if self.context.is_set(modifiers::LOOP_CONDITION) {
            return false;
        }
        let mods = match self.range_entry {
            Some((at, mods)) if at == op => mods,
            _ => 0,
        };
        let standalone = mods & modifiers::CONDITION_CONTEXT != 0 && mods & modifiers::CHAIN_OPERAND == 0;
        !standalone || h >= STANDALONE_MIN_VALUES - 1
    }

    /// The value the two comparisons test and the op that reads it: `x0`, or
    /// the deepest value under its in-place same-size casts and the extensions
    /// that keep `[lo, hi]` one interval, whose printed type keeps both
    /// comparisons exact. A cast's own output is never chosen: where C's
    /// conversions make it redundant it prints as its input.
    fn range_operand(&self, fd: &Funcdata, x0: VarnodeId, add: OpId, hi: u64) -> Option<(VarnodeId, OpId)> {
        let (mut x, mut reader) = (x0, add);
        let mut best = None;
        loop {
            let v = fd.vbank().get(x)?;
            let def = v.is_implied().then(|| v.get_def()).flatten();
            let code = def.and_then(|d| fd.obank().get(d)).map(|d| d.code());
            if code != Some(OpCode::CPUI_CAST) && self.prints_exactly(fd, x, reader, hi, 8 * v.get_size() as u32) {
                best = Some((x, reader));
            }
            let (Some(def), Some(code)) = (def, code) else { break };
            let inner = fd.obank().get(def)?.get_in(0)?;
            let inner_bits = 8 * fd.vbank().get(inner)?.get_size() as u32;
            let through = match code {
                OpCode::CPUI_CAST => inner_bits == 8 * v.get_size() as u32,
                OpCode::CPUI_INT_ZEXT => exact(hi, inner_bits, true),
                OpCode::CPUI_INT_SEXT => exact(hi, inner_bits, false),
                _ => false,
            };
            if !through {
                break;
            }
            x = inner;
            reader = def;
        }
        best
    }

    /// Whether `x`, read by `reader`, prints in an integer type of `bits` bits
    /// in which `lo <= x && x <= hi` is exact.
    fn prints_exactly(&self, fd: &Funcdata, x: VarnodeId, reader: OpId, hi: u64, bits: u32) -> bool {
        let ty = self.printed_operand_type(fd, x, reader);
        if 8 * ty.get_size() as u32 != bits || ty.is_enum_type() {
            return false;
        }
        match ty.get_metatype() {
            TYPE_UINT => exact(hi, bits, true),
            TYPE_INT | TYPE_UNKNOWN => exact(hi, bits, false),
            _ => false,
        }
    }

    /// Print `split`: `lo <= x && x <= hi` or `x < lo || hi < x`. The first
    /// clause of a chain nested in a comma-separated clause prints with no
    /// parenthesis of its own, so a split there that binds looser than the
    /// chain's operator carries its own.
    pub(crate) fn emit_range_split(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId, s: RangeSplit) {
        use crate::printc::tokens::{BOOLEAN_AND, BOOLEAN_OR, LESS_EQUAL, LESS_THAN};
        use crate::prettyprint::Emit;
        use crate::printlanguage::{CLOSE_PAREN, OPEN_PAREN};
        let key = Some(crate::printc::op_key(op));
        let tok = if s.inside { &BOOLEAN_AND } else { &BOOLEAN_OR };
        let paren = self.rpn_is_empty()
            && self.context.is_set(modifiers::COMMA_SEPARATE)
            && self.range_chain_head.is_some_and(|head| head.precedence > tok.precedence);
        let id = paren.then(|| self.emit.open_paren(OPEN_PAREN, 0));
        self.push_op(tok, key);
        if s.inside {
            self.push_op(&LESS_EQUAL, key);
            self.push_constant_ir(s.lo, s.size, op);
            self.push_vn_ir(fd, arch, s.x, s.reader);
            self.push_op(&LESS_EQUAL, key);
            self.push_vn_ir(fd, arch, s.x, s.reader);
            self.push_constant_ir(s.hi, s.size, op);
        } else {
            self.push_op(&LESS_THAN, key);
            self.push_vn_ir(fd, arch, s.x, s.reader);
            self.push_constant_ir(s.lo, s.size, op);
            self.push_op(&LESS_THAN, key);
            self.push_constant_ir(s.hi, s.size, op);
            self.push_vn_ir(fd, arch, s.x, s.reader);
        }
        if let Some(id) = id {
            self.emit.close_paren(CLOSE_PAREN, id);
        }
    }
}

/// The defining op of `vn` when it is `code` and prints in place.
fn implied_def(fd: &Funcdata, vn: VarnodeId, code: OpCode) -> Option<OpId> {
    let v = fd.vbank().get(vn)?;
    if !v.is_implied() {
        return None;
    }
    let def = v.get_def()?;
    (fd.obank().get(def)?.code() == code).then_some(def)
}

/// Whether `vn` prints the same value each time it is printed: a constant, a
/// non-volatile variable, or an in-place expression of pure operations on
/// such values within `budget` operations.
fn printable_twice(fd: &Funcdata, vn: VarnodeId, budget: &mut u32) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if v.is_constant() || v.is_annotation() {
        return true;
    }
    if v.is_volatile() {
        return false;
    }
    if !v.is_implied() {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else { return false };
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    let pure = matches!(
        op.code(),
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
    );
    pure && (0..op.num_input()).all(|i| op.get_in(i).is_some_and(|input| printable_twice(fd, input, budget)))
}

/// Whether the conditional branch on the comparison `op` (or its negation)
/// enters a block that ends in a computed jump, directly or through one block
/// with a single exit: the test is the bound check of a `switch`.
fn branches_to_switch(fd: &Funcdata, op: OpId) -> bool {
    let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()) else { return false };
    let mut values = vec![out];
    let mut branches = Vec::new();
    while let Some(vn) = values.pop() {
        let Some(v) = fd.vbank().get(vn) else { continue };
        for reader in v.descend_iter() {
            let Some(r) = fd.obank().get(reader) else { continue };
            match r.code() {
                OpCode::CPUI_CBRANCH => branches.extend(r.get_parent()),
                OpCode::CPUI_BOOL_NEGATE => values.extend(r.get_out()),
                _ => {}
            }
        }
    }
    let ends_in_switch = |blk| {
        fd.bb_op_tail(blk).and_then(|tail| fd.obank().get(tail)).is_some_and(|tail| tail.code() == OpCode::CPUI_BRANCHIND)
    };
    branches.into_iter().any(|blk| {
        let block = fd.bblocks_ref().block(blk);
        (0..block.size_out()).any(|i| {
            let next = block.get_out(i);
            let nb = fd.bblocks_ref().block(next);
            ends_in_switch(next) || (nb.size_out() == 1 && ends_in_switch(nb.get_out(0)))
        })
    })
}

/// Whether a value reached from `starts` within [`SWITCH_DEPTH`] data-flow
/// steps selects a `switch`: the test is then the switch's bound check.
fn feeds_switch(fd: &Funcdata, starts: &[VarnodeId]) -> bool {
    let mut seen: HashSet<VarnodeId> = starts.iter().copied().collect();
    let mut frontier: Vec<VarnodeId> = starts.to_vec();
    for _ in 0..SWITCH_DEPTH {
        let mut next = Vec::new();
        for vn in frontier {
            let Some(v) = fd.vbank().get(vn) else { continue };
            for reader in v.descend_iter() {
                let Some(op) = fd.obank().get(reader) else { continue };
                if op.code() == OpCode::CPUI_BRANCHIND {
                    return true;
                }
                if let Some(out) = op.get_out() {
                    if seen.insert(out) {
                        next.push(out);
                    }
                }
            }
        }
        frontier = next;
    }
    false
}

#[cfg(test)]
mod tests;
