//! GOLDEN GATE for `w1-base-space-address`: replay the C++ oracle's
//! `golden addrsort` vectors (`tests/golden/vectors/addrsort.csv`, emitted
//! by `decompiler/cpp/kuna_goldengen.cc`) against the Rust `Address`
//! comparators.
//!
//! Per `tests/golden/vectors/README.md`, the space set is a property of the
//! pinned architecture datatest (`tests/datatests/floatprint.xml`,
//! x86:LE:64:default:gcc) whose spaces, in space-index order, are:
//!
//! | index | space    | construction                                  |
//! |-------|----------|-----------------------------------------------|
//! | 0     | const    | ConstantSpace                                 |
//! | 1     | OTHER    | OtherSpace                                    |
//! | 2     | unique   | UniqueSpace                                   |
//! | 3     | ram      | AddrSpace IPTR_PROCESSOR (8-byte, LE)         |
//! | 4     | register | AddrSpace IPTR_PROCESSOR (4-byte, LE)         |
//! | 5     | fspec    | FspecSpace                                    |
//! | 6     | iop      | IopSpace                                      |
//! | 7     | join     | JoinSpace                                     |
//! | 8     | stack    | base AddrSpace, IPTR_SPACEBASE (*)            |
//!
//! (*) The C++ stack space is a `SpacebaseSpace` (translate.cc, not yet
//! ported); the Address comparators only consult the space *identity* and
//! *index*, so a base `AddrSpace` with type IPTR_SPACEBASE, name "stack" and
//! index 8 is comparator-equivalent.
//!
//! The vectors enumerate every ordered pair of Address(space, offset) over
//! all 9 spaces x offsets {0, 1, 0x1000, 0xffffffff} and record the C++
//! `<`, `==`, `<=` decisions: 36 x 36 = 1296 rows.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace, JoinSpace,
    OtherSpace, UniqueSpace,
};

/// Build the manager mirroring the pinned x86:LE:64 architecture's space set
/// (index order is the comparator-relevant property).
fn build_pinned_spaces() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap(); // 0 const
    manager.insert_space(Rc::new(OtherSpace::new(OtherSpace::INDEX))).unwrap(); // 1 OTHER
    manager.insert_space(Rc::new(UniqueSpace::new(2, 0, false))).unwrap(); // 2 unique
    manager
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            3,
            kuna_base::space::addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap(); // 3 ram
    manager
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            4,
            1,
            4,
            0,
            0,
            0,
        )))
        .unwrap(); // 4 register
    manager.insert_space(Rc::new(FspecSpace::new(5))).unwrap(); // 5 fspec
    manager.insert_space(Rc::new(IopSpace::new(6))).unwrap(); // 6 iop
    manager.insert_space(Rc::new(JoinSpace::new(7, false))).unwrap(); // 7 join
    manager
        .insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_SPACEBASE,
            "stack",
            false,
            8,
            1,
            8,
            0,
            1,
            1,
        )))
        .unwrap(); // 8 stack (SpacebaseSpace stand-in; see module docs)
    manager.set_default_code_space(3).unwrap();
    manager
}

/// Parse a `0x`-prefixed lower-case hex literal (goldenHex format).
fn parse_hex(s: &str) -> u64 {
    let hex = s.strip_prefix("0x").unwrap_or_else(|| panic!("bad hex literal: {s}"));
    u64::from_str_radix(hex, 16).unwrap_or_else(|_| panic!("bad hex literal: {s}"))
}

#[test]
fn golden_addrsort_comparator_decisions() {
    let manager = build_pinned_spaces();
    let mut by_name: BTreeMap<String, Rc<AddrSpace>> = BTreeMap::new();
    for i in 0..manager.num_spaces() {
        let spc = manager.get_space(i).expect("space registered");
        by_name.insert(spc.get_name().to_string(), Rc::clone(spc));
    }

    let csv_path = format!(
        "{}/../../../tests/golden/vectors/addrsort.csv",
        env!("CARGO_MANIFEST_DIR")
    );
    let data = std::fs::read_to_string(&csv_path)
        .unwrap_or_else(|e| panic!("cannot read {csv_path}: {e}"));

    let mut rows = 0usize;
    for (lineno, line) in data.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        assert_eq!(fields.len(), 7, "line {}: bad row: {line}", lineno + 1);
        let space_a = by_name
            .get(fields[0])
            .unwrap_or_else(|| panic!("line {}: unknown space {}", lineno + 1, fields[0]));
        let off_a = parse_hex(fields[1]);
        let space_b = by_name
            .get(fields[2])
            .unwrap_or_else(|| panic!("line {}: unknown space {}", lineno + 1, fields[2]));
        let off_b = parse_hex(fields[3]);
        let want_less = fields[4] == "1";
        let want_equal = fields[5] == "1";
        let want_lessequal = fields[6] == "1";

        let a = Address::new(Rc::clone(space_a), off_a);
        let b = Address::new(Rc::clone(space_b), off_b);

        assert_eq!(
            a < b,
            want_less,
            "line {}: ({},{:#x}) < ({},{:#x})",
            lineno + 1,
            fields[0],
            off_a,
            fields[2],
            off_b
        );
        assert_eq!(
            a == b,
            want_equal,
            "line {}: ({},{:#x}) == ({},{:#x})",
            lineno + 1,
            fields[0],
            off_a,
            fields[2],
            off_b
        );
        assert_eq!(
            a <= b,
            want_lessequal,
            "line {}: ({},{:#x}) <= ({},{:#x})",
            lineno + 1,
            fields[0],
            off_a,
            fields[2],
            off_b
        );
        rows += 1;
    }
    // 9 spaces x 4 offsets = 36 addresses; 36 x 36 ordered pairs
    assert_eq!(rows, 1296, "expected the full pair matrix from {csv_path}");
}
