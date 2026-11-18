//! kuna fix `kt_return_outside_lambda`: a `return` is dropped only from a
//! lambda's body.
//!
//! `KExitExprent` writes a `return` as its bare value whenever a current
//! method wrapper is installed, because the plugin's lambda writers
//! (`KNewExprent`, `KFieldExprent`) install the lambda's method while writing
//! its body, where the last expression is the value. But `writeMethod`
//! installs the method it writes too (every method without Kotlin metadata:
//! `@JvmOverloads` overloads, `$$forInline` copies, members of synthetic
//! objects), and the wrapper stays installed while an object written inside
//! a lambda or such a method writes its functions, so their early returns
//! print as expression statements and a value-less `return` throws. The fix
//! records, at each declaration boundary (a class being written and a method
//! `writeMethod` writes), which wrapper was current there: a wrapper
//! installed since then is a lambda's, and only its exits drop `return`.

use std::cell::Cell;

use crate::decompiler_context::DecompilerContext;
use crate::method_wrapper::MethodWrapper;

thread_local! {
    static SCOPE: Cell<Option<MethodWrapper>> = const { Cell::new(None) };
}

/// Restores the enclosing declaration's wrapper when dropped.
pub struct Scope(Option<MethodWrapper>);

impl Drop for Scope {
    fn drop(&mut self) {
        SCOPE.with(|c| c.set(self.0));
    }
}

/// Opens a declaration boundary: the method wrapper current now is the
/// declaration's own, until the returned guard is dropped.
pub fn enter() -> Scope {
    let current = DecompilerContext::get_current_method_wrapper();
    Scope(SCOPE.with(|c| c.replace(current)))
}

/// Whether `current`, the installed method wrapper, was installed by a lambda
/// writer inside the innermost declaration rather than by the declaration.
pub fn is_lambda_body(current: MethodWrapper) -> bool {
    SCOPE.with(Cell::get) != Some(current)
}
