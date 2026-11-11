use super::*;
use crate::dtype::DatatypeKind;
use crate::printlanguage::{Atom, SyntaxHighlight as Highlight, TagType};

fn base(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, 1, meta))
}
fn pointer(inner: Rc<Datatype>) -> Rc<Datatype> {
    let mut ty = Datatype::new_with_align(8, 8, type_metatype::TYPE_PTR);
    ty.kind = DatatypeKind::Pointer {
        ptrto: inner,
        spaceid: None,
        truncate: None,
        wordsize: 1,
    };
    Rc::new(ty)
}

#[derive(Clone)]
enum Expr {
    Leaf(&'static str),
    Binary(&'static OpToken, Box<Expr>, Box<Expr>),
    Unary(&'static OpToken, Box<Expr>),
}
fn binary(token: &'static OpToken, left: Expr, right: Expr) -> Expr {
    Expr::Binary(token, Box::new(left), Box::new(right))
}
fn emit(printer: &mut PrintC, expr: Expr) {
    match expr {
        Expr::Leaf(value) => {
            printer.push_atom(&Atom::syntax(value, TagType::Syntax, Highlight::no_color))
        }
        Expr::Binary(token, left, right) => {
            printer.push_op(map_token(token), None);
            emit(printer, *left);
            emit(printer, *right);
        }
        Expr::Unary(token, value) => {
            printer.push_op(map_token(token), None);
            emit(printer, *value);
        }
    }
}
fn render(expr: Expr) -> String {
    let mut printer = PrintC::new();
    printer.set_name("go-language");
    printer.set_output_stream();
    emit(&mut printer, expr);
    printer.emit_mut().output_str().to_string()
}

#[test]
fn go_operator_tiers_preserve_nested_operands() {
    use tokens::*;
    use Expr::Leaf as L;
    for (expr, expected) in [
        (
            binary(&BITWISE_AND, L("a"), binary(&SHIFT_SRIGHT, L("b"), L("63"))),
            "a & (b >> 63)",
        ),
        (
            binary(&SHIFT_SRIGHT, binary(&BITWISE_AND, L("a"), L("b")), L("63")),
            "(a & b) >> 63",
        ),
        (
            binary(&SHIFT_LEFT, L("a"), binary(&SHIFT_LEFT, L("b"), L("c"))),
            "a << (b << c)",
        ),
        (
            binary(&MULTIPLY, L("a"), binary(&MODULO, L("b"), L("c"))),
            "a * (b % c)",
        ),
        (
            binary(&BINARY_MINUS, L("a"), binary(&BINARY_PLUS, L("b"), L("c"))),
            "a - (b + c)",
        ),
        (
            binary(&BINARY_PLUS, L("a"), binary(&BINARY_MINUS, L("b"), L("c"))),
            "a + (b - c)",
        ),
        (
            binary(&BITWISE_OR, L("a"), binary(&BITWISE_XOR, L("b"), L("c"))),
            "a | (b ^ c)",
        ),
        (
            binary(&BINARY_PLUS, L("a"), binary(&BINARY_PLUS, L("b"), L("c"))),
            "a + b + c",
        ),
        (
            binary(&BITWISE_AND, L("a"), binary(&BITWISE_AND, L("b"), L("c"))),
            "a & b & c",
        ),
        (
            binary(&BITWISE_XOR, L("a"), binary(&BITWISE_XOR, L("b"), L("c"))),
            "a ^ b ^ c",
        ),
        (
            binary(
                &BINARY_PLUS,
                L("a"),
                binary(&BITWISE_AND, L("b"), binary(&SHIFT_SRIGHT, L("c"), L("63"))),
            ),
            "a + b & (c >> 63)",
        ),
        (
            binary(&MULTIPLY, binary(&BINARY_PLUS, L("a"), L("b")), L("c")),
            "(a + b) * c",
        ),
        (
            binary(&LESS_THAN, L("a"), binary(&BINARY_PLUS, L("b"), L("c"))),
            "a < b + c",
        ),
        (
            binary(&NOT_EQUAL, binary(&BITWISE_AND, L("a"), L("b")), L("c")),
            "a & b != c",
        ),
        (
            binary(
                &BOOLEAN_AND,
                binary(&LESS_THAN, L("a"), L("b")),
                binary(&LESS_THAN, L("b"), L("c")),
            ),
            "a < b && b < c",
        ),
        (
            binary(&EQUAL, L("a"), binary(&EQUAL, L("b"), L("c"))),
            "a == (b == c)",
        ),
        (binary(&BOOLEAN_XOR, L("a"), L("b")), "a != b"),
        (
            Expr::Unary(
                &UNARY_MINUS,
                Box::new(Expr::Unary(&UNARY_MINUS, Box::new(L("a")))),
            ),
            "-(-a)",
        ),
        (Expr::Unary(&BITWISE_NOT, Box::new(L("a"))), "^a"),
        (
            binary(&BINARY_MINUS, binary(&BINARY_MINUS, L("a"), L("b")), L("c")),
            "(a - b) - c",
        ),
        (
            binary(&BINARY_PLUS, L("a"), binary(&MULTIPLY, L("b"), L("c"))),
            "a + b * c",
        ),
        (
            Expr::Unary(&UNARY_MINUS, Box::new(binary(&BINARY_PLUS, L("a"), L("b")))),
            "-(a + b)",
        ),
    ] {
        assert_eq!(render(expr), expected);
    }
}

#[test]
fn go_type_spelling_preserves_machine_widths() {
    use type_metatype::*;
    let cx = SpellCtx {
        lang: OutLang::Go,
        ..SpellCtx::OFF
    };
    for (size, meta, expected) in [
        (1, TYPE_UINT, "byte"),
        (1, TYPE_INT, "int8"),
        (2, TYPE_INT, "int16"),
        (4, TYPE_UINT, "uint32"),
        (8, TYPE_INT, "int64"),
        (4, TYPE_FLOAT, "float32"),
        (8, TYPE_FLOAT, "float64"),
        (3, TYPE_UNKNOWN, "[3]byte"),
        (16, TYPE_UNKNOWN, "[16]byte"),
    ] {
        let ty = Rc::new(Datatype::new_with_align(size, -1, meta));
        assert_eq!(
            GO_SPELLER.declarator(&cx, &ty),
            (expected.into(), String::new())
        );
    }
    let i32 = base(4, TYPE_INT);
    let ptr = pointer(i32);
    assert_eq!(GO_SPELLER.declarator(&cx, &ptr).0, "*int32");
    let byte = base(1, TYPE_UINT);
    let array = {
        let mut array = Datatype::new_with_align(16, 1, TYPE_ARRAY);
        array.kind = DatatypeKind::Array {
            arrayof: byte,
            arraysize: 16,
        };
        Rc::new(array)
    };
    assert_eq!(GO_SPELLER.declarator(&cx, &array).0, "[16]byte");
}

#[test]
fn package_local_record_names_reach_pointer_and_array_types() {
    let cx = SpellCtx { lang: OutLang::Go, go_package: Some(Rc::from("main")), ..SpellCtx::OFF };
    for (name, expected) in [("main.point", "*point"), ("other.point", "*other.point")] {
        let mut record = Datatype::new_with_align(16, 8, type_metatype::TYPE_STRUCT);
        record.name = name.into();
        record.display_name = name.into();
        let ptr = pointer(Rc::new(record));
        assert_eq!(GO_SPELLER.declarator(&cx, &ptr).0, expected);
        let mut array = Datatype::new_with_align(16, 8, type_metatype::TYPE_ARRAY);
        array.kind = DatatypeKind::Array { arrayof: ptr, arraysize: 2 };
        assert_eq!(GO_SPELLER.declarator(&cx, &Rc::new(array)).0, format!("[2]{expected}"));
    }
}

#[test]
fn literals_are_not_sanitized_as_identifiers() {
    let mut p = PrintC::new();
    p.set_name("go-language");
    p.set_output_stream();
    p.push_atom(&Atom::syntax(
        "\"hello, world\"",
        TagType::VarToken,
        Highlight::const_color,
    ));
    assert_eq!(p.emit_mut().output_str(), "\"hello, world\"");
    assert_eq!(identifier("range"), "_kuna_ident_72616e6765");
    assert_eq!(identifier("123name"), "_kuna_ident_3132336e616d65");
    assert_eq!(path("pkg::select"), "pkg._kuna_ident_73656c656374");
    assert_ne!(identifier("range"), identifier("_kuna_ident_72616e6765"));
    assert_ne!(identifier("a-b"), identifier("a_b"));
    for reserved in ["uint64", "nil", "unsafe", "math"] {
        assert_ne!(identifier(reserved), reserved);
    }
    let mut escaped = String::new();
    for code in [0, 9, 10, 34, 92, 233, 0x1f600] {
        escape(&mut escaped, code, '"');
    }
    assert_eq!(escaped, "\\x00\\t\\n\\\"\\\\\\u00e9\\U0001f600");
}

#[test]
fn prototypes_and_opaque_code_pointers_have_distinct_spellings() {
    use crate::fspec::{FuncProto, ParameterPieces, ProtoModel};
    use kuna_base::address::Address;
    let cx = SpellCtx {
        lang: OutLang::Go,
        ..SpellCtx::OFF
    };
    let void = base(0, type_metatype::TYPE_VOID);
    let void_ptr = pointer(void.clone());
    assert_eq!(GO_SPELLER.declarator(&cx, &void_ptr).0, "unsafe.Pointer");
    let manager = kuna_base::space::AddrSpaceManager::new();
    let mut proto = FuncProto::new();
    proto.set_internal(Rc::new(ProtoModel::new(&manager)), void);
    proto.set_param(
        0,
        "x",
        &ParameterPieces {
            addr: Address::new_invalid(),
            type_: Some(base(4, type_metatype::TYPE_INT)),
            flags: 0,
        },
    );
    proto.set_output(&ParameterPieces {
        addr: Address::new_invalid(),
        type_: Some(base(8, type_metatype::TYPE_INT)),
        flags: 0,
    });
    let mut code = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
    code.kind = DatatypeKind::Code {
        proto: Some(Rc::new(proto)),
    };
    let ptr = pointer(Rc::new(code));
    assert_eq!(GO_SPELLER.declarator(&cx, &ptr).0, "func(int32) int64");
    let opaque = base(1, type_metatype::TYPE_CODE);
    let ptr = pointer(opaque);
    assert_eq!(GO_SPELLER.declarator(&cx, &ptr).0, "unsafe.Pointer");
}
