//! (kuna `unitcalls`) The p-code helpers and user-defined operations a
//! function's C calls by name, so a project header can declare each one.
//!
//! The printer spells an op C has no operator for as a call to a helper named
//! after it and its operand sizes (`SCARRY4(a,b)`, `SUB168(x,8)`,
//! `CONCAT44(hi,lo)`), and a SLEIGH user-defined operation as a call to its
//! name (`invalidInstructionException()`). Nothing declared them, so a unit
//! that named one did not compile. Each is declared with no fixed parameters,
//! `extern T name(KUNA_ANY_ARGS);`, because the printed arguments are whatever
//! the binary passed and a prototype would reject a faithful call. The
//! recompile prelude defines `KUNA_ANY_ARGS` as `...` where `()` means no
//! parameters (C23, and the c2x modes of gcc 13 and clang 16 on) and as nothing
//! before. `T` is the op's output, or `void` without one; a name a function
//! calls both with and without an output, or at two output sizes, takes the
//! widest.
//!
//! The printer records each call on its thread while it prints a function and
//! forgets them when it starts the next one, so [`take`] answers for the
//! function this thread printed last; [`crate::kuna_callprototypes::declarations`]
//! calls it right after the print.
//!
//! Limits: an output with no `undefinedN` or float spelling (9 or 12 bytes) is
//! not declared, and the unit still does not compile; a user-defined operation
//! named like a C library function (`syscall`) clashes with that library's
//! header if the unit includes it; and the widest output is chosen per
//! function, so two functions that call one user-defined operation at two
//! output sizes, or one with its output and one without, get two declarations,
//! which the header reports as disagreeing and leaves undeclared.

use std::cell::RefCell;
use std::collections::BTreeMap;

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
    let mut bytes = name.bytes();
    let identifier = bytes.next().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_');
    if !identifier {
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
    returns_of(code, out.map(|out| fd.vbank().get(out).map_or(0, |vn| vn.get_size())))
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
    fn a_call_is_declared_with_the_type_of_its_output() {
        use OpCode::*;
        let cases = [
            (CPUI_INT_SCARRY, Some(1), Some("bool")),
            (CPUI_FLOAT_NAN, Some(1), Some("bool")),
            (CPUI_SUBPIECE, Some(8), Some("undefined8")),
            (CPUI_PIECE, Some(16), Some("undefined16")),
            (CPUI_CALLOTHER, Some(3), Some("undefined3")),
            (CPUI_CALLOTHER, None, Some("void")),
            (CPUI_FLOAT_SQRT, Some(8), Some("double")),
            (CPUI_FLOAT_ROUND, Some(10), Some("long double")),
            (CPUI_CALLOTHER, Some(12), None),
            (CPUI_FLOAT_ABS, Some(2), None),
        ];
        for (code, size, ty) in cases {
            assert_eq!(returns_of(code, size).map(|(_, ty)| ty), ty, "{code:?} {size:?}");
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
