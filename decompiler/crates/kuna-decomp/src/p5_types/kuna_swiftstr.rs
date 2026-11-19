//! (kuna `fatstr`) Swift string literals — the two constant words a
//! `Swift.String` is at `-O`.
//!
//! ```text
//!   *(ulong *)&v1[8] = 0xe500000000000000;  -->  *(ulong *)&v1[8] = "shape";
//!   *(ulong *)v1 = 0x6570616873;
//!   p->field_0x8 = 0x80000001038e4750;      -->  p->field_0x8 = "a-literal-longer-than-fifteen-utf8-bytes";
//!   p->field_0x0 = 0xd000000000000028;
//! ```
//!
//! A `Swift.String` is two words: `_countAndFlagsBits` and `_object`. A
//! literal of up to 15 UTF-8 bytes is a **small string**: the bytes live in
//! the two words themselves (the first eight in the count word, the next
//! seven in the low bytes of the object word, whose top byte is the
//! discriminator: `0xa0..0xaf` for a non-ASCII, `0xe0..0xef` for an ASCII
//! literal, the count in the low nibble). A longer literal is an **immortal native
//! string**: the object word's top nibble is `8`, its low 60 bits point 32
//! bytes before the NUL-terminated UTF-8 payload in the image, and the count
//! word is `0xd000…` (ASCII) or `0x1000…` (not) over the byte count.
//!
//! The rule finds an object word among a `STORE`'s value, a `COPY`'s source,
//! a `CALL`'s or `RETURN`'s inputs or a `PIECE`'s high half, pairs it with the
//! count word the same code writes beside it (the slot eight bytes below in
//! the same block, the input before it, the low half; for a word copied into
//! a register, the partner its consumers take beside it, traced back along
//! the same branch arms), decodes the literal, and replaces the
//! object word with the bounded literal
//! ([`Funcdata::get_internal_string`], the `fatstr` mechanism), so the text
//! prints where the constant did. The count word is left as it is: the code
//! really stores it. An immortal literal needs no partner (the payload is
//! NUL-terminated; a partner found must agree on the count); a small one is
//! decoded only with both words.
//!
//! The Swift arm of `option fatstr off|auto|always`: `auto` (default) acts when
//! the image's source language is Swift; `always` drops the language test; `off` keeps the
//! constants.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_fatstr::{FatStrMode, bytes_are_text};

/// Is the rewrite live for this function?
pub fn live(data: &Funcdata) -> bool {
    match FatStrMode::from_u8(data.get_arch().fat_str) {
        FatStrMode::Off => false,
        FatStrMode::Auto => data.get_arch().source_lang.is_swift(),
        FatStrMode::Always => true,
    }
}

/// The longest immortal literal decoded.
pub const MAX_LEN: u64 = 0x10000;

/// A decoded literal: its UTF-8 bytes and, for an immortal string, the
/// address of its payload.
#[derive(Debug, PartialEq, Eq)]
pub struct Literal {
    pub bytes: Vec<u8>,
    pub payload: Option<u64>,
}

/// Is `object` the object word of a small string? The discriminator byte is
/// `0b1a10_cccc`: immortal (bit 63) and small (bit 61) set, bit 60 clear, bit
/// 62 the isASCII flag, the count in the low nibble (at most 15).
pub fn small_count(object: u64) -> Option<usize> {
    let disc = (object >> 56) as u8;
    if disc & 0xb0 != 0xa0 {
        return None;
    }
    let count = (disc & 0xf) as usize;
    (count <= 15).then_some(count)
}

