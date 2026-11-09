//! Go multiple-result syntax over verified ordinary struct storage.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_langtypes::SpellCtx;
use crate::printc::{op_key, op_token, tokens, PrintC};
use crate::printlanguage::{Atom, OpToken, SyntaxHighlight, TagType, TokenType};
use kuna_num::opcodes::OpCode;

pub(crate) fn fields(cx: &SpellCtx, ty: &Datatype) -> Option<[Rc<Datatype>; 2]> {
    if ty.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    cx.go_pair_types
        .as_ref()?
        .kuna_lang_facet(ty.get_name())?
        .go_pair_fields(ty)
}

fn names(fields: &[Rc<Datatype>; 2]) -> [&'static str; 2] {
    fields.each_ref().map(|ty| {
        if ty.get_metatype() == type_metatype::TYPE_INT {
            "int64"
        } else {
            "uint64"
        }
    })
}

fn storage(fields: &[Rc<Datatype>; 2]) -> String {
    let [a, b] = names(fields);
    format!("struct {{ result0 {a}; result1 {b} }}")
}

pub(crate) fn storage_type(cx: &SpellCtx, ty: &Datatype) -> Option<String> {
    Some(storage(&fields(cx, ty)?))
}

pub(crate) fn result_type(cx: &SpellCtx, ty: &Datatype) -> Option<String> {
    let [a, b] = names(&fields(cx, ty)?);
    Some(format!("({a}, {b})"))
}

fn call_fields(cx: &SpellCtx, fd: &Funcdata, op: OpId) -> Option<[Rc<Datatype>; 2]> {
    if !matches!(
        fd.obank().get(op)?.code(),
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
    ) {
        return None;
    }
    let proto = fd.get_call_specs(fd.get_call_specs_index(op)?).proto();
    fields(cx, proto.get_output_type()?)
}

pub(crate) fn call_projection(fd: &Funcdata, arch: &Architecture, op: OpId) -> Option<usize> {
    if !arch.analysis_godwarfsig {
        return None;
    }
    let value = fd.vbank().get(fd.obank().get(op)?.get_out()?)?;
    let proto = fd.get_call_specs(fd.get_call_specs_index(op)?).proto();
    let ty = proto.get_output_type()?;
    arch.types()
        .kuna_lang_facet(ty.get_name())?
        .go_pair_fields(ty)?;
    if value.get_size() != 8
        || proto.get_model_name() != "abi-internal"
        || !arch.archid.starts_with("x86:LE:64:")
    {
        return None;
    }
    ["RAX", "RBX"].iter().position(|register| {
        let Ok(expected) = arch.get_register_varnode(register.as_bytes()) else {
            return false;
        };
        expected.size == 8
            && value.get_offset() == expected.offset
            && value
                .get_addr()
                .get_space()
                .zip(expected.space.as_ref())
                .is_some_and(|(space, expected)| space.get_index() == expected.get_index())
    })
}

fn forwarded_call(
    cx: &SpellCtx,
    fd: &Funcdata,
    mut value: VarnodeId,
    target: &[Rc<Datatype>; 2],
) -> Option<OpId> {
    for _ in 0..8 {
        let vn = fd.vbank().get(value)?;
        if !vn.is_implied() || vn.is_volatile() || vn.get_size() != 16 {
            return None;
        }
        let op = vn.get_def()?;
        if call_fields(cx, fd, op).is_some_and(|results| names(&results) == names(target)) {
            return Some(op);
        }
        let operation = fd.obank().get(op)?;
        if !matches!(operation.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST)
            || operation.num_input() != 1
        {
            return None;
        }
        value = operation.get_in(0)?;
        if names(&fields(cx, &fd.vn_type_read_facing(value, op))?) != names(target) {
            return None;
        }
    }
    None
}

