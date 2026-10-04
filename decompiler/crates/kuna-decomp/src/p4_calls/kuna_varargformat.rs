//! Constant integer-format evidence for variadic trials with competing readers.
//! Uses `varargforward`; unknown formats retain the exclusive-reader rule.
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;

use super::kuna_varargforward::{constant_value, declared_value};
use crate::dtype::type_metatype;
use crate::fspec::{FuncCallSpecs, ParameterPieces, PrototypePieces};
use crate::funcdata::Funcdata;
use crate::funcdata_varnode::AncestorRealistic;

/// Match only ordinary promoted integer conversions. Decline the whole format
/// when another class, positional syntax, or an unsupported length appears.
fn integer_arguments(format: &[u8]) -> Option<Vec<bool>> {
    let mut args = Vec::new();
    let mut i = 0;
    while i < format.len() {
        if format[i] != b'%' {
            i += 1;
            continue;
        }
        i += 1;
        if format.get(i) == Some(&b'%') {
            i += 1;
            continue;
        }
        while format.get(i).is_some_and(|b| b"#0- +'I".contains(b)) {
            i += 1;
        }
        if format.get(i) == Some(&b'*') {
            args.push(false);
            i += 1;
        } else {
            while format.get(i).is_some_and(u8::is_ascii_digit) {
                i += 1;
            }
        }
        if format.get(i) == Some(&b'.') {
            i += 1;
            if format.get(i) == Some(&b'*') {
                args.push(false);
                i += 1;
            } else {
                while format.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
            }
        }
        let narrow = format.get(i) == Some(&b'h');
        if narrow {
            i += 1;
            if format.get(i) == Some(&b'h') {
                i += 1;
            }
        }
        args.push(match *format.get(i)? {
            b'd' | b'i' => false,
            b'c' if !narrow => false,
            b'o' | b'u' | b'x' | b'X' => true,
            _ => return None,
        });
        i += 1;
        if args.len() > 16 {
            return None;
        }
    }
    Some(args)
}

/// Assign the full format's integer arguments through the call's variadic ABI,
/// rather than treating a conversion count as a count of every register class.
pub(crate) fn arguments(data: &Funcdata, fc: &FuncCallSpecs) -> Vec<ParameterPieces> {
    let resolve = || -> Option<Vec<ParameterPieces>> {
        if !data.get_arch().vararg_forward
            || !fc.is_dotdotdot()
            || !fc.proto().is_input_locked()
            || !fc.proto().has_model()
            || fc.get_name().contains("scanf")
        {
            return None;
        }
        let fixed = fc.proto().num_params();
        let format = fc.proto().get_param(fixed.checked_sub(1)?)?;
        let ty = format.get_type()?;
        if !ty
            .get_ptr_to()
            .is_some_and(|t| t.get_size() == 1 && t.is_char_print())
        {
            return None;
        }
        let active = fc.active_input();
        let index = active.which_trial(&format.get_address(), format.get_size());
        if index < 0 {
            return None;
        }
        let call = data.obank().get(fc.get_op())?;
        let vn = call.get_in(active.get_trial(index).get_slot())?;
        let (value, width) = constant_value(data, vn, 0)?;
        let format = super::kuna_formatwitness::pointed_readonly_format(
            data,
            value,
            width,
            call.get_addr(),
        )?;
        let args = integer_arguments(&format)?;
        if args.is_empty() {
            return None;
        }
        let arch = data.get_arch();
        let types = arch.types()?;
        let mut pieces = PrototypePieces::default();
        fc.proto().get_pieces(&mut pieces);
        if pieces.first_var_arg_slot != pieces.intypes.len() as int4 {
            return None;
        }
        for unsigned in args {
            let meta = if unsigned {
                type_metatype::TYPE_UINT
            } else {
                type_metatype::TYPE_INT
            };
            pieces
                .intypes
                .push(types.get_base(types.get_size_of_int(), meta).ok()?);
            pieces.innames.push(String::new());
        }
        let mut res = Vec::new();
        fc.proto()
            .model()
            .assign_parameter_storage(&pieces, &mut res, true, types, arch.manage())
            .ok()?;
        if res.len() != pieces.intypes.len() + 1 {
            return None;
        }
        let args: Vec<_> = res.into_iter().skip(fixed as usize + 1).collect();
        args.iter().all(|a| a.flags == 0).then_some(args)
    };
    resolve().unwrap_or_default()
}

