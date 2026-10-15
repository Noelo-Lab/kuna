//! Typed views over shared frame storage (`stackviews`, #810).
//!
//! Spatial stack hints otherwise choose one pointee type for every incarnation
//! of an address. A Pair followed by a Handle becomes a Pair assigned a pointer.
//! Keep the physical address and collect distinct declared pointee views
//! before those hints are reconciled. The union's operation-specific resolutions
//! select members; no claim that an old pointer stopped aliasing is needed.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::unionresolve::{ResolveEdge, ResolvedUnion};
use crate::varmap::{MapState, SHARED_STORAGE};

#[derive(Clone)]
pub(crate) struct StorageView {
    pub backing: Rc<Datatype>,
    pub offset: i32,
}

pub struct OptionStackViews;

impl OptionStackViews {
    pub fn apply(&self, value: &str) -> KunaResult<(bool, String)> {
        let enabled = on_or_off(value)?;
        Ok((
            enabled,
            format!(
                "Shared stack-object views turned {}",
                if enabled { "on" } else { "off" }
            ),
        ))
    }
}

pub(crate) fn declared_view(fd: &Funcdata, call: OpId, slot: i32) -> Option<Rc<Datatype>> {
    crate::p6_variables::kuna_stackuses::declared_view(fd, call, slot)
}

fn indexed_width(fd: &Funcdata, base: crate::context::VarnodeId) -> Option<i32> {
    let mut width = 0;
    for id in fd.vbank().get(base)?.descend_iter() {
        let op = fd.obank().get(id)?;
        if op.is_marker() {
            continue;
        }
        let access = match op.code() {
            OpCode::CPUI_LOAD if op.get_in(1) == Some(base) => {
                fd.vbank().get(op.get_out()?)?.get_size()
            }
            OpCode::CPUI_STORE if op.get_in(1) == Some(base) && op.get_in(2) != Some(base) => {
                fd.vbank().get(op.get_in(2)?)?.get_size()
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                for slot in 1..op.num_input() {
                    if op.get_in(slot) == Some(base) {
                        width = width.max(declared_view(fd, id, slot)?.get_size());
                    }
                }
                continue;
            }
            OpCode::CPUI_COPY => {
                let out = fd.vbank().get(op.get_out()?)?;
                if out.is_persist() || out.is_stack_store() {
                    return None;
                }
                continue;
            }
            OpCode::CPUI_CAST
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB => continue,
            _ => return None,
        };
        width = width.max(access);
    }
    (width > 0).then_some(width)
}

fn insert_type(types: &mut Vec<Rc<Datatype>>, ty: Rc<Datatype>) {
    if !types.iter().any(|old| Rc::ptr_eq(old, &ty)) {
        types.push(ty);
    }
}

fn scalar(ty: &Datatype) -> bool {
    match ty.get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_FLOAT => true,
        type_metatype::TYPE_PTR => ty.get_ptr_to().is_some_and(|to| {
            to.get_size() > 0
                || to.get_metatype() == type_metatype::TYPE_VOID
                || to.get_code_prototype().is_some()
        }),
        _ => false,
    }
}

#[derive(Clone)]
struct PlacedView {
    name: Option<String>,
    offset: i32,
    datatype: Rc<Datatype>,
}

