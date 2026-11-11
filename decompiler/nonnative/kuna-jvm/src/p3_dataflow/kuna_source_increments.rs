//! The kuna option `jvm_source_locals` (`kuna_source_locals`), for increments:
//! an `x++`, `x--` or `x = x ± 1` the source wrote as a statement of its own
//! stays one, instead of being folded into the statement before it
//! (`SimplifyExprentsHelper.isIPPorIMM`: `a[i] = i; i++;` -> `a[i] = i++;`) or
//! into the next use of `x` (`inlinePPIandMMI`, `PPandMMHelper.inlinePPIandMMIIf`:
//! `x--; f(x);` -> `f(--x);`). Classes javac compiled only (kotlinc's are left
//! as they are).
//!
//! javac emits a statement's code in source order, an expression's operands
//! left to right, and a LineNumberTable entry where a statement starts on a
//! new line. A store followed by the increment is the two statements
//! `x = a; a++;`: in `x = a++` the increment comes first (javac keeps the old
//! value on the stack, which Vineflower folds through its own stack variable,
//! not here). An increment whose code comes before all of the next
//! statement's and starts another line than it is a statement of its own; in
//! `f(--x)` the increment is part of the line.

use crate::assignment_exprent::AssignmentExprent;
use crate::exprent::Exprent;
use crate::p6_variables::kuna_source_locals;
use crate::rt::BitSet;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_exprent::VarExprent;

fn active() -> bool {
    kuna_source_locals::enabled() && !kuna_source_locals::compiled_by_kotlinc()
}

/// Whether `store`, an assignment followed by `increment` of its value, is a
/// statement before it: its own store instruction precedes the increment's
/// code. A stack variable's assignment has no store of its own.
pub fn store_first(store: AssignmentExprent, increment: Exprent) -> bool {
    if !active() || store.get_left().try_as_var().is_some_and(|v| v.is_stack()) {
        return false;
    }
    let (Some(own), Some(inc)) = (store.0.get_bytecode(), range(increment)) else {
        return false;
    };
    !own.is_empty() && (own.length() as i32) <= inc.next_set_bit(0)
}

/// Whether `increment` (`++x` or `--x` of the variable `var`) is a statement
/// of its own before `next`: all of its code comes before `next`'s, and the
/// LineNumberTable puts `next`'s first instruction on another line.
pub fn own_statement(increment: Exprent, var: VarExprent, next: Exprent) -> bool {
    if !active() {
        return false;
    }
    let (Some(inc), Some(rest)) = (range(increment), range(next)) else {
        return false;
    };
    if (inc.length() as i32) > rest.next_set_bit(0) {
        return false;
    }
    let Some(table) = var
        .get_processor()
        .and_then(|p| p.get_method().get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE))
    else {
        return false;
    };
    let (first, line) = (
        table.find_line_number(inc.next_set_bit(0)),
        table.find_line_number(rest.next_set_bit(0)),
    );
    first >= 0 && line >= 0 && first != line
}

/// The instruction offsets of `exprent` and the exprents in it (an `assert`
/// without a message has a missing operand, which `getBytecodeRange` cannot
/// measure).
pub fn range(exprent: Exprent) -> Option<BitSet> {
    let mut values = BitSet::new();
    for e in exprent.get_all_exprents_rec(true).into_iter().chain([exprent]) {
        if let Some(bytecode) = e.get_bytecode() {
            values.or(&bytecode);
        }
    }
    (!values.is_empty()).then_some(values)
}
