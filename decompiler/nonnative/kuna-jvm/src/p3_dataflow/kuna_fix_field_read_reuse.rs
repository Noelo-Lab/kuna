//! kuna fix `field_read_reuse` (spec 10.16): a field value the bytecode reads
//! once and uses twice is not read again at each use.
//!
//! `StackVarsProcessor` copies the value of a stack variable into each of its
//! uses when the value may be used several times (`MULTIPLE_USES`), and a
//! field read on a variable counts as such a value. javac's `dup` of a field
//! read before a store (`while ((i = l.count) > 0)`, `a = b = this.f`) gives a
//! stack variable two uses, so the field came out read at both:
//! `int i = l.count; if (l.count <= 0)`. Each read may see another value once
//! another thread writes the field, and on a volatile field each is a separate
//! synchronizing read. With the fix such a value stays in its stack variable,
//! which the following passes write as the assignment expression the source
//! had. A read followed in its block by a store into the same field (the
//! `x++` Kotlin 1.3 compiles with a `dup`) keeps the copies, which
//! `PPandMMHelper` folds back into one read, and so does a read of an object
//! whose field the block then stores (`this.counter.value++`).
//!
//! A kept read of a field that may change (not final) is copied into none of
//! its uses: copied into those in the next statement while the variable
//! stays for the others, it was read there a second time. kotlinc's `x?.f`
//! and `x ?: y` (`dup; ifnull`) put both uses of a field read in the next
//! statement once the conditional is built, and the next-statement copies
//! printed `if (this.x != null) this.x.f`.

use crate::exprent::Exprent;
use crate::rt::JList;
use crate::var_exprent::VarExprent;
use crate::var_version_node::VarVersionNode;

/// Whether the field read `value`, assigned to a stack variable with the uses
/// `used`, must stay in that variable: it has several uses and no statement
/// after `index` stores into a field of the same name and class (a
/// read-modify-write, whose copies `PPandMMHelper` folds into one `x++`; its
/// receiver may still be a stack variable here, so receivers are not compared).
pub fn keeps_single_read(
    value: Exprent,
    used: &[VarVersionNode],
    exprents: &JList<Exprent>,
    index: usize,
) -> bool {
    used.len() > 1
        && !exprents.to_vec().into_iter().skip(index + 1).any(|e| {
            e.try_as_assignment()
                .is_some_and(|store| same_field(store.get_left(), value))
        })
}

/// Whether a statement after `index` stores into a field of the object the
/// stack variable `left` holds, directly or through a stack variable that
/// copies it (a read-modify-write `x.f.g++`, `dup` before `getfield`, whose
/// copies `PPandMMHelper` folds into one read of `x.f`).
pub fn stores_through(left: VarExprent, exprents: &JList<Exprent>, index: usize) -> bool {
    let mut holders = vec![left.get_index()];
    exprents.to_vec().into_iter().skip(index + 1).any(|e| {
        let Some(assignment) = e.try_as_assignment() else {
            return false;
        };
        let target = assignment.get_left();
        if let (Some(copy), Some(source)) =
            (target.try_as_var(), assignment.get_right().try_as_var())
        {
            if copy.is_stack() && holders.contains(&source.get_index()) {
                holders.push(copy.get_index());
            }
            return false;
        }
        target
            .try_as_field()
            .and_then(|f| f.get_instance())
            .and_then(|i| i.try_as_var())
            .is_some_and(|v| holders.contains(&v.get_index()))
    })
}

fn same_field(a: Exprent, b: Exprent) -> bool {
    let (Some(a), Some(b)) = (a.try_as_field(), b.try_as_field()) else {
        return false;
    };
    a.get_name() == b.get_name()
        && a.get_classname() == b.get_classname()
        && a.is_static() == b.is_static()
}
