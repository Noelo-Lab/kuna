//! `kuna flirt make` through the built CLI: patterns and a signature file from
//! a static library, then the names they give a stripped program linked
//! against it (fixture `tests/fixtures/flirt/make`).

use std::process::Command;

use crate::common;

#[test]
fn a_library_signature_names_the_functions_of_a_stripped_program() {
    let dir = common::repo_root().join("tests/fixtures/flirt/make");
    let scratch = tempfile::tempdir().unwrap();
    let (pat, sig) = (scratch.path().join("three.pat"), scratch.path().join("three.sig"));
    let made = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["flirt", "make"])
        .arg(dir.join("libthree.a"))
        .arg("-o")
        .arg(&pat)
        .arg("--sig")
        .arg(&sig)
        .output()
        .expect("run kuna flirt make");
    let stderr = String::from_utf8_lossy(&made.stderr);
    assert_eq!(made.status.code(), Some(0), "{stderr}");
    assert_eq!(std::fs::read(&pat).unwrap(), std::fs::read(dir.join("three.pat")).unwrap());
    assert!(stderr.contains("collision: twin_one, twin_two are not told apart"), "{stderr}");
    assert!(stderr.contains("6 module(s), 1 unresolved collision(s)"), "{stderr}");

    let parsed = flirt_sig::Signature::parse(&std::fs::read(&sig).unwrap()).expect("the reader accepts the file");
    assert_eq!(parsed.header.library_name, "libthree");
    let mut referenced: Vec<(String, String)> = parsed
        .modules
        .iter()
        .filter_map(|m| Some((m.publics[0].name.clone(), m.reference.as_ref()?.name.clone())))
        .map(|(a, b)| (String::from_utf8(a).unwrap(), String::from_utf8(b).unwrap()))
        .collect();
    referenced.sort();
    assert_eq!(referenced, [("wrap_mix".into(), "mix_fold".into()), ("wrap_sum".into(), "sum_bytes".into())]);

    let functions = |flirt: &str| {
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("functions")
            .arg(dir.join("prog"))
            .args(["--option", "flirt", flirt])
            .env("KUNA_FLIRT_SIGS", &sig)
            .env("SLEIGHHOME", common::repo_root().join("specs"))
            .output()
            .expect("run kuna functions");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let text = String::from_utf8(out.stdout).unwrap();
        text.lines().map(|l| l.split('\t').nth(1).unwrap_or_default().to_string()).collect::<Vec<_>>()
    };
    assert!(functions("off").iter().all(|n| n.starts_with("sub_")));
    assert_eq!(
        functions("on"),
        ["sub_401000", "sum_bytes", "sub_4010b0", "sub_4010c0", "wrap_mix", "wrap_sum", "wrap_seven", "mix_fold", "get_seven"]
    );
}

#[test]
fn usage_errors_and_mixed_architectures_write_nothing() {
    let dir = common::repo_root().join("tests/fixtures");
    let scratch = tempfile::tempdir().unwrap();
    let out = scratch.path().join("out.pat");
    let run = |args: &[&std::ffi::OsStr]| {
        Command::new(env!("CARGO_BIN_EXE_kuna")).args(["flirt", "make"]).args(args).output().expect("run kuna")
    };
    let no_output = run(&[dir.join("flirt/make/libthree.a").as_os_str()]);
    assert_eq!(no_output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&no_output.stderr).contains("-o <out.pat> is required"));
    let thumb = common::fixture("fid_thumb_probe.o");
    let mixed = run(&[
        dir.join("flirt/make/libthree.a").as_os_str(),
        thumb.as_ref(),
        "-o".as_ref(),
        out.as_os_str(),
    ]);
    assert_eq!(mixed.status.code(), Some(1), "{}", String::from_utf8_lossy(&mixed.stderr));
    assert!(!out.exists());
}
