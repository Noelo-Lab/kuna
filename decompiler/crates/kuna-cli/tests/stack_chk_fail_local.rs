use crate::common;
use common::process;
use std::process::Command;

fn decompile(path: &str, function: &str, mode: &[&str]) -> String {
    let (out, err, rc) = common::run_kuna(&[&["decompile", path, function], mode].concat());
    assert_eq!(rc, 0, "{err}\n{out}");
    out
}

fn decompile_renamed(fixture: &str, original: &str, name: &str, function: &str, mode: &[&str]) -> String {
    decompile_renamed_image(&common::fixture(fixture), original, name, function, mode)
}

fn decompile_renamed_image(image: &str, original: &str, name: &str, function: &str, mode: &[&str]) -> String {
    let original = format!("{original}\0");
    let mut renamed = format!("{name}\0").into_bytes();
    renamed.resize(original.len(), 0);
    let mut bytes = std::fs::read(image).unwrap();
    let at = bytes.windows(original.len()).position(|w| w == original.as_bytes()).expect("symbol name");
    bytes[at..at + original.len()].copy_from_slice(&renamed);
    let path = common::scratch_file("stack-chk-fail-local", "o");
    std::fs::write(&path, bytes).unwrap();
    let out = decompile(path.to_str().unwrap(), function, mode);
    std::fs::remove_file(path).unwrap();
    out
}

