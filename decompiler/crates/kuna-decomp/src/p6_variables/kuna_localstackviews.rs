//! Bounded native local writes and typed publications (#810).
//! Fixed aggregate recognition builds on mgostIH's PR #1162.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_stackobjects::SourcePoint;

const MAX_RECORDS: usize = 4096;
const MAX_FOOTPRINTS: usize = 32;
const MAX_VIEWS: usize = 32;

enum NativeWrite {
    Store(SourcePoint, i32),
    Copy(SourcePoint, Address, i32),
}

#[derive(Clone)]
struct Publication {
    start: u64,
    pointee: Rc<Datatype>,
    receiver: Address,
    width: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    Crossing,
    Unavailable,
    LockedLayout,
}

#[derive(Default)]
pub(crate) struct LocalStackViews {
    native: BTreeMap<OpId, NativeWrite>,
    writes: BTreeMap<OpId, (u64, i32)>,
    candidates: BTreeMap<(u64, i32), Vec<Rc<Datatype>>>,
    footprints: BTreeMap<(u64, i32), Vec<Rc<Datatype>>>,
    pending: BTreeMap<OpId, Publication>,
    publications: BTreeMap<OpId, Publication>,
    admission: BTreeMap<(u64, i32), Result<(), Refusal>>,
    early_window: Option<i32>,
    collected: bool,
    unrepresented_publication: bool,
    invalid: bool,
}

fn invalidate(fd: &mut Funcdata) {
    fd.local_stack_views = LocalStackViews {
        invalid: true,
        ..Default::default()
    };
}

fn enabled(fd: &Funcdata) -> bool {
    fd.get_arch().local_stack_views && !fd.get_arch().stack_views
}

fn valid_extent(space: &AddrSpace, start: u64, size: i32) -> bool {
    size > 0
        && start
            .checked_add(size as u64 - 1)
            .is_some_and(|end| space.wrap_offset(end) == end)
}

fn local(fd: &Funcdata, address: &Address, size: i32) -> bool {
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    if !fd.get_func_proto().has_store()
        || !(1..=256).contains(&size)
        || stack.get_word_size() != 1
        || !(1..=8).contains(&stack.get_addr_size())
        || address
            .get_space()
            .is_none_or(|s| s.get_index() != stack.get_index())
        || address
            .get_offset()
            .checked_add(size as u64 - 1)
            .is_none_or(|end| stack.wrap_offset(end) != end)
        || !fd
            .get_func_proto()
            .get_local_range()
            .in_range(address, size)
    {
        return false;
    }
    (0..fd.get_func_proto().num_params()).all(|index| {
        let Some(param) = fd.get_func_proto().get_param(index) else {
            return true;
        };
        let at = param.get_address();
        at.get_space()
            .is_none_or(|s| s.get_index() != stack.get_index())
            || param.get_type().is_some_and(|ty| {
                ty.get_size() > 0
                    && !overlaps(address.get_offset(), size, at.get_offset(), ty.get_size())
            })
    })
}

fn overlaps(start: u64, size: i32, other: u64, width: i32) -> bool {
    size > 0
        && width > 0
        && (start.wrapping_sub(other) < width as u64 || other.wrapping_sub(start) < size as u64)
}

fn fixed(fd: &Funcdata, input: VarnodeId, size: i32) -> Option<u64> {
    let frame = crate::kuna_stackranges::frame_address(fd, input)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    if frame.pointer_size as u32 != stack.get_addr_size()
        || frame.first != frame.last
        || !frame.terms.is_empty()
    {
        return None;
    }
    let at = stack.wrap_offset(frame.first as u64);
    local(fd, &Address::new(Rc::clone(stack), at), size).then_some(at)
}

fn nominal(ty: &Datatype) -> bool {
    !ty.is_incomplete()
        && !ty.get_name().is_empty()
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
        )
}

