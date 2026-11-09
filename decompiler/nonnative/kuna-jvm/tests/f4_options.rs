//! `IFernflowerPreferences` against Vineflower 1.12.0 (`data/GenF4Oracle.java`):
//! every option field with its annotations, and the defaults map in the JDK's
//! iteration order, both as built and as copied by `new HashMap<>(DEFAULTS)`.

use kuna_jvm::i_fernflower_preferences::{IFernflowerPreferences, OptionType};

use crate::f4_util::{golden, unesc};

fn threads() -> String {
    std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .to_string()
}

fn defaults_golden(tag: &str) -> Vec<(String, String)> {
    golden("f4_preferences.txt")
        .iter()
        .filter_map(|line| {
            let cols: Vec<&str> = line.split('\t').collect();
            (cols[0] == tag).then(|| {
                let value = unesc(cols[2]).unwrap();
                let value = if value == "@THREADS@" {
                    threads()
                } else {
                    value
                };
                (unesc(cols[1]).unwrap(), value)
            })
        })
        .collect()
}

#[test]
fn option_table_matches_the_annotations() {
    let fields: Vec<Vec<Option<String>>> = golden("f4_preferences.txt")
        .iter()
        .filter(|l| l.starts_with("F\t"))
        .map(|l| l.split('\t').skip(1).map(unesc).collect())
        .collect();
    let options = IFernflowerPreferences::all_options();
    let mut table = options.iter();
    for f in &fields {
        let field = f[0].as_deref().unwrap();
        let value = f[1].as_deref().unwrap();
        match field {
            "LINE_SEPARATOR_WIN" => assert_eq!(IFernflowerPreferences::LINE_SEPARATOR_WIN, value),
            "LINE_SEPARATOR_UNX" => assert_eq!(IFernflowerPreferences::LINE_SEPARATOR_UNX, value),
            _ => {
                let o = table.next().unwrap_or_else(|| panic!("{field} missing"));
                assert_eq!(o.field, field);
                assert_eq!(o.key, value, "{field}");
                assert_eq!(o.short_name, f[2].as_deref(), "{field}");
                assert_eq!(o.name, f[3].as_deref(), "{field}");
                assert_eq!(o.description, f[4].as_deref(), "{field}");
                assert_eq!(o.type_.map(OptionType::name), f[5].as_deref(), "{field}");
                assert_eq!(o.dynamic_default_value, f[6].as_deref(), "{field}");
            }
        }
    }
    assert!(table.next().is_none());
    assert_eq!(options.len() + 2, fields.len());
    assert_eq!(fields.len(), 72);
}

#[test]
fn defaults_iterate_in_jdk_order() {
    let expected = defaults_golden("D");
    let defaults = IFernflowerPreferences::get_defaults();
    assert_eq!(defaults.entry_vec(), expected);
    assert_eq!(
        IFernflowerPreferences::default_entries().len(),
        expected.len()
    );

    let copied = defaults_golden("C");
    assert_eq!(IFernflowerPreferences::DEFAULTS().entry_vec(), copied);
    assert_eq!(
        IFernflowerPreferences::DEFAULTS().clone().entry_vec(),
        copied
    );
}

#[test]
fn option_metadata_lookups() {
    let jrt = IFernflowerPreferences::find_option("jrt").unwrap();
    assert_eq!(jrt.key, IFernflowerPreferences::INCLUDE_JAVA_RUNTIME);
    assert_eq!(jrt.type_, Some(OptionType::String));
    assert_eq!(jrt.default_value().as_deref(), Some("1"));
    assert_eq!(OptionType::Boolean.friendly_name(), "bool");
    let threads_option = IFernflowerPreferences::find_option("thread-count").unwrap();
    assert_eq!(threads_option.default_value(), Some(threads()));
    assert_eq!(
        IFernflowerPreferences::find_option("mark-corresponding-synthetics")
            .unwrap()
            .type_,
        None
    );
    let internal = IFernflowerPreferences::find_option("__unit_test_mode__").unwrap();
    assert_eq!(
        (internal.name, internal.default_value().as_deref()),
        (None, Some("0"))
    );
    assert_eq!(
        IFernflowerPreferences::find_option("log")
            .unwrap()
            .default_value()
            .as_deref(),
        Some("INFO")
    );
}
