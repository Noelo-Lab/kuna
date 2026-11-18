//! kuna fix `inline_marker_finally_copy` (spec 10.16): the copies of a
//! `finally` in a Kotlin inline function's body are recognised although their
//! `InlineMarker` depths differ.
//!
//! kotlinc keeps every copy of a `finally` in an inline function's bytecode
//! between `InlineMarker.finallyStart(depth)` and `InlineMarker.finallyEnd(depth)`
//! (empty static methods the inliner reads); a copy on a non-local `return`
//! from a lambda passes a larger depth than the handler's copy.
//! `FinallyProcessor::compare_basic_blocks_ex` compares the depth pushes as
//! ordinary constants, rejects the copy and falls back to a semaphore, which
//! for kotlinc's `synchronized` also leaves the lock unbuilt: the monitor enter
//! printed as an empty `synchronized (x) {}` and the body outside it.

use crate::code_constants::{OPC_BIPUSH, OPC_INVOKESTATIC, OPC_SIPUSH};
use crate::constant_pool::ConstantPool;
use crate::instruction_sequence::InstructionSequence;

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
