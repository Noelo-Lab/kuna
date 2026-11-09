//! The `StructContext` class universe against Vineflower 1.12.0
//! (`data/GenF4Oracle.java` over `data/f4_classes/`): unit scanning and the
//! own-over-library rule, lazy and badly placed classes, multi-release entries,
//! `getClass`/`hasClass`, the `instanceOf` and `findCommonAncestor` matrices,
//! abstract parameter names, `reloadContext` and `clear` — including the exact
//! log stream, which pins the order classes are loaded in.

use std::cell::Cell;
use std::rc::Rc;

use kuna_jvm::classes_processor::ClassesProcessor;
use kuna_jvm::context_source::Entry;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, Severity};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::rt::{Caught, Shared};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;

use crate::f4_util::{golden, MemorySource, NoDecompiledData};

const ABSTRACT_NAMES: &str =
    "p/A foo (IJLjava/lang/String;)V x y z\np/A bar ()V\n\np/A baz (D[J)V d arr\nshort\n";
const UNIVERSE: [&str; 16] = [
    "p/I",
    "p/J",
    "p/A",
    "p/B",
    "p/C",
    "p/D",
    "p/E",
    "p/F",
    "p/G",
    "q/L",
    "q/K",
    "q/M",
    "java/lang/Object",
    "java/lang/Runnable",
    "x/Unknown",
    "wrong/Name",
];

struct RecordingLogger {
    severity: Cell<Severity>,
    lines: Shared<Vec<String>>,
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

impl IFernflowerLogger for RecordingLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, message: &str, severity: Severity) {
        if self.accepts(severity) {
            self.lines
                .borrow_mut()
                .push(format!("L\t{}\t{}", severity.name(), esc(message)));
        }
    }

    fn write_message_ex(&self, message: &str, severity: Severity, _t: &Caught) {
        self.write_message(message, severity);
    }
}

fn name_of(cl: Option<StructClass>) -> String {
    match cl {
        None => "\\N".to_string(),
        Some(cl) => format!(
            "{}/{}",
            cl.with(|d| d.qualified_name.clone()),
            if cl.is_own() { "own" } else { "lib" }
        ),
    }
}

fn own_classes(sc: StructContext) -> String {
    let names: Vec<String> = sc
        .get_own_classes()
        .into_iter()
        .map(|c| name_of(Some(c)))
        .collect();
    format!("O\t{}", names.join(" "))
}

fn queries(sc: StructContext, lines: &Shared<Vec<String>>) {
    let push = |line: String| lines.borrow_mut().push(line);
    push(own_classes(sc));
    for unit in sc.get_units() {
        let others: Vec<String> = unit.get_other_entries().iter().map(Entry::path).collect();
        push(format!(
            "U\t{}\t{}\t{}\t{}\t{}\t{}",
            unit.get_name(),
            unit.is_own() as i32,
            unit.is_root() as i32,
            unit.is_lazy() as i32,
            unit.get_class_names().join(" "),
            others.join(" ")
        ));
    }
    for n in UNIVERSE {
        let cl = sc.get_class(n);
        push(format!("G\t{}\t{}", n, name_of(cl)));
    }
    for n in UNIVERSE {
        let has = sc.has_class(n);
        push(format!("H\t{}\t{}", n, has as i32));
    }
    for a in UNIVERSE {
        let mut inst = String::new();
        let mut anc = Vec::new();
        for c in UNIVERSE {
            inst.push(if sc.instance_of(a, c) { '1' } else { '0' });
            let common = sc.find_common_ancestor(a, c);
            anc.push(common.map_or("-".to_string(), |cl| cl.with(|d| d.qualified_name.clone())));
        }
        push(format!("I\t{a}\t{inst}"));
        push(format!("A\t{}\t{}", a, anc.join(" ")));
    }
    for (class, method, desc) in [
        ("p/A", "foo", "(IJLjava/lang/String;)V"),
        ("p/A", "baz", "(D[J)V"),
        ("p/A", "bar", "()V"),
        ("p/B", "foo", "(IJLjava/lang/String;)V"),
    ] {
        let names: Vec<String> = (0..5)
            .map(|i| sc.rename_abstract_parameter(class, method, desc, i, &format!("def{i}")))
            .collect();
        push(format!(
            "P\t{}\t{}\t{}\t{}",
            class,
            method,
            desc,
            names.join(" ")
        ));
    }
}

