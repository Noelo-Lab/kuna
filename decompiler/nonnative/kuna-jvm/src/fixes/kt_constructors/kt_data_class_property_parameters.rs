//! kuna fix `kt_data_class_property_parameters`: a data class's primary
//! constructor declares its properties.
//!
//! A data class's primary constructor may only have property parameters
//! (`data class Header(val name: ByteString, val value: ByteString)`); the
//! metadata keeps the parameters and the properties apart, and
//! `KConstructor` writes the parameters bare while `KotlinWriter` writes the
//! properties in the class body and `init { this.name = name }`, which is
//! the same class for any other class and rejected for a data class
//! ("primary constructor of data class must only have property
//! parameters"). kotlinc stores a constructor property first in the
//! constructor, in parameter order, right after the superclass call. With
//! the fix, in a data class, each leading store `this.p = p` of a parameter
//! into the field of the property of the same name (declared in the class,
//! with default accessors) is taken out of the `init` block and the property
//! is written in the constructor's header in the parameter's place, with its
//! annotations, modifiers and type as the class body would write them; the
//! class body leaves it out. The members kotlinc synthesizes for a data
//! class (`componentN`, `copy`, and `equals`, `hashCode`, `toString` when
//! the class does not declare them; metadata kind `SYNTHESIZED`) are not
//! written either: Kotlin generates them from the constructor's properties
//! and rejects their declarations ("conflicting overloads").

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::exprent::Exprent;
use crate::interpreter_util::InterpreterUtil;
use crate::k_element::KElement;
use crate::k_parameter::KParameter;
use crate::k_property::KProperty;
use crate::key::Key;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::MemberKind;
use crate::method_wrapper::MethodWrapper;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{JHashSet, JList};
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;

/// Per parameter of the primary constructor, the field of the property it
/// declares.
type Plan = Rc<Vec<Option<StructField>>>;

#[allow(non_snake_case)]
fn PLAN() -> Key<Plan> {
    Key::of("kuna.kt_data_class_property_parameters.plan")
}

#[allow(non_snake_case)]
fn DECLARED() -> Key<bool> {
    Key::of("kuna.kt_data_class_property_parameters.declared")
}

/// `KotlinWriter`'s set of the properties already written, holding those
/// the constructor of `cl` declares, so the class body leaves them out.
pub fn visited(cl: StructClass) -> JHashSet<KProperty> {
    let mut visited = JHashSet::new();
    if kuna_fixes::enabled(KunaFix::KtDataClassPropertyParameters) {
        for field in cl.get_fields().to_vec() {
            if KtAttrs::has(field, &DECLARED()) {
                if let Some(property) = property(field) {
                    visited.add((*property).clone());
                }
            }
        }
    }
    visited
}

/// The fields of the properties the primary constructor `method` of the data
/// class `cl` declares, per parameter; the first call takes their stores out
/// of the constructor body (`first`, its first block's exprents).
pub fn plan(
    cl: StructClass,
    class_flags: i32,
    method: StructMethod,
    parameters: &[KParameter],
    method_wrapper: MethodWrapper,
    wrapper: ClassWrapper,
    first: impl FnOnce() -> JList<Exprent>,
) -> Plan {
    if let Some(plan) = KtAttrs::get(method, &PLAN()) {
        return plan;
    }
    let mut fields: Vec<Option<StructField>> = vec![None; parameters.len()];
    if Flags::IS_DATA.get(class_flags)
        && method_wrapper.with(|d| d.root.is_some() && d.decompile_error.is_none())
    {
        let exprents = first();
        let mut taken = Vec::new();
        collect(cl, method, parameters, &exprents, wrapper, &mut fields, &mut taken);
        for &index in taken.iter().rev() {
            exprents.remove_at(index);
        }
        for field in fields.iter().flatten() {
            KtAttrs::put(*field, &DECLARED(), true);
        }
        hide_synthesized(cl, wrapper);
    }
    let plan = Rc::new(fields);
    KtAttrs::put(method, &PLAN(), plan.clone());
    plan
}

/// Hides the members kotlinc synthesizes for a data class (`componentN`,
/// `copy`, and `equals`, `hashCode`, `toString` unless the class declares
/// them), which Kotlin rejects as declarations ("conflicting overloads").
fn hide_synthesized(cl: StructClass, wrapper: ClassWrapper) {
    for method in cl.get_methods().to_vec() {
        let Some(KElement::Function(function)) = KtAttrs::get(method, &KElement::KEY()) else {
            continue;
        };
        if function.class_struct == Some(cl)
            && Flags::MEMBER_KIND.get(function.flags) == Some(MemberKind::Synthesized)
        {
            let key = InterpreterUtil::make_unique_key(&method.get_name(), &method.get_descriptor());
            wrapper.get_hidden_members().borrow_mut().add(key);
        }
    }
}

