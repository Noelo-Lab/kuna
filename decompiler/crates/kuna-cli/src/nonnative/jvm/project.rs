//! `kuna decompile-project` for a JVM input: a Java and Kotlin source tree.
//!
//! Each top-level class is written to the engine's own output path,
//! `DIR/<package path>/<Class>.java`, or `<Class>.kt` for a class the Kotlin
//! plugin wrote (nested classes stay inside). A class the engine produced no
//! text for gets a `.java` stub at the path its name implies, holding
//! `// kuna: could not decompile <class>: <reason>`. The summary line counts
//! the files of each extension. The command exits 0 when at least one class
//! was produced.

use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

use super::args::Args;
use super::render;
use super::run::{self, Outcome};
use super::select::{java_name, Target};

pub(crate) fn run(args: &Args) -> i32 {
    let loaded = match render::load_or_report(args) {
        Ok(loaded) => loaded,
        Err(status) => return status,
    };
    kuna_jvm::kuna_driver::trace(|| "cli loaded".into());
    let own = loaded.own_class();
    let mut select: Option<BTreeSet<String>> = None;
    for text in args.functions.iter().flatten() {
        match loaded.universe.resolve(text, own.as_deref(), &args.input) {
            Ok(target) => {
                let class = match &target {
                    Target::Class(class) | Target::Methods { class, .. } => class,
                };
                select.get_or_insert_with(BTreeSet::new).insert(loaded.universe.root_of(class));
            }
            Err(e) => {
                eprintln!("error: {e}");
                return 1;
            }
        }
    }
    let out_dir = PathBuf::from(match &args.output {
        Some(dir) => dir.clone(),
        None => format!("{}.kuna", args.input.trim_end_matches(['/', '\\'])),
    });
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("error: cannot create {}: {e}", out_dir.display());
        return 1;
    }
    let status = render::execute(args, &loaded, select, &|outcome, _runtime| {
        kuna_jvm::kuna_driver::trace(|| "cli write start".into());
        let status = write_tree(args, &out_dir, &outcome);
        kuna_jvm::kuna_driver::trace(|| "cli write end".into());
        run::abandon(outcome);
        status
    });
    kuna_jvm::kuna_driver::trace(|| "cli end".into());
    status
}

/// `relative` (an engine output path) as a path under the export directory:
/// every component kept except ones that would leave it.
fn safe_relative(relative: &str) -> PathBuf {
    let mut out = PathBuf::new();
    for part in relative.split('/') {
        let ok = !part.is_empty()
            && !part.contains(['\\', ':', '\0'])
            && matches!(Path::new(part).components().next(), Some(Component::Normal(_)));
        out.push(if ok { part.to_string() } else { "_".repeat(part.len().max(1)) });
    }
    out
}

