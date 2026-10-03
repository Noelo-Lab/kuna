use super::*;
use crate::p0_knowledge::kuna_wrappedstackmap::{is_wrapped_join, pieces};
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, XmlEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace,
    SpacebaseSpace, UniqueSpace, VarnodeStorage,
};

fn manager(big: bool) -> (AddrSpaceManager, Rc<AddrSpace>) {
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage
        .insert_space(Rc::new(UniqueSpace::new(1, 0, big)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    manage
        .insert_space(Rc::new(JoinSpace::new(3, big)))
        .unwrap();
    let stack = Rc::new(SpacebaseSpace::new("stack", 4, 4, &ram, 1, true, big));
    manage.insert_space(Rc::clone(&stack)).unwrap();
    (manage, stack)
}

fn joined(manage: &AddrSpaceManager, stack: &Rc<AddrSpace>, prefix: i64) -> Address {
    let start = Address::new(Rc::clone(stack), stack.wrap_offset((-prefix) as u64));
    let mut storage: Vec<_> = pieces(&start, 72)
        .unwrap()
        .iter()
        .map(|(addr, _, size)| VarnodeStorage {
            space: addr.get_space().cloned(),
            offset: addr.get_offset(),
            size: *size as u32,
        })
        .collect();
    if !stack.is_big_endian() {
        storage.reverse();
    }
    let join = manage.find_add_join(&storage, 0).unwrap();
    assert!(is_wrapped_join(&join));
    join.get_unified().get_addr()
}

fn local_db() -> (Database, ScopeId) {
    let mut db = Database::new(true);
    let global = db.find_create_scope(0, "", None, 5).unwrap();
    let local = db.find_create_scope(1, "f", Some(global), 5).unwrap();
    (db, local)
}

fn assert_layout(
    db: &Database,
    local: ScopeId,
    sym: SymbolId,
    stack: &Rc<AddrSpace>,
    addr: &Address,
    prefix: i64,
) {
    assert_eq!(db.symbol(sym).whole_count, 1);
    assert_eq!(db.symbol(sym).num_entries(), 3);
    let main = db.entry(local, db.symbol(sym).mapentry[0]);
    assert_eq!(main.get_addr(), addr);
    assert_eq!(main.get_size(), 72);
    assert!(!main.is_piece());
    for byte in 0..72 {
        let offset = stack.wrap_offset((byte as i64 - prefix) as u64);
        let location = Address::new(Rc::clone(stack), offset);
        let entry = db.entry(local, db.find_overlap(local, &location, 1).unwrap());
        assert_eq!(entry.symbol, sym);
        assert!(entry.is_piece());
        assert_ne!(entry.extraflags & varnode_flags::mapped, 0);
        assert_eq!(
            (offset - entry.get_addr().get_offset()) as i32 + entry.get_offset(),
            byte
        );
    }
}

#[test]
fn wrapped_maps_keep_one_main_entry_and_roundtrip_the_join() {
    for big in [false, true] {
        for prefix in [4, 8, 12, 16] {
            let (manage, stack) = manager(big);
            let addr = joined(&manage, &stack, prefix);
            let (mut db, local) = local_db();
            let ty = Rc::new(Datatype::new(72, type_metatype::TYPE_STRUCT));
            let (sym, _) = db
                .add_symbol_mapped(
                    local,
                    "spill",
                    Rc::clone(&ty),
                    &addr,
                    &Address::new_invalid(),
                )
                .unwrap();
            assert_layout(&db, local, sym, &stack, &addr, prefix);
            let specs = db.scope_space_symbol_specs(local, stack.get_index() as usize);
            assert_eq!(specs.len(), 1);
            assert_eq!(specs[0].2, addr);
            assert!(Rc::ptr_eq(&specs[0].1, &ty));
            assert!(db
                .scope_nonstack_addrtied_specs(local, stack.get_index() as usize)
                .is_empty());
            let (mut restored, scope) = local_db();
            let (copy, _) = restored
                .add_symbol_mapped(
                    scope,
                    &specs[0].0,
                    Rc::clone(&specs[0].1),
                    &specs[0].2,
                    &Address::new_invalid(),
                )
                .unwrap();
            assert_layout(&restored, scope, copy, &stack, &addr, prefix);

            let mut xml = Vec::new();
            db.encode_scope(local, &mut XmlEncode::new(&mut xml))
                .unwrap();
            let text = String::from_utf8(xml).unwrap();
            assert_eq!(text.matches("<addr ").count(), 1, "{text}");
            assert_eq!(text.matches("space=\"join\"").count(), 1, "{text}");
            assert!(
                text.contains(&format!(
                    "stack:0x{:x}:{prefix}",
                    stack.wrap_offset((-prefix) as u64)
                )),
                "{text}"
            );
            assert!(
                text.contains(&format!("stack:0x0:{}", 72 - prefix)),
                "{text}"
            );
            let mut encoded = Vec::new();
            addr.encode(&mut XmlEncode::new(&mut encoded)).unwrap();
            let registry = IdRegistry::with_base_ids();
            let mut decoder = XmlDecode::new(&manage, &registry);
            decoder.ingest_stream(&encoded).unwrap();
            let decoded = Address::decode(&mut decoder).unwrap();
            assert_eq!(decoded, addr);
            let join = manage.find_join(decoded.get_offset()).unwrap();
            for byte in 0..72 {
                let mut position = 0;
                let physical =
                    join.get_equivalent_address(decoded.get_offset() + byte, &mut position);
                assert_eq!(
                    physical,
                    Address::new(
                        Rc::clone(&stack),
                        stack.wrap_offset((byte as i64 - prefix) as u64)
                    )
                );
            }
        }
    }
}

#[test]
fn wrapped_maps_retype_and_remove_all_physical_entries() {
    let (manage, stack) = manager(false);
    let addr = joined(&manage, &stack, 16);
    let (mut db, local) = local_db();
    let (sym, _) = db
        .add_symbol_mapped(
            local,
            "spill",
            Rc::new(Datatype::new(72, type_metatype::TYPE_STRUCT)),
            &addr,
            &Address::new_invalid(),
        )
        .unwrap();
    let same_size = Rc::new(Datatype::new(72, type_metatype::TYPE_STRUCT));
    db.retype_symbol(sym, Rc::clone(&same_size)).unwrap();
    assert!(Rc::ptr_eq(
        db.symbol(sym).dtype.as_ref().unwrap(),
        &same_size
    ));
    assert_layout(&db, local, sym, &stack, &addr, 16);
    assert!(db
        .retype_symbol(sym, Rc::new(Datatype::new(76, type_metatype::TYPE_STRUCT)))
        .is_err());
    assert!(Rc::ptr_eq(
        db.symbol(sym).dtype.as_ref().unwrap(),
        &same_size
    ));
    assert_layout(&db, local, sym, &stack, &addr, 16);
    let second = joined(&manage, &stack, 12);
    db.add_map_point(local, sym, &second, &Address::new_invalid())
        .unwrap();
    assert_eq!(db.symbol(sym).whole_count, 2);
    assert_eq!(db.symbol(sym).num_entries(), 6);
    assert!(db.symbol(sym).is_multi_entry());
    assert_eq!(db.scopes[local].multi_entry_set.len(), 1);
    let specs = db.scope_space_symbol_specs(local, stack.get_index() as usize);
    assert_eq!(specs.len(), 2);
    assert!(specs.iter().any(|spec| spec.2 == addr));
    assert!(specs.iter().any(|spec| spec.2 == second));
    assert!(db
        .scope_nonstack_addrtied_specs(local, stack.get_index() as usize)
        .is_empty());
    db.remove_symbol(sym);
    assert!(db.scopes[local].multi_entry_set.is_empty());
    for offset in [stack.wrap_offset((-16i64) as u64), 0, 59] {
        assert!(db
            .find_overlap(local, &Address::new(Rc::clone(&stack), offset), 1)
            .is_none());
    }
    assert!(db.find_overlap(local, &addr, 1).is_none());
    assert!(db.find_overlap(local, &second, 1).is_none());
}

#[test]
fn wire_storage_keeps_same_size_join_identities_distinct() {
    let (manage, stack) = manager(false);
    let first = joined(&manage, &stack, 16);
    let second = joined(&manage, &stack, 12);
    let ordinary = manage
        .find_add_join(
            &[
                VarnodeStorage {
                    space: Some(Rc::clone(&stack)),
                    offset: 100,
                    size: 36,
                },
                VarnodeStorage {
                    space: Some(Rc::clone(&stack)),
                    offset: 200,
                    size: 36,
                },
            ],
            0,
        )
        .unwrap()
        .get_unified()
        .get_addr();
    let arch = Rc::new(crate::context::ArchContext::new(manage));
    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
    let mut fd =
        crate::funcdata::Funcdata::new("f", "f", arch, Address::new(ram, 0x1000), 0x10000000, 4)
            .unwrap();
    let original = fd.new_unique(72, None);
    let original_addr = fd.vbank().get(original).unwrap().get_addr().clone();
    for (name, addr) in [
        ("first", &first),
        ("second", &second),
        ("ordinary", &ordinary),
    ] {
        fd.get_scope_local_mut()
            .unwrap()
            .add_symbol(
                name,
                Rc::new(Datatype::new(72, type_metatype::TYPE_STRUCT)),
                addr,
                &Address::new_invalid(),
            )
            .unwrap();
    }
    let maps = fd.mapped_symbol_specs();
    let count = fd.vbank().num_varnodes();
    let mut wire = Vec::new();
    fd.encode(&mut XmlEncode::new(&mut wire), 1, false).unwrap();
    let carriers = fd.kuna_wrapped_wire_storage.clone();
    assert_eq!(carriers.len(), 2);
    let locations: Vec<_> = carriers.values().map(|(addr, _)| addr).collect();
    assert_ne!(locations[0], locations[1]);
    for addr in &locations {
        assert_eq!(
            addr.get_space().unwrap().get_type(),
            spacetype::IPTR_INTERNAL
        );
        assert!(addr.get_offset() >= original_addr.get_offset() + 72);
        assert_ne!(addr.get_offset(), 0x20000000);
    }
    let text = String::from_utf8(wire.clone()).unwrap();
    assert_eq!(text.matches("space=\"unique\"").count(), 2, "{text}");
    assert_eq!(text.matches("space=\"join\"").count(), 1, "{text}");
    assert!(text.contains("stack:0x64:36"));
    assert!(!text.contains("stack:0xfffffff0:16"));
    assert!(!text.contains("stack:0xfffffff4:12"));
    let after = fd.mapped_symbol_specs();
    assert_eq!(after.len(), maps.len());
    for (before, after) in maps.iter().zip(&after) {
        assert_eq!(before.0, after.0);
        assert!(Rc::ptr_eq(&before.1, &after.1));
        assert_eq!(before.2, after.2);
        assert_eq!(before.3, after.3);
    }
    assert_eq!(fd.vbank().num_varnodes(), count);
    let mut repeated = Vec::new();
    fd.encode(&mut XmlEncode::new(&mut repeated), 1, false)
        .unwrap();
    assert_eq!(wire, repeated);
    assert_eq!(fd.kuna_wrapped_wire_storage, carriers);
    let first_id = *carriers.keys().next().unwrap();
    super::super::kuna_wrappedstackmap::reserve_wire_storage(&mut fd, first_id, 76);
    let grown = fd.kuna_wrapped_wire_storage[&first_id].clone();
    assert_eq!(grown.1, 76);
    assert!(locations
        .iter()
        .all(|addr| grown.0.get_offset() >= addr.get_offset() + 72));
    super::super::kuna_wrappedstackmap::reserve_wire_storage(&mut fd, first_id, 76);
    assert_eq!(fd.kuna_wrapped_wire_storage[&first_id], grown);
    let next = fd.new_unique(4, None);
    let next_addr = fd.vbank().get(next).unwrap().get_addr();
    assert!(locations
        .iter()
        .all(|addr| next_addr.get_offset() >= addr.get_offset() + 72));
    if let Some(path) = std::env::var_os("KUNA_WRAPPED_WIRE_DUMP") {
        std::fs::write(path, wire).unwrap();
    }
    fd.clear();
    assert!(fd.kuna_wrapped_wire_storage.is_empty());
}

#[test]
fn wire_storage_adapts_only_wrapped_entries_in_mixed_and_replaced_maps() {
    for wrapped_first in [false, true] {
        let (manage, stack) = manager(false);
        let wrapped = joined(&manage, &stack, 16);
        let ordinary = manage
            .find_add_join(
                &[
                    VarnodeStorage {
                        space: Some(Rc::clone(&stack)),
                        offset: 100,
                        size: 36,
                    },
                    VarnodeStorage {
                        space: Some(Rc::clone(&stack)),
                        offset: 200,
                        size: 36,
                    },
                ],
                0,
            )
            .unwrap()
            .get_unified()
            .get_addr();
        let (mut db, local) = local_db();
        let order = if wrapped_first {
            [&wrapped, &ordinary]
        } else {
            [&ordinary, &wrapped]
        };
        let (sym, _) = db
            .add_symbol_mapped(
                local,
                "mixed",
                Rc::new(Datatype::new(72, type_metatype::TYPE_STRUCT)),
                order[0],
                &Address::new_invalid(),
            )
            .unwrap();
        db.add_map_point(local, sym, order[1], &Address::new_invalid())
            .unwrap();
        let id = db.symbol(sym).get_id();
        assert_eq!(db.wrapped_stack_wire_specs(local), vec![(id, 72)]);
        let specs = db.scope_space_symbol_specs(local, stack.get_index() as usize);
        assert!(specs.iter().any(|spec| spec.2 == wrapped), "{specs:?}");
        let unique = Rc::clone(manage.get_unique_space().unwrap());
        let cache = BTreeMap::from([(id, (Address::new(unique, 0x10000000), 72))]);
        let mut xml = Vec::new();
        db.encode_scope_with_storage(local, &[], &cache, &mut XmlEncode::new(&mut xml))
            .unwrap();
        let text = String::from_utf8(xml).unwrap();
        assert_eq!(text.matches("space=\"unique\"").count(), 1, "{text}");
        assert_eq!(text.matches("space=\"join\"").count(), 1, "{text}");
        assert!(text.contains("stack:0x64:36"));
        db.remove_symbol_mappings(sym);
        db.add_map_point(local, sym, &ordinary, &Address::new_invalid())
            .unwrap();
        assert_eq!(db.symbol(sym).get_id(), id);
        assert!(db.wrapped_stack_wire_specs(local).is_empty());
        let mut replaced = Vec::new();
        db.encode_scope_with_storage(local, &[], &cache, &mut XmlEncode::new(&mut replaced))
            .unwrap();
        let text = String::from_utf8(replaced).unwrap();
        assert!(!text.contains("space=\"unique\""), "{text}");
        assert_eq!(text.matches("space=\"join\"").count(), 1, "{text}");
        assert!(text.contains("stack:0x64:36"));
    }
}

#[test]
fn nonstack_and_word_addressed_wraps_keep_the_existing_rejection() {
    for word in [1, 2] {
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            word,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        let addr = Address::new(Rc::clone(&ram), ram.get_highest() - 3);
        assert!(pieces(&addr, 8).is_none());
        let stack = Rc::new(SpacebaseSpace::new("stack", 4, 4, &ram, 1, true, false));
        let addr = Address::new(Rc::clone(&stack), stack.get_highest() - 3);
        assert_eq!(pieces(&addr, 8).is_some(), word == 1);
        assert!(pieces(&Address::new(stack, 0x100), 8).is_none());
    }
}
