//! Native-use assertions for logical objects over shared physical frame storage.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::Datatype;
use crate::dtype::{type_metatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_stackobjects::{ByteSource, SourcePoint};
use crate::unionresolve::{ResolveEdge, ResolvedUnion};
use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

#[derive(Clone)]
pub struct UseAnchor {
    pub point: SourcePoint,
    pub slot: i32,
    pub datatype: Rc<Datatype>,
}

#[derive(Clone)]
pub struct ObjectAssertion {
    pub selector: String,
    pub name: String,
    pub address: Address,
    pub size: i32,
    pub datatype: Option<Rc<Datatype>>,
    pub uses: Vec<UseAnchor>,
    pub bytes: Vec<BTreeSet<ByteSource>>,
}

impl ObjectAssertion {
    pub(crate) fn view_name(&self, ty: &Datatype) -> String {
        let types: BTreeSet<_> = self
            .uses
            .iter()
            .map(|use_| use_.datatype.get_id())
            .collect();
        if self.datatype.is_some() || types.len() == 1 {
            return self.name.clone();
        }
        let label: String = ty
            .get_name()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        format!("{}_{}", self.name, label)
    }
}

pub fn specs(fd: &Funcdata) -> &[ObjectAssertion] {
    &fd.stack_object_assertions
}

pub fn seed(fd: &mut Funcdata, assertions: &[ObjectAssertion]) {
    fd.stack_object_assertions = assertions.to_vec();
}

/// Return false when the name belongs to the ordinary symbol/high-variable plane.
pub fn apply(
    fd: &mut Funcdata,
    target: &str,
    newname: &str,
    datatype: Option<Rc<Datatype>>,
) -> Result<bool, String> {
    let objects = crate::kuna_stackobjects::objects(fd);
    let original: Vec<_> = objects
        .iter()
        .enumerate()
        .filter(|(_, o)| o.name == target)
        .collect();
    let given: Vec<_> = fd
        .stack_object_assertions
        .iter()
        .filter(|assertion| assertion.name == target && assertion.selector != target)
        .collect();
    if original.len() > 1 || given.len() > 1 {
        return Err(format!("More than one stack object named: {target}"));
    }
    if let (Some((_, object)), Some(other)) = (original.first(), given.first()) {
        if object.name != other.selector
            && !(datatype.is_none() && objects.iter().any(|object| object.name == newname))
        {
            return Err(format!("Ambiguous stack object name: {target}"));
        }
    }
    let selected = original.first().map(|(index, _)| *index).or_else(|| {
        given.first().and_then(|assertion| {
            objects
                .iter()
                .position(|object| object.name == assertion.selector)
        })
    });
    let Some(index) = selected else {
        return Ok(false);
    };
    let object = &objects[index];
    if let Some(ty) = &datatype {
        if ty.is_incomplete() || ty.get_size() != object.size {
            return Err(format!(
                "Stack object {} occupies {} bytes; the asserted type occupies {}",
                object.name,
                object.size,
                ty.get_size()
            ));
        }
    }
    if !newname.is_empty() {
        let mut chars = newname.chars();
        if !chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(format!("Invalid C object name: {newname}"));
        }
    }
    let mut assertion = fd
        .stack_object_assertions
        .iter()
        .find(|assertion| assertion.selector == object.name)
        .cloned()
        .unwrap_or_else(|| ObjectAssertion {
            selector: object.name.clone(),
            name: object.name.clone(),
            address: object.address.clone(),
            size: object.size,
            datatype: None,
            uses: object
                .uses
                .iter()
                .map(|use_| UseAnchor {
                    point: use_.point,
                    slot: use_.slot,
                    datatype: Rc::clone(&use_.datatype),
                })
                .collect(),
            bytes: object.bytes.clone(),
        });
    if !newname.is_empty() {
        assertion.name = newname.to_string();
    }
    if datatype.is_some() {
        assertion.datatype = datatype;
    }
    if fd
        .stack_object_assertions
        .iter()
        .any(|old| old.selector != assertion.selector && old.name == assertion.name)
    {
        return Err(format!(
            "Stack object name already used: {}",
            assertion.name
        ));
    }
    fd.stack_object_assertions
        .retain(|old| old.selector != assertion.selector);
    fd.stack_object_assertions.push(assertion);
    Ok(true)
}

