//! Unit tests for the XML datatest runner (`testfunction.rs`).
//!
//! Two layers:
//! 1. `FunctionTestProperty` in isolation — the min/max counting and the
//!    multi-line sequential matching state machine (the LOSS-007 regex engine).
//! 2. The full pipeline — `FunctionTestCollection`/`run_test_files` over real
//!    datatest XMLs from `decompiler/datatests/`, asserting the exact
//!    `Success`/`FAIL`/`Total tests applied`/file-error grammar the Python
//!    harness greps.

use super::*;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// FunctionTestProperty: min/max counting.
// ---------------------------------------------------------------------------

/// Feed a property a sequence of output lines (as `run_tests` does) and return
/// whether it passed.
fn run_property(prop: &mut FunctionTestProperty, lines: &[&str]) -> bool {
    prop.start_test();
    for l in lines {
        prop.process_line(l);
    }
    prop.end_test()
}

#[test]
fn single_line_min1_max1_exact() {
    let mut p = FunctionTestProperty::from_parts("eq", 1, 1, &[r"return a0 == 3;"]);
    assert!(run_property(&mut p, &["int foo(void)", "{", "  return a0 == 3;", "}"]));
}

#[test]
fn single_line_min1_max1_zero_matches_fails() {
    let mut p = FunctionTestProperty::from_parts("eq", 1, 1, &[r"return a0 == 3;"]);
    assert!(!run_property(&mut p, &["int foo(void)", "{", "  return 0;", "}"]));
}

#[test]
fn min0_max0_must_not_match() {
    // The "negative" assertion form (lzcount #2 / boolless #2): the pattern must
    // appear ZERO times.
    let mut p = FunctionTestProperty::from_parts("noshift", 0, 0, &[r"<< 7"]);
    assert!(run_property(&mut p, &["a = b + c;", "d = e;"])); // absent -> pass
    let mut p2 = FunctionTestProperty::from_parts("noshift", 0, 0, &[r"<< 7"]);
    assert!(!run_property(&mut p2, &["a = b << 7;"])); // present -> fail
}

#[test]
fn min_greater_than_one_counts() {
    let mut p = FunctionTestProperty::from_parts("calls", 2, 3, &[r"foo\("]);
    // Exactly 2 matches -> in [2,3] -> pass.
    assert!(run_property(&mut p, &["x = foo(1);", "y = bar();", "z = foo(2);"]));
    // 4 matches -> above max -> fail.
    let mut p2 = FunctionTestProperty::from_parts("calls", 2, 3, &[r"foo\("]);
    assert!(!run_property(&mut p2, &["foo(1);", "foo(2);", "foo(3);", "foo(4);"]));
    // 1 match -> below min -> fail.
    let mut p3 = FunctionTestProperty::from_parts("calls", 2, 3, &[r"foo\("]);
    assert!(!run_property(&mut p3, &["foo(1);"]));
}

#[test]
fn count_caps_at_match_run() {
    // Multiple matches on a single line are NOT counted -- regex_search finds at
    // most one per processLine call (C++ semantics).
    let mut p = FunctionTestProperty::from_parts("one", 1, 1, &[r"x"]);
    assert!(run_property(&mut p, &["x x x x"])); // one processLine -> one count
}

// ---------------------------------------------------------------------------
// FunctionTestProperty: multi-line sequential matching.
// ---------------------------------------------------------------------------

#[test]
fn multiline_consecutive_match() {
    // A two-line pattern matches only when the two regexes hit consecutive lines.
    let mut p = FunctionTestProperty::from_parts("two", 1, 1, &[r"^if ", r"^  return"]);
    assert!(run_property(&mut p, &["if (cond)", "  return 1;"]));
}

#[test]
fn multiline_nonconsecutive_does_not_match() {
    let mut p = FunctionTestProperty::from_parts("two", 1, 1, &[r"^if ", r"^  return"]);
    // A line in between breaks the sequence.
    assert!(!run_property(&mut p, &["if (cond)", "  x = 1;", "  return 1;"]));
}

