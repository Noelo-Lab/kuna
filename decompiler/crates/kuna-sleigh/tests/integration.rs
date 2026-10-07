// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "arm_neon_vldm_regression.rs"]
mod arm_neon_vldm_regression;

#[path = "arm_pop_return.rs"]
mod arm_pop_return;

#[path = "combined_decode.rs"]
mod combined_decode;

#[path = "golden_lift.rs"]
mod golden_lift;

#[path = "instruction_mask.rs"]
mod instruction_mask;

#[path = "macro_symbol.rs"]
mod macro_symbol;

#[path = "ppc_nested_operands.rs"]
mod ppc_nested_operands;

#[path = "verify_context_commits.rs"]
mod verify_context_commits;

#[path = "verify_ctxsnapshot.rs"]
mod verify_ctxsnapshot;

#[path = "verify_w2_sleigh_context.rs"]
mod verify_w2_sleigh_context;

#[path = "verify_w2_sleigh_core.rs"]
mod verify_w2_sleigh_core;

#[path = "verify_w2_sleigh_emulate.rs"]
mod verify_w2_sleigh_emulate;

#[path = "verify_w2_sleigh_loadimage.rs"]
mod verify_w2_sleigh_loadimage;

#[path = "verify_w2_sleigh_pattern.rs"]
mod verify_w2_sleigh_pattern;

#[path = "verify_w2_sleigh_pcodeparse.rs"]
mod verify_w2_sleigh_pcodeparse;

#[path = "verify_w2_sleigh_semantics.rs"]
mod verify_w2_sleigh_semantics;

#[path = "verify_w2_sleigh_symbol.rs"]
mod verify_w2_sleigh_symbol;

#[path = "verify_w2_sleigh_translate.rs"]
mod verify_w2_sleigh_translate;

#[path = "verify_w2core_walkback.rs"]
mod verify_w2core_walkback;

#[path = "ws4a_pattern_build_golden.rs"]
mod ws4a_pattern_build_golden;

#[path = "x86_maxlen_nop_regression.rs"]
mod x86_maxlen_nop_regression;

#[path = "x86_rdtsc_zero_extend.rs"]
mod x86_rdtsc_zero_extend;