/// The leading stores of parameters into their properties' fields.
fn collect(
    cl: StructClass,
    method: StructMethod,
    parameters: &[KParameter],
    exprents: &JList<Exprent>,
    wrapper: ClassWrapper,
    fields: &mut [Option<StructField>],
    taken: &mut Vec<usize>,
) {
    let jvm_params = method.method_descriptor().params.clone();
    let Some(lead) = jvm_params.len().checked_sub(parameters.len()) else {
        return;
    };
    let slots: Vec<i32> = (0..parameters.len())
        .map(|i| 1 + jvm_params[..lead + i].iter().map(|p| p.stack_size).sum::<i32>())
        .collect();
    let start = match exprents.len() {
        0 => return,
        _ => usize::from(
            exprents
                .get(0)
                .try_as_invocation()
                .is_some_and(|invocation| invocation.get_name() == "<init>"),
        ),
    };
    let mut next = 0;
    for index in start..exprents.len() {
        let Some((param, field)) = store(cl, exprents.get(index), &slots) else {
            break;
        };
        if param < next || !declarable(field, &parameters[param], cl, wrapper) {
            break;
        }
        fields[param] = Some(field);
        taken.push(index);
        next = param + 1;
    }
}

/// `this.f = p` of the parameter at one of `slots` into a field of `cl`.
fn store(cl: StructClass, exprent: Exprent, slots: &[i32]) -> Option<(usize, StructField)> {
    let assignment = exprent.try_as_assignment()?;
    if assignment.get_cond_type().is_some() {
        return None;
    }
    let left = assignment.get_left().try_as_field()?;
    if left.is_static() || left.get_classname() != cl.qualified_name() {
        return None;
    }
    let instance = left.get_instance()?.try_as_var()?;
    if instance.get_index() != 0 {
        return None;
    }
    let value = assignment.get_right().try_as_var()?;
    let index = value.get_index();
    let original = value
        .get_processor()
        .and_then(|p| p.get_var_original_index(index))
        .unwrap_or(index);
    let param = slots.iter().position(|&slot| slot == original)?;
    let field = cl.get_field(&left.get_name(), &left.get_descriptor().descriptor_string)?;
    Some((param, field))
}

/// Whether the property of `field` can be written as `parameter`.
fn declarable(
    field: StructField,
    parameter: &KParameter,
    cl: StructClass,
    wrapper: ClassWrapper,
) -> bool {
    let Some(KElement::Property(property)) = KtAttrs::get(field, &KElement::KEY()) else {
        return false;
    };
    if property.class_struct != cl
        || property.name != parameter.name
        || property.extension.is_some()
        || parameter.vararg_type.is_some()
        || Flags::IS_DELEGATED.get(property.flags)
        || Flags::IS_LATEINIT.get(property.flags)
        || Flags::IS_CONST.get(property.flags)
    {
        return false;
    }
    let custom = |accessor: &Option<crate::k_property_accessor::KPropertyAccessor>| {
        accessor
            .as_ref()
            .is_some_and(|a| Flags::IS_NOT_DEFAULT.get(a.flags))
    };
    if custom(&property.getter) || custom(&property.setter) {
        return false;
    }
    header(&property, wrapper, 0).is_some()
}

/// The property as a constructor parameter: its annotations, modifiers,
/// `val`/`var`, name and type on one line, as the class body writes them.
pub fn header(property: &KProperty, wrapper: ClassWrapper, indent: i32) -> Option<String> {
    let text = property.stringify(wrapper, indent).convert_to_string_and_allow_data_discard();
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    let (last, annotations) = lines.split_last()?;
    if annotations.iter().any(|l| !l.starts_with('@'))
        || last.contains(" = ")
        || last.contains('{')
        || !(last.contains("val ") || last.contains("var "))
    {
        return None;
    }
    let mut out = String::new();
    for annotation in annotations {
        out.push_str(annotation);
        out.push(' ');
    }
    out.push_str(last);
    Some(out)
}

/// The property of `field`.
pub fn property(field: StructField) -> Option<Rc<KProperty>> {
    match KtAttrs::get(field, &KElement::KEY()) {
        Some(KElement::Property(property)) => Some(property),
        _ => None,
    }
}