fn c_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Record original lifted memory operations; generational OpIds do not match clones.
pub(crate) fn record_native(fd: &mut Funcdata, id: OpId) {
    if !enabled(fd) || fd.local_stack_views.invalid {
        return;
    }
    if fd.local_stack_views.native.contains_key(&id) {
        return;
    }
    let Some(op) = fd.obank().get(id) else { return };
    let Some(point) = SourcePoint::op(fd, id) else {
        return;
    };
    let event = if op.code() == OpCode::CPUI_STORE {
        let Some(width) = op
            .get_in(2)
            .and_then(|id| fd.vbank().get(id))
            .map(|v| v.get_size())
            .filter(|&width| width > 0)
        else {
            return;
        };
        NativeWrite::Store(point, width)
    } else if op.code() == OpCode::CPUI_COPY {
        let Some(out) = op
            .get_out()
            .and_then(|id| fd.vbank().get(id))
            .filter(|v| v.is_persist())
        else {
            return;
        };
        NativeWrite::Copy(point, out.get_addr().clone(), out.get_size())
    } else {
        return;
    };
    if fd.local_stack_views.native.len() == MAX_RECORDS {
        invalidate(fd);
        return;
    }
    fd.local_stack_views.native.insert(id, event);
}

/// Capture actual STORE extent and expose positive ranges before the next heritage.
pub(crate) fn record_store(fd: &mut Funcdata, op: OpId, address: &Address, size: i32) {
    if !enabled(fd)
        || fd.local_stack_views.invalid
        || fd
            .obank()
            .get(op)
            .is_none_or(|o| o.is_dead() || o.code() != OpCode::CPUI_STORE)
        || !matches!(fd.local_stack_views.native.get(&op), Some(NativeWrite::Store(point, width))
            if *width == size && Some(*point) == SourcePoint::op(fd, op))
        || !local(fd, address, size)
    {
        return;
    }
    if fd.local_stack_views.writes.contains_key(&op) {
        return;
    }
    if fd.local_stack_views.writes.len() == MAX_RECORDS {
        invalidate(fd);
        return;
    }
    fd.local_stack_views
        .writes
        .insert(op, (address.get_offset(), size));
    let pass = fd.get_heritage_pass();
    let window = *fd.local_stack_views.early_window.get_or_insert(pass);
    if window != pass {
        return;
    }
    if !fd.local_stack_views.collected {
        collect(fd);
        fd.local_stack_views.collected = true;
    }
    promote(fd, true);
}

fn native_receiver(fd: &Funcdata, id: OpId) -> Option<(Address, i32)> {
    let op = fd
        .obank()
        .get(id)
        .filter(|op| !op.is_dead() && !op.is_return_copy())?;
    let point = SourcePoint::op(fd, id)?;
    if op.code() == OpCode::CPUI_STORE {
        let NativeWrite::Store(original, width) = fd.local_stack_views.native.get(&id)? else {
            return None;
        };
        if *original != point || fd.vbank().get(op.get_in(2)?)?.get_size() != *width {
            return None;
        }
        let space = fd.vbank().get(op.get_in(0)?)?;
        let space = space.is_constant().then(|| {
            fd.get_arch()
                .manage()
                .get_space(i32::try_from(space.get_offset()).ok()?)
        })??;
        if space.get_word_size() != 1
            || fd.get_arch().manage().get_default_data_space()?.get_index() != space.get_index()
        {
            return None;
        }
        let destination = fd.vbank().get(op.get_in(1)?)?;
        if !destination.is_constant()
            || destination.get_size() as u32 != space.get_addr_size()
            || !valid_extent(space, destination.get_offset(), *width)
        {
            return None;
        }
        return Some((
            Address::new(space.clone(), destination.get_offset()),
            *width,
        ));
    }
    if op.code() != OpCode::CPUI_COPY {
        return None;
    }
    let out = fd.vbank().get(op.get_out()?).filter(|v| v.is_persist())?;
    match fd.local_stack_views.native.get(&id)? {
        NativeWrite::Store(original, width) if *original == point && *width == out.get_size() => (),
        NativeWrite::Copy(original, address, width)
            if *original == point && address == out.get_addr() && *width == out.get_size() =>
        {
            ()
        }
        _ => return None,
    }
    Some((out.get_addr().clone(), out.get_size()))
}

