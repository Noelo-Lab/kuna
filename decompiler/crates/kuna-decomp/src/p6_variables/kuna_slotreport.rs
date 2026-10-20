//! (kuna `slotreport`) A frame slot a printed local lives in is reported at
//! that local's declared type on the `variables` rows (P6).
//!
//! `framelayout` reports the frame slots an earlier restructure pass recovered
//! and the final one folded away, typed `undefinedN`: nothing in the final IR
//! is bound to them as a stack Symbol.  At `-O0` the value in such a slot is
//! often a printed local that merged a register with its spill, so the C
//! declares `char *v1; // r0 | stack - 0xc` while the rows report
//! `local_c undefined4` at the same offset.  [`DeclaredSlots`] indexes, once
//! per function, the printed declarations whose storage homes (the homes their
//! declaration comments list) include a stack home of the declared width, and
//! [`covering_type`] types the slot row, still under its frame name, as those
//! declarations spell it.  Only declarations of the slot's own width count; a
//! declaration listing the slot at another width neither types it nor blocks
//! one that does.  Several same-width declarations type it only when they all
//! spell the same type (one source value the recovery split); when they differ
//! the slot keeps `undefinedN`.  A frame Symbol the type system left
//! uncommitted takes the type the C declares it at under its own name
//! ([`declared_type`]), even when the C spells that `undefinedN` too.
//!
//! Read from the printer after the `print_c` of the same function: nothing here
//! feeds the IR or the emitted C.  The rows reach `--json` and the `; stack:`
//! header of `decompile-project`'s `.asm`.

use std::collections::HashMap;

use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// Is `dt` a one-byte integer or `bool`?  Such a declaration says no more than
/// `undefined1` does: the type system's byte types are its default for any
/// byte it computes with (`char`), and the program may hold the same byte as
/// `_Bool`, `char` or `uint8_t`, so the row keeps the width, as `bytehonest`
/// reports a byte.
fn byte_scalar(dt: &crate::dtype::Datatype) -> bool {
    use crate::dtype::type_metatype;
    dt.get_size() == 1
        && matches!(dt.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL)
}

/// Is `dt` a type the recovery never committed to: an unknown of its width, or
/// an array of unknown bytes?
pub fn uncommitted(dt: &crate::dtype::Datatype) -> bool {
    use crate::dtype::type_metatype;
    match dt.get_metatype() {
        type_metatype::TYPE_UNKNOWN => true,
        type_metatype::TYPE_ARRAY => dt.get_array_base().is_some_and(|e| e.get_metatype() == type_metatype::TYPE_UNKNOWN),
        _ => false,
    }
}

/// Each frame slot a printed declaration lives in, keyed by offset and width:
/// the declarations of the printer's last document (when it printed `fd`)
/// whose storage homes, over the declared high and every high declared through
/// it, include a stack home of the declared type's width.  Built once per
/// function, so a frame row's lookup costs no walk of its own.
pub struct DeclaredSlots {
    slots: HashMap<(i64, i64), Vec<Declared>>,
}

struct Declared {
    name: String,
    /// The declared type's spelling, `None` for a one-byte integer or `bool`.
    type_name: Option<String>,
    /// Is the declared type a pointer or a floating-point type?
    pointer_or_float: bool,
}

impl DeclaredSlots {
    /// The declarations `fd`'s printed C places in frame slots; `None` with
    /// both `slotreport` and `slotsplit` off.
    pub fn of(arch: &Architecture, fd: &Funcdata) -> Option<Self> {
        if !arch.slot_report && !arch.slot_split {
            return None;
        }
        let print = arch.print();
        let through = print.declared_through();
        let mut slots: HashMap<(i64, i64), Vec<Declared>> = HashMap::new();
        for local in crate::kuna_reglocals::declarations_of(print, fd) {
            if !crate::kuna_reglocals::declares_high_of(fd, local) {
                continue;
            }
            let Some(ty) = &local.ty else { continue };
            let size = i64::from(ty.get_size());
            let mut homes: Vec<i64> = std::iter::once(local.high)
                .chain(through.get(&local.high).into_iter().flatten().copied())
                .flat_map(|high| crate::kuna_varsources::high_stack_homes(fd, high))
                .filter(|&(_, s)| i64::from(s) == size)
                .map(|(o, _)| o)
                .collect();
            homes.sort_unstable();
            homes.dedup();
            let type_name = (!byte_scalar(ty)).then(|| crate::kuna_bytehonest::exported_type_name(arch, ty));
            let pointer_or_float = {
                use crate::dtype::type_metatype::{TYPE_FLOAT, TYPE_PTR, TYPE_PTRREL};
                matches!(ty.get_metatype(), TYPE_PTR | TYPE_PTRREL | TYPE_FLOAT)
            };
            for off in homes {
                slots.entry((off, size)).or_default().push(Declared {
                    name: local.name.clone(),
                    type_name: type_name.clone(),
                    pointer_or_float,
                });
            }
        }
        Some(DeclaredSlots { slots })
    }

