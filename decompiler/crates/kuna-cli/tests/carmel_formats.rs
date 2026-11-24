use std::{path::PathBuf, process::Command};

use crate::common;
use serde_json::Value;

#[test]
fn carmel_object_relocations_reach_named_callees_in_emitted_c() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixtures = root.join("tests/fixtures/carmel/containers");
    for file in ["caller-object.ieee695", "caller-linked.ieee695"] {
        let output = common::process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(fixtures.join(file)).args(["_call_addone", "--json", "--sleighpath"])
            .arg(root.join("specs")));
        let json: Value = serde_json::from_slice(&output.stdout).unwrap();
        let function = &json["functions"][0];
        assert_eq!(function["name"], "_call_addone", "{file}: {function}");
        assert!(function["error"].is_null(), "{file}: {function}");
        assert!(function["code"].as_str().unwrap().contains("_addone("), "{file}: {function}");
    }
}
