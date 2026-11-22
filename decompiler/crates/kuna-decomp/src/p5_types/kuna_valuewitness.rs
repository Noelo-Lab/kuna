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
//! `vwt->initializeWithCopy(a0, a1, meta)`, `alloca(vwt->size)`. A constant
//! offset off a pointer typed `swift.type_metadata *` that falls outside its
//! 16-byte header (`meta - 8`, the field offsets and generic arguments after
//! it) stays a byte offset instead of becoming `meta[-1].field` or
//! `meta[1].kind`: those words are not header fields. In code of any other language the layouts are never interned and
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

/// Whether `RulePtrArith` must leave the `p + c` at `op` alone: `p` (input
/// `slot`) points at Swift type metadata and `c` falls outside its common
/// 16-byte header (the value witness table before it, the field offsets and
/// generic arguments after it), which the header's layout does not describe.
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
    let outside = data
        .vbank()
        .get(off)
        .is_some_and(|c| c.is_constant() && !(0..16).contains(&(c.get_offset() as i64)));
    outside
        && data
            .vn_type_read_facing(base, op)
            .get_ptr_to()
            .is_some_and(|t| t.get_name() == METADATA)
}

/// Whether the `INT_ADD` `op` adds to its input `slot`, typed `ptr`, a
/// constant outside the 16-byte header `ptr`'s `swift.type_metadata` lays
/// out: the sum points at a field offset or generic argument, not at another
/// metadata record, so the pointer type does not propagate to it.
pub fn outside_header(data: &Funcdata, op: OpId, slot: i32, ptr: &Rc<Datatype>) -> bool {
    if !data.get_arch().analysis_valuewitness
        || !ptr.get_ptr_to().is_some_and(|t| t.get_name() == METADATA)
    {
        return false;
    }
    data.obank()
        .get(op)
        .filter(|o| o.code() == OpCode::CPUI_INT_ADD)
        .and_then(|o| o.get_in(1 - slot))
        .and_then(|c| data.vbank().get(c))
        .is_some_and(|c| c.is_constant() && !(0..16).contains(&(c.get_offset() as i64)))
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

/// The end of the table bytes `op` reads through `vn` (offset plus the
/// width read), when it is a load through `vn` or through `vn` plus an
/// offset inside a table. The width matters: `flags` and
/// `extraInhabitantCount` are 4-byte words that end inside the common table.
fn table_read(data: &Funcdata, op: OpId, vn: VarnodeId) -> Option<u64> {
    let o = data.obank().get(op)?;
    let width = |load: &crate::op::PcodeOp| {
        load.get_out()
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.get_size() as u64)
    };
    match o.code() {
        OpCode::CPUI_LOAD if o.get_in(1) == Some(vn) => width(o),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB if o.get_in(0) == Some(vn) => {
            let c = data.vbank().get(o.get_in(1)?).filter(|c| c.is_constant())?;
            let off = c.get_offset();
            if off >= ENUM_TABLE_SIZE {
                return None;
            }
            let out = o.get_out()?;
            data.vbank()
                .get(out)?
                .descend_iter()
                .filter_map(|r| data.obank().get(r))
                .filter(|r| r.code() == OpCode::CPUI_LOAD && r.get_in(1) == Some(out))
                .filter_map(|r| width(r))
                .max()
                .map(|w| off + w)
        }
        _ => None,
    }
}

/// The tables' layouts when the Swift metadata pass interned them, the
/// value witness table's and the enum one's.
pub fn tables(arch: &crate::context::ArchContext) -> Option<(Rc<Datatype>, Option<Rc<Datatype>>)> {
    if !arch.analysis_valuewitness {
        return None;
    }
    let types = arch.types()?;
    let table = types.find_by_name(TABLE).ok().flatten()?;
    Some((table, types.find_by_name(ENUM_TABLE).ok().flatten()))
}

/// The function's ops in address order, for [`call_prototype`].
pub fn ordered_ops(data: &Funcdata) -> Vec<OpId> {
    let mut ops: Vec<OpId> = data.obank().iter_alive().collect();
    ops.sort_by(|a, b| {
        let seq = |o: &OpId| data.obank().get(*o).map(|o| o.get_seq_num().clone());
        seq(a).cmp(&seq(b))
    });
    ops
}