/// Prove the consumed bytes through bounded copies and integer expressions.
/// Realistic movement alone can include a PIECE with undeclared upper bytes.
fn defined_bytes(
    data: &Funcdata,
    vn: VarnodeId,
    offset: int4,
    width: int4,
    depth: u32,
    budget: &mut u32,
) -> bool {
    if depth >= 16 || *budget == 0 {
        return false;
    }
    *budget -= 1;
    let Some(v) = data.vbank().get(vn) else {
        return false;
    };
    let size = v.get_size();
    if offset < 0 || width <= 0 || offset > size - width {
        return false;
    }
    if v.is_constant() {
        return true;
    }
    if v.is_input() {
        let proto = data.get_func_proto();
        return proto.is_input_locked()
            && (0..proto.num_params())
                .filter_map(|i| proto.get_param(i))
                .any(|p| {
                    let (at, ps) = (p.get_address(), p.get_size());
                    at.justified_contain(ps, v.get_addr(), size, false) >= 0
                        || (v.get_addr().justified_contain(size, &at, ps, false) == 0
                            && offset + width <= ps)
                });
    }
    let Some(op) = v.get_def().and_then(|id| data.obank().get(id)) else {
        return false;
    };
    let mut input = |slot, off, len| {
        op.get_in(slot)
            .is_some_and(|id| defined_bytes(data, id, off, len, depth + 1, budget))
    };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => input(0, offset, width),
        OpCode::CPUI_PIECE => {
            let Some(low) = op.get_in(1).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            let low_size = low.get_size();
            let low_width = (low_size - offset).min(width);
            (low_width <= 0 || input(1, offset, low_width))
                && (offset + width <= low_size
                    || input(0, (offset - low_size).max(0), width - low_width.max(0)))
        }
        OpCode::CPUI_SUBPIECE => {
            let Some(skip) = op.get_in(1).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            let Some(source) = op.get_in(0).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            skip.is_constant()
                && skip.get_offset() <= source.get_size() as u64
                && input(0, offset + skip.get_offset() as int4, width)
        }
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            let Some(source) = op.get_in(0).and_then(|id| data.vbank().get(id)) else {
                return false;
            };
            let source_size = source.get_size();
            let source_width = (source_size - offset).min(width);
            (source_width <= 0 || input(0, offset, source_width))
                && (op.code() == OpCode::CPUI_INT_ZEXT
                    || offset + width <= source_size
                    || input(0, source_size - 1, 1))
        }
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).all(|slot| input(slot, offset, width)),
        OpCode::CPUI_LOAD => true,
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => declared_value(data, vn),
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_NEGATE
        | OpCode::CPUI_INT_2COMP
        | OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT => (0..op.num_input()).all(|slot| {
            op.get_in(slot)
                .and_then(|id| data.vbank().get(id))
                .is_some_and(|v| input(slot, 0, v.get_size()))
        }),
        _ => false,
    }
}

/// Keep a defined existing register trial consumed by the format, at the
/// format's promoted width. A clobber or undeclared live-in is not a value.
pub(crate) fn activate_trial(
    data: &mut Funcdata,
    idx: int4,
    i: int4,
    args: &[ParameterPieces],
) -> bool {
    if args.is_empty() {
        return false;
    }
    let fc = data.get_call_specs(idx);
    let trial = fc.active_input().get_trial(i);
    let (addr, size, slot, op) = (
        trial.get_address().clone(),
        trial.get_size(),
        trial.get_slot(),
        fc.get_op(),
    );
    if addr
        .get_space()
        .is_none_or(|s| s.get_type() != spacetype::IPTR_PROCESSOR)
    {
        return false;
    }
    let Some(arg) = args.iter().find(|a| {
        addr.justified_contain(
            size,
            &a.addr,
            a.type_.as_ref().map_or(0, |t| t.get_size()),
            false,
        ) == 0
    }) else {
        return false;
    };
    let Some(ty) = arg.type_.as_ref() else {
        return false;
    };
    let width = ty.get_size();
    if !fc.active_input().test_shrink(i, &arg.addr, width) {
        return false;
    }
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else {
        return false;
    };
    let (cond, killed) = (trial.has_cond_exe_effect(), trial.is_killed_by_call());
    let mut ancestry = None;
    if !defined_bytes(data, vn, 0, width, 0, &mut 64) {
        return false;
    }
    if !data.vbank().get(vn).is_some_and(|v| v.is_input()) && !declared_value(data, vn) {
        let mut ancestor = AncestorRealistic::new();
        let (realistic, solid) = ancestor.execute(data, op, slot, width, cond, killed, false);
        if !(realistic || solid) {
            return false;
        }
        ancestry = Some((ancestor, realistic, solid));
    }
    let active = data.get_call_specs_mut(idx).get_active_input();
    active.shrink(i, arg.addr.clone(), width);
    let trial = active.get_trial_mut(i);
    if let Some((ancestor, realistic, solid)) = ancestry {
        ancestor.apply_trial(trial, realistic, solid);
    } else {
        trial.set_ancestor_realistic();
        trial.set_ancestor_solid();
    }
    trial.mark_active();
    if trial.has_cond_exe_effect() {
        active.mark_needs_final_check();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::integer_arguments;

    #[test]
    fn consumes_promoted_integers_and_star_operands() {
        assert_eq!(
            integer_arguments(b"%i %hhu %% %*.*d"),
            Some(vec![false, true, false, false, false])
        );
        assert_eq!(integer_arguments(b"%08x %c"), Some(vec![true, false]));
        assert_eq!(integer_arguments(b"plain %%"), Some(vec![]));
    }

    #[test]
    fn declines_unknown_widths_classes_and_positions() {
        for format in [
            b"%lld".as_slice(),
            b"%f",
            b"%s",
            b"%n",
            b"%q",
            b"%hc",
            b"%2$d",
            b"%*2$d",
            b"tail %",
            b"%*.*",
        ] {
            assert_eq!(integer_arguments(format), None, "{format:?}");
        }
        assert_eq!(integer_arguments(&b"%i".repeat(17)), None);
    }
}