fn installed(names: &[&'static str]) -> Option<&'static str> {
    names.iter().copied().find(|name| process::optional_output(Command::new(name).arg("--version")).is_some())
}

/// The committed linked image, then one rebuilt from its source when `llvm-mc` and `ld.lld` are installed.
fn linked_images(endian: &str) -> Vec<String> {
    let mut images = vec![common::fixture(&format!("ppc32_stack_chk_local_linked_{endian}.so"))];
    let (Some(mc), Some(lld)) = (installed(&["llvm-mc", "llvm-mc-21"]), installed(&["ld.lld", "ld.lld-21"])) else {
        eprintln!("llvm-mc or ld.lld is unavailable; skipping the rebuild of ppc32_stack_chk_local_linked.s");
        return images;
    };
    let triple = if endian == "be" { "powerpc" } else { "powerpcle" };
    let object = common::scratch_file("stack-chk-fail-local-linked", "o");
    let image = common::scratch_file("stack-chk-fail-local-linked", "so");
    process::required_output(
        Command::new(mc)
            .args([&format!("-triple={triple}-unknown-linux-gnu"), "-filetype=obj"])
            .arg(common::fixture("ppc32_stack_chk_local_linked.s"))
            .arg("-o")
            .arg(&object),
    );
    process::required_output(Command::new(lld).args(["-shared", "-z", "max-page-size=0x1000"]).arg(&object).arg("-o").arg(&image));
    std::fs::remove_file(object).unwrap();
    images.push(image.to_str().unwrap().to_owned());
    images
}

#[test]
fn an_imported_stack_chk_fail_local_takes_no_arguments_unlike_an_undeclared_name() {
    for (endian, target) in [("be", "BE"), ("le", "LE")] {
        let fixture = format!("ppc32_stack_chk_local_{endian}.o");
        let e500 = format!("PowerPC:{target}:32:e500");
        let pinned = ["--mode", "fast", "--option", "tailform", "off"];
        let e500_args = ["--mode", "fast", "--option", "tailform", "off", "--target", &e500];
        for args in [&pinned[..], &e500_args[..]] {
            let out = decompile(&common::fixture(&fixture), "r6", args);
            assert!(out.contains("  __stack_chk_fail_local(); // no-return\n}"), "{endian} {args:?}\n{out}");
            assert!(out.contains("return a0 * 3;"), "{endian} {args:?}\n{out}");
            assert!(!out.contains("return 0;"), "{endian} {args:?}\n{out}");
        }
        let flowing = decompile(&common::fixture(&fixture), "r6", &["--mode", "fast", "--option", "noreturn_known", "off"]);
        assert!(flowing.contains("  __stack_chk_fail_local();\n  return 0;\n"), "{endian}\n{flowing}");
        let renamed = |name| decompile_renamed(&fixture, "__stack_chk_fail_local", name, "r6", &["--mode", "fast"]);
        let known = renamed("__stack_chk_fail\0local");
        assert!(known.contains("__stack_chk_fail(); // no-return"), "{endian}\n{known}");
        for name in ["__stack_chk_fail_lokal", "_stack_chk_fail_local", "stack_chk_fail_local"] {
            let undeclared = renamed(name);
            assert!(undeclared.contains(&format!("  {name}(a0 * 3);\n")), "{endian} {name}\n{undeclared}");
            assert!(undeclared.contains("return 0;"), "{endian} {name}\n{undeclared}");
        }
    }
}

#[test]
fn a_linked_stack_chk_fail_local_takes_no_arguments_under_libgccsigs() {
    for (endian, order) in [("be", "BE"), ("le", "LE")] {
        let images = linked_images(endian);
        for image in &images {
            for variant in ["default", "e500"] {
                let target = format!("PowerPC:{order}:32:{variant}");
                let default = ["--mode", "fast", "--target", &target];
                let on = [&default[..], &["--option", "libgccsigs", "on"]].concat();
                let out = decompile(image, "r6", &default);
                assert!(out.contains("  __stack_chk_fail_local(a0 * 3);\n  return 0;\n"), "{image} {target}\n{out}");
                let out = decompile(image, "r6", &on);
                assert!(out.contains("  __stack_chk_fail_local();\n  return 0;\n"), "{image} {target}\n{out}");
                assert!(out.contains("return a0 * 3;"), "{image} {target}\n{out}");
                for name in ["_stack_chk_fail_local", "stack_chk_fail_local"] {
                    let renamed = decompile_renamed_image(image, "__stack_chk_fail_local", name, "r6", &on);
                    assert!(renamed.contains(&format!("  {name}(a0 * 3);\n  return 0;\n")), "{image} {target} {name}\n{renamed}");
                }
            }
        }
        for rebuilt in &images[1..] {
            std::fs::remove_file(rebuilt).unwrap();
        }
    }
}

#[test]
fn a_returning_callee_spelled_like_stack_chk_fail_local_keeps_its_return() {
    for arch in ["x86_64", "aarch64"] {
        let fixture = format!("prefixed_noreturn_{arch}");
        for name in ["__stack_chk_fail_local", "_stack_chk_fail_local", "stack_chk_fail_local"] {
            for mode in [&[][..], &["--mode", "fast"][..]] {
                let out = decompile_renamed(&fixture, "mystack_chk_fail_count", name, "keeps_count", mode);
                assert!(out.contains(&format!("  {name}();\n  return 9;\n")), "{arch} {name} {mode:?}\n{out}");
                assert!(!out.contains("no-return"), "{arch} {name} {mode:?}\n{out}");
            }
        }
    }
}

#[test]
fn a_returning_stack_chk_fail_local_keeps_its_return_under_libgccsigs() {
    for arch in ["x86_64", "aarch64"] {
        let fixture = format!("prefixed_noreturn_{arch}");
        let on = ["--option", "libgccsigs", "on"];
        let out = decompile_renamed(&fixture, "mystack_chk_fail_count", "__stack_chk_fail_local", "keeps_count", &on);
        assert!(out.contains("  __stack_chk_fail_local();\n  return 9;\n"), "{arch}\n{out}");
        assert!(!out.contains("no-return"), "{arch}\n{out}");
    }
}

#[test]
fn a_defined_stack_chk_fail_local_keeps_its_aliases_untyped() {
    for args in [&["--mode", "fast"][..], &["--mode", "fast", "--option", "libgccsigs", "on"][..]] {
        let out = decompile(&common::fixture("mcount_x86_64"), "__gconv_transform_ascii_internal", args);
        assert!(out.starts_with("unsigned long __gconv_transform_ascii_internal("), "{args:?}\n{out}");
        assert!(!out.contains("undefined16"), "{args:?}\n{out}");
    }
}
