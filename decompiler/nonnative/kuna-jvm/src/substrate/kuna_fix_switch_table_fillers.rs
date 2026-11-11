//! kuna fix `switch_table_fillers` (`kuna_fixes`): the keys javac adds to a
//! `tableswitch` for values the source has no case for are not written as
//! case labels.
//!
//! A `tableswitch` has an entry for every value from its lowest to its highest
//! label; javac points the values without a label at the default's code.
//! Vineflower writes every entry, so `case 1 .. case 2 .. case 4 .. default`
//! comes out with `case 3:` (and any other gap) in front of `default:`. With
//! the fix a key of the default's case that jumps where the default jumps is
//! left out when javac would still compile the remaining labels to a
//! `tableswitch` of the same range: Gen.visitSwitch picks one when `4 + (hi -
//! lo + 1) + 3 * 3 <= 3 + 2 * n + 3 * n` for `n` labels. The lowest and highest
//! keys bound the table and stay, and so do the fewest of the lowest filler
//! keys that keep `n` large enough (a wide table with few cases).

use crate::code_constants::OPC_TABLESWITCH;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;

/// The edges of `switch`'s head that are filler keys of its `tableswitch`:
/// keys inside its range that jump where the default jumps, when javac would
/// emit the same table without them. Empty for any other switch.
pub fn filler_edges(switch: SwitchStatement) -> Vec<StatEdge> {
    let Some(first) = switch.get_first().and_then(|f| f.try_as_basic_block()) else {
        return Vec::new();
    };
    let Some(instr) = first
        .get_block()
        .get_last_instruction()
        .and_then(|i| i.try_as_switch())
    else {
        return Vec::new();
    };
    if instr.opcode() != OPC_TABLESWITCH || switch.is_pattern() {
        return Vec::new();
    }
    let destinations = instr.get_destinations();
    let succs = first.0.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    if destinations.len() < 2 || succs.len() != destinations.len() + 1 {
        return Vec::new();
    }
    let default = instr.get_default_destination();
    let last = destinations.len() - 1;
    let fillers: Vec<usize> = (1..last).filter(|&i| destinations[i] == default).collect();
    if fillers.is_empty() {
        return Vec::new();
    }
    let labels = (destinations.len() - fillers.len()) as i64;
    let range = destinations.len() as i64;
    let needed = (range + 10 + 4) / 5;
    let keep = (needed - labels).max(0) as usize;
    if keep >= fillers.len() {
        return Vec::new();
    }
    fillers[keep..].iter().map(|&i| succs[i + 1]).collect()
}
