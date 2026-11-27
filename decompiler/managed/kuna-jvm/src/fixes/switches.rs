//! Fix family `switches`: `switch_case_order` orders cases by the first
//! instruction their keys jump to; `switch_expression_assignments` makes a
//! switch an expression only when every way out of a case ends by assigning
//! the variable; `switch_expression_poly_cast` writes no cast on a switch
//! expression whose results a standalone switch would promote;
//! `switch_synthetic_default` hides the default javac adds to an exhaustive
//! switch, and writes a switch over an enum whose cases return as `return
//! switch`; `switch_table_fillers` leaves out the `tableswitch` keys javac
//! adds for values without a label; `switch_yield_const_retype` lets a
//! yielded constant keep its own type.

use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::{OPC_ACONST_NULL, OPC_ATHROW, OPC_DUP, OPC_INVOKESPECIAL, OPC_NEW, OPC_TABLESWITCH};
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::exprent::Exprent;
use crate::fixes::numeric_types::ternary_numeric_arms::{printed_type, promotes};
use crate::if_statement::IfStatement;
use crate::p6_variables::kuna_source_locals;
use crate::rt::JList;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::switch_exprent::SwitchExprent;
use crate::switch_statement::SwitchStatement;
use crate::var_type::VarType;
use crate::yield_exprent::YieldExprent;

