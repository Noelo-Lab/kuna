//! Incoming parameter values and later objects share frame storage, not variable identity.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::context::{HighVariableId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_stackobjects::{ByteSource, SourcePoint, StackObject};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub(crate) fn can_map_backing(fd: &Funcdata, start: u64, size: i32) -> bool {
    let Some(scope) = fd.get_scope_local() else {
        return false;
    };
    let address = Address::new(Rc::clone(scope.get_space_id()), start);
    let invalid = Address::new_invalid();
    scope.in_scope(&address, size)
        && !parameter_types(fd, start, size).is_empty()
        && scope
            .query_container_for_link_matching(&address, size, &invalid, |symbol| {
                symbol.get_category() != crate::database::symbol_category::FUNCTION_PARAMETER
            })
            .is_none()
}

pub(crate) fn map_backing(fd: &mut Funcdata, start: u64, datatype: Rc<Datatype>) -> bool {
    let Some(scope) = fd.get_scope_local_mut() else {
        return false;
    };
    let address = Address::new(Rc::clone(scope.get_space_id()), start);
    scope
        .add_symbol("", datatype, &address, &Address::new_invalid())
        .is_ok()
}

pub(crate) fn parameter_types(fd: &Funcdata, start: u64, size: i32) -> Vec<(i32, Rc<Datatype>)> {
    if !fd.get_func_proto().has_store() {
        return Vec::new();
    }
    let Some(scope) = fd.get_scope_local() else {
        return Vec::new();
    };
    let address = Address::new(Rc::clone(scope.get_space_id()), start);
    (0..fd.get_func_proto().num_params())
        .filter_map(|index| {
            let parameter = fd.get_func_proto().get_param(index)?;
            let datatype = parameter.get_type()?;
            let offset = parameter.get_address().overlap(0, &address, size);
            (offset >= 0 && offset as i64 + datatype.get_size() as i64 <= size as i64)
                .then(|| (offset, Rc::clone(datatype)))
        })
        .collect()
}

pub(crate) fn backing_for_address(
    fd: &Funcdata,
    address: &Address,
    size: i32,
) -> Option<crate::varmap::LinkEntryInfo> {
    if !fd.get_arch().stack_views || size <= 0 {
        return None;
    }
    let scope = fd.get_scope_local()?;
    let invalid = Address::new_invalid();
    let backing = scope.query_container_for_link_matching(address, size, &invalid, |symbol| {
        symbol.get_category() != crate::database::symbol_category::FUNCTION_PARAMETER
            && symbol
                .dtype
                .as_ref()
                .is_some_and(|datatype| datatype.get_name().starts_with("stack_views_"))
    })?;
    if parameter_types(fd, backing.entry_addr.get_offset(), backing.entry_size).is_empty() {
        return None;
    }
    Some(backing)
}

fn shared_backing(fd: &Funcdata, object: &StackObject) -> Option<crate::varmap::LinkEntryInfo> {
    backing_for_address(fd, &object.address, object.size)
}

