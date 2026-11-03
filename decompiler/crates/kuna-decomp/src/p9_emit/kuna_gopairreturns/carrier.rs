//! Recover paired fields from the integer carrier's zero extension or word shift.

use super::*;
use crate::kuna_lang::OutLang;
use crate::kuna_langgo::GO_SPELLER;
use crate::kuna_langtypes::TypeSpeller;

#[derive(Clone, Copy)]
enum Carrier {
    ZeroExtend(VarnodeId, OpId),
    HighWord(VarnodeId, OpId),
}

fn carrier(fd: &Funcdata, mut value: VarnodeId) -> Option<Carrier> {
    for _ in 0..8 {
        let vn = fd.vbank().get(value)?;
        if vn.get_size() != 16 || !vn.is_implied() || vn.is_volatile() {
            return None;
        }
        let id = vn.get_def()?;
        let operation = fd.obank().get(id)?;
        let input = operation.get_in(0)?;
        let source = fd.vbank().get(input)?;
        if source.is_volatile() {
            return None;
        }
        match operation.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST if operation.num_input() == 1 => {
                value = input;
            }
            OpCode::CPUI_INT_ZEXT if source.get_size() == 8
                && matches!(fd.vn_type_read_facing(input, id).get_metatype(),
                    type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN) => {
                return Some(Carrier::ZeroExtend(input, id));
            }
            OpCode::CPUI_INT_LEFT if source.get_size() == 16 => {
                let shift = fd.vbank().get(operation.get_in(1)?)?;
                return (shift.is_constant() && shift.get_offset() == 64)
                    .then_some(Carrier::HighWord(input, id));
            }
            _ => return None,
        }
    }
    None
}

fn raw_carrier(fd: &Funcdata, mut value: VarnodeId) -> bool {
    for _ in 0..8 {
        let Some(vn) = fd.vbank().get(value) else { return false };
        if vn.get_size() != 16 || vn.is_volatile() || vn.is_constant() {
            return false;
        }
        if !vn.is_implied() { return true; }
        let Some(id) = vn.get_def() else { return false };
        let Some(operation) = fd.obank().get(id) else { return false };
        match operation.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST if operation.num_input() == 1 => {
                let Some(input) = operation.get_in(0) else { return false };
                value = input;
            }
            OpCode::CPUI_LOAD => return true,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                return fd.get_call_specs_index(id)
                    .is_some_and(|index| {
                        let proto = fd.get_call_specs(index).proto();
                        proto.get_output_type().is_some_and(|ty| {
                            (ty.get_size() == 16 && matches!(ty.get_metatype(),
                                type_metatype::TYPE_INT | type_metatype::TYPE_UINT
                                    | type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_ARRAY))
                                || (!proto.is_output_locked()
                                    && ty.get_metatype() == type_metatype::TYPE_VOID)
                        })
                    });
            }
            _ => return false,
        }
    }
    false
}

impl PrintC {
    fn push_raw_pair_carrier(
        &mut self, fd: &Funcdata, arch: &Architecture, value: VarnodeId,
        op: OpId, target: &Rc<Datatype>, results: &[Rc<Datatype>; 2],
    ) -> bool {
        if !raw_carrier(fd, value) { return false; }
        let source = self.go_value_type(fd, arch, value, op);
        if source.get_size() != 16 || !matches!(source.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
                | type_metatype::TYPE_ARRAY)
        {
            return false;
        }
        match self.out_lang {
            OutLang::Go => {
                if GO_SPELLER.declarator(&self.rt_ctx, &source).0 != "[16]byte" {
                    return false;
                }
                let [a, b] = names(results);
                let ty = storage(results);
                self.push_go_call(&format!(
                    "func(v [16]byte) {ty} {{ return {ty}{{*(*{a})(unsafe.Pointer(&v[0])), *(*{b})(unsafe.Pointer(&v[8]))}} }}"
                ), op);
                self.push_vn_ir(fd, arch, value, op);
            }
            OutLang::C => {
                static UNION: OpToken = op_token("", "}).value", 2, 66, false, TokenType::Postsurround, 0, 0);
                self.push_op(&UNION, Some(op_key(op)));
                self.push_atom(&Atom::with_op(format!(
                    "((union {{ unsigned __int128 carrier; {} value; }}){{.carrier = ", target.get_name()),
                    TagType::Syntax, SyntaxHighlight::type_color, op_key(op)));
                self.push_vn_ir(fd, arch, value, op);
            }
            _ => return false,
        }
        true
    }

