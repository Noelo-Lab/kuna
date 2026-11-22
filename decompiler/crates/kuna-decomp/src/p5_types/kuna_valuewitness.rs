//! (kuna `valuewitness`) A value's witness table, loaded through its type
//! metadata, is a `swift.value_witness_table *`.
//!
//! Swift copies, moves and destroys a value whose layout it does not know at
//! compile time through the value witness table in the word before the
//! type's metadata, so generic and resilient code is full of
//!
//! ```text
//!   (**(void (**)(long, long, long))(*(long *)(meta + -8) + 0x10))(a0, a1, meta);
//!   alloca(*(unsigned long *)(*(long *)(meta + -8) + 0x40));
//! ```
//!
//! When the Swift metadata pass interned the table's layout, the word loaded
//! from `meta - 8` votes `swift.value_witness_table *` -- provided the word is
//! itself read at a table offset; `swift.enum_value_witness_table *` when one
//! read is past the common part -- and field rendering does the rest:
//! `vwt->initializeWithCopy(a0, a1, meta)`, `alloca(vwt->size)`. A `meta - 8`
//! off a pointer typed `swift.type_metadata *` stays a byte offset instead of
//! becoming `meta[-1].field`: the word before the metadata is not a metadata
//! field. In code of any other language the layouts are never interned and
//! nothing changes.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{Datatype, type_metatype};
use crate::funcdata::Funcdata;

/// The interned layouts (`kuna_swifttypes::witness_table_struct` and
/// `enum_witness_table_struct`) and the metadata layout.
const TABLE: &str = "swift.value_witness_table";
const ENUM_TABLE: &str = "swift.enum_value_witness_table";
const METADATA: &str = "swift.type_metadata";

/// The extent of the common table and of an enum's.
const TABLE_SIZE: u64 = 88;
const ENUM_TABLE_SIZE: u64 = 112;

/// The vote for `vn`, or `None` when it is not a witness table load.
pub fn output_vote(data: &Funcdata, vn: VarnodeId) -> Option<Rc<Datatype>> {
    let arch = data.get_arch();
    if !arch.analysis_valuewitness {
        return None;
    }
    let v = data.vbank().get(vn)?;
    if v.is_type_lock() || v.get_size() != 8 {
        return None;
    }
    let load = data.obank().get(v.get_def()?)?;
    if load.code() != OpCode::CPUI_LOAD || !below_metadata(data, load.get_in(1)?) {
        return None;
    }
    let end = v
        .descend_iter()
        .filter_map(|op| table_read(data, op, vn))
        .max()?;
    let name = if end <= TABLE_SIZE { TABLE } else { ENUM_TABLE };
    let types = arch.types()?;
    let table = types.find_by_name(name).ok()??;
    if table.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    types.get_type_pointer(8, table, 1).ok()
}

/// Whether `RulePtrArith` must leave the `p + -8` at `op` alone: `p` (input
/// `slot`) points at Swift type metadata.
pub fn keeps_offset(data: &Funcdata, op: OpId, slot: i32) -> bool {
    if !data.get_arch().analysis_valuewitness {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    let (Some(base), Some(off)) = (o.get_in(slot), o.get_in(1 - slot)) else {
        return false;
    };
    let is_minus8 = data
        .vbank()
        .get(off)
        .is_some_and(|c| c.is_constant() && c.get_offset() as i64 == -8);
    is_minus8
        && data
            .vn_type_read_facing(base, op)
            .get_ptr_to()
            .is_some_and(|t| t.get_name() == METADATA)
}

/// Whether `addr` is the word before another value: `meta + -8`.
fn below_metadata(data: &Funcdata, addr: VarnodeId) -> bool {
    let Some(op) = data
        .vbank()
        .get(addr)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
    else {
        return false;
    };
    let constant = |slot: i32| {
        op.get_in(slot)
            .and_then(|c| data.vbank().get(c))
            .filter(|c| c.is_constant())
            .map(|c| c.get_offset() as i64)
    };
    match op.code() {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => constant(1) == Some(-8),
        _ => false,
    }
}

/// The end of the table word `op` reads through `vn` (offset + 8), when it
/// is a load through `vn` or through `vn` plus an offset inside a table.
fn table_read(data: &Funcdata, op: OpId, vn: VarnodeId) -> Option<u64> {
    let o = data.obank().get(op)?;
    match o.code() {
        OpCode::CPUI_LOAD if o.get_in(1) == Some(vn) => Some(8),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB if o.get_in(0) == Some(vn) => {
            let c = data.vbank().get(o.get_in(1)?).filter(|c| c.is_constant())?;
            let off = c.get_offset();
            let out = o.get_out()?;
            let loaded = data.vbank().get(out)?.descend_iter().any(|r| {
                data.obank()
                    .get(r)
                    .is_some_and(|r| r.code() == OpCode::CPUI_LOAD && r.get_in(1) == Some(out))
            });
            (off < ENUM_TABLE_SIZE && loaded).then_some(off + 8)
        }
        _ => None,
    }
}
