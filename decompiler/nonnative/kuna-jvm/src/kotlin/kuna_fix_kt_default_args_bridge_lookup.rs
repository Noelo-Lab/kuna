//! kuna fix `kt_default_args_bridge_lookup` (`kuna_fixes`), called from
//! `KFunction::parse`: a function's default-arguments method is found by the
//! name and descriptor kotlinc gives it. kotlinc names it after the
//! function's JVM name (`@JvmName`, an `internal` function's `$module`
//! suffix, an inline-class function's hash) and gives it the function's JVM
//! parameters (unboxed inline classes, erased type parameters, a `suspend`
//! function's continuation), after the receiver object of a member, then one
//! `int` mask per 32 value parameters and an `Object` marker; an interface
//! member's is in the interface or, by default, in its `DefaultImpls` class.
//! Vineflower builds the name from the Kotlin name and the descriptor from
//! the Kotlin types and looks only in the class, so it finds no method for
//! those functions, whose defaults then print as unknown, and for a function
//! whose Kotlin name is another function's JVM name it can find the other
//! function's method and print its defaults.

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_INTERFACE, ACC_STATIC, ACC_SYNTHETIC};
use crate::decompiler_context::DecompilerContext;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const DEFAULT_IMPLS: &str = "$DefaultImpls";

/// A function's default-arguments method.
#[derive(Clone)]
pub struct Bridge {
    /// The class holding it, when that is not the function's class.
    pub holder: Option<String>,
    pub name: String,
    pub descriptor: String,
}

/// The default-arguments method of `method`, a function of `cl` with
/// `value_parameters` value parameters, when the class or, for an
/// interface, its `DefaultImpls` has it as a static synthetic method.
pub fn locate(cl: StructClass, method: StructMethod, value_parameters: usize) -> Option<Bridge> {
    let name = format!("{}$default", method.get_name());
    let own = method.get_descriptor();
    let (params, ret) = own.strip_prefix('(')?.split_once(')')?;
    let mut text = String::from("(");
    if !method.has_modifier(ACC_STATIC) {
        text.push_str(&format!("L{};", cl.qualified_name()));
    }
    text.push_str(params);
    text.push_str(&"I".repeat(value_parameters / 32 + 1));
    text.push_str("Ljava/lang/Object;)");
    text.push_str(ret);

    let is_bridge = |m: StructMethod| m.has_modifier(ACC_STATIC) && m.has_modifier(ACC_SYNTHETIC);
    if cl.get_method(&name, &text).is_some_and(is_bridge) {
        return Some(Bridge {
            holder: None,
            name,
            descriptor: text,
        });
    }
    if !cl.has_modifier(ACC_INTERFACE) || method.has_modifier(ACC_STATIC) {
        return None;
    }
    let holder = format!("{}{}", cl.qualified_name(), DEFAULT_IMPLS);
    let impls = DecompilerContext::get_struct_context().get_class(&holder)?;
    if !impls.get_method(&name, &text).is_some_and(is_bridge) {
        return None;
    }
    Some(Bridge {
        holder: Some(holder),
        name,
        descriptor: text,
    })
}

impl Bridge {
    /// The method, decompiled, when the class written is `writing`.
    pub fn wrapper(&self, writing: ClassWrapper) -> Option<MethodWrapper> {
        let wrapper = match &self.holder {
            None => writing,
            Some(holder) => DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get(holder)
                .copied()?
                .get_wrapper()?,
        };
        wrapper.get_method_wrapper(&self.name, &self.descriptor)
    }

    /// The method's class-file record.
    pub fn method(&self, cl: StructClass) -> Option<StructMethod> {
        match &self.holder {
            None => cl.get_method(&self.name, &self.descriptor),
            Some(holder) => DecompilerContext::get_struct_context()
                .get_class(holder)?
                .get_method(&self.name, &self.descriptor),
        }
    }
}
