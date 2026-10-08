//! Conservative physical frame-write preservation (`stackalias`, #810).
//!
//! An escaped pointer reloaded from a global need not resolve to a frame SSA
//! value. Dead-code elimination then sees no reader for the physical writes.
//! The opt-in policy holds those writes live without preventing ordinary value
//! propagation. Proved non-aliased spills and completely overwritten observable
//! bytes are excluded; uncertain stores remain live.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeSet;

/// Whether this Varnode is a physical write in this function's frame space.
pub fn holds_store(fd: &Funcdata, id: VarnodeId) -> bool {
    if !fd.get_arch().stack_alias_deadstore && !fd.get_arch().stack_views {
        return false;
    }
    let Some(scope) = fd.get_scope_local() else {
        return false;
    };
    fd.vbank().get(id).is_some_and(|v| {
        v.is_stack_store()
            && v.get_space().get_index() == scope.get_space_id().get_index()
            && !v.has_no_local_alias()
            && fd
                .stack_alias_holds
                .as_ref()
                .is_none_or(|holds| holds.contains(&id))
    })
}

/// Compute once per dead-code pass. A later full overwrite before any memory
/// observer kills only the earlier store's potentially aliased bytes.
pub fn prepare(fd: &mut Funcdata) -> i32 {
    if !fd.get_arch().stack_alias_deadstore && !fd.get_arch().stack_views {
        return 0;
    }
    let Some(space) = fd
        .get_scope_local()
        .map(|s| std::rc::Rc::clone(s.get_space_id()))
    else {
        return 0;
    };
    let mut checker = fd.build_alias_checker_deferred();
    let mut holds = BTreeSet::new();
    for id in fd.vbank().loc_space_ids(&space) {
        let Some(v) = fd.vbank().get(id) else {
            continue;
        };
        if !v.is_stack_store() || v.has_no_local_alias() {
            continue;
        }
        let size = v.get_size();
        if size <= 0 || size > 16 || !space.stack_grows_negative() {
            holds.insert(id);
            continue;
        }
        let mut wanted = 0u32;
        for byte in 0..size {
            let aliased = match checker.as_mut() {
                Some(checker) => {
                    let offset = v.get_offset().wrapping_add(byte as u64);
                    let mut access = fd.alias_gather_access();
                    checker.has_local_alias(Some((std::rc::Rc::clone(&space), offset)), &mut access)
                        || checker.has_parameter_alias(offset, &mut access)
                }
                None => true,
            };
            if aliased {
                wanted |= 1 << byte;
            }
        }
        if wanted != 0
            && !v
                .get_def()
                .is_some_and(|op| shadowed(fd, op, v.get_offset(), size, wanted))
        {
            holds.insert(id);
        }
    }
    let mut changed = 0;
    if let Some(old) = fd.stack_alias_holds.replace(holds) {
        let removed: Vec<_> = old
            .difference(fd.stack_alias_holds.as_ref().unwrap())
            .copied()
            .collect();
        for id in removed {
            if let Some(v) = fd.vbank_mut().get_mut(id) {
                if v.is_auto_live_hold() {
                    v.clear_auto_live_hold();
                    changed += 1;
                }
            }
        }
    }
    changed
}

fn byte_mask(start: u64, size: i32, at: u64, width: i32) -> u32 {
    let mut mask = 0;
    for byte in 0..size {
        if start.wrapping_add(byte as u64).wrapping_sub(at) < width as u64 {
            mask |= 1 << byte;
        }
    }
    mask
}

