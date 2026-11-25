//! The corpus harness: manifest parsing, the fixture's comparison, digests, failure keys, and the
//! vendored corpus under `tests/jvm/`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_corpus::*;

const SAMPLE: &str = "\
# a comment
kuna-jvm-corpus\t1
root\tdata
java_home\t8\t/opt/jdk8
set\tDefault\tdefault
opt\tlog-level\twarn
opt\terror-message\t
opt\tinclude-runtime\t
opt\tbanner\ta\\tb\\\\c
set\tJavadoc\tjavadoc
opt\tlog-level\twarn
javadoc\tplain
set\tSingle\tsingle
opt\tinclude-classpath\t1
current_jdk\t17
case\tdefault/pkg/TestA\tDefault
opt\tinclude-runtime\t${java.8.home}
opt\textra\t${java.9.home}/lib
class\tjava8/pkg/TestA.class
class\tjava8/pkg/TestA$1.class
lib\t/abs/lib.jar
expected\texpected/default/pkg/TestA.java
compare\tTestA.java\tTestA.kt
case\tjavadoc/pkg/TestB\tJavadoc
class\tjava8/pkg/TestB.class
expected\texpected/javadoc/pkg/TestB.java
compare\tTestB.java
case\tsingle/pkg/TestC\tSingle
class\tjava8/pkg/TestC.class
expected\texpected/single/pkg/TestC.java
compare\tTestC.java
";

fn sample() -> Vec<Case> {
    parse_cases(SAMPLE, Path::new("/m")).expect("sample parses")
}

fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn unit(path: &str, content: &str) -> DecompiledClass {
    DecompiledClass {
        qualified_name: path.trim_end_matches(".java").to_string(),
        path: path.to_string(),
        content: content.to_string(),
        mapping: None,
        ..DecompiledClass::default()
    }
}

fn result(id: &str, status: CaseStatus) -> CaseResult {
    CaseResult {
        id: id.to_string(),
        status,
        actual: Some("x".to_string()),
        actual_path: Some("pkg/X.java".to_string()),
        first_diff: None,
        error: None,
        failure: None,
        reason: None,
        log: Vec::new(),
    }
}

fn thrown(class: Option<&str>, message: &str, location: &str) -> Thrown {
    Thrown {
        class: class.map(str::to_string),
        message: message.to_string(),
        location: location.to_string(),
    }
}

#[test]
fn manifest_sets_cases_and_overlays() {
    let cases = sample();
    assert_eq!(cases.len(), 3);
    let a = &cases[0];
    assert_eq!(a.id, "default/pkg/TestA");
    assert_eq!(
        (a.set.as_str(), a.set_slug.as_str()),
        ("Default", "default")
    );
    assert_eq!(
        a.options,
        pairs(&[
            ("log-level", "warn"),
            ("error-message", ""),
            ("include-runtime", "/opt/jdk8"),
            ("banner", "a\tb\\c"),
            ("extra", "${java.9.home}/lib"),
        ])
    );
    assert_eq!(
        a.class_files,
        vec![
            PathBuf::from("/m/data/java8/pkg/TestA.class"),
            PathBuf::from("/m/data/java8/pkg/TestA$1.class")
        ]
    );
    assert_eq!(a.libraries, vec![PathBuf::from("/abs/lib.jar")]);
    assert_eq!(
        a.expected,
        Some(PathBuf::from("/m/data/expected/default/pkg/TestA.java"))
    );
    assert_eq!(
        a.compare,
        vec!["TestA.java".to_string(), "TestA.kt".to_string()]
    );
    assert_eq!(a.javadoc, None);
    assert_eq!(
        a.jdks,
        vec![
            JdkNeed {
                version: "8".into(),
                home: Some(PathBuf::from("/opt/jdk8")),
                current: false
            },
            JdkNeed {
                version: "9".into(),
                home: None,
                current: false
            },
        ]
    );
    assert_eq!(cases[1].javadoc.as_deref(), Some("plain"));
    assert_eq!(unsupported_reason(&cases[1]), None);
    assert_eq!(
        cases[2].jdks,
        vec![JdkNeed {
            version: "17".into(),
            home: None,
            current: true
        }]
    );
    assert!(unsupported_reason(&cases[2]).unwrap().starts_with("jdk 17"));
    assert!(unsupported_reason(a)
        .unwrap()
        .starts_with("jdk 8 not found (/opt/jdk8)"));
}

