//! A variadic call can pass an argument in the register its own value comes
//! back in.  `double vr(int n, ...)` called as `vr(k, k + 1, 0.5)` passes the
//! `0.5` in `xmm0` on x86-64, `d0` on AArch64 and `f1` on 32-bit PowerPC, the
//! registers the `double` returns in.  Upstream's call guard skips a heritaged
//! range the call's output covers exactly, so the argument trial for it was never
//! registered and the value was dropped.
//!
//! The trial is registered when the call's model would put the first variadic
//! argument of the range's class (a floating-point value in a floating-point
//! entry, an integer otherwise) exactly there.  The model's own `<varargs>` rules
//! keep the floating-point registers out on ARM, RISC-V, MIPS, Windows and Apple
//! arm64, where a variadic value travels in integer registers or on the stack.
//! Three ABIs the model does not describe are handled here. 64-bit PowerPC
//! passes a variadic floating-point value in general registers and the FPR copy
//! is only a shadow, so its FPRs are refused. x86-64 SysV counts the vector
//! registers that carry arguments in `al`, and 32-bit PowerPC sets CR bit 6 when
//! any FPR does: a count of zero set in the call's block refuses the register,
//! and a count of `n` makes the first `n` floating-point argument registers
//! arguments whatever their value, a result of another call included.
//! [`counted_float_entry`] names such a register for heritage when the range
//! being guarded is the whole `xmm` register or a lane of it.
use crate::{
    context::OpId,
    dtype::{type_class, type_metatype},
    fspec::{FuncCallSpecs, ParameterPieces, PrototypePieces},
    funcdata::Funcdata,
    infra::architecture::Architecture,
};
use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

/// How an image passes a floating-point value in the variadic part of a call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VarargFloats {
    /// Where the prototype model's rules put it.
    #[default]
    InModel,
    /// Where the model puts it, with the number of vector registers used in `al`
    /// (x86-64 SysV).
    VectorCount,
    /// Where the model puts it, with CR bit 6 set when any FPR is used (32-bit
    /// PowerPC SysV).
    ConditionBit,
    /// In general registers or on the stack; a floating-point register holds at
    /// most a copy (64-bit PowerPC).
    Shadowed,
}

/// The image's [`VarargFloats`], from its language id.
pub fn image_vararg_floats(arch: &Architecture) -> VarargFloats {
    let id = arch.archid.as_str();
    if id.starts_with("x86:LE:64:") && !id.ends_with(":windows") {
        VarargFloats::VectorCount
    } else if id.starts_with("PowerPC:") && id.split(':').nth(2) == Some("64") {
        VarargFloats::Shadowed
    } else if id.starts_with("PowerPC:") && id.split(':').nth(2) == Some("32") {
        VarargFloats::ConditionBit
    } else {
        VarargFloats::InModel
    }
}

/// Is the heritaged range `[addr, addr + size)`, which the variadic call `fc`'s
/// output covers exactly, also where the call passes its first variadic argument
/// of that class?
pub fn argument_in_own_output(
    fd: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> bool {
    if !fc.is_dotdotdot() || !fc.proto().has_model() {
        return false;
    }
    let arch = fd.get_arch();
    let Some(types) = arch.types() else {
        return false;
    };
    let model = fc.proto().model();
    let float = model.input().get_entry().iter().any(|e| {
        e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
    });
    if float {
        if size < 8 || arch.vararg_floats == VarargFloats::Shadowed {
            return false;
        }
        if float_register_count(fd, fc.get_op()) == Some(0) {
            return false;
        }
    }
    let metatype = if float {
        type_metatype::TYPE_FLOAT
    } else {
        type_metatype::TYPE_INT
    };
    let Ok(vararg) = types.get_base(size, metatype) else {
        return false;
    };
    let mut pieces = PrototypePieces::default();
    fc.proto().get_pieces(&mut pieces);
    if pieces.outtype.is_none() || pieces.first_var_arg_slot != pieces.intypes.len() as int4 {
        return false;
    }
    pieces.intypes.push(vararg);
    pieces.innames.push(String::new());
    let mut res: Vec<ParameterPieces> = Vec::new();
    if model
        .assign_parameter_storage(&pieces, &mut res, true, types, arch.manage())
        .is_err()
    {
        return false;
    }
    res.last().is_some_and(|p| p.addr == *addr)
}

/// The floating-point argument registers of the variadic call `fc` its caller
/// says it filled: the first `n` floating-point entries of the model, for `n`
/// from [`float_register_count`].
pub fn counted_float_arguments(fd: &Funcdata, fc: &FuncCallSpecs) -> Vec<Address> {
    if !fc.is_dotdotdot() || !fc.proto().has_model() {
        return Vec::new();
    }
    let Some(count) = float_register_count(fd, fc.get_op()) else {
        return Vec::new();
    };
    fc.proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 8)
        .take(count.min(8) as usize)
        .map(|e| Address::new(Rc::clone(e.get_space()), e.get_base()))
        .collect()
}

