//! Unit tests for the `calleereadarg` option surface, its decision over a callee
//! entry summary ([`reads_as_argument`]) and the list extension it plans
//! ([`extension`]).  The end-to-end behaviour is `tests/stages/kuna-calleereadarg.xml`
//! and `tests/stages/kuna-calleereadarg-arm.xml`.

use super::*;

use crate::p0_knowledge::options::KUNA_OPTION_NAMES;
use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

const REG: int4 = 3;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, REG, 0, 0, 0))
}

fn loc(sp: &Rc<AddrSpace>, off: u64) -> (Address, int4) {
    (Address::new(Rc::clone(sp), off), 8)
}

/// A complete walk with one RETURN terminator that read `reads` first.
fn body(reads: &[u64], complete: bool) -> CalleeEntryDead {
    CalleeEntryDead::from_parts(
        REG,
        reads.iter().map(|r| (REG, *r, 8)).collect(),
        vec![Vec::new()],
        complete,
    )
}

#[test]
fn option_parses_on_and_off() {
    assert_eq!(OptionCalleeReadArg::NAME, "calleereadarg");
    assert!(OptionCalleeReadArg.apply("on").unwrap().0);
    assert!(!OptionCalleeReadArg.apply("off").unwrap().0);
    assert!(OptionCalleeReadArg.apply("maybe").is_err());
}

#[test]
fn option_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionCalleeReadArg::NAME));
}

