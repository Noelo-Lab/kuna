//! (kuna) The java-language policy objects, after `printjava.cc`.
//!
//! Upstream `PrintJava` is `PrintC` with a handful of overrides; kuna's printer
//! is parameterized rather than subclassed, so the overrides become a profile,
//! a type speller and a few `OutLang::Java` arms in `printc.rs`:
//!
//! - `>>>` for a logical right shift (`TypeOp::selectJavaOperators`), `null`,
//!   and Java's string escapes (`PrintJava::printUnicode`);
//! - a Java reference is a Ghidra pointer, so the speller strips pointer levels:
//!   a pointer to a class structure is the class name, and a pointer to a
//!   primitive or to another reference is an array (`PrintJava::isArrayType`,
//!   `pushTypeStart`);
//! - member access through an object reference is `.`, `instanceof` is a binary
//!   operator, and a call's `this` argument is the object the method is selected
//!   from (`opCpoolRefOp`, `opCallind`), handled in `printc.rs`.
//!
//! The capability record turns off what Java cannot express: comma expressions,
//! integer-as-condition tests, and the `U`/`L` literal suffixes.

use std::borrow::Cow;
use std::rc::Rc;

use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype};
use crate::kuna_lang::{
    CastForm, CharForm, DeclForm, DoWhileForm, ForLoopForm, GotoForm, InfLoopForm, LabelForm,
    LangCaps, LangForms, LangProfile, MemberForm, ProtoForm, StringEscape, SwitchForm, WhileForm,
};
use crate::kuna_langtypes::{SpellCtx, TypeSpeller};
use crate::printc::{keywords, op_token, tokens};
use crate::printlanguage::{OpToken, TokenType};

/// Java's unsigned right shift.
pub static SHIFT_RIGHT_UNSIGNED: OpToken = op_token(">>>", "", 2, 46, false, TokenType::Binary, 1, 0);
/// Java's unsigned right-shift assignment.
pub static RIGHTEQUAL_UNSIGNED: OpToken = op_token(">>>=", "", 2, 14, false, TokenType::Binary, 1, 5);
/// Java's `throw`, the statement Dalvik's `throwException(x)` performs.
pub static THROW: OpToken = op_token("throw ", "", 1, 2, false, TokenType::UnaryPrefix, 0, 0);
/// Java's `instanceof` (`PrintJava::instanceof`).
pub static INSTANCEOF: OpToken = op_token("instanceof", "", 2, 60, true, TokenType::Binary, 1, 0);

fn java_map_token(tok: &'static OpToken) -> &'static OpToken {
    if std::ptr::eq(tok, &tokens::SHIFT_RIGHT) {
        &SHIFT_RIGHT_UNSIGNED
    } else if std::ptr::eq(tok, &tokens::RIGHTEQUAL) {
        &RIGHTEQUAL_UNSIGNED
    } else {
        tok
    }
}

/// The java-language profile.
pub static LANG_JAVA: LangProfile = LangProfile {
    name: "java-language",
    file_ext: "java",

    kw_open_curly: keywords::OPEN_CURLY,
    kw_close_curly: keywords::CLOSE_CURLY,
    kw_semicolon: keywords::SEMICOLON,
    kw_colon: keywords::COLON,
    kw_comma: keywords::COMMA,
    kw_dotdotdot: keywords::DOTDOTDOT,
    kw_void: keywords::KEYWORD_VOID,
    kw_true: keywords::KEYWORD_TRUE,
    kw_false: keywords::KEYWORD_FALSE,
    kw_if: keywords::KEYWORD_IF,
    kw_else: keywords::KEYWORD_ELSE,
    kw_do: keywords::KEYWORD_DO,
    kw_while: keywords::KEYWORD_WHILE,
    kw_for: keywords::KEYWORD_FOR,
    kw_goto: keywords::KEYWORD_GOTO,
    kw_break: keywords::KEYWORD_BREAK,
    kw_continue: keywords::KEYWORD_CONTINUE,
    kw_case: keywords::KEYWORD_CASE,
    kw_switch: keywords::KEYWORD_SWITCH,
    kw_default: keywords::KEYWORD_DEFAULT,
    kw_return: keywords::KEYWORD_RETURN,
    kw_type_pointer_rel: keywords::TYPE_POINTER_REL_TOKEN,
    null_literal: "null",
    sanitize_identifiers: false,

    map_token: java_map_token,
    tok_typecast: &tokens::TYPECAST,

    caps: LangCaps {
        goto: true,
        switch_captures_break: true,
        switch_falls_through: true,
        switch_multi_label: true,
        switch_default_optional: true,
        labeled_loop_break: false,
        labeled_block_break: false,
        do_while: true,
        c_for: true,
        ternary: true,
        comma_expression: false,
        implicit_bool_conditions: false,
        brace_elision: true,
        arrow_member: true,
        paren_conditions: true,
        integer_suffixes: false,
        integer_promotion: true,
        signed_text_byte: false,
    },
    forms: LangForms {
        proto: ProtoForm::CPrefixReturn,
        decl: DeclForm::CTypeThenName,
        switch: SwitchForm::CSwitch,
        inf_loop: InfLoopForm::CDoWhileTrue,
        while_loop: WhileForm::CParenWhile,
        do_while: DoWhileForm::CDoWhile,
        for_loop: ForLoopForm::CForHeader,
        goto: GotoForm::CGoto,
        label: LabelForm::CColon,
        member: MemberForm::CArrow,
        cast: CastForm::PrefixParen,
        char_lit: CharForm::CQuoted,
        string_escape: StringEscape::JavaEscapes,
    },
    fn_attributes: &[],
};

/// `PrintJava::printUnicode`, with the standard `\uXXXX` escape.
pub fn print_unicode_java(s: &mut String, onechar: int4) {
    if crate::printlanguage::unicode_needs_escape(onechar) {
        let fixed = match onechar {
            0 => "\\0",
            8 => "\\b",
            9 => "\\t",
            10 => "\\n",
            12 => "\\f",
            13 => "\\r",
            92 => "\\\\",
            0x22 => "\\\"",
            0x27 => "\\'",
            _ => "",
        };
        if !fixed.is_empty() {
            s.push_str(fixed);
        } else if onechar < 0x10000 {
            s.push_str(&format!("\\u{onechar:04x}"));
        } else {
            let v = (onechar - 0x10000) as u32;
            s.push_str(&format!("\\u{:04x}\\u{:04x}", 0xd800 + (v >> 10), 0xdc00 + (v & 0x3ff)));
        }
        return;
    }
    if let Some(c) = char::from_u32(onechar as u32) {
        s.push(c);
    }
}

/// `PrintJava::isArrayType`: a Java array is a pointer to a primitive or to
/// another reference; a pointer to a class structure (or to an unsigned
/// placeholder) is a plain object reference.
pub fn is_array_type(ct: &Datatype) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    let Some(to) = ct.get_ptr_to() else { return false };
    match to.get_metatype() {
        type_metatype::TYPE_UINT => to.is_char_print(),
        type_metatype::TYPE_INT
        | type_metatype::TYPE_BOOL
        | type_metatype::TYPE_FLOAT
        | type_metatype::TYPE_PTR => true,
        _ => false,
    }
}

