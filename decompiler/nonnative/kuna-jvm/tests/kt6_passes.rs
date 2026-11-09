//! The Kotlin plugin's per-method pass schedule (`KotlinPlugin.makePass`/`makeCfgPass`) on real
//! Kotlin classes, step by step against Vineflower 1.12.0 running its own Kotlin plugin
//! (`data/kt6/gen.sh`: `tools/vftrace` with `--hash index --plugins`). The kuna side swaps the
//! plugin's chooser and writer for an always-yes chooser and the Java writer (KotlinChooser and
//! KotlinWriter are other slices'); the passes before `ReplaceExprs` are compared: they include
//! ObfuscatedExceptions, Finally, StackVars, EnhanceLoops (`KMergePass`), EliminateDead and
//! ReplaceStats.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use kuna_jvm::class_writer::ClassWriter;
use kuna_jvm::dom_helper::DomHelper;
use kuna_jvm::fernflower::Fernflower;
use kuna_jvm::i_fernflower_logger::CollectingLogger;
use kuna_jvm::kotlin_plugin::KotlinPlugin;
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::kuna_trace;
use kuna_jvm::language_spec::LanguageSpec;
use kuna_jvm::plugin::Plugin;
use kuna_jvm::plugin_source::PluginSource;
use kuna_jvm::rt::try_catch;
use kuna_jvm::struct_class::StructClass;

/// `KotlinTests`' "Entire Classpath" options over the fixture base.
const OPTIONS: [(&str, &str); 14] = [
    ("log-level", "warn"),
    ("decompile-generics", "1"),
    ("remove-synthetic", "0"),
    ("remove-bridge", "1"),
    ("keep-literals", "1"),
    ("__unit_test_mode__", "1"),
    ("new-line-separator", "1"),
    ("error-message", ""),
    ("include-runtime", ""),
    ("bytecode-source-mapping", "1"),
    ("__dump_original_lines__", "1"),
    ("dump-exception-on-error", "0"),
    ("ignore-invalid-bytecode", "1"),
    ("verify-anonymous-classes", "1"),
];

/// Steps from here on belong to other slices (or come after the pipeline).
const LATER: [&str; 4] = [
    "ReplaceExprs",
    "ResugarMethods",
    "CollapseStringConcat",
    "ReplaceContinue",
];

struct KtPipe;

impl Plugin for KtPipe {
    fn id(&self) -> String {
        "KtPipe".to_string()
    }

    fn description(&self) -> String {
        "The Kotlin pass schedule with the Java writer.".to_string()
    }

    fn get_language_spec(&self) -> Option<LanguageSpec> {
        let kotlin = KotlinPlugin::new()
            .get_language_spec()
            .expect("Kotlin language spec");
        Some(LanguageSpec::new(
            "kotlin",
            Rc::new(|_: StructClass| true),
            Rc::new(DomHelper),
            Rc::new(ClassWriter::new()),
            kotlin.pass,
            kotlin.cfg_pass,
            kotlin.root_processor,
            "kt",
        ))
    }
}

struct KtPipeSource;

impl PluginSource for KtPipeSource {
    fn find_plugins(&self) -> Vec<Rc<dyn Plugin>> {
        vec![Rc::new(KtPipe)]
    }
}

fn corpus_classes() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt6/classes")
}

/// kuna's trace of the Kotlin schedule over `classes` (one unit, in order) with `libs`.
fn kuna_trace_of(classes: &[PathBuf], libs: &[PathBuf], name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("kt6-{name}-{}.vft", std::process::id()));
    let classes = classes.to_vec();
    let libs = libs.to_vec();
    let trace = path.clone();
    with_engine_stack(move || {
        reset_engine_thread();
        let options: Vec<(String, String)> = OPTIONS
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let mut fernflower = Fernflower::new_with_plugin_sources(
            &options,
            Rc::new(CollectingLogger::new()),
            vec![Rc::new(KtPipeSource)],
        );
        for lib in &libs {
            fernflower.add_library_file(lib);
        }
        for class in &classes {
            fernflower.add_source_file(class);
        }
        kuna_trace::start(&trace, None).expect("trace file");
        let run = try_catch(|| fernflower.decompile_context(&mut |_| {}));
        kuna_trace::finish();
        fernflower.clear_context();
        reset_engine_thread();
        if let Err(e) = run {
            panic!("decompilation stopped: {}", e.message);
        }
    });
    std::fs::read_to_string(&path).expect("kuna trace")
}

/// Per method key, the steps `(name, dump)` with `= n` back-references resolved.
fn parse_trace(text: &str) -> BTreeMap<String, Vec<(String, String)>> {
    let mut methods: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut method: Option<String> = None;
    let mut step: Option<(String, Vec<&str>)> = None;

    fn close(
        methods: &mut BTreeMap<String, Vec<(String, String)>>,
        method: &Option<String>,
        step: Option<(String, Vec<&str>)>,
    ) {
        let (Some(method), Some((name, lines))) = (method, step) else {
            return;
        };
        let steps = methods.entry(method.clone()).or_default();
        let dump = match lines.as_slice() {
            [single] if single.starts_with("= ") => {
                let n: usize = single[2..].trim().parse().expect("back-reference");
                steps[n].1.clone()
            }
            _ => lines.join("\n"),
        };
        steps.push((name, dump));
    }

    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        if let Some(key) = line.strip_prefix("=M ") {
            close(&mut methods, &method, step.take());
            method = Some(key.to_string());
        } else if let Some(header) = line.strip_prefix("@ ") {
            close(&mut methods, &method, step.take());
            let name = header.split_once(' ').map_or("", |(_, name)| name);
            step = Some((name.to_string(), Vec::new()));
        } else if let Some((_, lines)) = step.as_mut() {
            lines.push(line);
        }
    }
    close(&mut methods, &method, step.take());
    methods
}

