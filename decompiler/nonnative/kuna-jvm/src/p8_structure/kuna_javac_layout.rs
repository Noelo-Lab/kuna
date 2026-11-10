//! The kuna option `jvm_javac_layout` (`kuna_options::JAVAC_LAYOUT`): ifs and
//! exits of classes javac compiled, written as Java, are structured the way
//! javac laid out the bytecode.
//!
//! javac writes an if's then-branch right after the condition's last jump and
//! the else-branch, or the code after the if, at the jump's target; an exit
//! the source has (`return`, `throw`, `continue` through a `goto`) is an
//! instruction of its own there, and a source ternary returned is one return
//! instruction reached by a goto. Vineflower's `IfHelper.reorderIf` and
//! `SimplifyExprentsHelper.buildIff` decide without that order: `reorderIf`
//! turns `if (c) { ..; return; } rest` into an if-else (whose returns
//! `ExitHelper.removeRedundantReturns` then drops) and `if (c) continue; rest`
//! into `if (!c) { rest }`, and keeps the condition of the block it saw first
//! even when that is the jump's target; `buildIff` merges any if-else of two
//! returns into one return of a ternary. With the option on, [`reorder`]
//! decides from the bytecode offsets which branch javac placed first,
//! [`reorder_last`] puts back a `continue` or `break` once loops are built,
//! `collapseElse` and `ifElseChainDenesting` leave what javac placed first
//! ([`keeps_jump`], [`keeps_denesting_out`]), and [`same_return`] lets
//! `buildIff` merge only exits that are one instruction. Docs:
//! `docs/spec/10-jvm.md` section 10.9.

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::code_constants::{
    GROUP_RETURN, OPC_ALOAD, OPC_ASTORE, OPC_CHECKCAST, OPC_DSTORE, OPC_FSTORE, OPC_GOTO,
    OPC_GOTO_W, OPC_ISTORE, OPC_LSTORE,
};
use crate::do_statement::Type as LoopType;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::if_statement::IfStatement;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_options::{self, JAVAC_LAYOUT};
use crate::rt::BitSet;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

/// The local-variable store opcodes (the `_n` forms are read as these).
const STORES: [i32; 5] = [OPC_ISTORE, OPC_LSTORE, OPC_FSTORE, OPC_DSTORE, OPC_ASTORE];

/// What `reorderIf` does to an if without an else that has statements after
/// it in its sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reorder {
    /// Leave it.
    Keep,
    /// Make the statements after it its else-branch.
    Else,
    /// Make them its else-branch, then swap the branches.
    ElseSwapped,
    /// Negate it: the statements after it become its body, its body follows it.
    Swap,
}

/// The method being structured as Java and its instructions (offset,
/// opcode, length, first operand), read from its bytecode when first asked.
struct Method {
    mt: StructMethod,
    code: Option<Vec<(i32, i32, i32, i32)>>,
    /// How often [`record`] was told about each if.
    done: BTreeMap<i32, u32>,
}

thread_local! {
    static METHOD: RefCell<Option<Method>> = const { RefCell::new(None) };
}

/// The scope of [`enter`]: restores the previous method when it drops.
pub struct MethodScope(Option<Method>);

impl Drop for MethodScope {
    fn drop(&mut self) {
        let previous = self.0.take();
        METHOD.with(|m| *m.borrow_mut() = previous);
    }
}

/// Makes `mt` of `cl` the method [`active`] answers for until the scope
/// drops; none when kotlinc compiled `cl` (it carries `kotlin.Metadata`),
/// since javac's layout is what the decisions read.
pub fn enter(mt: StructMethod, cl: StructClass) -> MethodScope {
    let method = KotlinMetadataAnnotation::find(cl)
        .is_none()
        .then(|| Method {
            mt,
            code: None,
            done: BTreeMap::new(),
        });
    MethodScope(METHOD.with(|m| std::mem::replace(&mut *m.borrow_mut(), method)))
}

/// Whether the option is in effect: it is on, and a method javac compiled
/// is being structured as Java.
pub fn active() -> bool {
    METHOD.with(|m| m.borrow().is_some()) && kuna_options::form_enabled(JAVAC_LAYOUT)
}

