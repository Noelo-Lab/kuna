//! (kuna `mulblob`) Keep a widened multiply operand a value, not an aggregate.
//!
//! x86-64 `MUL r64` lowers (SLEIGH `ia.sinc:4128`) to
//! `tmp:16 = zext(RAX) * zext(rm64); RDX = tmp(8); RAX = tmp(0)`.  A 16-byte
//! varnode has no primitive type, so `TypeFactory::get_base` hands it the size
//! fallback `undefined1[16]`, an array — which `Datatype::is_piece_structured`
//! accepts.  `RulePieceStructure` therefore rewrites each `INT_ZEXT` into a
//! `PIECE` rooted at a partial-root varnode, and P6 gives that root its own
//! declaration.  The result is two 16-byte locals per multiply site whose whole
//! content is the operand:
//!
//! ```text
//!   v8._8_8_ = 0; v8._0_8_ = v29; v18._8_8_ = 0; v18._0_8_ = v27;
//!   v26 = SUB168(v8._0_16_ * v18._0_16_,0);
//! ```
//!
//! The signed sibling (`IMUL`, SLEIGH `sext`) is never rewritten — `INT_SEXT` is
//! not in `RulePieceStructure`'s op list — and already prints as
//! `SUB168(SEXT816(x) * SEXT816(y),8)`.  This module declines the zero-extension
//! rewrite for the same shape, so the unsigned form prints the same way.
//!
//! The decline is deliberately narrow.  It fires only when the extension's
//! output is an anonymous temporary whose data-type is the *size fallback*
//! (an array of `undefined1` exactly as wide as the varnode), and whose only
//! readers are wide integer arithmetic ops that are themselves read only through
//! `SUBPIECE`.  A varnode backed by a Symbol, an addr-tied one, or one whose
//! array type came from real type recovery keeps upstream's behaviour.
//!
//! Gated by `Architecture::mul_blob` (option `mulblob on|off`).

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// The widest extension output this module will leave un-structured.  A
/// 64x64->128 product is the whole motivating corpus; the cap keeps a large
/// genuine buffer out of scope.
const MAX_BLOB_SIZE: int4 = 16;

/// True when `ct` is the anonymous size fallback `TypeFactory::get_base(sz,
/// TYPE_UNKNOWN)` hands out for a width with no primitive: an array of `sz`
/// one-byte `undefined` elements.
fn is_size_fallback_array(ct: &Datatype, sz: int4) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_ARRAY || ct.get_size() != sz {
        return false;
    }
    if ct.num_elements() != Some(sz) {
        return false;
    }
    match ct.get_array_base() {
        Some(elem) => {
            elem.get_size() == 1 && elem.get_metatype() == type_metatype::TYPE_UNKNOWN
        }
        None => false,
    }
}

/// The wide integer operations whose operands this module keeps as values.
fn is_wide_arithmetic(opc: OpCode) -> bool {
    matches!(
        opc,
        OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_XOR
    )
}

/// True when every reader of `vn` takes it apart with `SUBPIECE` — i.e. the wide
/// value is consumed as a number and never as storage.
fn read_only_by_subpiece(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let mut any = false;
    for op in v.descend_iter() {
        any = true;
        let Some(o) = data.obank().get(op) else {
            return false;
        };
        if o.code() != OpCode::CPUI_SUBPIECE {
            return false;
        }
    }
    any
}

