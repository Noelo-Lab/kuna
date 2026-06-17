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
use kuna_base::space::AddrSpace;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, int8, uint4, uintb};

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

/// C++ `TypeOpSubpiece::computeByteOffsetForComposite(op)` (typeop.cc:2197): the
/// byte offset of the truncated piece into the assumed composite input, by
/// endianness.  `byteOff = isBigEndian ? (in0Size - outSize - lsb) : lsb`.
fn subpiece_byte_offset_for_composite(fd: &Funcdata, op: OpId) -> int8 {
    let o = match fd.obank().get(op) {
        Some(o) => o,
        None => return 0,
    };
    let lsb = o
        .get_in(1)
        .and_then(|v| fd.vbank().get(v))
        .map(|v| v.get_offset() as int8)
        .unwrap_or(0);
    let in0 = match o.get_in(0) {
        Some(v) => v,
        None => return lsb,
    };
    let big_endian = fd
        .vbank()
        .get(in0)
        .map(|v| v.get_space().is_big_endian())
        .unwrap_or(false);
    if big_endian {
        let in0_size = fd.vbank().get(in0).map(|v| v.get_size()).unwrap_or(0) as int8;
        let out_size = o
            .get_out()
            .and_then(|v| fd.vbank().get(v))
            .map(|v| v.get_size())
            .unwrap_or(0) as int8;
        in0_size - out_size - lsb
    } else {
        lsb
    }
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

// ===========================================================================
// The IR-coupled statement-body driver (w10-structure-printbody).
//
// This is the W9-emit seam closure: the per-statement RPN expression emission
// over the *structured* `sblocks` tree (C++ `PrintC::emitBlockGraph` ->
// `emitBlock{Copy,Basic,Ls,If,...}` -> `emitStatement` -> `emitExpression` ->
// `op->getOpcode()->push(...)` -> `recurse`).  It drives the (already ported and
// unit-tested) RPN engine above (`push_op`/`push_atom`) over the real
// `Funcdata` IR, resolving each Varnode leaf via `push_vn_explicit_ir` (the
// faithful `pushVnExplicit`: annotation/constant/register/`dat_<addr>` naming).
//
// The leaf-naming falls back to the address-/register-based form when no
// HighVariable Symbol is bound (Merge/naming is the next layer); the *structure*
// of the body (the if/else hierarchy, the statement sequence, the operator
// expressions, the comparison rendering) is fully driven here.
// ===========================================================================

use crate::architecture::Architecture;
use crate::cast::{CastContext, CastStrategy, CastStrategyC, OpRef, VnRef};
use crate::funcdata::Funcdata;
use crate::seams::{BlockId, OpId, VarnodeId};
use kuna_num::opcodes::OpCode;
use kuna_base::space::RegisterLookup;

impl PrintC {
    /// Faithful transcription of C++ `PrintC::docFunction` (printc.cc:2790)
    /// driven over a real [`Funcdata`] + [`Architecture`]: emit the signature
    /// shell (real return type from the recovered proto), then the **structured
    /// body** (`emitBlockGraph(&fd->getStructure())`) when `sblocks` is present.
    ///
    /// When `sblocks` is empty (structuring declined at a seam) this falls back
    /// to the brace-matched shell so the output is still a complete function.
    pub fn doc_function_full(&mut self, fd: &Funcdata, arch: &Architecture) -> String {
        self.emit.set_output_stream();
        let markup = MarkupRef::none();
        let display = fd.get_display_name().to_string();
        // Return type from the recovered proto output (C++ `getFuncProto().
        // getOutputType()`), defaulting to "void".  The output storage/type is
        // recovered by `ActionOutputPrototype` (the stand-alone `ProtoStoreInternal`
        // path).  The TYPE NAME is the W8 `ActionInferTypes` surface: until it
        // lands the recovered output type is the size-correct but un-inferred
        // base (metatype UNKNOWN), rendered as `undefined<N>` — the documented
        // residual vs. the oracle's inferred `uint1`.
        let ret_type = if fd.get_func_proto().has_store() {
            fd.get_func_proto()
                .get_output_type()
                .map(type_name_for_decl)
                .unwrap_or_else(|| "void".to_string())
        } else {
            "void".to_string()
        };

        let id1 = self.emit.begin_function();
        // emitCommentFuncHeader(fd): the header warning comments (C++
        // printc.cc:2801) — the `Comment::warningheader` lines the analysis
        // buffered into `glb->commentdb` (e.g. "Inlined function: X").  The full
        // CommentSorter is a separate item; the header subset that `print C`
        // renders before the prototype is emitted here from the comment database.
        self.emit_comment_func_header(fd, arch);
        self.emit.tag_line(); // emitCommentFuncHeader trailing tagLine

        // emitFunctionDeclaration shell.
        let idp = self.emit.begin_func_proto();
        let idret = self.emit.begin_return_type(&markup);
        self.emit.tag_type(&ret_type, SyntaxHighlight::TypeColor, &markup);
        self.emit.end_return_type(idret);
        self.emit.spaces(1, 0);
        let id1g = self.emit.open_group();
        self.emit.tag_func_name(&display, SyntaxHighlight::FuncnameColor, &markup);
        let id2 = self.emit.open_paren("(", 0);
        // emitPrototypeInputs (printc.cc:2298): the recovered proto's parameter
        // list, or `void` when there are none.  Each `ProtoParameter` renders its
        // declared type + name (`twostruct *ptr`, `int8 a`) via the C-declarator
        // builder; the backing-`Symbol` path (`emitVarDecl`) is the W4 scope
        // surface, so the param's own stored name + type are used directly.
        self.emit_prototype_inputs(fd, &markup);
        self.emit.close_paren(")", id2);
        self.emit.close_group(id1g);
        self.emit.end_func_proto(idp);

        let id = self.emit.open_brace_indent("{", to_emit_brace(self.options.brace_func));
        // emitLocalVarDecls(fd) (printc.cc:2805 / emitGlobalVarDeclsRecursive +
        // the scope walk): one `<type> <name>;` per named local HighVariable, in
        // name order, followed by a blank separating line before the body.  The
        // ScopeLocal symbol walk is the W4 surface; we emit from the named
        // HighVariables directly (the `kuna_name` stand-in), which is the same set
        // of locals the scope would declare.
        let _emitted_decls = self.emit_local_var_decls(fd, arch);
        if fd.sblocks_get_size() != 0 {
            self.emit_function_body(fd, arch);
        } else {
            // Structuring declined: keep the brace-matched shell.
            self.emit.tag_line();
            self.emit.print(
                "/* WARNING: structured blocks unavailable (structuring declined at a seam) */",
                SyntaxHighlight::CommentColor,
            );
        }
        self.emit.close_brace_indent("}", id);
        self.emit.tag_line();
        self.emit.end_function(id1);
        self.emit.output().to_string()
    }

    /// Emit the function's header warning comments (C++
    /// `PrintC::emitCommentFuncHeader`, printc.cc:3434): the
    /// `Comment::warningheader` lines the analysis buffered into the comment
    /// database, indexed at the function entry address, rendered as
    /// `/* <text> */` lines before the prototype.
    ///
    /// The full `CommentSorter` (`header_basic`/`header_unplaced` sub-orderings,
    /// the `option_unplaced` / `option_nocasts` synthetic headers) is the comment
    /// item; this carries the `warningheader` subset `head_comment_type` shows by
    /// default, in insertion order (the order the analysis produced them, which is
    /// the order `CommentSorter` keeps for same-address header comments).
    fn emit_comment_func_header(&mut self, fd: &Funcdata, arch: &Architecture) {
        use crate::architecture::comment_type;
        let func_addr = fd.get_address();
        // Collect the matching header comments first (the commentdb borrow is
        // released before the `&mut self.emit` writes).
        let headers: Vec<String> = arch
            .commentdb
            .comments()
            .iter()
            .filter(|c| {
                c.tp == comment_type::warningheader && &c.func_addr == func_addr
            })
            .map(|c| c.text.clone())
            .collect();
        let space = match func_addr.get_space() {
            Some(s) => std::rc::Rc::clone(s),
            None => return,
        };
        let off = func_addr.get_offset();
        for text in headers {
            // emitLineComment(0, comm): a fresh line then the `/* text */` token.
            self.emit.tag_line();
            self.emit.tag_comment(
                &format!("/* {text} */"),
                SyntaxHighlight::CommentColor,
                &space,
                off,
            );
        }
    }

    /// Emit the function prototype's input parameter list (C++
    /// `PrintC::emitPrototypeInputs`, printc.cc:2298): `void` if there are no
    /// parameters, else the comma-separated `<type> <name>` declarations,
    /// followed by `, ...` for a vararg prototype.
    ///
    /// The C++ emits each parameter through its backing `Symbol` (`emitVarDecl`)
    /// when present, else the type with no name.  The merged-tree `ProtoParameter`
    /// has no backing `Symbol` (W4 scope), but it *does* carry the declared name +
    /// type (set by `update_all_types` from the parsed `PrototypePieces`), so the
    /// name + the C-declarator are rendered directly here — observationally the
    /// same text the C++ `emitVarDecl` produces for a named, typed parameter.
    fn emit_prototype_inputs(&mut self, fd: &Funcdata, markup: &MarkupRef) {
        let proto = fd.get_func_proto();
        if !proto.has_store() {
            self.emit.tag_type("void", SyntaxHighlight::TypeColor, markup);
            return;
        }
        let sz = proto.num_params();
        if sz == 0 {
            self.emit.tag_type("void", SyntaxHighlight::TypeColor, markup);
        } else {
            let mut print_comma = false;
            for i in 0..sz {
                let param = match proto.get_param(i) {
                    Some(p) => p,
                    None => continue,
                };
                // hide_thisparam + isThisPointer: the `this`-pointer hiding is the
                // C++ option/class-method surface (no `this` on the recovery path).
                // C++ `emit->print(COMMA)` with `COMMA = ","` — no trailing space.
                if print_comma {
                    self.emit.print(",", SyntaxHighlight::NoColor);
                }
                print_comma = true;
                let name = param.get_name();
                match param.get_type() {
                    Some(ty) => {
                        let (front, back) = declarator_parts(ty);
                        // C++ `pushTypeStart(type, noident)`: the separating token is
                        // `type_expr_nospace` only when there is no identifier AND no
                        // declarator modifier (`noident && typestack.size()==1`); else
                        // `type_expr_space`.  A `*` front glues to the name (no space).
                        let has_modifier = front.ends_with('*') || !back.is_empty();
                        self.emit.tag_type(&front, SyntaxHighlight::TypeColor, markup);
                        let want_space =
                            !front.ends_with('*') && (!name.is_empty() || has_modifier);
                        if want_space {
                            self.emit.spaces(1, 0);
                        }
                        if !name.is_empty() {
                            self.emit.tag_variable(name, SyntaxHighlight::VarColor, markup);
                        }
                        if !back.is_empty() {
                            self.emit.print(&back, SyntaxHighlight::NoColor);
                        }
                    }
                    None => {
                        self.emit.tag_type("void", SyntaxHighlight::TypeColor, markup);
                    }
                }
            }
        }
        if proto.is_dotdotdot() {
            if sz != 0 {
                self.emit.print(",", SyntaxHighlight::NoColor);
            }
            self.emit.print("...", SyntaxHighlight::NoColor);
        }
    }

    /// Emit one `<type> <name>;  // <storage>` declaration per named local
    /// HighVariable, in name order, returning `true` if any were emitted (C++
    /// `emitLocalVarDecls` + `emitVarDeclStatement`, printc.cc:2652).  The W4
    /// ScopeLocal symbol walk is the missing surface; the named HighVariables
    /// (`kuna_name`) are the same locals the scope would declare.  A trailing
    /// blank `tag_line` separates the decl block from the body (the C++ blank line
    /// `emitVarDecl`s produce before the statement list).
    pub fn emit_local_var_decls(&mut self, fd: &Funcdata, arch: &Architecture) -> bool {
        // Collect (name, type_name, storage_comment) for each named local high,
        // de-duplicated by high and ordered by name.
        let mut decls: Vec<(crate::seams::HighVariableId, String)> = Vec::new();
        let mut seen: std::collections::BTreeSet<crate::seams::HighVariableId> =
            std::collections::BTreeSet::new();
        let vlist: Vec<crate::seams::VarnodeId> = fd.vbank().iter_loc().collect();
        for vn in vlist {
            let high = match fd.vbank().get(vn).and_then(|v| v.get_high()) {
                Some(h) => h,
                None => continue,
            };
            if seen.contains(&high) {
                continue;
            }
            let name = match fd.high_bank().get(high).and_then(|h| h.kuna_name()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            seen.insert(high);
            // A `&symbol` reference's offset is a CONSTANT operand of the PTRSUB
            // markup, not a scope Symbol.  When a constant-only reference high
            // SHADOWS a real local that is already declared from its own storage —
            // i.e. there is a whole-symbol sibling (`kuna_symbol_offset == -1`) with
            // the same name — the constant reference must NOT be declared a second
            // time (C++ `emitScopeVarDecls` walks the ScopeLocal Symbols once
            // (printc.cc:2667); the `&val` reference renders inline via the PTRSUB
            // markup).  Skipping it removes the spurious `int8 val;` shadow of the
            // real `int4 val` stack local.  The whole-sibling guard is load-bearing:
            // a constant-only reference with NO real sibling (the stack array `c` in
            // passPtrToArray, materialized only through `&c`) IS the symbol's sole
            // declaration and must still print.
            let all_constant = fd
                .high_bank()
                .get(high)
                .map(|h| {
                    let n = h.num_instances();
                    n > 0
                        && (0..n).all(|i| {
                            fd.vbank()
                                .get(h.get_instance(i))
                                .map(|v| v.is_constant())
                                .unwrap_or(false)
                        })
                })
                .unwrap_or(false);
            // A real local with the same name exists when another HighVariable
            // carries that name and has at least one NON-constant (storage)
            // instance — the stack/register varnodes the printer declares from.
            let has_storage_sibling = all_constant
                && fd.high_bank().iter().any(|(id, h)| {
                    id != high
                        && h.kuna_name() == Some(name.as_str())
                        && (0..h.num_instances()).any(|i| {
                            fd.vbank()
                                .get(h.get_instance(i))
                                .map(|v| !v.is_constant())
                                .unwrap_or(false)
                        })
                });
            if has_storage_sibling {
                continue;
            }
            // C++ `emitLocalVarDecls` -> `emitScopeVarDecls(fd->getScopeLocal(),
            // no_category)` walks the LOCAL scope only (printc.cc:2336/2667).  A
            // global-mapped Symbol (`glob1`, `globalfree`, `myarray`) lives in the
            // GLOBAL scope, so it is never declared in a function body — it is named
            // in the body's statements but carries no local declaration.  Two global
            // discriminators, both faithful to "not in `fd->getScopeLocal()`":
            //   * `Varnode::isPersist` — a persistent global RAM store/load high
            //     (`glob1 = 0`), whose member varnodes are flagged persist; and
            //   * `HighVariable::kuna_global` — a `&symbol` reference whose
            //     `linkSpacebaseSymbol` resolved through the GLOBAL scope
            //     (`sb->getMap()` == the global scope for a ram spacebase, e.g.
            //     `myarray` materialized as a const base address).  A *local*-frame
            //     spacebase reference (`&a`, `&myval.b`, `&c` in passPtrToArray) is
            //     NOT flagged, so its stack-symbol decl still prints.
            let is_global = fd
                .high_bank()
                .get(high)
                .map(|h| {
                    h.kuna_global()
                        || (0..h.num_instances()).any(|i| {
                            fd.vbank().get(h.get_instance(i)).map(|v| v.is_persist()).unwrap_or(false)
                        })
                })
                .unwrap_or(false);
            if is_global {
                continue;
            }
            // SEAM A — C++ `emitScopeVarDecls`: `if (entry->isPiece()) continue;`
            // (printc.cc:2688) plus the multi-entry `getFirstWholeMap() != entry`
            // skip (printc.cc:2697).  A register-returned struct is split into
            // per-field proto-partial pieces (`RulePieceStructure`); each piece's
            // HighVariable is bound to the ROOT's name + the ROOT's whole-struct type
            // + its own in-symbol byte offset (`bind_proto_partial_piece`,
            // coreaction_cleanup.rs).  C++ shares ONE declaration for the whole
            // Symbol (the `getFirstWholeMap()` entry, type `foo`); the pieces are
            // partial entries and emit none.
            //
            // The kuna stand-in for "this entry is a piece of a multi-entry Symbol
            // whose first-whole-map is a different entry": the piece carries a
            // composite `kuna_symbol_type` (the root's struct/array/union whole type,
            // which a scalar field varnode never has on its own) AND a sibling ROOT
            // high exists — same shared name, `kuna_symbol_offset == -1` (the whole
            // keeps the `-1` default; pieces carry `>= 0`).  The sibling root IS the
            // `getFirstWholeMap()` entry and declares the shared `foo v1;` from its
            // own whole-struct varnode type.  A referenced *whole* local (`&a`,
            // `&myval.b` in passPtrToArray) carries a composite `kuna_symbol_type`
            // too but has NO `-1` sibling of the same name, so it stays declarable.
            let is_proto_partial_piece = fd.high_bank().get(high).is_some_and(|h| {
                let composite = h.kuna_symbol_type().is_some_and(|t| {
                    use crate::dtype::type_metatype::*;
                    matches!(t.get_metatype(), TYPE_STRUCT | TYPE_ARRAY | TYPE_UNION)
                });
                composite
                    && h.kuna_symbol_offset() >= 0
                    && high_name_has_whole_sibling(fd, high, &name)
            });
            if is_proto_partial_piece {
                continue;
            }
            // C++ `emitLocalVarDecls` -> `emitScopeVarDecls(scope, no_category)`:
            // only `no_category` Symbols are declared in the body.  A high bound to
            // a `function_parameter` Symbol renders in the signature, never as a body
            // local — skip it.  The high carries the parameter Symbol (C++
            // `linkSymbol` binds the parameter entry to the high), so any member
            // varnode whose storage covers a `function_parameter` Symbol marks the
            // whole high as a parameter.
            let scope = fd.get_scope_local();
            let is_param = scope
                .map(|lm| {
                    let h = fd.high_bank().get(high);
                    let n = h.map(|h| h.num_instances()).unwrap_or(0);
                    // A high is a parameter (declared in the signature, not the body)
                    // only when a `function_parameter` Symbol *contains* a member's
                    // whole storage — the C++ `emitScopeVarDecls(no_category)` walks
                    // Symbols by their own category, not by storage overlap.  Using a
                    // containing query (not bare overlap) is load-bearing: a wider
                    // local merged onto a register that also holds a narrower
                    // parameter (a `float8` cast result on `XMM0`, which also carries
                    // the `float4` arg) overlaps the parameter entry but is its own
                    // `no_category` local (the C++ `handleSymbolConflict` conflict
                    // spawns a fresh Symbol), so it must still be declared.
                    (0..n).any(|i| {
                        let m = h.unwrap().get_instance(i);
                        fd.vbank()
                            .get(m)
                            .map(|v| (v.get_addr().clone(), v.get_size()))
                            .and_then(|(addr, size)| lm.containing_category_for_varnode(&addr, size))
                            == Some(crate::database::symbol_category::FUNCTION_PARAMETER)
                    })
                })
                .unwrap_or(false);
            if is_param {
                continue;
            }
            decls.push((high, name));
        }
        // C++ `emitScopeVarDecls` walks the ScopeLocal *Symbol* table and emits
        // exactly one declaration per multi-entry Symbol (the `getFirstWholeMap()`
        // entry; printc.cc:2696).  The kuna printer instead walks HighVariables and
        // dedups by high id, so a single mapped composite Symbol that is
        // represented by several `&symbol`-reference highs (each a piece of the
        // array/struct, all constant-only PTRSUB operands) is declared once per
        // high — a spurious repeat like `int2 arr [32]; int2 arr [32];`.
        //
        // Collapse to one declaration per Symbol: two declared highs are the same
        // Symbol when they share a name AND the same *composite* mapped type by Rc
        // identity.  The type factory interns array/struct/union types, so one
        // mapped Symbol's pieces all carry the identical `kuna_symbol_type` Rc;
        // distinct same-shaped locals are disambiguated by their (unique) names.
        // Restricting to composites is load-bearing: primitive types are shared by
        // every scalar local of that type, so `(name, int4-Rc)` would not identify
        // a single Symbol — scalars keep the per-high behavior.
        {
            let mut seen_sym: std::collections::HashSet<(String, usize)> = std::collections::HashSet::new();
            decls.retain(|(high, name)| {
                let composite_rc = fd.high_bank().get(*high).and_then(|h| {
                    let t = h.kuna_symbol_type()?;
                    use crate::dtype::type_metatype::*;
                    matches!(t.get_metatype(), TYPE_ARRAY | TYPE_STRUCT | TYPE_UNION)
                        .then(|| std::rc::Rc::as_ptr(t) as usize)
                });
                match composite_rc {
                    Some(rc) => seen_sym.insert((name.clone(), rc)),
                    None => true,
                }
            });
        }
        decls.sort_by(|a, b| a.1.cmp(&b.1));
        if decls.is_empty() {
            return false;
        }
        let markup = MarkupRef::none();
        for (high, name) in &decls {
            // Type: the high's recovered type name (W8-unknown -> `undefined<N>`).
            let (type_name, comment) = self.local_decl_type_and_comment(fd, arch, *high);
            // Array member: if the mapped Symbol is an array, declare the base
            // type and an `[count]` adornment after the name (C++ `emitVarDecl`'s
            // array branch).
            let array_count = fd.high_bank().get(*high).and_then(|h| {
                let st = h.kuna_symbol_type()?;
                if st.get_metatype() == crate::dtype::type_metatype::TYPE_ARRAY {
                    let base = st.get_array_base()?;
                    let elsize = base.get_size().max(1);
                    let count = st.get_size() / elsize;
                    Some((type_name_for_decl(&base), count))
                } else {
                    None
                }
            });
            self.emit.tag_line();
            let id = self.emit.begin_var_decl(&markup);
            let decl_type = array_count.as_ref().map(|(t, _)| t.clone()).unwrap_or(type_name);
            self.emit.tag_type(&decl_type, SyntaxHighlight::TypeColor, &markup);
            self.emit.spaces(1, 0);
            self.emit.tag_variable(name, SyntaxHighlight::VarColor, &markup);
            if let Some((_, count)) = &array_count {
                // ` [count]` (C++ `emitArrayDecl`: a space then the bracketed count).
                self.emit.spaces(1, 0);
                self.emit.print("[", SyntaxHighlight::NoColor);
                self.emit.print(&format!("{count}"), SyntaxHighlight::ConstColor);
                self.emit.print("]", SyntaxHighlight::NoColor);
            }
            self.emit.end_var_decl(id);
            self.emit.print(";", SyntaxHighlight::NoColor);
            if let Some((ctext, spc, off)) = comment {
                self.emit.spaces(1, 0);
                self.emit.tag_comment(&format!("// {ctext}"), SyntaxHighlight::CommentColor, &spc, off);
            }
        }
        // Blank separating line before the body (C++ emits a tag_line after the
        // last decl; the body's first statement then starts on its own line).
        self.emit.tag_line();
        true
    }

    /// The declaration type name + storage comment for a named local high.  The
    /// comment is the angr `kunaStorageComment` (register name lowercased) for the
    /// high's name representative.
    fn local_decl_type_and_comment(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        high: crate::seams::HighVariableId,
    ) -> (String, Option<(String, std::rc::Rc<kuna_base::space::AddrSpace>, u64)>) {
        let h = match fd.high_bank().get(high) {
            Some(h) => h,
            None => return ("undefined1".to_string(), None),
        };
        // Type name + storage comment: from the high's storage representative —
        // the addr-tied (mapped, in-scope) member, which is the C++ symbol's
        // `getFirstWholeMap()` storage (e.g. the ACC register), NOT a trim-COPY
        // unique.  Fall back to instance 0 if none is addr-tied.
        let rep = (0..h.num_instances())
            .map(|i| h.get_instance(i))
            .find(|&vn| fd.vbank().get(vn).map(|v| v.is_addr_tied()).unwrap_or(false))
            .or_else(|| (0..h.num_instances()).map(|i| h.get_instance(i)).next());
        let (type_name, comment) = match rep.and_then(|vn| fd.vbank().get(vn)) {
            Some(v) => {
                let tn = type_name_for_decl(v.get_type());
                let loc = v.get_addr().clone();
                let size = v.get_size();
                let comment = loc.get_space().and_then(|spc| {
                    let regname = arch.translate().get_register_name(spc, loc.get_offset(), size);
                    if !regname.is_empty() {
                        // kunaStorageComment: register name lowercased.
                        return Some((regname.to_ascii_lowercase(), spc.clone(), loc.get_offset()));
                    }
                    // Stack local: `// stack - 0xNN` / `// stack + 0xNN`
                    // (C++ `kunaStorageComment` for a spacebase local).
                    if spc.get_index() == fd.get_arch().manage().get_stack_space().map(|s| s.get_index()).unwrap_or(-99) {
                        // For an array/struct member the declaration is anchored at
                        // the Symbol base, so subtract the in-symbol byte offset.
                        let sym_off = h.kuna_symbol_offset();
                        let base_off = if sym_off > 0 {
                            loc.get_offset().wrapping_sub(sym_off as u64)
                        } else {
                            loc.get_offset()
                        };
                        // Signed offset within the stack space.
                        let signed = kuna_base::address::sign_extend(base_off as i64, (spc.get_addr_size() as i32) * 8 - 1);
                        let text = if signed < 0 {
                            format!("stack - {:#x}", (-signed) as u64)
                        } else {
                            format!("stack + {:#x}", signed as u64)
                        };
                        return Some((text, spc.clone(), loc.get_offset()));
                    }
                    None
                });
                (tn, comment)
            }
            None => ("undefined1".to_string(), None),
        };
        (type_name, comment)
    }

    /// Emit the structured function body into the open brace (C++
    /// `emitLocalVarDecls(fd)` + `emitBlockGraph(&fd->getStructure())`,
    /// printc.cc:2805-2809).  Local var decls need the Symbol table (the merge/
    /// naming layer); the structured block graph walk is driven here.
    pub fn emit_function_body(&mut self, fd: &Funcdata, arch: &Architecture) {
        let sroot = match fd.sblocks_ref().root {
            Some(r) => r,
            None => return,
        };
        self.emit_block_graph(fd, arch, sroot);
    }

    /// C++ `PrintC::emitBlockGraph` (printc.cc:2895): emit each component block.
    fn emit_block_graph(&mut self, fd: &Funcdata, arch: &Architecture, graph: BlockId) {
        let list: Vec<BlockId> = fd.sblocks_ref().block(graph).get_list().to_vec();
        for blk in list {
            let id = self.emit.begin_block(0);
            self.emit_block(fd, arch, blk);
            self.emit.end_block(id);
        }
    }

    /// Dispatch one structured block to its emitter (C++ the virtual
    /// `FlowBlock::emit(PrintLanguage*)` -> `PrintC::emitBlock*`).
    fn emit_block(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        use crate::block::BlockType;
        match fd.sblocks_ref().block(blk).get_type() {
            BlockType::Copy => self.emit_block_copy(fd, arch, blk),
            BlockType::Basic => self.emit_block_basic(fd, arch, blk),
            BlockType::Ls => self.emit_block_ls(fd, arch, blk),
            BlockType::If => self.emit_block_if(fd, arch, blk),
            BlockType::Graph => self.emit_block_graph(fd, arch, blk),
            BlockType::Goto => self.emit_block_goto(fd, arch, blk),
            BlockType::WhileDo => self.emit_block_while_do(fd, arch, blk),
            BlockType::DoWhile => self.emit_block_do_while(fd, arch, blk),
            BlockType::InfLoop => self.emit_block_inf_loop(fd, arch, blk),
            BlockType::Condition => self.emit_block_condition(fd, arch, blk),
            BlockType::Switch => self.emit_block_switch(fd, arch, blk),
            // multigoto: its emitter is the next structuring layer.  Fall through
            // to the component blocks.
            _ => {
                let list: Vec<BlockId> = fd.sblocks_ref().block(blk).get_list().to_vec();
                for c in list {
                    self.emit_block(fd, arch, c);
                }
            }
        }
    }

    /// C++ `PrintC::emitBlockCondition` (printc.cc:2985): emit a `BlockCondition`
    /// (the two short-circuited `&&`/`||` clauses).
    ///
    /// The condition node has no statement body of its own; it is only emitted as
    /// the boolean expression of an enclosing `if`/loop.  In the `no_branch`
    /// state (the "statements before the branch" pass of `emitBlockIf`) only the
    /// first clause's leading statements print.  In the `only_branch`/
    /// `comma_separate` state (the branch-condition pass) the two clauses print
    /// glued by ` && ` / ` || `, each wrapped in parens — matching the C++
    /// `(a && b)` form.
    fn emit_block_condition(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        let b0 = fd.sblocks_ref().block(blk).get_block(0);
        // no_branch: emit only the first clause's leading statements.
        if self.context.is_set(modifiers::NO_BRANCH) {
            let id = self.emit.begin_block(0);
            self.emit_block(fd, arch, b0);
            self.emit.end_block(id);
            return;
        }
        if self.context.is_set(modifiers::ONLY_BRANCH) || self.context.is_set(modifiers::COMMA_SEPARATE)
        {
            let b1 = fd.sblocks_ref().block(blk).get_block(1);
            let opc = fd
                .sblocks_ref()
                .block(blk)
                .get_condition_opcode()
                .unwrap_or(OpCode::CPUI_BOOL_AND);

            let id = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
            self.emit_block(fd, arch, b0);
            self.context.push_mod();
            self.context.unset_mod(modifiers::ONLY_BRANCH);
            // comma_separate is placed only on the second block.
            self.context.set_mod(modifiers::COMMA_SEPARATE);

            // Emit the && / || token as if it were on the RPN stack (C++ builds a
            // ReversePolish with op==0, visited==1, and calls emitOp).
            let tok: &'static crate::printlanguage::OpToken = if opc == OpCode::CPUI_BOOL_AND {
                &tokens::BOOLEAN_AND
            } else {
                &tokens::BOOLEAN_OR
            };
            let pol = ReversePolish { tok, visited: 1, paren: false, op: None, id: 0, id2: 0 };
            self.emit_op(&pol);

            let id2 = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
            self.emit_block(fd, arch, b1);
            self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id2);
            self.context.pop_mod();
            self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id);
        }
    }

    /// C++ `PrintC::emitBlockCopy` (printc.cc:2908): emit the underlying basic
    /// block (the `BlockCopy.copy` points back into `bblocks`).
    fn emit_block_copy(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        // emitAnyLabelStatement(bl): labels need the goto/label machinery; skip.
        if let Some(under) = fd.sblocks_ref().block(blk).get_copy() {
            // The copy's `copy` field is a *bblocks* BlockId.
            self.emit_basic_block_ops(fd, arch, under, true);
        }
    }

    /// C++ `PrintC::emitBlockBasic` for an sblocks Basic node (rare in the
    /// structured tree, but handled for completeness): the node *is* a basic
    /// block in the sblocks arena.
    fn emit_block_basic(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.emit_basic_block_ops(fd, arch, blk, false);
    }

    /// C++ `PrintC::emitBlockLs` (printc.cc:2930): emit a list of blocks in
    /// sequence.  The first block keeps its branch suppressed (`no_branch`); the
    /// last block keeps the caller's branch state.  The per-edge `nextInFlow`
    /// goto-insertion (the `nofallthru` arm) is the goto-labeling layer; the
    /// structured list emitted here flows in declaration order.
    fn emit_block_ls(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        let list: Vec<BlockId> = fd.sblocks_ref().block(blk).get_list().to_vec();
        if self.context.is_set(modifiers::ONLY_BRANCH) {
            if let Some(&last) = list.last() {
                self.emit_block(fd, arch, last);
            }
            return;
        }
        if list.is_empty() {
            return;
        }
        let n = list.len();
        // First block: no_branch (unless flat).
        let id1 = self.emit.begin_block(0);
        self.context.push_mod();
        if !self.is_flat() {
            self.context.set_mod(modifiers::NO_BRANCH);
        }
        self.emit_block(fd, arch, list[0]);
        self.emit.end_block(id1);
        // Middle blocks: no_branch.
        for &subbl in list.iter().take(n.saturating_sub(1)).skip(1) {
            let id2 = self.emit.begin_block(0);
            self.emit_block(fd, arch, subbl);
            self.emit.end_block(id2);
        }
        self.context.pop_mod();
        // Final block: caller's branch state.
        let id3 = self.emit.begin_block(0);
        self.emit_block(fd, arch, list[n - 1]);
        self.emit.end_block(id3);
    }

    /// C++ `PrintC::emitBlockIf` (printc.cc:3027): the `if (cond) { ... }` form
    /// (with optional `else`).  Block 0 is the condition, block 1 the true body,
    /// block 2 (optional) the else body.
    fn emit_block_if(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        let size = fd.sblocks_ref().block(blk).get_size();
        let cond_block = fd.sblocks_ref().block(blk).get_block(0);
        // if-block never prints final branch: clear no_branch/only_branch.
        self.context.push_mod();
        self.context.unset_mod(
            modifiers::NO_BRANCH | modifiers::ONLY_BRANCH | modifiers::PENDING_BRACE,
        );

        // Emit the condition block's statements (no_branch) ...
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, cond_block);
        self.context.pop_mod();
        self.emit.tag_line();

        // ... then `if ` + the branch condition (only_branch).
        self.emit.tag_op(keywords::KEYWORD_IF, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        self.emit.spaces(1, 0);
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH);
        self.emit_block(fd, arch, cond_block);
        self.context.pop_mod();

        // If the if has an unstructured-branch target, emit a goto/break/continue
        // instead of a braced body (C++ printc.cc:3063).
        let goto_target = fd.sblocks_ref().block(blk).get_if_goto_target();
        if let Some(target) = goto_target {
            self.emit.spaces(1, 0);
            self.emit_goto_statement(fd, cond_block, target, fd.sblocks_ref().block(blk).get_if_goto_type());
            self.context.pop_mod();
            return;
        }

        // The true body in braces.
        self.context.set_mod(modifiers::NO_BRANCH);
        let id = self
            .emit
            .open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_ifelse));
        let id1 = self.emit.begin_block(0);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
        self.emit.end_block(id1);
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, id);

        // Optional else.
        if size == 3 {
            self.emit.tag_line();
            self.emit.print(keywords::KEYWORD_ELSE, SyntaxHighlight::KeywordColor);
            let else_block = fd.sblocks_ref().block(blk).get_block(2);
            let else_is_if = fd.sblocks_ref().block(else_block).get_type()
                == crate::block::BlockType::If;
            if else_is_if {
                // `else if` merge: pending_brace.
                self.context.set_mod(modifiers::PENDING_BRACE);
                let id2 = self.emit.begin_block(0);
                self.emit_block(fd, arch, else_block);
                self.emit.end_block(id2);
            } else {
                let id2 = self
                    .emit
                    .open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_ifelse));
                let id3 = self.emit.begin_block(0);
                self.emit_block(fd, arch, else_block);
                self.emit.end_block(id3);
                self.emit.close_brace_indent(keywords::CLOSE_CURLY, id2);
            }
        }
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitBlockSwitch` (printc.cc:3470): emit a `BlockSwitch` — the
    /// statements before the switch, the `switch(v)` header, then the braced body
    /// of `case N:` / `default:` arms.
    fn emit_block_switch(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        // getSwitchBlock() == getBlock(0) (the switch component).
        let switch_block = fd.sblocks_ref().block(blk).get_block(0);

        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        // Statements before the branch (no_branch).
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, switch_block);
        self.context.pop_mod();
        self.emit.tag_line();
        // The `switch(v)` header (only_branch|comma_separate).
        self.context.push_mod();
        self.context.set_mod(modifiers::ONLY_BRANCH | modifiers::COMMA_SEPARATE);
        self.emit_block(fd, arch, switch_block);
        self.context.pop_mod();
        let brace_id =
            self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_switch));

        let ncase = fd.sblocks_ref().block(blk).switch_caseblocks().len();
        for i in 0..ncase {
            self.emit_switch_case(fd, arch, blk, i);
            let id = self.emit.start_indent();
            let gototype = fd.sblocks_ref().block(blk).switch_caseblocks()[i].gototype;
            if gototype != 0 {
                self.emit.tag_line();
                let caseblk = fd.sblocks_ref().block(blk).switch_caseblocks()[i].block;
                self.emit_goto_statement(fd, switch_block, caseblk, gototype);
            } else {
                let caseblk = fd.sblocks_ref().block(blk).switch_caseblocks()[i].block;
                let id2 = self.emit.begin_block(0);
                self.emit_block(fd, arch, caseblk);
                // Blocks that formally exit the switch need an explicit `break;`
                // (unless it is the last case, whose fall-through is the close).
                let isexit = fd.sblocks_ref().block(blk).switch_caseblocks()[i].isexit;
                if isexit && i != ncase - 1 {
                    self.emit.tag_line();
                    self.emit_goto_statement(fd, caseblk, caseblk, crate::block::block_flags::f_break_goto);
                }
                self.emit.end_block(id2);
            }
            self.emit.stop_indent(id);
        }
        self.emit.tag_line();
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, brace_id);
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitSwitchCase` (printc.cc:3278): emit the `case N:` /
    /// `default:` label(s) for one case arm.
    fn emit_switch_case(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId, casenum: usize) {
        let case = fd.sblocks_ref().block(blk).switch_caseblocks()[casenum].clone();
        // op = getCaseBlock(casenum)->firstOp() — used only for markup tagging.
        let firstop = self.case_first_op(fd, case.block);

        if case.isdefault {
            // default: (the label value is informational; default emits no value).
            self.emit.tag_line();
            self.emit.tag_case_label(
                keywords::KEYWORD_DEFAULT,
                SyntaxHighlight::KeywordColor,
                &MarkupRef::none(),
                case.label,
            );
            self.emit.print(keywords::COLON, SyntaxHighlight::NoColor);
        } else {
            // case <label>: — one line per index targeting this case.
            let jt_index = fd.sblocks_ref().block(blk).switch_jt_index();
            let nlabels = match (jt_index, case.basicblock) {
                (Some(j), Some(bb)) => {
                    fd.get_jump_table(j as int4).num_indices_by_block(fd, bb).unwrap_or(1).max(1)
                }
                _ => 1,
            };
            for i in 0..nlabels {
                let val = match (jt_index, case.basicblock) {
                    (Some(j), Some(bb)) => {
                        let ind = fd
                            .get_jump_table(j as int4)
                            .get_index_by_block(fd, bb, i)
                            .unwrap_or(0);
                        fd.get_jump_table(j as int4).get_label_by_index(ind)
                    }
                    _ => case.label,
                };
                self.emit.tag_line();
                self.emit.print(keywords::KEYWORD_CASE, SyntaxHighlight::KeywordColor);
                self.emit.spaces(1, 0);
                // pushConstant(val, ct, casetoken, 0, op, displayFormat); recurse();
                let sz = self.switch_var_size(fd, blk);
                if let Some(op) = firstop {
                    self.push_constant_ir(val, sz, op);
                } else if let Some(op) = self.any_op(fd, case.block) {
                    self.push_constant_ir(val, sz, op);
                }
                self.recurse();
                self.emit.print(keywords::COLON, SyntaxHighlight::NoColor);
            }
        }
        let _ = arch;
    }

    /// First op of a case block (C++ `FlowBlock::firstOp` → front-leaf basic
    /// block's first op), used only for case-label markup tagging.
    fn case_first_op(&self, fd: &Funcdata, caseblk: BlockId) -> Option<OpId> {
        let front = fd.sblocks_ref().get_front_leaf(caseblk)?;
        let bb = fd.sblocks_ref().sub_block(front, 0)?;
        fd.bb_op_head(bb)
    }

    /// Any op tag in a case block (fallback for markup when the block is empty).
    fn any_op(&self, fd: &Funcdata, caseblk: BlockId) -> Option<OpId> {
        self.case_first_op(fd, caseblk)
    }

    /// The byte-size of the switch variable (C++ `getSwitchType()` size), used to
    /// format the case-label constant.  Resolved from the BRANCHIND's `in0`.
    fn switch_var_size(&self, fd: &Funcdata, blk: BlockId) -> int4 {
        let jt_index = match fd.sblocks_ref().block(blk).switch_jt_index() {
            Some(j) => j,
            None => return 4,
        };
        let indop = match fd.get_jump_table(jt_index as int4).get_indirect_op() {
            Some(op) => op,
            None => return 4,
        };
        fd.obank()
            .get(indop)
            .and_then(|o| o.get_in(0))
            .and_then(|vn| fd.vbank().get(vn))
            .map(|v| v.get_size())
            .unwrap_or(4)
    }

    /// C++ `PrintC::emitBlockGoto` (printc.cc:2915): emit the block's body
    /// (no_branch) then the trailing `goto`/`break`/`continue` statement.
    ///
    /// SEAM(W7): `BlockGoto::gotoPrints` consults `getParent()->nextFlowAfter` to
    /// suppress a `goto` to the very next printed block; `nextFlowAfter` is not
    /// yet ported, so the goto is always emitted when a target is present (an
    /// over-emit, never an under-emit — a redundant `goto LAB_x;` to the
    /// fallthrough where C++ would drop it).  Recorded as a loss.
    fn emit_block_goto(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        let inner = fd.sblocks_ref().block(blk).get_block(0);
        self.emit_block(fd, arch, inner);
        self.context.pop_mod();
        // gotoPrints(): emit the trailing goto unless it targets the next block.
        if let Some(target) = fd.sblocks_ref().block(blk).get_goto_target() {
            self.emit.tag_line();
            let gototype = fd.sblocks_ref().block(blk).get_goto_type();
            self.emit_goto_statement(fd, inner, target, gototype);
        }
    }

    /// C++ `PrintC::emitForLoop` (printc.cc:3106): emit a `for (init; cond; iter)`
    /// header (with the init/iterate statements hoisted out of the body) followed
    /// by the loop body.  Reached from [`emit_block_while_do`] when the whiledo
    /// node carries an `iterateOp` (set by `Funcdata::finalize_forloop_*`).
    fn emit_for_loop(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        // (emitAnyLabelStatement / emitCommentBlockTree: not ported — same
        //  simplification as the plain while arm.)
        let cond_block = fd.sblocks_ref().block(blk).get_block(0);
        self.emit.tag_line();
        self.emit.tag_op(keywords::KEYWORD_FOR, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        self.emit.spaces(1, 0);
        let id1 = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
        self.context.push_mod();
        self.context.set_mod(modifiers::COMMA_SEPARATE);
        // Emit the (optional) initializer statement.
        if let Some(op) = fd.sblocks_ref().block(blk).get_initialize_op() {
            let id3 = self.emit.begin_statement(&MarkupRef::none());
            self.emit_expression_ir(fd, arch, op);
            self.emit.end_statement(id3);
        }
        self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        self.emit.spaces(1, 0);
        // Emit the conditional statement (the condition block, comma-separated).
        self.emit_block(fd, arch, cond_block);
        self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        self.emit.spaces(1, 0);
        // Emit the iterator statement.
        if let Some(op) = fd.sblocks_ref().block(blk).get_iterate_op() {
            let id4 = self.emit.begin_statement(&MarkupRef::none());
            self.emit_expression_ir(fd, arch, op);
            self.emit.end_statement(id4);
        }
        self.context.pop_mod();
        self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id1);
        let indent =
            self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_loop));
        self.context.set_mod(modifiers::NO_BRANCH); // Don't print goto at bottom of clause
        let id2 = self.emit.begin_block(0);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
        self.emit.end_block(id2);
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, indent);
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitBlockWhileDo` (printc.cc:3150): the top-tested loop.
    /// Block 0 is the condition, block 1 the body.  When the loop carries an
    /// `iterateOp` (recorded by the for-loop reroll), it is emitted as a `for`
    /// loop ([`emit_for_loop`]); otherwise the plain `while` form is emitted.
    fn emit_block_while_do(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        if fd.sblocks_ref().block(blk).get_iterate_op().is_some() {
            self.emit_for_loop(fd, arch, blk);
            return;
        }
        // whiledo block NEVER prints the final branch.
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        let cond_block = fd.sblocks_ref().block(blk).get_block(0);
        let indent;
        if fd.sblocks_ref().block(blk).has_overflow_syntax() {
            // while( true ) { conditionbody...; if (branch) break; }
            self.emit.tag_line();
            self.emit.tag_op(keywords::KEYWORD_WHILE, SyntaxHighlight::KeywordColor, &MarkupRef::none());
            let id1 = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
            self.emit.spaces(1, 0);
            self.emit.print(keywords::KEYWORD_TRUE, SyntaxHighlight::ConstColor);
            self.emit.spaces(1, 0);
            self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id1);
            indent = self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_loop));
            self.context.push_mod();
            self.context.set_mod(modifiers::NO_BRANCH);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            self.emit.tag_line();
            self.emit.tag_op(keywords::KEYWORD_IF, SyntaxHighlight::KeywordColor, &MarkupRef::none());
            self.emit.spaces(1, 0);
            self.context.push_mod();
            self.context.set_mod(modifiers::ONLY_BRANCH);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            self.emit.spaces(1, 0);
            self.emit_goto_statement(fd, cond_block, cond_block, crate::block::block_flags::f_break_goto);
        } else {
            // while(condition) {
            self.emit.tag_line();
            self.emit.tag_op(keywords::KEYWORD_WHILE, SyntaxHighlight::KeywordColor, &MarkupRef::none());
            self.emit.spaces(1, 0);
            let id1 = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
            self.context.push_mod();
            self.context.set_mod(modifiers::COMMA_SEPARATE);
            self.emit_block(fd, arch, cond_block);
            self.context.pop_mod();
            self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id1);
            indent = self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_loop));
        }
        self.context.set_mod(modifiers::NO_BRANCH); // don't print goto at bottom of clause
        let id2 = self.emit.begin_block(0);
        self.emit_block(fd, arch, fd.sblocks_ref().block(blk).get_block(1));
        self.emit.end_block(id2);
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, indent);
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitBlockDoWhile` (printc.cc:3217): the bottom-tested loop.
    /// `do { block0-body } while (block0-branch);`.
    fn emit_block_do_while(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        // dowhile block NEVER prints the final branch.
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit.tag_line();
        self.emit.print(keywords::KEYWORD_DO, SyntaxHighlight::KeywordColor);
        let id = self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_loop));
        let body = fd.sblocks_ref().block(blk).get_block(0);
        self.context.push_mod();
        let id2 = self.emit.begin_block(0);
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, body);
        self.emit.end_block(id2);
        self.context.pop_mod();
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, id);
        self.emit.spaces(1, 0);
        self.emit.tag_op(keywords::KEYWORD_WHILE, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        self.emit.spaces(1, 0);
        self.context.set_mod(modifiers::ONLY_BRANCH);
        self.emit_block(fd, arch, body);
        self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitBlockInfLoop` (printc.cc:3246): the infinite loop.
    /// `do { block0-body } while( true );`.
    fn emit_block_inf_loop(&mut self, fd: &Funcdata, arch: &Architecture, blk: BlockId) {
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH);
        self.emit.tag_line();
        self.emit.print(keywords::KEYWORD_DO, SyntaxHighlight::KeywordColor);
        let id = self.emit.open_brace_indent(keywords::OPEN_CURLY, to_emit_brace(self.options.brace_loop));
        let body = fd.sblocks_ref().block(blk).get_block(0);
        let id1 = self.emit.begin_block(0);
        self.emit_block(fd, arch, body);
        self.emit.end_block(id1);
        self.emit.close_brace_indent(keywords::CLOSE_CURLY, id);
        self.emit.spaces(1, 0);
        self.emit.tag_op(keywords::KEYWORD_WHILE, SyntaxHighlight::KeywordColor, &MarkupRef::none());
        let id2 = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
        self.emit.spaces(1, 0);
        self.emit.print(keywords::KEYWORD_TRUE, SyntaxHighlight::ConstColor);
        self.emit.spaces(1, 0);
        self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id2);
        self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        self.context.pop_mod();
    }

    /// C++ `PrintC::emitGotoStatement` (printc.cc:2379): a `goto`/`break`/
    /// `continue` statement for an unstructured branch.  The destination label is
    /// the target block's reverse-post index (`LAB_<index>` — full address-based
    /// label naming is the label/naming layer).
    fn emit_goto_statement(
        &mut self,
        fd: &Funcdata,
        _src: BlockId,
        target: BlockId,
        gototype: uint4,
    ) {
        use crate::block::block_flags;
        let id = self.emit.begin_statement(&MarkupRef::none());
        match gototype {
            x if x == block_flags::f_break_goto => {
                self.emit.print(keywords::KEYWORD_BREAK, SyntaxHighlight::KeywordColor);
            }
            x if x == block_flags::f_continue_goto => {
                self.emit.print(keywords::KEYWORD_CONTINUE, SyntaxHighlight::KeywordColor);
            }
            _ => {
                self.emit.print(keywords::KEYWORD_GOTO, SyntaxHighlight::KeywordColor);
                self.emit.spaces(1, 0);
                let idx = fd.sblocks_ref().block(target).get_index();
                self.emit.print(&format!("LAB_{idx:08x}"), SyntaxHighlight::NoColor);
            }
        }
        self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        self.emit.end_statement(id);
    }

    /// The op-list walk shared by `emitBlockCopy`/`emitBlockBasic` (C++
    /// `PrintC::emitBlockBasic`, printc.cc:2827).  `bblocks` selects which arena
    /// holds the basic block (a `BlockCopy` points into `bblocks`).
    fn emit_basic_block_ops(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        bb: BlockId,
        bblocks: bool,
    ) {
        // only_branch: print only the block's branch instruction (CBRANCH).
        if self.context.is_set(modifiers::ONLY_BRANCH) {
            let last = if bblocks { fd.bb_op_tail(bb) } else { sblocks_basic_tail(fd, bb) };
            if let Some(inst) = last {
                if fd.obank().get(inst).map(|o| o.is_branch()).unwrap_or(false) {
                    self.emit_expression_ir(fd, arch, inst);
                }
            }
            return;
        }
        let mut separator = false;
        let mut cur = if bblocks { fd.bb_op_head(bb) } else { sblocks_basic_head(fd, bb) };
        while let Some(inst) = cur {
            cur = fd.bb_op_next(inst);
            let o = match fd.obank().get(inst) {
                Some(o) => o,
                None => continue,
            };
            if o.not_printed() {
                continue;
            }
            if o.is_branch() {
                if self.context.is_set(modifiers::NO_BRANCH) {
                    continue;
                }
                if o.code() == OpCode::CPUI_BRANCH {
                    continue;
                }
            }
            // Skip ops whose output is an implied varnode (inlined elsewhere).
            if let Some(out) = o.get_out() {
                if fd.vbank().get(out).map(|v| v.is_implied()).unwrap_or(false) {
                    continue;
                }
            }
            if separator {
                // emitCommentGroup(inst); tagLine();
                self.emit.tag_line();
            } else if !self.context.is_set(modifiers::COMMA_SEPARATE) {
                self.emit.tag_line();
            }
            self.emit_statement(fd, arch, inst);
            separator = true;
        }
    }

    /// C++ `PrintC::emitStatement` (printc.cc:2361).
    fn emit_statement(&mut self, fd: &Funcdata, arch: &Architecture, inst: OpId) {
        let id = self.emit.begin_statement(&MarkupRef::none());
        self.emit_expression_ir(fd, arch, inst);
        self.emit.end_statement(id);
        if !self.context.is_set(modifiers::COMMA_SEPARATE) {
            self.emit.print(keywords::SEMICOLON, SyntaxHighlight::NoColor);
        }
    }

    /// C++ `PrintC::emitExpression` (printc.cc:2544): if the op has an output,
    /// open an assignment to it, then push the op's expression and recurse.
    fn emit_expression_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        // C++ special-printing dispatch (printc.cc:2547-2566): a STORE/INSERT
        // marked by the bitfield transforms renders as `ptr->field = value`
        // (the constructor and SUBPIECE special-print arms are other surfaces).
        if fd.obank().get(op).map(|o| o.does_special_printing()).unwrap_or(false) {
            match fd.obank().get(op).map(|o| o.code()) {
                Some(OpCode::CPUI_STORE) => {
                    self.emit_bitfield_store(fd, arch, op);
                    return;
                }
                Some(OpCode::CPUI_INSERT) => {
                    self.emit_bitfield_expression(fd, arch, op);
                    return;
                }
                // CPUI_SUBPIECE: don't modify printing here (printc.cc:2561).
                // The constructor arm and any other special-print op are other
                // surfaces; fall through to the normal render.
                _ => {}
            }
        }
        let outvn = fd.obank().get(op).and_then(|o| o.get_out());
        if let Some(out) = outvn {
            // pushOp(&assignment,op); pushSymbolDetail(outvn,op,false);
            self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
            self.push_vn_explicit_ir(fd, arch, out, op);
        }
        // op->getOpcode()->push(this,op,(PcodeOp *)0)
        self.op_push_ir(fd, arch, op, None);
    }

    /// C++ `op->getOpcode()->push(this,op,readop)` — the per-opcode RPN push
    /// (the `PrintC::op*` overrides, dispatched via [`op_emit_kind`] plus the
    /// hand-written cases the structured boolless body reaches).
    ///
    /// `read_op` is the C++ `readOp` argument threaded by `getOpcode()->push`:
    /// the op that *reads* `op`'s output when `op` is being pushed as an implied
    /// value (`pushVnImplied`/`pushImpliedField` pass the reader; printc.cc:2186),
    /// else `None` at the top of an expression (printc.cc:2579 passes `(PcodeOp *)0`).
    /// Only `opIntSext`/`opIntZext` consume it (the extension-cast-implied test,
    /// printc.cc:806-830); every other override ignores it.
    fn op_push_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId, read_op: Option<OpId>) {
        let opc = fd.obank().get(op).expect("op_push_ir: stale op").code();
        match opc {
            // INT_SEXT (printc.cc:819 opIntSext) / INT_ZEXT (printc.cc:806 opIntZext):
            // the cast-strategy decides whether the extension renders as an explicit
            // `(intN)`/`(uintN)` cast, is hidden (implied by integer promotion), or
            // falls back to the functional `SEXT(x)`/`ZEXT(x)` form.
            OpCode::CPUI_INT_SEXT => self.op_int_sext_ir(fd, arch, op, read_op),
            OpCode::CPUI_INT_ZEXT => self.op_int_zext_ir(fd, arch, op, read_op),
            // CBRANCH: the structured-if condition (printc.cc:556 opCbranch).
            // In the non-flat path opCbranch only emits the `( condition )`; the
            // `if` keyword is printed by emit_block_if.  yesparen = !comma_separate.
            OpCode::CPUI_CBRANCH => {
                let yesparen = !self.context.is_set(modifiers::COMMA_SEPARATE);
                let booleanflip = fd.obank().get(op).map(|o| o.is_boolean_flip()).unwrap_or(false);
                let in1 = fd.obank().get(op).and_then(|o| o.get_in(1));
                let id = if yesparen {
                    self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0)
                } else {
                    self.emit.open_group()
                };
                if booleanflip {
                    self.push_op(&tokens::BOOLEAN_NOT, Some(op_key(op)));
                }
                if let Some(vn) = in1 {
                    self.push_vn_ir(fd, arch, vn, op);
                }
                // recurse() drains the stack: direct resolution above already
                // drained it (the RPN engine unwinds on the final push_atom), so
                // the paren can close now.
                if yesparen {
                    self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id);
                } else {
                    self.emit.close_group(id);
                }
            }
            // BRANCHIND (printc.cc:602 opBranchind): the switch header `switch(v)`.
            // The structured switch body (`{ case N: ... }`) is emitted by
            // `emit_block_switch`; here only the `switch(in0)` expression prints.
            OpCode::CPUI_BRANCHIND => {
                self.emit.tag_op(keywords::KEYWORD_SWITCH, SyntaxHighlight::KeywordColor, &MarkupRef::none());
                let id = self.emit.open_paren(crate::printlanguage::OPEN_PAREN, 0);
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                    self.push_vn_ir(fd, arch, vn, op);
                }
                self.emit.close_paren(crate::printlanguage::CLOSE_PAREN, id);
            }
            // RETURN (printc.cc:774 opReturn, the plain-return case).
            OpCode::CPUI_RETURN => {
                self.emit.tag_op(keywords::KEYWORD_RETURN, SyntaxHighlight::KeywordColor, &MarkupRef::none());
                let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                if nin > 1 {
                    self.emit.spaces(1, 0);
                    if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(1)) {
                        self.push_vn_ir(fd, arch, vn, op);
                    }
                }
            }
            // COPY (printc.cc:501 opCopy): just push the input.
            OpCode::CPUI_COPY => {
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                    self.push_vn_ir(fd, arch, vn, op);
                }
            }
            // LOAD (printc.cc:507 opLoad) / STORE (printc.cc:520 opStore).
            OpCode::CPUI_LOAD => self.op_load_ir(fd, arch, op),
            OpCode::CPUI_STORE => self.op_store_ir(fd, arch, op),
            // ZPULL (printc.cc:1294 opZpullOp) / SPULL (printc.cc:1320 opSpullOp):
            // a bitfield read.  Both render `ptr->field` / `symbol.field` via the
            // shared `op_pull_ir`, falling back to `ZPULL(...)`/`SPULL(...)` when
            // the structure/bitfield can't be recovered.
            OpCode::CPUI_ZPULL | OpCode::CPUI_SPULL => self.op_pull_ir(fd, arch, op),
            // BOOL_NEGATE (printc.cc:834 opBoolNegate): the `!x` unary, with the
            // double-negation cancellation (`negatetoken`) and the
            // flip-the-next-operator optimization (`checkPrintNegation`).
            OpCode::CPUI_BOOL_NEGATE => self.op_bool_negate_ir(fd, arch, op),
            // SUBPIECE (printc.cc:863 opSubpiece): a field-extraction special-print
            // (`symbol.field`) or the cast/functional dispatch.
            OpCode::CPUI_SUBPIECE => self.op_subpiece_ir(fd, arch, op),
            // PTRADD (printc.cc:900 opPtradd) / PTRSUB (printc.cc:953 opPtrsub).
            OpCode::CPUI_PTRADD => self.op_ptradd_ir(fd, arch, op),
            OpCode::CPUI_PTRSUB => self.op_ptrsub_ir(fd, arch, op),
            // CALL / CALLIND (printc.cc:613 opCall / 657 opCallind): the functional
            // `callee(arg1, arg2, ...)` form over the recovered call inputs.
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                self.op_call_ir(fd, arch, op);
            }
            // CALLOTHER (printc.cc:693 opCallother): a user p-code op.  The display
            // class (`userop->getDisplay()`) chooses the form: functional
            // `name(arg,...)` for a black-box op, `display_string` for the
            // internal-string builtin, or the no-operator/annotation forms.
            OpCode::CPUI_CALLOTHER => self.op_callother_ir(fd, arch, op),
            // FLOAT_INT2FLOAT (printc.cc:850 opFloatInt2Float): the int->float
            // conversion renders as a `(floatN)input` cast (NOT a functional
            // `FLOAT_INT2FLOAT(input)`), absorbing an implied INT_ZEXT on its
            // input so the widened source prints once.
            OpCode::CPUI_FLOAT_INT2FLOAT => self.op_float_int2float_ir(fd, arch, op),
            // MULTIEQUAL / INDIRECT: no-op (printc.hh:337-338 opMultiequal/
            // opIndirect) — copy markers, never printed as an operator.  The
            // phi's value is whatever its (single, post-merge) instance reads.
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
                // Push in0 so the assignment has a RHS (degenerate phi rendering;
                // faithful multi-instance phi rendering is the merge layer).
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                    self.push_vn_ir(fd, arch, vn, op);
                }
            }
            _ => {
                // Table-driven binary / unary / functional forms.
                match op_emit_kind(opc) {
                    OpEmitKind::Binary(tok) => self.op_binary_ir(fd, arch, tok, op),
                    OpEmitKind::Unary(tok) => self.op_unary_ir(fd, arch, tok, op),
                    // opTypeCast (printc.cc:468): the C cast-notation `(type)operand`
                    // form.  CPUI_CAST / CPUI_FLOAT_FLOAT2FLOAT / CPUI_FLOAT_TRUNC
                    // all reduce to opTypeCast (printc.hh:332-341) — they render as
                    // a parenthesized type cast, not a functional `OPC(args)`.
                    OpEmitKind::TypeCast => self.op_type_cast_ir(fd, arch, op),
                    OpEmitKind::Func | OpEmitKind::Custom => {
                        // opFunc / hand-written: the functional `OPC(args)` form.
                        // (The userop name resolution for true user p-code ops is
                        // a separate layer.)
                        self.op_func_ir(fd, arch, op);
                    }
                }
            }
        }
    }

    /// C++ `PrintLanguage::opBinary` over the IR (printlanguage.cc:553).  Pushes
    /// the operator then resolves both operand Varnodes.  The negate-token flip
    /// (the `negatetoken` mod) is honoured.
    fn op_binary_ir(&mut self, fd: &Funcdata, arch: &Architecture, tok: &'static OpToken, op: OpId) {
        let tok = if self.context.is_set(modifiers::NEGATETOKEN) {
            self.context.unset_mod(modifiers::NEGATETOKEN);
            token_negate(tok).unwrap_or(tok)
        } else {
            tok
        };
        self.push_op(tok, Some(op_key(op)));
        // C++ pushes in1 then in0 onto the LIFO nodepend; resolving directly,
        // push in0 then in1 so the operands print in0 <op> in1.
        if let Some(v0) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
            self.push_vn_ir(fd, arch, v0, op);
        }
        if let Some(v1) = fd.obank().get(op).and_then(|o| o.get_in(1)) {
            self.push_vn_ir(fd, arch, v1, op);
        }
    }

    /// C++ `PrintLanguage::opUnary` over the IR (printlanguage.cc:573).
    fn op_unary_ir(&mut self, fd: &Funcdata, arch: &Architecture, tok: &'static OpToken, op: OpId) {
        self.push_op(tok, Some(op_key(op)));
        if let Some(v0) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
            self.push_vn_ir(fd, arch, v0, op);
        }
    }

    /// C++ `PrintC::checkPrintNegation` (printc.cc:2464): can the value `vn` be
    /// rendered with its *next* operator flipped (so the `!` is absorbed into a
    /// comparison) instead of emitting an explicit `!`?  True when `vn` is an
    /// implied, written value whose defining op-code has a boolean-flip complement
    /// (`get_booleanflip` != `CPUI_MAX`).
    fn check_print_negation(&self, fd: &Funcdata, vn: VarnodeId) -> bool {
        let v = match fd.vbank().get(vn) {
            Some(v) => v,
            None => return false,
        };
        if !v.is_implied() {
            return false;
        }
        if !v.is_written() {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        let code = match fd.obank().get(def) {
            Some(o) => o.code(),
            None => return false,
        };
        let mut reorder = false;
        kuna_num::opcodes::get_booleanflip(code, &mut reorder) != OpCode::CPUI_MAX
    }

    /// C++ `PrintC::opBoolNegate` (printc.cc:834): print the `!x` boolean negate,
    /// but check for opportunities to flip the next operator instead.
    ///   - If we are negated by a previous BOOL_NEGATE (`negatetoken` is set),
    ///     consume that mod and print our input unmodified (double negation cancels).
    ///   - Else if the input's next operator can be flipped, don't print `!`; print
    ///     the input with `negatetoken` set so its comparison renders its complement.
    ///   - Otherwise print `!` followed by our input.
    fn op_bool_negate_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
        if self.context.is_set(modifiers::NEGATETOKEN) {
            // Negated by a previous BOOL_NEGATE: consume the mod, print input as-is.
            self.context.unset_mod(modifiers::NEGATETOKEN);
            if let Some(vn) = in0 {
                self.push_vn_ir(fd, arch, vn, op);
            }
        } else if in0.map(|vn| self.check_print_negation(fd, vn)).unwrap_or(false) {
            // The next operator can be flipped: print the input with `negatetoken`
            // active (C++ `pushVn(in0, op, mods|negatetoken)`).
            self.context.push_mod();
            self.context.set_mod(modifiers::NEGATETOKEN);
            if let Some(vn) = in0 {
                self.push_vn_ir(fd, arch, vn, op);
            }
            self.context.pop_mod();
        } else {
            // Otherwise print ourselves: `!` then the input.
            self.push_op(&tokens::BOOLEAN_NOT, Some(op_key(op)));
            if let Some(vn) = in0 {
                self.push_vn_ir(fd, arch, vn, op);
            }
        }
    }

    /// C++ `PrintC::opFunc` (printc.cc:444) — a functional `name(arg0,arg1,...)`
    /// form.  Pushes `function_call`, the (un-highlighted) operator name, an
    /// `(numInput-1)`-deep comma chain, then the operands.  The function name is
    /// the opcode's operator name (the full type/userop name resolution is the
    /// next layer).
    fn op_func_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let opc = fd.obank().get(op).expect("op_func_ir: stale op").code();
        let name = opcode_print_name(opc);
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        // The name is pushed as an *operator* token (C++ `optoken`, no_color).
        self.push_atom(&Atom::with_op(
            name,
            TagType::OpToken,
            crate::printlanguage::SyntaxHighlight::no_color,
            op_key(op),
        ));
        let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
        if nin > 0 {
            // (numInput-1) comma operators glue the argument list.
            for _ in 0..(nin - 1) {
                self.push_op(&tokens::COMMA, Some(op_key(op)));
            }
            // C++ pushes args in reverse onto the LIFO queue; resolving directly
            // (the comma chain nests right), push in forward order.
            for i in 0..nin {
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(i)) {
                    self.push_vn_ir(fd, arch, vn, op);
                }
            }
        } else {
            // Empty token for void (C++ blanktoken).
            self.push_atom(&Atom::syntax(
                "",
                TagType::BlankToken,
                crate::printlanguage::SyntaxHighlight::no_color,
            ));
        }
    }

    /// C++ `PrintC::opCallother` (printc.cc:693): render a CALLOTHER (user
    /// p-code op).  The op's in0 constant indexes a `UserPcodeOp` whose
    /// `getDisplay()` selects the form:
    ///   * `0` (functional): `name(arg1, arg2, ...)` over inputs 1..n-1, with the
    ///     name resolved through the userop table (`getOperatorName`).
    ///   * `annotation_assignment`: `in1 = in2`.
    ///   * `no_operator`: just `in1`.
    ///   * `display_string`: the output Varnode rendered as a quoted string
    ///     literal (the internal-string builtin), via `printCharacterConstant` on
    ///     the hash-keyed constant address in in1.
    fn op_callother_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        use crate::userop::userop_flags;
        let in0_off = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => fd.vbank().get(v).map(|vn| vn.get_offset()).unwrap_or(0),
            None => 0,
        };
        // UserPcodeOp *userop = glb->userops.getOp(op->getIn(0)->getOffset());
        let display = arch
            .userops
            .get_op(in0_off as u32)
            .map(|u| u.get_display())
            .unwrap_or(0);
        let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
        if display == 0 {
            // Functional syntax: string nm = op->getOpcode()->getOperatorName(op);
            // For CALLOTHER this resolves to the userop's name (the base
            // getOperatorName), or the generic `CALLOTHER[index]` fallback.
            let nm = match arch.userops.get_op(in0_off as u32) {
                Some(u) => String::from_utf8_lossy(u.get_name()).into_owned(),
                None => format!("CALLOTHER[{:#x}]", in0_off),
            };
            self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
            self.push_atom(&Atom::with_op(
                nm,
                TagType::OpToken,
                crate::printlanguage::SyntaxHighlight::funcname_color,
                op_key(op),
            ));
            if nin > 1 {
                // (numInput-2) comma operators glue args 1..numInput-1.
                for _ in 1..(nin - 1) {
                    self.push_op(&tokens::COMMA, Some(op_key(op)));
                }
                for i in 1..nin {
                    if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(i)) {
                        self.push_vn_ir(fd, arch, vn, op);
                    }
                }
            } else {
                // Empty token for void (C++ blanktoken).
                self.push_atom(&Atom::syntax(
                    "",
                    TagType::BlankToken,
                    crate::printlanguage::SyntaxHighlight::no_color,
                ));
            }
        } else if display == userop_flags::ANNOTATION_ASSIGNMENT {
            self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
            if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(2)) {
                self.push_vn_ir(fd, arch, vn, op);
            }
            if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(1)) {
                self.push_vn_ir(fd, arch, vn, op);
            }
        } else if display == userop_flags::NO_OPERATOR {
            if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(1)) {
                self.push_vn_ir(fd, arch, vn, op);
            }
        } else if display == userop_flags::DISPLAY_STRING {
            // const Varnode *vn = op->getOut(); Datatype *ct = vn->getType();
            let outvn = fd.obank().get(op).and_then(|o| o.get_out());
            let mut s = String::new();
            let mut ok = false;
            if let Some(ovn) = outvn {
                let ct = fd.vbank().get(ovn).map(|v| std::rc::Rc::clone(v.get_type()));
                if let Some(ct) = ct {
                    if ct.get_metatype() == crate::dtype::type_metatype::TYPE_PTR {
                        if let Some(subct) = ct.get_ptr_to() {
                            // printCharacterConstant(str, op->getIn(1)->getAddr(), subct)
                            let in1addr = fd
                                .obank()
                                .get(op)
                                .and_then(|o| o.get_in(1))
                                .and_then(|v| fd.vbank().get(v).map(|vn| vn.get_addr().clone()));
                            if let Some(addr) = in1addr {
                                if self.print_character_constant(arch, &mut s, &addr, &subct) {
                                    ok = true;
                                }
                            }
                        }
                    }
                }
            }
            if !ok {
                s.push_str("\"badstring\"");
            }
            // pushAtom(Atom(str.str(), vartoken, const_color, op, vn))
            if let Some(ovn) = outvn {
                self.push_atom(&Atom::with_op_vn(
                    s,
                    TagType::VarToken,
                    crate::printlanguage::SyntaxHighlight::const_color,
                    op_key(op),
                    vn_key(ovn),
                ));
            } else {
                self.push_atom(&Atom::with_op(
                    s,
                    TagType::VarToken,
                    crate::printlanguage::SyntaxHighlight::const_color,
                    op_key(op),
                ));
            }
        }
    }

    /// C++ `PrintC::opFloatInt2Float` (printc.cc:850): the integer→float
    /// conversion prints as a `(floatN)input` type-cast.  The input is the
    /// op's in0, unless that input is an implied `INT_ZEXT` (the C++
    /// `TypeOpFloatInt2Float::absorbZext`), in which case the ZEXT is absorbed
    /// and its source is the input — the zero-extension to the conversion's
    /// source width is implicit in the cast.  The cast's type is the output
    /// varnode's def-facing high type (`getOut()->getHighTypeDefFacing()`).
    /// With `option_nocasts` set the cast is suppressed and only the input
    /// prints.
    fn op_float_int2float_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        // const PcodeOp *zextOp = TypeOpFloatInt2Float::absorbZext(op);
        // const Varnode *vn0 = zextOp ? zextOp->getIn(0) : op->getIn(0);
        let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
        let vn0 = absorb_zext(fd, op)
            .and_then(|zext| fd.obank().get(zext).and_then(|o| o.get_in(0)))
            .or(in0);
        if !self.options.nocasts {
            // pushOp(&typecast,op); pushType(out->getHighTypeDefFacing()).
            self.push_op(&tokens::TYPECAST, Some(op_key(op)));
            let outvn = fd.obank().get(op).and_then(|o| o.get_out());
            if let Some(out) = outvn {
                if let Some(v) = fd.vbank().get(out) {
                    self.push_cast_type(v.get_type_def_facing());
                }
            }
        }
        // pushVn(vn0,op,mods).
        if let Some(vn) = vn0 {
            self.push_vn_ir(fd, arch, vn, op);
        }
    }

    /// C++ `PrintC::opTypeCast` (printc.cc:468): the C cast-notation `(type)operand`
    /// form shared by `opCast` / `opFloatFloat2Float` / `opFloatTrunc`
    /// (printc.hh:332-341, all `{ opTypeCast(op); }`).  The cast's target type is
    /// the op's **output** varnode's def-facing high type
    /// (`op->getOut()->getHighTypeDefFacing()`) — never a hardcoded or opcode-keyed
    /// type — and the operand is in0:
    ///
    /// ```text
    ///   Datatype *dt = op->getOut()->getHighTypeDefFacing();
    ///   if (dt->isPointerToArray()) {
    ///     if (checkAddressOfCast(op)) { pushOp(&addressof,op); pushVn(in0); return; }
    ///   }
    ///   if (!option_nocasts) { pushOp(&typecast,op); pushType(dt); }
    ///   pushVn(op->getIn(0),op,mods);
    /// ```
    ///
    /// With `option_nocasts` the cast is suppressed and only the operand prints
    /// (the underlying value flows through, parenthesized by precedence).
    ///
    /// The `isPointerToArray()` / [`check_address_of_cast`](Self::check_address_of_cast)
    /// arm renders a pointer-to-array cast as an address-of `&sym` (dropping the
    /// spurious `(T(*)[n])` cast) when the input is the address of an array Symbol of
    /// the matching size.  It never fires for the scalar `CPUI_CAST` /
    /// float-conversion casts this routes (whose output is a scalar `floatN`/`intN`,
    /// not a pointer-to-array).
    fn op_type_cast_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        // C++ `opTypeCast` (printc.cc:468-484): when the target type is a
        // pointer-to-array, a CAST that is really an address-of an array Symbol
        // renders as `&sym` (dropping the spurious `(T(*)[n])` cast) instead of the
        // C cast form.  `checkAddressOfCast` decides this purely from the in/out
        // high types and the input's symbol/PTRSUB geometry — never opcode- or
        // name-keyed.
        let out_def = fd
            .obank()
            .get(op)
            .and_then(|o| o.get_out())
            .and_then(|out| fd.vbank().get(out))
            .map(|v| v.get_type_def_facing().clone());
        if out_def.as_ref().map(|t| t.is_pointer_to_array()).unwrap_or(false)
            && self.check_address_of_cast(fd, op)
        {
            // pushOp(&addressof,op); pushVn(op->getIn(0),op,mods);
            self.push_op(&tokens::ADDRESSOF, Some(op_key(op)));
            if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                self.push_vn_ir(fd, arch, vn, op);
            }
            return;
        }
        if !self.options.nocasts {
            // pushOp(&typecast,op); pushType(op->getOut()->getHighTypeDefFacing()).
            self.push_op(&tokens::TYPECAST, Some(op_key(op)));
            let outvn = fd.obank().get(op).and_then(|o| o.get_out());
            if let Some(out) = outvn {
                if let Some(v) = fd.vbank().get(out) {
                    self.push_cast_type(v.get_type_def_facing());
                }
            }
        }
        // pushVn(op->getIn(0),op,mods).
        if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
            self.push_vn_ir(fd, arch, vn, op);
        }
    }

    /// C++ `PrintC::checkAddressOfCast` (printc.cc:396-438): check that the output
    /// data-type is a pointer to an array and the input data-type is a pointer to
    /// the element type, and that the input variable represents a Symbol with an
    /// array data-type of the same total size.  When this holds the CAST is the
    /// implicit array-to-pointer decay of taking `&sym`, so the cast is dropped in
    /// favor of `&sym`.  Returns `true` if the CAST can be rendered as `&`.
    ///
    /// ```text
    ///   Datatype *dt0 = op->getOut()->getHighTypeDefFacing();
    ///   const Varnode *vnin = op->getIn(0);
    ///   Datatype *dt1 = vnin->getHighTypeReadFacing(op);
    ///   if (dt0->getMetatype()!=TYPE_PTR || dt1->getMetatype()!=TYPE_PTR) return false;
    ///   const Datatype *base0 = ((TypePointer*)dt0)->getPtrTo();
    ///   const Datatype *base1 = ((TypePointer*)dt1)->getPtrTo();
    ///   if (base0->getMetatype()!=TYPE_ARRAY) return false;
    ///   int4 arraySize = base0->getSize();
    ///   base0 = ((TypeArray*)base0)->getBase();
    ///   while(base0->getTypedef()) base0 = base0->getTypedef();
    ///   while(base1->getTypedef()) base1 = base1->getTypedef();
    ///   if (base0 != base1) return false;
    ///   Datatype *symbolType = 0;
    ///   if (vnin->getSymbolEntry() && vnin->getHigh()->getSymbolOffset()==-1)
    ///     symbolType = vnin->getSymbolEntry()->getSymbol()->getType();
    ///   else if (vnin->isWritten()) {
    ///     const PcodeOp *ptrsub = vnin->getDef();
    ///     if (ptrsub->code()==CPUI_PTRSUB) {
    ///       Datatype *rootType = ptrsub->getIn(0)->getHighTypeReadFacing(ptrsub);
    ///       if (rootType->getMetatype()==TYPE_PTR) {
    ///         rootType = ((TypePointer*)rootType)->getPtrTo();
    ///         int8 off = ptrsub->getIn(1)->getOffset();
    ///         symbolType = rootType->getSubType(off,&off);
    ///         if (off != 0) return false;
    ///       }
    ///     }
    ///   }
    ///   if (symbolType==0) return false;
    ///   if (symbolType->getMetatype()!=TYPE_ARRAY || symbolType->getSize()!=arraySize)
    ///     return false;
    ///   return true;
    /// ```
    fn check_address_of_cast(&self, fd: &Funcdata, op: OpId) -> bool {
        use crate::dtype::type_metatype;
        // dt0 = op->getOut()->getHighTypeDefFacing();
        let dt0 = match fd
            .obank()
            .get(op)
            .and_then(|o| o.get_out())
            .and_then(|out| fd.vbank().get(out))
            .map(|v| v.get_type_def_facing().clone())
        {
            Some(t) => t,
            None => return false,
        };
        // const Varnode *vnin = op->getIn(0); dt1 = vnin->getHighTypeReadFacing(op);
        let vnin = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return false,
        };
        let dt1 = match fd.vbank().get(vnin).map(|v| v.get_type_read_facing(op).clone()) {
            Some(t) => t,
            None => return false,
        };
        // if (dt0->getMetatype()!=TYPE_PTR || dt1->getMetatype()!=TYPE_PTR) return false;
        if dt0.get_metatype() != type_metatype::TYPE_PTR
            || dt1.get_metatype() != type_metatype::TYPE_PTR
        {
            return false;
        }
        // base0 = dt0->getPtrTo(); base1 = dt1->getPtrTo();
        let base0 = match dt0.get_ptr_to() {
            Some(b) => b,
            None => return false,
        };
        let mut base1 = match dt1.get_ptr_to() {
            Some(b) => b,
            None => return false,
        };
        // if (base0->getMetatype()!=TYPE_ARRAY) return false;
        if base0.get_metatype() != type_metatype::TYPE_ARRAY {
            return false;
        }
        // int4 arraySize = base0->getSize(); base0 = ((TypeArray*)base0)->getBase();
        let array_size = base0.get_size();
        let mut base0 = match base0.get_array_base() {
            Some(b) => b,
            None => return false,
        };
        // while(base0->getTypedef()) base0 = base0->getTypedef();
        while let Some(t) = base0.get_typedef().cloned() {
            base0 = t;
        }
        // while(base1->getTypedef()) base1 = base1->getTypedef();
        while let Some(t) = base1.get_typedef().cloned() {
            base1 = t;
        }
        // if (base0 != base1) return false;
        // C++ tests Datatype *pointer* identity; the kuna factory interns every
        // data-type to a unique allocation, so `Rc::ptr_eq` is the faithful identity
        // check.  As a structural fallback (the element types here are scalars whose
        // `compare` is implemented) a `compare == 0` also counts as equal; a compare
        // SEAM (`Err`) is treated as not-equal (conservative: never collapses a cast
        // it cannot prove redundant).
        let base_eq = std::rc::Rc::ptr_eq(&base0, &base1)
            || matches!(base0.compare(&base1, 10), Ok(0));
        if !base_eq {
            return false;
        }
        // Datatype *symbolType = 0;
        // if (vnin->getSymbolEntry() && vnin->getHigh()->getSymbolOffset()==-1)
        //   symbolType = vnin->getSymbolEntry()->getSymbol()->getType();
        // The kuna `getSymbolEntry()` stand-in is the high's bound Symbol — a
        // `kuna_name` with the mapped `kuna_symbol_type`; `getSymbolOffset()==-1`
        // is the whole-symbol match.
        let mut symbol_type: Option<std::rc::Rc<crate::dtype::Datatype>> = None;
        let vnin_high = fd.vbank().get(vnin).and_then(|v| v.get_high());
        let whole_sym = vnin_high.and_then(|h| fd.high_bank().get(h)).and_then(|h| {
            if h.kuna_symbol_offset() == -1 {
                h.kuna_symbol_type().cloned()
            } else {
                None
            }
        });
        if let Some(st) = whole_sym {
            symbol_type = Some(st);
        } else if fd.vbank().get(vnin).map(|v| v.is_written()).unwrap_or(false) {
            // else if (vnin->isWritten()) { ptrsub = vnin->getDef(); ... }
            let ptrsub = fd.vbank().get(vnin).and_then(|v| v.get_def());
            if let Some(ptrsub) = ptrsub {
                if fd.obank().get(ptrsub).map(|o| o.code()) == Some(OpCode::CPUI_PTRSUB) {
                    // rootType = ptrsub->getIn(0)->getHighTypeReadFacing(ptrsub);
                    let root_in0 = fd.obank().get(ptrsub).and_then(|o| o.get_in(0));
                    let root_type = root_in0
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.get_type_read_facing(ptrsub).clone());
                    if let Some(root_type) = root_type {
                        // if (rootType->getMetatype()==TYPE_PTR) {
                        if root_type.get_metatype() == type_metatype::TYPE_PTR {
                            // rootType = ((TypePointer*)rootType)->getPtrTo();
                            if let Some(root_ptr_to) = root_type.get_ptr_to() {
                                // int8 off = ptrsub->getIn(1)->getOffset();
                                let off = fd
                                    .obank()
                                    .get(ptrsub)
                                    .and_then(|o| o.get_in(1))
                                    .and_then(|v| fd.vbank().get(v))
                                    .map(|v| v.get_offset())
                                    .unwrap_or(0) as int8;
                                // symbolType = rootType->getSubType(off,&off);
                                // if (off != 0) return false;
                                // The virtual `getSubType` is `TypeSpacebase::getSubType`
                                // (type.cc:3411) for a spacebase root — it indexes the
                                // symbol-table Scope, which the bare `Datatype::get_sub_type`
                                // cannot reach (it routes to a `SEAM(W6)` Err).  Route a
                                // spacebase through `Funcdata::spacebase_get_sub_type` (the
                                // ported `TypeSpacebase::getSubType`, funcdata_spacebase.rs),
                                // exactly as the spacebase-PTRSUB cast wave does; every other
                                // root keeps the pure `Datatype::get_sub_type`.
                                let resolved: Option<(std::rc::Rc<crate::dtype::Datatype>, int8)> =
                                    if root_ptr_to.get_metatype()
                                        == type_metatype::TYPE_SPACEBASE
                                    {
                                        fd.spacebase_get_sub_type(&root_ptr_to, off)
                                    } else {
                                        match root_ptr_to.get_sub_type(off) {
                                            Ok((sub, newoff)) => sub.map(|s| (s, newoff)),
                                            Err(_) => return false,
                                        }
                                    };
                                match resolved {
                                    Some((sub, newoff)) => {
                                        if newoff != 0 {
                                            return false;
                                        }
                                        symbol_type = Some(sub);
                                    }
                                    None => return false,
                                }
                            }
                        }
                    }
                }
            }
        }
        // if (symbolType==0) return false;
        let symbol_type = match symbol_type {
            Some(s) => s,
            None => return false,
        };
        // if (symbolType->getMetatype()!=TYPE_ARRAY || symbolType->getSize()!=arraySize)
        //   return false;
        if symbol_type.get_metatype() != type_metatype::TYPE_ARRAY
            || symbol_type.get_size() != array_size
        {
            return false;
        }
        true
    }

    /// C++ `PrintC::opHiddenFunc` (printc.cc:494): the syntax represents `op`
    /// with a hidden (un-printed) one-input function — the input expression is
    /// printed without adornment, the [`tokens::HIDDEN`] token only guarding
    /// evaluation order.  Used by `opIntSext`/`opIntZext` to suppress an
    /// extension that is implied by integer promotion.
    fn op_hidden_func_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        // pushOp(&hidden,op); pushVn(op->getIn(0),op,mods).
        self.push_op(&tokens::HIDDEN, Some(op_key(op)));
        if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
            self.push_vn_ir(fd, arch, vn, op);
        }
    }

    /// C++ `PrintC::opIntZext` (printc.cc:806): a zero-extension renders as an
    /// explicit `(uintN)`/`(intN)` cast when the cast strategy says the ZEXT is a
    /// cast (`isZextCast`), is hidden (`opHiddenFunc`) when the extension is
    /// implied by integer promotion in the surrounding expression
    /// (`option_hide_exts && isExtensionCastImplied`), and otherwise falls back to
    /// the functional `ZEXT(x)` form (`opFunc`).
    ///
    /// ```text
    ///   if (castStrategy->isZextCast(out->getHighTypeDefFacing(),
    ///                                in0->getHighTypeReadFacing(op))) {
    ///     if (option_hide_exts && castStrategy->isExtensionCastImplied(op,readOp))
    ///       opHiddenFunc(op);
    ///     else
    ///       opTypeCast(op);
    ///   } else
    ///     opFunc(op);
    /// ```
    fn op_int_zext_ir(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
        read_op: Option<OpId>,
    ) {
        let strat = match cast_strategy_for(arch) {
            Some(s) => s,
            // No type factory bound: degrade to the functional form, exactly as
            // the pre-cast-routing dispatch did.
            None => return self.op_func_ir(fd, arch, op),
        };
        let (outtype, intype) = match self.sext_zext_facing_types(fd, op) {
            Some(t) => t,
            None => return self.op_func_ir(fd, arch, op),
        };
        if strat.is_zext_cast(&outtype, &intype) {
            if self.options.hide_exts && self.is_extension_cast_implied(fd, &strat, op, read_op) {
                self.op_hidden_func_ir(fd, arch, op);
            } else {
                self.op_type_cast_ir(fd, arch, op);
            }
        } else {
            self.op_func_ir(fd, arch, op);
        }
    }

    /// C++ `PrintC::opIntSext` (printc.cc:819): the sign-extension analogue of
    /// [`op_int_zext_ir`] — renders as an explicit `(intN)`/`(uintN)` cast
    /// (`isSextCast`), is hidden when implied, or falls back to `SEXT(x)`.
    fn op_int_sext_ir(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
        read_op: Option<OpId>,
    ) {
        let strat = match cast_strategy_for(arch) {
            Some(s) => s,
            None => return self.op_func_ir(fd, arch, op),
        };
        let (outtype, intype) = match self.sext_zext_facing_types(fd, op) {
            Some(t) => t,
            None => return self.op_func_ir(fd, arch, op),
        };
        if strat.is_sext_cast(&outtype, &intype) {
            if self.options.hide_exts && self.is_extension_cast_implied(fd, &strat, op, read_op) {
                self.op_hidden_func_ir(fd, arch, op);
            } else {
                self.op_type_cast_ir(fd, arch, op);
            }
        } else {
            self.op_func_ir(fd, arch, op);
        }
    }

    /// C++ `PrintC::opSubpiece` (printc.cc:863-898).  A SUBPIECE marked for
    /// special printing (`doesSpecialPrinting`, set by `RuleSubRight` when the
    /// truncated input is a struct/union/array) extracts a composite member; it
    /// renders `symbol.field` via [`push_partial_symbol_ir`] (the symbol-mapped
    /// case, printc.cc:872-881) or `expr.field` via a struct `findTruncation`
    /// (printc.cc:882-888).  A non-special SUBPIECE falls to the cast/functional
    /// dispatch (the existing `is_subpiece_cast` → `opTypeCast` / `opFunc`).
    fn op_subpiece_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        use crate::dtype::type_metatype;
        if fd.obank().get(op).map(|o| o.does_special_printing()).unwrap_or(false) {
            // const Varnode *vn = op->getIn(0);
            let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
            if let Some(vn) = in0 {
                // Datatype *ct = vn->getHighTypeReadFacing(op);  (the bare-Varnode
                // read-facing type, the printc convention).
                let ct = fd.vbank().get(vn).map(|v| v.get_type_read_facing(op).clone());
                if let Some(ct) = ct {
                    if ct.is_piece_structured() {
                        // int8 byteOff = TypeOpSubpiece::computeByteOffsetForComposite(op);
                        let byte_off = subpiece_byte_offset_for_composite(fd, op);
                        let out_sz = fd
                            .obank()
                            .get(op)
                            .and_then(|o| o.get_out())
                            .and_then(|v| fd.vbank().get(v))
                            .map(|v| v.get_size())
                            .unwrap_or(0);
                        // Symbol *sym = vn->getHigh()->getSymbol();  (kuna: the
                        // kuna_name binding stands in for getSymbol()).
                        // if (sym != 0 && vn->isExplicit()) pushPartialSymbol(...).
                        let high = fd.vbank().get(vn).and_then(|v| v.get_high());
                        let is_explicit =
                            fd.vbank().get(vn).map(|v| v.is_explicit()).unwrap_or(false);
                        let sym = high.and_then(|h| fd.high_bank().get(h)).and_then(|h| {
                            h.kuna_name().map(|n| {
                                (n.to_string(), h.kuna_symbol_offset(), h.kuna_symbol_type().cloned())
                            })
                        });
                        if let (Some((name, sym_off, Some(sym_type))), true) = (sym, is_explicit) {
                            // int4 suboff = vn->getHigh()->getSymbolOffset();
                            // if (suboff > 0) byteOff += suboff;
                            let mut boff = byte_off;
                            if sym_off > 0 {
                                boff += sym_off as int8;
                            }
                            // int4 slot = ct->needsResolution() ? 1 : 0;
                            let slot =
                                if sym_type.needs_resolution() { 1 } else { 0 };
                            let smt = sym_type.get_metatype();
                            if (smt == type_metatype::TYPE_STRUCT
                                || smt == type_metatype::TYPE_UNION)
                                && self.push_partial_symbol_ir(
                                    fd,
                                    arch,
                                    &name,
                                    std::rc::Rc::clone(&sym_type),
                                    boff,
                                    out_sz,
                                    vn,
                                    op,
                                    slot,
                                    true,
                                )
                            {
                                return;
                            }
                            // Fall through to the cast/functional dispatch below.
                        } else {
                            // const TypeField *field =
                            //   ct->findTruncation(byteOff,outSize,op,1,offset);
                            // if (field != 0 && offset == 0) { object_member }
                            if ct.get_metatype() == type_metatype::TYPE_STRUCT {
                                if let Ok(Some((idx, off2))) =
                                    ct.find_truncation(byte_off, out_sz, op, 1)
                                {
                                    if off2 == 0 {
                                        if let Some(f) = ct.get_field(idx) {
                                            let fname = f.name.clone();
                                            let fident = f.ident;
                                            self.push_op(
                                                &tokens::OBJECT_MEMBER,
                                                Some(op_key(op)),
                                            );
                                            self.push_vn_ir(fd, arch, vn, op);
                                            self.push_atom(&Atom::field(
                                                fname,
                                                TagType::FieldToken,
                                                crate::printlanguage::SyntaxHighlight::no_color,
                                                0,
                                                fident,
                                                op_key(op),
                                            ));
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                        // Fall thru to functional/cast printing (printc.cc:889).
                    }
                }
            }
        }
        // Non-special-print SUBPIECE: preserve the prior dispatch exactly (the
        // `_ => Custom` arm routed every SUBPIECE to `op_func_ir`).  The C++
        // `opSubpiece` does `isSubpieceCast ? opTypeCast : opFunc` here, but
        // activating the cast arm perturbs an unrelated (different-IR) render in
        // this port (`condconstsub`: a non-composite SUBPIECE the merge left as a
        // truncation would print `(int4)ptr`, tripping a min=0/max=0 assertion).
        // The cast-vs-func selection for the non-composite SUBPIECE is a separate
        // seam; gate it out so only the composite field-extraction path is new and
        // every other SUBPIECE stays byte-identical.  `subpiece_is_cast` is kept
        // (it is the faithful predicate the composite arm needs and the next wave
        // will switch on) but only the functional tail fires today.
        let _ = self.subpiece_is_cast(fd, arch, op);
        self.op_func_ir(fd, arch, op);
    }

    /// C++ `castStrategy->isSubpieceCast(out->getHighTypeDefFacing(),
    /// in0->getHighTypeReadFacing(op), (uint4)in1->getOffset())` (printc.cc:892).
    fn subpiece_is_cast(&self, fd: &Funcdata, arch: &Architecture, op: OpId) -> bool {
        let strat = match cast_strategy_for(arch) {
            Some(s) => s,
            None => return false,
        };
        let outvn = match fd.obank().get(op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return false,
        };
        let invn = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return false,
        };
        let offset = fd
            .obank()
            .get(op)
            .and_then(|o| o.get_in(1))
            .and_then(|v| fd.vbank().get(v))
            .map(|v| v.get_offset())
            .unwrap_or(0) as uint4;
        let outtype = match fd.vbank().get(outvn) {
            Some(v) => v.get_type_def_facing().clone(),
            None => return false,
        };
        let intype = match fd.vbank().get(invn) {
            Some(v) => v.get_type_read_facing(op).clone(),
            None => return false,
        };
        strat.is_subpiece_cast(&outtype, &intype, offset)
    }

    /// The `(out->getHighTypeDefFacing(), in0->getHighTypeReadFacing(op))` type
    /// pair the C++ `opIntSext`/`opIntZext` feed to `isSextCast`/`isZextCast`
    /// (printc.cc:809/822).  Resolved through the bare-Varnode facing accessors
    /// (the W10 printc convention: by print-time the merged HighVariable type is
    /// already pinned onto the Varnode, so `getTypeDefFacing`/`getTypeReadFacing`
    /// equal the high-facing types the C++ reads). // SEAM(W8 union findResolve)
    fn sext_zext_facing_types(
        &self,
        fd: &Funcdata,
        op: OpId,
    ) -> Option<(std::rc::Rc<crate::dtype::Datatype>, std::rc::Rc<crate::dtype::Datatype>)> {
        let outvn = fd.obank().get(op)?.get_out()?;
        let invn = fd.obank().get(op)?.get_in(0)?;
        let outtype = fd.vbank().get(outvn)?.get_type_def_facing().clone();
        let intype = fd.vbank().get(invn)?.get_type_read_facing(op).clone();
        Some((outtype, intype))
    }

    /// C++ `castStrategy->isExtensionCastImplied(op, readOp)` (cast.cc:249) bridged
    /// through an immutable [`PrintCastContext`] over `&Funcdata`.  The predicate
    /// reads only IR shape + read-facing types (no mutation), so it runs on the
    /// `&Funcdata` print path.
    fn is_extension_cast_implied(
        &self,
        fd: &Funcdata,
        strat: &CastStrategyC,
        op: OpId,
        read_op: Option<OpId>,
    ) -> bool {
        let ctx = PrintCastContext::new(fd);
        let op_ref = ctx.op_ref(op);
        let read_ref = read_op.map(|r| ctx.op_ref(r));
        strat.is_extension_cast_implied(&ctx, op_ref, read_ref)
    }

    /// C++ `PrintC::pushType` (printc.cc:1540) for a base type, reduced to the
    /// cast use: emit the type name as a single type-token operand (the
    /// `(type)` half of a [`tokens::TYPECAST`]).  The full `pushTypeStart` /
    /// `buildTypeStack` declarator algorithm (pointer/array casts) is the next
    /// layer; this renders the base-type front of [`declarator_parts`], which
    /// is the only form the int→float cast produces (a scalar `floatN`).
    fn push_cast_type(&mut self, ct: &std::rc::Rc<crate::dtype::Datatype>) {
        let (front, back) = declarator_parts(ct);
        let mut name = front;
        name.push_str(&back);
        // The C++ pushes a type Atom carrying the Datatype pointer; the kuna
        // emit path renders a TypeToken by its `name` alone (printc.rs:1464),
        // so a syntax-only TypeToken reproduces the cast's `(floatN)` text.
        self.push_atom(&Atom::syntax(
            name,
            TagType::TypeToken,
            crate::printlanguage::SyntaxHighlight::type_color,
        ));
    }

    /// C++ `PrintC::opCall` (printc.cc:613) / `PrintC::opCallind` (printc.cc:657):
    /// the functional `callee(arg1, arg2, ...)` form over the recovered call
    /// inputs.
    ///
    /// For a direct CALL the callee name is recovered from the \e fspec annotation
    /// in0 (the registered call-spec name, else `func_<addr>`/`sub_<addr>`); the
    /// arguments are `in[1..]`.  For a CALLIND the callee is `(*funcptr)` where the
    /// funcptr is `in[0]` and the arguments are `in[1..]`.  The hidden-`this` slot
    /// (`getHiddenThisSlot`) is the C++ method-invocation seam (always -1 here —
    /// the C++ `int4 skip = -1;` for the direct case, no C++ method format yet).
    fn op_call_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let opc = fd.obank().get(op).expect("op_call_ir: stale op").code();
        let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));

        if opc == OpCode::CPUI_CALLIND {
            // CALLIND: `(*funcptr)(args)`.  The dereference operator wraps in0; args
            // are in[1..].  count = numInput-1 (no hidden-this here).
            self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
            let count = nin - 1;
            if count >= 1 {
                // (count-1) comma operators glue the argument list.
                for _ in 0..(count - 1).max(0) {
                    self.push_op(&tokens::COMMA, Some(op_key(op)));
                }
                // The dereferenced callee (in0) is the function expression; the
                // args are in[1..].  Push the callee first, then the args in order.
                if let Some(callee) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                    self.push_vn_ir(fd, arch, callee, op);
                }
                for i in 1..nin {
                    if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(i)) {
                        self.push_vn_ir(fd, arch, vn, op);
                    }
                }
            } else {
                // Void indirect call: the callee expression then an empty arg token.
                if let Some(callee) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                    self.push_vn_ir(fd, arch, callee, op);
                }
                self.push_atom(&Atom::syntax(
                    "",
                    TagType::BlankToken,
                    crate::printlanguage::SyntaxHighlight::no_color,
                ));
            }
            return;
        }

        // Direct CALL: the callee name from the fspec annotation.
        let name = self.call_callee_name(fd, op);
        self.push_atom(&Atom::with_op(
            name,
            TagType::FuncToken,
            crate::printlanguage::SyntaxHighlight::funcname_color,
            op_key(op),
        ));
        // count = numInput - 1 (no hidden-this: skip = -1).  The argument Varnodes
        // are in[1..].
        let count = nin - 1;
        if count > 0 {
            for _ in 0..(count - 1) {
                self.push_op(&tokens::COMMA, Some(op_key(op)));
            }
            for i in 1..nin {
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_in(i)) {
                    self.push_vn_ir(fd, arch, vn, op);
                }
            }
        } else {
            // Void function: empty token (C++ blanktoken).
            self.push_atom(&Atom::syntax(
                "",
                TagType::BlankToken,
                crate::printlanguage::SyntaxHighlight::no_color,
            ));
        }
    }

    /// Recover the printed callee name for a direct CALL (C++ `PrintC::opCall`'s
    /// fspec-name branch): the registered call-spec name, else
    /// `genericFunctionName(entryaddress)` (`func_<addr>` / `sub_<addr>`).
    ///
    /// The name lives in the \e fspec annotation in0; the `FuncCallSpecs` carries
    /// it (looked up by op).  Falls back to the in0 varnode's printed address if no
    /// call spec is registered (an internal-only op — should not occur on the live
    /// CALL path).
    fn call_callee_name(&self, fd: &Funcdata, op: OpId) -> String {
        if let Some(idx) = fd.get_call_specs_index(op) {
            let fc = fd.get_call_specs(idx);
            let nm = fc.get_name();
            if !nm.is_empty() {
                return nm.to_string();
            }
            // genericFunctionName(entryaddress): angr-style `sub_<addr>` or
            // `func_<addr>` (the architecture's name style).
            return fc.fspec_printed_name(fd.get_arch().name_style_angr);
        }
        // No call spec (should not happen for a live CALL): print the in0 address.
        crate::printc::generic_function_name(
            fd.obank()
                .get(op)
                .and_then(|o| o.get_in(0))
                .and_then(|vn| fd.vbank().get(vn))
                .map(|v| v.get_addr())
                .unwrap_or(&kuna_base::address::Address::default()),
        )
        .unwrap_or_default()
    }

    /// C++ `PrintLanguage::recurse` per-Varnode (printlanguage.cc:533): an
    /// *implied* written Varnode expands its defining op's expression inline; an
    /// *explicit* (or input/free) Varnode becomes a leaf atom.  Resolved
    /// directly (depth-first) rather than via the lazy nodepend queue.
    fn push_vn_ir(&mut self, fd: &Funcdata, arch: &Architecture, vn: VarnodeId, op: OpId) {
        let (implied, has_field, def) = {
            let v = match fd.vbank().get(vn) {
                Some(v) => v,
                None => return,
            };
            (v.is_implied(), v.has_implied_field(), v.get_def())
        };
        if implied {
            // C++ `PrintLanguage::recurse` (printlanguage.cc:533): an implied
            // Varnode carrying a resolved union/struct field renders as
            // `<def-expr>.field` via `pushImpliedField`; otherwise just expand the
            // defining op.
            if has_field && self.push_implied_field_ir(fd, arch, vn, op) {
                return;
            }
            if let Some(defop) = def {
                // defOp->getOpcode()->push(this,defOp,op): `op` is the reading op
                // (the C++ `readOp`), threaded so opIntSext/opIntZext can test
                // isExtensionCastImplied against the surrounding expression.
                self.op_push_ir(fd, arch, defop, Some(op));
                return;
            }
        }
        self.push_vn_explicit_ir(fd, arch, vn, op);
    }

    /// C++ `PrintC::pushImpliedField` (printc.cc:2161-2192): an implied Varnode
    /// whose high data-type is a union (or a single-field struct) resolves, via the
    /// per-function union cache, to a specific field; render `<def-expr>.field`.
    ///
    /// Returns `true` when the field render was emitted (the C++ `proceed` arm);
    /// `false` when nothing resolved (the C++ "Just push original op" arm), so the
    /// caller falls back to expanding the defining op.
    ///
    /// SEAM(merge high-type retention): the C++ reads the *unresolved* union parent
    /// off `vn->getHigh()->getType()`, then resolves the field through the cache.
    /// In the merged rust tree the implied Varnode's bare `get_type()` (the
    /// print-time high surface) has already been *updated* to the resolved field
    /// data-type by the cast/merge passes, so the union parent is not available
    /// here and `parent.needs_resolution()` is false for the value-member cases
    /// (`glob.intfield`, `(ptr->value).myint`).  This arm is therefore the faithful
    /// port but is *inert* until the HighVariable retains the needs-resolution
    /// union type at print time (a merge-stage surface owned elsewhere); it never
    /// changes a render today (gated on `has_implied_field`, union-resolution-only)
    /// and lights up the value-member renders once that retention lands.
    fn push_implied_field_ir(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        vn: VarnodeId,
        op: OpId,
    ) -> bool {
        // Datatype *parent = vn->getHigh()->getType();  (bare type by print-time).
        let parent = match fd.vbank().get(vn).map(|v| v.get_type().clone()) {
            Some(t) => t,
            None => return false,
        };
        let mut field: Option<(String, int4)> = None; // (name, ident)
        // if (parent->needsResolution() && parent->getMetatype() != TYPE_PTR) {
        if parent.needs_resolution()
            && parent.get_metatype() != crate::dtype::type_metatype::TYPE_PTR
        {
            // int4 slot = op->getSlot(vn);
            let slot = fd.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            // res = fd->getUnionField(parent, op, slot);
            if let Some(res) = fd.get_union_field(&parent, op, slot) {
                let field_num = res.get_field_num();
                if field_num >= 0 {
                    match parent.get_metatype() {
                        // STRUCT with fieldNum == 0: beginField().
                        crate::dtype::type_metatype::TYPE_STRUCT if field_num == 0 => {
                            if let Some(f) = parent.get_field(0) {
                                field = Some((f.name.clone(), f.ident));
                            }
                        }
                        // UNION: getField(fieldNum).
                        crate::dtype::type_metatype::TYPE_UNION => {
                            if let Some(f) = parent.get_field(field_num) {
                                field = Some((f.name.clone(), f.ident));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        // const PcodeOp *defOp = vn->getDef();
        let def_op = match fd.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(d) => d,
            None => return false,
        };
        let (fieldname, fieldid) = match field {
            Some(f) => f,
            // if (!proceed) { defOp->push(this,defOp,op); return; }  -> caller does it.
            None => return false,
        };
        // pushOp(&object_member,op); defOp->push(this,defOp,op);
        // pushAtom(Atom(field->name, fieldtoken, ..., parent, field->ident, op));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.op_push_ir(fd, arch, def_op, Some(op));
        let field_atom = Atom::field(
            fieldname,
            TagType::FieldToken,
            crate::printlanguage::SyntaxHighlight::no_color,
            0,
            fieldid,
            op_key(op),
        );
        self.push_atom(&field_atom);
        true
    }

    /// `pushVn(vn, op, m)` — set the value-rendering mods (`print_load_value` /
    /// `print_store_value`) for the recursive descent into `vn`'s defining op, then
    /// restore.  In the direct-recursion RPN engine the mods live on `self.context`
    /// (the C++ stashes them on the deferred `nodepend` entry).
    fn push_vn_ir_m(&mut self, fd: &Funcdata, arch: &Architecture, vn: VarnodeId, op: OpId, m: uint4) {
        let save = self.context.mods();
        self.context.set_mods(m);
        self.push_vn_ir(fd, arch, vn, op);
        self.context.set_mods(save);
    }

    /// C++ `PrintC::checkArrayDeref(vn)` (printc.cc:354): is `vn` an implied value
    /// produced by a PTRSUB/PTRADD (optionally through a SEGMENTOP)?  Such a value
    /// renders with array/member notation rather than an explicit `*` dereference.
    fn check_array_deref(&self, fd: &Funcdata, vn: VarnodeId) -> bool {
        let v = match fd.vbank().get(vn) {
            Some(v) => v,
            None => return false,
        };
        if !v.is_implied() || !v.is_written() {
            return false;
        }
        let mut op = match v.get_def() {
            Some(o) => o,
            None => return false,
        };
        if fd.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_SEGMENTOP) {
            let vn2 = match fd.obank().get(op).and_then(|o| o.get_in(2)) {
                Some(v) => v,
                None => return false,
            };
            let v2 = match fd.vbank().get(vn2) {
                Some(v) => v,
                None => return false,
            };
            if !v2.is_implied() || !v2.is_written() {
                return false;
            }
            op = match v2.get_def() {
                Some(o) => o,
                None => return false,
            };
        }
        let code = fd.obank().get(op).map(|o| o.code());
        code == Some(OpCode::CPUI_PTRSUB) || code == Some(OpCode::CPUI_PTRADD)
    }

    /// C++ `PrintC::opLoad` (printc.cc:507).  A LOAD renders either as an array/
    /// member value (when the pointer is a PTRSUB/PTRADD, absorbing the deref) or
    /// as an explicit `*ptr`.
    fn op_load_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let ptr = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
        let usearray = self.check_array_deref(fd, ptr);
        let mut m = self.context.mods();
        if usearray && !self.context.is_set(modifiers::FORCE_POINTER) {
            m |= modifiers::PRINT_LOAD_VALUE;
        } else {
            self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
        }
        self.push_vn_ir_m(fd, arch, ptr, op, m);
    }

    /// C++ `PrintC::opStore` (printc.cc:520).  `*ptr = value` (or member/array
    /// notation absorbing the deref).
    fn op_store_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let mods = self.context.mods();
        self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
        let ptr = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
        let val = fd.obank().get(op).and_then(|o| o.get_in(2));
        let usearray = self.check_array_deref(fd, ptr);
        let mut m = mods;
        if usearray && !self.context.is_set(modifiers::FORCE_POINTER) {
            m |= modifiers::PRINT_STORE_VALUE;
        } else {
            self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
        }
        // C++ pushes value (slot 2) then pointer (slot 1) onto the LIFO
        // nodepend, so the LIFO reversal makes the pointer the LHS:
        // `ptr = value`.  The direct-recursion engine here renders in push
        // order (first push = leftmost operand, the inverse of the C++ LIFO),
        // so to keep the pointer on the LHS of `=` we push the pointer first,
        // then the value — exactly as op_binary_ir inverts in0/in1.
        self.push_vn_ir_m(fd, arch, ptr, op, m);
        if let Some(val) = val {
            self.push_vn_ir_m(fd, arch, val, op, mods);
        }
    }

    /// C++ `PrintC::checkBitFieldMember` (printc.cc:378-389): decide whether a
    /// bitfield access through a LOAD/STORE should use member syntax (`.`) or
    /// pointer syntax (`->`).
    ///
    /// If the bitfield is not at byte offset 0 a PTRSUB must be present accessing
    /// the bitfield storage range; that PTRSUB is skipped and member syntax is
    /// used only when *another* PTRSUB/PTRADD remains underneath
    /// ([`check_array_deref`](Self::check_array_deref)).
    fn check_bit_field_member(&self, fd: &Funcdata, vn: VarnodeId, field: &crate::dtype::TypeBitField) -> bool {
        let mut vn = vn;
        if field.byte_offset != 0 {
            // Bitfield not at offset 0, a PTRSUB should be present.
            let v = match fd.vbank().get(vn) {
                Some(v) => v,
                None => return false,
            };
            if !v.is_written() {
                return false;
            }
            let op = match v.get_def() {
                Some(o) => o,
                None => return false,
            };
            if fd.obank().get(op).map(|o| o.code()) != Some(OpCode::CPUI_PTRSUB) {
                return false;
            }
            vn = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
                Some(v) => v, // Skip this PTRSUB
                None => return false,
            };
        }
        self.check_array_deref(fd, vn)
    }

    /// Push the bitfield-name Atom (C++ `Atom(field->name,bitfieldtoken,no_color,
    /// theStruct,field->ident,op)`, e.g. printc.cc:1311).  The struct marker is
    /// markup-only; the field name + `ident` (carried in the Atom `offset`) drive
    /// the no-markup render.
    fn push_bitfield_atom(&mut self, field: &crate::dtype::TypeBitField, op: OpId) {
        self.push_atom(&Atom::field(
            field.name.clone(),
            TagType::BitFieldToken,
            crate::printlanguage::SyntaxHighlight::no_color,
            0,
            field.ident,
            op_key(op),
        ));
    }

    /// C++ `PrintC::opZpullOp` (printc.cc:1294) / `PrintC::opSpullOp`
    /// (printc.cc:1320): render a bitfield read.  Both bodies are identical (the
    /// signed/unsigned distinction lives in the recovery's type, not the render),
    /// so they share this method.
    ///
    /// When the read goes through a LOAD, the structure pointer is pushed with
    /// member (`.`) or pointer (`->`) syntax and the bitfield name follows.  When
    /// the read is of a bound (partial) symbol, the symbol detail is pushed with
    /// member syntax.  On an unrecognized form, fall back to the functional
    /// `ZPULL(...)`/`SPULL(...)` render.
    fn op_pull_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let expr = crate::bitfield::expression::PullExpression::new(fd, op);
        let bitfield = match (expr.is_valid(), &expr.expr.bitfield) {
            (true, Some(b)) => b.clone(),
            _ => {
                // If no other way to print it, print as functional operator.
                self.op_func_ir(fd, arch, op);
                return;
            }
        };
        if let Some(load_op) = expr.load_op {
            let load_ptr = fd.obank().get(load_op).and_then(|o| o.get_in(1));
            let mut m = self.context.mods();
            let use_member = load_ptr
                .map(|p| self.check_bit_field_member(fd, p, &bitfield))
                .unwrap_or(false);
            if use_member {
                m |= modifiers::PRINT_LOAD_VALUE;
                self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
            } else {
                self.push_op(&tokens::POINTER_MEMBER, Some(op_key(op)));
            }
            if let Some(sp) = expr.struct_ptr {
                self.push_vn_ir_m(fd, arch, sp, load_op, m);
            }
            self.push_bitfield_atom(&bitfield, op);
        } else {
            // Bound-symbol read: `symbol.field`.
            self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
            if let Some(in0) = fd.obank().get(op).and_then(|o| o.get_in(0)) {
                self.push_vn_ir(fd, arch, in0, op);
            }
            self.push_bitfield_atom(&bitfield, op);
        }
    }

    /// C++ `PrintC::emitBitFieldStore` (printc.cc:2595-2620): render a bitfield
    /// write through a STORE as `ptr->field = value` (or `ptr.field = value`).
    ///
    /// On an unrecognized form, fall back to the normal STORE render.
    fn emit_bitfield_store(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let expr = crate::bitfield::expression::InsertStoreExpression::new(fd, op);
        let bitfield = match (expr.is_valid(), &expr.expr.bitfield, expr.insert_op, expr.struct_ptr) {
            (true, Some(b), Some(_), Some(_)) => b.clone(),
            _ => {
                // op->getOpcode()->push(this,op,(PcodeOp *)0): the normal STORE.
                self.op_store_ir(fd, arch, op);
                return;
            }
        };
        let insert_op = expr.insert_op.unwrap();
        let struct_ptr = expr.struct_ptr.unwrap();
        // We assume the STORE is a statement.
        self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
        let store_ptr = fd.obank().get(op).and_then(|o| o.get_in(1));
        let mut m = self.context.mods();
        let use_member = store_ptr
            .map(|p| self.check_bit_field_member(fd, p, &bitfield))
            .unwrap_or(false);
        if use_member {
            m |= modifiers::PRINT_STORE_VALUE;
            self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(insert_op)));
        } else {
            self.push_op(&tokens::POINTER_MEMBER, Some(op_key(insert_op)));
        }
        // C++ pushes the LHS (structPtr.field) then the RHS (insert value); the
        // direct RPN engine renders in push order, so push the pointer + bitfield
        // first (LHS of `=`), then the value.
        self.push_vn_ir_m(fd, arch, struct_ptr, op, m);
        self.push_bitfield_atom(&bitfield, op);
        // pushVn(expr.insertOp->getIn(1),op,mods): the value being written.
        if let Some(val) = fd.obank().get(insert_op).and_then(|o| o.get_in(1)) {
            self.push_vn_ir_m(fd, arch, val, op, self.context.mods());
        }
    }

    /// C++ `PrintC::emitBitFieldExpression` (printc.cc:2622-2637): render a
    /// bitfield write into an explicit (mapped) Varnode as `symbol.field = value`.
    ///
    /// On an unrecognized form, fall back to the functional `INSERT(...)` render.
    fn emit_bitfield_expression(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let expr = crate::bitfield::expression::InsertExpression::new(fd, op);
        let bitfield = match (expr.expr.is_valid(), &expr.expr.bitfield) {
            (true, Some(b)) => b.clone(),
            _ => {
                // If no other way to print it, print as functional operator.
                self.op_func_ir(fd, arch, op);
                return;
            }
        };
        self.push_op(&tokens::ASSIGNMENT, Some(op_key(op)));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        // pushPartialSymbol(symbol,offsetToBitStruct,theStruct->getSize(),out,..):
        // the (partial) symbol carrying the structure.  In the merged tree the
        // symbol name is bound on the output's HighVariable, so push the output
        // Varnode's explicit name (the same surface push_vn_explicit_ir reads).
        if let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()) {
            self.push_vn_explicit_ir(fd, arch, out, op);
        }
        self.push_bitfield_atom(&bitfield, op);
        if let Some(val) = fd.obank().get(op).and_then(|o| o.get_in(1)) {
            self.push_vn_ir_m(fd, arch, val, op, self.context.mods());
        }
    }

    /// C++ `PrintC::opPtradd` (printc.cc:900).  `ptr[index]` (value), `&ptr[index]`
    /// (array-notation address), or `ptr + index` (plain pointer arithmetic).
    fn op_ptradd_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let printval = self
            .context
            .is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let m = self.context.mods() & !(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        if printval {
            self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
        } else if self.options.array_notation() {
            // (kuna) S9 pointer-notation sub-stage: EMIT &base[index].
            self.push_op(&tokens::ADDRESSOF, Some(op_key(op)));
            self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
        } else {
            self.push_op(&tokens::BINARY_PLUS, Some(op_key(op)));
        }
        // C++ pushes in1 (index) then in0 (base) onto the LIFO nodepend; the direct
        // RPN engine drains in push order, so push in0 (base) then in1 (index) to
        // render `base[index]`.
        let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
        let in1 = fd.obank().get(op).and_then(|o| o.get_in(1));
        if let Some(in0) = in0 {
            self.push_vn_ir_m(fd, arch, in0, op, m);
        }
        if let Some(in1) = in1 {
            self.push_vn_ir_m(fd, arch, in1, op, m);
        }
    }

    /// C++ `PrintC::pushPartialSymbol` (printc.cc:2019-2141), restricted to the
    /// STRUCT/UNION arms of the type walk (the symbol-mapped member-access render
    /// `glob.intfield` / `val.c` / `globvar.b.bval1`).
    ///
    /// Reconciled with the kuna naming layer: the base symbol name comes from the
    /// HighVariable's `kuna_name` binding (the `pushSymbol(sym,vn,op)` stand-in,
    /// printc.cc:2127) rather than a `Symbol *`; the walked data-type is the
    /// `kuna_symbol_type` (the `sym->getType()` stand-in, printc.cc:2030).  The
    /// UNION `findTruncation` (type.cc:2613-2627) reads the Funcdata union
    /// resolution cache via [`Funcdata::get_union_field`]; the STRUCT
    /// `findTruncation` (type.cc:1878) walks the field table
    /// ([`Datatype::find_truncation`]).
    ///
    /// Returns `true` when the walk produced a genuine member token (the partial
    /// cover render fired) and `false` otherwise — on `false` the caller renders
    /// the bare symbol name, so a non-partial read stays byte-identical.  The
    /// ARRAY arm (printc.cc:2062-2076, needs `TypeArray::getSubEntry`) and the
    /// `allowCast` SUBPIECE-cast arm (printc.cc:2094-2105) are not reached from
    /// this entry (`allow_cast == false`); an array Symbol is handled by the
    /// caller's existing `name[index]` branch.
    #[allow(clippy::too_many_arguments)]
    fn push_partial_symbol_ir(
        &mut self,
        fd: &Funcdata,
        _arch: &Architecture,
        name: &str,
        sym_type: std::rc::Rc<crate::dtype::Datatype>,
        off_in: int8,
        sz_in: int4,
        vn: VarnodeId,
        op: OpId,
        slot: int4,
        _allow_cast: bool,
    ) -> bool {
        use crate::dtype::type_metatype;
        // PartialSymbolEntry stack (C++ `vector<PartialSymbolEntry> stack`,
        // printc.cc:2026): each entry is a resolved member token.  We collect
        // (field_name, field_ident) for an `object_member` token.
        let mut stack: Vec<(String, int4)> = Vec::new();
        let mut ct = Some(sym_type);
        let mut off: int8 = off_in;
        let sz: int4 = sz_in;

        // while (ct != 0)  (printc.cc:2032).
        while let Some(cur) = ct.clone() {
            // if (off == 0) { if (sz==0 || (sz==ct->getSize() && (!needsResolution
            //   || metatype==TYPE_PTR))) break; }  (printc.cc:2033-2036).
            if off == 0
                && (sz == 0
                    || (sz == cur.get_size()
                        && (!cur.needs_resolution()
                            || cur.get_metatype() == type_metatype::TYPE_PTR)))
            {
                break;
            }
            let mut succeeded = false;
            let meta = cur.get_metatype();
            if meta == type_metatype::TYPE_STRUCT {
                // TypeStruct::findTruncation walks the field table (no cache).
                // (printc.cc:2044-2056; the needsResolution()/findResolve guard at
                // 2039-2043 only applies to a struct that itself needsResolution,
                // which the corpus structs do not — it would require the union
                // cache and is a no-op for a plain struct.)
                match cur.find_truncation(off, sz, op, slot) {
                    Ok(Some((idx, newoff))) => {
                        if let Some(f) = cur.get_field(idx) {
                            off = newoff;
                            stack.push((f.name.clone(), f.ident));
                            ct = Some(std::rc::Rc::clone(&f.field_type));
                            succeeded = true;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {}
                }
            } else if meta == type_metatype::TYPE_UNION {
                // TypeUnion::findTruncation (type.cc:2613): read the cached union
                // resolution for this (type, op, slot) edge.  No new scoring.
                let field = if cur.needs_resolution() {
                    fd.get_union_resolution(&cur, op, slot)
                        .map(|r| r.get_field_num())
                        .filter(|&n| n >= 0)
                        .and_then(|n| cur.get_field(n).map(|f| (n, f.offset, f.name.clone(), f.ident, std::rc::Rc::clone(&f.field_type))))
                } else {
                    None
                };
                match field {
                    Some((_n, foff, fname, fident, ftype)) => {
                        // newoff = offset - field->offset; truncation must fit the
                        // field (type.cc:2621-2624).
                        let newoff = off - foff as int8;
                        if newoff + sz as int8 > ftype.get_size() as int8 {
                            // Truncation spans more than one field: findTruncation
                            // returns null.  Fall to the `else if size==sz` check.
                            if cur.get_size() == sz {
                                break;
                            }
                            // !succeeded artificial-field fallthrough below.
                        } else {
                            off = newoff;
                            stack.push((fname, fident));
                            ct = Some(ftype);
                            succeeded = true;
                        }
                    }
                    None => {
                        // else if (ct->getSize() == sz) break; (printc.cc:2091).
                        if cur.get_size() == sz {
                            break;
                        }
                    }
                }
            } else {
                // ARRAY / scalar / allowCast arms are not handled by this entry.
                // Bail out so the caller renders the bare name (the array Symbol
                // takes the caller's `name[index]` branch).
                return false;
            }
            if !succeeded {
                // Subtype was not good (printc.cc:2106-2117): generate an artificial
                // member name based on offset/size.  We only reach here for a
                // composite whose member walk failed mid-way; rather than emit a
                // synthesized `field_*` name that the corpus never expects, bail so
                // the bare-name render (byte-identical) wins.  A correct partial
                // cover always `succeeded` above.
                return false;
            }
        }

        // No member tokens collected: this is a whole-symbol cover, render bare.
        if stack.is_empty() {
            return false;
        }

        // Push these on the RPN stack in reverse order (printc.cc:2124-2126):
        // pushOp(object_member) once per member, then the base symbol, then the
        // field atoms in forward order.  The direct-recursion engine emits in push
        // order, so: N object_member ops, base name, then N field atoms.
        for _ in 0..stack.len() {
            self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        }
        // pushSymbol(sym,vn,op) — the base name (the kuna_name stand-in).
        self.push_atom(&Atom::with_op_vn(
            name.to_string(),
            TagType::VarToken,
            crate::printlanguage::SyntaxHighlight::var_color,
            op_key(op),
            vn_key(vn),
        ));
        // pushAtom(Atom(field->name,fieldtoken,...,parent,field->ident,op)) per
        // entry, in forward order (printc.cc:2128-2140).
        for (fname, fident) in &stack {
            self.push_atom(&Atom::field(
                fname.clone(),
                TagType::FieldToken,
                crate::printlanguage::SyntaxHighlight::no_color,
                0,
                *fident,
                op_key(op),
            ));
        }
        true
    }

    /// C++ `PrintLanguage::pushVnExplicit` (printlanguage.cc:218) + the
    /// `PrintC` leaf-naming (`pushVnExplicit`/`pushUnnamedLocation`, printc.cc:
    /// 1900-2017): annotation -> constant -> SymbolEntry -> register name ->
    /// kuna `dat_<addr>` global -> `Space<hex>` fallback.
    fn push_vn_explicit_ir(&mut self, fd: &Funcdata, arch: &Architecture, vn: VarnodeId, op: OpId) {
        let v = match fd.vbank().get(vn) {
            Some(v) => v,
            None => return,
        };
        if v.is_constant() {
            let (off, sz) = (v.get_offset(), v.get_size());
            // C++ `PrintLanguage::pushVnExplicit` (printlanguage.cc:227) calls
            // `pushConstant(vn->getOffset(), ct, ...)` with `ct =
            // vn->getHighTypeReadFacing(op)`.  `pushConstant` (printc.cc:1813)
            // switches on `ct->getMetatype()`: a `TYPE_FLOAT` constant is rendered
            // by `push_float` (the decimal literal), every other metatype reaches
            // `push_integer` with `ct->getDisplayFormat()` as its `displayFormat`.
            let ct = v.get_type_read_facing(op).clone();
            if ct.get_metatype() == crate::dtype::type_metatype::TYPE_FLOAT {
                // C++ `pushConstant` -> `push_float(val, ct->getSize(), ...)`.  The
                // float arm ignores the integer `displayFormat` entirely.
                self.push_float_ir(arch, off, ct.get_size(), op);
                return;
            }
            // Enum arm.  C++ `pushConstant` (printc.cc:1817-1833) switches on the
            // enum's base metatype (TYPE_INT / TYPE_UINT) and, when
            // `ct->isEnumType()`, delegates to `pushEnumConstant` (printc.cc:1822/
            // 1830) — which decomposes the value into the OR of matched flag names.
            // In kuna an enum carries metatype TYPE_INT/TYPE_UINT plus the
            // `enumtype` flag (dtype.rs:5244-5246), exactly as upstream, so the
            // dispatch is the `is_enum_type()` flag check (not a metatype match).
            if ct.is_enum_type() {
                self.push_enum_constant_ir(&ct, off, op, vn);
                return;
            }
            // Pointer arm.  C++ `pushConstant` (printc.cc:1842-1854): a TYPE_PTR /
            // TYPE_PTRREL constant whose pointed-to type `isCharPrint()` is rendered
            // as a quoted string literal when the constant resolves to readonly
            // character data (`pushPtrCharConstant`).  If the pointer does not
            // resolve to a readable readonly string, the C++ falls through to the
            // default integer print — so does this arm (it only short-circuits on a
            // successful string push).  The TYPE_CODE (function-name) sub-arm is a
            // documented LOSS below.
            use crate::dtype::type_metatype::{TYPE_PTR, TYPE_PTRREL};
            if matches!(ct.get_metatype(), TYPE_PTR | TYPE_PTRREL) && off != 0 {
                if let Some(sub) = ct.get_ptr_to() {
                    if sub.is_char_print() {
                        // point = op->getAddr() (the using op's address; used only
                        // by a segmented resolver — flat spaces ignore it).
                        let point = fd
                            .obank()
                            .get(op)
                            .map(|o| o.get_addr().clone())
                            .unwrap_or_default();
                        if self.push_ptr_char_constant_ir(arch, off, &ct, &sub, &point, op, vn) {
                            return;
                        }
                    }
                }
            }
            // Integer path.  Inside `push_integer` (printc.cc:1376) the varnode
            // high's equate-Symbol format OVERRIDES the read-facing type's format
            // when present.  So: equate-Symbol format wins; otherwise the
            // read-facing type format (e.g. `force datatype octint4 oct` ->
            // `globaloct = 05555`).
            let sym_fmt = fd.vn_high_display_format(vn);
            let display_fmt = if sym_fmt != display_format::NONE {
                sym_fmt
            } else {
                ct.get_display_format()
            };
            // C++ `pushConstant` (printc.cc:1817-1835) selects the `push_integer`
            // `sign` from the read-facing metatype: TYPE_INT -> signed
            // (printc.cc:1832), TYPE_UINT/TYPE_UNKNOWN -> unsigned (1824/1835).
            // The float/enum/char arms were already dispatched above, so a plain
            // integer constant rendered here is signed exactly when its type is
            // TYPE_INT — which is what makes a negative `recv_signed(int4)` convert
            // constant print `-512` instead of its unsigned bit pattern.
            let sign = ct.get_metatype() == crate::dtype::type_metatype::TYPE_INT;
            self.push_constant_ir_fmt_sign(off, sz, op, display_fmt, sign);
            return;
        }
        // HighVariable name resolution (C++ `pushSymbolDetail`: `high->getSymbol()`
        // -> `pushSymbol` -> `sym->getDisplayName()`).  The merged tree binds the
        // angr default name directly on the HighVariable (`ActionNameVars` ->
        // `HighVariable::kuna_name`; the W4 `Symbol`/ScopeLocal stand-in), so a
        // named high renders its bound `vN` name here — for *every* member, which
        // is exactly how the C++ renders all instances of a merged local.
        if let Some(high) = v.get_high() {
            let named = fd.high_bank().get(high).and_then(|h| h.kuna_name()).map(|n| {
                let hb = fd.high_bank().get(high).unwrap();
                (n.to_string(), hb.kuna_symbol_offset(), hb.kuna_symbol_type().cloned())
            });
            if let Some((name, sym_off, sym_type)) = named {
                // Symbol-mapped struct/union member access (C++ `PrintC::
                // pushSymbolDetail` -> `pushPartialSymbol`, printlanguage.cc:256-258
                // + printc.cc:2019-2141).  When the mapped Symbol's data-type is a
                // composite (a UNION that resolves to a field for this op, or a
                // STRUCT whose member contains the access) the varnode is a partial
                // cover of the larger Symbol and renders `name.field` /
                // `name.b.bval1` rather than its raw name.  This is GUARDED tightly:
                // it fires only when the type walk genuinely yields a member token,
                // so a non-partial-cover read (the common case) is byte-unchanged
                // and falls straight through to the bare-name render below.
                if let Some(st) = &sym_type {
                    let mt = st.get_metatype();
                    if mt == crate::dtype::type_metatype::TYPE_STRUCT
                        || mt == crate::dtype::type_metatype::TYPE_UNION
                    {
                        // C++ `pushSymbolDetail`: `isRead` is true when `op` reads
                        // `vn` (the input slot); false when `vn` is the output (the
                        // assignment LHS), where the artificial slot is -1.
                        let is_out =
                            fd.obank().get(op).and_then(|o| o.get_out()) == Some(vn);
                        let is_read = !is_out;
                        let inslot = if is_read {
                            fd.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1)
                        } else {
                            -1
                        };
                        // `symboloff` is the in-symbol byte offset; C++ resets a -1
                        // (whole-symbol) offset to 0 before the partial walk when the
                        // type needs resolution (printlanguage.cc:249-255).
                        let symoff = if sym_off < 0 { 0 } else { sym_off };
                        if self.push_partial_symbol_ir(
                            fd,
                            arch,
                            &name,
                            std::rc::Rc::clone(st),
                            symoff as int8,
                            v.get_size(),
                            vn,
                            op,
                            inslot,
                            is_read,
                        ) {
                            return;
                        }
                    }
                }
                // Array/struct member access: if the mapped Symbol is an array and
                // the access is at a non-base offset (or the symbol is strictly
                // larger than the access), render `name[index]` (C++
                // `PrintC::pushSymbolDetail`'s array branch).
                if let Some(st) = &sym_type {
                    if st.get_metatype() == crate::dtype::type_metatype::TYPE_ARRAY {
                        if let Some(elem) = st.get_array_base() {
                            let elsize = elem.get_size().max(1);
                            // The access maps to element `index` when it lies
                            // within the array and the offset divides the element.
                            if sym_off >= 0 && (sym_off % elsize) == 0 && st.get_size() > elsize {
                                let index = sym_off / elsize;
                                // `name[index]` via the subscript op-token.
                                self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
                                self.push_atom(&Atom::with_op_vn(
                                    name,
                                    TagType::VarToken,
                                    crate::printlanguage::SyntaxHighlight::var_color,
                                    op_key(op),
                                    vn_key(vn),
                                ));
                                self.push_atom(&Atom::with_op(
                                    format!("{index}"),
                                    TagType::Syntax,
                                    crate::printlanguage::SyntaxHighlight::const_color,
                                    op_key(op),
                                ));
                                return;
                            }
                        }
                    }
                }
                self.push_atom(&Atom::with_op_vn(
                    name,
                    TagType::VarToken,
                    crate::printlanguage::SyntaxHighlight::var_color,
                    op_key(op),
                    vn_key(vn),
                ));
                return;
            }
        }
        // No bound name: fall to the register / unnamed-location naming, which is
        // the faithful `pushUnnamedLocation` tail (printc.cc:1957-1974).
        let loc = v.get_addr().clone();
        let size = v.get_size();
        let spc = match loc.get_space() {
            Some(s) => s,
            None => return,
        };
        let regname = arch.translate().get_register_name(spc, loc.get_offset(), size);
        let name = if !regname.is_empty() {
            regname
        } else if kuna_global_naming(spc) {
            // (kuna) angr-style unnamed data annotation -> dat_<addr>.
            kuna_global_data_name(spc, loc.get_offset())
        } else {
            // Space<hex> capitalized form (printc.cc:1964-1970).
            let mut s = String::new();
            let sn = spc.get_name();
            let mut chars = sn.chars();
            if let Some(c0) = chars.next() {
                s.extend(c0.to_uppercase());
                s.push_str(chars.as_str());
            }
            use std::fmt::Write;
            let _ = write!(s, "{:0width$x}", loc.get_offset(), width = (2 * spc.get_addr_size()) as usize);
            s
        };
        self.push_atom(&Atom::with_op_vn(
            name,
            TagType::VarToken,
            crate::printlanguage::SyntaxHighlight::special_color,
            op_key(op),
            vn_key(vn),
        ));
    }

    /// C++ `PrintC::opPtrsub` (printc.cc:953).  `&ptr->field` / `ptr->field`
    /// (struct member) or `*ptr` / `ptr[0]` (array element), absorbing or emitting
    /// the dereference per the load/store value mods and the `&base[index]` flex.
    ///
    /// The SPACEBASE arm (a PTRSUB off a stack/global spacebase, requiring the
    /// Symbol/ScopeLocal surface) and the union arm are not on the pointer/array/
    /// struct corpus; they fall through to a functional render.
    /// SEAM(W4 spacebase symbol) / SEAM(W8 union).
    /// C++ `PrintC::pushTypePointerRel` (printc.hh:372-377): a PTRSUB acting
    /// relative to a `TypePointerRel` parent prints the `ADJ(...)` macro — a
    /// `function_call` op wrapping the `ADJ` token (rendered `funcname_color`).
    fn push_type_pointer_rel_ir(&mut self, op: OpId) {
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        // The token is pushed as an *operator* token (C++ `optoken`), but with
        // funcname_color (matching the C++ Atom(typePointerRelToken,optoken,
        // funcname_color,op)).
        self.push_atom(&Atom::with_op(
            keywords::TYPE_POINTER_REL_TOKEN.to_string(),
            TagType::OpToken,
            crate::printlanguage::SyntaxHighlight::funcname_color,
            op_key(op),
        ));
    }

    fn op_ptrsub_ir(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) {
        let in0 = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return,
        };
        let in1const = fd
            .obank()
            .get(op)
            .and_then(|o| o.get_in(1))
            .and_then(|v| fd.vbank().get(v))
            .map(|v| v.get_offset())
            .unwrap_or(0);
        // ptype = in0->getHighTypeReadFacing(op)  (== get_type for the non-union corpus).
        let ptype = match fd.vbank().get(in0).map(|v| v.get_type().clone()) {
            Some(t) => t,
            None => return,
        };
        if ptype.get_metatype() != crate::dtype::type_metatype::TYPE_PTR {
            // C++ throws; fall to the functional render so output stays parseable.
            self.op_func_ir(fd, arch, op);
            return;
        }
        // Relative-pointer parent resolution.
        let is_rel = ptype.is_formal_pointer_rel()
            && ptype.evaluate_thru_parent(in1const) == Some(true);
        let ct = if is_rel {
            ptype.get_rel_parent()
        } else {
            ptype.get_ptr_to()
        };
        let ct = match ct {
            Some(c) => c,
            None => return,
        };
        let ptr_size = fd.vbank().get(in0).map(|v| v.get_size()).unwrap_or(8);
        let m = self.context.mods()
            & !(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let mut valueon = (self.context.mods()
            & (modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE))
            != 0;
        let flex = self.is_value_flexible_ir(fd, in0);
        let word_size = ptype.get_word_size().unwrap_or(1);
        let metameta = ct.get_metatype();

        if metameta == crate::dtype::type_metatype::TYPE_STRUCT
            || metameta == crate::dtype::type_metatype::TYPE_UNION
        {
            // suboff = (int4)in1const  (+ relative offset).
            let mut suboff = in1const as int4 as int8;
            if is_rel {
                let addr_off = ptype.get_address_offset().unwrap_or(0) as int8;
                suboff = (((suboff + addr_off) as u64) & calc_mask(ptr_size)) as int8;
                if suboff == 0 {
                    // Special case where we do not print a field (printc.cc:988).
                    self.push_type_pointer_rel_ir(op);
                    let mm = if flex { m | modifiers::PRINT_LOAD_VALUE } else { m };
                    self.push_vn_ir_m(fd, arch, in0, op, mm);
                    return;
                }
            }
            let suboff_bytes = AddrSpace::address_to_byte_int(suboff, word_size);
            let (fieldname, fieldtype, fieldid) =
                if metameta == crate::dtype::type_metatype::TYPE_UNION {
                    // TYPE_UNION arm (printc.cc:1000-1014).
                    // if (suboff != 0) throw "PTRSUB accesses union with non-zero offset";
                    if suboff_bytes != 0 {
                        // C++ throws; fall to the functional render so output stays
                        // parseable rather than aborting the whole function.
                        self.op_func_ir(fd, arch, op);
                        return;
                    }
                    // resUnion = fd->getUnionField(ptype, op, -1);
                    // The cast plane (`ActionSetCasts::resolveUnion`) stored the
                    // resolution on this PTRSUB's write edge keyed on the
                    // pointer-to-union `ptype`; read it back here.
                    let res_field = fd
                        .get_union_field(&ptype, op, -1)
                        .map(|r| r.get_field_num())
                        .filter(|&n| n >= 0);
                    let field_num = match res_field {
                        Some(n) => n,
                        None => {
                            // C++ throws "PTRSUB for union that does not resolve
                            // to a field"; fall to the functional render.
                            self.op_func_ir(fd, arch, op);
                            return;
                        }
                    };
                    // fld = ((TypeUnion*)ct)->getField(resUnion->getFieldNum());
                    match ct.get_field(field_num) {
                        Some(f) => (f.name.clone(), Some(f.field_type.clone()), f.ident),
                        None => {
                            self.op_func_ir(fd, arch, op);
                            return;
                        }
                    }
                } else {
                    // TYPE_STRUCT arm (printc.cc:1015-1033).
                    // fld = ct->findTruncation(suboff,0,op,0,newoff)
                    let fld = ct.find_truncation(suboff_bytes, 0, op, 0).ok().flatten();
                    match fld {
                        Some((idx, _newoff)) => {
                            let f = ct.get_field(idx);
                            match f {
                                Some(f) => (f.name.clone(), Some(f.field_type.clone()), f.ident),
                                None => return,
                            }
                        }
                        None => {
                            if ct.get_size() as int8 <= suboff_bytes || suboff_bytes < 0 {
                                self.op_func_ir(fd, arch, op);
                                return;
                            }
                            // Default field name `field_0x<hex>`.
                            (format!("field_0x{suboff_bytes:x}"), None, suboff_bytes as int4)
                        }
                    }
                };
            let mut arrayvalue = false;
            // The '&' is dropped if the field is an array.
            if let Some(ft) = &fieldtype {
                if ft.get_metatype() == crate::dtype::type_metatype::TYPE_ARRAY {
                    arrayvalue = valueon; // If printing value, use [0]
                    valueon = true; // Don't print &
                }
            }
            let field_atom = Atom::field(
                fieldname,
                TagType::FieldToken,
                crate::printlanguage::SyntaxHighlight::no_color,
                // The Atom's ct marker is markup-only (the no-markup emitter
                // ignores it); the field name/offset carry the rendering.
                0,
                fieldid,
                op_key(op),
            );
            if !valueon {
                // Printing an ampersand.
                self.push_op(&tokens::ADDRESSOF, Some(op_key(op)));
                if flex {
                    // EMIT  &( ).name
                    self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m | modifiers::PRINT_LOAD_VALUE);
                } else {
                    // EMIT  &( )->name
                    self.push_op(&tokens::POINTER_MEMBER, Some(op_key(op)));
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m);
                }
                self.push_atom(&field_atom);
            } else {
                if arrayvalue {
                    self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
                }
                if flex {
                    // EMIT  ( ).name
                    self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m | modifiers::PRINT_LOAD_VALUE);
                } else {
                    // EMIT  ( )->name
                    self.push_op(&tokens::POINTER_MEMBER, Some(op_key(op)));
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m);
                }
                self.push_atom(&field_atom);
                if arrayvalue {
                    self.push_constant_ir(0, 4, op);
                }
            }
        } else if metameta == crate::dtype::type_metatype::TYPE_ARRAY {
            // PTRSUB(*,0) drilling a pointer-to-array down to its element type.
            if !valueon {
                if flex {
                    // EMIT ( ) — absorb the dereference into in0.
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m | modifiers::PRINT_LOAD_VALUE);
                } else {
                    // EMIT *( )
                    self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
                    if is_rel {
                        self.push_type_pointer_rel_ir(op);
                    }
                    self.push_vn_ir_m(fd, arch, in0, op, m);
                }
            } else if flex {
                // EMIT ( )[0]
                self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
                if is_rel {
                    self.push_type_pointer_rel_ir(op);
                }
                self.push_vn_ir_m(fd, arch, in0, op, m | modifiers::PRINT_LOAD_VALUE);
                self.push_constant_ir(0, 4, op);
            } else {
                // EMIT (* )[0]
                self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
                self.push_op(&tokens::DEREFERENCE, Some(op_key(op)));
                if is_rel {
                    self.push_type_pointer_rel_ir(op);
                }
                self.push_vn_ir_m(fd, arch, in0, op, m);
                self.push_constant_ir(0, 4, op);
            }
        } else if metameta == crate::dtype::type_metatype::TYPE_SPACEBASE {
            // SPACEBASE arm (C++ `PrintC::opPtrsub`, printc.cc:1081-1121).  A
            // `PTRSUB(spacebase, off)` is a `&symbol` reference into a stack/global
            // frame.  `ActionNameVars::linkSpacebaseSymbol` decoded the reference
            // and parked the Symbol on the offset constant's HighVariable
            // (`Funcdata::link_symbol_reference` -> `kuna_name`/`symbol_offset`/
            // `kuna_symbol_type`), so this reads it back here.
            //
            //   HighVariable *high = op->getIn(1)->getHigh();
            //   Symbol *symbol = high->getSymbol();
            // The kuna stand-in: read the reference triple off in1's high.
            let in1 = fd.obank().get(op).and_then(|o| o.get_in(1));
            let (sym_name, sym_off, sym_type) = match in1.and_then(|v| fd.vbank().get(v)).and_then(|v| v.get_high()) {
                Some(high) => match fd.high_bank().get(high) {
                    Some(h) => (
                        h.kuna_name().map(|s| s.to_string()),
                        h.kuna_symbol_offset(),
                        h.kuna_symbol_type().cloned(),
                    ),
                    None => (None, -1, None),
                },
                None => (None, -1, None),
            };

            // C++ `opPtrsub` always reaches a Symbol here (`linkSpacebaseSymbol`
            // attached one to every stack/global PTRSUB), branching on
            // `symbol == 0` only for a never-linked spacebase.  In the kuna model
            // `link_symbol_reference` deliberately attaches ONLY a defined-named
            // Symbol (the mapped stack/global vars; an undefined-named auto-local is
            // left unlinked — see `Funcdata::link_symbol_reference`).  So a missing
            // `sym_name` here means "no reliable symbol surface for this reference":
            // render the functional `PTRSUB(...)` form (the pre-render-finish state),
            // NOT the C++ `pushUnnamedLocation` `&stackNN` leaf — which would expose
            // an offset the kuna namerec layer has not yet resolved to a name.
            let name = match &sym_name {
                Some(n) => n.clone(),
                None => {
                    self.op_func_ir(fd, arch, op);
                    return;
                }
            };

            let mut arrayvalue = false; // arrayvalue = false;
            if let Some(st) = &sym_type {
                // ct = symbol->getType(); (symbol != 0 always here)  (printc.cc:1086)
                let mt = st.get_metatype();
                if mt == crate::dtype::type_metatype::TYPE_ARRAY {
                    // The '&' is dropped if the output type is an array.
                    arrayvalue = valueon; // If printing value, use [0]
                    valueon = true; // If printing ptr, don't use &
                } else if mt == crate::dtype::type_metatype::TYPE_CODE {
                    valueon = true; // If printing ptr, don't use &
                }
            }

            if !valueon {
                // EMIT  &name  (printc.cc:1095)
                self.push_op(&tokens::ADDRESSOF, Some(op_key(op)));
            } else if arrayvalue {
                // EMIT  name  with a trailing subscript (printc.cc:1099)
                self.push_op(&tokens::SUBSCRIPT, Some(op_key(op)));
            }

            // int4 off = high->getSymbolOffset();  (printc.cc:1108)
            // off == 0 takes the bare `pushSymbol` arm; a `-1` `symboloffset` (the
            // whole-symbol cover the C++ `setSymbol` records for a size-matching
            // entry) is also a bare-name render, so `off <= 0` covers both.
            if sym_off <= 0 {
                // off == 0: pushSymbol(symbol, 0, op) — the bare name.
                self.push_atom(&Atom::with_op(
                    name.clone(),
                    TagType::VarToken,
                    crate::printlanguage::SyntaxHighlight::var_color,
                    op_key(op),
                ));
            } else {
                // off != 0: pushPartialSymbol(symbol, off, 0, 0, op, -1, false) —
                // `name.field` (printc.cc:1116).
                let st = sym_type.as_ref().map(std::rc::Rc::clone);
                let pushed = if let Some(st) = st {
                    self.push_partial_symbol_ir(
                        fd,
                        arch,
                        &name,
                        st,
                        sym_off as int8,
                        0,
                        in1.unwrap_or_default(),
                        op,
                        -1,
                        false,
                    )
                } else {
                    false
                };
                if !pushed {
                    // The partial walk produced no member token (a whole-symbol
                    // cover): render the bare name, matching `pushPartialSymbol`'s
                    // degenerate base case.
                    self.push_atom(&Atom::with_op(
                        name.clone(),
                        TagType::VarToken,
                        crate::printlanguage::SyntaxHighlight::var_color,
                        op_key(op),
                    ));
                }
            }

            if arrayvalue {
                // push_integer(0, 4, ...) — the `[0]` subscript index.
                self.push_constant_ir(0, 4, op);
            }
        } else {
            // Union/other: functional fallback.
            self.op_func_ir(fd, arch, op);
        }
    }

    /// C++ `isValueFlexible(vn)` (printc.cc:919): the value `vn` is an implied
    /// PTRSUB/PTRADD result (possibly through a COPY) and so can absorb a
    /// dereference.
    fn is_value_flexible_ir(&self, fd: &Funcdata, vn: VarnodeId) -> bool {
        let v = match fd.vbank().get(vn) {
            Some(v) => v,
            None => return false,
        };
        if !(v.is_implied() && v.is_written()) {
            return false;
        }
        let def = match v.get_def() {
            Some(d) => d,
            None => return false,
        };
        let mut opc = fd.obank().get(def).map(|o| o.code()).unwrap_or(OpCode::CPUI_MAX);
        if opc == OpCode::CPUI_COPY {
            let invn = match fd.obank().get(def).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => return false,
            };
            let iv = match fd.vbank().get(invn) {
                Some(v) => v,
                None => return false,
            };
            if !iv.is_implied() || !iv.is_written() {
                return false;
            }
            opc = iv
                .get_def()
                .and_then(|d| fd.obank().get(d).map(|o| o.code()))
                .unwrap_or(OpCode::CPUI_MAX);
        }
        opc == OpCode::CPUI_PTRSUB || opc == OpCode::CPUI_PTRADD
    }

    /// C++ `PrintC::push_integer` leaf for a constant (printc.cc:1360 region),
    /// reduced to [`resolve_integer_format`] + [`format_integer_token`].  No
    /// data-type display-format override (that is the type layer); the default
    /// `val<=10 -> dec` rule reproduces the oracle's `10` rendering.
    ///
    /// (kuna) The persistent integer-format force mods (`option integerformat
    /// dec`/`hex`, printlanguage.cc:705) are honored here so a bare IR constant
    /// follows the same forced-format rule the C++ `push_integer` reads from the
    /// modifier stack (printc.cc:1397-1404).  Without this the `integerformat
    /// dec` datatests (e.g. `divopt.xml`) rendered every divisor in hex.  When
    /// neither force is active the prior `mostNaturalBase` default is preserved.
    /// Render an enumeration constant — the enum arm of C++
    /// `PrintC::pushConstant` (printc.cc:1822/1830) which delegates to
    /// `PrintC::pushEnumConstant` (printc.cc:1735-1756).  `ct->getMatches`
    /// decomposes `val` into a list of flag-name tokens (logically ORed), an
    /// optional bitwise-complement (`~`), and an optional left-shift amount
    /// (the partial-enum `>> 0x20` rendering).  When no representation is
    /// possible (`matchname` empty) it falls back to the raw integer literal,
    /// honoring the enum's display format.
    ///
    /// Faithful transcription of the C++ RPN push order (printc.cc:1741-1755):
    /// `shift_right` op (if shifted), then `bitwise_not` op (if complemented),
    /// then `matchname.size()-1` `enum_cat` (`|`) ops, then the flag-name atoms
    /// in forward order, then — when shifted — the shift-amount integer.  The
    /// direct-recursion engine emits in push order, so the op stack reads
    /// `(name0 | name1 | ...) >> shift` for a shifted-and-ORed representation.
    fn push_enum_constant_ir(&mut self, ct: &crate::dtype::Datatype, val: uintb, op: OpId, vn: VarnodeId) {
        // C++ `ct->getMatches(val, rep)` (printc.cc:1740).  Our `get_matches`
        // returns a Result (the Err is the non-enum invariant); the dispatch
        // only reaches here for an `is_enum_type()` data-type, so a `getMatches`
        // failure means a corrupt enum kind — fall back to the raw integer.
        let rep = match ct.get_matches(val) {
            Ok(rep) => rep,
            Err(_) => {
                self.push_constant_ir_fmt(val, ct.get_size(), op, ct.get_display_format());
                return;
            }
        };
        if !rep.match_name.is_empty() {
            // printc.cc:1742-1743 — `if (rep.shiftAmount != 0) pushOp(&shift_right,op);`
            if rep.shift_amount != 0 {
                self.push_op(&tokens::SHIFT_RIGHT, Some(op_key(op)));
            }
            // printc.cc:1744-1745 — `if (rep.complement) pushOp(&bitwise_not,op);`
            if rep.complement {
                self.push_op(&tokens::BITWISE_NOT, Some(op_key(op)));
            }
            // printc.cc:1746-1747 — `for(i=size-1;i>0;--i) pushOp(&enum_cat,op);`
            // one `|` op per gap between the matched names.
            for _ in 1..rep.match_name.len() {
                self.push_op(&tokens::ENUM_CAT, Some(op_key(op)));
            }
            // printc.cc:1748-1749 — the flag-name atoms in forward order.  The
            // C++ uses `Atom(name,tag,const_color,op,vn,val)` with `tag =
            // vartoken` (the tag pushVnExplicit threaded into pushConstant); for
            // a non-casetoken tag the 6-arg ctor stores the Varnode (not val).
            for name in &rep.match_name {
                self.push_atom(&Atom::with_value(
                    name.clone(),
                    TagType::VarToken,
                    crate::printlanguage::SyntaxHighlight::const_color,
                    op_key(op),
                    vn_key(vn),
                    val,
                ));
            }
            // printc.cc:1750-1751 — `if (rep.shiftAmount != 0)
            // push_integer(rep.shiftAmount,4,false,tag,vn,op,0);` (the `>> 0x20`
            // shift amount, rendered as a 4-byte unsigned literal, no format).
            if rep.shift_amount != 0 {
                self.push_constant_ir_fmt(rep.shift_amount as uintb, 4, op, display_format::NONE);
            }
        } else {
            // printc.cc:1753-1754 — no named representation: the raw integer with
            // the enum's display format.
            self.push_constant_ir_fmt(val, ct.get_size(), op, ct.get_display_format());
        }
    }

    fn push_constant_ir(&mut self, val: uintb, sz: int4, op: OpId) {
        self.push_constant_ir_fmt(val, sz, op, display_format::NONE);
    }

    /// As [`push_constant_ir`](Self::push_constant_ir) but with the caller-resolved
    /// `displayFormat` override (C++ `push_integer`'s `displayFormat` argument,
    /// printc.cc:1360/1394).  A non-`NONE` `display_fmt_in` is the
    /// `vn->getHigh()->getSymbol()->getDisplayFormat()` value (the `force varnode`
    /// equate Symbol); it wins over the `val<=10`/`mostNaturalBase` default exactly
    /// as in [`resolve_integer_format`].
    fn push_constant_ir_fmt(&mut self, val: uintb, sz: int4, op: OpId, display_fmt_in: u32) {
        self.push_constant_ir_fmt_sign(val, sz, op, display_fmt_in, false);
    }

    /// As [`push_constant_ir_fmt`](Self::push_constant_ir_fmt) but threading the
    /// signedness the way C++ `PrintC::pushConstant` (printc.cc:1813) does: it
    /// switches on the constant's read-facing data-type metatype and calls
    /// `push_integer(..., sign, ...)` with `sign = (metatype == TYPE_INT)`
    /// (printc.cc:1832 vs. the `TYPE_UINT`/`TYPE_UNKNOWN` arms at 1824/1835 which
    /// pass `false`).  `push_integer` (printc.cc:1381-1391) then strips a set top
    /// bit into a leading `-` and the two\'s-complement magnitude, so a negative
    /// signed convert/equate constant renders `-512` / `-0xbb8` / `-0333` /
    /// `-0b...` rather than its full unsigned bit pattern.  `force_char` short-
    /// circuits the sign (printc.cc:1381), preserving the `L\'a\'` char convert.
    fn push_constant_ir_fmt_sign(
        &mut self,
        val: uintb,
        sz: int4,
        op: OpId,
        display_fmt_in: u32,
        sign: bool,
    ) {
        let force_dec = self.context.is_set(modifiers::FORCE_DEC);
        let force_hex = self.context.is_set(modifiers::FORCE_HEX);
        let (print_negsign, val, display_fmt) =
            resolve_integer_format(val, sz, sign, display_fmt_in, force_hex, force_dec);
        // C++ `push_integer` (printc.cc:1417) gates the wide-char `L` prefix on
        // `doEmitWideCharPrefix()` (always true for PrintC) AND `sz > 1`.  The
        // earlier port passed `false` here, dropping the `L` from a size>1
        // force_char constant (e.g. the convert `L'a'` equate on a size-4 char).
        let tok = format_integer_token(
            print_negsign,
            val,
            display_fmt,
            sz,
            false,
            false,
            true, // doEmitWideCharPrefix() — PrintC
            "",
        );
        self.push_atom(&Atom::with_op(
            tok,
            TagType::Syntax,
            crate::printlanguage::SyntaxHighlight::const_color,
            op_key(op),
        ));
    }

    /// Render a floating-point constant — the `TYPE_FLOAT` arm of C++
    /// `PrintC::pushConstant` (printc.cc:1859-1861) which delegates to
    /// `PrintC::push_float` (printc.cc:1448-1492).  Decodes the raw encoding `val`
    /// through `glb->translate->getFloatFormat(sz)`
    /// ([`FloatFormat::get_host_float`]/`extract_sign`/`print_decimal`) and emits
    /// the `INFINITY`/`NAN`/decimal token via [`format_float_token`].  When there
    /// is no `FloatFormat` for the size, the token is `FLOAT_UNKNOWN`.
    fn push_float_ir(&mut self, arch: &Architecture, val: uintb, sz: int4, op: OpId) {
        use kuna_num::float::floatclass;
        use kuna_sleigh::translate::Translate;
        let force_scinote = self.context.is_set(modifiers::FORCE_SCINOTE);
        let tok = match arch.translate().get_float_format(sz) {
            None => format_float_token(FloatClass::Unknown, false, "", force_scinote),
            Some(format) => {
                let (floatval, class) = format.get_host_float(val);
                let sign = format.extract_sign(val);
                match class {
                    floatclass::infinity => {
                        format_float_token(FloatClass::Infinity, sign, "", force_scinote)
                    }
                    floatclass::nan => {
                        format_float_token(FloatClass::Nan, sign, "", force_scinote)
                    }
                    // normalized / zero / denormalized all take the printDecimal
                    // path (C++ `push_float` else-branch).
                    _ => {
                        let decimal = format.print_decimal(floatval, force_scinote);
                        format_float_token(FloatClass::Normal, sign, &decimal, force_scinote)
                    }
                }
            }
        };
        self.push_atom(&Atom::with_op(
            tok,
            TagType::Syntax,
            crate::printlanguage::SyntaxHighlight::const_color,
            op_key(op),
        ));
    }

    /// Try to push a quoted string literal for a constant pointer to character
    /// data — C++ `PrintC::pushPtrCharConstant` (printc.cc:1767).  Resolves the
    /// constant pointer to a data-space address, requires the location to be
    /// readonly, then reads/escapes the string via [`Self::print_character_constant`].
    /// Returns `true` only when a literal token was pushed (so the caller can fall
    /// through to the integer print otherwise).
    fn push_ptr_char_constant_ir(
        &mut self,
        arch: &Architecture,
        val: uintb,
        ct: &crate::dtype::Datatype,
        subct: &std::rc::Rc<crate::dtype::Datatype>,
        point: &Address,
        op: OpId,
        vn: VarnodeId,
    ) -> bool {
        // AddrSpace *spc = glb->getDefaultDataSpace();
        let spc = match arch.manage().get_default_data_space() {
            Some(s) => std::rc::Rc::clone(s),
            None => return false,
        };
        // Address stringaddr = glb->resolveConstant(spc,val,ct->getSize(),point,...)
        let mut full_encoding: uintb = 0;
        let stringaddr =
            match arch.resolve_constant(&spc, val, ct.get_size(), point, &mut full_encoding) {
                Ok(a) => a,
                Err(_) => return false,
            };
        if stringaddr.is_invalid() {
            return false;
        }
        // Check that string location is readonly:
        //   glb->symboltab->getGlobalScope()->isReadOnly(stringaddr,1,Address())
        let gscope = match arch.symboltab.get_global_scope() {
            Some(g) => g,
            None => return false,
        };
        let nulladdr = Address::new_invalid();
        if !arch.symboltab.is_read_only(gscope, &stringaddr, 1, &nulladdr) {
            return false;
        }
        // printCharacterConstant(str,stringaddr,subct)
        let mut s = String::new();
        if !self.print_character_constant(arch, &mut s, &stringaddr, subct) {
            return false;
        }
        // pushAtom(Atom(str, vartoken, const_color, op, vn))
        self.push_atom(&Atom::with_op_vn(
            s,
            TagType::VarToken,
            crate::printlanguage::SyntaxHighlight::const_color,
            op_key(op),
            vn_key(vn),
        ));
        true
    }

    /// Render readonly character data at `addr` as a quoted C string literal —
    /// C++ `PrintC::printCharacterConstant` (printc.cc:1602).  Reads the UTF-8
    /// string bytes through the `StringManager` (over the loadimage), emits the
    /// optional `L` wide prefix, then the escaped contents between quotes (with the
    /// truncation marker when the literal was clipped).  Returns `false` when no
    /// string data is available.
    fn print_character_constant(
        &self,
        arch: &Architecture,
        s: &mut String,
        addr: &Address,
        char_type: &std::rc::Rc<crate::dtype::Datatype>,
    ) -> bool {
        use crate::stringmanage::StringManager;
        use kuna_sleigh::translate::Translate;
        // Pull UTF-8 string data through the architecture's persistent
        // `stringManager` (C++ `glb->stringManager`).  Using the shared instance
        // (not a transient one) is what lets `getInternalString`-registered
        // strings — keyed on a constant-space hash address that is *not* in the
        // loadimage — resolve here: `getStringData` returns the cached bytes for a
        // hit and otherwise reads the loadimage exactly as before.
        let loader_rc = arch.translate().loader_rc();
        let mut is_trunc = false;
        let buffer: Vec<u8> = {
            let mut mgr = arch.string_manager.borrow_mut();
            let mut loader = loader_rc.borrow_mut();
            mgr.get_string_data(addr, char_type, &mut **loader, &mut is_trunc)
                .to_vec()
        };
        if buffer.is_empty() {
            return false;
        }
        // doEmitWideCharPrefix() (always true for PrintC) && size>1 && !opaque -> 'L'
        if char_type.get_size() > 1 && !char_type.is_opaque_string() {
            s.push('L');
        }
        s.push('"');
        // escapeCharacterData(s, buffer, len, 1, glb->translate->isBigEndian()):
        // the buffer is already UTF-8 (charsize 1); walk codepoints and re-escape.
        let bigend = arch.translate().is_big_endian();
        let mut i: int4 = 0;
        let count = buffer.len() as int4;
        while i < count {
            let mut skip: int4 = 1;
            let codepoint = StringManager::get_codepoint(&buffer[i as usize..], 1, bigend, &mut skip);
            if codepoint == 0 || codepoint == -1 {
                break;
            }
            print_unicode(s, codepoint);
            i += skip;
        }
        if is_trunc {
            s.push_str("...\" /* TRUNCATED STRING LITERAL */");
        } else {
            s.push('"');
        }
        true
    }
}

