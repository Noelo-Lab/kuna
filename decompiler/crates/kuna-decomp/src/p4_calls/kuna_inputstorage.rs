//! Explicit argument storage reserves resources before default ABI allocation.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpaceManager};

use crate::dtype::{type_class, TypeFactory};
use crate::fspec::{
    parameter_pieces_flags, AssignActionResponse, ParamListStandard, ParameterPieces,
    PrototypePieces,
};

#[derive(Clone)]
struct Range {
    space: i32,
    start: u64,
    end: u64,
}

fn ranges(address: &Address, size: i32, manager: &AddrSpaceManager) -> KunaResult<Vec<Range>> {
    let error = || KunaError::param_unassigned("Invalid explicit parameter storage");
    let space = address.get_space().ok_or_else(error)?;
    if size <= 0 {
        return Err(error());
    }
    if space.get_type() == spacetype::IPTR_JOIN {
        let join = manager.find_join(address.get_offset())?;
        if join.get_unified().offset != address.get_offset()
            || join.get_unified().size != size as u32
        {
            return Err(error());
        }
        let mut result = Vec::new();
        for i in 0..join.num_pieces() {
            let piece = join.get_piece(i);
            let space = piece.space.as_ref().ok_or_else(error)?;
            if space.get_type() == spacetype::IPTR_JOIN {
                return Err(error());
            }
            let end = piece
                .offset
                .checked_add(u64::from(piece.size))
                .ok_or_else(error)?;
            result.push(Range {
                space: space.get_index(),
                start: piece.offset,
                end,
            });
        }
        return Ok(result);
    }
    let end = address
        .get_offset()
        .checked_add(size as u64)
        .ok_or_else(error)?;
    Ok(vec![Range {
        space: space.get_index(),
        start: address.get_offset(),
        end,
    }])
}

fn overlaps(a: &Range, b: &Range) -> bool {
    a.space == b.space && a.start < b.end && b.start < a.end
}

fn allocate(
    input: &ParamListStandard,
    reserved: &[Range],
    manager: &AddrSpaceManager,
    status: &mut [i32],
    piece: &mut ParameterPieces,
    mut assign: impl FnMut(&mut [i32], &mut ParameterPieces) -> KunaResult<AssignActionResponse>,
) -> KunaResult<AssignActionResponse> {
    loop {
        let previous = status.to_vec();
        let response = assign(status, piece)?;
        if matches!(
            response,
            AssignActionResponse::fail | AssignActionResponse::no_assignment
        ) {
            return Ok(response);
        }
        let size = piece.type_.as_ref().map_or(0, |ty| ty.get_size());
        let occupied = ranges(&piece.addr, size, manager)?;
        if !occupied
            .iter()
            .any(|a| reserved.iter().any(|b| overlaps(a, b)))
        {
            return Ok(response);
        }
        for entry in input
            .get_entry()
            .iter()
            .filter(|entry| !entry.is_exclusion())
        {
            let space = entry.get_space();
            if space.get_type() == spacetype::IPTR_JOIN {
                continue;
            }
            let Some(end) = entry.get_base().checked_add(entry.get_size() as u64) else {
                continue;
            };
            let pool = Range {
                space: space.get_index(),
                start: entry.get_base(),
                end,
            };
            for blocked in reserved.iter().filter(|blocked| {
                overlaps(blocked, &pool)
                    && occupied
                        .iter()
                        .any(|candidate| overlaps(candidate, blocked))
            }) {
                let boundary = if entry.is_reverse_stack() {
                    blocked.start.max(pool.start)
                } else {
                    blocked.end.min(pool.end) - 1
                };
                let slot = entry
                    .get_slot(&Address::new(space.clone(), boundary), 0)
                    .checked_sub(entry.get_group())
                    .and_then(|slot| slot.checked_add(1))
                    .ok_or_else(|| {
                        KunaError::param_unassigned("Explicit parameter slot overflow")
                    })?;
                if let Some(value) = status.get_mut(entry.get_group() as usize) {
                    *value = (*value).max(slot);
                }
            }
        }
        if previous == status {
            return Err(KunaError::param_unassigned(
                "Default argument allocation overlaps explicit storage",
            ));
        }
    }
}

