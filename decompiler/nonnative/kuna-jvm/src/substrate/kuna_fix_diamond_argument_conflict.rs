//! kuna fix `diamond_argument_conflict` (`kuna_fixes`): a generic `new` whose
//! type arguments `InvocationExprent.getInferredExprType` infers is written
//! with a diamond when the inferred type is an instance of the expected type's
//! class, without comparing type arguments. Where the constructor's arguments
//! bind a type argument to a type variable in scope and the expected type
//! needs another one there (`Supplier<Set<V>> s = new EnumSetSupplier<>(c)`
//! with `c` a `Class<V0>`), javac cannot infer the diamond and rejects the
//! code; such a `new` is written raw, as the bytecode's unchecked conversion
//! allows.

use crate::code_type::CodeType;
use crate::generic_type::GenericType;
use crate::rt::JHashMap;
use crate::var_type::{get_class, str_equals, VarType};

/// Whether `inferred`, the type a generic `new` of a class with type
/// parameters `fparams` gets from its arguments, binds a type argument to a
/// type variable in scope (`named`) where `expected` has a different one.
pub fn contradicts(
    inferred: Option<&VarType>,
    expected: Option<&VarType>,
    fparams: &[String],
    named: &JHashMap<VarType, Vec<VarType>>,
) -> bool {
    let (Some(inferred), Some(expected)) = (inferred, expected) else {
        return false;
    };
    if !inferred.is_generic() || !expected.is_generic() || inferred.type_ != CodeType::Object {
        return false;
    }
    let lifted = if str_equals(&inferred.value, &expected.value) {
        Some(inferred.clone())
    } else {
        GenericType::get_generic_super_type(inferred, expected)
            .filter(|l| str_equals(&l.value, &expected.value))
            .or_else(|| direct_super_type(inferred, expected))
    };
    match lifted {
        Some(lifted) if lifted.is_generic() => args_clash(&lifted, expected, fparams, named),
        _ => false,
    }
}

/// `inferred` as its direct supertype of `expected`'s class, read from its
/// class's own signature (the supertype's class may be outside the input).
fn direct_super_type(inferred: &VarType, expected: &VarType) -> Option<VarType> {
    let sig = get_class(&inferred.value)?.get_signature()?;
    let mut map: JHashMap<VarType, Option<VarType>> = JHashMap::new();
    sig.generic_type().map_gen_vars_to(&inferred.as_generic(), &mut map);
    let mut supers = vec![sig.superclass()];
    supers.extend(sig.superinterfaces().iter().cloned());
    supers
        .into_iter()
        .find(|t| str_equals(&t.value, &expected.value))
        .and_then(|t| t.remap(&map))
}

fn args_clash(
    a: &VarType,
    b: &VarType,
    fparams: &[String],
    named: &JHashMap<VarType, Vec<VarType>>,
) -> bool {
    if a.type_ == CodeType::Genvar && b.type_ == CodeType::Genvar {
        let own = fparams
            .iter()
            .any(|p| a.value.as_deref() == Some(p.as_str()));
        return a.array_dim == b.array_dim
            && !own
            && named.contains_key(a)
            && named.contains_key(b)
            && !a.equals(b);
    }
    if a.type_ != CodeType::Object
        || b.type_ != CodeType::Object
        || !a.is_generic_type()
        || !b.is_generic_type()
        || !str_equals(&a.value, &b.value)
        || a.array_dim != b.array_dim
    {
        return false;
    }
    let (ga, gb) = (a.as_generic(), b.as_generic());
    if ga.get_wildcard() != GenericType::WILDCARD_NO
        || gb.get_wildcard() != GenericType::WILDCARD_NO
    {
        return false;
    }
    let (xs, ys) = (ga.get_arguments(), gb.get_arguments());
    xs.len() == ys.len()
        && xs.iter().zip(ys.iter()).any(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => {
                let (gx, gy) = (x.try_as_generic(), y.try_as_generic());
                let wild = gx.is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO)
                    || gy.is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO);
                !wild && args_clash(x, y, fparams, named)
            }
            _ => false,
        })
}
