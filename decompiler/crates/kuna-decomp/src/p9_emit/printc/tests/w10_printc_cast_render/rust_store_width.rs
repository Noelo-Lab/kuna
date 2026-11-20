//! Compile and run emitted stores against a byte-exact model.
use super::*;
use crate::dtype::DatatypeKind;

fn render_store(
    width: i32,
    pointee: i32,
    source: Option<Rc<Datatype>>,
    constant: Option<u64>,
    lang: &str,
    nocasts: bool,
) -> String {
    let mut fd = build_fd();
    let space = ram(&fd);
    let mut pointer_type = Datatype::new_with_align(8, 8, type_metatype::TYPE_PTR);
    pointer_type.kind = DatatypeKind::Pointer {
        ptrto: named("destination", pointee, type_metatype::TYPE_UINT),
        spaceid: None,
        truncate: None,
        wordsize: 1,
    };
    let pointer = fd.new_varnode(
        8,
        &Address::new(Rc::clone(&space), 0x400),
        Some(Rc::new(pointer_type)),
    );
    let value = if let Some(raw) = constant {
        let constant_space = fd.get_arch().manage().get_space(0).unwrap().clone();
        fd.new_varnode(width, &Address::new(constant_space, raw), source)
    } else {
        fd.new_varnode(width, &Address::new(Rc::clone(&space), 0x500), source)
    };
    let space_id = fd.new_varnode_space(&space);
    let store = mk_op(&mut fd, 3, 0x20, OpCode::CPUI_STORE);
    fd.op_set_input(store, space_id, 0).unwrap();
    fd.op_set_input(store, pointer, 1).unwrap();
    fd.op_set_input(store, value, 2).unwrap();
    fd.set_high_level();
    for (varnode, name) in [(pointer, "pointer"), (value, "value")] {
        let high = fd.vbank().get(varnode).unwrap().get_high().unwrap();
        fd.high_bank_mut()
            .get_mut(high)
            .unwrap()
            .set_kuna_name(name);
    }
    let mut printer = PrintC::new();
    printer.set_name(lang);
    printer.rt_ctx.lang = printer.out_lang;
    printer.set_no_cast_printing(nocasts);
    printer.set_output_stream();
    printer.emit_statement(&fd, &bare_arch(), store);
    printer.emit_mut().output_str().to_string()
}

#[test]
fn rust_store_width_controls() {
    let unsigned = |width| Some(named("carrier", width, type_metatype::TYPE_UINT));
    assert_eq!(
        render_store(8, 8, unsigned(8), None, "rust", false),
        "*pointer = value;"
    );
    assert_eq!(
        render_store(1, 8, unsigned(8), None, "rust", true),
        "*pointer = value;"
    );
    assert_eq!(
        render_store(1, 8, unsigned(8), None, "c", false),
        "*pointer = value;"
    );
    assert_eq!(
        render_store(1, 8, unsigned(8), None, "go", false),
        "*pointer = value"
    );
    let aggregate = named("StringHeader", 16, type_metatype::TYPE_STRUCT);
    let rendered = render_store(8, 16, Some(aggregate), None, "rust", false);
    assert!(
        rendered.contains("STORE writes 8 bytes; no scalar conversion applied"),
        "{rendered}"
    );
    assert!(rendered.ends_with("*pointer = value;"), "{rendered}");
}

#[test]
fn rust_store_width_matches_memory_bytes() {
    use std::fmt::Write;
    use std::process::Command;

    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let mut program = String::from("#[repr(align(16))] struct Aligned([u8; 40]);\nfn main() {\n");
    let mut index = 0;
    for width in [1, 2, 4, 8, 16] {
        for (source_width, meta, carrier, expr, expected) in [
            (
                4,
                type_metatype::TYPE_UINT,
                "u32",
                "0xfedcba98",
                "0xfedcba98u128",
            ),
            (
                8,
                type_metatype::TYPE_INT,
                "i64",
                "-0x12345678",
                "-0x12345678i128 as u128",
            ),
            (
                8,
                type_metatype::TYPE_UINT,
                "u64",
                "0xfedcba9876543210",
                "0xfedcba9876543210u128",
            ),
            (
                16,
                type_metatype::TYPE_UNKNOWN,
                "u128",
                "0xfedcba98765432101234567890abcdef",
                "0xfedcba98765432101234567890abcdefu128",
            ),
            (1, type_metatype::TYPE_BOOL, "bool", "true", "1u128"),
        ] {
            let pointee = if width == 8 { 1 } else { 8 };
            let source =
                (meta != type_metatype::TYPE_UNKNOWN).then(|| named("carrier", source_width, meta));
            let source = source
                .or_else(|| (width != source_width).then(|| named("unknown", source_width, meta)));
            let store = render_store(width, pointee, source, None, "rust", false);
            assert!(store.contains(&format!("value: u{}", width * 8)), "{store}");
            let target = format!("u{}", pointee * 8);
            writeln!(
                program,
                r#"
    {{
        let value: {carrier} = {expr};
        let mut actual = Aligned([0x5au8; 40]);
        let mut expected = actual.0;
        let pointer = unsafe {{ actual.0.as_mut_ptr().add(3) }} as *mut {target};
        {store}
        expected[3..][..{width}].copy_from_slice(&(({expected}) as u{bits}).to_ne_bytes());
        assert_eq!(actual.0, expected, "case {index}");
    }}"#,
                bits = width * 8
            )
            .unwrap();
            index += 1;
        }
    }
    let truncated = render_store(
        1,
        8,
        Some(named("wide_literal", 8, type_metatype::TYPE_UINT)),
        Some(u64::MAX),
        "rust",
        false,
    );
    assert!(truncated.contains("(0xff,"), "{truncated}");
    writeln!(
        program,
        r#"
    {{
        let mut actual = [0x5au8; 40];
        let pointer = unsafe {{ actual.as_mut_ptr().add(3) }} as *mut u64;
        {truncated}
        assert_eq!(actual[3], 0xff);
        assert!(actual[..3].iter().chain(&actual[4..]).all(|byte| *byte == 0x5a));
    }}
}}"#
    )
    .unwrap();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("kuna-store-width-{}-{stamp}", std::process::id()));
    std::fs::create_dir(&directory).unwrap();
    let source = directory.join("stores.rs");
    let binary = directory.join("stores");
    std::fs::write(&source, program).unwrap();
    for optimization in ["0", "2"] {
        let compiled = Command::new(&rustc)
            .args(["--edition=2021", "-C", "overflow-checks=yes", "-C"])
            .arg(format!("opt-level={optimization}"))
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            compiled.status.success(),
            "source {}:\n{}",
            source.display(),
            String::from_utf8_lossy(&compiled.stderr)
        );
        let executed = Command::new(&binary).output().unwrap();
        assert!(
            executed.status.success(),
            "body {}:\n{}",
            source.display(),
            String::from_utf8_lossy(&executed.stderr)
        );
    }
    std::fs::remove_dir_all(directory).unwrap();
}