/// `getk(int a) { return g * a; }`: the callee reads the first argument
/// register and nothing else, so the caller's value there is its argument.
#[test]
fn a_register_the_callee_reads_is_an_argument() {
    let reg = reg_space();
    let live = body(&[0x38], true);
    let (a, s) = loc(&reg, 0x38);
    assert!(reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// `g(void)`: nothing read, nothing reinstated.
#[test]
fn a_register_the_callee_never_reads_is_not() {
    let reg = reg_space();
    let live = body(&[], true);
    let (a, s) = loc(&reg, 0x38);
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// A variadic register-save prologue reads every argument register through the
/// last one; a register it saves is not evidence of an argument.
#[test]
fn a_callee_that_reads_the_last_register_of_the_class_declines() {
    let reg = reg_space();
    let live = body(&[0x30, 0x10, 0x8, 0x80, 0x88], true);
    let (a, s) = loc(&reg, 0x30);
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// A walk that did not cover the body proves no read.
#[test]
fn an_incomplete_walk_declines() {
    let reg = reg_space();
    let live = body(&[0x38], false);
    let (a, s) = loc(&reg, 0x38);
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// No register entry of the trial's class in the model: no bound, no claim.
#[test]
fn no_class_bound_declines() {
    let reg = reg_space();
    let live = body(&[0x38], true);
    let (a, s) = loc(&reg, 0x38);
    assert!(!reads_as_argument(&live, &a, s, None, true));
}

/// A distinct arena handle; the extension only carries these.
fn vid(n: u64) -> VarnodeId {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}

/// Model entry `i` of a class, with a trial whose value the caller wrote.
fn slot(reg: &Rc<AddrSpace>, i: int4, reads: bool) -> Option<Slot> {
    Some(Slot { index: i, addr: loc(reg, 0x8 * i as u64).0, size: 8, width: 8, vn: Some(vid(i as u64)), reads })
}

/// Model entry `i` with a trial that holds nothing the caller wrote.
fn empty(reg: &Rc<AddrSpace>, i: int4) -> Option<Slot> {
    Some(Slot { index: i, addr: loc(reg, 0x8 * i as u64).0, size: 8, width: 8, vn: None, reads: false })
}

fn picked(row: &[Option<Slot>], used: &[int4]) -> Vec<int4> {
    let used: HashSet<int4> = used.iter().copied().collect();
    extension(row, &used).iter().map(|s| s.index).collect()
}

/// `ext(x)` after an empty list: the first register the callee reads.
#[test]
fn an_empty_list_gains_the_register_the_callee_reads() {
    let reg = reg_space();
    let row = vec![slot(&reg, 0, true), slot(&reg, 1, false), None];
    assert_eq!(picked(&row, &[]), vec![0]);
}

/// The list grows only up to the last register the callee reads, never into a
/// later value the caller merely holds.
#[test]
fn the_extension_stops_at_the_last_register_read() {
    let reg = reg_space();
    let row = vec![slot(&reg, 0, false), slot(&reg, 1, true), slot(&reg, 2, true), slot(&reg, 3, false)];
    assert_eq!(picked(&row, &[]), vec![0, 1, 2]);
    assert_eq!(picked(&row, &[0]), vec![1, 2]);
}

/// A partial list gains the next argument, as the `calleearitylive` witness
/// would have it.
#[test]
fn a_partial_list_gains_the_next_register() {
    let reg = reg_space();
    let row = vec![slot(&reg, 0, false), slot(&reg, 1, false), slot(&reg, 2, false), slot(&reg, 3, true)];
    assert_eq!(picked(&row, &[0, 1, 2]), vec![3]);
}

/// Nothing the callee reads past the list: nothing added.
#[test]
fn no_read_register_adds_nothing() {
    let reg = reg_space();
    let row = vec![slot(&reg, 0, false), slot(&reg, 1, false)];
    assert!(picked(&row, &[0]).is_empty());
}

/// A register with no trial, or a trial with no value, ends the run.
#[test]
fn a_hole_with_no_value_ends_the_run() {
    let reg = reg_space();
    let row = vec![None, slot(&reg, 1, true)];
    assert!(picked(&row, &[]).is_empty());
    let row = vec![slot(&reg, 0, false), empty(&reg, 1), slot(&reg, 2, true)];
    assert!(picked(&row, &[]).is_empty());
}

/// Three entries in a row the callee does not read are upstream's end of a
/// list; a read register past them is not reached.
#[test]
fn more_than_two_holes_end_the_run() {
    let reg = reg_space();
    let row =
        vec![slot(&reg, 0, false), slot(&reg, 1, false), slot(&reg, 2, false), slot(&reg, 3, true)];
    assert!(picked(&row, &[]).is_empty());
    assert_eq!(picked(&row, &[0]), vec![1, 2, 3]);
}

/// A list that is not a leading run of the class is left alone.
#[test]
fn a_list_with_a_gap_is_left_alone() {
    let reg = reg_space();
    let row = vec![slot(&reg, 0, true), slot(&reg, 1, false), slot(&reg, 2, true)];
    assert!(picked(&row, &[1]).is_empty());
}

/// clang -Os `reallymarkobject`: `mov 0x8(%rbx),%cl; lea -0x4(%rcx),%edx`
/// reads the unwritten upper bytes of rcx, but its low byte was written first,
/// so rcx carries no argument.
#[test]
fn a_wide_read_past_a_low_byte_write_is_not_an_argument() {
    let reg = reg_space();
    let live = body(&[0x8], true).with_written_byte(0x8);
    let (a, s) = loc(&reg, 0x8);
    assert!(live.proves_input(&a, s));
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// A body the walk could not decode all of (an ARM function decoded as Thumb
/// breaks into bad instruction data) proves no read.
#[test]
fn a_walk_that_met_bad_instruction_data_declines() {
    let reg = reg_space();
    let live = body(&[0x38], true).with_undecodable();
    let (a, s) = loc(&reg, 0x38);
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
}

/// gcc AArch64 bounds a variadic callee's register save to the `va_arg` it can
/// reach (`str x2, [sp, #24]`): a register only stored is evidence only where
/// callers mark their variadic calls.
#[test]
fn a_register_only_stored_counts_only_where_variadic_calls_are_marked() {
    let reg = reg_space();
    let live = body(&[0x38], true).with_stored_byte(0x38);
    let (a, s) = loc(&reg, 0x38);
    assert!(reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), true));
    assert!(!reads_as_argument(&live, &a, s, Some(&loc(&reg, 0x88)), false));
}
