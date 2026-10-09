//! Hide split-lane stack saves only when the ABI and memory uses prove them
//! to be bookkeeping.

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::fspec::effect_type;
use crate::funcdata::Funcdata;
use kuna_base::address::{sign_extend, Address};
use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

#[derive(Clone)]
struct EffectRange {
    address: Address,
    size: i32,
}

#[derive(Clone, Copy)]
struct FrameRange {
    start: u64,
    size: i32,
    exact: bool,
}

#[derive(Clone)]
struct Save {
    op: OpId,
    slot: FrameRange,
    source: VarnodeId,
}

fn contains(outer: &Address, outer_size: i32, inner: &Address, inner_size: i32) -> bool {
    if outer_size <= 0 || inner_size <= 0 {
        return false;
    }
    let (Some(outer_space), Some(inner_space)) = (outer.get_space(), inner.get_space()) else {
        return false;
    };
    if !Rc::ptr_eq(outer_space, inner_space) {
        return false;
    }
    let start = u128::from(outer.get_offset());
    let inner_start = u128::from(inner.get_offset());
    inner_start >= start && inner_start + inner_size as u128 <= start + outer_size as u128
}

fn overlaps(space: &AddrSpace, left: FrameRange, right: FrameRange) -> bool {
    if left.size <= 0 || right.size <= 0 {
        return false;
    }
    let mask = kuna_base::address::calc_mask(space.get_addr_size() as i32);
    (left.start.wrapping_sub(right.start) & mask) < right.size as u64
        || (right.start.wrapping_sub(left.start) & mask) < left.size as u64
}

fn overlaps_saves(space: &AddrSpace, range: FrameRange, saves: &[Save]) -> bool {
    saves.iter().any(|save| overlaps(space, range, save.slot))
}

fn frame_range(fd: &Funcdata, pointer: VarnodeId, width: i32) -> Option<FrameRange> {
    let address = crate::kuna_stackranges::frame_address(fd, pointer)?;
    let (start, size) = address.extent(width)?;
    Some(FrameRange {
        start,
        size,
        exact: address.first == address.last && address.terms.is_empty(),
    })
}

fn exact_frame_range(fd: &Funcdata, pointer: VarnodeId, width: i32) -> Option<FrameRange> {
    let range = frame_range(fd, pointer, width)?;
    (range.exact && range.size == width).then_some(range)
}

