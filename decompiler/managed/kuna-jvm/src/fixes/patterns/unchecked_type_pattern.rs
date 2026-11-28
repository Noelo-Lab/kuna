//! Fix `unchecked_type_pattern`: no type pattern is made for an unchecked cast
//! (to a type variable, or to a parameterization the operand does not
//! determine).

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::generic_type::GenericType;
use crate::if_statement::IfStatement;
use crate::kuna_static_types::{
    arg_equal, args_equal, args_of, formals, is_bare_var, param_type, supertype_args,
};
use crate::var_type::{get_class, VarType};

/// The static type of the `instanceof` operand: a variable assigned just
/// before the `if` has its value's type (Vineflower then moves the value into
/// the `instanceof`), a method parameter its declared generic type.
pub fn operand_type(source: Exprent, statement: IfStatement) -> VarType {
    if let Some(var) = source.try_as_var() {
        let last = statement
            .get_basichead()
            .get_exprents()
            .and_then(|e| (!e.is_empty()).then(|| e.get(e.len() - 1)));
        if let Some(assign) = last.and_then(|l| l.try_as_assignment()) {
            if assign
                .get_left()
                .try_as_var()
                .is_some_and(|l| l.0.equals(var.0))
            {
                let t = assign.get_right().get_inferred_expr_type(None);
                if t.is_generic() {
                    return t;
                }
            }
        }
    }
    let t = source.get_inferred_expr_type(None);
    if t.is_generic() {
        return t;
    }
    source
        .try_as_var()
        .and_then(|v| param_type(v).flatten())
        .unwrap_or(t)
}

/// Whether `x instanceof to v` is legal for an operand `x` of static type
/// `from`: the cast from `from` to `to` is checked (JLS 5.1.6.2).
pub fn is_checked_cast(from: &VarType, to: &VarType) -> bool {
    if is_reifiable(to) {
        return true;
    }
    if to.type_ != CodeType::Object
        || to.array_dim != 0
        || from.type_ != CodeType::Object
        || from.array_dim != 0
    {
        return false;
    }
    let to_g = to.as_generic();
    if to_g.get_parent().is_some() {
        return false;
    }
    if let Some(view) = supertype_args(from, to.value_str()) {
        if args_equal(&view, to_g.get_arguments()) {
            return true;
        }
    }
    determined_by(from, &to_g)
}

/// A type with no type arguments other than unbounded wildcards.
fn is_reifiable(t: &VarType) -> bool {
    let Some(g) = t.try_as_generic() else {
        return true;
    };
    if !g.is_generic() {
        return true;
    }
    t.type_ != CodeType::Genvar
        && g.get_wildcard() == GenericType::WILDCARD_NO
        && g.get_arguments().iter().all(Option::is_none)
        && g.get_parent().is_none_or(|p| is_reifiable(&p))
}

/// `to` is a subtype of `from` whose every type argument is fixed by one of
/// `from`'s, the shape of a checked downcast.
fn determined_by(from: &VarType, to: &GenericType) -> bool {
    let from_args = args_of(from);
    let Some(to_formals) = formals(to.value_str()) else {
        return false;
    };
    let Some(from_formals) = formals(from.value_str()) else {
        return false;
    };
    let to_args = to.get_arguments();
    if to_formals.len() != to_args.len() || from_formals.len() != from_args.len() {
        return false;
    }
    let Some(cls) = get_class(&to.value) else {
        return false;
    };
    let hierarchy = cls.get_all_generics();
    let Some(map) = hierarchy.get(&from.value_str().to_string()) else {
        return false;
    };
    let mut bound = vec![None; to_formals.len()];
    for (q, b) in from_formals.iter().zip(&from_args) {
        let Some(e) = map.get(q) else {
            return false;
        };
        if !unify(e, b, &to_formals, &mut bound) {
            return false;
        }
    }
    to_args.iter().zip(&bound).all(|(a, b)| match b {
        Some(b) => arg_equal(a, b),
        None => a.is_none(),
    })
}

/// Binds the type parameters `formals` of the target class so that `e`, the
/// argument the target class gives the operand's class written over them, is
/// the operand's argument `b`: a bare parameter is bound to `b`, a
/// parameterized class (`Box<V>`) matches `b` of that class argument by
/// argument; anything else must name no parameter and equal `b`.
fn unify(e: &VarType, b: &Option<VarType>, formals: &[VarType], bound: &mut [Option<Option<VarType>>]) -> bool {
    if let Some(i) = formals.iter().position(|p| p.equals(e) && is_bare_var(e)) {
        return match &bound[i] {
            Some(prev) => arg_equal(prev, b),
            None => {
                bound[i] = Some(b.clone());
                true
            }
        };
    }
    let Some(b) = b else {
        return false;
    };
    let parameterized = |t: &VarType| {
        t.type_ == CodeType::Object
            && t.array_dim == 0
            && t.try_as_generic()
                .is_some_and(|g| g.get_wildcard() == GenericType::WILDCARD_NO && g.get_parent().is_none())
    };
    if parameterized(e) && parameterized(b) && e.value_str() == b.value_str() {
        let (e_args, b_args) = (args_of(e), args_of(b));
        return e_args.len() == b_args.len()
            && e_args.iter().zip(&b_args).all(|(x, y)| match x {
                Some(x) => unify(x, y, formals, bound),
                None => y.is_none(),
            });
    }
    !mentions(e, formals) && arg_equal(&Some(e.clone()), &Some(b.clone()))
}

fn mentions(t: &VarType, formals: &[VarType]) -> bool {
    (t.type_ == CodeType::Genvar && formals.iter().any(|p| p.value_str() == t.value_str()))
        || args_of(t).iter().flatten().any(|a| mentions(a, formals))
}
