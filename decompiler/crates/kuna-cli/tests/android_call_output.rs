use crate::common;
use serde_json::Value;
use std::fs;

fn run(args: &[&str]) -> String {
    let (out, err, status) = common::run_kuna(args);
    assert_eq!(status, 0, "{args:?}: {err}");
    out
}

#[test]
fn dense_binder_transactions_annotate_their_own_case_labels() {
    let image = common::repo_root().join("tests/fixtures/android-binder/dense-helper.so");
    let source = run(&[
        "decompile",
        image.to_str().unwrap(),
        "org_example_ILevel_onTransact",
    ]);
    let mut seen = std::collections::BTreeSet::new();
    for line in source
        .lines()
        .filter(|line| line.contains("Binder org.example.ILevel transaction "))
    {
        let (label, note) = line
            .trim()
            .split_once(':')
            .unwrap_or_else(|| panic!("{line}"));
        let number = label
            .strip_prefix("case ")
            .unwrap_or_else(|| panic!("{line}"));
        let value: u32 = match number.strip_prefix("0x") {
            Some(hex) => u32::from_str_radix(hex, 16).unwrap(),
            None => number.parse().unwrap(),
        };
        let description = match value {
            1 => "1 = readLevel".into(),
            2 => "2 = writeLevel".into(),
            other => other.to_string(),
        };
        assert_eq!(
            note.trim(),
            format!("/* Binder org.example.ILevel transaction {description} */")
        );
        assert!(seen.insert(value), "{line}");
    }
    assert_eq!(seen, (1..=32).collect(), "{source}");
}

#[test]
fn single_parallel_batch_and_stream_agree_on_android_targets() {
    for (directory, selector, other, field, option) in [
        (
            "android-java-calls",
            "Java_demo_Bridge_direct__Ldemo_Player_2I",
            "Java_demo_Bridge_cached__Ldemo_Player_2I",
            "java_calls",
            "jnicalls",
        ),
        (
            "android-binder",
            "org_example_ILevel_onTransact",
            "define_level",
            "binder_services",
            "ndkbinder",
        ),
    ] {
        let image = common::repo_root().join(format!(
            "tests/fixtures/{directory}/aarch64-linux-android.so"
        ));
        let image = image.to_str().unwrap();
        let selected = format!("{selector},{other}");
        let single: Value =
            serde_json::from_str(&run(&["decompile", image, selector, "--json"])).unwrap();
        let row = &single["functions"][0];
        let off: Value = serde_json::from_str(&run(&[
            "decompile",
            image,
            row["address_hex"].as_str().unwrap(),
            "--addr",
            "--json",
            "--option",
            option,
            "off",
        ]))
        .unwrap();
        assert!(off["functions"][0].get(field).is_none());
        let facts = row[field].as_array().unwrap();
        assert!(!facts.is_empty());

        let batch: Value = serde_json::from_str(&run(&[
            "decompile-all",
            image,
            "--functions",
            &selected,
            "--jobs",
            "2",
            "--jobs-chunk",
            "1",
            "--json",
        ]))
        .unwrap();
        let matching = batch["functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["name"] == selector)
            .unwrap();
        assert_eq!(matching[field], row[field]);

        let dir = tempfile::tempdir().unwrap();
        run(&[
            "decompile-project",
            image,
            "--functions",
            &selected,
            "--stream",
            "--jobs",
            "2",
            "--jobs-chunk",
            "1",
            "-o",
            dir.path().to_str().unwrap(),
        ]);
        let index_file = fs::read_to_string(dir.path().join("index.jsonl")).unwrap();
        let index = index_file
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .find(|r| r["name"] == selector)
            .unwrap();
        assert_eq!(index[field], row[field]);
        let c = fs::read_dir(dir.path())
            .unwrap()
            .flatten()
            .map(|entry| entry.path())
            .find(|path| path.extension().is_some_and(|e| e == "c"))
            .unwrap();
        let source = fs::read_to_string(c).unwrap();
        let start = index["c_offset"].as_u64().unwrap() as usize;
        let len = index["c_len"].as_u64().unwrap() as usize;
        for source in [
            row["code"].as_str().unwrap(),
            matching["code"].as_str().unwrap(),
            &source[start..start + len],
        ] {
            if field == "java_calls" {
                for call in facts {
                    let key = call["targets"][0]["key"].as_str().unwrap();
                    assert!(source.contains(&format!("/* {key} */")), "{source}");
                }
                assert!(source.contains("CallVoidMethod"), "{source}");
            } else {
                for (code, name) in [(1, "readLevel"), (2, "writeLevel")] {
                    let note = format!("Binder org.example.ILevel transaction {code} = {name}");
                    let line = source
                        .lines()
                        .find(|line| line.contains(&note))
                        .unwrap_or_else(|| panic!("{source}"));
                    assert!(line.contains(&format!("code == {code}")), "{source}");
                    assert_eq!(source.matches(&note).count(), 1, "{source}");
                }
            }
        }
    }
}
