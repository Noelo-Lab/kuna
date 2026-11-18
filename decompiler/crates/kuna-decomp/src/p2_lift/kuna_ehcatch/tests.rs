//! Logic-level tests for the record lookup, the option, and [`plant`] on
//! hand-built dead-list IR.

use std::rc::Rc;

use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

use crate::context::ArchContext;

use super::*;

fn site(start: u64, end: u64, catches: &[&str]) -> EhCallSite {
    EhCallSite { start, end, landing_pad: end + 0x10, catches: catches.iter().map(|c| c.to_string()).collect() }
}

#[test]
fn site_at_finds_the_guarding_record_only() {
    let sites = [site(0x10, 0x18, &["int"]), site(0x20, 0x25, &[])];
    assert_eq!(site_at(&sites, 0x0f), None);
    assert_eq!(site_at(&sites, 0x10), Some(0));
    assert_eq!(site_at(&sites, 0x17), Some(0));
    assert_eq!(site_at(&sites, 0x18), None);
    assert_eq!(site_at(&sites, 0x24), Some(1));
    assert_eq!(site_at(&[], 0x10), None);
}

#[test]
fn clause_joins_types_and_spells_a_cleanup_as_catch_all() {
    assert_eq!(site(0, 1, &["int"]).clause(), "int");
    assert_eq!(site(0, 1, &["Odd", "..."]).clause(), "Odd | ...");
    assert_eq!(site(0, 1, &[]).clause(), "...");
}

#[test]
fn option_parses_its_three_values() {
    for (text, mode) in [("off", EhCatchMode::Off), ("catch", EhCatchMode::Catch), ("all", EhCatchMode::All)] {
        assert_eq!(EhCatchMode::apply(text).unwrap().0, mode);
        assert_eq!(mode.as_str(), text);
    }
    assert!(EhCatchMode::apply("on").is_err());
    assert_eq!(EhCatchMode::default(), EhCatchMode::Catch);
}

#[test]
fn unwinder_registers_are_known_on_four_processors() {
    assert_eq!(unwinder_registers("x86:LE:64:default:gcc"), Some((&b"RAX"[..], &b"RDX"[..])));
    assert_eq!(unwinder_registers("x86:LE:32:default:gcc"), Some((&b"EAX"[..], &b"EDX"[..])));
    assert_eq!(unwinder_registers("ARM:LE:32:v8:default"), Some((&b"r0"[..], &b"r1"[..])));
    assert_eq!(unwinder_registers("AARCH64:LE:64:v8A:default"), Some((&b"x0"[..], &b"x1"[..])));
    assert_eq!(unwinder_registers("PowerPC:BE:32:default:default"), None);
    assert_eq!(unwinder_registers("x86:LE:16:Real Mode:default"), None);
}

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    for (index, name) in [(2, "ram"), (3, "register")] {
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            false,
            8,
            1,
            index,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    }
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
}

fn addr(fd: &Funcdata, space: &str, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name(space).unwrap()), off)
}

fn call(fd: &mut Funcdata) -> OpId {
    let pc = addr(fd, "ram", 0x1000);
    let op = fd.new_op(1, pc);
    fd.op_set_opcode_code(op, OpCode::CPUI_CALL);
    let target = fd.new_code_ref(&addr(fd, "ram", 0x2000));
    fd.op_set_input(op, target, 0).unwrap();
    op
}

fn edge(fd: &Funcdata) -> EhEdge {
    EhEdge {
        pad: addr(fd, "ram", 0x1040),
        site: 3,
        exception: (addr(fd, "register", 0), 8),
        selector: (addr(fd, "register", 0x10), 8),
    }
}

fn dead_list(fd: &Funcdata, from: OpId) -> Vec<OpId> {
    let mut ops = vec![from];
    while let Some(next) = fd.obank().dead_next(*ops.last().unwrap()) {
        ops.push(next);
    }
    ops
}

/// The op a relative branch names: its own time plus the constant.
fn relative_target(fd: &Funcdata, branch: OpId) -> u32 {
    let o = fd.obank().get(branch).unwrap();
    let delta = fd.vbank().get(o.get_in(0).unwrap()).unwrap().get_offset() as u32;
    o.get_time().wrapping_add(delta)
}

#[test]
fn plant_adds_a_test_a_stub_and_a_join_after_a_returning_call() {
    let mut fd = build_fd();
    let call = call(&mut fd);
    let e = edge(&fd);
    let stub = plant(&mut fd, call, &e).unwrap();
    let ops = dead_list(&fd, call);
    assert_eq!(ops.len(), 8);
    assert_eq!(&ops[3..5], [stub.resume, stub.stub]);
    assert_eq!(&ops[6..], [stub.enter, stub.join]);
    let time = |op| fd.obank().get(op).unwrap().get_time();
    assert_eq!(fd.obank().get(ops[2]).unwrap().code(), OpCode::CPUI_CBRANCH);
    assert_eq!(relative_target(&fd, ops[2]), time(stub.stub));
    assert_eq!(relative_target(&fd, stub.resume), time(stub.join));
    assert_eq!(catch_site(&fd, ops[1]), Some(3));
    assert_eq!(catch_site(&fd, stub.stub), None);
    let pad = fd.obank().get(stub.enter).unwrap().get_in(0).unwrap();
    assert_eq!(fd.vbank().get(pad).unwrap().get_addr(), &e.pad);
    for (op, reg) in [(ops[4], &e.exception), (ops[5], &e.selector)] {
        let out = fd.obank().get(op).unwrap().get_out().unwrap();
        assert_eq!(fd.vbank().get(out).unwrap().get_addr(), &reg.0);
        assert!(is_pure(&fd, op));
    }
    assert!(!is_pure(&fd, call));
}

#[test]
fn plant_no_return_replaces_the_halt_with_the_pad_entry() {
    let mut fd = build_fd();
    let call = call(&mut fd);
    let halt = fd.new_op(1, addr(&fd, "ram", 0x1000));
    fd.op_set_opcode_code(halt, OpCode::CPUI_RETURN);
    let one = fd.new_constant(4, 1);
    fd.op_set_input(halt, one, 0).unwrap();
    let e = edge(&fd);
    let enter = plant_no_return(&mut fd, call, halt, &e).unwrap();
    let ops = dead_list(&fd, call);
    assert_eq!(ops.len(), 4);
    assert_eq!(ops[3], enter);
    assert_eq!(catch_site(&fd, ops[1]), Some(3));
    assert!(!ops.contains(&halt));
}