#[test]
fn class_universe_matches_vineflower() {
    let lines: Shared<Vec<String>> = Shared::new(Vec::new());
    let logger = Rc::new(RecordingLogger {
        severity: Cell::new(Severity::Info),
        lines: lines.clone(),
    });
    logger.set_severity(Severity::Trace);
    let sc = StructContext::new(Rc::new(NoDecompiledData));
    let ctx = DecompilerContext::new(
        IFernflowerPreferences::DEFAULTS(),
        logger,
        sc,
        ClassesProcessor::new(sc),
        None,
    );
    DecompilerContext::set_current_context(Some(ctx));

    let base = Entry::BASE_VERSION;
    let lib1 = MemorySource::new("lib1", false)
        .cls("p/D", base, "p/D")
        .cls("q/L", base, "q/L")
        .cls("q/K", base, "q/K");
    let mut own = MemorySource::new("own", false);
    for n in ["p/I", "p/J", "p/A", "p/B", "p/C", "p/D", "p/E", "p/F"] {
        own = own.cls(n, base, n);
    }
    let own = own
        .cls("wrong/Name", base, "p/G")
        .cls("p/MR", 11, "p/D")
        .other("fernflower_abstract_parameter_names.txt", ABSTRACT_NAMES)
        .other("META-INF/MANIFEST.MF", "Manifest-Version: 1.0\n");
    let lib2 = MemorySource::new("lib2", false).cls("p/B", base, "p/B");
    let lazy = MemorySource::new("lazy", true).cls("q/M", base, "q/M");

    sc.add_space(Rc::new(lib1), false);
    sc.add_space(Rc::new(own), true);
    sc.add_space(Rc::new(lib2), false);
    sc.add_space(Rc::new(lazy), false);
    queries(sc, &lines);
    lines.borrow_mut().push("== reload".to_string());
    sc.reload_context();
    queries(sc, &lines);
    lines.borrow_mut().push("== clear".to_string());
    sc.clear();
    let after_clear = own_classes(sc);
    lines.borrow_mut().push(after_clear);
    let units = sc.get_units().len();
    lines.borrow_mut().push(format!("U\t{units}"));

    let expected = golden("f4_struct_context.txt");
    let actual = lines.borrow().clone();
    for (i, (a, e)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(a, e, "line {}", i + 1);
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn jar_detection_scans_for_the_end_of_central_directory() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("f4-jar-detection");
    std::fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        let path = dir.join(name);
        std::fs::write(&path, bytes).unwrap();
        path
    };
    let mut zip_like = vec![0u8; 100];
    zip_like.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06]);
    zip_like.extend_from_slice(&[0u8; 18]);
    assert!(StructContext::is_jar_file(&write("archive.bin", &zip_like)));
    assert!(StructContext::is_jar_file(&write("x.jar", b"")));
    assert!(StructContext::is_jar_file(&write("x.zip", b"")));
    assert!(!StructContext::is_jar_file(&write("x.class", &zip_like)));
    assert!(!StructContext::is_jar_file(&write(
        "short.bin",
        &[0x50, 0x4b, 0x05, 0x06]
    )));
    assert!(!StructContext::is_jar_file(&write(
        "plain.bin",
        &[7u8; 2048]
    )));
    assert!(!StructContext::is_jar_file(&dir));
    assert!(!StructContext::is_jar_file(&dir.join("missing.bin")));
}
