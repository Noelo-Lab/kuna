use kuna_dex::{decode, Dex, NO_INDEX};

use crate::common::fixture;

#[test]
fn header_and_classes() {
    let data = fixture("debug.dex");
    let dex = Dex::parse(&data).unwrap();
    assert_eq!(dex.version, 35);
    let names: Vec<String> = (0..dex.class_count()).map(|i| dex.class_name(i).unwrap()).collect();
    for want in ["dx/Arith", "dx/Flow", "dx/Objects$Color", "dx/Objects$1Local"] {
        assert!(names.iter().any(|n| n == want), "{want} in {names:?}");
    }
}

#[test]
fn every_method_decodes_with_debug_info() {
    let data = fixture("debug.dex");
    let dex = Dex::parse(&data).unwrap();
    let (mut methods, mut tries, mut switches, mut locals) = (0, 0, 0, 0);
    for i in 0..dex.class_count() {
        let def = dex.class_def(i).unwrap();
        let cd = dex.class_data(def.class_data_off).unwrap();
        for em in cd.direct_methods.iter().chain(&cd.virtual_methods) {
            if em.code_off == 0 {
                continue;
            }
            let code = dex.code_item(em.code_off).unwrap();
            let insns = decode(code.insns).unwrap();
            assert!(!insns.is_empty());
            switches += insns.iter().filter(|x| matches!(x.op, 0x2b | 0x2c)).count();
            tries += code.tries.len();
            let first = u32::from(code.registers - code.ins);
            let info = dex.debug_info(&code, &[(first, NO_INDEX)], NO_INDEX - 1).unwrap();
            locals += info.locals.len();
            assert!(info.lines.windows(2).all(|w| w[0].0 < w[1].0));
            methods += 1;
        }
    }
    assert!(methods > 40 && tries > 5 && switches >= 3 && locals > 50, "{methods} {tries} {switches} {locals}");
}

#[test]
fn switch_and_array_payloads() {
    let data = fixture("debug.dex");
    let dex = Dex::parse(&data).unwrap();
    let mut packed = None;
    let mut sparse = None;
    let mut fill = 0;
    for i in 0..dex.class_count() {
        let cd = dex.class_data(dex.class_def(i).unwrap().class_data_off).unwrap();
        for em in cd.direct_methods.iter().chain(&cd.virtual_methods).filter(|m| m.code_off != 0) {
            let code = dex.code_item(em.code_off).unwrap();
            for x in decode(code.insns).unwrap() {
                match x.op {
                    0x2b => packed = Some(kuna_dex::packed_switch(code.insns, x.target()).unwrap()),
                    0x2c => sparse = Some(kuna_dex::sparse_switch(code.insns, x.target()).unwrap()),
                    0x26 => {
                        let (w, e) = kuna_dex::fill_array(code.insns, x.target()).unwrap();
                        assert!(matches!(w, 1 | 2 | 4 | 8) && !e.is_empty());
                        fill += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    let (first, targets) = packed.expect("a packed-switch");
    assert!(first <= 1 && !targets.is_empty());
    let keys: Vec<i32> = sparse.expect("a sparse-switch").iter().map(|p| p.0).collect();
    assert!(keys.windows(2).all(|w| w[0] < w[1]));
    assert!(fill > 3);
}

#[test]
fn kotlin_metadata_annotation_is_runtime_visible() {
    let data = fixture("kotlin.dex");
    let dex = Dex::parse(&data).unwrap();
    let i = (0..dex.class_count()).find(|&i| dex.class_name(i).unwrap() == "kx/Point").unwrap();
    let def = dex.class_def(i).unwrap();
    let dir = dex.annotations_directory(def.annotations_off).unwrap();
    let anns = dex.annotation_set(dir.class).unwrap();
    let meta = anns
        .iter()
        .find(|a| dex.type_desc(a.value.ty).unwrap() == b"Lkotlin/Metadata;")
        .expect("kotlin.Metadata");
    assert_eq!(meta.visibility, 1);
    assert!(meta.value.elements.len() >= 4);
}
