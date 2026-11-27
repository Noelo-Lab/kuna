use crate::common;
use serde_json::Value;
use std::collections::BTreeSet;

const ABIS: [&str; 4] = [
    "aarch64-linux-android",
    "armv7a-linux-androideabi",
    "i686-linux-android",
    "x86_64-linux-android",
];

fn image(dir: &str, name: &str) -> String {
    common::repo_root()
        .join("tests/fixtures")
        .join(dir)
        .join(format!("{name}.so"))
        .display()
        .to_string()
}

fn json(args: &[&str]) -> Value {
    let (text, err, status) = common::run_kuna(args);
    assert_eq!(status, 0, "{args:?}: {err}");
    serde_json::from_str(&text).unwrap()
}

fn selector(name: &str) -> String {
    format!("Java_demo_Bridge_{name}__Ldemo_Player_2I")
}

fn refs(lib: &str, function: &str) -> Vec<Value> {
    json(&[
        "xrefs",
        lib,
        "--from",
        &selector(function),
        "--kind",
        "java-call",
        "--json",
    ])["xrefs"]
        .as_array()
        .unwrap()
        .clone()
}

fn keys(call: &Value) -> BTreeSet<&str> {
    call["targets"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["key"].as_str())
        .collect()
}

#[test]
fn method_ids_survive_helpers_and_caches_on_all_android_abis() {
    let key = "Ldemo/Player;->seek(I)V";
    for abi in ABIS {
        let lib = image("android-java-calls", abi);
        let cached = refs(&lib, "cached");
        assert_eq!(cached.len(), 1, "{abi}: {cached:?}");
        let call = &cached[0];
        assert_eq!(keys(call), BTreeSet::from([key]), "{abi}");
        assert_eq!(call["incomplete"], true, "{abi}: a cached ID can be replaced by unwalked code");
        assert_eq!(call["dispatch"], "virtual");
        assert_ne!(call["site"], call["targets"][0]["lookup"]);
        let by_pc = json(&[
            "xrefs",
            &lib,
            "--from",
            call["site"].as_str().unwrap(),
            "--kind",
            "java-call",
            "--json",
        ]);
        assert_eq!(by_pc["xrefs"][0]["targets"], call["targets"], "{abi}");
        let reverse = json(&["xrefs", &lib, "--to", key, "--json"]);
        assert_eq!(reverse["coverage"]["enabled"], true);
        assert_eq!(reverse["coverage"]["limited"], false, "{abi}");
        assert!(
            reverse["xrefs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|row| row["owner"] == call["owner"] && row["site"] == call["site"]),
            "{abi}: {reverse}"
        );
        let registered = json(&[
            "xrefs",
            &lib,
            "--to",
            "Ldemo/Registered;->touch()V",
            "--json",
        ]);
        assert!(
            registered["xrefs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["from_function"]["name"]
                    .as_str()
                    .is_some_and(|n| n.contains("registeredCall"))
                    && r["incomplete"] == false),
            "{abi}: {registered}"
        );
        let direct = refs(&lib, "direct");
        assert_eq!(direct.len(), 2, "{abi}");
        assert!(direct.iter().any(|r| keys(r) == BTreeSet::from([key])));
        assert!(direct
            .iter()
            .any(|r| keys(r) == BTreeSet::from(["Ldemo/Player;->stop()V"])));
        let choice = refs(&lib, "choose");
        assert_eq!(choice.len(), 1, "{abi}");
        assert_eq!(
            keys(&choice[0]),
            BTreeSet::from(["Ldemo/Player;->left()V", "Ldemo/Player;->right()V"])
        );
        assert_eq!(choice[0]["incomplete"], false);
        let disabled = json(&[
            "xrefs", &lib, "--to", key, "--option", "jnicalls", "off", "--json",
        ]);
        assert_eq!(disabled["count"], 0);
        assert_eq!(disabled["coverage"]["enabled"], false);
        let short = json(&["xrefs", &lib, "--to", "Ldemo/Short;->level()I", "--json"]);
        let rows = short["xrefs"].as_array().unwrap();
        assert_eq!(rows.len(), 1, "{abi}: {short}");
        assert_eq!(
            rows[0]["from_function"]["name"],
            "Java_demo_Bridge_shortName"
        );
        assert_eq!(rows[0]["incomplete"], true);
        let unrelated = json(&[
            "xrefs",
            &lib,
            "--from",
            "Java_demo_Bridge_numeric__JI",
            "--kind",
            "java-call",
            "--json",
        ]);
        assert_eq!(unrelated["count"], 0, "{abi}");
    }
}

#[test]
fn distinct_dispatch_families_and_uncertain_handles_remain_distinguishable() {
    let lib = image("android-java-calls", ABIS[0]);
    for (key, dispatch, form, complete) in [
        ("Ldemo/Player;->reset(I)V", "static", "A", true),
        ("Ldemo/Player;-><init>(I)V", "constructor", "A", true),
        ("Ldemo/Player;->step(I)V", "nonvirtual", "variadic", true),
        ("Ldemo/Player;->step(I)V", "virtual", "V", false),
    ] {
        let reverse = json(&["xrefs", &lib, "--to", key, "--kind", "java-call", "--json"]);
        assert!(
            reverse["xrefs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["dispatch"] == dispatch
                    && r["form"] == form
                    && r["incomplete"] == !complete),
            "{reverse}"
        );
    }
    for func in ["unknown", "wrongFamily", "externalWrite", "otherClass"] {
        let rows = refs(&lib, func);
        assert!(!rows.is_empty(), "{func}");
        assert!(
            rows.iter().all(|r| r["incomplete"] == true),
            "{func}: {rows:?}"
        );
    }
    let rows = refs(&lib, "references");
    assert!(rows
        .iter()
        .any(|r| r["incomplete"] == true && keys(r).contains("Ldemo/Player;->fromGlobal()V")));
    for name in ["fromLocal", "fromWeak"] {
        assert!(
            rows.iter().any(|r| r["incomplete"] == true
                && r["targets"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|t| t["name"] == name && t["class"].is_null())),
            "{name}: {rows:?}"
        );
    }
}

#[test]
fn binder_callback_codes_come_from_the_registered_interface_and_name_table() {
    for abi in ABIS
        .into_iter()
        .chain(["no-map", "mutable-map", "dense-helper"])
    {
        let lib = image("android-binder", abi);
        let define = json(&["decompile", &lib, "define_level", "--json"]);
        let service = &define["functions"][0]["binder_services"][0];
        assert_eq!(service["descriptor"], "org.example.ILevel", "{define}");
        let address = service["transact"].as_str().unwrap();
        let dispatch = json(&["decompile", &lib, address, "--addr", "--json"]);
        assert_eq!(
            dispatch["functions"][0]["binder_services"][0], *service,
            "{abi}"
        );
        for (code, expected) in [(1, "readLevel"), (2, "writeLevel")] {
            let rows = service["transactions"].as_array().unwrap();
            assert!(
                rows.iter().any(|row| row["code"] == code
                    && row["site"].is_string()
                    && if matches!(abi, "no-map" | "mutable-map") {
                        row["name"].is_null()
                    } else {
                        row["name"] == expected
                    }),
                "{abi}: {rows:?}"
            );
        }
        if abi == "dense-helper" {
            let rows = service["transactions"].as_array().unwrap();
            assert_eq!(rows.len(), 32);
            let mut sites = BTreeSet::new();
            for (index, row) in rows.iter().enumerate() {
                assert_eq!(row["code"], index + 1);
                sites.insert(row["site"].as_str().unwrap());
                if index >= 2 {
                    assert!(row["name"].is_null());
                }
            }
            assert_eq!(sites.len(), 32);
        }
        let disabled = json(&[
            "decompile",
            &lib,
            address,
            "--addr",
            "--option",
            "ndkbinder",
            "off",
            "--json",
        ]);
        assert!(
            disabled["functions"][0].get("binder_services").is_none(),
            "{abi}"
        );
    }
}
