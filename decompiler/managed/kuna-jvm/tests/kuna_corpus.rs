//! The corpus harness: manifest parsing, the fixture's comparison, digests, failure keys, and the
//! vendored corpus under `decompiler/managed/kuna-jvm/corpus/`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::kuna_corpus::*;

const SAMPLE: &str = "\
# a comment
kuna-jvm-corpus\t1
root\tdata
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
case\tdefault/pkg/TestA\tDefault
opt\tinclude-runtime\t/opt/jdk8
opt\textra\tx/lib
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
            ("extra", "x/lib"),
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
    assert_eq!(cases[1].javadoc.as_deref(), Some("plain"));
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
    assert!(bad(&with("case\tq/Q\tDefault\nclass\tx\nexpected\ty\nkuna\t0\n"))
        .contains("both `expected` and `kuna`"));
    assert!(bad(&with("case\tq/Q\tDefault\nclass\tx\nvineflower\t0\n"))
        .contains("unknown record `vineflower`"));
    assert!(bad("kuna-jvm-corpus\t1\nopt\ta\tb\n").contains("line 2: `opt` outside a set or case"));
    assert!(
        bad("kuna-jvm-corpus\t1\nset\tS\ts\nclass\tx\n").contains("line 3: `class` outside a case")
    );
    assert!(bad("kuna-jvm-corpus\t1\nset\tS\ts\nset\tS\ts\n").contains("defined twice"));
    assert!(bad(&with("case\tq/Q\tDefault\nclass\tx\nkuna\t0\njavadoc\tfancy\n"))
        .contains("javadoc provider `fancy`"));
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

    let pinned = parse_cases(&format!("{text}case\ts/p/AB\tS\tc/p/AB\nkuna\tx\n"), &root).unwrap();
    assert_eq!((&pinned[2].expected, &pinned[2].compare[..]), (&None, &["AB.java".to_string()][..]));
}

#[test]
fn a_digest_record_pins_the_case() {
    let cases = parse_cases(
        &format!(
            "{SAMPLE}case\tsingle/pkg/TestD\tSingle\nclass\tjava8/pkg/TestD.class\n\
             kuna\t0123456789abcdef\n"
        ),
        Path::new("/m"),
    )
    .unwrap();
    assert_eq!(cases.len(), 4);
    let case = &cases[3];
    assert_eq!(case.id, "single/pkg/TestD");
    assert_eq!(
        (&case.expected, &case.compare[..], &case.digest),
        (&None, &["TestD.java".to_string()][..], &Some("0123456789abcdef".into()))
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
        "managed/kuna-jvm/src/p8_structure/if_helper.rs:120",
    ));
    assert_eq!(key(&r), "panic: src/p8_structure/if_helper.rs:120");

    r.error = Some(thrown(
        Some("RuntimeException"),
        "RuntimeException: Not a constant",
        "managed/kuna-jvm/src/a.rs:9",
    ));
    assert_eq!(key(&r), "RuntimeException: src/a.rs:9");

    r.error = Some(thrown(
        Some("RuntimeException"),
        "RuntimeException: java.util.concurrent.ExecutionException: NullPointerException: x",
        "managed/kuna-jvm/src/p0_knowledge/context_unit.rs:326",
    ));
    r.log = vec![LogLine {
        severity: Severity::Error,
        message: "Class pkg/A couldn't be fully decompiled.".into(),
        throwable: Some(thrown(
            Some("NullPointerException"),
            "NullPointerException: x",
            "managed/kuna-jvm/src/b.rs:4",
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

    let mut m = result("d", CaseStatus::Mismatch);
    m.log = vec![LogLine {
        severity: Severity::Warn,
        message: "Method x couldn't be decompiled.".into(),
        throwable: Some(thrown(
            Some("NullPointerException"),
            "NullPointerException: x",
            "managed/kuna-jvm/src/f.rs:3",
        )),
    }];
    assert_eq!(key(&m), "NullPointerException: src/f.rs:3");
    m.log.clear();
    assert_eq!(key(&m), "mismatch");

    let mut u2 = result("f", CaseStatus::EngineError);
    u2.error = Some(thrown(Some("RuntimeException"), "RuntimeException: A.b", ""));
    let ok = result("g", CaseStatus::Match);
    let all = vec![u2, ok, r, crash, m];
    assert_eq!(
        totals(&all),
        Totals {
            total: 5,
            matched: 1,
            mismatch: 1,
            error: 3,
        }
    );
}

fn vendored() -> Vec<Case> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../decompiler/managed/kuna-jvm/corpus/cases.tsv");
    load_cases(&path).expect("decompiler/managed/kuna-jvm/corpus/cases.tsv loads")
}

/// A kuna-fixes case in Kotlin: it runs with the plugins and compares `<Stem>.kt` (or `.java`), then
/// the units of its `compare_fallback` classes the same way.
fn kotlin_fix_case(case: &Case) -> bool {
    let kotlin = case.set_slug == "kuna-fixes" && case.compare[0].ends_with(".kt");
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

fn is_digest(digest: &Option<String>) -> bool {
    digest
        .as_deref()
        .is_some_and(|d| d.len() == 16 && d.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
}

#[test]
fn vendored_corpus_is_complete() {
    let cases = vendored();
    assert_eq!(cases.len(), 885);
    assert_eq!(cases.iter().filter(|c| c.set_slug != "kuna-fixes").count(), 591);
    assert_eq!(cases.iter().filter(|c| c.set_slug == "kuna-fixes").count(), 294);
    for case in cases.iter().filter(|c| c.set_slug == "kuna-fixes") {
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
        68
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
                | "plugin-defaults"
                | "kotlin-entire-classpath"
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
        let literals = case.set_slug == "literals";
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
            assert_eq!(r.status, CaseStatus::Match);
            let text = format!("{:?}{:?}", r.error, r.log);
            assert!(
                !text.contains("stale") && !text.contains("re-entered"),
                "{text}"
            );
        }
    });
}