/// The register of [`counted_float_arguments`] that the heritaged register
/// range `[addr, addr + size)` overlaps without being it: the whole 16-byte
/// `xmm` register `pxor`, `movq` or `cvtsi2sd` after `pxor` writes, or the
/// 4-byte lane of it a `movaps` copies.
pub fn counted_float_entry(
    fd: &Funcdata,
    fc: &FuncCallSpecs,
    addr: &Address,
    size: int4,
) -> Option<Address> {
    let space = addr.get_space()?;
    if space.get_type() != spacetype::IPTR_PROCESSOR
        || !fc.is_dotdotdot()
        || !fc.proto().has_model()
    {
        return None;
    }
    let (start, end) = (addr.get_offset(), addr.get_offset() + size as u64);
    let overlaps = |e: &Address| {
        e.get_space().is_some_and(|s| s.get_index() == space.get_index())
            && e.get_offset() < end
            && start < e.get_offset() + 8
            && !(e.get_offset() == start && size == 8)
    };
    let model_float = fc.proto().model().input().get_entry().iter().any(|e| {
        e.get_type() == type_class::TYPECLASS_FLOAT
            && e.get_size() == 8
            && overlaps(&Address::new(Rc::clone(e.get_space()), e.get_base()))
    });
    if !model_float {
        return None;
    }
    counted_float_arguments(fd, fc).into_iter().find(overlaps)
}

/// How many floating-point argument registers the caller says it filled for
/// `call`, when its block states it: `al` on x86-64 SysV, and 1 or 0 for
/// `crset 6` or `crclr 6` on 32-bit PowerPC, where the bit says only whether
/// any is filled.
pub fn float_register_count(fd: &Funcdata, call: OpId) -> Option<u64> {
    match fd.get_arch().vararg_floats {
        VarargFloats::VectorCount => vector_count(fd, call),
        VarargFloats::ConditionBit => condition_bit(fd, call),
        _ => None,
    }
}

/// CR bit 6 as the last instruction of the call's block that writes `cr1`
/// before `call` sets it: `creqv 6,6,6` (`crset 6`) or `crxor 6,6,6`
/// (`crclr 6`).
fn condition_bit(fd: &Funcdata, call: OpId) -> Option<u64> {
    let cr1 = fd
        .get_arch()
        .manage()
        .register_lookup()?
        .probe_register("cr1")?;
    let space = cr1.space?;
    let mut cur = fd.op_previous_op(call);
    while let Some(op) = cur {
        let o = fd.obank().get(op)?;
        if o.is_call() || o.code() == OpCode::CPUI_MULTIEQUAL {
            return None;
        }
        let writes = o.code() != OpCode::CPUI_INDIRECT
            && o.get_out()
                .and_then(|v| fd.vbank().get(v))
                .is_some_and(|v| {
                    v.get_space().get_index() == space.get_index()
                        && v.get_offset() <= cr1.offset
                        && cr1.offset < v.get_offset() + v.get_size() as u64
                });
        if writes {
            let at = o.get_addr();
            let mut word = [0u8; 4];
            fd.get_arch().loader_fill(&mut word, at).ok()?;
            let word = if at.is_big_endian() {
                u32::from_be_bytes(word)
            } else {
                u32::from_le_bytes(word)
            };
            return match word {
                0x4cc6_3242 => Some(1),
                0x4cc6_3182 => Some(0),
                _ => None,
            };
        }
        cur = fd.op_previous_op(op);
    }
    None
}

/// The value of `al` the call's block sets before `call`, when it is a constant:
/// `xor %eax,%eax`, `mov $n,%eax` or `mov $n,%al`.
fn vector_count(fd: &Funcdata, call: OpId) -> Option<u64> {
    let al = fd
        .get_arch()
        .manage()
        .register_lookup()?
        .probe_register("AL")?;
    let space = al.space?;
    let mut want = al.offset;
    let mut cur = fd.op_previous_op(call);
    while let Some(op) = cur {
        let o = fd.obank().get(op)?;
        if o.is_call() || o.code() == OpCode::CPUI_MULTIEQUAL {
            return None;
        }
        let out = o.get_out().and_then(|v| fd.vbank().get(v)).filter(|v| {
            o.code() != OpCode::CPUI_INDIRECT && v.get_space().get_index() == space.get_index()
        });
        if let Some(out) =
            out.filter(|v| v.get_offset() <= want && want < v.get_offset() + v.get_size() as u64)
        {
            if out.get_offset() != want {
                return None;
            }
            let input = |slot: int4| o.get_in(slot).and_then(|v| fd.vbank().get(v));
            let slot = match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => 0,
                OpCode::CPUI_PIECE => 1,
                OpCode::CPUI_SUBPIECE if input(1)?.get_offset() == 0 => 0,
                OpCode::CPUI_INT_XOR => {
                    let (a, b) = (input(0)?, input(1)?);
                    let same = a.get_space().get_index() == b.get_space().get_index()
                        && a.get_offset() == b.get_offset()
                        && a.get_size() == b.get_size();
                    return same.then_some(0);
                }
                _ => return None,
            };
            let src = input(slot)?;
            if src.is_constant() {
                return Some(src.get_offset() & 0xff);
            }
            if src.get_space().get_index() != space.get_index() {
                return None;
            }
            want = src.get_offset();
        }
        cur = fd.op_previous_op(op);
    }
    None
}
