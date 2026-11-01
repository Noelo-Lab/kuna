//! Go spelling and statement forms on the shared emission plane.

use std::borrow::Cow;
use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype};
use crate::kuna_lang::*;
use crate::kuna_langtypes::{SpellCtx, TypeSpeller};
use crate::printc::tokens;
use crate::printlanguage::{OpToken, TokenType};

const fn binary(
    text: &'static str,
    precedence: i32,
    associative: bool,
    negate: Option<&'static OpToken>,
) -> OpToken {
    OpToken {
        print1: text,
        print2: "",
        stage: 2,
        precedence,
        associative,
        token_type: TokenType::Binary,
        spacing: 1,
        bump: 0,
        negate,
        paren_before_angle: false,
        left_to_right_only: true,
    }
}

static AND: OpToken = binary("&", 54, true, None);
static OR: OpToken = binary("|", 50, true, None);
static XOR: OpToken = binary("^", 50, true, None);
static SHL: OpToken = binary("<<", 54, false, None);
static SHR: OpToken = binary(">>", 54, false, None);
static LT: OpToken = binary("<", 26, false, Some(&GE));
static LE: OpToken = binary("<=", 26, false, Some(&GT));
static GT: OpToken = binary(">", 26, false, Some(&LE));
static GE: OpToken = binary(">=", 26, false, Some(&LT));
static EQ: OpToken = binary("==", 26, false, Some(&NE));
static NE: OpToken = binary("!=", 26, false, Some(&EQ));
static COMPLEMENT: OpToken = OpToken {
    print1: "^",
    print2: "",
    stage: 1,
    precedence: 62,
    associative: false,
    token_type: TokenType::UnaryPrefix,
    spacing: 0,
    bump: 0,
    negate: None,
    paren_before_angle: false,
    left_to_right_only: false,
};

fn map_token(token: &'static OpToken) -> &'static OpToken {
    for (from, to) in [
        (&tokens::SCOPE, &tokens::OBJECT_MEMBER),
        (&tokens::BITWISE_AND, &AND),
        (&tokens::BITWISE_OR, &OR),
        (&tokens::BITWISE_XOR, &XOR),
        (&tokens::BITWISE_NOT, &COMPLEMENT),
        (&tokens::SHIFT_LEFT, &SHL),
        (&tokens::SHIFT_RIGHT, &SHR),
        (&tokens::SHIFT_SRIGHT, &SHR),
        (&tokens::LESS_THAN, &LT),
        (&tokens::LESS_EQUAL, &LE),
        (&tokens::GREATER_THAN, &GT),
        (&tokens::GREATER_EQUAL, &GE),
        (&tokens::EQUAL, &EQ),
        (&tokens::NOT_EQUAL, &NE),
        (&tokens::BOOLEAN_XOR, &NE),
        (&tokens::TYPECAST, &tokens::FUNCTION_CALL),
    ] {
        if std::ptr::eq(token, from) {
            return to;
        }
    }
    token
}

pub static LANG_GO: LangProfile = LangProfile {
    name: "go-language",
    file_ext: "go",
    kw_open_curly: "{",
    kw_close_curly: "}",
    kw_semicolon: "",
    kw_colon: ":",
    kw_comma: ",",
    kw_dotdotdot: "...",
    kw_void: "",
    kw_true: "true",
    kw_false: "false",
    kw_if: "if",
    kw_else: "else",
    kw_do: "",
    kw_while: "for",
    kw_for: "for",
    kw_goto: "goto",
    kw_break: "break",
    kw_continue: "continue",
    kw_case: "case",
    kw_switch: "switch",
    kw_default: "default",
    kw_return: "return",
    kw_type_pointer_rel: "ADJ",
    null_literal: "nil",
    sanitize_identifiers: true,
    map_token,
    tok_typecast: &tokens::FUNCTION_CALL,
    caps: LangCaps {
        goto: true,
        switch_captures_break: true,
        switch_falls_through: false,
        switch_multi_label: true,
        switch_default_optional: true,
        labeled_loop_break: false,
        labeled_block_break: false,
        do_while: false,
        c_for: true,
        ternary: false,
        comma_expression: false,
        implicit_bool_conditions: false,
        brace_elision: false,
        arrow_member: false,
        paren_conditions: false,
        integer_suffixes: false,
        integer_promotion: false,
        signed_text_byte: false,
    },
    forms: LangForms {
        proto: ProtoForm::GoFunc,
        decl: DeclForm::GoVar,
        switch: SwitchForm::GoSwitch,
        inf_loop: InfLoopForm::GoFor,
        while_loop: WhileForm::GoForCondition,
        do_while: DoWhileForm::GoBottomTest,
        for_loop: ForLoopForm::CForHeader,
        goto: GotoForm::CGoto,
        label: LabelForm::CColon,
        member: MemberForm::RustDerefParen,
        cast: CastForm::GoConversion,
        char_lit: CharForm::GoRune,
        string_escape: StringEscape::GoEscapes,
    },
    fn_attributes: &[],
};

pub(crate) fn identifier(name: &str) -> String {
    let valid = !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    let keyword = matches!(
        name,
        "break"
            | "default"
            | "func"
            | "interface"
            | "select"
            | "case"
            | "defer"
            | "go"
            | "map"
            | "struct"
            | "chan"
            | "else"
            | "goto"
            | "package"
            | "switch"
            | "const"
            | "fallthrough"
            | "if"
            | "range"
            | "type"
            | "continue"
            | "for"
            | "import"
            | "return"
            | "var"
            | "any"
            | "bool"
            | "byte"
            | "complex64"
            | "complex128"
            | "error"
            | "float32"
            | "float64"
            | "int"
            | "int8"
            | "int16"
            | "int32"
            | "int64"
            | "rune"
            | "string"
            | "uint"
            | "uint8"
            | "uint16"
            | "uint32"
            | "uint64"
            | "uintptr"
            | "true"
            | "false"
            | "iota"
            | "nil"
            | "append"
            | "cap"
            | "clear"
            | "close"
            | "complex"
            | "copy"
            | "delete"
            | "imag"
            | "len"
            | "make"
            | "max"
            | "min"
            | "new"
            | "panic"
            | "print"
            | "println"
            | "real"
            | "recover"
            | "math"
            | "unsafe"
    );
    if valid && !keyword && name != "_" && !name.starts_with("_kuna_ident_") {
        return name.to_string();
    }
    let mut result = "_kuna_ident_".to_string();
    for byte in name.bytes() {
        use std::fmt::Write;
        let _ = write!(result, "{byte:02x}");
    }
    result
}

pub(crate) fn path(name: &str) -> String {
    name.replace("::", ".")
        .split('.')
        .map(identifier)
        .collect::<Vec<_>>()
        .join(".")
}

pub struct GoSpeller;
pub static GO_SPELLER: GoSpeller = GoSpeller;

fn integer(size: i32, signed: bool) -> Option<&'static str> {
    Some(match (size, signed) {
        (1, true) => "int8",
        (2, true) => "int16",
        (4, true) => "int32",
        (8, true) => "int64",
        (1, false) => "byte",
        (2, false) => "uint16",
        (4, false) => "uint32",
        (8, false) => "uint64",
        _ => return None,
    })
}

