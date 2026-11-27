//! `kuna decompile-project` for a managed input: a source tree.
//!
//! Each root type is written to the engine's own output path (for the JVM,
//! `DIR/<package path>/<Class>.java`, or `<Class>.kt` for a class the Kotlin
//! plugin wrote; nested types stay inside). A type the engine produced no
//! text for gets a stub at `<name>.<extension>`, holding
//! `// kuna: could not decompile <type>: <reason>`. The summary line counts
//! the files of each extension, types the engine decompiled or not, members
//! whose text holds a failure (their own or a body written inside them,
//! `failures`), and, when there are any, failed bodies outside every member.
//! The command exits 0 when at least one type was decompiled.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use super::args::Args;
use super::failures::{self, Units};
use super::run::{self, Outcome};

pub(crate) fn run(args: &Args) -> i32 {
    let mut program = match run::open(args) {
        Ok(program) => program,
        Err(status) => return status,
    };
    let mut wanted: Option<BTreeSet<String>> = None;
    for text in args.functions.iter().flatten() {
        match program.resolve(text) {
            Ok(target) => {
                wanted.get_or_insert_with(BTreeSet::new).insert(target.owner().to_string());
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
    if let Err(status) = run::prepare(args, &mut *program) {
        return status;
    }
    let inventory = program.inventory();
    let select = wanted
        .as_ref()
        .map(|names| names.iter().flat_map(|c| inventory.chain(c)).collect::<BTreeSet<String>>());
    run::drive(&*program, args, select, &|mut outcome| {
        if wanted.is_some() {
            let keep: BTreeSet<String> = Units::new(&outcome)
                .holding(inventory, wanted.as_ref())
                .iter()
                .map(|r| r.name.clone())
                .collect();
            outcome.results.retain(|r| keep.contains(&r.name));
        }
        let status = write_tree(args, inventory.display, &out_dir, &outcome);
        run::abandon(outcome);
        status
    })
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

fn write_tree(args: &Args, display: fn(&str) -> String, out_dir: &Path, outcome: &Outcome) -> i32 {
    let extension = args.engine.extension();
    let mut written: BTreeSet<PathBuf> = BTreeSet::new();
    let mut files: BTreeMap<String, usize> = BTreeMap::new();
    let mut bytes = 0usize;
    let (mut classes_ok, mut classes_failed) = (0usize, 0usize);
    let (mut methods_ok, mut methods_failed) = (0usize, 0usize);
    let mut outside_failed = 0usize;
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
        let ext = relative.extension().map_or(String::new(), |e| e.to_string_lossy().into_owned());
        *files.entry(ext).or_insert(0) += 1;
        bytes += text.len();
        Ok(())
    };
    let stub = |name: &str, why: &str| {
        format!("// kuna: could not decompile {}: {}\n", display(name), why.replace('\n', " "))
    };
    let stub_path = |name: &str| safe_relative(&format!("{name}.{extension}"));
    let mut failed: Vec<(String, String)> = Vec::new();
    for unit in &outcome.results {
        let written = match &unit.text {
            Some(text) => {
                for member in &unit.members {
                    if member.error.is_some() || !member.nested.is_empty() {
                        methods_failed += 1;
                    } else {
                        methods_ok += 1;
                    }
                }
                outside_failed += failures::outside_members(unit, &unit.name).len();
                if let Some(why) = failures::unit_failure(unit) {
                    classes_failed += 1;
                    failed.push((unit.name.clone(), why));
                } else {
                    classes_ok += 1;
                }
                write(safe_relative(&unit.path), text)
            }
            None => {
                let why = unit.error.clone().unwrap_or_else(|| "no output".into());
                classes_failed += 1;
                failed.push((unit.name.clone(), why.clone()));
                write(stub_path(&unit.name), &stub(&unit.name, &why))
            }
        };
        if let Err(e) = written {
            eprintln!("error: {e}");
            return 1;
        }
    }
    for (name, why) in &outcome.missing {
        classes_failed += 1;
        failed.push((name.clone(), why.clone()));
        if let Err(e) = write(stub_path(name), &stub(name, why)) {
            eprintln!("error: {e}");
            return 1;
        }
    }
    for (name, why) in &failed {
        eprintln!("warning: could not decompile {}: {why}", display(name));
    }
    let files = if files.is_empty() {
        format!("0 .{extension} files")
    } else {
        files.iter().map(|(ext, n)| format!("{n} .{ext} files")).collect::<Vec<_>>().join(", ")
    };
    let outside = if outside_failed > 0 {
        format!("; outside methods: {outside_failed} failed")
    } else {
        String::new()
    };
    let summary = format!(
        "wrote {}: {files} ({bytes} bytes); classes: {classes_ok} ok, {classes_failed} failed; \
         methods: {methods_ok} ok, {methods_failed} failed{outside}\n",
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
    use super::super::{args, toy};
    use super::{safe_relative, write_tree};
    use crate::managed::run::Outcome;
    use kuna_managed::Unit;
    use std::path::PathBuf;

    #[test]
    fn a_type_without_output_gets_a_marker_stub_and_fails_the_export() {
        let dir = tempfile::tempdir().unwrap();
        let argv = vec!["x.toy".to_string()];
        let args = args::parse(&toy::TOY, args::Sub::Project, &argv, "x.toy", &toy::kind()).unwrap();
        let outcome = Outcome {
            results: vec![Unit {
                name: "ex/Broken".into(),
                error: Some("the engine failed: boom".into()),
                ..Unit::default()
            }],
            missing: vec![("ex/Gone".into(), "not a top-level class of the input".into())],
            error: None,
        };
        assert_eq!(write_tree(&args, |n| n.replace('/', "."), dir.path(), &outcome), 1);
        let broken = std::fs::read_to_string(dir.path().join("ex/Broken.toy")).unwrap();
        assert_eq!(broken, "// kuna: could not decompile ex.Broken: the engine failed: boom\n");
        let gone = std::fs::read_to_string(dir.path().join("ex/Gone.toy")).unwrap();
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
