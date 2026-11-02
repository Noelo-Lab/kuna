//! String Display arguments proven by a complete forwarding body and live stack words.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;

pub fn forwarded_target(address: u64, bytes: &[u8]) -> Option<u64> {
    const PREFIX: &[u8] = &[
        0x48, 0x89, 0xf0, 0x48, 0x8b, 0x37, 0x48, 0x8b, 0x57, 0x08, 0x48, 0x89, 0xc7, 0xe9,
    ];
    if bytes.get(..PREFIX.len())? != PREFIX {
        return None;
    }
    let displacement = i32::from_le_bytes(bytes.get(14..18)?.try_into().ok()?);
    address
        .checked_add(18)?
        .checked_add_signed(i64::from(displacement))
}

pub(crate) fn recover(
    fd: &Funcdata,
    arch: &Architecture,
    call: OpId,
    formatter: u64,
    pointer: VarnodeId,
) -> Option<super::kuna_rustfmtargs::Argument> {
    if !arch.rust_fmt_str
        || !matches!(arch.rustc_stable_version, Some((1, 93, 0 | 1)))
        || !arch.rust_str_display_formatters.contains(&formatter)
    {
        return None;
    }
    let base = super::kuna_rustfmtargs::stack_offset(fd, pointer)?;
    if base % 8 != 0 {
        return None;
    }
    let cells = super::kuna_rustfmtargs::stack_cells(fd, call, base, 2)?;
    let data = fd.vbank().get(cells[0])?;
    let length = fd.vbank().get(cells[1])?;
    if data.get_size() != 8 || length.get_size() != 8 {
        return None;
    }
    let pointer_bridge = match data.get_type_read_facing(call).get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => true,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    if !matches!(
        length.get_type_read_facing(call).get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    ) {
        return None;
    }
    Some(super::kuna_rustfmtargs::Argument::String {
        data: cells[0],
        length: cells[1],
        pointer_bridge,
    })
}
