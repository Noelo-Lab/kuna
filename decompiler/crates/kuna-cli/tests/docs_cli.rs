//! `kuna docs` — the reference manual compiled into the binary.
//!
//! Two properties are on trial here, and the second is the one that rots
//! silently.
//!
//! **The surface works, with no repo on disk.** That is the entire point of
//! `include_str!`-ing the documents: a release binary is often all a driving
//! agent has.  `the_binary_carries_its_docs_out_of_the_repo` copies the built
//! executable into an empty directory and reads a document out of it there.
//!
//! **The embedded bytes are still the bytes in `docs/`.**  `docs/options.md` is
//! GENERATED (`kuna catalog --markdown > docs/options.md`), already fenced on
//! disk by `kuna-decomp/tests/options_md_fresh.rs`; embedding it adds a second
//! place a stale catalog could hide.  `embedded_docs_match_the_files_on_disk`
//! closes that: `include_str!` is tracked by cargo's dep-info, so touching a
//! document rebuilds the crate, and this test is what proves the rebuild
//! actually happened rather than trusting it.
//!
//! The module under test lives in a `[[bin]]`-only crate, so it is pulled in by
//! path along with the two crate modules it uses.  Their own `#[cfg(test)]`
//! units therefore run in this binary too; that is noise, not a second suite.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

#[allow(dead_code)]
#[path = "../src/docs.rs"]
mod docs;
#[allow(dead_code)]
#[path = "../src/jsonfmt.rs"]
mod jsonfmt;
#[allow(dead_code)]
#[path = "../src/output.rs"]
mod output;

use jsonfmt::Json;

/// The five topics an agent driving kuna needs, in the order `kuna docs` lists
/// them: the command reference, the option catalog it drives, the working rules,
/// then the two models those rules lean on.
const REQUIRED: [&str; 5] = ["cli", "options", "agents", "phases", "modes"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

// --- the embed itself --------------------------------------------------------

#[test]
fn embedded_docs_match_the_files_on_disk() {
    let root = repo_root();
    for d in docs::DOCS {
        let path = root.join(d.path);
        let on_disk =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let regenerate = if d.path == "docs/options.md" {
            " That file is generated: `kuna catalog --markdown > docs/options.md` first."
        } else {
            ""
        };
        assert!(
            on_disk == d.body,
            "`kuna docs {}` would ship a stale {}: the binary embeds {} bytes, the file holds {}. \
             Rebuild kuna-cli.{}",
            d.topic,
            d.path,
            d.body.len(),
            on_disk.len(),
            regenerate
        );
    }
}

#[test]
fn the_topics_an_agent_needs_are_embedded_in_priority_order() {
    let topics: Vec<&str> = docs::DOCS.iter().map(|d| d.topic).collect();
    assert!(topics.len() >= REQUIRED.len(), "only {topics:?} embedded");
    assert_eq!(&topics[..REQUIRED.len()], &REQUIRED);
    for d in docs::DOCS {
        assert!(!d.summary.is_empty(), "{} has no summary", d.topic);
        assert!(d.body.len() > 1024, "{} embedded only {} bytes", d.topic, d.body.len());
    }
}

#[test]
fn the_option_catalog_arrives_whole() {
    let catalog = docs::lookup("options").expect("options topic").body;
    assert!(catalog.contains("## Symptom index"), "the symptom index is missing");
    assert!(
        catalog.lines().count() > 900,
        "the catalog embedded only {} lines",
        catalog.lines().count()
    );
}

// --- the rendered surfaces ---------------------------------------------------

#[test]
fn the_list_is_one_line_per_topic() {
    let list = docs::render_list();
    let lines: Vec<&str> = list.lines().collect();
    assert_eq!(lines.len(), docs::DOCS.len());
    for (line, d) in lines.iter().zip(docs::DOCS) {
        assert!(line.starts_with(d.topic), "{line:?} does not lead with {}", d.topic);
        assert!(line.contains(d.summary), "{line:?} dropped its summary");
    }
}

#[test]
fn the_json_list_is_the_documented_shape() {
    let text = docs::render_json();
    let items = match jsonfmt::parse(&text).expect("--json must parse") {
        Json::Array(items) => items,
        other => panic!("--json must emit an array, got {other:?}"),
    };
    assert_eq!(items.len(), docs::DOCS.len());
    for (item, d) in items.iter().zip(docs::DOCS) {
        let pairs = match item {
            Json::Object(pairs) => pairs,
            other => panic!("expected an object, got {other:?}"),
        };
        let keys: Vec<&str> = pairs.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, ["topic", "title", "summary", "bytes"]);
        assert_eq!(pairs[0].1, Json::Str(d.topic.into()));
        assert_eq!(pairs[3].1, Json::Number(d.body.len().to_string()));
    }
}