fn published_input(fd: &Funcdata, id: OpId) -> Option<(i32, VarnodeId)> {
    let op = fd.obank().get(id)?;
    match op.code() {
        OpCode::CPUI_COPY => {
            fd.vbank().get(op.get_out()?).filter(|v| v.is_persist())?;
            Some((0, op.get_in(0)?))
        }
        OpCode::CPUI_STORE => {
            let stack = fd.get_arch().manage().get_stack_space()?;
            let space = fd.vbank().get(op.get_in(0)?)?;
            if !space.is_constant()
                || space.get_offset() == stack.get_index() as u64
                || crate::kuna_stackranges::frame_address(fd, op.get_in(1)?).is_some()
            {
                return None;
            }
            Some((2, op.get_in(2)?))
        }
        _ => None,
    }
}

fn collect(fd: &mut Funcdata) {
    let gathered = (|| {
        let mut groups: BTreeMap<(u64, i32), Vec<Rc<Datatype>>> = BTreeMap::new();
        let mut publications = BTreeMap::new();
        let mut frame_publications = Vec::new();
        for (count, id) in fd.obank().iter_alive().enumerate() {
            if count == MAX_RECORDS {
                return None;
            }
            let op = fd.obank().get(id).unwrap();
            let call = matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND);
            let receiver = native_receiver(fd, id);
            if let Some((_, input)) = published_input(fd, id) {
                if crate::kuna_stackranges::frame_address(fd, input).is_some() {
                    frame_publications.push(id);
                }
            }
            if !call && receiver.is_none() {
                continue;
            }
            for slot in 0..op.num_input() {
                let Some(ty) = crate::p6_variables::kuna_stackuses::declared_view(fd, id, slot)
                else {
                    continue;
                };
                let aggregate = matches!(
                    ty.get_metatype(),
                    type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION
                );
                if receiver.is_none() && !aggregate {
                    continue;
                }
                let Some(at) = op.get_in(slot).and_then(|id| fixed(fd, id, ty.get_size())) else {
                    continue;
                };
                if aggregate && !c_identifier(ty.get_name()) {
                    return None;
                }
                let views = groups.entry((at, ty.get_size())).or_default();
                if !views.iter().any(|old| Rc::ptr_eq(old, &ty)) {
                    views.push(Rc::clone(&ty));
                }
                if views.len() > MAX_VIEWS || groups.len() > MAX_FOOTPRINTS {
                    return None;
                }
                if let Some((address, width)) = &receiver {
                    publications.insert(
                        id,
                        Publication {
                            start: at,
                            pointee: ty,
                            receiver: address.clone(),
                            width: *width,
                        },
                    );
                }
            }
        }
        let unrepresented = frame_publications
            .iter()
            .any(|id| !publications.contains_key(id));
        Some((groups, publications, unrepresented))
    })();
    let Some((groups, publications, unrepresented)) = gathered else {
        invalidate(fd);
        return;
    };
    fd.local_stack_views.candidates = groups;
    fd.local_stack_views.pending = publications;
    fd.local_stack_views.unrepresented_publication = unrepresented;
}

fn promote(fd: &mut Funcdata, early: bool) {
    if fd.local_stack_views.invalid {
        return;
    }
    let candidates = fd.local_stack_views.candidates.clone();
    for ((at, size), views) in candidates {
        let escaped = fd
            .local_stack_views
            .pending
            .values()
            .any(|p| p.start == at && p.pointee.get_size() == size);
        let incompatible = views.iter().filter(|ty| nominal(ty)).count() >= 2;
        let written = fd
            .local_stack_views
            .writes
            .values()
            .any(|&(start, width)| overlaps(at, size, start, width));
        if !written || (!escaped && !incompatible) {
            continue;
        }
        let key = (at, size);
        if !fd.local_stack_views.admission.contains_key(&key) {
            if fd.local_stack_views.admission.len() == MAX_FOOTPRINTS {
                invalidate(fd);
                return;
            }
            let decision = if early {
                accesses_fit_early(fd, at, size, views.iter().all(|ty| scalar(ty)))
            } else {
                Err(Refusal::Unavailable)
            };
            fd.local_stack_views.admission.insert(key, decision);
        }
        if fd.local_stack_views.admission[&key].is_err() {
            continue;
        }
        fd.local_stack_views.footprints.insert((at, size), views);
    }
    if fd.local_stack_views.footprints.len() > MAX_FOOTPRINTS {
        invalidate(fd);
        return;
    }
    for (&id, publication) in &fd.local_stack_views.pending {
        let covered = fd
            .local_stack_views
            .footprints
            .iter()
            .any(|(&(at, size), views)| {
                views.iter().filter(|ty| nominal(ty)).count() >= 2
                    && contains(at, size, publication.start, publication.pointee.get_size())
            });
        if covered
            || fd
                .local_stack_views
                .footprints
                .contains_key(&(publication.start, publication.pointee.get_size()))
        {
            fd.local_stack_views
                .publications
                .insert(id, publication.clone());
        }
    }
    if fd.local_stack_views.publications.len() > MAX_RECORDS {
        invalidate(fd);
    }
}

