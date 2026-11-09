//! The jar, directory and single-file context sources against Vineflower's own
//! (golden/*.txt, recorded by VfDump.java through `StructContext.addSpace`).

use std::path::Path;
use std::rc::Rc;

use kuna_jvm::context_source::{
    java_parse_int, read_class_info, DirectoryContextSource, Entries, Entry, IContextSource,
    JarContextSource, SingleFileContextSource, CLASS_SUFFIX,
};
use kuna_jvm::rt::try_catch;

use crate::f6_support::{data, dump_units, golden, source_for, units_of};

fn dump_paths(paths: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    let mut roots = Vec::new();
    for p in paths {
        match try_catch(|| source_for(&data(p))) {
            Ok(source) => roots.push(source),
            Err(caught) => out.push(format!(
                "THROW {}",
                caught.exception.map_or("RustPanic", |e| e.class)
            )),
        }
    }
    out.extend(dump_units(roots));
    out
}

#[test]
fn jar_sources_match_vineflower() {
    for (golden_name, file) in [
        ("mixed", "mixed.jar"),
        ("mr", "mr.jar"),
        ("mrtool", "mrtool.jar"),
        ("dup", "dup.zip"),
        ("prefixed", "prefixed.zip"),
        ("trailing", "trailing.zip"),
        ("comment", "comment.zip"),
        ("zip64", "zip64.zip"),
        ("inner", "inner.jar"),
        ("badver", "badver.jar"),
    ] {
        assert_eq!(dump_paths(&[file]), golden(golden_name), "{file}");
    }
}

#[test]
fn directory_source_matches_vineflower_up_to_listing_order() {
    let mine = dump_paths(&["tree"]);
    let theirs = golden("tree");
    assert_eq!(units_of(&mine, true), units_of(&theirs, true));
    let first = &units_of(&mine, false)[0].1;
    let dirs: Vec<&String> = first.iter().filter(|l| l.starts_with("D ")).collect();
    assert_eq!(dirs, ["D ", "D libs", "D pkg", "D pkg/util", "D res"]);
}

#[test]
fn single_file_sources_match_vineflower() {
    let mine = dump_paths(&[
        "tree/pkg/Hello.class",
        "tree/readme.txt",
        "tree/missing.class",
    ]);
    assert_eq!(mine, golden("single"));
}

#[test]
fn entry_parse_follows_integer_parse_int() {
    let cases = [
        ("pkg/A", "pkg/A", -1),
        ("META-INF/versions/9/a/B", "/a/B", 9),
        ("META-INF/versions/09/a/B", "/a/B", 9),
        ("META-INF/versions/+12/x", "/x", 12),
        ("META-INF/versions/-1/x", "/x", -1),
        ("META-INF/versions/\u{0669}\u{0661}/x", "/x", 91),
        ("META-INF/versions/\u{FF11}\u{FF11}/x", "/x", 11),
        ("META-INF/versions/x/y", "META-INF/versions/x/y", -1),
        ("META-INF/versions//y", "META-INF/versions//y", -1),
        ("META-INF/versions/9", "META-INF/versions/9", -1),
        ("META-INF/versions/", "META-INF/versions/", -1),
        (
            "META-INF/versions/2147483648/y",
            "META-INF/versions/2147483648/y",
            -1,
        ),
        (
            "META-INF/versions/\u{1D7D7}/y",
            "META-INF/versions/\u{1D7D7}/y",
            -1,
        ),
    ];
    for (path, base, version) in cases {
        let e = Entry::parse(path);
        assert_eq!((e.base_path(), e.multirelease()), (base, version), "{path}");
    }
    let e = Entry::parse("META-INF/versions/11/a/B");
    assert_eq!(e.path(), "META-INF/versions/11//a/B");
    assert_eq!(e.to_string(), "E[/a/B]");
    for bad in [
        "META-INF/versions/8/x",
        "META-INF/versions/0/x",
        "META-INF/versions/-2/x",
    ] {
        let caught = try_catch(|| Entry::parse(bad)).unwrap_err();
        assert!(caught.is("IllegalArgumentException"), "{bad}");
    }
}

