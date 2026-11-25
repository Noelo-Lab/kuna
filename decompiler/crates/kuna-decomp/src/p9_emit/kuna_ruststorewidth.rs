//! Keep Rust STOREs the width of the physical access.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype::{
    TYPE_BOOL, TYPE_INT, TYPE_PTR, TYPE_STRUCT, TYPE_UINT, TYPE_UNKNOWN,
};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::kuna_rustarrayaddr::{array_place, RAW_ADDRESS};
use crate::kuna_rusttypes::int_name;
use crate::prettyprint::Emit;
use crate::printc::{op_key, tokens, PrintC};
use crate::printlanguage::{modifiers, Atom, SyntaxHighlight, TagType};
use kuna_base::address::{calc_mask, Address};
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

fn integer(ty: &Datatype) -> Option<&'static str> {
    if ty.is_enum_type() {
        return None;
    }
    let signed = match ty.get_metatype() {
        TYPE_INT => !(ty.is_char_print() && ty.is_core_type()),
        TYPE_UINT | TYPE_UNKNOWN => false,
        _ => return None,
    };
    int_name(ty.get_size(), signed)
}

fn ordinary_pointer(ty: &Datatype, size: i32) -> bool {
    ty.get_metatype() == TYPE_PTR
        && ty.get_size() == size
        && ty.get_word_size() == Some(1)
        && !ty.is_formal_pointer_rel()
}

fn exact_field(fd: &Funcdata, def: OpId) -> bool {
    let Some(operation) = fd.obank().get(def) else {
        return false;
    };
    let Some(base) = operation.get_in(0) else {
        return false;
    };
    let Some(offset) = operation
        .get_in(1)
        .and_then(|id| fd.vbank().get(id))
        .filter(|node| node.is_constant())
    else {
        return false;
    };
    let ty = fd.vn_type_read_facing(base, def);
    if !ordinary_pointer(&ty, ty.get_size()) {
        return false;
    }
    let Some(owner) = ty
        .get_ptr_to()
        .filter(|ty| ty.get_metatype() == TYPE_STRUCT)
    else {
        return false;
    };
    let offset = offset.get_offset();
    offset < owner.get_size().max(0) as u64
        && owner
            .find_truncation(offset as i64, 0, def, 0)
            .ok()
            .flatten()
            .filter(|(_, remainder)| *remainder == 0)
            .and_then(|(field, _)| owner.get_field(field))
            .is_some_and(|field| !crate::kuna_typequal::is_volatile(&field.field_type))
}

#[derive(Clone, Copy)]
enum AddressForm {
    Ordinary,
    Array,
    Field,
    Index,
    UnsupportedPlace,
}

fn address_form(fd: &Funcdata, address: VarnodeId) -> AddressForm {
    let Some(def) = fd
        .vbank()
        .get(address)
        .filter(|node| node.is_implied())
        .and_then(|node| node.get_def())
    else {
        return AddressForm::Ordinary;
    };
    match fd.obank().get(def).map(|operation| operation.code()) {
        Some(OpCode::CPUI_PTRSUB) if array_place(fd, address) => AddressForm::Array,
        Some(OpCode::CPUI_PTRSUB) if exact_field(fd, def) => AddressForm::Field,
        Some(OpCode::CPUI_PTRADD) => AddressForm::Index,
        Some(OpCode::CPUI_PTRSUB | OpCode::CPUI_SEGMENTOP) => AddressForm::UnsupportedPlace,
        _ => AddressForm::Ordinary,
    }
}

impl PrintC {
    fn store_address(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        mut address: VarnodeId,
        mut reader: OpId,
        size: i32,
    ) -> (VarnodeId, OpId) {
        for _ in 0..8 {
            let Some(def) = fd
                .vbank()
                .get(address)
                .filter(|node| node.is_implied())
                .and_then(|node| node.get_def())
            else {
                break;
            };
            let Some(operation) = fd
                .obank()
                .get(def)
                .filter(|op| matches!(op.code(), OpCode::CPUI_CAST | OpCode::CPUI_COPY))
            else {
                break;
            };
            let Some(input) = operation.get_in(0) else {
                break;
            };
            if !fd
                .vbank()
                .get(input)
                .is_some_and(|node| node.get_size() == size && !node.is_constant())
                || !ordinary_pointer(&self.rust_integer_value_type(fd, arch, input, def), size)
                || !ordinary_pointer(
                    &self.rust_integer_value_type(fd, arch, address, reader),
                    size,
                )
            {
                break;
            }
            address = input;
            reader = def;
        }
        (address, reader)
    }

    fn push_store_address_operand(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        address: VarnodeId,
        reader: OpId,
        form: AddressForm,
        mods: u32,
    ) {
        if matches!(form, AddressForm::Array | AddressForm::Field) {
            self.push_op(&RAW_ADDRESS, Some(op_key(reader)));
        }
        let mods = match form {
            AddressForm::Field => mods | modifiers::PRINT_STORE_VALUE,
            AddressForm::Index => mods | modifiers::RUST_RAW_INDEX_ADDRESS,
            _ => mods,
        };
        self.push_vn_ir_m(fd, arch, address, reader, mods);
    }