fn storage(size: i32) -> String {
    format!("[{}]byte", size.max(1))
}

impl GoSpeller {
    fn expression(&self, cx: &SpellCtx, ty: &Rc<Datatype>, depth: usize) -> String {
        if depth > 16 {
            return "unsafe.Pointer".into();
        }
        match ty.get_metatype() {
            type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => match ty.get_ptr_to() {
                Some(inner) if inner.get_metatype() == type_metatype::TYPE_VOID => {
                    "unsafe.Pointer".into()
                }
                Some(inner) if inner.get_metatype() == type_metatype::TYPE_CODE => {
                    self.expression(cx, &inner, depth + 1)
                }
                Some(inner) => format!("*{}", self.expression(cx, &inner, depth + 1)),
                None => "unsafe.Pointer".into(),
            },
            type_metatype::TYPE_ARRAY => {
                let base = ty.get_array_base();
                let size = base.as_ref().map(|t| t.get_size()).unwrap_or(1).max(1);
                let count = ty.num_elements().unwrap_or(ty.get_size() / size);
                let element = base
                    .map(|t| self.expression(cx, &t, depth + 1))
                    .unwrap_or("byte".into());
                format!("[{count}]{element}")
            }
            type_metatype::TYPE_CODE => {
                let Some(proto) = ty.get_code_prototype() else {
                    return "unsafe.Pointer".into();
                };
                if !proto.has_store() {
                    return "unsafe.Pointer".into();
                }
                let mut params = (0..proto.num_params())
                    .map(|i| {
                        proto
                            .get_param(i)
                            .and_then(|p| p.get_type())
                            .map(|t| self.expression(cx, t, depth + 1))
                            .unwrap_or_else(|| "any".into())
                    })
                    .collect::<Vec<_>>();
                if proto.is_dotdotdot() {
                    params.push("...any".into());
                }
                let params = params.join(", ");
                let mut result = format!("func({params})");
                if let Some(output) = proto
                    .get_output_type()
                    .filter(|t| t.get_metatype() != type_metatype::TYPE_VOID)
                {
                    result.push(' ');
                    result.push_str(&self.expression(cx, output, depth + 1));
                }
                result
            }
            _ => self
                .relabel(cx, ty, false)
                .map(Cow::into_owned)
                .unwrap_or_else(|| {
                    if ty.get_name().is_empty() {
                        self.anonymous(ty)
                    } else {
                        path(ty.get_display_name())
                    }
                }),
        }
    }
}