fn write_tree(args: &Args, out_dir: &Path, outcome: &Outcome) -> i32 {
    let mut written: BTreeSet<PathBuf> = BTreeSet::new();
    let (mut java_files, mut kotlin_files, mut bytes) = (0usize, 0usize, 0usize);
    let (mut classes_ok, mut classes_failed) = (0usize, 0usize);
    let (mut methods_ok, mut methods_failed) = (0usize, 0usize);
    let mut write = |relative: PathBuf, text: &str| -> Result<(), String> {
        if !written.insert(relative.clone()) {
            eprintln!("warning: {} is written twice; keeping the first", relative.display());
            return Ok(());
        }
        let path = out_dir.join(&relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        std::fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        if relative.extension().is_some_and(|e| e == "kt") {
            kotlin_files += 1;
        } else {
            java_files += 1;
        }
        bytes += text.len();
        Ok(())
    };
    let stub = |class: &str, why: &str| {
        format!(
            "// kuna: could not decompile {}: {}\n",
            java_name(class),
            why.replace('\n', " ")
        )
    };
    let mut failures: Vec<(String, String)> = Vec::new();
    for result in &outcome.results {
        let written = match &result.unit {
            Some(unit) => {
                for method in &unit.methods {
                    if method.error.is_some() {
                        methods_failed += 1;
                    } else {
                        methods_ok += 1;
                    }
                }
                if let Some(why) = &unit.error {
                    classes_failed += 1;
                    failures.push((result.qualified_name.clone(), why.clone()));
                } else {
                    classes_ok += 1;
                }
                write(safe_relative(&unit.path), &unit.content)
            }
            None => {
                let why = result.error.clone().unwrap_or_else(|| "no output".into());
                classes_failed += 1;
                failures.push((result.qualified_name.clone(), why.clone()));
                write(
                    safe_relative(&format!("{}.java", result.qualified_name)),
                    &stub(&result.qualified_name, &why),
                )
            }
        };
        if let Err(e) = written {
            eprintln!("error: {e}");
            return 1;
        }
    }
    for (name, why) in &outcome.missing {
        classes_failed += 1;
        failures.push((name.clone(), why.clone()));
        if let Err(e) = write(safe_relative(&format!("{name}.java")), &stub(name, why)) {
            eprintln!("error: {e}");
            return 1;
        }
    }
    for (name, why) in &failures {
        eprintln!("warning: could not decompile {}: {why}", java_name(name));
    }
    let files = match (java_files, kotlin_files) {
        (java, 0) => format!("{java} .java files"),
        (0, kotlin) => format!("{kotlin} .kt files"),
        (java, kotlin) => format!("{java} .java files, {kotlin} .kt files"),
    };
    let summary = format!(
        "wrote {}: {files} ({bytes} bytes); classes: {classes_ok} ok, {classes_failed} failed; \
         methods: {methods_ok} ok, {methods_failed} failed\n",
        out_dir.display()
    );
    let status = crate::output::emit_with_status(&summary, i32::from(classes_ok == 0));
    if classes_ok == 0 {
        eprintln!(
            "error: no class of {} could be decompiled{}",
            args.input,
            outcome.error.as_deref().map(|e| format!(": {e}")).unwrap_or_default()
        );
    }
    status
}

#[cfg(test)]
mod tests {
    use super::{safe_relative, write_tree};
    use super::super::args::{parse, Sub};
    use super::super::run::Outcome;
    use kuna_jvm::kuna_driver::ClassResult;
    use kuna_jvm::kuna_input::InputKind;
    use std::path::PathBuf;

    #[test]
    fn a_class_without_output_gets_a_marker_stub_and_fails_the_export() {
        let dir = tempfile::tempdir().unwrap();
        let argv = vec!["app.jar".to_string()];
        let Ok(args) = parse(Sub::Project, &argv, "app.jar", InputKind::Archive) else {
            panic!("parse");
        };
        let outcome = Outcome {
            results: vec![ClassResult {
                index: 0,
                qualified_name: "com/ex/Broken".into(),
                unit: None,
                error: Some("the engine failed: boom".into()),
            }],
            missing: vec![("com/ex/Gone".into(), "not a top-level class of the input".into())],
            error: None,
        };
        assert_eq!(write_tree(&args, dir.path(), &outcome), 1);
        let broken = std::fs::read_to_string(dir.path().join("com/ex/Broken.java")).unwrap();
        assert_eq!(broken, "// kuna: could not decompile com.ex.Broken: the engine failed: boom\n");
        let gone = std::fs::read_to_string(dir.path().join("com/ex/Gone.java")).unwrap();
        let marker = regex::Regex::new(r"\$VF: Couldn't be decompiled|kuna: (?:could not|couldn't) decompile").unwrap();
        assert!(marker.is_match(&gone), "{gone}");
    }

    #[test]
    fn export_paths_stay_inside_the_directory() {
        assert_eq!(safe_relative("com/ex/Main.java"), PathBuf::from("com/ex/Main.java"));
        assert_eq!(safe_relative("../../etc/x.java"), PathBuf::from("__/__/etc/x.java"));
        assert_eq!(safe_relative("/abs/x.java"), PathBuf::from("_/abs/x.java"));
        assert_eq!(safe_relative("a//b.java"), PathBuf::from("a/_/b.java"));
    }
}
