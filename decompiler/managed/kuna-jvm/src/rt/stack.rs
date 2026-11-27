//! Java's `StackOverflowError` as an engine exception.
//!
//! The port recurses wherever Vineflower does, and on adversarial input
//! (expressions, `else if` chains or lambdas nested thousands deep) that
//! recursion can outgrow any stack. A JVM turns the overflow into a
//! `StackOverflowError` that the method or class boundary catches; a Rust
//! overflow aborts the process, which `catch_unwind` cannot stop. So an engine
//! thread declares its stack when it starts ([`enter`]), and [`guard`], called
//! on every arena access and at the recursive writers, throws
//! `StackOverflowError` once the thread has used [`DEPTH_LIMIT`] of it (or all
//! but a [`reserve`] of a smaller stack): the same unwinding every other
//! engine exception takes, caught where Vineflower catches a `Throwable`. A
//! thread that declared nothing is not guarded.

use std::cell::Cell;

thread_local! {
    static FLOOR: Cell<usize> = const { Cell::new(0) };
}

/// The class name a guard throws.
pub const STACK_OVERFLOW: &str = "StackOverflowError";

/// The stack a guarded thread may use. The deepest real input measured
/// (Elasticsearch, Guava, Ghidra, spring, okhttp) used 150 KiB; at this limit
/// kuna still nests expressions several times deeper than Vineflower on a
/// 1 MiB JVM thread stack, and an endless recursion fails within seconds,
/// touching no more stack than this per thread.
pub const DEPTH_LIMIT: usize = 32 << 20;

/// The part of a `stack_bytes` stack kept free below the guard's limit: room
/// for the unguarded frames between two guards and for unwinding.
pub fn reserve(stack_bytes: usize) -> usize {
    (stack_bytes / 8).max(256 << 10)
}

#[inline(always)]
fn stack_pointer() -> usize {
    let marker = 0u8;
    std::hint::black_box(&marker) as *const u8 as usize
}

/// Declares this thread's stack: `stack_bytes` in all, of which this call's
/// frame is near the top. With `0` the thread is not guarded.
#[inline(never)]
pub fn enter(stack_bytes: usize) {
    let usable = stack_bytes
        .saturating_sub(reserve(stack_bytes))
        .min(DEPTH_LIMIT);
    let floor = if stack_bytes == 0 {
        0
    } else {
        stack_pointer().saturating_sub(usable).max(1)
    };
    FLOOR.with(|f| f.set(floor));
}

/// Throws `StackOverflowError` when this thread has used its guarded stack up
/// (never while it is already unwinding).
#[inline(always)]
#[track_caller]
pub fn guard() {
    if stack_pointer() < FLOOR.with(Cell::get) {
        overflow();
    }
}

#[cold]
#[inline(never)]
#[track_caller]
fn overflow() {
    if !std::thread::panicking() {
        crate::rt::throw(STACK_OVERFLOW, "");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn depth(n: u64) -> u64 {
        guard();
        let pad = std::hint::black_box([n; 64]);
        if n == 0 {
            0
        } else {
            1 + depth(n - 1) + pad[0] * 0
        }
    }

    #[test]
    fn a_guarded_thread_throws_instead_of_overflowing() {
        let caught = std::thread::Builder::new()
            .stack_size(1 << 20)
            .spawn(|| {
                enter(1 << 20);
                let deep = crate::rt::try_catch(|| depth(1 << 30));
                let shallow = crate::rt::try_catch(|| depth(100));
                (deep.err().map(|c| c.message), shallow.ok())
            })
            .unwrap()
            .join()
            .unwrap();
        assert_eq!(caught, (Some(STACK_OVERFLOW.to_string()), Some(100)));
    }

    #[test]
    fn an_undeclared_thread_is_not_guarded() {
        assert_eq!(depth(1000), 1000);
    }
}
