//! Patterns for a single use of a complete, typed Rust Option scalar-pair result.

use crate::architecture::Architecture;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::kuna_lang::OutLang;
use crate::prettyprint::{Emit, SyntaxHighlight};
use crate::printc::PrintC;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

const OPTION_U64: &str = "core::option::Option<u64>";

fn single_reader(fd: &Funcdata, value: VarnodeId, reader: OpId) -> bool {
    fd.vbank().get(value).is_some_and(|node| {
        node.is_implied()
            && !node.is_volatile()
            && !node.is_persist()
            && node.num_descend() == 1
            && node.descend_iter().next() == Some(reader)
    })
}

fn option_u64_layout(arch: &Architecture) -> Option<()> {
    let layout = arch.types().kuna_variant_layout(OPTION_U64)?;
    if layout.type_name != OPTION_U64
        || layout.size != 16
        || layout.niche
        || (layout.tag_offset, layout.tag_size, layout.payload_offset) != (0, 8, 8)
        || layout.variants.len() != 2
    {
        return None;
    }
    let none = layout.facet_named("None")?;
    let some = layout.facet_named("Some")?;
    if none.discr != Some(0)
        || !none.fields.is_empty()
        || !none.payload_type.is_empty()
        || some.discr != Some(1)
        || some.label != "Some"
        || some.fields.len() != 1
        || some.fields[0].0 != "__0"
        || some.fields[0].1 != 8
    {
        return None;
    }
    let payload = arch.types().find_by_name(&some.payload_type).ok()??;
    if payload.get_metatype() != type_metatype::TYPE_STRUCT
        || payload.is_incomplete()
        || payload.num_depend() != 1
    {
        return None;
    }
    let field = payload.get_field(0)?;
    let ty = &field.field_type;
    (field.name == "__0"
        && field.offset == 0
        && (ty.get_metatype(), ty.get_size()) == (type_metatype::TYPE_UINT, 8)
        && !ty.is_enum_type()
        && !ty.is_incomplete()
        && !crate::kuna_typequal::is_volatile(ty))
    .then_some(())
}

fn scalar_pair(arch: &Architecture, address: &Address) -> bool {
    if !address.is_join() {
        return false;
    }
    let Ok(join) = arch.manage().find_join(address.get_offset()) else {
        return false;
    };
    join.get_unified().size == 16
        && join.num_pieces() == 2
        && ["RDX", "RAX"].iter().enumerate().all(|(index, register)| {
            let Ok(expected) = arch.get_register_varnode(register.as_bytes()) else {
                return false;
            };
            let piece = join.get_piece(index as i32);
            piece.size == 8
                && expected.size == 8
                && piece.offset == expected.offset
                && piece
                    .space
                    .as_ref()
                    .zip(expected.space.as_ref())
                    .is_some_and(|(actual, expected)| actual.get_index() == expected.get_index())
        })
}

fn producer(fd: &Funcdata, arch: &Architecture, value: VarnodeId, extract: OpId) -> Option<OpId> {
    if !single_reader(fd, value, extract) {
        return None;
    }
    let node = fd.vbank().get(value)?;
    let producer = node.get_def()?;
    let op = fd.obank().get(producer)?;
    if node.get_size() != 16 || op.code() != OpCode::CPUI_CALL || op.get_out() != Some(value) {
        return None;
    }
    let call = fd.get_call_specs(fd.get_call_specs_index(producer)?);
    let proto = call.proto();
    if call.get_entry_address().is_invalid()
        || !proto.has_store()
        || !proto.is_output_locked()
        || proto.is_inline()
        || proto.is_no_return()
        || proto.get_inject_id() >= 0
    {
        return None;
    }
    let output = proto.get_output();
    let ty = proto.get_output_type()?;
    if output.is_indirect_storage()
        || output.is_hidden_return()
        || output.get_address() != *node.get_addr()
        || output.get_size() != 16
        || ty.get_name() != OPTION_U64
        || ty.get_metatype() != type_metatype::TYPE_STRUCT
        || ty.get_size() != 16
        || ty.is_incomplete()
        || crate::kuna_typequal::is_volatile(ty)
        || !matches!(ty.compare(&fd.vn_type_def_facing(value), 10), Ok(0))
        || !scalar_pair(arch, node.get_addr())
    {
        return None;
    }
    option_u64_layout(arch)?;
    Some(producer)
}

fn pattern(fd: &Funcdata, arch: &Architecture, branch: OpId) -> Option<(OpId, bool)> {
    let op = fd.obank().get(branch)?;
    if op.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let mut reader = branch;
    let mut condition = op.get_in(1)?;
    let mut flipped = op.is_boolean_flip();
    for _ in 0..8 {
        if !single_reader(fd, condition, reader) {
            return None;
        }
        let comparison = fd.vbank().get(condition)?.get_def()?;
        let op = fd.obank().get(comparison)?;
        let equals = match op.code() {
            OpCode::CPUI_BOOL_NEGATE => {
                flipped = !flipped;
                reader = comparison;
                condition = op.get_in(0)?;
                continue;
            }
            OpCode::CPUI_INT_EQUAL => !flipped,
            OpCode::CPUI_INT_NOTEQUAL => flipped,
            _ => return None,
        };
        let (left, right) = (op.get_in(0)?, op.get_in(1)?);
        let (tag, constant) = if fd.vbank().get(left)?.is_constant() {
            (right, left)
        } else {
            (left, right)
        };
        let constant = fd.vbank().get(constant)?;
        if !constant.is_constant()
            || constant.get_size() != 8
            || !matches!(constant.get_offset(), 0 | 1)
            || !single_reader(fd, tag, comparison)
        {
            return None;
        }
        let node = fd.vbank().get(tag)?;
        let extract = node.get_def()?;
        let op = fd.obank().get(extract)?;
        if node.get_size() != 8 || op.code() != OpCode::CPUI_SUBPIECE {
            return None;
        }
        let offset = fd.vbank().get(op.get_in(1)?)?;
        if !offset.is_constant() || offset.get_offset() != 0 {
            return None;
        }
        let call = producer(fd, arch, op.get_in(0)?, extract)?;
        return Some((call, (constant.get_offset() == 1) == equals));
    }
    None
}

impl PrintC {
    pub(crate) fn emit_rust_option_condition(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        block: BlockId,
    ) -> bool {
        if self.out_lang != OutLang::Rust
            || !arch.source_lang().is_rust()
            || !arch.archid.starts_with("x86:LE:64:")
        {
            return false;
        }
        let Some(under) = fd.sblocks_ref().block(block).get_copy() else {
            return false;
        };
        let Some(branch) = fd.bb_op_tail(under) else {
            return false;
        };
        let Some((call, some)) = pattern(fd, arch, branch) else {
            return self.emit_rust_niche_option_condition(fd, arch, branch);
        };
        self.emit.print("let", SyntaxHighlight::KeywordColor);
        self.emit.spaces(1, 0);
        self.emit.print(
            if some {
                "::core::option::Option::Some(_)"
            } else {
                "::core::option::Option::None"
            },
            SyntaxHighlight::NoColor,
        );
        self.emit.spaces(1, 0);
        self.emit.print("=", SyntaxHighlight::NoColor);
        self.emit.spaces(1, 0);
        self.op_call_raw_ir(fd, arch, call);
        true
    }
}