#[test]
fn multiline_abort_then_restart_on_same_line() {
    // The C++ quirk: when a mid-pattern line fails, abort AND immediately re-test
    // that same line against the FIRST regex.  Pattern: ["A", "B"].  Stream:
    // "A" (patnum->1), "A" (not "B", abort; re-test "A" matches first -> patnum->1),
    // "B" (matches -> count).  Net: one full match.
    let mut p = FunctionTestProperty::from_parts("ab", 1, 1, &[r"^A$", r"^B$"]);
    assert!(run_property(&mut p, &["A", "A", "B"]));
}

#[test]
fn multiline_two_full_matches() {
    let mut p = FunctionTestProperty::from_parts("ab", 2, 2, &[r"^A$", r"^B$"]);
    assert!(run_property(&mut p, &["A", "B", "A", "B"]));
}

#[test]
fn multiline_three_lines() {
    let mut p =
        FunctionTestProperty::from_parts("abc", 1, 1, &[r"^one$", r"^two$", r"^three$"]);
    assert!(run_property(&mut p, &["zero", "one", "two", "three", "four"]));
    let mut p2 =
        FunctionTestProperty::from_parts("abc", 1, 1, &[r"^one$", r"^two$", r"^three$"]);
    // Break before the third line -> no match.
    assert!(!run_property(&mut p2, &["one", "two", "TWO", "three"]));
}

// ---------------------------------------------------------------------------
// restore_xml: pattern parsing (whitespace strip, multi-line split).
// ---------------------------------------------------------------------------

fn parse_stringmatch(xml: &str) -> FunctionTestProperty {
    let mut store = DocumentStorage::new();
    let root = store.parse_document(xml.as_bytes()).unwrap().get_root().clone();
    FunctionTestProperty::restore_xml(&root).unwrap()
}

#[test]
fn restore_single_line_attrs_and_pattern() {
    let p = parse_stringmatch(
        r#"<stringmatch name="Leading zeros count #1" min="1" max="1">return a0 == 3;</stringmatch>"#,
    );
    assert_eq!(p.get_name(), "Leading zeros count #1");
    assert_eq!(p.minimum_match, 1);
    assert_eq!(p.maximum_match, 1);
    assert_eq!(p.pattern.len(), 1);
    assert!(run_property(&mut parse_stringmatch(
        r#"<stringmatch name="x" min="1" max="1">return a0 == 3;</stringmatch>"#,
    ), &["  return a0 == 3;"]));
}

#[test]
fn restore_strips_leading_whitespace_per_line() {
    // Leading spaces/tabs in the pattern content are stripped (each line).
    let p = parse_stringmatch(
        "<stringmatch name=\"x\" min=\"1\" max=\"1\">    return 0;</stringmatch>",
    );
    assert_eq!(p.pattern.len(), 1);
    // The compiled pattern is "return 0;" (anchored search) -> matches a line
    // that contains it.
    let mut q = parse_stringmatch(
        "<stringmatch name=\"x\" min=\"1\" max=\"1\">    return 0;</stringmatch>",
    );
    assert!(run_property(&mut q, &["  return 0;"]));
}

#[test]
fn restore_multiline_pattern_splits_on_newline() {
    // A `\n` in the content makes a multi-line regex (two line-patterns).
    let xml = "<stringmatch name=\"ml\" min=\"1\" max=\"1\">if \\(cond\\)\n  return 1;</stringmatch>";
    let p = parse_stringmatch(xml);
    assert_eq!(p.pattern.len(), 2);
    let mut q = parse_stringmatch(xml);
    assert!(run_property(&mut q, &["if (cond)", "  return 1;"]));
    let mut r = parse_stringmatch(xml);
    assert!(!run_property(&mut r, &["if (cond)", "x = 1;", "  return 1;"]));
}

#[test]
fn restore_trailing_newline_makes_no_empty_pattern() {
    // A trailing newline / all-whitespace tail does NOT add an empty regex (the
    // C++ pos-skip-whitespace + break).
    let xml = "<stringmatch name=\"t\" min=\"1\" max=\"1\">alpha\n   </stringmatch>";
    let p = parse_stringmatch(xml);
    assert_eq!(p.pattern.len(), 1);
}

