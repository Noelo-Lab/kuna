//! (kuna) The p-code helpers and user-defined operations a function's C calls
//! by name, so a project header can declare each one.
//!
//! The printer spells three things as calls nothing declared, so a unit that
//! named one did not compile: an op C has no operator for, as a helper named
//! after it and its operand sizes (`SCARRY4(a,b)`, `SUB168(x,8)`,
//! `CONCAT44(hi,lo)`); a SLEIGH user-defined operation, by its name
//! (`invalidInstructionException()`); and a RETURN the engine planted where the
//! bytes would not decode, by the failure (`halt_baddata()`). Each is declared
//! with no fixed parameters, `extern T name(KUNA_ANY_ARGS);`, because the
//! printed arguments are whatever the binary passed and a prototype would
//! reject a faithful call. The recompile prelude defines `KUNA_ANY_ARGS` as
//! `...` where `()` means no parameters (C23, and the c2x modes of gcc 13 and
//! clang 16 on) and as nothing before. `T` is the op's output, spelled as the
//! float of its width where the output is a float, or `void` without one; a
//! name a function calls both with and without an output, or at two output
//! sizes, takes the widest.
//!
//! The printer keeps one [`Calls`] for the function it is printing, starts it
//! empty with each function and answers it from `PrintC::unit_calls`, which
//! [`crate::kuna_callprototypes::declarations`] reads to put the declarations
//! with the function's other call declarations.
//!
//! The header writes them in a block of their own ([`header_block`]), one per
//! name across every function at the widest type a call has, and only when the
//! including code defines `KUNA_DECLARE_HELPERS`: code that compiles an export
//! often brings its own definitions (a `CONCAT44` macro, `static inline` x87
//! conversions, `<unistd.h>`'s `syscall`), which a declaration would contradict.
//! Each one is also skipped where the including code defined its name as a
//! macro, and where the header already declares the name.
//!
//! Limits:
//! - An output with no `undefinedN` or float spelling (9 or 12 bytes) is not
//!   declared, and neither are the float constants the printer spells `NAN`,
//!   `INFINITY` and `FLOAT_UNKNOWN`, so a unit that holds one still does not
//!   compile.
//! - A name that is a C keyword (MIPS's `break`) is not declared; the call the
//!   body prints still does not compile.
//! - Code that defines `KUNA_DECLARE_HELPERS` and also includes a C library
//!   header that declares an operation's name (`syscall`, `sin`, `sleep`,
//!   `wait`) gets two declarations that disagree.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;

/// A declared return type and the output size it stands for (`-1` for
/// `void`), by which the widest of two calls is kept.
type Returns = (i32, &'static str);

/// The words C reserves, which no declaration can use as a name.
const C_KEYWORDS: &[&str] = &[
    "alignas", "alignof", "asm", "auto", "bool", "break", "case", "char", "const", "constexpr",
    "continue", "default", "do", "double", "else", "enum", "extern", "false", "float", "for",
    "goto", "if", "inline", "int", "long", "nullptr", "register", "restrict", "return", "short",
    "signed", "sizeof", "static", "static_assert", "struct", "switch", "thread_local", "true",
    "typedef", "typeof", "typeof_unqual", "union", "unsigned", "void", "volatile", "while",
    "_Alignas", "_Alignof", "_Atomic", "_BitInt", "_Bool", "_Complex", "_Decimal128",
    "_Decimal32", "_Decimal64", "_Generic", "_Imaginary", "_Noreturn", "_Static_assert",
    "_Thread_local",
];

/// The helpers and operations one function's C calls, by name, each at the
/// widest declared type a call has.
#[derive(Default)]
pub struct Calls(BTreeMap<String, Returns>);

impl Calls {
    /// Record that the C calls `name` for `op`. A name that is not a C
    /// identifier, such as the `CALLOTHER[0x5]` fallback, a C keyword, or an
    /// output C has no type for, is left out.
    pub fn record(&mut self, fd: &Funcdata, op: OpId, name: &str) {
        if !declarable(name) {
            return;
        }
        if let Some(returns) = fd.obank().get(op).and_then(|o| returns(fd, o.code(), o.get_out())) {
            self.note(name, returns);
        }
    }

    /// One `T name(KUNA_ANY_ARGS);` per name recorded.
    pub fn declarations(&self) -> Vec<String> {
        self.0.iter().map(|(name, (_, ty))| format!("{ty} {name}(KUNA_ANY_ARGS);")).collect()
    }

    fn note(&mut self, name: &str, returns: Returns) {
        match self.0.get_mut(name) {
            Some(kept) if returns.0 > kept.0 => *kept = returns,
            Some(_) => {}
            None => {
                self.0.insert(name.to_owned(), returns);
            }
        }
    }
}

/// Whether `declaration` is one [`Calls::declarations`] made.
pub fn is_helper(declaration: &str) -> bool {
    declaration.trim_end().ends_with("(KUNA_ANY_ARGS);")
}

/// The header's block of the helpers the `declarations` of every function
/// hold: one `extern` per name, at the widest type a call has, under
/// `#ifdef KUNA_DECLARE_HELPERS` and `#ifndef <name>`, skipping each name
/// `declared` says the header already declares. Empty when there is none.
pub fn header_block<'d>(declarations: impl IntoIterator<Item = &'d str>, declared: impl Fn(&str) -> bool) -> String {
    let mut widest: BTreeMap<&str, (i32, &str)> = BTreeMap::new();
    for declaration in declarations.into_iter().map(str::trim).filter(|d| is_helper(d)) {
        let Some((ty, name)) = declaration.trim_end_matches("(KUNA_ANY_ARGS);").rsplit_once(' ') else {
            continue;
        };
        if declared(name) {
            continue;
        }
        let rank = rank_of(ty);
        let kept = widest.entry(name).or_insert((rank, ty));
        if rank > kept.0 {
            *kept = (rank, ty);
        }
    }
    if widest.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "\n/* the p-code helpers, user-defined operations and halts the code calls:\n   \
         define KUNA_DECLARE_HELPERS to declare each one the including code has not\n   \
         defined as a macro */\n#ifdef KUNA_DECLARE_HELPERS\n",
    );
    for (name, (_, ty)) in widest {
        let _ = write!(out, "#ifndef {name}\nextern {ty} {name}(KUNA_ANY_ARGS);\n#endif\n");
    }
    out.push_str("#endif\n");
    out
}

