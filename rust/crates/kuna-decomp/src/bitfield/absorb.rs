//! Port of the two post-materialization bitfield simplification rules from
//! `decompiler/cpp/bitfield.cc`:
//!
//! * `RulePullAbsorb` (`bitfield.cc:1767-2208`) — simplifies expressions reading
//!   a materialized `CPUI_ZPULL`/`CPUI_SPULL` (the `absorbRight`/`absorbLeft`/
//!   `absorbAnd`/`absorbCompare`/`absorbExt`/`absorbSubpiece`/`absorbCompZero`
//!   family).
//! * `RuleInsertAbsorb` (`bitfield.cc:2214-2400`) — simplifies the *value* fed to
//!   a materialized `CPUI_INSERT` (the `leftShiftVarnode`/`absorbAnd`/
//!   `absorbRightLeft`/`absorbShiftAdd`/`absorbNestedAnd` family) so a container
//!   shared by several bitfields consolidates into one INSERT per field and
//!   renders as separate `ptr->field = ...` statements.
//!
//! # Faithfulness
//!
//! Every function is a line-for-line transcription of the C++ named in its doc
//! comment (file:line cited).  The rule-apply *order* (the descendant traversal
//! in `RulePullAbsorb::applyOp` / the def-op switch in `RuleInsertAbsorb::applyOp`)
//! and the per-helper traversal order are **semantics** — `newConstant` bumps the
//! function create-index, so the exact sequence is observable in `print C` text.
//! All field offsets / widths / masks come from the live op const inputs (the
//! pulls' `in(1)`/`in(2)`, the INSERT's `in(2)`/`in(3)`) and from
//! [`super::expression::InsertExpression`] / [`super::expression::get_pull_field`]
//! (which read the `TypeStruct` bitfields); nothing is hardcoded and no field
//! name / mask value / offset is special-cased.
//!
//! The C++ `Varnode *`/`PcodeOp *` graph is, per ADR 0001, a [`Funcdata`]-owned
//! pair of arenas reached through the banks; each helper threads `data` and the
//! op/varnode ids explicitly, exactly mirroring the C++ pointer walk.

use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use super::expression::{get_pull_field, InsertExpression};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

/// `sizeof(uintb)*8` — the C++ `uintb` is `uint8`, i.e. 64 bits.
const UINTB_BITS: int4 = 64;

// =============================================================================
// small Varnode/PcodeOp accessor shims (the C++ `vn->...`/`op->...` getters)
// =============================================================================

/// `op->getIn(slot)` — `None` if the op is stale or the slot is empty.
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op)?.get_in(slot)
}

/// `op->getOut()` — `None` if the op is stale or has no output.
fn op_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op)?.get_out()
}

/// `op->code()`.
fn op_code(data: &Funcdata, op: OpId) -> Option<OpCode> {
    data.obank().get(op).map(|o| o.code())
}

/// `vn->isConstant()`.
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}

/// `vn->getOffset()`.
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}

/// `vn->getSize()`.
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

/// `vn->constantMatch(val)`.
fn vn_constant_match(data: &Funcdata, vn: VarnodeId, val: uintb) -> bool {
    data.vbank().get(vn).map(|v| v.constant_match(val)).unwrap_or(false)
}

/// `vn->isWritten()`.
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
}

/// `vn->getDef()`.
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).and_then(|v| v.get_def())
}

/// The descendant ops of a Varnode's output (C++ `for(iter=outvn->beginDescend();
/// iter!=outvn->endDescend();++iter)`), snapshotted into a `Vec` so the walk is
/// stable across the in-loop graph mutation (the C++ returns on the first
/// successful absorb, so a snapshot is exactly equivalent).
fn descend_of(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    match data.vbank().get(vn) {
        Some(v) => v.descend_iter().collect(),
        None => Vec::new(),
    }
}

/// Copy the data-type of `src_vn` onto the (newly created) `dst_vn` — the C++
/// `newVn->updateType(srcVn->getType())` idiom.
fn copy_type(data: &mut Funcdata, dst_vn: VarnodeId, src_vn: VarnodeId) {
    let ty = data.vbank().get(src_vn).map(|v| Rc::clone(v.get_type()));
    if let Some(ty) = ty {
        if let Some(v) = data.vbank_mut().get_mut(dst_vn) {
            v.update_type(ty);
        }
    }
}

// =============================================================================
// RulePullAbsorb (bitfield.cc:1767-2208)
// =============================================================================

