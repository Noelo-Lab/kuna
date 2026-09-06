//! CLI end-to-end gate for the **triage** surface of `kuna functions` /
//! `kuna decompile-all`: `--filter`, `--min-size`/`--max-size`,
//! `--reachable-from`, `--sort`/`--limit`, and `--summary`.
//!
//! The load-bearing property is the one the RE loop recorded as a blocker
//! ("unfiltered whole-binary JSON is impractically large for triage"): a caller
//! must be able to narrow the RUN, not the output, and must be able to orient
//! itself first with a call that cannot emit a megabyte. So the assertions are
//! about what the commands SELECT and what they COST, not about pseudocode.
//!
//! `fauxware` is the fixture because its call graph is the textbook shape:
//! `_start` reaches `main` only through the address it hands
//! `__libc_start_main`, and `main` reaches `authenticate`/`accepted`/`rejected`
//! and nothing else. A reachability query that gets either of those wrong is
//! visibly wrong here.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). When it is absent the command cannot build an architecture;
//! each test prints that and returns early — a specs-less CI is a visible skip,
//! never a false green.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", specs())
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// A missing `.sla` is a skip, not a failure (see the module header).
fn no_specs(stderr: &str, code: i32) -> bool {
    code != 0 && (stderr.contains("could not build an architecture") || stderr.contains(".sla"))
}

/// Every `0x…\t<name>` row of a plain `kuna functions` listing, as `(addr, name)`.
fn listing_rows(stdout: &str) -> Vec<(u64, String)> {
    stdout
        .lines()
        .filter(|l| l.starts_with("0x"))
        .filter_map(|l| {
            let mut f = l.split('\t');
            let addr = u64::from_str_radix(f.next()?.trim_start_matches("0x"), 16).ok()?;
            Some((addr, f.next()?.to_string()))
        })
        .collect()
}

fn listing_names(stdout: &str) -> Vec<String> {
    listing_rows(stdout).into_iter().map(|(_, name)| name).collect()
}

/// A top-level integer field of a `--json` document (`count`, `total`, …).
fn json_field(stdout: &str, key: &str) -> Option<u64> {
    let pat = format!("\"{key}\":");
    let i = stdout.find(&pat)? + pat.len();
    stdout[i..].trim_start().split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()
}

/// Every `"size": N` in document order.
fn json_sizes(stdout: &str) -> Vec<u64> {
    stdout
        .match_indices("\"size\":")
        .filter_map(|(i, m)| {
            stdout[i + m.len()..]
                .trim_start()
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()
        })
        .collect()
}

