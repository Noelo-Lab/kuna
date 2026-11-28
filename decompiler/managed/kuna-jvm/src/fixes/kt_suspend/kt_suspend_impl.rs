//! Fix `kt_suspend_impl`: an open suspend member that only calls its
//! `$suspendImpl` method is written with that method's body, its first
//! parameter named `this`.

use std::cell::RefCell;

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_STATIC, ACC_SYNTHETIC};
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::k_element::KElement;
use crate::fixes::kt_extensions;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_wrapper::MethodWrapper;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::fixes::kt_util::SUSPEND_DESCRIPTOR_END;

const SUFFIX: &str = "$suspendImpl";

/// The suspend member whose body `mt`, a `$suspendImpl` method, holds.
pub fn member_of(mt: StructMethod) -> Option<StructMethod> {
    let name = mt.get_name();
    let member_name = name.strip_suffix(SUFFIX).filter(|n| !n.is_empty())?;
    if !mt.has_modifier(ACC_STATIC) || !mt.has_modifier(ACC_SYNTHETIC) {
        return None;
    }
    let class = mt.get_class_qualified_name();
    let descriptor = mt.get_descriptor();
    let rest = descriptor.strip_prefix(&format!("(L{class};"))?;
    if !descriptor.ends_with(SUSPEND_DESCRIPTOR_END) {
        return None;
    }
    let cl = DecompilerContext::get_struct_context().get_class(&class)?;
    let member_descriptor = format!("({rest}");
    cl.get_methods().to_vec().into_iter().find(|member| {
        member.get_descriptor() == member_descriptor
            && !member.has_modifier(ACC_STATIC)
            && matches!(
                KtAttrs::get(*member, &KElement::KEY()),
                Some(KElement::Function(function))
                    if Flags::IS_SUSPEND.get(function.flags)
                        && (function.name == member_name || member.get_name() == member_name)
            )
    })
}

/// The name of `mt` in Kotlin: its function's, which its JVM name mangles
/// for an `internal` function.
fn kotlin_name(mt: StructMethod) -> String {
    match KtAttrs::get(mt, &KElement::KEY()) {
        Some(KElement::Function(function)) => function.name.clone(),
        _ => mt.get_name(),
    }
}

/// The method of `wrapper`'s class whose body is written for `member`, a
/// method of it: its `$suspendImpl`, when `member`'s own body is only the
/// call of it.
pub fn body_of(wrapper: ClassWrapper, member: MethodWrapper) -> Option<MethodWrapper> {
    let mt = member.with(|d| d.method_struct);
    let descriptor = mt.get_descriptor();
    if !descriptor.ends_with(SUSPEND_DESCRIPTOR_END) || mt.has_modifier(ACC_STATIC) {
        return None;
    }
    let impl_descriptor = format!("(L{};{}", mt.get_class_qualified_name(), descriptor.strip_prefix('(')?);
    let (impl_name, impl_wrapper) = [kotlin_name(mt), mt.get_name()].into_iter().find_map(|name| {
        let impl_name = format!("{name}{SUFFIX}");
        wrapper.get_method_wrapper(&impl_name, &impl_descriptor).map(|w| (impl_name, w))
    })?;
    let declared = member_of(impl_wrapper.with(|d| d.method_struct))?;
    if declared.get_name() != mt.get_name() || declared.get_descriptor() != descriptor {
        return None;
    }
    let root = member.with(|d| d.root)?;
    delegates(root, &impl_name).then_some(impl_wrapper)
}

/// Whether the body `root` is only `return impl_name(..)`, a static call of
/// a method of its class.
fn delegates(root: RootStatement, impl_name: &str) -> bool {
    let mut stat: Statement = root.0;
    loop {
        if let Some(exprents) = stat.get_exprents() {
            if exprents.len() != 1 {
                return false;
            }
            let Some(exit) = exprents.get(0).try_as_exit() else {
                return false;
            };
            return exit.get_exit_type() == ExitType::Return
                && exit
                    .get_value()
                    .and_then(|v| v.try_as_invocation())
                    .is_some_and(|inv| inv.is_static() && inv.get_name() == impl_name);
        }
        let stats = stat.get_stats().borrow().to_vec();
        if stats.len() != 1 {
            return false;
        }
        stat = stats[0];
    }
}

thread_local! {
    static WRITTEN: RefCell<Vec<VarProcessor>> = const { RefCell::new(Vec::new()) };
}

/// Ends the writing of a `$suspendImpl` body as its member's when dropped.
pub struct ImplScope;

impl Drop for ImplScope {
    fn drop(&mut self) {
        WRITTEN.with(|w| w.borrow_mut().pop());
    }
}

/// Starts writing the body of `method`, a `$suspendImpl` method, as its
/// member's.
pub fn enter(method: MethodWrapper) -> ImplScope {
    let varproc = method.with(|d| d.varproc);
    WRITTEN.with(|w| w.borrow_mut().push(varproc));
    ImplScope
}

/// The name `var` is written with in a `$suspendImpl` body written as its
/// member's: `this` for its first parameter, `this@Class` for a copy of it
/// captured by a lambda or object in the body.
pub fn name(var: VarExprent) -> Option<String> {
    let current = WRITTEN.with(|w| w.borrow().last().copied())?;
    let mut at = (var.get_processor()?, var.get_var_version_pair());
    for depth in 0..64 {
        let (processor, pair) = at;
        if processor == current {
            if processor.get_var_original_index(pair.var).unwrap_or(pair.var) != 0 {
                return None;
            }
            if depth == 0 {
                return Some("this".to_string());
            }
            let cl = DecompilerContext::get_struct_context().get_class(&processor.get_method().get_class_qualified_name())?;
            return kt_extensions::class_label(cl).map(|label| format!("this@{label}"));
        }
        at = processor.get_captured_source(pair)?;
    }
    None
}
