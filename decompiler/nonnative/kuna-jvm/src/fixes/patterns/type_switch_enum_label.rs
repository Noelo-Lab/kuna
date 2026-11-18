//! kuna fix `type_switch_enum_label` (`kuna_fixes::KunaFix::TypeSwitchEnumLabel`).
//!
//! A pattern switch (`SwitchBootstraps.typeSwitch`, Java 21+) passes each
//! qualified enum constant label (`case E.X`) as a dynamic constant,
//! `ConstantBootstraps.invoke(EnumDesc.of, ClassDesc.of("pkg.E"), "X")`.
//! Vineflower 1.12.0 resolves only integer, string and class labels, so such
//! a label stays the switch's raw index. This resolves it to the enum field,
//! as upstream's `CondyHelper.simplifyCondy(LinkConstant, ...)` does on
//! `develop/1.13.0` (commit `36ac62ec`), restricted to the constants a
//! `typeSwitch` label can be, with `ClassDesc`'s binary name read in internal
//! form. The field is marked a dynamic constant, as upstream marks it, and
//! that mark is what makes the switch writers print it qualified when the
//! switch is not on the enum's type (`case E.X:`); other enum labels print
//! as Vineflower 1.12.0 prints them.

use crate::code_constants::{CONSTANT_DYNAMIC, TYPE_OBJECT};
use crate::const_exprent::ConstValue;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::link_constant::LinkConstant;
use crate::pooled_constant::PooledConstant;
use crate::primitive_constant::ConstValue as PoolValue;
use crate::statement::Statement;
use crate::struct_bootstrap_methods_attribute::StructBootstrapMethodsAttribute;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

const CONSTANT_BOOTSTRAPS_CLASS: &str = "java/lang/invoke/ConstantBootstraps";
const CONSTANT_ENUMDESC_CLASS: &str = "java/lang/Enum$EnumDesc";
const CONSTANT_CLASSDESC_CLASS: &str = "java/lang/constant/ClassDesc";

/// The enum field a dynamic-constant `typeSwitch` label names, if it is one.
pub fn resolve_label(link: &LinkConstant) -> Option<Exprent> {
    if link.type_ != CONSTANT_DYNAMIC {
        return None;
    }
    let bootstrap = bootstrap_methods()?;
    let constants = bootstrap.get_method_arguments(link.index1);
    let other = bootstrap.get_method_reference(link.index1)?;
    simplify_condy(
        &other,
        &VarType::new_signature(&link.descriptor),
        &constants,
    )
}

/// Whether a static field label is a resolved dynamic-constant enum label
/// that must print qualified: the switch is not on the field's own type, in a
/// Java 21+ method.
pub fn is_qualified_label(field: FieldExprent, switch_type: &VarType, stat: Statement) -> bool {
    field.is_was_condy() && !is_bare_enum_label(field, switch_type, stat)
}

/// The text of a qualified enum label: the field without the dynamic-constant marker.
pub fn qualified_label(field: FieldExprent, indent: i32) -> TextBuffer {
    field.to_java_condy_comment(indent, false)
}

/// Upstream's test: a static field label prints bare when the switch is on
/// the field's own type, or outside a Java 21+ method.
fn is_bare_enum_label(field: FieldExprent, switch_type: &VarType, stat: Statement) -> bool {
    if switch_type.equals(&field.get_expr_type()) {
        return true;
    }
    let mut top = stat;
    while let Some(parent) = top.get_parent() {
        top = parent;
    }
    !top.try_as_root()
        .is_some_and(|root| root.mt().get_bytecode_version().has_switch_pattern_match())
}

fn bootstrap_methods() -> Option<StructBootstrapMethodsAttribute> {
    DecompilerContext::get_current_class()?
        .get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS)
}

fn simplify_condy(
    method: &LinkConstant,
    type_: &VarType,
    args: &[Option<PooledConstant>],
) -> Option<Exprent> {
    match method.classname.as_deref() {
        Some(CONSTANT_BOOTSTRAPS_CLASS) if method.elementname == "invoke" => {
            let Some(Some(PooledConstant::Link(other))) = args.first() else {
                return None;
            };
            simplify_condy(other, type_, &args[1..])
        }
        Some(CONSTANT_ENUMDESC_CLASS) => simplify_enum_desc(args),
        Some(CONSTANT_CLASSDESC_CLASS) => class_desc_name(args).map(|name| {
            crate::const_exprent::ConstExprent::new(
                VarType::VARTYPE_CLASS(),
                Some(ConstValue::String(name)),
                None,
            )
            .0
        }),
        _ => None,
    }
}

fn string_arg(arg: Option<&Option<PooledConstant>>) -> Option<String> {
    let Some(Some(PooledConstant::Primitive(p))) = arg else {
        return None;
    };
    match &p.value {
        Some(PoolValue::String(s)) if p.type_ == TYPE_OBJECT => Some(s.to_string()),
        _ => None,
    }
}

fn class_desc_name(args: &[Option<PooledConstant>]) -> Option<String> {
    if args.len() != 1 {
        return None;
    }
    string_arg(args.first()).map(|binary| binary.replace('.', "/"))
}

fn simplify_enum_desc(args: &[Option<PooledConstant>]) -> Option<Exprent> {
    if args.len() != 2 {
        return None;
    }
    let Some(Some(PooledConstant::Link(get_class))) = args.first() else {
        return None;
    };
    let value_name = string_arg(args.get(1))?;
    let bootstrap = bootstrap_methods()?;
    let get_class_method = bootstrap.get_method_reference(get_class.index1)?;
    let get_class_args = bootstrap.get_method_arguments(get_class.index1);
    let enum_type = simplify_condy(
        &get_class_method,
        &VarType::new_signature(&get_class.descriptor),
        &get_class_args,
    )?;
    let const_exp = enum_type.try_as_const()?;
    if !const_exp.get_expr_type().equals(&VarType::VARTYPE_CLASS()) {
        return None;
    }
    let type_name = const_exp.get_value()?.java_to_string();
    Some(
        FieldExprent::new_full(
            value_name,
            type_name.clone(),
            true,
            None,
            FieldDescriptor::parse_descriptor(&format!("L{type_name};")),
            None,
            false,
            true,
        )
        .0,
    )
}