fn insert_view(views: &mut Vec<PlacedView>, offset: i32, datatype: Rc<Datatype>) {
    if !views
        .iter()
        .any(|v| v.offset == offset && Rc::ptr_eq(&v.datatype, &datatype))
    {
        views.push(PlacedView {
            name: None,
            offset,
            datatype,
        });
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

fn placed_type(
    fd: &Funcdata,
    start: uintb,
    view: &PlacedView,
    factory: &dyn TypeFactory,
) -> KunaResult<Rc<Datatype>> {
    if view.offset == 0 {
        return Ok(Rc::clone(&view.datatype));
    }
    let signed = start as i64;
    let label = if view.datatype.get_name().is_empty() {
        format!("t{:x}", view.datatype.get_id())
    } else {
        view.datatype
            .get_name()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    };
    let stem = format!(
        "stack_slice_{:x}_{}{:x}_at{:x}_{label}",
        fd.get_address().get_offset(),
        if signed < 0 { "m" } else { "p" },
        signed.unsigned_abs(),
        view.offset,
    );
    if let Some(existing) = factory.find_by_name(&stem)? {
        return Ok(existing);
    }
    let size = view.offset + view.datatype.get_size();
    let alignment = gcd(gcd(view.datatype.get_alignment(), view.offset), size);
    let shell = factory.get_type_struct(&stem)?;
    factory.set_fields_struct_raw(
        &shell,
        vec![TypeField::new(
            0,
            view.offset,
            "value",
            Rc::clone(&view.datatype),
        )],
        Vec::new(),
        size,
        alignment,
        0,
    )
}

fn backing_type(
    fd: &Funcdata,
    space: &Rc<AddrSpace>,
    start: uintb,
    size: i32,
    views: &[PlacedView],
    factory: &dyn TypeFactory,
) -> KunaResult<Rc<Datatype>> {
    let mut members = views.to_vec();
    for view in views {
        for i in 0..view.datatype.num_depend() {
            if let Some(field) = view.datatype.get_field(i) {
                if scalar(&field.field_type) {
                    insert_view(
                        &mut members,
                        view.offset + field.offset,
                        Rc::clone(&field.field_type),
                    );
                }
            }
        }
    }
    for id in fd.vbank().loc_space_ids(space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        if !contains(start, size, v.get_offset(), v.get_size()) || !v.is_written() {
            continue;
        }
        let offset = v.get_offset().wrapping_sub(start) as i32;
        if let Some(op) = v.get_def().and_then(|id| fd.obank().get(id)) {
            if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
                if let Some(ty) = op
                    .get_in(0)
                    .and_then(|id| fd.vbank().get(id))
                    .map(|v| Rc::clone(v.get_type()))
                {
                    if scalar(&ty) && ty.get_size() == v.get_size() {
                        insert_view(&mut members, offset, ty);
                    }
                }
            }
        }
        if matches!(v.get_size(), 1 | 2 | 4 | 8) {
            insert_view(
                &mut members,
                offset,
                factory.get_base(v.get_size(), type_metatype::TYPE_UINT)?,
            );
        }
    }
    for width in [1, 2, 4, 8] {
        if width <= size {
            insert_view(
                &mut members,
                0,
                factory.get_base(width, type_metatype::TYPE_UINT)?,
            );
        }
    }
    let bytes = factory.get_type_array(size, factory.get_base(1, type_metatype::TYPE_UINT)?)?;
    insert_view(&mut members, 0, bytes);
    members.sort_by(|a, b| {
        a.offset
            .cmp(&b.offset)
            .then_with(|| {
                (a.datatype.get_metatype() as u32).cmp(&(b.datatype.get_metatype() as u32))
            })
            .then_with(|| a.datatype.get_size().cmp(&b.datatype.get_size()))
            .then_with(|| a.datatype.get_name().cmp(b.datatype.get_name()))
            .then_with(|| a.datatype.get_id().cmp(&b.datatype.get_id()))
            .then_with(|| a.name.cmp(&b.name))
    });
    let placed: Vec<_> = members
        .iter()
        .map(|view| placed_type(fd, start, view, factory))
        .collect::<KunaResult<_>>()?;
    let mut used_names = BTreeSet::new();
    let field_names: Vec<_> = members
        .iter()
        .enumerate()
        .map(|(index, view)| {
            let mut name = view.name.clone().unwrap_or_else(|| {
                let mut name = if !view.datatype.get_name().is_empty() {
                    format!("view_{}", view.datatype.get_name().replace(" ", "_"))
                } else if view.datatype.get_array_base().is_some() {
                    "bytes".to_string()
                } else if view.datatype.get_metatype() == type_metatype::TYPE_UINT {
                    format!("bits_{}", view.datatype.get_size())
                } else {
                    format!("value_{index}")
                };
                if view.offset != 0 {
                    name.push_str(&format!("_at{:x}", view.offset));
                }
                name
            });
            if !used_names.insert(name.clone()) {
                name.push_str(&format!("_{index}"));
                used_names.insert(name.clone());
            }
            name
        })
        .collect();
    let signed = start as i64;
    let stem = format!(
        "stack_views_{:x}_{}{:x}",
        fd.get_address().get_offset(),
        if signed < 0 { "m" } else { "p" },
        signed.unsigned_abs()
    );
    let mut variant = 0;
    let name = loop {
        let name = if variant == 0 {
            stem.clone()
        } else {
            format!("{stem}_{variant}")
        };
        match factory.find_by_name(&name)? {
            Some(existing)
                if existing.get_size() == size
                    && existing.num_depend() as usize == placed.len()
                    && placed.iter().enumerate().all(|(i, t)| {
                        existing.get_field(i as i32).is_some_and(|field| {
                            Rc::ptr_eq(&field.field_type, t) && field.name == field_names[i]
                        })
                    }) =>
            {
                return Ok(existing)
            }
            Some(_) => variant += 1,
            None => break name,
        }
    };
    let alignment = placed.iter().map(|t| t.get_alignment()).max().unwrap_or(1);
    let alignment = gcd(alignment, size);
    let shell = factory.get_type_union(&name)?;
    let fields = placed
        .into_iter()
        .enumerate()
        .map(|(index, ty)| TypeField::new(index as i32, 0, field_names[index].clone(), ty))
        .collect();
    factory.set_fields_union_raw(&shell, fields, size, alignment)
}

/// Add exact-size backing objects only where several declared pointee views
/// conflict or intersect. Indexed references require a bounded byte extent;
/// explicit user-locked layouts and unresolved extents remain unchanged.
pub fn gather(fd: &mut Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) {
    if !fd.get_arch().stack_views {
        return;
    }
    let Some(factory) = fd.get_arch().types_rc() else {
        return;
    };
    let checker = state.checker_mut();
    let refs: Vec<_> = checker
        .get_alias()
        .iter()
        .copied()
        .zip(checker.get_add_base().iter().copied())
        .collect();
    let mut groups: BTreeMap<uintb, Vec<Rc<Datatype>>> = BTreeMap::new();
    let mut extents = Vec::new();
    let mut unresolved_index = false;
    for (offset, reference) in &refs {
        if reference.index.is_some() {
            let extent = crate::kuna_stackranges::frame_address(fd, reference.base)
                .and_then(|address| address.extent(indexed_width(fd, reference.base)?));
            if let Some((start, size)) = extent {
                extents.push((start, size, Vec::new()));
            } else {
                unresolved_index = true;
            }
            continue;
        }
        let Some(base) = fd.vbank().get(reference.base) else {
            continue;
        };
        for call in base.descend_iter() {
            let Some(op) = fd.obank().get(call) else {
                continue;
            };
            for slot in 0..op.num_input() {
                if op.get_in(slot) == Some(reference.base) {
                    if let Some(view) = declared_view(fd, call, slot) {
                        insert_type(groups.entry(*offset).or_default(), view);
                    }
                }
            }
        }
    }
    for id in fd.vbank().loc_space_ids(space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        if !v.is_stack_store() || !v.is_written() {
            continue;
        }
        let ty = v.get_type();
        let parent = ty.get_partial_base().unwrap_or_else(|| Rc::clone(ty));
        if parent.get_name().starts_with("stack_views_") {
            for i in 0..parent.num_depend() {
                if let Some(view) = parent.get_depend(i) {
                    if view.get_array_base().is_none() {
                        let root = v
                            .get_offset()
                            .wrapping_sub(ty.get_partial_offset().unwrap_or(0) as u64);
                        let placed = if view.get_name().starts_with("stack_slice_") {
                            view.get_field(0)
                                .map(|f| (f.offset, Rc::clone(&f.field_type)))
                        } else {
                            Some((0, view))
                        };
                        if let Some((offset, datatype)) = placed {
                            insert_type(
                                groups.entry(root.wrapping_add(offset as u64)).or_default(),
                                datatype,
                            );
                        }
                    }
                }
            }
            continue;
        }
        if scalar(ty) {
            insert_type(groups.entry(v.get_offset()).or_default(), Rc::clone(ty));
        } else if ty.get_metatype() == type_metatype::TYPE_UNKNOWN
            && matches!(ty.get_size(), 1 | 2 | 4 | 8)
        {
            if let Ok(raw) = factory.get_base(ty.get_size(), type_metatype::TYPE_UINT) {
                insert_type(groups.entry(v.get_offset()).or_default(), raw);
            }
        }
    }
    let hints = state.hints_mut().clone();
    extents.extend(groups.into_iter().map(|(at, types)| {
        let width = types.iter().map(|t| t.get_size()).max().unwrap();
        (at, width, types)
    }));
    extents.sort_by_key(|extent| extent.0);
    let mut components: Vec<(uintb, i32, Vec<PlacedView>)> = Vec::new();
    for (at, width, types) in extents {
        if let Some((start, size, views)) = components
            .last_mut()
            .filter(|(start, size, _)| at.wrapping_sub(*start) < *size as u64)
        {
            let offset = at.wrapping_sub(*start) as i32;
            *size = (*size).max(offset + width);
            for ty in types {
                insert_view(views, offset, ty);
            }
        } else {
            components.push((
                at,
                width,
                types
                    .into_iter()
                    .map(|t| PlacedView {
                        name: None,
                        offset: 0,
                        datatype: t,
                    })
                    .collect(),
            ));
        }
    }
    let mut recovered = Vec::new();
    for (start, size, mut views) in components {
        for assertion in crate::kuna_stackobjectasserts::specs(fd) {
            let at = assertion.address.get_offset();
            if !contains(start, size, at, assertion.size) {
                continue;
            }
            let mut types = Vec::new();
            if let Some(ty) = &assertion.datatype {
                insert_type(&mut types, Rc::clone(ty));
            } else {
                for use_ in &assertion.uses {
                    insert_type(&mut types, Rc::clone(&use_.datatype));
                }
            }
            for datatype in types {
                views.push(PlacedView {
                    name: Some(assertion.view_name(&datatype)),
                    offset: at.wrapping_sub(start) as i32,
                    datatype,
                });
            }
        }
        let parameter = !state.covers(start, size)
            && (views.iter().any(|view| matches!(view.datatype.get_metatype(),
                type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
                | type_metatype::TYPE_PTR | type_metatype::TYPE_FLOAT))
                || crate::kuna_stackparamviews::needs_backing(fd, start, size))
            && crate::kuna_stackparamviews::can_map_backing(fd, start, size);
        if parameter {
            for (offset, datatype) in crate::kuna_stackparamviews::parameter_types(fd, start, size) {
                insert_view(&mut views, offset, datatype);
            }
        }
        if views.len() < 2 || views.len() > 32 || size > 65536 || (!state.covers(start, size) && !parameter) {
            continue;
        }
        let address = Address::new(Rc::clone(space), start);
        let overlaps = |other: uintb, width: i32| {
            let at = Address::new(Rc::clone(space), other);
            address.overlap(0, &at, width) >= 0 || at.overlap(0, &address, size) >= 0
        };
        if hints
            .iter()
            .any(|h| h.is_type_lock() && overlaps(h.start, h.size))
            || unresolved_index
        {
            continue;
        }
        if let Ok(union) = backing_type(fd, space, start, size, &views, factory.as_ref()) {
            recovered.push((start, union, parameter));
        }
    }
    for (start, union, parameter) in recovered {
        let address = Address::new(Rc::clone(space), start);
        let size = union.get_size();
        state.hints_mut().retain(|h| {
            let other = Address::new(Rc::clone(space), h.start);
            address.overlap(0, &other, h.size) < 0 && other.overlap(0, &address, size) < 0
        });
        seed_writes(fd, space, start, &union, factory.as_ref());
        if parameter {
            crate::kuna_stackparamviews::map_backing(fd, start, union);
        } else {
            state.add_fixed_type_pub(start, union, SHARED_STORAGE, factory.as_ref());
        }
    }
}

/// Keep raw whole-word writes out of aggregate members. Pointer provenance
/// chooses a pointer view; an integer constant remains an integer even when its
/// bit pattern happens to look like a mapped address.
fn seed_writes(
    fd: &mut Funcdata,
    space: &Rc<AddrSpace>,
    start: uintb,
    union: &Rc<Datatype>,
    factory: &dyn TypeFactory,
) {
    let mut edges = Vec::new();
    for id in fd.vbank().loc_space_ids(space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        let offset = v.get_offset().wrapping_sub(start);
        if !contains(start, union.get_size(), v.get_offset(), v.get_size()) || !v.is_written() {
            continue;
        }
        let Some(op_id) = v.get_def() else { continue };
        let Some(op) = fd.obank().get(op_id) else {
            continue;
        };
        if op.is_marker() {
            continue;
        }
        let input = op.get_in(0).and_then(|id| fd.vbank().get(id));
        let wanted = if pointer_origin(fd, id) {
            type_metatype::TYPE_PTR
        } else if input.is_some_and(|v| v.get_type().get_metatype() == type_metatype::TYPE_FLOAT) {
            type_metatype::TYPE_FLOAT
        } else {
            type_metatype::TYPE_UINT
        };
        let pointer_type = input
            .filter(|_| matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST))
            .filter(|v| {
                scalar(v.get_type()) && v.get_type().get_metatype() == type_metatype::TYPE_PTR
            })
            .map(|v| v.get_type())
            .or_else(|| {
                (v.get_type().get_metatype() == type_metatype::TYPE_PTR).then(|| v.get_type())
            });
        let exact = (0..union.num_depend())
            .find(|&i| {
                union
                    .get_depend(i)
                    .and_then(|t| {
                        factory
                            .get_exact_piece(t, offset as i32, v.get_size())
                            .ok()
                            .flatten()
                    })
                    .is_some_and(|t| {
                        t.get_metatype() == wanted
                            && (wanted != type_metatype::TYPE_PTR
                                || pointer_type.is_some_and(|ty| Rc::ptr_eq(&t, ty)))
                    })
            })
            .or_else(|| {
                (wanted == type_metatype::TYPE_PTR)
                    .then(|| {
                        (0..union.num_depend()).find(|&i| {
                            union
                                .get_depend(i)
                                .and_then(|t| {
                                    factory
                                        .get_exact_piece(t, offset as i32, v.get_size())
                                        .ok()
                                        .flatten()
                                })
                                .is_some_and(|t| t.get_metatype() == wanted)
                        })
                    })
                    .flatten()
            });
        let bytes = (0..union.num_depend()).find(|&i| {
            union
                .get_depend(i)
                .is_some_and(|t| t.get_array_base().is_some())
        });
        if let Some(index) = exact.or(if v.get_size() == 1 { bytes } else { None }) {
            edges.push((op_id, index, offset as i32))
        }
    }
    for (op, index, offset) in edges {
        if let Ok(mut resolution) = ResolvedUnion::new_field(Rc::clone(union), index, factory) {
            resolution.set_lock(true);
            let time = fd.obank().get(op).unwrap().get_time();
            fd.union_map
                .insert(ResolveEdge::new_op(union, time, -1), resolution);
            fd.stack_write_views.insert(
                op,
                StorageView {
                    backing: Rc::clone(union),
                    offset,
                },
            );
        }
    }
}

