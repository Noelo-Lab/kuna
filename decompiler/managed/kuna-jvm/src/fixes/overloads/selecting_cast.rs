//! Fix `selecting_cast`: an argument of a call of a method of a class in the
//! input is cast to its parameter type when, written with its own type,
//! javac's overload resolution (JLS 15.12.2) would select another method of
//! that name or find the call ambiguous.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::fixes::overloads::vararg_spread_resolution::{
    candidates, more_specific, phase, same_parameters, written_type,
};
use crate::invocation_exprent::{InvocationExprent, Type};
use crate::method_descriptor::MethodDescriptor;
use crate::rt::BitSet;
use crate::struct_context::StructContext;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

/// Marks in `ambiguous` the arguments of `call` to cast to their parameter
/// types so that it resolves to its method. Only a call whose method and
/// rivals take no generic parameter types and whose arguments are written
/// with known types, applicable to its method by strict invocation, is
/// decided; casting each argument whose type differs from its parameter's
/// where a rival's differs leaves the method more specific than every rival
/// still applicable.
pub fn mark(call: InvocationExprent, ambiguous: &mut BitSet) {
    if call.get_functype() != Type::General {
        return;
    }
    let descriptor = call.get_descriptor();
    let params = call.get_lst_parameters();
    if params.is_empty() || params.len() != descriptor.params.len() {
        return;
    }
    let Some(methods) = rivals(call, params.len()) else {
        return;
    };
    let own_descriptor = call.get_string_descriptor();
    if !methods.iter().any(|(m, _)| {
        m.get_descriptor() == own_descriptor && !m.has_modifier(CodeConstants::ACC_VARARGS)
    }) {
        return;
    }
    let mut args = Vec::with_capacity(params.len());
    for (i, param) in params.iter().enumerate() {
        if param.try_as_new().is_some_and(|n| n.is_lambda()) {
            return;
        }
        let t = written_type(param, &descriptor.params[i]);
        if !known(&t) {
            return;
        }
        args.push(t);
    }
    if phase(&args, &descriptor.params) != Some(1) {
        return;
    }
    let mut differ = vec![false; params.len()];
    for (_, md) in methods.iter() {
        if same_parameters(&md.params, &descriptor.params)
            || phase(&args, &md.params) != Some(1)
            || more_specific(&descriptor.params, &md.params)
        {
            continue;
        }
        for (i, d) in differ.iter_mut().enumerate() {
            *d |= !md.params[i].equals(&descriptor.params[i]);
        }
    }
    for (i, d) in differ.into_iter().enumerate() {
        if d && !args[i].equals(&descriptor.params[i]) {
            ambiguous.set(i);
        }
    }
}

type Rivals = Rc<Vec<(StructMethod, MethodDescriptor)>>;
type RivalKey = (String, String, String, usize);
type RivalCache = Option<(StructContext, BTreeMap<RivalKey, Option<Rivals>>)>;

thread_local! {
    static RIVALS: RefCell<RivalCache> = const { RefCell::new(None) };
}

/// The methods of `call`'s name and of `arity` parameters the current class
/// can call on `call`'s class, with their descriptors, when there are two or
/// more and none has a generic parameter type; kept per class context.
fn rivals(call: InvocationExprent, arity: usize) -> Option<Rivals> {
    let ctx = DecompilerContext::get_struct_context();
    let current = DecompilerContext::get_current_class_node()
        .map(|n| n.with(|d| d.class_struct).qualified_name())
        .unwrap_or_default();
    let key = (current, call.get_classname(), call.get_name(), arity);
    RIVALS.with(|r| {
        let mut r = r.borrow_mut();
        if r.as_ref().is_none_or(|(c, _)| *c != ctx) {
            *r = Some((ctx, BTreeMap::new()));
        }
        let map = &mut r.as_mut().expect("set above").1;
        if let Some(found) = map.get(&key) {
            return found.clone();
        }
        let found = ctx.get_class(&key.1).and_then(|cl| {
            let methods: Vec<(StructMethod, MethodDescriptor)> = candidates(cl, &key.2)
                .into_iter()
                .filter(|m| accessible(*m))
                .map(|m| (m, m.method_descriptor()))
                .filter(|(_, md)| md.params.len() == arity)
                .collect();
            (methods.len() > 1 && !methods.iter().any(|(m, _)| generic_parameters(*m)))
                .then(|| Rc::new(methods))
        });
        map.insert(key, found.clone());
        found
    })
}

/// Whether the current class may call `mt` by its access flags (a private
/// method is filtered by `candidates`): a package-private one from its
/// package, a protected one also from a subclass or a class nested in one.
fn accessible(mt: StructMethod) -> bool {
    if mt.has_modifier(CodeConstants::ACC_PUBLIC) || mt.has_modifier(CodeConstants::ACC_PRIVATE) {
        return true;
    }
    let owner = mt.get_class_qualified_name();
    let mut scope = DecompilerContext::get_current_class_node();
    let Some(current) = scope.map(|n| n.with(|d| d.class_struct).qualified_name()) else {
        return true;
    };
    if package(&current) == package(&owner) {
        return true;
    }
    if !mt.has_modifier(CodeConstants::ACC_PROTECTED) {
        return false;
    }
    let ctx = DecompilerContext::get_struct_context();
    while let Some(node) = scope {
        let (class, parent) = node.with(|d| (d.class_struct.qualified_name(), d.parent));
        if ctx.instance_of(&class, &owner) {
            return true;
        }
        scope = parent;
    }
    false
}

fn package(class: &str) -> &str {
    class.rfind('/').map_or("", |i| &class[..i])
}

/// Whether a parameter type of `mt`'s generic signature is a type variable
/// or has type arguments, so that its descriptor's type is not the one javac
/// resolves the call with.
fn generic_parameters(mt: StructMethod) -> bool {
    mt.get_signature().is_some_and(|sig| {
        sig.parameter_types()
            .iter()
            .any(|t| t.type_ == CodeType::Genvar || t.is_generic())
    })
}

/// Whether `t` is a type whose conversions are known: a primitive, `null`, or
/// a class or array of the class context.
fn known(t: &VarType) -> bool {
    match t.type_ {
        CodeType::Null => true,
        CodeType::Object => t
            .value
            .as_deref()
            .is_some_and(|v| DecompilerContext::get_struct_context().get_class(v).is_some()),
        CodeType::Byte
        | CodeType::Char
        | CodeType::Short
        | CodeType::Int
        | CodeType::Long
        | CodeType::Float
        | CodeType::Double
        | CodeType::Boolean => true,
        _ => false,
    }
}
