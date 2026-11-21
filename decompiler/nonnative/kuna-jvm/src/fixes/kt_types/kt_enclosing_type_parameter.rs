//! Fix `kt_enclosing_type_parameter`: a type-parameter id that no metadata
//! scope declares, in a class declared in a function, takes the name of the type
//! variable at the same place of its JVM `Signature` when that is a type
//! parameter of the enclosing methods or classes.

use std::collections::{BTreeMap, BTreeSet};

use crate::classes_processor::ClassNodeType;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::k_type::KType;
use crate::kt_proto;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

/// The type parameters a class declared in a method may name besides its
/// own: those of the method its `EnclosingMethod` attribute names, of that
/// method's class and of the classes around them, outward. Empty for a
/// class not declared in a method or not written inside its enclosing class.
pub fn enclosing_names(cl: StructClass) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    if cl
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD)
        .is_none()
        || !written_inside(cl)
    {
        return names;
    }
    let context = DecompilerContext::get_struct_context();
    let mut current = cl;
    for _ in 0..16 {
        match current.get_attribute(StructGeneralAttribute::ATTRIBUTE_ENCLOSING_METHOD) {
            Some(attr) => {
                let Some(owner) = attr.get_class_name().and_then(|n| context.get_class(&n)) else {
                    break;
                };
                if let (Some(name), Some(desc)) =
                    (attr.get_method_name(), attr.get_method_descriptor())
                {
                    if let Some(sig) = owner
                        .get_method(&name, &desc)
                        .and_then(|m| m.get_signature())
                    {
                        names.extend(sig.type_parameters().iter().cloned());
                    }
                }
                if let Some(sig) = owner.get_signature() {
                    names.extend(sig.fparameters().iter().cloned());
                }
                current = owner;
            }
            None => {
                let mut name = current.qualified_name();
                let Some(dollar) = name.rfind('$') else {
                    break;
                };
                name.truncate(dollar);
                let Some(outer) = outer_class(&name) else {
                    break;
                };
                if let Some(sig) = outer.get_signature() {
                    names.extend(sig.fparameters().iter().cloned());
                }
                current = outer;
            }
        }
    }
    names
}

/// The nearest class whose name `name` is or starts with, up to a `$`.
fn outer_class(name: &str) -> Option<StructClass> {
    let context = DecompilerContext::get_struct_context();
    let mut name = name.to_string();
    loop {
        if let Some(cl) = context.get_class(&name) {
            return Some(cl);
        }
        let dollar = name.rfind('$')?;
        name.truncate(dollar);
    }
}

/// Whether `cl` is written inside the class it is declared in (an object
/// expression or a local class), where the enclosing type parameters are
/// in scope; a class written apart from it is not.
fn written_inside(cl: StructClass) -> bool {
    let node = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&cl.qualified_name())
        .copied();
    node.is_some_and(|n| {
        matches!(
            n.with(|d| d.type_),
            ClassNodeType::Anonymous | ClassNodeType::Local
        )
    })
}

/// The names of the type-parameter ids in the metadata `proto` of `cl` that
/// no entry of `known` declares, from the class's `Signature` (see the
/// module header).
pub fn scope_entries(
    cl: StructClass,
    proto: &kt_proto::Class,
    resolver: &MetadataNameResolver,
    known: &[(i32, String)],
) -> Vec<(i32, String)> {
    if proto.get_type_parameter_count() > 0 {
        return Vec::new();
    }
    let Some(sig) = cl.get_signature() else {
        return Vec::new();
    };
    if !sig.fparameters().is_empty() {
        return Vec::new();
    }
    let names = enclosing_names(cl);
    if names.is_empty() {
        return Vec::new();
    }
    let mut jvm_supertypes: Vec<VarType> = vec![sig.superclass()];
    jvm_supertypes.extend(sig.superinterfaces().iter().cloned());
    let known: BTreeSet<i32> = known.iter().map(|(id, _)| *id).collect();
    let mut found: BTreeMap<i32, Option<String>> = BTreeMap::new();
    for supertype in proto.get_supertype_list() {
        let class = KType::from(&supertype, resolver).value.clone();
        let Some(jvm) = jvm_supertypes.iter().find(|t| t.value == class) else {
            continue;
        };
        pair(&supertype, jvm, resolver, &known, &names, &mut found);
    }
    found
        .into_iter()
        .filter_map(|(id, name)| name.map(|n| (id, n)))
        .collect()
}

/// Records the name the `Signature` type `jvm` gives each unknown id in
/// the metadata type `meta` (`None` when it gives a class or two names).
fn pair(
    meta: &kt_proto::Type,
    jvm: &VarType,
    resolver: &MetadataNameResolver,
    known: &BTreeSet<i32>,
    names: &BTreeSet<String>,
    found: &mut BTreeMap<i32, Option<String>>,
) {
    if meta.has_type_parameter() {
        let id = meta.get_type_parameter();
        if known.contains(&id) {
            return;
        }
        let name = (jvm.type_ == CodeType::Genvar && jvm.array_dim == 0)
            .then(|| jvm.value.as_deref().map(str::to_string))
            .flatten()
            .filter(|n| names.contains(n));
        let entry = found.entry(id).or_insert_with(|| name.clone());
        if *entry != name {
            *entry = None;
        }
        return;
    }
    if meta.get_argument_count() == 0 || !meta.has_class_name() {
        return;
    }
    if KType::from(meta, resolver).value != jvm.value {
        return;
    }
    let Some(generic) = jvm.try_as_generic() else {
        return;
    };
    let arguments = generic.get_arguments();
    if arguments.len() != meta.get_argument_count() as usize {
        return;
    }
    for (argument, jvm_argument) in meta.get_argument_list().iter().zip(arguments.iter()) {
        if let (true, Some(jvm_argument)) = (argument.has_type(), jvm_argument) {
            pair(
                &argument.get_type(),
                jvm_argument,
                resolver,
                known,
                names,
                found,
            );
        }
    }
}
