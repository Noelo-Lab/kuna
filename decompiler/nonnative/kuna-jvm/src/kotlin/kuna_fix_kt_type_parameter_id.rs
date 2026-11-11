//! kuna fix `kt_type_parameter_id`: a metadata type that refers to a type
//! parameter by id is written with that parameter's name.
//!
//! kotlinc refers to a type parameter of the declaration being serialized by
//! name (`Type.type_parameter_name`) and to any other one, such as a class's
//! `T` in the types of its members or an outer class's in an inner class, by
//! id (`Type.type_parameter`). Vineflower reads only the name, so every type
//! that is a class's type parameter is written as `Any`
//! (`fun get(i: Int): Any` for `fun get(i: Int): T`, `Array<Any>` for
//! `Array<T>`). The metadata parsers open a scope with the ids and names of
//! the type parameters their types can refer to: a class's own and, for an
//! inner class, those of the classes it is inner to; a function's and a
//! property's own.

use std::cell::RefCell;

use crate::k_element::KElement;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;

thread_local! {
    static SCOPE: RefCell<Vec<(i32, String)>> = const { RefCell::new(Vec::new()) };
}

/// Closes a scope opened by [`enter`] when dropped.
pub struct Scope(usize);

impl Drop for Scope {
    fn drop(&mut self) {
        SCOPE.with(|s| s.borrow_mut().truncate(self.0));
    }
}

fn enter(params: Vec<(i32, String)>) -> Scope {
    SCOPE.with(|s| {
        let mut s = s.borrow_mut();
        let len = s.len();
        s.extend(params);
        Scope(len)
    })
}

fn own(params: &[kt_proto::TypeParameter], resolver: &MetadataNameResolver) -> Vec<(i32, String)> {
    params
        .iter()
        .map(|p| (p.get_id(), resolver.resolve(p.get_name())))
        .collect()
}

/// The class `cl` is a member of, from its `InnerClasses` entry.
fn enclosing(cl: StructClass) -> Option<StructClass> {
    let attribute = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)?;
    let name = cl.qualified_name();
    let outer = attribute.get_entries().into_iter().find_map(|entry| {
        (entry.outer_name_idx != 0 && *entry.inner_name == *name)
            .then_some(entry.enclosing_name)
            .flatten()
    })?;
    crate::decompiler_context::DecompilerContext::get_struct_context().get_class(&outer)
}

/// The type parameters of the classes an inner class `cl` (Kotlin flags
/// `flags`) is inner to, outermost first.
fn outer_params(cl: StructClass, flags: i32) -> Vec<(i32, String)> {
    if !Flags::IS_INNER.get(flags) {
        return Vec::new();
    }
    let Some(outer) = enclosing(cl) else {
        return Vec::new();
    };
    KotlinChooser::parse_metadata_for(outer);
    let Some(KElement::Class(class)) = KtAttrs::get(outer, &KElement::KEY()) else {
        return Vec::new();
    };
    let Some(resolver) = &class.resolver else {
        return Vec::new();
    };
    let mut params = outer_params(outer, class.proto.get_flags());
    params.extend(own(&class.proto.get_type_parameter_list(), resolver));
    params
}

/// Opens the scope of class `cl`'s members.
pub fn class_scope(
    cl: StructClass,
    proto: &kt_proto::Class,
    resolver: &MetadataNameResolver,
) -> Scope {
    let mut params = outer_params(cl, proto.get_flags());
    params.extend(own(&proto.get_type_parameter_list(), resolver));
    enter(params)
}

/// Opens the scope of a function's or property's own type parameters.
pub fn member_scope(params: &[kt_proto::TypeParameter], resolver: &MetadataNameResolver) -> Scope {
    enter(own(params, resolver))
}

/// The name of the type parameter with id `id` in the open scopes.
pub fn name(id: i32) -> Option<String> {
    SCOPE.with(|s| {
        s.borrow()
            .iter()
            .rev()
            .find(|(i, _)| *i == id)
            .map(|(_, n)| n.clone())
    })
}
