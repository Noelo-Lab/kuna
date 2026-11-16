//! A caller can scan its spec roots into one `LanguageDatabase` and hand it to
//! every bootstrap: images of different languages load from the one database,
//! on several threads at once, without scanning the roots again, and decompile
//! as the same bootstraps over the spec roots do.

use std::path::PathBuf;

use kuna_base::error::KunaResult;
use kuna_console::engine::{language_database, Bootstrap, ConsoleProgram};
use kuna_console::project::{decompile_entry, DecompileOptions};
use kuna_decomp::kuna_loadoptions::LoadOptions;
use kuna_decomp::kuna_memoryspecs::SpecRoot;
use kuna_decomp::sleigh_arch::LanguageDatabase;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs() -> [SpecRoot; 1] {
    [repo_root().join("specs").into()]
}

fn fixture(name: &str) -> String {
    let path = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    path.to_str().unwrap().to_string()
}

/// Two images in different languages, with the functions each decompiles.
const IMAGES: [(&str, &str, &[&str]); 2] = [
    ("fauxware", "x86:LE:64:default", &["authenticate", "main"]),
    ("adrnext_aarch64", "AARCH64:LE:64:v8A", &["audit_calls", "main"]),
];

fn decompile(mut prog: ConsoleProgram, funcs: &[&str]) -> (String, Vec<String>) {
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let c = funcs
        .iter()
        .map(|func| {
            let entry = prog.find_entry_by_name(func).expect("function in fixture");
            decompile_entry(&mut prog, entry, &DecompileOptions::default())
                .code
                .expect("decompile succeeds")
        })
        .collect();
    (prog.arch().archid.clone(), c)
}

fn each_image(
    load: impl Fn(&str) -> KunaResult<ConsoleProgram> + Sync,
) -> Vec<(String, Vec<String>)> {
    // One thread per image, all reading the same database at once.
    std::thread::scope(|scope| {
        let threads: Vec<_> = IMAGES
            .iter()
            .map(|(name, _, funcs)| {
                let load = &load;
                scope.spawn(move || decompile(load(&fixture(name)).expect("bootstrap"), funcs))
            })
            .collect();
        threads.into_iter().map(|thread| thread.join().unwrap()).collect()
    })
}

#[test]
fn one_database_serves_images_of_two_languages() {
    let db = language_database(&specs()).expect("scan specs");
    let options = LoadOptions::default();
    let shared = each_image(|path| Bootstrap::new(&db).options(&options).object(path));
    let rescanned = each_image(|path| Bootstrap::new(&specs()).options(&options).object(path));
    for ((name, language, _), (archid, _)) in IMAGES.iter().zip(&shared) {
        assert!(archid.starts_with(language), "{name} loaded as {archid}");
    }
    assert!(shared[0].1[1].contains("authenticate"), "{}", shared[0].1[1]);
    assert_eq!(shared, rescanned);
}

#[test]
fn the_image_dispatch_takes_a_database() {
    let db = language_database(&specs()).expect("scan specs");
    let options = LoadOptions::default();
    let shared = each_image(|path| Bootstrap::new(&db).options(&options).image(path));
    let rescanned = each_image(|path| Bootstrap::new(&specs()).options(&options).image(path));
    assert_eq!(shared, rescanned);
}

#[test]
fn a_database_without_the_language_refuses_the_image() {
    let db = language_database(&[repo_root().join("no-such-specs")]).expect("an empty root scans");
    let path = fixture("fauxware");
    let err = match Bootstrap::new(&db).options(&LoadOptions::default()).object(&path) {
        Ok(_) => panic!("bootstrapped from an empty database"),
        Err(err) => err.to_string(),
    };
    assert!(err.contains("No sleigh specification"), "{err}");
}

#[test]
fn a_bootstrap_over_a_database_does_not_rescan_the_roots() {
    let languages = "Ghidra/Processors/x86/data/languages";
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("kuna_language_database_{}", std::process::id()));
    let copy = root.join(languages);
    std::fs::create_dir_all(&copy).unwrap();
    for entry in std::fs::read_dir(repo_root().join("specs").join(languages)).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            std::fs::copy(entry.path(), copy.join(entry.file_name())).unwrap();
        }
    }
    let roots: [SpecRoot; 1] = [root.clone().into()];
    let db = language_database(&roots).expect("scan the copy");
    std::fs::remove_file(copy.join("x86.ldefs")).unwrap();

    let path = fixture("fauxware");
    let options = LoadOptions::default();
    let loaded = Bootstrap::new(&db).options(&options).object(&path);
    let rescanned = Bootstrap::new(&roots).options(&options).object(&path);
    std::fs::remove_dir_all(&root).unwrap();
    let archid = loaded.expect("bootstrap over the database").arch().archid.clone();
    assert!(archid.starts_with("x86:LE:64:default"), "{archid}");
    let err = match rescanned {
        Ok(_) => panic!("bootstrapped without x86.ldefs"),
        Err(err) => err.to_string(),
    };
    assert!(err.contains("No sleigh specification"), "{err}");
}

#[test]
fn a_database_is_shareable_across_threads() {
    fn shareable<T: Send + Sync>() {}
    shareable::<LanguageDatabase>();
}