fn pointer_temporary(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(value) = fd.vbank().get(vn) else {
        return false;
    };
    let space = value.get_space();
    let register = fd.get_arch().manage().get_space_by_name("register");
    (register.is_some_and(|register| Rc::ptr_eq(register, space))
        || space.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
        && !value.is_persist()
        && !value.is_stack_store()
}

fn pointer_propagation(fd: &Funcdata, id: OpId) -> bool {
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    matches!(
        op.code(),
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
    ) && op.get_out().is_some_and(|out| {
        pointer_temporary(fd, out) && crate::kuna_stackranges::frame_address(fd, out).is_some()
    })
}

fn fully_covered_lane(effect: &EffectRange, address: &Address, size: i32) -> bool {
    size > 0 && size < effect.size && contains(&effect.address, effect.size, address, size)
}

fn eligible_effect_lane(
    effect: &EffectRange,
    address: &Address,
    size: i32,
    is_input: bool,
    is_unaffected: bool,
    prototype_effect: u32,
) -> bool {
    is_input
        && is_unaffected
        && prototype_effect == effect_type::UNAFFECTED
        && fully_covered_lane(effect, address, size)
}

fn stack_memory_access(fd: &Funcdata, id: OpId, stack_space: &Rc<AddrSpace>) -> bool {
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
        && crate::ruleaction_4::RuleLoadVarnode::check_spacebase(fd, id)
            .is_some_and(|(space, _)| Rc::ptr_eq(&space, stack_space))
}

fn may_be_stack_derived(
    fd: &Funcdata,
    vn: VarnodeId,
    budget: &mut usize,
    seen: &mut std::collections::BTreeSet<VarnodeId>,
) -> bool {
    let Some(left) = (*budget).checked_sub(1) else {
        return true;
    };
    *budget = left;
    if !seen.insert(vn) {
        return false;
    }
    let Some(value) = fd.vbank().get(vn) else {
        return true;
    };
    if value.is_spacebase() && value.is_input() {
        return true;
    }
    let Some(def) = value.get_def().and_then(|id| fd.obank().get(id)) else {
        return false;
    };
    if def.code() == OpCode::CPUI_LOAD {
        return false;
    }
    (0..def.num_input()).any(|slot| {
        def.get_in(slot)
            .is_some_and(|input| may_be_stack_derived(fd, input, budget, seen))
    })
}

fn may_be_stack_derived_from(fd: &Funcdata, vn: VarnodeId) -> bool {
    may_be_stack_derived(fd, vn, &mut 128, &mut std::collections::BTreeSet::new())
}

fn copied_from(fd: &Funcdata, mut value: VarnodeId, source: VarnodeId) -> bool {
    for _ in 0..32 {
        if value == source {
            return true;
        }
        let Some(vn) = fd.vbank().get(value) else {
            return false;
        };
        let Some(def) = vn.get_def().and_then(|id| fd.obank().get(id)) else {
            return false;
        };
        if !matches!(def.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) || def.num_input() != 1 {
            return false;
        }
        let Some(input) = def.get_in(0) else {
            return false;
        };
        let Some(parent) = fd.vbank().get(input) else {
            return false;
        };
        if parent.get_size() != vn.get_size() {
            return false;
        }
        value = input;
    }
    false
}

fn negative_local(space: &AddrSpace, range: FrameRange) -> bool {
    space.stack_grows_negative()
        && range.size > 0
        && sign_extend(range.start as i64, space.get_addr_size() as i32 * 8 - 1) < 0
        && sign_extend(
            range.start.wrapping_add(range.size as u64 - 1) as i64,
            space.get_addr_size() as i32 * 8 - 1,
        ) < 0
}

fn save_op_matches(
    fd: &Funcdata,
    id: OpId,
    range: FrameRange,
    saves: &[Save],
    stack_space: &Rc<AddrSpace>,
) -> bool {
    saves.iter().any(|save| {
        if save.op != id || save.slot.start != range.start || save.slot.size != range.size {
            return false;
        }
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        if op.code() == OpCode::CPUI_STORE && !stack_memory_access(fd, id, stack_space) {
            return false;
        }
        let source = match op.code() {
            OpCode::CPUI_STORE => op.get_in(2),
            OpCode::CPUI_COPY => op.get_in(0),
            _ => None,
        };
        source.is_some_and(|value| copied_from(fd, value, save.source))
    })
}

fn access_width(fd: &Funcdata, id: OpId, pointer_slot: i32) -> i32 {
    let Some(op) = fd.obank().get(id) else {
        return 0;
    };
    match op.code() {
        OpCode::CPUI_LOAD => op
            .get_out()
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_size())
            .unwrap_or(0),
        OpCode::CPUI_STORE if pointer_slot == 1 => op
            .get_in(2)
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_size())
            .unwrap_or(0),
        _ => op
            .get_in(pointer_slot)
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_size())
            .unwrap_or(0),
    }
}

fn call_unobserved(
    fd: &Funcdata,
    id: OpId,
    range: FrameRange,
    stack_space: &Rc<AddrSpace>,
) -> bool {
    if !range.exact || !(1..=16).contains(&range.size) {
        return false;
    }
    let Some(bits) = 1u32.checked_shl(range.size as u32) else {
        return false;
    };
    let mask = bits - 1;
    let address = Address::new(Rc::clone(stack_space), range.start);
    let Some(index) = fd.get_call_specs_index(id) else {
        return false;
    };
    let call = fd.get_call_specs(index);
    crate::kuna_calleememory::unobserved(fd, id, range.start, range.size, mask) == Some(mask)
        && crate::kuna_calleememory::preserves(fd, call, &address, range.size)
}