impl TypeSpeller for GoSpeller {
    fn relabel(
        &self,
        _cx: &SpellCtx,
        ty: &Rc<Datatype>,
        _under_pointer: bool,
    ) -> Option<Cow<'static, str>> {
        let size = ty.get_size();
        let name = match ty.get_metatype() {
            type_metatype::TYPE_INT => integer(size, true),
            type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => integer(size, false),
            type_metatype::TYPE_FLOAT => match size {
                4 => Some("float32"),
                8 => Some("float64"),
                _ => None,
            },
            type_metatype::TYPE_BOOL => Some("bool"),
            type_metatype::TYPE_VOID => Some("any"),
            _ => return None,
        };
        Some(
            name.map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(storage(size))),
        )
    }

    fn anonymous(&self, ty: &Rc<Datatype>) -> String {
        integer(ty.get_size(), false)
            .map(str::to_owned)
            .unwrap_or_else(|| storage(ty.get_size()))
    }

    fn declarator(&self, cx: &SpellCtx, ty: &Rc<Datatype>) -> (String, String) {
        (self.expression(cx, ty, 0), String::new())
    }

    fn type_name(&self, cx: &SpellCtx, ty: &Rc<Datatype>) -> (String, String) {
        self.declarator(cx, ty)
    }

    fn unknown_base(
        &self,
        size: i32,
        _under_pointer: bool,
        _long_is_8: bool,
    ) -> Option<Cow<'static, str>> {
        Some(
            integer(size, false)
                .map(Cow::Borrowed)
                .unwrap_or_else(|| Cow::Owned(storage(size))),
        )
    }
}

use crate::architecture::Architecture;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::prettyprint::{BraceStyle, Emit, MarkupRef, SyntaxHighlight};
use crate::printc::{declarator_parts, PrintC};
use crate::printlanguage::modifiers;

impl PrintC {
    pub(crate) fn emit_prototype_declaration_go(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        markup: &MarkupRef,
    ) {
        let display = fd.get_display_name();
        let qualified = display.replace("::", ".");
        let name = identifier(qualified.rsplit('.').next().unwrap_or(&qualified));
        if name != display {
            self.emit
                .print(&format!("// {display}"), SyntaxHighlight::CommentColor);
            self.emit.tag_line();
        }
        let group = self.emit.begin_func_proto();
        self.emit.print("func ", SyntaxHighlight::KeywordColor);
        self.emit
            .tag_func_name(&name, SyntaxHighlight::FuncnameColor, markup);
        let paren = self.emit.open_paren("(", 0);
        let proto = fd.get_func_proto();
        if proto.has_store() {
            for i in 0..proto.num_params() {
                let Some(param) = proto.get_param(i) else {
                    continue;
                };
                if i > 0 {
                    self.emit.print(", ", SyntaxHighlight::NoColor);
                }
                let name = if param.get_name().is_empty() {
                    crate::database::kuna_materialized_param_name(arch.name_style_angr, i, "")
                } else {
                    param.get_name().to_string()
                };
                self.remember_go_parameter(i, &name);
                self.emit
                    .tag_variable(&identifier(&name), SyntaxHighlight::VarColor, markup);
                self.emit.print(" ", SyntaxHighlight::NoColor);
                let ty = param
                    .get_type()
                    .map(|t| declarator_parts(t, self.rt_ctx).0)
                    .unwrap_or("any".into());
                self.emit.tag_type(&ty, SyntaxHighlight::TypeColor, markup);
            }
            if proto.is_dotdotdot() {
                if proto.num_params() > 0 {
                    self.emit.print(", ", SyntaxHighlight::NoColor);
                }
                self.emit.print("_ ...any", SyntaxHighlight::TypeColor);
            }
        }
        self.emit.close_paren(")", paren);
        if let Some(output) = proto
            .has_store()
            .then(|| proto.get_output_type())
            .flatten()
            .filter(|t| t.get_metatype() != type_metatype::TYPE_VOID)
        {
            let ty = declarator_parts(output, self.rt_ctx).0;
            self.emit.print(" ", SyntaxHighlight::NoColor);
            let ret = self.emit.begin_return_type(markup);
            self.emit.tag_type(&ty, SyntaxHighlight::TypeColor, markup);
            self.emit.end_return_type(ret);
        }
        self.emit.end_func_proto(group);
    }

