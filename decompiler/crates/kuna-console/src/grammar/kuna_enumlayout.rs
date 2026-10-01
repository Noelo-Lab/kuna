//! (kuna) The width and constants of a C `enum`, as C lays them out.
//!
//! Upstream's `newEnum` interns every parsed enum at the factory's `enumsize`,
//! which `setupSizes` fills with the default data space's address size: eight
//! bytes on every 64-bit target, so a struct member after an `enum` field landed
//! four bytes past where the compiler put it. And `assignValues` numbers an
//! enumerator with no `=` from one past the largest explicit value, so `enum {
//! A, B }` was `A=1, B=2`.
//!
//! An enumerator with no `=` is one past the enumerator before it, the first
//! zero. The enumeration takes gcc's and clang's underlying type: with no
//! negative constant the first of `unsigned int`, `unsigned long` and `unsigned
//! long long` that holds every constant, otherwise the first of `int`, `long` and
//! `long long`. A C23 fixed underlying type (`enum E : unsigned char { ... }`)
//! gives the width and signedness outright, which is how a `-fshort-enums` enum
//! (gcc's default for `arm-none-eabi`) or a packed one is declared.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};
use kuna_decomp::dtype::{type_metatype, Datatype};

use super::{CParse, Enumerator};

impl CParse<'_> {
    /// The optional `: type` after an enum's tag (C23), which must name an
    /// integer type or `_Bool`.
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
        let integer = matches!(
            meta,
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL
        );
        if tp.is_enum_type() || !integer {
            self.set_error("An enum's underlying type must be an integer type");
            return Err(KunaError::parse(self.lasterror.clone()));
        }
        Ok(Some(tp))
    }

    /// The width and enum meta-type for constants `values`: the underlying
    /// type's when one was given, else the first of `int`, `long` and `long long`
    /// that holds every constant, signed when one is negative.  `None` when the
    /// factory knows no `int` width, so the caller keeps upstream's default.
    pub(super) fn enum_layout(
        &self,
        underlying: Option<&Rc<Datatype>>,
        values: &[uintb],
    ) -> Option<(int4, type_metatype)> {
        if let Some(tp) = underlying {
            let meta = if is_signed(tp) { type_metatype::TYPE_ENUM_INT } else { type_metatype::TYPE_ENUM_UINT };
            return Some((tp.get_size(), meta));
        }
        let int = self.factory.get_size_of_int();
        if int <= 0 {
            return None;
        }
        let signed = values.iter().any(|&v| (v as i64) < 0);
        let size = [int, self.factory.get_size_of_long(), self.factory.get_size_of_long_long(), 8]
            .into_iter()
            .filter(|&s| s >= int && s <= 8)
            .find(|&s| values.iter().all(|&v| fits(v, s, signed)))
            .unwrap_or(8);
        let meta = if signed { type_metatype::TYPE_ENUM_INT } else { type_metatype::TYPE_ENUM_UINT };
        Some((size, meta))
    }
}

/// Is the integer type `tp` signed?
fn is_signed(tp: &Datatype) -> bool {
    tp.get_metatype() == type_metatype::TYPE_INT
}

/// Does the constant `v` (two's complement in 64 bits) fit `size` bytes of a
/// signed or unsigned integer?
fn fits(v: uintb, size: int4, signed: bool) -> bool {
    let v = v as i64 as i128;
    let bits = 8 * size.clamp(1, 8) as u32;
    if signed {
        v >= -(1i128 << (bits - 1)) && v < (1i128 << (bits - 1))
    } else {
        v >= 0 && v < (1i128 << bits)
    }
}

/// The first enumerator whose constant the underlying type `tp` cannot
/// represent (C23 makes that a constraint violation), or `None`.
pub(super) fn enumerator_out_of_range<'e>(
    tp: &Datatype,
    vecenum: &'e [Enumerator],
    values: &[uintb],
) -> Option<&'e str> {
    let in_range = |v: uintb| {
        if tp.get_metatype() == type_metatype::TYPE_BOOL {
            v <= 1
        } else {
            fits(v, tp.get_size(), is_signed(tp))
        }
    };
    vecenum.iter().zip(values).find(|(_, &v)| !in_range(v)).map(|(e, _)| e.enumconstant.as_str())
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

/// The `(name, constant)` pairs an enum of `size` bytes can name: C allows two
/// enumerators one constant, but a value names one constant, so the first
/// enumerator of a value keeps it and a later alias is dropped.
pub(super) fn named_constants(vecenum: &[Enumerator], values: &[uintb], size: int4) -> (Vec<String>, Vec<uintb>) {
    let mask = kuna_base::address::calc_mask(size);
    let mut seen = std::collections::BTreeSet::new();
    vecenum
        .iter()
        .zip(values)
        .filter(|(_, &v)| seen.insert(v & mask))
        .map(|(e, &v)| (e.enumconstant.clone(), v))
        .unzip()
}

#[cfg(test)]
mod tests;
