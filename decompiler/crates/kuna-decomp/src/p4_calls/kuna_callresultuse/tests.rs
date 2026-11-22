use super::{runtime_entry, Evidence, RuntimeEntry};

#[test]
fn a_register_specific_entry_point_takes_the_register_in_its_name() {
    assert_eq!(runtime_entry("_objc_release_x19"), Some(RuntimeEntry { input: 19, result: Evidence::No }));
    assert_eq!(runtime_entry("swift_retain_x8"), Some(RuntimeEntry { input: 8, result: Evidence::Unknown }));
    assert_eq!(runtime_entry("_objc_retain_x22"), Some(RuntimeEntry { input: 22, result: Evidence::Unknown }));
}

#[test]
fn the_x0_entry_points_take_their_object_in_x0() {
    let x0 = |result| Some(RuntimeEntry { input: 0, result });
    assert_eq!(runtime_entry("_objc_retainAutoreleasedReturnValue"), x0(Evidence::Unknown));
    assert_eq!(runtime_entry("swift_bridgeObjectRelease"), x0(Evidence::No));
    assert_eq!(runtime_entry("objc_msgSend$containsObject:"), x0(Evidence::Unknown));
}

#[test]
fn other_names_are_not_runtime_entry_points() {
    assert_eq!(runtime_entry("objc_release_x+1"), None);
    assert_eq!(runtime_entry("objc_release_x29"), None);
    assert_eq!(runtime_entry("my_retain_x3"), None);
}
