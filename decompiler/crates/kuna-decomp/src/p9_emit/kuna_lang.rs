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

use kuna_base::types::int4;

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

    /// Every language kuna can emit, by `option setlanguage` name.
    pub fn names() -> Vec<&'static str> {
        [OutLang::C, OutLang::Rust].iter().map(|l| l.print_name()).collect()
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

    /// How this language renders a recovered calling convention.
    pub fn abi(self) -> &'static dyn crate::kuna_langabi::LangAbi {
        match self {
            OutLang::C => &crate::kuna_langabi::C_ABI,
            OutLang::Rust => &crate::kuna_langabi::RUST_ABI,
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

/// A tier in an operator-precedence ladder.
///
/// The vocabulary is shared; the ORDER is per-language, which is the whole point
/// — C and Rust disagree about where the bitwise operators sit relative to
/// equality, and that disagreement is a silent wrong answer rather than a syntax
/// error when it is got wrong (see [`crate::kuna_langrust`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prec {
    Comma,
    Assign,
    LogOr,
    /// Ghidra's invented C `^^`. Rust has no separate tier for it — its `^` is
    /// the bitwise operator — so a Rust ladder places it with [`Prec::BitXor`].
    LogXor,
    LogAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    /// `< <= > >=`. C ranks these above [`Prec::Equality`]; Rust puts both in one
    /// non-associative tier.
    Relational,
    Shift,
    Additive,
    Multiplicative,
    /// Rust's `as`. C has no such tier — its cast is prefix and sits at
    /// [`Prec::Unary`] — so this appears only in a Rust ladder.
    Cast,
    Unary,
    Postfix,
    Scope,
}

/// A language's precedence ladder: `(tier, rank)` pairs, loosest first.
///
/// The rank is explicit rather than positional, because a language remaps only
/// the tokens whose tier it MOVES — every other token keeps the ported C table's
/// number. Positional ranks would put the two groups on different scales, and
/// the parenthesizer compares them directly, so the ladder has to speak the same
/// numbers the table does. `ladder_is_consistent_with` asserts exactly that: a
/// tier a language does not move must keep its C rank, or the two halves of the
/// table disagree.
///
/// Declaring the order and the numbers together, in one place, is what makes an
/// out-of-step operator a test failure rather than a silent wrong answer.
pub struct PrecLadder(pub &'static [(Prec, int4)]);

impl PrecLadder {
    /// The rank of `p`, or `None` if this language has no such tier.
    pub fn rank(&self, p: Prec) -> Option<int4> {
        self.0.iter().find(|(t, _)| *t == p).map(|(_, r)| *r)
    }

    /// [`PrecLadder::rank`] in a const context, so a token table DERIVES its
    /// numbers from the ladder instead of restating them. `-1` for a tier this
    /// language does not rank -- not a valid precedence, so it fails loudly.
    pub const fn const_rank(&self, p: Prec) -> int4 {
        let mut i = 0;
        while i < self.0.len() {
            if self.0[i].0 as u8 == p as u8 {
                return self.0[i].1;
            }
            i += 1;
        }
        -1
    }

    /// The ladder in declared order, for tests.
    pub fn tiers(&self) -> impl Iterator<Item = (Prec, int4)> + '_ {
        self.0.iter().copied()
    }
}

/// The ladder tier a token from the ported C table belongs to.
///
/// Read off the token's own precedence, which is bijective with the tier for
/// every value in that table -- except `TYPECAST`, whose C rank it shares with
/// the unary operators but whose Rust spelling (`as`) has a tier of its own.
/// That one is resolved by pointer identity.
pub fn prec_of(tok: &'static OpToken) -> Option<Prec> {
    if std::ptr::eq(tok, &tokens::TYPECAST) {
        return Some(Prec::Cast);
    }
    Some(match tok.precedence {
        2 => Prec::Comma,
        14 => Prec::Assign,
        18 => Prec::LogOr,
        20 => Prec::LogXor,
        22 => Prec::LogAnd,
        26 => Prec::BitOr,
        30 => Prec::BitXor,
        34 => Prec::BitAnd,
        38 => Prec::Equality,
        42 => Prec::Relational,
        46 => Prec::Shift,
        50 => Prec::Additive,
        54 => Prec::Multiplicative,
        62 => Prec::Unary,
        66 => Prec::Postfix,
        70 => Prec::Scope,
        _ => return None,
    })
}