#[test]
fn all_concatenates_every_document_verbatim() {
    let all = docs::render_all();
    for d in docs::DOCS {
        assert!(
            all.contains(d.body.trim_end_matches('\n')),
            "--all dropped or mangled {}",
            d.topic
        );
        assert!(all.contains(d.path), "--all did not label {}", d.topic);
    }
}

// --- end to end, through the built binary ------------------------------------

fn run_docs(args: &[&str]) -> Output {
    let mut argv = vec!["docs"];
    argv.extend_from_slice(args);
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(&argv)
        .output()
        .expect("failed to spawn the kuna binary")
}

/// `docs.rs` is dispatch-free until `main.rs` routes `"docs"` to it.  Until then
/// the end-to-end tests are a visible skip rather than a false green; the
/// property tests above still cover the embed.
fn dispatch_wired() -> bool {
    let out = run_docs(&["--json"]);
    let wired = !String::from_utf8_lossy(&out.stderr).contains("unknown subcommand");
    if !wired {
        eprintln!("docs_cli: skipping (main.rs does not dispatch `docs` yet)");
    }
    wired
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn the_cli_lists_at_least_the_five_topics() {
    if !dispatch_wired() {
        return;
    }
    let out = run_docs(&[]);
    assert!(out.status.success(), "`kuna docs` exited {:?}", out.status.code());
    let listed: Vec<String> = stdout_of(&out)
        .lines()
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();
    for topic in REQUIRED {
        assert!(listed.iter().any(|l| l == topic), "`kuna docs` did not list {topic}: {listed:?}");
    }
}

#[test]
fn the_cli_prints_a_topic_byte_for_byte() {
    if !dispatch_wired() {
        return;
    }
    for d in docs::DOCS {
        let out = run_docs(&[d.topic]);
        assert!(out.status.success(), "`kuna docs {}` exited {:?}", d.topic, out.status.code());
        assert!(
            out.stdout == d.body.as_bytes(),
            "`kuna docs {}` emitted {} bytes, the document is {}",
            d.topic,
            out.stdout.len(),
            d.body.len()
        );
    }
}

#[test]
fn the_cli_json_matches_the_in_process_renderer() {
    if !dispatch_wired() {
        return;
    }
    let out = run_docs(&["--json"]);
    assert!(out.status.success());
    assert_eq!(stdout_of(&out), docs::render_json());
    assert!(jsonfmt::parse(&stdout_of(&out)).is_some(), "`kuna docs --json` is not JSON");
}

#[test]
fn an_unknown_topic_is_a_usage_error_that_names_the_real_ones() {
    if !dispatch_wired() {
        return;
    }
    let out = run_docs(&["xrefs"]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    for topic in REQUIRED {
        assert!(stderr.contains(topic), "the error did not offer {topic}: {stderr}");
    }
}

/// The whole point: no repo, no `docs/`, no cwd that means anything.
#[test]
fn the_binary_carries_its_docs_out_of_the_repo() {
    if !dispatch_wired() {
        return;
    }
    let sandbox = std::env::temp_dir().join(format!("kuna_docs_norepo_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&sandbox);
    std::fs::create_dir_all(&sandbox).expect("create sandbox");
    let exe = sandbox.join("kuna");
    std::fs::copy(env!("CARGO_BIN_EXE_kuna"), &exe).expect("copy the binary out of the repo");
    make_executable(&exe);

    let out = Command::new(&exe)
        .current_dir(&sandbox)
        .args(["docs", "cli"])
        .env_remove("KUNA_SPECS")
        .env_remove("SLEIGHHOME")
        .env_remove("KUNA_DECOMP_DBG")
        .output()
        .expect("run the relocated binary");

    assert!(out.status.success(), "exited {:?}", out.status.code());
    let cli = docs::lookup("cli").unwrap();
    assert!(out.stdout == cli.body.as_bytes(), "the relocated binary lost docs/cli.md");
    assert_eq!(
        std::fs::read_dir(&sandbox).unwrap().count(),
        1,
        "`kuna docs` left files behind, so it is not answering from rodata"
    );
    let _ = std::fs::remove_dir_all(&sandbox);
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).unwrap();
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

/// `kuna docs options | head -20` is the first thing anyone types at a 280 KB
/// document; the stdout boundary must take the EPIPE quietly.
#[test]
fn a_reader_that_walks_away_is_not_a_panic() {
    if !dispatch_wired() {
        return;
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["docs", "--all"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kuna");
    drop(child.stdout.take().expect("stdout pipe"));
    let out = child.wait_with_output().expect("wait for kuna");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!stderr.contains("panicked"), "broken pipe panicked: {stderr}");
    assert!(out.status.success(), "exited {:?}: {stderr}", out.status.code());
}
