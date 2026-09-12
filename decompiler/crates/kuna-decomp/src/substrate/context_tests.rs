use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::space::{spacetype, AddrSpace};

use crate::dtype::{type_metatype, Datatype, DatatypeKind};

use super::{GlobalEntry, GlobalQuery, IndirectSlotType};

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
        func_no_return: false,
        symbol_id: 0,
    }
}

fn query(entries: Vec<GlobalEntry>) -> GlobalQuery {
    GlobalQuery::new(entries, RangeList::new(), PartMap::new(0))
}

#[test]
fn global_symbol_name_snapshot_exposes_names_reserved_from_local_suffixes() {
    let ram = space(2);
    let globals = query(vec![
        entry(&ram, 0x1000, 4, 0, "value_1", true, RangeList::new()),
        entry(&ram, 0x2000, 1, 0, "callee_1", true, RangeList::new()),
    ]);
    assert_eq!(globals.symbol_names().collect::<Vec<_>>(), ["value_1", "callee_1"]);
}

fn typed_entry(
    space: &Rc<AddrSpace>,
    first: u64,
    size: i32,
    name: &str,
    ty: Rc<Datatype>,
    is_function: bool,
) -> GlobalEntry {
    let mut result = entry(space, first, size, 0, name, true, RangeList::new());
    result.symbol_type = Some(ty);
    result.is_function = is_function;
    result
}

fn code_noproto() -> Rc<Datatype> {
    let mut ty = Datatype::new(1, type_metatype::TYPE_CODE);
    ty.kind = DatatypeKind::Code { proto: None };
    Rc::new(ty)
}

fn code_proto(proto: Rc<crate::fspec::FuncProto>) -> Rc<Datatype> {
    let mut ty = Datatype::new(1, type_metatype::TYPE_CODE);
    ty.kind = DatatypeKind::Code { proto: Some(proto) };
    Rc::new(ty)
}

fn ptr_to(ty: Rc<Datatype>) -> Rc<Datatype> {
    let mut ptr = Datatype::new(8, type_metatype::TYPE_PTR);
    ptr.kind = DatatypeKind::Pointer {
        ptrto: ty,
        spaceid: None,
        truncate: None,
        wordsize: 1,
    };
    Rc::new(ptr)
}

#[test]
fn indirect_slot_accepts_only_an_exact_function_or_prototyped_function_pointer() {
    let data = space(3);
    let at = addr(&data, 0x100);
    let usepoint = Address::new_invalid();

    let function = query(vec![typed_entry(
        &data,
        0x100,
        1,
        "target",
        code_noproto(),
        true,
    )]);
    assert!(matches!(
        function.indirect_slot_type(&at, 8, &usepoint),
        Some(IndirectSlotType::FunctionCode)
    ));
    for malformed_width in [4, 16] {
        assert!(matches!(
            function.indirect_slot_type(&at, malformed_width, &usepoint),
            Some(IndirectSlotType::Untyped)
        ));
    }

    let non_pointer = query(vec![typed_entry(
        &data,
        0x100,
        8,
        "scalar",
        Rc::new(Datatype::new(8, type_metatype::TYPE_INT)),
        false,
    )]);
    assert!(matches!(
        non_pointer.indirect_slot_type(&at, 8, &usepoint),
        Some(IndirectSlotType::Untyped)
    ));

    let pointer_to_data = query(vec![typed_entry(
        &data,
        0x100,
        8,
        "data_ptr",
        ptr_to(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))),
        false,
    )]);
    assert!(matches!(
        pointer_to_data.indirect_slot_type(&at, 8, &usepoint),
        Some(IndirectSlotType::Untyped)
    ));

    let unprototyped_code_pointer = query(vec![typed_entry(
        &data,
        0x100,
        8,
        "code_ptr",
        ptr_to(code_noproto()),
        false,
    )]);
    assert!(matches!(
        unprototyped_code_pointer.indirect_slot_type(&at, 8, &usepoint),
        Some(IndirectSlotType::Untyped)
    ));

    let proto = Rc::new(crate::fspec::FuncProto::new());
    let function_pointer = query(vec![typed_entry(
        &data,
        0x100,
        8,
        "function_ptr",
        ptr_to(code_proto(Rc::clone(&proto))),
        false,
    )]);
    match function_pointer.indirect_slot_type(&at, 8, &usepoint) {
        Some(IndirectSlotType::Prototype(resolved)) => assert!(
            Rc::ptr_eq(&resolved, &proto),
            "the exact FuncProto, including its model, must survive classification"
        ),
        other => panic!("function pointer was not classified as callable: {other:?}"),
    }
    assert!(matches!(
        function_pointer.indirect_slot_type(&at, 4, &usepoint),
        Some(IndirectSlotType::Untyped)
    ));
}

