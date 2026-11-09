//! swifttypes end-to-end gate: lay out a fully stripped Swift binary's
//! structs and classes from its static type metadata.
//!
//! Fixture: `tests/fixtures/swift_types_elf` — an x86-64 ELF executable built
//! from `swift_types_elf.swift` with swiftc 6.3 (`-O`) and `llvm-strip
//! --strip-all`. The expected layouts are the ones the unstripped build's
//! code uses (`Pixel.area()` reads `0(%r13)`, `8(%r13)`, `0x19(%r13)`;
//! `Sprite.count.getter` reads `0x60(%r13)`), and its symbol table pins the
//! metadata records (`$s7typefix5PointVN` at 0x5318, `$s7typefix5PixelVWV`
//! at 0x5330).
//!
//! Two-state: with `swifttypes` off no layout, record or parameter is
//! installed; on, every expected one is.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, bootstrap_from_object};
use kuna_decomp::dtype::TypeFactory;

const FIXTURE: &str = "swift_types_elf";

/// `(type, size, [(field, offset, type name)])`; a pointer's type name is
/// empty, as the factory spells it.
const LAYOUTS: &[(&str, i32, &[(&str, i32, &str)])] = &[
    ("typefix.Point", 16, &[("x", 0, "int8"), ("y", 8, "int8")]),
    (
        "typefix.Pixel",
        64,
        &[
            ("at", 0, "typefix.Point"),
            ("alpha", 16, "float8"),
            ("on", 24, "bool"),
            ("tag", 25, "uint1"),
            ("label", 32, "Swift.String"),
            ("owner", 48, ""),
            ("shadow", 56, ""),
        ],
    ),
    (
        "typefix.Animal",
        40,
        &[
            ("isa", 0, ""),
            ("refcounts", 8, "uint8"),
            ("legs", 16, "int8"),
            ("name", 24, "Swift.String"),
        ],
    ),
    (
        "typefix.Dog",
        48,
        &[("super", 0, "typefix.Animal"), ("tricks", 40, "int8")],
    ),
    (
        "typefix.Sprite",
        112,
        &[
            ("isa", 0, ""),
            ("refcounts", 8, "uint8"),
            ("origin", 16, "typefix.Point"),
            ("pixel", 32, "typefix.Pixel"),
            ("count", 96, "int8"),
            ("mate", 104, ""),
        ],
    ),
];

/// `(VMA, data symbol, its type)` from the unstripped symbol table.
const RECORDS: &[(u64, &str, &str)] = &[
    (0x5318, "typefix.Point.type_metadata", "swift.type_metadata"),
    (
        0x52b0,
        "typefix.Point.value_witness_table",
        "swift.value_witness_table",
    ),
    (0x5398, "typefix.Pixel.type_metadata", "swift.type_metadata"),
    (
        0x6790,
        "typefix.Sprite.class_metadata",
        "swift.class_metadata",
    ),
];

/// `(VMA, slot, parameter, register, type)`: the register-pinned parameters.
const PARAMS: &[(u64, i32, &str, &str, &str)] = &[
    (0x3e60, 0, "self", "R13", "typefix.Sprite *"),
    (0x37f0, 0, "self", "R13", "typefix.Animal *"),
    (0x2ce0, 2, "self", "R13", "typefix.Pixel *"),
    (0x2ce0, 0, "Self", "RDI", "swift.type_metadata *"),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn load(types_on: bool) -> ConsoleProgram {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(FIXTURE);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("swiftmetadata", "on")
        .expect("swiftmetadata flips on");
    if !types_on {
        prog.arch_mut()
            .set_kuna_option("swifttypes", "off")
            .expect("swifttypes flips off");
    }
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    prog
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .expect("a default code space")
        .clone();
    Address::new(space, vma)
}

#[test]
fn layouts_records_and_self_parameters_come_from_the_metadata() {
    let prog = load(true);
    let arch = prog.arch();
    for &(name, size, fields) in LAYOUTS {
        let t = arch
            .types()
            .find_by_name(name)
            .unwrap()
            .unwrap_or_else(|| panic!("{name} is interned"));
        assert_eq!(t.get_size(), size, "{name} size");
        let mut got = Vec::new();
        let mut i = 0;
        while let Some(f) = t.get_field(i) {
            got.push((
                f.name.clone(),
                f.offset,
                f.field_type.get_name().to_string(),
            ));
            i += 1;
        }
        let want: Vec<(String, i32, String)> = fields
            .iter()
            .map(|&(n, o, ty)| (n.to_string(), o, ty.to_string()))
            .collect();
        assert_eq!(got, want, "{name} fields");
    }
    for &(vma, name, type_name) in RECORDS {
        let addr = code_addr(&prog, vma);
        let global = arch.symboltab.get_global_scope().expect("global scope");
        let eref = arch
            .symboltab
            .find_container(global, &addr, 1, &Address::new_invalid())
            .unwrap_or_else(|| panic!("a data symbol at 0x{vma:x}"));
        let sym = arch
            .symboltab
            .symbol(arch.symboltab.entry(global, eref).symbol);
        assert_eq!(sym.get_name(), name, "record at 0x{vma:x}");
        assert_eq!(
            sym.dtype.as_ref().map(|t| t.get_name()),
            Some(type_name),
            "record type at 0x{vma:x}"
        );
    }
    for &(vma, slot, pname, reg, type_name) in PARAMS {
        let locks = arch
            .loader_param_locks
            .get(&code_addr(&prog, vma))
            .unwrap_or_else(|| panic!("parameters pinned at 0x{vma:x}"));
        let (_, name, piece) = locks
            .iter()
            .find(|(i, _, _)| *i == slot)
            .unwrap_or_else(|| panic!("slot {slot} pinned at 0x{vma:x}"));
        assert_eq!(name, pname);
        let loc = arch.get_register_varnode(reg.as_bytes()).expect("register");
        assert_eq!(
            (
                piece.addr.get_space().unwrap().get_name(),
                piece.addr.get_offset()
            ),
            (loc.space.unwrap().get_name(), loc.offset),
            "{pname} register at 0x{vma:x}"
        );
        let ty = piece.type_.as_ref().expect("typed");
        let spelled = format!(
            "{} *",
            ty.get_ptr_to()
                .map(|p| p.get_name().to_string())
                .unwrap_or_default()
        );
        assert_eq!(spelled, type_name, "{pname} type at 0x{vma:x}");
    }
}

#[test]
fn swifttypes_off_installs_nothing() {
    let prog = load(false);
    let arch = prog.arch();
    assert!(arch.loader_param_locks.is_empty(), "no pinned parameters");
    for &(vma, name, _) in RECORDS {
        let addr = code_addr(&prog, vma);
        let global = arch.symboltab.get_global_scope().expect("global scope");
        let found = arch
            .symboltab
            .find_container(global, &addr, 1, &Address::new_invalid())
            .map(|e| {
                arch.symboltab
                    .symbol(arch.symboltab.entry(global, e).symbol)
                    .get_name()
                    .to_string()
            });
        assert_ne!(found.as_deref(), Some(name), "no record at 0x{vma:x}");
    }
}
