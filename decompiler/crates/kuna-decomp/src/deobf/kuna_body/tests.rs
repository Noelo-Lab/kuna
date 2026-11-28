use super::*;
use crate::context::ArchContext;
use kuna_base::space::{addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

struct Fixture {
    fd: Funcdata,
    register: Rc<AddrSpace>,
    stack: Rc<AddrSpace>,
}

fn fixture(facts: Option<BodyFacts>) -> Fixture {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2, addrspace_flags::hasphysical, 1, 1));
    manager.insert_space(ram.clone()).unwrap();
    let register = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, addrspace_flags::hasphysical, 1, 1));
    manager.insert_space(register.clone()).unwrap();
    let stack = Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 4, 0, 1, 1));
    manager.insert_space(stack.clone()).unwrap();
    let mut fd = Funcdata::new("f", "f", Rc::new(ArchContext::new(manager)), Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap();
    if let Some(facts) = facts {
        let mut body = RecoveredBody::with_policy(facts.policy);
        body.facts = facts;
        fd.kuna_set_body(Some(Rc::new(body)));
    }
    Fixture { fd, register, stack }
}

fn at(space: &Rc<AddrSpace>, offset: i64) -> Address {
    Address::new(Rc::clone(space), offset as u64)
}

#[test]
fn a_function_lifted_from_its_bytes_states_nothing() {
    let f = fixture(None);
    assert!(f.fd.kuna_recovered().is_none());
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x10), 8), None);
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x40), 8), None);
    assert!(!guards_every_store(&f.fd, &at(&f.stack, -0x40)));
    assert!(!frame_escaped(&f.fd, &at(&f.stack, -0x40), 8));
    assert!(!holds_escape(&f.fd, &at(&f.stack, -0x40), 8));
    assert!(!restructure_settled(&f.fd));
}

#[test]
fn a_call_kills_before_it_preserves_and_only_whole_ranges() {
    let facts = BodyFacts { call_kills: vec![(0x10, 8)], call_preserves: vec![(0x10, 16), (0x40, 8)], ..Default::default() };
    let f = fixture(Some(facts));
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x10), 8), Some(effect_type::KILLEDBYCALL));
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x14), 4), Some(effect_type::KILLEDBYCALL));
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x18), 8), Some(effect_type::UNAFFECTED));
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x40), 8), Some(effect_type::UNAFFECTED));
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x44), 8), None);
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x80), 8), None);
    assert_eq!(call_effect(&f.fd, &at(&f.stack, 0x10), 8), None);
}

#[test]
fn a_call_overwrites_the_stack_below_its_floor() {
    let f = fixture(Some(BodyFacts { call_floor: Some(-0x80), ..Default::default() }));
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x100), 8), Some(effect_type::KILLEDBYCALL));
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x88), 8), Some(effect_type::KILLEDBYCALL));
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x84), 8), None);
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x80), 8), None);
    assert_eq!(call_effect(&f.fd, &at(&f.stack, -0x100), 0), None);
    assert_eq!(call_effect(&f.fd, &at(&f.register, 0x10), 8), None);
}

#[test]
fn an_escaped_frame_address_marks_its_slot_and_the_locals_above_it() {
    let f = fixture(Some(BodyFacts { escapes: vec![-0x30, 0x8], ..Default::default() }));
    assert!(holds_escape(&f.fd, &at(&f.stack, -0x30), 8));
    assert!(holds_escape(&f.fd, &at(&f.stack, -0x34), 8));
    assert!(!holds_escape(&f.fd, &at(&f.stack, -0x38), 8));
    assert!(holds_escape(&f.fd, &at(&f.stack, 0x8), 8));
    assert!(frame_escaped(&f.fd, &at(&f.stack, -0x30), 8));
    assert!(frame_escaped(&f.fd, &at(&f.stack, -0x8), 8));
    assert!(frame_escaped(&f.fd, &at(&f.stack, -0x34), 8));
    assert!(!frame_escaped(&f.fd, &at(&f.stack, -0x38), 8));
    assert!(!frame_escaped(&f.fd, &at(&f.stack, 0x8), 8));
    assert!(!frame_escaped(&f.fd, &at(&f.register, 0x10), 8));
}

#[test]
fn each_stage_reads_its_own_policy_flag() {
    let f = fixture(Some(BodyFacts { policy: BodyPolicy { guard_stack_stores: true, ..Default::default() }, ..Default::default() }));
    assert!(guards_every_store(&f.fd, &at(&f.stack, -0x40)));
    assert!(!guards_every_store(&f.fd, &at(&f.register, 0x10)));
    assert!(!f.fd.kuna_body().policy.join_stack_inputs);
    assert!(!f.fd.kuna_body().policy.residue_rules);
    assert!(!restructure_settled(&f.fd));
}