#[test]
fn manifest_errors_name_the_line() {
    let body = SAMPLE
        .split_once('\n')
        .unwrap()
        .1
        .split_once('\n')
        .unwrap()
        .1;
    let with = |extra: &str| format!("kuna-jvm-corpus\t1\n{body}{extra}");
    let bad = |text: &str| parse_cases(text, Path::new("/m")).unwrap_err();
    assert!(bad("set\tX\tx\n").contains("expected header"));
    assert!(bad("").contains("empty manifest"));
    assert!(bad("kuna-jvm-corpus\t2\n").contains("expected header"));
    assert!(bad(&with("frob\t1\n")).contains("unknown record `frob`"));
    assert!(bad(&with(
        "case\tdefault/pkg/TestA\tDefault\nclass\tx\nexpected\ty\ncompare\tz\n"
    ))
    .contains("duplicate case id"));
    assert!(bad(&with(
        "case\tq/Q\tNope\nclass\tx\nexpected\ty\ncompare\tz\n"
    ))
    .contains("unknown set `Nope`"));
    assert!(bad(&with("case\tq/Q\tDefault\nexpected\ty\ncompare\tz\n")).contains("no class files"));
    assert!(bad(&with("case\tq/Q\tDefault\nclass\tx\ncompare\tz\n")).contains("no expected file"));
    assert!(bad(&with("case\tq/Q\tDefault\nclass\tx\nexpected\ty\nvineflower\t0\n"))
        .contains("both `expected` and `vineflower`"));
    assert!(bad("kuna-jvm-corpus\t1\nopt\ta\tb\n").contains("line 2: `opt` outside a set or case"));
    assert!(
        bad("kuna-jvm-corpus\t1\nset\tS\ts\nclass\tx\n").contains("line 3: `class` outside a case")
    );
    assert!(bad("kuna-jvm-corpus\t1\nset\tS\ts\nset\tS\ts\n").contains("defined twice"));
}

#[test]
fn the_short_form_derives_classes_expected_and_compare() {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("kuna_corpus_short_form");
    let _ = std::fs::remove_dir_all(&root);
    for f in [
        "c/p/A.class",
        "c/p/A$1.class",
        "c/p/A$B$C.class",
        "c/p/A$x.txt",
        "c/p/AB.class",
        "c/p/B.class",
        "expected/s/p/A.kt",
        "x/p/B.java",
    ] {
        std::fs::create_dir_all(root.join(f).parent().unwrap()).unwrap();
        std::fs::write(root.join(f), "").unwrap();
    }
    let text = "kuna-jvm-corpus\t1\nset\tS\ts\nset\tX\tx\tx\n\
                case\ts/p/A\tS\tc/p/A\tc/p/B\ncase\tx/p/B\tX\tc/p/B\nclass\tc/p/A.class\n";
    let cases = parse_cases(text, &root).unwrap();
    let names = |c: &Case| -> Vec<String> {
        c.class_files
            .iter()
            .map(|p| p.strip_prefix(&root).unwrap().display().to_string())
            .collect()
    };
    let classes = ["c/p/A.class", "c/p/A$1.class", "c/p/A$B$C.class", "c/p/B.class"];
    assert_eq!(names(&cases[0]), classes);
    assert_eq!(cases[0].expected, Some(root.join("expected/s/p/A.kt")));
    assert_eq!(cases[0].compare, ["A.kt", "A.java"]);
    assert_eq!(names(&cases[1]), ["c/p/B.class", "c/p/A.class"]);
    assert_eq!(cases[1].expected, Some(root.join("x/p/B.java")));
    assert_eq!(cases[1].compare, ["B.java"]);

    let pinned = parse_cases(&format!("{text}case\ts/p/AB\tS\tc/p/AB\nvineflower\tx\n"), &root).unwrap();
    assert_eq!((&pinned[2].expected, &pinned[2].compare[..]), (&None, &["AB.java".to_string()][..]));
}

