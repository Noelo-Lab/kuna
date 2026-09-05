//! (kuna `rodatastring`) Read-only string block-copy recovery — the
//! rodata-sourced half of the S5 "constsequence" group
//! ([`constseq`](crate::constseq)).
//!
//! `RuleStringCopy` only fires for a run of COPYs whose inputs are pcode
//! CONSTANTS — the character-at-a-time stack string a compiler emits at `-O0`.
//! A compiler that has the whole literal in read-only memory instead emits a
//! BLOCK copy: one or more wide loads out of `.rodata`/`__cstring` re-stored
//! into the frame.  Those loads survive as free read-only memory varnodes, so
//! the run reaches the printer as a handful of partial-symbol slice
//! assignments — `v1[0] = (char[8])s_100003f1d._0_8_;` and
//! `v8._0_9_ = s_100003f1d._16_9_;` — which are not valid C (there is no array
//! cast, and `._0_9_` is member syntax on an array object), and which hide the
//! string kuna has already recovered at that address.
//!
//! [`RuleRodataStringCopy`] recognizes such a run and collapses it into the
//! same `builtin_strncpy(dest, "…", n)` user-op `RuleStringCopy` produces,
//! reusing the shared [`StringSequence`] build/teardown
//! (`constructTypedPointer` + `removeCopyOps`) through the `pub(crate)`
//! entry points `from_rodata_run`/`transform_rodata`.
//!
//! # What is required before a run is claimed
//!
//! Each guard exists so the rewrite is a fact, not a guess:
//!
//! * every COPY's source is a **read-only** (`Varnode::isReadOnly`) free memory
//!   varnode, so its bytes are the same at run time as in the image;
//! * the sources all lie inside ONE covering data symbol whose type is a
//!   char-printable array — the symbol the string-literal analysis planted;
//! * source and destination advance in lockstep (one constant delta, equal
//!   sizes), so the run is a straight block copy and not a shuffle;
//! * the COPYs tile the destination **exactly** — no gap, no overlap — over the
//!   symbol's whole length, so nothing is invented and nothing is dropped;
//! * the image bytes really are one NUL-terminated string of exactly that
//!   length;
//! * all the COPYs are in one basic block with no interfering LOAD/STORE/CALL
//!   between them (`ArraySequence::interfereBetween`, the same window
//!   `RuleStringCopy` demands).
//!
//! A run of ONE COPY is left alone: the filed defect is the *split* copy, and a
//! whole-string single COPY is already a single assignment.
//!
//! Gated by the arch flag `rodata_string` (option `rodatastring on|off`,
//! shipped default `on`); inert when off.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::constseq::{ArraySequence, StringSequence, WriteNode};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, TypeFactory};
use crate::funcdata::Funcdata;

/// One COPY of the block-copy run: its byte offset from the start of the string
/// and the op that moves it.
struct Slice {
    /// Byte offset into the copied string.
    off: int4,
    /// Byte width of this COPY.
    size: int4,
    /// The COPY op.
    op: OpId,
    /// Block order of the COPY (`op->getSeqNum().getOrder()`).
    order: u32,
}

/// (kuna `rodatastring`) Collapse a read-only string block copy into
/// `builtin_strncpy`.
pub struct RuleRodataStringCopy {
    /// Resolved `glb->rodata_string` gate, ORed with the live architecture flag.
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleRodataStringCopy {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleRodataStringCopy {
        RuleRodataStringCopy { enabled, group: String::from("analysis") }
    }

    /// Construct with an explicit group.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleRodataStringCopy {
        RuleRodataStringCopy { enabled, group: group.into() }
    }
}

impl Rule for RuleRodataStringCopy {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleRodataStringCopy {
            enabled: self.enabled,
            group: self.group.clone(),
        }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !data.get_arch().rodata_string {
            return 0;
        }
        match try_recover(op, data) {
            Some(seq) if seq.transform_rodata(data) => 1,
            _ => 0,
        }
    }
}

