//! The JVM option table: its rows and their resolution.

use std::collections::BTreeSet;
use std::path::Path;

use kuna_jvm::kuna_options::{self, find, resolve, rows, ValueKind};

#[test]
fn rows_are_complete_and_point_at_real_modules() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut modules = BTreeSet::new();
    for folder in std::fs::read_dir(&src).unwrap() {
        let folder = folder.unwrap().path();
        if folder.is_dir() {
            for file in std::fs::read_dir(&folder).unwrap() {
                let file = file.unwrap().path();
                modules.insert(file.file_stem().unwrap().to_string_lossy().into_owned());
            }
        }
    }
    let mut names = BTreeSet::new();
    for row in rows() {
        assert!(names.insert(row.option), "{} twice", row.option);
        assert!(
            src.join(row.phase).is_dir(),
            "{}: no folder {}",
            row.option,
            row.phase
        );
        assert_eq!(row.rewind, row.phase, "{}", row.option);
        assert_eq!(row.strength, "HARD");
        for module in row.subphase.split(", ") {
            assert!(
                modules.contains(module),
                "{}: no module {module}",
                row.option
            );
        }
        assert!(["core", "transform", "analysis"].contains(&row.tier));
        assert!(row.example.starts_with("kuna ") && row.example.contains(row.option));
        assert!(!row.symptom_list().is_empty());
        assert!(row.symptom_list().iter().all(|s| !s.trim().is_empty()));
        for field in [row.summary, row.use_when, row.inspiration, row.values] {
            assert!(!field.trim().is_empty(), "{}", row.option);
        }
        if row.kind() == ValueKind::Bool {
            assert!(
                row.default == "on" || row.default == "off",
                "{}",
                row.option
            );
        }
    }
    assert!(rows().iter().all(|r| !r.destructive));
}

#[test]
fn resolution() {
    let name = "jvm_decompile_finally";
    assert_eq!(
        resolve(name, "off").unwrap(),
        ("decompile-finally", "0".to_string())
    );
    assert_eq!(
        resolve(name, "ON").unwrap(),
        ("decompile-finally", "1".to_string())
    );
    assert_eq!(
        resolve(name, "true").unwrap(),
        ("decompile-finally", "1".to_string())
    );
    assert_eq!(
        resolve(name, "0").unwrap(),
        ("decompile-finally", "0".to_string())
    );
    for alias in ["decompile-finally", "fdi", "rbr", "jrt"] {
        assert!(find(alias).is_none(), "{alias}");
    }
    assert_eq!(
        resolve("jvm_include_runtime", "off").unwrap(),
        ("include-runtime", "0".to_string())
    );
    assert_eq!(
        resolve("jvm_include_runtime", "on").unwrap(),
        ("include-runtime", "1".to_string())
    );
    assert_eq!(
        resolve("jvm_include_runtime", "current").unwrap(),
        ("include-runtime", "current".to_string())
    );
    assert_eq!(
        resolve("jvm_include_runtime", "/opt/jdk-21").unwrap(),
        ("include-runtime", "/opt/jdk-21".to_string())
    );
    assert_eq!(
        resolve("jvm_siblings", "off").unwrap(),
        ("jvm_siblings", "0".to_string())
    );
    assert_eq!(resolve("jvm_old_try_dedup", "1").unwrap().0, "old-try-dedup");
}

#[test]
fn resolution_errors() {
    let err = resolve("jvm_decompile_finaly", "on").unwrap_err();
    assert!(err.contains("did you mean jvm_decompile_finally"), "{err}");
    let err = resolve("decompile-finally", "on").unwrap_err();
    assert!(err.contains("did you mean jvm_decompile_finally"), "{err}");
    for name in ["jvm_nothing_like_this_at_all", "thread-count", "jvm_thread_count"] {
        let err = resolve(name, "on").unwrap_err();
        assert!(err.contains("unknown JVM option"), "{err}");
    }
    let err = resolve("jvm_remove_bridge", "maybe").unwrap_err();
    assert!(err.contains("expected on|off"), "{err}");
    assert_eq!(
        kuna_options::suggest("jvm_remove-bridge"),
        Some("jvm_remove_bridge")
    );
}