    pub(crate) fn emit_var_decl_go(
        &mut self,
        name: &str,
        ty: &str,
        count: Option<i32>,
        markup: &MarkupRef,
    ) {
        self.emit.print("var ", SyntaxHighlight::KeywordColor);
        self.emit
            .tag_variable(&identifier(name), SyntaxHighlight::VarColor, markup);
        self.emit.print(" ", SyntaxHighlight::NoColor);
        let ty = count
            .map(|n| format!("[{n}]{ty}"))
            .unwrap_or_else(|| ty.into());
        self.emit.tag_type(&ty, SyntaxHighlight::TypeColor, markup);
    }

    pub(crate) fn emit_block_inf_loop_go(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        self.context.push_mod();
        self.context
            .unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        self.emit.tag_line();
        self.emit.print("for", SyntaxHighlight::KeywordColor);
        let brace = self
            .emit
            .open_brace_indent("{", BraceStyle::SameLineRequired);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(0));
        self.emit.close_brace_indent("}", brace);
        self.flush_eol_warnings();
        self.context.pop_mod();
    }

    pub(crate) fn emit_block_while_go(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.context.push_mod();
        self.context
            .unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        let cond = fd.sblocks_ref().block(blk).get_block(0);
        let for_header = fd.sblocks_ref().block(blk).get_iterate_op().is_some();
        let overflow = fd.sblocks_ref().block(blk).has_overflow_syntax();
        let statements = self.cond_block_emits_statements(fd, cond);
        self.emit_comment_block_tree(fd, cond);
        self.emit.tag_line();
        self.emit.print("for", SyntaxHighlight::KeywordColor);
        if for_header {
            self.emit.print(" ", SyntaxHighlight::NoColor);
            self.context.push_mod();
            self.context.set_mod(modifiers::COMMA_SEPARATE);
            if let Some(op) = fd.sblocks_ref().block(blk).get_initialize_op() {
                self.emit_expression_ir(fd, arch, op);
            }
            self.emit.print("; ", SyntaxHighlight::NoColor);
            self.context.push_mod();
            self.context.unset_mod(modifiers::COMMA_SEPARATE);
            if statements {
                self.emit
                    .print("func() bool", SyntaxHighlight::KeywordColor);
                let expression = self
                    .emit
                    .open_brace_indent("{", BraceStyle::SameLineRequired);
                self.context.push_mod();
                self.context.set_mod(modifiers::NO_BRANCH);
                self.emit_block(fd, arch, cond);
                self.context.pop_mod();
                self.emit.tag_line();
                self.emit.print("return ", SyntaxHighlight::KeywordColor);
                self.context.set_mod(modifiers::ONLY_BRANCH);
                self.emit_block(fd, arch, cond);
                self.emit.close_brace_indent("}", expression);
                self.emit.print("()", SyntaxHighlight::NoColor);
            } else {
                self.context.set_mod(modifiers::ONLY_BRANCH);
                self.emit_block(fd, arch, cond);
            }
            self.context.pop_mod();
            self.emit.print("; ", SyntaxHighlight::NoColor);
            if let Some(op) = fd.sblocks_ref().block(blk).get_iterate_op() {
                self.emit_expression_ir(fd, arch, op);
            }
            self.context.pop_mod();
        } else if !overflow && !statements {
            self.emit.print(" ", SyntaxHighlight::NoColor);
            self.context.push_mod();
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, cond);
            self.context.pop_mod();
        }
        let brace = self
            .emit
            .open_brace_indent("{", BraceStyle::SameLineRequired);
        self.flush_eol_warnings();
        if !for_header && (overflow || statements) {
            self.context.push_mod();
            self.context.set_mod(modifiers::NO_BRANCH);
            self.emit_block(fd, arch, cond);
            self.context.pop_mod();
            self.emit.tag_line();
            self.emit.print(
                if overflow { "if " } else { "if !(" },
                SyntaxHighlight::KeywordColor,
            );
            self.context.push_mod();
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, cond);
            self.context.pop_mod();
            if !overflow {
                self.emit.print(")", SyntaxHighlight::NoColor);
            }
            let guard = self
                .emit
                .open_brace_indent("{", BraceStyle::SameLineRequired);
            self.emit.tag_line();
            self.emit.print("break", SyntaxHighlight::KeywordColor);
            self.emit.close_brace_indent("}", guard);
        }
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
        self.emit.close_brace_indent("}", brace);
        self.context.pop_mod();
    }

    pub(crate) fn emit_block_do_while_go(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        self.context.push_mod();
        self.context
            .unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit_any_label_statement(fd, blk);
        let name = self.go_loop_temporary(fd.sblocks_ref().block(blk).get_index());
        self.emit.tag_line();
        self.emit.print(
            &format!("for {name} := true; {name} || ("),
            SyntaxHighlight::KeywordColor,
        );
        let body = fd.sblocks_ref().block(blk).get_block(0);
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH);
        self.emit_block(fd, arch, body);
        self.context.pop_mod();
        self.emit
            .print(&format!("); {name} = false"), SyntaxHighlight::NoColor);
        let brace = self
            .emit
            .open_brace_indent("{", BraceStyle::SameLineRequired);
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, body);
        self.emit.close_brace_indent("}", brace);
        self.flush_eol_warnings();
        self.context.pop_mod();
    }

    pub(crate) fn emit_block_switch_go(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        blk: BlockId,
    ) {
        let head = fd.sblocks_ref().block(blk).get_block(0);
        self.context.push_mod();
        self.context
            .unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, head);
        self.context.pop_mod();
        self.emit.tag_line();
        self.context.push_mod();
        self.context
            .set_mod(modifiers::ONLY_BRANCH | modifiers::COMMA_SEPARATE);
        self.emit_block(fd, arch, head);
        self.context.pop_mod();
        let brace = self
            .emit
            .open_brace_indent("{", BraceStyle::SameLineRequired);
        self.flush_eol_warnings();
        let (cases, labels) = ordered_cases(fd, blk);
        self.go_suppressed_labels.extend(labels);
        for (case, fallthrough) in &cases {
            self.emit.tag_line();
            if case.isdefault {
                self.emit.print("default", SyntaxHighlight::KeywordColor);
            } else {
                self.emit.print("case ", SyntaxHighlight::KeywordColor);
                for index in 0..self.switch_case_label_count(fd, blk, case) {
                    if index > 0 {
                        self.emit.print(", ", SyntaxHighlight::NoColor);
                    }
                    let (value, size, signed) = self.switch_case_label(fd, arch, blk, case, index);
                    if let Some(op) = self
                        .case_first_op(fd, case.block)
                        .or_else(|| self.any_op(fd, case.block))
                    {
                        self.push_constant_ir_fmt_sign(
                            value,
                            size,
                            op,
                            crate::printc::display_format::NONE,
                            signed,
                        );
                        self.recurse();
                    } else {
                        let text = if signed {
                            kuna_base::address::sign_extend(value as i64, size * 8 - 1).to_string()
                        } else {
                            value.to_string()
                        };
                        self.emit.print(&text, SyntaxHighlight::ConstColor);
                    }
                }
            }
            self.emit.print(":", SyntaxHighlight::NoColor);
            let indent = self.emit.start_indent();
            if case.gototype != 0 {
                self.emit.tag_line();
                self.emit_goto_statement(fd, head, case.block, case.gototype);
            } else {
                let body = if *fallthrough {
                    fd.sblocks_ref().block(case.block).get_block(0)
                } else {
                    case.block
                };
                self.emit_block(fd, arch, body);
                if *fallthrough {
                    self.emit.tag_line();
                    self.emit
                        .print("fallthrough", SyntaxHighlight::KeywordColor);
                } else if self.emit.label_dangling() {
                    self.emit.tag_line();
                    self.emit.print("break", SyntaxHighlight::KeywordColor);
                }
            }
            self.emit.stop_indent(indent);
        }
        self.emit.close_brace_indent("}", brace);
        self.context.pop_mod();
    }
}