/// Notes that `IfHelper.reorderIf` laid `ifstat` out as [`reorder`] or
/// [`reorder_last`] chose. After [`RELAYOUTS`] times Vineflower's choice
/// stands for it, so the decisions and a pass that undoes them cannot
/// alternate without end.
pub fn record(ifstat: IfStatement) {
    METHOD.with(|m| {
        if let Some(method) = m.borrow_mut().as_mut() {
            *method.done.entry(ifstat.0.id()).or_insert(0) += 1;
        }
    });
}

/// How many times one if may be laid out.
const RELAYOUTS: u32 = 3;

fn recorded(ifstat: IfStatement) -> bool {
    METHOD.with(|m| {
        m.borrow().as_ref().is_some_and(|method| {
            method
                .done
                .get(&ifstat.0.id())
                .is_some_and(|n| *n >= RELAYOUTS)
        })
    })
}

/// The instructions of the method being structured from bytecode offset
/// `offset` on, `n` at most: (offset, opcode, length, first operand).
fn code_at(offset: i32, n: usize) -> Vec<(i32, i32, i32, i32)> {
    METHOD.with(|m| {
        let mut m = m.borrow_mut();
        let Some(method) = m.as_mut() else {
            return Vec::new();
        };
        let mt = method.mt;
        let code = method.code.get_or_insert_with(|| {
            let Some(seq) = mt.get_instruction_sequence() else {
                return Vec::new();
            };
            let mut code: Vec<(i32, i32, i32, i32)> = seq
                .instructions()
                .into_iter()
                .map(|i| {
                    let (opcode, start, length) = i.with(|d| (d.opcode, d.start_offset, d.length));
                    let operand = if i.operands_count() > 0 {
                        i.operand(0)
                    } else {
                        0
                    };
                    (start, opcode, length, operand)
                })
                .collect();
            code.sort_unstable();
            code
        });
        match code.binary_search_by_key(&offset, |c| c.0) {
            Ok(index) => code[index..].iter().take(n).copied().collect(),
            Err(_) => Vec::new(),
        }
    })
}

/// The `goto` (or `goto_w`) at bytecode offset `offset`: its length and the
/// offset it jumps to.
fn goto_at(offset: i32) -> Option<(i32, i32)> {
    let code = code_at(offset, 1);
    let &(start, opcode, length, operand) = code.first()?;
    (opcode == OPC_GOTO || opcode == OPC_GOTO_W).then_some((length, start + operand))
}

/// Where javac's then-branch and the code after it start when the code at
/// `after_jump` binds the `instanceof` pattern of a negated test
/// (`if (!(x instanceof T v))`): `aload; checkcast; astore` and a `goto`
/// over the then-branch, which belong to the condition.
fn after_binding(after_jump: i32) -> Option<(i32, i32)> {
    let code = code_at(after_jump, 4);
    let ops: Vec<i32> = code.iter().map(|c| c.1).collect();
    if ops.len() == 4
        && ops[..3] == [OPC_ALOAD, OPC_CHECKCAST, OPC_ASTORE]
        && (ops[3] == OPC_GOTO || ops[3] == OPC_GOTO_W)
    {
        let (offset, _, length, jump) = code[3];
        Some((offset + length, offset + jump))
    } else {
        None
    }
}

/// The slot of the variable a positive `instanceof` pattern binds at
/// `after_jump`, its then-branch's start: `aload; checkcast; astore v` not
/// followed by a `goto` (that of a negated test, [`after_binding`]).
fn positive_binding(after_jump: i32) -> Option<i32> {
    let bind = code_at(after_jump, 4);
    let ops: Vec<i32> = bind.iter().map(|c| c.1).collect();
    (ops.len() == 4
        && ops[..3] == [OPC_ALOAD, OPC_CHECKCAST, OPC_ASTORE]
        && ops[3] != OPC_GOTO
        && ops[3] != OPC_GOTO_W)
        .then(|| bind[2].3)
}