/// The small string in `(count_word, object)`: the first eight bytes from the
/// count word, the next seven from the object word, every byte past the count
/// zero, the isASCII flag true to the bytes, the bytes UTF-8.
pub fn decode_small(count_word: u64, object: u64) -> Option<Literal> {
    let count = small_count(object)?;
    let mut raw = [0u8; 15];
    raw[..8].copy_from_slice(&count_word.to_le_bytes());
    raw[8..].copy_from_slice(&object.to_le_bytes()[..7]);
    if raw[count..].iter().any(|&b| b != 0) {
        return None;
    }
    let bytes = raw[..count].to_vec();
    let ascii = (object >> 62) & 1 != 0;
    if ascii != bytes.is_ascii() || core::str::from_utf8(&bytes).is_err() {
        return None;
    }
    Some(Literal {
        bytes,
        payload: None,
    })
}

/// The payload address of an immortal native string's object word (top
/// nibble `8`): 32 bytes past the pointed-at object header.
pub fn immortal_payload(object: u64) -> Option<u64> {
    (object >> 60 == 8).then(|| (object & 0x0fff_ffff_ffff_ffff).wrapping_add(0x20))
}

/// The count an immortal string's count word carries (`0xd000…` ASCII,
/// `0x1000…` not), `None` for any other flag byte.
pub fn immortal_count(count_word: u64) -> Option<(u64, bool)> {
    match count_word >> 48 {
        0xd000 => Some((count_word & 0xffff_ffff_ffff, true)),
        0x1000 => Some((count_word & 0xffff_ffff_ffff, false)),
        _ => None,
    }
}

/// The literal `payload` holds: the bytes up to the NUL within [`MAX_LEN`],
/// text, read-only in the image, and of the count the partner word states
/// when one is known.
fn read_immortal(
    data: &Funcdata,
    payload: u64,
    count: Option<(u64, bool)>,
    at: &Address,
) -> Option<Literal> {
    let spc = data.get_arch().manage().get_default_data_space().cloned()?;
    let addr = Address::new(Rc::clone(&spc), payload);
    let want = count.map(|(n, _)| n as usize);
    let chunk = want.unwrap_or(256).max(1).min(MAX_LEN as usize) + 1;
    let mut buf = Vec::new();
    loop {
        let mut piece = vec![0u8; chunk];
        let piece_addr = Address::new(Rc::clone(&spc), payload.checked_add(buf.len() as u64)?);
        data.get_arch().loader_fill(&mut piece, &piece_addr).ok()?;
        if let Some(nul) = piece.iter().position(|&b| b == 0) {
            buf.extend_from_slice(&piece[..nul]);
            break;
        }
        buf.extend_from_slice(&piece);
        if buf.len() > MAX_LEN as usize {
            return None;
        }
    }
    if buf.is_empty() || buf.len() > MAX_LEN as usize {
        return None;
    }
    match count {
        Some((n, ascii)) => {
            if n as usize != buf.len() || ascii != buf.is_ascii() {
                return None;
            }
        }
        // Without the count word's confirmation, a one- or two-byte payload
        // is no evidence that the word was a string at all.
        None if buf.len() < 3 => return None,
        None => {}
    }
    if data
        .get_arch()
        .query_global_properties(&addr, buf.len() as int4, at)
        & crate::varnode::varnode_flags::readonly
        == 0
    {
        return None;
    }
    if !bytes_are_text(&buf) {
        return None;
    }
    Some(Literal {
        bytes: buf,
        payload: Some(payload),
    })
}

/// Where a constant word sits, for pairing: `(base varnode, byte offset)` of
/// the address a `STORE` writes, the stack slot a `COPY` fills, the register
/// or temporary a `COPY` fills (paired by data flow), or a call / return /
/// piece input slot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Slot {
    Store {
        base: Option<VarnodeId>,
        offset: i64,
    },
    Stack {
        space: int4,
        offset: u64,
    },
    Flow {
        out: VarnodeId,
    },
    Input {
        op: OpId,
        index: int4,
    },
}

