//! kuna fix `anonymous_static_qualifier` (`kuna_fixes`): a static
//! method of an anonymous class (allowed since Java 16) called from a class
//! nested in it is invoked as `$N.m(...)`, and `InvocationExprent.toJava`
//! qualifies a static call whose owner is not the class being written with
//! the owner's short name, which for an anonymous class is
//! `<unrepresentable>`. Java can only reach such a method by its simple name,
//! which finds it through the enclosing classes (JLS 15.12.1); javac then
//! compiles the call against the same owner (JLS 13.1). With the fix the call
//! is written unqualified when the owner encloses the class being written and
//! no class in between has a method of that name.

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::kuna_fixes::{self, KunaFix};
use crate::struct_class::StructClass;

/// Whether the static call of `owner.name` whose qualifier would be
/// `short_name` is written without a qualifier.
pub fn drop_qualifier(owner: &str, name: &str, short_name: &str) -> bool {
    if short_name != ExprProcessor::UNREPRESENTABLE_TYPE_STRING
        || !kuna_fixes::enabled(KunaFix::AnonymousStaticQualifier)
    {
        return false;
    }
    enclosed_without_shadow(DecompilerContext::get_current_class_node(), owner, name)
}

/// Walks from `node` out through its enclosing classes: true when it reaches
/// `owner` before any class (other than a lambda's, which is its enclosing
/// class) that has a method `name` of its own or from a supertype.
fn enclosed_without_shadow(mut node: Option<ClassNode>, owner: &str, name: &str) -> bool {
    while let Some(n) = node {
        let (type_, cl, parent) = n.with(|d| (d.type_, d.class_struct, d.parent));
        if cl.qualified_name() == owner {
            return true;
        }
        if type_ != ClassNodeType::Lambda && has_method_named(cl, name) {
            return false;
        }
        node = parent;
    }
    false
}

/// Whether `cl` or a supertype in the struct context declares a method `name`
/// (every such method counts, so the test errs towards keeping the qualifier).
fn has_method_named(cl: StructClass, name: &str) -> bool {
    let context = DecompilerContext::get_struct_context();
    let mut queue = vec![cl];
    let mut visited: Vec<String> = Vec::new();
    while let Some(cls) = queue.pop() {
        let qualified = cls.qualified_name();
        if visited.contains(&qualified) {
            continue;
        }
        visited.push(qualified);
        if cls
            .get_methods()
            .to_vec()
            .iter()
            .any(|m| m.get_name() == name)
        {
            return true;
        }
        let supers = cls
            .super_class()
            .map(|s| s.get_string().to_string())
            .into_iter()
            .chain(cls.get_interface_names());
        queue.extend(supers.filter_map(|s| context.get_class(&s)));
    }
    false
}
