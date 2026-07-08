//! Port of the bitfield-expression family from `decompiler/cpp/expression.cc`
//! (`expression.cc:403-586`) and its declarations in `expression.hh:175-229`.
//!
//! These containers recover, from a materialized `CPUI_INSERT`/`CPUI_ZPULL`/
//! `CPUI_SPULL` op (built by the [`super::insert`]/[`super::pull`] transforms),
//! the *structure*, the *bitfield description*, and (for the LOAD/STORE forms)
//! the *Varnode holding the pointer to the parent structure* — exactly the
//! elements [`crate::printc`] needs to render `ptr->field` member syntax instead
//! of the raw `INSERT(...)`/`ZPULL(...)` functional form.
//!
//! # Faithfulness
//!
//! Every method is a line-for-line transcription of the C++ named in its doc
//! comment.  The field offsets/widths are read from the op's const inputs
//! (`in(2)`/`in(3)` for INSERT, `in(1)`/`in(2)` for the pulls) and from the
//! `TypeStruct` bitfield `TypeField`s via [`Datatype::find_matching_bit_field`];
//! nothing is hardcoded and no field name / mask value / offset is special-cased.
//!
//! # API-surface adaptation
//!
//! The C++ `Varnode *`/`PcodeOp *` graph is, per ADR 0001, a [`Funcdata`]-owned
//! pair of arenas reached through the banks.  Each recovery threads `data`
//! explicitly.  The C++ `value->getHigh()->getSymbol()` Symbol surface (used by
//! [`InsertExpression`] and the no-LOAD [`PullExpression`] arm) is, in the merged
//! tree, the angr/`ScopeLocal` stand-in carried on the [`HighVariable`]
//! (`kuna_symbol_type`/`kuna_symbol_offset`); a recovery reads the symbol's type
//! and in-symbol offset off the high exactly where the C++ reads them off the
//! Symbol.  // SEAM(W4 symbol) — mirrors `printc::push_vn_explicit_ir`.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use super::BitRange;
use crate::dtype::{type_metatype, Datatype, TypeBitField};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

/// `sizeof(uintb)` — the C++ `uintb` is `uint8`, i.e. 8 bytes.
const SIZEOF_UINTB: int4 = 8;

/// A container for an expression manipulating a bitfield (C++
/// `BitFieldExpression`, expression.hh:181-192).
///
/// The recovered structure(s) and the matched bitfield are carried here; the
/// concrete [`InsertExpression`]/[`InsertStoreExpression`]/[`PullExpression`]
/// records embed one and fill it via [`get_structures`](Self::get_structures).
#[derive(Default)]
pub struct BitFieldExpression {
    /// Parent structure containing the bitfield (C++ `theStruct`).  `None` when
    /// the structure could not be recovered.
    pub the_struct: Option<Rc<Datatype>>,
    /// Formal bitfield description (C++ `bitfield`).  `None` ⇒ invalid.
    pub bitfield: Option<TypeBitField>,
    /// Offset of byte range into the encompassing struct (C++ `byteRangeOffset`).
    pub byte_range_offset: int4,
    /// Offset of structure containing bitfield in encompassStruct (C++
    /// `offsetToBitStruct`).
    pub offset_to_bit_struct: int4,
}

impl BitFieldExpression {
    /// Is this a valid bitfield expression (C++ `BitFieldExpression::isValid`,
    /// expression.hh:190): a bitfield was matched.
    pub fn is_valid(&self) -> bool {
        self.bitfield.is_some()
    }

