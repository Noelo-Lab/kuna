// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "lift_diff.rs"]
mod lift_diff;

#[path = "variadic_readonly_format.rs"]
mod variadic_readonly_format;

#[path = "verify_buildstamp.rs"]
mod verify_buildstamp;

#[path = "verify_w10_console_family_e2e.rs"]
mod verify_w10_console_family_e2e;

#[path = "verify_w10_indproto_e2e.rs"]
mod verify_w10_indproto_e2e;

#[path = "verify_w10_spacebase_typing_switch_guard.rs"]
mod verify_w10_spacebase_typing_switch_guard;

#[path = "verify_w5_infra_lift_diff.rs"]
mod verify_w5_infra_lift_diff;
