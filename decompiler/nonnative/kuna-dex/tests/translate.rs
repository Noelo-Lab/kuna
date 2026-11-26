use kuna_dex::{Dex, Options};

use crate::common::{fixture, read_class};

fn translate(name: &str) -> Vec<(String, crate::common::Class)> {
    let data = fixture(name);
    let dex = Dex::parse(&data).unwrap();
    dex.translate_all(&Options::default(), 4)
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
    for f in ["debug.dex", "release.dex", "kotlin.dex"] {
        let classes = translate(f);
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
