//! `--types` loads a header's declarations for one run.
//!
//! `fixtures/types_header/curlish.so` is `curlish.c` built with
//! `clang -O2 -c curlish.c -o curlish.o && ld.lld -shared -o curlish.so curlish.o`.
use crate::common;
use std::process::Command;

fn fixture(name: &str) -> String {
    common::repo_root()
        .join("decompiler/crates/kuna-cli/tests/fixtures/types_header")
        .join(name)
        .to_str()
        .unwrap()
        .to_owned()
}

fn decompile(extra: &[&str]) -> (String, String, i32) {
    let image = fixture("curlish.so");
    let mut args = vec!["decompile", image.as_str(), "fetch"];
    args.extend(extra);
    common::run_kuna(&args)
}

#[test]
fn a_header_types_the_calls_into_its_library() {
    let (bare, _, _) = decompile(&[]);
    assert!(bare.contains("curl_easy_setopt(v1,0x2712,a0)"), "{bare}");

    let (c, stderr, code) = decompile(&["--types", &fixture("curl.h")]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
    assert!(c.contains("CURL *h;"), "{c}");
    assert!(c.contains("curl_easy_setopt(h,CURLOPT_URL,a0)"), "{c}");
    assert!(c.contains("curl_easy_setopt(h,CURLOPT_TIMEOUT,0x1e)"), "{c}");
}

#[test]
fn a_directory_and_a_second_header_combine() {
    let (c, stderr, code) = decompile(&["--types", &fixture("dir"), "--types", &fixture("app.h")]);
    assert_eq!(code, 0, "{stderr}");
    assert!(c.contains("int fetch(char *url)"), "{c}");
    assert!(c.contains("curl_easy_setopt(h,CURLOPT_URL,url)"), "{c}");
    assert!(c.contains("return curl_easy_setopt(h,CURLOPT_TIMEOUT,0x1e)"), "{c}");
}

#[test]
fn a_bad_declaration_is_reported_and_the_rest_apply() {
    for surface in [&["--assert-strict"][..], &["--assert-strict", "--json"][..]] {
        let mut args = vec!["--types".to_string(), fixture("bad.h")];
        args.extend(surface.iter().map(|s| s.to_string()));
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let (out, stderr, code) = decompile(&args);
        assert_eq!(code, 0, "{stderr}");
        assert!(out.contains("curl_easy_setopt(h,CURLOPT_URL,a0)"), "{out}");
        let warnings: Vec<&str> = stderr.lines().collect();
        assert_eq!(warnings.len(), 1, "{stderr}");
        assert!(
            warnings[0].contains("bad.h:4: CURL_EXTERN CURL *curl_easy_init(void): rejected:"),
            "{stderr}"
        );
    }
    let (json, _, _) = decompile(&["--types", &fixture("bad.h"), "--json"]);
    let doc: serde_json::Value = serde_json::from_str(&json).unwrap();
    let rows = doc["assertions"].as_array().unwrap();
    let status: Vec<&str> = rows.iter().map(|r| r["status"].as_str().unwrap()).collect();
    assert_eq!(status, ["applied", "applied", "rejected", "applied"]);
    assert_eq!(rows[2]["kind"], "prototype");
}

#[test]
fn an_assertion_outranks_the_header() {
    let (c, stderr, code) = decompile(&[
        "--assert",
        "prototype curl_easy_setopt long curl_easy_setopt(void *handle,int option,long value)",
        "--types",
        &fixture("curl.h"),
    ]);
    assert_eq!(code, 0, "{stderr}");
    assert!(c.contains("curl_easy_setopt(handle,0x2712,a0)"), "{c}");
}

#[test]
fn the_whole_binary_surface_takes_the_header() {
    let image = fixture("curlish.so");
    let header = fixture("curl.h");
    let (c, stderr, code) = common::run_kuna(&["decompile-all", &image, "--types", &header]);
    assert_eq!(code, 0, "{stderr}");
    assert!(c.contains("curl_easy_setopt(h,CURLOPT_URL,a0)"), "{c}");
    assert!(c.contains("int curl_easy_setopt(CURL *h,CURLoption opt,...)"), "{c}");
}

#[test]
fn the_preprocessor_runs_only_when_asked() {
    let header = fixture("macro.h");
    let (c, stderr, code) = decompile(&["--types", &header]);
    assert_eq!(code, 0, "{stderr}");
    assert!(c.contains("curl_easy_setopt(v1,0x2712,a0)"), "{c}");
    assert_eq!(stderr.lines().filter(|l| l.contains("macro.h:")).count(), 3, "{stderr}");

    if !Command::new("cc").arg("--version").output().is_ok_and(|o| o.status.success()) {
        eprintln!("cc unavailable; skipping the --types-cpp half");
        return;
    }
    let (c, stderr, code) = decompile(&["--types", &header, "--types-cpp", "cc"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stderr.is_empty(), "{stderr}");
    assert!(c.contains("curl_easy_setopt(handle,CURLOPT_URL,a0)"), "{c}");
}

#[test]
fn unusable_inputs_are_usage_errors() {
    let missing = fixture("absent.h");
    let (_, stderr, code) = decompile(&["--types", &missing]);
    assert_ne!(code, 0);
    assert!(stderr.contains("--types") && stderr.contains("absent.h"), "{stderr}");

    let (_, stderr, code) = decompile(&["--types-cpp", "cc"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("--types-cpp needs a --types"), "{stderr}");

    let (_, stderr, code) =
        decompile(&["--types", &fixture("curl.h"), "--types-cpp", "kuna-no-such-preprocessor"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("--types-cpp"), "{stderr}");
}
