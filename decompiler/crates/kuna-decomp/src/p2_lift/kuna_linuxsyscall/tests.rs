//! Table tests for [`super`].
//!
//! The pass itself is exercised end to end by `tests/stages/kuna-linuxsyscall.xml`
//! (option off = the `swi` indirect call, on = the named syscall); what is worth
//! pinning here is the table the rewrite trusts, since a wrong arity is the one
//! way this feature can state something false.

use super::{constant_syscall_number, syscall_entry, ARG_REGISTERS, SYSCALL_TABLE};

#[test]
fn syscall_number_widths_are_exact_and_the_compat_lane_is_low_32_bits() {
    assert_eq!(constant_syscall_number(1, 4, 4, 4), Some(1));
    assert_eq!(constant_syscall_number(u32::MAX as u64, 4, 4, 4), Some(u32::MAX));
    assert_eq!(constant_syscall_number(u32::MAX as u64 + 1, 4, 4, 4), None);

    assert_eq!(constant_syscall_number(0x1234_5678_0000_0001, 8, 8, 8), Some(1));
    assert_eq!(constant_syscall_number(u64::MAX, 8, 8, 8), Some(u32::MAX));

    // A malformed COPY, an arbitrary slice, and an eight-byte i386 write are
    // all refusals rather than implicit truncations.
    assert_eq!(constant_syscall_number(1, 4, 8, 8), None);
    assert_eq!(constant_syscall_number(1, 8, 4, 8), None);
    assert_eq!(constant_syscall_number(1, 2, 2, 8), None);
    assert_eq!(constant_syscall_number(1, 8, 8, 4), None);
}

#[test]
fn table_is_sorted_and_within_the_abi_register_budget() {
    let mut prev: Option<u32> = None;
    for &(num, name, argc) in SYSCALL_TABLE {
        assert!(prev.map(|p| p < num).unwrap_or(true), "table out of order at {num} ({name})");
        assert!(!name.is_empty(), "empty name at {num}");
        assert!(
            (argc as usize) <= ARG_REGISTERS.len(),
            "{name} ({num}) wants {argc} arguments, the i386 ABI passes at most {}",
            ARG_REGISTERS.len()
        );
        prev = Some(num);
    }
}

#[test]
fn the_witnessed_numbers_resolve_to_their_kernel_arity() {
    assert_eq!(syscall_entry(1), Some(("exit", 1)));
    assert_eq!(syscall_entry(3), Some(("read", 3)));
    assert_eq!(syscall_entry(4), Some(("write", 3)));
    assert_eq!(syscall_entry(11), Some(("execve", 3)));
    assert_eq!(syscall_entry(125), Some(("mprotect", 3)));
    assert_eq!(syscall_entry(192), Some(("mmap2", 6)));
    assert_eq!(syscall_entry(252), Some(("exit_group", 1)));
}

#[test]
fn the_i386_divergent_numbers_are_absent_rather_than_wrong() {
    // Each of these has a documented libc wrapper whose arity is NOT the i386
    // entry point's register count; the table drops them so the pass declines.
    for num in [82u32 /* old_select */, 90 /* old_mmap */, 72 /* sigsuspend */, 117 /* ipc */] {
        assert_eq!(syscall_entry(num), None, "syscall {num} must not be in the table");
    }
    // Out of range.
    assert_eq!(syscall_entry(4096), None);
}
