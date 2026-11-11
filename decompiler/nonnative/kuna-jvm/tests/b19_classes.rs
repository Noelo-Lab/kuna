//! Slice B19 against Vineflower 1.12.0: the class node trees
//! `ClassesProcessor.loadClasses` builds (and the warnings it logs) under
//! several options, the lambda nodes `LambdaProcessor` adds, the names
//! `ClassWrapper.init` gives methods without code, and the Java regex subset
//! of the class filters. Goldens: `data/b19/golden.txt` (`gen.sh`) and the
//! `== nodes` section of `data/class_model/golden.txt`.

use std::path::{Path, PathBuf};

use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::classes_processor::java_regex::JavaPattern;
use kuna_jvm::classes_processor::{string_compare_to, ClassNode, ClassNodeType};
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::import_collector::ImportCollector;
use kuna_jvm::lambda_processor::LambdaProcessor;
use kuna_jvm::rt::try_catch;
use kuna_jvm::text_buffer::TextBuffer;

use crate::common::{self, ContextGuard};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

fn esc(s: Option<&str>) -> String {
    match s {
        None => "\\N".to_string(),
        Some(s) => s
            .replace('\\', "\\\\")
            .replace('\t', "\\t")
            .replace('\n', "\\n"),
    }
}

fn unesc(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            other => panic!("bad escape {other:?} in {s}"),
        }
    }
    out
}

/// The golden lines from `== <name>` up to the next section.
fn section(golden: &str, name: &str) -> Vec<String> {
    let header = format!("== {name}");
    golden
        .lines()
        .skip_while(|l| *l != header)
        .skip(1)
        .take_while(|l| !l.starts_with("== "))
        .map(str::to_string)
        .collect()
}

fn b19_golden() -> String {
    std::fs::read_to_string(data_dir().join("b19/golden.txt")).unwrap()
}

/// The generator's class file list: every `.class` under `dir`, by path.
fn class_files(dir: &Path) -> Vec<PathBuf> {
    fn collect(f: &Path, out: &mut Vec<PathBuf>) {
        if f.is_dir() {
            for entry in std::fs::read_dir(f).unwrap() {
                collect(&entry.unwrap().path(), out);
            }
        } else if f.extension().is_some_and(|e| e == "class") {
            out.push(f.to_path_buf());
        }
    }
    let mut out = Vec::new();
    collect(dir, &mut out);
    out.sort_by(|a, b| string_compare_to(&a.to_string_lossy(), &b.to_string_lossy()));
    out
}

