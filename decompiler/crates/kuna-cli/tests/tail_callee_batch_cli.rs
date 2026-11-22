//! A narrowed `decompile-all` and `kuna decompile --json` state the prototypes
//! of the callees their functions tail-call, as `kuna decompile` does (option
//! `protoorder`). Neither has a callee-first order behind it, so the AArch64
//! forwarder `ldr x8,[x3]; cbz x8,L; b target` printed `target(); // tail-call`.
//!
//! Fixture: `tailcallee_fwd_aarch64` (`forward`@`0x21015c`, `target`@`0x21012c`).
//! Integration tests require the built processor specs under `specs/`.

use crate::common;

const FORWARD: &str = "0x21015c";

fn run(args: &[&str]) -> String {
    let bin = common::fixture("tailcallee_fwd_aarch64");
    let mut argv = vec![args[0], bin.as_str()];
    argv.extend_from_slice(&args[1..]);
    let (stdout, stderr, rc) = common::run_kuna(&argv);
    assert_eq!(rc, 0, "{stderr}\n{stdout}");
    stdout
}

#[test]
fn narrowed_decompile_all_forwards_the_tail_callees_arguments() {
    let out = run(&["decompile-all", "--addr", FORWARD]);
    assert!(out.contains("target(a0,a1,a2,a3,a4); // tail-call"), "{out}");
    let off = run(&["decompile-all", "--addr", FORWARD, "--option", "protoorder", "off"]);
    assert!(off.contains("target(); // tail-call"), "{off}");
}

#[test]
fn decompile_json_forwards_the_tail_callees_arguments() {
    let out = run(&["decompile", FORWARD, "--addr", "--json"]);
    assert!(out.contains("target(a0,a1,a2,a3,a4); // tail-call"), "{out}");
}
