use kuna_dex::{Dex, Options};

use crate::common::{fixture, read_class};

fn translate(name: &str) -> Vec<(String, crate::common::Class)> {
    translate_with(name, &Options::default())
}

fn translate_with(name: &str, opts: &Options) -> Vec<(String, crate::common::Class)> {
    let data = fixture(name);
    let dex = Dex::parse(&data).unwrap();
    dex.translate_all(opts, 4)
        .into_iter()
        .map(|r| {
            let t = r.unwrap();
            assert!(t.failures.is_empty(), "{}: {:?}", t.name, t.failures);
            let c = read_class(&t.bytes);
            (t.name, c)
        })
        .collect()
}

fn method<'a>(classes: &'a [(String, crate::common::Class)], class: &str, name: &str) -> &'a crate::common::Method {
    let c = &classes.iter().find(|c| c.0 == class).unwrap().1;
    c.methods.iter().find(|m| m.name == name).unwrap()
}

#[test]
fn every_fixture_class_translates_at_version_50() {
    let verifiable = Options {
        verifiable: true,
        ..Options::default()
    };
    for f in ["debug.dex", "release.dex", "kotlin.dex"] {
        let classes = translate_with(f, &verifiable);
        assert!(classes.len() > 5);
        for (name, c) in &classes {
            assert_eq!(c.version, 50, "{f} {name}");
            assert!(!c.utf8.iter().any(|s| s == kuna_dex::ERROR_ATTRIBUTE));
        }
    }
}

#[test]
fn debug_build_carries_lines_and_locals() {
    let classes = translate("debug.dex");
    let m = method(&classes, "dx/Wide", "mix");
    assert!(m.attrs.iter().any(|a| a == "LineNumberTable"));
    assert!(m.attrs.iter().any(|a| a == "LocalVariableTable"));
    let c = &classes.iter().find(|c| c.0 == "dx/Wide").unwrap().1;
    for v in ["a", "b", "c", "d", "t", "u", "v", "w"] {
        assert!(c.utf8.iter().any(|s| s == v), "local {v}");
    }
}

#[test]
fn release_expressions_stay_on_the_stack() {
    let classes = translate("release.dex");
    let lits = method(&classes, "dx/Arith", "lits");
    assert_eq!(lits.max_locals, 1, "one chain of single-use values needs only the parameter");
    let ternary = method(&classes, "dx/Arith", "ternary");
    assert_eq!(ternary.max_locals, 1);
    let istores = lits.code.iter().filter(|&&b| (0x36..=0x3a).contains(&b) || (0x3b..=0x4e).contains(&b)).count();
    assert!(istores <= 1, "{istores} stores");
}

#[test]
fn nesting_and_signatures_become_attributes() {
    let classes = translate("debug.dex");
    let objects = &classes.iter().find(|c| c.0 == "dx/Objects").unwrap().1;
    assert!(objects.attrs.iter().any(|a| a == "InnerClasses"));
    for inner in ["Inner", "Square", "Color", "Shape"] {
        assert!(objects.utf8.iter().any(|s| s == inner), "{inner}");
    }
    assert!(objects.utf8.iter().any(|s| s == "dx/Objects$1"), "anonymous class listed by its encloser");
    let local = &classes.iter().find(|c| c.0 == "dx/Objects$1Local").unwrap().1;
    assert!(local.attrs.iter().any(|a| a == "EnclosingMethod"));
    assert!(objects.utf8.iter().any(|s| s == "Ljava/util/List<Ljava/lang/String;>;"));
    let flow = &classes.iter().find(|c| c.0 == "dx/Objects").unwrap().1;
    assert!(flow.utf8.iter().any(|s| s == "ConstantValue"));
    for (name, c) in &classes {
        for (inner, outer, simple) in &c.inner {
            let tail = inner.rsplit('$').next().unwrap_or("");
            let local = tail.starts_with(|ch: char| ch.is_ascii_digit());
            assert!(!outer.is_empty() || local, "{name}: {inner} listed with no outer");
            assert!(outer.is_empty() || inner == &format!("{outer}${simple}"), "{name}: {inner} {outer} {simple}");
        }
    }
    let member = &classes.iter().find(|c| c.0 == "dx/Objects$Inner").unwrap().1;
    assert!(member.inner.contains(&("dx/Objects$Inner".into(), "dx/Objects".into(), "Inner".into())));
}

#[test]
fn kotlin_classes_keep_metadata() {
    let classes = translate("kotlin.dex");
    let point = &classes.iter().find(|c| c.0 == "kx/Point").unwrap().1;
    assert!(point.attrs.iter().any(|a| a == "RuntimeVisibleAnnotations"));
    assert!(point.utf8.iter().any(|s| s == "Lkotlin/Metadata;"));
}

#[test]
fn an_untranslatable_method_fails_alone() {
    let mut data = fixture("release.dex");
    let dex = Dex::parse(&data).unwrap();
    let i = (0..dex.class_count()).find(|&i| dex.class_name(i).unwrap() == "dx/Arith").unwrap();
    let cd = dex.class_data(dex.class_def(i).unwrap().class_data_off).unwrap();
    let em = cd
        .direct_methods
        .iter()
        .find(|m| dex.string(dex.method(m.method).unwrap().name).unwrap() == b"ii")
        .unwrap();
    let at = em.code_off as usize + 16;
    drop(dex);
    data[at..at + 4].copy_from_slice(&[0x0f, 0x00, 0x00, 0x00]);
    let dex = Dex::parse(&data).unwrap();
    let t = dex.translate(i, &Options::default()).unwrap();
    assert_eq!(t.failures.len(), 1, "{:?}", t.failures);
    assert!(t.failures[0].starts_with("ii(II)I: register v0 is read"), "{:?}", t.failures);
    let c = read_class(&t.bytes);
    let m = c.methods.iter().find(|m| m.name == "ii").unwrap();
    assert!(m.attrs.iter().any(|a| a == kuna_dex::ERROR_ATTRIBUTE));
    assert!(c.methods.iter().filter(|m| m.name != "ii").all(|m| !m.attrs.iter().any(|a| a == kuna_dex::ERROR_ATTRIBUTE)));
}

