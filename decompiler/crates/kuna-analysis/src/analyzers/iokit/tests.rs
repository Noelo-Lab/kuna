use super::*;

/// Accuracy against the symbols of a real kernelcache entry, recovering as if
/// the image were stripped (manual run: `KUNA_KC=<kernelcache>
/// KUNA_KC_ENTRY=<id> cargo test ... -- --ignored --nocapture`).
#[test]
#[ignore]
fn real_kernelcache_entry() {
    let path = std::env::var("KUNA_KC").unwrap();
    let entry = std::env::var("KUNA_KC_ENTRY").unwrap_or_else(|_| "auto".into());
    let choice = kuna_decomp::kuna_kernelcache::KernelcacheChoice::parse(&entry).unwrap();
    let bytes = crate::loader::kuna_kernelcache::peel(std::fs::read(path).unwrap(), &choice).unwrap().bytes;
    let file = object::File::parse(&*bytes).unwrap();
    let img = MachoImage::with_fixups(&file, resolve_chained_fixups(&file, &bytes));
    let truth = symbol_names(&file);
    let t = std::time::Instant::now();
    let rec = recover_with(&file, &img, HashMap::new());
    let elapsed = t.elapsed();
    let sym = |a: u64| truth.get(&a).cloned();
    let metas_ok = rec.classes.iter().filter(|c| sym(c.meta).as_deref() == Some(&format!("{}::gMetaClass", c.name))).count();
    let metas_named = rec.classes.iter().filter(|c| sym(c.meta).is_some()).count();
    let vt = rec.classes.iter().filter(|c| c.vtable.is_some()).count();
    let vt_sym = rec.classes.iter().filter_map(|c| c.vtable.as_ref().map(|(s, _)| s - 16)).filter(|a| sym(*a).is_some()).count();
    let names = rec.function_names();
    let (mut exact, mut placeholder, mut wrong, mut unknown) = (0, 0, 0, 0);
    let mut bad = Vec::new();
    for (addr, name) in &names {
        match sym(*addr) {
            Some(s) if s == *name => exact += 1,
            Some(s) if name.contains("::vfn_") && s.starts_with(name.split("::vfn_").next().unwrap()) => placeholder += 1,
            Some(s) => {
                wrong += 1;
                if bad.len() < 12 {
                    bad.push(format!("{addr:#x} {name} vs {s}"));
                }
            }
            None => unknown += 1,
        }
    }
    eprintln!(
        "{entry}: classes {} (gMetaClass agrees {metas_ok}/{metas_named}); vtables {vt} ({vt_sym} at a symbol); \
         metaclass vtables {}; function names {} = exact {exact} + class-right placeholder {placeholder} + wrong {wrong} + no symbol {unknown}; {elapsed:?}",
        rec.classes.len(),
        rec.meta_vtables.len(),
        names.len()
    );
    for b in bad {
        eprintln!("  {b}");
    }
    if let Ok(probe) = std::env::var("KUNA_KC_PROBE") {
        let probe = u64::from_str_radix(probe.trim_start_matches("0x"), 16).unwrap();
        let live = recover(&file, &img);
        eprintln!("probe {probe:#x}: stripped {:?} live {:?} sym {:?}", names.get(&probe), live.function_names().get(&probe), sym(probe));
    }
}

fn fixture() -> Vec<u8> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/kernelcache/iokit_kext.macho");
    std::fs::read(path).unwrap()
}

const B: u64 = 0xffff_fff0_07a0_0000;

#[test]
fn recovers_classes_vtables_and_method_names() {
    let bytes = fixture();
    let file = object::File::parse(&*bytes).unwrap();
    let img = MachoImage::with_fixups(&file, resolve_chained_fixups(&file, &bytes));
    let rec = recover(&file, &img);
    assert_eq!(rec.ctor, Some(B + 0x4000));
    let summary: Vec<(&str, u64, Option<u64>, u64, bool)> = rec
        .classes
        .iter()
        .map(|c| (c.name.as_str(), c.meta, c.super_meta, c.size, c.vtable.is_some()))
        .collect();
    assert_eq!(
        summary,
        [("Base", B + 0x6000, None, 0x20, true), ("Derived", B + 0x6040, Some(B + 0x6000), 0x30, true)]
    );
    assert_eq!(rec.meta_vtables.len(), 2);
    let names = rec.function_names();
    let name_at = |off: u64| names.get(&(B + off)).map(String::as_str);
    assert_eq!(name_at(0x4000), Some("OSMetaClass::OSMetaClass"));
    assert_eq!(name_at(0x4090), Some("Base::getMetaClass"));
    let by_name: std::collections::BTreeSet<&str> = names.values().map(String::as_str).collect();
    for want in ["Derived::m2", "Derived::vfn_0x28", "Derived::~Derived", "Base::~Base", "Derived::MetaClass::alloc", "Base::MetaClass::~MetaClass"] {
        assert!(by_name.contains(want), "{want} missing from {by_name:?}");
    }
    assert!(!by_name.iter().any(|n| n.starts_with("Base::m")), "symbol-named Base methods are not renamed");
}

#[test]
fn inert_on_a_non_xnu_image() {
    let mut bytes = fixture();
    bytes[12] = 2;
    let file = object::File::parse(&*bytes).unwrap();
    assert!(!is_xnu_image(&file, &bytes));
}

#[test]
fn reads_driverkit_class_records() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/kernelcache/dext_osclassinfo.macho");
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let img = MachoImage::with_fixups(&file, resolve_chained_fixups(&file, &bytes));
    let b = 0x1_0000_0000u64;
    assert_eq!(
        osclassinfo::classes(&file, &img),
        [osclassinfo::DextClass {
            name: "MyDriver".into(),
            super_name: "IOService".into(),
            load_info: b + 0x4000,
            description: b + 0x2000,
            description_size: 0x138,
            meta_pointer: b + 0x8000,
            instance_size: 0x40,
            new: b + 0x1000,
        }]
    );
}