/// The java-language type speller.
pub struct JavaSpeller;

/// The singleton reached through `OutLang::Java.speller()`.
pub static JAVA_SPELLER: JavaSpeller = JavaSpeller;

fn int_name(size: int4) -> Option<&'static str> {
    match size {
        1 => Some("byte"),
        2 => Some("short"),
        4 => Some("int"),
        8 => Some("long"),
        _ => None,
    }
}

impl JavaSpeller {
    fn type_expr(&self, cx: &SpellCtx, ct: &Rc<Datatype>) -> String {
        let mut arrays = 0;
        let mut cur = Rc::clone(ct);
        for _ in 0..16 {
            if cur.get_metatype() != type_metatype::TYPE_PTR {
                break;
            }
            if is_array_type(&cur) {
                arrays += 1;
            }
            match cur.get_ptr_to() {
                Some(to) => cur = to,
                None => break,
            }
        }
        let mut name = match self.relabel(cx, &cur, false) {
            Some(n) => n.into_owned(),
            None if cur.get_name().is_empty() => self.anonymous(&cur),
            None => cur.get_display_name().to_string(),
        };
        for _ in 0..arrays {
            name.push_str("[]");
        }
        name
    }
}

impl TypeSpeller for JavaSpeller {
    fn relabel(&self, _cx: &SpellCtx, dt: &Rc<Datatype>, _under_pointer: bool) -> Option<Cow<'static, str>> {
        let size = dt.get_size();
        let name = match dt.get_metatype() {
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT if dt.is_char_print() => Some("char"),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => int_name(size),
            type_metatype::TYPE_FLOAT if size == 4 => Some("float"),
            type_metatype::TYPE_FLOAT if size == 8 => Some("double"),
            type_metatype::TYPE_BOOL => Some("boolean"),
            type_metatype::TYPE_VOID => Some("void"),
            type_metatype::TYPE_CODE => Some("Object"),
            _ => None,
        };
        name.map(Cow::Borrowed)
    }

    fn anonymous(&self, dt: &Rc<Datatype>) -> String {
        match dt.get_metatype() {
            type_metatype::TYPE_VOID => "void".to_string(),
            _ => int_name(dt.get_size()).map_or_else(|| "Object".to_string(), str::to_string),
        }
    }

    fn declarator(&self, cx: &SpellCtx, ct: &Rc<Datatype>) -> (String, String) {
        (self.type_expr(cx, ct), String::new())
    }

    fn type_name(&self, cx: &SpellCtx, t: &Rc<Datatype>) -> (String, String) {
        (self.type_expr(cx, t), String::new())
    }

    fn unknown_base(&self, size: int4, _under_pointer: bool, _long_is_8: bool) -> Option<Cow<'static, str>> {
        int_name(size).map(Cow::Borrowed)
    }
}