#[test]
fn d8_lambda_classes_become_invokedynamic_again() {
    let classes = translate("release.dex");
    let objects = &classes.iter().find(|c| c.0 == "dx/Objects").unwrap().1;
    assert!(objects.version >= 52, "{}", objects.version);
    assert!(objects.attrs.iter().any(|a| a == "BootstrapMethods"));
    assert!(objects.utf8.iter().any(|s| s == "java/lang/invoke/LambdaMetafactory"));
    assert!(objects.utf8.iter().any(|s| s == "applyAsInt"));
}


#[test]
fn d8_backports_become_the_jdk_calls() {
    let data = fixture("backports/probe.dex");
    let dex = Dex::parse(&data).unwrap();
    let table = include_str!("../src/backports.tsv");
    let index = String::from_utf8(fixture("backports/index.tsv")).unwrap();
    let mut omitted = 0;
    for t in dex.translate_all(&Options::default(), 4) {
        let t = t.unwrap();
        if t.omit {
            omitted += 1;
            continue;
        }
        assert_eq!(t.name, "bp/Probe");
        let c = read_class(&t.bytes);
        assert!(!c.utf8.iter().any(|s| s.contains("ExternalSynthetic")));
        for line in index.lines() {
            let method = line.split('\t').nth(1).unwrap();
            if !table.lines().any(|l| l.ends_with(method)) {
                continue;
            }
            let (owner, rest) = method.split_once('#').unwrap();
            let name = &rest[..rest.find('(').unwrap()];
            assert!(c.utf8.iter().any(|s| s == owner) && c.utf8.iter().any(|s| s == name), "{method}");
        }
    }
    assert!(omitted > 0, "the backport classes are left out");
}

/// The JVM local slots `code` reads that no instruction of it writes.
fn unwritten_reads(code: &[u8], params: u16) -> Vec<u16> {
    let (mut reads, mut writes) = (Vec::new(), Vec::new());
    let mut i = 0;
    while i < code.len() {
        let op = code[i];
        let wide = op == 0xc4;
        let at = if wide { i + 1 } else { i };
        let op = code[at];
        let idx = |n: usize| if wide { u16::from_be_bytes([code[at + 1], code[at + 2]]) } else { u16::from(code[at + n]) };
        let len = match op {
            0x15..=0x19 => {
                reads.push(idx(1));
                2
            }
            0x1a..=0x2d => {
                reads.push(u16::from((op - 0x1a) % 4));
                1
            }
            0x36..=0x3a => {
                writes.push(idx(1));
                2
            }
            0x3b..=0x4e => {
                writes.push(u16::from((op - 0x3b) % 4));
                1
            }
            0x84 => {
                reads.push(idx(1));
                3
            }
            0x10 | 0x12 | 0xbc => 2,
            0x11 | 0x13 | 0x14 | 0x99..=0xa8 | 0xb2..=0xb8 | 0xbb | 0xbd | 0xc0 | 0xc1 | 0xc6 | 0xc7 => 3,
            0xc5 => 4,
            0xb9 | 0xba | 0xc8 => 5,
            0xaa | 0xab => {
                let p = (at + 4) & !3;
                let word = |k: usize| i32::from_be_bytes([code[p + k], code[p + k + 1], code[p + k + 2], code[p + k + 3]]) as usize;
                p - at + if op == 0xaa { 12 + 4 * (word(8) - word(4) + 1) } else { 8 + 8 * word(4) }
            }
            _ => 1,
        };
        i = at + if wide { len * 2 - 1 } else { len };
    }
    reads.into_iter().filter(|&s| s >= params && !writes.contains(&s)).collect()
}

#[test]
fn parameters_updated_in_place_keep_their_local() {
    for f in ["sugar.dex", "sugar-release.dex"] {
        let data = fixture(f);
        let dex = Dex::parse(&data).unwrap();
        let idx = (0..dex.class_count()).find(|&i| dex.class_name(i).unwrap() == "sx/Sugar$Params").unwrap();
        let def = dex.class_def(idx).unwrap();
        let cd = dex.class_data(def.class_data_off).unwrap();
        let t = dex.translate(idx, &Options::default()).unwrap();
        let c = read_class(&t.bytes);
        for em in cd.direct_methods.iter().chain(&cd.virtual_methods) {
            let mi = dex.method(em.method).unwrap();
            let name = kuna_dex::mutf8_to_string(dex.string(mi.name).unwrap());
            let p = dex.proto(mi.proto).unwrap();
            let mut slots = u16::from(em.access & 0x8 == 0);
            for &ty in &p.params {
                slots += if matches!(dex.type_desc(ty).unwrap()[0], b'J' | b'D') { 2 } else { 1 };
            }
            let m = c.methods.iter().find(|m| m.name == name).unwrap();
            assert!(unwritten_reads(&m.code, slots).is_empty(), "{f} {name}: reads {:?}", unwritten_reads(&m.code, slots));
        }
    }
}
