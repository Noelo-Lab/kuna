//! The operands of x86 REP MOVS at the entry to its lifted repeat loop.

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::{mem_space, read, Insn, RawOp};
use crate::state::State;
use crate::value::Halt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct CopyOperands {
    pub(crate) dest: u64,
    pub(crate) src: u64,
    pub(crate) bytes: u128,
    pub(crate) backwards: bool,
}

impl std::fmt::Display for CopyOperands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = if self.backwards { "backward" } else { "forward" };
        write!(f, "dest={:#x}, src={:#x}, {} bytes ({direction})", self.dest, self.src, self.bytes)
    }
}

fn in_space(vn: &VarnodeData, ty: spacetype) -> bool {
    vn.space.as_ref().is_some_and(|space| space.get_type() == ty)
}

fn constant(vn: &VarnodeData, value: u64) -> bool {
    in_space(vn, spacetype::IPTR_CONSTANT) && vn.offset == value
}

/// The register whose entry value supplies a memory address. The lift copies
/// the address before advancing the string register.
fn entry_pointer<'a>(ops: &'a [RawOp], index: usize) -> Option<&'a VarnodeData> {
    let pointer = ops[index].ins.get(1)?;
    let (register, before) = if in_space(pointer, spacetype::IPTR_INTERNAL) {
        let defined = ops[..index].iter().rposition(|op| op.out.as_ref() == Some(pointer))?;
        let copy = &ops[defined];
        if copy.opc != OpCode::CPUI_COPY {
            return None;
        }
        (copy.ins.first()?, defined)
    } else {
        (pointer, index)
    };
    if !in_space(register, spacetype::IPTR_PROCESSOR) || register.size != pointer.size {
        return None;
    }
    let overwritten = ops[..before].iter().filter_map(|op| op.out.as_ref()).any(|out| {
        let same_space = out.space.as_ref().zip(register.space.as_ref()).is_some_and(|(a, b)| a.get_index() == b.get_index());
        same_space && out.offset < register.offset + u64::from(register.size) && register.offset < out.offset + u64::from(out.size)
    });
    (!overwritten).then_some(register)
}

fn lift_operands(insn: &Insn, at: u64) -> Option<(&VarnodeData, &VarnodeData, &VarnodeData, u32)> {
    let [test, guard, decrement, .., back] = insn.ops.as_slice() else { return None };
    let [count, zero] = test.ins.as_slice() else { return None };
    let [_, test_result] = guard.ins.as_slice() else { return None };
    let [previous_count, one] = decrement.ins.as_slice() else { return None };
    if test.opc != OpCode::CPUI_INT_EQUAL || !constant(zero, 0) || !in_space(count, spacetype::IPTR_PROCESSOR)
        || guard.opc != OpCode::CPUI_CBRANCH || test.out.as_ref() != Some(test_result)
        || decrement.opc != OpCode::CPUI_INT_SUB || decrement.out.as_ref() != Some(count) || previous_count != count || !constant(one, 1)
        || back.opc != OpCode::CPUI_BRANCH || !back.ins.first().is_some_and(|target| target.offset == at && !in_space(target, spacetype::IPTR_CONSTANT))
    {
        return None;
    }
    let mut memory = insn.ops.iter().enumerate().filter(|(_, op)| matches!(op.opc, OpCode::CPUI_LOAD | OpCode::CPUI_STORE));
    let [(src_index, load), (dest_index, store)] = [memory.next()?, memory.next()?];
    if load.opc != OpCode::CPUI_LOAD || store.opc != OpCode::CPUI_STORE || memory.next().is_some()
        || load.ins.first() != store.ins.first()
    {
        return None;
    }
    let loaded = load.out.as_ref()?;
    let stored = store.ins.get(2)?;
    let copies_load = insn.ops[src_index + 1..dest_index].iter().any(|op| {
        op.opc == OpCode::CPUI_COPY && op.ins.first() == Some(loaded) && op.out.as_ref() == Some(stored)
    });
    if !matches!(loaded.size, 1 | 2 | 4 | 8) || stored.size != loaded.size || (loaded != stored && !copies_load) {
        return None;
    }
    Some((entry_pointer(&insn.ops, dest_index)?, entry_pointer(&insn.ops, src_index)?, count, loaded.size))
}

pub(super) fn operands(state: &mut State, at: u64, insn: &Insn) -> Result<CopyOperands, Halt> {
    let env = state.env;
    let unsupported = || Halt::Unsupported(format!("cannot read REP MOVS operands at {at:#x}"));
    if !env.prog.arch().archid.starts_with("x86") {
        return Err(unsupported());
    }
    let (dest, src, count, width) = lift_operands(insn, at).ok_or_else(unsupported)?;
    let load = insn.ops.iter().find(|op| op.opc == OpCode::CPUI_LOAD).ok_or_else(unsupported)?;
    if mem_space(env, &load.ins[0])?.get_index() != env.ram.get_index() {
        return Err(unsupported());
    }
    let df = env.prog.arch().translate().get_register_data("DF").map_err(|_| unsupported())?;
    Ok(CopyOperands {
        dest: env.ram.wrap_offset(read(state, dest)? as u64),
        src: env.ram.wrap_offset(read(state, src)? as u64),
        bytes: read(state, count)? * u128::from(width),
        backwards: read(state, &df)? != 0,
    })
}