#[test]
fn restore_html_escaped_pattern() {
    // The corpus uses XML entity escaping; the XML reader decodes &lt;/&gt;/&amp;
    // before the regex compiles, so `&lt;= 10` becomes `<= 10`.
    let xml = r#"<stringmatch name="le" min="1" max="1">if \(dat_52 &lt;= 10\)</stringmatch>"#;
    let mut p = parse_stringmatch(xml);
    assert!(run_property(&mut p, &["  if (dat_52 <= 10) {"]));
}

// ---------------------------------------------------------------------------
// istream_read_int: the >> int extraction.
// ---------------------------------------------------------------------------

#[test]
fn istream_int_parsing() {
    assert_eq!(istream_read_int("1"), 1);
    assert_eq!(istream_read_int("  42"), 42);
    assert_eq!(istream_read_int("-3"), -3);
    assert_eq!(istream_read_int("10x"), 10); // trailing junk ignored
    assert_eq!(istream_read_int("abc"), 0); // no digits -> 0
    assert_eq!(istream_read_int(""), 0);
}

// ---------------------------------------------------------------------------
// End-to-end through real datatest XMLs.
// ---------------------------------------------------------------------------

/// The repo root (the worktree containing `decompiler/datatests/`).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn datatest(name: &str) -> String {
    repo_root().join("decompiler/datatests").join(name).to_str().unwrap().to_string()
}

#[test]
fn load_real_datatest_parses_script_and_tests() {
    // lzcount.xml: 4 commands, 2 stringmatch assertions, 1 binaryimage.
    let mut c = FunctionTestCollection::new();
    c.load_test(&datatest("lzcount.xml")).expect("lzcount parses");
    assert_eq!(c.num_commands(), 4);
    assert_eq!(c.get_command(0), "lo fu cntlzwtest");
    assert_eq!(c.get_command(3), "quit");
    assert_eq!(c.test_list.len(), 2);
    assert_eq!(c.test_list[0].get_name(), "Leading zeros count #1");
    assert!(c.saw_program);
}

#[test]
fn end_to_end_real_datatest_error_path_grammar() {
    // Drive lzcount.xml end-to-end through the ported console.  The engine
    // commands (`lo fu`/`decompile`/`print C`) are stubs in the merged tree, so
    // the console enters its error state on the first one and runTests reports
    // the faithful "Error: Did not apply tests in <f>" path -- exactly what a
    // console in that engine state emits.  This exercises the WHOLE pipeline:
    // XML parse -> buildProgram -> command feed -> mainloop -> error capture.
    let files = vec![datatest("lzcount.xml")];
    let mut out = String::new();
    let failed = run_test_files(&files, &mut out);

    // No assertion was APPLIED (the script never produced print-C output), so the
    // totals are zero and the file lands in the capped Failures summary as an
    // execution failure -- the byte grammar run_tests.py greps.
    assert!(out.contains("Total tests applied = 0"), "out:\n{out}");
    assert!(out.contains("Total passing tests = 0"), "out:\n{out}");
    assert!(out.contains("Error: Did not apply tests in"), "out:\n{out}");
    assert!(out.contains(&format!("Execution failed for {}", datatest("lzcount.xml"))), "out:\n{out}");
    assert!(out.contains("Failures: "), "out:\n{out}");
    // Zero applied -> zero failed assertions (the file error is tracked in the
    // summary, not the assertion count).
    assert_eq!(failed, 0);
}

#[test]
fn end_to_end_multiple_files_accumulate() {
    let files =
        vec![datatest("lzcount.xml"), datatest("boolless.xml"), datatest("skipnext2.xml")];
    let mut out = String::new();
    run_test_files(&files, &mut out);
    // Each file enters the error path; the Failures summary lists all three
    // (capped at 10), and the totals footer is present exactly once.
    assert_eq!(out.matches("Error: Did not apply tests in").count(), 3, "out:\n{out}");
    assert_eq!(out.matches("Total tests applied =").count(), 1, "out:\n{out}");
}

// ---------------------------------------------------------------------------
// Full Success/FAIL + summary grammar, driven without the (stubbed) engine.
//
// The `echo` console command writes its argument to `fileoptr` (the bulkout the
// matcher reads), so an `echo`-only script produces deterministic decompiler-
// output lines and exercises the WHOLE run_tests -> evaluate_tests -> summary
// path (Success/FAIL emission + the totals/Failures footer) end to end.
// ---------------------------------------------------------------------------

