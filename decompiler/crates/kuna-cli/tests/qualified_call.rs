//! Incomplete parked declarations must not erase an indirect call's result.
use crate::common;

#[test]
fn partial_unit_declarations_preserve_the_indirect_call_result() {
    let path = common::repo_root().join("tests/fixtures/qualified-call/partial-unit.elf");
    for options in [&[][..], &["--option", "protoorder", "off"][..]] {
        let mut args = vec!["decompile", path.to_str().unwrap(), "mode"];
        args.extend_from_slice(options);
        let (code, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{err}");
        assert!(code.lines().any(|line| line.contains("if (")
            && line.contains("initialize(") && line.contains("callback")), "{code}");
        assert!(code.contains(" = 1;"), "{code}");
    }
}
