// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "compiler_cli.rs"]
mod compiler_cli;

#[path = "constructor_builder.rs"]
mod constructor_builder;

#[path = "encode_roundtrip.rs"]
mod encode_roundtrip;

#[path = "local_collisions.rs"]
mod local_collisions;

#[path = "macrobuilder_golden.rs"]
mod macrobuilder_golden;

#[path = "pattern_diagnostics.rs"]
mod pattern_diagnostics;

#[path = "slghparse_golden.rs"]
mod slghparse_golden;

#[path = "slghscan_golden.rs"]
mod slghscan_golden;
