use crate::common;

/// A go1.16 amd64 image passes arguments on the stack: its calls keep them.
#[test]
fn a_go_image_older_than_the_register_abi_keeps_its_call_arguments() {
    let binary = common::fixture("go116_stackabi_x86_64");
    let (out, stderr, rc) = common::run_kuna(&["decompile", binary.as_str(), "main.main"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(out.contains("main.add3(dat_4c8048,2,7);"), "{out}");
    assert!(out.contains("runtime.printint(v1);"), "{out}");
    assert!(!out.contains("g->"), "{out}");
}