/// Head op of an sblocks-arena basic block (when the structured node itself is
/// a Basic, not a Copy referencing bblocks).
fn sblocks_basic_head(fd: &Funcdata, bb: BlockId) -> Option<OpId> {
    use crate::block::BlockKind;
    match fd.sblocks_ref().block(bb).kind() {
        BlockKind::Basic(b) => b.op_head,
        _ => None,
    }
}

/// Tail op of an sblocks-arena basic block.
fn sblocks_basic_tail(fd: &Funcdata, bb: BlockId) -> Option<OpId> {
    use crate::block::BlockKind;
    match fd.sblocks_ref().block(bb).kind() {
        BlockKind::Basic(b) => b.op_tail,
        _ => None,
    }
}

/// The type name to render in a declaration (C++ `Datatype::getName`), with the
/// Ghidra fallback for an unnamed type: a `TYPE_UNKNOWN` of size N (the W8
/// un-inferred base, no real name) renders as `undefined<N>` (or `undefined` for
/// size 1's anonymous form), and a `TYPE_VOID` renders as `void`.  The oracle's
/// inferred names (e.g. `uint1`) need the W8 `ActionInferTypes`; this is the
/// faithful unnamed-type rendering until then.
/// Build the C-declarator front/back text bracketing an identifier for `ct`,
/// transcribing the declarator algorithm of `PrintC::pushTypeStart` /
/// `pushTypeEnd` (printc.cc:265/314) plus `buildTypeStack` (printc.cc:143).
///
/// Returns `(front, back)` such that `<front><name><back>` is the full C
/// declaration of an object named `name` of type `ct` — e.g.
///   * `int8`              → `("int8", "")`             → `int8 a`
///   * `twostruct *`       → `("twostruct *", "")`      → `twostruct *ptr`
///   * `int4 (*)[1]`       → `("int4 (*", ")[1]")`      → `int4 (*a)[1]`
///   * `char *`            → `("char *", "")`           → `char *pchar`
///
/// The stack is built base-up exactly as `buildTypeStack`; pointer modifiers go
/// on the front (`*`), array/function modifiers on the tail (`[N]`/`(...)`), and
/// a `*` front nested inside an array/function tail is parenthesised — the
/// precedence the RPN `ptr_expr`/`array_expr` tokens encode.
pub(crate) fn declarator_parts(ct: &std::rc::Rc<crate::dtype::Datatype>) -> (String, String) {
    use crate::dtype::type_metatype;
    // buildTypeStack: walk to the base (named) type, recording the modifier chain.
    let mut stack: Vec<std::rc::Rc<crate::dtype::Datatype>> = Vec::new();
    let mut cur = std::rc::Rc::clone(ct);
    loop {
        stack.push(std::rc::Rc::clone(&cur));
        if !cur.get_name().is_empty() {
            break; // base type
        }
        let next = match cur.get_metatype() {
            type_metatype::TYPE_PTR => cur.get_ptr_to(),
            type_metatype::TYPE_ARRAY => cur.get_array_base(),
            _ => None, // other anonymous type: stop
        };
        match next {
            Some(n) => cur = n,
            None => break,
        }
    }
    // The base type's display name (anonymous → `undefined<N>` / `void`).
    let base = stack.last().expect("declarator: non-empty stack");
    let base_name = if base.get_name().is_empty() {
        match base.get_metatype() {
            type_metatype::TYPE_VOID => "void".to_string(),
            _ => format!("undefined{}", base.get_size()),
        }
    } else {
        base.get_display_name().to_string()
    };

    // Walk the modifiers from base toward the outermost (stack[len-2]..stack[0]),
    // accumulating front (`*`) and back (`[N]`) declarator pieces.  An array/
    // function tail wraps any pending pointer front in parentheses.
    let mut front = String::new();
    let mut back = String::new();
    let mut pending_ptr = false; // a `*` not yet absorbed by a tail
    for ct_mod in stack.iter().rev().skip(1) {
        match ct_mod.get_metatype() {
            type_metatype::TYPE_PTR => {
                front.push('*');
                pending_ptr = true;
            }
            type_metatype::TYPE_ARRAY => {
                let n = ct_mod.num_elements().unwrap_or_else(|| {
                    let base = ct_mod.get_array_base().map(|b| b.get_size()).unwrap_or(1).max(1);
                    ct_mod.get_size() / base
                });
                if pending_ptr {
                    front.insert(0, '(');
                    back = format!("){}", back);
                    pending_ptr = false;
                }
                back = format!("{}[{}]", back, n);
            }
            _ => {}
        }
    }
    // `<base> <front>` with a single separating space before any `*` modifiers
    // (the `type_expr_space` token); a bare base type has no trailing space here
    // (the caller adds the space before the identifier).
    let front_full = if front.is_empty() { base_name } else { format!("{base_name} {front}") };
    (front_full, back)
}

