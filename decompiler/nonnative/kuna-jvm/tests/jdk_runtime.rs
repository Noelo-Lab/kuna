//! The JDK runtime loader against Vineflower's `jrt:` view and the JDK's own
//! `jimage`/`jmod` tools. Tests that need a local JDK skip when there is none.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use kuna_jvm::context_source::{java_absolute_path, IContextSource};
use kuna_jvm::jdk_runtime::{JImage, JavaRuntimeContextSource, JdkRuntime};
use kuna_jvm::zip_reader::ZipReader;

use crate::f6_support::{data, dump_units, jdk_home, run, skip, tmp_dir, tool, vineflower_jar};

fn home_id(home: &Path) -> String {
    home.to_string_lossy().replace('/', "_")
}

fn root(home: &Path) -> JavaRuntimeContextSource {
    let identifier = java_absolute_path(&home.to_string_lossy());
    JavaRuntimeContextSource::open(home, &identifier).expect("open runtime")
}

#[test]
fn locate_maps_include_runtime_values() {
    assert_eq!(JdkRuntime::locate(""), None);
    assert_eq!(JdkRuntime::locate("0"), None);
    assert_eq!(JdkRuntime::locate("/no/such/jdk"), None);
    let Some(home) = JdkRuntime::locate("current") else {
        return skip("locate_maps_include_runtime_values", "no JDK installed");
    };
    assert!(JdkRuntime::is_java_home(&home));
    assert_eq!(JdkRuntime::locate("1"), Some(home.clone()));
    assert_eq!(JdkRuntime::locate("CURRENT"), Some(home.clone()));
    let explicit = home.to_string_lossy().into_owned();
    assert_eq!(JdkRuntime::locate(&explicit), Some(home.clone()));
    eprintln!("include-runtime=current -> {}", home.display());
}

#[test]
fn runtime_matches_vineflower_jrt_view() {
    let (Some(home), Some(vf)) = (jdk_home(), vineflower_jar()) else {
        return skip(
            "runtime_matches_vineflower_jrt_view",
            "no JDK or vineflower jar",
        );
    };
    let runner = JdkRuntime::locate("current");
    let (Some(javac), Some(java)) = (
        tool(runner.as_deref(), "javac"),
        tool(runner.as_deref(), "java"),
    ) else {
        return skip("runtime_matches_vineflower_jrt_view", "no javac/java");
    };
    let out = tmp_dir("vfdump");
    let compiled = run(
        &javac,
        &[
            "-cp",
            &vf.to_string_lossy(),
            "-d",
            &out.to_string_lossy(),
            &data("VfDump.java").to_string_lossy(),
        ],
    );
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let classpath = format!("{}:{}", vf.display(), out.display());
    let dumped = run(
        &java,
        &[
            "-cp",
            &classpath,
            "org.jetbrains.java.decompiler.struct.VfDump",
            "runtime",
            &home.to_string_lossy(),
        ],
    );
    assert!(
        dumped.status.success(),
        "{}",
        String::from_utf8_lossy(&dumped.stderr)
    );
    let theirs: Vec<String> = String::from_utf8(dumped.stdout)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    let mine = dump_units(vec![Rc::new(root(&home))]);
    let units = mine.iter().filter(|l| l.starts_with("UNIT ")).count();
    let classes = mine.iter().filter(|l| l.starts_with("C ")).count();
    if let Some(i) = mine.iter().zip(&theirs).position(|(a, b)| a != b) {
        let around = |v: &[String]| v[i.saturating_sub(3)..(i + 3).min(v.len())].join("\n");
        panic!(
            "first difference at line {i}:\nmine:\n{}\nvineflower:\n{}",
            around(&mine),
            around(&theirs)
        );
    }
    assert_eq!(mine.len(), theirs.len());
    eprintln!(
        "{}: {units} units, {classes} classes, {} lines identical to Vineflower's jrt view",
        home.display(),
        mine.len()
    );
}

fn jimage_list(jimage: &Path, image: &Path, module: &str) -> BTreeSet<String> {
    let out = run(jimage, &["list", &image.to_string_lossy()]);
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    let mut names = BTreeSet::new();
    let mut inside = false;
    for line in text.lines() {
        if let Some(m) = line.strip_prefix("Module: ") {
            inside = m == module;
        } else if inside && line.starts_with("    ") {
            names.insert(line.trim_start().to_string());
        }
    }
    names
}

