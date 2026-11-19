//! (kuna `dexmodel`) Printing the byte-code operators: `CPOOLREF` and `NEW`
//! (`PrintC::opCpoolRefOp`, `PrintC::opNewOp`), and the java-language forms of
//! LOAD, STORE and CALLIND (`PrintJava::opLoad`, `opStore`, `opCallind`),
//! the last also as `new T(args)` (kuna `javanew`).

use kuna_num::opcodes::OpCode;

use super::{keywords, modifiers, op_key, tokens, vn_key, Architecture, Atom, Funcdata, OpId, PrintC, TagType, VarnodeId};
use crate::cpool::cpool_tag;
use crate::dtype::type_metatype;
use crate::kuna_lang::OutLang;
use crate::printlanguage::SyntaxHighlight;

impl PrintC {
    fn java(&self) -> bool {
        self.out_lang == OutLang::Java
    }

    /// `PrintC::opCpoolRefOp` / `PrintJava::opCpoolRefOp`.
    pub(super) fn op_cpoolref_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(o) = fd.obank().get(op) else { return };
        let outvn = o.get_out();
        let vn0 = o.get_in(0);
        let refs: Vec<u64> = (1..o.num_input())
            .filter_map(|i| o.get_in(i).and_then(|v| fd.vbank().get(v)).map(|v| v.get_offset()))
            .collect();
        let atom = |text: String, tag: TagType, hl: SyntaxHighlight| match outvn {
            Some(out) => Atom::with_op_vn(text, tag, hl, op_key(op), vn_key(out)),
            None => Atom::with_op(text, tag, hl, op_key(op)),
        };
        let Some(rec) = fd.get_arch().cpool_record(&refs) else {
            self.push_atom(&atom("UNKNOWNREF".to_string(), TagType::Syntax, SyntaxHighlight::const_color));
            return;
        };
        let token = String::from_utf8_lossy(rec.get_token()).into_owned();
        match rec.get_tag() {
            cpool_tag::STRING_LITERAL => {
                let data = rec.get_byte_data().unwrap_or(&[]);
                let shown = &data[..data.len().min(2048)];
                let mut text = String::from("\"");
                for ch in String::from_utf8_lossy(shown).chars() {
                    self.push_escaped_codepoint(&mut text, ch as i32);
                }
                text.push_str(if shown.len() == data.len() { "\"" } else { "...\"" });
                self.push_atom(&atom(text, TagType::VarToken, SyntaxHighlight::const_color));
            }
            cpool_tag::CLASS_REFERENCE => {
                self.push_atom(&atom(token, TagType::VarToken, SyntaxHighlight::type_color));
            }
            cpool_tag::INSTANCE_OF => {
                let mut dt = rec.get_type().cloned();
                while let Some(to) = dt.as_ref().filter(|t| t.get_metatype() == type_metatype::TYPE_PTR).and_then(|t| t.get_ptr_to()) {
                    dt = Some(to);
                }
                let type_name = dt.map(|t| t.get_display_name().to_string()).unwrap_or_default();
                if self.java() {
                    self.push_op(&crate::kuna_langjava::INSTANCEOF, Some(op_key(op)));
                    if let Some(vn) = vn0 {
                        self.push_vn_ir(fd, arch, vn, op);
                    }
                } else {
                    self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
                    self.push_atom(&atom(token, TagType::FuncToken, SyntaxHighlight::funcname_color));
                    self.push_op(&tokens::COMMA, None);
                    if let Some(vn) = vn0 {
                        self.push_vn_ir(fd, arch, vn, op);
                    }
                }
                self.push_atom(&atom(type_name, TagType::Syntax, SyntaxHighlight::type_color));
            }
            _ => {
                let code = rec.get_type().is_some_and(|t| {
                    t.get_metatype() == type_metatype::TYPE_PTR
                        && t.get_ptr_to().is_some_and(|to| to.get_metatype() == type_metatype::TYPE_CODE)
                });
                let color = if code { SyntaxHighlight::funcname_color } else { SyntaxHighlight::var_color };
                let relative = vn0.filter(|vn| fd.vbank().get(*vn).is_some_and(|v| !v.is_constant()));
                match relative {
                    None => self.push_atom(&atom(token, TagType::VarToken, color)),
                    Some(vn) => {
                        let member = if self.java() { &tokens::OBJECT_MEMBER } else { &tokens::POINTER_MEMBER };
                        self.push_op(member, Some(op_key(op)));
                        self.push_vn_ir(fd, arch, vn, op);
                        self.push_atom(&atom(token, TagType::Syntax, color));
                    }
                }
            }
        }
    }

    /// `PrintC::opNewOp`: `new T[n]` for an array allocation, `new(T)` otherwise.
    pub(super) fn op_new_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(o) = fd.obank().get(op) else { return };
        let outvn = o.get_out();
        let (vn0, vn1) = (o.get_in(0), o.get_in(1));
        let keyword = |hl| match outvn {
            Some(out) => Atom::with_op_vn(keywords::KEYWORD_NEW, TagType::OpToken, hl, op_key(op), vn_key(out)),
            None => Atom::with_op(keywords::KEYWORD_NEW, TagType::OpToken, hl, op_key(op)),
        };
        if let (2, Some(vn0), Some(vn1)) = (o.num_input(), vn0, vn1) {
            if fd.vbank().get(vn0).is_some_and(|v| !v.is_constant()) {
                self.push_op(&tokens::NEW_OP, Some(op_key(op)));
                self.push_atom(&keyword(SyntaxHighlight::keyword_color));
                let name = match outvn {
                    None => "<unused>".to_string(),
                    Some(out) => {
                        let mut dt = fd.vn_type_def_facing(out);
                        while let Some(to) = Some(&dt).filter(|t| t.get_metatype() == type_metatype::TYPE_PTR).and_then(|t| t.get_ptr_to()) {
                            dt = to;
                        }
                        let (front, back) = super::declarator_parts(&dt, self.rt_ctx.clone());
                        format!("{front}{back}")
                    }
                };
                self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
                self.push_atom(&Atom::with_op(name, TagType::OpToken, SyntaxHighlight::type_color, op_key(op)));
                self.push_vn_ir(fd, arch, vn1, op);
                return;
            }
        }
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_atom(&keyword(SyntaxHighlight::keyword_color));
        if let Some(vn) = vn0 {
            self.push_vn_ir(fd, arch, vn, op);
        }
    }

    /// `PrintJava::needZeroArray`: does a dereferenced Java array need `[0]`?
    fn need_zero_array(&self, fd: &Funcdata, vn: VarnodeId) -> bool {
        let Some(v) = fd.vbank().get(vn) else { return false };
        if !crate::kuna_langjava::is_array_type(v.get_type()) {
            return false;
        }
        if v.is_explicit() || !v.is_written() {
            return true;
        }
        let def = v.get_def().and_then(|d| fd.obank().get(d)).map(|d| d.code());
        !matches!(def, Some(OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_CPOOLREF))
    }

    fn push_zero_index(&mut self) {
        self.push_atom(&Atom::syntax("0", TagType::Syntax, SyntaxHighlight::const_color));
    }

    /// `PrintJava::opLoad`: a load through a reference prints its value.
    pub(super) fn java_load_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(ptr) = fd.obank().get(op).and_then(|o| o.get_in(1)) else { return };
        let m = self.context.mods() | modifiers::PRINT_LOAD_VALUE;
        let zero = self.need_zero_array(fd, ptr);
        if zero {
            self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
        }
        self.push_vn_ir_m(fd, arch, ptr, op, m);
        if zero {
            self.push_zero_index();
        }
    }

    /// `PrintJava::opStore`.
    pub(super) fn java_store_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(o) = fd.obank().get(op) else { return };
        let (Some(ptr), val) = (o.get_in(1), o.get_in(2)) else { return };
        let mods = self.context.mods();
        self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
        let zero = self.need_zero_array(fd, ptr);
        if zero {
            self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
        }
        self.push_vn_ir_m(fd, arch, ptr, op, mods | modifiers::PRINT_STORE_VALUE);
        if zero {
            self.push_zero_index();
        }
        if let Some(val) = val {
            self.push_vn_ir_m(fd, arch, val, op, mods);
        }
    }

    /// `PrintJava::opCallind`: `object.method(args)`, the object being the
    /// call's hidden `this` argument (`PrintC::getHiddenThisSlot`).
    pub(super) fn java_callind_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(o) = fd.obank().get(op) else { return };
        let nin = o.num_input();
        let skip = fd.get_call_specs_index(op).map_or(-1, |index| {
            let proto = fd.get_call_specs(index).proto();
            if !proto.has_this_pointer() {
                return -1;
            }
            let is_this = |slot: i32| proto.get_param(slot).is_some_and(|p| p.is_this_pointer());
            (1..nin - 1).find(|&i| is_this(i - 1)).or_else(|| (nin >= 2 && is_this(nin - 2)).then_some(nin - 1)).unwrap_or(-1)
        });
        let merged = self.java_new.get(&op).cloned();
        if let Some(pair) = &merged {
            if fd.vbank().get(pair.object).is_some_and(|v| !v.is_implied()) {
                self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
                self.push_vn_explicit_ir(fd, arch, pair.object, pair.new_op);
            }
            self.push_op(&tokens::NEW_OP, Some(op_key(op)));
            self.push_atom(&Atom::with_op(keywords::KEYWORD_NEW, TagType::OpToken, SyntaxHighlight::keyword_color, op_key(op)));
        }
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        let ctor = self.java_ctor_calls.get(&op).copied();
        match &merged {
            Some(pair) => self.push_atom(&Atom::with_op(pair.class.clone(), TagType::OpToken, SyntaxHighlight::type_color, op_key(op))),
            None if ctor.is_some() => {
                let keyword = if ctor == Some(crate::kuna_javanew::CtorCall::This) { "this" } else { "super" };
                self.push_atom(&Atom::with_op(keyword.to_string(), TagType::OpToken, SyntaxHighlight::keyword_color, op_key(op)));
            }
            None => {
                if let Some(callee) = o.get_in(0) {
                    self.push_vn_ir(fd, arch, callee, op);
                }
            }
        }
        let args: Vec<VarnodeId> = (1..nin).filter(|&i| i != skip).filter_map(|i| o.get_in(i)).collect();
        if args.is_empty() {
            self.push_atom(&Atom::syntax("", TagType::BlankToken, SyntaxHighlight::no_color));
            return;
        }
        for _ in 1..args.len() {
            self.push_op(&tokens::COMMA, Some(op_key(op)));
        }
        for vn in args {
            self.push_vn_ir(fd, arch, vn, op);
        }
    }
}
