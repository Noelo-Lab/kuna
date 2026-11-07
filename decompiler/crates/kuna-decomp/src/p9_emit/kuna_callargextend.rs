//! Match primitive Go and Rust arguments to the emitted callee's integer type.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_lang::OutLang;
use crate::printc::{op_key, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

fn integer(ty: &Datatype) -> bool {
    matches!(ty.get_size(), 1 | 2 | 4 | 8) && !ty.is_enum_type()
        && matches!(ty.get_metatype(), type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
}

fn target(fd: &Funcdata, op: OpId, slot: i32) -> Option<(Rc<Datatype>, bool)> {
    if slot < 1 { return None; }
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_CALL { return None; }
    let value = fd.vbank().get(operation.get_in(slot)?)?;
    let call = fd.get_call_specs(fd.get_call_specs_index(op)?);
    if call.get_entry_address() == fd.get_address()
        || call.final_input_storage().len() + 1 != operation.num_input() as usize
    {
        return None;
    }
    if call.proto().has_store().then(|| call.proto().get_param(slot - 1)).flatten().map_or(
        call.is_input_locked() && !call.is_dotdotdot(), |parameter| parameter.is_type_locked(),
    ) {
        return None;
    }
    let stated = fd.kuna_protoorder_types(call.get_entry_address())?;
    let (address, original_size) = call.final_input_storage().get((slot - 1) as usize)?;
    let (parameter, size, ty) = stated.inputs.get((slot - 1) as usize)?;
    if !integer(ty) || *size != ty.get_size() || *size < value.get_size()
        || !matches!(value.get_size(), 1 | 2 | 4 | 8) || *original_size < value.get_size()
        || address != parameter
    {
        return None;
    }
    let extended = *size > value.get_size();
    if extended && !call.is_zext_trimmed_input(slot, value.get_size(), operation.num_input()) {
        return None;
    }
    let space = address.get_space()?;
    if space.get_type() != spacetype::IPTR_PROCESSOR || space.get_word_size() != 1
        || address.get_offset().checked_add((*size).max(*original_size) as u64 - 1)? > space.get_highest()
    {
        return None;
    }
    Some((Rc::clone(ty), extended))
}

impl PrintC {
    pub(crate) fn push_extended_call_argument(
        &mut self, fd: &Funcdata, arch: &Architecture, value: VarnodeId, op: OpId, slot: i32,
    ) -> bool {
        if !matches!(self.out_lang, OutLang::Go | OutLang::Rust)
            || (self.out_lang == OutLang::Rust && self.options.nocasts)
        {
            return false;
        }
        let Some((target, extended)) = target(fd, op, slot) else { return false; };
        let source = if self.out_lang == OutLang::Go {
            self.go_value_type(fd, arch, value, op)
        } else { self.rust_integer_value_type(fd, arch, value, op) };
        if !integer(&source) { return false; }
        let node = fd.vbank().get(value).unwrap();
        if !extended {
            if source.get_size() != node.get_size() { return false; }
            if self.out_lang == OutLang::Go {
                self.push_go_value_as(fd, arch, value, op, &target);
            } else if !self.push_rust_integer_as(fd, arch, value, op, &target) {
                self.push_vn_ir(fd, arch, value, op);
            }
            return true;
        }
        if node.is_constant() {
            let raw = node.get_offset() & kuna_base::address::calc_mask(node.get_size());
            self.push_atom(&Atom::with_op(raw.to_string(), TagType::Syntax,
                SyntaxHighlight::const_color, op_key(op)));
            return true;
        }
        let Ok(unsigned) = arch.types().get_base(node.get_size(), type_metatype::TYPE_UINT)
            else { return false; };
        self.push_cast_open(&target, op);
        self.push_cast_open(&unsigned, op);
        self.push_vn_ir(fd, arch, value, op);
        self.push_cast_close(&unsigned);
        self.push_cast_close(&target);
        true
    }
}