pub(crate) fn assign(
    input: &ParamListStandard,
    proto: &PrototypePieces,
    types: &dyn TypeFactory,
    result: &mut Vec<ParameterPieces>,
    manager: &AddrSpaceManager,
    status: &mut [i32],
) -> KunaResult<()> {
    let mut custom = BTreeMap::new();
    for (slot, piece) in &proto.input_storage {
        let Ok(slot) = usize::try_from(*slot) else {
            continue;
        };
        let Some(default) = proto.intypes.get(slot) else {
            continue;
        };
        let mut piece = piece.clone();
        piece.type_.get_or_insert_with(|| Rc::clone(default));
        custom.insert(slot, piece);
    }
    let mut reserved = Vec::new();
    for piece in custom.values() {
        reserved.extend(ranges(
            &piece.addr,
            piece.type_.as_ref().unwrap().get_size(),
            manager,
        )?);
    }
    for entry in input
        .get_entry()
        .iter()
        .filter(|entry| entry.is_exclusion())
    {
        let occupied = ranges(
            &Address::new(entry.get_space().clone(), entry.get_base()),
            entry.get_size(),
            manager,
        )?;
        if occupied
            .iter()
            .any(|a| reserved.iter().any(|b| overlaps(a, b)))
        {
            for &group in entry.get_all_groups() {
                let value = status.get_mut(group as usize).ok_or_else(|| {
                    KunaError::param_unassigned("Invalid parameter resource group")
                })?;
                *value = -1;
            }
        }
    }
    if result.len() == 2 {
        let mut hidden = result.pop().unwrap();
        let datatype = hidden
            .type_
            .clone()
            .ok_or_else(|| KunaError::param_unassigned("Missing hidden parameter type"))?;
        let is_hidden = hidden.flags & parameter_pieces_flags::HIDDENRETPARM != 0;
        let response = allocate(
            input,
            &reserved,
            manager,
            status,
            &mut hidden,
            |status, piece| {
                if is_hidden {
                    input.assign_address_fallback(
                        type_class::TYPECLASS_HIDDENRET,
                        &datatype,
                        false,
                        status,
                        piece,
                        manager,
                    )
                } else {
                    input.assign_address(&datatype, proto, 0, types, status, piece, manager)
                }
            },
        )?;
        if matches!(
            response,
            AssignActionResponse::fail | AssignActionResponse::no_assignment
        ) {
            return Err(KunaError::param_unassigned(
                "Cannot allocate hidden return input beside explicit storage",
            ));
        }
        hidden.flags |= parameter_pieces_flags::HIDDENRETPARM;
        result.push(hidden);
    }
    for (index, datatype) in proto.intypes.iter().enumerate() {
        if let Some(piece) = custom.remove(&index) {
            result.push(piece);
            continue;
        }
        let mut piece = ParameterPieces::default();
        let response = allocate(
            input,
            &reserved,
            manager,
            status,
            &mut piece,
            |status, piece| {
                input.assign_address(datatype, proto, index as i32, types, status, piece, manager)
            },
        )?;
        if matches!(
            response,
            AssignActionResponse::fail | AssignActionResponse::no_assignment
        ) {
            return Err(KunaError::param_unassigned(
                "Cannot allocate default input beside explicit storage",
            ));
        }
        result.push(piece);
    }
    Ok(())
}

/// Local console maps and qualified declarations share the same assignment path.
pub(crate) fn merge_mapped<'a>(
    prototype: Option<&'a PrototypePieces>,
    mapped: &[(i32, String, ParameterPieces)],
) -> Option<Cow<'a, PrototypePieces>> {
    let prototype = prototype?;
    if mapped.is_empty() {
        return Some(Cow::Borrowed(prototype));
    }
    let mut pieces = prototype.clone();
    for (index, name, piece) in mapped {
        let Ok(slot) = usize::try_from(*index) else {
            continue;
        };
        let Some(datatype) = pieces.intypes.get_mut(slot) else {
            continue;
        };
        if let Some(ty) = &piece.type_ {
            *datatype = ty.clone();
        }
        pieces.innames.resize(pieces.intypes.len(), String::new());
        pieces.innames[slot] = name.clone();
        pieces.input_storage.retain(|(i, _)| i != index);
        pieces.input_storage.push((*index, piece.clone()));
    }
    Some(Cow::Owned(pieces))
}