impl PrintC {
    pub(crate) fn push_go_pair_extract(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> bool {
        let Some(operation) = fd.obank().get(op) else {
            return false;
        };
        let (Some(output), Some(input), Some(offset)) = (
            operation.get_out(),
            operation.get_in(0),
            operation.get_in(1),
        ) else {
            return false;
        };
        let Some(value) = fd.vbank().get(input) else {
            return false;
        };
        let Some(offset) = fd.vbank().get(offset).filter(|vn| vn.is_constant()) else {
            return false;
        };
        if value.get_size() != 16 || value.is_volatile() || !matches!(offset.get_offset(), 0 | 8) {
            return false;
        }
        let Some(fields) = fields(&self.rt_ctx, &self.go_value_type(fd, arch, input, op)) else {
            return false;
        };
        let target = fd.vn_type_def_facing(output);
        if target.get_size() != 8
            || !matches!(
                target.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            )
        {
            return false;
        }
        let index = (offset.get_offset() / 8) as usize;
        if (target.get_metatype() == type_metatype::TYPE_INT)
            != (fields[index].get_metatype() == type_metatype::TYPE_INT)
        {
            self.push_go_conversion(&target, &fields[index], op);
        }
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_vn_ir(fd, arch, input, op);
        self.push_atom(&Atom::with_op(
            format!("result{index}"),
            TagType::OpToken,
            SyntaxHighlight::var_color,
            op_key(op),
        ));
        true
    }

    pub(crate) fn push_go_pair_call_adapter(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> Option<usize> {
        let Some(fields) = call_fields(&self.rt_ctx, fd, op) else {
            return None;
        };
        let projection = call_projection(fd, arch, op);
        if projection.is_some() {
            self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        }
        let [a, b] = names(&fields);
        let ty = storage(&fields);
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_atom(&Atom::with_op(
            format!("func(r0 {a}, r1 {b}) {ty} {{ return {ty}{{r0, r1}} }}"),
            TagType::OpToken,
            SyntaxHighlight::no_color,
            op_key(op),
        ));
        projection
    }

    pub(crate) fn push_go_pair_projection(&mut self, op: OpId, index: usize) {
        self.push_atom(&Atom::with_op(
            format!("result{index}"),
            TagType::OpToken,
            SyntaxHighlight::var_color,
            op_key(op),
        ));
    }

    fn push_go_pair_member(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        index: usize,
    ) {
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_vn_explicit_ir(fd, arch, value, op);
        self.push_atom(&Atom::with_op(
            format!("result{index}"),
            TagType::OpToken,
            SyntaxHighlight::var_color,
            op_key(op),
        ));
    }

    pub(crate) fn emit_go_pair_assignment(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> bool {
        static RESULTS: OpToken = op_token(",", "", 2, 16, true, TokenType::Binary, 0, 0);
        let Some(results) = call_fields(&self.rt_ctx, fd, op) else {
            return false;
        };
        let Some(out) = fd.obank().get(op).and_then(|operation| operation.get_out()) else {
            return false;
        };
        let Some(target) = fields(&self.rt_ctx, &self.go_value_type(fd, arch, out, op)) else {
            return false;
        };
        if names(&results) != names(&target) {
            return false;
        }
        self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
        self.push_op(&RESULTS, Some(op_key(op)));
        self.push_go_pair_member(fd, arch, out, op, 0);
        self.push_go_pair_member(fd, arch, out, op, 1);
        self.op_call_raw_ir(fd, arch, op);
        true
    }

    pub(crate) fn push_go_pair_return(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
    ) -> bool {
        let Some(target) = fd
            .get_func_proto()
            .get_output_type()
            .and_then(|ty| fields(&self.rt_ctx, ty))
        else {
            return false;
        };
        let Some(source) = fields(&self.rt_ctx, &self.go_value_type(fd, arch, value, op)) else {
            return false;
        };
        if names(&target) != names(&source) {
            return false;
        }
        let Some(vn) = fd.vbank().get(value) else {
            return false;
        };
        if vn.is_implied() {
            if let Some(call) = forwarded_call(&self.rt_ctx, fd, value, &target) {
                self.op_call_raw_ir(fd, arch, call);
                return true;
            }
        } else if !vn.is_constant() && !vn.is_persist() && !vn.is_volatile() {
            self.push_op(&tokens::COMMA, Some(op_key(op)));
            self.push_go_pair_member(fd, arch, value, op, 0);
            self.push_go_pair_member(fd, arch, value, op, 1);
            return true;
        }
        let [a, b] = names(&target);
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_atom(&Atom::with_op(
            format!(
                "func(v {}) ({a}, {b}) {{ return v.result0, v.result1 }}",
                storage(&target)
            ),
            TagType::OpToken,
            SyntaxHighlight::no_color,
            op_key(op),
        ));
        self.push_vn_ir(fd, arch, value, op);
        true
    }

    pub(crate) fn push_go_pair_piece(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> bool {
        static COMPOUND: OpToken = op_token("", "}", 2, 66, false, TokenType::Postsurround, 0, 0);
        let Some(operation) = fd.obank().get(op) else {
            return false;
        };
        let (Some(output), Some(high), Some(low)) = (
            operation.get_out(),
            operation.get_in(0),
            operation.get_in(1),
        ) else {
            return false;
        };
        let Some(fields) = fields(&self.rt_ctx, &fd.vn_type_def_facing(output)) else {
            return false;
        };
        if [high, low]
            .iter()
            .any(|&input| fd.vbank().get(input).is_none_or(|vn| vn.get_size() != 8))
        {
            return false;
        }
        self.push_op(&COMPOUND, Some(op_key(op)));
        self.push_atom(&Atom::with_op(
            format!("{}{{", storage(&fields)),
            TagType::OpToken,
            SyntaxHighlight::type_color,
            op_key(op),
        ));
        self.push_op(&tokens::COMMA, Some(op_key(op)));
        self.push_go_value_as(fd, arch, low, op, &fields[0]);
        self.push_go_value_as(fd, arch, high, op, &fields[1]);
        true
    }
}

#[cfg(test)]
mod tests;

mod carrier;
