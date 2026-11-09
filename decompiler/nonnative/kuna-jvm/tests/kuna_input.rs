//! Input recognition and shared input sets against the engine's own sources.

use std::path::Path;

use kuna_jvm::context_source::{
    DirectoryContextSource, Entries, IContextSource, JarContextSource, SingleFileContextSource,
};
use kuna_jvm::kuna_input::{
    self, classify, is_class_header, sniff, InputError, InputKind, LoadOptions,
};

use crate::kuna_support::{copy_classes, corpus, make_zip, scratch};

fn write(dir: &Path, name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

fn class_bytes() -> Vec<u8> {
    std::fs::read(corpus("java8/pkg/TestClassLoop.class")).unwrap()
}

#[test]
fn class_headers_and_fat_mach_o() {
    assert!(is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 52]));
    assert!(is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 45]));
    assert!(is_class_header(&[
        0xCA, 0xFE, 0xBA, 0xBE, 0xFF, 0xFF, 0, 69
    ]));
    assert!(is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 255]));
    assert!(!is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 2]));
    assert!(!is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 44]));
    assert!(!is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 1, 0]));
    assert!(!is_class_header(&[0xCA, 0xFE, 0xBA, 0xBF, 0, 0, 0, 52]));
    assert!(!is_class_header(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0]));
    assert!(!is_class_header(&[]));
}

#[test]
fn classify_and_sniff_files() {
    let dir = scratch("input-classify");
    let fat = write(
        &dir,
        "fat",
        &[
            0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 2, 1, 0, 0, 7, 0x80, 0, 0, 3,
        ],
    );
    let fat64 = write(
        &dir,
        "fat64",
        &[0xCA, 0xFE, 0xBA, 0xBF, 0, 0, 0, 2, 0, 0, 0, 0],
    );
    let class = write(&dir, "Main.class", &class_bytes());
    let truncated = write(&dir, "Cut.class", &[0xCA, 0xFE, 0xBA, 0xBE]);
    let empty = write(&dir, "empty.class", &[]);
    let garbage = write(&dir, "garbage.bin", b"\x00\x01garbage that is not anything");
    let elf = write(
        &dir,
        "tool",
        b"\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00",
    );
    let jar = write(&dir, "app.jar", &make_zip(&[("a/B.class", &class_bytes())]));
    let apk = write(
        &dir,
        "app.apk",
        &make_zip(&[("classes.dex", b"dex\n035\0")]),
    );
    let mut launcher = b"#!/bin/sh\nexec java -jar \"$0\" \"$@\"\n".to_vec();
    launcher.extend(make_zip(&[("BOOT-INF/classes/a/B.class", &class_bytes())]));
    let boot = write(&dir, "boot.jar", &launcher);
    let script = write(&dir, "boot.sh", &launcher);
    let jmod = write(&dir, "java.base.jmod", b"JM\x01\x00PK\x05\x06");
    let jimage = write(&dir, "modules", &[0xDA, 0xDA, 0xFE, 0xCA, 1, 0, 0, 0]);
    let dex = write(&dir, "classes.dex", b"dex\n035\0rest");

    let expect = [
        (&fat, InputKind::Other),
        (&fat64, InputKind::Other),
        (&class, InputKind::ClassFile),
        (&truncated, InputKind::Other),
        (&empty, InputKind::Other),
        (&garbage, InputKind::Other),
        (&elf, InputKind::Other),
        (&jar, InputKind::Archive),
        (&apk, InputKind::NoJvmClasses),
        (&boot, InputKind::Archive),
        (&script, InputKind::Other),
        (&jmod, InputKind::Jmod),
        (&jimage, InputKind::JImage),
        (&dex, InputKind::Dex),
    ];
    for (path, kind) in expect {
        assert_eq!(classify(path).unwrap(), kind, "{}", path.display());
        assert_eq!(sniff(path), kind.is_jvm(), "sniff {}", path.display());
    }
    assert_eq!(classify(&dir).unwrap(), InputKind::Directory);
    assert!(sniff(&dir));
    let missing = dir.join("does-not-exist");
    assert!(!sniff(&missing));
    assert!(classify(&missing).is_err());

    let real_fat = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/kuna-analysis/tests/fixtures/macho_fat");
    if real_fat.is_file() {
        assert!(!sniff(&real_fat));
        assert_eq!(classify(&real_fat).unwrap(), InputKind::Other);
    }
}

