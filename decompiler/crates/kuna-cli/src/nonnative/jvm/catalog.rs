//! The JVM engine's option rows in `kuna catalog`: JSON rows in the native
//! row shape plus `aliases` and `engine`, the generated `docs/options-jvm.md`,
//! and the `--check` rules for the table.

use std::collections::BTreeSet;

use kuna_decomp::kuna_phases::{emit_catalog_markdown_from, MarkdownDoc, MarkdownRow};
use kuna_jvm::kuna_options::{self, JvmOption};

use crate::jsonfmt::Json;

fn strings(items: impl Iterator<Item = &'static str>) -> Json {
    Json::Array(items.map(|s| Json::Str(s.to_string())).collect())
}

/// One row, keyed as `emit_settable_json` keys a native row.
fn row_json(row: &JvmOption) -> Json {
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
        ("symptoms".into(), strings(row.symptom_list().into_iter())),
        ("aliases".into(), strings(row.aliases.iter().copied())),
        ("engine".into(), text("jvm")),
    ])
}

/// Every JVM row, or the one `option` names (its `jvm_` name or an alias).
pub(crate) fn rows(option: Option<&str>) -> Vec<Json> {
    match option {
        Some(name) => kuna_options::find(name).map(row_json).into_iter().collect(),
        None => kuna_options::rows().iter().map(row_json).collect(),
    }
}

/// Whether `name` is a JVM option name or alias.
pub(crate) fn knows(name: &str) -> bool {
    kuna_options::find(name).is_some()
}

/// `docs/options-jvm.md`.
pub(crate) fn markdown() -> String {
    let rows: Vec<MarkdownRow<'_>> = kuna_options::rows()
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
    let mut preamble = String::from("# kuna JVM option catalog\n\n");
    preamble.push_str(
        "**Generated** by `kuna catalog --markdown --engine jvm` from \
         `decompiler/nonnative/kuna-jvm/options.toml` -- do not edit by hand; edit the \
         table row and regenerate.\n\n",
    );
    preamble.push_str(
        "These options steer the JVM engine (`kuna-jvm`, a port of Vineflower 1.12.0), \
         which decompiles class files, jars and directories of classes to Java, and \
         classes compiled from Kotlin to Kotlin. They apply to \
         those inputs only, as the options of `docs/options.md` apply to native inputs \
         only. Set any of them per run on `decompile`, `decompile-all`, \
         `decompile-project` and `functions`: `kuna decompile app.jar com.ex.Main \
         --option <name> <value>` (repeatable). Every Vineflower option also answers \
         to Vineflower's long and three-letter names (its `aliases`). Booleans take \
         `on|off` (also `1|0|true|false`). Defaults are Vineflower 1.12.0's, its \
         bundled plugins' included.\n\n",
    );
    preamble.push_str("Vineflower keys kuna sets itself, so they are not options:\n\n");
    for owned in kuna_options::driver_owned() {
        preamble.push_str(&format!("- `{}` -- {}\n", owned.vineflower, owned.kuna));
    }
    preamble.push_str("\nThree tiers:\n\n");
    preamble.push_str(
        "- **Toggleable transforms** -- restructure, remove, or insert code; the \
         on/off control surface.\n",
    );
    preamble.push_str(
        "- **Input & analysis** -- what the engine reads: the class universe, \
         libraries, the JDK.\n",
    );
    preamble.push_str(
        "- **Rendering defaults** -- how the text is written: Java or Kotlin, and how it \
         is spelled.\n\n",
    );
    emit_catalog_markdown_from(
        &rows,
        &MarkdownDoc {
            preamble: &preamble,
            sections: [
                (
                    "transform",
                    "## Toggleable transforms",
                    "Each of these restructures, removes or inserts code, so each stays flippable.",
                ),
                (
                    "analysis",
                    "## Input & analysis",
                    "What the engine loads and infers before any class is written.",
                ),
                (
                    "core",
                    "## Rendering defaults",
                    "How the text is written: its language (Java, or Kotlin for Kotlin classes), \
                     formatting, literals, comments.",
                ),
            ],
            tail: "## Programmatic use\n\n```bash\n# discover (machine-readable; includes tier, symptoms, aliases):\n\
                   kuna catalog --engine jvm --json\n\n# transforms only:\nkuna catalog --engine jvm --tier transform\n\n\
                   # decompile with an option flipped (repeatable):\n\
                   kuna decompile app.jar com.ex.Main --option jvm_decompile_finally off\n```\n",
        },
    )
}

/// What is wrong with the JVM table: names, tiers, values and defaults.
pub(crate) fn check() -> Vec<String> {
    let mut problems = Vec::new();
    let mut names: BTreeSet<&str> = BTreeSet::new();
    for row in kuna_options::rows() {
        if !row.option.starts_with("jvm_") {
            problems.push(format!("JVM option {:?} is not spelled jvm_<name>", row.option));
        }
        for name in std::iter::once(row.option).chain(row.aliases.iter().copied()) {
            if !names.insert(name) {
                problems.push(format!("JVM option name {name:?} is used twice"));
            }
            if crate::optname::is_known(name) {
                problems.push(format!("JVM option name {name:?} is also a native option"));
            }
        }
        if !matches!(row.tier, "transform" | "analysis" | "core") {
            problems.push(format!("JVM option {:?} has tier {:?}", row.option, row.tier));
        }
        if row.symptom_list().iter().any(|s| s.trim().is_empty()) {
            problems.push(format!("JVM option {:?} has an empty symptom", row.option));
        }
        if !row.example.contains(row.option) {
            problems.push(format!("JVM option {:?}: its example does not use it", row.option));
        }
        if let Err(e) = kuna_options::resolve(row.option, row.default) {
            problems.push(format!("JVM option {:?}: its default does not resolve: {e}", row.option));
        }
    }
    for owned in kuna_options::driver_owned() {
        if kuna_options::find(owned.vineflower).is_some() {
            problems.push(format!("driver-owned key {:?} is also an option row", owned.vineflower));
        }
    }
    problems
}

pub(crate) fn count() -> usize {
    kuna_options::rows().len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_jvm_table_passes_its_own_check() {
        assert_eq!(super::check(), Vec::<String>::new());
    }

    #[test]
    fn the_markdown_is_a_document() {
        let md = super::markdown();
        assert!(md.starts_with("# kuna JVM option catalog\n"));
        assert!(md.contains("### `jvm_decompile_finally`"));
        assert!(md.contains("## Symptom index"));
    }
}