pub(crate) fn active(fd: &Funcdata) -> bool {
    enabled(fd) && !fd.local_stack_views.invalid && !fd.local_stack_views.footprints.is_empty()
}

pub(crate) fn preserves_range(fd: &Funcdata, address: &Address, size: i32) -> bool {
    active(fd)
        && address.get_space().is_some_and(|s| {
            fd.get_arch()
                .manage()
                .get_stack_space()
                .is_some_and(|stack| s.get_index() == stack.get_index())
        })
        && fd
            .local_stack_views
            .footprints
            .keys()
            .any(|&(at, width)| overlaps(at, width, address.get_offset(), size))
}

/// Preserve the original physical write across memory-SSA marker inputs.
pub(crate) fn native_store(fd: &Funcdata, value: VarnodeId) -> bool {
    if !active(fd) {
        return false;
    }
    let Some(v) = fd.vbank().get(value).filter(|v| v.is_stack_store()) else {
        return false;
    };
    let Some(id) = v.get_def() else { return false };
    if fd.obank().get(id).is_none_or(|op| {
        op.is_dead()
            || op.is_return_copy()
            || op.code() != OpCode::CPUI_COPY
            || op.get_out() != Some(value)
    }) {
        return false;
    }
    let Some(NativeWrite::Store(point, width)) = fd.local_stack_views.native.get(&id) else {
        return false;
    };
    if Some(*point) != SourcePoint::op(fd, id)
        || *width != v.get_size()
        || fd.local_stack_views.writes.get(&id) != Some(&(v.get_offset(), *width))
    {
        return false;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    v.get_space().get_index() == stack.get_index()
        && fd
            .local_stack_views
            .footprints
            .keys()
            .any(|&(at, size)| contains(at, size, v.get_offset(), *width))
}

/// Keep only the captured native publication with its exact declared receiver and frame input.
pub(crate) fn publication(fd: &Funcdata, value: VarnodeId) -> bool {
    if !active(fd) {
        return false;
    }
    let Some(v) = fd.vbank().get(value) else {
        return false;
    };
    let Some(id) = v.get_def() else { return false };
    let Some(p) = fd.local_stack_views.publications.get(&id) else {
        return false;
    };
    native_receiver(fd, id).is_some_and(|(at, width)| at == p.receiver && width == p.width)
        && fd
            .obank()
            .get(id)
            .and_then(|op| op.get_in(0))
            .and_then(|input| fixed(fd, input, p.pointee.get_size()))
            == Some(p.start)
        && crate::p6_variables::kuna_stackuses::declared_view(fd, id, 0)
            .is_some_and(|ty| Rc::ptr_eq(&ty, &p.pointee))
}

pub(crate) fn aggregate(fd: &Funcdata, start: u64, size: i32) -> bool {
    active(fd)
        && fd
            .local_stack_views
            .footprints
            .get(&(start, size))
            .is_some_and(|views| views.iter().filter(|ty| nominal(ty)).count() >= 2)
}

fn scalar(ty: &Datatype) -> bool {
    matches!(ty.get_size(), 1 | 2 | 4 | 8)
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_BOOL
                | type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_FLOAT
                | type_metatype::TYPE_PTR
        )
}

