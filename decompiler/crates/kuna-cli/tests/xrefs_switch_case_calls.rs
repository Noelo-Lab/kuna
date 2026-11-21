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

/// The fix on fixed bytes: gcc's `-O0` dispatch in `switchtable_gcc_x86_64`
/// (table base in the unscaled index slot, range check ten instructions up)
/// has its case callees' caller.
#[test]
fn a_far_dispatch_reaches_its_case_callees() {
    let binary = common::fixture("switchtable_gcc_x86_64");
    let path = std::path::Path::new(&binary);
    for callee in ["t_o0_0", "t_o0_1", "t_o0_2", "t_o0_3"] {
        assert_eq!(callers(path, callee, &[]), ["o0_dispatch"], "{callee}");
    }
}

/// GH-1824: the case helpers of a switch whose base or range check a linear
/// walk back from the jump cannot reach. `loop_ifelse_switch` hoists the base
/// out of the loop above an `if/else` whose first arm ends in `jmp`,
/// `two_switches` shares one base between two switches (MSVC's `__ImageBase`;
/// Clang and gcc materialize each table on its own), `far_guard_switch` puts
/// the range check more than sixteen instructions up, and `control_switch` is
/// the shape the slice always read.
const FLOW_SOURCE: &str = r#"#include <stdio.h>
#define NOINL __attribute__((noinline))
static volatile int g_sink;
#define MK(p, id, n) \
    NOINL int p##_##n(int x, const char *s) { g_sink += x + (int)s[0] + (id) + (n); return x + (n); }
#define MK8(p, id) MK(p,id,0) MK(p,id,1) MK(p,id,2) MK(p,id,3) MK(p,id,4) MK(p,id,5) MK(p,id,6) MK(p,id,7)
#define CASES8(p, acc) \
    case 0: acc = p##_0(acc, #p " case 0"); break; \
    case 1: acc = p##_1(acc, #p " case 1"); break; \
    case 2: acc = p##_2(acc, #p " case 2"); break; \
    case 3: acc = p##_3(acc, #p " case 3"); break; \
    case 4: acc = p##_4(acc, #p " case 4"); break; \
    case 5: acc = p##_5(acc, #p " case 5"); break; \
    case 6: acc = p##_6(acc, #p " case 6"); break; \
    case 7: acc = p##_7(acc, #p " case 7"); break;
MK8(li, 0) MK8(s1, 100) MK8(s2, 200) MK8(fg, 300) MK8(ok, 400)
NOINL int branch_a(int x) { g_sink += x * 3; return x + 11; }
NOINL int branch_b(int x) { g_sink -= x * 5; return x - 7; }
NOINL int loop_ifelse_switch(const unsigned *code)
{
    int acc = 0;
    for (int i = 0; i < 64; i++) {
        unsigned op = code[i];
        acc = (op & 1) ? branch_a(acc) : branch_b(acc);
        switch (op) { CASES8(li, acc) default: acc ^= 1; break; }
    }
    return acc;
}
NOINL int two_switches(unsigned x, unsigned y)
{
    int acc = 0;
    switch (x) { CASES8(s1, acc) default: acc = -1; break; }
    switch (y) { CASES8(s2, acc) default: acc ^= 99; break; }
    return acc;
}
NOINL int far_guard_switch(unsigned op, int acc)
{
    if (op < 8) {
        g_sink += acc + 1; acc ^= g_sink; g_sink += acc + 2; acc ^= g_sink;
        g_sink += acc + 3; acc ^= g_sink; g_sink += acc + 4; acc ^= g_sink;
        g_sink += acc + 5; acc ^= g_sink; g_sink += acc + 6; acc ^= g_sink;
        switch (op) { CASES8(fg, acc) }
    }
    return acc;
}
NOINL int control_switch(unsigned op)
{
    int acc = 7;
    switch (op) { CASES8(ok, acc) default: acc = -1; break; }
    return acc;
}
int main(int argc, char **argv)
{
    unsigned code[64];
    for (int i = 0; i < 64; i++) code[i] = (unsigned)((argc + i + argv[0][0]) % 9);
    printf("%d %d %d %d\n",
        loop_ifelse_switch(code), two_switches((unsigned)argc, (unsigned)argv[0][0]),
        far_guard_switch((unsigned)argc, argc), control_switch((unsigned)argc));
    return 0;
}
"#;

/// The `coverage.notes` of `kuna xrefs --from function`.
fn notes_from(binary: &std::path::Path, function: &str) -> Vec<String> {
    let (stdout, stderr, rc) =
        common::run_kuna(&["xrefs", binary.to_str().unwrap(), "--from", function, "--json"]);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    parsed["coverage"]["notes"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|n| n.as_str().map(str::to_owned))
        .collect()
}

/// Every case helper of the four GH-1824 switches has its dispatcher as
/// caller on Clang `-O2 -fPIE -pie` and gcc `-O2`, except where the compiler
/// keeps the base in a caller-saved register across calls to functions it
/// has seen the bodies of (gcc's `-fipa-ra` on `loop_ifelse_switch`): the
/// walk, which has no model of a callee's clobbers, reports that switch as
/// an unresolved computed jump rather than saying nothing.
#[test]
fn a_hoisted_base_a_shared_base_and_a_far_guard_reach_their_case_helpers() {
    let dir = common::scratch_file("xrefs-switch-flow", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join("flow.c");
    std::fs::write(&source, FLOW_SOURCE).unwrap();
    let switches = [
        ("loop_ifelse_switch", "li"),
        ("two_switches", "s1"),
        ("two_switches", "s2"),
        ("far_guard_switch", "fg"),
        ("control_switch", "ok"),
    ];
    for (cc, opt) in [("clang", &["-O2", "-fPIE", "-pie"][..]), ("gcc", &["-O2"][..])] {
        if process::optional_output(Command::new(cc).arg("--version")).is_none() {
            eprintln!("skipping {cc}: not installed");
            continue;
        }
        let binary = dir.join(format!("flow-{cc}"));
        process::required_output(Command::new(cc).args(opt).arg(&source).arg("-o").arg(&binary));
        for (function, prefix) in switches {
            let missing: Vec<String> = (0..8)
                .map(|n| format!("{prefix}_{n}"))
                .filter(|callee| !callers(&binary, callee, &[]).iter().any(|c| c == function))
                .collect();
            if missing.is_empty() {
                continue;
            }
            let notes = notes_from(&binary, function);
            assert!(
                cc == "gcc" && function == "loop_ifelse_switch" && notes.iter().any(|n| n.contains("computed jump")),
                "{cc} {opt:?}: {missing:?} are called from case bodies of {function}; notes {notes:?}"
            );
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}