    pub(crate) fn op_rust_store_width(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> bool {
        let Some(operation) = fd.obank().get(op) else {
            return false;
        };
        let (Some(space), Some(pointer), Some(value)) = (
            operation.get_in(0),
            operation.get_in(1),
            operation.get_in(2),
        ) else {
            return false;
        };
        let Some(node) = fd.vbank().get(value) else {
            return false;
        };
        let width = node.get_size();
        let Some(store_type) = int_name(width, false) else {
            return false;
        };
        let Some(address_node) = fd.vbank().get(pointer).filter(|node| !node.is_volatile()) else {
            return false;
        };
        let size = address_node.get_size();
        if !matches!(size, 4 | 8) {
            return false;
        }
        let source = self.rust_integer_value_type(fd, arch, value, op);
        let destination = self.rust_integer_value_type(fd, arch, pointer, op);
        let (address, reader) = self.store_address(fd, arch, pointer, op, size);
        let pointee = destination.get_ptr_to();
        let base = if address == pointer {
            pointee.clone()
        } else {
            self.rust_integer_value_type(fd, arch, address, reader)
                .get_ptr_to()
        };
        let pointer_width = pointee.as_ref().map(|ty| ty.get_size());
        let base_width = base.as_ref().map(|ty| ty.get_size());
        if source.get_size() == width && pointer_width == Some(width) && base_width == Some(width) {
            return false;
        }
        if !(ordinary_pointer(&destination, size)
            || (destination.get_size() == size && integer(&destination).is_some()))
        {
            return false;
        }
        if [pointee.as_deref(), base.as_deref()]
            .into_iter()
            .flatten()
            .any(crate::kuna_typequal::is_volatile)
        {
            return false;
        }
        let Some(space) = fd
            .vbank()
            .get(space)
            .filter(|node| node.is_constant())
            .and_then(|node| i32::try_from(node.get_offset()).ok())
            .and_then(|index| fd.get_arch().manage().get_space(index))
            .filter(|space| {
                space.get_word_size() == 1
                    && space.get_type() == spacetype::IPTR_PROCESSOR
                    && space.get_addr_size() == size as u32
            })
        else {
            return false;
        };
        if address_node.is_constant() {
            let address = Address::new(space.clone(), address_node.get_offset());
            let properties = fd.query_local_properties(&address, width, operation.get_addr())
                | fd.get_arch()
                    .query_global_properties(&address, width, operation.get_addr());
            if properties & crate::varnode::varnode_flags::volatil != 0 {
                return false;
            }
        }
        let form = address_form(fd, address);
        let source_int = integer(&source);
        let source_size = source.get_size();
        let source_pointer = ordinary_pointer(&source, source_size) && matches!(source_size, 4 | 8);
        let (source_value, source_reader) = if source_pointer {
            self.store_address(fd, arch, value, op, source_size)
        } else {
            (value, op)
        };
        let source_form = if source_pointer {
            address_form(fd, source_value)
        } else {
            AddressForm::Ordinary
        };
        if matches!(form, AddressForm::UnsupportedPlace)
            || matches!(source_form, AddressForm::UnsupportedPlace)
            || !(source_int.is_some() || source.get_metatype() == TYPE_BOOL || source_pointer)
        {
            self.emit_mut().print(
                &format!("/* STORE writes {width} bytes; no scalar conversion applied */ "),
                crate::prettyprint::SyntaxHighlight::CommentColor,
            );
            return false;
        }

        let mods =
            self.context.mods() & !(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let key = Some(op_key(op));
        self.push_op(&tokens::FUNCTION_CALL, key);
        self.push_atom(&Atom::with_op(
            format!("(|value: {store_type}, pointer: *mut {store_type}| unsafe {{ ::core::ptr::write_unaligned(pointer, value) }})"),
            TagType::Syntax, SyntaxHighlight::no_color, op_key(op),
        ));
        self.push_op(&tokens::COMMA, key);
        if node.is_constant() {
            self.push_atom(&Atom::with_op(
                format!("{:#x}", node.get_offset() & calc_mask(width)),
                TagType::Syntax,
                SyntaxHighlight::const_color,
                op_key(op),
            ));
        } else {
            let convert = source_int != Some(store_type);
            if convert {
                self.push_op(self.lang().tok_typecast, key);
            }
            if source_pointer {
                self.push_op(self.lang().tok_typecast, key);
            }
            if let Some(source_int) = source_int.filter(|_| {
                convert
                    && node.is_implied()
                    && crate::kuna_rustintegers::needs_integer_context(fd, value, op, 8)
                        != Some(false)
            }) {
                self.push_op(&tokens::FUNCTION_CALL, key);
                self.push_atom(&Atom::with_op(
                    format!("::core::convert::identity::<{source_int}>"),
                    TagType::OpToken,
                    SyntaxHighlight::funcname_color,
                    op_key(op),
                ));
            }
            if source_pointer {
                self.push_store_address_operand(
                    fd,
                    arch,
                    source_value,
                    source_reader,
                    source_form,
                    mods,
                );
                self.push_atom(&Atom::syntax(
                    "usize",
                    TagType::TypeToken,
                    SyntaxHighlight::type_color,
                ));
            } else {
                self.push_vn_ir_m(fd, arch, value, op, mods);
            }
            if convert {
                self.push_atom(&Atom::syntax(
                    store_type,
                    TagType::TypeToken,
                    SyntaxHighlight::type_color,
                ));
            }
        }
        self.push_op(self.lang().tok_typecast, key);
        if let Some(node) = fd.vbank().get(address).filter(|node| node.is_constant()) {
            let raw = node.get_offset() & calc_mask(size);
            self.push_atom(&Atom::with_op(
                format!("{raw:#x}usize"),
                TagType::Syntax,
                SyntaxHighlight::const_color,
                op_key(op),
            ));
        } else {
            self.push_store_address_operand(fd, arch, address, reader, form, mods);
        }
        self.push_atom(&Atom::syntax(
            format!("*mut {store_type}"),
            TagType::TypeToken,
            SyntaxHighlight::type_color,
        ));
        true
    }
}
