//! `kuna docs` — the reference manual, compiled into the binary.
//!
//! A release binary is frequently all a driving agent has: no repo checkout, no
//! `docs/` tree, no network.  kuna's control surface is its documentation (the
//! option catalog *is* the product), so shipping the binary without it ships
//! something unusable.  Every topic below is therefore pulled in with
//! `include_str!` at compile time and answered out of the binary's own rodata —
//! `kuna docs` opens no file and resolves no path, so it behaves identically in
//! an empty directory and in the repo.
//!
//! Compiling the bytes in also fixes the freshness hazard the other direction:
//! `docs/options.md` is generated (`kuna catalog --markdown > docs/options.md`)
//! and gated by `kuna-decomp/tests/options_md_fresh.rs`, so a stale catalog
//! cannot reach a binary that is rebuilt — and `tests/docs_cli.rs` asserts the
//! embedded bytes still equal the on-disk file, which is what makes that true.

use std::fmt::Write as _;

use crate::jsonfmt::{self, Json};
use crate::output;

/// One embedded document.  `body` is the source file verbatim, so
/// `kuna docs <topic>` is byte-identical to `cat docs/<topic>.md`.
pub struct Doc {
    pub topic: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    /// Repo-relative source, reported in `--all` banners and by the freshness test.
    pub path: &'static str,
    pub body: &'static str,
}

/// Ordered by what an agent reaches for first, not alphabetically: the command
/// reference, then the option catalog it drives, then the working rules, then
/// the two models those rules refer to.
pub const DOCS: &[Doc] = &[
    Doc {
        topic: "cli",
        title: "The `kuna` CLI reference",
        summary: "Every subcommand, flag, exit code and JSON schema of the kuna binary.",
        path: "docs/cli.md",
        body: include_str!("../../../../docs/cli.md"),
    },
    Doc {
        topic: "options",
        title: "kuna option catalog",
        summary: "Generated catalog of every --option, by tier, with a symptom index from bad output to the flip that fixes it.",
        path: "docs/options.md",
        body: include_str!("../../../../docs/options.md"),
    },
    Doc {
        topic: "agents",
        title: "AGENTS.md — working in and with kuna",
        summary: "The repo rulebook: what kuna is, how it is laid out, the build/test gates, and the doc map.",
        path: "docs/agents.md",
        body: include_str!("../../../../docs/agents.md"),
    },
    Doc {
        topic: "phases",
        title: "Decompiler phases",
        summary: "The P0-P9 phase model on one screen: what each phase computes and where the feedback edges run.",
        path: "docs/phases.md",
        body: include_str!("../../../../docs/phases.md"),
    },
    Doc {
        topic: "modes",
        title: "Decompiler modes",
        summary: "The --mode auto|reliable|aggressive|fast presets and the input-size thresholds auto selects on.",
        path: "docs/modes.md",
        body: include_str!("../../../../docs/modes.md"),
    },
];

const USAGE: &str = "usage: kuna docs [<topic>] [--json] [--all]\n\
                     \n\
                     kuna docs                list the embedded topics, one per line\n\
                     kuna docs <topic>        print that document to stdout\n\
                     kuna docs --json         the topic list as [{topic,title,summary,bytes}]\n\
                     kuna docs --all          every document concatenated, for a context window";

pub fn run(argv: &[String]) -> i32 {
    let mut topic: Option<&str> = None;
    let mut json = false;
    let mut all = false;

    for a in argv {
        match a.as_str() {
            "--json" => json = true,
            "--all" => all = true,
            "-h" | "--help" => {
                eprintln!("{USAGE}");
                return 0;
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                eprintln!("{USAGE}");
                return 2;
            }
            other => {
                if topic.is_some() {
                    eprintln!("error: unexpected argument {other:?}");
                    return 2;
                }
                topic = Some(other);
            }
        }
    }

    if json && all {
        eprintln!("error: --json and --all are mutually exclusive");
        return 2;
    }
    if topic.is_some() && (json || all) {
        eprintln!("error: a topic cannot be combined with --json or --all");
        return 2;
    }

    if all {
        return output::emit_with_status(&render_all(), 0);
    }
    if json {
        return output::emit_with_status(&render_json(), 0);
    }
    match topic {
        Some(name) => match lookup(name) {
            Some(doc) => output::emit_with_status(doc.body, 0),
            None => {
                eprintln!("error: no embedded doc named {name:?}");
                eprintln!("known topics: {}", topic_names().join(", "));
                2
            }
        },
        None => {
            let status = output::emit_with_status(&render_list(), 0);
            eprintln!("\nread one with `kuna docs <topic>`; all of it with `kuna docs --all`");
            status
        }
    }
}