fn proven_unobserved(fd: &Funcdata, saves: &[Save], stack_space: &Rc<AddrSpace>) -> bool {
    if saves.is_empty()
        || saves
            .iter()
            .any(|save| !negative_local(stack_space, save.slot))
    {
        return false;
    }
    let ops: Vec<OpId> = fd.obank().iter_all().map(|(_, id)| id).collect();
    for id in ops {
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        match op.code() {
            OpCode::CPUI_CALL => {
                if saves
                    .iter()
                    .any(|save| !call_unobserved(fd, id, save.slot, stack_space))
                {
                    return false;
                }
            }
            OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return false,
            _ => {}
        }
        if let Some(out) = op.get_out().and_then(|vn| fd.vbank().get(vn)) {
            if out.get_space().get_index() == stack_space.get_index() {
                let write = FrameRange {
                    start: out.get_offset(),
                    size: out.get_size(),
                    exact: true,
                };
                if overlaps_saves(stack_space, write, saves)
                    && !save_op_matches(fd, id, write, saves, stack_space)
                {
                    return false;
                }
            }
        }
        for slot in 0..op.num_input() {
            let Some(input_id) = op.get_in(slot) else {
                continue;
            };
            let input_size = fd
                .vbank()
                .get(input_id)
                .map(|vn| vn.get_size())
                .unwrap_or(0);
            let width = match op.code() {
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE if slot == 1 => access_width(fd, id, slot),
                _ => input_size,
            };
            let mut frame = frame_range(fd, input_id, width);
            if matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) && slot == 1 {
                if frame.is_some_and(|range| overlaps_saves(stack_space, range, saves))
                    && !stack_memory_access(fd, id, stack_space)
                {
                    return false;
                }
                if !stack_memory_access(fd, id, stack_space) {
                    frame = None;
                }
            }
            let possibly_frame_derived = frame.is_some() || may_be_stack_derived_from(fd, input_id);
            if possibly_frame_derived {
                match op.code() {
                    OpCode::CPUI_CALL => {
                        // The direct-call loop above requires complete
                        // unobserved/preserves summaries for every candidate slot.
                    }
                    OpCode::CPUI_LOAD if slot == 1 => {
                        if frame.is_none() {
                            return false;
                        }
                    }
                    OpCode::CPUI_STORE if slot == 1 => {
                        if frame.is_none() {
                            return false;
                        }
                    }
                    OpCode::CPUI_COPY
                    | OpCode::CPUI_CAST
                    | OpCode::CPUI_INT_ADD
                    | OpCode::CPUI_INT_SUB
                    | OpCode::CPUI_INT_2COMP
                    | OpCode::CPUI_PTRADD
                    | OpCode::CPUI_PTRSUB => {
                        if !pointer_propagation(fd, id) {
                            return false;
                        }
                    }
                    // Returning a frame-derived value, or storing one as data,
                    // can expose a saved slot beyond this function's frame scan.
                    OpCode::CPUI_RETURN => return false,
                    OpCode::CPUI_STORE => return false,
                    _ => return false,
                }
            }
            if let Some(frame) = frame {
                if overlaps_saves(stack_space, frame, saves) {
                    let allowed = match op.code() {
                        OpCode::CPUI_STORE if slot == 1 => {
                            save_op_matches(fd, id, frame, saves, stack_space)
                        }
                        OpCode::CPUI_CALL => call_unobserved(fd, id, frame, stack_space),
                        OpCode::CPUI_COPY
                        | OpCode::CPUI_CAST
                        | OpCode::CPUI_INT_ADD
                        | OpCode::CPUI_INT_SUB
                        | OpCode::CPUI_INT_2COMP
                        | OpCode::CPUI_PTRADD
                        | OpCode::CPUI_PTRSUB => pointer_propagation(fd, id),
                        _ => false,
                    };
                    if !allowed {
                        return false;
                    }
                }
            }
            if let Some(input) = fd.vbank().get(input_id) {
                if input.get_space().get_index() == stack_space.get_index()
                    && overlaps_saves(
                        stack_space,
                        FrameRange {
                            start: input.get_offset(),
                            size: input.get_size(),
                            exact: true,
                        },
                        saves,
                    )
                {
                    return false;
                }
            }
        }
    }
    true
}

