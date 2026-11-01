//! Go parses across statement forms and executes the supported scalar surface.

use crate::common;
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::{Path, PathBuf};
use std::process::Command;

const FUNCTIONS: &[&str] = &[
    "add",
    "tiers",
    "choose",
    "sum_for",
    "bottom",
    "short_calls",
    "loop_calls",
    "dispatch",
    "read_at",
    "write_at",
    "from_word",
    "to_word",
    "retype",
    "narrow",
    "extend",
    "number",
    "bits",
    "unbits",
    "text",
    "rune",
    "record",
    "bottom_continue",
    "probe",
    "condition_calls",
    "condition_loop",
    "fallthroughs",
    "condition_value",
    "condition_for",
    "name_collision",
    "go",
    "callback",
    "call_callback",
    "shadow",
    "bool_value",
    "bool_number",
    "bool_true",
    "bool_false",
    "null_pointer",
    "is_null",
    "shadow_null",
    "nested_calls",
];
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn fixture(name: &str) -> PathBuf {
    root().join("tests/fixtures/go-output").join(name)
}

fn scratch() -> PathBuf {
    let path = common::scratch_file("go-output", "dir");
    std::fs::create_dir(&path).unwrap();
    path
}

fn decompile(binary: &Path, names: &[&str], options: &[&str]) -> Vec<String> {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let mut commands = options.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let prefix = commands.len();
    for name in names {
        commands.extend([
            format!("load function {name}"),
            "decompile".into(),
            "print C".into(),
        ]);
    }
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..prefix {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    let mut code = Vec::new();
    for name in names {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
        let start = status.optr.len();
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
        let body = status.optr[start..].trim().to_string();
        assert!(
            body.contains("func ") || body.contains("{"),
            "{name}: {}",
            status.optr
        );
        code.push(body);
    }
    code
}
fn run(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
fn go_available() -> bool {
    match Command::new("go").arg("version").output() {
        Ok(output) => {
            assert!(output.status.success());
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Go toolchain absent; external syntax/execution check skipped");
            false
        }
        Err(error) => panic!("go: {error}"),
    }
}
fn parse(directory: &Path, code: &[String]) {
    let parser = directory.join("parser");
    run(Command::new("go")
        .args(["build", "-o"])
        .arg(&parser)
        .arg(fixture("parse.go")));
    let mut check = Command::new(parser);
    for (index, body) in code.iter().enumerate() {
        let file = directory.join(format!("body_{index}.go"));
        std::fs::write(&file, format!("package recovered\n{body}\n")).unwrap();
        check.arg(file);
    }
    run(&mut check);
}

#[test]
fn go_scalar_functions_compile_and_preserve_effects() {
    if !go_available() {
        return;
    }
    for extra in [
        vec![],
        vec![
            "option taildup off",
            "option crossjumprevert off",
            "option gotoreduce off",
        ],
    ] {
        let mut options = vec![
            "option outlang go",
            "map prototype call_callback long call_callback(long (*fn)(long), long x);",
        ];
        options.extend(extra);
        let code = decompile(&fixture("output-c"), FUNCTIONS, &options);
        let harness = std::fs::read_to_string(fixture("check.go")).unwrap();
        let split = harness.find("func expect").unwrap();
        let source = format!(
            "{}\n{}\n{}",
            &harness[..split],
            code.join("\n"),
            &harness[split..]
        );
        let directory = scratch();
        let file = &directory.join("recovered.go");
        std::fs::write(&file, source).unwrap();
        run(Command::new("go").arg("run").arg(file));
    }
    let code = decompile(
        &fixture("output-c"),
        &["add"],
        &[
            "option outlang go",
            "map prototype add long add(long _args, long b, ...);",
        ],
    );
    let directory = scratch();
    let file = directory.join("variadic.go");
    std::fs::write(&file, format!("package main\n{}\nfunc main() {{ if add(2,3,7)!=5 {{ panic(\"variadic parameter collision\") }} }}\n", code[0])).unwrap();
    run(Command::new("go").arg("run").arg(file));
}

#[test]
fn go_syntax_survives_brace_preferences() {
    if !go_available() {
        return;
    }
    let directory = scratch();
    let mut bodies = Vec::new();
    for style in ["same", "next", "skip"] {
        let mut options = vec![
            "option outlang go".to_string(),
            "option maxlinewidth 24".to_string(),
        ];
        for category in ["function", "ifelse", "loop", "switch"] {
            options.push(format!("option braceformat {category} {style}"));
        }
        bodies.extend(decompile(
            &fixture("output-c"),
            FUNCTIONS,
            &options.iter().map(String::as_str).collect::<Vec<_>>(),
        ));
    }
    for (binary, names) in [
        ("faillog", &["sub_2f60", "sub_3050", "sub_31a0"][..]),
        ("grep", &["sub_144a0", "sub_f9c0", "sub_15020"][..]),
    ] {
        bodies.extend(decompile(
            &root().join("tests/bug-repro").join(binary),
            names,
            &["option outlang go", "option maxlinewidth 24"],
        ));
    }
    parse(&directory, &bodies);
}

#[test]
fn native_go_uses_the_same_output_plane() {
    if !go_available() {
        return;
    }
    let directory = scratch();
    let binary = &directory.join("native");
    run(Command::new("go")
        .args(["build", "-trimpath", "-buildvcs=false", "-o"])
        .arg(&binary)
        .arg(fixture("output.go")));
    let names = [
        "main.add",
        "main.choose",
        "main.sumFor",
        "main.shortCalls",
        "main.loopCalls",
        "main.dispatch",
        "main.readAt",
        "main.writeAt",
        "main.fromWord",
        "main.toWord",
        "main.retype",
        "main.narrow",
        "main.extend",
        "main.number",
    ];
    let bodies = decompile(&binary, &names, &["option outlang go"]);
    assert!(bodies[0].contains("func add(") && bodies[0].contains("int64"));
    parse(&directory, &bodies);
    let source = format!("package main\n{}\nfunc main() {{ if add(2,3)!=5 || choose(11,2)!=3 || extend(-7)!=-7 || number(7)!=7 {{ panic(\"native Go scalars\") }} }}\n", [bodies[0].as_str(), bodies[1].as_str(), bodies[12].as_str(), bodies[13].as_str()].join("\n"));
    let file = &directory.join("recovered.go");
    std::fs::write(&file, source).unwrap();
    run(Command::new("go").arg("run").arg(file));
}

#[test]
fn go_selection_does_not_mutate_c_brace_preferences() {
    let c = decompile(
        &fixture("output-c"),
        &["choose"],
        &[
            "option braceformat function skip",
            "option braceformat ifelse skip",
        ],
    );
    let roundtrip = decompile(
        &fixture("output-c"),
        &["choose"],
        &[
            "option braceformat function skip",
            "option braceformat ifelse skip",
            "option outlang go",
            "option outlang c",
        ],
    );
    assert_eq!(c, roundtrip);
    assert!(!c[0].contains("func "));
}