/// `(base, offset)` of an address expression: a constant, or a chain of
/// `PTRSUB` / `PTRADD` / `INT_ADD` by constants from a base varnode (a typed
/// pointer's field path, `p[2].isa`, folds to its byte offset).
fn address_parts(data: &Funcdata, mut vn: VarnodeId) -> (Option<VarnodeId>, i64) {
    let mut offset = 0i64;
    for _ in 0..8 {
        let Some(v) = data.vbank().get(vn) else {
            return (Some(vn), offset);
        };
        if v.is_constant() {
            return (None, offset.wrapping_add(v.get_offset() as i64));
        }
        let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else {
            return (Some(vn), offset);
        };
        let input = |i: int4| {
            def.get_in(i)
                .and_then(|x| data.vbank().get(x).map(|v| (x, v)))
        };
        let step = match def.code() {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => match (input(0), input(1)) {
                (Some((b, bv)), Some((_, c))) if c.is_constant() && !bv.is_constant() => {
                    Some((b, c.get_offset() as i64))
                }
                (Some((_, c)), Some((b, bv))) if c.is_constant() && !bv.is_constant() => {
                    Some((b, c.get_offset() as i64))
                }
                _ => None,
            },
            OpCode::CPUI_PTRADD => match (input(0), input(1), input(2)) {
                (Some((b, _)), Some((_, i)), Some((_, s)))
                    if i.is_constant() && s.is_constant() =>
                {
                    Some((
                        b,
                        (i.get_offset() as i64).wrapping_mul(s.get_offset() as i64),
                    ))
                }
                _ => None,
            },
            _ => None,
        };
        match step {
            Some((b, off)) => {
                offset = offset.wrapping_add(off);
                vn = b;
            }
            None => return (Some(vn), offset),
        }
    }
    (Some(vn), offset)
}

/// The slot the constant at input `index` of `op` occupies.
fn slot_of(data: &Funcdata, op: OpId, index: int4) -> Option<Slot> {
    let o = data.obank().get(op)?;
    Some(match o.code() {
        OpCode::CPUI_STORE => {
            let (base, offset) = address_parts(data, o.get_in(1)?);
            Slot::Store { base, offset }
        }
        OpCode::CPUI_COPY => {
            let out = data.vbank().get(o.get_out()?)?;
            let space = out.get_addr().get_space()?;
            if space.get_type() == spacetype::IPTR_CONSTANT {
                return None;
            }
            if space.get_type() != spacetype::IPTR_SPACEBASE {
                return Some(Slot::Flow { out: o.get_out()? });
            }
            Slot::Stack {
                space: space.get_index(),
                offset: out.get_addr().get_offset(),
            }
        }
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN | OpCode::CPUI_PIECE => {
            Slot::Input { op, index }
        }
        _ => return None,
    })
}

/// The 8-byte constant `vn` is, or is a `COPY` of (a register loaded with
/// the word and concatenated or passed on).
fn constant_of(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    if v.get_size() != 8 {
        return None;
    }
    if v.is_constant() {
        return Some(v.get_offset());
    }
    let def = data.obank().get(v.get_def()?)?;
    if def.code() != OpCode::CPUI_COPY {
        return None;
    }
    let src = data.vbank().get(def.get_in(0)?)?;
    (src.is_constant() && src.get_size() == 8).then(|| src.get_offset())
}

/// How far from a `STORE` or stack `COPY` its neighbour slot is looked for.
const MAX_SCAN: usize = 24;

/// The constant count word paired with the object word at `slot`.
fn partner(data: &Funcdata, op: OpId, slot: Slot, object: u64) -> Option<u64> {
    let word =
        |o: OpId, i: int4| -> Option<u64> { constant_of(data, data.obank().get(o)?.get_in(i)?) };
    match slot {
        Slot::Flow { out } => flow_partner(data, out, object),
        Slot::Input { op, index } => {
            let o = data.obank().get(op)?;
            // A piece is `PIECE(high, low)`; a call or return lists the
            // words in order.
            if o.code() == OpCode::CPUI_PIECE {
                return word(op, 1);
            }
            if index <= 1 {
                return None;
            }
            word(op, index - 1)
        }
        Slot::Store { .. } | Slot::Stack { .. } => adjacent_values(data, op, slot)
            .into_iter()
            .filter_map(|vn| constant_of(data, vn))
            .find(|&w| w != object),
    }
}

