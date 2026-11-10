//! kuna fix `vararg_spread_resolution` (spec 10.16): the array a varargs call
//! passes is written as its elements only when javac would pack those
//! elements into the same array for the same method.
//!
//! `InvocationExprent.appendParamList` spreads the last `new T[]{..}` of a
//! varargs call unless an overload with as many parameters makes a one-element
//! array ambiguous. javac resolves the spread call afresh, and its first two
//! phases ignore varargs: a method taking the spread arguments as they are
//! wins (`build(new String[0])` inside `build()` came out `build()`, a call
//! to itself), and a lone element that converts to `T[]` is passed as the
//! array (`count(new Object[]{names})` came out `count(names)`, the `String[]`
//! as the argument list; `call(new Object[]{null})` came out `call(null)`, a
//! null array). Vineflower casts a lone element to `Object` only when its type
//! is `Object[]`. With the fix a lone element that converts to `T[]` is cast to
//! `T`, and the array is spread only when, with that cast, no method of the
//! name in the callee's class and its known supertypes is applicable to the
//! written arguments without varargs and no other varargs method is applicable
//! with them. For a callee whose class is not in the input the methods are
//! the descriptors the input calls it by (fix `unseen_class_overload`).

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::rt::JHashSet;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::substrate::kuna_fix_unseen_class_overload;
use crate::var_type::VarType;

/// Whether `element`, the only element of an array of type `array_type`
/// passed to a varargs parameter, converts to the array type itself, so that
/// written alone it would be taken for the array.
pub fn lone_element_cast(array_type: &VarType, element: Exprent) -> bool {
    let component = array_type.decrease_array_dim();
    convertible(&written_type(element, &component), array_type)
}

/// Whether writing `array`, the last argument of `call`, as its elements keeps
/// the call resolving to the same method with the same array.
pub fn spread_resolves(call: InvocationExprent, array: NewExprent) -> bool {
    let params = call.get_lst_parameters();
    let descriptor = call.get_descriptor();
    let array_type = array.get_new_type();
    let component = array_type.decrease_array_dim();
    let elements = array.get_lst_array_elements();
    let mut args: Vec<VarType> = (0..params.len() - 1)
        .map(|i| written_type(params.get(i), &descriptor.params[i]))
        .collect();
    if elements.len() == 1 && lone_element_cast(&array_type, elements.get(0)) {
        args.push(component);
    } else {
        args.extend(elements.iter().map(|e| written_type(e, &component)));
    }

    let own = &descriptor.params;
    overloads(call).iter().all(|(md, varargs, _)| {
        if same_parameters(&md.params, own) {
            return true;
        }
        if md.params.len() == args.len() && phase(&args, &md.params).is_some() {
            return false;
        }
        !(*varargs
            && applicable_by_arity(&args, md)
            && !more_specific(
                &expanded(own, args.len()),
                &expanded(&md.params, args.len()),
            ))
    })
}

/// Whether two methods take the same parameter types (one overrides the other).
pub(crate) fn same_parameters(a: &[VarType], b: &[VarType]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.equals(y))
}

/// The parameter types a variable-arity method with parameters `params`
/// takes `len` arguments with.
fn expanded(params: &[VarType], len: usize) -> Vec<VarType> {
    let Some((last, fixed)) = params.split_last() else {
        return Vec::new();
    };
    let element = last.decrease_array_dim();
    (0..len)
        .map(|i| fixed.get(i).cloned().unwrap_or_else(|| element.clone()))
        .collect()
}

/// The methods `call` may resolve to by its name: (descriptor, varargs,
/// descriptor string). For a class in the input, its and its known
/// supertypes' methods the current class can see, less an interface's static
/// methods, which are not inherited (constructors: its own);
/// for another class, the descriptors the input calls it by (fix
/// `unseen_class_overload`), none known to be varargs.
pub(crate) fn overloads(call: InvocationExprent) -> Vec<(MethodDescriptor, bool, String)> {
    let classname = call.get_classname();
    let name = call.get_name();
    match DecompilerContext::get_struct_context().get_class(&classname) {
        Some(cl) => candidates(cl, &name)
            .into_iter()
            .map(|mt| {
                (
                    mt.method_descriptor(),
                    mt.has_modifier(CodeConstants::ACC_VARARGS),
                    mt.get_descriptor(),
                )
            })
            .collect(),
        None => kuna_fix_unseen_class_overload::referenced(&classname, &name)
            .into_iter()
            .map(|d| (MethodDescriptor::parse_descriptor(&d), false, d))
            .collect(),
    }
}

