//! Port of `decompiler/cpp/printc.{cc,hh}` — the c-language token emitter
//! (`PrintC`), the concrete `PrintLanguage` back-end the M2/M3 datatests
//! string-match against.
//!
//! ## What this module ports faithfully *now*
//!
//! `PrintC` is a 3.9k-line subclass of `PrintLanguage`.  As of the
//! `w10-printc-body` item the **`PrintLanguage` RPN driver** (`pushOp`/
//! `pushAtom`/`pushVn`/`recurse`/`emitOp`/`emitAtom`/`opBinary`/`opUnary`/
//! `parentheses`, printlanguage.cc:129-580) is **ported here** (the
//! `impl PrintC` RPN-driver block) and drives the real `Emit` low-level driver
//! ([`crate::prettyprint`]'s `EmitNoMarkup`).  It is byte-faithfully
//! unit-tested against the C++ `emitOp`/`emitAtom`/`parentheses` logic (the
//! `rpn_*` tests).
//!
//! What still cannot be driven over a real function body is the IR-coupled leaf
//! of the driver — `recurse`'s implied-op `defOp->getOpcode()->push(...)`
//! dispatch and `pushVnExplicit`'s Symbol/HighVariable/Datatype/constant
//! resolution — together with the structured-/flat-block walk (`emitBlock*`,
//! printc.cc:2827-3514) and the real-prototype signature.  These are blocked
//! **upstream of the printer**: the merged tree's decompilation passes
//! (heritage / simplification / merge / type + proto recovery / block
//! structuring) are seam stubs, so the IR reaching the printer is *raw lifted
//! p-code* (no HighVariables-with-symbols, no recovered types, **empty
//! `sblocks`**, a `void NAME(void)` proto stub).  Printing it would emit non-C
//! garbage, not byte-parity.  Those edges are `// SEAM(decompile-passes)`
//! (LOSS-130 / W10) and fall to the upstream pass items; the RPN engine they
//! feed is in place.  The parity gate `tests/printc_parity.rs` measures this
//! honestly: it decompiles + prints >= 8 corpus functions and byte-compares
//! each against the C++ oracle, reporting the (upstream-bounded) match count.
//!
//! What **is** ported faithfully here, and is what this module's tests exercise:
//!
//!   1. **The operator-token table** ([`tokens`]) — every `PrintC::*` `OpToken`
//!      static (printc.cc:24-78) with its exact precedence / associativity /
//!      token-type / spacing / bump, plus the negate cross-links
//!      (printc.cc:130-135) realized as [`token_negate`].  This is *the*
//!      precedence/parenthesization data that drives [`crate::printlanguage::parentheses`]
//!      — the print-fidelity-critical decision.
//!   2. **The keyword / punctuation constants** ([`keywords`], printc.cc:80-104).
//!   3. **`PrintCCapability`** registration metadata ([`CAPABILITY_NAME`],
//!      [`CAPABILITY_IS_DEFAULT`], printc.cc:109-114).
//!   4. **The PrintC options** ([`PrintCOptions`]) — the option fields, the
//!      `resetDefaultsPrintC` defaults (printc.cc:1649-1664, including the kuna
//!      DIV-2 default-on `option_arraynotation`), and the `set*` toggles
//!      (printc.hh:242-255).  These are the PrintC side of the options.cc
//!      `// SEAM(W8)` markers (`setNULLPrinting`/`setInplaceOps`/… on the
//!      print object).
//!   5. **The self-contained constant/type formatting** — the byte-for-byte
//!      token-string builders that the M2/M3 datatests match:
//!      [`print_char_hex_escape`] (printc.cc:1580-1591), [`print_unicode`]
//!      (printc.cc:1494-1538), [`format_integer_token`] (the
//!      `push_integer` string body, printc.cc:1407-1434), [`format_float_token`]
//!      (the `push_float` string body, printc.cc:1449-1492, the cfmt `%g`
//!      path), [`generic_type_name`] (printc.cc:3532-3558), and
//!      [`generic_function_name`] (printc.cc:3516-3526).
//!   6. **The opcode→token dispatch** ([`op_emit_kind`]) — the data half of the
//!      inline `op*` overrides (printc.hh:289-351): which [`OpToken`] each
//!      arithmetic/comparison op maps to and through which RPN form
//!      (`opBinary`/`opUnary`/`opFunc`/`opTypeCast`).  The *emission* is the
//!      seam; the *mapping* is faithful data.
//!
//! ## Compareform / arraynotation kuna hooks
//!
//! The kuna `compareform` rendering hook and the `arraynotation` `&base[index]`
//! mode are both controlled from here: `arraynotation` is the
//! [`PrintCOptions::array_notation`] toggle (default on, DIV-2), consulted by
//! the seam `opPtradd` body; `compareform` is a stage-model assertion that
//! flips which comparison `OpToken` `op_emit_kind` selects (present vs.
//! canonical).  The toggle state lives here; the emission that reads it is the
//! W9 seam.

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4, uintb};

use crate::dtype::type_metatype;
use crate::options::{BraceStyle, NamespaceStrategy};
use crate::prettyprint::{
    BraceStyle as EmitBraceStyle, Emit, EmitNoMarkup, MarkupRef, SyntaxHighlight,
};
use crate::printlanguage::{
    format_binary, modifiers, most_natural_base, parentheses, unicode_needs_escape, Atom, OpToken,
    PrintContext, ReversePolish, TagType, TokenType,
};

// ===========================================================================
// PrintCCapability — the c-language back-end factory metadata
// (printc.cc:109-114)
// ===========================================================================

/// The name registered by `PrintCCapability` (C++ `name = "c-language"`,
/// printc.cc:112).
pub const CAPABILITY_NAME: &str = "c-language";

/// Whether `PrintCCapability` registers as the default language (C++
/// `isdefault = true`, printc.cc:113).
pub const CAPABILITY_IS_DEFAULT: bool = true;

// ===========================================================================
// Operator token table (printc.cc:24-78)
// ===========================================================================

/// Construct a `static` [`OpToken`] in field order matching the C++ aggregate
/// initializer `{ print1, print2, stage, precedence, associative, type,
/// spacing, bump, negate }`.  `negate` is always `None` here; the six negate
/// cross-links (printc.cc:130-135) are resolved by [`token_negate`] to avoid a
/// self-referential static.
// The eight parameters are the eight C++ `OpToken` aggregate-initializer fields
// in source order; keeping them positional makes the `tokens::*` table a
// line-for-line transcription of the printc.cc table.
#[allow(clippy::too_many_arguments)]
const fn op_token(
    print1: &'static str,
    print2: &'static str,
    stage: int4,
    precedence: int4,
    associative: bool,
    token_type: TokenType,
    spacing: int4,
    bump: int4,
) -> OpToken {
    OpToken {
        print1,
        print2,
        stage,
        precedence,
        associative,
        token_type,
        spacing,
        bump,
        negate: None,
    }
}

/// The `PrintC` operator-token singletons (printc.cc:24-78).
///
/// These are `static` so [`crate::printlanguage::parentheses`]'s `std::ptr::eq`
/// identity check (the C++ `topToken == op2`) is meaningful.  The numbers are
/// the precedence/associativity/spacing/bump that define C operator
/// parenthesization; transcribed value-for-value from the C++ table.
pub mod tokens {
    use super::{op_token, OpToken, TokenType};