/// Finalize write views after type propagation and symbol naming. Earlier
/// reconstruction rounds may have seen incomplete source types.
pub fn prepare_casts(fd: &mut Funcdata) {
    if !fd.get_arch().stack_views {
        return;
    }
    let Some(factory) = fd.get_arch().types_rc() else {
        return;
    };
    let Some(space) = fd.get_scope_local().map(|s| Rc::clone(s.get_space_id())) else {
        return;
    };
    let mut objects = BTreeMap::new();
    for id in fd.vbank().loc_space_ids(&space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        let Some(info) = fd.get_scope_local().and_then(|scope| {
            scope.query_container_for_link_width(v.get_addr(), v.get_size(), &Address::new_invalid())
        }) else {
            continue;
        };
        let Some(backing) = info.sym_type else {
            continue;
        };
        if backing.get_name().starts_with("stack_views_") {
            objects.insert(v.get_offset().wrapping_sub(info.sym_off as u64), backing);
        }
    }
    crate::p6_variables::kuna_stackvaluecopies::prepare(fd, factory.as_ref());
    for (start, backing) in objects {
        seed_writes(fd, &space, start, &backing, factory.as_ref());
        crate::kuna_stackobjectasserts::seed_writes(fd, start, &backing, factory.as_ref());
    }
    crate::p6_variables::kuna_stackvalueviews::prepare(fd, factory.as_ref());
    crate::kuna_stackobjects::finish(fd);
    crate::kuna_stackparamviews::prepare_casts(fd);
    crate::kuna_stackobjectasserts::prepare_casts(fd, factory.as_ref());
    crate::p6_variables::kuna_stackuses::prepare_casts(fd, factory.as_ref());
}

