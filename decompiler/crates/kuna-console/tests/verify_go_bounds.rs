//! Go compiler checks fold without hiding explicit panics or preceding stores.
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use std::process::{Command, Stdio};

const SOURCE: &str = r#"package main
var sink int
//go:noinline
func at(s []int, i int) int { return s[i] }
//go:noinline
func third(s []int) int { return s[2] }
//go:noinline
func divide(a, b int) int { return a / b }
//go:noinline
func explicit(x int) int { if x < 0 { panic("user panic") }; return x }
//go:noinline
func before(s []int, i int) int { sink = i; return s[i] }
func main() { s := []int{1,2,3}; println(at(s,1),third(s),divide(6,2),explicit(1),before(s,0)) }
"#;

fn decompile(binary: &std::path::Path, name: &str, enabled: bool) -> String {
    let specs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[specs.to_str().unwrap().into()],
    )
    .unwrap();
    program
        .arch_mut()
        .set_kuna_option("gobounds", if enabled { "on" } else { "off" })
        .unwrap();
    program.commit_pending_analysis().unwrap();
    let commands = vec![
        format!("load function main.{name}"),
        "decompile".into(),
        "print C".into(),
    ];
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..3 {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn go_bounds_and_division_checks_with_explicit_panic_control() {
    if !Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
    {
        eprintln!("skipping Go fixture: compiler unavailable");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna-go-bounds-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.go"), SOURCE).unwrap();
    for arch in ["amd64", "arm64"] {
        let binary = dir.join(arch);
        let build = Command::new("go")
            .args(["build", "-ldflags=-s -w", "-o"])
            .arg(&binary)
            .arg("main.go")
            .current_dir(&dir)
            .env("GOOS", "linux")
            .env("GOARCH", arch)
            .env("CGO_ENABLED", "0")
            .output()
            .unwrap();
        assert!(
            build.status.success(),
            "{}",
            String::from_utf8_lossy(&build.stderr)
        );
        for function in ["at", "third", "divide", "before"] {
            let off = decompile(&binary, function, false);
            let on = decompile(&binary, function, true);
            assert!(
                off.contains("runtime.panic"),
                "{arch} {function} off: {off}"
            );
            assert!(!on.contains("runtime.panic"), "{arch} {function} on: {on}");
            assert!(on.contains("return "), "{arch} {function}: {on}");
            if function == "before" {
                assert!(
                    on.lines()
                        .any(|line| line.contains("dat_") && line.contains(" = ")),
                    "global store missing: {on}"
                );
            }
        }
        let explicit = decompile(&binary, "explicit", true);
        assert!(explicit.contains("runtime.gopanic"), "{arch}: {explicit}");
        assert!(explicit.contains("if ("), "{arch}: {explicit}");
    }
    std::fs::remove_dir_all(dir).unwrap();
}
