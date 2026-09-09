//! Logic-level tests for the option parse and the ABI table.

use super::*;

#[test]
fn option_parse_accepts_the_three_values() {
    assert_eq!(OptionX64Syscall.apply("off").unwrap().0, X64SyscallMode::Off);
    assert_eq!(OptionX64Syscall.apply("on").unwrap().0, X64SyscallMode::On);
    assert_eq!(OptionX64Syscall.apply("abi").unwrap().0, X64SyscallMode::Abi);
}

#[test]
fn option_parse_rejects_anything_else() {
    for bad in ["", "yes", "ON", "all", "1"] {
        assert!(OptionX64Syscall.apply(bad).is_err(), "accepted {bad:?}");
    }
}

#[test]
fn mode_predicates() {
    assert!(!X64SyscallMode::Off.rewrites());
    assert!(X64SyscallMode::On.rewrites());
    assert!(X64SyscallMode::Abi.rewrites());
    assert!(!X64SyscallMode::On.takes_all_args());
    assert!(X64SyscallMode::Abi.takes_all_args());
}

#[test]
fn default_is_off() {
    assert_eq!(X64SyscallMode::default(), X64SyscallMode::Off);
}

#[test]
fn abi_table_is_the_kernel_order_and_excludes_rcx() {
    // RCX carries the return address the instruction itself writes, which is why
    // the kernel ABI uses R10 where the C ABI would use RCX.
    assert_eq!(ARG_REGISTERS, ["RDI", "RSI", "RDX", "R10", "R8", "R9"]);
    assert!(!ARG_REGISTERS.contains(&"RCX"));
    assert_eq!(NUM_REGISTER, "RAX");
}

#[test]
fn element_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_X64SYSCALL.get_id(), 4161);
    assert_eq!(ELEM_X64SYSCALL.get_name(), "x64syscall");
}
