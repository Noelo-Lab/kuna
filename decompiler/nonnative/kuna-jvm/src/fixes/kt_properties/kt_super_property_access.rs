//! kuna fix `kt_super_property_access`: a property read or write through
//! the superclass's accessor is written `super.name`.
//!
//! `super.isActive` compiles to `invokespecial Base.isActive()` on `this`.
//! The plugin resugars a call of a property accessor to the property and
//! writes its receiver, which for an `invokespecial` is the `this` variable:
//! `this.isActive`, a virtual call that, in an override of `isActive`, calls
//! itself without end. Java's writer qualifies such a call with `super`; the
//! plugin's property form does not. With the fix an accessor call made with
//! `invokespecial` on `this`, to a class other than the caller's, is written
//! on `super` (`super<Name>` for an interface's).

use crate::code_constants::ACC_INTERFACE;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::invocation_exprent::{InvocationExprent, InvocationType};
use crate::var_version_pair::VarVersionPair;

/// `super` (or `super<Iface>`) when `call` is a super call of an accessor.
pub fn qualifier(call: InvocationExprent) -> Option<String> {
    if call.get_invocation_type() != InvocationType::Special {
        return None;
    }
    let this = call.get_instance()?.try_as_var()?;
    let processor = this.get_processor().or_else(|| {
        DecompilerContext::get_current_method_wrapper().map(|m| m.with(|d| d.varproc))
    })?;
    let this_class = processor
        .get_this_vars()
        .borrow()
        .get(&VarVersionPair::from_var(this))
        .cloned()?;
    let classname = call.get_classname();
    if classname == this_class {
        return None;
    }
    let interface = DecompilerContext::get_struct_context()
        .get_class(&classname)
        .is_some_and(|cl| cl.has_modifier(ACC_INTERFACE));
    Some(if interface {
        let name = DecompilerContext::get_import_collector()
            .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(&classname));
        format!("super<{name}>")
    } else {
        "super".to_string()
    })
}