#[test]
fn digest_records_pin_the_compat_and_the_default_mode_run() {
    let cases = parse_cases(
        &format!(
            "{SAMPLE}case\tsingle/pkg/TestD\tSingle\nclass\tjava8/pkg/TestD.class\n\
             vineflower\tfedcba9876543210\nkuna\t0123456789abcdef\n"
        ),
        Path::new("/m"),
    )
    .unwrap();
    let (case, run) = (&cases[3], &cases[4]);
    assert_eq!(case.id, "single/pkg/TestD");
    assert_eq!(
        (&case.expected, &case.compare[..], &case.digest),
        (&None, &["TestD.java".to_string()][..], &Some("fedcba9876543210".into()))
    );
    assert_eq!(run.id, "kuna/single/pkg/TestD");
    let mut options = case.options.clone();
    options.push(("jvm_vineflower_compat".into(), "0".into()));
    assert_eq!(run.options, options);
    assert_eq!(
        (&run.class_files, &run.compare, &run.expected, &run.digest),
        (&case.class_files, &case.compare, &None, &Some("0123456789abcdef".into()))
    );
    assert_eq!(cases[2].digest, None);

    assert_eq!(text_digest("abc"), "ba7816bf8f01cfea");
    let mut case = sample()[0].clone();
    case.digest = Some(text_digest("class TestA {\n}"));
    let run = |text: &str| EngineRun {
        units: vec![unit("pkg/TestA.java", text)],
        ..EngineRun::default()
    };
    let r = evaluate(&case, run("\r\nclass TestA {\r\n}\n  "));
    assert_eq!(r.status, CaseStatus::Match);
    let r = evaluate(&case, run("class TestA {}"));
    assert_eq!(r.status, CaseStatus::Mismatch);
    assert_eq!(
        failure_key(&r).unwrap(),
        format!(
            "digest {}, pinned {} (--out DIR writes the text)",
            text_digest("class TestA {}"),
            text_digest("class TestA {\n}")
        )
    );
}

#[test]
fn fields_unescape() {
    for (field, s) in [
        ("", ""),
        ("plain", "plain"),
        ("a\\tb", "a\tb"),
        ("line\\nnext\\r\\n", "line\nnext\r\n"),
        ("back\\\\slash", "back\\slash"),
        ("\\\\t literal", "\\t literal"),
        ("trailing\\", "trailing\\"),
    ] {
        assert_eq!(unescape_field(field), s);
    }
}

#[test]
fn normalize_is_get_content() {
    assert_eq!(normalize("\n\t a\r\nb\r\n\r\n  "), "a\nb");
    assert_eq!(normalize("a\rb"), "a\rb");
    assert_eq!(normalize("\u{1}\u{1f}x\u{0}"), "x");
    assert_eq!(normalize("\u{a0}x\u{2003}"), "\u{a0}x\u{2003}");
    assert_eq!(normalize("  \r\n "), "");
}

#[test]
fn first_diff_locates_the_line() {
    assert_eq!(first_diff("a\nb", "a\nb"), None);
    let d = first_diff("a\nb\nc", "a\nb\nX").unwrap();
    assert_eq!(
        (d.line, d.expected.as_deref(), d.actual.as_deref()),
        (3, Some("c"), Some("X"))
    );
    let d = first_diff("a\nb\nc", "a\nb").unwrap();
    assert_eq!(
        (d.line, d.expected.as_deref(), d.actual),
        (3, Some("c"), None)
    );
    let d = first_diff("a", "a\nmore").unwrap();
    assert_eq!(
        (d.line, d.expected, d.actual.as_deref()),
        (2, None, Some("more"))
    );
    let d = first_diff("x", "").unwrap();
    assert_eq!(
        (d.line, d.expected.as_deref(), d.actual.as_deref()),
        (1, Some("x"), Some(""))
    );
}

