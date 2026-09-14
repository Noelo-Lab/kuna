//! Logic-level tests for the option parse, the gate and the two layouts.

use super::*;

use crate::dtype::TypeFactoryImpl;

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("void", 1, type_metatype::TYPE_VOID, false).unwrap();
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined4", 4, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    f.set_core_type("uint2", 2, type_metatype::TYPE_UINT, false).unwrap();
    f.set_core_type("uint4", 4, type_metatype::TYPE_UINT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn field<'a>(ct: &'a Datatype, name: &str) -> &'a TypeField {
    (0..ct.num_depend()).filter_map(|i| ct.get_field(i)).find(|f| f.name == name).expect(name)
}

#[test]
fn option_parse_accepts_the_three_values() {
    assert_eq!(OptionPebNames.apply("off").unwrap().0, PebNamesMode::Off);
    assert_eq!(OptionPebNames.apply("auto").unwrap().0, PebNamesMode::Auto);
    assert_eq!(OptionPebNames.apply("on").unwrap().0, PebNamesMode::On);
    for bad in ["", "yes", "AUTO", "1"] {
        assert!(OptionPebNames.apply(bad).is_err(), "accepted {bad:?}");
    }
}

#[test]
fn every_mode_needs_a_windows_compiler_spec() {
    for mode in [PebNamesMode::Off, PebNamesMode::Auto, PebNamesMode::On] {
        assert!(!mode.fires(false, true), "{mode:?} fired on a non-Windows cspec");
        assert!(!mode.fires(false, false), "{mode:?} fired on a non-Windows cspec");
    }
    assert!(PebNamesMode::On.fires(true, false));
    assert!(PebNamesMode::Auto.fires(true, true));
    assert!(!PebNamesMode::Auto.fires(true, false), "auto fired without the user-mode PE fact");
    assert!(!PebNamesMode::Off.fires(true, true));
}

#[test]
fn layouts_are_sorted_non_overlapping_and_inside_their_structure() {
    for layout in [&LAYOUT64, &LAYOUT32] {
        for (spec, size) in [(layout.teb, layout.teb_size), (layout.peb, layout.peb_size)] {
            let widths: Vec<int4> = spec
                .iter()
                .map(|&(_, _, k)| match k {
                    Kind::U1 => 1,
                    Kind::U2 => 2,
                    Kind::U4 => 4,
                    Kind::OpaqueArray(n) => n * layout.ptr_size,
                    Kind::Opaque | Kind::TebPtr | Kind::PebPtr => layout.ptr_size,
                })
                .collect();
            for i in 0..spec.len() {
                assert!(spec[i].0 + widths[i] <= size, "{} overruns", spec[i].1);
                if i > 0 {
                    assert!(spec[i - 1].0 + widths[i - 1] <= spec[i].0, "{} overlaps", spec[i].1);
                }
            }
        }
    }
}

#[test]
fn the_anti_debug_offsets_are_the_documented_ones() {
    let off = |spec: &[Field], name: &str| spec.iter().find(|f| f.1 == name).unwrap().0;
    assert_eq!(off(LAYOUT64.teb, "ProcessEnvironmentBlock"), 0x60);
    assert_eq!(off(LAYOUT64.teb, "Self"), 0x30);
    assert_eq!(off(LAYOUT32.teb, "ProcessEnvironmentBlock"), 0x30);
    assert_eq!(off(LAYOUT32.teb, "Self"), 0x18);
    assert_eq!(off(LAYOUT64.peb, "BeingDebugged"), 2);
    assert_eq!(off(LAYOUT32.peb, "BeingDebugged"), 2);
    assert_eq!(off(LAYOUT64.peb, "NtGlobalFlag"), 0xbc);
    assert_eq!(off(LAYOUT32.peb, "NtGlobalFlag"), 0x68);
    assert_eq!(off(LAYOUT64.peb, "ProcessHeap"), 0x30);
    assert_eq!(off(LAYOUT32.peb, "ProcessHeap"), 0x18);
    assert_eq!(off(LAYOUT64.peb, "Ldr"), 0x18);
    assert_eq!(off(LAYOUT32.peb, "Ldr"), 0xc);
    assert_eq!(off(LAYOUT64.peb, "ImageBaseAddress"), 0x10);
    assert_eq!(off(LAYOUT32.peb, "ImageBaseAddress"), 0x8);
    assert_eq!((LAYOUT64.register, LAYOUT32.register), ("GS_OFFSET", "FS_OFFSET"));
}