/// Whether the if at `after_jump` is javac's if-else around a pattern: it
/// binds `x instanceof T v` and either starts an else-branch (javac gives an
/// else-if chain's bindings their slots up front, so a nested `if` reads
/// differently) or the first local `rest`, from `rest_start`, declares takes
/// a slot after `v`'s, which an if-else keeps for `v` and code after the if
/// reuses.
fn pattern_if_else(ifstat: IfStatement, after_jump: i32, rest_start: i32) -> bool {
    let Some(slot) = positive_binding(after_jump) else {
        return false;
    };
    starts_else(ifstat)
        || code_at(rest_start, usize::MAX)
            .into_iter()
            .find(|c| STORES.contains(&c.1) && c.3 >= slot)
            .is_some_and(|c| c.3 > slot)
}

/// Whether `ifstat` starts the else-branch of an if.
fn starts_else(ifstat: IfStatement) -> bool {
    let mut stat = ifstat.0;
    while let Some(parent) = stat.get_parent() {
        if let Some(outer) = parent.try_as_if() {
            return outer.get_elsestat() == Some(stat);
        }
        if !parent.is_sequence() || parent.get_first() != Some(stat) {
            return false;
        }
        stat = parent;
    }
    false
}

/// Whether `then` (where the then-branch javac wrote is) is a `goto` at
/// `after_jump` that jumps away, not to `next`, the code right after it.
fn goto_away(after_jump: i32, next: i32) -> bool {
    goto_at(after_jump)
        .is_some_and(|(length, target)| after_jump + length == next && target != next)
}

/// Whether two exits `buildIff` would merge are one bytecode instruction.
pub fn same_return(a: Exprent, b: Exprent) -> bool {
    match (a.get_bytecode(), b.get_bytecode()) {
        (Some(x), Some(y)) => x.intersects(&y),
        _ => false,
    }
}

/// javac's choice for `ifstat`, an if without an else followed by `rest` in
/// `parent`; `None` when the offsets do not tell (Vineflower's choice stands).
/// `ifdirect` says whether its body reaches where the sequence ends, `next`,
/// `elsedirect` whether `rest` ends there.
pub fn reorder(
    ifstat: IfStatement,
    parent: SequenceStatement,
    next: Statement,
    ifdirect: bool,
    elsedirect: bool,
) -> Option<Reorder> {
    if recorded(ifstat) {
        return None;
    }
    if loop_head(ifstat) {
        return None;
    }
    let rest = following(ifstat, parent)?;
    let Some(body) = ifstat.get_ifstat() else {
        return keeps_jump(ifstat, &rest).then_some(Reorder::Keep);
    };
    let (body_start, mut rest_start) = (start(&[body])?, start(&rest)?);
    let mut body_first = body_start < rest_start;
    let after_jump = fall_through(ifstat);
    if body_first && after_jump.is_some_and(|f| pattern_if_else(ifstat, f, rest_start)) {
        return None;
    }
    if let Some(after_jump) = after_jump {
        if let Some((then_start, after)) = after_binding(after_jump).filter(|b| b.0 == body_start) {
            body_first = then_start == body_start;
            rest_start = after;
        } else if body_start != after_jump
            && rest_start != after_jump
            && (goto_away(after_jump, body_start) || goto_away(after_jump, rest_start))
        {
            let to_body = goto_at(after_jump).is_some_and(|(_, target)| target == body_start);
            return (!to_body).then_some(Reorder::Keep);
        }
    }
    let choice = if body_first {
        if ifdirect && !(next.is_dummy_exit() && exits_before(body, rest_start)) {
            Reorder::Else
        } else {
            Reorder::Keep
        }
    } else if exits_before(*rest.last()?, body_start) || !elsedirect {
        Reorder::Swap
    } else {
        Reorder::ElseSwapped
    };
    let else_branch = match choice {
        Reorder::Else => *rest.last()?,
        Reorder::ElseSwapped => body,
        _ => return Some(choice),
    };
    if next.is_do() || continues_loop(else_branch) {
        return None;
    }
    Some(choice)
}