/// The values written to the slot eight bytes below `slot` (a `STORE` or a
/// stack `COPY` like `op`), nearest first in each direction, looked for within
/// [`MAX_SCAN`] ops of `op` in its own block.
fn adjacent_values(data: &Funcdata, op: OpId, slot: Slot) -> Vec<VarnodeId> {
    let want = match slot {
        Slot::Store { base, offset } => Slot::Store {
            base,
            offset: offset.wrapping_sub(8),
        },
        Slot::Stack { space, offset } => Slot::Stack {
            space,
            offset: offset.wrapping_sub(8),
        },
        _ => return Vec::new(),
    };
    let Some(o) = data.obank().get(op) else {
        return Vec::new();
    };
    let (kind, block) = (o.code(), o.get_parent());
    let value_in = if kind == OpCode::CPUI_STORE { 2 } else { 0 };
    let mut found = Vec::new();
    for dir in [true, false] {
        let step = |id: OpId| {
            if dir {
                data.op_next_op(id)
            } else {
                data.op_previous_op(id)
            }
        };
        let mut cur = step(op);
        for _ in 0..MAX_SCAN {
            let Some(id) = cur else { break };
            let Some(o) = data.obank().get(id) else { break };
            if o.get_parent() != block {
                break;
            }
            if o.code() == kind && slot_of(data, id, value_in) == Some(want) {
                found.extend(o.get_in(value_in));
                break;
            }
            cur = step(id);
        }
    }
    found
}

/// Bounds on the walk from a register-held object word to the ops that
/// consume it beside its count word.
const MAX_FLOW_DEPTH: usize = 8;
const MAX_FLOW_VISITS: usize = 64;

/// The `MULTIEQUAL`s (and the slot taken in each) the object word passed
/// through on one way from its definition to a consumer.
type FlowPath = Vec<(OpId, int4)>;

