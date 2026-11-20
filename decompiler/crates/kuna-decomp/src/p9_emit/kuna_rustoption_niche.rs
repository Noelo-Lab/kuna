//! Whole named reference-Option inputs whose only use tests the null niche.

use super::*;
use crate::dtype::{type_metatype, Datatype};

const OPTION_REF_U64: &str = "core::option::Option<&u64>";

fn complete(ty: &Datatype, meta: type_metatype) -> bool {
    ty.get_metatype() == meta
        && ty.get_size() == 8
        && !ty.is_incomplete()
        && !crate::kuna_typequal::is_volatile(ty)
}

fn reference_layout(arch: &Architecture, ty: &Datatype) -> Option<()> {
    if ty.get_name() != OPTION_REF_U64
        || !complete(ty, type_metatype::TYPE_UNION)
        || ty.num_depend() != 1
    {
        return None;
    }
    let layout = arch.types().kuna_variant_layout(OPTION_REF_U64)?;
    if layout.type_name != OPTION_REF_U64
        || layout.union_type != OPTION_REF_U64
        || !layout.niche
        || layout.size != 8
        || (layout.tag_offset, layout.tag_size, layout.payload_offset) != (0, 8, 0)
        || layout.variants.len() != 2
    {
        return None;
    }
    let none = layout.facet_named("None")?;
    let some = layout.facet_named("Some")?;
    if none.discr != Some(0)
        || !none.fields.is_empty()
        || !none.payload_type.is_empty()
        || !none.label.is_empty()
        || some.discr.is_some()
        || some.label != "Some"
        || some.fields.len() != 1
        || some.fields[0].0 != "__0"
        || some.fields[0].1 != 0
    {
        return None;
    }
    let arm = ty.get_field(0)?;
    let payload = arch.types().find_by_name(&some.payload_type).ok()??;
    if arm.name != "Some"
        || arm.offset != 0
        || arm.field_type.get_name() != some.payload_type
        || !matches!(arm.field_type.compare(&payload, 10), Ok(0))
        || !complete(&payload, type_metatype::TYPE_STRUCT)
        || payload.num_depend() != 1
    {
        return None;
    }
    let field = payload.get_field(0)?;
    let pointer = &field.field_type;
    let referent = pointer.get_ptr_to()?;
    (field.name == "__0"
        && field.offset == 0
        && complete(pointer, type_metatype::TYPE_PTR)
        && !pointer.is_pointer_rel()
        && pointer.get_word_size() == Some(1)
        && complete(&referent, type_metatype::TYPE_UINT)
        && !referent.is_enum_type())
    .then_some(())
}

fn single_condition(fd: &Funcdata, value: VarnodeId, reader: OpId) -> bool {
    fd.vbank().get(value).is_some_and(|node| {
        node.get_size() == 1
            && node.is_implied()
            && !node.is_volatile()
            && !node.is_persist()
            && node.num_descend() == 1
            && node.descend_iter().next() == Some(reader)
    })
}

impl PrintC {
    fn named_reference_option(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        read: OpId,
    ) -> Option<String> {
        let node = fd.vbank().get(value)?;
        if !node.is_input()
            || node.is_implied()
            || node.is_constant()
            || node.is_volatile()
            || node.is_persist()
            || node.get_size() != 8
            || node.num_descend() != 1
            || node.descend_iter().next() != Some(read)
        {
            return None;
        }
        let high_id = node.get_high()?;
        let high = fd.high_bank().get(high_id)?;
        if high.num_instances() != 1
            || high.kuna_global()
            || high.kuna_ref_symbol().is_some()
            || self.local_name_aliases.contains_key(&high_id)
        {
            return None;
        }
        let (name, 0, Some(symbol_type)) = self.emitted_high_symbol(fd, high_id)? else {
            return None;
        };
        if name.is_empty()
            || name.contains("::")
            || matches!(name.as_str(), "_" | "self" | "Self" | "super" | "crate")
            || crate::kuna_rustuserop::raw_keyword(&name)
            || crate::kuna_rusttypes::sanitize_path(&name) != name
        {
            return None;
        }
        let proto = fd.get_func_proto();
        if !proto.has_store() {
            return None;
        }
        let mut parameter_type = None;
        for index in 0..proto.num_params() {
            let parameter = proto.get_param(index)?;
            if parameter.get_address() != *node.get_addr()
                || parameter.get_size() != node.get_size()
            {
                continue;
            }
            if parameter_type.is_some()
                || parameter.is_indirect_storage()
                || parameter.is_hidden_return()
            {
                return None;
            }
            parameter_type = Some(parameter.get_type()?.clone());
        }
        let parameter_type = parameter_type?;
        let declared = self.rust_integer_value_type(fd, arch, value, read);
        if !matches!(declared.compare(&symbol_type, 10), Ok(0))
            || !matches!(declared.compare(&parameter_type, 10), Ok(0))
        {
            return None;
        }
        reference_layout(arch, &declared)?;
        Some(name)
    }

    pub(crate) fn emit_rust_niche_option_condition(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        branch: OpId,
    ) -> bool {
        let Some((name, value, read, some)) = self.rust_niche_option_pattern(fd, arch, branch)
        else {
            return false;
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
        self.push_atom(&Atom::with_op_vn(
            name,
            TagType::VarToken,
            crate::printlanguage::SyntaxHighlight::var_color,
            op_key(read),
            vn_key(value),
        ));
        true
    }

    fn rust_niche_option_pattern(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        branch: OpId,
    ) -> Option<(String, VarnodeId, OpId, bool)> {
        if self.out_lang != crate::kuna_lang::OutLang::Rust
            || !arch.source_lang().is_rust()
            || !arch.archid.starts_with("x86:LE:64:")
        {
            return None;
        }
        let op = fd.obank().get(branch)?;
        if op.code() != OpCode::CPUI_CBRANCH {
            return None;
        }
        let mut reader = branch;
        let mut condition = op.get_in(1)?;
        let mut flipped = op.is_boolean_flip();
        for _ in 0..8 {
            if !single_condition(fd, condition, reader) {
                return None;
            }
            let comparison = fd.vbank().get(condition)?.get_def()?;
            let op = fd.obank().get(comparison)?;
            let some = match op.code() {
                OpCode::CPUI_BOOL_NEGATE => {
                    flipped = !flipped;
                    reader = comparison;
                    condition = op.get_in(0)?;
                    continue;
                }
                OpCode::CPUI_INT_EQUAL => flipped,
                OpCode::CPUI_INT_NOTEQUAL => !flipped,
                _ => return None,
            };
            let (left, right) = (op.get_in(0)?, op.get_in(1)?);
            let (value, constant) = if fd.vbank().get(left)?.is_constant() {
                (right, left)
            } else {
                (left, right)
            };
            let constant = fd.vbank().get(constant)?;
            if !constant.is_constant() || constant.get_size() != 8 || constant.get_offset() != 0 {
                return None;
            }
            let name = self.named_reference_option(fd, arch, value, comparison)?;
            return Some((name, value, comparison, some));
        }
        None
    }
}
