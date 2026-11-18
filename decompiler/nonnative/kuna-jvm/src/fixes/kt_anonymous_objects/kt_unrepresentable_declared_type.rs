//! Fix `kt_unrepresentable_declared_type`: a declaration with an initializer
//! whose type Kotlin cannot name (an anonymous class) is written without a type.

use std::cell::Cell;

/// What Vineflower writes for a type that has no name.
pub const UNREPRESENTABLE: &str = "<unrepresentable>";

thread_local! {
    static INITIALIZED: Cell<bool> = const { Cell::new(false) };
}

struct Scope(bool);

impl Drop for Scope {
    fn drop(&mut self) {
        INITIALIZED.with(|i| i.set(self.0));
    }
}

/// Runs `f`, which writes a declaration that an initializer follows.
pub fn initialized<R>(f: impl FnOnce() -> R) -> R {
    let _scope = Scope(INITIALIZED.with(|i| i.replace(true)));
    f()
}

/// Whether a local declaration written now leaves out its type `written`.
pub fn omits(written: &str) -> bool {
    written == UNREPRESENTABLE && INITIALIZED.with(|i| i.get())
}