#[test]
fn java_parse_int_vectors() {
    let ok = [
        ("0", 0),
        ("-0", 0),
        ("+7", 7),
        ("2147483647", i32::MAX),
        ("-2147483648", i32::MIN),
        ("\u{0966}\u{0967}", 1),
    ];
    for (s, v) in ok {
        assert_eq!(java_parse_int(s), Some(v), "{s}");
    }
    for s in [
        "",
        "+",
        "-",
        "2147483648",
        "-2147483649",
        "1_0",
        " 1",
        "1 ",
        "0x10",
        "١٢a",
    ] {
        assert_eq!(java_parse_int(s), None, "{s:?}");
    }
}

#[test]
fn jar_source_reads_like_zipfile() {
    let jar = JarContextSource::new(&data("mixed.jar"));
    assert_eq!(
        jar.get_class_bytes("pkg/Hello").unwrap(),
        std::fs::read(data("tree/pkg/Hello.class")).unwrap()
    );
    assert!(jar.has_class("pkg/util/Util"));
    assert_eq!(jar.get_input_stream("pkg").unwrap(), Vec::<u8>::new());
    let entry = jar.get_entries().others()[0].clone();
    assert_eq!(
        jar.get_input_stream_entry(&entry).unwrap(),
        b"Manifest-Version: 1.0\r\n\r\n"
    );
    let caught = try_catch(|| jar.has_class("pkg/Missing")).unwrap_err();
    assert!(caught.is("NullPointerException"));
    jar.close();
    let caught = try_catch(|| jar.get_entries()).unwrap_err();
    assert!(caught.is("IllegalStateException"));

    let caught = try_catch(|| JarContextSource::new(&data("tree/readme.txt"))).unwrap_err();
    assert!(caught.is("IOException"), "{}", caught.message);
    assert!(JarContextSource::open(&data("no-such.jar"), "").is_err());
}

#[test]
fn directory_source_reads_files_and_nests_archives() {
    let dir = DirectoryContextSource::new(&data("tree"));
    assert_eq!(
        dir.get_class_bytes("pkg/util/Util").unwrap(),
        std::fs::read(data("tree/pkg/util/Util.class")).unwrap()
    );
    let caught = try_catch(|| dir.has_class("pkg/Missing")).unwrap_err();
    assert!(caught.is("IOException"));
    let caught = try_catch(|| dir.get_input_stream("pkg")).unwrap_err();
    assert!(caught.is("IOException"));
    let entries = dir.get_entries();
    let children: Vec<&dyn IContextSource> = entries
        .child_contexts()
        .iter()
        .map(|c| c.as_ref())
        .collect();
    assert_eq!(children.len(), 2);
    assert!(children[0].get_name().ends_with("tree/libs/inner.jar"));
    assert_eq!(
        children[1].get_input_stream("a.txt").unwrap(),
        b"after the stub\n"
    );
}

#[test]
fn single_file_source_serves_its_bytes_for_any_name() {
    let path = data("tree/pkg/Hello.class");
    let single = SingleFileContextSource::new(&path);
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(single.get_class_bytes("anything").unwrap(), bytes);
    assert!(single.has_class("whatever"));
    let missing = SingleFileContextSource::new(&data("tree/nope.class"));
    assert!(missing.get_entries().classes().is_empty());
    assert!(try_catch(|| missing.get_input_stream("x"))
        .unwrap_err()
        .is("NullPointerException"));
    assert_eq!(read_class_info(&bytes).unwrap().this_class, "pkg/Hello");
    assert!(read_class_info(&bytes[..bytes.len() - 1]).is_err());
    let caught = try_catch(|| SingleFileContextSource::new(&data("tree/res/.keep")));
    assert!(caught.is_ok());
    let broken = crate::f6_support::tmp_dir("single").join("Broken.class");
    std::fs::write(&broken, &bytes[..40]).unwrap();
    assert!(try_catch(|| SingleFileContextSource::new(&broken))
        .unwrap_err()
        .is("IOException"));
}

#[test]
fn entries_hold_children_as_shared_sources() {
    let child: Rc<dyn IContextSource> = Rc::new(JarContextSource::new(&data("inner.jar")));
    let entries = Entries::new_with_children(
        vec![Entry::at_base("a/B")],
        vec!["a".to_string()],
        vec![Entry::at_base("a/x.txt")],
        vec![child.clone()],
    );
    let copy = entries.clone();
    assert!(Rc::ptr_eq(&copy.child_contexts()[0], &child));
    assert_eq!(Entries::EMPTY().classes().len(), 0);
    assert!(format!("{entries:?}").contains("inner.jar"));
    assert_eq!(CLASS_SUFFIX, ".class");
    assert!(Path::new(&data("tree")).is_dir());
}
