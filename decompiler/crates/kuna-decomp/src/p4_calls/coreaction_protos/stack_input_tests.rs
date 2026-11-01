use super::*;
use crate::context::{ArchContext, OpId};
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};

fn stack_fixture(big: bool, trunc_ptr_flow: bool) -> Funcdata {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
        .unwrap();
    let register = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        big,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&register)).unwrap();
    manager.set_default_code_space(2).unwrap();
    let stack = Rc::new(SpacebaseSpace::new("stack", 3, 4, &register, 1, true, big));
    manager.insert_space(Rc::clone(&stack)).unwrap();
    manager
        .add_spacebase_pointer(
            &stack,
            &VarnodeStorage {
                space: Some(Rc::clone(&register)),
                offset: 0x100,
                size: 8,
            },
            4,
            true,
        )
        .unwrap();
    let mut arch = ArchContext::new(manager);
    arch.trunc_ptr_flow = trunc_ptr_flow;
    let mut fd = Funcdata::new(
        "probe",
        "probe",
        Rc::new(arch),
        Address::new(register, 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let root = fd.bblocks_ref().root.unwrap();
    fd.bblocks_mut().new_block_basic(root);
    ActionPrototypeTypes::boxed("base").apply(&mut fd, &mut ActionContext::new());
    fd
}

fn full_stack_defs(fd: &Funcdata) -> Vec<OpId> {
    fd.obank()
        .iter_alive()
        .filter(|&op| {
            fd.obank()
                .get(op)
                .and_then(|o| o.get_out())
                .is_some_and(|v| {
                    fd.vbank()
                        .get(v)
                        .is_some_and(|v| v.get_offset() == 0x100 && v.get_size() == 8)
                })
        })
        .collect()
}

#[test]
fn narrow_stack_base_retains_every_physical_input_bit() {
    for big in [false, true] {
        let fd = stack_fixture(big, true);
        let defs = full_stack_defs(&fd);
        assert_eq!(defs.len(), 1);
        let op = fd.obank().get(defs[0]).unwrap();
        assert_eq!(op.code(), OpCode::CPUI_PIECE);
        let high = fd.vbank().get(op.get_in(0).unwrap()).unwrap();
        let low = fd.vbank().get(op.get_in(1).unwrap()).unwrap();
        assert!(high.is_input() && low.is_input());
        assert_eq!((high.get_size(), low.get_size()), (4, 4));
        assert_eq!(high.get_offset(), if big { 0x100 } else { 0x104 });
        assert_eq!(low.get_offset(), if big { 0x104 } else { 0x100 });
        for physical in [0x80001000u64, 0xffffffff80001000, 0x1234567880001000] {
            let bytes = if big {
                physical.to_be_bytes()
            } else {
                physical.to_le_bytes()
            };
            let read = |offset: u64| {
                let start = (offset - 0x100) as usize;
                let bytes: [u8; 4] = bytes[start..start + 4].try_into().unwrap();
                u64::from(if big {
                    u32::from_be_bytes(bytes)
                } else {
                    u32::from_le_bytes(bytes)
                })
            };
            assert_eq!(
                (read(high.get_offset()) << 32) | read(low.get_offset()),
                physical
            );
            assert_eq!(read(low.get_offset()), 0x80001000);
        }
    }
}

#[test]
fn option_off_seeds_no_stack_reconstruction() {
    for big in [false, true] {
        let fd = stack_fixture(big, false);
        assert!(full_stack_defs(&fd).is_empty());
    }
}
