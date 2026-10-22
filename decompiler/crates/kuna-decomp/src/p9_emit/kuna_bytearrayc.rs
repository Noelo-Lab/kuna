//! Opt-in C byte transfers for direct, nonvolatile mapped byte-array accesses.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

pub(crate) struct Access {
    pub name: String,
    pub offset: i32,
    pub size: i32,
    pub big: bool,
}

pub(super) fn byte_array(ty: &Datatype) -> bool {
    ty.get_metatype() == type_metatype::TYPE_ARRAY
        && ty.get_array_base().is_some_and(|elem| {
            elem.get_size() == 1
                && elem.get_align_size() == 1
                && !elem.is_enum_type()
                && matches!(
                    elem.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                )
        })
}

/// Refuse wider containers, noninteger values and accesses without a complete
/// byte-array storage contract. Volatile annotations are never split into bytes.
pub(crate) fn access(fd: &Funcdata, vn: VarnodeId) -> Option<Access> {
    let value = fd.vbank().get(vn)?;
    if value.is_constant() || value.is_annotation() || value.is_volatile() {
        return None;
    }
    let size = value.get_size();
    if !matches!(size, 1 | 2 | 4 | 8) {
        return None;
    }
    let ty = value.get_type();
    if !matches!(
        ty.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    ) && !byte_array(ty)
        && !(ty.get_metatype() == type_metatype::TYPE_PARTIALSTRUCT
            && ty.get_partial_base().is_some_and(|base| byte_array(&base))
            && ty.get_stripped().is_some_and(|base| {
                matches!(
                    base.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                )
            }))
    {
        return None;
    }
    let high = fd.high_bank().get(value.get_high()?)?;
    let global = high.kuna_global()
        || (0..high.num_instances()).any(|i| {
            fd.vbank()
                .get(high.get_instance(i))
                .is_some_and(|v| v.is_persist())
        });
    if !global {
        return None;
    }
    let symbol = high.kuna_symbol_type()?;
    let offset = high.kuna_symbol_offset().max(0);
    if !byte_array(symbol) || offset.checked_add(size)? > symbol.get_size() {
        return None;
    }
    if size == 1 && symbol.get_size() != 1 {
        return None;
    }
    let space = value.get_addr().get_space()?;
    if space.get_word_size() != 1 {
        return None;
    }
    Some(Access {
        name: high.kuna_name()?.to_owned(),
        offset,
        size,
        big: space.is_big_endian(),
    })
}

pub(crate) fn helper(size: i32, big: bool, store: bool) -> String {
    format!(
        "kuna_bytearray_{}{}_{}",
        if store { "store" } else { "load" },
        size * 8,
        if big { "be" } else { "le" }
    )
}

/// A bit-preserving integer CAST to a byte-array view has no C array cast.
pub(crate) fn integer_array_cast(fd: &Funcdata, op: OpId) -> Option<VarnodeId> {
    let operation = fd.obank().get(op)?;
    let out = fd.vbank().get(operation.get_out()?)?;
    let input = operation.get_in(0)?;
    let value = fd.vbank().get(input)?;
    if !matches!(out.get_size(), 1 | 2 | 4 | 8)
        || !byte_array(out.get_type_def_facing())
        || out.get_size() != value.get_size()
    {
        return None;
    }
    let ty = value.get_type_read_facing(op);
    let ty = ty.get_stripped().unwrap_or_else(|| ty.clone());
    matches!(
        ty.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    )
    .then_some(input)
}

/// Each width/endian pair has its own guard so separately emitted functions can
/// be compiled together, including functions that use different transfer widths.
pub(crate) fn preamble(size: i32, big: bool) -> String {
    let bits = size * 8;
    let endian = if big { "BE" } else { "LE" };
    let guard = format!("KUNA_BYTEARRAY_C_{bits}_{endian}");
    let load = helper(size, big, false);
    let store = helper(size, big, true);
    let terms = (0..size)
        .map(|i| {
            let shift = 8 * if big { size - 1 - i } else { i };
            format!("((uint64_t)b[{i}] << {shift})")
        })
        .collect::<Vec<_>>()
        .join(" | ");
    let stores = (0..size)
        .map(|i| {
            let shift = 8 * if big { size - 1 - i } else { i };
            format!("  b[{i}] = (unsigned char)(value >> {shift});\n")
        })
        .collect::<String>();
    format!("#ifndef {guard}\n#define {guard}\n#include <stdint.h>\n#include <limits.h>\n_Static_assert(CHAR_BIT == 8, \"bytearrayc requires 8-bit bytes\");\nstatic inline uint{bits}_t {load}(const void *p)\n{{\n  const unsigned char *b = p;\n  return (uint{bits}_t)({terms});\n}}\nstatic inline void {store}(void *p, uint{bits}_t value)\n{{\n  unsigned char *b = p;\n{stores}}}\n#endif\n")
}