/// Casts must use the selected storage view, even if the SSA value's preferred
/// type differs. The C lvalue is a union member, not the raw high type.
pub fn access_type(
    fd: &Funcdata,
    vn: crate::context::VarnodeId,
    op: OpId,
    slot: i32,
) -> Option<Rc<Datatype>> {
    if !fd.get_arch().stack_views {
        return None;
    }
    let v = fd.vbank().get(vn)?;
    if v.is_persist() {
        if let Some(global) = fd.get_arch().query_container_global(
            v.get_addr(),
            v.get_size(),
            &Address::new_invalid(),
        ) {
            if let Some(ty) = global.symbol_type {
                if scalar(&ty)
                    && ty.get_size() == v.get_size()
                    && global.entry_addr == *v.get_addr()
                {
                    return Some(ty);
                }
            }
        }
    }
    let high = v.get_high().and_then(|h| fd.high_bank().get(h));
    let from_high = high.and_then(|h| {
        h.kuna_symbol_type()
            .map(|ty| (Rc::clone(ty), h.get_symbol_offset().max(0)))
    });
    let from_scope = || {
        let info = fd
            .get_scope_local()?
            .query_container_for_link(v.get_addr(), &Address::new_invalid())?;
        Some((info.sym_type?, info.sym_off))
    };
    let from_edge = (slot == -1)
        .then(|| fd.stack_write_views.get(&op))
        .flatten()
        .map(|view| (Rc::clone(&view.backing), view.offset));
    let from_type = || {
        let ty = v.get_type();
        let parent = ty.get_partial_base().unwrap_or_else(|| Rc::clone(ty));
        parent
            .get_name()
            .starts_with("stack_views_")
            .then(|| (parent, ty.get_partial_offset().unwrap_or(0)))
    };
    let (backing, offset) = from_edge
        .or_else(|| from_high.filter(|(t, _)| t.get_name().starts_with("stack_views_")))
        .or_else(|| from_scope().filter(|(t, _)| t.get_name().starts_with("stack_views_")))
        .or_else(from_type)?;
    if !backing.get_name().starts_with("stack_views_") {
        return None;
    }
    let resolution = fd.get_union_resolution(&backing, op, slot);
    let index = resolution?.get_field_num();
    let field = backing.get_field(index)?;
    let offset = offset - field.offset;
    scalar_piece(fd.get_arch().types()?, Rc::clone(&field.field_type), offset, v.get_size())
}

