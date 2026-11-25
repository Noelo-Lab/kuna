//! The RFC #1093 auth parser and the negatives in `testdata/`, learned
//! against their machine code. Needs z3; skipped without it.

use std::path::Path;

use kuna_verify::state::Env;
use kuna_wbcq::engine::{learn, report, Config};
use kuna_wbcq::machine::Abi;
use object::{Object, ObjectSymbol};
use serde_json::Value;

fn z3() -> bool {
    std::process::Command::new("z3")
        .arg("-version")
        .output()
        .is_ok_and(|o| o.status.success())
}

fn learned(elf: &str, function: &str) -> Value {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = dir.join("testdata").join(elf);
    let bytes = std::fs::read(&path).expect("fixture");
    let file = object::File::parse(&*bytes).expect("an ELF");
    let entry = file
        .symbols()
        .find(|s| s.name() == Ok(function))
        .expect("the function's symbol")
        .address();
    let specs = dir
        .join("../../../specs")
        .canonicalize()
        .expect("specs")
        .to_string_lossy()
        .into_owned();
    kuna_verify::session::with_program(path.to_str().expect("a path"), &[], &[specs], |prog| {
        let env = Env::new(prog, 1).expect("an environment");
        let abi = Abi::for_program(prog).expect("a calling convention");
        let cfg = Config {
            entry,
            ..Config::default()
        };
        match learn(&env, &abi, &cfg) {
            Ok((l, m)) => report(&l, &m),
            Err(why) => serde_json::json!({ "status": format!("unsupported: {why}") }),
        }
    })
    .expect("the program loads")
}

#[test]
fn the_flattened_auth_parser_is_three_proved_states() {
    if !z3() {
        eprintln!("z3 not found: skipped");
        return;
    }
    for elf in ["auth_x86_64_O2.elf", "auth_aarch64_O0.elf"] {
        for function in ["auth_step", "auth_flat_step"] {
            let r = learned(elf, function);
            assert_eq!(r["status"], "proved_by_path_closure", "{elf} {function}");
            assert_eq!(
                r["states"].as_array().map(Vec::len),
                Some(3),
                "{elf} {function}"
            );
            assert_eq!(
                r["transitions"].as_array().map(Vec::len),
                Some(5),
                "{elf} {function}"
            );
            let words: Vec<&Value> = r["witnesses"]
                .as_array()
                .expect("witnesses")
                .iter()
                .map(|w| &w["word"])
                .collect();
            assert_eq!(
                words,
                [
                    &serde_json::json!([0xa5]),
                    &serde_json::json!([0]),
                    &serde_json::json!([0])
                ]
            );
            assert_eq!(r["control_bytes"], serde_json::json!([0, 1, 2, 3]));
            assert_eq!(r["data_bytes"], serde_json::json!([4, 5, 6, 7]));
        }
    }
}

#[test]
fn what_the_model_does_not_cover_fails_closed() {
    if !z3() {
        eprintln!("z3 not found: skipped");
        return;
    }
    for (function, status) in [
        ("echo_step", "partial_frontier: more than 256 paths"),
        (
            "counter_step",
            "partial_frontier: more than 32 control valuations",
        ),
        ("global_step", "unsupported: a write outside the context"),
    ] {
        let r = learned("auth_x86_64_O2.elf", function);
        let got = r["status"].as_str().expect("a status");
        assert!(got.starts_with(status), "{function}: {got}");
    }
}
