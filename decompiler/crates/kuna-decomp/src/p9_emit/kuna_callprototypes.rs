//! C declarations for named calls with complete locked prototypes.
use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::{architecture::Architecture, funcdata::Funcdata};
use kuna_num::opcodes::OpCode;
use std::{collections::BTreeSet, rc::Rc};

pub fn declarations(arch: &Architecture, fd: &Funcdata) -> Vec<String> {
    let mut declarations = BTreeSet::new();
    let rt = crate::printc::RealTypeCtx::from_arch(arch, arch.print().out_lang());
    for index in 0..fd.num_calls() {
        let call = fd.get_call_specs(index);
        let Some(op) = fd.obank().get(call.get_op()) else {
            continue;
        };
        if op.is_dead() || op.code() != OpCode::CPUI_CALL {
            continue;
        }
        let proto = call.proto();
        if !proto.has_store() || !proto.is_input_locked() || !proto.is_output_locked() {
            continue;
        }
        let name = call.fspec_printed_name(arch.kuna_name_style());
        let mut bytes = name.bytes();
        if !bytes
            .next()
            .is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
            || !bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            continue;
        }
        let mut snapshot = crate::fspec::FuncProto::new();
        snapshot.copy(proto);
        let mut code = Datatype::new(1, type_metatype::TYPE_CODE);
        code.kind = DatatypeKind::Code {
            proto: Some(Rc::new(snapshot)),
        };
        declarations.insert(format!(
            "{};",
            crate::printc::declaration_text(&Rc::new(code), &name, rt)
        ));
    }
    declarations.into_iter().collect()
}
