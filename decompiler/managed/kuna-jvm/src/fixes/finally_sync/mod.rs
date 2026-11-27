//! Fix family `finally_sync`: `finally` and `synchronized` blocks. In this
//! file, `inline_marker_finally_copy` compares two `finally` copies equal
//! although their `InlineMarker` depth pushes differ; `protected_back_edge`
//! builds a loop whose back edge lies in a `try` or `synchronized` range inside
//! that statement; `synchronized_break_exit` marks dead the lock release at the
//! target of a `break` out of a `synchronized` body, which the statement
//! already releases; `synchronized_lock_assignment` takes for the lock
//! variable's assignment the one followed in the head block only by stores of
//! constants and unrelated locals.

pub(crate) mod finally_exit_versions;
pub(crate) mod kt_semaphore_store_liveness;

use crate::basic_block::BasicBlock;
use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::{
    OPC_ALOAD, OPC_ASTORE, OPC_BIPUSH, OPC_DUP, OPC_INVOKESTATIC, OPC_MONITORENTER, OPC_MONITOREXIT,
    OPC_SIPUSH,
};
use crate::constant_pool::ConstantPool;
use crate::exprent::Exprent;
use crate::instruction_sequence::InstructionSequence;
use crate::rt::JList;
use crate::statement::{Statement, StatementType};

const INLINE_MARKER: &str = "kotlin/jvm/internal/InlineMarker";

/// Whether instruction `i` of `pattern` and of `sample` push the depth argument
/// of the same `InlineMarker.finallyStart`/`finallyEnd` call that follows them.
pub fn is_marker_depth(
    pool: Option<ConstantPool>,
    pattern: InstructionSequence,
    sample: InstructionSequence,
    i: i32,
) -> bool {
    let Some(pool) = pool else {
        return false;
    };
    if i + 1 >= pattern.length() || i + 1 >= sample.length() {
        return false;
    }
    let (push_p, push_s) = (pattern.get_instr(i), sample.get_instr(i));
    let is_push = |op: i32| op == OPC_BIPUSH || op == OPC_SIPUSH;
    if !is_push(push_p.opcode()) || !is_push(push_s.opcode()) {
        return false;
    }
    let (call_p, call_s) = (pattern.get_instr(i + 1), sample.get_instr(i + 1));
    if call_p.opcode() != OPC_INVOKESTATIC
        || call_s.opcode() != OPC_INVOKESTATIC
        || call_p.operand(0) != call_s.operand(0)
    {
        return false;
    }
    pool.get_link_constant(call_p.operand(0)).is_some_and(|cn| {
        cn.classname.as_deref() == Some(INLINE_MARKER)
            && (cn.elementname == "finallyStart" || cn.elementname == "finallyEnd")
            && cn.descriptor == "(I)V"
    })
}

/// Whether `head` holds a back edge to its own first block: the body of a
/// loop that is not built yet.
pub fn is_open_loop_body(head: Statement) -> bool {
    let basichead = head.get_basichead().0;
    head.get_continue_set().borrow().contains(&basichead)
}

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
