use super::*;

#[test]
fn auto_engages_every_technique_on_detection() {
    let sel = Selection::parse("auto").unwrap();
    assert_eq!(sel, Selection::Auto);
    for d in DEOBFUSCATORS {
        assert_eq!(sel.engage(d.name()), Some(Engage::Detect));
    }
    assert_eq!(sel.value(), "auto");
}

#[test]
fn off_engages_none() {
    let sel = Selection::parse("off").unwrap();
    for d in DEOBFUSCATORS {
        assert_eq!(sel.engage(d.name()), None);
    }
    assert_eq!(sel.value(), "off");
}

#[test]
fn a_named_technique_is_forced_and_the_rest_are_off() {
    let sel = Selection::parse("devirtualize").unwrap();
    assert_eq!(sel.engage("devirtualize"), Some(Engage::Force));
    assert_eq!(sel.engage("another"), None);
    assert_eq!(sel.value(), "devirtualize");
    assert_eq!(Selection::parse(" devirtualize , devirtualize ").unwrap(), sel);
}

#[test]
fn every_registered_technique_is_a_value_and_names_are_distinct() {
    let names: Vec<&str> = DEOBFUSCATORS.iter().map(|d| d.name()).collect();
    let all = Selection::parse(&names.join(",")).unwrap();
    assert_eq!(all, Selection::Only(names.clone()));
    for (i, n) in names.iter().enumerate() {
        assert!(!matches!(*n, "auto" | "off") && !n.contains(','), "{n}");
        assert!(!names[..i].contains(n), "{n} is registered twice");
    }
}

#[test]
fn an_unknown_technique_is_refused_with_the_known_ones() {
    for bad in ["on", "devirtualize,unpack", "", "devirtualize,", "auto,devirtualize", "Devirtualize"] {
        let err = Selection::parse(bad).unwrap_err();
        assert!(err.explain().contains("devirtualize"), "{bad:?}: {}", err.explain());
    }
}