/// The output width a declared type stands for, by which the widest of two is
/// kept: [`returns_of`]'s spellings.
fn rank_of(ty: &str) -> i32 {
    match ty {
        "void" => -1,
        "bool" => 1,
        "float" => 4,
        "double" => 8,
        "long double" => 16,
        _ => ty.strip_prefix("undefined").and_then(|n| n.parse().ok()).unwrap_or(0),
    }
}

fn declarable(name: &str) -> bool {
    is_identifier(name) && !C_KEYWORDS.contains(&name)
}

fn is_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn returns(fd: &Funcdata, code: OpCode, out: Option<VarnodeId>) -> Option<Returns> {
    let out = out.and_then(|out| fd.vbank().get(out));
    let float = out.is_some_and(|vn| vn.get_type().get_metatype() == type_metatype::TYPE_FLOAT);
    returns_of(code, out.map(|vn| vn.get_size()), float)
}

/// The declared type of a call to `code`'s helper or operation with an output
/// of `size` bytes, or with none; `float` says the output is typed a float
/// (`FLOAT_RINT`).
fn returns_of(code: OpCode, size: Option<i32>, float: bool) -> Option<Returns> {
    use OpCode::*;
    let Some(size) = size else {
        return Some((-1, "void"));
    };
    if matches!(code, CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_FLOAT_NAN) {
        return Some((size, "bool"));
    }
    let ty = if float || matches!(
        code,
        CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT | CPUI_FLOAT_CEIL | CPUI_FLOAT_FLOOR | CPUI_FLOAT_ROUND
    ) {
        match size {
            4 => "float",
            8 => "double",
            10 | 16 => "long double",
            _ => return None,
        }
    } else {
        match size {
            1 => "undefined1",
            2 => "undefined2",
            3 => "undefined3",
            4 => "undefined4",
            5 => "undefined5",
            6 => "undefined6",
            7 => "undefined7",
            8 => "undefined8",
            16 => "undefined16",
            32 => "undefined32",
            _ => return None,
        }
    };
    Some((size, ty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_called_with_and_without_an_output_is_declared_with_the_widest() {
        let mut calls = Calls::default();
        calls.note("rdtsc", (-1, "void"));
        calls.note("rdtsc", (8, "undefined8"));
        calls.note("rdtsc", (4, "undefined4"));
        calls.note("LOCK", (-1, "void"));

        assert_eq!(
            calls.declarations(),
            ["void LOCK(KUNA_ANY_ARGS);", "undefined8 rdtsc(KUNA_ANY_ARGS);"]
        );
        assert_eq!(calls.declarations().len(), 2, "reading the set leaves it in place");
        assert!(Calls::default().declarations().is_empty());
    }

    #[test]
    fn the_header_declares_each_helper_once_at_its_widest_under_the_opt_in() {
        let declarations = [
            "void rdtsc(KUNA_ANY_ARGS);",
            "bool SCARRY4(KUNA_ANY_ARGS);",
            "undefined8 rdtsc(KUNA_ANY_ARGS);",
            "undefined4 rdtsc(KUNA_ANY_ARGS);",
            "long double FLOOR(KUNA_ANY_ARGS);",
            "int puts(char *a0);",
            "void syscall(KUNA_ANY_ARGS);",
        ];

        let block = header_block(declarations, |name| name == "syscall");

        assert_eq!(
            block,
            "\n/* the p-code helpers, user-defined operations and halts the code calls:\n   \
             define KUNA_DECLARE_HELPERS to declare each one the including code has not\n   \
             defined as a macro */\n#ifdef KUNA_DECLARE_HELPERS\n\
             #ifndef FLOOR\nextern long double FLOOR(KUNA_ANY_ARGS);\n#endif\n\
             #ifndef SCARRY4\nextern bool SCARRY4(KUNA_ANY_ARGS);\n#endif\n\
             #ifndef rdtsc\nextern undefined8 rdtsc(KUNA_ANY_ARGS);\n#endif\n\
             #endif\n"
        );
        assert!(is_helper(" bool SCARRY4(KUNA_ANY_ARGS); ") && !is_helper("int puts(char *a0);"));
        assert_eq!(header_block(["int puts(char *a0);"], |_| false), "");
    }

    #[test]
    fn only_a_c_identifier_that_is_not_a_keyword_is_declared() {
        for name in ["SCARRY4", "halt_baddata", "_x", "a1", "breakpoint", "syscall"] {
            assert!(declarable(name), "{name}");
        }
        for name in ["CALLOTHER[0x5]", "r#match", "1a", "", "a-b", "break", "asm", "_Bool", "typeof"] {
            assert!(!declarable(name), "{name}");
        }
    }

    #[test]
    fn a_call_is_declared_with_the_type_of_its_output() {
        use OpCode::*;
        let cases = [
            (CPUI_INT_CARRY, Some(1), false, Some((1, "bool"))),
            (CPUI_INT_SCARRY, Some(1), false, Some((1, "bool"))),
            (CPUI_INT_SBORROW, Some(1), false, Some((1, "bool"))),
            (CPUI_FLOAT_NAN, Some(1), false, Some((1, "bool"))),
            (CPUI_INT_ZEXT, Some(1), false, Some((1, "undefined1"))),
            (CPUI_INT_SEXT, Some(2), false, Some((2, "undefined2"))),
            (CPUI_CALLOTHER, Some(3), false, Some((3, "undefined3"))),
            (CPUI_POPCOUNT, Some(4), false, Some((4, "undefined4"))),
            (CPUI_CALLOTHER, Some(5), false, Some((5, "undefined5"))),
            (CPUI_CALLOTHER, Some(6), false, Some((6, "undefined6"))),
            (CPUI_CALLOTHER, Some(7), false, Some((7, "undefined7"))),
            (CPUI_SUBPIECE, Some(8), false, Some((8, "undefined8"))),
            (CPUI_PIECE, Some(16), false, Some((16, "undefined16"))),
            (CPUI_CALLOTHER, Some(32), false, Some((32, "undefined32"))),
            (CPUI_CALLOTHER, None, false, Some((-1, "void"))),
            (CPUI_FLOAT_ABS, Some(4), false, Some((4, "float"))),
            (CPUI_FLOAT_SQRT, Some(8), false, Some((8, "double"))),
            (CPUI_FLOAT_ROUND, Some(10), false, Some((10, "long double"))),
            (CPUI_FLOAT_CEIL, Some(16), false, Some((16, "long double"))),
            (CPUI_FLOAT_FLOOR, Some(8), false, Some((8, "double"))),
            (CPUI_CALLOTHER, Some(12), false, None),
            (CPUI_FLOAT_ABS, Some(2), false, None),
            (CPUI_CALLOTHER, Some(8), true, Some((8, "double"))),
            (CPUI_CALLOTHER, Some(4), true, Some((4, "float"))),
            (CPUI_CALLOTHER, None, true, Some((-1, "void"))),
        ];
        for (code, size, float, returns) in cases {
            assert_eq!(returns_of(code, size, float), returns, "{code:?} {size:?} {float}");
        }
    }
}
