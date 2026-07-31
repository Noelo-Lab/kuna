use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::space::{spacetype, AddrSpace};

use super::{GlobalEntry, GlobalQuery};

fn space(index: i32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "space",
        false,
        8,
        1,
        index,
        0,
        0,
        0,
    ))
}

fn addr(space: &Rc<AddrSpace>, offset: u64) -> Address {
    Address::new(Rc::clone(space), offset)
}

fn entry(
    space: &Rc<AddrSpace>,
    first: u64,
    size: i32,
    flags: u32,
    name: &str,
    addrtied: bool,
    uselimit: RangeList,
) -> GlobalEntry {
    GlobalEntry {
        space_index: space.get_index(),
        first,
        last: first.wrapping_add(size as u64).wrapping_sub(1),
        size,
        all_flags: flags,
        addrtied,
        uselimit,
        symbol_name: name.to_string(),
        symbol_offset: 0,
        symbol_type: None,
        scope_path: Vec::new(),
        is_function: false,
        func_inject_id: -1,
    }
}

fn query(entries: Vec<GlobalEntry>) -> GlobalQuery {
    GlobalQuery::new(entries, RangeList::new(), PartMap::new(0))
}

#[test]
fn groups_cross_space_entries_without_changing_within_space_order() {
    let space3 = space(3);
    let space9 = space(9);
    let query = query(vec![
        entry(
            &space9,
            0x100,
            4,
            0x91,
            "first-nine",
            true,
            RangeList::new(),
        ),
        entry(
            &space3,
            0x100,
            4,
            0x31,
            "only-three",
            true,
            RangeList::new(),
        ),
        entry(
            &space9,
            0x200,
            4,
            0x92,
            "second-nine",
            true,
            RangeList::new(),
        ),
    ]);

    let names: Vec<&str> = query
        .entries_for_space(9)
        .iter()
        .map(|entry| entry.symbol_name.as_str())
        .collect();
    assert_eq!(names, ["first-nine", "second-nine"]);
    assert!(query.entries_for_space(5).is_empty());
    assert_eq!(
        query.query_properties(&addr(&space3, 0x100), 4, &Address::new_invalid()),
        0x31
    );
    assert_eq!(
        query.query_properties(&addr(&space9, 0x100), 4, &Address::new_invalid()),
        0x91
    );
}

#[test]
fn equal_size_overlap_keeps_the_original_first_match() {
    let space3 = space(3);
    let space9 = space(9);
    let query = query(vec![
        entry(&space3, 0x100, 4, 0x31, "first", true, RangeList::new()),
        entry(
            &space9,
            0x100,
            4,
            0x91,
            "other-space",
            true,
            RangeList::new(),
        ),
        entry(&space3, 0x100, 4, 0x32, "second", true, RangeList::new()),
    ]);

    assert_eq!(
        query.query_properties(&addr(&space3, 0x100), 4, &Address::new_invalid()),
        0x31
    );
}

#[test]
fn in_use_filter_still_selects_the_smallest_active_entry() {
    let data = space(3);
    let code = space(4);
    let mut inactive = RangeList::new();
    inactive.insert_range(Rc::clone(&code), 0x400, 0x4ff);
    let mut active = RangeList::new();
    active.insert_range(Rc::clone(&code), 0x500, 0x5ff);
    let query = query(vec![
        entry(&data, 0x100, 16, 0x10, "outer", true, RangeList::new()),
        entry(&data, 0x100, 4, 0x20, "inactive", false, inactive),
        entry(&data, 0x100, 4, 0x30, "active", false, active),
    ]);

    assert_eq!(
        query.query_properties(&addr(&data, 0x100), 4, &addr(&code, 0x500)),
        0x30
    );
    assert_eq!(
        query.query_properties(&addr(&data, 0x100), 4, &Address::new_invalid()),
        0x10
    );
}
