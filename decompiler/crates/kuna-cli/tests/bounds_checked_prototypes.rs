use crate::common;
use common::process;
use std::process::Command;

const AARCH64_CALLS: &[(&str, &str)] = &[
    ("clear_obj", "memset_s(v1,0x40,0,0x40);"),
    ("dup_obj", "memcpy_s(v1,0x20,a0,0x20);"),
    ("shift_obj", "memmove_s(v1,0x20,a0,0x20);"),
    ("dup_name", "strncpy_s(v1,0x10,a0,0x10);"),
    ("add_name", "strncat_s(a0,0x10,a1,0x10);"),
    ("join", "strcpy_s(a0,0x10,a1)"),
    ("join", "strcat_s(a0,0x10,a2);"),
    ("format_pair", "sprintf_s(a0,0x20,\"%d-%d\",a1,a2);"),
];

const THUMB_CALLS: &[(&str, &str)] = &[
    ("clear_obj", "memset_s(v1,0x40,0,0x40);"),
    ("dup_obj", "memcpy_s(v1,0x20,a0,0x20);"),
];

fn decompile(image: &str, function: &str) -> String {
    let (out, err, rc) = common::run_kuna(&["decompile", image, function]);
    assert_eq!(rc, 0, "{image} {function}\n{err}\n{out}");
    out
}

fn assert_calls(image: &str, calls: &[(&str, &str)]) {
    for (function, call) in calls {
        let out = decompile(image, function);
        assert!(out.contains(call), "{image} {function}: want {call}\n{out}");
    }
}

fn installed(names: &[&'static str]) -> Option<&'static str> {
    names.iter().copied().find(|name| process::optional_output(Command::new(name).arg("--version")).is_some())
}

/// Assemble and link a fixture source the way its committed image was built.
fn rebuild(source: &str, triple: &str, big_endian: bool) -> Option<String> {
    let (Some(mc), Some(lld)) = (installed(&["llvm-mc", "llvm-mc-21"]), installed(&["ld.lld", "ld.lld-21"])) else {
        eprintln!("llvm-mc or ld.lld is unavailable; skipping the rebuild of {source}");
        return None;
    };
    let object = common::scratch_file("bounds-checked", "o");
    let image = common::scratch_file("bounds-checked", "so");
    process::required_output(
        Command::new(mc).arg(format!("-triple={triple}")).arg("-filetype=obj").arg(common::fixture(source)).arg("-o").arg(&object),
    );
    let mut link = Command::new(lld);
    link.args(["-shared", "-z", "max-page-size=0x1000"]);
    if big_endian {
        link.arg("-EB");
    }
    process::required_output(link.arg(&object).arg("-o").arg(&image));
    std::fs::remove_file(object).unwrap();
    Some(image.to_str().unwrap().to_owned())
}

#[test]
fn aarch64_bounds_checked_calls_keep_every_declared_argument() {
    for (endian, triple, big) in [("le", "aarch64", false), ("be", "aarch64_be", true)] {
        assert_calls(&common::fixture(&format!("boundschecked_aarch64_{endian}.so")), AARCH64_CALLS);
        if let Some(image) = rebuild("boundschecked_aarch64.s", triple, big) {
            assert_calls(&image, AARCH64_CALLS);
            std::fs::remove_file(image).unwrap();
        }
    }
}

#[test]
fn thumb_bounds_checked_calls_keep_exactly_the_declared_arguments() {
    assert_calls(&common::fixture("boundschecked_thumb.so"), THUMB_CALLS);
    if let Some(image) = rebuild("boundschecked_thumb.s", "thumbv7-linux-gnueabihf", false) {
        assert_calls(&image, THUMB_CALLS);
        std::fs::remove_file(image).unwrap();
    }
}

#[test]
fn an_undeclared_spelling_keeps_its_inferred_arguments() {
    let original = std::fs::read(common::fixture("boundschecked_aarch64_le.so")).unwrap();
    for (name, renamed) in [("memset_s", "memset_x"), ("memset_s", "Memset_s")] {
        let mut bytes = original.clone();
        let needle = format!("{name}\0").into_bytes();
        let mut hits = 0;
        for at in (0..bytes.len() - needle.len()).rev() {
            if bytes[at..].starts_with(&needle) && (at == 0 || bytes[at - 1] == 0) {
                bytes[at..at + name.len()].copy_from_slice(renamed.as_bytes());
                hits += 1;
            }
        }
        assert!(hits > 0, "{name} is a symbol name in the fixture");
        let image = common::scratch_file("bounds-checked-renamed", "so");
        std::fs::write(&image, bytes).unwrap();
        let out = decompile(image.to_str().unwrap(), "clear_obj");
        assert!(out.contains(&format!("  {renamed}(v1,0x40,0);\n")), "{renamed}\n{out}");
        std::fs::remove_file(image).unwrap();
    }
}

/// How many arguments each printed call to `callee` passes.
fn arities(out: &str, callee: &str) -> Vec<usize> {
    let open = format!("{callee}(");
    out.match_indices(&open)
        .map(|(at, _)| {
            let args = &out[at + open.len()..];
            let args = &args[..args.find(')').unwrap_or(args.len())];
            if args.trim().is_empty() { 0 } else { args.split(',').count() }
        })
        .collect()
}

/// A library that defines its own `memset_s` and `strcpy_s`, each with a
/// parameter list of its own, calls them through their PLT stubs: no row is
/// seeded there, so neither call is handed arguments its definition never
/// takes (`memset_s(v1,0x40,a2,a3)`, `strcpy_s(a0,a1,a2)`).
#[test]
fn a_bounds_checked_name_the_image_defines_takes_no_declared_arguments() {
    let rebuilt = rebuild("boundschecked_own_aarch64.s", "aarch64", false);
    for image in std::iter::once(common::fixture("boundschecked_own_aarch64.so")).chain(rebuilt.clone()) {
        let out = decompile(&image, "clear_obj");
        assert!(out.contains("memset_s(v1,0x40);"), "{image}\n{out}");
        let out = decompile(&image, "copy_name");
        let calls = arities(&out, "strcpy_s");
        assert!(!calls.is_empty() && calls.iter().all(|&n| n <= 2), "{image}\n{out}");
    }
    if let Some(image) = rebuilt {
        std::fs::remove_file(image).unwrap();
    }
}
