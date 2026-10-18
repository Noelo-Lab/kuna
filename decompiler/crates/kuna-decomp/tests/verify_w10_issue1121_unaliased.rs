//! Regression for overlapping stack entries reordered by EntryMap.
//!
//! A wide local at 8..0x2b is split when the home-slot entry at 0x28 is added.
//! Adding a short local at 8..0xf then leaves the record list ordered as
//! 0x28, 8, 8. `mark_unaliased` must walk those entries by end offset so the
//! alias at 0x28 does not make the short local's distance wrap around. The
//! home entry is type-locked: its alias still reaches the overlapping wide
//! local, but it must not reach a later disjoint local or cross a mapped gap.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::database::Database;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::varnode::varnode_flags;

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn stack_space() -> Rc<AddrSpace> {
    let mut spaces = AddrSpaceManager::new();
    spaces.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    spaces
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    spaces
        .insert_space(Rc::new(AddrSpace::new(
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
    spaces
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            4,
            1,
            3,
            0,
            0,
            0,
        )))
        .unwrap();
    spaces
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "stack",
            false,
            8,
            1,
            4,
            0,
            0,
            0,
        )))
        .unwrap();
    Rc::clone(spaces.get_space(4).unwrap())
}

#[test]
fn overlapping_stack_entries_remain_aliased() {
    let stack = stack_space();
    let mut db = Database::new(true);
    let scope = db.find_create_scope(0, "", None, 5).unwrap();
    db.add_range(scope, Rc::clone(&stack), 8, 0x2f);
    db.add_range(scope, Rc::clone(&stack), 0x80, 0x83);
    db.add_range(scope, Rc::clone(&stack), 0x1000, 0x1003);

    // This order creates the non-address-ordered EntryMap record list: adding
    // the home slot splits the wide range, then the short range is inserted
    // before the wide record but after the home-slot record.
    let (wide, _) = db
        .add_symbol_mapped(
            scope,
            "wide",
            dt(36),
            &Address::new(Rc::clone(&stack), 8),
            &Address::new_invalid(),
        )
        .unwrap();
    let (home, _) = db
        .add_symbol_mapped(
            scope,
            "home",
            Rc::new(Datatype::new(1, type_metatype::TYPE_STRUCT)),
            &Address::new(Rc::clone(&stack), 0x28),
            &Address::new_invalid(),
        )
        .unwrap();
    db.set_attribute(home, varnode_flags::typelock);
    let (short, _) = db
        .add_symbol_mapped(
            scope,
            "short",
            dt(8),
            &Address::new(Rc::clone(&stack), 8),
            &Address::new_invalid(),
        )
        .unwrap();
    let (after_block, _) = db
        .add_symbol_mapped(
            scope,
            "after_block",
            dt(4),
            &Address::new(Rc::clone(&stack), 0x2c),
            &Address::new_invalid(),
        )
        .unwrap();
    let (new_range_alias, _) = db
        .add_symbol_mapped(
            scope,
            "new_range_alias",
            dt(4),
            &Address::new(Rc::clone(&stack), 0x80),
            &Address::new_invalid(),
        )
        .unwrap();
    let (after_gap, _) = db
        .add_symbol_mapped(
            scope,
            "after_gap",
            dt(4),
            &Address::new(Rc::clone(&stack), 0x1000),
            &Address::new_invalid(),
        )
        .unwrap();

    let record_starts: Vec<_> = db
        .scope_space_symbol_hints(scope, 4)
        .into_iter()
        .map(|(offset, _, _)| offset)
        .collect();
    assert_eq!(record_starts, vec![0x28, 8, 8, 0x2c, 0x80, 0x1000]);

    db.mark_unaliased(scope, 4, &[8, 40, 40, 0x80], 2);

    for sym in [wide, home, short] {
        assert_eq!(
            db.symbol(sym).get_flags() & varnode_flags::nolocalalias,
            0,
            "the overlapping alias should preserve {}",
            db.symbol(sym).get_name()
        );
    }
    assert_ne!(
        db.symbol(after_block).get_flags() & varnode_flags::nolocalalias,
        0,
        "a type-locked entry must stop aliases from reaching later symbols"
    );
    assert_eq!(
        db.symbol(new_range_alias).get_flags() & varnode_flags::nolocalalias,
        0,
        "an alias in a later mapped range should start a new alias run"
    );
    assert_ne!(
        db.symbol(after_gap).get_flags() & varnode_flags::nolocalalias,
        0,
        "an alias must not propagate across the unmapped stack gap"
    );
}