#[test]
fn the_compared_unit_is_the_last_with_the_name() {
    let units = vec![
        unit("pkg/Shadow.java", "first"),
        unit("pkg/Other.java", "other"),
        unit("ext/Shadow.java", "second"),
    ];
    let pick = |names: &[&str]| {
        let names: Vec<String> = names.iter().map(|s| s.to_string()).collect();
        select_unit(&units, &names).map(|u| u.content.as_str())
    };
    assert_eq!(pick(&["Shadow.java"]), Some("second"));
    assert_eq!(pick(&["Missing.kt", "Other.java"]), Some("other"));
    assert_eq!(pick(&["Missing.java"]), None);
    assert_eq!(pick(&["Shadow"]), None);
}

#[test]
fn evaluation_statuses() {
    let case = &sample()[0];
    let expected = "package pkg;\n\nclass TestA {\n   // $VF: Couldn't be decompiled\n}\n";
    let run = |units: Vec<DecompiledClass>, error: Option<Thrown>| EngineRun {
        units,
        error,
        log: vec![
            LogLine {
                severity: Severity::Info,
                message: "info".into(),
                throwable: None,
            },
            LogLine {
                severity: Severity::Warn,
                message: "warn".into(),
                throwable: None,
            },
        ],
    };

    let crlf = expected.replace('\n', "\r\n") + "  \r\n";
    let r = evaluate_text(
        case,
        run(vec![unit("pkg/TestA.java", &crlf)], None),
        expected,
    );
    assert_eq!(r.status, CaseStatus::Match);
    assert_eq!(r.first_diff, None);
    assert_eq!(
        r.log.iter().map(|l| l.message.as_str()).collect::<Vec<_>>(),
        ["warn"]
    );
    assert_eq!(failure_key(&r), None);

    let wrong = "package pkg;\n\nclass TestA {\n   // $VF: Couldn't be decompiled\n   // $VF: Couldn't be decompiled\n}";
    let r = evaluate_text(
        case,
        run(vec![unit("pkg/TestA.java", wrong)], None),
        expected,
    );
    assert_eq!(r.status, CaseStatus::Mismatch);
    let d = r.first_diff.clone().unwrap();
    assert_eq!((d.line, d.expected.as_deref()), (5, Some("}")));
    assert_eq!(failure_key(&r).as_deref(), Some("mismatch"));

    let r = evaluate_text(
        case,
        run(vec![unit("pkg/Other.java", expected)], None),
        expected,
    );
    assert_eq!(r.status, CaseStatus::Mismatch);
    assert_eq!(r.actual, None);
    assert_eq!(failure_key(&r).as_deref(), Some("mismatch: no output"));

    let err = thrown(
        Some("NullPointerException"),
        "NullPointerException: x",
        "src/x.rs:1",
    );
    let r = evaluate_text(
        case,
        run(vec![unit("pkg/TestA.java", expected)], Some(err)),
        expected,
    );
    assert_eq!(r.status, CaseStatus::EngineError);
    assert_eq!(
        failure_key(&r).as_deref(),
        Some("NullPointerException: src/x.rs:1")
    );
}

#[test]
fn innermost_messages() {
    let wrapped = "RuntimeException: java.util.concurrent.ExecutionException: UnportedError: IfHelper.mergeAllIfs";
    assert_eq!(
        innermost_message(wrapped),
        "UnportedError: IfHelper.mergeAllIfs"
    );
    assert_eq!(
        innermost_message("RuntimeException: NullPointerException"),
        "NullPointerException"
    );
    assert_eq!(
        innermost_message("IllegalStateException: bad: thing"),
        "IllegalStateException: bad: thing"
    );
    assert_eq!(
        innermost_message("called `Option::unwrap()` on a `None` value"),
        "called `Option::unwrap()` on a `None` value"
    );
}

