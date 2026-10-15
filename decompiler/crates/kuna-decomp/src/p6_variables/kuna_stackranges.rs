//! Bounded byte addresses relative to the frame's input spacebase.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

const LIMIT: i64 = 65536;

#[derive(Clone)]
pub(crate) struct IndexTerm {
    pub value: VarnodeId,
    pub scale: i64,
    pub op: OpId,
}

pub(crate) struct FrameAddress {
    pub pointer_size: i32,
    pub offset: i64,
    pub first: i64,
    pub last: i64,
    pub terms: Vec<IndexTerm>,
}

impl FrameAddress {
    fn add(&mut self, fd: &Funcdata, value: VarnodeId, scale: i64, op: OpId) -> Option<()> {
        self.add_scalar(fd, value, scale, op, &mut 32)
    }

    fn add_scalar(
        &mut self,
        fd: &Funcdata,
        value: VarnodeId,
        scale: i64,
        op: OpId,
        budget: &mut usize,
    ) -> Option<()> {
        *budget = budget.checked_sub(1)?;
        let vn = fd.vbank().get(value)?;
        if !(1..=8).contains(&vn.get_size()) {
            return None;
        }
        if scale == 0 {
            return Some(());
        }
        if vn.is_constant() {
            let value = if vn.get_size() == self.pointer_size {
                signed_constant(vn.get_offset(), vn.get_size())
            } else {
                vn.get_offset() as i64
            };
            let shift = value.checked_mul(scale)?;
            self.offset = self.offset.checked_add(shift)?;
            self.first = self.first.checked_add(shift)?;
            self.last = self.last.checked_add(shift)?;
        } else {
            if vn.get_size() == self.pointer_size {
                if let Some(def) = vn
                    .get_def()
                    .and_then(|id| fd.obank().get(id).map(|op| (id, op)))
                {
                    let (id, operation) = def;
                    let input = operation.get_in(0)?;
                    match operation.code() {
                        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                            self.add_scalar(fd, input, scale, id, budget)?;
                            let factor = if operation.code() == OpCode::CPUI_INT_SUB {
                                -1
                            } else {
                                1
                            };
                            return self.add_scalar(
                                fd,
                                operation.get_in(1)?,
                                scale.checked_mul(factor)?,
                                id,
                                budget,
                            );
                        }
                        OpCode::CPUI_INT_2COMP => {
                            return self.add_scalar(fd, input, scale.checked_neg()?, id, budget)
                        }
                        OpCode::CPUI_INT_MULT => {
                            let other = operation.get_in(1)?;
                            let (input, constant) = if fd.vbank().get(other)?.is_constant() {
                                (input, other)
                            } else if fd.vbank().get(input)?.is_constant() {
                                (other, input)
                            } else {
                                return None;
                            };
                            let constant = fd.vbank().get(constant)?;
                            let factor = signed_constant(constant.get_offset(), self.pointer_size);
                            return self.add_scalar(
                                fd,
                                input,
                                scale.checked_mul(factor)?,
                                id,
                                budget,
                            );
                        }
                        _ => (),
                    }
                }
            }
            let mask = i64::try_from(vn.get_nz_mask()).ok()?;
            if mask as u64 & (1u64 << (8 * vn.get_size() - 1)) != 0
                && vn.get_type().get_metatype() != crate::dtype::type_metatype::TYPE_UINT
            {
                return None;
            }
            let delta = mask.checked_mul(scale)?;
            if delta.unsigned_abs() >= LIMIT as u64 || self.terms.len() == 16 {
                return None;
            }
            self.first = self.first.checked_add(delta.min(0))?;
            self.last = self.last.checked_add(delta.max(0))?;
            self.terms.push(IndexTerm { value, scale, op });
        }
        (self.last.checked_sub(self.first)? < LIMIT).then_some(())
    }

    pub fn extent(&self, width: i32) -> Option<(u64, i32)> {
        let size = self
            .last
            .checked_sub(self.first)?
            .checked_add(i64::from(width))?;
        (width > 0 && size <= LIMIT).then_some((
            self.first as u64 & kuna_base::address::calc_mask(self.pointer_size),
            size as i32,
        ))
    }
}

fn signed_constant(value: u64, size: i32) -> i64 {
    let bits = size * 8;
    if bits < 64 {
        ((value << (64 - bits)) as i64) >> (64 - bits)
    } else {
        value as i64
    }
}

