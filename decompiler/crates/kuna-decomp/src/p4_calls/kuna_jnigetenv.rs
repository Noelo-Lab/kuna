//! Specialize the output parameter of JavaVM.GetEnv for the requested JNI version.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncProto, PrototypePieces};
use crate::funcdata::Funcdata;

fn is_get_env(data: &Funcdata, mut target: VarnodeId) -> Option<bool> {
    for _ in 0..8 {
        let def = data.obank().get(data.vbank().get(target)?.get_def()?)?;
        match def.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => target = def.get_in(0)?,
            OpCode::CPUI_LOAD => {
                let field = data.vbank().get(def.get_in(1)?)?.get_def()?;
                let add = data.obank().get(field)?;
                if !matches!(add.code(), OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD) {
                    return None;
                }
                let table = data
                    .vn_type_read_facing(add.get_in(0)?, field)
                    .get_ptr_to()?;
                if table.get_name() != "JNIInvokeInterface" {
                    return None;
                }
                let offset = data.vbank().get(add.get_in(1)?)?;
                let get_env = (0..)
                    .map_while(|i| table.get_field(i))
                    .find(|f| f.name == "GetEnv")?;
                return Some(offset.constant_match(get_env.offset as u64));
            }
            _ => break,
        }
    }
    None
}

fn jni_version(data: &Funcdata, version: Option<VarnodeId>) -> bool {
    version
        .and_then(|vn| super::kuna_varargforward::constant_value(data, vn, 0))
        .is_some_and(|(value, _)| matches!(value, 0x10001 | 0x10002 | 0x10004 | 0x10006))
}

/// Type recovery can give one address-taken local a single type across its lifetimes.
fn mixed_output(data: &Funcdata, call: OpId, output: VarnodeId) -> bool {
    (0..data.num_calls()).any(|index| {
        let other = data.get_call_specs(index).get_op();
        if other == call {
            return false;
        }
        let Some(op) = data.obank().get(other) else {
            return false;
        };
        let Some(out) = op.get_in(2) else {
            return false;
        };
        op.code() == OpCode::CPUI_CALLIND
            && !jni_version(data, op.get_in(3))
            && crate::expression::functional_equality(output, out, data.vbank(), data.obank())
    })
}

pub(crate) fn specialized(data: &Funcdata, call: OpId, proto: &FuncProto) -> Option<Rc<FuncProto>> {
    let arch = data.get_arch();
    if !arch.android_jni || proto.num_params() != 3 {
        return None;
    }
    let vm = proto.get_param(0)?.get_type()?.get_ptr_to()?;
    if vm.get_name() != "JavaVM" {
        return None;
    }
    let op = data.obank().get(call)?;
    if !jni_version(data, op.get_in(3))
        || !is_get_env(data, op.get_in(0)?)?
        || mixed_output(data, call, op.get_in(2)?)
    {
        return None;
    }
    let types = arch.types()?;
    let env = types.find_by_name("JNIEnv").ok()??;
    let pointer = |to| {
        types
            .get_type_pointer(vm.get_size(), to, vm.get_word_size()?)
            .ok()
    };
    let mut pieces = PrototypePieces::default();
    proto.get_pieces(&mut pieces);
    *pieces.intypes.get_mut(1)? = pointer(pointer(env)?)?;
    let mut typed = FuncProto::new();
    typed
        .seed_locked_from_pieces(
            &pieces,
            proto.model().clone(),
            types.get_type_void().ok()?,
            types,
            arch.manage(),
        )
        .ok()?;
    Some(Rc::new(typed))
}
