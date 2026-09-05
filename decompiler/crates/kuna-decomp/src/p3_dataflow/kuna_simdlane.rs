//! (kuna) `simdlane` — resolve a byte-lane read of a CONSTANT-mask `pshufb`
//! shuffle to the source lane it actually selects.
//!
//! # The symptom
//!
//! A macOS crackme builds two consecutive character ranges with an SSE loop.
//! The compiler broadcasts one byte across an XMM register with the standard
//! `pxor xmm2,xmm2; pshufb xmm0,xmm2` idiom, and kuna renders the result as a
//! wall of lane temporaries:
//!
//! ```text
//! v7 = pshufb(ZEXT116(dat_100008040),0);
//! v6  = SUB161(v7,0);
//! v26 = SUB161(v7,1);
//! v28 = SUB161(v7,2);
//! ...                      // 16 per register, 32 in the function
//! *(char *)(v4 + 0x100008070) = v6  + v10;
//! *(char *)(v4 + 0x100008071) = v26 + v11;
//! ```
//!
//! Thirty-two `SUB161` temporaries, thirty of them declared only to be read
//! once, all naming the SAME byte. An agent reading this has to simulate SSE
//! byte lanes to find out that `v6`, `v26`, `v28` … are one value.
//!
//! # Why they are not folded
//!
//! `pshufb` has no p-code semantics: the x86 SLEIGH spec models it as a
//! `CALLOTHER` user op, an opaque 16-byte result. `ActionLaneDivide` then splits
//! the vector consumers into byte lanes, and each lane read of the opaque result
//! becomes its own `SUBPIECE`. Nothing downstream can see through the
//! `CALLOTHER`, so `RuleSubExtComm` / `RuleSubZext` and the copy propagation that
//! would collapse them never fire — even though the shuffle mask is a compile-time
//! constant sitting right there in the operand.
//!
//! # The rule
//!
//! `pshufb dst, mask` is a pure byte permutation with a per-lane definition that
//! is exact when the mask is known:
//!
//! ```text
//! dst[i] = (mask[i] & 0x80) ? 0 : src[mask[i] & (N-1)]        N = 8 or 16
//! ```
//!
//! so a one-byte `SUBPIECE(pshufb(src,mask), i)` with a constant `mask` is
//! literally `SUBPIECE(src, mask[i] & (N-1))`, or the constant `0`. The rewrite
//! is an identity, not a heuristic: it does not guess, and it declines whenever
//! the mask is not a constant. Once every lane read is re-anchored on `src` the
//! `CALLOTHER` loses its last reader and the ordinary dead-code fixpoint removes
//! it, and the lanes of a broadcast (an all-zero mask — the `pxor` idiom) all
//! resolve to `SUBPIECE(src, 0)`, which the existing rules then collapse into one
//! value.
//!
//! # Bounds
//!
//! * Only a user op the architecture registered under a shuffle name is matched
//!   ([`SHUFFLE_USEROP_NAMES`]); the ids are resolved once per program in
//!   `Architecture::build_arch_handle` and carried on the
//!   [`ArchContext`](crate::context::ArchContext), so the rule never sees a
//!   `CALLOTHER` it cannot name.
//! * Only the three-input form (`userop-id, src, mask`) whose two operands and
//!   whose output all have the vector width, and only widths 8 (MMX) and 16
//!   (SSE), are matched.
//! * Only a ONE-BYTE lane read is rewritten. A wider `SUBPIECE` of a shuffle is
//!   a concatenation of lanes, not another `SUBPIECE`, so it is declined rather
//!   than approximated.
//! * A mask wider than 8 bytes cannot be decoded from a `uintb` offset, so it is
//!   accepted only at value `0` — where the offset IS the whole value and every
//!   lane byte is provably zero. That is the broadcast idiom, and it is the only
//!   wide constant mask the engine can construct.
//!
//! Gated by the arch flag `simd_lane_fold` (option `simdlane on|off`, shipped
//! default `on`); with the option off the rule is inert and the output is
//! byte-identical to upstream.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::context::{OpId, TypeOp, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// SLEIGH user-op names whose semantics are "permute the bytes of operand 1 by
/// the byte mask in operand 2" — the x86 `PSHUFB` family. The AVX spelling is
/// carried because the same three-input CALLOTHER shape is emitted for it.
pub const SHUFFLE_USEROP_NAMES: &[&[u8]] = &[b"pshufb", b"vpshufb"];

/// Marshalling element id for the option (kuna 4000+ range).
pub const ELEM_SIMDLANE: ElementId = ElementId::new("simdlane", 4142);

/// The `simdlane on|off` option (`Architecture::simd_lane_fold`).
pub struct OptionSimdLane;

impl OptionSimdLane {
    /// The option name.
    pub const NAME: &'static str = "simdlane";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::simd_lane_fold`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Constant-mask SIMD lane resolution turned {prop}")))
    }
}