/// The count word paired with the object word `out` (a register the word was
/// copied into) by data flow, not register numbering: every consumer that
/// takes the word beside its partner (a call or return input after the count,
/// a `PIECE`'s high half) names the partner, which is traced back along the
/// same `MULTIEQUAL` arms the object word came through, so a word chosen by a
/// `csel` or a branch is paired only with the count of its own arm. `None`
/// unless every consumer agrees on one constant.
fn flow_partner(data: &Funcdata, out: VarnodeId, object: u64) -> Option<u64> {
    let mut found: Option<u64> = None;
    let mut work: Vec<(VarnodeId, FlowPath)> = vec![(out, Vec::new())];
    let mut visits = 0usize;
    while let Some((vn, path)) = work.pop() {
        visits += 1;
        if visits > MAX_FLOW_VISITS {
            return None;
        }
        let Some(v) = data.vbank().get(vn) else {
            return None;
        };
        let users: Vec<OpId> = v.descend_iter().collect();
        for user in users {
            let Some(u) = data.obank().get(user) else {
                return None;
            };
            let partner_vn = match u.code() {
                OpCode::CPUI_MULTIEQUAL => {
                    if path.len() >= MAX_FLOW_DEPTH {
                        return None;
                    }
                    let Some(next) = u.get_out() else { continue };
                    for j in 0..u.num_input() {
                        if u.get_in(j) == Some(vn) {
                            let mut p = path.clone();
                            p.push((user, j));
                            work.push((next, p));
                        }
                    }
                    continue;
                }
                OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => {
                    if u.get_in(0) == Some(vn) {
                        if let Some(next) = u.get_out() {
                            work.push((next, path.clone()));
                        }
                    }
                    continue;
                }
                OpCode::CPUI_PIECE if u.get_in(0) == Some(vn) => u.get_in(1),
                OpCode::CPUI_STORE if u.get_in(2) == Some(vn) => {
                    let Some(slot) = slot_of(data, user, 2) else {
                        continue;
                    };
                    adjacent_values(data, user, slot)
                        .into_iter()
                        .find(|&w| trace_arm(data, w, &path).is_some())
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => {
                    let mut partner = None;
                    for i in 2..u.num_input() {
                        if u.get_in(i) == Some(vn) {
                            if partner.is_some() {
                                return None;
                            }
                            partner = u.get_in(i - 1);
                        }
                    }
                    partner
                }
                _ => None,
            };
            let Some(partner_vn) = partner_vn else {
                continue;
            };
            let word = trace_arm(data, partner_vn, &path)?;
            if word == object || found.is_some_and(|w| w != word) {
                return None;
            }
            found = Some(word);
        }
    }
    found
}

/// The constant `vn` holds on the way `path` describes: through each
/// `MULTIEQUAL` of `path` (last first), a partner that is itself a
/// `MULTIEQUAL` of the same block is replaced by its input on the same arm.
fn trace_arm(data: &Funcdata, mut vn: VarnodeId, path: &FlowPath) -> Option<u64> {
    let peel = |mut vn: VarnodeId| -> VarnodeId {
        for _ in 0..MAX_FLOW_DEPTH {
            let Some(def) = data
                .vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .and_then(|d| data.obank().get(d))
            else {
                break;
            };
            if !matches!(def.code(), OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT) {
                break;
            }
            match def.get_in(0) {
                Some(src) if !data.vbank().get(src).is_some_and(|s| s.is_constant()) => vn = src,
                _ => break,
            }
        }
        vn
    };
    for &(phi, slot) in path.iter().rev() {
        vn = peel(vn);
        let block = data.obank().get(phi)?.get_parent();
        let def = data
            .vbank()
            .get(vn)?
            .get_def()
            .and_then(|d| data.obank().get(d));
        let Some(def) = def else {
            continue;
        };
        if def.code() != OpCode::CPUI_MULTIEQUAL {
            continue;
        }
        if def.get_parent() == block {
            vn = def.get_in(slot)?;
            continue;
        }
        let Some(arm) = arm_condition(data, phi, slot) else {
            continue;
        };
        let partner_phi = data.vbank().get(vn)?.get_def()?;
        let mut matched = None;
        for j in 0..def.num_input() {
            if arm_condition(data, partner_phi, j).is_some_and(|other| same_arm(data, arm, other)) {
                matched = def.get_in(j);
                break;
            }
        }
        if let Some(m) = matched {
            vn = m;
        }
    }
    constant_of(data, vn)
}

/// Do two branch arms `(condition, side)` take the same side on every
/// execution? The conditions must be one varnode or one pure expression of
/// the same inputs, either possibly under `BOOL_NEGATE`s.
fn same_arm(data: &Funcdata, a: (VarnodeId, bool), b: (VarnodeId, bool)) -> bool {
    let peel = |(mut vn, mut side): (VarnodeId, bool)| {
        for _ in 0..4 {
            let Some(def) = data
                .vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .and_then(|d| data.obank().get(d))
            else {
                break;
            };
            if def.code() != OpCode::CPUI_BOOL_NEGATE {
                break;
            }
            let Some(src) = def.get_in(0) else { break };
            vn = src;
            side = !side;
        }
        (vn, side)
    };
    let ((va, sa), (vb, sb)) = (peel(a), peel(b));
    sa == sb && same_value(data, va, vb)
}

/// Are `a` and `b` one varnode, or the outputs of the same pure operation on
/// the same inputs (equal constants included)?
fn same_value(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    if a == b {
        return true;
    }
    let def = |vn: VarnodeId| {
        data.vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .and_then(|d| data.obank().get(d))
    };
    let (Some(da), Some(db)) = (def(a), def(b)) else {
        return false;
    };
    if da.code() != db.code()
        || da.num_input() != db.num_input()
        || matches!(
            da.code(),
            OpCode::CPUI_LOAD
                | OpCode::CPUI_STORE
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_NEW
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
        )
    {
        return false;
    }
    (0..da.num_input()).all(|i| match (da.get_in(i), db.get_in(i)) {
        (Some(x), Some(y)) if x == y => true,
        (Some(x), Some(y)) => match (data.vbank().get(x), data.vbank().get(y)) {
            (Some(vx), Some(vy)) => {
                vx.is_constant()
                    && vy.is_constant()
                    && vx.get_offset() == vy.get_offset()
                    && vx.get_size() == vy.get_size()
            }
            _ => false,
        },
        _ => false,
    })
}

/// The branch that selects input `slot` of the two-input `MULTIEQUAL` `phi`:
/// the condition of the `CBRANCH` ending the block both incoming edges leave
/// (directly, or through one block between), and whether the edge leaves on
/// the condition's true side. Two `MULTIEQUAL`s whose arms are chosen by the
/// same condition varnode take corresponding arms on every execution, which
/// is how a run of `csel`s (one per register of a `Swift.String`) stays
/// paired.
fn arm_condition(data: &Funcdata, phi: OpId, slot: int4) -> Option<(VarnodeId, bool)> {
    let graph = data.bblocks_ref();
    let merge = graph.block(data.obank().get(phi)?.get_parent()?);
    if merge.size_in() != 2 || !(0..2).contains(&slot) {
        return None;
    }
    let merge_id = data.obank().get(phi)?.get_parent()?;
    let header_of = |edge: int4| -> Option<(crate::context::BlockId, int4)> {
        let pred = merge.get_in(edge);
        let p = graph.block(pred);
        if p.size_out() == 2 {
            return Some((pred, merge.get_in_rev_index(edge)));
        }
        if p.size_out() == 1 && p.size_in() == 1 && graph.block(p.get_in(0)).size_out() == 2 {
            return Some((p.get_in(0), p.get_in_rev_index(0)));
        }
        None
    };
    let (header, out) = header_of(slot)?;
    let (other_header, other_out) = header_of(1 - slot)?;
    if header != other_header || out == other_out || header == merge_id {
        return None;
    }
    let branch = data.obank().get(data.bb_op_tail(header)?)?;
    if branch.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    Some((branch.get_in(1)?, (out == 1) != branch.is_boolean_flip()))
}

/// The literal the constant at input `index` of `op` is the object word of.
fn literal_at(data: &Funcdata, op: OpId, index: int4) -> Option<Literal> {
    let o = data.obank().get(op)?;
    let object = constant_of(data, o.get_in(index)?)?;
    let at = o.get_addr().clone();
    if let Some(payload) = immortal_payload(object) {
        let slot = slot_of(data, op, index);
        let count = slot
            .and_then(|s| partner(data, op, s, object))
            .and_then(immortal_count);
        return read_immortal(data, payload, count, &at);
    }
    if small_count(object).is_some() {
        let slot = slot_of(data, op, index)?;
        let count_word = partner(data, op, slot, object)?;
        return decode_small(count_word, object);
    }
    None
}

/// (kuna) `RuleSwiftString` — replace a Swift string literal's object word
/// with the literal (option `fatstr`, default auto).
pub struct RuleSwiftString {
    enabled: bool,
    group: String,
}

impl RuleSwiftString {
    pub fn with_group(enabled: bool, g: impl Into<String>) -> RuleSwiftString {
        RuleSwiftString {
            enabled,
            group: g.into(),
        }
    }
}

impl Rule for RuleSwiftString {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![
            OpCode::CPUI_STORE,
            OpCode::CPUI_COPY,
            OpCode::CPUI_CALL,
            OpCode::CPUI_CALLIND,
            OpCode::CPUI_RETURN,
            OpCode::CPUI_PIECE,
        ]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSwiftString {
            enabled: self.enabled,
            group: self.group.clone(),
        }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !live(data) {
            return 0;
        }
        let Some(o) = data.obank().get(op) else {
            return 0;
        };
        let slots: Vec<int4> = match o.code() {
            OpCode::CPUI_STORE => vec![2],
            OpCode::CPUI_COPY | OpCode::CPUI_PIECE => vec![0],
            _ => (1..o.num_input()).collect(),
        };
        let Some(spc) = data.get_arch().manage().get_default_data_space().cloned() else {
            return 0;
        };
        let ptr_size = spc.get_addr_size() as int4;
        for index in slots {
            let Some(lit) = literal_at(data, op, index) else {
                continue;
            };
            let Some(tf) = data.get_arch().types() else {
                return 0;
            };
            let Ok(ch) = tf.get_type_char(1) else {
                return 0;
            };
            let Ok(char_ptr) = tf.get_type_pointer(ptr_size, ch, spc.get_word_size()) else {
                return 0;
            };
            // The empty literal is registered as its terminator alone.
            let bytes = if lit.bytes.is_empty() {
                vec![0u8]
            } else {
                lit.bytes
            };
            let len = bytes.len() as int4;
            // Replace the constant where it is written: the input itself, or
            // the `COPY` that loaded it into a register.
            let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(index)) else {
                continue;
            };
            let (site, slot) = match data.vbank().get(vn) {
                Some(v) if v.is_constant() => (op, index),
                Some(v) => match v.get_def() {
                    Some(d) => (d, 0),
                    None => continue,
                },
                None => continue,
            };
            let Some(res) = data.get_internal_string(&bytes, len, char_ptr, site) else {
                continue;
            };
            data.op_set_input(site, res, slot)
                .expect("RuleSwiftString: setInput");
            return 1;
        }
        0
    }
}