    /// Recover the structure(s) holding the bitfield (C++
    /// `BitFieldExpression::getStructures`, expression.cc:403-438).
    ///
    /// Find the structure immediately containing the byte range described by the
    /// given data-type, then — given the least significant bit of the bitfield —
    /// descend into the structure immediately containing the bitfield (which may
    /// be a different, nested structure).
    fn get_structures(&mut self, dt: &Rc<Datatype>, init_byte_off: int4, least_bit_off: int4, is_big_endian: bool) {
        self.the_struct = None;
        self.byte_range_offset = if init_byte_off < 0 { 0 } else { init_byte_off };
        let encompass_struct: Rc<Datatype> = match dt.get_metatype() {
            type_metatype::TYPE_PARTIALSTRUCT => {
                // TypePartialStruct *partial = (TypePartialStruct *)dt;
                let struct_tmp = match dt.get_partial_base() {
                    Some(s) => s,
                    None => return,
                };
                if struct_tmp.get_metatype() != type_metatype::TYPE_STRUCT {
                    return;
                }
                self.byte_range_offset += dt.get_partial_offset().unwrap_or(0);
                struct_tmp
            }
            type_metatype::TYPE_STRUCT => Rc::clone(dt),
            _ => return,
        };
        // int8 offset = byteRangeOffset; leastBitOff /= 8;
        let mut offset: i64 = self.byte_range_offset as i64;
        let least_byte = least_bit_off / 8;
        if is_big_endian {
            // offset += (dt->getSize() - leastBitOff - 1);
            offset += (dt.get_size() - least_byte - 1) as i64;
        } else {
            offset += least_byte as i64;
        }
        let mut the_struct = encompass_struct;
        self.offset_to_bit_struct = 0;
        // C++ `for(;;)`: descend while `getSubType` yields a nested struct.  The
        // three break conditions (no subtype, a non-struct subtype, or a getSubType
        // error which the struct walk never raises) collapse into one `while let`.
        while let Ok((Some(tmp_dt), newoff)) = the_struct.get_sub_type(offset) {
            if tmp_dt.get_metatype() != type_metatype::TYPE_STRUCT {
                break;
            }
            // offsetToBitStruct += (offset-newoff);
            self.offset_to_bit_struct += (offset - newoff) as int4;
            offset = newoff;
            the_struct = tmp_dt;
        }
        self.the_struct = Some(the_struct);
    }

    /// Recover the Varnode holding the pointer to the parent structure (C++
    /// `BitFieldExpression::recoverStructurePointer`, expression.cc:440-456).
    ///
    /// `vn` is the pointer input into the LOAD/STORE; `offset` is the byte range
    /// offset.  If the access is already at offset 0 and the pointer's size
    /// matches the structure, the pointer is returned as-is; otherwise a PTRSUB
    /// writing the pointer at the matching offset is unwrapped.
    fn recover_structure_pointer(&self, data: &Funcdata, vn: VarnodeId, offset: int4) -> Option<VarnodeId> {
        let v = data.vbank().get(vn)?;
        let struct_size = self.the_struct.as_ref().map(|s| s.get_size()).unwrap_or(-1);
        if offset == 0 && struct_size == v.get_size() {
            return Some(vn);
        } else if v.is_written() {
            let ptr_sub = v.get_def()?;
            let pso = data.obank().get(ptr_sub)?;
            if pso.code() == OpCode::CPUI_PTRSUB {
                let in1 = pso.get_in(1)?;
                // (int4)ptrSub->getIn(1)->getOffset() == offset
                if data.vbank().get(in1).map(|c| c.get_offset() as int4) == Some(offset) {
                    return pso.get_in(0);
                }
            }
        }
        if offset != 0 {
            return None;
        }
        Some(vn)
    }
}