/// A context over `files`, each its own source as `addSource(File)` adds it.
fn open(files: &[PathBuf], options: &[(&str, &str)]) -> ContextGuard {
    let mut all = vec![(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")];
    all.extend_from_slice(options);
    let guard = common::install_context_with(&all);
    let sc = guard.struct_context();
    for f in files {
        sc.add_space_file(f, true);
    }
    guard.logger().records().borrow_mut().clear();
    guard
}

fn warnings(guard: &ContextGuard) -> Vec<String> {
    guard
        .logger()
        .records()
        .borrow()
        .iter()
        .filter(|r| r.severity.ordinal() >= Severity::Warn.ordinal())
        .map(|r| format!("log\t{}\t{}", r.severity.name(), esc(Some(&r.message))))
        .collect()
}

fn qualified_name(node: ClassNode) -> String {
    node.with(|d| d.class_struct).qualified_name()
}

fn id(node: ClassNode) -> String {
    node.with(|d| {
        if d.type_ == ClassNodeType::Lambda {
            d.simple_name.clone().unwrap()
        } else {
            d.class_struct.qualified_name()
        }
    })
}

fn node_line(n: ClassNode) -> String {
    let (type_, simple_name, access, parent, nested, enclosing_method, enclosing, anonymous) = n
        .with(|d| {
            (
                d.type_,
                d.simple_name.clone(),
                d.access,
                d.parent,
                d.nested.to_vec(),
                d.enclosing_method.clone(),
                d.enclosing_classes.borrow().to_vec(),
                d.anonymous_class_type.clone(),
            )
        });
    let mut enclosing = enclosing;
    enclosing.sort_by(|a, b| string_compare_to(a, b));
    [
        id(n),
        type_.name().to_string(),
        esc(simple_name.as_deref()),
        access.to_string(),
        parent.map_or("\\N".to_string(), id),
        nested.into_iter().map(id).collect::<Vec<_>>().join(","),
        esc(enclosing_method.as_deref()),
        enclosing.join(","),
        anonymous.map_or("\\N".to_string(), |t| t.value_str().to_string()),
    ]
    .join("\t")
}

fn lambda_line(n: ClassNode) -> String {
    let (simple_name, l) = n.with(|d| {
        (
            d.simple_name.clone().unwrap(),
            d.lambda_information.clone().unwrap(),
        )
    });
    [
        "lambda".to_string(),
        simple_name,
        l.method_name,
        l.method_descriptor,
        l.content_class_name,
        l.content_method_name,
        l.content_method_descriptor,
        l.content_method_invocation_type.to_string(),
        l.content_method_key,
        l.is_method_reference.to_string(),
        l.is_content_method_static.to_string(),
    ]
    .join("\t")
}

fn sorted_roots(guard: &ContextGuard) -> Vec<(String, ClassNode)> {
    let roots = guard.class_processor().get_map_root_classes();
    let mut entries = roots.borrow().entry_vec();
    entries.sort_by(|a, b| string_compare_to(&a.0, &b.0));
    entries
}

fn own_classes() -> Vec<PathBuf> {
    class_files(&data_dir().join("b19/classes"))
}

#[test]
fn load_classes_matches_vineflower() {
    let golden = b19_golden();
    let own = own_classes();
    let inconsistent = class_files(&data_dir().join("b19/inconsistent"));
    let mut reversed = inconsistent.clone();
    reversed.reverse();

    let runs: [(&str, &[PathBuf], &[(&str, &str)]); 9] = [
        ("default", &own, &[]),
        (
            "verify-anonymous",
            &own,
            &[(IFernflowerPreferences::VERIFY_ANONYMOUS_CLASSES, "1")],
        ),
        (
            "no-inner",
            &own,
            &[(IFernflowerPreferences::DECOMPILE_INNER, "0")],
        ),
        (
            "excluded",
            &own,
            &[(IFernflowerPreferences::EXCLUDED_CLASSES, "b/Outer\\$A.*")],
        ),
        (
            "included",
            &own,
            &[(
                IFernflowerPreferences::INCLUDED_CLASSES,
                "b/(Natives|Outer(\\$[0-9]+)?)",
            )],
        ),
        (
            "no-name-check",
            &own,
            &[(IFernflowerPreferences::VALIDATE_INNER_CLASSES_NAMES, "0")],
        ),
        ("inconsistent", &inconsistent, &[]),
        ("inconsistent-reversed", &reversed, &[]),
        (
            "inconsistent-quiet",
            &reversed,
            &[(IFernflowerPreferences::WARN_INCONSISTENT_INNER_CLASSES, "0")],
        ),
    ];
    for (name, files, options) in runs {
        let guard = open(files, options);
        guard.class_processor().load_classes(None);
        let mut out: Vec<String> = sorted_roots(&guard)
            .into_iter()
            .map(|(_, node)| node_line(node))
            .collect();
        out.extend(warnings(&guard));
        assert_eq!(
            out,
            section(&golden, &format!("load {name}")),
            "load {name}"
        );
    }
}

#[test]
fn lambda_processor_matches_vineflower() {
    let golden = b19_golden();
    let guard = open(&own_classes(), &[]);
    guard.class_processor().load_classes(None);
    let roots = sorted_roots(&guard);
    for (_, root) in &roots {
        if root.with(|d| d.type_) == ClassNodeType::Root {
            LambdaProcessor::new().process_class(*root);
        }
    }

    fn dump(n: ClassNode, out: &mut Vec<String>) {
        out.push(node_line(n));
        if n.with(|d| d.type_) == ClassNodeType::Lambda {
            out.push(lambda_line(n));
        }
        for c in n.with(|d| d.nested.to_vec()) {
            if c.with(|d| d.parent) == Some(n) {
                dump(c, out);
            }
        }
    }
    let mut out = Vec::new();
    for (_, root) in &roots {
        if root.with(|d| d.type_) == ClassNodeType::Root {
            dump(*root, &mut out);
        }
    }
    let mut keys: Vec<String> = guard
        .class_processor()
        .get_map_root_classes()
        .borrow()
        .key_vec()
        .into_iter()
        .filter(|k| k.contains("##"))
        .collect();
    keys.sort_by(|a, b| string_compare_to(a, b));
    out.push(format!("keys\t{}", keys.join(",")));
    assert_eq!(out, section(&golden, "lambdas"));
}

#[test]
fn class_wrapper_names_the_parameters_of_methods_without_code() {
    let golden = b19_golden();
    let guard = open(&own_classes(), &[]);
    guard.class_processor().load_classes(None);
    let natives = guard
        .class_processor()
        .get_map_root_classes()
        .borrow()
        .get_cloned(&"b/Natives".to_string())
        .unwrap();
    DecompilerContext::start_class(ImportCollector::new(natives));
    let wrapper = ClassWrapper::new(natives);
    wrapper.init(None);
    assert_eq!(
        DecompilerContext::get_current_class_wrapper(),
        Some(wrapper)
    );
    assert_eq!(DecompilerContext::get_current_class_node(), Some(natives));

    let mut out = Vec::new();
    for mw in wrapper.get_methods().borrow().to_vec() {
        let (mt, vp, error, root) = mw.with(|d| {
            (
                d.method_struct,
                d.varproc,
                d.decompile_error.is_none(),
                d.root.is_none(),
            )
        });
        if mt.contains_code() {
            continue;
        }
        out.push(format!(
            "method\t{}\t{}\t{}\t{}",
            mt.get_name(),
            mt.get_descriptor(),
            error,
            root
        ));
        let mut pairs = vp.get_used_var_versions();
        pairs.sort_by_key(|p| (p.var, p.version));
        for p in pairs {
            out.push(format!(
                "var\t{}\t{}\t{}",
                p.var,
                p.version,
                vp.get_var_name(p).unwrap()
            ));
        }
        for p in vp.get_params() {
            out.push(format!("param\t{}\t{}", p.var, p.version));
        }
        for (p, name) in vp.get_this_vars().borrow().entry_vec() {
            out.push(format!("this\t{}\t{}\t{}", p.var, p.version, name));
        }
    }
    assert_eq!(out, section(&golden, "wrapper"));
    assert!(wrapper
        .get_method_properties("m", "(IJLjava/lang/String;)V")
        .is_some());
}

#[test]
fn only_root_classes_are_processed_and_written() {
    let guard = open(&own_classes(), &[]);
    let cp = guard.class_processor();
    cp.load_classes(None);
    let member = guard.struct_context().get_class("b/Outer$B").unwrap();
    cp.process_class(member);
    let mut buffer = TextBuffer::new();
    cp.write_class(member, &mut buffer);
    assert_eq!(buffer.convert_to_string_and_allow_data_discard(), "");
    assert!(guard.logger().records().borrow().is_empty());
}

#[test]
fn java_regex_matches_like_java() {
    let golden = b19_golden();
    let cases = section(&golden, "regex");
    assert!(!cases.is_empty());
    for case in cases {
        let fields: Vec<&str> = case.split('\t').collect();
        let (regex, input, want) = (unesc(fields[1]), unesc(fields[2]), fields[3]);
        let got = match try_catch(|| JavaPattern::compile(&regex).matches(&input)) {
            Ok(matched) => matched.to_string(),
            Err(e) => {
                assert!(e.is("PatternSyntaxException"), "{regex}: {}", e.message);
                "error".to_string()
            }
        };
        assert_eq!(got, want, "Pattern.matches({regex:?}, {input:?})");
    }
}

/// `loadClasses` itself reproduces the hand-built tree `class_model.rs` checks.
#[test]
fn load_classes_matches_the_class_model_golden() {
    let golden = std::fs::read_to_string(data_dir().join("class_model/golden.txt")).unwrap();
    let guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
    ]);
    let sc = guard.struct_context();
    sc.add_space_file(&data_dir().join("class_model/classes"), true);
    sc.add_space_file(&data_dir().join("class_model/lib"), false);
    guard.class_processor().load_classes(None);

    let mut out = Vec::new();
    let roots = sorted_roots(&guard);
    for (_, n) in &roots {
        let n = *n;
        let (type_, simple_name, access, parent, nested, enclosing_method, enclosing, anonymous) =
            n.with(|d| {
                (
                    d.type_,
                    d.simple_name.clone(),
                    d.access,
                    d.parent,
                    d.nested.to_vec(),
                    d.enclosing_method.clone(),
                    d.enclosing_classes.borrow().to_vec(),
                    d.anonymous_class_type.clone(),
                )
            });
        let mut enclosing = enclosing;
        enclosing.sort_by(|a, b| string_compare_to(a, b));
        out.push(
            [
                qualified_name(n),
                type_.name().to_string(),
                esc(simple_name.as_deref()),
                access.to_string(),
                parent.map_or("\\N".to_string(), qualified_name),
                nested
                    .into_iter()
                    .map(qualified_name)
                    .collect::<Vec<_>>()
                    .join(","),
                esc(enclosing_method.as_deref()),
                enclosing.join(","),
                anonymous.map_or("\\N".to_string(), |t| t.value_str().to_string()),
                n.to_string(),
            ]
            .join("\t"),
        );
    }
    let outer = roots.iter().find(|(name, _)| name == "p/Outer").unwrap().1;
    let all: Vec<String> = outer
        .get_all_nested()
        .to_vec()
        .into_iter()
        .map(qualified_name)
        .collect();
    out.push(format!("all\t{}", all.join(",")));
    assert_eq!(out, section(&golden, "nodes"));
    assert_eq!(warnings(&guard), Vec::<String>::new());
}
