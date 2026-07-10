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