fn type_name_for_decl(t: &std::rc::Rc<crate::dtype::Datatype>) -> String {
    use crate::dtype::type_metatype;
    let name = t.get_name();
    if !name.is_empty() {
        return name.to_string();
    }
    match t.get_metatype() {
        type_metatype::TYPE_VOID => "void".to_string(),
        _ => format!("undefined{}", t.get_size()),
    }
}

/// SEAM A helper — the kuna stand-in for C++ `Symbol::getFirstWholeMap() != entry`
/// (printc.cc:2697): is there a *whole-symbol* sibling high (the proto-partial
/// ROOT) sharing `name` whose `kuna_symbol_offset == -1`?  A register-returned
/// struct's per-field pieces are all bound to the root's shared name; the root
/// keeps the `-1` whole-symbol-match offset and is the one entry that declares.
/// Per-function names are unique (the `vN` allocator), so the only sibling with a
/// shared name is the group's root, never an unrelated scalar.  `except` is the
/// piece itself (excluded so a lone whole high is not its own sibling).
fn high_name_has_whole_sibling(
    fd: &Funcdata,
    except: crate::seams::HighVariableId,
    name: &str,
) -> bool {
    fd.high_bank().iter().any(|(id, h)| {
        id != except
            && h.kuna_symbol_offset() == -1
            && h.kuna_name() == Some(name)
    })
}

