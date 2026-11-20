use super::*;
use crate::context::{ArchContext, TypeOp};
use crate::kuna_sourcelang::SourceLang;
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use std::rc::Rc;

fn function(lang: SourceLang) -> Funcdata {
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    for (name, index) in [("register", 2), ("ram", 3)] {
        manage
            .insert_space(Rc::new(AddrSpace::new(
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
    let mut arch = ArchContext::new(manage);
    arch.source_lang = lang;
    let entry = Address::new(arch.manage().get_space(3).unwrap().clone(), 0x1000);
    Funcdata::new("probe", "probe", Rc::new(arch), entry, 0x1000_0000, 0x40).unwrap()
}

fn value(fd: &mut Funcdata, loc: Storage) -> VarnodeId {
    if loc.0 == 0 {
        return fd.new_constant(loc.2, loc.1);
    }
    let space = fd.get_arch().manage().get_space(loc.0).unwrap().clone();
    fd.new_varnode_space_off(loc.2, space, loc.1)
}

fn operation(fd: &mut Funcdata, code: OpCode, inputs: &[Storage], output: Option<Storage>) -> OpId {
    let id = fd.new_op(inputs.len() as i32, fd.get_address().clone());
    fd.op_set_opcode(id, TypeOp::new(code, 0, format!("{code:?}")));
    for (i, input) in inputs.iter().enumerate() {
        let vn = value(fd, *input);
        fd.op_set_input(id, vn, i as i32).unwrap();
    }
    if let Some(out) = output {
        let address = Address::new(
            fd.get_arch().manage().get_space(out.0).unwrap().clone(),
            out.1,
        );
        fd.new_varnode_out(out.2, &address, id).unwrap();
    }
    id
}

fn xadd(fd: &mut Funcdata, width: i32) -> Vec<OpId> {
    let p = Storage(2, 0, 8);
    let d = Storage(2, 24, width);
    let loaded = Storage(1, 0x100, width);
    let sum = Storage(1, 0x200, width);
    let ram = Storage(0, 3, 8);
    vec![
        operation(fd, OpCode::CPUI_CALLOTHER, &[Storage(0, 19, 4)], None),
        operation(fd, OpCode::CPUI_LOAD, &[ram, p], Some(loaded)),
        operation(fd, OpCode::CPUI_INT_ADD, &[loaded, d], Some(sum)),
        operation(fd, OpCode::CPUI_LOAD, &[ram, p], Some(loaded)),
        operation(fd, OpCode::CPUI_COPY, &[loaded], Some(d)),
        operation(fd, OpCode::CPUI_COPY, &[sum], Some(loaded)),
        operation(fd, OpCode::CPUI_STORE, &[ram, p, loaded], None),
        operation(fd, OpCode::CPUI_CALLOTHER, &[Storage(0, 20, 4)], None),
    ]
}

#[test]
fn keeps_original_old_load_and_member_identity_at_both_widths() {
    for width in [4, 8] {
        let mut fd = function(SourceLang::Go);
        let ops = xadd(&mut fd, width);
        let proof = instruction(&fd, &ops).unwrap();
        assert_eq!(proof.old_load, ops[3]);
        assert_eq!(proof.store, ops[6]);
        assert_eq!(proof.members, ops);
        assert_eq!(proof.width, width);
        observe(&mut fd, &ops);
        assert_eq!(fd.kuna_atomic_xadds.len(), 1);
        fd.clear();
        assert!(fd.kuna_atomic_xadds.is_empty());
    }
}

#[test]
fn mismatched_target_width_and_overwritten_address_refuse() {
    let mut fd = function(SourceLang::Go);
    let ops = xadd(&mut fd, 4);
    let different = value(&mut fd, Storage(2, 8, 8));
    fd.op_set_input(ops[6], different, 1).unwrap();
    assert!(instruction(&fd, &ops).is_none());

    let mut fd = function(SourceLang::Go);
    let ops = xadd(&mut fd, 4);
    let wider = value(&mut fd, Storage(1, 0x100, 8));
    fd.op_set_input(ops[6], wider, 2).unwrap();
    assert!(instruction(&fd, &ops).is_none());

    let mut fd = function(SourceLang::Go);
    let mut ops = xadd(&mut fd, 4);
    ops[4] = operation(
        &mut fd,
        OpCode::CPUI_COPY,
        &[Storage(1, 0x100, 4)],
        Some(Storage(2, 0, 4)),
    );
    assert!(instruction(&fd, &ops).is_none());
}

#[test]
fn extra_effect_and_missing_old_exchange_refuse() {
    for extra in [
        OpCode::CPUI_CALLOTHER,
        OpCode::CPUI_CALLIND,
        OpCode::CPUI_CBRANCH,
    ] {
        let mut fd = function(SourceLang::Go);
        let mut ops = xadd(&mut fd, 4);
        let op = operation(&mut fd, extra, &[Storage(0, 21, 4)], None);
        ops.insert(1, op);
        assert!(instruction(&fd, &ops).is_none());
    }
    let mut fd = function(SourceLang::Go);
    let mut ops = xadd(&mut fd, 4);
    ops.remove(4);
    assert!(instruction(&fd, &ops).is_none());
    let mut fd = function(SourceLang::Go);
    let ops = xadd(&mut fd, 2);
    assert!(instruction(&fd, &ops).is_none());
}

#[test]
fn foreign_source_is_inert_and_multiple_candidates_are_bounded() {
    for lang in [SourceLang::C, SourceLang::Rust, SourceLang::Unknown] {
        let mut fd = function(lang);
        let ops = xadd(&mut fd, 4);
        observe(&mut fd, &ops);
        assert!(fd.kuna_atomic_xadds.is_empty());
    }
    let mut fd = function(SourceLang::Go);
    for _ in 0..4 {
        let ops = xadd(&mut fd, 4);
        observe(&mut fd, &ops);
    }
    assert_eq!(fd.kuna_atomic_xadds.len(), 2);
}
