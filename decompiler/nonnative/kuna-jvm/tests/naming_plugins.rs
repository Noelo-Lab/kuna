//! The bundled Kotlin and IdeaNotNull plugins' registration.

#[test]
fn bundled_plugins_register_in_oracle_order_with_their_options() {
    let plugins = kuna_jvm::kuna_plugins::bundled_plugins();
    let ids: Vec<String> = plugins.iter().map(|p| p.id()).collect();
    assert_eq!(ids, ["Kotlin", "IdeaNotNull"]);
    let options: Vec<(String, &str, Option<String>)> =
        kuna_jvm::kuna_plugins::bundled_plugin_options()
            .into_iter()
            .map(|o| (o.plugin, o.option.key, o.default))
            .collect();
    assert_eq!(
        options,
        [
            ("Kotlin".to_string(), "kt-enable", Some("1".to_string())),
            (
                "Kotlin".to_string(),
                "kt-unknown-defaults",
                Some("...".to_string())
            ),
            (
                "Kotlin".to_string(),
                "kt-decompile-facades",
                Some("0".to_string())
            ),
        ]
    );
}