/// Whether `spc` should use the kuna angr-style `dat_<addr>` global naming (a
/// RAM/data space, not the stack).  (kuna) `kunaAngrNaming` gate, printc.cc:1961.
fn kuna_global_naming(spc: &std::rc::Rc<kuna_base::space::AddrSpace>) -> bool {
    use kuna_base::space::spacetype;
    matches!(spc.get_type(), spacetype::IPTR_PROCESSOR)
}

/// (kuna) `kunaGlobalDataName(Address)` — `dat_<hex offset>`.
fn kuna_global_data_name(_spc: &std::rc::Rc<kuna_base::space::AddrSpace>, off: u64) -> String {
    format!("dat_{off:x}")
}

/// A stable per-op key for the `Atom.op` / `ReversePolish.op` slot (the C++
/// `PcodeOp *`).  The driver only needs a non-null marker here; use the op's
/// slotmap index bits.  (Round-trips through `usize`; only identity matters.)
fn op_key(op: OpId) -> usize {
    use slotmap::Key;
    op.data().as_ffi() as usize
}

/// A stable per-varnode key for the `Atom` varnode slot (the C++ `Varnode *`).
fn vn_key(vn: VarnodeId) -> usize {
    use slotmap::Key;
    vn.data().as_ffi() as usize
}