/// C's ladder, transcribed from the ported `printc::tokens` table. Asserted
/// against that table token-by-token in this module's tests, so it is a
/// restatement of the C precedences rather than a second opinion about them.
pub const C_LADDER: PrecLadder = PrecLadder(&[
    (Prec::Comma, 2),
    (Prec::Assign, 14),
    (Prec::LogOr, 18),
    (Prec::LogXor, 20),
    (Prec::LogAnd, 22),
    (Prec::BitOr, 26),
    (Prec::BitXor, 30),
    (Prec::BitAnd, 34),
    (Prec::Equality, 38),
    (Prec::Relational, 42),
    (Prec::Shift, 46),
    (Prec::Additive, 50),
    (Prec::Multiplicative, 54),
    // C's cast is prefix and shares the unary tier.
    (Prec::Cast, 62),
    (Prec::Unary, 62),
    (Prec::Postfix, 66),
    (Prec::Scope, 70),
]);

/// How the function signature is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtoForm {
    /// `<ret> name(<params>)` -- C.
    CPrefixReturn,
    /// `unsafe fn name(<params>) -> <ret>` -- Rust.
    RustFnArrow,
}

/// How a local declaration is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclForm {
    /// `<type> name;` (array count after the name) -- C.
    CTypeThenName,
    /// `let mut name: <type>;` (array count inside the type) -- Rust.
    RustLetColon,
}

/// How a multi-way branch is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchForm {
    /// `switch (v) { case N: ... break; default: ... }` -- C.
    CSwitch,
    /// `match v { N | M => { ... } _ => {} }` -- Rust.
    RustMatch,
}

/// How an unconditional loop is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfLoopForm {
    /// `do { ... } while( true );` -- C.
    CDoWhileTrue,
    /// `loop { ... }` -- Rust.
    RustLoop,
}

/// How a top-tested loop is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhileForm {
    /// `while (c) { ... }`, and `while( true ) { ...; if (c) break; ... }` for the
    /// overflow form -- C.
    CParenWhile,
    /// `while c { ... }`, and `loop { ...; if c { break; } ... }` for the overflow
    /// form -- Rust.
    RustBareWhile,
}

/// How a bottom-tested loop is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoWhileForm {
    /// `do { ... } while (c);` -- C.
    CDoWhile,
    /// `loop { ...; if !(c) { break; } }` -- Rust.
    RustLoopBreakIf,
}

/// How a counted loop is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForLoopForm {
    /// `for (init; cond; iter) { ... }` -- C.
    CForHeader,
    /// No header form: render as `while`. The `for` reroll is disabled upstream
    /// (P6 `analyze_for_loops`) so the init/iterate ops stay where the CFG put
    /// them -- moving them back at print time would let a `continue` skip the
    /// increment.
    DegradeToWhile,
}

/// How an unstructured jump is rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GotoForm {
    /// `goto label;` -- C.
    CGoto,
    /// No form: a comment plus a diverging marker. See `LangCaps::goto`.
    Unrepresentable,
}

/// How a jump target is labelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelForm {
    /// `label:` -- C.
    CColon,
    /// A comment: Rust has no statement labels, only loop labels.
    CommentOnly,
}

/// How a field is reached through a pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberForm {
    /// `p->f` -- C.
    CArrow,
    /// `(*p).f` -- Rust raw pointers have no auto-deref.
    RustDerefParen,
}

