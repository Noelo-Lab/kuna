// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "boolean_complement.rs"]
mod boolean_complement;

#[path = "golden_float.rs"]
mod golden_float;

#[path = "golden_opbehavior.rs"]
mod golden_opbehavior;

#[path = "testfloatemu.rs"]
mod testfloatemu;

#[path = "testmultiprec.rs"]
mod testmultiprec;

#[path = "verify_w1_harness_unittests.rs"]
mod verify_w1_harness_unittests;

#[path = "verify_w1_num_float_multiprec.rs"]
mod verify_w1_num_float_multiprec;

#[path = "verify_w1_num_pcode_semantics.rs"]
mod verify_w1_num_pcode_semantics;

#[path = "verify_w2_harness_floatemu.rs"]
mod verify_w2_harness_floatemu;
