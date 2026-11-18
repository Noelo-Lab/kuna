//! kuna fix `kt_hidden_empty_super_call`: a Kotlin constructor whose empty
//! superclass call was hidden still has one.
//!
//! `InitializerProcessor.hideEmptySuper` removes a constructor's leading
//! `super()` when it passes nothing (or only synthetic arguments): Java
//! makes that call implicit. Kotlin does not, and `KConstructor` reads the
//! call from the constructor's first statement. A primary constructor whose
//! call was removed is written without it, and `writeClassDefinition` then
//! names the superclass from the class signature: `class A(x: Int) : Base`
//! and `object O : Base`, which kotlinc rejects ("this type has a
//! constructor, and thus must be initialized here"). A secondary one takes
//! its next statement for the call when that is an invocation (`constructor(s:
//! String) : println(s)`, which drops the statement from the body), and is
//! left out when nothing follows. `hideEmptySuper` now keeps the call it
//! removes on the method ([`hidden_call`]); the primary constructor writes it
//! as `Base()` in the header, and a secondary one is written with no
//! delegation call, which in a class without a primary constructor means
//! that same `super()`.

use crate::invocation_exprent::InvocationExprent;
use crate::method_wrapper::MethodWrapper;

/// The superclass call `hideEmptySuper` removed from `method`, if any.
pub fn hidden_call(method: MethodWrapper) -> Option<InvocationExprent> {
    method.with(|d| d.hidden_super)
}