#[test]
fn the_built_teb_resolves_its_peb_and_its_self_pointer() {
    let f = factory();
    let ptr = teb_pointer_type(&f, &LAYOUT64).expect("built");
    assert!(is_teb_pointer(&ptr, &LAYOUT64));
    assert!(!is_teb_pointer(&ptr, &LAYOUT32));
    let teb = ptr.get_ptr_to().unwrap();
    assert_eq!((teb.get_name(), teb.get_size()), ("TEB", 0x1878));
    let peb = field(&teb, "ProcessEnvironmentBlock").field_type.get_ptr_to().unwrap();
    assert_eq!((peb.get_name(), peb.get_size()), ("PEB", 0x7d0));
    assert_eq!(field(&peb, "BeingDebugged").field_type.get_size(), 1);
    let heap = &field(&peb, "ProcessHeap").field_type;
    assert_eq!((heap.get_metatype(), heap.get_size()), (type_metatype::TYPE_UNKNOWN, 8));
    let inner = field(&teb, "Self").field_type.get_ptr_to().unwrap();
    assert_eq!(inner.get_name(), "_TEB");
    assert!(!inner.is_incomplete(), "Self must point at a completed structure");
    let inner_peb = field(&inner, "ProcessEnvironmentBlock").field_type.get_ptr_to().unwrap();
    assert!(Rc::ptr_eq(&inner_peb, &peb));
}

#[test]
fn building_twice_reuses_the_same_types() {
    let f = factory();
    let a = teb_pointer_type(&f, &LAYOUT32).unwrap();
    let b = teb_pointer_type(&f, &LAYOUT32).unwrap();
    assert!(Rc::ptr_eq(&a.get_ptr_to().unwrap(), &b.get_ptr_to().unwrap()));
}

#[test]
fn a_foreign_type_under_either_name_makes_the_build_decline() {
    for taken in ["TEB", "_TEB", "PEB"] {
        let f = factory();
        let shell = f.get_type_struct(taken).unwrap();
        let int4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        f.set_fields_struct_raw(&shell, vec![TypeField::new(0, 0, "x", int4)], Vec::new(), 4, 4, 0)
            .unwrap();
        assert!(teb_pointer_type(&f, &LAYOUT64).is_none(), "built over a foreign {taken}");
    }
}

#[test]
fn names_that_changed_between_windows_releases_are_holes() {
    for layout in [&LAYOUT64, &LAYOUT32] {
        for spec in [layout.teb, layout.peb] {
            for name in ["CrossProcessFlags", "EnvironmentUpdateCount", "ApiSetMap", "FreeList", "SparePebPtr0", "SpareBool"] {
                assert!(spec.iter().all(|f| f.1 != name), "{name} is release-dependent");
            }
        }
    }
    assert!(LAYOUT32.peb.iter().all(|f| f.0 != 3), "PEB32+3 is SpareBool on XP and BitField after it");
}

#[test]
fn every_structure_is_variable_length_at_the_largest_published_size() {
    assert_eq!((LAYOUT64.teb_size, LAYOUT64.peb_size), (0x1878, 0x7d0));
    assert_eq!((LAYOUT32.teb_size, LAYOUT32.peb_size), (0x1038, 0x488));
    for layout in [&LAYOUT64, &LAYOUT32] {
        let f = factory();
        let teb = teb_pointer_type(&f, layout).unwrap().get_ptr_to().unwrap();
        let peb = field(&teb, "ProcessEnvironmentBlock").field_type.get_ptr_to().unwrap();
        let inner = field(&teb, "Self").field_type.get_ptr_to().unwrap();
        for t in [&teb, &peb, &inner] {
            assert!(t.is_variable_length(), "{} must not be indexed like an array", t.get_name());
        }
        let inner_self = &field(&inner, "Self").field_type;
        assert_eq!(inner_self.get_metatype(), type_metatype::TYPE_UNKNOWN, "a second Self hop stays untyped");
    }
}

#[test]
fn a_same_shaped_fixed_length_type_is_not_reused() {
    let f = factory();
    let shell = f.get_type_struct("PEB").unwrap();
    let byte = f.get_base(1, type_metatype::TYPE_UINT).unwrap();
    f.set_fields_struct_raw(&shell, vec![TypeField::new(0, 2, "BeingDebugged", byte)], Vec::new(), 0x7d0, 8, 0)
        .unwrap();
    assert!(teb_pointer_type(&f, &LAYOUT64).is_none(), "a fixed-length PEB would wrap offsets into a neighbour");
}
