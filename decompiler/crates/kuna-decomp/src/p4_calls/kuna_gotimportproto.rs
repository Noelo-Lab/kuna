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
//!
//! A void result binds only where the image proves whose declaration it is:
//! the built-in library table's glibc declaration of the very symbol an import's
//! version ties to glibc (`free@GLIBC_2.0`). The same instructions call a
//! zero-argument `free` from another library, so the import name alone is no
//! evidence, and a declaration of another name at that import is not glibc's.
//! The parameters must be at most four integers or pointers no wider than a
//! pointer, which every MIPS ABI passes in its first argument registers.

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
        && pieces.intypes.iter().all(|ty| concrete(ty))
        && match pieces.outtype.as_ref() {
            Some(ty) if concrete(ty) => true,
            Some(ty) if ty.get_metatype() == type_metatype::TYPE_VOID => glibc_declares(arch, entry, pieces),
            _ => false,
        }
}

/// Whether `pieces` is glibc's own declaration of the import at `entry`, with
/// parameters every MIPS ABI passes in argument registers.
fn glibc_declares(arch: &ArchContext, entry: &Address, pieces: &PrototypePieces) -> bool {
    let Some(pointer) = arch.types().map(|types| types.get_size_of_pointer()) else {
        return false;
    };
    pieces.kuna_library_table
        && arch.loader_glibc_imports.get(entry).is_some_and(|name| *name == pieces.name)
        && pieces.intypes.len() <= 4
        && pieces.intypes.iter().all(|ty| {
            matches!(ty.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_PTR)
                && ty.get_size() <= pointer
        })
}
