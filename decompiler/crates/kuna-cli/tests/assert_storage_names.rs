use crate::common;
use serde_json::Value;

const AARCH64: &str = "assert_storage_aarch64.so";

fn run(fixture: &str, function: &str, directive: &str) -> (i32, Value, String) {
    let image = common::fixture(fixture);
    let (out, err, rc) =
        common::run_kuna(&["decompile", &image, function, "--json", "--assert-strict", "--assert", directive]);
    let json: Value = serde_json::from_str(&out).unwrap_or_else(|e| panic!("{directive}: {e}\n{out}\n{err}"));
    (rc, json, err)
}

fn outcome(json: &Value) -> (&str, &str) {
    let first = &json["assertions"][0];
    (first["status"].as_str().unwrap_or(""), first["detail"].as_str().unwrap_or(""))
}

fn code(json: &Value) -> &str {
    json["functions"][0]["code"].as_str().unwrap_or("")
}

#[test]
fn a_return_or_param_storage_the_target_does_not_name_is_rejected() {
    for function in ["pick", "read_back"] {
        for (storage, named) in [
            ("no_such_register", "no_such_register"),
            ("%no_such_register", "no_such_register"),
            ("RAX", "RAX"),
            ("%x0:junk", "x0:junk"),
            ("%0x4000zz", "0x4000zz"),
        ] {
            for directive in
                [format!("return pick::{storage} struct sample *"), format!("param pick::0 {storage} void *p")]
            {
                let (rc, json, err) = run(AARCH64, function, &directive);
                let (status, detail) = outcome(&json);
                assert_eq!(status, "rejected", "{function} {directive}\n{json}");
                assert!(detail.contains(named), "{function} {directive}: {detail}");
                assert_ne!(rc, 0, "{function} {directive}: strict mode exited 0\n{err}");
            }
        }
    }
}

#[test]
fn an_unqualified_return_storage_the_target_does_not_name_is_rejected() {
    let (rc, json, _) = run(AARCH64, "pick", "return no_such_register struct sample *");
    let (status, detail) = outcome(&json);
    assert_eq!(status, "rejected", "{json}");
    assert!(detail.contains("no_such_register"), "{detail}");
    assert_ne!(rc, 0);
}

#[test]
fn named_and_numeric_register_storage_still_applies() {
    for storage in ["x0", "%x0", "[register,0x4000,8]"] {
        let directive = format!("return pick::{storage} void *");
        let (rc, json, err) = run(AARCH64, "pick", &directive);
        assert_eq!(outcome(&json).0, "applied", "{directive}\n{json}");
        assert_eq!(rc, 0, "{directive}\n{err}");
        assert!(code(&json).contains("return a0;"), "{directive}\n{}", code(&json));
    }
    let (rc, json, _) = run(AARCH64, "pick", "param pick::1 w1 unsigned int v");
    assert_eq!((rc, outcome(&json).0), (0, "applied"), "{json}");
}

#[test]
fn register_names_are_resolved_against_the_target_language() {
    for storage in ["RAX", "rax", "EAX", "eax"] {
        let (rc, json, _) = run("prefixed_noreturn_x86_64", "keeps_count", &format!("return keeps_count::{storage} int"));
        assert_eq!((rc, outcome(&json).0), (0, "applied"), "{storage}\n{json}");
    }
    let (rc, json, _) = run("prefixed_noreturn_x86_64", "keeps_count", "return keeps_count::x0 int");
    let (status, detail) = outcome(&json);
    assert_eq!(status, "rejected", "{json}");
    assert!(detail.contains("x0"), "{detail}");
    assert_ne!(rc, 0);
}