/// How a character constant is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharForm {
    /// `'a'`, `'\xff'`, `L'a'` -- C, where a char literal is an integer of the
    /// declared width and `\x` admits any byte.
    CQuoted,
    /// `b'a'` -- Rust, where a `char` is a 4-byte Unicode scalar with a validity
    /// invariant and `'\xff'` does not exist. Only a 1-byte value with a
    /// printable-ASCII or standard-escape spelling takes this form; anything else
    /// falls back to the integer, which is always exact.
    RustByte,
}

/// How a string literal's body is escaped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringEscape {
    /// C's set, which includes `\a`, `\b`, `\v`, `\f` and an escaped `\'`.
    CEscapes,
    /// Rust's set. It has no `\a`/`\b`/`\v`/`\f` (those become `\xNN`), and a
    /// single quote inside a double-quoted string must NOT be escaped -- `\'` is
    /// only an escape in a character literal, so `"PCRE\'s"` does not tokenize.
    RustEscapes,
}

/// How a conversion is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastForm {
    /// `(T)x` -- C.
    PrefixParen,
    /// `x as T` -- Rust.
    PostfixAs,
}

/// The statement and expression *shapes* the emitters branch on.
///
/// Data, not a trait: every one of these is a plain choice with no behaviour, and
/// a struct of enums keeps the whole language definition readable in one place.
/// The emitter matches on the form; adding a language adds a variant, and the
/// compiler enumerates every site that must answer for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LangForms {
    pub proto: ProtoForm,
    pub decl: DeclForm,
    pub switch: SwitchForm,
    pub inf_loop: InfLoopForm,
    pub while_loop: WhileForm,
    pub do_while: DoWhileForm,
    pub for_loop: ForLoopForm,
    pub goto: GotoForm,
    pub label: LabelForm,
    pub member: MemberForm,
    pub cast: CastForm,
    pub char_lit: CharForm,
    pub string_escape: StringEscape,
}

/// Outer attributes emitted above a function, if the language wants any.
pub type FnAttributes = &'static [&'static str];

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
    /// An `if`/`while` condition is parenthesised.
    pub paren_conditions: bool,
    /// Integer literals carry `U`/`L`/`LL` type suffixes.
    pub integer_suffixes: bool,
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
    /// The spelling of a null pointer constant (`option nullprinting`).
    pub null_literal: &'static str,
    /// Recovered names are rewritten to the language's identifier charset before
    /// they reach the page. `false` for C, which emits demangled paths verbatim.
    pub sanitize_identifiers: bool,

    /// Map a token from the opcode table onto this language's spelling and
    /// precedence.
    ///
    /// Most of the ~46 operator tokens are the right token in every language
    /// kuna targets, so a whole parallel table would be noise. The ones that are
    /// not fall into two groups, and the second is the dangerous one:
    ///
    /// * **spelling** -- Rust writes bitwise complement `!`, not `~`.
    /// * **precedence** -- C puts `& ^ |` BELOW `== !=`, Rust puts them ABOVE.
    ///   The parenthesizer decides parens from precedence, so leaving C's numbers
    ///   in place would emit `a | b == c` for `(a | b) == c`, which Rust reads as
    ///   `a | (b == c)`. That is a silent wrong answer, not a syntax error, which
    ///   is why the mapping is a language property rather than a spelling table.
    ///
    /// C's is the identity.
    pub map_token: fn(&'static OpToken) -> &'static OpToken,
    /// The conversion token, reached directly by the cast emitters.
    pub tok_typecast: &'static OpToken,

    pub caps: LangCaps,
    pub forms: LangForms,
    /// Outer attributes emitted above the function (Rust `#[allow(...)]`).
    pub fn_attributes: FnAttributes,
}

/// C's token mapping: the identity, because `printc::tokens` IS the C table.
fn c_map_token(tok: &'static OpToken) -> &'static OpToken {
    tok
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
    null_literal: "NULL",
    sanitize_identifiers: false,

    map_token: c_map_token,
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
        paren_conditions: true,
        integer_suffixes: true,
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
        string_escape: StringEscape::CEscapes,
    },
    fn_attributes: &[],
};

#[cfg(test)]
mod tests;