fn contains(start: u64, size: i32, at: u64, width: i32) -> bool {
    width > 0
        && at.wrapping_sub(start) < size as u64
        && at.wrapping_sub(start) + width as u64 <= size as u64
}

fn locked_layout(fd: &Funcdata, start: u64, size: i32) -> bool {
    let Some(scope) = fd.get_scope_local() else {
        return true;
    };
    let address = Address::new(scope.get_space_id().clone(), start);
    scope
        .database()
        .find_overlap_matching(scope.scope_id(), &address, size, |symbol| {
            symbol.is_type_locked()
        })
        .is_some()
}

/// A possible saved-receiver use is sufficient to refuse crossing, not to infer pointer equality.
fn receiver_offset(
    fd: &Funcdata,
    value: VarnodeId,
    publication: &Publication,
    reader: OpId,
    budget: &mut usize,
) -> Option<(i64, OpId)> {
    *budget = budget.checked_sub(1)?;
    let v = fd.vbank().get(value)?;
    if v.get_size() != publication.width {
        return None;
    }
    if v.is_free() && v.is_persist() && v.get_addr() == &publication.receiver {
        return Some((0, reader));
    }
    let def = v.get_def()?;
    let op = fd.obank().get(def)?;
    let input = op.get_in(0)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            receiver_offset(fd, input, publication, def, budget)
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_SUB => {
            let other = op.get_in(1)?;
            let (base, constant) = if fd.vbank().get(other)?.is_constant() {
                (input, other)
            } else if op.code() == OpCode::CPUI_INT_ADD && fd.vbank().get(input)?.is_constant() {
                (other, input)
            } else {
                return None;
            };
            let constant = fd.vbank().get(constant)?;
            if constant.get_size() != publication.width {
                return None;
            }
            let bits = publication.width * 8;
            let delta = if bits < 64 {
                ((constant.get_offset() << (64 - bits)) as i64) >> (64 - bits)
            } else {
                constant.get_offset() as i64
            };
            let delta = if op.code() == OpCode::CPUI_INT_SUB {
                delta.checked_neg()?
            } else {
                delta
            };
            let (offset, reader) = receiver_offset(fd, base, publication, def, budget)?;
            Some((offset.checked_add(delta)?, reader))
        }
        _ => None,
    }
}

fn receiver_reaches(fd: &Funcdata, publication: OpId, reader: OpId, p: &Publication) -> bool {
    let Some(read) = fd.obank().get(reader) else {
        return false;
    };
    let Some(parent) = read.get_parent() else {
        return false;
    };
    if fd.obank().get(publication).and_then(|op| op.get_parent()) != Some(parent) {
        return false;
    }
    let Some(receiver_space) = p.receiver.get_space() else {
        return false;
    };
    let mut previous = read.basic_neighbours().0;
    for _ in 0..MAX_RECORDS {
        let Some(id) = previous else { return false };
        if id == publication {
            return true;
        }
        let Some(op) = fd
            .obank()
            .get(id)
            .filter(|op| op.get_parent() == Some(parent))
        else {
            return false;
        };
        if matches!(
            op.code(),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        ) {
            return false;
        }
        if op
            .get_out()
            .and_then(|id| fd.vbank().get(id))
            .is_some_and(|out| {
                out.get_space().get_index() == receiver_space.get_index()
                    && overlaps(
                        p.receiver.get_offset(),
                        p.width,
                        out.get_offset(),
                        out.get_size(),
                    )
            })
        {
            return false;
        }
        if op.code() == OpCode::CPUI_STORE
            && op
                .get_in(1)
                .and_then(|id| crate::kuna_stackranges::frame_address(fd, id))
                .is_none_or(|address| {
                    fd.get_arch()
                        .manage()
                        .get_stack_space()
                        .is_none_or(|stack| address.pointer_size as u32 != stack.get_addr_size())
                })
        {
            return false;
        }
        previous = op.basic_neighbours().0;
    }
    false
}

