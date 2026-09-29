use std::fs;
use std::process::Command;

fn spec(value: &str, other: &str, body: &str) -> String {
    format!(
        "define endian = little;\n\
         define alignment = 1;\n\
         define space ram type=ram_space size=4 default;\n\
         define space register type=register_space size=4;\n\
         define register offset=0 size=4 r0;\n\
         define token instr8(8) op=(0,7);\n\
         value: \"val\" is op=0 {{ {value} }}\n\
         other: \"alias\" {other}\n\
         :test value,other is value & other {{ {body} }}\n"
    )
}

fn check_warnings(source: &str, expected: [&str; 2]) {
    let scratch = tempfile::tempdir().unwrap();
    let input = scratch.path().join("collision.slaspec");
    fs::write(&input, source).unwrap();
    let mut previous = None;
    for (detailed, warning) in [false, true].into_iter().zip(expected) {
        let output = scratch.path().join(format!("compiled-{detailed}.sla"));
        let mut command = Command::new(env!("CARGO_BIN_EXE_slacomp"));
        if detailed {
            command.arg("-c");
        }
        let result = command.arg(&input).arg(&output).output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            std::str::from_utf8(&result.stderr).unwrap(),
            warning,
            "{source}"
        );
        let bytes = fs::read(&output).unwrap();
        assert!(bytes.starts_with(b"sla\x04"));
        if let Some(plain) = &previous {
            assert_eq!(&bytes, plain, "-c changed the compiled image");
        }
        previous = Some(bytes);
    }
}

#[test]
fn local_export_collisions_report_summary_and_optional_details() {
    for value in ["local tmp:4=r0+1; export tmp;", "export *[ram]:4 r0;"] {
        check_warnings(
            &spec(value, "is value { export value; }", "r0=value+other;"),
            [
                "WARN  1 constructors with local collisions between operands\n\
                 WARN  Use -c switch to list each individually\n",
                "WARN  collision.slaspec:9: Possible operand collision between symbols 'value' and 'other'\n\
                 WARN  1 constructors with local collisions between operands\n",
            ],
        );
    }
}

#[test]
fn unrelated_exports_and_build_only_constructors_have_no_collisions() {
    for (value, other, body) in [
        (
            "local tmp:4=r0+1; export tmp;",
            "is value { export value; }",
            "build value; build other;",
        ),
        (
            "local tmp:4=r0+1; export tmp;",
            "is op=0 { local another:4=r0+2; export another; }",
            "r0=value+other;",
        ),
        (
            "export r0;",
            "is value { export value; }",
            "r0=value+other;",
        ),
        (
            "export 1:4;",
            "is value { export value; }",
            "r0=value+other;",
        ),
    ] {
        check_warnings(&spec(value, other, body), ["", ""]);
    }
}