pub(crate) fn restrict_local_saved_lanes(fd: &mut Funcdata) -> i32 {
    if !fd.get_arch().stack_views && !fd.get_arch().stack_alias_deadstore {
        return 0;
    }
    let Some(stack_space) = fd.get_arch().manage().get_stack_space().map(Rc::clone) else {
        return 0;
    };
    let effects: Vec<EffectRange> = fd
        .get_func_proto()
        .effect_list()
        .iter()
        .filter(|effect| effect.get_type() == effect_type::UNAFFECTED)
        .filter(|effect| (1..=16).contains(&effect.get_size()))
        .map(|effect| EffectRange {
            address: effect.get_address(),
            size: effect.get_size(),
        })
        .filter(|effect| {
            effect.address.get_space().is_some_and(|space| {
                space.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR
            })
        })
        .collect();
    let ops: Vec<OpId> = fd.obank().iter_all().map(|(_, id)| id).collect();
    let mut marks = Vec::new();
    for effect in effects {
        if fd
            .find_varnode_input(effect.size, &effect.address)
            .is_some()
        {
            continue;
        }
        let end = &effect.address + effect.size as i64;
        let lanes: Vec<VarnodeId> = fd
            .vbank()
            .iter_loc_addr_range(&effect.address, &end)
            .filter(|&id| {
                fd.vbank().get(id).is_some_and(|lane| {
                    eligible_effect_lane(
                        &effect,
                        lane.get_addr(),
                        lane.get_size(),
                        lane.is_input(),
                        lane.is_unaffected(),
                        fd.get_func_proto()
                            .has_effect(lane.get_addr(), lane.get_size()),
                    )
                })
            })
            .collect();
        let mut saves = Vec::new();
        for lane in lanes {
            for &id in &ops {
                let Some(op) = fd.obank().get(id) else {
                    continue;
                };
                match op.code() {
                    OpCode::CPUI_STORE => {
                        let Some(value) = op.get_in(2) else { continue };
                        if !copied_from(fd, value, lane)
                            || !stack_memory_access(fd, id, &stack_space)
                        {
                            continue;
                        }
                        let Some(pointer) = op.get_in(1) else {
                            continue;
                        };
                        let size = fd.vbank().get(value).map(|vn| vn.get_size()).unwrap_or(0);
                        let Some(slot) = exact_frame_range(fd, pointer, size) else {
                            continue;
                        };
                        saves.push(Save {
                            op: id,
                            slot,
                            source: lane,
                        });
                    }
                    OpCode::CPUI_COPY => {
                        let Some(value) = op.get_in(0) else { continue };
                        if !copied_from(fd, value, lane) {
                            continue;
                        }
                        let Some(out) = op.get_out().and_then(|vn| fd.vbank().get(vn)) else {
                            continue;
                        };
                        if out.get_space().get_index() != stack_space.get_index()
                            || out.get_size()
                                != fd.vbank().get(lane).map(|vn| vn.get_size()).unwrap_or(0)
                        {
                            continue;
                        }
                        saves.push(Save {
                            op: id,
                            slot: FrameRange {
                                start: out.get_offset(),
                                size: out.get_size(),
                                exact: true,
                            },
                            source: lane,
                        });
                    }
                    _ => {}
                }
            }
        }
        if !saves.is_empty() && proven_unobserved(fd, &saves, &stack_space) {
            marks.extend(saves.into_iter().map(|save| save.slot));
        }
    }
    marks.sort_by_key(|slot| (slot.start, slot.size));
    marks.dedup_by_key(|slot| (slot.start, slot.size));
    for slot in &marks {
        fd.scope_local_mark_not_mapped(&stack_space, slot.start, slot.size, false);
    }
    i32::from(!marks.is_empty())
}

pub(crate) struct ActionRestrictLocalSavedLanes {
    base: ActionBase,
}

impl ActionRestrictLocalSavedLanes {
    pub(crate) fn boxed() -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(
                ruleflags::rule_oneactperfunc,
                "restrictlocalsavedlanes",
                "localrecovery",
            ),
        })
    }
}

impl Action for ActionRestrictLocalSavedLanes {
    fn base(&self) -> &ActionBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| {
            Box::new(Self {
                base: self.base.clone(),
            }) as Box<dyn Action>
        })
    }

    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let count = restrict_local_saved_lanes(data);
        self.base_mut().count += count;
        0
    }
}

#[cfg(test)]
mod tests;