/// Keep the ABI input separate from writes to its reused physical slot.
fn initialize_parameters(
    fd: &mut Funcdata,
    info: &crate::varmap::LinkEntryInfo,
    factory: &dyn TypeFactory,
) -> Option<()> {
    let backing = info.sym_type.as_ref()?;
    let parameters: Vec<_> = (0..fd.get_func_proto().num_params())
        .filter_map(|index| {
            let parameter = fd.get_func_proto().get_param(index)?;
            let datatype = parameter.get_type()?;
            let address = parameter.get_address();
            let offset = address.overlap(0, &info.entry_addr, info.entry_size);
            if offset < 0 || offset as i64 + datatype.get_size() as i64 > info.entry_size as i64 {
                return None;
            }
            let scope = fd.get_scope_local()?;
            let symbol = scope
                .query_container_for_link_matching(
                    &address,
                    datatype.get_size(),
                    &Address::new_invalid(),
                    |symbol| {
                        symbol.get_category()
                            == crate::database::symbol_category::FUNCTION_PARAMETER
                    },
                )?
                .symbol;
            Some((
                crate::database::kuna_materialized_param_name(
                    fd.get_arch().name_style_angr,
                    index,
                    parameter.get_name(),
                ),
                address,
                offset,
                Rc::clone(datatype),
                symbol,
            ))
        })
        .collect();
    for (name, address, offset, datatype, parameter_symbol) in parameters {
        let member = (0..backing.num_depend()).find(|&index| {
            backing.get_field(index).is_some_and(|field| {
                (offset == 0 && Rc::ptr_eq(&field.field_type, &datatype))
                    || (field.field_type.get_name().starts_with("stack_slice_")
                        && field.field_type.get_field(0).is_some_and(|piece| {
                            piece.offset == offset && Rc::ptr_eq(&piece.field_type, &datatype)
                        }))
            })
        })?;
        let source = fd.new_varnode(datatype.get_size(), &address, Some(Rc::clone(&datatype)));
        let source = fd.set_input_varnode(source).ok()?;
        let mut high = fd.vbank().get(source)?.get_high()?;
        let shared = fd.high_bank().get(high).is_some_and(|h| {
            (0..h.num_instances()).any(|index| {
                fd.vbank().get(h.get_instance(index)).is_some_and(|value| {
                    value.is_written()
                        && backing_for_address(fd, value.get_addr(), value.get_size())
                            .is_some_and(|entry| entry.symbol == info.symbol)
                })
            })
        });
        if shared {
            fd.high_remove_member(high, source);
            high = fd.assign_high_var(source)?;
        }
        let h = fd.high_bank_mut().get_mut(high)?;
        h.set_kuna_name(name);
        h.set_symbol_offset(-1);
        h.set_symbol_type(Rc::clone(&datatype));
        h.set_kuna_link_symbol(parameter_symbol);
        let block = fd.bblocks_get_block(0);
        let point = fd.bblocks_block_start(block);
        let op = fd.new_op(1, point);
        fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
        let out = fd.new_varnode_out(datatype.get_size(), &address, op).ok()?;
        fd.vn_update_type(out, datatype);
        fd.vbank_mut().get_mut(out)?.set_explicit();
        let high = fd.vbank().get(out)?.get_high()?;
        let h = fd.high_bank_mut().get_mut(high)?;
        h.set_kuna_name(info.display_name.clone());
        h.set_symbol_offset(offset);
        h.set_symbol_type(Rc::clone(backing));
        h.set_kuna_link_symbol(info.symbol);
        let mut resolution =
            crate::unionresolve::ResolvedUnion::new_field(Rc::clone(backing), member, factory)
                .ok()?;
        resolution.set_lock(true);
        fd.set_union_field(backing, op, -1, resolution);
        fd.stack_write_views.insert(
            op,
            crate::kuna_stackviews::StorageView {
                backing: Rc::clone(backing),
                offset,
            },
        );
        fd.op_set_input(op, source, 0).ok()?;
        fd.op_insert_begin(op, block);
    }
    bind_values(fd, info);
    Some(())
}