pub(crate) fn escape(output: &mut String, value: i32, quote: char) {
    use std::fmt::Write;
    match value {
        0 => output.push_str("\\x00"),
        7 => output.push_str("\\a"),
        8 => output.push_str("\\b"),
        9 => output.push_str("\\t"),
        10 => output.push_str("\\n"),
        11 => output.push_str("\\v"),
        12 => output.push_str("\\f"),
        13 => output.push_str("\\r"),
        92 => output.push_str("\\\\"),
        c if c == quote as i32 => {
            output.push('\\');
            output.push(quote);
        }
        32..=126 => output.push(char::from_u32(value as u32).unwrap()),
        0..=127 => {
            let _ = write!(output, "\\x{value:02x}");
        }
        128..=65535 => {
            let _ = write!(output, "\\u{value:04x}");
        }
        _ => {
            let _ = write!(output, "\\U{value:08x}");
        }
    }
}

use crate::context::{OpId, VarnodeId};
use crate::printc::op_key;
use crate::printlanguage::{Atom, SyntaxHighlight as AtomHighlight, TagType};

impl PrintC {
    pub(crate) fn emit_go_condition_expression(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        block: BlockId,
    ) {
        self.context.push_mod();
        self.context
            .unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH | modifiers::COMMA_SEPARATE);
        if self.cond_block_emits_statements(fd, block) {
            self.emit
                .print("func() bool", SyntaxHighlight::KeywordColor);
            let brace = self
                .emit
                .open_brace_indent("{", BraceStyle::SameLineRequired);
            self.context.set_mod(modifiers::NO_BRANCH);
            self.emit_block(fd, arch, block);
            self.context.unset_mod(modifiers::NO_BRANCH);
            self.emit.tag_line();
            self.emit.print("return ", SyntaxHighlight::KeywordColor);
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, block);
            self.emit.close_brace_indent("}", brace);
            self.emit.print("()", SyntaxHighlight::NoColor);
        } else {
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, block);
        }
        self.context.pop_mod();
    }

    pub(crate) fn push_go_call(&mut self, name: &str, op: OpId) {
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_atom(&Atom::syntax(
            name,
            TagType::Syntax,
            AtomHighlight::funcname_color,
        ));
    }

    pub(crate) fn push_go_pointer_conversion(
        &mut self,
        target: &Rc<Datatype>,
        source_pointer: bool,
        op: OpId,
    ) {
        self.push_cast_open(target, op);
        if !target
            .get_ptr_to()
            .is_some_and(|t| t.get_metatype() == type_metatype::TYPE_VOID)
        {
            self.push_go_call("unsafe.Pointer", op);
        }
        if !source_pointer {
            self.push_go_call("uintptr", op);
        }
    }

    pub(crate) fn push_go_conversion(
        &mut self,
        target: &Rc<Datatype>,
        source: &Rc<Datatype>,
        op: OpId,
    ) {
        let pointer = |t: &Datatype| {
            matches!(
                t.get_metatype(),
                type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL
            )
        };
        if source.get_metatype() == type_metatype::TYPE_BOOL
            && matches!(
                target.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            )
        {
            let ty = GO_SPELLER.declarator(&self.rt_ctx, target).0;
            self.push_go_call(
                &format!("func(value bool) {ty} {{ if value {{ return 1 }}; return 0 }}"),
                op,
            );
        } else if target.get_metatype() == type_metatype::TYPE_BOOL
            && source.get_metatype() != type_metatype::TYPE_BOOL
        {
            let ty = GO_SPELLER.declarator(&self.rt_ctx, source).0;
            let zero = if pointer(source) { "nil" } else { "0" };
            self.push_go_call(
                &format!("func(value {ty}) bool {{ return value != {zero} }}"),
                op,
            );
        } else if pointer(target) {
            self.push_go_pointer_conversion(target, pointer(source), op);
        } else {
            self.push_cast_open(target, op);
            if pointer(source) {
                self.push_go_call("uintptr", op);
                self.push_go_call("unsafe.Pointer", op);
            }
        }
    }

    pub(crate) fn op_go_cast(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(operation) = fd.obank().get(op) else {
            return;
        };
        let (Some(input), Some(output)) = (operation.get_in(0), operation.get_out()) else {
            return;
        };
        let target = fd.vn_type_def_facing(output);
        self.push_go_value_as(fd, arch, input, op, &target);
    }

    pub(crate) fn go_value_type(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        vn: VarnodeId,
        op: OpId,
    ) -> Rc<Datatype> {
        let value = fd.vbank().get(vn).unwrap();
        if !value.is_implied() && !value.is_constant() {
            if let Some(ty) = crate::printc::declared_variable_type(fd, arch.decl_high_type, vn) {
                return ty;
            }
        }
        fd.vn_type_read_facing(vn, op)
    }

    pub(crate) fn op_go_ptradd(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let Some(operation) = fd.obank().get(op) else {
            return;
        };
        let (Some(base), Some(index), Some(scale)) = (
            operation.get_in(0),
            operation.get_in(1),
            operation.get_in(2),
        ) else {
            return;
        };
        let Some(output) = operation.get_out() else {
            return;
        };
        let target = fd.vn_type_def_facing(output);
        let source = self.go_value_type(fd, arch, base, op);
        let value = self
            .context
            .is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let mods =
            self.context.mods() & !(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        if value {
            self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
        }
        self.push_cast_open(&target, op);
        self.push_go_call("unsafe.Add", op);
        self.push_op(&tokens::COMMA, Some(op_key(op)));
        self.push_go_call("unsafe.Pointer", op);
        if !matches!(
            source.get_metatype(),
            type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL
        ) {
            self.push_go_call("uintptr", op);
        }
        self.push_vn_ir_m(fd, arch, base, op, mods);
        self.push_op(&tokens::MULTIPLY, Some(op_key(op)));
        self.push_vn_ir_m(fd, arch, index, op, mods);
        self.push_vn_ir_m(fd, arch, scale, op, mods);
    }

    pub(crate) fn op_go_bit_transfer(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
        transfer: crate::kuna_bitcast::Transfer,
    ) {
        let bits = transfer.size * 8;
        if transfer.to_float {
            self.push_go_call(&format!("math.Float{bits}frombits"), op);
            self.push_go_call(&format!("uint{bits}"), op);
            if transfer.source_pointer {
                self.push_go_call("uintptr", op);
                self.push_go_call("unsafe.Pointer", op);
            }
        } else {
            if matches!(
                transfer.target.get_metatype(),
                type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL
            ) {
                self.push_go_pointer_conversion(&transfer.target, false, op);
            } else {
                self.push_cast_open(&transfer.target, op);
            }
            if !fd.vbank().get(transfer.input).unwrap().is_constant() {
                self.push_go_call(&format!("math.Float{bits}bits"), op);
            }
        }
        let value = fd.vbank().get(transfer.input).unwrap();
        if value.is_constant() {
            let raw = value.get_offset();
            let literal = if !transfer.to_float
                && transfer.target.get_metatype() == type_metatype::TYPE_INT
            {
                let shift = 64 - bits as u32;
                (((raw << shift) as i64) >> shift).to_string()
            } else {
                format!("0x{raw:x}")
            };
            self.push_atom(&Atom::syntax(
                literal,
                TagType::Syntax,
                AtomHighlight::const_color,
            ));
        } else {
            self.push_vn_ir_body(fd, arch, transfer.input, op);
        }
    }
}