/// Decide once while raw native receiver reads remain available, before local preservation.
fn accesses_fit_early(
    fd: &Funcdata,
    at: u64,
    width: i32,
    scalar_component: bool,
) -> Result<(), Refusal> {
    if fd.local_stack_views.unrepresented_publication {
        return Err(Refusal::Unavailable);
    }
    let stack = fd
        .get_arch()
        .manage()
        .get_stack_space()
        .ok_or(Refusal::Unavailable)?;
    let (start, size) = fd
        .local_stack_views
        .writes
        .values()
        .copied()
        .filter(|&(start, size)| {
            scalar_component && matches!(size, 1 | 2 | 4 | 8) && contains(start, size, at, width)
        })
        .max_by_key(|&(_, size)| size)
        .unwrap_or((at, width));
    if fd
        .local_stack_views
        .candidates
        .keys()
        .any(|&(at, width)| overlaps(start, size, at, width) && !contains(start, size, at, width))
    {
        return Err(Refusal::Crossing);
    }
    if locked_layout(fd, start, size) {
        return Err(Refusal::LockedLayout);
    }
    let accesses = fd
        .obank()
        .iter_code(OpCode::CPUI_STORE)
        .chain(fd.obank().iter_code(OpCode::CPUI_LOAD));
    for (count, id) in accesses.enumerate() {
        if count == MAX_RECORDS {
            return Err(Refusal::Unavailable);
        }
        let op = fd.obank().get(id).unwrap();
        if op.is_dead() {
            continue;
        }
        let space = op
            .get_in(0)
            .and_then(|id| fd.vbank().get(id))
            .filter(|v| v.is_constant())
            .ok_or(Refusal::Unavailable)?;
        let index = space.get_offset();
        if index != stack.get_index() as u64
            && stack
                .get_contain()
                .is_none_or(|space| index != space.get_index() as u64)
        {
            continue;
        }
        let pointer = op.get_in(1).ok_or(Refusal::Unavailable)?;
        let pointer_vn = fd.vbank().get(pointer).ok_or(Refusal::Unavailable)?;
        if index != stack.get_index() as u64 && pointer_vn.is_constant() {
            continue;
        }
        let bytes = (if op.code() == OpCode::CPUI_LOAD {
            op.get_out()
        } else {
            op.get_in(2)
        })
        .and_then(|id| fd.vbank().get(id))
        .map(|v| v.get_size())
        .filter(|&size| size > 0)
        .ok_or(Refusal::Unavailable)?;
        if op.code() == OpCode::CPUI_STORE {
            if let Some(NativeWrite::Store(point, original)) = fd.local_stack_views.native.get(&id)
            {
                if *original != bytes || Some(*point) != SourcePoint::op(fd, id) {
                    return Err(Refusal::Unavailable);
                }
            }
        }
        let extent = if index == stack.get_index() as u64 && pointer_vn.is_constant() {
            Some((pointer_vn.get_offset(), bytes))
        } else {
            crate::kuna_stackranges::frame_address(fd, pointer)
                .filter(|address| address.pointer_size as u32 == stack.get_addr_size())
                .and_then(|address| address.extent(bytes))
        };
        if let Some((access_at, access_width)) = extent {
            if !valid_extent(stack, access_at, access_width) {
                return Err(Refusal::Unavailable);
            }
            if overlaps(start, size, access_at, access_width)
                && !contains(start, size, access_at, access_width)
            {
                return Err(Refusal::Crossing);
            }
            continue;
        }
        let mut related = false;
        for (&publication_id, publication) in fd
            .local_stack_views
            .pending
            .iter()
            .filter(|(_, p)| contains(start, size, p.start, p.pointee.get_size()))
        {
            if let Some((delta, reader)) = receiver_offset(fd, pointer, publication, id, &mut 64) {
                let access_at = publication.start.wrapping_add_signed(delta);
                if !valid_extent(stack, access_at, bytes) {
                    return Err(Refusal::Unavailable);
                }
                if overlaps(start, size, access_at, bytes)
                    && !contains(start, size, access_at, bytes)
                {
                    return Err(Refusal::Crossing);
                }
                related |= receiver_reaches(fd, publication_id, reader, publication);
            }
        }
        if !related && crate::kuna_stackranges::may_alias(fd, pointer, start, size) {
            return Err(Refusal::Unavailable);
        }
    }
    Ok(())
}

