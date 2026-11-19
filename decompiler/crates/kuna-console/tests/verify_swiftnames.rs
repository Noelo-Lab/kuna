//! Swift shape-name end-to-end gate: every name the Swift metadata pass derives
//! from code shape on a stripped arm64 Mach-O is the name its unstripped twin's
//! symbol gives, or a strictly less specific one.
//!
//! Fixtures: `swift_names_macho` (swiftc 5.10, arm64 macOS, `llvm-strip
//! --strip-all`: only `_main` keeps a symbol) and `swift_names_macho_symbols`,
//! the same link before the strip, the oracle (see the fixtures README).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, bootstrap_from_object};
use object::{Object, ObjectSymbol};

const STRIPPED: &str = "swift_names_macho";
const SYMBOLS: &str = "swift_names_macho_symbols";

/// One function of each kind the arm names: `(VMA, the name)`, each also
/// the unstripped symbol's name.
const PINNED: &[(u64, &str)] = &[
    (0x1_0000_1514, "namefix.Board.width.getter"),
    (0x1_0000_1544, "namefix.Board.width.setter"),
    (0x1_0000_1580, "namefix.Board.width.modify"),
    (0x1_0000_15d0, "namefix.Board.title.getter"),
    (0x1_0000_16ac, "namefix.Board.cells.getter"),
    (0x1_0000_1748, "namefix.Board.cells.modify"),
    (0x1_0000_1ab8, "namefix.Tile.id.getter"),
    (0x1_0000_1e08, "<namefix.Tile:namefix.Shape>.id.getter"),
    (0x1_0000_1e58, "<namefix.Tile:namefix.Shape>.id.modify"),
    (0x1_0000_20bc, "<namefix.Dot:namefix.Shape>.id.getter"),
    (0x1_0000_2604, "___swift_mutable_project_boxed_opaque_existential_1"),
    (0x1_0000_262c, "___swift_instantiateConcreteTypeFromMangledName"),
    (0x1_0000_2764, "___swift_project_boxed_opaque_existential_1"),
    (0x1_0000_2788, "___swift_destroy_boxed_opaque_existential_1"),
    (0x1_0000_26d4, "namefix.Token.outlined_consume"),
    (0x1_0000_27a8, "namefix.Token.outlined_copy"),
    (0x1_0000_27f4, "namefix.Token.destroy_value_witness"),
    (0x1_0000_29cc, "namefix.Token.destructiveInjectEnumTag_value_witness"),
    (0x1_0000_28d0, "___swift_memcpy25_8"),
    (0x1_0000_2b60, "___swift_noop_void_return"),
];

fn fixture(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

fn load(name: &str, options: &[(&str, &str)]) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixture(name);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    for (k, v) in options {
        prog.arch_mut().set_kuna_option(k, v).expect("option flips");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .expect("a default code space")
        .clone();
    let addr = Address::new(Rc::clone(&space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// The addresses of the twin's own functions.
fn functions() -> Vec<u64> {
    let bytes = std::fs::read(fixture(SYMBOLS)).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let mut out: Vec<u64> = file
        .symbols()
        .filter(|s| s.kind() == object::SymbolKind::Text && s.is_definition())
        .map(|s| s.address())
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `<T:P>.requirement#3.getter` for `<T:P>.area.getter`: the same witness
/// prefix, the member replaced by its position and a kind that agrees.
fn less_specific(derived: &str, real: &str) -> bool {
    let Some((prefix, rest)) = derived.split_once(">.requirement#") else {
        return false;
    };
    let Some((_, kind)) = rest.split_once('.') else {
        return false;
    };
    let Some(member) = real.strip_prefix(prefix).and_then(|r| r.strip_prefix(">.")) else {
        return false;
    };
    match kind {
        "method" => member.ends_with(')'),
        tag => member.ends_with(&format!(".{tag}")),
    }
}

#[test]
fn derived_swift_names_match_the_unstripped_symbols() {
    let real = load(SYMBOLS, &[]);
    let on = load(STRIPPED, &[("swiftmetadata", "on")]);
    let mut named = 0;
    for vma in functions() {
        let (r, n) = (name_at(&real, vma), name_at(&on, vma));
        // A placeholder, a bare vtable position and a folded body claim no
        // member name.
        if n.starts_with("sub_") || n.contains(".vtable#") || n.contains(".merged#") {
            continue;
        }
        named += 1;
        assert!(
            n == r || less_specific(&n, &r),
            "0x{vma:x}: derived `{n}` is neither the symbol's `{r}` nor less specific"
        );
    }
    assert!(named >= 70, "the pass names {named} functions of the fixture");
    for &(vma, want) in PINNED {
        assert_eq!(name_at(&on, vma), want, "the pass names 0x{vma:x}");
        assert_eq!(name_at(&real, vma), want, "the twin's symbol at 0x{vma:x}");
    }
}

#[test]
fn less_specific_keeps_the_witness_and_kind() {
    assert!(less_specific("<M.T:M.P>.requirement#3.getter", "<M.T:M.P>.area.getter"));
    assert!(less_specific("<M.T:M.P>.requirement#4.method", "<M.T:M.P>.scaled(by:)"));
    assert!(!less_specific("<M.T:M.P>.requirement#4.getter", "<M.T:M.P>.scaled(by:)"));
    assert!(!less_specific("<M.U:M.P>.requirement#3.getter", "<M.T:M.P>.area.getter"));
    assert!(!less_specific("M.T.width.getter", "M.T.title.getter"));
}