pub(crate) fn frame_address(fd: &Funcdata, value: VarnodeId) -> Option<FrameAddress> {
    if fd.get_arch().manage().get_stack_space()?.get_word_size() != 1 {
        return None;
    }
    walk(fd, value, &mut 64)
}

/// Pointer choices and unresolved pointer arms cannot prove frame disjointness.
pub(crate) fn may_alias(fd: &Funcdata, value: VarnodeId, start: u64, size: i32) -> bool {
    let Some(space) = fd.get_arch().manage().get_stack_space() else {
        return true;
    };
    if space.get_word_size() != 1 || size <= 0 {
        return true;
    }
    let mut pending = vec![(value, false, false)];
    let mut visited = std::collections::BTreeSet::new();
    let mut budget = 64usize;
    while let Some((value, pointer, unresolved)) = pending.pop() {
        if budget == 0 {
            return true;
        }
        budget -= 1;
        if !visited.insert((value, pointer, unresolved)) {
            continue;
        }
        if let Some(address) = frame_address(fd, value) {
            if unresolved {
                return true;
            }
            let Some((at, width)) = address.extent(1) else {
                return true;
            };
            if space.wrap_offset(at.wrapping_sub(start)) < size as u64
                || space.wrap_offset(start.wrapping_sub(at)) < width as u64
            {
                return true;
            }
            continue;
        }
        let Some(v) = fd.vbank().get(value) else {
            return true;
        };
        if v.is_constant() {
            continue;
        }
        let pointer = pointer
            || v.get_type().get_metatype() == crate::dtype::type_metatype::TYPE_PTR
            || (v.get_type().get_metatype() == crate::dtype::type_metatype::TYPE_UNKNOWN
                && v.get_size() == space.get_addr_size() as i32);
        let Some(op) = v.get_def().and_then(|id| fd.obank().get(id)) else {
            if pointer {
                return true;
            }
            continue;
        };
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
                if let Some(input) = op.get_in(0) {
                    pending.push((input, pointer, unresolved));
                } else {
                    return true;
                }
            }
            OpCode::CPUI_MULTIEQUAL => {
                if op.num_input() as usize > budget.saturating_sub(pending.len()) {
                    return true;
                }
                pending.extend((0..op.num_input()).filter_map(|slot| {
                    op.get_in(slot).map(|input| (input, pointer, unresolved))
                }));
            }
            OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB => {
                if (0..op.num_input()).all(|slot| {
                    op.get_in(slot)
                        .and_then(|input| fd.vbank().get(input))
                        .is_some_and(|input| input.is_constant())
                }) {
                    continue;
                }
                if pointer {
                    return true;
                }
                pending.extend((0..op.num_input().min(2)).filter_map(|slot| {
                    op.get_in(slot).map(|input| (input, pointer, true))
                }));
            }
            _ if pointer => return true,
            _ => (),
        }
    }
    false
}

fn walk(fd: &Funcdata, value: VarnodeId, budget: &mut usize) -> Option<FrameAddress> {
    *budget = budget.checked_sub(1)?;
    let vn = fd.vbank().get(value)?;
    if vn.is_spacebase() && vn.is_input() {
        return Some(FrameAddress {
            pointer_size: vn.get_size(),
            offset: 0,
            first: 0,
            last: 0,
            terms: Vec::new(),
        });
    }
    let id = vn.get_def()?;
    let op = fd.obank().get(id)?;
    let input = op.get_in(0)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => (fd.vbank().get(input)?.get_size()
            == vn.get_size())
        .then(|| walk(fd, input, budget))
        .flatten(),
        OpCode::CPUI_PTRADD => {
            let scale = fd.vbank().get(op.get_in(2)?)?;
            if !scale.is_constant() {
                return None;
            }
            let mut base = walk(fd, input, budget)?;
            base.add(
                fd,
                op.get_in(1)?,
                signed_constant(scale.get_offset(), base.pointer_size),
                id,
            )?;
            Some(base)
        }
        OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_SUB => {
            let mut base = walk(fd, input, budget)?;
            base.add(
                fd,
                op.get_in(1)?,
                if op.code() == OpCode::CPUI_INT_SUB {
                    -1
                } else {
                    1
                },
                id,
            )?;
            Some(base)
        }
        OpCode::CPUI_INT_ADD => {
            let other = op.get_in(1)?;
            let (mut base, index) = if let Some(base) = walk(fd, input, budget) {
                (base, other)
            } else {
                (walk(fd, other, budget)?, input)
            };
            base.add(fd, index, 1, id)?;
            Some(base)
        }
        _ => None,
    }
}
