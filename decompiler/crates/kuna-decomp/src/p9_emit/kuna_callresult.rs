//! Preserve low scalar return storage when a recovered callee prints a wider integer.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_lang::OutLang;
use crate::printc::{declarator_parts, op_key, tokens, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};
use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

fn integer(ty: &Datatype) -> bool {
    matches!(ty.get_size(), 1 | 2 | 4 | 8) && !ty.is_enum_type()
        && matches!(ty.get_metatype(), type_metatype::TYPE_INT
            | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
}

fn low_storage(source: &Address, source_size: i32, target: &Address, target_size: i32) -> bool {
    let (Some(space), Some(target_space)) = (source.get_space(), target.get_space())
        else { return false; };
    if source_size <= target_size || target_size <= 0
        || space.get_type() != spacetype::IPTR_PROCESSOR
        || target_space.get_type() != spacetype::IPTR_PROCESSOR
        || space.get_index() != target_space.get_index() || space.get_word_size() != 1
    {
        return false;
    }
    let start = source.get_offset();
    let Some(end) = start.checked_add(source_size as u64 - 1) else { return false; };
    if end > space.get_highest() { return false; }
    let low = if space.is_big_endian() { end - target_size as u64 + 1 } else { start };
    target.get_offset() == low
}

pub(crate) struct CallResultPlan {
    pub target: Rc<Datatype>,
    cast_type: Rc<Datatype>,
    boolean: bool,
}

impl PrintC {
    pub(crate) fn call_result_plan(
        &self, fd: &Funcdata, arch: &Architecture, op: OpId,
    ) -> Option<CallResultPlan> {
        if !matches!(self.out_lang, OutLang::Go | OutLang::Rust)
            || (self.out_lang == OutLang::Rust && self.options.nocasts)
        {
            return None;
        }
        let operation = fd.obank().get(op)?;
        if operation.code() != OpCode::CPUI_CALL { return None; }
        let output = operation.get_out()?;
        let node = fd.vbank().get(output)?;
        let call = fd.get_call_specs(fd.get_call_specs_index(op)?);
        if call.proto().is_output_locked() || call.get_entry_address() == fd.get_address() {
            return None;
        }
        let entry = call.get_entry_address();
        let (source_addr, source_size, source_type) = if let Some(stated) =
            fd.kuna_callret_stated((entry.get_space()?.get_index(), entry.get_offset()))
        {
            (&stated.addr, stated.size, &stated.ct)
        } else {
            let (addr, size, ty) = fd.kuna_protoorder_types(entry)?.output.as_ref()?;
            (addr, *size, ty)
        };
        if !integer(source_type) || source_size != source_type.get_size()
            || !low_storage(source_addr, source_size, node.get_addr(), node.get_size())
        {
            return None;
        }
        if let Some(proto_type) = call.proto().has_store().then(|| call.proto().get_output_type()).flatten() {
            if proto_type.get_metatype() != type_metatype::TYPE_VOID
                && (!integer(proto_type) || declarator_parts(proto_type, self.rt_ctx.clone())
                    != declarator_parts(source_type, self.rt_ctx.clone()))
            {
                return None;
            }
        }
        let target = if self.out_lang == OutLang::Rust {
            self.rust_integer_output_type(fd, arch, output)
        } else if node.is_implied() {
            fd.vn_type_def_facing(output)
        } else {
            self.go_value_type(fd, arch, output, op)
        };
        let boolean = target.get_metatype() == type_metatype::TYPE_BOOL && target.get_size() == 1;
        if target.get_size() != node.get_size() || (!boolean && !integer(&target)) {
            return None;
        }
        let cast_type = if boolean {
            arch.types().get_base(1, type_metatype::TYPE_UINT).ok()?
        } else { Rc::clone(&target) };
        Some(CallResultPlan { target, cast_type, boolean })
    }

    pub(crate) fn push_call_result_open(
        &mut self, fd: &Funcdata, arch: &Architecture, op: OpId,
    ) -> Option<CallResultPlan> {
        let plan = self.call_result_plan(fd, arch, op)?;
        if plan.boolean {
            if self.out_lang == OutLang::Rust {
                self.push_op(&tokens::NOT_EQUAL, Some(op_key(op)));
            } else {
                self.push_go_conversion(&plan.target, &plan.cast_type, op);
            }
        }
        self.push_cast_open(&plan.cast_type, op);
        Some(plan)
    }

    pub(crate) fn push_call_result_close(&mut self, plan: Option<CallResultPlan>, op: OpId) {
        let Some(plan) = plan else { return; };
        self.push_cast_close(&plan.cast_type);
        if plan.boolean && self.out_lang == OutLang::Rust {
            self.push_atom(&Atom::with_op(
                "0", TagType::Syntax, SyntaxHighlight::const_color, op_key(op),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::AddrSpace;

    fn space(big: bool, index: i32, width: u32, word: u32) -> Rc<AddrSpace> {
        Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", big, width, word, index, 0, 1, 1))
    }

    #[test]
    fn narrowing_requires_the_low_storage_slice_in_either_byte_order() {
        for big in [false, true] {
            let register = space(big, 2, 8, 1);
            for source_size in [2, 4, 8] {
                for target_size in [1, 2, 4].into_iter().filter(|size| *size < source_size) {
                    let source = Address::new(register.clone(), 0x100);
                    for offset in 0..source_size {
                        let target = Address::new(register.clone(), 0x100 + offset as u64);
                        assert_eq!(low_storage(&source, source_size, &target, target_size),
                            offset == if big { source_size - target_size } else { 0 });
                    }
                }
            }
        }
    }

    #[test]
    fn ambiguous_storage_and_wrapped_ranges_are_rejected() {
        let register = space(false, 2, 8, 1);
        let source = Address::new(register.clone(), 0x100);
        assert!(!low_storage(&source, 8, &Address::new(space(false, 3, 8, 1), 0x100), 1));
        for size in [0, 8, 16] { assert!(!low_storage(&source, 8, &source, size)); }
        let wrapped = Address::new(register, u64::MAX - 3);
        assert!(!low_storage(&wrapped, 8, &wrapped, 1));
        let limited = Address::new(space(false, 2, 1, 1), 0xfc);
        assert!(!low_storage(&limited, 8, &limited, 1));
        let word = Address::new(space(false, 2, 8, 2), 0x100);
        assert!(!low_storage(&word, 8, &word, 1));
        for kind in [spacetype::IPTR_INTERNAL, spacetype::IPTR_JOIN, spacetype::IPTR_SPACEBASE] {
            let address = Address::new(Rc::new(AddrSpace::new(kind, "other", false, 8, 1, 2, 0, 1, 1)), 0x100);
            assert!(!low_storage(&address, 8, &address, 1));
        }
    }
}