/// The prototype of the value witness slot the indirect call `op` calls
/// through, read off the lifted ops: its target is loaded at a table offset
/// from a word itself loaded from `p + -8`, in one block or a run of blocks
/// with one way in. `ActionDefaultParams` gives it to the call before
/// parameter recovery, so type recovery never has to force it (which would
/// cost a restart once the call's inputs are fixed).
pub fn call_prototype(
    data: &Funcdata,
    op: OpId,
    ops: &[OpId],
    (table, enum_table): &(Rc<Datatype>, Option<Rc<Datatype>>),
) -> Option<Rc<crate::fspec::FuncProto>> {
    if data.obank().get(op)?.code() != OpCode::CPUI_CALLIND {
        return None;
    }
    let pos = ops.iter().position(|&o| o == op)?;
    let off = slot_offset(data, &ops[..pos], op)?;
    let owner = if off < TABLE_SIZE {
        Some(table)
    } else {
        enum_table.as_ref()
    };
    let proto = slot_prototype(owner?, off)?;
    crate::kuna_typedcallabi::admit(data, proto)
}

/// Whether `code` is the function type of a value witness table slot.
pub fn is_slot_code(arch: &crate::context::ArchContext, code: &Rc<Datatype>) -> bool {
    let Some((table, enum_table)) = tables(arch) else {
        return false;
    };
    std::iter::once(&table).chain(enum_table.as_ref()).any(|t| {
        (0..).map_while(|i| t.get_field(i)).any(|f| {
            f.field_type
                .get_ptr_to()
                .is_some_and(|p| Rc::ptr_eq(&p, code))
        })
    })
}

/// The prototype of the function-pointer field at `off` of `table`.
fn slot_prototype(table: &Rc<Datatype>, off: u64) -> Option<Rc<crate::fspec::FuncProto>> {
    let field = (0..)
        .map_while(|i| table.get_field(i))
        .find(|f| f.offset as u64 == off)?;
    field.field_type.get_ptr_to()?.get_code_prototype().cloned()
}

/// The table offset the CALLIND `call` loads its target from, read off the
/// lifted ops before it.
fn slot_offset(data: &Funcdata, before: &[OpId], call: OpId) -> Option<u64> {
    let target = data.obank().get(call)?.get_in(0)?;
    let (load, rest) = defined_before(data, before, target)?;
    let (base, off) = load_address(data, rest, load)?;
    if !(0..ENUM_TABLE_SIZE).contains(&off) || off % 8 != 0 {
        return None;
    }
    let (table, rest) = defined_before(data, rest, base)?;
    let (_, below) = load_address(data, rest, table)?;
    (below as i64 == -8).then_some(off)
}

/// The LOAD `load` reads `base + off`: the base storage's varnode and `off`.
fn load_address(data: &Funcdata, before: &[OpId], load: OpId) -> Option<(VarnodeId, u64)> {
    let o = data.obank().get(load)?;
    if o.code() != OpCode::CPUI_LOAD {
        return None;
    }
    let addr = o.get_in(1)?;
    if data.vbank().get(addr)?.is_constant() {
        return None;
    }
    match defined_before(data, before, addr) {
        Some((add, rest)) => {
            if data.obank().get(add)?.code() != OpCode::CPUI_INT_ADD {
                return Some((addr, 0));
            }
            let (base, off) = add_parts(data, add)?;
            Some((base, constant_before(data, rest, off)?))
        }
        None => Some((addr, 0)),
    }
}

/// The base and the offset input of the INT_ADD `add`: the lifter puts the
/// constant of `call *0x20(%r12)` first and that of `mov 0x10(%r12),%rax`
/// second.
fn add_parts(data: &Funcdata, add: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let a = data.obank().get(add)?;
    let (x, y) = (a.get_in(0)?, a.get_in(1)?);
    if data.vbank().get(x)?.is_constant() {
        Some((y, x))
    } else {
        Some((x, y))
    }
}

/// The constant `vn` holds: itself, or the constant the last write before
/// copied into it.
fn constant_before(data: &Funcdata, before: &[OpId], vn: VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    let (op, _) = defined_before(data, before, vn)?;
    let o = data.obank().get(op)?;
    let c = data.vbank().get(o.get_in(0)?).filter(|c| c.is_constant())?;
    (o.code() == OpCode::CPUI_COPY).then(|| c.get_offset())
}

