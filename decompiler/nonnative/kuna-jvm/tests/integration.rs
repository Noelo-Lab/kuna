//! Integration tests for the JVM engine.

#[path = "rt_jhash.rs"]
mod rt_jhash;
#[path = "rt_kernel.rs"]
mod rt_kernel;
#[path = "rt_jchar.rs"]
mod rt_jchar;
#[path = "rt_jnum.rs"]
mod rt_jnum;
#[path = "collections.rs"]
mod collections;
#[path = "text_buffer.rs"]
mod text_buffer;
#[path = "util_misc.rs"]
mod util_misc;
#[path = "f2_types.rs"]
mod f2_types;
#[path = "f2_pool_match.rs"]
mod f2_pool_match;
#[path = "f4_context.rs"]
mod f4_context;
#[path = "f4_driver.rs"]
mod f4_driver;
#[path = "f4_logger.rs"]
mod f4_logger;
#[path = "f4_options.rs"]
mod f4_options;
#[path = "f4_renamer.rs"]
mod f4_renamer;
#[path = "f4_struct_context.rs"]
mod f4_struct_context;
#[path = "f4_util.rs"]
mod f4_util;
#[path = "f5_cfg_golden.rs"]
mod f5_cfg_golden;
#[path = "f5_dominator.rs"]
mod f5_dominator;
#[path = "f5_cfg_blocks.rs"]
mod f5_cfg_blocks;
#[path = "f6_support.rs"]
mod f6_support;
#[path = "zip_reader.rs"]
mod zip_reader;
#[path = "context_source.rs"]
mod context_source;
#[path = "jdk_runtime.rs"]
mod jdk_runtime;
#[path = "jimage_synthetic.rs"]
mod jimage_synthetic;
#[path = "k2_statement_graph.rs"]
mod k2_statement_graph;
#[path = "class_model.rs"]
mod class_model;
#[path = "k5_vars.rs"]
mod k5_vars;
#[path = "k5_flow.rs"]
mod k5_flow;
#[path = "struct_classfile.rs"]
mod struct_classfile;
#[path = "struct_bytecode.rs"]
mod struct_bytecode;
#[path = "common/mod.rs"]
pub mod common;
#[path = "k2_dom_helper.rs"]
mod k2_dom_helper;
#[path = "f5_irreducible.rs"]
mod f5_irreducible;
#[path = "f5_method_properties.rs"]
mod f5_method_properties;
#[path = "f1_context.rs"]
mod f1_context;
#[path = "f2_context.rs"]
mod f2_context;
#[path = "k1_exprents.rs"]
mod k1_exprents;
#[path = "b01_invocation.rs"]
mod b01_invocation;
#[path = "b02_const_exprent.rs"]
mod b02_const_exprent;
#[path = "b02_function_exprent.rs"]
mod b02_function_exprent;
#[path = "b02_expr_util.rs"]
mod b02_expr_util;
#[path = "b03_exprents.rs"]
mod b03_exprents;
#[path = "b04_render.rs"]
mod b04_render;
#[path = "b06_sforms.rs"]
mod b06_sforms;
#[path = "b07a_var_definition.rs"]
mod b07a_var_definition;
#[path = "b07b_classpath.rs"]
mod b07b_classpath;
#[path = "b08_lift.rs"]
mod b08_lift;
#[path = "b09_stack_vars.rs"]
mod b09_stack_vars;
#[path = "b10_simplify.rs"]
mod b10_simplify;
#[path = "b11_condense.rs"]
mod b11_condense;
#[path = "b11_finally.rs"]
mod b11_finally;
#[path = "b12_loops.rs"]
mod b12_loops;
#[path = "b13_structure_helpers.rs"]
mod b13_structure_helpers;
#[path = "b14_structure.rs"]
mod b14_structure;
#[path = "b15_switch.rs"]
mod b15_switch;
#[path = "b16_pattern_match.rs"]
mod b16_pattern_match;
#[path = "b18_class_writer.rs"]
mod b18_class_writer;
#[path = "b19_classes.rs"]
mod b19_classes;
#[path = "b20_nested.rs"]
mod b20_nested;
#[path = "kuna_corpus.rs"]
mod kuna_corpus;
#[path = "trace_vectors.rs"]
mod trace_vectors;
#[path = "trace_file.rs"]
mod trace_file;
#[path = "s3_parity.rs"]
mod s3_parity;
#[path = "kuna_support.rs"]
mod kuna_support;
#[path = "kuna_input.rs"]
mod kuna_input;
#[path = "kuna_options.rs"]
mod kuna_options;
#[path = "kuna_fixes.rs"]
mod kuna_fixes;
#[path = "kuna_fix_kt_writer.rs"]
mod kuna_fix_kt_writer;
#[path = "kuna_fix_kt_exits.rs"]
mod kuna_fix_kt_exits;
#[path = "kuna_fix_kt_ctors.rs"]
mod kuna_fix_kt_ctors;
#[path = "kuna_fix_kt_references.rs"]
mod kuna_fix_kt_references;
#[path = "kuna_fix_followup_merges.rs"]
mod kuna_fix_followup_merges;
#[path = "kuna_fix_kt_loops.rs"]
mod kuna_fix_kt_loops;
#[path = "kuna_fix_kt_print.rs"]
mod kuna_fix_kt_print;
#[path = "kuna_fix_handler_copies.rs"]
mod kuna_fix_handler_copies;
#[path = "kuna_fix_type_switch.rs"]
mod kuna_fix_type_switch;
#[path = "kuna_fix_irreducible_try.rs"]
mod kuna_fix_irreducible_try;
#[path = "kuna_fix_jinfer.rs"]
mod kuna_fix_jinfer;
#[path = "kuna_fix_jinfer2.rs"]
mod kuna_fix_jinfer2;
#[path = "kuna_inventory.rs"]
mod kuna_inventory;
#[path = "kuna_spans.rs"]
mod kuna_spans;
#[path = "kuna_deadline.rs"]
mod kuna_deadline;
#[path = "kuna_driver.rs"]
mod kuna_driver;
#[path = "kt1_metadata.rs"]
mod kt1_metadata;
#[path = "trace_sweep.rs"]
mod trace_sweep;
#[path = "kt0_hooks.rs"]
mod kt0_hooks;
#[path = "kt3_elements.rs"]
mod kt3_elements;
#[path = "kt7_kotlin_writer.rs"]
mod kt7_kotlin_writer;
#[path = "kt4_members.rs"]
mod kt4_members;
#[path = "kt6_passes.rs"]
mod kt6_passes;
#[path = "kt6_render.rs"]
mod kt6_render;
#[path = "kp1_plugins.rs"]
mod kp1_plugins;
#[path = "kt5_exprents.rs"]
mod kt5_exprents;
#[path = "kt8_kotlin_runs.rs"]
mod kt8_kotlin_runs;
#[path = "kuna_fix_switch_yield_const_retype.rs"]
mod kuna_fix_switch_yield_const_retype;
#[path = "kuna_fix_numeric_casts.rs"]
mod kuna_fix_numeric_casts;
#[path = "kuna_stackguard.rs"]
mod kuna_stackguard;
#[path = "kuna_fix_kt_semaphore.rs"]
mod kuna_fix_kt_semaphore;
#[path = "kuna_fix_wrong2.rs"]
mod kuna_fix_wrong2;
#[path = "kuna_fix_kt_ctor_jumps.rs"]
mod kuna_fix_kt_ctor_jumps;
#[path = "kuna_fix_kt_synthetic_object_super_call.rs"]
mod kuna_fix_kt_synthetic_object_super_call;
#[path = "kuna_fix_kt_when_jumps.rs"]
mod kuna_fix_kt_when_jumps;
#[path = "kuna_fix_anonymous_class_created_elsewhere.rs"]
mod kuna_fix_anonymous_class_created_elsewhere;
#[path = "kuna_fix_captured_vars.rs"]
mod kuna_fix_captured_vars;
#[path = "kuna_fix_kt_ctor_super_calls.rs"]
mod kuna_fix_kt_ctor_super_calls;
#[path = "kuna_fix_kt_operators.rs"]
mod kuna_fix_kt_operators;
#[path = "kuna_fix_handler_copy_shared_body.rs"]
mod kuna_fix_handler_copy_shared_body;
#[path = "kuna_fix_kt_valid.rs"]
mod kuna_fix_kt_valid;
#[path = "kuna_fix_kt_members_without_metadata.rs"]
mod kuna_fix_kt_members_without_metadata;
#[path = "kuna_fix_kt_ops2.rs"]
mod kuna_fix_kt_ops2;
#[path = "kuna_fix_jsugar.rs"]
mod kuna_fix_jsugar;
#[path = "kuna_fix_jwrong.rs"]
mod kuna_fix_jwrong;
#[path = "kuna_javac_layout.rs"]
mod kuna_javac_layout;
#[path = "kuna_source_locals.rs"]
mod kuna_source_locals;
#[path = "kuna_fix_kt_names_types.rs"]
mod kuna_fix_kt_names_types;
#[path = "kuna_fix_ktinline.rs"]
mod kuna_fix_ktinline;
#[path = "kuna_fix_jwrong2.rs"]
mod kuna_fix_jwrong2;
#[path = "kuna_fix_jpatterns.rs"]
mod kuna_fix_jpatterns;
#[path = "kuna_fix_jfinally.rs"]
mod kuna_fix_jfinally;
#[path = "kuna_fix_jswitch.rs"]
mod kuna_fix_jswitch;
#[path = "kuna_fix_jwrong3.rs"]
mod kuna_fix_jwrong3;
#[path = "kuna_javac_loops.rs"]
mod kuna_javac_loops;
#[path = "kuna_fix_overlapping_ranges.rs"]
mod kuna_fix_overlapping_ranges;
#[path = "kuna_fix_jkmode.rs"]
mod kuna_fix_jkmode;
#[path = "kuna_fix_kt_valid2.rs"]
mod kuna_fix_kt_valid2;
#[path = "kuna_fix_ktunrep.rs"]
mod kuna_fix_ktunrep;
#[path = "kuna_fix_ktlockexpr.rs"]
mod kuna_fix_ktlockexpr;
