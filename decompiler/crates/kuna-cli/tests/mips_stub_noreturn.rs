//! An import reached through a `.MIPS.stubs` lazy stub returns unless its own
//! name is a known no-return one, wherever the linker put its stub.
use crate::common;
use serde_json::Value;

const IMAGES: [(&str, &[&str]); 3] = [
    ("mipsstubs_noreturn_el.so", &[]),
    ("mipsstubs_noreturn_eb.so", &[]),
    ("mipsstubs_noreturn_n64el.so", &["--target", "MIPS:LE:64:default"]),
];

const RETURNING: [(&str, &str); 4] = [
    ("save", "fclose("),
    ("copy", "+ 1;"),
    ("same", "== 0;"),
    ("say", "= 2;"),
];

fn code<'a>(doc: &'a Value, name: &str) -> &'a str {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == name)
        .and_then(|f| f["code"].as_str())
        .unwrap_or_else(|| panic!("no {name} in {doc}"))
}

fn no_return_calls(code: &str) -> Vec<&str> {
    code.lines().filter(|line| line.contains("no-return")).map(str::trim).collect()
}

#[test]
fn lazy_stub_imports_keep_their_own_no_return() {
    for (image, extra) in IMAGES {
        let path = common::fixture(image);
        let mut args = vec!["decompile-all", path.as_str(), "--json"];
        args.extend_from_slice(extra);
        let (out, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{image}: {err}");
        let doc: Value = serde_json::from_str(&out).unwrap();
        for (name, tail) in RETURNING {
            let body = code(&doc, name);
            assert!(no_return_calls(body).is_empty(), "{image} {name}: {body}");
            assert!(body.contains(tail), "{image} {name} lost its tail: {body}");
        }
        for (name, callee) in [("die", "exit("), ("check", "abort(")] {
            let body = code(&doc, name);
            let calls = no_return_calls(body);
            assert!(
                calls.len() == 1 && calls[0].contains(callee),
                "{image} {name}: {body}"
            );
        }
    }
}
