//! (kuna) The output-language plane: which surface language P9 renders into.
//!
//! # Why this exists
//!
//! The C++ tree kuna was ported from had a three-level printer hierarchy —
//! `PrintLanguage` (the RPN driver and the mod/scope stacks) → `PrintC` →
//! `PrintJava : public PrintC` — selected through a `PrintLanguageCapability`
//! registry. The port flattened all of it into one concrete `PrintC`
//! (`p9_emit/printc.rs`), and `p9_emit/printjava.rs` is the stub that records
//! the loss. This module re-erects the seam, in the shape kuna actually needs.
//!
//! # The shape, and why it is not a trait object over the whole printer
//!
//! `PrintC` (`p9_emit/printc.rs`) is **parameterized**, not subclassed: it
//! carries one [`OutLang`] field and every language-varying site reads a
//! `&'static` policy object instead of a `keywords::`/`tokens::` constant. The
//! RPN driver, `op_push_ir`, `push_vn`, `emit_atom`, `parentheses_top`, the
//! `CastStrategy` plumbing, the `CommentSorter` and the markup emitter are
//! shared verbatim — there is no second printer and no duplicated emitter.
//!
//! A `Box<dyn PrintLanguage>` over the whole printer (the C++ shape) would put
//! vtable dispatch in the RPN inner loop and make byte-identity unprovable by
//! inspection; a second `PrintRust` struct would fork ~4000 lines, which is the
//! choice the reference implementation (SEFCOM Oxidizer, merged into angr as
//! `structured_codegen/rust.py`) made and then drifted from inside one release
//! cycle. Parameterization keeps exactly one implementation of "emit an if".
//!
//! # The byte-identity invariant
//!
//! Every [`LANG_C`] field **is** the `keywords::`/`tokens::` constant it
//! replaces, asserted field-by-field in this module's tests. Reading the C
//! profile at a call site therefore produces the identical token, so the
//! migration is a provably byte-identical rewrite and `docs/baseline.json` is
//! never re-pinned.

use crate::printc::{keywords, tokens};
use crate::printlanguage::OpToken;

/// The surface language P9 renders the recovered function into.
///
/// `C` is the corpus-pinned default: only its rendering is asserted by
/// `docs/baseline.json`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum OutLang {
    /// The c-language back-end (`printc.cc`'s successor).
    #[default]
    C,
    /// The rust-language back-end.
    Rust,
}

impl OutLang {
    /// Resolve from an `option setlanguage` name, or `None` when the name names
    /// no back-end kuna can emit.
    pub fn from_print_name(name: &str) -> Option<OutLang> {
        match name {
            "c-language" | "c" => Some(OutLang::C),
            "rust-language" | "rust" => Some(OutLang::Rust),
            _ => None,
        }
    }

    /// The `option setlanguage` name this language answers to.
    pub fn print_name(self) -> &'static str {
        match self {
            OutLang::C => "c-language",
            OutLang::Rust => "rust-language",
        }
    }

    /// The surface vocabulary and capability record.
    pub fn profile(self) -> &'static LangProfile {
        match self {
            OutLang::C => &LANG_C,
            OutLang::Rust => &crate::kuna_langrust::LANG_RUST,
        }
    }

    /// How this language spells a recovered type.
    pub fn speller(self) -> &'static dyn crate::kuna_langtypes::TypeSpeller {
        match self {
            OutLang::C => &crate::kuna_langc::C_SPELLER,
            OutLang::Rust => &crate::kuna_rusttypes::RUST_SPELLER,
        }
    }
}

