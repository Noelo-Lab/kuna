//! The class node trees `ClassesProcessor.loadClasses` builds (and the warnings it logs, on
//! inconsistent `InnerClasses` entries too) under the options that shape them, against
//! Vineflower 1.12.0 (`data/b19/golden.txt`, `gen.sh`).

use std::path::{Path, PathBuf};

use kuna_jvm::classes_processor::{string_compare_to, ClassNode, ClassNodeType};
use kuna_jvm::i_fernflower_logger::Severity;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;

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

    let runs: [(&str, &[PathBuf], &[(&str, &str)]); 6] = [
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
            "no-name-check",
            &own,
            &[(IFernflowerPreferences::VALIDATE_INNER_CLASSES_NAMES, "0")],
        ),
        ("inconsistent", &inconsistent, &[]),
        ("inconsistent-reversed", &reversed, &[]),
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
