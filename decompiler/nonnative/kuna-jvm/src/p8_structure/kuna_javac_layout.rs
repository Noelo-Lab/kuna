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
//! `buildIff` merge only exits that are one instruction. In loop bodies the
//! same evidence keeps an if-else that ends a body once loops are built,
//! a `continue` or `break` javac wrote ([`splits_else`]), two tests with
//! their own jumps apart ([`separate_test`], [`separate_jumps`]), a loop's
//! first or last if in the loop ([`inside_loop`]), and a try body's or
//! then-branch's last statement after it ([`place_tails`]). Docs:
//! `docs/spec/10-jvm.md` section 10.9.

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::basic_block::BasicBlock;
use crate::code_constants::{
    GROUP_RETURN, OPC_ALOAD, OPC_ASTORE, OPC_ATHROW, OPC_CHECKCAST, OPC_DSTORE, OPC_FSTORE,
    OPC_GOTO, OPC_GOTO_W, OPC_IFEQ, OPC_IFNONNULL, OPC_IFNULL, OPC_IF_ACMPNE, OPC_IRETURN,
    OPC_ISTORE, OPC_JSR, OPC_JSR_W, OPC_LOOKUPSWITCH, OPC_LSTORE, OPC_RET, OPC_RETURN,
    OPC_TABLESWITCH,
};
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationType;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_options::{self, JAVAC_LAYOUT};
use crate::rt::BitSet;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
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
    /// Whether `MethodProcessor`'s main loop has started: loops have been
    /// enhanced (`MergeHelper.enhanceLoops`) before any if is laid out.
    loops_built: bool,
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
            loops_built: false,
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

/// Whether javac reached `next`, a successor of `block`, by a `goto` over
/// other code (gone from the graph): the `return` of a value after a
/// try-catch whose handlers exit or after a `synchronized` block, which
/// `DeadCodeHelper.incorporateValueReturns` would take into the try.
pub fn jumped_to(block: BasicBlock, next: BasicBlock) -> bool {
    !block.get_seq().is_empty()
        && !next.get_seq().is_empty()
        && block.get_end_instruction() != next.get_start_instruction()
}

/// Notes that the main loop of `MethodProcessor.codeToJava` starts: from
/// here on every loop has been through `MergeHelper.enhanceLoops` before an
/// if in it is laid out.
pub fn loops_built() {
    METHOD.with(|m| {
        if let Some(method) = m.borrow_mut().as_mut() {
            method.loops_built = true;
        }
    });
}

fn are_loops_built() -> bool {
    METHOD.with(|m| m.borrow().as_ref().is_some_and(|method| method.loops_built))
}

/// Whether an if-else whose else-branch (`choice`) ends a loop body can be
/// made from `rest`: loops are built (a `for` loop's update is out of the
/// body), the loop is not a pattern switch's, `rest` does not end in an
/// infinite loop's last if (the condition
/// `MergeHelper.matchDoWhile` takes), and with [`Reorder::ElseSwapped`] javac
/// placed all of `rest` before the body (a block that goes on after it holds
/// code that follows the if, the update of a `for` loop).
fn loop_end_ok(rest: &[Statement], choice: Reorder, body_start: i32) -> bool {
    if !are_loops_built() {
        return false;
    }
    if rest.first().is_none_or(|f| in_pattern_switch(*f)) {
        return false;
    }
    if choice == Reorder::ElseSwapped && end(rest).is_none_or(|e| e > body_start) {
        return false;
    }
    let Some(&last) = rest.last() else {
        return false;
    };
    let mut tail = last;
    while tail.is_sequence() {
        tail = tail.get_stats().borrow().get_last();
    }
    if let Some(lastif) = tail.try_as_if() {
        if lastif.iftype() == IfStatement::IFTYPE_IF
            && lastif.get_ifstat().is_none()
            && enclosing_loop(lastif.0).is_some_and(|d| d.get_looptype() == LoopType::Infinite)
        {
            return false;
        }
    }
    true
}

/// Whether `loop_` is the loop javac wraps around a pattern `switch`
/// (`typeSwitch` or `enumSwitch`, whose failing guards continue it with the
/// next label): `SwitchPatternMatchProcessor` recovers the switch from
/// Vineflower's layout of its body.
fn pattern_switch_loop(loop_: Statement) -> bool {
    let mut first = loop_.get_first();
    while let Some(seq) = first.filter(|f| f.is_sequence()) {
        first = seq.get_first();
    }
    first.is_some_and(pattern_switch)
}

fn pattern_switch(stat: Statement) -> bool {
    stat.try_as_switch()
        .and_then(|sw| sw.get_headexprent().try_as_switch_head())
        .and_then(|head| head.get_value().try_as_invocation())
        .is_some_and(|invoc| {
            invoc.get_invocation_type() == InvocationType::Dynamic
                && matches!(invoc.get_name().as_str(), "typeSwitch" | "enumSwitch")
        })
}

/// Whether `stat` is in a pattern `switch` or its loop ([`pattern_switch_loop`]),
/// where the loop rules leave Vineflower's layout for the switch's recovery.
fn in_pattern_switch(stat: Statement) -> bool {
    let mut parent = stat.get_parent();
    while let Some(p) = parent {
        if pattern_switch(p) || (p.is_do() && pattern_switch_loop(p)) {
            return true;
        }
        parent = p.get_parent();
    }
    false
}

