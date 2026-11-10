//! kuna fix `kt_unrepresentable_declared_type` (spec 10.16): a declaration
//! with an initializer whose type has no name in Kotlin is written without a
//! type.
//!
//! The type of a local variable or a private property can be an anonymous
//! class: an `object` expression, or the synthetic class kotlinc makes for a
//! callable reference (`JobSupport::registerSelectForOnJoin`) or a lambda.
//! Vineflower writes such a type `<unrepresentable>`, which is not Kotlin
//! (its Java writer prints `var` there). When an initializer follows, Kotlin
//! infers the declaration's type from it: the same object, statically typed
//! as the object expression or as the reference's function type.

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
