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
    walk(fd, value, &mut 64)
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