/// Get the field description corresponding to a given ZPULL or SPULL (C++
/// `BitFieldExpression::getPullField`, expression.cc:459-475).
///
/// A static helper used by the simplification rules to identify the bitfield a
/// pull reads, independent of full [`PullExpression`] recovery.
pub fn get_pull_field(data: &Funcdata, pull: OpId) -> Option<TypeBitField> {
    let mut expr = BitFieldExpression::default();
    let o = data.obank().get(pull)?;
    let in_vn = o.get_in(0)?;
    let least_bit_off = data.vbank().get(o.get_in(1)?)?.get_offset() as int4;
    let bit_size = data.vbank().get(o.get_in(2)?)?.get_offset() as int4;
    let is_big = data.vbank().get(in_vn)?.get_space().is_big_endian();
    let dt = data.vn_type_read_facing(in_vn, pull);
    expr.get_structures(&dt, 0, least_bit_off, is_big);
    let the_struct = expr.the_struct.as_ref()?;
    let in_size = data.vbank().get(in_vn)?.get_size();
    let range = BitRange::new(
        expr.byte_range_offset - expr.offset_to_bit_struct,
        in_size,
        least_bit_off,
        bit_size,
        is_big,
    );
    the_struct.find_matching_bit_field(&range)
}

/// A write to a bitfield stored in an explicit Varnode (C++ `InsertExpression`,
/// expression.hh:198-205).
///
/// The INSERT output is associated with a (partial) symbol and holds the
/// data-type information; the structure/bitfield are recovered from the symbol.
pub struct InsertExpression {
    /// The embedded recovery state.
    pub expr: BitFieldExpression,
    /// The bound symbol's in-symbol byte offset (C++ `value->getHigh()
    /// ->getSymbolOffset()`), `-1` if there is no bound symbol.
    pub symbol_offset: int4,
}

impl InsertExpression {
    /// Construct from an INSERT (C++ `InsertExpression::InsertExpression`,
    /// expression.cc:477-492).
    pub fn new(data: &Funcdata, insert: OpId) -> InsertExpression {
        let mut expr = BitFieldExpression::default();
        let mut symbol_offset = -1;
        let recovered = (|| -> Option<()> {
            let o = data.obank().get(insert)?;
            let value = o.get_out()?;
            let v = data.vbank().get(value)?;
            // symbol = value->getHigh()->getSymbol(); if null return.
            let high = v.get_high()?;
            let hb = data.high_bank().get(high)?;
            let sym_type = hb.kuna_symbol_type()?;
            symbol_offset = hb.kuna_symbol_offset();
            let spc = v.get_space();
            let least_bit_off = data.vbank().get(o.get_in(2)?)?.get_offset() as int4;
            let bit_size = data.vbank().get(o.get_in(3)?)?.get_offset() as int4;
            let sym_type = Rc::clone(sym_type);
            expr.get_structures(&sym_type, hb.kuna_symbol_offset(), least_bit_off, spc.is_big_endian());
            let the_struct = expr.the_struct.as_ref()?;
            let range = BitRange::new(
                expr.byte_range_offset - expr.offset_to_bit_struct,
                v.get_size(),
                least_bit_off,
                bit_size,
                spc.is_big_endian(),
            );
            expr.bitfield = the_struct.find_matching_bit_field(&range);
            Some(())
        })();
        let _ = recovered;
        InsertExpression { expr, symbol_offset }
    }

    /// Get a mask representing the INSERTed range of bits (C++
    /// `InsertExpression::getRangeMask`, expression.cc:496-507).
    pub fn get_range_mask(data: &Funcdata, insert: OpId) -> uintb {
        let o = data.obank().get(insert).expect("get_range_mask: stale op");
        let least_bit_off = data.vbank().get(o.get_in(2).unwrap()).unwrap().get_offset() as int4;
        let bit_size = data.vbank().get(o.get_in(3).unwrap()).unwrap().get_offset() as int4;
        let mut res: uintb = !0u64;
        if bit_size < 8 * SIZEOF_UINTB {
            res = !(res << bit_size);
        }
        res << least_bit_off
    }

    /// Get the mask of the least significant bits (C++
    /// `InsertExpression::getLSBMask`, expression.cc:509-520).
    pub fn get_lsb_mask(data: &Funcdata, insert: OpId) -> uintb {
        let o = data.obank().get(insert).expect("get_lsb_mask: stale op");
        let bit_size = data.vbank().get(o.get_in(3).unwrap()).unwrap().get_offset() as int4;
        let mut res: uintb = !0u64;
        if bit_size < 8 * SIZEOF_UINTB {
            res = !(res << bit_size);
        }
        res
    }
}