pub(crate) fn validate(fd: &Funcdata) -> Result<(), String> {
    for assertion in specs(fd) {
        if !fd.get_arch().stack_views {
            return Err("Logical stack object assertions require stackviews on".to_string());
        }
        let found = crate::kuna_stackobjects::objects(fd).iter().any(|object| {
            object.address == assertion.address
                && object.size == assertion.size
                && object.bytes == assertion.bytes
                && assertion.uses.iter().all(|anchor| {
                    object
                        .uses
                        .iter()
                        .any(|use_| use_.point == anchor.point && use_.slot == anchor.slot)
                })
        });
        if !found {
            return Err(format!(
                "Stack object {} no longer matches its native uses",
                assertion.selector
            ));
        }
        for use_ in &assertion.uses {
            let calls: Vec<_> = fd.obank().iter_alive()
                .filter(|&op| SourcePoint::op(fd, op) == Some(use_.point))
                .filter(|&op| crate::kuna_stackviews::declared_view(fd, op, use_.slot).is_some())
                .collect();
            if !fd
                .stack_object_bound_uses
                .contains(&(use_.point, use_.slot))
                || calls.len() != 1
                || !selected_view(fd, calls[0], use_, assertion)
            {
                return Err(format!(
                    "Cannot bind stack object {} at {:#x}:{} to shared storage",
                    assertion.selector, use_.point.address, use_.slot
                ));
            }
        }
    }
    Ok(())
}

fn selected_view(fd: &Funcdata, call: OpId, use_: &UseAnchor, assertion: &ObjectAssertion) -> bool {
    let name = assertion.view_name(assertion.datatype.as_ref().unwrap_or(&use_.datatype));
    let Some(mut input) = fd.obank().get(call).and_then(|op| op.get_in(use_.slot)) else {
        return false;
    };
    for _ in 0..16 {
        if crate::kuna_stackobjectalias::matches(fd, input, assertion, use_) {
            return true;
        }
        let Some(v) = fd.vbank().get(input) else {
            return false;
        };
        let Some(id) = v.get_def() else { return false };
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        if op.code() == OpCode::CPUI_PTRSUB {
            if let Some(ty) = op
                .get_in(0)
                .and_then(|input| fd.vbank().get(input))
                .map(|v| v.get_type())
            {
                if fd
                    .get_union_field(ty, id, -1)
                    .and_then(|resolution| {
                        ty.get_ptr_to()?
                            .get_field(resolution.get_field_num())
                            .map(|field| field.name == name)
                    })
                    .unwrap_or(false)
                {
                    return true;
                }
            }
        } else if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return false;
        }
        let Some(next) = op.get_in(0) else {
            return false;
        };
        input = next;
    }
    false
}

fn ptrsub(
    fd: &mut Funcdata,
    call: OpId,
    input: VarnodeId,
    offset: u64,
    ty: Rc<Datatype>,
) -> KunaResult<(OpId, VarnodeId)> {
    let pc = fd.obank().get(call).unwrap().get_addr().clone();
    let op = fd.new_op(2, pc);
    fd.op_set_opcode_code(op, OpCode::CPUI_PTRSUB);
    let out = fd.new_unique_out(ty.get_size(), op)?;
    fd.vn_update_type(out, ty);
    fd.vbank_mut().get_mut(out).unwrap().set_implied();
    let width = fd.vbank().get(input).unwrap().get_size();
    let off = fd.new_constant(width, offset);
    fd.op_set_all_input(op, &[input, off])?;
    fd.op_insert_before(op, call);
    Ok((op, out))
}

