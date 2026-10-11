// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "decompile_at_e2e.rs"]
mod decompile_at_e2e;

#[path = "ghidra_sim_e2e.rs"]
mod ghidra_sim_e2e;

#[path = "inject_e2e.rs"]
mod inject_e2e;

#[path = "protocol_e2e.rs"]
mod protocol_e2e;

#[path = "register_probe_e2e.rs"]
mod register_probe_e2e;

#[path = "v850_register_lookup_e2e.rs"]
mod v850_register_lookup_e2e;