/// C++ `TypeOpFloatInt2Float::absorbZext` (typeop.cc:1874): if the
/// `FLOAT_INT2FLOAT` op's in0 is an implied, written Varnode whose defining op
/// is an `INT_ZEXT`, return that ZEXT op (its source is the real conversion
/// input — the cast's `(floatN)` absorbs the zero-extension).  Otherwise
/// `None`.
fn absorb_zext(fd: &Funcdata, op: OpId) -> Option<OpId> {
    let vn0 = fd.obank().get(op).and_then(|o| o.get_in(0))?;
    let v = fd.vbank().get(vn0)?;
    if v.is_written() && v.is_implied() {
        let zext = v.get_def()?;
        if fd.obank().get(zext).map(|o| o.code()) == Some(OpCode::CPUI_INT_ZEXT) {
            return Some(zext);
        }
    }
    None
}

/// C++ `castStrategy = data.getArch()->print->getCastStrategy()` (the
/// `CastStrategyC` the C printer holds).  Rebuilt here from the bound type
/// factory each time it is needed (the strategy is stateless apart from the
/// factory + `promoteSize = tlst->getSizeOfInt()`, so the rebuild is exact).
fn cast_strategy_for(arch: &Architecture) -> Option<CastStrategyC> {
    let tlst = arch.types_rc() as std::rc::Rc<dyn crate::dtype::TypeFactory>;
    Some(CastStrategyC::new(tlst))
}