/// True when the `INT_ZEXT` at `op` is the widening of a multiply-style operand
/// that should stay a value.  Called from `RulePieceStructure` before it converts
/// the extension into a `PIECE`.
pub fn declines_zext(data: &Funcdata, op: OpId) -> bool {
    if !data.get_arch().mul_blob {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if o.code() != OpCode::CPUI_INT_ZEXT {
        return false;
    }
    let Some(outvn) = o.get_out() else {
        return false;
    };
    let Some(ov) = data.vbank().get(outvn) else {
        return false;
    };
    // Real storage keeps upstream's structuring: only an anonymous temporary is
    // in scope.
    if ov.is_addr_tied() || ov.is_persist() || ov.is_mapped() || ov.is_proto_partial() {
        return false;
    }
    if ov.get_space().get_type() != kuna_base::space::spacetype::IPTR_INTERNAL {
        return false;
    }
    let sz = ov.get_size();
    if sz <= 8 || sz > MAX_BLOB_SIZE {
        return false;
    }
    if !is_size_fallback_array(ov.get_type(), sz) {
        return false;
    }
    // A Symbol covering the output would have given `determineDatatype` a real
    // structured type; decline to interfere with that path.
    if data
        .get_scope_local()
        .and_then(|lm| lm.query_container_for_link(ov.get_addr(), &data.vn_use_point(outvn)))
        .is_some()
    {
        return false;
    }
    // Every reader must be wide arithmetic that is itself only taken apart.
    let mut any = false;
    for descend in ov.descend_iter() {
        any = true;
        let Some(d) = data.obank().get(descend) else {
            return false;
        };
        if !is_wide_arithmetic(d.code()) {
            return false;
        }
        let Some(dout) = d.get_out() else {
            return false;
        };
        if data.vbank().get(dout).map(|v| v.get_size()) != Some(sz) {
            return false;
        }
        if !read_only_by_subpiece(data, dout) {
            return false;
        }
    }
    any
}

/// `option mulblob on|off` (kuna).
pub struct OptionMulBlob;

impl OptionMulBlob {
    /// The option name.
    pub const NAME: &'static str = "mulblob";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::mul_blob`.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(bool, String)> {
        let val = crate::p0_knowledge::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Wide-multiply operand blobs turned {prop}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    fn undefined_array(elems: int4) -> Datatype {
        let elem = Datatype::new(1, type_metatype::TYPE_UNKNOWN);
        let mut arr = Datatype::new(elems, type_metatype::TYPE_ARRAY);
        arr.kind = crate::dtype::DatatypeKind::Array { arrayof: Rc::new(elem), arraysize: elems };
        arr
    }

    /// The width fallback `TypeFactory::get_base(16, TYPE_UNKNOWN)` hands out is
    /// the only array shape this module declines for.
    #[test]
    fn size_fallback_array_is_the_only_accepted_shape() {
        assert!(is_size_fallback_array(&undefined_array(16), 16));
        // right shape, wrong width for the Varnode
        assert!(!is_size_fallback_array(&undefined_array(16), 8));
        // a recovered array of a real element type is not the fallback
        let elem = Datatype::new(1, type_metatype::TYPE_INT);
        let mut arr = Datatype::new(16, type_metatype::TYPE_ARRAY);
        arr.kind = crate::dtype::DatatypeKind::Array { arrayof: Rc::new(elem), arraysize: 16 };
        assert!(!is_size_fallback_array(&arr, 16));
        // a struct is piece-structured but is not an array
        assert!(!is_size_fallback_array(&Datatype::new(16, type_metatype::TYPE_STRUCT), 16));
    }

    /// Only integer arithmetic consumes a widened operand as a value; a LOAD,
    /// a STORE or a CALL reading it means it is storage.
    #[test]
    fn wide_arithmetic_set_excludes_memory_and_calls() {
        assert!(is_wide_arithmetic(OpCode::CPUI_INT_MULT));
        assert!(is_wide_arithmetic(OpCode::CPUI_INT_ADD));
        assert!(is_wide_arithmetic(OpCode::CPUI_INT_OR));
        assert!(!is_wide_arithmetic(OpCode::CPUI_LOAD));
        assert!(!is_wide_arithmetic(OpCode::CPUI_STORE));
        assert!(!is_wide_arithmetic(OpCode::CPUI_CALL));
        assert!(!is_wide_arithmetic(OpCode::CPUI_PIECE));
        assert!(!is_wide_arithmetic(OpCode::CPUI_SUBPIECE));
    }
}
