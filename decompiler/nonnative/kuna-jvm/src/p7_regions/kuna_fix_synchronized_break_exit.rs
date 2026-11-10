//! kuna fix `synchronized_break_exit` (spec 10.16): the lock release at the
//! target of a `break` out of a `synchronized` body is the statement's own.
//!
//! kotlinc releases the lock of `synchronized(lock) { .. return x .. }` after
//! the protected range: the body stores `x`, then jumps to
//! `aload lock; monitorexit; load x; return`. `DomHelper.buildSynchronized`
//! marks the releases in the body, the handler and the regular successors of
//! the catch-all and of its parent; a return block that `DomHelper` placed
//! further out is reached by a `break` from the body and keeps its release,
//! written `// $VF: monitorexit` after the block (with "Could not create
//! synchronized statement" in Java) and blocking the `return` from moving
//! into the break. The `synchronized` statement already releases the lock on
//! that `break`, exactly where the bytecode does, so such a release is dead
//! too.

use crate::basic_block::BasicBlock;
use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::{OPC_ALOAD, OPC_ASTORE, OPC_DUP, OPC_MONITORENTER, OPC_MONITOREXIT};
use crate::statement::{Statement, StatementType};

/// Marks dead the release in each block that a `break` leaves `body` for, when
/// the block starts by releasing `enter`'s lock, holds no other release, and
/// every control-flow predecessor of it is a block of `body`.
pub fn mark_break_exits(enter: Statement, body: Statement) {
    let Some(lock) = lock_var(enter.get_basichead()) else {
        return;
    };
    let mut inner = vec![body];
    let mut i = 0;
    while i < inner.len() {
        let stats = inner[i].get_stats().borrow().to_vec();
        inner.extend(stats);
        i += 1;
    }
    let body_blocks: Vec<BasicBlock> = inner
        .iter()
        .filter(|st| st.type_() == StatementType::BasicBlock)
        .map(|&st| BasicBlockStatement(st).get_block())
        .collect();
    for &st in &inner {
        for edge in st.get_all_successor_edges() {
            let dest = edge.get_destination();
            if dest.type_() != StatementType::BasicBlock || body.contains_statement(dest) {
                continue;
            }
            let block = BasicBlockStatement(dest);
            let preds = block.get_block().get_preds();
            let entered_from_body =
                !preds.is_empty() && preds.iter().all(|p| body_blocks.contains(&p));
            if entered_from_body && releases_first(block, lock) {
                block.mark_monitorexit_dead();
            }
        }
    }
}

/// The local the block's closing `monitorenter` locks: `aload n; monitorenter`
/// or `dup; astore n; monitorenter`.
fn lock_var(enter: BasicBlockStatement) -> Option<i32> {
    let seq = enter.get_block().get_seq();
    let n = seq.length();
    if n < 2 || seq.get_instr(n - 1).opcode() != OPC_MONITORENTER {
        return None;
    }
    let prev = seq.get_instr(n - 2);
    if prev.opcode() == OPC_ALOAD {
        return Some(prev.operand(0));
    }
    (n >= 3 && prev.opcode() == OPC_ASTORE && seq.get_instr(n - 3).opcode() == OPC_DUP)
        .then(|| prev.operand(0))
}

/// Whether the block starts with `aload lock; monitorexit` and has no other
/// `monitorexit`.
fn releases_first(block: BasicBlockStatement, lock: i32) -> bool {
    let seq = block.get_block().get_seq();
    if seq.length() < 2
        || seq.get_instr(0).opcode() != OPC_ALOAD
        || seq.get_instr(0).operand(0) != lock
        || seq.get_instr(1).opcode() != OPC_MONITOREXIT
    {
        return false;
    }
    (2..seq.length()).all(|i| seq.get_instr(i).opcode() != OPC_MONITOREXIT)
}