/// The per-root rule registrations (the `fatstr` pattern).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        ctor: || Box::new(RuleSwiftString::with_group(true, "analysis")),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_strings_decode_from_their_two_words() {
        // "shape": count 5, ASCII.
        let lit = decode_small(0x6570616873, 0xe500000000000000).unwrap();
        assert_eq!(lit.bytes, b"shape");
        assert_eq!(lit.payload, None);
        // 15 bytes fill both words.
        let s = b"abcdefghijklmno";
        let w0 = u64::from_le_bytes(s[..8].try_into().unwrap());
        let mut hi = [0u8; 8];
        hi[..7].copy_from_slice(&s[8..15]);
        hi[7] = 0xef;
        let w1 = u64::from_le_bytes(hi);
        assert_eq!(decode_small(w0, w1).unwrap().bytes, s);
        // A non-ASCII string carries the flag off: "é" is two bytes.
        assert_eq!(
            decode_small(0xa9c3, 0xa200000000000000).unwrap().bytes,
            "é".as_bytes()
        );
        // Refused: a count the bytes contradict, the ASCII flag wrong, not UTF-8.
        assert!(decode_small(0x6570616873, 0xe400000000000000).is_none());
        assert!(decode_small(0x6570616873, 0xa500000000000000).is_none());
        assert!(decode_small(0xff, 0xa100000000000000).is_none());
        assert!(small_count(0x80000001038e4750).is_none());
    }

    #[test]
    fn immortal_words_name_their_payload_and_count() {
        assert_eq!(immortal_payload(0x80000001038e4750), Some(0x1038e4770));
        assert_eq!(immortal_payload(0xe500000000000000), None);
        assert_eq!(immortal_count(0xd000000000000028), Some((40, true)));
        assert_eq!(immortal_count(0x1000000000000003), Some((3, false)));
        assert_eq!(immortal_count(0x6570616873), None);
    }
}