/// `--filter` is a regex over the name, and it matches aliases too — every
/// fauxware entry carries a generated `sub_<addr>` alias, so a caller that only
/// knows the generated spelling still selects the function.
#[test]
fn filter_selects_by_name_regex() {
    let (out, err, code) = run_kuna(&["functions", &fauxware(), "--filter", "auth|accept|reject"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert_eq!(listing_names(&out), vec!["authenticate", "accepted", "rejected"]);

    let (aliased, _, code) = run_kuna(&["functions", &fauxware(), "--filter", "^sub_400664$"]);
    assert_eq!(code, 0);
    assert_eq!(listing_names(&aliased), vec!["authenticate"], "an alias selects too");

    // A filter that matches nothing is an ANSWER, not a discovery failure: the
    // zero-discovery verdict must stay attached to discovery.
    let (empty, _, code) = run_kuna(&["functions", &fauxware(), "--filter", "no_such_symbol"]);
    assert_eq!(code, 0, "an empty selection is not a failed run");
    assert!(listing_rows(&empty).is_empty());
}

#[test]
fn a_malformed_filter_is_a_usage_error() {
    let (_, err, code) = run_kuna(&["functions", &fauxware(), "--filter", "["]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("invalid --filter regex"), "{err}");
}

/// `--min-size` / `--max-size` bound the inventory extent, and an inverted pair
/// is rejected before the binary is even loaded.
#[test]
fn size_bounds_are_inclusive_and_ordered() {
    let (out, err, code) = run_kuna(&["functions", &fauxware(), "--min-size", "100", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert!(json_sizes(&out).iter().all(|&s| s >= 100), "{out}");
    assert!(json_field(&out, "count").unwrap() < json_field(&out, "total").unwrap());

    let (bounded, _, code) =
        run_kuna(&["functions", &fauxware(), "--min-size", "32", "--max-size", "44", "--json"]);
    assert_eq!(code, 0);
    assert!(json_sizes(&bounded).iter().all(|&s| (32..=44).contains(&s)), "{bounded}");

    let (_, err, code) =
        run_kuna(&["functions", &fauxware(), "--min-size", "100", "--max-size", "10"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("greater than --max-size"), "{err}");
}

/// `--sort size` leads with the biggest (the triage order) and `--limit` caps
/// AFTER the sort, so "the three biggest functions" is one call.
#[test]
fn sort_and_limit_answer_the_three_biggest_question() {
    let (out, err, code) =
        run_kuna(&["functions", &fauxware(), "--sort", "size", "--limit", "3", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert_eq!(json_field(&out, "count"), Some(3));
    assert_eq!(json_field(&out, "total"), Some(21), "the count before narrowing");
    let sizes = json_sizes(&out);
    assert_eq!(sizes.len(), 3, "{out}");
    assert!(sizes.windows(2).all(|w| w[0] >= w[1]), "largest first: {sizes:?}");
    assert!(out.contains("\"name\": \"main\""), "{out}");

    let (by_name, _, code) = run_kuna(&["functions", &fauxware(), "--sort", "name"]);
    assert_eq!(code, 0);
    let names = listing_names(&by_name);
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);

    let (_, err, code) = run_kuna(&["functions", &fauxware(), "--sort", "entropy"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("unknown --sort key"), "{err}");
}

/// The call-graph question. `main` reaches exactly its own callees; `_start`
/// reaches `main` only through the address it hands `__libc_start_main`, which
/// is why an address-taken function pointer has to count as an edge.
#[test]
fn reachable_from_walks_the_call_graph() {
    let (out, err, code) = run_kuna(&["functions", &fauxware(), "--reachable-from", "main"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    let reached = listing_names(&out);
    for want in ["main", "authenticate", "accepted", "rejected", "strcmp", "puts"] {
        assert!(reached.iter().any(|n| n == want), "{want} missing from {reached:?}");
    }
    for unwanted in ["_start", "_init", "__libc_csu_init", "frame_dummy"] {
        assert!(
            !reached.iter().any(|n| n == unwanted),
            "{unwanted} is not reachable from main: {reached:?}"
        );
    }
    assert!(reached.len() < 21, "the query narrowed nothing: {reached:?}");

    let (from_start, _, code) = run_kuna(&["functions", &fauxware(), "--reachable-from", "_start"]);
    assert_eq!(code, 0);
    assert!(
        listing_names(&from_start).iter().any(|n| n == "main"),
        "the entry point reaches main through the pointer it passes __libc_start_main"
    );

    // An address operand resolves the same way a name does.
    let (by_addr, _, code) = run_kuna(&["functions", &fauxware(), "--reachable-from", "0x40071d"]);
    assert_eq!(code, 0);
    assert_eq!(listing_names(&by_addr), reached);

    let (_, err, code) = run_kuna(&["functions", &fauxware(), "--reachable-from", "no_such_fn"]);
    assert_eq!(code, 1, "{err}");
    assert!(err.contains("no function named"), "{err}");
}

/// `--summary` answers "where do I start" in a few hundred bytes: the entry
/// point, its reach, the unreferenced count, the size histogram, and the largest
/// functions — without emitting an inventory at all.
#[test]
fn summary_orients_without_emitting_the_inventory() {
    let (out, err, code) = run_kuna(&["functions", &fauxware(), "--summary", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    for key in [
        "\"summary\"",
        "\"entry\"",
        "\"reachable_from_entry\"",
        "\"no_callers\"",
        "\"size_buckets\"",
        "\"largest\"",
        "\"code_bytes\"",
    ] {
        assert!(out.contains(key), "{key} missing from {out}");
    }
    assert!(out.contains("\"name\": \"_start\""), "the ELF entry point: {out}");
    assert!(out.contains("\"name\": \"main\""), "main is the largest function: {out}");
    assert_eq!(json_field(&out, "total"), Some(21));
    assert!(!out.contains("\"code\":"), "a summary never carries pseudocode: {out}");

    let (full, _, code) = run_kuna(&["functions", &fauxware(), "--json"]);
    assert_eq!(code, 0);
    assert!(
        out.len() < full.len(),
        "the summary ({} B) must be cheaper than the inventory ({} B)",
        out.len(),
        full.len()
    );

    // `--limit` sizes the `largest` list.
    let (capped, _, code) =
        run_kuna(&["functions", &fauxware(), "--summary", "--json", "--limit", "2"]);
    assert_eq!(code, 0);
    assert_eq!(capped.matches("\"address_hex\":").count(), 3, "entry + 2 largest: {capped}");

    let (text, _, code) = run_kuna(&["functions", &fauxware(), "--summary"]);
    assert_eq!(code, 0);
    assert!(text.contains("size buckets:"), "{text}");
    assert!(text.contains("reachable from entry"), "{text}");
}

/// The point of the whole feature: `decompile-all` narrows the RUN, so a
/// selected batch is both far smaller and far cheaper than the whole binary.
#[test]
fn decompile_all_narrows_the_run_not_the_output() {
    let (out, err, code) = run_kuna(&[
        "decompile-all",
        &fauxware(),
        "--json",
        "--reachable-from",
        "main",
        "--sort",
        "size",
        "--limit",
        "2",
    ]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert_eq!(json_field(&out, "count"), Some(2));
    // `total` is the pre-narrowing target count, so a capped answer can never be
    // mistaken for the whole program.
    assert!(json_field(&out, "total").unwrap() > 2, "{out}");
    assert!(out.contains("\"name\": \"main\""), "{out}");
    assert!(out.contains("\"name\": \"authenticate\""), "{out}");
    assert!(!out.contains("\"name\": \"_start\""), "{out}");

    let (whole, _, code) = run_kuna(&["decompile-all", &fauxware(), "--json"]);
    assert_eq!(code, 0);
    assert!(
        out.len() * 2 < whole.len(),
        "narrowed {} B vs whole-binary {} B",
        out.len(),
        whole.len()
    );
    // An UNNARROWED run is untouched, `total` included: the decbench backend and
    // `kuna decompile --json` read that document.
    assert!(!whole.contains("\"total\":"), "unfiltered documents keep their schema");

    let (filtered, _, code) =
        run_kuna(&["decompile-all", &fauxware(), "--json", "--filter", "^authenticate$"]);
    assert_eq!(code, 0);
    assert_eq!(json_field(&filtered, "count"), Some(1));
    assert!(filtered.contains("\"code\":"), "a narrowed run still decompiles: {filtered}");
}

/// An ARM `e_entry` and a symbol `st_value` both carry the Thumb mode bit while
/// the inventory records the even entry, so the summary and `--reachable-from`
/// have to resolve an odd address THROUGH the inventory or answer nothing at all.
#[test]
fn an_arm_thumb_entry_resolves_to_its_even_address() {
    let arm = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/entrymain_arm")
        .to_str()
        .unwrap()
        .to_string();
    let (out, err, code) = run_kuna(&["functions", &arm, "--summary", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"address_hex\": \"0x3dc\""), "the even entry: {out}");
    let reachable = json_field(&out, "reachable_from_entry");
    assert!(reachable.is_some_and(|n| n > 0), "the entry must reach something: {out}");

    let (odd, _, code) = run_kuna(&["functions", &arm, "--reachable-from", "0x3dd"]);
    assert_eq!(code, 0);
    let (even, _, _) = run_kuna(&["functions", &arm, "--reachable-from", "0x3dc"]);
    assert_eq!(listing_names(&odd), listing_names(&even), "both spellings, one answer");
    assert!(!listing_rows(&odd).is_empty(), "{odd}");
}

/// A Mach-O `LC_MAIN` states its entry as a `__TEXT`-relative FILE OFFSET where
/// every other format states a VMA, so the summary reported `0x5b0` -- an address
/// that names no function, which then made `reachable_from_entry` null and left
/// the one orientation field an agent starts from useless.
#[test]
fn a_macho_lc_main_entry_is_reported_as_a_vma() {
    let macho = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/macho_stripped_main")
        .to_str()
        .unwrap()
        .to_string();
    let (out, err, code) = run_kuna(&["functions", &macho, "--summary", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"address_hex\": \"0x1000005b0\""), "__TEXT.vmaddr + entryoff: {out}");
    assert!(!out.contains("\"address_hex\": \"0x5b0\""), "the raw entryoff must be gone: {out}");
    assert!(out.contains("\"name\": \"main\""), "the entry names a function: {out}");
    let reachable = json_field(&out, "reachable_from_entry");
    assert!(reachable.is_some_and(|n| n > 0), "the entry must reach something: {out}");
}

/// The zero-discovery verdict belongs to DISCOVERY. A narrowed run on an image
/// that yielded nothing must still fail loudly — the packer diagnosis is the one
/// thing the caller can act on, and a filter must not be able to swallow it.
#[test]
fn a_narrowed_run_on_a_packed_image_still_fails_loudly() {
    let packed = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/upx_packed_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let (out, err, code) = run_kuna(&["functions", &packed, "--filter", "main", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla under {} ({err})", specs());
        return;
    }
    assert_eq!(code, 1, "{err}");
    assert!(out.contains("UPX-packed"), "{out}");
    assert!(err.contains("kuna unpack"), "{err}");

    let (summary, _, code) = run_kuna(&["functions", &packed, "--summary", "--json"]);
    assert_eq!(code, 1, "a summary of nothing is still a failed run");
    assert!(summary.contains("UPX-packed"), "{summary}");
}

/// The triage flags belong to the two surfaces that act on them. Offering them
/// where they are ignored would be worse than not offering them.
#[test]
fn triage_flags_are_not_offered_where_they_do_nothing() {
    let (_, err, code) = run_kuna(&["decompile-project", &fauxware(), "--filter", "main"]);
    assert_eq!(code, 2, "{err}");
    assert!(err.contains("unknown option --filter"), "{err}");
}