/// The innermost loop around `stat`.
fn enclosing_loop(stat: Statement) -> Option<DoStatement> {
    let mut parent = stat.get_parent();
    while let Some(p) = parent {
        if let Some(d) = p.try_as_do() {
            return Some(d);
        }
        parent = p.get_parent();
    }
    None
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
    with_code(|code| match code.binary_search_by_key(&offset, |c| c.0) {
        Ok(index) => code[index..].iter().take(n).copied().collect(),
        Err(_) => Vec::new(),
    })
}

/// `f` of the instructions of the method being structured, sorted by offset.
fn with_code<R>(f: impl FnOnce(&[(i32, i32, i32, i32)]) -> R) -> R {
    METHOD.with(|m| {
        let mut m = m.borrow_mut();
        let Some(method) = m.as_mut() else {
            return f(&[]);
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
        f(code)
    })
}

/// The instruction that ends right before the one at `offset`.
fn instr_before(offset: i32) -> Option<(i32, i32, i32, i32)> {
    with_code(|code| {
        let index = code.binary_search_by_key(&offset, |c| c.0).ok()?;
        code.get(index.checked_sub(1)?).copied()
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
/// over the then-branch, which belong to the condition. The graph may take
/// the binding for the start of the code after the then-branch (the `goto`
/// gone) and the then-branch for the code after the if: [`reorder`] then
/// negates the if.
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
    if loop_head(ifstat) && !body_kept_in_loop(ifstat) {
        return None;
    }
    let rest = following(ifstat, parent)?;
    let Some(body) = ifstat.get_ifstat() else {
        return keeps_jump(ifstat, &rest).then_some(Reorder::Keep);
    };
    let (mut body_start, mut rest_start) = (
        start(&[body]).or_else(|| copied_exit_start(body))?,
        start(&rest).or_else(|| {
            copied_exit_start(body)?;
            rest.iter().filter_map(|s| code_start(*s)).min()
        })?,
    );
    let mut body_first = body_start < rest_start;
    let after_jump = fall_through(ifstat);
    if body_first && after_jump.is_some_and(|f| pattern_if_else(ifstat, f, rest_start)) {
        return None;
    }
    if let Some(after_jump) = after_jump {
        if let Some((then_start, after)) = after_binding(after_jump).filter(|b| b.0 == body_start) {
            body_first = then_start == body_start;
            rest_start = after;
        } else if let Some((_, after)) =
            after_binding(after_jump).filter(|b| body_start == after_jump && b.0 == rest_start)
        {
            body_first = false;
            body_start = after;
        } else if body_start != after_jump
            && rest_start != after_jump
            && (goto_away(after_jump, body_start) || goto_away(after_jump, rest_start))
        {
            let to_body = goto_at(after_jump).is_some_and(|(_, target)| target == body_start);
            return (!to_body).then_some(Reorder::Keep);
        }
    }
    let choice = if body_first {
        if jumps_out(body, rest_start, &rest) {
            Reorder::Keep
        } else if ifdirect && !(next.is_dummy_exit() && exits_before(body, rest_start)) {
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
    if (next.is_do() || continues_loop(else_branch)) && !loop_end_ok(&rest, choice, body_start) {
        return None;
    }
    Some(choice)
}

/// Whether `body`, the then-branch javac placed before `rest_start`, ends
/// with a `continue` or `break` of the source: its last statement's edge
/// continues or leaves a loop, and the `goto` javac placed last in it goes
/// right to that loop's head or past the loop's code, where javac's jump
/// over an else-branch goes to the if-else's end inside the loop. When the
/// if-else would end the loop body, javac sends that jump to the loop's head
/// too unless a local of the body goes out of scope at the end of `rest`
/// (javac resolves pending jumps when it closes a scope, and otherwise
/// chains them into the back edge): without such a scope, a `goto` to the
/// head is either form and the if stays Vineflower's. A `continue` of a
/// `for` loop goes to its update, where the end of an if-else ending its
/// body goes too.
fn jumps_out(body: Statement, rest_start: i32, rest: &[Statement]) -> bool {
    let Some((start, length, target)) = goto_before(rest_start) else {
        return false;
    };
    if start + length != rest_start {
        return false;
    }
    let Some((loop_, continues)) =
        jump_loop(body).filter(|l| l.0.is_do() && (target > start) != l.1)
    else {
        return false;
    };
    if in_pattern_switch(body) {
        return false;
    }
    if !continues {
        return loop_exit(loop_).is_some_and(|exit| target >= exit);
    }
    let Some(loop_start) = first_offset(loop_).filter(|s| target <= *s) else {
        return false;
    };
    rest_end(rest_start, rest).is_some_and(|end| scope_ends_at(end, loop_start))
}

/// The offset after `rest`, which starts at `rest_start`, and the back edges
/// of its loops.
fn rest_end(rest_start: i32, rest: &[Statement]) -> Option<i32> {
    let mut end = extent_end(rest)?;
    while let Some((length, _)) = goto_at(end).filter(|g| (rest_start..end).contains(&g.1)) {
        end += length;
    }
    Some(end)
}

/// The loop or switch the last statement of `stat` continues (`true`) or
/// leaves.
fn jump_loop(stat: Statement) -> Option<(Statement, bool)> {
    let mut last = stat;
    while last.is_sequence() {
        last = last.get_stats().borrow().get_last();
    }
    let edge = last.get_all_successor_edges().first().copied()?;
    if edge.get_type() == StatEdge::TYPE_CONTINUE {
        return Some((edge.get_destination(), true));
    }
    if edge.get_type() != StatEdge::TYPE_BREAK {
        return None;
    }
    let mut parent = last.get_parent();
    while let Some(p) = parent {
        if p.is_do() || p.is_switch() {
            return p
                .get_all_successor_edges()
                .iter()
                .any(|e| e.get_destination() == edge.get_destination())
                .then_some((p, false));
        }
        parent = p.get_parent();
    }
    None
}

/// The offset after the code of `loop_`: its last instruction and the back
/// edges that follow it.
fn loop_exit(loop_: Statement) -> Option<i32> {
    let mut end = extent_end(&[loop_])?;
    while let Some((length, _)) = goto_at(end).filter(|g| g.1 < end) {
        end += length;
    }
    Some(end)
}

/// Whether a local of the LocalVariableTable declared after `loop_start`
/// (in the loop's body) goes out of scope at `offset`.
fn scope_ends_at(offset: i32, loop_start: i32) -> bool {
    METHOD.with(|m| {
        m.borrow().as_ref().is_some_and(|method| {
            method.mt.get_local_variable_attr().is_some_and(|lvt| {
                lvt.get_variables()
                    .iter()
                    .any(|v| v.start_pc > loop_start && v.start_pc + v.length == offset)
            })
        })
    })
}

/// The `goto` that ends right at `offset`: its offset, length and target.
fn goto_before(offset: i32) -> Option<(i32, i32, i32)> {
    let (start, opcode, length, operand) = instr_before(offset)?;
    (opcode == OPC_GOTO || opcode == OPC_GOTO_W).then_some((start, length, start + operand))
}

/// The offset after the last instruction of the blocks of `stats`.
fn extent_end(stats: &[Statement]) -> Option<i32> {
    fn block_end(stat: Statement) -> Option<i32> {
        if let Some(block) = stat.try_as_basic_block() {
            let seq = block.get_block().get_seq();
            return (0..seq.length())
                .map(|i| seq.get_instr(i).with(|d| d.start_offset + d.length))
                .max();
        }
        let stats = stat.get_stats().borrow().to_vec();
        stats.into_iter().filter_map(block_end).max()
    }
    stats.iter().filter_map(|s| block_end(*s)).max()
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
    (loop_jump(ifstat.0, edge)
        && (jumps_over_body(ifstat) || ifstat.get_ifstat().is_some_and(|b| or_jump(ifstat, b))))
    .then_some(Reorder::Swap)
}

/// Whether `ifstat`, followed by `next` (its body or the code after it), is
/// the first test of javac's `if (a || b) continue;` (or `break`): its jump
/// goes where the `goto` after the last test goes, each test between
/// jumping there too. javac emits a test's jump to code that starts with a
/// `goto` as a jump to the `goto`'s target, so the first test of `a || b`
/// jumps right to the loop; an if of its own ending a loop body jumps there
/// too, but the code after its test is not such a chain of tests.
fn or_jump(ifstat: IfStatement, next: Statement) -> bool {
    let Some(after_jump) = fall_through(ifstat) else {
        return false;
    };
    let Some(target) = cond_jump_target(after_jump) else {
        return false;
    };
    if in_pattern_switch(ifstat.0) || start(&[next]) != Some(after_jump) {
        return false;
    }
    let mut test = next;
    loop {
        while test.is_sequence() {
            let Some(first) = test.get_first() else {
                return false;
            };
            test = first;
        }
        let Some(next) = test.try_as_if() else {
            return false;
        };
        let (Some(head), Some(end)) = (next.get_first(), fall_through(next)) else {
            return false;
        };
        if head.get_exprents().is_none_or(|e| !e.is_empty()) {
            return false;
        }
        if goto_at(end).is_some_and(|g| g.1 == target) {
            return true;
        }
        if cond_jump_target(end) != Some(target) {
            return false;
        }
        let Some(inner) = next.get_ifstat().filter(|b| start(&[*b]) == Some(end)) else {
            return false;
        };
        test = inner;
    }
}

/// Whether `IfHelper.ifElseChainDenesting` would move the body of
/// `outer_if`, which javac placed first, behind the code that follows it.
pub fn keeps_denesting_out(outer_if: IfStatement, parent: SequenceStatement) -> bool {
    following(outer_if, parent).is_some_and(|rest| then_first(outer_if, &rest) == Some(true))
}

/// Whether the if-edge of `ifstat`, an if without a body followed by `rest`,
/// is a `continue` or `break` javac wrote as the then-branch (a `goto` right
/// after the condition), which `IfHelper.collapseElse` and `reorderIf` would
/// turn into `if (!c) { rest }`; or an exit javac wrote there with copies of
/// the code a try-with-resources, `finally` or `synchronized` runs on the
/// way out ([`copied_exit`]).
pub fn keeps_jump(ifstat: IfStatement, rest: &[Statement]) -> bool {
    let Some(edge) = ifstat.get_if_edge() else {
        return false;
    };
    ifstat.get_ifstat().is_none()
        && ((loop_jump(ifstat.0, edge)
            && start(&[edge.get_destination()]) != fall_through(ifstat)
            && (then_first(ifstat, rest) == Some(true)
                || rest.first().is_some_and(|next| or_jump(ifstat, *next))))
            || copied_exit(ifstat, edge, rest))
}

/// Whether `edge`, the if-edge of `ifstat`, is a `return`, `throw`,
/// `break` or `continue` the source wrote right after the condition, inside
/// a try-with-resources, `finally` or `synchronized` block: the condition
/// jumps to `rest`, and the code between is javac's then-branch, copies of
/// the close call, finally block or `monitorexit` (code the structurer
/// took out: no statement holds it) and then the exit: a return or throw,
/// or the `goto` of a `continue` or `break` of the loop around. No other
/// jump goes to that code (the then-branch of `a || b` is the target of
/// `a`'s jump).
fn copied_exit(ifstat: IfStatement, edge: StatEdge, rest: &[Statement]) -> bool {
    if edge.get_type() != StatEdge::TYPE_BREAK && edge.get_type() != StatEdge::TYPE_CONTINUE {
        return false;
    }
    let (Some(after_jump), Some(rest_start)) = (fall_through(ifstat), start(rest)) else {
        return false;
    };
    if rest_start <= after_jump || cond_jump_target(after_jump) != Some(rest_start) {
        return false;
    }
    if in_pattern_switch(ifstat.0) || !in_protected_exit(ifstat.0) {
        return false;
    }
    let Some((exit_at, opcode, _, _)) = instr_before(rest_start) else {
        return false;
    };
    let exits = if opcode == OPC_GOTO || opcode == OPC_GOTO_W {
        loop_jump(ifstat.0, edge)
    } else {
        edge.get_type() == StatEdge::TYPE_BREAK
            && (opcode == OPC_ATHROW || (OPC_IRETURN..=OPC_RETURN).contains(&opcode))
    };
    exits
        && exit_at > after_jump
        && !jumped_into(after_jump)
        && !holds_code(ifstat.0.get_top_parent().0, after_jump, exit_at)
}

/// Whether an instruction of the method being structured jumps to `offset`.
fn jumped_into(offset: i32) -> bool {
    with_code(|code| {
        code.iter().any(|&(start, opcode, _, operand)| {
            (is_conditional(opcode) || opcode == OPC_GOTO || opcode == OPC_GOTO_W)
                && start + operand == offset
        })
    })
}

/// Whether a block or an expression in `stat` still holds an instruction at
/// an offset in `from..to`.
fn holds_code(stat: Statement, from: i32, to: i32) -> bool {
    let range = from..to;
    if let Some(block) = stat.try_as_basic_block() {
        let seq = block.get_block().get_seq();
        if (0..seq.length()).any(|i| range.contains(&seq.get_instr(i).with(|d| d.start_offset))) {
            return true;
        }
    }
    let mut exprents = stat.get_stat_exprents();
    if let Some(own) = stat.get_exprents() {
        exprents.extend(own.to_vec());
    }
    for exprent in exprents {
        for sub in exprent.get_all_exprents_rec_self(true, true) {
            if let Some(offsets) = sub.get_bytecode() {
                if offsets.iter().any(|o| range.contains(&(o as i32))) {
                    return true;
                }
            }
        }
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().any(|s| holds_code(s, from, to))
}

/// The bytecode offset of `body` when it is only an exit `ExitHelper`
/// copied into an if from a block of its own ([`copied_exit`]): a block
/// without instructions whose last expression came from the exit's.
fn copied_exit_start(body: Statement) -> Option<i32> {
    let exprents = body.get_exprents()?;
    let last = exprents.get(exprents.len().checked_sub(1)?);
    if first_offset(body).is_some() || !last.is_exit() || !in_protected_exit(body) {
        return None;
    }
    code_start(body)
}

/// Whether `ExitHelper.integrateExits` must leave the if-edge of `ifstat`,
/// an if without a body, where it goes instead of making the exit after the
/// statement that follows the if (or after the if's own break) its body:
/// that body runs where the if-edge went, which is right only for the
/// `switch` on strings the step was written for. Vineflower's own layout
/// never leaves such an if outside a switch; the option's can (a test of
/// `a || b` kept as an if of its own would return the value of the code
/// after it).
pub fn keeps_if_edge(ifstat: IfStatement) -> bool {
    if !active() {
        return false;
    }
    let parent = ifstat.get_parent();
    let in_switch = parent.is_some_and(|p| {
        p.is_switch() || (p.is_sequence() && p.get_parent().is_some_and(|g| g.is_switch()))
    });
    !in_switch
}

/// Whether `stat` is inside a try (with resources), a `finally` or a
/// `synchronized` block, whose exits javac precedes with copies.
fn in_protected_exit(stat: Statement) -> bool {
    let mut parent = stat.get_parent();
    while let Some(p) = parent {
        if p.is_catch() || p.is_catch_all() || p.is_synchronized() {
            return true;
        }
        parent = p.get_parent();
    }
    false
}

/// Whether `first`, an if the structurer would join with the next test into
/// one `&&`/`||` condition, is a statement of its own in javac's layout: its
/// condition is followed by a `goto` (javac's then-branch `continue`,
/// `break` or jump to the end of an if-else), where a compound condition
/// goes on with its next test.
pub fn separate_test(first: IfStatement) -> bool {
    !in_pattern_switch(first.0)
        && fall_through(first)
            .and_then(|after_jump| {
                goto_at(after_jump).map(|(length, target)| target != after_jump + length)
            })
            .unwrap_or(false)
}

/// Whether `first` and `second`, two ifs whose jumps the structurer would
/// join into `first || second`, are javac's loop condition and an
/// `if (..) break;` after it: `first` starts the body of an infinite loop
/// (the condition `MergeHelper.matchWhile` takes) and jumps where the
/// `goto` of the second one's then-branch goes (its head holds no statement,
/// as `matchWhile` wants a loop condition's). A loop condition `a && b`
/// has no such goto; the same bytecode as `for (;;) { if (a || b) break; }`,
/// which is not the loop condition either.
pub fn separate_jumps(first: IfStatement, second: IfStatement) -> bool {
    let (Some(first_end), Some(then_start)) = (fall_through(first), fall_through(second)) else {
        return false;
    };
    loop_head(first)
        && first
            .get_first()
            .and_then(|head| head.get_exprents())
            .is_some_and(|exprents| exprents.is_empty())
        && !in_pattern_switch(first.0)
        && goto_at(then_start)
            .is_some_and(|(_, target)| cond_jump_target(first_end) == Some(target))
}

fn is_conditional(opcode: i32) -> bool {
    (OPC_IFEQ..=OPC_IF_ACMPNE).contains(&opcode) || opcode == OPC_IFNULL || opcode == OPC_IFNONNULL
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
            } else if goto_away(after_jump, rest_start)
                || (!in_pattern_switch(ifstat.0) && jumps_over_goto(after_jump, rest_start))
            {
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
    start(&[body]).is_some_and(|body_start| {
        goto_away(after_jump, body_start)
            || (!in_pattern_switch(ifstat.0) && jumps_over_goto(after_jump, body_start))
    })
}

/// Whether the condition ending at `after_jump` jumps over a `goto` right
/// after it (javac's `if (c) continue;`, `break` or jump to an else-branch's
/// end) to the code that starts at `code_start` or before it: measured on
/// the conditional jump, as that code may have lost its first instructions
/// to a loop header (a `for` loop's init).
fn jumps_over_goto(after_jump: i32, code_start: i32) -> bool {
    goto_at(after_jump).is_some_and(|(length, target)| {
        let next = after_jump + length;
        target != next
            && cond_jump_target(after_jump) == Some(next)
            && code_start >= next
            && (target < next || code_start < target)
    })
}

/// The target of the conditional jump that ends right at `offset`.
fn cond_jump_target(offset: i32) -> Option<i32> {
    let (start, opcode, length, operand) = instr_before(offset)?;
    (start + length == offset && is_conditional(opcode)).then_some(start + operand)
}

/// Whether `ifstat`, the first statement of an infinite loop, is an if of
/// the loop's body, not its condition: its condition jumps forward inside
/// javac's loop (a `while` condition jumps past it), and its body stays in
/// the loop ([`inside_loop`]), so neither `MergeHelper.matchWhile` nor
/// `LoopExtractHelper` makes it the loop's condition.
fn body_kept_in_loop(ifstat: IfStatement) -> bool {
    let (Some(_), Some(loop_), Some(after_jump)) = (
        ifstat.get_ifstat(),
        enclosing_loop(ifstat.0),
        fall_through(ifstat),
    ) else {
        return false;
    };
    let (Some(target), Some(end)) = (cond_jump_target(after_jump), javac_loop_end(loop_)) else {
        return false;
    };
    target > after_jump && target < end && inside_loop(loop_, ifstat)
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

/// Whether if-else `ifstat` is javac's `if (c) { ..; continue; } rest` (or
/// `break`): the then-branch javac placed first ends with a `goto` that
/// leaves the loop body or goes back to its head, not over the else-branch,
/// and its last statement's edge is that jump.
pub fn splits_else(ifstat: IfStatement) -> bool {
    if recorded(ifstat) || ifstat.is_pattern_matched() || in_pattern_switch(ifstat.0) {
        return false;
    }
    let (Some(then_stat), Some(else_stat)) = (ifstat.get_ifstat(), ifstat.get_elsestat()) else {
        return false;
    };
    let (Some(then_start), Some(else_start)) = (start(&[then_stat]), start(&[else_stat])) else {
        return false;
    };
    then_start < else_start && jumps_out(then_stat, else_start, &[else_stat])
}

/// Makes the else-branch of `ifstat` the statement after it.
pub fn split_else(ifstat: IfStatement) {
    let else_stat = ifstat.get_elsestat().expect("else");
    let else_edge = ifstat.get_else_edge().expect("else edge");
    ifstat.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
    let seq = SequenceStatement::new_varargs(&[ifstat.0, else_stat]);
    ifstat
        .get_stats()
        .borrow_mut()
        .remove_with_key(&else_stat.id());
    ifstat.set_elsestat(None);
    ifstat.set_else_edge(None);
    ifstat.0.replace_with(seq.0);
    else_edge.change_source(ifstat.0);
    seq.set_all_parent();
}

/// Whether javac placed the body of `ifstat`, the first or last if of
/// infinite loop `loop_` that `LoopExtractHelper` would move after the
/// loop, inside the loop: before its last jump back to its start, where the
/// code after a loop follows it.
pub fn inside_loop(loop_: DoStatement, ifstat: IfStatement) -> bool {
    let Some(body) = ifstat.get_ifstat().filter(|_| !in_pattern_switch(ifstat.0)) else {
        return false;
    };
    match (start(&[body]), javac_loop_end(loop_)) {
        (Some(body_start), Some(end)) => body_start < end,
        _ => false,
    }
}

/// The if in the body of infinite loop `loop_` (in its sequences and those
/// of the loops in it) whose body javac placed after the loop's code and
/// reached by a `break` of its condition ([`exit_jump`]): the code after
/// the loop, which the structurer took into the loop as the if's body. An
/// if whose body follows the code of a loop around `loop_` too is that
/// loop's.
pub fn tail_if(loop_: DoStatement) -> Option<IfStatement> {
    if pattern_switch_loop(loop_.0) || in_pattern_switch(loop_.0) {
        return None;
    }
    let end = loop_code_end(loop_)?;
    let mut todo = vec![loop_.get_first()?];
    while let Some(stat) = todo.pop() {
        if stat.is_sequence() {
            todo.extend(stat.get_stats().borrow().to_vec());
        } else if let Some(inner) = stat.try_as_do() {
            todo.extend(inner.get_first());
        } else if let Some(ifstat) = stat
            .try_as_if()
            .filter(|i| i.iftype() == IfStatement::IFTYPE_IF)
        {
            let (Some(body_start), Some(after_jump)) =
                (ifstat.get_ifstat().and_then(code_start), fall_through(ifstat))
            else {
                continue;
            };
            if body_start >= end
                && exit_jump(loop_.0, after_jump, body_start)
                && !outer_tail(loop_, body_start)
            {
                return Some(ifstat);
            }
        }
    }
    None
}

/// Whether `start` is after the code of a loop around `loop_`.
fn outer_tail(loop_: DoStatement, start: i32) -> bool {
    let mut parent = loop_.get_parent();
    while let Some(p) = parent {
        if let Some(outer) = p.try_as_do() {
            if loop_code_end(outer).is_some_and(|end| start >= end) {
                return true;
            }
        }
        parent = p.get_parent();
    }
    false
}

/// Where javac placed `init`, the assignment before while loop `loop_`, and
/// `update`, the last statement of its body: in a `for` header or not. A
/// local `init` declares whose LocalVariableTable scope ends where the loop
/// exits is the header's (`None`: Vineflower's choice stands); otherwise
/// the LineNumberTable decides, as javac gives a `for` header's parts the
/// line of its condition and a statement before or in a `while` loop its
/// own. `None` without a table or offsets.
pub fn for_header(
    loop_: DoStatement,
    init: Option<Exprent>,
    update: Exprent,
) -> Option<(bool, bool)> {
    let scope_end = init
        .and_then(|i| i.try_as_assignment())
        .and_then(|a| a.get_left().try_as_var())
        .and_then(|v| v.get_lvt())
        .map(|lvt| lvt.get_end());
    if let (Some(scope_end), Some(exit)) = (scope_end, loop_code_end(loop_)) {
        if scope_end <= exit {
            return None;
        }
    }
    let line = |exprent: Exprent| exprent_start(exprent).and_then(line_at);
    let cond_line = line(loop_.get_condition_exprent()?)?;
    let update_line = line(update)?;
    let init_line = match init {
        Some(init) => line(init)? == cond_line,
        None => false,
    };
    Some((init_line, update_line == cond_line))
}

/// The source line of the instruction at `pc` in the LineNumberTable of the
/// method being structured.
fn line_at(pc: i32) -> Option<i32> {
    METHOD.with(|m| {
        let m = m.borrow();
        let table = m
            .as_ref()?
            .mt
            .get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE)?;
        let line = table.find_line_number(pc);
        (line >= 0).then_some(line)
    })
}

/// Where javac's code of `loop_` ends: after the last jump back to its
/// first instruction (a `goto`, or a `do`-`while` condition).
fn javac_loop_end(loop_: DoStatement) -> Option<i32> {
    back_jumps_end(first_offset(loop_.0)?)
}

/// The offset after the last jump back to `loop_start`.
fn back_jumps_end(loop_start: i32) -> Option<i32> {
    with_code(|code| {
        code.iter()
            .filter(|&&(offset, opcode, _, operand)| {
                (opcode == OPC_GOTO || opcode == OPC_GOTO_W || is_conditional(opcode))
                    && offset > loop_start
                    && offset + operand == loop_start
            })
            .map(|&(offset, _, length, _)| offset + length)
            .max()
    })
}

/// The offset of the last instruction of the blocks of `stat`, those of
/// `skip` left out.
fn max_offset(stat: Statement, skip: Option<Statement>) -> Option<i32> {
    if Some(stat) == skip {
        return None;
    }
    if let Some(block) = stat.try_as_basic_block() {
        let seq = block.get_block().get_seq();
        return (0..seq.length())
            .map(|i| seq.get_instr(i).with(|d| d.start_offset))
            .max();
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().filter_map(|s| max_offset(s, skip)).max()
}

/// Moves the last statement of a try body, or of the branch of an if-else
/// javac placed first, out after the try-catch or if-else when javac placed
/// that statement after all of their code: `DomHelper` makes code only one
/// branch reaches part of that branch, `try { ..; return x; } catch (..) {
/// throw ..; }`, where the protected range shows `try { .. } catch (..) {
/// throw ..; } return x;`. javac ends those two branches with a `goto` over
/// the rest; the last catch or else-branch runs into the code after it, so
/// its end does not show. A negated `instanceof` pattern test is left to
/// the pattern recovery (its binding and `goto` belong to the condition).
/// The edges into the statement become breaks out of the try-catch or
/// if-else, which `InlineSingleBlockHelper` does not inline again.
pub fn place_tails(stat: Statement) -> bool {
    let mut res = false;
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats {
        res |= place_tails(st);
    }
    let (branch, other) = if stat.is_catch() {
        let stats = stat.get_stats().borrow().to_vec();
        (stat.get_first(), stats.get(1).copied())
    } else if let Some(ifstat) = stat
        .try_as_if()
        .filter(|i| i.iftype() == IfStatement::IFTYPE_IFELSE)
    {
        let (Some(then_stat), Some(else_stat)) = (ifstat.get_ifstat(), ifstat.get_elsestat())
        else {
            return res;
        };
        if fall_through(ifstat).and_then(after_binding).is_some() {
            return res;
        }
        match (first_offset(then_stat), first_offset(else_stat)) {
            (Some(a), Some(b)) if a < b => (Some(then_stat), Some(else_stat)),
            (Some(_), Some(_)) => (Some(else_stat), Some(then_stat)),
            _ => (None, None),
        }
    } else {
        return res;
    };
    let Some(other_start) = other.and_then(first_offset) else {
        return res;
    };
    if !stat.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty() {
        return res;
    }
    let Some(seq) = branch.and_then(|b| b.try_as_sequence()) else {
        return res;
    };
    let (len, tail) = {
        let stats = seq.get_stats();
        let stats = stats.borrow();
        (stats.len(), stats.get_last())
    };
    if len < 2 || !movable_tail(stat, tail, other_start) {
        return res;
    }
    move_tail(stat, seq, tail);
    true
}

/// Whether `tail`, the last statement of a branch of `stat`, is code javac
/// placed after all of `stat`, at the target of the `goto` that ends the
/// branch right before the other branch or the handler (at `other_start`),
/// that only that branch reaches and that leaves it without coming back.
fn movable_tail(stat: Statement, tail: Statement, other_start: i32) -> bool {
    let (Some(rest_end), Some(tail_start)) = (max_offset(stat, Some(tail)), first_offset(tail))
    else {
        return false;
    };
    tail_start > rest_end
        && !bare_return(tail)
        && goto_before(other_start).is_some_and(|(_, _, target)| target == tail_start)
        && tail.get_all_predecessor_edges().iter().all(|e| {
            stat.contains_statement_strict(e.get_source())
                && !tail.contains_statement(e.get_source())
        })
        && no_edge_into(tail, stat, tail)
}

/// Whether `tail`, the statement after `pre` that `InlineSingleBlockHelper`
/// would move to its only break, from `source` inside `pre` (an infinite
/// loop or a switch), is code javac placed after all of `pre` and reached
/// by the jump of a `break` ([`exit_jump`]). Code the source wrote at the
/// break is javac's there, inside `pre`; code a switch's last case runs
/// into, or a branch an if-else chain placed after a loop's last jump back,
/// follows its test without a `goto`.
pub fn keeps_tail(pre: Statement, tail: Statement, source: Statement) -> bool {
    if !(pre.is_do() || pre.is_switch()) || pattern_switch(pre) || in_pattern_switch(pre) {
        return false;
    }
    if pre.is_do() && pattern_switch_loop(pre) {
        return false;
    }
    let (Some(tail_start), Some(source_end)) = (code_start(tail), extent_end(&[source])) else {
        return false;
    };
    exit_jump(pre, source_end, tail_start)
}

/// Whether the code ending at `offset` inside `pre`, a loop or a switch,
/// leaves it for `target`, code javac placed after all of `pre`: a `break`'s
/// `goto` (gone from the graph) follows it, or for a loop it is the
/// conditional jump of javac's loop condition ([`head_exit`]). A loop's code
/// ends after its last jump back to its start or at its condition's exit,
/// a switch's after its last case.
fn exit_jump(pre: Statement, offset: i32, target: i32) -> bool {
    let by_goto = goto_at(offset).is_some_and(|g| g.1 == target);
    match pre.try_as_do() {
        Some(loop_) => {
            let head = head_exit(loop_);
            let by_condition = cond_jump_target(offset) == Some(target) && head == Some(target);
            (by_goto || by_condition) && loop_code_end(loop_).is_some_and(|end| target >= end)
        }
        None => by_goto && max_offset(pre, None).is_some_and(|last| target > last),
    }
}

/// Where javac's code of `loop_` ends: after its last jump back to its
/// start (its condition's, when the loop took it from a block), or at the
/// exit of its condition when that is later.
fn loop_code_end(loop_: DoStatement) -> Option<i32> {
    back_jumps_end(code_start(loop_.0)?).max(head_exit(loop_))
}

/// The target of the first branch of `loop_`'s code when it is a conditional
/// jump forward: the exit javac's `while` or `for` condition jumps to.
fn head_exit(loop_: DoStatement) -> Option<i32> {
    let start = code_start(loop_.0)?;
    with_code(|code| {
        let index = code.binary_search_by_key(&start, |c| c.0).ok()?;
        let &(offset, opcode, _, operand) = code[index..].iter().find(|c| {
            is_conditional(c.1)
                || matches!(
                    c.1,
                    OPC_GOTO
                        | OPC_GOTO_W
                        | OPC_TABLESWITCH
                        | OPC_LOOKUPSWITCH
                        | OPC_ATHROW
                        | OPC_JSR
                        | OPC_JSR_W
                        | OPC_RET
                )
                || (OPC_IRETURN..=OPC_RETURN).contains(&c.1)
        })?;
        (is_conditional(opcode) && operand > 0).then_some(offset + operand)
    })
}

/// Whether `stat` is only a `return;`, which is not printed at the end of a
/// method: moving it would only move the declarations of the locals around.
fn bare_return(stat: Statement) -> bool {
    stat.get_exprents().is_some_and(|exprents| {
        exprents.len() == 1
            && exprents
                .get(0)
                .try_as_exit()
                .is_some_and(|e| e.get_exit_type() == ExitType::Return && e.get_value().is_none())
    })
}

/// Whether no edge from `from` (or a statement in it) goes to a statement of
/// `stat` outside `tail`.
fn no_edge_into(from: Statement, stat: Statement, tail: Statement) -> bool {
    from.get_all_successor_edges().iter().all(|e| {
        let d = e.get_destination();
        !(stat.contains_statement(d) && !tail.contains_statement(d))
    }) && from
        .get_stats()
        .borrow()
        .to_vec()
        .into_iter()
        .all(|s| no_edge_into(s, stat, tail))
}

fn move_tail(stat: Statement, seq: SequenceStatement, tail: Statement) {
    seq.get_stats().borrow_mut().remove_with_key(&tail.id());
    for edge in tail.get_all_predecessor_edges() {
        if edge.get_type() == StatEdge::TYPE_REGULAR {
            edge.get_source()
                .change_edge_type(EdgeDirection::Forward, edge, StatEdge::TYPE_BREAK);
        }
        if edge.closure().is_none_or(|c| stat.contains_statement(c)) {
            stat.add_labeled_edge(edge);
        }
        edge.with_mut(|d| d.can_inline = false);
    }
    let parent = stat.get_parent().expect("parent");
    let block = SequenceStatement::new(vec![stat, tail]);
    parent.replace_statement(stat, block.0);
    block.set_all_parent();
    stat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, stat, tail));
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

/// The last bytecode offset of the code `stats` still hold: of the
/// instructions their exprents came from (an exprent moved out, a `for`
/// loop's update, leaves its instructions in the block).
fn end(stats: &[Statement]) -> Option<i32> {
    stats.iter().filter_map(|s| last_offset(*s)).max()
}

fn last_offset(stat: Statement) -> Option<i32> {
    if let Some(block) = stat.try_as_basic_block() {
        let mut live = BitSet::new();
        for e in stat.get_exprents()?.to_vec() {
            for sub in e.get_all_exprents_rec_self(true, true) {
                if let Some(offsets) = sub.get_bytecode() {
                    live.or(&offsets);
                }
            }
        }
        let block = block.get_block();
        let (seq, old) = (block.get_seq(), block.get_instr_old_offsets());
        return (0..seq.length())
            .filter(|i| {
                let i = *i as usize;
                i < old.len() && old.get(i) >= 0 && live.get(old.get(i) as usize)
            })
            .map(|i| seq.get_instr(i).with(|d| d.start_offset))
            .max();
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().filter_map(last_offset).max()
}

/// The first bytecode offset of the code of `stat`: of its blocks and of
/// the expressions its statements took from blocks that are gone (a `while`
/// loop's condition, a ternary's arms).
fn code_start(stat: Statement) -> Option<i32> {
    let mut min = first_offset(stat);
    let mut todo = vec![stat];
    while let Some(st) = todo.pop() {
        let mut exprents = st.get_stat_exprents();
        if let Some(own) = st.get_exprents() {
            exprents.extend(own.to_vec());
        }
        for first in exprents.into_iter().filter_map(exprent_start) {
            if min.is_none_or(|m| first < m) {
                min = Some(first);
            }
        }
        if st.get_exprents().is_none() {
            todo.extend(st.get_stats().borrow().to_vec());
        }
    }
    min
}

/// The first bytecode offset of the instructions `exprent` and the
/// expressions in it came from.
fn exprent_start(exprent: Exprent) -> Option<i32> {
    exprent
        .get_all_exprents_rec_self(true, true)
        .into_iter()
        .filter_map(|e| e.get_bytecode())
        .map(|offsets| offsets.next_set_bit(0))
        .filter(|first| *first >= 0)
        .min()
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
