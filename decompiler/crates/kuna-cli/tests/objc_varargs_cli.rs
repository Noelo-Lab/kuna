use crate::common;

/// `objc_msgSendSuper2` and `NSLog` are declared with their fixed parameters
/// and `...`: the `CGRect`-like doubles of `[super initWithFrame:]` follow the
/// `objc_super` record and the selector, and the `%@` values `NSLog` takes on
/// the stack under Apple's AArch64 variadic convention are its arguments.
#[test]
fn runtime_varargs_follow_the_fixed_arguments() {
    let binary = common::fixture("macho_objc_varargs_arm64");
    let decompile = |addr: &str| {
        let args = ["decompile", binary.as_str(), addr, "--addr", "--option", "objc", "on", "--option", "cfstrings", "on"];
        let (out, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}");
        out
    };
    let init = decompile("0x1000007b8");
    assert!(init.contains("objc_msgSendSuper2(&v1,sel_initWithFrame_,8.0,16.0,arg1,arg2)"), "{init}");
    let report = decompile("0x1000007fc");
    assert!(report.contains("NSLog(CFSTR(\"Error %@ parsing token payload %@\"),a0,a1)"), "{report}");
}
