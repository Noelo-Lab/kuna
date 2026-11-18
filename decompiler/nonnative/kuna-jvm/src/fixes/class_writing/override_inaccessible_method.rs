//! kuna fix `override_inaccessible_method` (`kuna_fixes`): an instance method
//! overrides a superclass method only if that method is public, protected, or
//! has package access in the overriding class's package (JLS 8.4.8.1); a
//! private method is never overridden. `ClassWriter.searchForMethod` matches
//! any non-static method of a supertype by name and descriptor, so a
//! package-private method redeclared in a subclass in another package (which
//! hides nothing and overrides nothing) gets an `@Override` javac rejects.
//! With the fix, the search skips the supertype methods the class cannot
//! override and goes on to their supertypes.

use crate::code_constants::{ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC};
use crate::decompiler_context::DecompilerContext;
use crate::method_descriptor::MethodDescriptor;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

fn package(internal_name: &str) -> &str {
    internal_name.rfind('/').map_or("", |i| &internal_name[..i])
}

fn overridable(method: StructMethod, declaring: StructClass, origin_package: &str) -> bool {
    !method.has_modifier(ACC_PRIVATE)
        && (method.has_modifier(ACC_PUBLIC)
            || method.has_modifier(ACC_PROTECTED)
            || package(&declaring.qualified_name()) == origin_package)
}

/// Whether a supertype of `cl` declares a method that a method `name` with
/// descriptor `md` declared in `cl` overrides.
pub fn overrides(cl: StructClass, name: &str, md: &MethodDescriptor) -> bool {
    let qualified_name = cl.qualified_name();
    search(Some(cl), name, md, false, package(&qualified_name))
}

fn search(
    cl: Option<StructClass>,
    name: &str,
    md: &MethodDescriptor,
    search_here: bool,
    origin_package: &str,
) -> bool {
    let Some(cl) = cl else {
        return false;
    };
    if search_here
        && cl.get_methods().to_vec().into_iter().any(|method| {
            md.equals(&MethodDescriptor::parse_descriptor(&method.get_descriptor()))
                && name == method.get_name()
                && !method.has_modifier(ACC_STATIC)
                && overridable(method, cl, origin_package)
        })
    {
        return true;
    }
    let context = DecompilerContext::get_struct_context();
    if let Some(super_class) = cl.super_class() {
        if search(context.get_class(&super_class.get_string()), name, md, true, origin_package) {
            return true;
        }
    }
    cl.get_interface_names()
        .into_iter()
        .any(|iface| search(context.get_class(&iface), name, md, true, origin_package))
}