/// Everything up to (but not including) the rewrite: validate `root` as a
/// member of a read-only string block copy and assemble the sequence.
fn try_recover(root: OpId, data: &mut Funcdata) -> Option<StringSequence> {
    let block = data.obank().get(root)?.get_parent()?;
    let op_addr = data.obank().get(root)?.get_addr().clone();

    // --- the root's destination: an address-tied stack (spacebase) varnode ---
    let dst_vn = data.obank().get(root)?.get_out()?;
    let (dst_addr, dst_size) = {
        let v = data.vbank().get(dst_vn)?;
        if !v.is_addr_tied() {
            return None;
        }
        (v.get_addr().clone(), v.get_size())
    };
    let dst_space = dst_addr.get_space()?.clone();
    if dst_space.get_type() != spacetype::IPTR_SPACEBASE {
        return None;
    }

    // --- the root's source: a free read-only image varnode ---
    let src_vn = readonly_source(data, root)?;
    let (src_addr, src_size) = {
        let v = data.vbank().get(src_vn)?;
        (v.get_addr().clone(), v.get_size())
    };
    if src_size != dst_size {
        return None;
    }

    // --- the covering string symbol the string-literal analysis planted ---
    let container = data.get_arch().query_container_global(&src_addr, src_size, &op_addr)?;
    let sym_type = container.symbol_type.clone()?;
    if sym_type.get_metatype() != type_metatype::TYPE_ARRAY || sym_type.is_opaque_string() {
        return None;
    }
    let elem = sym_type.get_array_base()?;
    if elem.get_size() != 1 || !elem.is_char_print() || elem.is_opaque_string() {
        return None;
    }
    let str_len = sym_type.get_size();
    if str_len < ArraySequence::MINIMUM_SEQUENCE_LENGTH
        || str_len > ArraySequence::MAXIMUM_SEQUENCE_LENGTH
    {
        return None;
    }
    let str_addr = container.entry_addr.clone();
    if !Rc::ptr_eq(str_addr.get_space()?, src_addr.get_space()?) {
        return None;
    }

    // The destination address the string's byte 0 lands on.
    let root_off = src_addr.get_offset().wrapping_sub(str_addr.get_offset()) as i64;
    if root_off < 0 || root_off + src_size as i64 > str_len as i64 {
        return None;
    }
    let dst_start = &dst_addr + (-root_off);

    let mut foreign: Vec<u32> = Vec::new();
    let slices = collect_slices(data, block, &dst_start, &str_addr, str_len, &mut foreign)?;
    if !covers_exactly(&slices, str_len) {
        return None;
    }
    if !slices.iter().any(|s| s.op == root) {
        return None;
    }
    // The CALLOTHER is inserted at the run's FIRST member, so any other write
    // into the region between the first and last member would be re-ordered
    // against the run and could win where it used to lose.
    let lo = slices.iter().map(|s| s.order).min()?;
    let hi = slices.iter().map(|s| s.order).max()?;
    if foreign.iter().any(|&o| o > lo && o < hi) {
        return None;
    }
    if !no_interference(data, &slices) {
        return None;
    }

    // --- the bytes themselves: exactly one NUL-terminated string ---
    let bytes = read_image(data, &str_addr, str_len)?;
    if bytes[str_len as usize - 1] != 0 || bytes[..str_len as usize - 1].contains(&0) {
        return None;
    }

    // The `char` the pointer-identity check in `selectStringCopyFunction` wants,
    // and a synthesized `char[n]` container so `constructTypedPointer` walks
    // straight down to a `char *` at `dst_start` (no PTRADD, no residual add).
    let types = data.get_arch().types_rc()?;
    let char_type = types.get_type_char(types.get_size_of_char()).ok()?;
    if char_type.get_size() != 1 {
        return None;
    }
    let dest_type = types.get_type_array(str_len, Rc::clone(&char_type)).ok()?;
    let entry = crate::varmap::StringContainerEntry {
        first: dst_start.get_offset(),
        size: str_len,
        addr: dst_start.clone(),
        sym_type: dest_type,
    };

    let move_ops: Vec<WriteNode> = slices
        .iter()
        .map(|s| {
            WriteNode::new(
                dst_start.get_offset().wrapping_add(s.off as u64),
                s.op,
                -1,
                s.order,
            )
        })
        .collect();
    Some(StringSequence::from_rodata_run(
        char_type, entry, root, block, dst_start, move_ops, bytes,
    ))
}

/// The COPY's source varnode, looked through any CAST, if it is a free
/// (undefined, non-constant) read-only memory varnode.
fn readonly_source(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    let mut vn = data.obank().get(op)?.get_in(0)?;
    // ActionSetCasts has not run at constsequence time, but a CAST inserted by an
    // earlier rule can already sit between the memory read and the COPY.
    for _ in 0..2 {
        let v = data.vbank().get(vn)?;
        if !v.is_written() {
            break;
        }
        let def = v.get_def()?;
        if data.obank().get(def)?.code() != OpCode::CPUI_CAST {
            return None;
        }
        vn = data.obank().get(def)?.get_in(0)?;
    }
    let v = data.vbank().get(vn)?;
    if v.is_written() || v.is_constant() || !v.is_read_only() {
        return None;
    }
    if v.get_addr().get_space()?.get_type() != spacetype::IPTR_PROCESSOR {
        return None;
    }
    Some(vn)
}

