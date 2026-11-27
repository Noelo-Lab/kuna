//! Where a two-stage jump table's byte map begins when no guard bounds the
//! map's index.
//!
//! MSVC compiles a sparse `switch` to a byte map with one byte per selector
//! value, naming a row of a 4-byte offset table (`movzx eax, byte [map +
//! idx]`, `mov ecx, [table + rax*4]`), and lays the offset table out right
//! before its map. When the basic model finds no guard on the map's index, it
//! sizes the table from the map byte's width instead: 256 rows, all but the
//! first few read from the map's own bytes and whatever follows it. In a large
//! image those rows are destinations spread over other functions, mostly
//! below this one, where the run-on cut keeps them, and the flow decodes
//! their code for over a minute.
//!
//! The selector-map cut (`JumpBasicModel::end_table_at_selector_map`) ends a
//! table below its map at the map. Without a guarded index range the map's
//! extent is unknown, but it starts at its base, the constant part of the
//! LOAD's address, and runs up from there. A compiler may fold the index's
//! normalization into that constant (`map - 1 + idx`), which puts the last
//! real rows past it, so a row past the base ends the table only when it
//! reads the map: it lands outside the function's own code, and each of its
//! bytes is a selector value below its own, naming a row already kept. A
//! real row that gcc sends to the function's split-off `.cold` part lands
//! outside the function too, but it holds an address or an offset.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::types::{uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::p2_lift::jumptable::LoadTable;

/// The space and first byte of the map `selector` is loaded from: the sum of
/// the constants in its address, when the rest of the address is one term.
pub(crate) fn map_base(fd: &Funcdata, selector: VarnodeId) -> Option<(Rc<AddrSpace>, uintb)> {
    let load = fd.obank().get(fd.vbank().get(selector)?.get_def()?)?;
    if load.code() != OpCode::CPUI_LOAD {
        return None;
    }
    let ptr = load.get_in(1)?;
    let space_index = fd.vbank().get(load.get_in(0)?)?.get_offset();
    let space = fd.get_arch().manage().get_space(i32::try_from(space_index).ok()?)?.clone();
    let mut base: Option<uintb> = None;
    let mut term = None;
    let mut pending = vec![ptr];
    let mut adds = 0;
    while let Some(vn) = pending.pop() {
        let v = fd.vbank().get(vn)?;
        if v.is_constant() {
            base = Some(base.unwrap_or(0).wadd(v.get_offset()));
            continue;
        }
        let add = v.get_def().and_then(|d| fd.obank().get(d)).filter(|op| op.code() == OpCode::CPUI_INT_ADD);
        match add {
            Some(op) if adds < 4 => {
                adds += 1;
                pending.extend([op.get_in(0)?, op.get_in(1)?]);
            }
            _ if term.is_none() => term = Some(vn),
            _ => return None,
        }
    }
    term?;
    let size = fd.vbank().get(ptr)?.get_size();
    let mask = if size >= 8 { uintb::MAX } else { (1 << (8 * size)) - 1 };
    let byte = AddrSpace::address_to_byte(base? & mask, space.get_word_size());
    Some((space, byte))
}

/// Whether a jump to `raw` stays in the function's own code: between its
/// entry and the next known entry, in its section.
pub(crate) fn own_code(fd: &Funcdata, raw: uintb) -> bool {
    let entry = fd.get_address();
    let Some(space) = entry.get_space() else { return false };
    let bit = fd.get_arch().funcptr_align;
    let mask = if bit != 0 { (uintb::MAX >> bit) << bit } else { uintb::MAX };
    let dest = Address::new(Rc::clone(space), AddrSpace::address_to_byte(raw, space.get_word_size()) & mask);
    let Some(owner) = fd.get_arch().function_owner(entry) else { return false };
    owner(dest.get_offset()) == Some(entry.get_offset()) && fd.get_arch().in_section_of(entry, &dest)
}

/// Whether the bytes `row` reads are all below `value`: selector values
/// naming earlier rows, as the map holds. A row that cannot be read is not.
pub(crate) fn map_bytes(fd: &Funcdata, row: &LoadTable, value: uintb) -> bool {
    let mut buf = [0u8; 16];
    let len = (row.size.max(1) as usize).saturating_mul(row.num.max(1) as usize);
    let Some(bytes) = buf.get_mut(..len) else { return false };
    fd.get_arch().loader_fill(bytes, &row.addr).is_ok() && bytes.iter().all(|&b| uintb::from(b) < value)
}