type Listing = (
    Vec<(String, i32)>,
    Vec<String>,
    Vec<(String, i32)>,
    Vec<String>,
);

fn entry_list(entries: &Entries) -> Listing {
    let pair = |e: &kuna_jvm::context_source::Entry| (e.base_path().to_string(), e.multirelease());
    (
        entries.classes().iter().map(pair).collect(),
        entries.directories().to_vec(),
        entries.others().iter().map(pair).collect(),
        entries
            .child_contexts()
            .iter()
            .map(|c| c.get_name())
            .collect(),
    )
}

#[test]
fn shared_jar_lists_and_reads_like_jar_context_source() {
    let dir = scratch("input-jar");
    let class = class_bytes();
    let jar = write(
        &dir,
        "app.jar",
        &make_zip(&[
            ("META-INF/MANIFEST.MF", b"Manifest-Version: 1.0\n"),
            ("pkg/TestClassLoop.class", &class),
            ("META-INF/versions/11/pkg/TestClassLoop.class", &class),
            ("BOOT-INF/classes/pkg/Other.class", &class),
            ("res/data.txt", b"hello"),
        ]),
    );
    let set = kuna_input::load(&jar, &LoadOptions::default()).unwrap();
    assert_eq!(set.kind(), InputKind::Archive);
    let views = set.sources();
    assert_eq!(views.len(), 1);
    let original = JarContextSource::new(&jar);
    assert_eq!(views[0].get_name(), original.get_name());
    assert_eq!(
        entry_list(&views[0].get_entries()),
        entry_list(&original.get_entries())
    );
    for resource in [
        "pkg/TestClassLoop.class",
        "META-INF/versions/11/pkg/TestClassLoop.class",
        "BOOT-INF/classes/pkg/Other.class",
        "res/data.txt",
    ] {
        assert_eq!(
            views[0].get_input_stream(resource),
            original.get_input_stream(resource),
            "{resource}"
        );
    }
    let classes: Vec<&str> = set.classes().iter().map(|c| c.entry).collect();
    assert_eq!(classes, ["pkg/TestClassLoop", "BOOT-INF/classes/pkg/Other"]);
    assert_eq!(set.class_bytes(), 2 * class.len());
}

#[test]
fn shared_directory_lists_like_directory_context_source() {
    let dir = scratch("input-dir");
    copy_classes(
        &dir,
        &[
            "java8/pkg/TestSuperInner.class",
            "java8/pkg/TestSuperInner$Inner2.class",
            "java8/pkg/TestSuperInnerBase.class",
            "java8/pkg/TestSuperInnerBase$Inner.class",
        ],
    );
    write(&dir, "notes.txt", b"not a class");
    write(
        &dir,
        "lib.jar",
        &make_zip(&[("ext/X.class", &class_bytes())]),
    );
    let set = kuna_input::load(&dir, &LoadOptions::default()).unwrap();
    assert_eq!(set.kind(), InputKind::Directory);
    let views = set.sources();
    let original = DirectoryContextSource::new(&dir);
    let (mine, theirs) = (views[0].get_entries(), original.get_entries());
    assert_eq!(entry_list(&mine), entry_list(&theirs));
    assert_eq!(
        entry_list(&mine.child_contexts()[0].get_entries()),
        entry_list(&theirs.child_contexts()[0].get_entries())
    );
    assert_eq!(
        views[0].get_input_stream("pkg/TestSuperInner.class"),
        original.get_input_stream("pkg/TestSuperInner.class")
    );
    assert_eq!(
        views[0].get_input_stream("notes.txt").unwrap(),
        b"not a class"
    );
    let classes: Vec<&str> = set.classes().iter().map(|c| c.entry).collect();
    assert_eq!(
        classes,
        [
            "pkg/TestSuperInner$Inner2",
            "pkg/TestSuperInner",
            "pkg/TestSuperInnerBase$Inner",
            "pkg/TestSuperInnerBase",
            "ext/X"
        ]
    );
}

