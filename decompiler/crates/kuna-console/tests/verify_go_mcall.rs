//! Go scheduler resumption preserves the caller's fall-through after mcall.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

use crate::common;

const SOURCE: &str = r#"package main

import "runtime"

var Sink uint32

//go:noinline
func Yield(x uint32) uint32 {
	runtime.Gosched()
	return 3*x + 7
}

func main() {
	Sink = Yield(Sink)
}
"#;

fn build_fixture(source: &Path, output: &Path, stripped: bool) {
    let mut command = Command::new("go");
    command.args(["build", "-trimpath"]);
    if stripped {
        command.arg("-ldflags=-s -w");
    }
    let result = command
        .arg("-o")
        .arg(output)
        .arg(source)
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("CGO_ENABLED", "0")
        .env("GO111MODULE", "off")
        .output()
        .expect("start Go fixture build");
    assert!(
        result.status.success(),
        "Go fixture build failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn verify_scheduler_edges(image: &Path, specs: &[String]) {
    let mut prog = bootstrap_from_object(image.to_str().unwrap(), "", specs)
        .expect("bootstrap Go scheduler fixture");
    for option in [
        "listing",
        "noreturn_known",
        "noreturn_propagate",
        "noreturn_reach",
    ] {
        prog.arch_mut().set_kuna_option(option, "on").unwrap();
    }
    prog.commit_pending_analysis().expect("commit Go analysis");
    let is_no_return = |name| {
        let entry = prog.lookup_symbol(name).expect("recover runtime entry");
        prog.arch()
            .symboltab
            .function_is_no_return_across_scopes(&entry)
    };
    assert!(!is_no_return("runtime.mcall"));
    assert!(is_no_return("runtime.goexit0"));

    let commands = ["load function runtime.Gosched", "decompile", "print raw"];
    let mut status = ConsoleCommands::into_status(commands.map(str::to_owned).into());
    register_decomp_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let data = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    data.conf = Some(prog);
    for _ in commands {
        execute(&mut status);
    }
    let text = status.optr;
    let after_call = text
        .split_once("call fruntime.mcall")
        .expect("mcall in P-code")
        .1;
    assert!(
        after_call.contains("return(#0x0)"),
        "caller epilogue missing:\n{text}"
    );
}

#[test]
fn go_scheduler_resumes_while_exit_callbacks_remain_terminal() {
    if !Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
    {
        return;
    }
    let source = common::scratch_file("go-mcall", "go");
    std::fs::write(&source, SOURCE).unwrap();
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs")
        .canonicalize()
        .unwrap();
    let specs = [specs.to_str().unwrap().to_owned()];
    for stripped in [false, true] {
        let image = common::scratch_file("go-mcall", "elf");
        build_fixture(&source, &image, stripped);
        verify_scheduler_edges(&image, &specs);
        std::fs::remove_file(image).unwrap();
    }
    std::fs::remove_file(source).unwrap();
}
