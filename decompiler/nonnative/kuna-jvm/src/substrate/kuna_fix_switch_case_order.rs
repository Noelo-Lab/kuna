//! kuna fix `switch_case_order` (`kuna_fixes`): the cases of a switch are
//! written in the order their code has in the method, not by key.
//!
//! javac sorts the keys of a `tableswitch` or `lookupswitch` and lays the case
//! bodies out in the order the source has them (a switch over strings
//! included: its hash buckets and its second switch follow the source's
//! cases). Vineflower's `SwitchStatement.sortEdgesAndNodes` orders the cases
//! by their smallest key (the default last), so `case 'M' .. case 'L' .. case
//! '-'` comes out as `'-', 'L', 'M'`. With the fix a case group is ordered by
//! the first instruction its keys jump to, the `default` included. javac's
//! synthetic default of an exhaustive switch (`throw new MatchException(null,
//! null)`, before Java 21 `throw new IncompatibleClassChangeError()`) is laid
//! out before the cases although the source has none; it stays last, where
//! Vineflower puts it. A class kotlinc compiled (it carries `kotlin.Metadata`)
//! keeps Vineflower's order: kotlinc lays out the branches of a `when` over
//! strings in hash-map order.

use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::{OPC_ACONST_NULL, OPC_ATHROW, OPC_DUP, OPC_INVOKESPECIAL, OPC_NEW};
use crate::decompiler_context::DecompilerContext;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::statement::Statement;

/// The position each successor of the switch head `first` sorts at, by the
/// successor's index in `first`'s successor list (0 is the default): the
/// instruction its key jumps to, the default's past every case when it is
/// javac's synthetic one. `None` when the head's switch instruction does not
/// account for `succs` successors.
pub fn positions(first: BasicBlockStatement, succs: usize, default: Statement) -> Option<Vec<i64>> {
    let cl = DecompilerContext::get_current_class()?;
    if KotlinMetadataAnnotation::find(cl).is_some() {
        return None;
    }
    let instr = first.get_block().get_last_instruction()?.try_as_switch()?;
    let destinations = instr.get_destinations();
    if destinations.len() + 1 != succs {
        return None;
    }
    let default_destination = instr.get_default_destination();
    let mut positions = Vec::with_capacity(succs);
    let synthetic =
        destinations.iter().all(|d| *d > default_destination) && is_synthetic_default(default);
    positions.push(if synthetic {
        i64::MAX
    } else {
        default_destination as i64
    });
    positions.extend(destinations.iter().map(|d| *d as i64));
    Some(positions)
}

/// Whether `stat` is a block of exactly `new MatchException(null, null);
/// throw` or `new IncompatibleClassChangeError(); throw`, the instructions
/// javac gives the default it adds to an exhaustive switch.
pub fn is_synthetic_default(stat: Statement) -> bool {
    let Some(block) = stat.try_as_basic_block() else {
        return false;
    };
    let Some(pool) = DecompilerContext::get_current_class().map(|cl| cl.get_pool()) else {
        return false;
    };
    let instrs = block.get_block().get_seq().instructions();
    let opcodes: Vec<i32> = instrs.iter().map(|i| i.opcode()).collect();
    let (class, descriptor) = match opcodes.as_slice() {
        [OPC_NEW, OPC_DUP, OPC_ACONST_NULL, OPC_ACONST_NULL, OPC_INVOKESPECIAL, OPC_ATHROW] => (
            "java/lang/MatchException",
            "(Ljava/lang/String;Ljava/lang/Throwable;)V",
        ),
        [OPC_NEW, OPC_DUP, OPC_INVOKESPECIAL, OPC_ATHROW] => {
            ("java/lang/IncompatibleClassChangeError", "()V")
        }
        _ => return false,
    };
    let new_class = pool
        .get_primitive_constant(instrs[0].operand(0))
        .and_then(|c| c.get_string_opt());
    let init = pool.get_link_constant(instrs[opcodes.len() - 2].operand(0));
    new_class.as_deref() == Some(class)
        && init.is_some_and(|l| {
            l.classname.as_deref() == Some(class)
                && l.elementname == "<init>"
                && l.descriptor == descriptor
        })
}