/// Every resource of `module`: names against `jimage list`, bytes against `jimage extract`.
fn compare_with_jimage_tool(
    jimage_tool: &Path,
    image_path: &Path,
    module: &str,
) -> (usize, usize, usize) {
    let image = JImage::open(image_path).unwrap();
    let files: Vec<String> = image
        .walk_module(module)
        .unwrap()
        .into_iter()
        .filter(|(_, is_dir)| !is_dir)
        .map(|(name, _)| name)
        .collect();
    let mine: BTreeSet<String> = files.iter().cloned().collect();
    assert_eq!(mine.len(), files.len());
    assert_eq!(mine, jimage_list(jimage_tool, image_path, module));
    let dir = tmp_dir(&format!("jimage-extract{}", home_id(image_path)));
    let pattern = format!("regex:/{module}/.*");
    let out = run(
        jimage_tool,
        &[
            "extract",
            "--dir",
            &dir.to_string_lossy(),
            "--include",
            &pattern,
            &image_path.to_string_lossy(),
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut bytes = 0;
    for name in &files {
        let expected = std::fs::read(dir.join(module).join(name)).unwrap();
        let actual = image.read(&format!("/{module}/{name}")).unwrap().unwrap();
        assert!(actual == expected, "{module}/{name} differs");
        bytes += actual.len();
    }
    let classes = files.iter().filter(|n| n.ends_with(".class")).count();
    (files.len(), classes, bytes)
}

#[test]
fn jimage_matches_jimage_tool() {
    let Some(home) = jdk_home() else {
        return skip("jimage_matches_jimage_tool", "no JDK installed");
    };
    let Some(jimage) = tool(Some(&home), "jimage") else {
        return skip("jimage_matches_jimage_tool", "no jimage tool");
    };
    let (files, classes, bytes) =
        compare_with_jimage_tool(&jimage, &home.join("lib/modules"), "java.base");
    eprintln!("java.base: {files} resources ({classes} classes), {bytes} bytes identical to jimage extract");
}

#[test]
fn listing_and_reads_are_fast() {
    let Some(home) = jdk_home() else {
        return skip("listing_and_reads_are_fast", "no JDK installed");
    };
    let start = Instant::now();
    let rt = root(&home);
    let entries = rt.get_entries();
    let mut classes = 0;
    for module in entries.child_contexts() {
        classes += module.get_entries().classes().len();
    }
    let listing = start.elapsed();
    let start = Instant::now();
    let fresh = root(&home);
    let base = fresh.get_entries().child_contexts()[0].clone();
    let object = base.get_class_bytes("java/lang/Object").unwrap();
    let cold = start.elapsed();
    let start = Instant::now();
    let string = base.get_class_bytes("java/lang/String").unwrap();
    let warm = start.elapsed();
    assert_eq!(&object[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(&string[..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    eprintln!(
        "listed {classes} classes in {} modules: {:.1} ms; cold open+Object.class: {:.1} ms; String.class: {:.3} ms",
        entries.child_contexts().len(),
        listing.as_secs_f64() * 1e3,
        cold.as_secs_f64() * 1e3,
        warm.as_secs_f64() * 1e3
    );
    assert!(listing.as_secs() < 5, "listing took {listing:?}");
}

#[cfg(unix)]
fn fake_home(name: &str, links: &[(&str, &Path)]) -> PathBuf {
    let home = tmp_dir(name);
    for (rel, target) in links {
        let link = home.join(rel);
        std::fs::create_dir_all(link.parent().unwrap()).unwrap();
        if std::fs::symlink_metadata(&link).is_err() {
            std::os::unix::fs::symlink(target, &link).unwrap();
        }
    }
    home
}

#[cfg(unix)]
#[test]
fn jmods_backend_matches_jmod_tool_and_jimage_order() {
    let Some(home) = jdk_home() else {
        return skip("jmods_backend", "no JDK installed");
    };
    let jmod_file = home.join("jmods/java.base.jmod");
    let Some(jmod_tool) = tool(Some(&home), "jmod").filter(|_| jmod_file.is_file()) else {
        return skip("jmods_backend", "no jmods/ or jmod tool");
    };
    let jrt_fs = home.join("lib/jrt-fs.jar");
    let fake = fake_home(
        &format!("jmods-home{}", home_id(&home)),
        &[
            ("jmods/java.base.jmod", &jmod_file),
            ("lib/jrt-fs.jar", &jrt_fs),
        ],
    );
    let sources = JdkRuntime::sources(&fake);
    assert_eq!(sources.len(), 1);
    let from_jmod = sources[0].clone();
    let from_image = JdkRuntime::sources(&home)[0].clone();
    assert_eq!(from_jmod.get_name(), from_image.get_name());

    let extracted = tmp_dir(&format!("jmod-extract{}", home_id(&home)));
    let out = run(
        &jmod_tool,
        &[
            "extract",
            "--dir",
            &extracted.to_string_lossy(),
            &jmod_file.to_string_lossy(),
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let jmod_entries = from_jmod.get_entries();
    let mut bytes = 0;
    for class in jmod_entries.classes() {
        let expected = std::fs::read(
            extracted
                .join("classes")
                .join(format!("{}.class", class.base_path())),
        )
        .unwrap();
        let actual = from_jmod.get_class_bytes(class.base_path()).unwrap();
        assert!(actual == expected, "{} differs", class.base_path());
        bytes += actual.len();
    }

    let names = |e: &kuna_jvm::context_source::Entries| -> Vec<String> {
        e.classes()
            .iter()
            .map(|c| format!("{}.class", c.base_path()))
            .chain(e.others().iter().map(|o| o.base_path().to_string()))
            .collect()
    };
    let image_entries = from_image.get_entries();
    let jmod_set: BTreeSet<String> = names(&jmod_entries).into_iter().collect();
    let image_set: BTreeSet<String> = names(&image_entries).into_iter().collect();
    let common = |e: &kuna_jvm::context_source::Entries, other: &BTreeSet<String>| -> Vec<String> {
        names(e).into_iter().filter(|n| other.contains(n)).collect()
    };
    assert_eq!(
        common(&jmod_entries, &image_set),
        common(&image_entries, &jmod_set)
    );
    eprintln!(
        "java.base.jmod: {} classes, {bytes} bytes identical to jmod extract; {} shared entries in jimage order",
        jmod_entries.classes().len(),
        jmod_set.intersection(&image_set).count()
    );
}

#[test]
fn tiny_jmod_is_a_module_source() {
    let home = tmp_dir("tiny-home");
    std::fs::create_dir_all(home.join("jmods")).unwrap();
    std::fs::copy(data("tiny.jmod"), home.join("jmods/f6.tiny.jmod")).unwrap();
    let rt = root(&home);
    assert!(!rt.is_jimage());
    assert!(rt.get_input_stream("anything").is_none());
    let entries = rt.get_entries();
    assert_eq!(entries.child_contexts().len(), 1);
    let module = entries.child_contexts()[0].clone();
    assert_eq!(module.get_name(), "module f6.tiny@1.0");
    assert!(!module.is_lazy());
    let e = module.get_entries();
    let classes: Vec<&str> = e.classes().iter().map(|c| c.base_path()).collect();
    let others: Vec<&str> = e.others().iter().map(|c| c.base_path()).collect();
    assert_eq!(classes, ["module-info", "f6/tiny/Tiny"]);
    assert_eq!(others, ["", "f6", "f6/tiny"]);
    assert!(e.directories().is_empty());
    let zip = ZipReader::open(&data("tiny.jmod")).unwrap();
    assert_eq!(
        module.get_class_bytes("f6/tiny/Tiny"),
        zip.read("classes/f6/tiny/Tiny.class").unwrap()
    );
    let caught = kuna_jvm::rt::try_catch(|| module.get_class_bytes("f6/tiny/Missing")).unwrap_err();
    assert!(caught.is("IOException"));
    rt.close();
    let caught = kuna_jvm::rt::try_catch(|| module.get_class_bytes("f6/tiny/Tiny")).unwrap_err();
    assert!(caught.is("ClosedFileSystemException"));
}

#[test]
fn java8_home_contributes_its_jars() {
    let home = tmp_dir("java8-home");
    for (rel, src) in [
        ("jre/lib/rt.jar", "mixed.jar"),
        ("lib/tools.jar", "inner.jar"),
        ("lib/notes.txt", "tree/readme.txt"),
    ] {
        let to = home.join(rel);
        std::fs::create_dir_all(to.parent().unwrap()).unwrap();
        std::fs::copy(data(src), &to).unwrap();
    }
    std::fs::create_dir_all(home.join("jre/lib/ext")).unwrap();
    assert!(JdkRuntime::is_java_home(&home));
    assert_eq!(
        JdkRuntime::locate(&home.to_string_lossy()),
        Some(home.clone())
    );
    let names: Vec<String> = JdkRuntime::sources(&home)
        .iter()
        .map(|s| s.get_name())
        .collect();
    assert_eq!(
        names,
        [
            format!("archive {}", home.join("jre/lib/rt.jar").display()),
            format!("archive {}", home.join("lib/tools.jar").display()),
        ]
    );
}

#[test]
fn jimage_rejects_garbage() {
    let dir = tmp_dir("jimage-garbage");
    let path = dir.join("modules");
    for bytes in [
        Vec::new(),
        vec![0xDA, 0xDA, 0xFE, 0xCA],
        [0xDA, 0xDA, 0xFE, 0xCA, 0, 0, 2, 0].repeat(4),
        [
            0xDA, 0xDA, 0xFE, 0xCA, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0x0F,
        ]
        .into_iter()
        .chain([0u8; 8])
        .collect(),
    ] {
        std::fs::write(&path, &bytes).unwrap();
        assert!(JImage::open(&path).is_err());
    }
    let Some(home) = jdk_home() else {
        return;
    };
    let mut head = Vec::new();
    let file = std::fs::File::open(home.join("lib/modules")).unwrap();
    std::io::Read::read_to_end(&mut std::io::Read::take(file, 2 << 20), &mut head).unwrap();
    std::fs::write(&path, &head).unwrap();
    if let Ok(image) = JImage::open(&path) {
        let _ = image.module_names();
        let _ = image.read("/java.base/java/lang/Object.class");
        let _ = image.walk_module("java.base");
    }
}

/// Builds (once) jlink images of java.base with zip and string-sharing
/// compression, without compression, and big-endian. Every java.base resource
/// of the first three is checked against `jimage extract` of the same image;
/// the big-endian one (which the host's `jimage` cannot open) against the
/// uncompressed little-endian one. Slow: about 30 s and 150 MB.
#[test]
#[ignore]
fn compressed_and_big_endian_images_match_jimage_extract() {
    let Some(home) = jdk_home() else {
        return skip("compressed images", "no JDK installed");
    };
    let (Some(jlink), Some(jimage)) = (tool(Some(&home), "jlink"), tool(Some(&home), "jimage"))
    else {
        return skip("compressed images", "no jlink/jimage");
    };
    if !home.join("jmods").is_dir() {
        return skip("compressed images", "no jmods/");
    }
    let parent = tmp_dir(&format!("jlink{}", home_id(&home)));
    let build = |name: &str, flags: &[&str]| -> PathBuf {
        let image_home = parent.join(name);
        if !image_home.join("lib/modules").is_file() {
            let target = image_home.to_string_lossy().into_owned();
            let mut args = vec!["--add-modules", "java.base", "--output", &target];
            args.extend(flags.iter().copied());
            let out = run(&jlink, &args);
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        image_home
    };
    for (name, flags) in [
        ("zip", &["--compress=zip-6"][..]),
        ("cp", &["--compress=1"][..]),
        ("plain", &[][..]),
    ] {
        let image_home = build(name, flags);
        let modules = image_home.join("lib/modules");
        let (files, classes, bytes) = compare_with_jimage_tool(&jimage, &modules, "java.base");
        let base = root(&image_home).get_entries().child_contexts()[0].clone();
        assert!(base.get_name().starts_with("module java.base@"));
        eprintln!(
            "jlink {name}: {files} resources ({classes} classes), {bytes} bytes identical to jimage extract"
        );
    }
    let le = JImage::open(&build("plain", &[]).join("lib/modules")).unwrap();
    let be = JImage::open(&build("be", &["--endian", "big"]).join("lib/modules")).unwrap();
    assert!(be.is_big_endian() && !le.is_big_endian());
    assert_eq!(be.module_names().unwrap(), le.module_names().unwrap());
    let walk = le.walk_module("java.base").unwrap();
    assert_eq!(be.walk_module("java.base").unwrap(), walk);
    let mut bytes = 0;
    for (name, is_dir) in &walk {
        if !is_dir {
            let path = format!("/java.base/{name}");
            let mine = be.read(&path).unwrap().unwrap();
            assert!(mine == le.read(&path).unwrap().unwrap(), "{path} differs");
            bytes += mine.len();
        }
    }
    eprintln!(
        "jlink be: {} entries, {bytes} bytes identical to the little-endian image",
        walk.len()
    );
}
