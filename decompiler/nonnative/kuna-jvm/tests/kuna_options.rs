//! The JVM option table: one to one with Vineflower's options, resolution, aliases, and the
//! kuna form options (their texts: `fix_isolation.rs`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::Fernflower;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_fixes::VINEFLOWER_COMPAT;
use kuna_jvm::kuna_options::{
    self, driver_owned, find, form_enabled, resolve, rows, take_form_options, ValueKind,
    JAVAC_LAYOUT, KUNA_FORM, SOURCE_LOCALS, SOURCE_PATTERNS,
};
use kuna_jvm::kuna_plugins;

#[test]
fn table_is_one_to_one_with_vineflower_options() {
    let defaults = IFernflowerPreferences::DEFAULTS();
    let plugin_options = kuna_plugins::bundled_plugin_options();
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
    let core: BTreeSet<&str> = IFernflowerPreferences::all_options()
        .iter()
        .map(|o| o.key)
        .collect();
    let mut all = core.clone();
    for plugin_option in &plugin_options {
        assert!(
            !core.contains(plugin_option.option.key),
            "{} of plugin {} is also a core option",
            plugin_option.option.key,
            plugin_option.plugin
        );
        assert!(
            all.insert(plugin_option.option.key),
            "{} declared twice",
            plugin_option.option.key
        );
    }
    let covered: BTreeSet<&str> = claimed.keys().copied().collect();
    assert_eq!(
        covered, all,
        "rows + driver-owned must be exactly all_options() and the bundled plugins' options"
    );

    for row in rows().iter().filter(|r| !r.is_kuna()) {
        let (option, default) = match IFernflowerPreferences::find_option(row.vineflower) {
            Some(option) => (option, defaults.get_cloned(&row.vineflower.to_string())),
            None => {
                let plugin_option = plugin_options
                    .iter()
                    .find(|o| o.option.key == row.vineflower)
                    .unwrap_or_else(|| panic!("{}: no such option", row.option));
                (plugin_option.option, plugin_option.default.clone())
            }
        };
        assert_eq!(row.short, option.short_name.unwrap_or(""), "{}", row.option);
        let (key, value) = resolve(row.option, row.default).unwrap();
        assert_eq!(key, row.vineflower);
        if default.is_none() {
            assert_eq!(row.default, "", "{}: Vineflower has no default", row.option);
        } else {
            assert_eq!(
                Some(&value),
                default.as_ref(),
                "{}: default {:?} disagrees with Vineflower's",
                row.option,
                row.default
            );
        }
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
    assert!(rows().iter().all(|r| !r.destructive));
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
    assert_eq!(
        resolve("jvm_plugins", "off").unwrap(),
        (kuna_plugins::JVM_PLUGINS, "0".to_string())
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
    for name in [
        "thread-count",
        "jvm_thread_count",
        "max-time-per-method",
        "jvm_indent_string",
        "jvm_preferred_line_length",
    ] {
        let err = resolve(name, "4").unwrap_err();
        assert!(err.contains("set by kuna itself"), "{err}");
    }
    assert!(resolve("thread-count", "4").unwrap_err().contains("--jobs"));
    let err = resolve("jvm_remove_bridge", "maybe").unwrap_err();
    assert!(err.contains("expected on|off"), "{err}");
    let err = resolve("jvm_included_classes", "(unclosed").unwrap_err();
    assert!(err.contains("invalid regular expression"), "{err}");
    assert_eq!(
        kuna_options::suggest("jvm_remove-bridge"),
        Some("jvm_remove_bridge")
    );
}

fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn form_options_are_kuna_rows_on_by_default() {
    for name in [JAVAC_LAYOUT, SOURCE_LOCALS, SOURCE_PATTERNS] {
        let row = find(name).unwrap();
        assert!(row.is_kuna());
        assert_eq!(
            (row.values, row.default, row.change_kind),
            ("on|off", "on", KUNA_FORM)
        );
        assert_eq!(resolve(name, "off").unwrap(), (name, "0".to_string()));
        let (forms, rest) = take_form_options(&pairs(&[(name, "off"), ("a", "1")]), false);
        assert!(!forms.contains(name));
        assert_eq!(rest, pairs(&[("a", "1")]));
        assert!(take_form_options(&[], false).0.contains(name));
        assert!(
            take_form_options(&pairs(&[(name, "0"), (name, "on")]), false)
                .0
                .contains(name)
        );
        assert!(!take_form_options(&pairs(&[(name, "on")]), true)
            .0
            .contains(name));
    }
}

/// Whether the context `Fernflower::new` installs for `options` has `jvm_source_locals` on.
fn engine_has_source_locals(options: &[(&str, &str)]) -> bool {
    let options = pairs(options);
    with_engine_stack(move || {
        reset_engine_thread();
        let mut fernflower = Fernflower::new(&options, <dyn IFernflowerLogger>::NO_OP());
        let on = form_enabled(SOURCE_LOCALS);
        let leaked = DecompilerContext::get_property(SOURCE_LOCALS).is_some();
        assert!(!leaked, "the option stays out of Vineflower's option map");
        fernflower.clear_context();
        reset_engine_thread();
        on
    })
}

#[test]
fn compat_turns_the_form_options_off() {
    assert!(engine_has_source_locals(&[]));
    assert!(engine_has_source_locals(&[(SOURCE_LOCALS, "1")]));
    assert!(!engine_has_source_locals(&[(SOURCE_LOCALS, "0")]));
    assert!(!engine_has_source_locals(&[(VINEFLOWER_COMPAT, "1")]));
    assert!(!engine_has_source_locals(&[
        (VINEFLOWER_COMPAT, "1"),
        (SOURCE_LOCALS, "1")
    ]));
}
