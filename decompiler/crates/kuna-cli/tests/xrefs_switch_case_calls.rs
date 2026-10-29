//! A function called only from a jump-table case body has its caller in
//! `kuna xrefs`, at every optimization level gcc and Clang lower the switch
//! through a table.

#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use std::process::Command;

const SOURCE: &str = r#"#include <stdio.h>
__attribute__((noinline)) void fa(const char *s) { puts(s); }
__attribute__((noinline)) void fb(const char *s) { fputs(s, stderr); }
__attribute__((noinline)) void fc(const char *s) { printf("%s!\n", s); }
__attribute__((noinline)) void fd(const char *s) { printf("[%s]\n", s); }
__attribute__((noinline)) void fe(const char *s) { printf("<%s>\n", s); }
void dispatch(int c, const char *s) {
    switch (c) {
    case 'a': fa(s); break;
    case 'b': fb(s); break;
    case 'c': fc(s); break;
    case 'd': fd(s); break;
    case 'e': fe(s); break;
    case 'f': fa(s); fb(s); break;
    default: break;
    }
}
int main(int argc, char **argv) { dispatch(argv[0][0], argv[0]); return 0; }
"#;

/// The callers `kuna xrefs` lists for `callee`, by function name.
fn callers(binary: &std::path::Path, callee: &str, extra: &[&str]) -> Vec<String> {
    let mut args = vec!["xrefs", binary.to_str().unwrap(), "--to", callee, "--json"];
    args.extend_from_slice(extra);
    let (stdout, stderr, rc) = common::run_kuna(&args);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    parsed["xrefs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| matches!(r["kind"].as_str(), Some("call" | "jump")))
        .filter_map(|r| r["from_function"]["name"].as_str().map(str::to_owned))
        .collect()
}

#[test]
fn every_case_body_callee_has_the_dispatcher_as_caller() {
    let dir = common::scratch_file("xrefs-switch-case", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("sw.c");
    std::fs::write(&source, SOURCE).unwrap();
    for cc in ["gcc", "clang"] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            eprintln!("skipping {cc}: not installed");
            continue;
        }
        for opt in [&["-O0"][..], &["-O2", "-fno-inline"][..]] {
            let binary = dir.join(format!("sw-{cc}{}", opt[0]));
            process::required_output(Command::new(cc).args(opt).arg(&source).arg("-o").arg(&binary));
            for callee in ["fa", "fb", "fc", "fd", "fe"] {
                let found = callers(&binary, callee, &[]);
                assert!(
                    found.iter().any(|c| c == "dispatch"),
                    "{cc} {opt:?}: {callee} is called from a case body of dispatch; callers {found:?}"
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The defect and the fix on fixed bytes: gcc's `-O0` dispatch in
/// `switchtable_gcc_x86_64` (table base in the unscaled index slot, range check
/// ten instructions up) has its case callees' caller only with `switchreach` on.
#[test]
fn switchreach_off_keeps_the_eight_instruction_reach() {
    let binary = common::fixture("switchtable_gcc_x86_64");
    let path = std::path::Path::new(&binary);
    for callee in ["t_o0_0", "t_o0_1", "t_o0_2", "t_o0_3"] {
        assert_eq!(callers(path, callee, &[]), ["o0_dispatch"], "{callee} with switchreach on");
        let off = callers(path, callee, &["--option", "switchreach", "off"]);
        assert!(off.is_empty(), "{callee} with switchreach off has no caller; got {off:?}");
    }
}