/// Whether `stat` ends by continuing a loop: the end of a loop body, which
/// holds a `for` loop's update that `MergeHelper.matchFor` takes from the
/// body's last statement.
fn continues_loop(stat: Statement) -> bool {
    let mut last = stat;
    while last.is_sequence() {
        last = last.get_stats().borrow().get_last();
    }
    last.get_all_successor_edges()
        .iter()
        .any(|e| e.get_type() == StatEdge::TYPE_CONTINUE)
}

/// javac's choice for `ifstat`, an if without an else that ends its
/// sequence: [`Reorder::Swap`] turns `if (!c) { body }` back into
/// `if (c) continue; body` (or `break`) when javac wrote the jump as the
/// then-branch; `None` leaves Vineflower's choice.
pub fn reorder_last(ifstat: IfStatement) -> Option<Reorder> {
    if recorded(ifstat) {
        return None;
    }
    ifstat.get_ifstat()?;
    if loop_head(ifstat) {
        return None;
    }
    let edge = ifstat.get_all_successor_edges().first().copied()?;
    (loop_jump(ifstat.0, edge) && jumps_over_body(ifstat)).then_some(Reorder::Swap)
}

/// Whether `IfHelper.ifElseChainDenesting` would move the body of
/// `outer_if`, which javac placed first, behind the code that follows it.
pub fn keeps_denesting_out(outer_if: IfStatement, parent: SequenceStatement) -> bool {
    following(outer_if, parent).is_some_and(|rest| then_first(outer_if, &rest) == Some(true))
}

/// Whether the if-edge of `ifstat`, an if without a body followed by `rest`,
/// is a `continue` or `break` javac wrote as the then-branch (a `goto` right
/// after the condition), which `IfHelper.collapseElse` and `reorderIf` would
/// turn into `if (!c) { rest }`.
pub fn keeps_jump(ifstat: IfStatement, rest: &[Statement]) -> bool {
    let Some(edge) = ifstat.get_if_edge() else {
        return false;
    };
    ifstat.get_ifstat().is_none()
        && loop_jump(ifstat.0, edge)
        && start(&[edge.get_destination()]) != fall_through(ifstat)
        && then_first(ifstat, rest) == Some(true)
}

/// Whether `edge`, leaving `from`, is a `continue`, or a `break` to the
/// code after the innermost loop or switch around `from`.
fn loop_jump(from: Statement, edge: StatEdge) -> bool {
    if edge.get_type() == StatEdge::TYPE_CONTINUE {
        return true;
    }
    if edge.get_type() != StatEdge::TYPE_BREAK {
        return false;
    }
    let mut parent = from.get_parent();
    while let Some(p) = parent {
        if p.is_do() || p.is_switch() {
            return p
                .get_all_successor_edges()
                .iter()
                .any(|e| e.get_destination() == edge.get_destination());
        }
        parent = p.get_parent();
    }
    false
}

/// Whether javac placed the then-branch of `ifstat` (its body, or without one
/// the jump of its if-edge) before `rest`, the code after it.
fn then_first(ifstat: IfStatement, rest: &[Statement]) -> Option<bool> {
    let rest_start = start(rest)?;
    match ifstat.get_ifstat() {
        None => {
            let after_jump = fall_through(ifstat)?;
            if rest_start == after_jump {
                Some(false)
            } else if goto_away(after_jump, rest_start) {
                Some(true)
            } else {
                None
            }
        }
        Some(body) => Some(start(&[body])? < rest_start),
    }
}

/// Whether javac's then-branch of `ifstat` was a `goto` (gone from the
/// graph, its target taken as the if's successor) and the body is the code
/// its condition jumps to.
fn jumps_over_body(ifstat: IfStatement) -> bool {
    let (Some(body), Some(after_jump)) = (ifstat.get_ifstat(), fall_through(ifstat)) else {
        return false;
    };
    start(&[body]).is_some_and(|body_start| goto_away(after_jump, body_start))
}

