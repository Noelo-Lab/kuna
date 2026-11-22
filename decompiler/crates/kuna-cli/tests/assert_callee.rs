use std::path::PathBuf;
use std::process::{Command, Output};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/assert_callee_x86_64.elf")
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg(args[0])
        .arg(fixture())
        .args(&args[1..])
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_stated_indirect_call_is_a_direct_call() {
    for callee in ["handle_open", "0x1430"] {
        let directive = format!("callee 0x147a {callee}");
        for extra in [&[][..], &["--json"][..]] {
            let mut args = vec!["decompile", "twice", "--assert", &directive];
            args.extend_from_slice(extra);
            let out = kuna(&args);
            let stdout = text(&out.stdout);
            assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
            assert!(stdout.contains("v1 = handle_open(a0,a2);"), "{stdout}");
            assert!(!stdout.contains("table"), "{stdout}");
        }
    }
}

#[test]
fn a_stated_indirect_jump_is_a_tail_call() {
    let plain = text(&kuna(&["decompile", "dispatch"]).stdout);
    assert!(!plain.contains("handle_open"), "{plain}");
    let out = kuna(&["decompile", "dispatch", "--assert", "callee dispatch::0x1457 handle_open"]);
    let stdout = text(&out.stdout);
    assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
    assert!(stdout.contains("  return handle_open(a0,a2);"), "{stdout}");
    assert!(!stdout.contains("switch"), "{stdout}");
    let off = kuna(&[
        "decompile",
        "dispatch",
        "--assert",
        "callee dispatch::0x1457 handle_open",
        "--option",
        "protoorder",
        "off",
    ]);
    let off = text(&off.stdout);
    assert!(off.contains("  handle_open(a0,a2);"), "{off}");
    assert!(!off.contains("return handle_open"), "{off}");
}

#[test]
fn every_function_of_a_whole_binary_run_takes_its_own_statement() {
    let out = kuna(&[
        "decompile-all",
        "--assert",
        "callee dispatch::0x1457 handle_open",
        "--assert",
        "callee twice::0x147a handle_open",
    ]);
    let stdout = text(&out.stdout);
    assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
    assert!(stdout.contains("return handle_open(a0,a2);"), "{stdout}");
    assert!(stdout.contains("v1 = handle_open(a0,a2);"), "{stdout}");
}

#[test]
fn a_statement_the_image_contradicts_is_rejected() {
    for (directive, reason) in [
        ("callee 0x1486 handle_open", "the instruction at 0x1486 is a direct call"),
        ("callee 0x1478 handle_open", "the instruction at 0x1478 is not a call"),
        ("callee 0x147a 0x1434", "0x1434 is not a function entry"),
        ("callee 0x147a no_such_function", "\"no_such_function\" does not name a function"),
    ] {
        for extra in [&[][..], &["--json"][..]] {
            let mut args = vec!["decompile", "twice", "--assert", directive];
            args.extend_from_slice(extra);
            let out = kuna(&args);
            let all = format!("{}{}", text(&out.stdout), text(&out.stderr));
            assert!(all.contains("rejected"), "{directive}: {all}");
            assert!(all.contains(&reason.replace('"', "\\\"")) || all.contains(reason), "{directive}: {all}");
            assert!(all.contains("table"), "{directive}: {all}");
        }
    }
}

#[test]
fn a_call_site_outside_the_function_is_refused_after_flow() {
    for extra in [&[][..], &["--json"][..]] {
        let mut args = vec!["decompile", "twice", "--assert", "callee 0x1457 handle_open"];
        args.extend_from_slice(extra);
        let out = kuna(&args);
        let all = format!("{}{}", text(&out.stdout), text(&out.stderr));
        assert!(!out.status.success(), "{all}");
        assert!(all.contains("makes no indirect call at this address"), "{all}");
    }
}

#[test]
fn a_malformed_statement_is_a_usage_error() {
    for directive in ["callee 0x147a", "callee here handle_open", "callee 0x147a a b"] {
        let out = kuna(&["decompile", "twice", "--assert", directive]);
        assert_eq!(out.status.code(), Some(2), "{directive}: {}", text(&out.stderr));
        assert!(text(&out.stderr).contains("callee"), "{directive}");
    }
}

#[test]
fn a_plt_stub_destination_is_applied() {
    for extra in [&[][..], &["--json"][..]] {
        let mut args = vec!["decompile", "twice", "--assert", "callee 0x147a 0x14c0"];
        args.extend_from_slice(extra);
        let out = kuna(&args);
        let stdout = text(&out.stdout);
        assert!(out.status.success(), "{stdout}\n{}", text(&out.stderr));
        assert!(stdout.contains("v1 = handle_open(a0,a2);"), "{stdout}");
    }
}

#[test]
fn a_restated_call_site_keeps_the_later_destination() {
    for extra in [&[][..], &["--json"][..]] {
        let mut args = vec![
            "decompile",
            "twice",
            "--assert",
            "callee 0x147a dispatch",
            "--assert",
            "callee 0x147a handle_open",
        ];
        args.extend_from_slice(extra);
        let out = kuna(&args);
        let all = format!("{}{}", text(&out.stdout), text(&out.stderr));
        assert!(out.status.success(), "{all}");
        assert!(all.contains("v1 = handle_open(a0,a2);"), "{all}");
        assert!(all.contains("a later callee states this call site again"), "{all}");
    }
}
