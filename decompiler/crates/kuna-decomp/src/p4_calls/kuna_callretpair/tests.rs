//! Unit tests for `callretpair`'s gate.  The arm it opens is shared with
//! [`crate::kuna_rustabi`] and its IR-level behaviour is tested there, next to
//! the scaffolding that builds a CALL with two output trials
//! (`the_call_seam_builds_the_pair_on_callretpair_alone` and
//! `the_call_seam_fails_closed_when_both_gates_are_off`); the end-to-end
//! behaviour is `tests/stages/kuna-callretpair.xml` and
//! `tests/cli/bytecode-reader-return-discarded.json`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::ArchContext;
use crate::p0_knowledge::kuna_phases::{KunaPhase, SETTABLE_TABLE};

use super::*;

fn build_fd(call_ret_pair: bool) -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.call_ret_pair = call_ret_pair;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn the_gate_follows_the_option() {
    assert!(live(&build_fd(true)));
    assert!(!live(&build_fd(false)));
}

#[test]
fn the_catalog_row_ships_on_in_p4_as_a_core_option() {
    let row = SETTABLE_TABLE
        .iter()
        .find(|r| r.option == "callretpair")
        .expect("callretpair has a settableTable row");
    assert_eq!(row.shipped, "on", "the stubbed arm is completed by default");
    assert_eq!(row.values, "on|off");
    assert_eq!(row.phase, KunaPhase::P4);
    assert_eq!(row.tier, "core");
}
