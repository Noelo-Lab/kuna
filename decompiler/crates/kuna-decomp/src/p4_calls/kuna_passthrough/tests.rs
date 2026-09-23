//! Unit tests for the `passthrough` option surface and the decisions the module
//! makes without a function in front of it: which ranges a claim covers
//! ([`overlaps_claim`]), which forwarded registers extend a
//! settled argument list ([`extension`]), and the width a callee's body says it
//! consumes ([`crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead::live_input_width`]).
//! The end-to-end behaviour is
//! `tests/cli/passthrough-gives-a-forwarding-function-its-parameter.json` and
//! its option-off arm.

use super::*;

use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;
use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

const REG: int4 = 3;
const RDI: u64 = 0x38;
const RAX: u64 = 0x0;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, REG, 0, 0, 0))
}

fn ram_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 1, 0, 0, 0))
}

fn op(n: u64) -> OpId {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}

fn claim(sp: &Rc<AddrSpace>, off: u64, size: int4, args: &[u64], rets: &[u64]) -> PassThroughClaim {
    PassThroughClaim {
        addr: Address::new(Rc::clone(sp), off),
        size,
        arg_owners: args.iter().map(|&n| op(n)).collect(),
        ret_owners: rets.iter().map(|&n| op(n)).collect(),
    }
}

#[test]
fn option_parses_on_and_off_and_rejects_anything_else() {
    assert!(OptionPassThrough.apply("on").unwrap().0);
    assert!(!OptionPassThrough.apply("off").unwrap().0);
    assert!(OptionPassThrough.apply("args").is_err());
    assert_eq!(OptionPassThrough::NAME, "passthrough");
}

#[test]
fn no_claim_overlaps_nothing() {
    let sp = reg_space();
    let a = Address::new(Rc::clone(&sp), RDI);
    assert!(!overlaps_claim(&[], &a, 8));
}

#[test]
fn a_claim_covers_every_overlapping_width_and_nothing_beside_it() {
    let sp = reg_space();
    let claims = [claim(&sp, RDI, 4, &[1], &[])];
    let rdi = Address::new(Rc::clone(&sp), RDI);
    let rsi = Address::new(Rc::clone(&sp), 0x30);
    assert!(overlaps_claim(&claims, &rdi, 8), "rdi overlaps a claimed edi");
    assert!(overlaps_claim(&claims, &rdi, 1), "and so does dil");
    assert!(!overlaps_claim(&claims, &rsi, 8), "rsi does not");
    let ram = Address::new(ram_space(), RDI);
    assert!(!overlaps_claim(&claims, &ram, 8), "the same offset in another space does not");
}

#[test]
fn a_return_claim_covers_the_return_register() {
    let sp = reg_space();
    let claims = [claim(&sp, RAX, 8, &[], &[7])];
    let rax = Address::new(Rc::clone(&sp), RAX);
    assert!(overlaps_claim(&claims, &rax, 8), "the RETURN trial is the pass's own");
    assert!(overlaps_claim(&claims, &rax, 4), "eax too");
}

fn facts(reads: Vec<(int4, u64, int4)>) -> CalleeEntryDead {
    CalleeEntryDead::from_parts(REG, reads, vec![vec![]], true)
}

#[test]
fn the_width_is_what_the_callee_body_reads() {
    let sp = reg_space();
    let rdi = Address::new(Rc::clone(&sp), RDI);
    assert_eq!(facts(vec![(REG, RDI, 4)]).live_input_width(&rdi, 8), Some(4), "mov %edi,..: an int");
    assert_eq!(facts(vec![(REG, RDI, 8)]).live_input_width(&rdi, 8), Some(8), "movzbl (%rdi): a pointer");
    assert_eq!(
        facts(vec![(REG, RDI, 1), (REG, RDI, 4)]).live_input_width(&rdi, 8),
        Some(4),
        "the widest read decides"
    );
}

#[test]
fn no_width_without_a_low_justified_read() {
    let sp = reg_space();
    let rdi = Address::new(Rc::clone(&sp), RDI);
    assert_eq!(facts(vec![]).live_input_width(&rdi, 8), None, "nothing read");
    assert_eq!(facts(vec![(REG, 0x30, 8)]).live_input_width(&rdi, 8), None, "another register read");
    assert_eq!(facts(vec![(REG, RDI + 1, 1)]).live_input_width(&rdi, 8), None, "a read above the low byte");
    let incomplete = CalleeEntryDead::from_parts(REG, vec![(REG, RDI, 8)], vec![vec![]], false);
    assert_eq!(incomplete.live_input_width(&rdi, 8), None, "an incomplete walk proves nothing");
}

const RSI: u64 = 0x30;
const RDX: u64 = 0x10;

fn regs(sp: &Rc<AddrSpace>, offs: &[u64]) -> Vec<Address> {
    offs.iter().map(|&o| Address::new(Rc::clone(sp), o)).collect()
}

#[test]
fn an_empty_list_takes_the_forwarded_first_parameter() {
    let sp = reg_space();
    let got = extension(&[], &regs(&sp, &[RDI]), &regs(&sp, &[RDI]));
    assert_eq!(got, vec![0], "gzip_base_name: jmp last_component");
}

#[test]
fn a_settled_list_is_extended_along_the_stated_order() {
    let sp = reg_space();
    let stated = regs(&sp, &[RDI, RSI, RDX]);
    assert_eq!(extension(&regs(&sp, &[RDI]), &stated, &regs(&sp, &[RDX, RSI])), vec![1, 0]);
    assert_eq!(
        extension(&regs(&sp, &[RDI, RSI]), &stated, &regs(&sp, &[RDX])),
        vec![0],
        "only what follows the settled list"
    );
}

#[test]
fn the_extension_stops_at_the_first_register_not_forwarded() {
    let sp = reg_space();
    let stated = regs(&sp, &[RDI, RSI, RDX]);
    assert!(extension(&[], &stated, &regs(&sp, &[RSI])).is_empty(), "rdi is not forwarded: rsi would be misplaced");
    assert_eq!(extension(&[], &stated, &regs(&sp, &[RDI, RDX])), vec![0], "rsi is not forwarded: stop before rdx");
}

#[test]
fn a_list_that_already_covers_or_contradicts_the_stated_one_is_left_alone() {
    let sp = reg_space();
    let stated = regs(&sp, &[RDI, RSI]);
    assert!(extension(&regs(&sp, &[RDI, RSI]), &stated, &regs(&sp, &[RDI, RSI])).is_empty(), "sysinttostr after the sibling rescue");
    assert!(extension(&regs(&sp, &[RDI, RSI, RDX]), &stated, &regs(&sp, &[RDX])).is_empty(), "longer than stated");
    assert!(extension(&regs(&sp, &[RSI]), &stated, &regs(&sp, &[RSI])).is_empty(), "not a leading run");
}