    pub(crate) fn push_pair_carrier(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        if !arch.analysis_godwarfsig || !arch.analysis_gopairreturns
            || target.get_metatype() != type_metatype::TYPE_STRUCT
            || !arch.archid.starts_with("x86:LE:64:")
        {
            return false;
        }
        let Some(results) = arch.types().kuna_lang_facet(target.get_name())
            .and_then(|facet| facet.go_pair_fields(target)) else { return false };
        let Some(plan) = carrier(fd, value) else {
            return self.push_raw_pair_carrier(fd, arch, value, op, target, &results);
        };
        let mut pointer = None;
        let mut pair_source = None;
        if let Carrier::HighWord(input, read_op) = plan {
            if self.out_lang == OutLang::Rust {
                return false;
            }
            if self.out_lang == OutLang::Go {
                let source = self.go_value_type(fd, arch, input, read_op);
                pair_source = fields(&self.rt_ctx, &source);
                if pair_source.is_none() {
                    let vn = fd.vbank().get(input).unwrap();
                    if vn.is_implied() || vn.is_constant()
                        || GO_SPELLER.declarator(&self.rt_ctx, &source).0 != "[16]byte"
                    {
                        return false;
                    }
                    let Ok(ty) = arch.types().get_type_pointer(8, results[1].clone(), 1)
                        else { return false };
                    pointer = Some(ty);
                }
            }
        }
        static COMPOUND: OpToken = op_token("", "}", 2, 66, false, TokenType::Postsurround, 0, 0);
        static FIELD: OpToken = op_token(":", "", 2, 17, false, TokenType::Binary, 0, 0);
        self.push_op(&COMPOUND, Some(op_key(op)));
        self.push_atom(&Atom::with_op(
            match self.out_lang {
                OutLang::Go => format!("{}{{", storage(&results)),
                OutLang::Rust => format!("{} {{", target.get_name()),
                _ => format!("({}){{", target.get_name()),
            },
            TagType::TypeToken, SyntaxHighlight::type_color, op_key(op),
        ));
        self.push_op(&tokens::COMMA, Some(op_key(op)));
        for index in 0..2 {
            if self.out_lang == OutLang::Rust {
                self.push_op(&FIELD, Some(op_key(op)));
                self.push_atom(&Atom::syntax(format!("result{index}"), TagType::Syntax,
                    SyntaxHighlight::var_color));
            }
            match (plan, index) {
                (Carrier::ZeroExtend(input, read_op), 0) => {
                    if self.out_lang == OutLang::Go {
                        self.push_go_value_as(fd, arch, input, read_op, &results[0]);
                    } else if self.out_lang == OutLang::Rust {
                        self.push_cast_open(&results[0], op);
                        self.push_vn_ir(fd, arch, input, read_op);
                        self.push_cast_close(&results[0]);
                    } else {
                        self.push_vn_ir(fd, arch, input, read_op);
                    }
                }
                (Carrier::HighWord(input, read_op), 1) => {
                    if let Some(ty) = &pointer {
                        self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
                        self.push_go_pointer_conversion(ty, true, op);
                        self.push_op(&tokens::ADDRESSOF, Some(op_key(op)));
                        self.push_vn_explicit_ir(fd, arch, input, read_op);
                    } else if let Some(source) = &pair_source {
                        if names(source)[0] != names(&results)[1] {
                            self.push_go_conversion(&results[1], &source[0], op);
                        }
                        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
                        self.push_vn_ir(fd, arch, input, read_op);
                        self.push_go_pair_projection(op, 0);
                    } else {
                        self.push_vn_ir(fd, arch, input, read_op);
                    }
                }
                _ => self.push_atom(&Atom::syntax("0", TagType::Syntax,
                    SyntaxHighlight::const_color)),
            }
        }
        true
    }
}
