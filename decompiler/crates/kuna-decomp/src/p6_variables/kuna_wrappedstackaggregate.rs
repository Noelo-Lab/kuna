//! Recover the contiguous spill beside a joined struct parameter's stack tail.

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

pub(super) fn map(
    fd: &mut Funcdata,
    space: &Rc<AddrSpace>,
    offset: u64,
    indexed: bool,
    ty: Option<&Rc<Datatype>>,
) {
    if indexed {
        return;
    }
    let Some(ty) = ty.filter(|ty| ty.get_metatype() == type_metatype::TYPE_STRUCT) else {
        return;
    };
    let addr = Address::new(Rc::clone(space), offset);
    let Some(pieces) = crate::p0_knowledge::kuna_wrappedstackmap::pieces(&addr, ty.get_size())
    else {
        return;
    };
    let prefix = pieces[0].2 as u64;
    let proto = fd.get_func_proto();
    if !proto.has_store() {
        return;
    }
    let proven = (0..proto.num_params()).any(|i| {
        let Some(param) = proto.get_param(i) else {
            return false;
        };
        if !param.is_type_locked() || !param.get_type().is_some_and(|t| Rc::ptr_eq(t, ty)) {
            return false;
        }
        let input = param.get_address();
        if input
            .get_space()
            .is_none_or(|s| s.get_type() != spacetype::IPTR_JOIN)
        {
            return false;
        }
        let Ok(join) = fd.get_arch().manage().find_join(input.get_offset()) else {
            return false;
        };
        let mut position = 0;
        let tail = join.get_equivalent_address(input.get_offset() + prefix, &mut position);
        if tail != pieces[1].0 {
            return false;
        }
        let piece = join.get_piece(position);
        piece.size as i32 == pieces[1].2
            && (0..join.num_pieces()).all(|j| {
                j == position
                    || join
                        .get_piece(j)
                        .space
                        .as_ref()
                        .is_none_or(|s| !Rc::ptr_eq(s, space))
            })
    });
    if !proven {
        return;
    }
    let mut storage: Vec<_> = pieces
        .iter()
        .map(|(addr, _, size)| kuna_base::space::VarnodeStorage {
            space: addr.get_space().cloned(),
            offset: addr.get_offset(),
            size: *size as u32,
        })
        .collect();
    if !space.is_big_endian() {
        storage.reverse();
    }
    let joined = fd
        .get_arch()
        .manage()
        .find_add_join(&storage, 0)
        .map(|join| join.get_unified().get_addr());
    let result = joined.and_then(|joined| {
        fd.get_scope_local_mut().map_or(Ok(()), |scope| {
            if scope
                .name_for_varnode(&addr, 1)
                .is_some_and(|(_, offset, existing)| {
                    offset == 0 && existing.is_some_and(|existing| Rc::ptr_eq(&existing, ty))
                })
            {
                return Ok(());
            }
            scope
                .add_symbol("", Rc::clone(ty), &joined, &Address::new_invalid())
                .map(|_| ())
        })
    });
    if let Err(error) = result {
        fd.warning_header(&format!("Could not map the spilled struct: {error}"));
    }
}