    /// The type the C declares the `size`-byte frame slot at `off` at: the
    /// type every declaration of that width living there spells.  `None` when
    /// none lives there, when they spell different types, or when they are
    /// one-byte integers or `bool`.
    pub fn covering_type(&self, off: i64, size: i64) -> Option<String> {
        let (first, rest) = self.slots.get(&(off, size))?.split_first()?;
        let type_name = first.type_name.as_ref()?;
        rest.iter().all(|d| d.type_name.as_ref() == Some(type_name)).then(|| type_name.clone())
    }

    /// The type the C declares the frame Symbol `name`, whose own type is
    /// uncommitted, at: its declaration's type, when that declaration lives at
    /// the Symbol's offset and width and is not a one-byte integer or `bool`.
    pub fn declared_type(&self, name: &str, off: i64, size: i64) -> Option<String> {
        self.slots.get(&(off, size))?.iter().find(|d| d.name == name)?.type_name.clone()
    }

    /// An index of the given `(offset, width)` homes, each with its
    /// declarations' names, spellings and pointer-or-float flags.
    #[cfg(test)]
    pub(crate) fn for_test(entries: &[((i64, i64), &[(&str, Option<&str>, bool)])]) -> Self {
        let slots = entries
            .iter()
            .map(|(home, decls)| {
                let decls = decls
                    .iter()
                    .map(|&(name, ty, pointer_or_float)| Declared {
                        name: name.to_string(),
                        type_name: ty.map(str::to_string),
                        pointer_or_float,
                    })
                    .collect();
                (*home, decls)
            })
            .collect();
        DeclaredSlots { slots }
    }

    /// The `(offset, width)` homes of declarations lying wholly inside
    /// `[lo, hi)` and narrower than it, in offset order.
    pub fn homes_within(&self, lo: i64, hi: i64) -> Vec<(i64, i64)> {
        let mut homes: Vec<(i64, i64)> = self
            .slots
            .keys()
            .filter(|&&(off, size)| lo <= off && off + size <= hi && size < hi - lo)
            .copied()
            .collect();
        homes.sort_unstable();
        homes
    }

    /// How the declarations living at `(off, size)` spell that piece of a wider
    /// frame row: their common type, `undefined1` when they are one-byte
    /// integers or `bool`, `undefinedN` when they disagree.  With
    /// `committed_only`, only a common pointer or floating-point type.
    pub fn piece_type(&self, off: i64, size: i64, committed_only: bool) -> Option<String> {
        let (first, rest) = self.slots.get(&(off, size))?.split_first()?;
        let agree = rest.iter().all(|d| d.type_name == first.type_name);
        if committed_only {
            let pointer_or_float = first.pointer_or_float && rest.iter().all(|d| d.pointer_or_float);
            return first.type_name.clone().filter(|_| agree && pointer_or_float);
        }
        match &first.type_name {
            _ if !agree => Some(format!("undefined{size}")),
            Some(type_name) => Some(type_name.clone()),
            None => Some("undefined1".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slots(entries: &[(&str, Option<&str>)]) -> DeclaredSlots {
        let declared = entries
            .iter()
            .map(|(name, ty)| Declared {
                name: name.to_string(),
                type_name: ty.map(str::to_string),
                pointer_or_float: ty.is_some_and(|t| t.ends_with('*') || t == "double"),
            })
            .collect();
        DeclaredSlots { slots: HashMap::from([((-0x18, 8), declared)]) }
    }

    #[test]
    fn one_declaration_types_its_slot() {
        assert_eq!(slots(&[("v1", Some("double"))]).covering_type(-0x18, 8).as_deref(), Some("double"));
    }

    #[test]
    fn same_width_declarations_that_agree_type_the_slot() {
        let s = slots(&[("v1", Some("char *")), ("v4", Some("char *"))]);
        assert_eq!(s.covering_type(-0x18, 8).as_deref(), Some("char *"));
    }

    #[test]
    fn same_width_declarations_that_differ_leave_it_undefined() {
        assert_eq!(slots(&[("v1", Some("char *")), ("v4", Some("long"))]).covering_type(-0x18, 8), None);
    }

    #[test]
    fn a_byte_scalar_declaration_leaves_it_undefined() {
        assert_eq!(slots(&[("v1", None)]).covering_type(-0x18, 8), None);
        assert_eq!(slots(&[("v1", None), ("v2", None)]).covering_type(-0x18, 8), None);
        assert_eq!(slots(&[("v1", Some("long")), ("v2", None)]).covering_type(-0x18, 8), None);
    }

    #[test]
    fn a_declaration_of_another_width_is_not_consulted() {
        let s = slots(&[("v1", Some("double"))]);
        assert_eq!(s.covering_type(-0x18, 4), None);
        assert_eq!(s.covering_type(-0x10, 8), None);
    }

    #[test]
    fn a_symbol_takes_its_own_declaration_among_sharers() {
        let s = slots(&[("v1", Some("char *")), ("v4", Some("long"))]);
        assert_eq!(s.declared_type("v4", -0x18, 8).as_deref(), Some("long"));
        assert_eq!(s.declared_type("v9", -0x18, 8), None);
    }
}