#[test]
fn failure_keys() {
    let key = |r: &CaseResult| failure_key(r).unwrap();
    let mut r = result("a", CaseStatus::EngineError);
    r.error = Some(thrown(
        None,
        "index out of bounds: the len is 3 but the index is 5",
        "nonnative/kuna-jvm/src/p8_structure/if_helper.rs:120",
    ));
    assert_eq!(key(&r), "panic: src/p8_structure/if_helper.rs:120");

    r.error = Some(thrown(
        Some("RuntimeException"),
        "RuntimeException: Not a constant",
        "nonnative/kuna-jvm/src/a.rs:9",
    ));
    assert_eq!(key(&r), "RuntimeException: src/a.rs:9");

    r.error = Some(thrown(
        Some("RuntimeException"),
        "RuntimeException: java.util.concurrent.ExecutionException: NullPointerException: x",
        "nonnative/kuna-jvm/src/p0_knowledge/context_unit.rs:326",
    ));
    r.log = vec![LogLine {
        severity: Severity::Error,
        message: "Class pkg/A couldn't be fully decompiled.".into(),
        throwable: Some(thrown(
            Some("NullPointerException"),
            "NullPointerException: x",
            "nonnative/kuna-jvm/src/b.rs:4",
        )),
    }];
    assert_eq!(key(&r), "NullPointerException: src/b.rs:4");

    r.error = Some(thrown(
        None,
        "boom",
        "/rustc/abc123/library/core/src/option.rs:2",
    ));
    assert_eq!(key(&r), "panic: library/core/src/option.rs:2");
    r.error = Some(thrown(None, "boom", ""));
    assert_eq!(key(&r), "panic: boom");

    let mut crash = result("b", CaseStatus::EngineError);
    crash.failure = Some("harness: no result".into());
    assert_eq!(key(&crash), "harness: no result");

    let mut unsupported = result("c", CaseStatus::Unsupported);
    unsupported.reason = Some("jdk 8 not found (/x)".into());
    assert_eq!(key(&unsupported), "unsupported: jdk 8 not found");

    let mut m = result("d", CaseStatus::Mismatch);
    m.log = vec![LogLine {
        severity: Severity::Warn,
        message: "Method x couldn't be decompiled.".into(),
        throwable: Some(thrown(
            Some("NullPointerException"),
            "NullPointerException: x",
            "nonnative/kuna-jvm/src/f.rs:3",
        )),
    }];
    assert_eq!(key(&m), "NullPointerException: src/f.rs:3");
    m.log.clear();
    assert_eq!(key(&m), "mismatch");

    let mut u2 = result("f", CaseStatus::EngineError);
    u2.error = Some(thrown(Some("RuntimeException"), "RuntimeException: A.b", ""));
    let ok = result("g", CaseStatus::Match);
    let all = vec![u2, ok, r, crash, unsupported];
    assert_eq!(
        totals(&all),
        Totals {
            total: 5,
            matched: 1,
            mismatch: 0,
            error: 3,
            unsupported: 1
        }
    );
}

fn vendored() -> Vec<Case> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&path).expect("tests/jvm/cases.tsv loads")
}

/// A kuna-fixes case in Kotlin: it runs with the plugins and compares `<Stem>.kt` (or `.java`), then
/// the units of its `compare_fallback` classes the same way.
fn kotlin_fix_case(case: &Case) -> bool {
    let kotlin = is_kuna_fix(case) && case.compare[0].ends_with(".kt");
    if kotlin {
        let class = case.class_files[0].file_stem().unwrap().to_string_lossy();
        assert_eq!(
            case.compare[..2],
            [format!("{class}.kt"), format!("{class}.java")]
        );
        assert!(case.compare[2..].chunks(2).all(|unit| unit.len() == 2
            && unit[0].strip_suffix(".kt").is_some()
            && unit[0].strip_suffix(".kt") == unit[1].strip_suffix(".java")));
    }
    kotlin
}

fn is_kuna_fix(case: &Case) -> bool {
    case.set_slug == "kuna-fixes" || case.set_slug == "kuna-fixes-compat"
}