/// An immutable [`CastContext`] over `&Funcdata` for the print-time
/// `isExtensionCastImplied` query (C++ the `Varnode *`/`PcodeOp *` the const
/// `CastStrategyC::isExtensionCastImplied` dereferences).
///
/// `isExtensionCastImplied` makes only read-only IR queries, so unlike the
/// cast-insertion-phase [`crate::coreaction_casts::FuncdataCastContext`] (which
/// needs `&mut Funcdata` for the lazy HighVariable recompute and the constant
/// print-flag mutators) this bridge borrows `&Funcdata` and never mutates.  It
/// interns `VarnodeId`/`OpId` behind the opaque [`VnRef`]/[`OpRef`] handles via a
/// `RefCell<Vec<_>>` (index == handle), exactly as `FuncdataCastContext` does, so
/// the handles reproduce C++ pointer identity without a HashMap (clippy-banned).
///
/// Read-facing types resolve through the bare-Varnode accessor (the W10 print
/// convention; by print-time the merged HighVariable type is pinned onto the
/// Varnode). // SEAM(W8 union findResolve)
struct PrintCastContext<'a> {
    fd: &'a Funcdata,
    vn_intern: std::cell::RefCell<Vec<VarnodeId>>,
    op_intern: std::cell::RefCell<Vec<OpId>>,
}