/// The steps before the first step of a later slice's pass.
fn prefix(steps: &[(String, String)]) -> &[(String, String)] {
    let end = steps
        .iter()
        .position(|(name, _)| LATER.iter().any(|later| name.starts_with(later)))
        .unwrap_or(steps.len());
    &steps[..end]
}

/// Every method of the Vineflower trace, compared step by step up to `ReplaceExprs`: the
/// differences, and the number of steps compared.
fn compare(vf: &str, kuna: &str) -> (Vec<String>, usize) {
    let vf = parse_trace(vf);
    let kuna = parse_trace(kuna);
    let mut diffs = Vec::new();
    let mut steps = 0;
    for (key, vf_steps) in vf.iter() {
        let vf_steps = prefix(vf_steps);
        let kuna_steps = prefix(kuna.get(key).map_or(&[][..], |s| s.as_slice()));
        if vf_steps.len() != kuna_steps.len() {
            diffs.push(format!(
                "{key}: {} steps, kuna {} ({:?} vs {:?})",
                vf_steps.len(),
                kuna_steps.len(),
                vf_steps.iter().map(|s| &s.0).collect::<Vec<_>>(),
                kuna_steps.iter().map(|s| &s.0).collect::<Vec<_>>()
            ));
            continue;
        }
        steps += vf_steps.len();
        for (n, (a, b)) in vf_steps.iter().zip(kuna_steps).enumerate() {
            if a != b {
                diffs.push(format!(
                    "{key}: step {n} {} / {}\n--- vineflower\n{}\n--- kuna\n{}",
                    a.0, b.0, a.1, b.1
                ));
                break;
            }
        }
    }
    (diffs, steps)
}

/// `data/kt6/<case>.vft` against kuna on `classes`; `covers` are steps (or dump text) the
/// vector must contain, so that it keeps exercising what the test is named after.
fn check(case: &str, classes: &[&str], covers: &[&str]) {
    let vf = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/kt6")
            .join(format!("{case}.vft")),
    )
    .expect("Vineflower trace vector");
    for cover in covers {
        assert!(vf.contains(cover), "{case}: the vector has no {cover:?}");
    }
    let classes: Vec<PathBuf> = classes.iter().map(|c| corpus_classes().join(c)).collect();
    let kuna = kuna_trace_of(&classes, &[], case);
    let (diffs, steps) = compare(&vf, &kuna);
    assert!(diffs.is_empty(), "{case}:\n{}", diffs.join("\n"));
    assert!(steps > 0, "{case}: nothing compared");
}

#[test]
fn synchronized_blocks_and_dead_unit_stores() {
    check(
        "TestSynchronized",
        &["TestSynchronized.class"],
        &[" EliminateDead\n", " RemoveSynchronized\n"],
    );
}

#[test]
fn finally_blocks() {
    check(
        "TestTryFinally",
        &["TestTryFinally.class"],
        &[" Finally\n", " ReplaceStats\n", " CatchAll "],
    );
}

#[test]
fn for_each_loops() {
    check(
        "TestFunVarargs",
        &["TestFunVarargs.class"],
        &["loop=FOR_EACH", " ReplaceStats\n"],
    );
}

#[test]
fn when_switches() {
    check(
        "TestWhen",
        &["TestWhen.class"],
        &[" Switch ", " ReplaceStats\n"],
    );
}

/// Every top-level class of `KT6_SWEEP_CLASSES` (a directory of Kotlin classes) with its nested
/// classes against `KT6_SWEEP_VF/<Class>.vft` (Vineflower's trace of the same unit, as `gen.sh`
/// makes it), with the libraries of `KT6_SWEEP_LIBS` (space-separated, as `gen.sh`'s `LIBS`):
/// `cargo test -p kuna-jvm --test integration kt6_passes::sweep -- --ignored`.
#[test]
#[ignore]
fn sweep() {
    let (Ok(classes), Ok(vf_dir)) = (
        std::env::var("KT6_SWEEP_CLASSES"),
        std::env::var("KT6_SWEEP_VF"),
    ) else {
        panic!("set KT6_SWEEP_CLASSES and KT6_SWEEP_VF");
    };
    let libs: Vec<PathBuf> = std::env::var("KT6_SWEEP_LIBS")
        .unwrap_or_default()
        .split_whitespace()
        .map(PathBuf::from)
        .collect();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&classes)
        .expect("class directory")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "class"))
        .collect();
    files.sort();
    let mut failures = Vec::new();
    let mut compared = 0;
    let mut steps = 0;
    let mut replaced = 0;
    for top in files.iter().filter(|p| !p.to_string_lossy().contains('$')) {
        let stem = top.file_stem().unwrap().to_string_lossy().to_string();
        let Ok(vf) = std::fs::read_to_string(Path::new(&vf_dir).join(format!("{stem}.vft"))) else {
            continue;
        };
        let unit: Vec<PathBuf> = files
            .iter()
            .filter(|p| {
                let name = p.file_stem().unwrap().to_string_lossy();
                name == stem || name.starts_with(&format!("{stem}$"))
            })
            .cloned()
            .collect();
        let kuna = kuna_trace_of(&unit, &libs, &stem);
        compared += 1;
        let (diffs, n) = compare(&vf, &kuna);
        steps += n;
        replaced += parse_trace(&kuna)
            .values()
            .filter(|s| s.iter().any(|(name, _)| name == "ReplaceStats"))
            .count();
        if !diffs.is_empty() {
            failures.push(format!("== {stem}\n{}", diffs.join("\n")));
        }
    }
    eprintln!(
        "kt6 sweep: {compared} units, {steps} steps, {replaced} kuna methods reach ReplaceStats, {} units differ",
        failures.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