fn applicable_by_arity(args: &[VarType], md: &MethodDescriptor) -> bool {
    let Some((last, fixed)) = md.params.split_last() else {
        return false;
    };
    if args.len() < fixed.len() || last.array_dim == 0 {
        return false;
    }
    let element = last.decrease_array_dim();
    args.iter().enumerate().all(|(i, a)| {
        if i < fixed.len() {
            convertible(a, &fixed[i])
        } else {
            convertible(a, &element)
        }
    })
}

fn candidates(cl: StructClass, name: &str) -> Vec<StructMethod> {
    let ctx = DecompilerContext::get_struct_context();
    let current = DecompilerContext::get_current_class_node()
        .map(|n| n.with(|d| d.class_struct).qualified_name());
    let mut out = Vec::new();
    let mut seen: JHashSet<String> = JHashSet::new();
    let mut queue = vec![cl];
    while let Some(c) = queue.pop() {
        if !seen.add(c.qualified_name()) {
            continue;
        }
        let inherited_interface = c != cl && c.has_modifier(CodeConstants::ACC_INTERFACE);
        for mt in c.get_methods().to_vec() {
            if mt.name_eq(name)
                && !mt.has_modifier(CodeConstants::ACC_BRIDGE)
                && !mt.is_synthetic()
                && !(inherited_interface && mt.has_modifier(CodeConstants::ACC_STATIC))
                && visible(mt, current.as_deref())
            {
                out.push(mt);
            }
        }
        if name == "<init>" {
            break;
        }
        let supers = c
            .super_class()
            .map(|s| s.get_string().to_string())
            .into_iter()
            .chain(c.get_interface_names());
        queue.extend(supers.filter_map(|s| ctx.get_class(&s)));
    }
    out
}

fn visible(mt: StructMethod, current: Option<&str>) -> bool {
    if !mt.has_modifier(CodeConstants::ACC_PRIVATE) {
        return true;
    }
    let owner = mt.get_class_qualified_name();
    current.is_none_or(|c| outermost(c) == outermost(&owner))
}

fn outermost(name: &str) -> &str {
    name.split('$').next().unwrap_or(name)
}

/// The type the argument `e` of a parameter of type `target` has as
/// Vineflower writes it: a boxing or unboxing call or a widening it leaves
/// implicit is not written, and an `int`-sized constant is an `int` literal
/// (a `char` one for a `char` target).
pub(crate) fn written_type(e: Exprent, target: &VarType) -> VarType {
    if let Some(inv) = e.try_as_invocation() {
        if inv.is_boxing_call() && !inv.should_force_boxing() {
            let value = inv.get_lst_parameters().get(0);
            return written_type(value, &value.get_expr_type());
        }
        if inv.is_unboxing_call() && !inv.should_force_unboxing() {
            if let Some(instance) = inv.get_instance() {
                return written_type(instance, &instance.get_expr_type());
            }
        }
    }
    if let Some(f) = e.try_as_function() {
        if f.get_simple_cast_type().is_some() && !f.does_cast() {
            let operand = f.get_lst_operands().get(0);
            return written_type(operand, &operand.get_expr_type());
        }
    }
    let t = e.get_expr_type();
    if e.is_const()
        && t.array_dim == 0
        && matches!(
            t.type_,
            CodeType::Byte | CodeType::Short | CodeType::Bytechar | CodeType::Shortchar
        )
    {
        return if target.array_dim == 0 && target.type_ == CodeType::Char {
            VarType::VARTYPE_CHAR()
        } else {
            VarType::VARTYPE_INT()
        };
    }
    t
}

/// The phase of JLS 15.12.2 in which a method with parameters `params` is
/// applicable to arguments of types `args`: 1 (strict invocation), 2 (loose).
pub(crate) fn phase(args: &[VarType], params: &[VarType]) -> Option<u8> {
    if args.len() != params.len() {
        return None;
    }
    let mut result = 1;
    for (a, p) in args.iter().zip(params) {
        if !convertible(a, p) {
            return None;
        }
        if primitive(a) != primitive(p) && a.type_ != CodeType::Null {
            result = 2;
        }
    }
    Some(result)
}

/// Whether a method with parameters `a` is more specific than one with `b`
/// (JLS 15.12.2.5, each parameter a subtype).
pub(crate) fn more_specific(a: &[VarType], b: &[VarType]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(x, y)| primitive(x) == primitive(y) && convertible(x, y))
}