/// Resolve a caller-supplied topic.  Agents reach for the file name they saw in
/// a doc map (`options.md`, `docs/options.md`) as often as for the bare topic,
/// and singular/plural is a coin flip, so all of those land on the same page
/// rather than on an error.
pub fn lookup(name: &str) -> Option<&'static Doc> {
    let lowered = name.to_ascii_lowercase();
    let trimmed =
        lowered.trim_start_matches("./").trim_start_matches("docs/").trim_end_matches(".md");
    DOCS.iter().find(|d| {
        d.topic == trimmed
            || d.topic.trim_end_matches('s') == trimmed.trim_end_matches('s')
            || (d.topic == "cli" && matches!(trimmed, "command" | "commands" | "reference"))
    })
}

pub fn topic_names() -> Vec<&'static str> {
    DOCS.iter().map(|d| d.topic).collect()
}

pub fn render_list() -> String {
    let width = DOCS.iter().map(|d| d.topic.len()).max().unwrap_or(0);
    let mut out = String::new();
    for d in DOCS {
        let _ = writeln!(out, "{:width$}  {:>8}  {}", d.topic, human_size(d.body.len()), d.summary);
    }
    out
}

pub fn render_json() -> String {
    let items: Vec<Json> = DOCS
        .iter()
        .map(|d| {
            Json::Object(vec![
                ("topic".into(), Json::Str(d.topic.into())),
                ("title".into(), Json::Str(d.title.into())),
                ("summary".into(), Json::Str(d.summary.into())),
                ("bytes".into(), Json::Number(d.body.len().to_string())),
            ])
        })
        .collect();
    format!("{}\n", jsonfmt::dumps_indent2(&Json::Array(items)))
}

/// One stream of every document.  The banners are HTML comments so the result is
/// still valid markdown, and a reader (human or model) can still see where one
/// document ends and the next begins.
pub fn render_all() -> String {
    let total: usize = DOCS.iter().map(|d| d.body.len()).sum();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "<!-- kuna docs --all: {} documents, {} total; topics: {} -->\n",
        DOCS.len(),
        human_size(total),
        topic_names().join(", ")
    );
    for d in DOCS {
        let _ = writeln!(
            out,
            "<!-- ===== kuna docs: {} — {} ({}) ===== -->\n",
            d.topic,
            d.path,
            human_size(d.body.len())
        );
        out.push_str(d.body.trim_end_matches('\n'));
        out.push_str("\n\n");
    }
    out
}

pub fn human_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else {
        format!("{} KB", bytes.div_ceil(1024))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_topic_is_embedded_and_nonempty() {
        assert!(DOCS.len() >= 5);
        for d in DOCS {
            assert!(!d.body.is_empty(), "{} embedded empty", d.topic);
            assert!(d.body.starts_with('#'), "{} is not the markdown file", d.topic);
        }
    }

    #[test]
    fn topics_are_unique_and_resolvable() {
        let mut seen: Vec<&str> = Vec::new();
        for d in DOCS {
            assert!(!seen.contains(&d.topic), "duplicate topic {}", d.topic);
            seen.push(d.topic);
            assert_eq!(lookup(d.topic).unwrap().topic, d.topic);
        }
    }

    #[test]
    fn lookup_forgives_the_shapes_agents_type() {
        for name in ["options", "option", "OPTIONS", "options.md", "docs/options.md"] {
            assert_eq!(lookup(name).unwrap().topic, "options", "{name}");
        }
        assert_eq!(lookup("commands").unwrap().topic, "cli");
        assert!(lookup("nonesuch").is_none());
    }

    #[test]
    fn all_carries_every_body_verbatim() {
        let all = render_all();
        for d in DOCS {
            assert!(all.contains(d.body.trim_end_matches('\n')), "{} missing", d.topic);
        }
    }
}
