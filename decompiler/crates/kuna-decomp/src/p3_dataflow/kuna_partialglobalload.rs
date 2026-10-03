//! Preserve partial global stores without forcing heritage's synthetic writes.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::address::{calc_mask, Address};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub(crate) struct Guard {
    pub addr: Address,
    pub size: int4,
    pub writes: Vec<VarnodeId>,
    pub loads: Vec<OpId>,
}

/// Resolve pointer bounds after renaming, before dead-code removal. A pointer
/// proven outside the global does not require its partial stores to survive.
pub(crate) fn apply(fd: &mut Funcdata, guards: Vec<Guard>) {
    let mut bounds = HashMap::new();
    let mut inputs = HashMap::new();
    for guard in guards {
        let lo = u128::from(guard.addr.get_offset());
        let hi = lo + guard.size as u128;
        let reads = guard.loads.iter().any(|&id| {
            let Some(op) = fd.obank().get(id) else {
                return false;
            };
            let Some(ptr) = op.get_in(1) else { return true };
            let Some(v) = fd.vbank().get(ptr) else {
                return true;
            };
            let size = op
                .get_out()
                .and_then(|vn| fd.vbank().get(vn))
                .map_or(1, |v| v.get_size());
            if !from_input(fd, ptr, &mut inputs) {
                return false;
            }
            let space = guard.addr.get_space().expect("partial global guard space");
            let range = unsigned_bounds(fd, ptr, &mut bounds, 0);
            may_overlap(
                range,
                size,
                lo,
                hi,
                v.get_size(),
                space.get_addr_size() as int4,
                space.get_word_size(),
            )
        });
        if !reads {
            continue;
        }
        for vn in guard.writes {
            let real = fd
                .vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .and_then(|d| fd.obank().get(d))
                .is_some_and(|d| !d.is_marker() && !d.is_return_copy());
            if real {
                if let Some(v) = fd.vbank_mut().get_mut(vn) {
                    v.set_addr_force();
                }
            }
        }
    }
}

/// This correction covers pointer reads derived from function inputs. Values
/// fetched from memory or created by calls need separate alias provenance.
fn from_input(fd: &Funcdata, root: VarnodeId, memo: &mut HashMap<VarnodeId, bool>) -> bool {
    let mut work = vec![root];
    let mut seen = HashSet::new();
    while let Some(vn) = work.pop() {
        if let Some(&input) = memo.get(&vn) {
            if input {
                memo.insert(root, true);
                return true;
            }
            continue;
        }
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > 1024 {
            return false;
        }
        let Some(v) = fd.vbank().get(vn) else {
            continue;
        };
        if v.is_input() {
            if !v.is_persist() {
                memo.insert(root, true);
                return true;
            }
            continue;
        }
        if v.is_constant() {
            continue;
        }
        let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else {
            continue;
        };
        if op.code() == OpCode::CPUI_LOAD || op.is_call() || op.code() == OpCode::CPUI_CALLOTHER {
            continue;
        }
        let count = if op.code() == OpCode::CPUI_INDIRECT {
            1
        } else {
            op.num_input()
        };
        work.extend((0..count).filter_map(|i| op.get_in(i)));
    }
    for vn in seen {
        memo.insert(vn, false);
    }
    false
}

fn may_overlap(
    (a, b): (u64, u64),
    size: int4,
    lo: u128,
    hi: u128,
    pointer_bytes: int4,
    address_bytes: int4,
    word_bytes: u32,
) -> bool {
    if pointer_bytes != address_bytes || word_bytes != 1 {
        return true;
    }
    let end = u128::from(b) + size as u128;
    end > u128::from(calc_mask(pointer_bytes)) + 1 || (u128::from(a) < hi && lo < end)
}