fn is_digest(digest: &Option<String>) -> bool {
    digest
        .as_deref()
        .is_some_and(|d| d.len() == 16 && d.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
}

#[test]
fn vendored_corpus_is_complete() {
    let (default_runs, cases): (Vec<Case>, Vec<Case>) = vendored()
        .into_iter()
        .partition(|c| c.id.starts_with(DEFAULT_RUN));
    assert_eq!(default_runs.len(), 607);
    for run in &default_runs {
        assert!(run.expected.is_none() && is_digest(&run.digest), "{}", run.id);
    }
    assert_eq!(cases.iter().filter(|c| !is_kuna_fix(c)).count(), 607);
    let compat: BTreeSet<&str> = cases
        .iter()
        .filter_map(|c| c.id.strip_prefix("kuna-fixes-compat/"))
        .collect();
    let fixed: BTreeSet<&str> = cases
        .iter()
        .filter_map(|c| c.id.strip_prefix("kuna-fixes/"))
        .collect();
    assert!(!fixed.is_empty());
    assert_eq!(compat, fixed, "every kuna-fixes case runs in both modes");
    for case in cases.iter().filter(|c| c.set_slug == "kuna-fixes-compat") {
        let twin = cases
            .iter()
            .find(|c| c.id == case.id.replacen("kuna-fixes-compat/", "kuna-fixes/", 1))
            .unwrap();
        assert_eq!(case.compare, twin.compare, "{}", case.id);
    }
    for case in cases.iter().filter(|c| is_kuna_fix(c)) {
        let compat = if case.set_slug == "kuna-fixes-compat" { "1" } else { "0" };
        assert_eq!(
            case.options.iter().find(|(k, _)| k == "jvm_vineflower_compat"),
            Some(&("jvm_vineflower_compat".to_string(), compat.to_string())),
            "{}",
            case.id
        );
        assert!(
            case.options
                .contains(&("dump-exception-on-error".to_string(), "0".to_string())),
            "{}",
            case.id
        );
    }
    assert_eq!(
        cases
            .iter()
            .filter(|c| c.set_slug.starts_with("kotlin-"))
            .count(),
        71
    );
    let ids: BTreeSet<&str> = cases.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids.len(), cases.len());
    let base = [
        ("log-level", "warn"),
        ("decompile-generics", "1"),
        ("remove-synthetic", "1"),
        ("remove-bridge", "1"),
        ("keep-literals", "1"),
        ("__unit_test_mode__", "1"),
        ("new-line-separator", "1"),
        ("error-message", ""),
        ("include-runtime", ""),
    ];
    for case in &cases {
        assert!(
            case.id.starts_with(&format!("{}/", case.set_slug)),
            "{}",
            case.id
        );
        assert!(case.jdks.is_empty(), "{} needs a JDK", case.id);
        if case.set_slug == "kuna-fixes" {
            assert!(case.digest.is_none(), "{}", case.id);
            assert!(case.expected.as_ref().is_some_and(|e| e.is_file()), "{}", case.id);
        } else {
            assert!(case.expected.is_none() && is_digest(&case.digest), "{}", case.id);
        }
        for class in &case.class_files {
            assert!(class.is_file(), "{}", class.display());
        }
        let plugin_set = matches!(
            case.set_slug.as_str(),
            "with-idea-not-null"
                | "jad-naming"
                | "tiny-naming"
                | "plugin-defaults"
                | "kotlin-entire-classpath"
                | "kotlin-multifile-facade"
        ) || kotlin_fix_case(case);
        assert_eq!(
            case.options
                .iter()
                .any(|(k, v)| k == "jvm_plugins" && v == "1"),
            plugin_set,
            "{}",
            case.id
        );
        if case.set_slug == "plugin-defaults" {
            assert_eq!(case.options, [("jvm_plugins".to_string(), "1".to_string())]);
            continue;
        }
        let kotlin = case.set_slug.starts_with("kotlin-");
        if kotlin {
            assert_eq!(case.options[0].0, "jvm_plugins", "{}", case.id);
            let class = case.class_files[0].file_stem().unwrap().to_string_lossy();
            assert_eq!(
                case.compare,
                [format!("{class}.kt"), format!("{class}.java")]
            );
        }
        let keys: Vec<&str> = case
            .options
            .iter()
            .skip(usize::from(kotlin))
            .take(base.len())
            .map(|(k, _)| k.as_str())
            .collect();
        assert_eq!(keys, base.map(|(k, _)| k), "{}", case.id);
        let literals = matches!(
            case.set_slug.as_str(),
            "literals" | "ternary-constant-simplification"
        );
        for (k, v) in base {
            if k == "keep-literals" && case.set_slug.starts_with("kuna-fixes") {
                continue;
            }
            let v = if literals && k == "keep-literals" {
                "0"
            } else if kotlin && !case.set_slug.contains("core") && k == "remove-synthetic" {
                "0"
            } else {
                v
            };
            assert_eq!(
                case.options.iter().find(|(key, _)| key == k).unwrap().1,
                v,
                "{} {k}",
                case.id
            );
        }
    }
    let javadoc: Vec<&str> = cases
        .iter()
        .filter(|c| c.javadoc.is_some())
        .map(|c| c.id.as_str())
        .collect();
    assert_eq!(
        javadoc,
        [
            "javadoc/pkg/TestJavadoc",
            "markdown-javadoc/pkg/TestMarkdownJavadoc"
        ]
    );
    let reference = cases
        .iter()
        .find(|c| c.id == "default/pkg/TestInnerClassReference")
        .unwrap();
    let names: Vec<String> = reference
        .class_files
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names
            .iter()
            .filter(|n| *n == "SomeOuterClass$SomeInner.class")
            .count(),
        2
    );
}

