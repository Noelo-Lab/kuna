use super::*;

const RENDERERS: [fn(&Json) -> String; 3] = [dumps_compact, dumps_indent2, dumps_indent2_sorted];

#[test]
fn nested_containers_and_numeric_spelling_are_byte_stable() {
    let value = Json::Object(vec![
        (
            "z".into(),
            Json::Array(vec![
                Json::Number("-0".into()),
                Json::Number("1.2300e+02".into()),
                Json::Number("18446744073709551616".into()),
            ]),
        ),
        (
            "a".into(),
            Json::Array(vec![Json::Object(vec![]), Json::Array(vec![])]),
        ),
    ]);
    assert_eq!(
        dumps_compact(&value),
        r#"{"z":[-0,1.2300e+02,18446744073709551616],"a":[{},[]]}"#
    );
    assert_eq!(
        dumps_indent2(&value),
        "{\n  \"z\": [\n    -0,\n    1.2300e+02,\n    18446744073709551616\n  ],\n  \"a\": [\n    {},\n    []\n  ]\n}"
    );
    assert_eq!(
        dumps_indent2_sorted(&value),
        "{\n  \"a\": [\n    {},\n    []\n  ],\n  \"z\": [\n    -0,\n    1.2300e+02,\n    18446744073709551616\n  ]\n}"
    );
}

#[test]
fn sorting_is_recursive_and_stable_for_duplicate_keys() {
    let value = Json::Object(vec![
        ("z".into(), Json::Number("2".into())),
        (
            "a".into(),
            Json::Object(vec![
                ("z".into(), Json::Bool(false)),
                ("a".into(), Json::Null),
            ]),
        ),
        ("z".into(), Json::Number("3".into())),
    ]);
    assert_eq!(
        dumps_compact(&value),
        r#"{"z":2,"a":{"z":false,"a":null},"z":3}"#
    );
    assert_eq!(
        dumps_indent2_sorted(&value),
        "{\n  \"a\": {\n    \"a\": null,\n    \"z\": false\n  },\n  \"z\": 2,\n  \"z\": 3\n}"
    );
}

#[test]
fn all_modes_keep_ascii_safe_control_and_surrogate_escapes() {
    let value = Json::Str(
        "\0\x1f\x7f\"\\/\u{85}\u{2028}\u{feff}\u{ffff}\u{10000}\u{10ffff}\x08\x0c\n\r\t".into(),
    );
    let expected =
        r#""\u0000\u001f\u007f\"\\/\u0085\u2028\ufeff\uffff\ud800\udc00\udbff\udfff\b\f\n\r\t""#;
    for render in RENDERERS {
        assert_eq!(render(&value), expected);
    }
}

#[test]
fn primitives_and_empty_containers_have_no_layout_whitespace() {
    for (value, expected) in [
        (Json::Null, "null"),
        (Json::Bool(true), "true"),
        (Json::Bool(false), "false"),
        (Json::Number("-0.00E-010".into()), "-0.00E-010"),
        (Json::Str(String::new()), "\"\""),
        (Json::Array(vec![]), "[]"),
        (Json::Object(vec![]), "{}"),
    ] {
        for render in RENDERERS {
            assert_eq!(render(&value), expected);
        }
    }
}

#[test]
fn every_unicode_scalar_round_trips_through_ascii_output() {
    let text: String = (0..=0x10ffff).filter_map(char::from_u32).collect();
    let value = Json::Str(text.clone());
    for render in RENDERERS {
        let output = render(&value);
        assert!(output.is_ascii());
        assert_eq!(serde_json::from_str::<String>(&output).unwrap(), text);
    }
}