    /// Hidden functional (that may force parentheses) (printc.cc:24).
    pub static HIDDEN: OpToken = op_token("", "", 1, 70, false, TokenType::HiddenFunction, 0, 0);
    /// The sub-scope/namespace operator `::` (printc.cc:25).
    pub static SCOPE: OpToken = op_token("::", "", 2, 70, true, TokenType::Binary, 0, 0);
    /// The member operator `.` (printc.cc:26).
    pub static OBJECT_MEMBER: OpToken = op_token(".", "", 2, 66, true, TokenType::Binary, 0, 0);
    /// The points-to-member operator `->` (printc.cc:27).
    pub static POINTER_MEMBER: OpToken = op_token("->", "", 2, 66, true, TokenType::Binary, 0, 0);
    /// The array subscript operator `[ ]` (printc.cc:28).
    pub static SUBSCRIPT: OpToken = op_token("[", "]", 2, 66, false, TokenType::Postsurround, 0, 0);
    /// The function-call operator `( )` (printc.cc:29).
    pub static FUNCTION_CALL: OpToken =
        op_token("(", ")", 2, 66, false, TokenType::Postsurround, 0, 10);
    /// The bitwise-negate operator `~` (printc.cc:30).
    pub static BITWISE_NOT: OpToken = op_token("~", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The boolean-not operator `!` (printc.cc:31).
    pub static BOOLEAN_NOT: OpToken = op_token("!", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The unary-minus operator `-` (printc.cc:32).
    pub static UNARY_MINUS: OpToken = op_token("-", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The unary-plus operator `+` (printc.cc:33).
    pub static UNARY_PLUS: OpToken = op_token("+", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The address-of operator `&` (printc.cc:34).
    pub static ADDRESSOF: OpToken = op_token("&", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The pointer-dereference operator `*` (printc.cc:35).
    pub static DEREFERENCE: OpToken = op_token("*", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// The type-cast operator `( )` (printc.cc:36).
    pub static TYPECAST: OpToken = op_token("(", ")", 2, 62, false, TokenType::Presurround, 0, 0);
    /// The multiplication operator `*` (printc.cc:37).
    pub static MULTIPLY: OpToken = op_token("*", "", 2, 54, true, TokenType::Binary, 1, 0);
    /// The division operator `/` (printc.cc:38).
    pub static DIVIDE: OpToken = op_token("/", "", 2, 54, false, TokenType::Binary, 1, 0);
    /// The modulo operator `%` (printc.cc:39).
    pub static MODULO: OpToken = op_token("%", "", 2, 54, false, TokenType::Binary, 1, 0);
    /// The binary-addition operator `+` (printc.cc:40).
    pub static BINARY_PLUS: OpToken = op_token("+", "", 2, 50, true, TokenType::Binary, 1, 0);
    /// The binary-subtraction operator `-` (printc.cc:41).
    pub static BINARY_MINUS: OpToken = op_token("-", "", 2, 50, false, TokenType::Binary, 1, 0);
    /// The left-shift operator `<<` (printc.cc:42).
    pub static SHIFT_LEFT: OpToken = op_token("<<", "", 2, 46, false, TokenType::Binary, 1, 0);
    /// The right-shift operator `>>` (printc.cc:43).
    pub static SHIFT_RIGHT: OpToken = op_token(">>", "", 2, 46, false, TokenType::Binary, 1, 0);
    /// The signed right-shift operator `>>` (printc.cc:44).
    pub static SHIFT_SRIGHT: OpToken = op_token(">>", "", 2, 46, false, TokenType::Binary, 1, 0);
    /// The less-than operator `<` (printc.cc:45).
    pub static LESS_THAN: OpToken = op_token("<", "", 2, 42, false, TokenType::Binary, 1, 0);
    /// The less-than-or-equal operator `<=` (printc.cc:46).
    pub static LESS_EQUAL: OpToken = op_token("<=", "", 2, 42, false, TokenType::Binary, 1, 0);
    /// The greater-than operator `>` (printc.cc:47).
    pub static GREATER_THAN: OpToken = op_token(">", "", 2, 42, false, TokenType::Binary, 1, 0);
    /// The greater-than-or-equal operator `>=` (printc.cc:48).
    pub static GREATER_EQUAL: OpToken = op_token(">=", "", 2, 42, false, TokenType::Binary, 1, 0);
    /// The equal operator `==` (printc.cc:49).
    pub static EQUAL: OpToken = op_token("==", "", 2, 38, false, TokenType::Binary, 1, 0);
    /// The not-equal operator `!=` (printc.cc:50).
    pub static NOT_EQUAL: OpToken = op_token("!=", "", 2, 38, false, TokenType::Binary, 1, 0);
    /// The logical-and operator `&` (printc.cc:51).
    pub static BITWISE_AND: OpToken = op_token("&", "", 2, 34, true, TokenType::Binary, 1, 0);
    /// The logical-xor operator `^` (printc.cc:52).
    pub static BITWISE_XOR: OpToken = op_token("^", "", 2, 30, true, TokenType::Binary, 1, 0);
    /// The logical-or operator `|` (printc.cc:53).
    pub static BITWISE_OR: OpToken = op_token("|", "", 2, 26, true, TokenType::Binary, 1, 0);
    /// The boolean-and operator `&&` (printc.cc:54).
    pub static BOOLEAN_AND: OpToken = op_token("&&", "", 2, 22, false, TokenType::Binary, 1, 0);
    /// The boolean-xor operator `^^` (printc.cc:55).
    pub static BOOLEAN_XOR: OpToken = op_token("^^", "", 2, 20, false, TokenType::Binary, 1, 0);
    /// The boolean-or operator `||` (printc.cc:56).
    pub static BOOLEAN_OR: OpToken = op_token("||", "", 2, 18, false, TokenType::Binary, 1, 0);
    /// The assignment operator `=` (printc.cc:57).
    pub static ASSIGNMENT: OpToken = op_token("=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The comma operator `,` for parameter lists (printc.cc:58).
    pub static COMMA: OpToken = op_token(",", "", 2, 2, true, TokenType::Binary, 0, 0);
    /// The `new` operator (printc.cc:59).
    pub static NEW_OP: OpToken = op_token("", "", 2, 62, false, TokenType::Space, 1, 0);

    // In-place assignment operators (printc.cc:62-71)
    /// The in-place multiplication operator `*=` (printc.cc:62).
    pub static MULTEQUAL: OpToken = op_token("*=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place division operator `/=` (printc.cc:63).
    pub static DIVEQUAL: OpToken = op_token("/=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place modulo operator `%=` (printc.cc:64).
    pub static REMEQUAL: OpToken = op_token("%=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place addition operator `+=` (printc.cc:65).
    pub static PLUSEQUAL: OpToken = op_token("+=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place subtraction operator `-=` (printc.cc:66).
    pub static MINUSEQUAL: OpToken = op_token("-=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place left-shift operator `<<=` (printc.cc:67).
    pub static LEFTEQUAL: OpToken = op_token("<<=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place right-shift operator `>>=` (printc.cc:68).
    pub static RIGHTEQUAL: OpToken = op_token(">>=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place logical-and operator `&=` (printc.cc:69).
    pub static ANDEQUAL: OpToken = op_token("&=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place logical-or operator `|=` (printc.cc:70).
    pub static OREQUAL: OpToken = op_token("|=", "", 2, 14, false, TokenType::Binary, 1, 5);
    /// The in-place logical-xor operator `^=` (printc.cc:71).
    pub static XOREQUAL: OpToken = op_token("^=", "", 2, 14, false, TokenType::Binary, 1, 5);

    // Operator tokens for type expressions (printc.cc:74-78)
    /// Type declaration involving a space (printc.cc:74).
    pub static TYPE_EXPR_SPACE: OpToken = op_token("", "", 2, 10, false, TokenType::Space, 1, 0);
    /// Type declaration with no space (printc.cc:75).
    pub static TYPE_EXPR_NOSPACE: OpToken = op_token("", "", 2, 10, false, TokenType::Space, 0, 0);
    /// Pointer adornment for a type declaration `*` (printc.cc:76).
    pub static PTR_EXPR: OpToken = op_token("*", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);
    /// Array adornment for a type declaration `[ ]` (printc.cc:77).
    pub static ARRAY_EXPR: OpToken = op_token("[", "]", 2, 66, false, TokenType::Postsurround, 1, 0);
    /// The concatenation operator `|` for enumerated values (printc.cc:78).
    pub static ENUM_CAT: OpToken = op_token("|", "", 2, 26, true, TokenType::Binary, 0, 0);
}

/// The complementary (negated) token for the six comparison operators
/// (C++ `PrintC::PrintC` flip-token assignments, printc.cc:130-135).
///
/// In C++ these are stored in each `OpToken::negate` field, set in the
/// constructor.  Because a `static OpToken` cannot hold a `&'static` reference
/// to another `static` defined later (no self-referential statics), the link is
/// realized here as a pointer-identity lookup — the only consumer is the (seam)
/// `op_binary` reading the complement under the `negatetoken` modifier.
/// Returns `None` for any token without a complement (every C++ token whose
/// `negate` stays null).
pub fn token_negate(tok: &'static OpToken) -> Option<&'static OpToken> {
    use tokens::*;
    if std::ptr::eq(tok, &LESS_THAN) {
        Some(&GREATER_EQUAL)
    } else if std::ptr::eq(tok, &LESS_EQUAL) {
        Some(&GREATER_THAN)
    } else if std::ptr::eq(tok, &GREATER_THAN) {
        Some(&LESS_EQUAL)
    } else if std::ptr::eq(tok, &GREATER_EQUAL) {
        Some(&LESS_THAN)
    } else if std::ptr::eq(tok, &EQUAL) {
        Some(&NOT_EQUAL)
    } else if std::ptr::eq(tok, &NOT_EQUAL) {
        Some(&EQUAL)
    } else {
        None
    }
}

// ===========================================================================
// Keyword / punctuation constants (printc.cc:80-104)
// ===========================================================================

/// The c-language keyword and punctuation tokens (C++ `PrintC::EMPTY_STRING`
/// .. `PrintC::typePointerRelToken`, printc.cc:80-104).
pub mod keywords {
    /// An empty token (printc.cc:80).
    pub const EMPTY_STRING: &str = "";
    /// `"{"` token (printc.cc:81).
    pub const OPEN_CURLY: &str = "{";
    /// `"}"` token (printc.cc:82).
    pub const CLOSE_CURLY: &str = "}";
    /// `";"` token (printc.cc:83).
    pub const SEMICOLON: &str = ";";
    /// `":"` token (printc.cc:84).
    pub const COLON: &str = ":";
    /// `"="` token (printc.cc:85).
    pub const EQUALSIGN: &str = "=";
    /// `","` token (printc.cc:86).
    pub const COMMA: &str = ",";
    /// `"..."` token (printc.cc:87).
    pub const DOTDOTDOT: &str = "...";
    /// `"void"` keyword (printc.cc:88).
    pub const KEYWORD_VOID: &str = "void";
    /// `"true"` keyword (printc.cc:89).
    pub const KEYWORD_TRUE: &str = "true";
    /// `"false"` keyword (printc.cc:90).
    pub const KEYWORD_FALSE: &str = "false";
    /// `"if"` keyword (printc.cc:91).
    pub const KEYWORD_IF: &str = "if";
    /// `"else"` keyword (printc.cc:92).
    pub const KEYWORD_ELSE: &str = "else";
    /// `"do"` keyword (printc.cc:93).
    pub const KEYWORD_DO: &str = "do";
    /// `"while"` keyword (printc.cc:94).
    pub const KEYWORD_WHILE: &str = "while";
    /// `"for"` keyword (printc.cc:95).
    pub const KEYWORD_FOR: &str = "for";
    /// `"goto"` keyword (printc.cc:96).
    pub const KEYWORD_GOTO: &str = "goto";
    /// `"break"` keyword (printc.cc:97).
    pub const KEYWORD_BREAK: &str = "break";
    /// `"continue"` keyword (printc.cc:98).
    pub const KEYWORD_CONTINUE: &str = "continue";
    /// `"case"` keyword (printc.cc:99).
    pub const KEYWORD_CASE: &str = "case";
    /// `"switch"` keyword (printc.cc:100).
    pub const KEYWORD_SWITCH: &str = "switch";
    /// `"default"` keyword (printc.cc:101).
    pub const KEYWORD_DEFAULT: &str = "default";
    /// `"return"` keyword (printc.cc:102).
    pub const KEYWORD_RETURN: &str = "return";
    /// `"new"` keyword (printc.cc:103).
    pub const KEYWORD_NEW: &str = "new";
    /// The token printed for a PTRSUB relative to a `TypePointerRel`
    /// (C++ `typePointerRelToken = "ADJ"`, printc.cc:104).
    pub const TYPE_POINTER_REL_TOKEN: &str = "ADJ";
}

// ===========================================================================
// Symbol display-format constants (Symbol::force_*, used by push_integer)
// ===========================================================================

/// The `Symbol::force_*` display-format selectors used by [`format_integer_token`]
/// (C++ `Symbol` anon enum; identical to [`crate::database::symbol_dispflags`]).
/// Re-stated here so the formatter is self-describing and matches the C++
/// `displayFormat` switch (printc.cc:1410-1429) value-for-value.
pub mod display_format {
    /// No format forced (C++ `0`).
    pub const NONE: u32 = 0;
    /// Force hexadecimal (`Symbol::force_hex`).
    pub const FORCE_HEX: u32 = 1;
    /// Force decimal (`Symbol::force_dec`).
    pub const FORCE_DEC: u32 = 2;
    /// Force octal (`Symbol::force_oct`).
    pub const FORCE_OCT: u32 = 3;
    /// Force binary (`Symbol::force_bin`).
    pub const FORCE_BIN: u32 = 4;
    /// Force character (`Symbol::force_char`).
    pub const FORCE_CHAR: u32 = 5;
}

// ===========================================================================
// PrintC options (printc.hh:146-156, 242-255; printc.cc:1649-1664)
// ===========================================================================

/// The PrintC-specific options block (C++ `PrintC` `option_*` members,
/// printc.hh:146-156).
///
/// `resetDefaultsPrintC` (printc.cc:1649-1664) establishes the defaults; the
/// `set*` methods (printc.hh:242-255) are the toggles wired from the options.cc
/// `// SEAM(W8)` markers (`PrintC::setNULLPrinting(val)` etc.).  The kuna
/// `arraynotation` toggle (printc.hh:250-251) and its DIV-2 default-on
/// (printc.cc:1658) are carried here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintCOptions {
    /// Emit a `NULL` token for a null pointer (C++ `option_NULL`).
    pub null: bool,
    /// Use `+=` / `&=` etc. in-place operators (C++ `option_inplace_ops`).
    pub inplace_ops: bool,
    /// Print the calling convention (C++ `option_convention`).
    pub convention: bool,
    /// Do not print casts (C++ `option_nocasts`).
    pub nocasts: bool,
    /// Display unplaced comments (C++ `option_unplaced`).
    pub unplaced: bool,
    /// Hide implied extension operations (C++ `option_hide_exts`).
    pub hide_exts: bool,
    /// (kuna) Render standalone PTRADD as `&base[index]` rather than
    /// `base + index` (C++ `option_arraynotation`, printc.hh:152).
    pub array_notation: bool,
    /// How function-declaration braces are formatted (C++ `option_brace_func`).
    pub brace_func: BraceStyle,
    /// How if/else-block braces are formatted (C++ `option_brace_ifelse`).
    pub brace_ifelse: BraceStyle,
    /// How loop-block braces are formatted (C++ `option_brace_loop`).
    pub brace_loop: BraceStyle,
    /// How switch-block braces are formatted (C++ `option_brace_switch`).
    pub brace_switch: BraceStyle,
}

impl Default for PrintCOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl PrintCOptions {
    /// Construct with the `resetDefaultsPrintC` defaults (printc.cc:1649-1664).
    ///
    /// Note the kuna DIV-2 default-on `array_notation = true` (printc.cc:1658),
    /// the `&base[index]` form for a standalone PTRADD (GH-558).
    pub fn new() -> PrintCOptions {
        PrintCOptions {
            convention: true,
            hide_exts: true,
            inplace_ops: false,
            nocasts: false,
            null: false,
            unplaced: false,
            array_notation: true, // (kuna) DIV-2 default-on (GH-558)
            brace_func: BraceStyle::SkipLine,   // Emit::skip_line
            brace_ifelse: BraceStyle::SameLine, // Emit::same_line
            brace_loop: BraceStyle::SameLine,   // Emit::same_line
            brace_switch: BraceStyle::SameLine, // Emit::same_line
        }
    }

    /// C++ `setNULLPrinting(val)` (printc.hh:242).
    pub fn set_null_printing(&mut self, val: bool) {
        self.null = val;
    }
    /// C++ `setInplaceOps(val)` (printc.hh:243).
    pub fn set_inplace_ops(&mut self, val: bool) {
        self.inplace_ops = val;
    }
    /// C++ `setConvention(val)` (printc.hh:244).
    pub fn set_convention(&mut self, val: bool) {
        self.convention = val;
    }
    /// C++ `setNoCastPrinting(val)` (printc.hh:245).
    pub fn set_no_cast_printing(&mut self, val: bool) {
        self.nocasts = val;
    }
    /// C++ `setDisplayUnplaced(val)` (printc.hh:248).
    pub fn set_display_unplaced(&mut self, val: bool) {
        self.unplaced = val;
    }
    /// C++ `setHideImpliedExts(val)` (printc.hh:249).
    pub fn set_hide_implied_exts(&mut self, val: bool) {
        self.hide_exts = val;
    }
    /// (kuna) C++ `setArrayNotation(val)` (printc.hh:250).
    pub fn set_array_notation(&mut self, val: bool) {
        self.array_notation = val;
    }
    /// (kuna) C++ `getArrayNotation()` (printc.hh:251).
    pub fn array_notation(&self) -> bool {
        self.array_notation
    }
    /// C++ `setBraceFormatFunction(style)` (printc.hh:252).
    pub fn set_brace_format_function(&mut self, style: BraceStyle) {
        self.brace_func = style;
    }
    /// C++ `setBraceFormatIfElse(style)` (printc.hh:253).
    pub fn set_brace_format_ifelse(&mut self, style: BraceStyle) {
        self.brace_ifelse = style;
    }
    /// C++ `setBraceFormatLoop(style)` (printc.hh:254).
    pub fn set_brace_format_loop(&mut self, style: BraceStyle) {
        self.brace_loop = style;
    }
    /// C++ `setBraceFormatSwitch(style)` (printc.hh:255).
    pub fn set_brace_format_switch(&mut self, style: BraceStyle) {
        self.brace_switch = style;
    }
}

// ===========================================================================
// Self-contained constant / type formatting
// ===========================================================================

/// C++ `PrintC::printCharHexEscape` (printc.cc:1580-1591).
///
/// Append `\x` followed by `val` in lowercase hex, zero-padded to 2/4/8 digits
/// by magnitude.  Transcribed including the `setfill('0')`/`setw` widths.
pub fn print_char_hex_escape(s: &mut String, val: int4) {
    use std::fmt::Write;
    if val < 256 {
        let _ = write!(s, "\\x{val:02x}");
    } else if val < 65536 {
        let _ = write!(s, "\\x{val:04x}");
    } else {
        let _ = write!(s, "\\x{val:08x}");
    }
}

/// C++ `PrintC::printUnicode` (printc.cc:1494-1538).
///
/// Emit a single (unicode) codepoint into a quoted-string/char context: special
/// C escapes for the small control characters, a generic `\x` escape for other
/// escape-needing codepoints, otherwise the raw UTF-8 bytes.  Transcribed
/// case-for-case from the C++ switch; the final non-escape branch is the C++
/// `StringManager::writeUtf8` (encoded here as a `char` push when the codepoint
/// is a valid scalar value, matching the UTF-8 byte emission).
pub fn print_unicode(s: &mut String, onechar: int4) {
    if unicode_needs_escape(onechar) {
        match onechar {
            0 => {
                s.push_str("\\0");
                return;
            }
            7 => {
                s.push_str("\\a");
                return;
            }
            8 => {
                s.push_str("\\b");
                return;
            }
            9 => {
                s.push_str("\\t");
                return;
            }
            10 => {
                s.push_str("\\n");
                return;
            }
            11 => {
                s.push_str("\\v");
                return;
            }
            12 => {
                s.push_str("\\f");
                return;
            }
            13 => {
                s.push_str("\\r");
                return;
            }
            92 => {
                s.push_str("\\\\");
                return;
            }
            0x22 => {
                // '"'
                s.push_str("\\\"");
                return;
            }
            0x27 => {
                // '\''
                s.push_str("\\\'");
                return;
            }
            _ => {}
        }
        // Generic escape code (C++ printCharHexEscape).
        print_char_hex_escape(s, onechar);
        return;
    }
    // C++ `StringManager::writeUtf8(s, onechar)` — emit the UTF-8 bytes of the
    // codepoint.  `char::from_u32` yields the same bytes Rust's UTF-8 encoder
    // and the C++ writer produce for any valid scalar value.
    if let Some(c) = char::from_u32(onechar as u32) {
        s.push(c);
    }
}

/// The string body of C++ `PrintC::push_integer` (printc.cc:1407-1434) — the
/// byte-for-byte token characters for an integer constant, given the
/// already-resolved sign decision and display format.
///
/// Mirrors the C++ `ostringstream t` construction exactly: optional leading
/// `-`, then the format-specific digits (`0x`+lower-hex / decimal / `0`+octal /
/// quoted char / `0b`+binary), then the optional `U` and size suffix.
///
/// `print_negsign`/`val`/`display_fmt`/`sz` are the values the C++ computes
/// before line 1407 (the sign-stripping at printc.cc:1381-1391 and the
/// hex/dec decision at printc.cc:1393-1405); [`resolve_integer_format`] computes
/// them so callers can reproduce the full path.  `force_unsigned`/`force_sized`
/// are the `vn->isUnsignedPrint()`/`isLongPrint()` flags (printc.cc:1378-1379);
/// `wide_char_prefix` is `doEmitWideCharPrefix()` (printc.cc:1417);
/// `size_suffix` is the `sizeSuffix` member (printc.cc:1433).
#[allow(clippy::too_many_arguments)]
pub fn format_integer_token(
    print_negsign: bool,
    val: uintb,
    display_fmt: u32,
    sz: int4,
    force_unsigned: bool,
    force_sized: bool,
    wide_char_prefix: bool,
    size_suffix: &str,
) -> String {
    use std::fmt::Write;
    let mut t = String::new();
    if print_negsign {
        t.push('-');
    }
    if display_fmt == display_format::FORCE_HEX {
        let _ = write!(t, "0x{val:x}");
    } else if display_fmt == display_format::FORCE_DEC {
        let _ = write!(t, "{val}");
    } else if display_fmt == display_format::FORCE_OCT {
        let _ = write!(t, "0{val:o}");
    } else if display_fmt == display_format::FORCE_CHAR {
        if wide_char_prefix && sz > 1 {
            t.push('L'); // wide character marker
        }
        t.push('\''); // char surrounded with single quotes
        if sz == 1 && val >= 0x80 {
            print_char_hex_escape(&mut t, val as int4);
        } else {
            print_unicode(&mut t, val as int4);
        }
        t.push('\'');
    } else {
        // Must be Symbol::force_bin
        t.push_str("0b");
        format_binary(&mut t, val);
    }
    if force_unsigned {
        t.push('U'); // force unsignedness explicitly
    }
    if force_sized {
        t.push_str(size_suffix);
    }
    t
}

/// The `print_negsign`/`val`/`display_fmt` resolution C++ `push_integer`
/// performs before formatting (printc.cc:1381-1405).
///
/// `sign` is the signedness request; `force_hex`/`force_dec` are the active
/// `mods` bits.  Returns `(print_negsign, val_to_print, display_fmt)`.  The
/// caller still owns the `vn`/`Symbol`-driven `displayFormat` override and the
/// equate short-circuit (printc.cc:1368-1380), which need the W7 Varnode/Symbol
/// graph; `display_fmt_in` is whatever override they resolved (`0` for none).
pub fn resolve_integer_format(
    mut val: uintb,
    sz: int4,
    sign: bool,
    display_fmt_in: u32,
    force_hex: bool,
    force_dec: bool,
) -> (bool, uintb, u32) {
    let print_negsign;
    // Sign handling (printc.cc:1381-1391).
    if sign && display_fmt_in != display_format::FORCE_CHAR {
        let mask = calc_mask(sz);
        let flip = val ^ mask;
        print_negsign = flip < val;
        if print_negsign {
            // C++ `val = flip+1;` — two's-complement magnitude.
            val = flip.wrapping_add(1);
        }
    } else {
        print_negsign = false;
    }

    // Hex/dec decision (printc.cc:1393-1405).
    let display_fmt = if display_fmt_in != display_format::NONE {
        display_fmt_in // forced by the Symbol or data-type
    } else if force_hex {
        display_format::FORCE_HEX
    } else if val <= 10 || force_dec {
        display_format::FORCE_DEC
    } else if most_natural_base(val) == 16 {
        display_format::FORCE_HEX
    } else {
        display_format::FORCE_DEC
    };
    (print_negsign, val, display_fmt)
}

/// The classification a host float resolves to (C++ `FloatFormat::floatclass`),
/// supplied by the caller of [`format_float_token`] from the
/// `glb->translate->getFloatFormat(sz)` decode plus
/// [`kuna_num::float::FloatFormat::get_host_float`] (W6, kuna-num).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatClass {
    /// A normal finite value (the `printDecimal` path).
    Normal,
    /// Positive or negative infinity.
    Infinity,
    /// Not-a-number.
    Nan,
    /// No `FloatFormat` for the size (`FLOAT_UNKNOWN`).
    Unknown,
}

/// The string body of C++ `PrintC::push_float` (printc.cc:1449-1492) — the
/// byte-for-byte token characters for a floating-point constant.
///
/// The `FloatFormat` decode (`getHostFloat`/`extractSign`/`printDecimal`) is a
/// W6 seam (`FloatFormat`, `float.cc`, a separate item).  In particular
/// `printDecimal` is the shortest-round-trip precision loop at float.cc:446-479
/// — **not** a fixed-precision `%g` — so it must come from `FloatFormat`, not be
/// reinvented here.  This function therefore takes the *already-decoded*
/// results: the [`FloatClass`], the sign, and (for [`FloatClass::Normal`]) the
/// [`kuna_num::float::FloatFormat::print_decimal`]`(floatval, force_scinote)`
/// string the caller obtained from `FloatFormat`.  It reproduces only the parts
/// that live in `push_float`
/// itself: the `INFINITY`/`NAN`/`FLOAT_UNKNOWN` names (printc.cc:1454-1469) and
/// the `.0` suffix forced onto a non-scientific decimal that doesn't already
/// look like a float (printc.cc:1477-1487).
///
/// `force_scinote` is the active `mods & force_scinote` bit (printc.cc:1472);
/// when set the C++ skips the `.0` fix-up (the scientific form always has an
/// `e`), so `printed_decimal` is returned verbatim.
pub fn format_float_token(
    class: FloatClass,
    sign: bool,
    printed_decimal: &str,
    force_scinote: bool,
) -> String {
    match class {
        FloatClass::Unknown => "FLOAT_UNKNOWN".to_string(),
        FloatClass::Infinity => {
            if sign {
                "-INFINITY".to_string()
            } else {
                "INFINITY".to_string()
            }
        }
        FloatClass::Nan => {
            if sign {
                "-NAN".to_string()
            } else {
                "NAN".to_string()
            }
        }
        FloatClass::Normal => {
            if force_scinote {
                // C++ `token = format->printDecimal(floatval, true)` — used as is.
                printed_decimal.to_string()
            } else {
                let mut token = printed_decimal.to_string();
                // printc.cc:1477-1487: force the token to look like a float.
                let looks_like_float = token.bytes().any(|c| c == b'.' || c == b'e');
                if !looks_like_float {
                    token.push_str(".0");
                }
                token
            }
        }
    }
}

/// C++ `PrintC::genericFunctionName` (printc.cc:3516-3526), the non-kuna
/// (`func_<addr>`) branch.
///
/// The kuna angr-style `sub_<addr>` branch (printc.cc:3519-3520) is gated on
/// `kunaAngrNaming(glb)`, an Architecture query the caller owns; pass the
/// already-decided flag.  When `angr_naming` is true the caller substitutes
/// `kunaFunctionName(addr)` (the `kuna_naming` module); this function produces
/// the plain `func_` + raw address form.
pub fn generic_function_name(addr: &Address) -> KunaResult<String> {
    let mut s = String::from("func_");
    addr.print_raw(&mut s)?;
    Ok(s)
}

/// C++ `PrintC::genericTypeName` (printc.cc:3532-3558).
///
/// A generic name for an unnamed data-type: an `unk*` prefix by metatype with
/// the size appended, or a `BADSPACEBASE`/`BADTYPE` sentinel.  Transcribed
/// case-for-case.
pub fn generic_type_name(metatype: type_metatype, size: int4) -> String {
    use std::fmt::Write;
    use type_metatype::*;
    let mut s = String::new();
    let prefix = match metatype {
        TYPE_INT => "unkint",
        TYPE_UINT => "unkuint",
        TYPE_UNKNOWN => "unkbyte",
        TYPE_SPACEBASE => return "BADSPACEBASE".to_string(),
        TYPE_FLOAT => "unkfloat",
        _ => return "BADTYPE".to_string(),
    };
    s.push_str(prefix);
    let _ = write!(s, "{size}");
    s
}

// ===========================================================================
// Opcode -> token dispatch (printc.hh:289-351)
// ===========================================================================

/// How a `PcodeOp` is pushed onto the RPN stack by the `PrintC` `op*` override
/// (the C++ inline bodies in printc.hh:289-351).
///
/// This is the *data* half of those overrides: which [`OpToken`] and which RPN
/// form.  The *emission* (the actual `opBinary`/`opUnary`/`opFunc`/`opTypeCast`
/// call that pushes onto the stack and drives `Emit`) is the W9 seam.
#[derive(Debug, Clone, Copy)]
pub enum OpEmitKind {
    /// `opBinary(&token, op)` — a binary operator (printc.hh `opBinary` form).
    Binary(&'static OpToken),
    /// `opUnary(&token, op)` — a unary-prefix operator (printc.hh `opUnary` form).
    Unary(&'static OpToken),
    /// `opFunc(op)` — a functional `name(args)` form (printc.cc:444).
    Func,
    /// `opTypeCast(op)` — a type-cast form (printc.cc:468).
    TypeCast,
    /// The op has a hand-written override (`opLoad`/`opStore`/`opCall`/… and the
    /// no-op `opMultiequal`/`opIndirect`); not a simple table entry.
    Custom,
}

/// The token/form each `PcodeOp` maps to in the C++ `PrintC` inline `op*`
/// overrides (printc.hh:289-351).
///
/// Returns the [`OpEmitKind`] for the opcodes whose override is a one-line
/// `opBinary`/`opUnary`/`opFunc`/`opTypeCast` delegation; [`OpEmitKind::Custom`]
/// for the opcodes with a hand-written body in printc.cc (those are seam-noted).
/// This is the faithful dispatch table; the emission it feeds is the W9 seam.
pub fn op_emit_kind(opcode: kuna_num::opcodes::OpCode) -> OpEmitKind {
    use kuna_num::opcodes::OpCode::*;
    use tokens::*;
    match opcode {
        // Comparisons (printc.hh:289-294, 319-322).
        CPUI_INT_EQUAL => OpEmitKind::Binary(&EQUAL),
        CPUI_INT_NOTEQUAL => OpEmitKind::Binary(&NOT_EQUAL),
        CPUI_INT_SLESS => OpEmitKind::Binary(&LESS_THAN),
        CPUI_INT_SLESSEQUAL => OpEmitKind::Binary(&LESS_EQUAL),
        CPUI_INT_LESS => OpEmitKind::Binary(&LESS_THAN),
        CPUI_INT_LESSEQUAL => OpEmitKind::Binary(&LESS_EQUAL),
        CPUI_FLOAT_EQUAL => OpEmitKind::Binary(&EQUAL),
        CPUI_FLOAT_NOTEQUAL => OpEmitKind::Binary(&NOT_EQUAL),
        CPUI_FLOAT_LESS => OpEmitKind::Binary(&LESS_THAN),
        CPUI_FLOAT_LESSEQUAL => OpEmitKind::Binary(&LESS_EQUAL),
        // Integer arithmetic (printc.hh:297-313).
        CPUI_INT_ADD => OpEmitKind::Binary(&BINARY_PLUS),
        CPUI_INT_SUB => OpEmitKind::Binary(&BINARY_MINUS),
        CPUI_INT_XOR => OpEmitKind::Binary(&BITWISE_XOR),
        CPUI_INT_AND => OpEmitKind::Binary(&BITWISE_AND),
        CPUI_INT_OR => OpEmitKind::Binary(&BITWISE_OR),
        CPUI_INT_LEFT => OpEmitKind::Binary(&SHIFT_LEFT),
        CPUI_INT_RIGHT => OpEmitKind::Binary(&SHIFT_RIGHT),
        CPUI_INT_SRIGHT => OpEmitKind::Binary(&SHIFT_SRIGHT),
        CPUI_INT_MULT => OpEmitKind::Binary(&MULTIPLY),
        CPUI_INT_DIV => OpEmitKind::Binary(&DIVIDE),
        CPUI_INT_SDIV => OpEmitKind::Binary(&DIVIDE),
        CPUI_INT_REM => OpEmitKind::Binary(&MODULO),
        CPUI_INT_SREM => OpEmitKind::Binary(&MODULO),
        // Integer unary (printc.hh:302-303).
        CPUI_INT_2COMP => OpEmitKind::Unary(&UNARY_MINUS),
        CPUI_INT_NEGATE => OpEmitKind::Unary(&BITWISE_NOT),
        // Integer functional (printc.hh:299-301).
        CPUI_INT_CARRY => OpEmitKind::Func,
        CPUI_INT_SCARRY => OpEmitKind::Func,
        CPUI_INT_SBORROW => OpEmitKind::Func,
        // Boolean (printc.hh:316-318).
        CPUI_BOOL_XOR => OpEmitKind::Binary(&BOOLEAN_XOR),
        CPUI_BOOL_AND => OpEmitKind::Binary(&BOOLEAN_AND),
        CPUI_BOOL_OR => OpEmitKind::Binary(&BOOLEAN_OR),
        // Float arithmetic (printc.hh:324-336).
        CPUI_FLOAT_ADD => OpEmitKind::Binary(&BINARY_PLUS),
        CPUI_FLOAT_DIV => OpEmitKind::Binary(&DIVIDE),
        CPUI_FLOAT_MULT => OpEmitKind::Binary(&MULTIPLY),
        CPUI_FLOAT_SUB => OpEmitKind::Binary(&BINARY_MINUS),
        CPUI_FLOAT_NEG => OpEmitKind::Unary(&UNARY_MINUS),
        CPUI_FLOAT_NAN => OpEmitKind::Func,
        CPUI_FLOAT_ABS => OpEmitKind::Func,
        CPUI_FLOAT_SQRT => OpEmitKind::Func,
        CPUI_FLOAT_CEIL => OpEmitKind::Func,
        CPUI_FLOAT_FLOOR => OpEmitKind::Func,
        CPUI_FLOAT_ROUND => OpEmitKind::Func,
        CPUI_FLOAT_FLOAT2FLOAT => OpEmitKind::TypeCast,
        CPUI_FLOAT_TRUNC => OpEmitKind::TypeCast,
        // Cast (printc.hh:341).
        CPUI_CAST => OpEmitKind::TypeCast,
        // Misc functional (printc.hh:339, 350-351).
        CPUI_PIECE => OpEmitKind::Func,
        CPUI_POPCOUNT => OpEmitKind::Func,
        CPUI_LZCOUNT => OpEmitKind::Func,
        // Everything else has a hand-written override (printc.cc) or is a no-op
        // (opMultiequal/opIndirect, printc.hh:337-338).
        _ => OpEmitKind::Custom,
    }
}

// ===========================================================================
// The RPN/Emit-driven body of PrintC (w10-printc-body)
// ===========================================================================
//
// The `PrintLanguage` RPN driver (`pushOp`/`pushAtom`/`pushVn`/`recurse`/
// `emitOp`/`emitAtom`/`opBinary`/`opUnary`/`parentheses`, printlanguage.cc:
// 129-580) is now **ported** as the `impl PrintC` block below, driving the real
// `Emit` back-end (`prettyprint.rs`'s `EmitNoMarkup`).  It is byte-faithfully
// unit-tested against the C++ `emitOp`/`emitAtom`/`parentheses` logic (see the
// `rpn_*` tests) — `a + b`, `x = a`, `-a`, `a * (b + c)`, `a + b * c`,
// associativity, the negate-token flip, all match the upstream emitter.
//
// What is NOT yet driven over a real function body is the per-op IR leaf
// expansion (`recurse`'s `defOp->getOpcode()->push(...)` implied-op dispatch
// and `pushVnExplicit`'s Symbol/HighVariable/Datatype/constant resolution) plus
// the structured-/flat-block walk (`emitBlock*`).  Those are blocked NOT in the
// printer but UPSTREAM: the merged tree's decompilation passes (heritage /
// simplification / merge / type + proto recovery / block structuring) are seam
// stubs, so the IR reaching the printer is raw lifted p-code with no
// HighVariables-with-symbols, no recovered types, and **empty `sblocks`**.
// Printing it would emit non-C garbage, not byte-parity.  Those edges are
// `// SEAM(decompile-passes)` (LOSS-130 / W10) and fall to the upstream pass
// items; the RPN engine they feed is in place here.
//
// The remaining data this module provides (the token table, negate links,
// keyword constants, options, the constant/float/char formatters, and the
// opcode dispatch) is exactly what those bodies consume:
//   - `parentheses` (printlanguage.rs) reads the `tokens::*` precedence data;
//   - `push_integer`/`push_float`/`pushCharConstant` reduce to
//     `format_integer_token`/`format_float_token`/`print_unicode`;
//   - the `op*` overrides reduce to `op_emit_kind` + `op_binary`/`op_unary`/…;
//   - the option toggles (`PrintCOptions`) gate the seam branches.

// ===========================================================================
// PrintC — the stateful c-language printer object (the `glb->print` the
// `Architecture` owns).  (w9x-arch-engine-glue)
// ===========================================================================

/// Convert an `options::BraceStyle` (the PrintC-option enum) to the
/// `prettyprint::BraceStyle` the [`Emit`] driver consumes.  Both are the same
/// 3-variant `same_line`/`next_line`/`skip_line` enum (printc.hh:252-255 vs
/// emit.hh); the conversion is the identity mapping.
fn to_emit_brace(style: BraceStyle) -> EmitBraceStyle {
    match style {
        BraceStyle::SameLine => EmitBraceStyle::SameLine,
        BraceStyle::NextLine => EmitBraceStyle::NextLine,
        BraceStyle::SkipLine => EmitBraceStyle::SkipLine,
    }
}

/// \brief The c-language print object (C++ `class PrintC : public
/// PrintLanguage`, printc.hh:138).
///
/// In C++ `PrintC` *is-a* `PrintLanguage`, owning the [`PrintContext`] member
/// state (mod/scope stacks, comment/namespace defaults) and an `Emit *` driver,
/// plus the c-language [`PrintCOptions`].  The [`Architecture`](crate::architecture::Architecture)
/// holds it as `glb->print`.  This port carries:
///
///   * the **[`PrintCOptions`]** (the option toggles the `option NAME VALUE`
///     command flips through `ArchOptionContext`),
///   * the **[`PrintContext`]** (the shared print-modification / comment /
///     namespace state),
///   * the **language name** (`"c-language"`, the `getName()` the options
///     `print_is_c_language` predicate reads),
///   * the **flat** flag (`print C flat`, C++ `flat` mod bit), and
///   * an owned **[`EmitNoMarkup`]** back-end (the plain-text `print C` sink).
///
/// ## What `doc_function` emits today
///
/// [`doc_function`](PrintC::doc_function) faithfully transcribes the **shell**
/// of C++ `PrintC::docFunction` / `emitFunctionDeclaration` (printc.cc:2726,
/// 2790) — `beginFunction` → header comment line → the prototype declaration
/// (return type, function name, parenthesized parameters) → `openBraceIndent`
/// → … → `closeBraceIndent` → `endFunction` → `flush` — driving the **real**
/// [`Emit`] primitives.  The function **body** (`emitLocalVarDecls` +
/// `emitBlockGraph`, the per-statement RPN expression emission) is the
/// `// SEAM(W9-emit)` RPN/`Emit` driver documented in this module's header
/// (`pushVn`/`recurse`/`emitOp` against the IR), absent from the merged tree;
/// the body slot emits a single seam-marker comment line so the C output is a
/// structurally-complete, compilable-looking function shell (a real signature +
/// matched braces), not full byte-parity C.  The W9 closure fills the body in.
pub struct PrintC {
    /// The c-language options (C++ the `option_*` members).
    pub options: PrintCOptions,
    /// The shared print context (mod/scope stacks, comment/namespace state).
    pub context: PrintContext,
    /// The language name (C++ `PrintLanguage::name`, `"c-language"`).
    name: String,
    /// Whether `print C flat` is active (C++ the `flat` mod bit).
    flat: bool,
    /// The plain-text emit back-end (C++ the bound `Emit *`, an `EmitNoMarkup`
    /// for the non-pretty `print C` path).
    emit: EmitNoMarkup,
    /// The reverse-polish-notation operator stack (C++ `PrintLanguage::revpol`).
    /// Owned here because `printlanguage.rs` deferred its RPN driver to this
    /// closure (the driver and the `PrintC` op-emitters are one unit).
    revpol: Vec<ReversePolish>,
    /// The pending data-flow node stack (C++ `PrintLanguage::nodepend`).
    nodepend: Vec<crate::printlanguage::NodePending>,
    /// How much of `nodepend` is claimed (C++ `PrintLanguage::pending`).
    pending: usize,
}

impl Default for PrintC {
    fn default() -> Self {
        PrintC::new()
    }
}

impl PrintC {
    /// Construct the c-language printer (C++ `PrintC::PrintC` +
    /// `resetDefaultsPrintC`, printc.cc:118 / 1649).
    pub fn new() -> PrintC {
        PrintC {
            options: PrintCOptions::new(),
            context: PrintContext::new(),
            name: CAPABILITY_NAME.to_string(),
            flat: false,
            emit: EmitNoMarkup::new(),
            revpol: Vec::new(),
            nodepend: Vec::new(),
            pending: 0,
        }
    }

    /// The printer name (C++ `PrintLanguage::getName`, `"c-language"`).
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set the active print language name (C++ `setPrintLanguage` swaps which
    /// `PrintLanguage` is current; here the single owned printer records the
    /// requested name so `print_is_c_language` reflects it).
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// `print C flat` toggle (C++ `PrintLanguage::setFlat`).
    pub fn set_flat(&mut self, val: bool) {
        self.flat = val;
    }

    /// Whether `print C flat` is active.
    pub fn is_flat(&self) -> bool {
        self.flat
    }

    /// Reset the emit buffer (C++ `setOutputStream`).
    pub fn set_output_stream(&mut self) {
        self.emit.set_output_stream();
    }

    /// Initialize from the architecture (C++ `PrintLanguage::initializeFromArchitecture`).
    /// The sizes/types coupling is the W6 type factory, already built by the
    /// architecture; the printer needs no per-arch state beyond its options here.
    pub fn initialize_from_architecture(&mut self) {}

    /// Faithful transcription of the **shell** of C++ `PrintC::docFunction`
    /// (printc.cc:2790) + `emitFunctionDeclaration` (printc.cc:2726), driving
    /// the real [`Emit`] primitives.  The body (`emitBlockGraph`) is the
    /// `// SEAM(W9-emit)` RPN driver; this emits a seam-marker line in its place.
    ///
    /// `display_name` is `fd->getDisplayName()`; `model_name` is the prototype
    /// model name when `printModelInDecl()` (None when the model is hidden);
    /// `ret_type` is the return-type token (`fd->getFuncProto()` output type
    /// name, defaulting to `"void"`); `params` are the input parameters'
    /// `(type, name)` tokens.  Returns the rendered C text.
    pub fn doc_function(
        &mut self,
        display_name: &str,
        model_name: Option<&str>,
        ret_type: &str,
        params: &[(String, String)],
    ) -> String {
        self.emit.set_output_stream();
        let markup = MarkupRef::none();

        // int4 id1 = emit->beginFunction(fd);
        let id1 = self.emit.begin_function();
        // emitCommentFuncHeader(fd) — the header comment line (seam: full
        // CommentSorter is the comment item; emit the seam-marker header).
        // emit->tagLine();
        self.emit.tag_line();

        // --- emitFunctionDeclaration -------------------------------------
        let idp = self.emit.begin_func_proto();
        // emitPrototypeOutput: the return type token.
        let idret = self.emit.begin_return_type(&markup);
        self.emit.tag_type(ret_type, SyntaxHighlight::TypeColor, &markup);
        self.emit.end_return_type(idret);
        self.emit.spaces(1, 0);
        // option_convention: print the model name when shown.
        if self.options.convention {
            if let Some(m) = model_name {
                self.emit.print(m, SyntaxHighlight::KeywordColor);
                self.emit.spaces(1, 0);
            }
        }
        let id1g = self.emit.open_group();
        // emit->tagFuncName(fd->getDisplayName(), funcname_color, fd, 0);
        self.emit.tag_func_name(display_name, SyntaxHighlight::FuncnameColor, &markup);
        // function_call spacing (C++ function_call.spacing==0,bump==0).
        let id2 = self.emit.open_paren("(", 0);
        // emitPrototypeInputs: void or the comma-separated (type name) list.
        if params.is_empty() {
            self.emit.tag_type("void", SyntaxHighlight::TypeColor, &markup);
        } else {
            for (i, (ty, nm)) in params.iter().enumerate() {
                if i != 0 {
                    self.emit.print(",", SyntaxHighlight::NoColor);
                    self.emit.spaces(1, 0);
                }
                self.emit.tag_type(ty, SyntaxHighlight::TypeColor, &markup);
                if !nm.is_empty() {
                    self.emit.spaces(1, 0);
                    self.emit.tag_variable(nm, SyntaxHighlight::ParamColor, &markup);
                }
            }
        }
        self.emit.close_paren(")", id2);
        self.emit.close_group(id1g);
        self.emit.end_func_proto(idp);

        // int4 id = emit->openBraceIndent(OPEN_CURLY, option_brace_func);
        let id = self.emit.open_brace_indent("{", to_emit_brace(self.options.brace_func));
        // emitLocalVarDecls(fd) + emitBlockGraph(...).  The RPN body *engine*
        // (push_op/push_atom/op_binary/op_unary/emit_op/emit_atom/parentheses)
        // is now ported and unit-tested in this module (byte-faithful to the
        // C++ emitOp/emitAtom/parentheses).  Driving it over a real function
        // body is blocked NOT in the printer but UPSTREAM: the merged tree's
        // decompilation passes (heritage / simplification / merge / type +
        // proto recovery / block structuring) are seam stubs, so the IR
        // reaching the printer is raw lifted p-code (no HighVariables with
        // symbols, no recovered types, no structured blocks) — printing it
        // would emit non-C garbage, not byte-parity (see LOSS-130 / W10).
        // Until those passes land, the body slot is a single marker line so the
        // shell is a complete, brace-matched function.
        self.emit.tag_line();
        self.emit.print(
            "/* WARNING: body emission blocked on upstream decompilation passes (raw p-code IR) */",
            SyntaxHighlight::CommentColor,
        );
        // emit->closeBraceIndent(CLOSE_CURLY, id);
        self.emit.close_brace_indent("}", id);
        self.emit.tag_line();
        self.emit.end_function(id1);

        // C++ emit->flush() then the bound ostream holds the text.
        self.emit.output().to_string()
    }

    // --- the options.cc `// SEAM(W8)` print setters (now wired) -----------

    /// C++ `PrintC::setNULLPrinting` (options.cc:444).
    pub fn set_null_printing(&mut self, val: bool) {
        self.options.set_null_printing(val);
    }
    /// C++ `PrintC::setInplaceOps` (options.cc:459).
    pub fn set_inplace_ops(&mut self, val: bool) {
        self.options.set_inplace_ops(val);
    }
    /// C++ `PrintC::setConvention` (options.cc:474).
    pub fn set_convention_printing(&mut self, val: bool) {
        self.options.set_convention(val);
    }
    /// C++ `PrintC::setNoCastPrinting` (options.cc:489).
    pub fn set_no_cast_printing(&mut self, val: bool) {
        self.options.set_no_cast_printing(val);
    }
    /// C++ `PrintC::setHideImpliedExts` (options.cc:504).
    pub fn set_hide_implied_exts(&mut self, val: bool) {
        self.options.set_hide_implied_exts(val);
    }
    /// C++ `glb->print->setMaxLineSize(val)` (options.cc:524).
    pub fn set_max_line_size(&mut self, _val: int4) -> KunaResult<()> {
        // SEAM(W8 prettyprint): EmitNoMarkup ignores line size; EmitPrettyPrint
        // honours it.  Recorded so the option succeeds (the C++ validates the
        // range inside Emit::setMaxLineSize; the no-markup path is unbounded).
        Ok(())
    }
    /// C++ `glb->print->setIndentIncrement(val)` (options.cc:541).
    pub fn set_indent_increment(&mut self, val: int4) {
        self.emit.set_indent_increment(val);
    }
    /// C++ `glb->print->setLineCommentIndent(val)` (options.cc:559).
    pub fn set_line_comment_indent(&mut self, val: int4) -> KunaResult<()> {
        // C++ PrintLanguage::setLineCommentIndent validates against maxlinesize;
        // the EmitNoMarkup max is unbounded, so any non-negative value is valid.
        self.context.set_line_comment_indent(val, int4::MAX)
    }
    /// C++ `glb->print->getHeaderComment()` (options.cc:583).
    pub fn header_comment_flags(&self) -> uint4 {
        self.context.header_comment()
    }
    /// C++ `glb->print->setHeaderComment(flags)` (options.cc:589).
    pub fn set_header_comment_flags(&mut self, flags: uint4) {
        self.context.set_header_comment(flags);
    }
    /// C++ `glb->print->getInstructionComment()` (options.cc:604).
    pub fn instruction_comment_flags(&self) -> uint4 {
        self.context.instruction_comment()
    }
    /// C++ `glb->print->setInstructionComment(flags)` (options.cc:610).
    pub fn set_instruction_comment_flags(&mut self, flags: uint4) {
        self.context.set_instruction_comment(flags);
    }
    /// C++ `glb->print->setIntegerFormat(p1)` (options.cc:623).
    pub fn set_integer_format(&mut self, fmt: &str) -> KunaResult<()> {
        self.context.set_integer_format(fmt)
    }
    /// C++ `glb->print->setNamespaceStrategy(strategy)` (options.cc:1014).
    ///
    /// The option surface (`options::NamespaceStrategy`) and the print-context
    /// surface (`printlanguage::NamespaceStrategy`) are the same 3-variant
    /// `minimal`/`none`/`all` enum (printlanguage.hh); convert across the seam.
    pub fn set_namespace_strategy(&mut self, strategy: NamespaceStrategy) {
        use crate::printlanguage::NamespaceStrategy as PlStrat;
        let pl = match strategy {
            NamespaceStrategy::Minimal => PlStrat::MinimalNamespaces,
            NamespaceStrategy::None => PlStrat::NoNamespaces,
            NamespaceStrategy::All => PlStrat::AllNamespaces,
        };
        self.context.set_namespace_strategy(pl);
    }
    /// C++ `PrintC::setBraceFormat*` (options.cc:655-664).
    pub fn set_brace_format(&mut self, category: crate::options::BraceCategory, style: BraceStyle) {
        use crate::options::BraceCategory;
        match category {
            BraceCategory::Function => self.options.set_brace_format_function(style),
            BraceCategory::IfElse => self.options.set_brace_format_ifelse(style),
            BraceCategory::Loop => self.options.set_brace_format_loop(style),
            BraceCategory::Switch => self.options.set_brace_format_switch(style),
        }
    }
    /// C++ `PrintC::setCommentStyle` (options.cc:570).
    pub fn set_comment_style(&mut self, _style: &str) {
        // SEAM(comment): the slash-star vs slash-slash comment delimiters live
        // with the comment item; recorded as a no-op so the option succeeds.
    }

    // =====================================================================
    // The PrintLanguage RPN driver (printlanguage.cc:129-580), realized here
    // because printlanguage.rs deferred its token-emitting driver to this
    // closure (the driver + the PrintC op-emitters are one unit; see the
    // module header).  These methods drive the real [`Emit`] back-end.
    //
    // The IR-coupled leaves of the driver (the implied-varnode `recurse` step
    // `defOp->getOpcode()->push(...)`, and `pushVnExplicit`'s symbol/constant
    // resolution) need the seamed Symbol/HighVariable/Datatype/TypeOp
    // subsystems and the proto-/type-/heritage-recovery passes, which the
    // merged tree leaves unported (LOSS-130: the decompilation passes are
    // seam stubs, so the IR reaching the printer is raw lifted p-code).  The
    // RPN *engine* below is therefore transcribed and unit-tested against
    // synthetic atoms/tokens (byte-faithful to `emitOp`/`emitAtom`/
    // `parentheses`); the IR-leaf push is the `// SEAM(decompile-passes)`
    // edge handed to the caller via [`push_atom`].
    // =====================================================================

    /// Borrow the emit back-end (so a body driver can interleave `tag_line`
    /// etc. between RPN expressions).
    pub fn emit_mut(&mut self) -> &mut EmitNoMarkup {
        &mut self.emit
    }

    /// Whether the RPN stack is fully drained (C++ `isStackEmpty`).
    pub fn is_stack_empty(&self) -> bool {
        self.revpol.is_empty() && self.nodepend.is_empty()
    }

    /// C++ `PrintLanguage::clear` (printlanguage.cc:685) — drop any partial RPN
    /// state, leaving the modstack/scope to the [`PrintContext`].
    pub fn clear_rpn(&mut self) {
        self.revpol.clear();
        self.nodepend.clear();
        self.pending = 0;
    }

    /// C++ `PrintLanguage::pushOp` (printlanguage.cc:129).  Push an operator
    /// token onto the RPN stack, emitting any front part of the enclosing
    /// operator and opening the right group/paren.
    pub fn push_op(&mut self, tok: &'static OpToken, op: Option<usize>) {
        if self.pending < self.nodepend.len() {
            self.recurse(); // Pending varnode pushes before op
        }
        let paren;
        let id;
        if self.revpol.is_empty() {
            paren = false;
            id = self.emit.open_group();
        } else {
            let back = self.revpol.last().unwrap().clone();
            self.emit_op(&back);
            // Reflect any id2 mutation emit_op performed back onto the stack.
            *self.revpol.last_mut().unwrap() = back;
            paren = self.parentheses_top(tok);
            if paren {
                id = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
            } else {
                id = self.emit.open_group();
            }
        }
        self.revpol.push(ReversePolish { tok, visited: 0, paren, op, id, id2: 0 });
    }

    /// C++ `PrintLanguage::pushAtom` (printlanguage.cc:162).  Push a leaf token,
    /// draining as much of the RPN stack as is now complete.
    pub fn push_atom(&mut self, atom: &Atom) {
        if self.pending < self.nodepend.len() {
            self.recurse();
        }
        if self.revpol.is_empty() {
            self.emit_atom(atom);
        } else {
            let back = self.revpol.last().unwrap().clone();
            self.emit_op(&back);
            *self.revpol.last_mut().unwrap() = back;
            self.emit_atom(atom);
            loop {
                {
                    let top = self.revpol.last_mut().unwrap();
                    top.visited += 1;
                    if top.visited != top.tok.stage {
                        break;
                    }
                }
                let entry = self.revpol.last().unwrap().clone();
                self.emit_op(&entry);
                if entry.paren {
                    self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, entry.id);
                } else {
                    self.emit.close_group(entry.id);
                }
                self.revpol.pop();
                if self.revpol.is_empty() {
                    break;
                }
            }
        }
    }

    /// C++ `PrintLanguage::pushVn` (printlanguage.cc:197).  Queue an implied
    /// Varnode whose producing expression will be recursed.  Inputs of one op
    /// are pushed in reverse order (C++ comment).
    pub fn push_vn(&mut self, vn: usize, op: usize, m: uint4) {
        self.nodepend.push(crate::printlanguage::NodePending::new(vn, op, m));
    }

    /// C++ `PrintLanguage::recurse` (printlanguage.cc:521).  Resolve every
    /// pending Varnode the current op claimed: in C++ an implied one expands its
    /// defining op (`defOp->getOpcode()->push`) and an explicit one becomes a
    /// leaf atom (`pushVnExplicit`).
    ///
    /// SEAM(decompile-passes): the implied-op `push` dispatch and the explicit
    /// `pushVnExplicit` symbol/constant resolution need the seamed
    /// Symbol/HighVariable/Datatype/TypeOp graph (absent in the merged tree).
    /// The `op_binary`/`op_unary` scaffold above therefore pushes already-
    /// resolved leaf [`Atom`]s directly (never via `push_vn`), so on the tested
    /// path `nodepend` is empty and this drains nothing.  When the upstream
    /// passes land and the body driver stages implied varnodes, this restores
    /// the C++ claim/pop loop; the pop-without-dispatch here just guarantees
    /// termination until then.
    pub fn recurse(&mut self) {
        let modsave = self.context.mods();
        let last_pending = self.pending;
        self.pending = self.nodepend.len();
        // C++: while (lastPending < pending) { pop nodepend.back(); ... }
        while self.nodepend.len() > last_pending {
            if let Some(pend) = self.nodepend.pop() {
                self.context.set_mods(pend.vnmod);
                // SEAM(decompile-passes): no implied/explicit leaf expansion.
            }
            self.pending = self.nodepend.len();
        }
        self.context.set_mods(modsave);
    }

    /// C++ `PrintLanguage::opBinary` (printlanguage.cc:553) — the data-flow-free
    /// scaffold: push the operator, then its two operand atoms (supplied by the
    /// caller as the IR-leaf seam).  The negate-token flip is applied.
    pub fn op_binary(&mut self, tok: &'static OpToken, op: Option<usize>, lhs: &Atom, rhs: &Atom) {
        let tok = if self.context.is_set(modifiers::NEGATETOKEN) {
            self.context.unset_mod(modifiers::NEGATETOKEN);
            token_negate(tok).unwrap_or(tok)
        } else {
            tok
        };
        self.push_op(tok, op);
        // C++ pushes in[1] then in[0]; pushAtom drains in stack order, so the
        // operands print in0 <op> in1.
        self.push_atom(lhs);
        self.push_atom(rhs);
    }

    /// C++ `PrintLanguage::opUnary` (printlanguage.cc:573) — the scaffold form.
    pub fn op_unary(&mut self, tok: &'static OpToken, op: Option<usize>, operand: &Atom) {
        self.push_op(tok, op);
        self.push_atom(operand);
    }

    /// C++ `PrintLanguage::emitOp` (printlanguage.cc:332) — resolve final
    /// spacing / parens for one RPN entry at its current stage.  Mutates the
    /// entry's `id2` for surround tokens (mirrored back by the callers).
    fn emit_op(&mut self, entry_in: &ReversePolish) {
        let mut entry = entry_in.clone();
        match entry.tok.token_type {
            TokenType::Binary => {
                if entry.visited != 1 {
                    return;
                }
                self.emit.spaces(entry.tok.spacing, entry.tok.bump);
                self.emit.tag_op(entry.tok.print1, SyntaxHighlight::NoColor, &MarkupRef::none());
                self.emit.spaces(entry.tok.spacing, entry.tok.bump);
            }
            TokenType::UnaryPrefix => {
                if entry.visited != 0 {
                    return;
                }
                self.emit.tag_op(entry.tok.print1, SyntaxHighlight::NoColor, &MarkupRef::none());
                self.emit.spaces(entry.tok.spacing, entry.tok.bump);
            }
            TokenType::Postsurround => {
                if entry.visited == 0 {
                    return;
                }
                if entry.visited == 1 {
                    self.emit.spaces(entry.tok.spacing, entry.tok.bump);
                    entry.id2 = self.emit.open_paren(entry.tok.print1, 0);
                    self.emit.spaces(0, entry.tok.bump);
                } else {
                    self.emit.close_paren(entry.tok.print2, entry.id2);
                }
            }
            TokenType::Presurround => {
                if entry.visited == 2 {
                    return;
                }
                if entry.visited == 0 {
                    entry.id2 = self.emit.open_paren(entry.tok.print1, 0);
                } else {
                    self.emit.close_paren(entry.tok.print2, entry.id2);
                    self.emit.spaces(entry.tok.spacing, entry.tok.bump);
                }
            }
            TokenType::Space => {
                if entry.visited != 1 {
                    return;
                }
                self.emit.spaces(entry.tok.spacing, entry.tok.bump);
            }
            TokenType::HiddenFunction => {
                // Never directly prints anything.
            }
        }
        // Persist any id2 update for the corresponding stack entry: find the
        // top entry whose token/id matches and copy id2 (the only mutated
        // field).  push_op/push_atom re-read the top after calling emit_op.
        if let Some(top) = self.revpol.last_mut() {
            if std::ptr::eq(top.tok, entry.tok) && top.id == entry.id {
                top.id2 = entry.id2;
            }
        }
    }

    /// C++ `PrintLanguage::emitAtom` (printlanguage.cc:379) — send a leaf token
    /// to the low-level emitter according to its tag type.
    fn emit_atom(&mut self, atom: &Atom) {
        let markup = MarkupRef::none();
        match atom.tag {
            TagType::Syntax => self.emit.print(&atom.name, to_emit_hl(atom.highlight)),
            TagType::VarToken => {
                self.emit.tag_variable(&atom.name, to_emit_hl(atom.highlight), &markup)
            }
            TagType::FuncToken => {
                self.emit.tag_func_name(&atom.name, to_emit_hl(atom.highlight), &markup)
            }
            TagType::OpToken => self.emit.tag_op(&atom.name, to_emit_hl(atom.highlight), &markup),
            TagType::TypeToken => {
                self.emit.tag_type(&atom.name, to_emit_hl(atom.highlight), &markup)
            }
            TagType::FieldToken => {
                self.emit.tag_field(&atom.name, to_emit_hl(atom.highlight), atom.offset, &markup)
            }
            TagType::BitFieldToken => {
                self.emit.tag_bit_field(&atom.name, to_emit_hl(atom.highlight), atom.offset, &markup)
            }
            TagType::CaseToken => {
                let value = match atom.data {
                    crate::printlanguage::AtomData::IntValue(v) => v,
                    _ => 0,
                };
                self.emit.tag_case_label(&atom.name, to_emit_hl(atom.highlight), &markup, value)
            }
            TagType::BlankToken => {} // Print nothing.
        }
    }

    /// C++ `PrintLanguage::parentheses` against the current RPN top
    /// (printlanguage.cc:270 reads `revpol.back()`).  Delegates to the pure
    /// [`crate::printlanguage::parentheses`] with the previous token for the
    /// `HiddenFunction` arm.
    fn parentheses_top(&self, op2: &OpToken) -> bool {
        let top = self.revpol.last().expect("parentheses on empty revpol");
        let prev = if self.revpol.len() > 1 {
            Some(self.revpol[self.revpol.len() - 2].tok)
        } else {
            None
        };
        parentheses(top, op2, prev)
    }
}

/// Convert a [`crate::printlanguage::SyntaxHighlight`] (the [`Atom`] field, the
/// forward placeholder) to the [`prettyprint`](crate::prettyprint) enum the
/// [`Emit`] driver consumes.  Both carry the same 11 discriminants in the same
/// order (printlanguage.hh / prettyprint.hh "must match ClangToken").
fn to_emit_hl(hl: crate::printlanguage::SyntaxHighlight) -> SyntaxHighlight {
    use crate::printlanguage::SyntaxHighlight as Pl;
    match hl {
        Pl::keyword_color => SyntaxHighlight::KeywordColor,
        Pl::comment_color => SyntaxHighlight::CommentColor,
        Pl::type_color => SyntaxHighlight::TypeColor,
        Pl::funcname_color => SyntaxHighlight::FuncnameColor,
        Pl::var_color => SyntaxHighlight::VarColor,
        Pl::const_color => SyntaxHighlight::ConstColor,
        Pl::param_color => SyntaxHighlight::ParamColor,
        Pl::global_color => SyntaxHighlight::GlobalColor,
        Pl::no_color => SyntaxHighlight::NoColor,
        Pl::error_color => SyntaxHighlight::ErrorColor,
        Pl::special_color => SyntaxHighlight::SpecialColor,
    }
}

#[cfg(test)]
mod tests;