/// Which source byte lane does `mask` select for output lane `k` of an
/// `n`-byte shuffle?
///
/// `Some(None)` is the zeroing case (`mask[k]` has bit 7 set); `Some(Some(j))`
/// selects source lane `j`; `None` means the mask byte is not decodable and the
/// caller must decline.
///
/// `mask_value` is the constant Varnode's offset, which holds the whole value
/// only up to eight bytes. A wider mask is decodable only at `0`, the broadcast
/// mask the `pxor` idiom produces.
pub fn shuffle_lane_source(
    mask_value: uintb,
    mask_size: int4,
    n: int4,
    k: int4,
) -> Option<Option<int4>> {
    if k < 0 || k >= n {
        return None;
    }
    let byte: u8 = if mask_size <= 8 {
        ((mask_value >> (8 * k as u32)) & 0xff) as u8
    } else if mask_value == 0 {
        0
    } else {
        return None;
    };
    if byte & 0x80 != 0 {
        return Some(None);
    }
    Some(Some((byte as int4) & (n - 1)))
}

/// (kuna) Resolve `SUBPIECE(pshufb(src, <const mask>), k)` to the source lane the
/// mask selects.
pub struct RuleSimdShuffleLane {
    /// Unit-test override for the `Architecture::simd_lane_fold` gate; the live
    /// read is `data.get_arch().simd_lane_fold` (the ArchSeam, see module docs).
    enabled: bool,
    /// Rule group.
    group: String,
}

impl RuleSimdShuffleLane {
    /// Construct with the resolved gate.
    pub fn new(enabled: bool) -> RuleSimdShuffleLane {
        RuleSimdShuffleLane { enabled, group: String::from("analysis") }
    }

    /// Construct with an explicit group string.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleSimdShuffleLane {
        RuleSimdShuffleLane { enabled, group: group.into() }
    }
}

impl Rule for RuleSimdShuffleLane {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSimdShuffleLane { enabled: self.enabled, group: self.group.clone() }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !data.get_arch().simd_lane_fold {
            return 0;
        }
        if data.get_arch().simd_shuffle_userops.is_empty() {
            return 0;
        }

        // The lane read must be a one-byte extraction at a constant offset.
        let offvn = match op_in(data, op, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, offvn) {
            return 0;
        }
        let k = vn_offset(data, offvn) as int4;
        if op_out_size(data, op) != 1 {
            return 0;
        }

        // ... of a CALLOTHER the architecture named as a byte shuffle.
        let shufvn = match op_in(data, op, 0) {
            Some(v) => v,
            None => return 0,
        };
        let shuf = match vn_def(data, shufvn) {
            Some(d) => d,
            None => return 0,
        };
        if op_code(data, shuf) != OpCode::CPUI_CALLOTHER || op_num_input(data, shuf) != 3 {
            return 0;
        }
        let idvn = match op_in(data, shuf, 0) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, idvn) {
            return 0;
        }
        let userop_id = vn_offset(data, idvn) as uint4;
        if !data.get_arch().simd_shuffle_userops.contains(&userop_id) {
            return 0;
        }

        let srcvn = match op_in(data, shuf, 1) {
            Some(v) => v,
            None => return 0,
        };
        let maskvn = match op_in(data, shuf, 2) {
            Some(v) => v,
            None => return 0,
        };
        let n = vn_size(data, srcvn);
        if (n != 8 && n != 16) || vn_size(data, maskvn) != n || vn_size(data, shufvn) != n {
            return 0;
        }
        if !vn_is_constant(data, maskvn) {
            return 0;
        }

        let sel = match shuffle_lane_source(vn_offset(data, maskvn), vn_size(data, maskvn), n, k) {
            Some(s) => s,
            None => return 0,
        };

        match sel {
            // `mask[k]` has bit 7 set: the output lane is a hard zero.
            None => {
                let zero = data.new_constant(1, 0);
                data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
                data.op_set_input(op, zero, 0).expect("RuleSimdShuffleLane: opSetInput");
                data.op_remove_input(op, 1);
            }
            // The output lane IS source lane `j`.
            Some(j) => {
                data.op_set_input(op, srcvn, 0).expect("RuleSimdShuffleLane: opSetInput");
                let c = data.new_constant(vn_size(data, offvn), j as uintb);
                data.op_set_input(op, c, 1).expect("RuleSimdShuffleLane: opSetInput");
            }
        }
        1
    }
}

/// Per-file registration rows.
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        // Shipped default: `option simdlane on`.
        ctor: || Box::new(RuleSimdShuffleLane::with_group(true, "analysis")),
    }]
}

// ---------------------------------------------------------------------------
// Local IR read helpers (the `kuna_booleanmask.rs` access pattern).
// ---------------------------------------------------------------------------

fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}

fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}

fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).map(|o| o.num_input()).unwrap_or(0)
}

fn op_out_size(data: &Funcdata, op: OpId) -> int4 {
    data.obank()
        .get(op)
        .and_then(|o| o.get_out())
        .and_then(|v| data.vbank().get(v).map(|vn| vn.get_size()))
        .unwrap_or(0)
}

fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}

fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).filter(|v| v.is_written()).and_then(|v| v.get_def())
}

fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}

fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

#[cfg(test)]
#[path = "kuna_simdlane/tests.rs"]
mod tests;