/// Whether `ifstat` starts the body of an infinite loop: the loop condition
/// `MergeHelper.matchWhile` looks for, which javac lays out as a loop's.
fn loop_head(ifstat: IfStatement) -> bool {
    let mut stat = ifstat.0;
    while let Some(parent) = stat.get_parent() {
        if parent.get_first() != Some(stat) {
            return false;
        }
        if let Some(do_stat) = parent.try_as_do() {
            return do_stat.get_looptype() == LoopType::Infinite;
        }
        if !parent.is_sequence() {
            return false;
        }
        stat = parent;
    }
    false
}

/// The statements after `stat` in `parent`; `None` when there are none.
fn following(ifstat: IfStatement, parent: SequenceStatement) -> Option<Vec<Statement>> {
    let stats = parent.get_stats().borrow().to_vec();
    let index = stats.iter().position(|s| *s == ifstat.0)?;
    let rest = stats[index + 1..].to_vec();
    (!rest.is_empty()).then_some(rest)
}

/// Swaps the branches of if-else `ifstat` and negates its condition.
pub fn swap_else(ifstat: IfStatement) {
    let (Some(then_stat), Some(else_stat)) = (ifstat.get_ifstat(), ifstat.get_elsestat()) else {
        return;
    };
    let (if_edge, else_edge) = (ifstat.get_if_edge(), ifstat.get_else_edge());
    ifstat.set_ifstat(Some(else_stat));
    ifstat.set_elsestat(Some(then_stat));
    ifstat.set_if_edge(else_edge);
    ifstat.set_else_edge(if_edge);
    ifstat.set_negated(!ifstat.is_negated());
    {
        let stats = ifstat.get_stats();
        let mut stats = stats.borrow_mut();
        stats.remove_with_key(&then_stat.id());
        stats.remove_with_key(&else_stat.id());
        stats.add_with_key(else_stat, else_stat.id());
        stats.add_with_key(then_stat, then_stat.id());
    }
    let head = ifstat.get_headexprent();
    head.set_condition(
        FunctionExprent::new_single(FunctionType::BoolNot, head.get_condition(), None).0,
    );
}

/// The first bytecode offset of `stats`: of the first instruction of their
/// basic blocks (exprent offsets shift after a merged block's removed `goto`).
fn start(stats: &[Statement]) -> Option<i32> {
    stats.iter().filter_map(|s| first_offset(*s)).min()
}

fn first_offset(stat: Statement) -> Option<i32> {
    if let Some(block) = stat.try_as_basic_block() {
        let seq = block.get_block().get_seq();
        return (0..seq.length())
            .map(|i| seq.get_instr(i).with(|d| d.start_offset))
            .filter(|o| *o >= 0)
            .min();
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().filter_map(first_offset).min()
}

/// The offset after the last instruction of `ifstat`'s condition: where javac
/// put its then-branch.
fn fall_through(ifstat: IfStatement) -> Option<i32> {
    let mut offsets = BitSet::new();
    ifstat
        .get_headexprent()
        .get_bytecode_range(Some(&mut offsets));
    let condition_end = offsets.length() as i32;
    let head_end = ifstat
        .get_first()
        .and_then(|f| f.try_as_basic_block())
        .and_then(|b| b.get_block().get_last_instruction())
        .map_or(0, |i| i.with(|d| d.start_offset + d.length));
    let end = condition_end.max(head_end);
    (end > 0).then_some(end)
}

/// Whether `stat` ends with a `return` or `throw` javac placed before offset
/// `limit`: one of its own, not the method's shared last return that the
/// graph merged into it across a `goto` over the code at `limit`.
fn exits_before(stat: Statement, limit: i32) -> bool {
    let mut last = stat;
    loop {
        if let Some(exprents) = last.get_exprents() {
            if exprents.is_empty() || !exprents.get(exprents.len() - 1).is_exit() {
                return false;
            }
            let instr = last
                .try_as_basic_block()
                .and_then(|b| b.get_block().get_last_instruction());
            return match instr.map(|i| i.with(|d| (d.group, d.start_offset))) {
                Some((group, offset)) if group == GROUP_RETURN => offset < limit,
                _ => true,
            };
        }
        if !last.is_sequence() {
            return false;
        }
        last = last.get_stats().borrow().get_last();
    }
}
