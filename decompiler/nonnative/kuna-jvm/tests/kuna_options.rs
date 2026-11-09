//! The JVM option table: one to one with Vineflower's options, resolution, aliases.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_options::{self, driver_owned, find, resolve, rows, ValueKind};

#[test]
fn table_is_one_to_one_with_vineflower_options() {
    let defaults = IFernflowerPreferences::DEFAULTS();
    let mut claimed: BTreeMap<&str, &str> = BTreeMap::new();
    for row in rows() {
        if row.is_kuna() {
            assert_eq!(row.source_decompiler, "kuna", "{}", row.option);
            continue;
        }
        assert_eq!(row.source_decompiler, "vineflower", "{}", row.option);
        assert_eq!(row.change_kind, "vineflower-option", "{}", row.option);
        assert!(
            claimed.insert(row.vineflower, row.option).is_none(),
            "{} has two rows",
            row.vineflower
        );
    }
    for owned in driver_owned() {
        assert!(
            claimed.insert(owned.vineflower, "driver").is_none(),
            "{} is both a row and driver-owned",
            owned.vineflower
        );
    }
    let all: BTreeSet<&str> = IFernflowerPreferences::all_options()
        .iter()
        .map(|o| o.key)
        .collect();
    let covered: BTreeSet<&str> = claimed.keys().copied().collect();
    assert_eq!(
        covered, all,
        "rows + driver-owned must be exactly all_options()"
    );

    for row in rows().iter().filter(|r| !r.is_kuna()) {
        let option = IFernflowerPreferences::find_option(row.vineflower).unwrap();
        assert_eq!(row.short, option.short_name.unwrap_or(""), "{}", row.option);
        let (key, value) = resolve(row.option, row.default).unwrap();
        assert_eq!(key, row.vineflower);
        assert_eq!(
            Some(&value),
            defaults.get(&row.vineflower.to_string()),
            "{}: default {:?} disagrees with Vineflower's",
            row.option,
            row.default
        );
        assert!(
            row.inspiration.contains(option.field),
            "{}: inspiration must name IFernflowerPreferences.{}",
            row.option,
            option.field
        );
    }
}

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
    let destructive: Vec<&str> = rows()
        .iter()
        .filter(|r| r.destructive)
        .map(|r| r.option)
        .collect();
    assert_eq!(
        destructive,
        ["jvm_show_hidden_statements", "jvm_dump_text_tokens"]
    );
}

#[test]
fn resolution_and_aliases() {
    for name in ["jvm_decompile_finally", "decompile-finally", "fdi"] {
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
    }
    assert_eq!(find("rbr").unwrap().option, "jvm_remove_bridge");
    assert_eq!(
        resolve("jvm_preferred_line_length", "120").unwrap(),
        ("preferred-line-length", "120".to_string())
    );
    assert_eq!(
        resolve("jvm_indent_string", "\t").unwrap(),
        ("indent-string", "\t".to_string())
    );
    assert_eq!(
        resolve("jvm_excluded_classes", "^com/ex/.*").unwrap(),
        ("excluded-classes", "^com/ex/.*".to_string())
    );
    assert_eq!(
        resolve("jrt", "off").unwrap(),
        ("include-runtime", "0".to_string())
    );
    assert_eq!(
        resolve("jrt", "on").unwrap(),
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
    assert_eq!(resolve("old-try-dedup", "1").unwrap().0, "old-try-dedup");
}

#[test]
fn resolution_errors() {
    let err = resolve("jvm_decompile_finaly", "on").unwrap_err();
    assert!(err.contains("did you mean jvm_decompile_finally"), "{err}");
    let err = resolve("decompile-finaly", "on").unwrap_err();
    assert!(err.contains("did you mean decompile-finally"), "{err}");
    let err = resolve("jvm_nothing_like_this_at_all", "on").unwrap_err();
    assert!(err.contains("unknown JVM option"), "{err}");
    for name in ["thread-count", "jvm_thread_count", "max-time-per-method"] {
        let err = resolve(name, "4").unwrap_err();
        assert!(err.contains("set by kuna itself"), "{err}");
    }
    assert!(resolve("thread-count", "4").unwrap_err().contains("--jobs"));
    let err = resolve("jvm_remove_bridge", "maybe").unwrap_err();
    assert!(err.contains("expected on|off"), "{err}");
    let err = resolve("jvm_preferred_line_length", "wide").unwrap_err();
    assert!(err.contains("integer"), "{err}");
    let err = resolve("jvm_included_classes", "(unclosed").unwrap_err();
    assert!(err.contains("invalid regular expression"), "{err}");
    assert_eq!(
        kuna_options::suggest("jvm_remove-bridge"),
        Some("jvm_remove_bridge")
    );
}
