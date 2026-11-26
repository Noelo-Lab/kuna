use crate::common;
use kuna_base::address::Address;
use kuna_console::{assertions, assertsyntax, engine};
use kuna_decomp::varnode::varnode_flags;
use std::rc::Rc;

// AVR lds r24, 0x181; ret. `mem` is byte-addressed; `code` is word-addressed.
const IMAGE: &str = r#"<binaryimage arch="avr8:LE:16:default:gcc">
<bytechunk space="code" offset="0x20">809181010895</bytechunk>
<bytechunk space="mem" offset="0x181">0000</bytechunk>
<symbol space="code" offset="0x20" name="read_sample"/>
</binaryimage>"#;

fn image() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("read_sample.xml");
    std::fs::write(&path, IMAGE).unwrap();
    (dir, path)
}

#[test]
fn explicit_spaces_use_their_own_address_units() {
    let (_dir, path) = image();
    let prog_roots = [common::repo_root().join("specs").to_str().unwrap().to_owned()];
    let mut prog = engine::bootstrap_from_file(path.to_str().unwrap(), "", &prog_roots).unwrap();
    let directives = [
        "data 0x181 unsigned char original_default",
        "data [code,181] unsigned char word_addressed",
        "data [mem,0x181] volatile unsigned short memory_mapped",
    ];
    prog.set_assertions(directives.map(|spec| assertsyntax::parse_directive(spec).unwrap()).into());
    assertions::apply_program_scoped(&mut prog);
    let outcomes = prog.assertion_outcomes();
    assert_eq!(outcomes.len(), directives.len());
    assert!(outcomes.iter().all(|row| row.status == "applied"), "{outcomes:?}");

    let arch = prog.arch();
    let db = &arch.symboltab;
    let scope = db.get_global_scope().unwrap();
    for (space_name, byte_offset, name, size) in [
        ("code", 0x181, "original_default", 1),
        ("code", 0x302, "word_addressed", 1),
        ("mem", 0x181, "memory_mapped", 2),
    ] {
        let space = arch.manage().get_space_by_name(space_name).unwrap();
        let rows = db.scope_space_symbol_specs(scope, space.get_index() as usize);
        let (_, ty, addr, _) = rows.iter().find(|(label, ..)| label == name).expect(name);
        assert_eq!(*addr, Address::new(Rc::clone(space), byte_offset));
        assert_eq!(ty.get_size(), size);
    }

    let mem = arch.manage().get_space_by_name("mem").unwrap();
    let code = arch.manage().get_space_by_name("code").unwrap();
    let volatile_at = |space, offset| {
        db.get_property(&Address::new(Rc::clone(space), offset)) & varnode_flags::volatil != 0
    };
    assert!(volatile_at(mem, 0x181));
    assert!(volatile_at(mem, 0x182));
    assert!(!volatile_at(mem, 0x183));
    assert!(!volatile_at(code, 0x181));
    assert!(!volatile_at(code, 0x302));
}

#[test]
fn named_data_renders_and_rejects_unknown_spaces_in_both_cli_paths() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("read_sample.bin");
    std::fs::write(&path, [0x80, 0x91, 0x81, 0x01, 0x08, 0x95]).unwrap();
    for format in [&[][..], &["--json"][..]] {
        let mut args = vec![
            "decompile", path.to_str().unwrap(), "0x10", "--addr", "--assert-strict",
            "--raw-image", "--target", "avr8:LE:16:default:gcc", "--base", "0x10",
        ];
        args.extend_from_slice(format);
        let mut valid = args.clone();
        valid.extend([
            "--assert", "data 0x181 unsigned char in_default_space",
            "--assert", "data [mem,0x181] unsigned char named_sample",
        ]);
        let (stdout, stderr, code) = common::run_kuna(&valid);
        assert_eq!(code, 0, "{stdout}\n{stderr}");
        let body = if format.is_empty() {
            stdout
        } else {
            serde_json::from_str::<serde_json::Value>(&stdout).unwrap()["functions"][0]["code"]
                .as_str().unwrap().to_owned()
        };
        assert!(body.contains("named_sample"), "{body}");
        assert!(!body.contains("in_default_space"), "{body}");

        for (assertion, detail) in [
            (
                "data [MEM,0x181] unsigned char misplaced",
                "unknown processor space \"MEM\" (this target has: OTHER, code, register, mem, codebyte)",
            ),
            ("data [code,0x10000] unsigned char misplaced", "address 0x10000 is outside space \"code\""),
        ] {
            let mut invalid = args.clone();
            invalid.extend(["--assert", assertion]);
            let (stdout, stderr, code) = common::run_kuna(&invalid);
            assert_eq!(code, 1, "{stdout}\n{stderr}");
            assert!(stderr.contains(detail), "{stderr}");
        }
    }
}
