//! A `typedef` assertion may name its own tag, a tag declared ahead of its body,
//! and a sibling record defined later, and the printed C reads through those
//! pointers as fields.
mod common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::path::PathBuf;
use std::process::Command;

/// `n->next->val`, `n->next->next->val`, `p->b->a->x` and `p->b->y` over
/// `struct Node { struct Node *next; int val; }` and `struct A { struct B *b;
/// long x; }` / `struct B { struct A *a; int y; }`, as clang -O2 emits them.
const FUNCTIONS: [(&str, &[u8]); 4] = [
    ("second_val", &[0x48, 0x8b, 0x07, 0x8b, 0x40, 0x08, 0xc3]),
    ("third_val", &[0x48, 0x8b, 0x07, 0x48, 0x8b, 0x00, 0x8b, 0x40, 0x08, 0xc3]),
    ("a_b_a_x", &[0x48, 0x8b, 0x07, 0x48, 0x8b, 0x00, 0x48, 0x8b, 0x40, 0x08, 0xc3]),
    ("a_b_y", &[0x48, 0x8b, 0x07, 0x8b, 0x40, 0x08, 0xc3]),
];

const PROTOTYPES: [&str; 4] = [
    "prototype second_val int second_val(struct Node *n)",
    "prototype third_val int third_val(struct Node *n)",
    "prototype a_b_a_x long a_b_a_x(struct A *p)",
    "prototype a_b_y int a_b_y(struct A *p)",
];

fn fixture() -> PathBuf {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    for (name, code) in FUNCTIONS {
        let value = obj.append_section_data(section, code, 16);
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size: code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let path = common::scratch_file("self-referential-records", "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    path
}

fn decompile(input: &PathBuf, func: &str, asserts: &[&str]) -> (serde_json::Value, String, i32) {
    let mut args = vec!["decompile", input.to_str().unwrap(), func, "--json", "--assert-strict"];
    for a in asserts {
        args.extend(["--assert", a]);
    }
    let (stdout, stderr, code) = common::run_kuna(&args);
    let doc = serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}\n{stderr}"));
    (doc, stderr, code)
}

fn statuses(doc: &serde_json::Value) -> Vec<(String, String)> {
    doc["assertions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| (a["assertion"].as_str().unwrap_or("").to_string(), a["status"].as_str().unwrap().to_string()))
        .collect()
}

fn code(doc: &serde_json::Value) -> String {
    doc["functions"][0]["code"].as_str().unwrap_or("").to_string()
}

/// The reporter's declarations: a member (plain, or a callback's parameter)
/// naming the record being defined, and the forward typedef ahead of the body.
#[test]
fn a_record_may_name_its_own_tag_and_a_tag_declared_ahead() {
    let input = fixture();
    for decls in [
        &["typedef struct Node { void (*callback)(struct Node *node); };"][..],
        &["typedef struct Node { void (**callbacks)(struct Node *node); };"],
        &["typedef struct Node { struct Node *next; };"],
        &["typedef struct Node Node;", "typedef struct Node { void (*callback)(Node *node); };"],
        &["typedef struct Node;", "typedef struct Node { Node *next; int val; };"],
        &["typedef union U { union U *self; int v; };"],
        &["typedef struct A { struct B *b; long x; };", "typedef struct B { struct A *a; int y; };"],
    ] {
        let (doc, stderr, code) = decompile(&input, "second_val", decls);
        assert_eq!(code, 0, "{decls:?}: {stderr}");
        for (text, status) in statuses(&doc) {
            assert_eq!(status, "applied", "{decls:?}: {text}");
        }
    }
    std::fs::remove_file(input).unwrap();
}

/// C rejects a record that holds itself by value, and an unknown tag named
/// anywhere but a member list or a bare forward declaration keeps its error.
#[test]
fn an_incomplete_record_by_value_and_an_undeclared_tag_are_rejected() {
    let input = fixture();
    for (decl, why) in [
        ("typedef struct Node { struct Node n; };", "has incomplete type"),
        ("typedef struct Node { int v; struct Node arr[2]; };", "has incomplete type"),
        ("typedef struct Node *NodePtr;", "does not represent a struct"),
        ("prototype second_val int second_val(struct Nowhere *n)", "does not represent a struct"),
    ] {
        let (doc, stderr, code) = decompile(&input, "second_val", &[decl]);
        assert_ne!(code, 0, "{decl}: strict mode fails on a rejected directive");
        let rows = statuses(&doc);
        assert_eq!(rows[0].1, "rejected", "{decl}");
        assert!(stderr.contains(why), "{decl}: {stderr}");
    }
    std::fs::remove_file(input).unwrap();
}

/// A pointer built while its record was incomplete reads the completed
/// record: the chains print as fields, and the printed C, compiled against the
/// same declarations, computes what the machine code does.
#[test]
fn a_chain_through_a_forward_pointer_prints_fields_and_round_trips() {
    let input = fixture();
    let decls = [
        "typedef struct Node Node;",
        "typedef struct Node { struct Node *next; int val; };",
        "typedef struct A { struct B *b; long x; };",
        "typedef struct B { struct A *a; int y; };",
    ];
    let expect = [
        ("second_val", "n->next->val"),
        ("third_val", "n->next->next->val"),
        ("a_b_a_x", "p->b->a->x"),
        ("a_b_y", "p->b->y"),
    ];
    let mut printed = String::new();
    for ((func, chain), proto) in expect.iter().zip(PROTOTYPES) {
        let asserts: Vec<&str> = decls.iter().copied().chain([proto]).collect();
        let (doc, stderr, code) = decompile(&input, func, &asserts);
        assert_eq!(code, 0, "{func}: {stderr}");
        let body = self::code(&doc);
        assert!(body.contains(chain), "{func}: {body}");
        printed.push_str(&body);
        printed.push('\n');
    }
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let src = common::scratch_file("self-referential-records", "c");
    let exe = common::scratch_file("self-referential-records", "exe");
    std::fs::write(
        &src,
        format!(
            r#"
typedef struct Node Node;
struct Node {{ struct Node *next; int val; }};
typedef struct A A;
typedef struct B B;
struct A {{ struct B *b; long x; }};
struct B {{ struct A *a; int y; }};
{printed}
int main(void) {{
    Node n3 = {{0, 3}}, n2 = {{&n3, 2}}, n1 = {{&n2, 1}};
    A a2 = {{0, 20}};
    B b = {{&a2, 7}};
    A a1 = {{&b, 10}};
    return !(second_val(&n1) == 2 && third_val(&n1) == 3 && a_b_a_x(&a1) == 20 && a_b_y(&a1) == 7);
}}
"#
        ),
    )
    .unwrap();
    for cc in &compilers {
        let compile = Command::new(cc).args(["-std=c11", "-O1"]).arg(&src).arg("-o").arg(&exe).output().unwrap();
        assert!(compile.status.success(), "{cc}: {}\n{printed}", String::from_utf8_lossy(&compile.stderr));
        assert!(Command::new(&exe).status().unwrap().success(), "{cc}: {printed}");
    }
    std::fs::remove_file(src).unwrap();
    std::fs::remove_file(exe).unwrap();
    std::fs::remove_file(input).unwrap();
}
