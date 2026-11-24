//! (kuna `unitcalls`) The p-code helpers and user-defined operations a
//! function's C calls by name, so a project header can declare each one.
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
//! clang 16 on) and as nothing before. `T` is the op's output, or `void`
//! without one; a name a function calls both with and without an output, or at
//! two output sizes, takes the widest.
//!
//! The printer records each call on its thread while it prints a function and
//! forgets them when it starts the next one, so [`take`] answers for the
//! function this thread printed last; [`crate::kuna_callprototypes::declarations`]
//! calls it right after the print, which puts the declarations with the
//! function's other call declarations.
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
//! - Code that defines `KUNA_DECLARE_HELPERS` and also includes a C library
//!   header that declares an operation's name (`syscall`) gets two declarations
//!   that disagree.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write as _;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// A declared return type and the output size it stands for (`-1` for
/// `void`), by which the widest of two calls is kept.
type Returns = (i32, &'static str);

thread_local! {
    static CALLED: RefCell<BTreeMap<String, Returns>> = const { RefCell::new(BTreeMap::new()) };
}

/// Forget what was recorded: the printer starts a new function.
pub fn forget() {
    CALLED.with(|called| called.borrow_mut().clear());
}

/// Record that the C calls `name` for `op`. A name that is not a C identifier,
/// such as the `CALLOTHER[0x5]` fallback, or an output C has no type for, is
/// left out.
pub fn record(fd: &Funcdata, op: OpId, name: &str) {
    if !is_identifier(name) {
        return;
    }
    if let Some(returns) = fd.obank().get(op).and_then(|o| returns(fd, o.code(), o.get_out())) {
        note(name, returns);
    }
}

/// The declarations recorded since the printer started the function, each
/// `T name(KUNA_ANY_ARGS);`, which are then forgotten.
pub fn take() -> Vec<String> {
    CALLED
        .with(|called| std::mem::take(&mut *called.borrow_mut()))
        .into_iter()
        .map(|(name, (_, ty))| format!("{ty} {name}(KUNA_ANY_ARGS);"))
        .collect()
}

/// Whether `declaration` is one [`take`] made.
pub fn is_helper(declaration: &str) -> bool {
    declaration.trim().ends_with("(KUNA_ANY_ARGS);")
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

fn is_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn note(name: &str, returns: Returns) {
    CALLED.with(|called| {
        let mut called = called.borrow_mut();
        let kept = called.entry(name.to_owned()).or_insert(returns);
        if returns.0 > kept.0 {
            *kept = returns;
        }
    });
}

fn returns(fd: &Funcdata, code: OpCode, out: Option<VarnodeId>) -> Option<Returns> {
    returns_of(code, out.and_then(|out| fd.vbank().get(out)).map(|vn| vn.get_size()))
}

/// The declared type of a call to `code`'s helper or operation with an output
/// of `size` bytes, or with none.
fn returns_of(code: OpCode, size: Option<i32>) -> Option<Returns> {
    use OpCode::*;
    let Some(size) = size else {
        return Some((-1, "void"));
    };
    if matches!(code, CPUI_INT_CARRY | CPUI_INT_SCARRY | CPUI_INT_SBORROW | CPUI_FLOAT_NAN) {
        return Some((size, "bool"));
    }
    let ty = if matches!(
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
        forget();
        note("rdtsc", (-1, "void"));
        note("rdtsc", (8, "undefined8"));
        note("rdtsc", (4, "undefined4"));
        note("LOCK", (-1, "void"));

        assert_eq!(
            take(),
            ["void LOCK(KUNA_ANY_ARGS);", "undefined8 rdtsc(KUNA_ANY_ARGS);"]
        );
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
    fn only_a_c_identifier_is_declared() {
        for name in ["SCARRY4", "halt_baddata", "_x", "a1"] {
            assert!(is_identifier(name), "{name}");
        }
        for name in ["CALLOTHER[0x5]", "r#match", "1a", "", "a-b"] {
            assert!(!is_identifier(name), "{name}");
        }
    }

    #[test]
    fn a_call_is_declared_with_the_type_of_its_output() {
        use OpCode::*;
        let cases = [
            (CPUI_INT_CARRY, Some(1), Some((1, "bool"))),
            (CPUI_INT_SCARRY, Some(1), Some((1, "bool"))),
            (CPUI_INT_SBORROW, Some(1), Some((1, "bool"))),
            (CPUI_FLOAT_NAN, Some(1), Some((1, "bool"))),
            (CPUI_INT_ZEXT, Some(1), Some((1, "undefined1"))),
            (CPUI_INT_SEXT, Some(2), Some((2, "undefined2"))),
            (CPUI_CALLOTHER, Some(3), Some((3, "undefined3"))),
            (CPUI_POPCOUNT, Some(4), Some((4, "undefined4"))),
            (CPUI_CALLOTHER, Some(5), Some((5, "undefined5"))),
            (CPUI_CALLOTHER, Some(6), Some((6, "undefined6"))),
            (CPUI_CALLOTHER, Some(7), Some((7, "undefined7"))),
            (CPUI_SUBPIECE, Some(8), Some((8, "undefined8"))),
            (CPUI_PIECE, Some(16), Some((16, "undefined16"))),
            (CPUI_CALLOTHER, Some(32), Some((32, "undefined32"))),
            (CPUI_CALLOTHER, None, Some((-1, "void"))),
            (CPUI_FLOAT_ABS, Some(4), Some((4, "float"))),
            (CPUI_FLOAT_SQRT, Some(8), Some((8, "double"))),
            (CPUI_FLOAT_ROUND, Some(10), Some((10, "long double"))),
            (CPUI_FLOAT_CEIL, Some(16), Some((16, "long double"))),
            (CPUI_FLOAT_FLOOR, Some(8), Some((8, "double"))),
            (CPUI_CALLOTHER, Some(12), None),
            (CPUI_FLOAT_ABS, Some(2), None),
        ];
        for (code, size, returns) in cases {
            assert_eq!(returns_of(code, size), returns, "{code:?} {size:?}");
        }
    }

    #[test]
    fn a_function_takes_only_what_was_recorded_since_its_print_started() {
        forget();
        note("SCARRY4", (1, "bool"));
        forget();
        note("SUB168", (8, "undefined8"));

        assert_eq!(take(), ["undefined8 SUB168(KUNA_ANY_ARGS);"]);
        assert_eq!(take(), Vec::<String>::new(), "take forgets what it answered");
    }
}
