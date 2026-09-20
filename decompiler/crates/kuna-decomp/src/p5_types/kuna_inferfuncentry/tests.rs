//! Logic-level tests for `kuna_is_function_entry` (GH-6930), exercising the
//! exact-function-entry decision on hand-built resolution facts.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use super::*;

fn ram() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn addr(space: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(space), off)
}

#[test]
fn gate_off_returns_false() {
    let ram = ram();
    let p = addr(&ram, 0x10_0000);
    // gate=false: upstream byte-identical, never fires (even on an exact match).
    assert!(!kuna_is_function_entry(false, Some(&p), Some(&p)));
}

#[test]
fn exact_function_entry_fires() {
    let ram = ram();
    let p = addr(&ram, 0x10_0000); // 1<<20, a single-bit image base
                                   // resolveConstant gave rampoint == 0x100000, and a function lives
                                   // exactly there.
    assert!(kuna_is_function_entry(true, Some(&p), Some(&p)));
}

#[test]
fn invalid_rampoint_returns_false() {
    let ram = ram();
    let p = addr(&ram, 0x10_0000);
    // resolveConstant returned the invalid address: the constant is not a ptr.
    assert!(!kuna_is_function_entry(true, None, Some(&p)));
    // An explicitly-invalid Address is likewise rejected.
    assert!(!kuna_is_function_entry(true, Some(&Address::new_invalid()), Some(&p)));
}

#[test]
fn no_function_at_rampoint_returns_false() {
    let ram = ram();
    let p = addr(&ram, 0x10_0000);
    // resolveConstant succeeded but no function is registered there:
    // an ordinary single-bit integer constant, left as a literal.
    assert!(!kuna_is_function_entry(true, Some(&p), None));
}

#[test]
fn function_not_at_exact_entry_returns_false() {
    let ram = ram();
    let rampoint = addr(&ram, 0x10_0004); // inside a function, not its entry
    let entry = addr(&ram, 0x10_0000); // the function's actual entry
                                       // queryFunction returns the enclosing function, but its entry differs
                                       // from rampoint -> the C++ `fd->getAddress() == rampoint` fails.
    assert!(!kuna_is_function_entry(true, Some(&rampoint), Some(&entry)));
}

#[test]
fn option_default_is_on_and_apply_flips() {
    // Shipped default: option inferfuncentry on (DIV-2 default-on).
    let mut opt = InferFuncEntryOption::default();
    assert!(opt.is_enabled());
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
}

#[test]
fn escape_applies_where_a_code_address_can_appear() {
    for opcode in [
        OpCode::CPUI_CALL,
        OpCode::CPUI_CALLIND,
        OpCode::CPUI_COPY,
        OpCode::CPUI_STORE,
        OpCode::CPUI_INT_ADD,
        OpCode::CPUI_INT_EQUAL,
        OpCode::CPUI_INT_NOTEQUAL,
    ] {
        assert!(entry_escape_applies(opcode, false), "{opcode:?} should keep the escape");
    }
}

#[test]
fn escape_declines_a_reader_that_is_arithmetic() {
    for opcode in [
        OpCode::CPUI_INT_LESS,
        OpCode::CPUI_INT_LESSEQUAL,
        OpCode::CPUI_INT_SLESS,
        OpCode::CPUI_INT_SLESSEQUAL,
        OpCode::CPUI_INT_MULT,
        OpCode::CPUI_INT_DIV,
        OpCode::CPUI_INT_SDIV,
        OpCode::CPUI_INT_REM,
        OpCode::CPUI_INT_SREM,
        OpCode::CPUI_INT_LEFT,
        OpCode::CPUI_INT_RIGHT,
        OpCode::CPUI_INT_SRIGHT,
    ] {
        assert!(reads_as_integer(opcode), "{opcode:?} reads its constant as a number");
        assert!(!entry_escape_applies(opcode, false), "{opcode:?} should decline the escape");
    }
}

#[test]
fn escape_declines_a_value_the_function_orders_elsewhere() {
    // coreutils tail: the COPY arm of MIN (n_remaining, BUFSIZ) is the same shape
    // as a function-pointer table's `v = handler`, and the only thing that tells
    // them apart is that one of the two values is also compared as a number.
    assert!(entry_escape_applies(OpCode::CPUI_COPY, false));
    assert!(!entry_escape_applies(OpCode::CPUI_COPY, true));
    assert!(!entry_escape_applies(OpCode::CPUI_CALL, true));
}
