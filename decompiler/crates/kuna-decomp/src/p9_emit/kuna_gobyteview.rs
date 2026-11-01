//! Bounded integer views and whole-value reads of Go byte-array storage.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

pub(crate) enum View {
    Whole,
    Integer(Rc<Datatype>),
}

fn byte_array(ty: &Datatype) -> bool {
    ty.get_metatype() == type_metatype::TYPE_ARRAY
        && !crate::kuna_typequal::is_volatile(ty)
        && ty.get_array_base().is_some_and(|base| {
            base.get_size() == 1
                && base.get_align_size() == 1
                && !crate::kuna_typequal::is_volatile(&base)
                && matches!(
                    base.get_metatype(),
                    type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
                )
        })
}

fn whole_view(container: &Datatype, value: &Datatype, offset: i64, size: i32) -> Option<bool> {
    if !byte_array(container)
        || crate::kuna_typequal::is_volatile(value)
        || offset < 0
        || size <= 0
        || value.get_size() != size
        || offset.checked_add(i64::from(size))? > i64::from(container.get_size())
    {
        return None;
    }
    if offset == 0
        && size == container.get_size()
        && (byte_array(value) || (size > 8 && value.get_metatype() == type_metatype::TYPE_UNKNOWN))
    {
        return Some(true);
    }
    (matches!(size, 1 | 2 | 4 | 8)
        && matches!(
            value.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        ))
    .then_some(false)
}

fn value_type(ty: Rc<Datatype>) -> Option<Rc<Datatype>> {
    if crate::kuna_typequal::is_volatile(&ty) {
        return None;
    }
    if ty.get_metatype() != type_metatype::TYPE_PARTIALSTRUCT {
        return Some(ty);
    }
    if !ty.get_name().is_empty() || !ty.get_partial_base().is_some_and(|base| byte_array(&base)) {
        return None;
    }
    let stripped = ty.get_stripped()?;
    matches!(
        stripped.get_metatype(),
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    )
    .then_some(stripped)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn plan(
    fd: &Funcdata,
    arch: &Architecture,
    vn: VarnodeId,
    op: OpId,
    container: &Datatype,
    offset: i64,
    size: i32,
    rendered: Rc<Datatype>,
) -> Option<View> {
    let value = fd.vbank().get(vn)?;
    if value.is_annotation() || value.is_volatile() || value.get_size() != size {
        return None;
    }
    let facing = value_type(if value.get_def() == Some(op) {
        fd.vn_type_def_facing(vn)
    } else {
        fd.vn_type_read_facing(vn, op)
    })?;
    let whole = whole_view(container, &facing, offset, size)?;
    let ty = value_type(rendered)?;
    if whole != whole_view(container, &ty, offset, size)? {
        return None;
    }
    if whole {
        return Some(View::Whole);
    }
    if (ty.get_metatype() == type_metatype::TYPE_UNKNOWN
        || facing.get_metatype() == type_metatype::TYPE_UNKNOWN)
        && crate::kuna_partialaccess::carries_float(fd, vn, op)
    {
        return None;
    }
    let types = arch.types();
    Some(View::Integer(
        types
            .get_type_pointer(types.get_size_of_pointer(), ty, 1)
            .ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{TypeFactory, TypeFactoryImpl};

    fn factory() -> TypeFactoryImpl {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types
            .set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false)
            .unwrap();
        types.cache_core_types().unwrap();
        types
    }

    #[test]
    fn views_preserve_the_exact_bytes_and_value_width() {
        let types = factory();
        let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
        let array = types.get_type_array(16, byte).unwrap();
        for size in [1, 2, 4, 8] {
            let value = types.get_base(size, type_metatype::TYPE_UINT).unwrap();
            assert_eq!(
                whole_view(&array, &value, 16 - i64::from(size), size),
                Some(false)
            );
        }
        let wide = Datatype::new(16, type_metatype::TYPE_UNKNOWN);
        assert_eq!(whole_view(&array, &wide, 0, 16), Some(true));
        assert_eq!(whole_view(&array, &array, 0, 16), Some(true));
        let piece = types.get_type_partial_struct(array.clone(), 8, 8).unwrap();
        let stripped = value_type(piece).unwrap();
        assert_eq!(whole_view(&array, &stripped, 8, 8), Some(false));
    }

    #[test]
    fn incompatible_types_and_unbounded_pieces_stay_explicit() {
        let types = factory();
        let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
        let array = types.get_type_array(16, byte).unwrap();
        let word = types.get_base(8, type_metatype::TYPE_UINT).unwrap();
        for (offset, size) in [(-1, 8), (9, 8), (i64::MAX, 8), (0, 0), (0, 4)] {
            assert_eq!(whole_view(&array, &word, offset, size), None);
        }
        let float = types.get_base(8, type_metatype::TYPE_FLOAT).unwrap();
        assert_eq!(whole_view(&array, &float, 0, 8), None);
        let volatile = types
            .get_qualified_type(array.clone(), crate::kuna_typequal::VOLATILE)
            .unwrap();
        assert_eq!(whole_view(&volatile, &word, 0, 8), None);
        let volatile = types
            .get_qualified_type(word.clone(), crate::kuna_typequal::VOLATILE)
            .unwrap();
        assert_eq!(whole_view(&array, &volatile, 0, 8), None);
        assert!(value_type(volatile).is_none());
        let volatile = types
            .get_qualified_type(
                array.get_array_base().unwrap(),
                crate::kuna_typequal::VOLATILE,
            )
            .unwrap();
        let volatile = types.get_type_array(16, volatile).unwrap();
        assert_eq!(whole_view(&volatile, &word, 0, 8), None);
        let odd = types.get_base(3, type_metatype::TYPE_UNKNOWN).unwrap();
        assert_eq!(whole_view(&array, &odd, 1, 3), None);
        assert_eq!(whole_view(&word, &word, 0, 8), None);
        let words = types.get_type_array(2, word.clone()).unwrap();
        assert_eq!(whole_view(&words, &word, 0, 8), None);
        let piece = types.get_type_partial_struct(words, 0, 8).unwrap();
        assert!(value_type(piece).is_none());
    }
}
