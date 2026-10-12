// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "golden_addrsort.rs"]
mod golden_addrsort;

#[path = "partmap_mut.rs"]
mod partmap_mut;

#[path = "testmarshal.rs"]
mod testmarshal;

#[path = "verify_w1_base_foundation.rs"]
mod verify_w1_base_foundation;

#[path = "verify_w1_base_marshal.rs"]
mod verify_w1_base_marshal;

#[path = "verify_w1_base_marshal_r2.rs"]
mod verify_w1_base_marshal_r2;

#[path = "verify_w1_base_space_address.rs"]
mod verify_w1_base_space_address;

#[path = "verify_w1_base_space_address_r2.rs"]
mod verify_w1_base_space_address_r2;

#[path = "verify_w1_base_util.rs"]
mod verify_w1_base_util;

#[path = "verify_w1_base_xml.rs"]
mod verify_w1_base_xml;

#[path = "verify_w1_base_xml_r2.rs"]
mod verify_w1_base_xml_r2;

#[path = "verify_w1_harness_unittests.rs"]
mod verify_w1_harness_unittests;