pub(crate) fn scalar_piece(
    factory: &dyn TypeFactory,
    ty: Rc<Datatype>,
    offset: i32,
    size: i32,
) -> Option<Rc<Datatype>> {
    let mut piece = factory.get_exact_piece(ty, offset, size).ok()??;
    for _ in 0..32 {
        if scalar(&piece) {
            return Some(piece);
        }
        let (subtype, residual) = piece.get_sub_type(0).ok()?;
        let subtype = subtype?;
        if residual != 0 || subtype.get_size() != size {
            return None;
        }
        piece = subtype;
    }
    None
}

fn contains(start: u64, size: i32, at: u64, width: i32) -> bool {
    at.wrapping_sub(start)
        .checked_add(width as u64)
        .is_some_and(|end| end <= size as u64)
}

/// Reinterpreting packed integer bytes as a pointer is not address provenance.
/// Follow value copies to a literal or an actual address expression.
pub(crate) fn pointer_origin(fd: &Funcdata, mut id: crate::context::VarnodeId) -> bool {
    for _ in 0..32 {
        let Some(v) = fd.vbank().get(id) else {
            return false;
        };
        let literal = |offset| {
            fd.get_arch()
                .manage()
                .get_default_data_space()
                .is_some_and(|space| {
                    fd.get_arch()
                        .query_container_global(
                            &Address::new(Rc::clone(space), offset),
                            1,
                            &Address::new_invalid(),
                        )
                        .is_some()
                })
        };
        if v.is_constant() {
            return v.get_type().get_metatype() == type_metatype::TYPE_PTR
                && literal(v.get_offset());
        }
        let Some(op) = v.get_def().and_then(|op| fd.obank().get(op)) else {
            return v.get_type().get_metatype() == type_metatype::TYPE_PTR;
        };
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                let Some(input) = op.get_in(0) else {
                    return false;
                };
                id = input;
            }
            OpCode::CPUI_PTRSUB => {
                if op
                    .get_in(0)
                    .and_then(|v| fd.vbank().get(v))
                    .is_some_and(|v| v.is_constant() && v.is_spacebase())
                {
                    return op
                        .get_in(1)
                        .and_then(|v| fd.vbank().get(v))
                        .is_some_and(|v| v.is_constant() && literal(v.get_offset()));
                }
                return true;
            }
            OpCode::CPUI_PTRADD => {
                let Some(input) = op.get_in(0) else {
                    return false;
                };
                id = input;
            }
            _ => return false,
        }
    }
    false
}
