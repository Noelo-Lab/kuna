//! A MIPS import with no lazy stub keeps its GOT slot variable: the dynamic
//! linker writes its address there, so the 0 the image stores is not a value.
use crate::common;

#[test]
fn stubless_import_slots_are_not_null() {
    for (image, extra) in [
        ("mipsgot_nostub_el.so", &[][..]),
        ("mipsgot_nostub_n64el.so", &["--target", "MIPS:LE:64:default"][..]),
    ] {
        let path = common::fixture(image);
        let mut args = vec!["decompile", path.as_str(), "user"];
        args.extend_from_slice(extra);
        let (code, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{image}: {err}");
        assert!(!code.contains("NULL"), "{image}: {code}");
        assert!(code.contains("if ("), "{image} lost the weak guard: {code}");
    }
}