fn stores_fit(fd: &Funcdata, start: u64, size: i32) -> bool {
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    for (count, id) in fd.obank().iter_code(OpCode::CPUI_STORE).enumerate() {
        if count == MAX_RECORDS {
            return false;
        }
        let op = fd.obank().get(id).unwrap();
        if op.is_dead() {
            continue;
        }
        let Some(space) = op
            .get_in(0)
            .and_then(|id| fd.vbank().get(id))
            .filter(|v| v.is_constant())
        else {
            return false;
        };
        let index = space.get_offset();
        if index != stack.get_index() as u64
            && stack
                .get_contain()
                .is_none_or(|space| index != space.get_index() as u64)
        {
            continue;
        }
        let Some(pointer) = op.get_in(1).and_then(|id| fd.vbank().get(id)) else {
            return false;
        };
        if index != stack.get_index() as u64 && pointer.is_constant() {
            continue;
        }
        let Some(width) = op
            .get_in(2)
            .and_then(|id| fd.vbank().get(id))
            .map(|v| v.get_size())
            .filter(|&width| width > 0)
        else {
            return false;
        };
        if let Some(NativeWrite::Store(point, original)) = fd.local_stack_views.native.get(&id) {
            if *original != width || Some(*point) != SourcePoint::op(fd, id) {
                return false;
            }
        }
        let extent = if index == stack.get_index() as u64 && pointer.is_constant() {
            Some((pointer.get_offset(), width))
        } else {
            crate::kuna_stackranges::frame_address(fd, op.get_in(1).unwrap())
                .filter(|address| address.pointer_size as u32 == stack.get_addr_size())
                .and_then(|address| address.extent(width))
        };
        let Some((at, width)) = extent else {
            return false;
        };
        if !valid_extent(stack, at, width) {
            return false;
        }
        if overlaps(start, size, at, width) && !contains(start, size, at, width) {
            return false;
        }
    }
    true
}

/// Byte views require one proved scalar extent, or one exact containing native STORE.
pub(crate) fn backing(fd: &Funcdata, start: u64, size: i32) -> bool {
    if aggregate(fd, start, size) {
        return true;
    }
    if !active(fd) || !matches!(size, 1 | 2 | 4 | 8) || !stores_fit(fd, start, size) {
        return false;
    }
    let writes = &fd.local_stack_views.writes;
    if writes.values().any(|&(at, width)| {
        overlaps(start, size, at, width)
            && (!matches!(width, 1 | 2 | 4 | 8) || !contains(start, size, at, width))
    }) {
        return false;
    }
    let exact = fd.local_stack_views.footprints.contains_key(&(start, size));
    let native_component = writes
        .values()
        .any(|&(at, width)| at == start && width == size);
    if !exact && !native_component {
        return false;
    }
    fd.local_stack_views.publications.values().any(|p| {
        let width = p.pointee.get_size();
        if !scalar(&p.pointee) || !contains(start, size, p.start, width) {
            return false;
        }
        let interior = p.start != start || width != size;
        (interior && native_component)
            || (!interior
                && exact
                && writes.values().any(|&(at, width)| {
                    contains(start, size, at, width) && (at != start || width != size)
                }))
    })
}

pub(crate) fn mapped_range(fd: &Funcdata, start: u64, size: i32) -> bool {
    if !active(fd) {
        return false;
    }
    let Some(scope) = fd.get_scope_local() else {
        return false;
    };
    let address = Address::new(scope.get_space_id().clone(), start);
    scope
        .query_container_for_link_width(&address, size, &Address::new_invalid())
        .is_some_and(|info| {
            info.sym_type
                .as_ref()
                .is_some_and(|ty| ty.get_name().starts_with("stack_views_"))
                && backing(fd, info.entry_addr.get_offset(), info.entry_size)
        })
}

pub(crate) fn views(fd: &Funcdata) -> Vec<(u64, Rc<Datatype>)> {
    if !active(fd) {
        return Vec::new();
    }
    fd.local_stack_views
        .footprints
        .iter()
        .flat_map(|(&(start, _), views)| views.iter().map(move |ty| (start, Rc::clone(ty))))
        .collect()
}

#[cfg(test)]
mod tests;
