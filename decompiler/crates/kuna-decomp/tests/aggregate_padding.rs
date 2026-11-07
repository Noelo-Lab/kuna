//! Logical ABI padding must not hide a real register field or a different value.
use std::rc::Rc;
use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager,
    ConstantSpace, JoinSpace, UniqueSpace, VarnodeStorage};
use kuna_decomp::{context::ArchContext, funcdata::Funcdata, merge::MergeContext};

#[test]
fn only_matching_constant_join_pieces_are_padding() {
    for big in [false, true] {
        let mut manager = AddrSpaceManager::new();
        let constant = Rc::new(ConstantSpace::new());
        manager.insert_space(Rc::clone(&constant)).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(1, 0, big))).unwrap();
        let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", big,
            8, 1, 2, addrspace_flags::hasphysical, 1, 1));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager.insert_space(Rc::new(JoinSpace::new(3, big))).unwrap();
        let mut storage = [
            VarnodeStorage { space: Some(Rc::clone(&ram)), offset: 0x20, size: 4 },
            VarnodeStorage { space: Some(constant), offset: 0, size: 4 },
            VarnodeStorage { space: Some(Rc::clone(&ram)), offset: 0x30, size: 8 },
        ];
        if !big { storage.reverse(); }
        let address = manager.find_add_join(&storage, 0).unwrap().get_unified().get_addr();
        let arch = Rc::new(ArchContext::new(manager));
        let mut data = Funcdata::new("padding", "padding", arch,
            Address::new(Rc::clone(&ram), 0x1000), 0x10000000, 4).unwrap();
        let whole = data.new_varnode(16, &address, None);
        let zero = data.new_constant(4, 0);
        let nonzero = data.new_constant(4, 1);
        let wide = data.new_constant(8, 0);
        assert!(data.vn_is_padding_piece(zero, whole, 4));
        for (value, offset) in [(zero, 0), (zero, 8), (nonzero, 4), (wide, 4)] {
            assert!(!data.vn_is_padding_piece(value, whole, offset));
        }
        let physical = data.new_varnode(16, &Address::new(ram, 0x20), None);
        assert!(!data.vn_is_padding_piece(zero, physical, 4));
    }
}
