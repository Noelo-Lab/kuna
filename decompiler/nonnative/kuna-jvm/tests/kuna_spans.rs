//! Class and method spans recorded while writing.

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::kuna_input::{self, LoadOptions};

use crate::kuna_support::{corpus, decompile, options};

fn line_of(content: &str, at: usize) -> u32 {
    content[..at].matches('\n').count() as u32 + 1
}

fn simple_name(internal: &str) -> &str {
    internal.rsplit(['/', '$']).next().unwrap()
}

/// The declaration line of a method slice: the first line that is not an
/// annotation or a comment.
fn declaration(slice: &str) -> &str {
    slice
        .lines()
        .map(str::trim)
        .find(|l| !(l.starts_with('@') || l.starts_with("//") || l.starts_with("/*")))
        .unwrap_or("")
}

fn check(unit: &DecompiledClass) {
    let content = &unit.content;
    assert!(
        !unit.methods.is_empty(),
        "{}: no method spans",
        unit.qualified_name
    );
    let root = unit
        .classes
        .iter()
        .find(|c| c.internal == unit.qualified_name)
        .unwrap_or_else(|| panic!("{}: no span for the root class", unit.qualified_name));
    assert!(root.error.is_none());
    let root_text = &content[root.start..root.end];
    assert!(
        root_text.contains(&format!(" {} ", simple_name(&unit.qualified_name)))
            || root_text.contains(&format!(" {}<", simple_name(&unit.qualified_name))),
        "{}: root span does not declare the class:\n{root_text}",
        unit.qualified_name
    );
    for class in &unit.classes {
        assert!(root.start <= class.start && class.end <= root.end);
        assert_eq!(class.start_line, line_of(content, class.start));
    }
    let mut mapped = 0;
    for method in &unit.methods {
        assert!(method.start < method.end && method.end <= content.len());
        let slice = &content[method.start..method.end];
        assert!(method.start == 0 || content.as_bytes()[method.start - 1] == b'\n');
        assert!(slice.ends_with('\n'), "{}: {slice:?}", method.name);
        assert_eq!(method.start_line, line_of(content, method.start));
        assert_eq!(method.end_line, line_of(content, method.end - 1));
        let declared = declaration(slice);
        let expected = match method.name.as_str() {
            "<clinit>" => "static {".to_string(),
            "<init>" => format!("{}(", simple_name(&method.class)),
            name => format!("{name}("),
        };
        assert!(
            declared.contains(&expected),
            "{}.{}{}: declaration line {declared:?} lacks {expected:?}",
            method.class,
            method.name,
            method.descriptor
        );
        assert!(
            method.error.is_none(),
            "{}: {:?}",
            method.name,
            method.error
        );
        let mut last_line = 0;
        for (line, offsets) in &method.lines {
            assert!(*line > last_line);
            last_line = *line;
            assert!(
                (method.start_line..=method.end_line).contains(line),
                "{}: mapped line {line} outside {}..={}",
                method.name,
                method.start_line,
                method.end_line
            );
            assert!(!offsets.is_empty());
            assert!(offsets.windows(2).all(|w| w[0] < w[1]));
        }
        mapped += method.lines.len();
    }
    assert!(mapped > 0, "{}: no bytecode lines", unit.qualified_name);
    let starts: Vec<usize> = unit.methods.iter().map(|m| m.start).collect();
    assert!(starts.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn method_and_class_spans_of_corpus_classes() {
    for class in [
        "java8/pkg/TestAnonymousClassNaming.class",
        "java8/pkg/TestAccessReplace.class",
        "java8/pkg/TestClassLoop.class",
        "java8/pkg/TestAnonymousClassConstructor.class",
    ] {
        let set = kuna_input::load(&corpus(class), &LoadOptions::default()).unwrap();
        let units = decompile(&set, &options(&[]));
        assert_eq!(units.len(), 1, "{class}");
        let unit = &units[0];
        assert!(unit.error.is_none());
        check(unit);
    }
}

#[test]
fn nested_classes_have_spans_inside_their_outer_class() {
    let set = kuna_input::load(
        &corpus("java8/pkg/TestAnonymousClassConstructor.class"),
        &LoadOptions::default(),
    )
    .unwrap();
    let unit = &decompile(&set, &options(&[]))[0];
    let inner = unit
        .classes
        .iter()
        .find(|c| c.internal == "pkg/TestAnonymousClassConstructor$InnerPublic")
        .expect("member class span");
    let text = &unit.content[inner.start..inner.end];
    assert!(
        text.trim_start().starts_with("class InnerPublic {"),
        "{text}"
    );
    assert!(unit
        .methods
        .iter()
        .any(|m| m.class == inner.internal && m.start >= inner.start && m.end <= inner.end));
    let anonymous = unit
        .classes
        .iter()
        .find(|c| c.internal == "pkg/TestAnonymousClassConstructor$1")
        .expect("anonymous class span");
    assert!(unit.content[anonymous.start..anonymous.end]
        .trim_start()
        .starts_with('{'));
}

#[test]
fn line_mappings_point_at_statement_lines() {
    let set = kuna_input::load(
        &corpus("java8/pkg/TestClassLoop.class"),
        &LoadOptions::default(),
    )
    .unwrap();
    let unit = &decompile(&set, &options(&[]))[0];
    let lines: Vec<&str> = unit.content.lines().collect();
    for method in unit.methods.iter().filter(|m| m.name != "<init>") {
        for (line, _) in &method.lines {
            let text = lines[*line as usize - 1].trim();
            assert!(
                !text.is_empty() && text != "}",
                "{}: line {line} is {text:?}",
                method.name
            );
        }
    }
}

/// `tests/fixtures/jvm/failures/classes` (kuna-cli's fixture): lambdas and an
/// anonymous class method that fail in compat mode, inside methods and a field.
#[test]
fn failures_written_inside_a_method_are_recorded_against_it() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/fixtures/jvm/failures/classes");
    let set = kuna_input::load(&dir, &LoadOptions::default()).unwrap();
    let units = decompile(&set, &options(&[("jvm_vineflower_compat", "1")]));
    let unit = units.iter().find(|u| u.qualified_name == "com/ex/fail/Nested").unwrap();
    let method = |name: &str| unit.methods.iter().find(|m| m.name == name).unwrap();
    let bodies = |name: &str| {
        method(name)
            .nested
            .iter()
            .map(|f| format!("{}.{}{}", f.class, f.name, f.descriptor))
            .collect::<Vec<_>>()
    };
    assert!(method("escape").error.is_none());
    assert_eq!(bodies("escape"), ["com/ex/fail/Nested.lambda$escape$1(Ljava/lang/StringBuilder;I)V"]);
    assert_eq!(bodies("both").len(), 2);
    assert_eq!(bodies("later"), ["com/ex/fail/Nested$1.run()V"]);
    let run = method("run");
    assert!(run.error.is_some() && run.nested.is_empty());
    assert!(bodies("plain").is_empty());
    let root = unit.classes.iter().find(|c| c.internal == unit.qualified_name).unwrap();
    assert!(root.error.is_none());
    assert_eq!(root.nested.len(), 1, "the field initializer's lambda");
    assert!(root.nested[0].name.starts_with("lambda$static$"));
    for failure in unit.methods.iter().flat_map(|m| &m.nested) {
        assert!(failure.error.contains("invalid constant type"), "{failure:?}");
    }
}

#[test]
fn flagged_vf_comments_are_method_warnings() {
    let set = kuna_input::load(&corpus("java25/pkg/TestAssertsEnabled.class"), &LoadOptions::default()).unwrap();
    let unit = &decompile(&set, &options(&[]))[0];
    let test7 = unit.methods.iter().find(|m| m.name == "test7").unwrap();
    assert_eq!(test7.warnings, ["$VF: Could not resugar all assert statements!"]);
    assert!(test7.error.is_none() && test7.nested.is_empty());
    let test = unit.methods.iter().find(|m| m.name == "test").unwrap();
    assert!(test.warnings.is_empty());
}
