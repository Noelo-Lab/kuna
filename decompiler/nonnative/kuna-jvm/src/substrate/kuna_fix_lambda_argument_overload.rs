//! kuna fix `lambda_argument_overload` (`kuna_fixes`): an implicitly typed
//! lambda or a method reference whose target is chosen by overload
//! resolution says nothing about its type until javac has chosen the
//! overload, so it is compatible with every overload whose parameter at its
//! place is a functional interface of the same arity (JLS 15.12.2.1). Two
//! such overloads, neither of whose interfaces is a subtype of the other's,
//! make the call ambiguous: `spliterator.forEachRemaining(this::add)` on a
//! `Spliterator.OfInt` matches `forEachRemaining(IntConsumer)` and
//! `forEachRemaining(Consumer<? super Integer>)`. Vineflower's ambiguity
//! check compares the lambda's own interface type with each overload's
//! parameter, finds only the overload the bytecode calls, and writes no
//! cast. With the fix such an argument is cast to its interface.

use crate::code_constants::CodeConstants;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::BitSet;
use crate::struct_method::StructMethod;
use crate::var_type::{instance_of, VarType};

const OBJECT_METHODS: &[&str] = &["equals", "hashCode", "toString"];

/// The arguments of `inv` that are lambdas or method references another of
/// the overloads `matches` makes ambiguous.
pub fn ambiguous_arguments(inv: InvocationExprent, matches: &[StructMethod]) -> BitSet {
    let mut out = BitSet::new();
    if matches.len() < 2 {
        return out;
    }
    let args = inv.get_lst_parameters();
    let own = inv.get_descriptor();
    if own.params.len() != args.len() {
        return out;
    }
    let own_descriptor = inv.get_string_descriptor();
    for i in 0..args.len() {
        if !is_lambda(args.get(i)) {
            continue;
        }
        let target = &own.params[i];
        let Some((arity, _)) = sam(target) else {
            continue;
        };
        let (void_ok, value_ok) = lambda_results(args.get(i));
        for &mt in matches {
            if mt.get_descriptor() == own_descriptor {
                continue;
            }
            let md = mt.method_descriptor();
            let other = &md.params[i];
            let compatible = match sam(other) {
                Some((a, void)) => a == arity && if void { void_ok } else { value_ok },
                None => false,
            };
            if other.equals(target) || !compatible || instance_of(&target.value, &other.value) {
                continue;
            }
            let applicable = (0..args.len()).all(|j| {
                let arg = args.get(j);
                if is_lambda(arg) {
                    sam(&md.params[j]).is_some()
                } else {
                    md.params[j].higher_cross_family_than(&peek(arg).get_expr_type(), true)
                }
            });
            if applicable {
                out.set(i);
            }
        }
    }
    out
}

fn peek(e: Exprent) -> Exprent {
    match e.try_as_function() {
        Some(f) if !f.does_cast() => f.get_lst_operands().get(0),
        _ => e,
    }
}

fn is_lambda(e: Exprent) -> bool {
    peek(e).try_as_new().is_some_and(|n| n.is_lambda())
}

/// Whether the lambda `e` can stand for a function that returns nothing
/// and for one that returns a value. A method reference can stand for a
/// void function always, for a value one when its method returns a value;
/// a lambda is taken to be what its implementation method is (an expression
/// lambda that is a statement expression would be both).
fn lambda_results(e: Exprent) -> (bool, bool) {
    let Some(new) = peek(e).try_as_new() else {
        return (false, false);
    };
    let info = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&new.get_new_type().value_str().to_string())
        .copied()
        .and_then(|node| node.with(|d| d.lambda_information.clone()));
    let Some(info) = info else {
        return (false, false);
    };
    let returns = info.content_method_name == "<init>"
        || MethodDescriptor::parse_descriptor(&info.content_method_descriptor)
            .ret
            .type_
            != CodeType::Void;
    if info.is_method_reference {
        (true, returns)
    } else {
        (!returns, returns)
    }
}

/// The parameter count of the one abstract method of the interface `t`, and
/// whether it returns nothing, when `t` is a known functional interface.
fn sam(t: &VarType) -> Option<(usize, bool)> {
    if t.type_ != CodeType::Object || t.array_dim != 0 {
        return None;
    }
    let cls = DecompilerContext::get_struct_context().get_class(t.value_str())?;
    if !cls.has_modifier(CodeConstants::ACC_INTERFACE) {
        return None;
    }
    let mut found = None;
    for mt in cls.get_methods().to_vec() {
        if !mt.has_modifier(CodeConstants::ACC_ABSTRACT)
            || mt.has_modifier(CodeConstants::ACC_STATIC)
            || OBJECT_METHODS.contains(&mt.get_name().as_str())
        {
            continue;
        }
        if found.is_some() {
            return None;
        }
        let md = mt.method_descriptor();
        found = Some((md.params.len(), md.ret.type_ == CodeType::Void));
    }
    found
}