/// A `<decompilertest>` whose script echoes `lines` (one `echo` per line) and a
/// list of `<stringmatch name min max>body</stringmatch>` assertions.
fn synth_xml(lines: &[&str], matches: &[(&str, i32, i32, &str)]) -> String {
    let mut s = String::from("<decompilertest>\n");
    s.push_str("<binaryimage arch=\"x86:LE:64:default:gcc\"></binaryimage>\n");
    s.push_str("<script>\n");
    for l in lines {
        s.push_str(&format!("<com>echo {l}</com>\n"));
    }
    s.push_str("<com>quit</com>\n</script>\n");
    for (name, min, max, body) in matches {
        s.push_str(&format!(
            "<stringmatch name=\"{name}\" min=\"{min}\" max=\"{max}\">{body}</stringmatch>\n"
        ));
    }
    s.push_str("</decompilertest>\n");
    s
}

#[test]
fn end_to_end_success_and_fail_grammar() {
    // Echo three output lines; one assertion must match (Success), one must be
    // absent (Success, min=0/max=0), one is present-but-expected-absent (FAIL).
    let xml = synth_xml(
        &["return a0 == 3;", "x = y + z;", "done"],
        &[
            ("match-eq", 1, 1, "return a0 == 3;"),     // present once -> Success
            ("no-shift", 0, 0, "&lt;&lt; 7"),          // absent -> Success
            ("must-be-absent", 0, 0, "x = y"),         // present -> FAIL
        ],
    );
    let f = write_tmp("synth_grammar.xml", &xml);
    let mut out = String::new();
    let failed = run_test_files(std::slice::from_ref(&f), &mut out);

    // Per-assertion verdicts (the exact Success/FAIL grammar the harness greps).
    assert!(out.contains("Success -- match-eq"), "out:\n{out}");
    assert!(out.contains("Success -- no-shift"), "out:\n{out}");
    assert!(out.contains("FAIL -- must-be-absent"), "out:\n{out}");
    // 3 applied, 2 passing, 1 failed assertion.
    assert!(out.contains("Total tests applied = 3"), "out:\n{out}");
    assert!(out.contains("Total passing tests = 2"), "out:\n{out}");
    assert_eq!(failed, 1);
    // The failed assertion's name lands in the Failures summary.
    assert!(out.contains("\n  must-be-absent\n"), "out:\n{out}");
}

#[test]
fn end_to_end_all_success_no_failures_block() {
    let xml = synth_xml(
        &["int main(void)", "  return 0;", "}"],
        &[("has-return", 1, 1, "return 0;"), ("has-main", 1, 1, "int main")],
    );
    let f = write_tmp("synth_allpass.xml", &xml);
    let mut out = String::new();
    let failed = run_test_files(std::slice::from_ref(&f), &mut out);
    assert_eq!(failed, 0);
    assert!(out.contains("Total tests applied = 2"), "out:\n{out}");
    assert!(out.contains("Total passing tests = 2"), "out:\n{out}");
    // All pass -> no Failures: block.
    assert!(!out.contains("Failures:"), "out:\n{out}");
}

#[test]
fn end_to_end_multiline_match_across_echoed_lines() {
    // A two-line stringmatch must match across two consecutively-echoed lines.
    let body = "if \\(cond\\)\n  return 1;";
    let xml = synth_xml(&["if (cond)", "  return 1;"], &[("ml", 1, 1, body)]);
    let f = write_tmp("synth_multiline.xml", &xml);
    let mut out = String::new();
    let failed = run_test_files(std::slice::from_ref(&f), &mut out);
    assert_eq!(failed, 0, "out:\n{out}");
    assert!(out.contains("Success -- ml"), "out:\n{out}");
}

// ---------------------------------------------------------------------------
// run_test_files: file-error grammar (parse errors at column 0).
// ---------------------------------------------------------------------------

