//! Logical frame-object identities from byte definitions at declared uses.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;

const MAX_BYTES: usize = 4096;
const MAX_OBJECT_SIZE: i32 = 256;
const MAX_STEPS: usize = 65536;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourcePoint {
    pub space: i32,
    pub address: u64,
}

impl SourcePoint {
    pub(crate) fn op(fd: &Funcdata, id: OpId) -> Option<Self> {
        let address = fd.obank().get(id)?.get_addr();
        Some(Self {
            space: address.get_space()?.get_index(),
            address: address.get_offset(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ByteSource {
    Incoming,
    Write(SourcePoint),
    Effect(SourcePoint),
    Unknown,
}

#[derive(Clone)]
pub struct ObjectUse {
    pub point: SourcePoint,
    pub slot: i32,
    pub datatype: Rc<Datatype>,
    pub(crate) op: OpId,
}

#[derive(Clone)]
pub struct StackObject {
    pub name: String,
    pub address: Address,
    pub size: i32,
    pub bytes: Vec<BTreeSet<ByteSource>>,
    pub uses: Vec<ObjectUse>,
}

impl StackObject {
    pub fn is_defined(&self) -> bool {
        self.bytes.iter().all(|byte| {
            !byte.is_empty()
                && byte
                    .iter()
                    .all(|source| matches!(source, ByteSource::Write(_)))
        })
    }
}

pub fn objects(fd: &Funcdata) -> &[StackObject] {
    &fd.stack_objects
}

struct Origins<'a> {
    fd: &'a Funcdata,
    space: Rc<AddrSpace>,
    budget: usize,
    cache: BTreeMap<(OpId, u64), BTreeSet<ByteSource>>,
}

impl Origins<'_> {
    fn step(&mut self) -> Option<()> {
        self.budget = self.budget.checked_sub(1)?;
        Some(())
    }

    fn contains(&self, start: u64, size: i32, byte: u64) -> Option<u32> {
        let offset = self.space.wrap_offset(byte.wrapping_sub(start));
        (offset < size as u64).then_some(offset as u32)
    }

    fn effect(&self, id: OpId, byte: u64) -> Option<bool> {
        let op = self.fd.obank().get(id)?;
        if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            let call = self
                .fd
                .get_call_specs_index(id)
                .map(|index| self.fd.get_call_specs(index));
            return Some(!call.is_some_and(|call| {
                crate::kuna_calleememory::preserves(
                    self.fd,
                    call,
                    &Address::new(Rc::clone(&self.space), byte),
                    1,
                )
            }));
        }
        if op.code() == OpCode::CPUI_STORE {
            let pointer = self.fd.vbank().get(op.get_in(1)?)?;
            if pointer.is_constant() {
                return Some(false);
            }
            if let Some(address) = crate::kuna_stackranges::frame_address(self.fd, op.get_in(1)?) {
                let width = self.fd.vbank().get(op.get_in(2)?)?.get_size();
                let (start, size) = address.extent(width)?;
                return Some(self.contains(start, size, byte).is_some());
            }
            return Some(true);
        }
        Some(op.code() == OpCode::CPUI_CALLOTHER)
    }

    fn before(&mut self, point: OpId, byte: u64) -> BTreeSet<ByteSource> {
        if let Some(result) = self.cache.get(&(point, byte)) {
            return result.clone();
        }
        let result = self
            .search(point, byte)
            .unwrap_or_else(|| BTreeSet::from([ByteSource::Unknown]));
        self.cache.insert((point, byte), result.clone());
        result
    }

    fn search(&mut self, point: OpId, byte: u64) -> Option<BTreeSet<ByteSource>> {
        let op = self.fd.obank().get(point)?;
        let mut work = vec![(op.get_parent()?, op.basic_neighbours().0)];
        let mut visited = BTreeSet::new();
        let mut result = BTreeSet::new();
        while let Some((block, mut previous)) = work.pop() {
            if !visited.insert((block, previous)) {
                continue;
            }
            self.step()?;
            let mut found = false;
            while let Some(id) = previous {
                self.step()?;
                let op = self.fd.obank().get(id)?;
                previous = op.basic_neighbours().0;
                if op.code() == OpCode::CPUI_INDIRECT && self.affector(id) == Some(point) {
                    continue;
                }
                if let Some(out) = op.get_out().and_then(|out| self.fd.vbank().get(out)) {
                    if out.get_space().get_index() == self.space.get_index() {
                        if let Some(offset) = self.contains(out.get_offset(), out.get_size(), byte)
                        {
                            result.extend(self.value(op.get_out()?, offset)?);
                            found = true;
                            break;
                        }
                    }
                }
                if self.effect(id, byte)? {
                    result.insert(ByteSource::Effect(SourcePoint::op(self.fd, id)?));
                }
            }
            if found {
                continue;
            }
            let block = self.fd.bblocks_ref().block(block);
            if block.size_in() == 0 {
                result.insert(ByteSource::Incoming);
            }
            for input in 0..block.size_in() {
                let predecessor = block.get_in(input);
                work.push((predecessor, self.fd.bb_op_tail(predecessor)));
            }
        }
        if result.is_empty() {
            result.insert(ByteSource::Unknown);
        }
        Some(result)
    }

    fn affector(&self, id: OpId) -> Option<OpId> {
        let op = self.fd.obank().get(id)?;
        let input = self.fd.vbank().get(op.get_in(1)?)?;
        let iop = self.fd.get_arch().manage().get_iop_space()?;
        (input.get_space().get_index() == iop.get_index())
            .then(|| crate::funcdata_varnode::op_iop_decode(input.get_offset()))
    }

    fn native_write_before(&mut self, id: OpId, byte: u64) -> Option<SourcePoint> {
        let source = SourcePoint::op(self.fd, id)?;
        let mut previous = self.fd.obank().get(id)?.basic_neighbours().0;
        while let Some(id) = previous {
            self.step()?;
            let op = self.fd.obank().get(id)?;
            if SourcePoint::op(self.fd, id)? != source {
                break;
            }
            if op.get_out().and_then(|id| self.fd.vbank().get(id)).is_some_and(|out| {
                out.is_stack_store()
                    && out.get_space().get_index() == self.space.get_index()
                    && self.contains(out.get_offset(), out.get_size(), byte).is_some()
            }) {
                return Some(source);
            }
            if self.effect(id, byte)? {
                break;
            }
            previous = op.basic_neighbours().0;
        }
        None
    }

    fn value(&mut self, value: VarnodeId, byte: u32) -> Option<BTreeSet<ByteSource>> {
        let mut work = vec![(value, byte)];
        let mut visited = BTreeSet::new();
        let mut result = BTreeSet::new();
        while let Some((value, byte)) = work.pop() {
            if !visited.insert((value, byte)) {
                continue;
            }
            self.step()?;
            let v = self.fd.vbank().get(value)?;
            if byte >= v.get_size() as u32 {
                return None;
            }
            let Some(id) = v.get_def() else {
                result.insert(if v.is_input() {
                    ByteSource::Incoming
                } else {
                    ByteSource::Unknown
                });
                continue;
            };
            let op = self.fd.obank().get(id)?;
            if !v.is_stack_store()
                && v.get_space().get_index() == self.space.get_index()
                && !op.is_marker()
            {
                let address = self.space.wrap_offset(v.get_offset().wrapping_add(u64::from(byte)));
                if let Some(write) = self.native_write_before(id, address) {
                    result.insert(ByteSource::Write(write));
                    continue;
                }
            }
            if v.is_stack_store()
                && v.get_space().get_index() == self.space.get_index()
                && !matches!(
                    op.code(),
                    OpCode::CPUI_MULTIEQUAL
                        | OpCode::CPUI_INDIRECT
                        | OpCode::CPUI_PIECE
                        | OpCode::CPUI_SUBPIECE
                )
            {
                result.insert(ByteSource::Write(SourcePoint::op(self.fd, id)?));
                continue;
            }
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => work.push((op.get_in(0)?, byte)),
                OpCode::CPUI_MULTIEQUAL => {
                    for slot in 0..op.num_input() {
                        work.push((op.get_in(slot)?, byte));
                    }
                }
                OpCode::CPUI_INDIRECT => {
                    let effect = self.affector(id)?;
                    if v.get_space().get_index() != self.space.get_index()
                        || self.effect(
                            effect,
                            self.space
                                .wrap_offset(v.get_offset().wrapping_add(u64::from(byte))),
                        )?
                    {
                        result.insert(ByteSource::Effect(SourcePoint::op(self.fd, effect)?));
                    }
                    if !op.is_indirect_creation() {
                        work.push((op.get_in(0)?, byte));
                    }
                }
                OpCode::CPUI_PIECE => {
                    let high = op.get_in(0)?;
                    let low = op.get_in(1)?;
                    let (first, second) = if self.space.is_big_endian() {
                        (high, low)
                    } else {
                        (low, high)
                    };
                    let width = self.fd.vbank().get(first)?.get_size() as u32;
                    work.push(if byte < width {
                        (first, byte)
                    } else {
                        (second, byte - width)
                    });
                }
                OpCode::CPUI_SUBPIECE => {
                    let input = op.get_in(0)?;
                    let offset = self.fd.vbank().get(op.get_in(1)?)?;
                    if !offset.is_constant() {
                        return None;
                    }
                    let offset: u32 = offset.get_offset().try_into().ok()?;
                    let offset = if self.space.is_big_endian() {
                        (self.fd.vbank().get(input)?.get_size() as u32)
                            .checked_sub(v.get_size() as u32)?
                            .checked_sub(offset)?
                    } else {
                        offset
                    };
                    work.push((input, byte.checked_add(offset)?));
                }
                _ => {
                    result.insert(ByteSource::Unknown);
                }
            }
        }
        if result.is_empty() {
            result.insert(ByteSource::Unknown);
        }
        Some(result)
    }
}

/// Keep content families separate without assigning them different addresses.
pub fn recover(fd: &mut Funcdata) {
    fd.stack_objects.clear();
    if !fd.get_arch().stack_views {
        return;
    }
    let Some(space) = fd
        .get_scope_local()
        .map(|scope| Rc::clone(scope.get_space_id()))
    else {
        return;
    };
    if space.get_word_size() != 1 || !(1..=8).contains(&space.get_addr_size()) {
        return;
    }
    let mut requests = Vec::new();
    let mut total = 0;
    for call in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(call).filter(|op| !op.is_dead()) else {
            continue;
        };
        for slot in 0..op.num_input() {
            let Some(ty) = crate::kuna_stackviews::declared_view(fd, call, slot) else {
                continue;
            };
            if ty.get_size() > MAX_OBJECT_SIZE {
                continue;
            }
            let Some(address) = op
                .get_in(slot)
                .and_then(|input| crate::kuna_stackranges::frame_address(fd, input))
            else {
                continue;
            };
            if address.first != address.last || !address.terms.is_empty() {
                continue;
            }
            total += ty.get_size() as usize;
            if total > MAX_BYTES {
                break;
            }
            let Some(point) = SourcePoint::op(fd, call) else {
                continue;
            };
            requests.push((
                space.wrap_offset(address.first as u64),
                ty.get_size(),
                ObjectUse {
                    point,
                    slot,
                    datatype: ty,
                    op: call,
                },
            ));
        }
        if total > MAX_BYTES {
            break;
        }
    }
    requests.sort_by_key(|(start, size, use_)| (use_.point, use_.slot, *start, *size));
    let mut origins = Origins {
        fd,
        space: Rc::clone(&space),
        budget: MAX_STEPS,
        cache: BTreeMap::new(),
    };
    let mut keys: BTreeMap<_, usize> = BTreeMap::new();
    let mut objects: Vec<StackObject> = Vec::new();
    for (start, size, use_) in requests {
        let bytes: Vec<_> = (0..size)
            .map(|byte| origins.before(use_.op, space.wrap_offset(start.wrapping_add(byte as u64))))
            .collect();
        let defined = bytes.iter().all(|byte| {
            !byte.is_empty()
                && byte
                    .iter()
                    .all(|source| matches!(source, ByteSource::Write(_)))
        });
        let key = (start, size, bytes.clone(), (!defined).then_some(use_.point));
        if let Some(&index) = keys.get(&key) {
            objects[index].uses.push(use_);
        } else {
            keys.insert(key, objects.len());
            objects.push(StackObject {
                name: format!("object_{:x}_{}", use_.point.address, use_.slot),
                address: Address::new(Rc::clone(&space), start),
                size,
                bytes,
                uses: vec![use_],
            });
        }
    }
    fd.stack_objects = objects;
}