/// Scalar slots also need backing when their addresses have wider observers.
pub(crate) fn needs_backing(fd: &Funcdata, start: u64, size: i32) -> bool {
    let first = start as i64 as i128;
    let end = first + size as i128;
    let aliases = |input| {
        crate::kuna_stackranges::frame_address(fd, input)
            .filter(|address| (address.first as i128) < end && address.last as i128 >= first)
    };
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else {
            continue;
        };
        match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                for slot in 1..op.num_input() {
                    let Some(address) = op.get_in(slot).and_then(aliases) else {
                        continue;
                    };
                    if address.first != address.last
                        || !address.terms.is_empty()
                        || address.first as i128 != first
                        || crate::kuna_stackviews::declared_view(fd, id, slot)
                            .is_none_or(|ty| ty.get_size() != size)
                    {
                        return true;
                    }
                }
            }
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_MULTIEQUAL => {
                if op
                    .get_out()
                    .and_then(|out| fd.vbank().get(out))
                    .is_some_and(|value| value.is_persist() || value.is_stack_store())
                    && (0..op.num_input()).any(|slot| op.get_in(slot).and_then(aliases).is_some())
                {
                    return true;
                }
            }
            OpCode::CPUI_STORE
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_CALLOTHER => {
                if (1..op.num_input()).any(|slot| op.get_in(slot).and_then(aliases).is_some()) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn bind_values(fd: &mut Funcdata, info: &crate::varmap::LinkEntryInfo) {
    let Some(space) = info.entry_addr.get_space() else {
        return;
    };
    let mut values = BTreeMap::new();
    for id in fd.vbank().loc_space_ids(space) {
        let Some(value) = fd.vbank().get(id) else {
            continue;
        };
        if value.is_input() || value.is_free() {
            continue;
        }
        let offset = value
            .get_addr()
            .overlap(0, &info.entry_addr, info.entry_size);
        if offset < 0 || offset as i64 + value.get_size() as i64 > info.entry_size as i64 {
            continue;
        }
        if let Some(high) = value.get_high() {
            values
                .entry(high)
                .or_insert_with(BTreeSet::new)
                .insert(offset);
        }
    }
    for (high, offsets) in values {
        if offsets.len() != 1 {
            continue;
        }
        let has_input = fd.high_bank().get(high).is_none_or(|h| {
            (0..h.num_instances()).any(|index| {
                fd.vbank()
                    .get(h.get_instance(index))
                    .is_some_and(|value| value.is_input())
            })
        });
        if has_input {
            continue;
        }
        let h = fd.high_bank_mut().get_mut(high).unwrap();
        h.set_kuna_name(info.display_name.clone());
        h.set_symbol_offset(*offsets.first().unwrap());
        h.set_symbol_type(Rc::clone(info.sym_type.as_ref().unwrap()));
        h.set_kuna_link_symbol(info.symbol);
    }
}

fn aliases(fd: &Funcdata, input: VarnodeId, object: &StackObject) -> bool {
    let Some(address) = crate::kuna_stackranges::frame_address(fd, input) else {
        return false;
    };
    let start = object.address.get_offset() as i64 as i128;
    (address.first as i128) < start + object.size as i128 && address.last as i128 >= start
}

fn closed_reuse(fd: &Funcdata, object: &StackObject) -> bool {
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                for slot in 1..op.num_input() {
                    let Some(input) = op.get_in(slot) else {
                        return false;
                    };
                    if aliases(fd, input, object)
                        && !fd.stack_objects.iter().any(|other| {
                            other.address == object.address
                                && other.size == object.size
                                && other.is_defined()
                                && other
                                    .uses
                                    .iter()
                                    .any(|use_| use_.op == id && use_.slot == slot)
                        })
                    {
                        return false;
                    }
                }
            }
            OpCode::CPUI_COPY => {
                if op
                    .get_out()
                    .and_then(|out| fd.vbank().get(out))
                    .is_some_and(|v| v.is_persist())
                    && op.get_in(0).is_some_and(|input| aliases(fd, input, object))
                {
                    return false;
                }
            }
            OpCode::CPUI_STORE
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_CALLOTHER => {
                if (1..op.num_input()).any(|slot| {
                    op.get_in(slot)
                        .is_some_and(|input| aliases(fd, input, object))
                }) {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

pub(crate) fn separates_parameter(
    fd: &Funcdata,
    first: HighVariableId,
    second: HighVariableId,
) -> bool {
    if !fd.get_arch().stack_views || first == second {
        return false;
    }
    [(first, second), (second, first)]
        .into_iter()
        .any(|(input, local)| {
            let Some(input) = fd.high_bank().get(input) else {
                return false;
            };
            let Some(local) = fd.high_bank().get(local) else {
                return false;
            };
            if (0..local.num_instances()).any(|i| {
                fd.vbank()
                    .get(local.get_instance(i))
                    .is_some_and(|v| v.is_input())
            }) {
                return false;
            }
            (0..input.num_instances()).any(|i| {
                let Some(parameter) = fd
                    .vbank()
                    .get(input.get_instance(i))
                    .filter(|v| v.is_input())
                else {
                    return false;
                };
                let Some(scope) = fd.get_scope_local() else {
                    return false;
                };
                let Some(mapped) = scope.query_container_for_link_matching(
                    parameter.get_addr(),
                    parameter.get_size(),
                    &Address::new_invalid(),
                    |symbol| {
                        symbol.get_category()
                            == crate::database::symbol_category::FUNCTION_PARAMETER
                    },
                ) else {
                    return false;
                };
                if mapped.category != crate::database::symbol_category::FUNCTION_PARAMETER
                    || mapped.entry_addr != *parameter.get_addr()
                    || mapped.entry_size != parameter.get_size()
                {
                    return false;
                }
                if let Some(backing) =
                    backing_for_address(fd, parameter.get_addr(), parameter.get_size())
                {
                    if (0..local.num_instances()).any(|i| {
                        fd.vbank().get(local.get_instance(i)).is_some_and(|value| {
                            backing_for_address(fd, value.get_addr(), value.get_size())
                                .is_some_and(|entry| entry.symbol == backing.symbol)
                        })
                    }) {
                        return true;
                    }
                }
                (0..local.num_instances()).any(|i| {
                    let Some(value) = fd.vbank().get(local.get_instance(i)) else {
                        return false;
                    };
                    value.is_stack_store()
                        && value.get_addr() == parameter.get_addr()
                        && value.get_size() > parameter.get_size()
                        && fd.stack_objects.iter().any(|object| {
                            object.address == *value.get_addr()
                                && object.size == value.get_size()
                                && object.is_defined()
                                && closed_reuse(fd, object)
                        })
                })
            })
        })
}

fn storage(fd: &Funcdata, object: &StackObject) -> Option<VarnodeId> {
    if !object.is_defined() {
        return None;
    }
    let space = object.address.get_space()?;
    let scope = fd.get_scope_local()?;
    let mapped = scope.query_container_for_link(&object.address, &Address::new_invalid())?;
    if mapped.category != crate::database::symbol_category::FUNCTION_PARAMETER
        || mapped.entry_addr != object.address
        || mapped.entry_size >= object.size
    {
        return None;
    }
    if !closed_reuse(fd, object) {
        return None;
    }
    let mut points = BTreeSet::new();
    for byte in &object.bytes {
        for source in byte {
            let ByteSource::Write(point) = source else {
                return None;
            };
            points.insert(*point);
        }
    }
    let mut definitions = BTreeMap::new();
    for id in fd.vbank().loc_space_ids(space) {
        let v = fd.vbank().get(id)?;
        if v.get_addr() != &object.address || v.get_size() != object.size || !v.is_stack_store() {
            continue;
        }
        let Some(def) = v.get_def() else { continue };
        let Some(point) = SourcePoint::op(fd, def) else {
            continue;
        };
        if !points.contains(&point) || fd.obank().get(def)?.is_marker() {
            continue;
        }
        let high = v.get_high()?;
        if definitions.insert(point, (high, id)).is_some() {
            return None;
        }
    }
    if points.len() != definitions.len() {
        return None;
    }
    let mut selected: Option<(HighVariableId, VarnodeId)> = None;
    for (_, (high, id)) in definitions {
        if selected.is_some_and(|(old, _)| old != high) {
            return None;
        }
        selected = Some((high, id));
    }
    let (high, id) = selected?;
    let h = fd.high_bank().get(high)?;
    if h.kuna_name().is_none()
        || (0..h.num_instances()).any(|i| {
            fd.vbank()
                .get(h.get_instance(i))
                .is_some_and(|v| v.is_input())
        })
    {
        return None;
    }
    let ty = fd.vbank().get(id)?.get_type();
    if !matches!(
        ty.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    ) || object.uses.iter().any(|use_| {
        !matches!(
            use_.datatype.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT
        )
    }) {
        return None;
    }
    Some(id)
}

pub(crate) fn prepare_casts(fd: &mut Funcdata) {
    let Some(factory) = fd.get_arch().types_rc() else {
        return;
    };
    let objects = fd.stack_objects.clone();
    let mut initialized = BTreeSet::new();
    for object in objects {
        if let Some(backing) = shared_backing(fd, &object) {
            if initialized.insert(backing.entry_addr.get_offset()) {
                initialize_parameters(fd, &backing, factory.as_ref());
            }
            bind_uses(
                fd,
                &object,
                &backing.sym_type.unwrap(),
                &backing.display_name,
                backing.symbol,
                backing.sym_off,
                factory.as_ref(),
            );
            continue;
        }
        let Some(storage) = storage(fd, &object) else {
            continue;
        };
        let Some(value) = fd.vbank().get(storage) else {
            continue;
        };
        let Some(high) = value.get_high().and_then(|high| fd.high_bank().get(high)) else {
            continue;
        };
        let high_id = value.get_high().unwrap();
        let name = high.kuna_name().unwrap().to_string();
        let datatype = if value.get_type().get_metatype() == type_metatype::TYPE_UNKNOWN {
            let Ok(ty) = factory.get_base(value.get_size(), type_metatype::TYPE_UINT) else {
                continue;
            };
            ty
        } else {
            Rc::clone(value.get_type())
        };
        let invalid = Address::new_invalid();
        let existing = fd.get_scope_local().and_then(|scope| {
            scope.query_container_for_link_width(&object.address, object.size, &invalid)
        });
        let (name, symbol) = match existing {
            Some(info) if info.entry_addr == object.address && info.entry_size == object.size => {
                (info.display_name, info.symbol)
            }
            Some(_) => continue,
            None => {
                let scope = fd.get_scope_local_mut().unwrap();
                let name = scope.make_local_name_unique(&name);
                let Ok(symbol) =
                    scope.add_symbol(&name, Rc::clone(&datatype), &object.address, &invalid)
                else {
                    continue;
                };
                (name, symbol)
            }
        };
        let h = fd.high_bank_mut().get_mut(high_id).unwrap();
        h.set_kuna_name(name.clone());
        h.set_symbol_offset(-1);
        h.set_symbol_type(Rc::clone(&datatype));
        h.set_kuna_link_symbol(symbol);
        bind_uses(fd, &object, &datatype, &name, symbol, 0, factory.as_ref());
    }
}

fn bind_uses(
    fd: &mut Funcdata,
    object: &StackObject,
    datatype: &Rc<Datatype>,
    name: &str,
    symbol: crate::database::SymbolId,
    symbol_offset: i32,
    factory: &dyn TypeFactory,
) {
    let Some(space) = object.address.get_space() else {
        return;
    };
    for use_ in &object.uses {
        let call = use_.op;
        let Some(input) = fd.obank().get(call).and_then(|op| op.get_in(use_.slot)) else {
            continue;
        };
        let Some(address) = crate::kuna_stackranges::frame_address(fd, input) else {
            continue;
        };
        if address.first != address.last
            || !address.terms.is_empty()
            || space.wrap_offset(address.first as u64) != object.address.get_offset()
        {
            continue;
        }
        let width = fd.vbank().get(input).unwrap().get_size();
        if datatype.get_name().starts_with("stack_views_") {
            let Some(info) = shared_backing(fd, object) else {
                continue;
            };
            let member = (0..datatype.num_depend()).find(|&index| {
                datatype.get_depend(index).is_some_and(|member| {
                    (info.sym_off == 0 && Rc::ptr_eq(&member, &use_.datatype))
                        || (member.get_name().starts_with("stack_slice_")
                            && member.get_field(0).is_some_and(|field| {
                                field.offset == info.sym_off
                                    && Rc::ptr_eq(&field.field_type, &use_.datatype)
                            }))
                })
            });
            if let Some(value) = member.and_then(|member| {
                crate::kuna_stackobjectasserts::backing_view(
                    fd,
                    call,
                    width,
                    &info,
                    member,
                    &use_.datatype,
                    factory,
                )
            }) {
                let _ = fd.op_set_input(call, value, use_.slot);
            }
            continue;
        }
        let Ok(pointer) =
            factory.get_type_pointer(width, Rc::clone(datatype), space.get_word_size())
        else {
            continue;
        };
        let Ok(base) = fd.construct_spacebase_input(space) else {
            continue;
        };
        let point = fd.obank().get(call).unwrap().get_addr().clone();
        let op = fd.new_op(2, point);
        fd.op_set_opcode_code(op, OpCode::CPUI_PTRSUB);
        let Ok(out) = fd.new_unique_out(width, op) else {
            continue;
        };
        fd.vn_update_type(out, pointer);
        let offset = fd.new_constant(width, object.address.get_offset());
        if fd.op_set_all_input(op, &[base, offset]).is_err() {
            continue;
        }
        fd.vbank_mut().get_mut(out).unwrap().set_implied();
        let Some(reference) = fd.vbank().get(offset).and_then(|v| v.get_high()) else {
            continue;
        };
        let h = fd.high_bank_mut().get_mut(reference).unwrap();
        h.set_kuna_name(name.to_string());
        h.set_symbol_offset(symbol_offset);
        h.set_symbol_type(Rc::clone(datatype));
        h.set_kuna_ref_symbol(symbol);
        fd.op_insert_before(op, call);
        let _ = fd.op_set_input(call, out, use_.slot);
    }
}

#[cfg(test)]
mod tests;
