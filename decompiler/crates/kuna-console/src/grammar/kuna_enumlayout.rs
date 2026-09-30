//! (kuna) The width and constants of a C `enum`, as C lays them out.
//!
//! Upstream's `newEnum` interns every parsed enum at the factory's `enumsize`,
//! which `setupSizes` fills with the default data space's address size: eight
//! bytes on every 64-bit target, so a struct member after an `enum` field landed
//! four bytes past where the compiler put it. And `assignValues` numbers an
//! enumerator with no `=` from one past the largest explicit value, so `enum {
//! A, B }` was `A=1, B=2`.
//!
//! C gives an enumeration the width of `int` unless a constant does not fit, and
//! numbers an enumerator with no `=` one past the enumerator before it, the first
//! from zero. A C23 fixed underlying type (`enum E : unsigned char { ... }`) gives
//! the width and signedness outright, which is how a `-fshort-enums` or packed
//! enum is declared.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};
use kuna_decomp::dtype::{type_metatype, Datatype};

use super::{CParse, Enumerator};

impl CParse<'_> {
    /// The optional `: type` after an enum's tag (C23), which must name an
    /// integer type.
    pub(super) fn enum_underlying_type(&mut self) -> KunaResult<Option<Rc<Datatype>>> {
        if !matches!(self.peek()?, super::PToken::Punct(b':')) {
            return Ok(None);
        }
        self.next()?;
        let spec = self.specifier_qualifier_list()?;
        let Some(tp) = spec.type_specifier else {
            return self.syntax_error();
        };
        let meta = tp.get_metatype();
        if tp.is_enum_type() || (meta != type_metatype::TYPE_INT && meta != type_metatype::TYPE_UINT) {
            self.set_error("An enum's underlying type must be an integer type");
            return Err(KunaError::parse(self.lasterror.clone()));
        }
        Ok(Some(tp))
    }

    /// The width and enum meta-type for constants `values`: the underlying
    /// type's when one was given, else the first of `int`, `long` and `long long`
    /// wide enough for every constant.  `None` when the factory knows no `int`
    /// width, so the caller keeps upstream's default.
    pub(super) fn enum_layout(
        &self,
        underlying: Option<&Rc<Datatype>>,
        values: &[uintb],
    ) -> Option<(int4, type_metatype)> {
        if let Some(tp) = underlying {
            let meta = if tp.get_metatype() == type_metatype::TYPE_INT {
                type_metatype::TYPE_ENUM_INT
            } else {
                type_metatype::TYPE_ENUM_UINT
            };
            return Some((tp.get_size(), meta));
        }
        let int = self.factory.get_size_of_int();
        if int <= 0 {
            return None;
        }
        let widest = values.iter().copied().max().unwrap_or(0);
        let size = [int, self.factory.get_size_of_long(), self.factory.get_size_of_long_long(), 8]
            .into_iter()
            .filter(|&s| s >= int && s <= 8)
            .find(|&s| s == 8 || widest <= kuna_base::address::calc_mask(s))
            .unwrap_or(8);
        Some((size, type_metatype::TYPE_ENUM_UINT))
    }
}

/// The first enumerator whose constant the underlying type `tp` cannot
/// represent (C23 makes that a constraint violation), or `None`.
pub(super) fn enumerator_out_of_range<'e>(
    tp: &Datatype,
    vecenum: &'e [Enumerator],
    values: &[uintb],
) -> Option<&'e str> {
    let mut max = kuna_base::address::calc_mask(tp.get_size());
    if tp.get_metatype() == type_metatype::TYPE_INT {
        max >>= 1;
    }
    vecenum.iter().zip(values).find(|(_, &v)| v > max).map(|(e, _)| e.enumconstant.as_str())
}

/// C's constants for `vecenum`: an enumerator with no `=` is one past the one
/// before it, the first zero.
pub(super) fn enum_constants(vecenum: &[Enumerator]) -> Vec<uintb> {
    let mut next: uintb = 0;
    vecenum
        .iter()
        .map(|e| {
            let v = if e.constantassigned { e.value } else { next };
            next = v.wrapping_add(1);
            v
        })
        .collect()
}

#[cfg(test)]
mod tests;