#[test]
fn vendored_expected_texts_match_themselves() {
    let cases: Vec<Case> = vendored()
        .into_iter()
        .filter(|c| c.digest.is_none())
        .collect();
    assert_eq!(cases.len(), 290);
    let mut matched = 0;
    for case in &cases {
        let run = expected_as_actual(case).unwrap();
        let r = evaluate(case, run);
        assert_eq!(r.status, CaseStatus::Match, "{}", case.id);
        assert_eq!(
            r.actual_path
                .as_deref()
                .map(|p| p.rsplit('/').next().unwrap()),
            Some(case.compare[0].as_str())
        );
        matched += 1;
    }
    assert_eq!(matched, cases.len());
}

#[test]
fn engine_failures_are_contained_and_the_thread_stays_usable() {
    let cases = vendored();
    let real = cases
        .iter()
        .find(|c| c.id == "default/pkg/TestEnhancedForLoops")
        .unwrap()
        .clone();
    let garbage_path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("kuna_corpus_garbage.class");
    std::fs::write(&garbage_path, b"\xca\xfe\xba\xbe not a class file").unwrap();
    let mut garbage = real.clone();
    garbage.id = "default/pkg/Garbage".to_string();
    garbage.class_files = vec![garbage_path];

    with_engine_stack(move || {
        let bad = run_case(&garbage);
        assert_eq!(bad.status, CaseStatus::EngineError);
        assert!(bad.error.is_some());
        for _ in 0..2 {
            let r = run_case(&real);
            assert_ne!(r.status, CaseStatus::Unsupported);
            let text = format!("{:?}{:?}", r.error, r.log);
            assert!(
                !text.contains("stale") && !text.contains("re-entered"),
                "{text}"
            );
        }
        let options = effective_options(&real).unwrap();
        assert!(options.contains(&("log-level".to_string(), "warn".to_string())));
    });
}

#[test]
fn effective_options_overlay_the_defaults() {
    let cases = vendored();
    let case = cases
        .iter()
        .find(|c| c.id == "default/pkg/TestEnhancedForLoops")
        .unwrap()
        .clone();
    let options = with_engine_stack(move || {
        let options = effective_options(&case).unwrap();
        (case, options)
    });
    let (case, options) = options;
    for (k, v) in &case.options {
        assert!(options.contains(&(k.clone(), v.clone())), "{k}={v}");
    }
    let defaults = IFernflowerPreferences::DEFAULTS();
    let extra = case
        .options
        .iter()
        .filter(|(k, _)| !defaults.contains_key(k))
        .count();
    assert_eq!(options.len(), defaults.len() + extra);
    let indent = IFernflowerPreferences::INDENT_STRING.to_string();
    assert_eq!(
        options
            .iter()
            .find(|(k, _)| *k == indent)
            .map(|(_, v)| v.clone()),
        defaults.get_cloned(&indent)
    );
    assert!(options.windows(2).all(|w| w[0].0 < w[1].0));
}