fn bind_use(
    fd: &mut Funcdata,
    assertion: &ObjectAssertion,
    use_: &UseAnchor,
    call: OpId,
    factory: &dyn TypeFactory,
    aliases: &mut BTreeMap<(String, u64, i32), VarnodeId>,
) -> Option<()> {
    let input = fd.obank().get(call)?.get_in(use_.slot)?;
    let address = crate::kuna_stackranges::frame_address(fd, input)?;
    let space = assertion.address.get_space()?.clone();
    if address.first != address.last
        || !address.terms.is_empty()
        || space.wrap_offset(address.first as u64) != assertion.address.get_offset()
    {
        return None;
    }
    let info = crate::kuna_stackparamviews::backing_for_address(
        fd,
        &assertion.address,
        assertion.size,
    )
    .or_else(|| {
        fd.get_scope_local()?.query_container_for_link_width(
            &assertion.address,
            assertion.size,
            &Address::new_invalid(),
        )
    })?;
    let backing = info.sym_type.as_ref()?.clone();
    let ty = assertion.datatype.as_ref().unwrap_or(&use_.datatype);
    let name = assertion.view_name(ty);
    let member = (0..backing.num_depend())
        .filter(|_| backing.get_name().starts_with("stack_views_"))
        .find(|&i| backing.get_field(i).is_some_and(|field| field.name == name));
    let width = fd.vbank().get(input)?.get_size();
    let Some(member) = member else {
        let key = (assertion.selector.clone(), ty.get_id(), width);
        let alias = if let Some(alias) = aliases.get(&key) {
            *alias
        } else {
            let alias = crate::kuna_stackobjectalias::create(
                fd, assertion, use_, &info, width, factory,
            )?;
            aliases.insert(key, alias);
            alias
        };
        fd.op_set_input(call, alias, use_.slot).ok()?;
        return Some(());
    };
    let value = backing_view(fd, call, width, &info, member, ty, factory)?;
    fd.op_set_input(call, value, use_.slot).ok()?;
    Some(())
}

pub(crate) fn backing_view(
    fd: &mut Funcdata,
    call: OpId,
    width: i32,
    info: &crate::varmap::LinkEntryInfo,
    member: i32,
    ty: &Rc<Datatype>,
    factory: &dyn TypeFactory,
) -> Option<VarnodeId> {
    let space = info.entry_addr.get_space()?;
    let backing = info.sym_type.as_ref()?;
    let member_ty = backing.get_depend(member)?;
    let pointer = |ty| {
        factory
            .get_type_pointer_strip_array(width, ty, space.get_word_size())
            .ok()
    };
    let root = info.entry_addr.get_offset();
    let base = fd.construct_spacebase_input(space).ok()?;
    let backing_ptr = pointer(Rc::clone(backing))?;
    let (root_op, storage) = ptrsub(fd, call, base, root, Rc::clone(&backing_ptr)).ok()?;
    let offset = fd.obank().get(root_op)?.get_in(1)?;
    let reference = fd.vbank().get(offset)?.get_high()?;
    let high = fd.high_bank_mut().get_mut(reference)?;
    high.set_kuna_name(info.display_name.clone());
    high.set_symbol_offset(0);
    high.set_symbol_type(Rc::clone(backing));
    high.set_kuna_ref_symbol(info.symbol);
    let (facet, mut value) = ptrsub(fd, call, storage, 0, pointer(member_ty)?).ok()?;
    let mut parent = ResolvedUnion::new_field(backing_ptr.clone(), -1, factory).ok()?;
    parent.set_lock(true);
    fd.set_union_field(&backing_ptr, facet, 0, parent);
    let mut resolution = ResolvedUnion::new_field(backing_ptr.clone(), member, factory).ok()?;
    resolution.set_lock(true);
    let time = fd.obank().get(facet)?.get_time();
    fd.union_map
        .insert(ResolveEdge::new_op(&backing_ptr, time, -1), resolution);
    if info.sym_off != 0 {
        value = ptrsub(
            fd,
            call,
            value,
            info.sym_off as u64,
            pointer(Rc::clone(ty))?,
        )
        .ok()?
        .1;
    }
    Some(value)
}