pub(crate) fn primitive(t: &VarType) -> bool {
    t.array_dim == 0
        && matches!(
            t.type_,
            CodeType::Byte
                | CodeType::Char
                | CodeType::Short
                | CodeType::Int
                | CodeType::Long
                | CodeType::Float
                | CodeType::Double
                | CodeType::Boolean
                | CodeType::Bytechar
                | CodeType::Shortchar
        )
}

/// The primitive types `t` may stand for (Vineflower's `BYTECHAR` and
/// `SHORTCHAR` leave the choice open).
fn kinds(t: CodeType) -> &'static [CodeType] {
    match t {
        CodeType::Bytechar => &[CodeType::Byte, CodeType::Char],
        CodeType::Shortchar => &[CodeType::Short, CodeType::Char],
        CodeType::Byte => &[CodeType::Byte],
        CodeType::Char => &[CodeType::Char],
        CodeType::Short => &[CodeType::Short],
        CodeType::Int => &[CodeType::Int],
        CodeType::Long => &[CodeType::Long],
        CodeType::Float => &[CodeType::Float],
        CodeType::Double => &[CodeType::Double],
        CodeType::Boolean => &[CodeType::Boolean],
        _ => &[],
    }
}

fn widens(from: CodeType, to: CodeType) -> bool {
    use CodeType::*;
    kinds(from).iter().any(|&f| {
        f == to
            || matches!(
                (f, to),
                (Byte, Short | Int | Long | Float | Double)
                    | (Short | Char, Int | Long | Float | Double)
                    | (Int, Long | Float | Double)
                    | (Long, Float | Double)
                    | (Float, Double)
            )
    })
}

fn box_class(t: CodeType) -> &'static str {
    match t {
        CodeType::Byte => "java/lang/Byte",
        CodeType::Char => "java/lang/Character",
        CodeType::Short => "java/lang/Short",
        CodeType::Int => "java/lang/Integer",
        CodeType::Long => "java/lang/Long",
        CodeType::Float => "java/lang/Float",
        CodeType::Double => "java/lang/Double",
        _ => "java/lang/Boolean",
    }
}

fn unboxed(class: &str) -> Option<CodeType> {
    Some(match class {
        "java/lang/Byte" => CodeType::Byte,
        "java/lang/Character" => CodeType::Char,
        "java/lang/Short" => CodeType::Short,
        "java/lang/Integer" => CodeType::Int,
        "java/lang/Long" => CodeType::Long,
        "java/lang/Float" => CodeType::Float,
        "java/lang/Double" => CodeType::Double,
        "java/lang/Boolean" => CodeType::Boolean,
        _ => return None,
    })
}

fn array_super(class: &str) -> bool {
    matches!(
        class,
        "java/lang/Object" | "java/lang/Cloneable" | "java/io/Serializable"
    )
}

/// Whether an argument of type `a` may be passed to a parameter of type `p` by
/// a method invocation conversion without varargs (JLS 5.3, strict or loose);
/// a type variable or an unknown type counts as convertible.
pub(crate) fn convertible(a: &VarType, p: &VarType) -> bool {
    match a.type_ {
        CodeType::Null => return !primitive(p),
        CodeType::Unknown | CodeType::Any | CodeType::Genvar => return true,
        _ => {}
    }
    if p.type_ == CodeType::Genvar {
        return true;
    }
    match (primitive(a), primitive(p)) {
        (true, true) => widens(a.type_, p.type_),
        (true, false) => {
            p.array_dim == 0
                && kinds(a.type_)
                    .iter()
                    .any(|&k| is_subclass(box_class(k), p.value_str()))
        }
        (false, true) => {
            a.array_dim == 0 && unboxed(a.value_str()).is_some_and(|k| widens(k, p.type_))
        }
        (false, false) => {
            if p.array_dim == 0 {
                return if a.array_dim > 0 {
                    array_super(p.value_str())
                } else {
                    is_subclass(a.value_str(), p.value_str())
                };
            }
            if a.array_dim < p.array_dim {
                return false;
            }
            if a.array_dim > p.array_dim {
                return p.type_ == CodeType::Object && array_super(p.value_str());
            }
            if p.type_ != CodeType::Object || a.type_ != CodeType::Object {
                return a.type_ == p.type_;
            }
            is_subclass(a.value_str(), p.value_str())
        }
    }
}

/// Whether class `a` is a subtype of `b`, as `StructContext.instanceOf`
/// sees it: a class outside the input and the runtime is taken to extend only
/// what is seen of it, as everywhere in Vineflower's typing.
fn is_subclass(a: &str, b: &str) -> bool {
    DecompilerContext::get_struct_context().instance_of(a, b)
}