#[test]
fn explicit_data_at_slot_shadows_loader_function_metadata() {
    let data = space(3);
    let at = addr(&data, 0x100);
    let query = query(vec![
        typed_entry(&data, 0x100, 1, "target", code_noproto(), true),
        typed_entry(
            &data,
            0x100,
            8,
            "operator_data",
            Rc::new(Datatype::new(8, type_metatype::TYPE_INT)),
            false,
        ),
    ]);

    assert!(matches!(
        query.indirect_slot_type(&at, 8, &Address::new_invalid()),
        Some(IndirectSlotType::Untyped)
    ));
}

#[test]
fn usepoint_limited_data_shadows_only_where_it_is_active() {
    let data = space(3);
    let code = space(4);
    let at = addr(&data, 0x100);
    let mut limited = typed_entry(
        &data,
        0x100,
        8,
        "local_view",
        Rc::new(Datatype::new(8, type_metatype::TYPE_INT)),
        false,
    );
    limited.addrtied = false;
    limited.uselimit.insert_range(Rc::clone(&code), 0x400, 0x4ff);
    let query = query(vec![
        typed_entry(&data, 0x100, 1, "target", code_noproto(), true),
        limited,
    ]);

    assert!(matches!(
        query.indirect_slot_type(&at, 8, &addr(&code, 0x440)),
        Some(IndirectSlotType::Untyped)
    ));
    assert!(matches!(
        query.indirect_slot_type(&at, 8, &addr(&code, 0x540)),
        Some(IndirectSlotType::FunctionCode)
    ));
}

#[test]
fn indirect_slot_does_not_reinterpret_an_interior_aggregate_address() {
    let data = space(3);
    let query = query(vec![typed_entry(
        &data,
        0x100,
        16,
        "aggregate",
        Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT)),
        false,
    )]);

    assert!(matches!(
        query.indirect_slot_type(&addr(&data, 0x108), 8, &Address::new_invalid()),
        Some(IndirectSlotType::Untyped)
    ));
    assert!(query
        .indirect_slot_type(&addr(&data, 0x200), 8, &Address::new_invalid())
        .is_none());
}

