//! Bounded, body-derived memory effects for opt-in frame lifetime recovery.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::context::OpId;
use crate::fspec::{FuncCallSpecs, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;

const MAX_STEPS: usize = 256;
const MAX_VALUES: usize = 256;
const MAX_EXTENT: i64 = 65536;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Storage {
    space: i32,
    offset: u64,
    size: u32,
}

impl Storage {
    fn raw(value: &VarnodeData) -> Option<Self> {
        Some(Self {
            space: value.space.as_ref()?.get_index(),
            offset: value.offset,
            size: value.size,
        })
    }

    fn overlaps(self, other: Self) -> bool {
        self.space == other.space
            && self.offset < other.offset.saturating_add(u64::from(other.size))
            && other.offset < self.offset.saturating_add(u64::from(self.size))
    }

    fn contains(self, other: Self) -> bool {
        if self.space != other.space || self.offset > other.offset {
            return false;
        }
        let Some(end) = self.offset.checked_add(u64::from(self.size)) else {
            return false;
        };
        let Some(other_end) = other.offset.checked_add(u64::from(other.size)) else {
            return false;
        };
        end >= other_end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Origin {
    Register(Storage),
    StackArgument(i64, u32),
    Frame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Value {
    Constant(u64),
    Pointer(Origin, i64),
    ReturnAddress,
    Unknown,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    values: BTreeMap<Storage, Value>,
    spills: BTreeMap<(i64, u32), Value>,
    return_clobbered: bool,
    arguments_clobbered: bool,
}

impl State {
    fn read(&self, value: &VarnodeData, register: i32, stack: Storage) -> Value {
        let Some(space) = value.space.as_ref() else {
            return Value::Unknown;
        };
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Value::Constant(value.offset);
        }
        let Some(key) = Storage::raw(value) else {
            return Value::Unknown;
        };
        if let Some(value) = self.values.get(&key) {
            return *value;
        }
        if self.values.keys().any(|other| other.overlaps(key)) {
            return Value::Unknown;
        }
        if key == stack {
            Value::Pointer(Origin::Frame, 0)
        } else if key.space == register && key.size == stack.size {
            Value::Pointer(Origin::Register(key), 0)
        } else {
            Value::Unknown
        }
    }

    fn write(&mut self, out: &VarnodeData, value: Value) -> Option<()> {
        let key = Storage::raw(out)?;
        self.values.retain(|other, _| !other.overlaps(key));
        self.values.insert(key, value);
        (self.values.len() <= MAX_VALUES).then_some(())
    }
}

fn pointer_sources(
    state: &State,
    input: &VarnodeData,
    register: i32,
    stack: Storage,
) -> (BTreeSet<Origin>, BTreeSet<Storage>) {
    let mut roots = BTreeSet::new();
    let mut ranges = BTreeSet::new();
    if let Value::Pointer(root, _) = state.read(input, register, stack) {
        roots.insert(root);
    }
    if let Some(key) = Storage::raw(input) {
        let mut initialized = false;
        let mut overlaps_pointer = false;
        for (stored, value) in &state.values {
            if stored.contains(key) {
                initialized = true;
            }
            if stored.overlaps(key) {
                if let Value::Pointer(root, _) = value {
                    roots.insert(*root);
                    overlaps_pointer = true;
                }
            }
        }
        let may_be_entry_fragment = !initialized || overlaps_pointer;
        if key.space == register && may_be_entry_fragment {
            ranges.insert(key);
        }
        if key.space == stack.space && key.overlaps(stack) && may_be_entry_fragment {
            roots.insert(Origin::Frame);
        }
    }
    (roots, ranges)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Effect {
    root: Origin,
    offset: i64,
    size: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct DeferredFrameEscape {
    offset: i64,
    size: u32,
    root: Origin,
}

#[derive(Clone, Debug, Default)]
pub struct CalleeMemory {
    complete: bool,
    reads: BTreeSet<Effect>,
    writes: BTreeSet<Effect>,
    escapes: BTreeSet<Origin>,
    deferred_frame_escapes: BTreeSet<DeferredFrameEscape>,
    home_write_conditions: BTreeSet<(DeferredFrameEscape, Effect)>,
    lossy_pointer_roots: BTreeSet<Origin>,
    lossy_pointer_ranges: BTreeSet<Storage>,
    lossy_pointer_overflow: bool,
    returns: Vec<State>,
}

impl CalleeMemory {
    fn note_lossy_root(&mut self, root: Origin) {
        if self.lossy_pointer_overflow || self.lossy_pointer_roots.contains(&root) {
            return;
        }
        if self.lossy_pointer_roots.len() + self.lossy_pointer_ranges.len() >= MAX_VALUES {
            self.lossy_pointer_overflow = true;
            return;
        }
        self.lossy_pointer_roots.insert(root);
    }

    fn note_lossy_range(&mut self, range: Storage) {
        if self.lossy_pointer_overflow || self.lossy_pointer_ranges.contains(&range) {
            return;
        }
        if self.lossy_pointer_roots.len() + self.lossy_pointer_ranges.len() >= MAX_VALUES {
            self.lossy_pointer_overflow = true;
            return;
        }
        self.lossy_pointer_ranges.insert(range);
    }

    fn note_lossy_sources(
        &mut self,
        roots: impl IntoIterator<Item = Origin>,
        ranges: impl IntoIterator<Item = Storage>,
    ) {
        for root in roots {
            self.note_lossy_root(root);
        }
        for range in ranges {
            self.note_lossy_range(range);
        }
    }
}

fn note_partial_pointer_clobber(summary: &mut CalleeMemory, state: &State, output: &VarnodeData) {
    let Some(key) = Storage::raw(output) else {
        return;
    };
    for (stored, value) in &state.values {
        if stored.overlaps(key) && !key.contains(*stored) {
            if let Value::Pointer(root, _) = value {
                summary.note_lossy_root(*root);
            }
        }
    }
}

#[derive(Clone)]
struct RawOp {
    code: OpCode,
    out: Option<VarnodeData>,
    inputs: Vec<VarnodeData>,
}

#[derive(Default)]
struct Emit(Vec<RawOp>);

impl kuna_sleigh::translate::PcodeEmit for Emit {
    fn dump(
        &mut self,
        _: &Address,
        code: OpCode,
        out: Option<&VarnodeData>,
        inputs: &[VarnodeData],
    ) {
        self.0.push(RawOp {
            code,
            out: out.cloned(),
            inputs: inputs.to_vec(),
        });
    }
}

fn signed(value: u64, size: u32) -> Option<i64> {
    let bits = size.checked_mul(8)?;
    match bits {
        1..=63 => Some(((value << (64 - bits)) as i64) >> (64 - bits)),
        64 => Some(value as i64),
        _ => None,
    }
}

fn arithmetic(code: OpCode, left: Value, right: Value, size: u32) -> Option<Value> {
    match (code, left, right) {
        (
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB,
            Value::Pointer(root, offset),
            Value::Constant(c),
        ) => {
            let delta = signed(c, size)?;
            let offset = if code == OpCode::CPUI_INT_SUB {
                offset.checked_sub(delta)?
            } else {
                offset.checked_add(delta)?
            };
            (offset.unsigned_abs() < MAX_EXTENT as u64).then_some(Value::Pointer(root, offset))
        }
        (OpCode::CPUI_INT_ADD, Value::Constant(c), Value::Pointer(root, offset)) => {
            arithmetic(code, Value::Pointer(root, offset), Value::Constant(c), size)
        }
        (OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB, Value::Constant(a), Value::Constant(b)) => {
            let mask = kuna_base::address::calc_mask(size as i32);
            Some(Value::Constant(
                if code == OpCode::CPUI_INT_SUB {
                    a.wrapping_sub(b)
                } else {
                    a.wrapping_add(b)
                } & mask,
            ))
        }
        (_, Value::Pointer(..), _) | (_, _, Value::Pointer(..)) => None,
        _ => Some(Value::Unknown),
    }
}

fn memory(
    summary: &mut CalleeMemory,
    state: &mut State,
    op: &RawOp,
    stack: Storage,
    register: i32,
    memory_space: i32,
) -> Option<Value> {
    if op.inputs.first()?.offset as i32 != memory_space {
        return None;
    }
    let pointer = state.read(op.inputs.get(1)?, register, stack);
    let writing = op.code == OpCode::CPUI_STORE;
    let raw = if writing {
        op.inputs.get(2)?
    } else {
        op.out.as_ref()?
    };
    let size = raw.size;
    if size == 0 || size > 16 {
        return None;
    }
    let value = if writing {
        state.read(raw, register, stack)
    } else {
        Value::Unknown
    };
    if writing && value == Value::Unknown {
        let (roots, ranges) = pointer_sources(state, raw, register, stack);
        summary.note_lossy_sources(roots, ranges);
    }
    if let Value::Pointer(root, offset) = pointer {
        let end = offset.checked_add(i64::from(size))?;
        if root == Origin::Frame {
            if offset < -MAX_EXTENT || end > MAX_EXTENT {
                return None;
            }
            if end > i64::from(stack.size) {
                let effect = Effect { root, offset, size };
                if writing {
                    summary.writes.insert(effect);
                } else {
                    summary.reads.insert(effect);
                }
                (summary.reads.len() + summary.writes.len() <= MAX_VALUES).then_some(())?;
            }
            if writing {
                state.return_clobbered |= offset < i64::from(stack.size) && end > 0;
                state
                    .spills
                    .retain(|&(at, width), _| at >= end || at + i64::from(width) <= offset);
                state.spills.insert((offset, size), value);
                (state.spills.len() <= 64).then_some(())?;
            } else {
                let loaded = state
                    .spills
                    .get(&(offset, size))
                    .copied()
                    .unwrap_or_else(|| {
                        if offset == 0 && size == stack.size && !state.return_clobbered {
                            Value::ReturnAddress
                        } else if offset >= i64::from(stack.size)
                            && size == stack.size
                            && !state.arguments_clobbered
                            && !state
                                .spills
                                .keys()
                                .any(|&(at, width)| at < end && at + i64::from(width) > offset)
                        {
                            Value::Pointer(Origin::StackArgument(offset, size), 0)
                        } else {
                            Value::Unknown
                        }
                    });
                if loaded == Value::Unknown {
                    let lost_roots: Vec<_> = summary
                        .deferred_frame_escapes
                        .iter()
                        .filter(|home| offset == home.offset && size == home.size)
                        .map(|home| home.root)
                        .collect();
                    for root in lost_roots {
                        summary.note_lossy_root(root);
                    }
                }
                return Some(loaded);
            }
        } else {
            let effect = Effect { root, offset, size };
            if writing {
                let protected_homes: Vec<_> = state
                    .spills
                    .iter()
                    .filter_map(|(&(at, width), &value)| {
                        if width == stack.size
                            && crate::p3_dataflow::kuna_calleehomes::is_home_slot(at, width)
                        {
                            if let Value::Pointer(root, _) = value {
                                return Some(DeferredFrameEscape {
                                    offset: at,
                                    size: width,
                                    root,
                                });
                            }
                        }
                        None
                    })
                    .collect();
                if protected_homes.is_empty() {
                    state.spills.clear();
                } else {
                    state.spills.retain(|&(at, width), &mut value| {
                        width == stack.size
                            && crate::p3_dataflow::kuna_calleehomes::is_home_slot(at, width)
                            && matches!(value, Value::Pointer(..))
                    });
                    for home in protected_homes {
                        summary.home_write_conditions.insert((home, effect));
                    }
                    (summary.home_write_conditions.len() <= MAX_VALUES).then_some(())?;
                }
                state.arguments_clobbered = true;
                summary.writes.insert(effect);
            } else {
                summary.reads.insert(effect);
            }
        }
    } else if !matches!(pointer, Value::Constant(_)) {
        return None;
    }
    if writing {
        let local_frame_store = matches!(pointer, Value::Pointer(Origin::Frame, offset)
            if offset + i64::from(size) <= i64::from(stack.size));
        if !local_frame_store {
            match (pointer, value) {
                (Value::Pointer(Origin::Frame, offset), Value::Pointer(root, _)) => {
                    summary.deferred_frame_escapes.insert(DeferredFrameEscape {
                        offset,
                        size,
                        root,
                    });
                    (summary.escapes.len() + summary.deferred_frame_escapes.len() <= MAX_VALUES)
                        .then_some(())?;
                }
                (_, Value::Pointer(root, _)) => {
                    summary.escapes.insert(root);
                }
                (_, Value::Unknown) if size == stack.size => return None,
                _ => (),
            }
        }
    }
    (summary.reads.len() + summary.writes.len() <= MAX_VALUES).then_some(Value::Unknown)
}

fn walk(
    mut decode: impl FnMut(&Address) -> Option<(i32, Vec<RawOp>)>,
    entry: &Address,
    stack: Storage,
    register: i32,
    memory_space: i32,
    link: Option<Storage>,
) -> Option<CalleeMemory> {
    let mut result = CalleeMemory::default();
    let mut substantive = false;
    let mut initial = State::default();
    if let Some(link) = link {
        initial.values.insert(link, Value::ReturnAddress);
    }
    let mut work = vec![(entry.clone(), initial)];
    let mut visited = BTreeSet::new();
    let mut budget = MAX_STEPS;
    while let Some((at, mut state)) = work.pop() {
        state.values.retain(|key, _| key.space == register);
        let code_space = at.get_space()?;
        if !visited.insert((code_space.get_index(), at.get_offset(), state.clone())) {
            continue;
        }
        budget = budget.checked_sub(1)?;
        let (length, ops) = decode(&at)?;
        if length <= 0 || ops.len() > MAX_VALUES {
            return None;
        }
        let mut ends = false;
        for op in ops {
            match op.code {
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_BRANCHIND => return None,
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    let target = op.inputs.first()?;
                    let space = target.space.as_ref()?;
                    if space.get_index() != code_space.get_index() {
                        return None;
                    }
                    work.push((Address::new(Rc::clone(space), target.offset), state.clone()));
                    if op.code == OpCode::CPUI_BRANCH {
                        ends = true;
                        break;
                    }
                }
                OpCode::CPUI_RETURN => {
                    if state.read(op.inputs.first()?, register, stack) != Value::ReturnAddress {
                        return None;
                    }
                    state.values.retain(|key, _| key.space == register);
                    result.returns.push(state.clone());
                    if result.returns.len() > 64 {
                        return None;
                    }
                    ends = true;
                    break;
                }
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    let value =
                        memory(&mut result, &mut state, &op, stack, register, memory_space)?;
                    substantive |=
                        value != Value::ReturnAddress
                            && (op.code == OpCode::CPUI_STORE
                                || op.out.as_ref().and_then(Storage::raw).is_some_and(|key| {
                                    key.space == register && !key.overlaps(stack)
                                }));
                    if let Some(out) = op.out.as_ref() {
                        note_partial_pointer_clobber(&mut result, &state, out);
                        state.write(out, value)?;
                    }
                }
                _ => {
                    let Some(out) = op.out.as_ref() else {
                        return None;
                    };
                    let space = out.space.as_ref()?;
                    if space.get_index() != register && space.get_type() != spacetype::IPTR_INTERNAL
                    {
                        return None;
                    }
                    let input =
                        |slot: usize| op.inputs.get(slot).map(|v| state.read(v, register, stack));
                    let (input_roots, input_ranges) = op.inputs.iter().fold(
                        (BTreeSet::new(), BTreeSet::new()),
                        |(mut roots, mut ranges), v| {
                            let (new_roots, new_ranges) =
                                pointer_sources(&state, v, register, stack);
                            roots.extend(new_roots);
                            ranges.extend(new_ranges);
                            (roots, ranges)
                        },
                    );
                    let value = match op.code {
                        OpCode::CPUI_COPY | OpCode::CPUI_CAST
                            if op.inputs.first()?.size == out.size =>
                        {
                            input(0)?
                        }
                        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB if out.size == stack.size => {
                            arithmetic(op.code, input(0)?, input(1)?, out.size)?
                        }
                        _ => {
                            if out.size >= stack.size
                                && op.inputs.iter().any(|v| {
                                    matches!(state.read(v, register, stack), Value::Pointer(..))
                                })
                            {
                                return None;
                            }
                            Value::Unknown
                        }
                    };
                    if value == Value::Unknown {
                        result.note_lossy_sources(input_roots, input_ranges);
                    }
                    substantive |= value != Value::ReturnAddress
                        && Storage::raw(out)
                            .is_some_and(|key| key.space == register && !key.overlaps(stack));
                    note_partial_pointer_clobber(&mut result, &state, out);
                    state.write(out, value)?;
                }
            }
        }
        if !ends {
            work.push((
                Address::new(
                    Rc::clone(code_space),
                    at.get_offset().checked_add(length as u64)?,
                ),
                state,
            ));
        }
    }
    if result.returns.is_empty() || !substantive {
        return None;
    }
    result.complete = true;
    Some(result)
}

pub fn seed(arch: &mut crate::architecture::Architecture, fd: &mut Funcdata) {
    if !arch.stack_views && !arch.stack_alias_deadstore {
        return;
    }
    let Some(stack_space) = arch.manage().get_stack_space() else {
        return;
    };
    if !stack_space.stack_grows_negative() || stack_space.get_word_size() != 1 {
        return;
    }
    let Some(stack) = stack_space.get_spacebase(0).ok().and_then(|v| {
        v.space.map(|space| Storage {
            space: space.get_index(),
            offset: v.offset,
            size: v.size,
        })
    }) else {
        return;
    };
    let Some(register) = arch
        .manage()
        .get_space_by_name("register")
        .map(|s| s.get_index())
    else {
        return;
    };
    let Some(data_space) = arch.manage().get_default_data_space() else {
        return;
    };
    if data_space.get_word_size() != 1 {
        return;
    }
    let memory_space = data_space.get_index();
    let link = fd
        .get_arch()
        .default_return_addr
        .as_ref()
        .and_then(Storage::raw)
        .filter(|key| key.space == register);
    for i in 0..fd.num_calls() {
        let entry = fd.get_call_specs(i).get_entry_address().clone();
        let Some(space) = entry.get_space().filter(|_| !entry.is_invalid()) else {
            continue;
        };
        let key = (space.get_index(), entry.get_offset());
        if !arch.kuna_callee_memory_cache.contains_key(&key) {
            let translator = arch.translate();
            let summary = walk(
                |at| {
                    let mut emit = Emit::default();
                    let length = translator.one_instruction(&mut emit, at).ok()?;
                    Some((length, emit.0))
                },
                &entry,
                stack,
                register,
                memory_space,
                link,
            )
            .unwrap_or_default();
            arch.kuna_callee_memory_cache.insert(key, Rc::new(summary));
        }
        if let Some(summary) = arch.kuna_callee_memory_cache.get(&key) {
            fd.callee_memory.insert(key, Rc::clone(summary));
        }
    }
}

fn argument(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    root: Origin,
) -> Option<crate::context::VarnodeId> {
    for slot in 0..call.proto().num_params() {
        let param = call.proto().get_param(slot)?;
        let address = param.get_address();
        let matches = match root {
            Origin::Register(storage) => {
                address.get_space().map(|s| s.get_index()) == Some(storage.space)
                    && address.get_offset() == storage.offset
                    && param.get_size() == storage.size as i32
            }
            Origin::StackArgument(offset, size) => {
                address.get_space().map(|s| s.get_index())
                    == fd.get_scope_local().map(|s| s.get_space_id().get_index())
                    && signed(address.get_offset(), address.get_space()?.get_addr_size())? == offset
                    && param.get_size() == size as i32
            }
            Origin::Frame => false,
        };
        if matches {
            return fd.obank().get(call.get_op())?.get_in(slot + 1);
        }
    }
    None
}

fn register_range_argument(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    range: Storage,
) -> Option<crate::context::VarnodeId> {
    for slot in 0..call.proto().num_params() {
        let param = call.proto().get_param(slot)?;
        let size = u32::try_from(param.get_size()).ok()?;
        let address = param.get_address();
        let storage = Storage {
            space: address.get_space()?.get_index(),
            offset: address.get_offset(),
            size,
        };
        if storage.contains(range) {
            return fd.obank().get(call.get_op())?.get_in(slot + 1);
        }
    }
    None
}

fn reaches(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    effect: Effect,
    start: u64,
    size: i32,
    wanted: u32,
) -> Option<bool> {
    let space = fd.get_scope_local()?.get_space_id();
    let (first, last) = if effect.root == Origin::Frame {
        let at = signed(call.get_spacebase_offset(), space.get_addr_size())?;
        (at, at)
    } else {
        let input = argument(fd, call, effect.root)?;
        if fd.vbank().get(input)?.is_constant() {
            return Some(false);
        }
        let address = crate::kuna_stackranges::frame_address(fd, input)?;
        (address.first, address.last)
    };
    let mask = kuna_base::address::calc_mask(space.get_addr_size() as i32);
    let at = first.checked_add(effect.offset)? as u64 & mask;
    let width = last
        .checked_sub(first)?
        .checked_add(i64::from(effect.size))? as u64;
    Some((0..size).any(|byte| {
        wanted & (1 << byte) != 0 && start.wrapping_add(byte as u64).wrapping_sub(at) & mask < width
    }))
}

/// Unknown code, aliases, returned pointers and escaped pointers remain observers.
fn for_call<'a>(
    fd: &'a Funcdata,
    call: &FuncCallSpecs,
    start: u64,
    size: i32,
) -> Option<&'a CalleeMemory> {
    if fd.obank().get(call.get_op())?.code() != OpCode::CPUI_CALL
        || call.proto().has_effect_override()
    {
        return None;
    }
    let entry = call.get_entry_address();
    let summary = fd
        .callee_memory
        .get(&(entry.get_space()?.get_index(), entry.get_offset()))?;
    if !summary.complete || call.get_spacebase_offset() == OFFSET_UNKNOWN {
        return None;
    }
    let space = fd.get_scope_local()?.get_space_id();
    let mut floor = i64::from(crate::kuna_calleeprotostack::declared_caller_frame_floor(
        true,
        call.proto(),
    )?);
    for slot in 0..call.proto().num_params() {
        let param = call.proto().get_param(slot)?;
        let address = param.get_address();
        if address.get_space().map(|s| s.get_index()) == Some(space.get_index()) {
            let at = signed(address.get_offset(), space.get_addr_size())?;
            if at < 0 || param.get_size() <= 0 {
                return None;
            }
            floor = floor.max(at.checked_add(i64::from(param.get_size()))?);
        }
    }
    let floor = i32::try_from(floor).ok()?;
    if !crate::kuna_calleeprotostack::above_callee_frame(
        space.wrap_offset(start.wrapping_sub(call.get_spacebase_offset())),
        size,
        space.get_addr_size() as i32,
        floor,
    ) {
        return None;
    }
    for &(home, effect) in &summary.home_write_conditions {
        if !summary.deferred_frame_escapes.contains(&home) {
            return None;
        }
        let (home_start, home_size) = crate::p3_dataflow::kuna_calleehomes::callee_frame_cell(
            fd,
            call,
            home.offset,
            home.size,
        )?;
        if !(1..=16).contains(&home_size) {
            return None;
        }
        let wanted = (1u32 << home_size) - 1;
        if reaches(fd, call, effect, home_start, home_size, wanted)? {
            return None;
        }
    }
    Some(summary)
}

#[cfg(test)]
pub(crate) fn test_call_summary_is_usable(
    fd: &Funcdata,
    id: crate::context::OpId,
    start: u64,
    size: i32,
) -> bool {
    let Some(index) = fd.get_call_specs_index(id) else {
        return false;
    };
    for_call(fd, fd.get_call_specs(index), start, size).is_some()
}

/// Only complete body evidence can narrow the frame's call-write guard.
pub(crate) fn preserves(fd: &Funcdata, call: &FuncCallSpecs, address: &Address, size: i32) -> bool {
    if !fd.get_arch().stack_views
        || !(1..=16).contains(&size)
        || address.get_space().map(|s| s.get_index())
            != fd.get_scope_local().map(|s| s.get_space_id().get_index())
    {
        return false;
    }
    let Some(summary) = for_call(fd, call, address.get_offset(), size) else {
        return false;
    };
    let wanted = (1 << size) - 1;
    summary
        .writes
        .iter()
        .all(|&effect| reaches(fd, call, effect, address.get_offset(), size, wanted) == Some(false))
}

/// Unknown code, aliases, returned pointers and escaped pointers remain observers.
pub(crate) fn unobserved(
    fd: &Funcdata,
    id: OpId,
    start: u64,
    size: i32,
    wanted: u32,
) -> Option<u32> {
    unobserved_inner(fd, id, start, size, wanted, true)
}

/// Prove that a call does not observe a frame range without discharging any
/// deferred callee-home pointer stores. This bounds the home proof to one
/// callsite and prevents recursive summary walks.
pub(crate) fn unobserved_strict(
    fd: &Funcdata,
    id: OpId,
    start: u64,
    size: i32,
    wanted: u32,
) -> Option<u32> {
    unobserved_inner(fd, id, start, size, wanted, false)
}

fn home_discharge_returns_pointer(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    summary: &CalleeMemory,
    root: Origin,
) -> Option<bool> {
    let output = call.proto().get_output();
    if output.get_size() <= 0 {
        return Some(false);
    }
    let address = output.get_address();
    let output_space = Rc::clone(address.get_space()?);
    let output_key = Storage {
        space: output_space.get_index(),
        offset: address.get_offset(),
        size: u32::try_from(output.get_size()).ok()?,
    };
    let output_value = VarnodeData {
        space: Some(output_space),
        offset: output_key.offset,
        size: output_key.size,
    };
    let manager = fd.get_arch().manage();
    let register = manager.get_space_by_name("register")?.get_index();
    let stack_space = manager.get_stack_space()?;
    let stack_base = stack_space.get_spacebase(0).ok()?;
    let stack = Storage {
        space: stack_base.space?.get_index(),
        offset: stack_base.offset,
        size: stack_base.size,
    };
    let root_storage = match root {
        Origin::Register(storage) => Some(storage),
        Origin::Frame => Some(stack),
        Origin::StackArgument(..) => None,
    };
    let may_alias_root = root_storage.is_some_and(|storage| storage.overlaps(output_key));
    for state in &summary.returns {
        match state.read(&output_value, register, stack) {
            Value::Pointer(..) => return Some(true),
            Value::Unknown if may_alias_root => return Some(true),
            _ => (),
        }
    }
    Some(false)
}

fn unobserved_inner(
    fd: &Funcdata,
    id: OpId,
    start: u64,
    size: i32,
    wanted: u32,
    allow_home_discharge: bool,
) -> Option<u32> {
    let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
    let summary = for_call(fd, call, start, size)?;
    if summary.lossy_pointer_overflow || summary.lossy_pointer_roots.contains(&Origin::Frame) {
        return None;
    }
    for &root in &summary.escapes {
        let input = argument(fd, call, root)?;
        if !fd.vbank().get(input)?.is_constant() {
            return None;
        }
    }
    if !allow_home_discharge || !summary.deferred_frame_escapes.is_empty() {
        let pointer_size =
            u32::try_from(fd.get_scope_local()?.get_space_id().get_addr_size()).ok()?;
        for &root in &summary.lossy_pointer_roots {
            if root == Origin::Frame {
                return None;
            }
            let input = argument(fd, call, root)?;
            if !fd.vbank().get(input)?.is_constant() {
                return None;
            }
        }
        for &range in &summary.lossy_pointer_ranges {
            let input = register_range_argument(fd, call, range)?;
            if !fd.vbank().get(input)?.is_constant() {
                return None;
            }
        }
        if summary
            .reads
            .iter()
            .any(|effect| effect.root == Origin::Frame && effect.size != pointer_size)
        {
            return None;
        }
    }
    let mut caller_home_scan_cache = BTreeMap::new();
    for deferred in &summary.deferred_frame_escapes {
        let pointer_size =
            u32::try_from(fd.get_scope_local()?.get_space_id().get_addr_size()).ok()?;
        let constant_actual = argument(fd, call, deferred.root)
            .and_then(|input| fd.vbank().get(input))
            .is_some_and(|input| input.is_constant());
        if constant_actual {
            continue;
        }
        if deferred.size != pointer_size {
            return None;
        }
        if !allow_home_discharge {
            return None;
        }
        let (home_start, home_size) = crate::p3_dataflow::kuna_calleehomes::caller_home_cell(
            fd,
            call,
            deferred.offset,
            deferred.size,
        )?;
        if home_discharge_returns_pointer(fd, call, summary, deferred.root)? {
            return None;
        }

        // The current callee may reload its own shadow-space spill. Other
        // stack-root reads are internal too; argument-root reads can alias the
        // same caller bytes and therefore remain observers.
        for &effect in &summary.reads {
            if effect.root == Origin::Frame {
                let home_end = deferred.offset.checked_add(i64::from(deferred.size))?;
                let read_end = effect.offset.checked_add(i64::from(effect.size))?;
                if effect.offset < home_end
                    && deferred.offset < read_end
                    && (effect.offset != deferred.offset || effect.size != deferred.size)
                {
                    return None;
                }
                continue;
            }
            if reaches(
                fd,
                call,
                effect,
                home_start,
                home_size,
                (1 << home_size) - 1,
            )? {
                return None;
            }
        }
        let home_is_unobserved = *caller_home_scan_cache
            .entry((home_start, home_size))
            .or_insert_with(|| {
                crate::p3_dataflow::kuna_calleehomes::caller_home_unobserved(
                    fd, id, home_start, home_size,
                )
            });
        if !home_is_unobserved {
            return None;
        }
    }
    let output = call.proto().get_output();
    if output.get_size() > 0 {
        let address = output.get_address();
        let key = Storage {
            space: address.get_space()?.get_index(),
            offset: address.get_offset(),
            size: output.get_size() as u32,
        };
        for state in &summary.returns {
            for (&storage, &value) in &state.values {
                if storage.overlaps(key) {
                    if matches!(value, Value::Pointer(..)) {
                        return None;
                    }
                }
            }
        }
    }
    for &effect in &summary.reads {
        if reaches(fd, call, effect, start, size, wanted)? {
            return None;
        }
    }
    Some(wanted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{type_metatype, Datatype};
    use crate::fspec::ParameterPieces;
    use crate::p3_dataflow::kuna_calleehomes::tests::{
        call_with_r8_argument, locked_home_call_on, Fx, Out,
    };
    use kuna_base::space::{addrspace_flags, AddrSpace};

    struct Program {
        code: Rc<AddrSpace>,
        constant: Rc<AddrSpace>,
        register: Rc<AddrSpace>,
        unique: Rc<AddrSpace>,
    }

    impl Program {
        fn new() -> Self {
            let space = |kind, name, index| {
                Rc::new(AddrSpace::new(
                    kind,
                    name,
                    false,
                    8,
                    1,
                    index,
                    addrspace_flags::hasphysical,
                    1,
                    1,
                ))
            };
            Self {
                code: space(spacetype::IPTR_PROCESSOR, "ram", 1),
                constant: space(spacetype::IPTR_CONSTANT, "const", 0),
                register: space(spacetype::IPTR_PROCESSOR, "register", 2),
                unique: space(spacetype::IPTR_INTERNAL, "unique", 3),
            }
        }

        fn value(&self, space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
            VarnodeData {
                space: Some(Rc::clone(space)),
                offset,
                size,
            }
        }

        fn reg(&self, offset: u64) -> VarnodeData {
            self.value(&self.register, offset, 8)
        }
        fn c(&self, value: u64) -> VarnodeData {
            self.value(&self.constant, value, 8)
        }
        fn op(&self, code: OpCode, out: Option<VarnodeData>, inputs: Vec<VarnodeData>) -> RawOp {
            RawOp { code, out, inputs }
        }
        fn ret(&self) -> RawOp {
            self.op(OpCode::CPUI_RETURN, None, vec![self.reg(0x40)])
        }
        fn load(&self, offset: u64) -> Vec<RawOp> {
            let pointer = self.value(&self.unique, 0, 8);
            vec![
                self.op(
                    OpCode::CPUI_INT_ADD,
                    Some(pointer.clone()),
                    vec![self.reg(0x20), self.c(offset)],
                ),
                self.op(
                    OpCode::CPUI_LOAD,
                    Some(self.value(&self.register, 0, 4)),
                    vec![self.c(1), pointer],
                ),
            ]
        }

        fn run(&self, instructions: Vec<Vec<RawOp>>) -> Option<CalleeMemory> {
            walk(
                |at| Some((1, instructions.get(at.get_offset() as usize)?.clone())),
                &Address::new(Rc::clone(&self.code), 0),
                Storage {
                    space: 2,
                    offset: 0x30,
                    size: 8,
                },
                2,
                1,
                Some(Storage {
                    space: 2,
                    offset: 0x40,
                    size: 8,
                }),
            )
        }
    }

    #[test]
    fn a_leaf_reports_the_actual_read_and_write_bytes() {
        let p = Program::new();
        let mut ops = p.load(4);
        ops.push(p.op(
            OpCode::CPUI_STORE,
            None,
            vec![p.c(1), p.reg(0x20), p.value(&p.constant, 9, 4)],
        ));
        ops.push(p.ret());
        let summary = p.run(vec![ops]).unwrap();
        let root = Origin::Register(Storage {
            space: 2,
            offset: 0x20,
            size: 8,
        });
        assert_eq!(
            summary.reads,
            BTreeSet::from([Effect {
                root,
                offset: 4,
                size: 4
            }])
        );
        assert_eq!(
            summary.writes,
            BTreeSet::from([Effect {
                root,
                offset: 0,
                size: 4
            }])
        );
        assert!(summary.escapes.is_empty());
    }

    #[test]
    fn branch_and_loop_reads_are_unioned_and_missing_paths_decline() {
        let p = Program::new();
        let branch = p.op(
            OpCode::CPUI_CBRANCH,
            None,
            vec![p.value(&p.code, 2, 8), p.value(&p.register, 0x48, 1)],
        );
        let mut first = p.load(0);
        first.push(p.ret());
        let mut second = p.load(4);
        second.push(p.op(
            OpCode::CPUI_CBRANCH,
            None,
            vec![p.value(&p.code, 2, 8), p.value(&p.register, 0x48, 1)],
        ));
        let summary = p
            .run(vec![
                vec![branch.clone()],
                first.clone(),
                second.clone(),
                vec![p.ret()],
            ])
            .unwrap();
        assert_eq!(summary.reads.len(), 2);
        assert!(p.run(vec![vec![branch], first]).is_none());
        assert!(p
            .run(vec![vec![p.op(
                OpCode::CPUI_BRANCH,
                None,
                vec![p.value(&p.code, 0, 8)]
            )]])
            .is_none());
    }

    #[test]
    fn pointer_stores_and_returns_retain_their_entry_identity() {
        let p = Program::new();
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x2000), p.reg(0x20)],
                ),
                p.op(OpCode::CPUI_COPY, Some(p.reg(0)), vec![p.reg(0x20)]),
                p.ret(),
            ]])
            .unwrap();
        let root = Origin::Register(Storage {
            space: 2,
            offset: 0x20,
            size: 8,
        });
        assert!(summary.escapes.contains(&root));
        assert_eq!(
            summary.returns[0].values.get(&Storage {
                space: 2,
                offset: 0,
                size: 8
            }),
            Some(&Value::Pointer(root, 0))
        );
    }

    #[test]
    fn positive_frame_pointer_stores_remain_deferred_after_overwrites() {
        let p = Program::new();
        let store_pointer = |p: &Program| {
            let slot = p.value(&p.unique, 0, 8);
            vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(slot.clone()),
                    vec![p.reg(0x30), p.c(16)],
                ),
                p.op(OpCode::CPUI_STORE, None, vec![p.c(1), slot, p.reg(0x20)]),
            ]
        };
        let root = Origin::Register(Storage {
            space: 2,
            offset: 0x20,
            size: 8,
        });
        let deferred = DeferredFrameEscape {
            offset: 16,
            size: 8,
            root,
        };
        let effect = Effect {
            root: Origin::Frame,
            offset: 16,
            size: 8,
        };

        let mut residual = store_pointer(&p);
        residual.push(p.ret());
        let summary = p.run(vec![residual]).unwrap();
        assert!(summary.deferred_frame_escapes.contains(&deferred));
        assert!(summary.escapes.is_empty());
        assert!(summary.writes.contains(&effect));

        for (offset, size) in [(16, 8), (20, 4)] {
            let slot = p.value(&p.unique, 8, 8);
            let mut ops = store_pointer(&p);
            ops.extend([
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(slot.clone()),
                    vec![p.reg(0x30), p.c(offset)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), slot, p.value(&p.constant, 0x1234, size)],
                ),
                p.ret(),
            ]);
            let summary = p.run(vec![ops]).unwrap();
            assert!(summary.deferred_frame_escapes.contains(&deferred));
            assert!(summary.writes.contains(&effect));
        }
    }

    #[test]
    fn exact_home_pointer_spills_survive_alias_guarded_writes_and_partial_overwrites() {
        let p = Program::new();
        let body = |p: &Program, home_offset: u64| {
            let home = p.value(&p.unique, 0, 8);
            let destination = p.value(&p.unique, 8, 8);
            let reloaded = p.reg(0x18);
            let reloaded_field = p.value(&p.unique, 16, 8);
            let partial = p.value(&p.unique, 24, 8);
            vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(home.clone()),
                    vec![p.reg(0x30), p.c(home_offset)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), home.clone(), p.reg(0x20)],
                ),
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(destination.clone()),
                    vec![p.reg(0x20), p.c(4)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), destination, p.value(&p.constant, 0x1234, 4)],
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(reloaded.clone()),
                    vec![p.c(1), home.clone()],
                ),
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(reloaded_field.clone()),
                    vec![reloaded, p.c(4)],
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(p.value(&p.register, 0, 4)),
                    vec![p.c(1), reloaded_field],
                ),
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(partial.clone()),
                    vec![p.reg(0x30), p.c(home_offset + 4)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), partial, p.value(&p.constant, 0, 4)],
                ),
                p.ret(),
            ]
        };

        let branch = p.op(
            OpCode::CPUI_CBRANCH,
            None,
            vec![p.value(&p.code, 2, 8), p.value(&p.register, 0x48, 1)],
        );
        let summary = p
            .run(vec![vec![branch], body(&p, 16), vec![p.ret()]])
            .unwrap();
        let root = Origin::Register(Storage {
            space: 2,
            offset: 0x20,
            size: 8,
        });
        let home = DeferredFrameEscape {
            offset: 16,
            size: 8,
            root,
        };
        let through_destination = Effect {
            root,
            offset: 4,
            size: 4,
        };
        assert!(summary.deferred_frame_escapes.contains(&home));
        assert!(summary
            .home_write_conditions
            .contains(&(home, through_destination)));
        assert!(summary.writes.contains(&Effect {
            root: Origin::Frame,
            offset: 20,
            size: 4,
        }));
        assert!(summary.reads.contains(&through_destination));
        assert!(summary
            .returns
            .iter()
            .any(|state| state.arguments_clobbered));

        assert!(p.run(vec![body(&p, 40)]).is_none());
    }

    #[test]
    fn unknown_full_width_positive_stores_still_decline_the_summary() {
        let p = Program::new();
        let slot = p.value(&p.unique, 0, 8);
        assert!(p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(slot.clone()),
                    vec![p.reg(0x30), p.c(16)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), slot, p.value(&p.unique, 0x80, 8)],
                ),
                p.ret(),
            ]])
            .is_none());
    }

    #[test]
    fn opaque_memory_and_control_flow_never_prove_a_call_safe() {
        let p = Program::new();
        let indirect = p.op(OpCode::CPUI_LOAD, Some(p.reg(0)), vec![p.c(1), p.c(0x2000)]);
        let dereference = p.op(
            OpCode::CPUI_LOAD,
            Some(p.value(&p.register, 0, 4)),
            vec![p.c(1), p.reg(0)],
        );
        assert!(p.run(vec![vec![indirect, dereference, p.ret()]]).is_none());
        for code in [
            OpCode::CPUI_CALL,
            OpCode::CPUI_CALLIND,
            OpCode::CPUI_CALLOTHER,
            OpCode::CPUI_BRANCHIND,
        ] {
            assert!(p
                .run(vec![vec![p.op(code, None, vec![p.reg(0x20)]), p.ret()]])
                .is_none());
        }
        assert!(p
            .run(vec![vec![
                p.op(OpCode::CPUI_CBRANCH, None, vec![p.c(1)]),
                p.ret()
            ]])
            .is_none());
    }

    #[test]
    fn a_rewritten_return_address_does_not_end_a_complete_body() {
        let p = Program::new();
        let return_load = p.op(
            OpCode::CPUI_LOAD,
            Some(p.reg(0x40)),
            vec![p.c(1), p.reg(0x30)],
        );
        assert!(p.run(vec![vec![p.ret()]]).is_none());
        assert!(p
            .run(vec![vec![
                p.op(OpCode::CPUI_COPY, Some(p.reg(0)), vec![p.c(1)]),
                return_load.clone(),
                p.ret(),
            ]])
            .is_some());
        for (offset, value) in [(0, p.reg(0x20)), (1, p.value(&p.constant, 1, 1))] {
            let slot = p.value(&p.unique, 0, 8);
            assert!(p
                .run(vec![vec![
                    p.op(
                        OpCode::CPUI_INT_ADD,
                        Some(slot.clone()),
                        vec![p.reg(0x30), p.c(offset)]
                    ),
                    p.op(OpCode::CPUI_STORE, None, vec![p.c(1), slot, value]),
                    return_load.clone(),
                    p.ret(),
                ]])
                .is_none());
        }
        assert!(p
            .run(vec![vec![
                p.op(OpCode::CPUI_COPY, Some(p.reg(0x40)), vec![p.reg(0x20)]),
                p.ret()
            ]])
            .is_none());
    }

    #[test]
    fn local_spills_preserve_pointers_and_partial_register_writes_forget_them() {
        let p = Program::new();
        let slot = p.value(&p.unique, 0, 8);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(slot.clone()),
                    vec![p.reg(0x30), p.c((-16i64) as u64)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), slot.clone(), p.reg(0x20)],
                ),
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), slot]),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(p.value(&p.register, 0, 4)),
                    vec![p.c(1), p.reg(0x10)],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert_eq!(summary.reads.len(), 1);
        assert!(summary.escapes.is_empty());
        assert!(p
            .run(vec![vec![
                p.op(OpCode::CPUI_COPY, Some(p.reg(0)), vec![p.reg(0x20)]),
                p.op(
                    OpCode::CPUI_COPY,
                    Some(p.value(&p.register, 0, 4)),
                    vec![p.value(&p.constant, 1, 4)]
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(p.value(&p.register, 0x10, 4)),
                    vec![p.c(1), p.reg(0)]
                ),
                p.ret(),
            ]])
            .is_none());
    }

    #[test]
    fn incoming_stack_pointers_keep_their_slot_identity_through_spills() {
        let p = Program::new();
        let incoming = p.value(&p.unique, 0, 8);
        let spill = p.value(&p.unique, 8, 8);
        let field = p.value(&p.unique, 16, 8);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(incoming.clone()),
                    vec![p.reg(0x30), p.c(8)],
                ),
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), incoming]),
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(spill.clone()),
                    vec![p.reg(0x30), p.c((-16i64) as u64)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), spill.clone(), p.reg(0x10)],
                ),
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x18)), vec![p.c(1), spill]),
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(field.clone()),
                    vec![p.reg(0x18), p.c(4)],
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(p.value(&p.register, 0, 4)),
                    vec![p.c(1), field],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert_eq!(
            summary.reads,
            BTreeSet::from([
                Effect {
                    root: Origin::Frame,
                    offset: 8,
                    size: 8
                },
                Effect {
                    root: Origin::StackArgument(8, 8),
                    offset: 4,
                    size: 4
                },
            ])
        );
        assert!(summary.escapes.is_empty());
    }

    #[test]
    fn direct_stack_accesses_outside_arguments_are_visible_effects() {
        let p = Program::new();
        let slot = p.value(&p.unique, 0, 8);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(slot.clone()),
                    vec![p.reg(0x30), p.c(24)],
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(p.value(&p.register, 0, 4)),
                    vec![p.c(1), slot.clone()],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), slot, p.value(&p.constant, 9, 4)],
                ),
                p.ret(),
            ]])
            .unwrap();
        let effect = Effect {
            root: Origin::Frame,
            offset: 24,
            size: 4,
        };
        assert!(summary.reads.contains(&effect));
        assert!(summary.writes.contains(&effect));
    }

    #[test]
    fn partial_and_aliased_argument_writes_cannot_restore_an_incoming_pointer() {
        let p = Program::new();
        let slot = p.value(&p.unique, 0, 8);
        for write in [
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), slot.clone(), p.value(&p.constant, 1, 1)],
            ),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), p.reg(0x20), p.value(&p.constant, 1, 4)],
            ),
        ] {
            assert!(p
                .run(vec![vec![
                    p.op(
                        OpCode::CPUI_INT_ADD,
                        Some(slot.clone()),
                        vec![p.reg(0x30), p.c(8)]
                    ),
                    write,
                    p.op(
                        OpCode::CPUI_LOAD,
                        Some(p.reg(0)),
                        vec![p.c(1), slot.clone()]
                    ),
                    p.op(
                        OpCode::CPUI_LOAD,
                        Some(p.value(&p.register, 0, 4)),
                        vec![p.c(1), p.reg(0)]
                    ),
                    p.ret(),
                ]])
                .is_none());
        }
    }

    fn make_call_site_with_stack_base(
        fx: &mut Fx,
        actual: crate::context::VarnodeId,
        stack_base: u64,
    ) -> OpId {
        let stack_ref = fx.fd.new_varnode(
            8,
            &Address::new(Rc::clone(&fx.stack), (-56i64) as u64),
            None,
        );
        let (_, placeholder) = fx.emit(OpCode::CPUI_COPY, &[stack_ref], Some(Out::Unique(8)));
        let placeholder = placeholder.unwrap();
        let target = fx.constant(8, 0x2000);
        let call_op = fx
            .emit(OpCode::CPUI_CALL, &[target, actual, placeholder], None)
            .0;
        let mut call = locked_home_call_on(fx, call_op, stack_base, false);
        let value_type = Rc::new(Datatype::new(8, type_metatype::TYPE_PTR));
        call.proto_mut().set_param(
            0,
            "rect",
            &ParameterPieces {
                addr: Address::new(Rc::clone(&fx.reg), 0x20),
                type_: Some(Rc::clone(&value_type)),
                flags: 0,
            },
        );
        call.proto_mut().set_param(
            1,
            "stack_arg",
            &ParameterPieces {
                addr: Address::new(Rc::clone(&fx.stack), stack_base),
                type_: Some(value_type),
                flags: 0,
            },
        );
        call.proto_mut().resolve_extra_pop();
        call.resolve_spacebase_relative(&mut fx.fd, placeholder)
            .unwrap();
        fx.fd.push_call_specs(call);
        call_op
    }

    fn make_call_site(fx: &mut Fx, actual: crate::context::VarnodeId) -> OpId {
        make_call_site_with_stack_base(fx, actual, 40)
    }

    fn install_call_summary_with_stack_base(
        fx: &mut Fx,
        actual: crate::context::VarnodeId,
        stack_base: u64,
    ) -> OpId {
        let call_op = make_call_site_with_stack_base(fx, actual, stack_base);
        let root = Origin::Register(Storage {
            space: fx.reg.get_index(),
            offset: 0x20,
            size: 8,
        });
        let home = DeferredFrameEscape {
            offset: 16,
            size: 8,
            root,
        };
        let effect = Effect {
            root,
            offset: 0,
            size: 8,
        };
        let mut summary = CalleeMemory {
            complete: true,
            ..Default::default()
        };
        summary.deferred_frame_escapes.insert(home);
        summary.home_write_conditions.insert((home, effect));
        summary.writes.insert(effect);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
        call_op
    }

    fn install_call_summary(fx: &mut Fx, actual: crate::context::VarnodeId) -> OpId {
        install_call_summary_with_stack_base(fx, actual, 40)
    }

    #[test]
    fn home_write_conditions_are_checked_against_each_call_argument_before_use() {
        let query = (-32i64) as u64;

        let mut fx = Fx::new();
        let disjoint_actual = fx.frame_pointer(-64);
        let call = install_call_summary(&mut fx, disjoint_actual);
        assert!(test_call_summary_is_usable(&fx.fd, call, query, 8));

        let mut fx = Fx::new();
        let overlapping_actual = fx.frame_pointer(-80);
        let call = install_call_summary(&mut fx, overlapping_actual);
        assert!(!test_call_summary_is_usable(&fx.fd, call, query, 8));

        let mut fx = Fx::new();
        let unresolved_actual = fx
            .fd
            .new_varnode(8, &Address::new(Rc::clone(&fx.reg), 0x88), None);
        let call = install_call_summary(&mut fx, unresolved_actual);
        assert!(!test_call_summary_is_usable(&fx.fd, call, query, 8));
    }

    #[test]
    fn non_win64_frame_spill_conditions_use_physical_aliasing_without_discharge() {
        let query = (-32i64) as u64;

        let mut fx = Fx::new();
        let disjoint_actual = fx.frame_pointer(-64);
        let call = install_call_summary_with_stack_base(&mut fx, disjoint_actual, 8);
        let index = fx.fd.get_call_specs_index(call).unwrap();
        let call_specs = fx.fd.get_call_specs(index);
        assert!(
            crate::p3_dataflow::kuna_calleehomes::callee_frame_cell(&fx.fd, call_specs, 16, 8,)
                .is_some_and(|(start, size)| start == ((-48i64) as u64) && size == 8)
        );
        assert!(
            crate::p3_dataflow::kuna_calleehomes::caller_home_cell(&fx.fd, call_specs, 16, 8,)
                .is_none()
        );
        assert!(crate::p3_dataflow::kuna_calleehomes::callee_frame_cell(
            &fx.fd,
            call_specs,
            i64::MIN,
            8,
        )
        .is_none());
        assert!(crate::p3_dataflow::kuna_calleehomes::callee_frame_cell(
            &fx.fd,
            call_specs,
            16,
            u32::MAX,
        )
        .is_none());
        assert!(test_call_summary_is_usable(&fx.fd, call, query, 8));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);

        let mut fx = Fx::new();
        let overlapping_actual = fx.frame_pointer(-48);
        let call = install_call_summary_with_stack_base(&mut fx, overlapping_actual, 8);
        assert!(!test_call_summary_is_usable(&fx.fd, call, query, 8));

        let mut fx = Fx::new();
        let unresolved_actual = fx
            .fd
            .new_varnode(8, &Address::new(Rc::clone(&fx.reg), 0x88), None);
        let call = install_call_summary_with_stack_base(&mut fx, unresolved_actual, 8);
        assert!(!test_call_summary_is_usable(&fx.fd, call, query, 8));
    }

    fn set_call_output_register(fx: &mut Fx, call: OpId, offset: u64) {
        let index = fx.fd.get_call_specs_index(call).unwrap();
        let output_type = Rc::new(Datatype::new(8, type_metatype::TYPE_PTR));
        fx.fd
            .get_call_specs_mut(index)
            .proto_mut()
            .set_output(&ParameterPieces {
                addr: Address::new(Rc::clone(&fx.reg), offset),
                type_: Some(output_type),
                flags: 0,
            });
    }

    fn set_call_return_states(fx: &mut Fx, returns: Vec<State>) {
        let key = (fx.reg.get_index(), 0x2000);
        let mut summary = fx.fd.callee_memory.get(&key).unwrap().as_ref().clone();
        summary.returns = returns;
        fx.fd.callee_memory.insert(key, Rc::new(summary));
    }

    #[test]
    fn unchanged_custom_output_aliasing_home_root_blocks_but_constant_output_is_safe() {
        let query = (-32i64) as u64;

        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = install_call_summary(&mut fx, actual);
        set_call_output_register(&mut fx, call, 0x20);
        set_call_return_states(&mut fx, vec![State::default()]);
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);

        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = install_call_summary(&mut fx, actual);
        set_call_output_register(&mut fx, call, 0x88);
        set_call_return_states(&mut fx, vec![State::default()]);
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);

        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = install_call_summary(&mut fx, actual);
        set_call_output_register(&mut fx, call, 0x20);
        let mut returned = State::default();
        returned
            .write(
                &VarnodeData {
                    space: Some(Rc::clone(&fx.reg)),
                    offset: 0x20,
                    size: 8,
                },
                Value::Constant(7),
            )
            .unwrap();
        set_call_return_states(&mut fx, vec![returned]);
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), Some(0xff));
    }

    fn home_reload_summary(p: &Program, width: u32) -> CalleeMemory {
        let home = p.value(&p.unique, 0, 8);
        let mut ops = vec![
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(home.clone()),
                vec![p.reg(0x30), p.c(16)],
            ),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), home.clone(), p.reg(0x20)],
            ),
            p.op(
                OpCode::CPUI_LOAD,
                Some(p.value(&p.register, 0x10, width)),
                vec![p.c(1), home],
            ),
        ];
        if width == 8 {
            ops.push(p.op(OpCode::CPUI_COPY, Some(p.reg(0x18)), vec![p.reg(0x10)]));
        }
        ops.push(p.ret());
        p.run(vec![ops]).unwrap()
    }

    #[test]
    fn only_exact_pointer_width_home_reloads_can_be_discharged() {
        let query = (-32i64) as u64;

        let p = Program::new();
        let exact = home_reload_summary(&p, 8);
        assert!(exact.lossy_pointer_roots.is_empty());
        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = make_call_site(&mut fx, actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(exact));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), Some(0xff));

        for width in [4, 16] {
            let p = Program::new();
            let summary = home_reload_summary(&p, width);
            assert!(summary
                .deferred_frame_escapes
                .iter()
                .any(|home| home.offset == 16));
            let mut fx = Fx::new();
            let actual = fx.frame_pointer(-64);
            let call = make_call_site(&mut fx, actual);
            fx.fd
                .callee_memory
                .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
            assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
        }
    }

    fn home_with_other_register_loss(p: &Program) -> CalleeMemory {
        let home = p.value(&p.unique, 0, 8);
        p.run(vec![vec![
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(home.clone()),
                vec![p.reg(0x30), p.c(16)],
            ),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), home.clone(), p.reg(0x20)],
            ),
            p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), home]),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), p.c(0x8000), p.value(&p.register, 0x28, 4)],
            ),
            p.ret(),
        ]])
        .unwrap()
    }

    #[test]
    fn lossy_r8_argument_cannot_expose_a_discharged_rcx_home() {
        let query = (-32i64) as u64;
        let p = Program::new();
        let summary = home_with_other_register_loss(&p);
        assert!(summary.complete);
        assert!(summary.deferred_frame_escapes.iter().any(|home| {
            home.root
                == Origin::Register(Storage {
                    space: 2,
                    offset: 0x20,
                    size: 8,
                })
        }));
        assert!(summary.lossy_pointer_ranges.contains(&Storage {
            space: 2,
            offset: 0x28,
            size: 4,
        }));

        let mut fx = Fx::new();
        let rcx_actual = fx.frame_pointer(-64);
        let r8_actual = fx.frame_pointer(-40);
        let call = call_with_r8_argument(&mut fx, rcx_actual, 40, r8_actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary.clone()));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);

        let mut fx = Fx::new();
        let rcx_actual = fx.frame_pointer(-64);
        let r8_actual = fx.constant(8, 0x8000);
        let call = call_with_r8_argument(&mut fx, rcx_actual, 40, r8_actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), Some(0xff));
    }

    #[test]
    fn partial_known_register_overwrite_keeps_the_pointer_root_sticky() {
        let query = (-32i64) as u64;
        let p = Program::new();
        let home = p.value(&p.unique, 0, 8);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(home.clone()),
                    vec![p.reg(0x30), p.c(16)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), home.clone(), p.reg(0x20)],
                ),
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), home]),
                p.op(OpCode::CPUI_COPY, Some(p.reg(0x28)), vec![p.reg(0x20)]),
                p.op(
                    OpCode::CPUI_COPY,
                    Some(p.value(&p.register, 0x28, 4)),
                    vec![p.value(&p.constant, 7, 4)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x8000), p.value(&p.register, 0x2c, 4)],
                ),
                p.ret(),
            ]])
            .unwrap();
        let root = Origin::Register(Storage {
            space: 2,
            offset: 0x20,
            size: 8,
        });
        assert!(summary.complete);
        assert!(summary.lossy_pointer_roots.contains(&root));

        let mut fx = Fx::new();
        let rcx_actual = fx.frame_pointer(-64);
        let r8_actual = fx.frame_pointer(-40);
        let call = call_with_r8_argument(&mut fx, rcx_actual, 40, r8_actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
    }

    #[test]
    fn strict_observer_checks_keep_fragment_barriers_without_homes() {
        let query = (-32i64) as u64;
        let p = Program::new();
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x8000), p.value(&p.register, 0x28, 4)],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert!(summary.complete);
        assert!(summary.deferred_frame_escapes.is_empty());
        for constant_actual in [false, true] {
            let mut fx = Fx::new();
            let rcx_actual = fx.frame_pointer(-64);
            let r8_actual = if constant_actual {
                fx.constant(8, 0x8000)
            } else {
                fx.frame_pointer(-40)
            };
            let call = call_with_r8_argument(&mut fx, rcx_actual, 40, r8_actual);
            fx.fd
                .callee_memory
                .insert((fx.reg.get_index(), 0x2000), Rc::new(summary.clone()));
            assert_eq!(
                unobserved_strict(&fx.fd, call, query, 8, 0xff),
                constant_actual.then_some(0xff)
            );
        }

        let pointer = p.value(&p.unique, 0, 8);
        let fragment = p.value(&p.unique, 8, 4);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(pointer.clone()),
                    vec![p.reg(0x30), p.c(40)],
                ),
                p.op(
                    OpCode::CPUI_LOAD,
                    Some(fragment.clone()),
                    vec![p.c(1), pointer],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x8000), fragment],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert!(summary.complete);
        assert!(summary.deferred_frame_escapes.is_empty());
        assert!(summary.lossy_pointer_roots.is_empty());
        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = make_call_site(&mut fx, actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
        assert_eq!(unobserved_strict(&fx.fd, call, query, 8, 0xff), None);
    }

    fn home_with_partial_stack_argument_read(p: &Program, width: u32) -> CalleeMemory {
        let home = p.value(&p.unique, 0, 8);
        let stack_arg = p.value(&p.unique, 8, 8);
        let loaded = p.value(&p.unique, 16, width);
        p.run(vec![vec![
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(home.clone()),
                vec![p.reg(0x30), p.c(16)],
            ),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), home.clone(), p.reg(0x20)],
            ),
            p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), home]),
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(stack_arg.clone()),
                vec![p.reg(0x30), p.c(40)],
            ),
            p.op(
                OpCode::CPUI_LOAD,
                Some(loaded.clone()),
                vec![p.c(1), stack_arg],
            ),
            p.op(OpCode::CPUI_STORE, None, vec![p.c(1), p.c(0x8000), loaded]),
            p.ret(),
        ]])
        .unwrap()
    }

    #[test]
    fn partial_and_bulk_stack_argument_reads_block_home_discharge() {
        let query = (-32i64) as u64;
        for width in [4, 16] {
            let p = Program::new();
            let summary = home_with_partial_stack_argument_read(&p, width);
            assert!(summary.complete);
            assert!(summary.lossy_pointer_roots.is_empty());
            assert!(summary.lossy_pointer_ranges.is_empty());
            assert!(summary.reads.contains(&Effect {
                root: Origin::Frame,
                offset: 40,
                size: width,
            }));

            let mut fx = Fx::new();
            let actual = fx.frame_pointer(-64);
            let call = make_call_site(&mut fx, actual);
            fx.fd
                .callee_memory
                .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
            assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
        }
    }

    #[test]
    fn lossy_provenance_overflow_blocks_unobserved_proofs() {
        let query = (-32i64) as u64;
        let mut fx = Fx::new();
        let actual = fx.frame_pointer(-64);
        let call = install_call_summary(&mut fx, actual);
        let key = (fx.reg.get_index(), 0x2000);
        let mut summary = fx.fd.callee_memory.get(&key).unwrap().as_ref().clone();
        summary.lossy_pointer_overflow = true;
        assert!(summary.lossy_pointer_roots.is_empty());
        fx.fd.callee_memory.insert(key, Rc::new(summary));
        assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
    }

    #[test]
    fn initialized_scalar_fragments_do_not_hide_sticky_pointer_losses() {
        let p = Program::new();
        let stack = Storage {
            space: 2,
            offset: 0x30,
            size: 8,
        };
        let low_r8 = p.value(&p.register, 0x28, 4);
        let (roots, ranges) = pointer_sources(&State::default(), &low_r8, 2, stack);
        assert!(roots.is_empty());
        assert!(ranges.contains(&Storage {
            space: 2,
            offset: 0x28,
            size: 4,
        }));

        let mut scalar = State::default();
        scalar.write(&low_r8, Value::Unknown).unwrap();
        let (roots, ranges) = pointer_sources(&scalar, &low_r8, 2, stack);
        assert!(roots.is_empty());
        assert!(ranges.is_empty());

        let mut partial_scalar = State::default();
        partial_scalar
            .write(&p.value(&p.register, 0x28, 2), Value::Unknown)
            .unwrap();
        let (roots, ranges) = pointer_sources(&partial_scalar, &low_r8, 2, stack);
        assert!(roots.is_empty());
        assert!(ranges.contains(&Storage {
            space: 2,
            offset: 0x28,
            size: 4,
        }));

        let (roots, ranges) = pointer_sources(&partial_scalar, &p.reg(0x28), 2, stack);
        assert!(roots.is_empty());
        assert!(ranges.contains(&Storage {
            space: 2,
            offset: 0x28,
            size: 8,
        }));

        let pointer_root = Origin::Register(Storage {
            space: 2,
            offset: 0x28,
            size: 8,
        });
        scalar
            .write(&p.reg(0x28), Value::Pointer(pointer_root, 0))
            .unwrap();
        let (roots, ranges) = pointer_sources(&scalar, &low_r8, 2, stack);
        assert!(roots.contains(&pointer_root));
        assert!(ranges.contains(&Storage {
            space: 2,
            offset: 0x28,
            size: 4,
        }));

        let mut summary = CalleeMemory::default();
        summary.note_lossy_sources(roots, ranges);
        scalar.write(&low_r8, Value::Unknown).unwrap();
        let (roots, ranges) = pointer_sources(&scalar, &low_r8, 2, stack);
        assert!(roots.is_empty());
        assert!(ranges.is_empty());
        assert!(summary.lossy_pointer_roots.contains(&pointer_root));
    }

    fn truncation_publication_summary(p: &Program, after_home: bool) -> CalleeMemory {
        let truncated = p.value(&p.unique, 0, 4);
        let home = p.value(&p.unique, 8, 8);
        let mut ops = Vec::new();
        if !after_home {
            ops.extend([
                p.op(
                    OpCode::CPUI_SUBPIECE,
                    Some(truncated.clone()),
                    vec![p.reg(0x20), p.c(0)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x2000), truncated.clone()],
                ),
            ]);
        }
        ops.extend([
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(home.clone()),
                vec![p.reg(0x30), p.c(16)],
            ),
            p.op(
                OpCode::CPUI_STORE,
                None,
                vec![p.c(1), home.clone(), p.reg(0x20)],
            ),
        ]);
        if after_home {
            ops.extend([
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), home]),
                p.op(
                    OpCode::CPUI_SUBPIECE,
                    Some(truncated.clone()),
                    vec![p.reg(0x10), p.c(0)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x2000), truncated],
                ),
            ]);
        }
        ops.push(p.ret());
        p.run(vec![ops]).unwrap()
    }

    #[test]
    fn narrow_pointer_publication_before_or_after_homing_blocks_discharge() {
        let query = (-32i64) as u64;
        for after_home in [false, true] {
            let p = Program::new();
            let summary = truncation_publication_summary(&p, after_home);
            let root = Origin::Register(Storage {
                space: 2,
                offset: 0x20,
                size: 8,
            });
            assert!(summary
                .deferred_frame_escapes
                .iter()
                .any(|home| home.root == root));
            assert!(summary.lossy_pointer_roots.contains(&root));

            let mut fx = Fx::new();
            let actual = fx.frame_pointer(-64);
            let call = make_call_site(&mut fx, actual);
            fx.fd
                .callee_memory
                .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
            assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
        }
    }

    fn partial_register_copy_summary(p: &Program, after_home: bool) -> CalleeMemory {
        let low_register = p.value(&p.register, 0x20, 4);
        let low_copy = p.value(&p.unique, 0, 4);
        let home = p.value(&p.unique, 8, 8);
        let lossy_copy = p.op(
            OpCode::CPUI_COPY,
            Some(low_copy.clone()),
            vec![low_register],
        );
        let publish = p.op(
            OpCode::CPUI_STORE,
            None,
            vec![p.c(1), p.c(0x2000), low_copy],
        );
        let save = [
            p.op(
                OpCode::CPUI_INT_ADD,
                Some(home.clone()),
                vec![p.reg(0x30), p.c(16)],
            ),
            p.op(OpCode::CPUI_STORE, None, vec![p.c(1), home, p.reg(0x20)]),
        ];
        let mut ops = Vec::new();
        if after_home {
            ops.extend(save);
            ops.extend([lossy_copy, publish]);
        } else {
            ops.extend([lossy_copy, publish]);
            ops.extend(save);
        }
        ops.push(p.ret());
        p.run(vec![ops]).unwrap()
    }

    #[test]
    fn partial_register_copy_before_or_after_homing_blocks_discharge() {
        let query = (-32i64) as u64;
        for after_home in [false, true] {
            let p = Program::new();
            let summary = partial_register_copy_summary(&p, after_home);
            let root = Origin::Register(Storage {
                space: 2,
                offset: 0x20,
                size: 8,
            });
            assert!(summary
                .deferred_frame_escapes
                .iter()
                .any(|home| home.root == root));
            assert!(summary
                .lossy_pointer_ranges
                .iter()
                .any(|range| { range.space == 2 && range.offset == 0x20 && range.size == 4 }));

            let mut fx = Fx::new();
            let actual = fx.frame_pointer(-64);
            let call = make_call_site(&mut fx, actual);
            fx.fd
                .callee_memory
                .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
            assert_eq!(unobserved(&fx.fd, call, query, 8, 0xff), None);
        }
    }

    #[test]
    fn a_partial_stack_register_view_is_tied_to_the_frame_root() {
        let p = Program::new();
        let stack = Storage {
            space: 2,
            offset: 0x30,
            size: 8,
        };
        let input = p.value(&p.register, 0x30, 4);
        let (roots, ranges) = pointer_sources(&State::default(), &input, 2, stack);
        assert!(roots.contains(&Origin::Frame));
        assert!(ranges.contains(&Storage {
            space: 2,
            offset: 0x30,
            size: 4,
        }));
    }

    #[test]
    fn lossy_frame_publication_blocks_an_unrelated_argument_home() {
        for (publish_frame, after_home) in [(false, false), (true, false), (true, true)] {
            let p = Program::new();
            let home = p.value(&p.unique, 0, 8);
            let mut ops = vec![
                p.op(
                    OpCode::CPUI_INT_ADD,
                    Some(home.clone()),
                    vec![p.reg(0x30), p.c(16)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), home.clone(), p.reg(0x20)],
                ),
                p.op(OpCode::CPUI_LOAD, Some(p.reg(0x10)), vec![p.c(1), home]),
            ];
            if publish_frame {
                ops.insert(
                    if after_home { 2 } else { 0 },
                    p.op(
                        OpCode::CPUI_STORE,
                        None,
                        vec![p.c(1), p.c(0x8000), p.value(&p.register, 0x30, 4)],
                    ),
                );
            }
            ops.push(p.ret());
            let summary = p.run(vec![ops]).unwrap();
            assert!(summary.complete);
            assert_eq!(
                summary.lossy_pointer_roots.contains(&Origin::Frame),
                publish_frame,
            );
            assert!(summary
                .deferred_frame_escapes
                .iter()
                .all(|home| home.root != Origin::Frame));

            for constant_actual in [false, true] {
                let mut fx = Fx::new();
                let actual = if constant_actual {
                    fx.constant(8, 0x4000)
                } else {
                    fx.frame_pointer(-64)
                };
                let call = make_call_site(&mut fx, actual);
                if publish_frame {
                    let space = fx.constant(4, 4);
                    let global = fx.constant(8, 0x8000);
                    let (_, low) =
                        fx.emit(OpCode::CPUI_LOAD, &[space, global], Some(Out::Unique(4)));
                    let (_, wide) =
                        fx.emit(OpCode::CPUI_INT_ZEXT, &[low.unwrap()], Some(Out::Unique(8)));
                    let offset = fx.constant(8, 16);
                    let (_, pointer) = fx.emit(
                        OpCode::CPUI_INT_ADD,
                        &[wide.unwrap(), offset],
                        Some(Out::Unique(8)),
                    );
                    fx.emit(
                        OpCode::CPUI_LOAD,
                        &[space, pointer.unwrap()],
                        Some(Out::Unique(8)),
                    );
                }
                fx.fd
                    .callee_memory
                    .insert((fx.reg.get_index(), 0x2000), Rc::new(summary.clone()));
                assert_eq!(
                    unobserved(&fx.fd, call, (-32i64) as u64, 8, 0xff),
                    if publish_frame { None } else { Some(0xff) },
                );
            }
        }
    }

    #[test]
    fn lossy_frame_publication_without_homes_is_still_an_observer() {
        let p = Program::new();
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x8000), p.value(&p.register, 0x30, 4)],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert!(summary.complete);
        assert!(summary.deferred_frame_escapes.is_empty());
        assert!(summary.lossy_pointer_roots.contains(&Origin::Frame));
        let mut fx = Fx::new();
        let actual = fx.constant(8, 0x4000);
        let call = make_call_site(&mut fx, actual);
        fx.fd
            .callee_memory
            .insert((fx.reg.get_index(), 0x2000), Rc::new(summary));
        assert_eq!(unobserved(&fx.fd, call, (-32i64) as u64, 8, 0xff), None);
    }

    #[test]
    fn partial_register_zext_keeps_unrelated_complete_summaries() {
        let p = Program::new();
        let extended = p.value(&p.unique, 0, 8);
        let narrowed = p.value(&p.unique, 8, 4);
        let summary = p
            .run(vec![vec![
                p.op(
                    OpCode::CPUI_INT_ZEXT,
                    Some(extended.clone()),
                    vec![p.value(&p.register, 0x20, 4)],
                ),
                p.op(
                    OpCode::CPUI_SUBPIECE,
                    Some(narrowed.clone()),
                    vec![extended, p.c(0)],
                ),
                p.op(
                    OpCode::CPUI_STORE,
                    None,
                    vec![p.c(1), p.c(0x2000), narrowed],
                ),
                p.ret(),
            ]])
            .unwrap();
        assert!(summary.deferred_frame_escapes.is_empty());
        assert!(summary
            .lossy_pointer_ranges
            .iter()
            .any(|range| { range.space == 2 && range.offset == 0x20 && range.size == 4 }));
    }
}