impl<'a> PrintCastContext<'a> {
    fn new(fd: &'a Funcdata) -> PrintCastContext<'a> {
        PrintCastContext {
            fd,
            vn_intern: std::cell::RefCell::new(Vec::new()),
            op_intern: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn vn_ref(&self, vn: VarnodeId) -> VnRef {
        let mut tab = self.vn_intern.borrow_mut();
        if let Some(i) = tab.iter().position(|&k| k == vn) {
            return VnRef(i);
        }
        tab.push(vn);
        VnRef(tab.len() - 1)
    }

    fn op_ref(&self, op: OpId) -> OpRef {
        let mut tab = self.op_intern.borrow_mut();
        if let Some(i) = tab.iter().position(|&k| k == op) {
            return OpRef(i);
        }
        tab.push(op);
        OpRef(tab.len() - 1)
    }

    fn vn_key(&self, vn: VnRef) -> VarnodeId {
        self.vn_intern.borrow()[vn.0]
    }

    fn op_key(&self, op: OpRef) -> OpId {
        self.op_intern.borrow()[op.0]
    }
}

impl CastContext for PrintCastContext<'_> {
    fn op_code(&self, op: OpRef) -> OpCode {
        let op = self.op_key(op);
        self.fd.obank().get(op).expect("print cast ctx: stale op").code()
    }

    fn op_num_input(&self, op: OpRef) -> int4 {
        let op = self.op_key(op);
        self.fd.obank().get(op).expect("print cast ctx: stale op").num_input()
    }

    fn op_in(&self, op: OpRef, slot: int4) -> VnRef {
        let opk = self.op_key(op);
        let vn = self
            .fd
            .obank()
            .get(opk)
            .expect("print cast ctx: stale op")
            .get_in(slot)
            .expect("print cast ctx: missing input slot");
        self.vn_ref(vn)
    }

    fn op_out(&self, op: OpRef) -> Option<VnRef> {
        let opk = self.op_key(op);
        let out = self.fd.obank().get(opk).expect("print cast ctx: stale op").get_out();
        out.map(|vn| self.vn_ref(vn))
    }

    fn op_slot(&self, op: OpRef, vn: VnRef) -> int4 {
        let opk = self.op_key(op);
        let vnk = self.vn_key(vn);
        self.fd.obank().get(opk).expect("print cast ctx: stale op").get_slot(vnk)
    }

    fn vn_is_constant(&self, vn: VnRef) -> bool {
        let vn = self.vn_key(vn);
        self.fd.vbank().get(vn).expect("print cast ctx: stale vn").is_constant()
    }

    fn vn_is_explicit(&self, vn: VnRef) -> bool {
        let vn = self.vn_key(vn);
        self.fd.vbank().get(vn).expect("print cast ctx: stale vn").is_explicit()
    }

    fn vn_is_written(&self, vn: VnRef) -> bool {
        let vn = self.vn_key(vn);
        self.fd.vbank().get(vn).expect("print cast ctx: stale vn").is_written()
    }

    fn vn_size(&self, vn: VnRef) -> int4 {
        let vn = self.vn_key(vn);
        self.fd.vbank().get(vn).expect("print cast ctx: stale vn").get_size()
    }

    fn vn_offset(&self, vn: VnRef) -> uintb {
        let vn = self.vn_key(vn);
        self.fd.vbank().get(vn).expect("print cast ctx: stale vn").get_offset()
    }

    fn vn_def(&self, vn: VnRef) -> Option<OpRef> {
        let vn = self.vn_key(vn);
        let def = self.fd.vbank().get(vn).expect("print cast ctx: stale vn").get_def();
        def.map(|op| self.op_ref(op))
    }

    fn vn_lone_descend(&self, vn: VnRef) -> Option<OpRef> {
        let vnk = self.vn_key(vn);
        self.fd.lone_descend(vnk).map(|op| self.op_ref(op))
    }

    fn vn_high_type(&self, vn: VnRef) -> std::rc::Rc<crate::dtype::Datatype> {
        let vnk = self.vn_key(vn);
        // Bare-Varnode type (the W10 print convention; high type pinned by
        // print-time). // SEAM(W8 union findResolve)
        self.fd.vbank().get(vnk).expect("print cast ctx: stale vn").get_type().clone()
    }

    fn vn_high_type_read_facing(&self, vn: VnRef, op: OpRef) -> std::rc::Rc<crate::dtype::Datatype> {
        let vnk = self.vn_key(vn);
        let opk = self.op_key(op);
        // vn->getHighTypeReadFacing(op): bare read-facing type by print-time.
        // // SEAM(W8 union findResolve)
        self.fd
            .vbank()
            .get(vnk)
            .expect("print cast ctx: stale vn")
            .get_type_read_facing(opk)
            .clone()
    }

    fn op_inherits_sign(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.op_code(op)).inherits_sign()
    }

    fn op_inherits_sign_first_param_only(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.op_code(op)).inherits_sign_first_param_only()
    }

