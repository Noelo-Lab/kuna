//! Coalesced register instances keep the scalar view of their shared frame symbol.

use std::rc::Rc;

use crate::dtype::{type_metatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::unionresolve::ResolvedUnion;

pub(crate) fn prepare(fd: &mut Funcdata, factory: &dyn TypeFactory) {
    let mut edges = Vec::new();
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id).unwrap();
        if op.is_marker() || op.is_return_copy() {
            continue;
        }
        for slot in -1..op.num_input() {
            let value = if slot < 0 {
                op.get_out()
            } else {
                op.get_in(slot)
            };
            let Some(value) = value.and_then(|id| fd.vbank().get(id)) else {
                continue;
            };
            if value.is_constant() {
                continue;
            }
            let Some(high) = value.get_high().and_then(|id| fd.high_bank().get(id)) else {
                continue;
            };
            let Some(backing) = high.kuna_symbol_type() else {
                continue;
            };
            if !backing.get_name().starts_with("stack_views_") {
                continue;
            }
            let offset = high.get_symbol_offset().max(0);
            let ty = value.get_type();
            if let Some(resolution) = fd.get_union_resolution(backing, id, slot) {
                let member = resolution.get_field_num();
                if member >= 0 {
                    let aggregate = ty.get_partial_base().unwrap_or_else(|| Rc::clone(ty));
                    let declared_aggregate = matches!(
                        aggregate.get_metatype(),
                        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
                    ) && !aggregate.get_name().starts_with("stack_views_");
                    let declared_aggregate = declared_aggregate
                        || (value.is_type_lock()
                            && aggregate.get_metatype() == type_metatype::TYPE_ARRAY);
                    if declared_aggregate
                        || backing.get_depend(member).is_some_and(|member| {
                            crate::kuna_stackviews::scalar_piece(
                                factory,
                                member,
                                offset,
                                value.get_size(),
                            )
                            .is_some()
                        })
                    {
                        continue;
                    }
                }
            }
            let wanted = match ty.get_metatype() {
                type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_PTR
                | type_metatype::TYPE_FLOAT => ty.get_metatype(),
                _ => type_metatype::TYPE_UINT,
            };
            let piece = |index| {
                crate::kuna_stackviews::scalar_piece(
                    factory,
                    backing.get_depend(index)?,
                    offset,
                    value.get_size(),
                )
            };
            let member = (0..backing.num_depend())
                .find(|&index| piece(index).is_some_and(|piece| Rc::ptr_eq(&piece, ty)))
                .or_else(|| {
                    (0..backing.num_depend()).find(|&index| {
                        piece(index).is_some_and(|piece| piece.get_metatype() == wanted)
                    })
                });
            if let Some(member) = member {
                edges.push((id, slot, Rc::clone(backing), offset, member));
            }
        }
    }
    for (id, slot, backing, offset, member) in edges {
        let Ok(mut resolution) = ResolvedUnion::new_field(Rc::clone(&backing), member, factory)
        else {
            continue;
        };
        resolution.set_lock(true);
        fd.set_union_field(&backing, id, slot, resolution);
        if slot < 0 {
            fd.stack_write_views
                .insert(id, crate::kuna_stackviews::StorageView { backing, offset });
        }
    }
}