pub(crate) fn prepare_casts(fd: &mut Funcdata, factory: &dyn TypeFactory) {
    let assertions = specs(fd).to_vec();
    let mut aliases = BTreeMap::new();
    for assertion in assertions {
        for use_ in &assertion.uses {
            if fd
                .stack_object_bound_uses
                .contains(&(use_.point, use_.slot))
            {
                continue;
            }
            let calls: Vec<_> = fd.obank().iter_alive()
                .filter(|&op| crate::kuna_stackobjects::SourcePoint::op(fd, op) == Some(use_.point))
                .filter(|&op| crate::kuna_stackviews::declared_view(fd, op, use_.slot).is_some())
                .collect();
            if calls.len() == 1
                && bind_use(fd, &assertion, use_, calls[0], factory, &mut aliases).is_some()
            {
                fd.stack_object_bound_uses.insert((use_.point, use_.slot));
            }
        }
    }
}

pub(crate) fn seed_writes(
    fd: &mut Funcdata,
    start: u64,
    backing: &Rc<Datatype>,
    factory: &dyn TypeFactory,
) {
    let Some(space) = fd
        .get_scope_local()
        .map(|scope| scope.get_space_id().clone())
    else {
        return;
    };
    let assertions = specs(fd).to_vec();
    let mut edges = Vec::new();
    for id in fd.vbank().loc_space_ids(&space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        let Some(op) = v.get_def().and_then(|op| fd.obank().get(op)) else {
            continue;
        };
        if !v.is_stack_store() || op.is_marker() {
            continue;
        }
        let Some(point) = SourcePoint::op(fd, v.get_def().unwrap()) else {
            continue;
        };
        let candidates: Vec<_> = assertions
            .iter()
            .filter(|assertion| {
                let at = space
                    .wrap_offset(v.get_offset().wrapping_sub(assertion.address.get_offset()))
                    as usize;
                let end = at.checked_add(v.get_size() as usize);
                end.is_some_and(|end| {
                    end <= assertion.bytes.len()
                        && assertion.bytes[at..end]
                            .iter()
                            .all(|byte| byte.contains(&ByteSource::Write(point)))
                })
            })
            .collect();
        if candidates.len() != 1 {
            continue;
        }
        let assertion = candidates[0];
        let ty = assertion
            .datatype
            .as_ref()
            .unwrap_or(&assertion.uses[0].datatype);
        let name = assertion.view_name(ty);
        let offset = space.wrap_offset(v.get_offset().wrapping_sub(start)) as i32;
        let member = (0..backing.num_depend()).find(|&i| {
            backing.get_field(i).is_some_and(|field| {
                if field.name != name {
                    return false;
                }
                let Some(piece) = crate::kuna_stackviews::scalar_piece(
                    factory,
                    Rc::clone(&field.field_type),
                    offset,
                    v.get_size(),
                ) else {
                    return false;
                };
                match piece.get_metatype() {
                    type_metatype::TYPE_INT | type_metatype::TYPE_UINT => {
                        !crate::kuna_stackviews::pointer_origin(fd, id)
                            && op
                                .get_in(0)
                                .and_then(|id| fd.vbank().get(id))
                                .is_some_and(|v| {
                                    matches!(
                                        v.get_type().get_metatype(),
                                        type_metatype::TYPE_INT
                                            | type_metatype::TYPE_UINT
                                            | type_metatype::TYPE_UNKNOWN
                                    )
                                })
                    }
                    type_metatype::TYPE_PTR => crate::kuna_stackviews::pointer_origin(fd, id),
                    type_metatype::TYPE_FLOAT => op
                        .get_in(0)
                        .and_then(|id| fd.vbank().get(id))
                        .is_some_and(|v| v.get_type().get_metatype() == type_metatype::TYPE_FLOAT),
                    _ => false,
                }
            })
        });
        if let Some(member) = member {
            edges.push((v.get_def().unwrap(), member, offset));
        }
    }
    for (op, member, offset) in edges {
        let Ok(mut resolution) = ResolvedUnion::new_field(Rc::clone(backing), member, factory)
        else {
            continue;
        };
        resolution.set_lock(true);
        let time = fd.obank().get(op).unwrap().get_time();
        fd.union_map
            .insert(ResolveEdge::new_op(backing, time, -1), resolution);
        fd.stack_write_views.insert(
            op,
            crate::kuna_stackviews::StorageView {
                backing: Rc::clone(backing),
                offset,
            },
        );
    }
}
