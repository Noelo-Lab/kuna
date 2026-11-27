//! A managed engine's option rows in `kuna catalog`: JSON rows in the native
//! row shape plus `aliases` and `engine`, the generated
//! `docs/options-<engine>.md`, and the `--check` rules every engine's table
//! follows.

use std::collections::BTreeSet;

use kuna_decomp::kuna_phases::{emit_catalog_markdown_from, MarkdownDoc, MarkdownRow};
use kuna_managed::{Engine, OptionRow};

use crate::jsonfmt::Json;

fn strings<'a>(items: impl Iterator<Item = &'a str>) -> Json {
    Json::Array(items.map(|s| Json::Str(s.to_string())).collect())
}

/// Whether `row` answers to `name` (its own name or an alias).
fn answers(row: &OptionRow, name: &str) -> bool {
    row.option == name || row.aliases.contains(&name)
}

/// One row, keyed as `emit_settable_json` keys a native row.
fn row_json(row: &OptionRow, engine: &str) -> Json {
    let text = |s: &str| Json::Str(s.to_string());
    Json::Object(vec![
        ("option".into(), text(row.option)),
        ("values".into(), strings(row.values.split('|'))),
        ("default".into(), text(row.default)),
        ("destructive_as_default".into(), Json::Bool(row.destructive)),
        ("phase".into(), text(row.phase)),
        ("subphase".into(), text(row.subphase)),
        ("strength".into(), text(row.strength)),
        ("rewind".into(), text(row.rewind)),
        ("issue".into(), text(row.issue)),
        ("summary".into(), text(row.summary)),
        ("use_when".into(), text(row.use_when)),
        ("example".into(), text(row.example)),
        ("source_decompiler".into(), text(row.source_decompiler)),
        ("inspiration".into(), text(row.inspiration)),
        ("change_kind".into(), text(row.change_kind)),
        ("tier".into(), text(row.tier)),
        ("symptoms".into(), strings(row.symptoms.split('|'))),
        ("aliases".into(), strings(row.aliases.iter().copied())),
        ("engine".into(), text(engine)),
    ])
}

/// Every row of `engine`, or the one `option` names.
pub(crate) fn rows(engine: &dyn Engine, option: Option<&str>) -> Vec<Json> {
    let rows = engine.options().rows;
    match option {
        Some(name) => rows.iter().find(|r| answers(r, name)).map(|r| row_json(r, engine.name())).into_iter().collect(),
        None => rows.iter().map(|r| row_json(r, engine.name())).collect(),
    }
}

/// Whether `name` is one of `engine`'s option names or aliases.
pub(crate) fn knows(engine: &dyn Engine, name: &str) -> bool {
    engine.options().rows.iter().any(|r| answers(r, name))
}

/// `docs/options-<engine>.md`.
pub(crate) fn markdown(engine: &dyn Engine) -> String {
    let catalog = engine.options();
    let rows: Vec<MarkdownRow<'_>> = catalog
        .rows
        .iter()
        .map(|r| MarkdownRow {
            option: r.option,
            values: r.values,
            default: r.default,
            destructive: r.destructive,
            phase: r.phase,
            subphase: r.subphase,
            source_decompiler: r.source_decompiler,
            change_kind: r.change_kind,
            issue: r.issue,
            summary: r.summary,
            use_when: r.use_when,
            example: r.example,
            tier: r.tier,
            symptoms: r.symptoms,
            aliases: r.aliases,
        })
        .collect();
    let doc = MarkdownDoc { preamble: &catalog.preamble, sections: catalog.sections, tail: catalog.tail };
    emit_catalog_markdown_from(&rows, &doc)
}

/// What is wrong with `engine`'s table: names, tiers, values and defaults,
/// then the engine's own rules; and the line `--check` prints when nothing is.
pub(crate) fn check(engine: &dyn Engine) -> (Vec<String>, String) {
    let (label, prefix) = (engine.label(), format!("{}_", engine.name()));
    let rows = engine.options().rows;
    let mut problems = Vec::new();
    let mut names: BTreeSet<&str> = BTreeSet::new();
    for row in &rows {
        if !row.option.starts_with(&prefix) {
            problems.push(format!("{label} option {:?} is not spelled {prefix}<name>", row.option));
        }
        for name in std::iter::once(row.option).chain(row.aliases.iter().copied()) {
            if !names.insert(name) {
                problems.push(format!("{label} option name {name:?} is used twice"));
            }
            if crate::optname::is_known(name) {
                problems.push(format!("{label} option name {name:?} is also a native option"));
            }
        }
        if !matches!(row.tier, "transform" | "analysis" | "core") {
            problems.push(format!("{label} option {:?} has tier {:?}", row.option, row.tier));
        }
        if row.symptoms.split('|').any(|s| s.trim().is_empty()) {
            problems.push(format!("{label} option {:?} has an empty symptom", row.option));
        }
        if !row.example.contains(row.option) {
            problems.push(format!("{label} option {:?}: its example does not use it", row.option));
        }
        if let Err(e) = engine.resolve(row.option, row.default) {
            problems.push(format!("{label} option {:?}: its default does not resolve: {e}", row.option));
        }
    }
    problems.extend(engine.check());
    let ok = format!(
        "catalog OK: {} {label} engine options, each spelled {prefix}<name> with a resolvable default\n",
        rows.len()
    );
    (problems, ok)
}

#[cfg(all(test, feature = "jvm"))]
mod tests {
    use kuna_jvm::kuna_engine::Jvm;

    #[test]
    fn the_jvm_table_passes_its_own_check() {
        assert_eq!(super::check(&Jvm).0, Vec::<String>::new());
    }

    #[test]
    fn the_markdown_is_a_document() {
        let md = super::markdown(&Jvm);
        assert!(md.starts_with("# kuna JVM option catalog\n"));
        assert!(md.contains("### `jvm_decompile_finally`"));
        assert!(md.contains("## Symptom index"));
    }
}
