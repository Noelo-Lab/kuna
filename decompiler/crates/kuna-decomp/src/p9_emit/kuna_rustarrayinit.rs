//! Coalesce complete consecutive element copies without inventing array contents.

use super::*;

impl PrintC {
    fn rust_array_piece(
        &self,
        fd: &Funcdata,
        op: OpId,
    ) -> Option<(String, i32, std::rc::Rc<crate::dtype::Datatype>)> {
        let operation = fd.obank().get(op)?;
        if operation.code() != OpCode::CPUI_COPY || operation.num_input() != 1
            || operation.does_special_printing() || self.is_suppressed_statement(fd, op)
        {
            return None;
        }
        let output = fd.vbank().get(operation.get_out()?)?;
        if output.is_implied() || output.is_volatile() || output.is_persist() {
            return None;
        }
        let high = output.get_high()?;
        let variable = fd.high_bank().get(high)?;
        if variable.kuna_global() || variable.kuna_symbol_type()?.get_metatype()
            != crate::dtype::type_metatype::TYPE_ARRAY
        {
            return None;
        }
        let (name, offset, Some(array)) = self.emitted_high_symbol(fd, high)? else {
            return None;
        };
        if array.get_metatype() != crate::dtype::type_metatype::TYPE_ARRAY || offset < 0 {
            return None;
        }
        let element = array.get_array_base()?;
        if element.get_size() <= 0 || element.get_align_size() != element.get_size()
            || output.get_size() != element.get_size()
            || offset % element.get_size() != 0
            || !matches!(element.get_metatype(), crate::dtype::type_metatype::TYPE_INT
                | crate::dtype::type_metatype::TYPE_UINT | crate::dtype::type_metatype::TYPE_UNKNOWN)
            || element.is_enum_type()
        {
            return None;
        }
        let input = fd.vbank().get(operation.get_in(0)?)?;
        if input.is_volatile() || input.is_implied() || input.is_persist() {
            return None;
        }
        if !input.is_constant() {
            let source = input.get_high()?;
            let source_high = fd.high_bank().get(source)?;
            if source_high.kuna_global() || source_high.kuna_ref_symbol().is_some()
                || self.emitted_high_name(fd, source)? == name
            {
                return None;
            }
            let space = output.get_addr().get_space()?;
            let base = output.get_addr().get_offset().wrapping_sub(offset as u64);
            for index in 0..source_high.num_instances() {
                let member = fd.vbank().get(source_high.get_instance(index))?;
                let address = member.get_addr();
                if address.get_space().is_some_and(|s| s.get_index() == space.get_index())
                    && (address.get_offset().wrapping_sub(base) < array.get_size() as u64
                        || base.wrapping_sub(address.get_offset()) < member.get_size() as u64)
                {
                    return None;
                }
            }
        }
        Some((name, offset, array))
    }

    /// Return the last consumed op only after proving a complete ordered cover.
    pub(super) fn emit_rust_array_init(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        first: OpId,
    ) -> Option<OpId> {
        if self.out_lang != crate::kuna_lang::OutLang::Rust
            || self.context.is_set(modifiers::COMMA_SEPARATE)
        {
            return None;
        }
        let (name, 0, array) = self.rust_array_piece(fd, first)? else {
            return None;
        };
        let element = array.get_array_base()?;
        let count = array.num_elements()?;
        if count <= 1 || count.checked_mul(element.get_size())? != array.get_size() {
            return None;
        }
        let mut last = first;
        for index in 1..count {
            last = fd.bb_op_next(last)?;
            let (next_name, offset, next_array) = self.rust_array_piece(fd, last)?;
            if next_name != name || !std::rc::Rc::ptr_eq(&array, &next_array)
                || offset != index * element.get_size()
            {
                return None;
            }
        }
        let output = fd.obank().get(first)?.get_out()?;
        let markup = self.op_markup(fd, first);
        let statement = self.emit.begin_statement(&markup);
        self.push_atom(&Atom::with_op_vn(
            name, TagType::VarToken, crate::printlanguage::SyntaxHighlight::var_color,
            op_key(first), vn_key(output),
        ));
        self.emit.print(" = [", SyntaxHighlight::NoColor);
        let mut current = first;
        for index in 0..count {
            if index != 0 {
                self.emit.print(", ", SyntaxHighlight::NoColor);
                self.emit_comment_group(fd, Some(current));
            }
            self.op_push_ir(fd, arch, current, None);
            if current != last {
                current = fd.bb_op_next(current)?;
            }
        }
        self.emit.print("]", SyntaxHighlight::NoColor);
        self.emit.end_statement(statement);
        self.emit.print(self.lang().kw_semicolon, SyntaxHighlight::NoColor);
        Some(last)
    }
}