pub(crate) fn finish(fd: &mut Funcdata) {
    if fd.stack_objects.is_empty() {
        recover(fd);
        return;
    }
    let mut objects = std::mem::take(&mut fd.stack_objects);
    for object in &mut objects {
        object
            .uses
            .retain(|use_| fd.obank().get(use_.op).is_some_and(|op| !op.is_dead()));
        if let Some(use_) = object.uses.first() {
            object.name = format!("object_{:x}_{}", use_.point.address, use_.slot);
        }
    }
    objects.retain(|object| !object.uses.is_empty());
    fd.stack_objects = objects;
}

pub fn describe(fd: &Funcdata) -> String {
    use std::fmt::Write;
    let mut text = String::new();
    for object in objects(fd) {
        let space = object
            .address
            .get_space()
            .expect("recovered object has a space");
        let bits = space.get_addr_size() * 8;
        let offset = if bits == 64 {
            object.address.get_offset() as i64
        } else {
            ((object.address.get_offset() << (64 - bits)) as i64) >> (64 - bits)
        };
        let _ = writeln!(
            text,
            "{} stack={}{:#x} size={} state={}",
            object.name,
            if offset < 0 { "-" } else { "" },
            offset.unsigned_abs(),
            object.size,
            if object.is_defined() {
                "defined"
            } else {
                "uncertain"
            }
        );
        for use_ in &object.uses {
            let _ = writeln!(
                text,
                "  use {:#x}:{} {}",
                use_.point.address,
                use_.slot,
                use_.datatype.get_name()
            );
        }
        let mut at = 0;
        while at < object.bytes.len() {
            let mut end = at + 1;
            while end < object.bytes.len() && object.bytes[end] == object.bytes[at] {
                end += 1;
            }
            let sources = object.bytes[at]
                .iter()
                .map(|source| match source {
                    ByteSource::Incoming => "incoming".to_string(),
                    ByteSource::Unknown => "unknown".to_string(),
                    ByteSource::Write(point) => format!("{:#x}", point.address),
                    ByteSource::Effect(point) => format!("effect@{:#x}", point.address),
                })
                .collect::<Vec<_>>()
                .join("|");
            let _ = writeln!(text, "  bytes {at}..{end} {sources}");
            at = end;
        }
    }
    text
}

#[cfg(test)]
mod tests;
