//! kuna fix `kt_constructor_without_metadata`: a constructor that has no
//! Kotlin metadata is written as a Kotlin constructor.
//!
//! `KConstructor` writes the constructors the Kotlin metadata lists. The
//! others go through `write_method`, which writes them as Java does, with
//! `fun` in front: `fun Cls(x: Int) { super(x); .. }` for a named class and
//! the bare block `{ .. }` for an anonymous one, both rejected by kotlinc.
//! They are the constructors of the classes kotlinc gives no constructor
//! metadata: classes without Class metadata (the continuations and flow
//! collectors it copies into `$$inlined$` classes) and anonymous objects,
//! whose metadata lists none, written as named classes when they are not
//! nested where they are created (`$$inlined$` copies, objects created in
//! another method). With the fix a named class's constructor is a secondary
//! constructor, `constructor(x: Int) : super(x) { .. }`: the class has no
//! primary constructor, so each constructor delegates, and the delegation
//! is the superclass or `this` call that is the body's first statement (as
//! `KConstructor` takes it), or none when `hideEmptySuper` removed an empty
//! `super()`, which is then the implicit delegation. A constructor whose
//! first statement is not that call (one that failed, or computes the
//! call's arguments in statements before it) keeps Vineflower's form. The
//! visibility is the constructor's (package-private: `internal`). An
//! anonymous class's constructor body is its initializer, `init { .. }`.
//! Such a constructor is never hidden as a default constructor when it has
//! an explicit delegation or its class's superclass is not `Object`: a
//! class with no constructor gets a default primary one, whose superclass
//! call `: Base` in the header would then be missing.

use crate::class_wrapper::ClassWrapper;
use crate::exprent::Exprent;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kotlin::kuna_fix_kt_hidden_empty_super_call;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_wrapper::MethodWrapper;
use crate::rt::JList;
use crate::statements::Statements;
use crate::text_buffer::TextBuffer;

/// How a constructor delegates.
#[derive(Clone, Copy)]
pub enum Delegation {
    /// The superclass or `this` call that is the body's first statement.
    Call(InvocationExprent),
    /// None written: the empty `super()` that `hideEmptySuper` removed.
    Implicit,
}

/// The delegation of the constructor `method` of `wrapper`'s class, when it
/// can be written as a Kotlin constructor's (not in a file class, whose
/// members are top-level declarations).
pub fn delegation(wrapper: ClassWrapper, method: MethodWrapper) -> Option<Delegation> {
    if matches!(
        KtAttrs::get(wrapper.get_class_struct(), &KElement::KEY()),
        Some(KElement::File(_) | KElement::MultifileFacade(_))
    ) {
        return None;
    }
    let (root, failed) = method.with(|d| (d.root, d.decompile_error.is_some()));
    let root = root.filter(|_| !failed)?;
    if kuna_fixes::enabled(KunaFix::KtHiddenEmptySuperCall)
        && kuna_fix_kt_hidden_empty_super_call::hidden_call(method).is_some()
    {
        return Some(Delegation::Implicit);
    }
    let exprents = root.get_basichead().get_exprents()?;
    if exprents.is_empty() {
        return None;
    }
    let call = exprents.get(0).try_as_invocation()?;
    Statements::is_invocation_init_constructor(call, method, wrapper, true)
        .then_some(Delegation::Call(call))
}

/// Puts the delegation call back as the body's first statement when dropped.
pub struct Restore(Option<(JList<Exprent>, InvocationExprent)>);

impl Drop for Restore {
    fn drop(&mut self) {
        if let Some((exprents, call)) = self.0.take() {
            exprents.add_at(0, call.0);
        }
    }
}

/// Writes ` : super(..)` or ` : this(..)` for `delegation` and takes the
/// call out of `method`'s body until the returned guard is dropped.
pub fn write_delegation(
    method: MethodWrapper,
    delegation: Delegation,
    buffer: &mut TextBuffer,
    indent: i32,
) -> Restore {
    let Delegation::Call(call) = delegation else {
        return Restore(None);
    };
    if kuna_fixes::enabled(KunaFix::KtDefaultArgsCall) {
        kuna_fix_kt_default_args_call::resugar_delegation(call);
    }
    let mt = method.with(|d| d.method_struct);
    buffer.append(" : ");
    buffer.append_buffer_mapped(
        &call.0.to_java(indent + 1),
        Some(&mt.get_class_qualified_name()),
        Some(&InterpreterUtil::make_unique_key(
            &mt.get_name(),
            &mt.get_descriptor(),
        )),
    );
    let exprents = method
        .with(|d| d.root)
        .and_then(|root| root.get_basichead().get_exprents())
        .expect("delegation without a body");
    exprents.remove_at(0);
    Restore(Some((exprents, call)))
}

/// Whether the constructor, written with `delegation`, may be hidden as the
/// class's default constructor: only with no explicit delegation, in a class
/// whose superclass is `Object`.
pub fn may_hide(wrapper: ClassWrapper, delegation: Delegation) -> bool {
    matches!(delegation, Delegation::Implicit)
        && wrapper
            .get_class_struct()
            .super_class()
            .is_none_or(|s| &*s.get_string() == "java/lang/Object")
}