#[test]
fn single_class_with_siblings() {
    let main = corpus("java8/pkg/TestAnonymousClassNaming.class");
    let set = kuna_input::load(&main, &LoadOptions::default()).unwrap();
    assert_eq!(set.kind(), InputKind::ClassFile);
    let names: Vec<String> = set.sources().iter().map(|s| s.get_name()).collect();
    let files: Vec<&str> = names
        .iter()
        .map(|n| n.rsplit('/').next().unwrap())
        .collect();
    assert_eq!(
        files,
        [
            "TestAnonymousClassNaming.class",
            "TestAnonymousClassNaming$1.class",
            "TestAnonymousClassNaming$2.class",
            "TestAnonymousClassNaming$InnerClass$1.class",
            "TestAnonymousClassNaming$InnerClass.class",
        ]
    );
    let original = SingleFileContextSource::new(&main);
    assert_eq!(names[0], original.get_name());
    assert_eq!(
        entry_list(&set.sources()[0].get_entries()),
        entry_list(&original.get_entries())
    );
    let alone = kuna_input::load(&main, &LoadOptions { siblings: false }).unwrap();
    assert_eq!(alone.sources().len(), 1);
}

#[test]
fn load_errors() {
    let dir = scratch("input-errors");
    let apk = write(
        &dir,
        "app.apk",
        &make_zip(&[("classes.dex", b"dex\n035\0")]),
    );
    let err = kuna_input::load(&apk, &LoadOptions::default()).unwrap_err();
    assert!(matches!(
        err,
        InputError::Unsupported(_, InputKind::NoJvmClasses)
    ));
    assert!(
        err.to_string().contains("DEX bytecode is not JVM bytecode"),
        "{err}"
    );
    let jmod = write(&dir, "x.jmod", b"JM\x01\x00");
    assert!(matches!(
        kuna_input::load(&jmod, &LoadOptions::default()),
        Err(InputError::Unsupported(_, InputKind::Jmod))
    ));
    let empty = scratch("input-errors/empty-dir");
    let err = kuna_input::load(&empty, &LoadOptions::default()).unwrap_err();
    assert!(matches!(err, InputError::NoClasses(_)), "{err}");
    let elf = write(&dir, "tool", b"\x7fELF\x02\x01\x01\x00");
    assert!(matches!(
        kuna_input::load(&elf, &LoadOptions::default()),
        Err(InputError::Unsupported(_, InputKind::Other))
    ));
}

#[test]
fn a_library_without_classes_is_empty_and_an_input_without_classes_fails() {
    let dir = scratch("input-classless-library");
    let jar = write(
        &dir,
        "kmp-root.jar",
        &make_zip(&[
            ("META-INF/MANIFEST.MF", b"Manifest-Version: 1.0\n"),
            ("commonMain/default/manifest", b"unique_name=x\n"),
        ]),
    );
    let empty = scratch("input-classless-library/empty-dir");
    for path in [&jar, &empty] {
        let set = kuna_input::load_library(path).unwrap();
        assert!(set.classes().is_empty(), "{}", path.display());
        assert!(set.sources().is_empty(), "{}", path.display());
        assert!(kuna_input::load(path, &LoadOptions::default()).is_err(), "{}", path.display());
    }
    let library = write(&dir, "lib.jar", &make_zip(&[("a/B.class", &class_bytes())]));
    assert_eq!(kuna_input::load_library(&library).unwrap().classes().len(), 1);
    let elf = write(&dir, "tool", b"\x7fELF\x02\x01\x01\x00");
    assert!(matches!(
        kuna_input::load_library(&elf),
        Err(InputError::Unsupported(_, InputKind::Other))
    ));
}
