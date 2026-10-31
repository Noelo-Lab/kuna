//! (kuna `pltlocal`) A shared library's call through its own export's PLT stub
//! is built at the definition the stub's relocation binds it to (GH-1342).

use crate::common;

fn decompile_all(lib: &str, extra: &[&str]) -> String {
    let lib = common::fixture(lib);
    let mut args = vec!["decompile-all", lib.as_str()];
    args.extend_from_slice(extra);
    let (out, err, code) = common::run_kuna(&args);
    assert_eq!(code, 0, "{err}");
    out
}

/// The text of the function `name` decompiled at `addr`.
fn function<'a>(out: &'a str, name: &str, addr: &str) -> &'a str {
    let header = format!("// Function: {name} @ {addr}\n");
    let start = out.find(&header).unwrap_or_else(|| panic!("no {header:?} in\n{out}")) + header.len();
    let end = out[start..].find("// Function: ").map_or(out.len(), |e| start + e);
    &out[start..end]
}

#[test]
fn a_forwarded_register_reaches_the_definition_through_its_stub() {
    let out = decompile_all("pltlocal_aarch64.so", &[]);
    let wrapper = function(&out, "timer_wrapper", "0x10308");
    assert!(wrapper.contains("  timer_helper(a0,a1,a2,a3);\n"), "{wrapper}");
    assert!(wrapper.contains("  send_helper(a0,a1); // tail-call\n"), "{wrapper}");
    assert!(wrapper.starts_with("void timer_wrapper(void *a0,void *a1,unsigned int a2,unsigned int a3)\n"), "{wrapper}");
    let stub = function(&out, "timer_helper", "0x10380");
    assert!(stub.contains("(*dat_30468)(); // jump-as-call"), "{stub}");
}

#[test]
fn off_builds_the_call_at_the_stub() {
    let out = decompile_all("pltlocal_aarch64.so", &["--option", "pltlocal", "off"]);
    let wrapper = function(&out, "timer_wrapper", "0x10308");
    assert!(wrapper.contains("  timer_helper();\n"), "{wrapper}");
    assert!(wrapper.starts_with("void timer_wrapper(unsigned long a0,void *a1)\n"), "{wrapper}");
}

#[test]
fn a_selection_of_the_caller_and_the_definition_is_enough() {
    let out = decompile_all(
        "pltlocal_aarch64.so",
        &["--functions", "timer_wrapper,timer_helper", "--option", "protoorder", "cycles"],
    );
    assert!(function(&out, "timer_wrapper", "0x10308").contains("  timer_helper(a0,a1,a2,a3);\n"), "{out}");
    let reached = decompile_all("pltlocal_aarch64.so", &["--reachable-from", "timer_wrapper"]);
    assert!(reached.contains("// Function: timer_helper @ 0x10344\n"), "{reached}");
    assert!(function(&reached, "timer_wrapper", "0x10308").contains("  timer_helper(a0,a1,a2,a3);\n"), "{reached}");
}

#[test]
fn an_x86_64_gcc_library_passes_its_own_export_the_argument() {
    let out = decompile_all("selfplt_x86_64.so", &[]);
    let reader = function(&out, "bump_and_read", "0x1150");
    assert!(reader.starts_with("unsigned int bump_and_read(int a0)\n"), "{reader}");
    assert!(reader.contains("  bump(a0);\n"), "{reader}");
    let off = decompile_all("selfplt_x86_64.so", &["--option", "pltlocal", "off"]);
    assert!(function(&off, "bump_and_read", "0x1150").contains("  bump();\n"), "{off}");
}
