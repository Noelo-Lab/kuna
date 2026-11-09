//! Shared helpers for the class-file source tests (zip, context sources, JDK runtime).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::rc::Rc;

use kuna_jvm::context_source::{
    DirectoryContextSource, IContextSource, JarContextSource, SingleFileContextSource,
};
use kuna_jvm::jdk_runtime::JdkRuntime;
use kuna_jvm::rt::try_catch;

pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/f6")
}

pub fn data(name: &str) -> PathBuf {
    data_dir().join(name)
}

/// A per-purpose directory under the cargo target tmp dir, reused across runs.
pub fn tmp_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("kuna-jvm-f6")
        .join(name);
    std::fs::create_dir_all(&dir).expect("create tmp dir");
    dir
}

pub fn skip(test: &str, why: &str) {
    eprintln!("note: {test} skipped: {why}");
}

fn on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(name))
        .find(|p| p.is_file())
}

/// An executable from the JDK at `home` or, failing that, from `PATH`.
pub fn tool(home: Option<&Path>, name: &str) -> Option<PathBuf> {
    home.map(|h| h.join("bin").join(name))
        .filter(|p| p.is_file())
        .or_else(|| on_path(name))
}

pub fn run(program: &Path, args: &[&str]) -> Output {
    Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("run {}: {e}", program.display()))
}

/// The JDK the runtime tests read: `KUNA_JVM_TEST_JDK`, else the one
/// `include-runtime=current` finds.
pub fn jdk_home() -> Option<PathBuf> {
    std::env::var_os("KUNA_JVM_TEST_JDK")
        .map(PathBuf::from)
        .or_else(|| JdkRuntime::locate("current"))
        .filter(|h| h.join("lib/modules").is_file())
}

pub fn vineflower_jar() -> Option<PathBuf> {
    let explicit = std::env::var_os("VINEFLOWER_JAR").map(PathBuf::from);
    let default = std::env::var_os("HOME")
        .map(|h| PathBuf::from(h).join("kwt/jvm/oracle/vineflower-1.12.0.jar"));
    explicit.into_iter().chain(default).find(|p| p.is_file())
}

/// `StructContext.addSpace(File)`'s choice of source for a path.
pub fn source_for(path: &Path) -> Rc<dyn IContextSource> {
    let name = path.to_string_lossy();
    if path.is_dir() {
        Rc::new(DirectoryContextSource::new(path))
    } else if name.ends_with(".jar") || name.ends_with(".zip") {
        Rc::new(JarContextSource::new(path))
    } else {
        Rc::new(SingleFileContextSource::new(path))
    }
}

fn caught_class(caught: &kuna_jvm::rt::Caught) -> String {
    caught
        .exception
        .as_ref()
        .map_or_else(|| "RustPanic".to_string(), |e| e.class.to_string())
}

/// What `VfDump` prints: sources are registered like `StructContext.addSpace`
/// (each unit, then its child contexts depth-first), then every unit's entries.
pub fn dump_units(roots: Vec<Rc<dyn IContextSource>>) -> Vec<String> {
    fn register(
        source: Rc<dyn IContextSource>,
        root: bool,
        units: &mut Vec<(Rc<dyn IContextSource>, bool)>,
        out: &mut Vec<String>,
    ) {
        units.push((source.clone(), root));
        match try_catch(|| source.get_entries()) {
            Ok(entries) => {
                for child in entries.child_contexts() {
                    register(child.clone(), false, units, out);
                }
            }
            Err(caught) => out.push(format!("THROW {}", caught_class(&caught))),
        }
    }
    let mut out = Vec::new();
    let mut units = Vec::new();
    for root in roots {
        register(root, true, &mut units, &mut out);
    }
    for (source, root) in units {
        out.push(format!("UNIT {}", source.get_name()));
        let entries = match try_catch(|| source.get_entries()) {
            Ok(entries) => entries,
            Err(caught) => {
                out.push(format!("THROW {}", caught_class(&caught)));
                continue;
            }
        };
        out.push(format!("R {} {}", root, source.is_lazy()));
        for e in entries.classes() {
            out.push(format!("C {} {}", e.multirelease(), e.base_path()));
        }
        for d in entries.directories() {
            out.push(format!("D {d}"));
        }
        for e in entries.others() {
            out.push(format!("O {} {}", e.multirelease(), e.base_path()));
        }
        out.push(format!("K {}", entries.child_contexts().len()));
    }
    out
}

/// Split a dump into units (name line plus body), body sorted when `sort` holds.
pub fn units_of(lines: &[String], sort: bool) -> Vec<(String, Vec<String>)> {
    let mut units: Vec<(String, Vec<String>)> = Vec::new();
    for line in lines {
        if line.starts_with("UNIT ") {
            units.push((line.clone(), Vec::new()));
        } else if let Some(last) = units.last_mut() {
            last.1.push(line.clone());
        } else {
            units.push((String::new(), vec![line.clone()]));
        }
    }
    if sort {
        for unit in &mut units {
            unit.1.sort();
        }
        units.sort();
    }
    units
}

pub fn golden(name: &str) -> Vec<String> {
    let text = std::fs::read_to_string(data(&format!("golden/{name}.txt"))).expect("golden");
    let dir = data_dir().to_string_lossy().into_owned();
    text.lines().map(|l| l.replace("$DATA", &dir)).collect()
}