/// `RulePullAbsorb::absorbRight` (bitfield.cc:1767-1780): walk the descendants of
/// `field >> #c` looking for an `INT_AND` to fold a sign-bit comparison.
fn pull_absorb_right(data: &mut Funcdata, right_op: OpId, pull_op: OpId) -> int4 {
    let outvn = match op_out(data, right_op) {
        Some(v) => v,
        None => return 0,
    };
    for read_op in descend_of(data, outvn) {
        if op_code(data, read_op) == Some(OpCode::CPUI_INT_AND) {
            let res = pull_absorb_right_and_comp_zero(data, right_op, read_op, pull_op);
            if res != 0 {
                return res;
            }
        }
    }
    0
}

/// `RulePullAbsorb::absorbRightAndCompZero` (bitfield.cc:1790-1822):
/// `((sfield >> #n) & #1) == #0  =>  #0 <= sfield` (and the `!=` variant
/// `=> sfield < #0`).
fn pull_absorb_right_and_comp_zero(
    data: &mut Funcdata,
    right_op: OpId,
    and_op: OpId,
    pull_op: OpId,
) -> int4 {
    if op_code(data, pull_op) != Some(OpCode::CPUI_SPULL) {
        return 0;
    }
    let cvn = match op_in(data, right_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, cvn) {
        return 0;
    }
    let sa = vn_offset(data, cvn) as int4;
    let numbits = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    if numbits - 1 != sa {
        return 0; // Check that shift puts sign bit into least sig position
    }
    let and_in1 = match op_in(data, and_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_constant_match(data, and_in1, 1) {
        return 0;
    }
    let outvn = match op_out(data, and_op) {
        Some(v) => v,
        None => return 0,
    };
    for read_op in descend_of(data, outvn) {
        let opc = match op_code(data, read_op) {
            Some(c) => c,
            None => continue,
        };
        if opc != OpCode::CPUI_INT_EQUAL && opc != OpCode::CPUI_INT_NOTEQUAL {
            continue;
        }
        let read_in1 = match op_in(data, read_op, 1) {
            Some(v) => v,
            None => continue,
        };
        if !vn_constant_match(data, read_in1, 0) {
            continue;
        }
        let vn = match op_out(data, pull_op) {
            Some(v) => v,
            None => return 0,
        };
        if opc == OpCode::CPUI_INT_EQUAL {
            data.op_set_opcode_code(read_op, OpCode::CPUI_INT_LESSEQUAL);
            let zvn = match op_in(data, read_op, 1) {
                Some(v) => v,
                None => return 0,
            };
            data.op_set_input(read_op, vn, 1).ok();
            data.op_set_input(read_op, zvn, 0).ok();
        } else {
            data.op_set_opcode_code(read_op, OpCode::CPUI_INT_SLESS);
            data.op_set_input(read_op, vn, 0).ok();
        }
        data.destroy_varnode_recursive(outvn).ok();
        return 1;
    }
    0
}

/// `RulePullAbsorb::absorbLeft` (bitfield.cc:1830-1848): dispatch over the
/// descendants of `field << #c`.
fn pull_absorb_left(data: &mut Funcdata, left_op: OpId, pull_op: OpId) -> int4 {
    let outvn = match op_out(data, left_op) {
        Some(v) => v,
        None => return 0,
    };
    for read_op in descend_of(data, outvn) {
        let opc = match op_code(data, read_op) {
            Some(c) => c,
            None => continue,
        };
        let res = if opc == OpCode::CPUI_INT_SLESS {
            pull_absorb_compare(data, read_op, Some(left_op), pull_op)
        } else if opc == OpCode::CPUI_INT_RIGHT {
            pull_absorb_left_right(data, read_op, left_op, pull_op)
        } else if opc == OpCode::CPUI_INT_AND {
            pull_absorb_left_and(data, read_op, left_op, pull_op)
        } else {
            0
        };
        if res != 0 {
            return res;
        }
    }
    0
}

/// `RulePullAbsorb::absorbLeftRight` (bitfield.cc:1857-1887):
/// `(field << #c) >> #d  =>  field >> (#d-#c)`.
fn pull_absorb_left_right(data: &mut Funcdata, right_op: OpId, left_op: OpId, pull_op: OpId) -> int4 {
    let leftcvn = match op_in(data, left_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, leftcvn) {
        return 0;
    }
    let rightcvn = match op_in(data, right_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, rightcvn) {
        return 0;
    }
    let bitsize = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    let invn = match op_in(data, pull_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    let container_size = vn_size(data, invn) * 8;
    let leftshift = vn_offset(data, leftcvn) as int4;
    let rightshift = vn_offset(data, rightcvn) as int4;
    if leftshift + bitsize > container_size {
        return 0; // Check if left shift destroys field data
    }
    let sa = rightshift - leftshift;
    let rightcvn_size = vn_size(data, rightcvn);
    let pull_out = match op_out(data, pull_op) {
        Some(v) => v,
        None => return 0,
    };
    let right_out = match op_out(data, right_op) {
        Some(v) => v,
        None => return 0,
    };
    let left_out = match op_out(data, left_op) {
        Some(v) => v,
        None => return 0,
    };
    if sa == 0 {
        data.total_replace(right_out, pull_out).ok();
        data.destroy_varnode_recursive(right_out).ok();
    } else if sa > 0 {
        // Right shift is bigger than left
        let c = data.new_constant(rightcvn_size, sa as uintb);
        data.op_set_input(right_op, c, 1).ok();
        data.op_set_input(right_op, pull_out, 0).ok();
        data.destroy_varnode_recursive(left_out).ok();
    } else {
        // Left shift is bigger than right
        data.op_set_opcode_code(right_op, OpCode::CPUI_INT_LEFT);
        let c = data.new_constant(rightcvn_size, (-sa) as uintb);
        data.op_set_input(right_op, c, 1).ok();
        data.op_set_input(right_op, pull_out, 0).ok();
        data.destroy_varnode_recursive(left_out).ok();
    }
    1
}

/// `RulePullAbsorb::absorbLeftAnd` (bitfield.cc:1896-1931):
/// `((field << #c) & #b) == #d  =>  (field & #b>>c) == #d>>c`.
fn pull_absorb_left_and(data: &mut Funcdata, and_op: OpId, left_op: OpId, _pull_op: OpId) -> int4 {
    let shift_amount = match op_in(data, left_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, shift_amount) {
        return 0;
    }
    let sa = vn_offset(data, shift_amount) as int4;
    // C++: `if (sa < 0 || sa >= sizeof(uintb)*8) return 0;`
    if !(0..UINTB_BITS).contains(&sa) {
        return 0;
    }
    let mask_vn = match op_in(data, and_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, mask_vn) {
        return 0;
    }
    let mask0 = vn_offset(data, mask_vn);
    let mask_vn_size = vn_size(data, mask_vn);
    let outvn = match op_out(data, and_op) {
        Some(v) => v,
        None => return 0,
    };
    for read_op in descend_of(data, outvn) {
        let opc = match op_code(data, read_op) {
            Some(c) => c,
            None => continue,
        };
        if opc == OpCode::CPUI_INT_EQUAL || opc == OpCode::CPUI_INT_NOTEQUAL {
            let comp_val = match op_in(data, read_op, 1) {
                Some(v) => v,
                None => continue,
            };
            if !vn_is_constant(data, comp_val) {
                continue;
            }
            let comp_off = vn_offset(data, comp_val);
            let val = comp_off >> sa;
            if val << sa != comp_off {
                continue;
            }
            let mask = mask0 >> sa;
            let new_and = data.new_constant(mask_vn_size, mask);
            copy_type(data, new_and, mask_vn);
            data.op_set_input(and_op, new_and, 1).ok();
            if val != comp_off {
                let comp_size = vn_size(data, comp_val);
                let new_val = data.new_constant(comp_size, val);
                copy_type(data, new_val, comp_val);
                data.op_set_input(read_op, new_val, 1).ok();
            }
            let left_in0 = match op_in(data, left_op, 0) {
                Some(v) => v,
                None => return 0,
            };
            data.op_set_input(and_op, left_in0, 0).ok();
            let left_out = match op_out(data, left_op) {
                Some(v) => v,
                None => return 0,
            };
            data.destroy_varnode_recursive(left_out).ok();
            return 1;
        }
    }
    0
}

/// `RulePullAbsorb::absorbAnd` (bitfield.cc:1939-1975):
/// `field & #signbit == #0  =>  field < 0`.
fn pull_absorb_and(data: &mut Funcdata, and_op: OpId, pull_op: OpId) -> int4 {
    let mask_vn = match op_in(data, and_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, mask_vn) {
        return 0;
    }
    let vn = match op_out(data, pull_op) {
        Some(v) => v,
        None => return 0,
    };
    if op_code(data, pull_op) != Some(OpCode::CPUI_SPULL) {
        return 0; // Not signed
    }
    let bitsize = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    let mut match_val: uintb = 1;
    match_val <<= bitsize - 1; // Mask for sign-bit
    if match_val != vn_offset(data, mask_vn) {
        return 0;
    }
    let outvn = match op_out(data, and_op) {
        Some(v) => v,
        None => return 0,
    };
    let vn_sz = vn_size(data, vn);
    for read_op in descend_of(data, outvn) {
        let opc = match op_code(data, read_op) {
            Some(c) => c,
            None => continue,
        };
        if opc == OpCode::CPUI_INT_EQUAL || opc == OpCode::CPUI_INT_NOTEQUAL {
            let read_in1 = match op_in(data, read_op, 1) {
                Some(v) => v,
                None => continue,
            };
            if !vn_constant_match(data, read_in1, 0) {
                continue;
            }
            let new_zero = data.new_constant(vn_sz, 0);
            // dt = types->resizeInteger(vn->getType(), vn->getSize());
            let vn_ty = data.vbank().get(vn).map(|v| Rc::clone(v.get_type()));
            let arch = Rc::clone(data.get_arch());
            if let (Some(types), Some(vn_ty)) = (arch.types(), vn_ty) {
                if let Ok(dt) = types.resize_integer(vn_ty, vn_sz) {
                    if let Some(v) = data.vbank_mut().get_mut(new_zero) {
                        v.update_type(dt);
                    }
                }
            }
            if opc == OpCode::CPUI_INT_EQUAL {
                data.op_set_opcode_code(read_op, OpCode::CPUI_INT_SLESSEQUAL);
                data.op_set_input(read_op, new_zero, 0).ok();
                data.op_set_input(read_op, vn, 1).ok();
            } else {
                data.op_set_opcode_code(read_op, OpCode::CPUI_INT_SLESS);
                data.op_set_input(read_op, vn, 0).ok();
                data.op_set_input(read_op, new_zero, 1).ok();
            }
            data.destroy_varnode_recursive(outvn).ok();
            return 1;
        }
    }
    0
}

/// `RulePullAbsorb::absorbCompare` (bitfield.cc:1990-2060): the comparison family
/// (`INT_LESS`/`INT_SLESS`), optionally through an `INT_LEFT` (`left_op`).
fn pull_absorb_compare(
    data: &mut Funcdata,
    comp_op: OpId,
    left_op: Option<OpId>,
    pull_op: OpId,
) -> int4 {
    let mut sa: int4 = 0;
    if let Some(lop) = left_op {
        let cvn = match op_in(data, lop, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, cvn) {
            return 0;
        }
        sa = vn_offset(data, cvn) as int4;
    }
    let numbits = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    let invn = match op_in(data, pull_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    let sz = vn_size(data, invn) * 8;
    if numbits + sa != sz {
        return 0; // Verify that high bit of field is shifted into sign-bit
    }
    let pull_out = match op_out(data, pull_op) {
        Some(v) => v,
        None => return 0,
    };
    let in_vn = match left_op {
        None => pull_out,
        Some(lop) => match op_out(data, lop) {
            Some(v) => v,
            None => return 0,
        },
    };
    let less_vn0 = match op_in(data, comp_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    let less_vn1 = match op_in(data, comp_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    let in_vn_size = vn_size(data, in_vn);
    if op_code(data, comp_op) == Some(OpCode::CPUI_INT_SLESS) {
        if numbits == 1
            && less_vn0 == in_vn
            && vn_is_constant(data, less_vn1)
            && vn_offset(data, less_vn1) == 0
        {
            let old_vn = match op_out(data, comp_op) {
                Some(v) => v,
                None => return 0,
            };
            data.total_replace(old_vn, pull_out).ok();
            data.destroy_varnode_recursive(old_vn).ok();
            return 1;
        }
        if numbits == 1
            && less_vn1 == in_vn
            && vn_is_constant(data, less_vn0)
            && vn_offset(data, less_vn0) == calc_mask(in_vn_size)
        {
            data.op_remove_input(comp_op, 0);
            data.op_set_opcode_code(comp_op, OpCode::CPUI_BOOL_NEGATE);
            data.op_set_input(comp_op, pull_out, 0).ok();
            data.destroy_varnode_recursive(in_vn).ok();
            return 1;
        }
    }
    let mut mask: uintb = 1;
    mask = (mask << sa).wrapping_sub(1);
    if sa > 0 && sa < UINTB_BITS && in_vn == less_vn0 && vn_is_constant(data, less_vn1) {
        let orig_val = vn_offset(data, less_vn1);
        let low_bits = mask & orig_val;
        if low_bits == 0 || low_bits == 1 {
            let new_val = if low_bits == 1 {
                let nv = (orig_val.wrapping_sub(1)) >> sa; // Convert to constant for LESSEQUAL
                (nv.wrapping_add(1)) & calc_mask(in_vn_size) // Convert back to LESS after shift
            } else {
                orig_val >> sa
            };
            data.op_set_input(comp_op, pull_out, 0).ok();
            let c = data.new_constant(in_vn_size, new_val);
            data.op_set_input(comp_op, c, 1).ok();
            data.destroy_varnode_recursive(in_vn).ok();
            return 1;
        }
    }
    if sa > 0 && sa < UINTB_BITS && in_vn == less_vn1 && vn_is_constant(data, less_vn0) {
        let orig_val = vn_offset(data, less_vn0);
        let low_bits = mask & orig_val;
        if low_bits == 0 || low_bits == mask {
            let new_val = if low_bits == mask {
                let nv = (orig_val.wrapping_add(1)) >> sa; // Convert to constant for LESSEQUAL
                (nv.wrapping_sub(1)) & calc_mask(in_vn_size) // Convert back to LESS after shift
            } else {
                orig_val >> sa
            };
            data.op_set_input(comp_op, pull_out, 1).ok();
            let c = data.new_constant(in_vn_size, new_val);
            data.op_set_input(comp_op, c, 0).ok();
            data.destroy_varnode_recursive(in_vn).ok();
            return 1;
        }
    }
    0
}

/// `RulePullAbsorb::absorbExt` (bitfield.cc:2070-2086):
/// `y = SEXT(SPULL(...))  =>  y = SPULL(...)` (and the ZEXT/ZPULL variant).
fn pull_absorb_ext(data: &mut Funcdata, ext_op: OpId, pull_op: OpId) -> int4 {
    let pull_signed = op_code(data, pull_op) == Some(OpCode::CPUI_SPULL);
    let ext_signed = op_code(data, ext_op) == Some(OpCode::CPUI_INT_SEXT);
    if ext_signed != pull_signed {
        return 0;
    }
    let vn = match op_in(data, ext_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    if data.lone_descend(vn) != Some(ext_op) {
        return 0;
    }
    let pull_code = op_code(data, pull_op).unwrap();
    let pull_in0 = match op_in(data, pull_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    data.op_set_opcode_code(ext_op, pull_code);
    data.op_set_input(ext_op, pull_in0, 0).ok();
    let pos_vn = match op_in(data, pull_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    let num_vn = match op_in(data, pull_op, 2) {
        Some(v) => v,
        None => return 0,
    };
    data.op_insert_input(ext_op, pos_vn, 1).ok();
    data.op_insert_input(ext_op, num_vn, 2).ok();
    data.destroy_varnode_recursive(vn).ok();
    1
}

/// `RulePullAbsorb::absorbSubpiece` (bitfield.cc:2094-2111):
/// `y = SUB(PULL(...))  =>  y = PULL(...)`.
fn pull_absorb_subpiece(data: &mut Funcdata, sub_op: OpId, pull_op: OpId) -> int4 {
    let sub_in1 = match op_in(data, sub_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if vn_offset(data, sub_in1) != 0 {
        return 0;
    }
    let bitsize = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    let outvn = match op_out(data, sub_op) {
        Some(v) => v,
        None => return 0,
    };
    if bitsize > 8 * vn_size(data, outvn) {
        return 0;
    }
    let vn = match op_in(data, sub_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    if data.lone_descend(vn) != Some(sub_op) {
        return 0;
    }
    let pull_code = op_code(data, pull_op).unwrap();
    let pull_in0 = match op_in(data, pull_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    data.op_set_opcode_code(sub_op, pull_code);
    data.op_set_input(sub_op, pull_in0, 0).ok();
    let pos_vn = match op_in(data, pull_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    let num_vn = match op_in(data, pull_op, 2) {
        Some(v) => v,
        None => return 0,
    };
    data.op_set_input(sub_op, pos_vn, 1).ok();
    data.op_insert_input(sub_op, num_vn, 2).ok();
    data.destroy_varnode_recursive(vn).ok();
    1
}

/// `RulePullAbsorb::absorbCompZero` (bitfield.cc:2120-2159):
/// `ZPULL(x,#p,#1) != #0  =>  ZPULL(x,#p,#1)` (and the `== #0  =>  !ZPULL(...)`
/// variant).
fn pull_absorb_comp_zero(data: &mut Funcdata, comp_op: OpId, pull_op: OpId) -> int4 {
    let zvn = match op_in(data, comp_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_constant_match(data, zvn, 0) {
        return 0;
    }
    let bitsize = match op_in(data, pull_op, 2) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    if bitsize != 1 {
        return 0;
    }
    let vn = match op_in(data, comp_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    if data.lone_descend(vn) != Some(comp_op) {
        return 0;
    }
    if data.vbank().get(vn).map(|v| v.is_addr_tied()).unwrap_or(false) {
        return 0;
    }
    if op_code(data, pull_op) == Some(OpCode::CPUI_SPULL) {
        return 0;
    }
    // const TypeBitField *field = BitFieldExpression::getPullField(pullOp);
    // if (field == 0 || field->type->getMetatype() != TYPE_BOOL) return 0;
    match get_pull_field(data, pull_op) {
        Some(field) if field.field_type.get_metatype() == type_metatype::TYPE_BOOL => {}
        _ => return 0,
    }
    let comp_code = op_code(data, comp_op).unwrap();
    if comp_code == OpCode::CPUI_INT_EQUAL {
        if vn_size(data, vn) > 1 {
            // Build a 1-byte output at the (endian-adjusted) address of vn.
            let (addr, space_big) = {
                let v = data.vbank().get(vn).expect("absorbCompZero: stale vn");
                (v.get_addr().clone(), v.get_space().is_big_endian())
            };
            let vn_sz = vn_size(data, vn);
            let smalladdr = if space_big {
                &addr + ((vn_sz - 1) as i64)
            } else {
                addr
            };
            data.op_unset_output(pull_op);
            let new_vn = match data.new_varnode_out(1, &smalladdr, pull_op) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            // dt = types->getBase(1, TYPE_BOOL);
            let arch = Rc::clone(data.get_arch());
            if let Some(types) = arch.types() {
                if let Ok(dt) = types.get_base(1, type_metatype::TYPE_BOOL) {
                    if let Some(v) = data.vbank_mut().get_mut(new_vn) {
                        v.update_type(dt);
                    }
                }
            }
            data.op_set_input(comp_op, new_vn, 0).ok();
            data.delete_varnode(vn).ok();
        }
        data.op_set_opcode_code(comp_op, OpCode::CPUI_BOOL_NEGATE);
        data.op_remove_input(comp_op, 1);
    } else {
        let pull_code = op_code(data, pull_op).unwrap();
        data.op_set_opcode_code(comp_op, pull_code);
        let pull_in0 = match op_in(data, pull_op, 0) {
            Some(v) => v,
            None => return 0,
        };
        data.op_set_input(comp_op, pull_in0, 0).ok();
        let pos_vn = match op_in(data, pull_op, 1) {
            Some(v) => v,
            None => return 0,
        };
        let num_vn = match op_in(data, pull_op, 2) {
            Some(v) => v,
            None => return 0,
        };
        data.op_set_input(comp_op, pos_vn, 1).ok();
        data.op_insert_input(comp_op, num_vn, 2).ok();
        data.destroy_varnode_recursive(vn).ok();
    }
    1
}

/// `RulePullAbsorb::applyOp` (bitfield.cc:2168-2208): walk the descendants of the
/// `ZPULL`/`SPULL` output and dispatch into the absorb* helpers.
pub(super) fn pull_absorb_apply(op: OpId, data: &mut Funcdata) -> int4 {
    let outvn = match op_out(data, op) {
        Some(v) => v,
        None => return 0,
    };
    for read_op in descend_of(data, outvn) {
        let res = match op_code(data, read_op) {
            Some(OpCode::CPUI_INT_RIGHT) | Some(OpCode::CPUI_INT_SRIGHT) => {
                pull_absorb_right(data, read_op, op)
            }
            Some(OpCode::CPUI_INT_LEFT) => pull_absorb_left(data, read_op, op),
            Some(OpCode::CPUI_INT_AND) => pull_absorb_and(data, read_op, op),
            Some(OpCode::CPUI_INT_SLESS) | Some(OpCode::CPUI_INT_LESS) => {
                pull_absorb_compare(data, read_op, None, op)
            }
            Some(OpCode::CPUI_INT_ZEXT) | Some(OpCode::CPUI_INT_SEXT) => {
                pull_absorb_ext(data, read_op, op)
            }
            Some(OpCode::CPUI_SUBPIECE) => pull_absorb_subpiece(data, read_op, op),
            Some(OpCode::CPUI_INT_EQUAL) | Some(OpCode::CPUI_INT_NOTEQUAL) => {
                pull_absorb_comp_zero(data, read_op, op)
            }
            _ => 0,
        };
        if res != 0 {
            return res;
        }
    }
    0
}

// =============================================================================
// RuleInsertAbsorb (bitfield.cc:2214-2400)
// =============================================================================

/// `RuleInsertAbsorb::leftShiftVarnode` (bitfield.cc:2214-2233): if `vn` is
/// `x * (1<<sa)` (INT_MULT) or `x << sa` (INT_LEFT), return `x`; else `None`.
fn insert_left_shift_varnode(data: &Funcdata, vn: VarnodeId, sa: int4) -> Option<VarnodeId> {
    if !vn_is_written(data, vn) {
        return None;
    }
    let mult_op = vn_def(data, vn)?;
    let mult_val = op_in(data, mult_op, 1)?;
    if !vn_is_constant(data, mult_val) {
        return None;
    }
    let match_val: uintb = match op_code(data, mult_op) {
        Some(OpCode::CPUI_INT_MULT) => {
            let mut m: uintb = 1;
            m <<= sa;
            m
        }
        Some(OpCode::CPUI_INT_LEFT) => sa as uintb,
        _ => return None,
    };
    if vn_offset(data, mult_val) != match_val {
        return None;
    }
    op_in(data, mult_op, 0)
}

/// `RuleInsertAbsorb::absorbAnd` (bitfield.cc:2241-2252):
/// `INSERT(x & #mask, #p, #n)  =>  INSERT(x, #p, #n)`.
fn insert_absorb_and(data: &mut Funcdata, and_op: OpId, insert_op: OpId) -> int4 {
    let cvn = match op_in(data, and_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, cvn) {
        return 0;
    }
    let val = vn_offset(data, cvn);
    let mask = InsertExpression::get_lsb_mask(data, insert_op);
    if (mask & val) != mask {
        return 0; // AND mask must be of least significant bits that get INSERTed
    }
    let and_in0 = match op_in(data, and_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    data.op_set_input(insert_op, and_in0, 1).ok();
    let and_out = match op_out(data, and_op) {
        Some(v) => v,
        None => return 0,
    };
    data.destroy_varnode_recursive(and_out).ok();
    1
}

/// `RuleInsertAbsorb::absorbRightLeft` (bitfield.cc:2257-2286):
/// `INSERT((x << #c) >> #c, #p, #n)  =>  INSERT(x, #p, #n)` (and the
/// `SUB(x << #c, #0) >> #c` variant).
fn insert_absorb_right_left(
    data: &mut Funcdata,
    next_op: OpId,
    right_op: OpId,
    insert_op: OpId,
) -> int4 {
    let left_op = match op_code(data, next_op) {
        Some(OpCode::CPUI_INT_LEFT) => next_op,
        Some(OpCode::CPUI_SUBPIECE) => {
            let next_in1 = match op_in(data, next_op, 1) {
                Some(v) => v,
                None => return 0,
            };
            if vn_offset(data, next_in1) != 0 {
                return 0;
            }
            let subin = match op_in(data, next_op, 0) {
                Some(v) => v,
                None => return 0,
            };
            if !vn_is_written(data, subin) {
                return 0;
            }
            let lop = match vn_def(data, subin) {
                Some(o) => o,
                None => return 0,
            };
            if op_code(data, lop) != Some(OpCode::CPUI_INT_LEFT) {
                return 0;
            }
            lop
        }
        _ => return 0,
    };
    let lvn = match op_in(data, left_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, lvn) {
        return 0;
    }
    let rvn = match op_in(data, right_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_constant(data, rvn) {
        return 0;
    }
    let lsa = vn_offset(data, lvn) as int4;
    let rsa = vn_offset(data, rvn) as int4;
    if lsa != rsa {
        return 0;
    }
    let bitsize = match op_in(data, insert_op, 3) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    let insert_in1 = match op_in(data, insert_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if bitsize > vn_size(data, insert_in1) * 8 - lsa {
        // Shifts cancel unless bitsize exceeds number of bits preserved
        return 0;
    }
    let left_in0 = match op_in(data, left_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    data.op_set_input(insert_op, left_in0, 1).ok();
    let right_out = match op_out(data, right_op) {
        Some(v) => v,
        None => return 0,
    };
    data.destroy_varnode_recursive(right_out).ok();
    1
}

/// `RuleInsertAbsorb::absorbShiftAdd` (bitfield.cc:2295-2324):
/// `field = (a * #c + b * #c) >> #n  =>  field = a + b`.
fn insert_absorb_shift_add(
    data: &mut Funcdata,
    right_op: OpId,
    add_op: OpId,
    insert_op: OpId,
) -> int4 {
    let right_in1 = match op_in(data, right_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    let sa = vn_offset(data, right_in1) as int4;
    if sa <= 0 || sa >= UINTB_BITS {
        return 0;
    }
    let add_in0 = match op_in(data, add_op, 0) {
        Some(v) => v,
        None => return 0,
    };
    let vn0 = match insert_left_shift_varnode(data, add_in0, sa) {
        Some(v) => v,
        None => return 0,
    };
    let vn0_size = vn_size(data, vn0);
    let add_vn1 = match op_in(data, add_op, 1) {
        Some(v) => v,
        None => return 0,
    };
    let vn1: VarnodeId = if vn_is_constant(data, add_vn1) {
        let add_off = vn_offset(data, add_vn1);
        let add_val = add_off >> sa;
        if (add_val << sa) != add_off {
            return 0;
        }
        let c = data.new_constant(vn0_size, add_val);
        copy_type(data, c, add_vn1);
        c
    } else {
        match insert_left_shift_varnode(data, add_vn1, sa) {
            Some(v) => v,
            None => return 0,
        }
    };
    let bitsize = match op_in(data, insert_op, 3) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    if bitsize > vn0_size * 8 - sa {
        // Check that none of the carry bits make it into field
        return 0;
    }
    data.op_set_opcode_code(right_op, OpCode::CPUI_INT_ADD);
    data.op_set_input(right_op, vn0, 0).ok();
    data.op_set_input(right_op, vn1, 1).ok();
    let add_out = match op_out(data, add_op) {
        Some(v) => v,
        None => return 0,
    };
    data.destroy_varnode_recursive(add_out).ok();
    1
}

/// `RuleInsertAbsorb::absorbNestedAnd` (bitfield.cc:2333-2355):
/// `INSERT((x & #0xff) + y)  =>  INSERT(x + y)` for ops whose more significant
/// input bits do not affect the less significant result bits.
fn insert_absorb_nested_and(data: &mut Funcdata, base_op: OpId, insert_op: OpId) -> int4 {
    let base_out = match op_out(data, base_op) {
        Some(v) => v,
        None => return 0,
    };
    if data.lone_descend(base_out) != Some(insert_op) {
        return 0; // Result only used by INSERT
    }
    let bitsize = match op_in(data, insert_op, 3) {
        Some(v) => vn_offset(data, v) as int4,
        None => return 0,
    };
    for slot in 0..2 {
        let vn = match op_in(data, base_op, slot) {
            Some(v) => v,
            None => continue,
        };
        if !vn_is_written(data, vn) {
            continue;
        }
        let and_op = match vn_def(data, vn) {
            Some(o) => o,
            None => continue,
        };
        if op_code(data, and_op) != Some(OpCode::CPUI_INT_AND) {
            continue;
        }
        let cvn = match op_in(data, and_op, 1) {
            Some(v) => v,
            None => continue,
        };
        if !vn_is_constant(data, cvn) {
            continue;
        }
        let cvn_off = vn_offset(data, cvn);
        let mask = kuna_base::address::coveringmask(cvn_off);
        if mask != cvn_off {
            continue;
        }
        if (mask & 1) == 0 {
            continue; // Masking off least significant bits
        }
        let count = kuna_base::address::popcount(mask);
        if count < bitsize {
            continue; // INSERT masks off fewer bits, so AND still has an effect
        }
        let and_in0 = match op_in(data, and_op, 0) {
            Some(v) => v,
            None => continue,
        };
        data.op_set_input(base_op, and_in0, slot).ok();
        let and_out = match op_out(data, and_op) {
            Some(v) => v,
            None => return 0,
        };
        data.destroy_varnode_recursive(and_out).ok();
        return 1;
    }
    0
}

/// `RuleInsertAbsorb::applyOp` (bitfield.cc:2363-2400): switch on the def opcode
/// of the INSERT value input (`in(1)`) and dispatch into the absorb* helpers.
pub(super) fn insert_absorb_apply(op: OpId, data: &mut Funcdata) -> int4 {
    let in_vn = match op_in(data, op, 1) {
        Some(v) => v,
        None => return 0,
    };
    if !vn_is_written(data, in_vn) {
        return 0;
    }
    let in_op = match vn_def(data, in_vn) {
        Some(o) => o,
        None => return 0,
    };
    match op_code(data, in_op) {
        Some(OpCode::CPUI_SUBPIECE) => {
            let in_op_in1 = match op_in(data, in_op, 1) {
                Some(v) => v,
                None => return 0,
            };
            if vn_offset(data, in_op_in1) != 0 {
                return 0;
            }
            let in_op_in0 = match op_in(data, in_op, 0) {
                Some(v) => v,
                None => return 0,
            };
            data.op_set_input(op, in_op_in0, 1).ok();
            data.destroy_varnode_recursive(in_vn).ok();
            1
        }
        Some(OpCode::CPUI_INT_RIGHT) | Some(OpCode::CPUI_INT_SRIGHT) => {
            let in_op_in1 = match op_in(data, in_op, 1) {
                Some(v) => v,
                None => return 0,
            };
            if !vn_is_constant(data, in_op_in1) {
                return 0;
            }
            let vn = match op_in(data, in_op, 0) {
                Some(v) => v,
                None => return 0,
            };
            if !vn_is_written(data, vn) {
                return 0;
            }
            let next_op = match vn_def(data, vn) {
                Some(o) => o,
                None => return 0,
            };
            match op_code(data, next_op) {
                Some(OpCode::CPUI_INT_ADD) => insert_absorb_shift_add(data, in_op, next_op, op),
                Some(OpCode::CPUI_INT_LEFT) | Some(OpCode::CPUI_SUBPIECE) => {
                    insert_absorb_right_left(data, next_op, in_op, op)
                }
                _ => 0,
            }
        }
        Some(OpCode::CPUI_INT_AND) => insert_absorb_and(data, in_op, op),
        Some(OpCode::CPUI_INT_ADD)
        | Some(OpCode::CPUI_INT_OR)
        | Some(OpCode::CPUI_INT_XOR)
        | Some(OpCode::CPUI_INT_MULT) => insert_absorb_nested_and(data, in_op, op),
        _ => 0,
    }
}