/// The position each successor of the switch head `first` sorts at, by the
/// successor's index in `first`'s successor list (0 is the default): the
/// instruction its key jumps to, the default's past every case when it is
/// javac's synthetic one. `None` when the head's switch instruction does not
/// account for `succs` successors.
pub fn positions(first: BasicBlockStatement, succs: usize, default: Statement) -> Option<Vec<i64>> {
    let cl = DecompilerContext::get_current_class()?;
    if kuna_source_locals::kotlinc_class(cl) {
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

/// Whether each assignment to the variable of index `var` in `stat`'s cases
/// is the last exprent of one of `break_jumps`, and each of those that is
/// reached by completing normally ends in such an assignment or a throw.
pub fn every_assignment_yields(stat: SwitchStatement, break_jumps: &[Statement], var: i32) -> bool {
    let mut finals: Vec<Exprent> = Vec::new();
    for jump in break_jumps {
        let Some(exprents) = jump.get_exprents() else {
            if falls_out(*jump) {
                return false;
            }
            continue;
        };
        let Some(last) = exprents.to_vec().last().copied() else {
            return false;
        };
        if last.try_as_exit().is_some_and(|exit| exit.get_exit_type() == ExitType::Throw) {
            continue;
        }
        if !assigns(last, var) {
            return false;
        }
        finals.push(last);
    }
    stat.get_case_statements()
        .iter()
        .all(|case| assignments_in(case, var, &finals))
}

/// Whether the break edge of the compound statement `jump` is taken when
/// it completes by itself: an `if` without else whose condition is false,
/// a loop whose condition fails. (Any other compound statement completes
/// through the edges of the statements inside it.)
fn falls_out(jump: Statement) -> bool {
    jump.try_as_if().is_some_and(|i| i.iftype() == IfStatement::IFTYPE_IF)
        || jump.try_as_do().is_some_and(|d| d.get_looptype() != do_statement::Type::Infinite)
}

fn assigns(exprent: Exprent, var: i32) -> bool {
    exprent
        .try_as_assignment()
        .and_then(|assign| assign.get_left().try_as_var())
        .is_some_and(|left| left.get_index() == var)
}

/// Whether every assignment to `var` among the exprents `findReplacements`
/// visits under `stat` is in `finals`.
fn assignments_in(stat: Statement, var: i32, finals: &[Exprent]) -> bool {
    if let Some(exprents) = stat.get_exprents() {
        if exprents.iter().any(|e| assigns(e, var) && !finals.contains(&e)) {
            return false;
        }
    }
    stat.get_stats()
        .borrow()
        .to_vec()
        .into_iter()
        .all(|st| assignments_in(st, var, finals))
}

/// Whether `expr` is a switch expression whose results a standalone switch
/// would unbox or promote together.
pub fn results_promote(expr: Exprent) -> bool {
    let Some(switch) = expr.try_as_switch() else {
        return false;
    };
    let mut results = Vec::new();
    for stat in switch.get_backing().get_case_statements().iter() {
        collect_yields(stat, &mut results);
    }
    let types: Vec<_> = results.into_iter().map(printed_type).collect();
    promotes(&types)
}

fn collect_yields(stat: Statement, out: &mut Vec<Exprent>) {
    match stat.get_exprents() {
        Some(exprents) => {
            for e in exprents.iter() {
                if let Some(y) = e.try_as_yield() {
                    out.push(y.get_content());
                }
            }
        }
        None => {
            for st in stat.get_stats().borrow().iter() {
                collect_yields(*st, out);
            }
        }
    }
}

/// Turns `stat` into `return switch (..) {..}` when it is a switch over an
/// enum with javac's synthetic default whose cases return or throw.
pub fn return_switch_expression(stat: SwitchStatement) -> bool {
    if stat.is_phantom()
        || stat.is_pattern()
        || stat.get_parent().is_none()
        || !stat
            .0
            .get_successor_edges(StatEdge::TYPE_REGULAR)
            .is_empty()
        || !is_javac_default(stat)
    {
        return false;
    }
    let Some(default) = stat.get_default_edge() else {
        return false;
    };
    let mut returns: Vec<(Statement, ExitExprent)> = Vec::new();
    let cases = stat.get_case_statements();
    let case_edges = stat.get_case_edges();
    for i in 0..cases.len() {
        let case = cases.get(i);
        let edges = case_edges.get(i);
        if edges.contains(&default) {
            if edges.len() != 1 {
                return false;
            }
            continue;
        }
        if !case.is_basic_block() {
            return false;
        }
        let Some(exprents) = case.get_exprents().filter(|e| !e.is_empty()) else {
            return false;
        };
        let Some(exit) = exprents.get(exprents.len() - 1).try_as_exit() else {
            return false;
        };
        if exprents
            .iter()
            .take(exprents.len() - 1)
            .any(|e| e.is_exit())
        {
            return false;
        }
        let succs = case.get_all_successor_edges();
        if succs.len() != 1 || !succs[0].get_destination().is_dummy_exit() {
            return false;
        }
        match exit.get_exit_type() {
            ExitType::Throw => {}
            ExitType::Return if exit.get_value().is_some() => returns.push((case, exit)),
            _ => return false,
        }
    }
    let Some(&(_, sample)) = returns.first() else {
        return false;
    };
    let Some(ret_type) = sample.get_ret_type() else {
        return false;
    };
    let exit_edge = returns[0].0.get_all_successor_edges()[0];

    for &(case, exit) in &returns {
        let value = exit.get_value().expect("return value");
        let yield_ = YieldExprent::new(value, ret_type.clone());
        case.replace_exprent(exit.0, yield_.0);
    }
    stat.set_phantom(true);

    let suc = BasicBlockStatement::create();
    let switch = SwitchExprent::new(stat, ret_type.clone(), false, false);
    let ret = ExitExprent::new(
        ExitType::Return,
        Some(switch.0),
        Some(ret_type),
        None,
        sample.get_method_descriptor(),
    );
    suc.set_exprents(JList::from(vec![ret.0]));

    let seq = SequenceStatement::new_varargs(&[stat.0, suc.0]);
    seq.set_parent(stat.get_parent());
    stat.replace_with(seq.0);
    seq.set_all_parent();
    suc.add_successor(StatEdge::new_closure(
        exit_edge.get_type(),
        suc.0,
        exit_edge.get_destination(),
        exit_edge.closure(),
    ));
    true
}

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

impl ConstExprent {
    /// Whether `to_java` can write this constant with `type_` as its type:
    /// exactly the types its dispatch does not reject (a primitive or its box,
    /// `null`, `String`, `Class`, `MethodType`; any type without a value).
    pub fn can_be_written_as(self, type_: &VarType) -> bool {
        if type_.type_ != CodeType::Null && self.get_value().is_none() {
            return true;
        }
        let unboxed = VarType::UNBOXING_TYPES().get_or_default(type_, type_.clone());
        match unboxed.type_ {
            CodeType::Boolean
            | CodeType::Char
            | CodeType::Bytechar
            | CodeType::Shortchar
            | CodeType::Byte
            | CodeType::Short
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
            | CodeType::Null => true,
            CodeType::Object => {
                type_.equals(&VarType::VARTYPE_STRING())
                    || type_.equals(&VarType::VARTYPE_CLASS())
                    || type_.equals(&VarType::VARTYPE_METHODTYPE())
            }
            _ => false,
        }
    }
}