/// A write to a bitfield through a STORE op (C++ `InsertStoreExpression`,
/// expression.hh:211-217).
///
/// The INSERT output holds the data-type info.  A final STORE must be present and
/// a LOAD (recovering the parts of the structure left unaffected) may be present.
pub struct InsertStoreExpression {
    /// The embedded recovery state.
    pub expr: BitFieldExpression,
    /// The INSERT op feeding the STORE value (C++ `insertOp`).
    pub insert_op: Option<OpId>,
    /// The LOAD op recovering unaffected bytes (C++ `loadOp`), may be `None`.
    pub load_op: Option<OpId>,
    /// Varnode holding the pointer to the parent structure (C++ `structPtr`).
    pub struct_ptr: Option<VarnodeId>,
}

impl InsertStoreExpression {
    /// Construct from a STORE (C++ `InsertStoreExpression::InsertStoreExpression`,
    /// expression.cc:522-549).
    pub fn new(data: &Funcdata, store: OpId) -> InsertStoreExpression {
        let mut me = InsertStoreExpression {
            expr: BitFieldExpression::default(),
            insert_op: None,
            load_op: None,
            struct_ptr: None,
        };
        me.recover(data, store);
        me
    }

    fn recover(&mut self, data: &Funcdata, store: OpId) -> Option<()> {
        let so = data.obank().get(store)?;
        let value = so.get_in(2)?;
        let val_v = data.vbank().get(value)?;
        if !val_v.is_written() {
            return None;
        }
        let insert_op = val_v.get_def()?;
        if data.obank().get(insert_op)?.code() != OpCode::CPUI_INSERT {
            return None;
        }
        self.insert_op = Some(insert_op);
        let io = data.obank().get(insert_op)?;
        // dest can either be a constant or LOAD.
        let dest = io.get_in(0)?;
        let dest_v = data.vbank().get(dest)?;
        if dest_v.is_written() {
            let load_op = dest_v.get_def()?;
            if data.obank().get(load_op)?.code() != OpCode::CPUI_LOAD {
                return None;
            }
            self.load_op = Some(load_op);
        } else if !dest_v.is_constant() {
            return None;
        }
        // AddrSpace *spc = store->getIn(0)->getSpaceFromConst();
        let spc = space_from_const(data, store, 0)?;
        let least_bit_off = data.vbank().get(io.get_in(2)?)?.get_offset() as int4;
        let bit_size = data.vbank().get(io.get_in(3)?)?.get_offset() as int4;
        let def_facing = data.vn_type_def_facing(value);
        self.expr.get_structures(&def_facing, 0, least_bit_off, spc.is_big_endian());
        let the_struct = match &self.expr.the_struct {
            Some(s) => Rc::clone(s),
            None => return None,
        };
        // structPtr = recoverStructurePointer(store->getIn(1), byteRangeOffset);
        let store_ptr = data.obank().get(store)?.get_in(1)?;
        self.struct_ptr = self.expr.recover_structure_pointer(data, store_ptr, self.expr.byte_range_offset);
        self.struct_ptr?;
        let range = BitRange::new(
            self.expr.byte_range_offset - self.expr.offset_to_bit_struct,
            val_v.get_size(),
            least_bit_off,
            bit_size,
            spc.is_big_endian(),
        );
        self.expr.bitfield = the_struct.find_matching_bit_field(&range);
        Some(())
    }

    /// Is this a valid bitfield STORE expression (C++ `isValid`).
    pub fn is_valid(&self) -> bool {
        self.expr.is_valid()
    }
}