/// Write a temporary XML file and return its path.
fn write_tmp(name: &str, content: &str) -> String {
    let dir = std::env::temp_dir().join(format!("kuna-harness-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path.to_str().unwrap().to_string()
}

#[test]
fn parse_error_unrecognized_root_tag() {
    // A root tag that is neither decompilertest nor binaryimage -> Error parsing.
    let f = write_tmp("badroot.xml", "<wrongtag></wrongtag>");
    let mut out = String::new();
    run_test_files(std::slice::from_ref(&f), &mut out);
    assert!(out.contains(&format!("Error parsing {f}: ")), "out:\n{out}");
    assert!(out.contains("unrecognized XML tag: wrongtag"), "out:\n{out}");
}

#[test]
fn parse_error_missing_script_tag() {
    // A <decompilertest> with a stringmatch + binaryimage but NO script.
    let f = write_tmp(
        "noscript.xml",
        r#"<decompilertest><binaryimage arch="x86:LE:64:default:gcc"></binaryimage><stringmatch name="x" min="0" max="0">y</stringmatch></decompilertest>"#,
    );
    let mut out = String::new();
    run_test_files(std::slice::from_ref(&f), &mut out);
    assert!(out.contains("Did not see <script> tag in <decompilertest>"), "out:\n{out}");
    assert!(out.contains(&format!("Error parsing {f}: ")), "out:\n{out}");
}

#[test]
fn parse_error_unknown_child_tag() {
    let f = write_tmp(
        "badchild.xml",
        r#"<decompilertest><bogus/></decompilertest>"#,
    );
    let mut out = String::new();
    run_test_files(&[f], &mut out);
    assert!(out.contains("Unknown tag in <decompilertest>: bogus"), "out:\n{out}");
}

#[test]
fn old_form_binaryimage_root_is_unsupported() {
    // A bare <binaryimage> root hits restoreXmlOldForm -> "Old format test not
    // supported" (a parse error).
    let f = write_tmp("oldform.xml", r#"<binaryimage arch="x86:LE:64:default:gcc"></binaryimage>"#);
    let mut out = String::new();
    run_test_files(&[f], &mut out);
    assert!(out.contains("Old format test not supported"), "out:\n{out}");
}

#[test]
fn empty_file_list_prints_zero_totals() {
    let mut out = String::new();
    let failed = run_test_files(&[], &mut out);
    assert_eq!(failed, 0);
    assert!(out.contains("Total tests applied = 0"));
    assert!(out.contains("Total passing tests = 0"));
    // No Failures: block when there are no failures.
    assert!(!out.contains("Failures:"), "out:\n{out}");
}

#[test]
fn failures_summary_caps_at_ten() {
    // Eleven malformed files -> the Failures: block lists only the first 10.
    let mut files = Vec::new();
    for i in 0..11 {
        files.push(write_tmp(&format!("bad{i}.xml"), "<wrongtag/>"));
    }
    let mut out = String::new();
    run_test_files(&files, &mut out);
    // Eleven "Error parsing" lines at column 0 (one per file)...
    assert_eq!(out.matches("Error parsing ").count(), 11 + 10, "out:\n{out}");
    // ...but the indented Failures summary repeats only 10 of them (two-space
    // indent).  Count the indented form.
    assert_eq!(out.matches("\n  Error parsing ").count(), 10, "out:\n{out}");
}

// ===========================================================================
// w9-harness-runner verifier adversarial tests.
// ===========================================================================

/// `endTest` is `count >= minimumMatch && count <= maximumMatch` with `count`
/// (uint4) compared against `int4` min/max — the C++ `-Wno-sign-compare`
/// conversion makes the int4 unsigned. The Rust cast `as uint4` reproduces it.
/// Probe the exact `[min,max]` inclusive boundary: count==min and count==max
/// pass; count==min-1 and count==max+1 fail.
#[test]
fn w9_endtest_inclusive_boundary_count() {
    // min=2, max=2: exactly two matches pass; one fails; three fails.
    let mut at2 = FunctionTestProperty::from_parts("c", 2, 2, &[r"hit"]);
    assert!(run_property(&mut at2, &["hit", "x", "hit"])); // 2 -> pass
    let mut at1 = FunctionTestProperty::from_parts("c", 2, 2, &[r"hit"]);
    assert!(!run_property(&mut at1, &["hit"])); // 1 -> below min
    let mut at3 = FunctionTestProperty::from_parts("c", 2, 2, &[r"hit"]);
    assert!(!run_property(&mut at3, &["hit", "hit", "hit"])); // 3 -> above max
}

/// A min=max=0 ("must be absent") assertion is the corpus's negative form. The
/// `count <= maximumMatch` term with max=0 means a single match fails it. This
/// is the signedness-sensitive comparison (`0u32 <= 0i32 as u32`) at the zero
/// boundary.
#[test]
fn w9_endtest_zero_max_negative_assertion() {
    let mut absent = FunctionTestProperty::from_parts("none", 0, 0, &[r"forbidden"]);
    assert!(run_property(&mut absent, &["clean output", "more"])); // 0 -> pass
    let mut present = FunctionTestProperty::from_parts("none", 0, 0, &[r"forbidden"]);
    assert!(!run_property(&mut present, &["has forbidden token"])); // 1 -> fail
}

/// The multi-line abort-then-restart quirk under an OVERLAPPING pattern where
/// the failing mid-line is ALSO the first line of a fresh match. Pattern
/// ["^X", "^Y$"]. Stream: "X1" (patnum 0->1), "X2" (not "^Y$"; abort to 0, then
/// re-test "X2" against "^X" -> matches -> patnum 1), "Y" (matches "^Y$" ->
/// count). Net exactly ONE full match — the re-test salvages the second X.
#[test]
fn w9_multiline_abort_restart_salvages_overlapping_first_line() {
    let mut p = FunctionTestProperty::from_parts("xy", 1, 1, &[r"^X", r"^Y$"]);
    assert!(run_property(&mut p, &["X1", "X2", "Y"]));
    // Contrast: without a salvageable first line in between, the partial aborts
    // and NO match completes (min=1 -> fail).
    let mut q = FunctionTestProperty::from_parts("xy", 1, 1, &[r"^X", r"^Y$"]);
    assert!(!run_property(&mut q, &["X1", "Q", "Y"]));
}

/// The bulk-output line slicer must feed the final line WITHOUT a trailing
/// newline (C++ `if (prevpos != result.size())` tail). Drive two synthetic
/// runs: one whose matched token is only on the final, newline-terminated echo
/// line, and one where the matcher must see a line that the `echo` newline does
/// produce. Both must reach the property. (echo always appends `\n`, so this
/// asserts the slicer counts the last real line and does not drop it.)
#[test]
fn w9_bulkout_final_line_is_fed_to_matcher() {
    // Single echoed line -> bulkout "marker\n"; the slicer's while-loop consumes
    // it as [0,6) and prevpos becomes 7 == len, so the tail branch does NOT
    // double-feed. The property must still see "marker" exactly once.
    let xml = synth_xml(&["marker"], &[("once", 1, 1, "marker")]);
    let f = write_tmp("w9_final_line.xml", &xml);
    let mut out = String::new();
    let failed = run_test_files(std::slice::from_ref(&f), &mut out);
    assert_eq!(failed, 0, "out:\n{out}");
    assert!(out.contains("Success -- once"), "out:\n{out}");
    // And a max=1 assertion proves the final line was fed exactly once, not
    // twice (a double-feed would push count to 2 and fail max=1).
    assert!(out.contains("Total passing tests = 1"), "out:\n{out}");
}

/// A whitespace-only `<stringmatch>` tail must not emit an empty trailing
/// pattern (C++ pos-skip-ws + break before emplace). A pattern split on `\n`
/// whose second segment is all spaces yields ONE regex, not two; the property
/// then matches single-line, not as a (never-satisfiable) two-line pattern.
#[test]
fn w9_trailing_whitespace_segment_emits_no_empty_regex() {
    let xml = "<stringmatch name=\"t\" min=\"1\" max=\"1\">target\n      \t  </stringmatch>";
    let p = parse_stringmatch(xml);
    assert_eq!(p.pattern.len(), 1, "trailing ws tail must not add a regex");
    // It behaves as a single-line match (would never fire if it were two-line).
    let mut q = parse_stringmatch(xml);
    assert!(run_property(&mut q, &["a target here"]));
}