/// What the emitter is **allowed** to produce in this language.
///
/// These gate the decision sites of kuna's default-ON C rendering options, so a
/// language that cannot express a construct never sees it — rather than the
/// operator being told to flip four options. `switch_captures_break` is the
/// load-bearing one: a C `switch` captures a bare `break`, so
/// `kuna_scope_break` (`p8_structure/kuna_loopbreak_recovery.rs`) legitimately retags a
/// goto-to-switch-exit as `f_break_goto`; a language whose switch does not
/// capture `break` must re-resolve that scope or emit a jump to the wrong place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LangCaps {
    /// `goto <label>;` to an arbitrary label in the function body.
    pub goto: bool,
    /// A bare `break;` inside a switch terminates the switch.
    pub switch_captures_break: bool,
    /// A case falls into the next when it does not break.
    pub switch_falls_through: bool,
    /// One arm may carry several labels (`case A: case B:` / `A | B =>`).
    pub switch_multi_label: bool,
    /// A switch with no default is legal.
    pub switch_default_optional: bool,
    /// `break 'l` / `continue 'l` naming an enclosing loop.
    pub labeled_loop_break: bool,
    /// `break 'l` out of an arbitrary labeled block.
    pub labeled_block_break: bool,
    /// Bottom-tested `do { } while (c);`.
    pub do_while: bool,
    /// C-style `for (init; cond; iter)`.
    pub c_for: bool,
    /// `cond ? a : b` as an expression.
    pub ternary: bool,
    /// `(a, b)` usable as an operand — what `option condfold` requires.
    pub comma_expression: bool,
    /// A non-boolean scalar is usable directly as a condition — what
    /// `option truthycond` (DIV-37) produces.
    pub implicit_bool_conditions: bool,
    /// A single-statement block body may drop its braces — what
    /// `option braceelide` (DIV-38) produces.
    pub brace_elision: bool,
    /// `p->f` is spelled as such (rather than requiring an explicit deref).
    pub arrow_member: bool,
}

/// The per-language surface vocabulary.
///
/// The `kw_*` and punctuation fields are the spellings; the `tok_*` fields are
/// the [`OpToken`]s whose *spelling* varies. The other ~40 operator tokens —
/// arithmetic, comparison, shifts, the ten `op=` forms — are identical across
/// every language kuna targets and stay in `printc::tokens`, un-churned.
pub struct LangProfile {
    /// The `option setlanguage` name.
    pub name: &'static str,
    /// The source-file extension for project export.
    pub file_ext: &'static str,

    pub kw_open_curly: &'static str,
    pub kw_close_curly: &'static str,
    pub kw_semicolon: &'static str,
    pub kw_colon: &'static str,
    pub kw_comma: &'static str,
    pub kw_dotdotdot: &'static str,
    pub kw_void: &'static str,
    pub kw_true: &'static str,
    pub kw_false: &'static str,
    pub kw_if: &'static str,
    pub kw_else: &'static str,
    pub kw_do: &'static str,
    pub kw_while: &'static str,
    pub kw_for: &'static str,
    pub kw_goto: &'static str,
    pub kw_break: &'static str,
    pub kw_continue: &'static str,
    pub kw_case: &'static str,
    pub kw_switch: &'static str,
    pub kw_default: &'static str,
    pub kw_return: &'static str,
    /// The `TypePointerRel` display macro (C++ `typePointerRelToken`).
    pub kw_type_pointer_rel: &'static str,

    pub tok_bitwise_not: &'static OpToken,
    pub tok_boolean_and: &'static OpToken,
    pub tok_boolean_or: &'static OpToken,
    pub tok_boolean_xor: &'static OpToken,
    pub tok_addressof: &'static OpToken,
    pub tok_dereference: &'static OpToken,
    pub tok_typecast: &'static OpToken,

    pub caps: LangCaps,
}

/// The c-language profile. Every field is the `keywords::`/`tokens::` constant
/// it replaces — see this module's byte-identity invariant.
pub static LANG_C: LangProfile = LangProfile {
    name: "c-language",
    file_ext: "c",

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

    tok_bitwise_not: &tokens::BITWISE_NOT,
    tok_boolean_and: &tokens::BOOLEAN_AND,
    tok_boolean_or: &tokens::BOOLEAN_OR,
    tok_boolean_xor: &tokens::BOOLEAN_XOR,
    tok_addressof: &tokens::ADDRESSOF,
    tok_dereference: &tokens::DEREFERENCE,
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
        comma_expression: true,
        implicit_bool_conditions: true,
        brace_elision: true,
        arrow_member: true,
    },
};

#[cfg(test)]
mod tests;