/// Search every reachable path with the bytes that still belong to the old write.
fn shadowed(fd: &Funcdata, def: OpId, start: u64, size: i32, wanted: u32) -> bool {
    let Some(initial) = fd.obank().get(def) else {
        return false;
    };
    let Some(parent) = initial.get_parent() else {
        return false;
    };
    let space = fd.get_scope_local().unwrap().get_space_id().get_index();
    let mut work = vec![(parent, initial.basic_neighbours().1, wanted)];
    let mut visited = BTreeSet::new();
    let mut budget = 4096usize;
    while let Some((block, mut next, mut wanted)) = work.pop() {
        if !visited.insert((block, next, wanted)) {
            continue;
        }
        if budget == 0 {
            return false;
        }
        budget -= 1;
        while let Some(id) = next {
            if budget == 0 {
                return false;
            }
            budget -= 1;
            let Some(op) = fd.obank().get(id) else {
                return false;
            };
            if op.get_parent() != Some(block) {
                return false;
            }
            next = op.basic_neighbours().1;
            if op.is_marker() {
                continue;
            }
            let Some(remaining) = unobserved_bytes(fd, id, start, size, wanted, space) else {
                return false;
            };
            wanted = remaining;
            if wanted == 0 {
                break;
            }
        }
        if wanted == 0 {
            continue;
        }
        let block = fd.bblocks_ref().block(block);
        if block.size_out() == 0 {
            return false;
        }
        for edge in 0..block.size_out() {
            let target = block.get_out(edge);
            work.push((target, fd.bb_op_head(target), wanted));
        }
    }
    true
}

fn unobserved_bytes(
    fd: &Funcdata,
    id: OpId,
    start: u64,
    size: i32,
    mut wanted: u32,
    space: i32,
) -> Option<u32> {
    let op = fd.obank().get(id)?;
    if op.code() == OpCode::CPUI_CALL {
        wanted = crate::kuna_calleememory::unobserved(fd, id, start, size, wanted)?;
    }
    if matches!(
        op.code(),
        OpCode::CPUI_CALLIND
            | OpCode::CPUI_RETURN
            | OpCode::CPUI_LOAD
            | OpCode::CPUI_STORE
            | OpCode::CPUI_CALLOTHER
    ) {
        return None;
    }
    for slot in 0..op.num_input() {
        let Some(v) = op.get_in(slot).and_then(|id| fd.vbank().get(id)) else {
            continue;
        };
        if v.get_space().get_index() != space {
            continue;
        }
        let (at, width) = if slot == 0 && op.code() == OpCode::CPUI_SUBPIECE {
            let offset = op
                .get_in(1)
                .and_then(|id| fd.vbank().get(id))
                .filter(|v| v.is_constant())?;
            let out = op.get_out().and_then(|id| fd.vbank().get(id))?;
            let offset = offset.get_offset();
            let relative = if v.get_addr().is_big_endian() {
                let total: u64 = (v.get_size() - out.get_size()).try_into().ok()?;
                total.checked_sub(offset)?
            } else {
                offset
            };
            (v.get_offset().wrapping_add(relative), out.get_size())
        } else {
            (v.get_offset(), v.get_size())
        };
        if byte_mask(start, size, at, width) & wanted != 0 {
            return None;
        }
    }
    if let Some(out) = op.get_out().and_then(|id| fd.vbank().get(id)) {
        if out.is_stack_store() && out.get_space().get_index() == space {
            wanted &= !byte_mask(start, size, out.get_offset(), out.get_size());
        }
    }
    Some(wanted)
}

/// (kuna GH-8500) Toggle preservation of a store-through-a-stack-pointer-alias
/// across the deadcode race (C++ `OptionStackAlias`).
///
/// Off by default because the conservative policy retains dead frame writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackAliasOption {
    /// True if a store-through-a-stack-pointer-alias is held across the deadcode
    /// race (C++ `Architecture::stack_alias_deadstore`).
    pub enabled: bool,
}

impl Default for StackAliasOption {
    /// Shipped default: `option stackalias off` (upstream byte-identical;
    /// `architecture.cc` sets `stack_alias_deadstore = false`).
    fn default() -> Self {
        StackAliasOption { enabled: false }
    }
}

impl StackAliasOption {
    /// (kuna) Set the gate (C++ `OptionStackAlias::apply`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Stack-alias dead-store preservation turned on"
        } else {
            "Stack-alias dead-store preservation turned off"
        }
    }

    /// Read the gate (C++ `glb->stack_alias_deadstore`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_stackalias/tests.rs"]
mod tests;