/// Unsigned intervals are conservative across unknown operations, cycles and
/// arithmetic wrap. Byte concatenation and bounded carries expose constant
/// pointer prefixes, including pointers assembled in address-tied memory.
fn unsigned_bounds(
    fd: &Funcdata,
    vn: VarnodeId,
    memo: &mut HashMap<VarnodeId, (u64, u64)>,
    depth: usize,
) -> (u64, u64) {
    if let Some(&range) = memo.get(&vn) {
        return range;
    }
    let Some(v) = fd.vbank().get(vn) else {
        return (0, u64::MAX);
    };
    let mask = calc_mask(v.get_size());
    let full = (0, mask);
    if v.is_constant() {
        return (v.get_offset() & mask, v.get_offset() & mask);
    }
    if depth >= 24 || memo.len() >= 1024 || v.get_size() > 8 {
        return full;
    }
    memo.insert(vn, full);
    let Some(op) = v.get_def().and_then(|d| fd.obank().get(d)) else {
        return full;
    };
    let mut input = |slot| {
        op.get_in(slot)
            .map(|i| unsigned_bounds(fd, i, memo, depth + 1))
    };
    let range = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => input(0).unwrap_or(full),
        OpCode::CPUI_PIECE => {
            let bytes = op
                .get_in(1)
                .and_then(|i| fd.vbank().get(i))
                .map_or(8, |v| v.get_size());
            match (input(0), input(1)) {
                (Some((a, b)), Some((c, d))) if bytes < 8 => {
                    ((a << (bytes * 8)) | c, (b << (bytes * 8)) | d)
                }
                _ => full,
            }
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => match (input(0), input(1)) {
            (Some((a, b)), Some((c, d))) if u128::from(b) + u128::from(d) <= u128::from(mask) => {
                (a + c, b + d)
            }
            _ => full,
        },
        OpCode::CPUI_INT_MULT => match (input(0), input(1)) {
            (Some((a, b)), Some((c, d))) if u128::from(b) * u128::from(d) <= u128::from(mask) => {
                (a * c, b * d)
            }
            _ => full,
        },
        OpCode::CPUI_INT_AND => match (input(0), input(1)) {
            (Some((_, b)), Some((_, d))) => (0, b.min(d)),
            _ => full,
        },
        OpCode::CPUI_INT_LEFT => match (input(0), input(1)) {
            (Some((a, b)), Some((c, d)))
                if c == d && c < 64 && (u128::from(b) << c) <= u128::from(mask) =>
            {
                (a << c, b << c)
            }
            _ => full,
        },
        OpCode::CPUI_INT_RIGHT | OpCode::CPUI_SUBPIECE => {
            let shift_bytes = op.code() == OpCode::CPUI_SUBPIECE;
            match (input(0), input(1)) {
                (Some((a, b)), Some((c, d))) if c == d && c < if shift_bytes { 8 } else { 64 } => {
                    let shift = if shift_bytes { c * 8 } else { c };
                    let (a, b) = (a >> shift, b >> shift);
                    if a <= mask && b <= mask {
                        (a, b)
                    } else {
                        full
                    }
                }
                _ => full,
            }
        }
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_CARRY
        | OpCode::CPUI_INT_SCARRY
        | OpCode::CPUI_INT_SBORROW
        | OpCode::CPUI_BOOL_NEGATE
        | OpCode::CPUI_BOOL_XOR
        | OpCode::CPUI_BOOL_AND
        | OpCode::CPUI_BOOL_OR => (0, 1),
        OpCode::CPUI_MULTIEQUAL => {
            let mut range = (mask, 0);
            for i in 0..op.num_input() {
                let (a, b) = input(i).unwrap_or(full);
                range = (range.0.min(a), range.1.max(b));
            }
            if op.num_input() == 0 {
                full
            } else {
                range
            }
        }
        _ => full,
    };
    let range = if range.0 <= range.1 && range.1 <= mask {
        range
    } else {
        full
    };
    memo.insert(vn, range);
    range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_intervals_include_wrapped_bytes() {
        assert!(may_overlap((0xfffe, 0xffff), 2, 0, 1, 2, 2, 1));
        assert!(may_overlap((0x1010, 0x1020), 4, 0x1021, 0x1024, 2, 2, 1));
        assert!(!may_overlap((0x1010, 0x1020), 1, 0x1021, 0x1024, 2, 2, 1));
        assert!(!may_overlap((0xd11a, 0xd219), 2, 0, 4, 2, 2, 1));
        assert!(may_overlap((0x100, 0x100), 1, 0, 4, 2, 1, 1));
        assert!(may_overlap((0x100, 0x100), 1, 0, 4, 2, 2, 2));
    }
}
