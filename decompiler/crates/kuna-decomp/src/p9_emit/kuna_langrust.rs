//! (kuna) The rust-language policy objects: the profile and capability record
//! for Rust output.
//!
//! The type speller lives next door in [`crate::kuna_rusttypes`]; this file owns
//! the surface vocabulary and the capability record that tells the emitters what
//! Rust is willing to be handed.
//!
//! # The capabilities that matter, and why they are capabilities
//!
//! Three of kuna's rendering defaults are **on** and produce text that is not
//! Rust: `truthycond` (DIV-37) renders `x != 0` as `x`, which is not a condition
//! in a language without implicit bool conversion; `braceelide` (DIV-38) drops
//! the braces from a single-statement body, which Rust requires; and
//! `nullprinting` (DIV-35) emits `NULL`. Expressing those as `LangCaps` members
//! rather than telling an operator to flip three options is what makes selecting
//! Rust a single correct choice.
//!
//! `switch_captures_break: false` is the load-bearing one. A C `switch` captures
//! a bare `break`, so `kuna_scope_break` (`p8_structure/kuna_loopbreak_recovery.rs`,
//! DIV-10, default-on) legitimately retags a goto that targets the switch exit as
//! `f_break_goto`. A Rust `match` does **not** capture `break`: the same flag,
//! rendered naively inside a `match` arm inside a loop, breaks the *loop*. The
//! capability is what turns that from a latent miscompile into a gate.
//!
//! # Token precedences
//!
//! Rust's `& ^ | && ||` ladder has the same relative order as C's, so the
//! inherited table is already correct — with one exception. Ghidra's
//! `BOOLEAN_XOR` is the invented C spelling `^^` at precedence 20 (between `&&`
//! and `||`); Rust spells logical xor `^`, which binds at the *bitwise* tier.
//! Emitting `^` while keeping precedence 20 would make the parenthesizer omit
//! parentheses Rust needs, so the Rust token carries precedence 30.

use crate::kuna_lang::{LangCaps, LangProfile};
use crate::printc::tokens;
use crate::printlanguage::{OpToken, TokenType};

/// The Rust operator tokens whose spelling or precedence differs from C.
/// Everything else is inherited from `printc::tokens` unchanged.
pub mod rust_tokens {
    use super::{OpToken, TokenType};

    /// Rust spells bitwise complement `!`, not `~`; same prefix-unary tier.
    pub static BITWISE_NOT: OpToken = OpToken {
        print1: "!",
        print2: "",
        stage: 1,
        precedence: 62,
        associative: false,
        token_type: TokenType::UnaryPrefix,
        spacing: 0,
        bump: 0,
        negate: None,
    };

    /// Rust spells logical xor `^` at the bitwise tier (precedence 30), not at
    /// Ghidra's invented `^^` tier (20). See the module docs.
    pub static BOOLEAN_XOR: OpToken = OpToken {
        print1: "^",
        print2: "",
        stage: 2,
        precedence: 30,
        associative: true,
        token_type: TokenType::Binary,
        spacing: 1,
        bump: 0,
    negate: None,
    };

    /// `expr as T` — postfix-typed rather than C's prefix `(T)expr`. Binds
    /// tighter than the multiplicative operators (54) and looser than unary (62).
    pub static TYPECAST_AS: OpToken = OpToken {
        print1: " as ",
        print2: "",
        stage: 2,
        precedence: 58,
        associative: false,
        token_type: TokenType::Binary,
        spacing: 0,
        bump: 0,
        negate: None,
    };
}

/// The rust-language profile.
///
/// Keywords with no Rust spelling (`case`, `goto`, `do`, `for`) are the empty
/// string; the matching [`LangCaps`] member forbids the form that would emit
/// them, so an empty spelling can never reach the output.
pub static LANG_RUST: LangProfile = LangProfile {
    name: "rust-language",
    file_ext: "rs",

    kw_open_curly: "{",
    kw_close_curly: "}",
    kw_semicolon: ";",
    kw_colon: ":",
    kw_comma: ",",
    // Rust variadics are `...` only in `extern` blocks; a recovered varargs
    // prototype spells the same.
    kw_dotdotdot: "...",
    kw_void: "()",
    kw_true: "true",
    kw_false: "false",
    kw_if: "if",
    kw_else: "else",
    // No `do`/`for`/`goto`/`case`: see `caps`.
    kw_do: "",
    kw_while: "while",
    kw_for: "",
    kw_goto: "",
    kw_break: "break",
    kw_continue: "continue",
    kw_case: "",
    kw_switch: "match",
    kw_default: "_",
    kw_return: "return",
    // `ADJ` is a Ghidra display macro for a `TypePointerRel`, not a language
    // construct; it renders the same in any output.
    kw_type_pointer_rel: "ADJ",

    tok_bitwise_not: &rust_tokens::BITWISE_NOT,
    tok_boolean_and: &tokens::BOOLEAN_AND,
    tok_boolean_or: &tokens::BOOLEAN_OR,
    tok_boolean_xor: &rust_tokens::BOOLEAN_XOR,
    tok_addressof: &tokens::ADDRESSOF,
    tok_dereference: &tokens::DEREFERENCE,
    tok_typecast: &rust_tokens::TYPECAST_AS,

    caps: LangCaps {
        goto: false,
        switch_captures_break: false,
        switch_falls_through: false,
        switch_multi_label: true,
        switch_default_optional: false,
        labeled_loop_break: true,
        labeled_block_break: true,
        do_while: false,
        c_for: false,
        ternary: false,
        comma_expression: false,
        implicit_bool_conditions: false,
        brace_elision: false,
        arrow_member: false,
    },
};

#[cfg(test)]
mod tests;
