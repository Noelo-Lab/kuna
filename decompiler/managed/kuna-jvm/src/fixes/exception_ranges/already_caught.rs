//! Fix `already_caught`: javac compiles `try { try { A } catch (E e) {} } catch (F f) { B }`
//! with both ranges over `A` alone when the inner handler only stores the exception, and
//! `ExceptionDeobfuscator::restore_pop_ranges` gives such a store to an enclosing range only
//! when that range is larger, so the two handlers became `catch` clauses of one `try`. When
//! `F` is `E` or a subclass of it, javac rejects that form, and the class file says the
//! second handler encloses the first.

use crate::decompiler_context::DecompilerContext;
use crate::exception_range_cfg::ExceptionRangeCFG;

/// Whether `outer`, a range over the same blocks as `inner` and after it in the exception
/// table, catches a type `inner` already catches, so its handler belongs to a `try` around
/// `inner`'s.
pub fn encloses(inner: ExceptionRangeCFG, outer: ExceptionRangeCFG) -> bool {
    let (Some(inner_types), Some(outer_types)) =
        (inner.get_exception_types(), outer.get_exception_types())
    else {
        return false;
    };
    let context = DecompilerContext::get_struct_context();
    outer_types
        .iter()
        .any(|t| inner_types.iter().any(|c| context.instance_of(t, c)))
}