/// The op among `before` that defines `vn`'s value, through register COPYs,
/// and the ops before it.
fn defined_before<'a>(
    data: &Funcdata,
    before: &'a [OpId],
    vn: VarnodeId,
) -> Option<(OpId, &'a [OpId])> {
    let (mut op, mut rest) = written_before(data, before, vn)?;
    for _ in 0..6 {
        let o = data.obank().get(op)?;
        let input = match o.code() {
            OpCode::CPUI_COPY => o.get_in(0)?,
            OpCode::CPUI_LOAD => match spilled_value(data, rest, op) {
                Some((value, earlier)) => {
                    rest = earlier;
                    value
                }
                None => break,
            },
            _ => break,
        };
        if data.vbank().get(input)?.is_constant() {
            break;
        }
        (op, rest) = written_before(data, rest, input)?;
    }
    Some((op, rest))
}

/// The value the LOAD `load` reads back from where a STORE before it left
/// it (a spill and its reload: the same base storage, not written between,
/// and the same offset), and the ops before that STORE.
fn spilled_value<'a>(
    data: &Funcdata,
    before: &'a [OpId],
    load: OpId,
) -> Option<(VarnodeId, &'a [OpId])> {
    let (base, off) = load_address(data, before, load)?;
    let base_vn = data.vbank().get(base)?;
    let (base_addr, base_size) = (base_vn.get_addr().clone(), base_vn.get_size());
    for (k, &id) in before.iter().enumerate().rev() {
        let o = data.obank().get(id)?;
        let writes_base = o
            .get_out()
            .and_then(|out| data.vbank().get(out))
            .is_some_and(|out| out.get_addr() == &base_addr && out.get_size() == base_size);
        if writes_base || o.is_branch() && o.code() != OpCode::CPUI_CBRANCH {
            return None;
        }
        if o.code() != OpCode::CPUI_STORE {
            continue;
        }
        let rest = &before[..k];
        let Some((addr, _)) = written_before(data, rest, o.get_in(1)?) else {
            continue;
        };
        let a = data.obank().get(addr)?;
        let same = a.code() == OpCode::CPUI_INT_ADD
            && add_parts(data, addr).is_some_and(|(b, c)| {
                data.vbank()
                    .get(b)
                    .is_some_and(|b| b.get_addr() == &base_addr)
                    && constant_before(data, rest, c) == Some(off)
            });
        if same {
            return Some((o.get_in(2)?, rest));
        }
    }
    None
}

/// Whether the block `start` opens has one way in, from the block `prev`
/// (the op just before it) ends.
fn falls_into(data: &Funcdata, start: OpId, prev: Option<OpId>) -> bool {
    let parent = |op: OpId| data.obank().get(op).and_then(|o| o.get_parent());
    let (Some(bl), Some(pb)) = (parent(start), prev.and_then(parent)) else {
        return false;
    };
    let block = data.bblocks_ref().block(bl);
    block.size_in() == 1 && block.get_in(0) == pb
}

/// Whether the default calling convention keeps `addr` across a call.
fn survives_calls(data: &Funcdata, addr: &kuna_base::address::Address, size: i32) -> bool {
    data.get_arch()
        .default_fp()
        .is_some_and(|model| model.has_effect(addr, size) == crate::fspec::effect_type::UNAFFECTED)
}

/// The last op among `before` writing the storage `vn` reads, and the ops
/// before it; `None` past the start of a block with another way in, an
/// unconditional branch, or a call that may change the storage.
fn written_before<'a>(
    data: &Funcdata,
    before: &'a [OpId],
    vn: VarnodeId,
) -> Option<(OpId, &'a [OpId])> {
    let v = data.vbank().get(vn)?;
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    for (k, &id) in before.iter().enumerate().rev() {
        let o = data.obank().get(id)?;
        if o.is_branch() && o.code() != OpCode::CPUI_CBRANCH {
            return None;
        }
        if o.is_call() && !survives_calls(data, &addr, size) {
            return None;
        }
        let writes = o
            .get_out()
            .and_then(|out| data.vbank().get(out))
            .is_some_and(|out| out.get_addr() == &addr && out.get_size() == size);
        if writes {
            return Some((id, &before[..k]));
        }
        if o.is_block_start() && !falls_into(data, id, before[..k].last().copied()) {
            return None;
        }
    }
    None
}
