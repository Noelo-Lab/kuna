//! (kuna) Bind a declared prototype that states no storage to
//! a call through a MIPS external-symbol GOT entry.
//!
//! A declaration that names only types (a libc type database entry, a
//! `parse line` prototype) is normally not forced on an indirect call: the
//! call keeps the arguments its trials recover, because an incomplete
//! declaration from a partial debug-info unit would otherwise erase a result
//! the caller uses. A MIPS PIC call to an import loads its target from the
//! import's external-symbol GOT slot and calls through `jalr t9`, so the callee
//! is proven by the loader, and a preserved incoming argument register cannot
//! establish the import's arity. Such a call
//! takes the declared prototype when the loader proved the entry an import and
//! every parameter and the return are concrete, non-void types. Missing, void
//! and unknown types, variadic declarations, which fix no arity, and entries
//! without import provenance keep upstream's storage requirement.

use kuna_base::address::Address;

use crate::context::ArchContext;
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::PrototypePieces;

fn concrete(ty: &Datatype) -> bool {
    ty.get_size() > 0 && !matches!(ty.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN)
}

/// Whether the type-only declaration `pieces` binds at a call to `entry`.
pub fn binds_type_only(arch: &ArchContext, entry: &Address, pieces: &PrototypePieces) -> bool {
    arch.loader_mips_imports.contains(entry)
        && pieces.first_var_arg_slot < 0
        && pieces.outtype.as_ref().is_some_and(|ty| concrete(ty))
        && pieces.intypes.iter().all(|ty| concrete(ty))
}