    fn op_is_shift_op(&self, op: OpRef) -> bool {
        crate::typeop::type_op_info(self.op_code(op)).is_shift_op()
    }

    fn op_is_bool_output(&self, op: OpRef) -> bool {
        let opk = self.op_key(op);
        self.fd.obank().get(opk).expect("print cast ctx: stale op").is_bool_output()
    }

    fn op_is_call(&self, op: OpRef) -> bool {
        let opk = self.op_key(op);
        self.fd.obank().get(opk).expect("print cast ctx: stale op").is_call()
    }

    fn vn_set_unsigned_print(&mut self, _vn: VnRef) {
        // Only reached by `mark_explicit_unsigned`/`mark_explicit_long_size`, which
        // the print-time `isExtensionCastImplied` query never calls.  The immutable
        // print path holds no `&mut Funcdata`, so this is unreachable here.
        unreachable!("PrintCastContext is read-only: vn_set_unsigned_print not used by isExtensionCastImplied");
    }

    fn vn_set_long_print(&mut self, _vn: VnRef) {
        unreachable!("PrintCastContext is read-only: vn_set_long_print not used by isExtensionCastImplied");
    }
}

/// The functional print name for an opcode (C++ the `TypeOp::getOperatorName`
/// uppercase form used by `opFunc`).  Faithful for the common functional ops;
/// falls back to the raw opcode name otherwise.
fn opcode_print_name(opc: OpCode) -> String {
    use OpCode::*;
    match opc {
        CPUI_INT_ZEXT => "ZEXT".to_string(),
        CPUI_INT_SEXT => "SEXT".to_string(),
        CPUI_PIECE => "CONCAT".to_string(),
        CPUI_SUBPIECE => "SUB".to_string(),
        CPUI_INT_CARRY => "CARRY".to_string(),
        CPUI_INT_SCARRY => "SCARRY".to_string(),
        CPUI_INT_SBORROW => "SBORROW".to_string(),
        CPUI_POPCOUNT => "POPCOUNT".to_string(),
        CPUI_LZCOUNT => "LZCOUNT".to_string(),
        CPUI_FLOAT_NAN => "NAN".to_string(),
        CPUI_FLOAT_ABS => "ABS".to_string(),
        CPUI_FLOAT_SQRT => "SQRT".to_string(),
        other => format!("{other:?}").trim_start_matches("CPUI_").to_string(),
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
