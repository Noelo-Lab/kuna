use std::fs;
use std::process::{Command, Output};

const HEADER: &str = "define endian=little;\n\
    define space ram type=ram_space size=4 default;\n\
    define space register type=register_space size=4;\n\
    define register offset=0 size=4 r0;\n";

fn compile(body: &str, includes: &[(&str, &str)], strict: bool) -> (Output, Option<Vec<u8>>) {
    let scratch = tempfile::tempdir().unwrap();
    let source = scratch.path().join("main.slaspec");
    let output = scratch.path().join("compiled.sla");
    fs::write(&source, format!("{HEADER}{body}")).unwrap();
    for (name, text) in includes {
        fs::write(scratch.path().join(name), text).unwrap();
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_slacomp"));
    if strict {
        command.arg("-l");
    }
    let result = command.arg(&source).arg(&output).output().unwrap();
    (result, fs::read(output).ok())
}

#[test]
fn pattern_build_errors_report_the_undefined_operand() {
    for (body, includes, expected) in [
        (
            "define token instr(8) op=(0,7);\n:bad ghost is op=0 { r0=1; }\n",
            &[][..],
            "main.slaspec:7 - ERROR Error: ghost: operand is undefined: for table \"instruction\" constructor starting at line 6\n\nNo output produced\n",
        ),
        (
            "define token instr(8) op=(0,7);\n@include \"value.sinc\"\n:bad value is value { r0=value; }\n",
            &[("value.sinc", "value: ghost is op=0 { export r0; }\n")][..],
            "main.slaspec:8 - ERROR value.sinc:1: Problem in table 'value':Error: ghost: operand is undefined: for table \"value\" constructor starting at line 1\n\nNo output produced\n",
        ),
    ] {
        let (result, image) = compile(body, includes, false);
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), expected);
        assert!(image.is_none());
    }
}

#[test]
fn unreferenced_table_warning_names_the_table() {
    let (result, image) = compile(
        "define token instr(8) op=(0,7);\n@include \"unused.sinc\"\n:good is op=0 { r0=1; }\n",
        &[("unused.sinc", "unused: \"value\" is op=1 { export r0; }\n")],
        false,
    );
    assert!(result.status.success());
    assert_eq!(
        std::str::from_utf8(&result.stderr).unwrap(),
        "WARN  unused.sinc:1: Unreferenced table 'unused'\n"
    );
    assert!(image.unwrap().starts_with(b"sla\x04"));
}

#[test]
fn pattern_conflicts_keep_both_table_and_constructor_locations() {
    for identical in [false, true] {
        let left = format!(
            "left: \"low\" is lo=1 {{ export r0; }}\nleft: \"high\" is {} {{ export r0; }}\n",
            if identical { "lo=1" } else { "hi=2" },
        );
        let right = format!(
            "right: \"low\" is lo=3 {{ export r0; }}\nright: \"high\" is {} {{ export r0; }}\n",
            if identical { "lo=3" } else { "hi=4" },
        );
        for strict in [false, true] {
            let (result, image) = compile(
                "define token instr(16) lo=(0,3) hi=(4,7) sel=(8,15);\n\
                 @include \"left.sinc\"\n\
                 @include \"right.sinc\"\n\
                 :one left is sel=0 & left { r0=left; }\n\
                 :two right is sel=1 & right { r0=right; }\n",
                &[("left.sinc", &left), ("right.sinc", &right)],
                strict,
            );
            if !identical && !strict {
                assert!(result.status.success());
                assert!(result.stderr.is_empty());
                assert!(image.unwrap().starts_with(b"sla\x04"));
                continue;
            }
            let message = if identical {
                "Constructor has identical pattern to constructor at"
            } else {
                "Constructor pattern cannot be distinguished from constructor at"
            };
            let expected = format!(
                "main.slaspec:10 - ERROR left.sinc:2: {message} left.sinc:1\n\
                 main.slaspec:10 - ERROR left.sinc:1: {message} left.sinc:2\n\
                 main.slaspec:10 - ERROR right.sinc:2: {message} right.sinc:1\n\
                 main.slaspec:10 - ERROR right.sinc:1: {message} right.sinc:2\n\
                 No output produced\n"
            );
            assert_eq!(result.status.code(), Some(2));
            assert_eq!(std::str::from_utf8(&result.stderr).unwrap(), expected);
            assert!(image.is_none());
        }
    }
}