/// A read of a bitfield via a ZPULL or SPULL operator (C++ `PullExpression`,
/// expression.hh:222-229).
///
/// The first input to the operator must either be a (partial) symbol or be
/// written by a LOAD.
pub struct PullExpression {
    /// The embedded recovery state.
    pub expr: BitFieldExpression,
    /// The ZPULL or SPULL op (C++ `pullOp`).
    pub pull_op: OpId,
    /// The LOAD reading the bytes containing the bitfield (C++ `loadOp`), may be
    /// `None`.
    pub load_op: Option<OpId>,
    /// Varnode holding the pointer to the parent structure (C++ `structPtr`).
    pub struct_ptr: Option<VarnodeId>,
    /// `true` if the pull reads a bound symbol (no LOAD) — the symbol-detail
    /// render path (C++ `symbol != null`).
    pub has_symbol: bool,
}

impl PullExpression {
    /// Construct from a ZPULL or SPULL (C++ `PullExpression::PullExpression`,
    /// expression.cc:551-586).
    pub fn new(data: &Funcdata, pull: OpId) -> PullExpression {
        let mut me = PullExpression {
            expr: BitFieldExpression::default(),
            pull_op: pull,
            load_op: None,
            struct_ptr: None,
            has_symbol: false,
        };
        me.recover(data, pull);
        me
    }

    fn recover(&mut self, data: &Funcdata, pull: OpId) -> Option<()> {
        let po = data.obank().get(pull)?;
        let in_vn = po.get_in(0)?;
        let in_v = data.vbank().get(in_vn)?;
        let dt: Rc<Datatype>;
        let offset: int4;
        let spc;
        if in_v.is_written()
            && data.obank().get(in_v.get_def()?).map(|o| o.code()) == Some(OpCode::CPUI_LOAD)
        {
            let load_op = in_v.get_def()?;
            self.load_op = Some(load_op);
            spc = space_from_const(data, load_op, 0)?;
            dt = data.vn_type_read_facing(in_vn, pull);
            offset = 0;
        } else {
            // symbol = inVn->getHigh()->getSymbol(); if null return.
            let high = in_v.get_high()?;
            let hb = data.high_bank().get(high)?;
            let sym_type = hb.kuna_symbol_type()?;
            self.has_symbol = true;
            spc = Rc::clone(in_v.get_space());
            dt = Rc::clone(sym_type);
            offset = hb.kuna_symbol_offset();
        }
        let least_bit_off = data.vbank().get(po.get_in(1)?)?.get_offset() as int4;
        let bit_size = data.vbank().get(po.get_in(2)?)?.get_offset() as int4;
        self.expr.get_structures(&dt, offset, least_bit_off, spc.is_big_endian());
        let the_struct = match &self.expr.the_struct {
            Some(s) => Rc::clone(s),
            None => return None,
        };
        if let Some(load_op) = self.load_op {
            let load_ptr = data.obank().get(load_op)?.get_in(1)?;
            self.struct_ptr =
                self.expr.recover_structure_pointer(data, load_ptr, self.expr.byte_range_offset);
            self.struct_ptr?;
        }
        let in_size = data.vbank().get(in_vn)?.get_size();
        let range = BitRange::new(
            self.expr.byte_range_offset - self.expr.offset_to_bit_struct,
            in_size,
            least_bit_off,
            bit_size,
            spc.is_big_endian(),
        );
        self.expr.bitfield = the_struct.find_matching_bit_field(&range);
        Some(())
    }

    /// Is this a valid bitfield read expression (C++ `isValid`).
    pub fn is_valid(&self) -> bool {
        self.expr.is_valid()
    }
}

/// C++ `op->getIn(slot)->getSpaceFromConst()`: resolve the address space whose
/// manager index is the constant offset in the given input slot (the LOSS-015
/// model, same resolution as `coreaction_casts::load_store_space`).
fn space_from_const(data: &Funcdata, op: OpId, slot: int4) -> Option<Rc<kuna_base::space::AddrSpace>> {
    let cvn = data.obank().get(op)?.get_in(slot)?;
    let idx = data.vbank().get(cvn)?.get_offset();
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as u64 {
        return None;
    }
    manage.get_space(idx as i32).cloned()
}
