//! kuna fix `switch_synthetic_default` (`kuna_fixes`): the default javac adds
//! to an exhaustive switch is not written.
//!
//! For a switch expression over an enum and for a pattern switch, javac adds
//! a default the source does not have, `throw new MatchException(null, null)`
//! (before Java 21 `throw new IncompatibleClassChangeError()` for an enum),
//! and lays it out before every case. Vineflower hides it only in a switch
//! expression over an enum whose class it has loaded and whose constants all
//! have a case, and never in a pattern switch statement, so a sealed-type
//! switch prints `default: throw new MatchException(null, null);`. With the fix
//! the default is javac's when its code is exactly that throw and comes before
//! every case's code, and it is left out of a switch expression and of a
//! pattern switch statement (javac adds it again to both).

use crate::exit_exprent::Type as ExitType;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;

/// Whether `switch`'s default is the one javac adds to an exhaustive switch.
pub fn is_javac_default(switch: SwitchStatement) -> bool {
    let Some(first) = switch.get_first().and_then(|f| f.try_as_basic_block()) else {
        return false;
    };
    let Some(instr) = first
        .get_block()
        .get_last_instruction()
        .and_then(|i| i.try_as_switch())
    else {
        return false;
    };
    let Some(default) = switch.get_default_edge() else {
        return false;
    };
    let succs = first.0.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    let destination = instr.get_default_destination();
    succs.first() == Some(&default)
        && instr.get_destinations().iter().all(|d| *d > destination)
        && is_synthetic_throw(default.get_destination())
}

/// Whether `stat` is one statement, `throw new MatchException(null, null)` or
/// `throw new IncompatibleClassChangeError()`.
fn is_synthetic_throw(stat: Statement) -> bool {
    let Some(exprents) = stat.get_exprents().filter(|e| e.len() == 1) else {
        return false;
    };
    let Some(exit) = exprents.get(0).try_as_exit() else {
        return false;
    };
    let Some(new) = exit
        .get_value()
        .filter(|_| exit.get_exit_type() == ExitType::Throw)
        .and_then(|v| v.try_as_new())
    else {
        return false;
    };
    let Some(constructor) = new.get_constructor() else {
        return false;
    };
    let args = constructor.get_lst_parameters().to_vec();
    match new.get_new_type().value.as_deref() {
        Some("java/lang/MatchException") => {
            args.len() == 2
                && args
                    .iter()
                    .all(|a| a.try_as_const().is_some_and(|c| c.is_null()))
        }
        Some("java/lang/IncompatibleClassChangeError") => args.is_empty(),
        _ => false,
    }
}

/// Whether the case of `edges` is javac's default of the pattern switch
/// `switch` and is left out of the statement.
pub fn hides_case(switch: SwitchStatement, edges: &[StatEdge]) -> bool {
    edges.len() == 1
        && Some(edges[0]) == switch.get_default_edge()
        && switch.is_pattern()
        && is_javac_default(switch)
}