#[cfg(test)]
mod tests;

impl PrintC {
    pub(crate) fn push_go_value_as(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
    ) {
        let source = self.go_value_type(fd, arch, value, op);
        let vn = fd.vbank().get(value).unwrap();
        let integer = |t: &Datatype| {
            matches!(
                t.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            ) && matches!(t.get_size(), 1 | 2 | 4 | 8)
        };
        if vn.is_constant() && integer(&source) && integer(target) {
            let bits = (target.get_size() * 8) as u32;
            let raw = vn.get_offset() & kuna_base::address::calc_mask(target.get_size());
            let literal = if target.get_metatype() == type_metatype::TYPE_INT {
                (((raw << (64 - bits)) as i64) >> (64 - bits)).to_string()
            } else {
                format!("0x{raw:x}")
            };
            self.push_atom(&Atom::syntax(
                literal,
                TagType::Syntax,
                AtomHighlight::const_color,
            ));
            return;
        }
        if GO_SPELLER.declarator(&self.rt_ctx, target).0
            != GO_SPELLER.declarator(&self.rt_ctx, &source).0
        {
            self.push_go_conversion(target, &source, op);
        }
        self.push_vn_ir(fd, arch, value, op);
    }

    pub(crate) fn op_go_binary(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        token: &'static OpToken,
        op: OpId,
    ) -> bool {
        use kuna_num::opcodes::OpCode::*;
        let Some(operation) = fd.obank().get(op) else {
            return false;
        };
        let (Some(left), Some(right), Some(output)) = (
            operation.get_in(0),
            operation.get_in(1),
            operation.get_out(),
        ) else {
            return false;
        };
        let opcode = operation.code();
        let comparison = matches!(
            opcode,
            CPUI_INT_EQUAL
                | CPUI_INT_NOTEQUAL
                | CPUI_INT_LESS
                | CPUI_INT_LESSEQUAL
                | CPUI_INT_SLESS
                | CPUI_INT_SLESSEQUAL
                | CPUI_FLOAT_EQUAL
                | CPUI_FLOAT_NOTEQUAL
                | CPUI_FLOAT_LESS
                | CPUI_FLOAT_LESSEQUAL
        );
        let boolean = matches!(opcode, CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_BOOL_XOR);
        let float = matches!(
            opcode,
            CPUI_FLOAT_ADD
                | CPUI_FLOAT_SUB
                | CPUI_FLOAT_MULT
                | CPUI_FLOAT_DIV
                | CPUI_FLOAT_EQUAL
                | CPUI_FLOAT_NOTEQUAL
                | CPUI_FLOAT_LESS
                | CPUI_FLOAT_LESSEQUAL
        );
        let shift = matches!(opcode, CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT);
        let outtype = fd.vn_type_def_facing(output);
        let mut target = outtype.clone();
        if comparison || boolean {
            let size = fd.vbank().get(left).unwrap().get_size();
            let meta = if boolean {
                type_metatype::TYPE_BOOL
            } else if float {
                type_metatype::TYPE_FLOAT
            } else if matches!(opcode, CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL) {
                type_metatype::TYPE_INT
            } else {
                type_metatype::TYPE_UINT
            };
            let Ok(ty) = arch.types().get_base(size, meta) else {
                return false;
            };
            target = ty;
        }
        let pointer = matches!(
            target.get_metatype(),
            type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL
        );
        if pointer {
            let Ok(word) = arch
                .types()
                .get_base(target.get_size(), type_metatype::TYPE_UINT)
            else {
                return false;
            };
            self.push_go_pointer_conversion(&target, false, op);
            target = word;
        }
        if !matches!(
            target.get_metatype(),
            type_metatype::TYPE_FLOAT | type_metatype::TYPE_BOOL
        ) && !matches!(target.get_size(), 1 | 2 | 4 | 8)
        {
            return false;
        }
        if shift {
            let signed = opcode == CPUI_INT_SRIGHT;
            let Ok(ty) = arch.types().get_base(
                target.get_size(),
                if signed {
                    type_metatype::TYPE_INT
                } else {
                    type_metatype::TYPE_UINT
                },
            ) else {
                return false;
            };
            if GO_SPELLER.declarator(&self.rt_ctx, &ty).0
                != GO_SPELLER.declarator(&self.rt_ctx, &target).0
            {
                self.push_cast_open(&target, op);
            }
            target = ty;
        }
        self.context.push_mod();
        self.context.unset_mod(modifiers::CONDITION_CONTEXT);
        self.push_op(token, Some(op_key(op)));
        self.push_go_value_as(fd, arch, left, op, &target);
        if shift {
            self.push_vn_ir(fd, arch, right, op);
        } else {
            self.push_go_value_as(fd, arch, right, op, &target);
        }
        self.context.pop_mod();
        true
    }
}

