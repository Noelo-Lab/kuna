//! kuna fix `synchronized_lock_assignment` (spec 10.16): which assignment in
//! front of a `synchronized` is the compiler's lock variable.
//!
//! javac compiles `synchronized (e)` to `e; dup; astore n; monitorenter` and
//! kotlinc to `e; astore n; aload n; monitorenter`, at most with stores of
//! constants to its inline markers in between: the lock variable `n` (read
//! again only by the `monitorexit`s) is assigned right before the monitor is
//! entered. `SynchronizedHelper` removes that assignment, but took any
//! assignment of the statement's head block for it:
//! `Object l = this.lock; this.lock = o; synchronized (l) {..}` lost `l` and
//! became `this.lock = o; synchronized (this.lock) {..}`, locking the new
//! object, and `Object l = next(); synchronized (next()) {..}` lost the first
//! call.
//!
//! With the fix, the lock variable's assignment is one followed in the head
//! block only by stores of constants and locals to locals its value does not
//! read, and an assignment whose value the monitor's operand repeats is
//! dropped only where that value is the copy stack-variable inlining made of
//! one evaluation (a value with several uses: no call, allocation or store).

use crate::exprent::Exprent;
use crate::rt::JList;

/// Whether `assignment` of the head block `head` with value `value` may be
/// the compiler's lock variable assignment: moved into the monitor's operand
/// (`moved`), or dropped because the operand repeats `value`.
pub fn is_lock_variable(
    head: &JList<Exprent>,
    assignment: Exprent,
    value: Exprent,
    moved: bool,
) -> bool {
    let exprents = head.to_vec();
    let Some(at) = exprents.iter().position(|e| *e == assignment) else {
        return false;
    };
    if !exprents[at + 1..]
        .iter()
        .all(|e| local_store_unread_by(*e, value))
    {
        return moved && value.is_const();
    }
    moved || value.get_exprent_use() & Exprent::MULTIPLE_USES != 0
}

/// Whether `exprent` stores a constant or a local to a local `value` does not read.
fn local_store_unread_by(exprent: Exprent, value: Exprent) -> bool {
    let Some(ass) = exprent.try_as_assignment() else {
        return false;
    };
    let Some(left) = ass.get_left().try_as_var() else {
        return false;
    };
    let right = ass.get_right();
    (right.is_const() || right.is_var())
        && !value.get_all_exprents_rec_self(true, true).iter().any(|e| {
            e.try_as_var()
                .is_some_and(|v| v.get_index() == left.get_index())
        })
}