fn linear_container_flags(
    entries: &[GlobalEntry],
    address: &Address,
    size: i32,
    usepoint: &Address,
) -> Option<u32> {
    let space_index = address.get_space()?.get_index();
    let start = address.get_offset();
    let end = start.wrapping_add(size as u64).wrapping_sub(1);
    let mut best = None;
    let mut oldsize = -1;
    for candidate in entries {
        if candidate.space_index != space_index
            || candidate.first > start
            || candidate.last < end
        {
            continue;
        }
        if candidate.size >= oldsize && oldsize != -1 {
            continue;
        }
        let in_use = if candidate.addrtied {
            true
        } else if usepoint.is_invalid() {
            false
        } else {
            candidate.uselimit.in_range(usepoint, 1)
        };
        if in_use {
            best = Some(candidate.all_flags);
            if candidate.size == size {
                break;
            }
            oldsize = candidate.size;
        }
    }
    best
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

#[test]
fn interval_order_does_not_replace_original_order_for_equal_sizes() {
    let data = space(3);
    let query = query(vec![
        entry(&data, 0x108, 16, 0x11, "original-first", true, RangeList::new()),
        entry(&data, 0x100, 16, 0x22, "offset-first", true, RangeList::new()),
    ]);

    assert_eq!(
        query.find_container_flags(&addr(&data, 0x108), 8, &Address::new_invalid()),
        Some(0x11)
    );
}

#[test]
fn wrapped_end_preserves_exact_size_break_order() {
    let data = space(3);
    let exact = entry(
        &data,
        u64::MAX - 1,
        2,
        0x11,
        "exact",
        true,
        RangeList::new(),
    );
    let smaller = entry(&data, 0, 1, 0x22, "smaller", true, RangeList::new());
    let address = addr(&data, u64::MAX);

    let exact_first = query(vec![exact.clone(), smaller.clone()]);
    assert_eq!(
        exact_first.find_container_flags(&address, 2, &Address::new_invalid()),
        Some(0x11)
    );

    let smaller_first = query(vec![smaller, exact]);
    assert_eq!(
        smaller_first.find_container_flags(&address, 2, &Address::new_invalid()),
        Some(0x22)
    );
}

#[test]
fn interval_index_matches_linear_overlap_and_usepoint_semantics() {
    let data = space(3);
    let other_data = space(9);
    let code = space(4);

    let mut use_400 = RangeList::new();
    use_400.insert_range(Rc::clone(&code), 0x400, 0x4ff);
    let mut use_500 = RangeList::new();
    use_500.insert_range(Rc::clone(&code), 0x500, 0x5ff);

    let entries = vec![
        entry(&data, 0x108, 16, 0x11, "original-first", true, RangeList::new()),
        entry(
            &other_data,
            0x100,
            64,
            0x90,
            "other-space",
            true,
            RangeList::new(),
        ),
        entry(&data, 0x80, 192, 0x12, "outer", true, RangeList::new()),
        entry(&data, 0x100, 16, 0x13, "offset-first", true, RangeList::new()),
        entry(&data, 0x104, 12, 0x14, "limited-400", false, use_400),
        entry(&data, 0x104, 12, 0x15, "limited-500", false, use_500),
        entry(&data, 0x108, 4, 0x16, "exact", false, {
            let mut use_exact = RangeList::new();
            use_exact.insert_range(Rc::clone(&code), 0x520, 0x52f);
            use_exact
        }),
        entry(&data, 0x200, 8, 0x17, "disjoint", true, RangeList::new()),
        entry(
            &data,
            u64::MAX - 1,
            2,
            0x18,
            "tail-exact",
            true,
            RangeList::new(),
        ),
        entry(&data, 0, 1, 0x19, "wrapped-smaller", true, RangeList::new()),
    ];
    let query = query(entries.clone());
    let usepoints = [
        Address::new_invalid(),
        addr(&code, 0x450),
        addr(&code, 0x525),
        addr(&code, 0x580),
        addr(&code, 0x600),
    ];

    for offset in (0x70..=0x210).step_by(3) {
        for size in [1, 2, 4, 8, 12, 16, 24] {
            for usepoint in &usepoints {
                let address = addr(&data, offset);
                assert_eq!(
                    query.find_container_flags(&address, size, usepoint),
                    linear_container_flags(&entries, &address, size, usepoint),
                    "offset={offset:#x} size={size} usepoint={usepoint:?}"
                );
            }
        }
    }

    for offset in [u64::MAX - 4, u64::MAX - 3, u64::MAX - 2, u64::MAX - 1, u64::MAX] {
        for size in [1, 2, 3, 4, 8] {
            for usepoint in &usepoints {
                let address = addr(&data, offset);
                assert_eq!(
                    query.find_container_flags(&address, size, usepoint),
                    linear_container_flags(&entries, &address, size, usepoint),
                    "wrapped offset={offset:#x} size={size} usepoint={usepoint:?}"
                );
            }
        }
    }
}