/// Gather every same-block COPY that moves `str_addr + k` to `dst_start + k`,
/// plus the block order of every OTHER in-block write into the same region (a
/// foreign write interleaved with the run would be re-ordered against it).
fn collect_slices(
    data: &Funcdata,
    block: BlockId,
    dst_start: &Address,
    str_addr: &Address,
    str_len: int4,
    foreign: &mut Vec<u32>,
) -> Option<Vec<Slice>> {
    let scan_end = dst_start + i64::from(str_len);
    let ids: Vec<VarnodeId> = data.vbank().iter_loc_addr_range(dst_start, &scan_end).collect();
    let mut out: Vec<Slice> = Vec::new();
    for vn in ids {
        let v = data.vbank().get(vn)?;
        if !v.is_written() || !v.is_addr_tied() {
            continue;
        }
        let def = v.get_def()?;
        let o = data.obank().get(def)?;
        if o.get_parent() != Some(block) {
            continue;
        }
        let order = o.get_seq_num().get_order();
        let off = v.get_offset().wrapping_sub(dst_start.get_offset()) as i64;
        let size = v.get_size();
        let member = o.code() == OpCode::CPUI_COPY
            && off >= 0
            && off + size as i64 <= str_len as i64
            && match readonly_source(data, def) {
                Some(src) => {
                    let sv = data.vbank().get(src)?;
                    sv.get_size() == size
                        && sv.get_offset() == str_addr.get_offset().wrapping_add(off as u64)
                        && Rc::ptr_eq(sv.get_addr().get_space()?, str_addr.get_space()?)
                }
                None => false,
            };
        if !member {
            foreign.push(order);
            continue;
        }
        out.push(Slice { off: off as int4, size, op: def, order });
    }
    Some(out)
}

/// Do the collected COPYs tile `[0, str_len)` exactly — no gap, no overlap —
/// with at least two of them?
fn covers_exactly(slices: &[Slice], str_len: int4) -> bool {
    if slices.len() < 2 {
        return false;
    }
    let mut sorted: Vec<(int4, int4)> = slices.iter().map(|s| (s.off, s.size)).collect();
    sorted.sort_unstable();
    let mut cursor: int4 = 0;
    for (off, size) in sorted {
        if off != cursor {
            return false;
        }
        cursor += size;
    }
    cursor == str_len
}

/// No interfering LOAD/STORE/CALL between consecutive COPYs of the run, in
/// block order (`ArraySequence::interfereBetween`, the window `RuleStringCopy`
/// demands of its own run).
fn no_interference(data: &Funcdata, slices: &[Slice]) -> bool {
    let mut ordered: Vec<(u32, OpId)> = slices.iter().map(|s| (s.order, s.op)).collect();
    ordered.sort_unstable_by_key(|(order, _)| *order);
    ordered
        .windows(2)
        .all(|w| ArraySequence::interfere_between(data, w[0].1, w[1].1))
}

/// Read `len` bytes of the load image at `addr` (C++ `loader->loadFill`).
fn read_image(data: &Funcdata, addr: &Address, len: int4) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; len.max(0) as usize];
    data.get_arch().loader_fill(&mut buf, addr).ok()?;
    Some(buf)
}

/// (kuna `rodatastring`) How read-only string block-copy recovery is toggled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RodataStringForm {
    /// `on`: collapse the block copy into `builtin_strncpy`.
    On,
    /// `off`: leave the partial-symbol slice assignments.
    Off,
}

impl RodataStringForm {
    /// The resolved `glb->rodata_string` flag for this form.
    pub fn rodata_string(self) -> bool {
        matches!(self, RodataStringForm::On)
    }
}

/// Parse the `option rodatastring on|off` argument and produce the resolved
/// form plus the confirmation message.
pub fn parse_rodata_string_form(p1: &str) -> KunaResult<(RodataStringForm, String)> {
    let val = crate::options::on_or_off(p1)
        .map_err(|_: KunaError| KunaError::parse("Must specify toggle value, on/off"))?;
    let form = if val { RodataStringForm::On } else { RodataStringForm::Off };
    let prop = if val { "on" } else { "off" };
    Ok((form, format!("Read-only string block-copy recovery turned {prop}")))
}

/// Per-file registration rows (one rule).  Shipped default:
/// `option rodatastring on`.
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        ctor: || Box::new(RuleRodataStringCopy::with_group(true, "analysis")),
    }]
}

#[cfg(test)]
#[path = "kuna_rodatastring/tests.rs"]
mod tests;