fn ordered_cases(
    fd: &Funcdata,
    block: BlockId,
) -> (Vec<(crate::block::CaseOrder, bool)>, Vec<BlockId>) {
    use crate::block::BlockType;
    use std::collections::HashMap;
    let graph = fd.sblocks_ref();
    let cases = graph.block(block).switch_caseblocks();
    let mut counts = HashMap::new();
    let mut stack = graph.root.into_iter().collect::<Vec<_>>();
    while let Some(id) = stack.pop() {
        let b = graph.block(id);
        if let Some(front) = b.get_goto_target().and_then(|t| graph.get_front_leaf(t)) {
            *counts.entry(front).or_insert(0usize) += 1;
        }
        for case in b.switch_caseblocks().iter().filter(|c| c.gototype != 0) {
            if let Some(front) = graph.get_front_leaf(case.block) {
                *counts.entry(front).or_insert(0usize) += 1;
            }
        }
        stack.extend_from_slice(b.get_list());
    }
    let edges = cases
        .iter()
        .map(|case| {
            if case.gototype != 0 || graph.block(case.block).get_type() != BlockType::Goto {
                return None;
            }
            let target = graph.block(case.block).get_goto_target()?;
            let front = graph.get_front_leaf(target)?;
            if counts.get(&front) != Some(&1) {
                return None;
            }
            cases
                .iter()
                .position(|c| graph.get_front_leaf(c.block) == Some(front))
        })
        .collect::<Vec<_>>();
    let mut incoming = vec![0; cases.len()];
    for target in edges.iter().flatten() {
        incoming[*target] += 1;
    }
    let starts = (0..cases.len())
        .filter(|i| incoming[*i] == 0)
        .chain(0..cases.len());
    let mut visited = vec![false; cases.len()];
    let mut result = Vec::new();
    let mut labels = Vec::new();
    for mut current in starts {
        loop {
            if visited[current] {
                break;
            }
            visited[current] = true;
            let next = edges[current].filter(|n| !visited[*n]);
            result.push((cases[current].clone(), next.is_some()));
            let Some(next) = next else { break };
            if let Some(front) = graph.get_front_leaf(cases[next].block) {
                labels.push(front);
            }
            current = next;
        }
    }
    (result, labels)
}
