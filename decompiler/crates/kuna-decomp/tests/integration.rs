// Shared integration-test executable. Process-sensitive tests stay separate.
#[path = "catalog_bytecompat.rs"]
mod catalog_bytecompat;

#[path = "corpus_bootstrap.rs"]
mod corpus_bootstrap;

#[path = "deadcode_b4.rs"]
mod deadcode_b4;

#[path = "decompile_e2e.rs"]
mod decompile_e2e;

#[path = "flow_linkage.rs"]
mod flow_linkage;

#[path = "funcdata_encode_e2e.rs"]
mod funcdata_encode_e2e;

#[path = "heritage_b3.rs"]
mod heritage_b3;

#[path = "options_md_fresh.rs"]
mod options_md_fresh;

#[path = "phase_codegen.rs"]
mod phase_codegen;

#[path = "print_b5_boolless.rs"]
mod print_b5_boolless;

#[path = "printc_parity.rs"]
mod printc_parity;

#[path = "proto_b4.rs"]
mod proto_b4;

#[path = "testcirclerange.rs"]
mod testcirclerange;

#[path = "testfuncproto.rs"]
mod testfuncproto;

#[path = "testkunaregion.rs"]
mod testkunaregion;

#[path = "testparamstore.rs"]
mod testparamstore;

#[path = "testtypes.rs"]
mod testtypes;

#[path = "universalaction_listing.rs"]
mod universalaction_listing;

#[path = "verify_cspecprotos.rs"]
mod verify_cspecprotos;

#[path = "verify_decompile_project_emit.rs"]
mod verify_decompile_project_emit;

#[path = "verify_symbol_snapshot_cache.rs"]
mod verify_symbol_snapshot_cache;

#[path = "verify_w10_actionsetcasts.rs"]
mod verify_w10_actionsetcasts;

#[path = "verify_w10_baseexplicit_piece_verifier.rs"]
mod verify_w10_baseexplicit_piece_verifier;

#[path = "verify_w10_bitfield.rs"]
mod verify_w10_bitfield;

#[path = "verify_w10_bitfield_absorb_r1.rs"]
mod verify_w10_bitfield_absorb_r1;

#[path = "verify_w10_bitfield_activate_r1.rs"]
mod verify_w10_bitfield_activate_r1;

#[path = "verify_w10_bitfield_full.rs"]
mod verify_w10_bitfield_full;

#[path = "verify_w10_callarg_piece.rs"]
mod verify_w10_callarg_piece;

#[path = "verify_w10_callsite_args_adversarial.rs"]
mod verify_w10_callsite_args_adversarial;

#[path = "verify_w10_cast_strategy_adversarial.rs"]
mod verify_w10_cast_strategy_adversarial;

#[path = "verify_w10_charptr_signedness.rs"]
mod verify_w10_charptr_signedness;

#[path = "verify_w10_concat_piece.rs"]
mod verify_w10_concat_piece;

#[path = "verify_w10_condexe_structure_adversarial.rs"]
mod verify_w10_condexe_structure_adversarial;

#[path = "verify_w10_console_family.rs"]
mod verify_w10_console_family;

#[path = "verify_w10_const_prop_phi.rs"]
mod verify_w10_const_prop_phi;

#[path = "verify_w10_displayformat.rs"]
mod verify_w10_displayformat;

#[path = "verify_w10_divmod.rs"]
mod verify_w10_divmod;

#[path = "verify_w10_divmod_corpus.rs"]
mod verify_w10_divmod_corpus;

#[path = "verify_w10_divmod_independent.rs"]
mod verify_w10_divmod_independent;

#[path = "verify_w10_dominant_copy.rs"]
mod verify_w10_dominant_copy;

#[path = "verify_w10_emptyblock_orform.rs"]
mod verify_w10_emptyblock_orform;

#[path = "verify_w10_f0flag_v2_untie.rs"]
mod verify_w10_f0flag_v2_untie;

#[path = "verify_w10_float_cluster.rs"]
mod verify_w10_float_cluster;

#[path = "verify_w10_float_family.rs"]
mod verify_w10_float_family;

#[path = "verify_w10_funcdata_union_cache.rs"]
mod verify_w10_funcdata_union_cache;

#[path = "verify_w10_global_persist_adversarial.rs"]
mod verify_w10_global_persist_adversarial;

#[path = "verify_w10_implied_vars_adversarial.rs"]
mod verify_w10_implied_vars_adversarial;

#[path = "verify_w10_inline_inject.rs"]
mod verify_w10_inline_inject;

#[path = "verify_w10_input_params.rs"]
mod verify_w10_input_params;

#[path = "verify_w10_input_prototype_adversarial.rs"]
mod verify_w10_input_prototype_adversarial;

#[path = "verify_w10_jts_chain.rs"]
mod verify_w10_jts_chain;

#[path = "verify_w10_jumptable_emulate.rs"]
mod verify_w10_jumptable_emulate;

#[path = "verify_w10_lanedivide.rs"]
mod verify_w10_lanedivide;

#[path = "verify_w10_merge_casts.rs"]
mod verify_w10_merge_casts;

#[path = "verify_w10_merge_cluster.rs"]
mod verify_w10_merge_cluster;

#[path = "verify_w10_merge_facing.rs"]
mod verify_w10_merge_facing;

#[path = "verify_w10_mergepiece_dynsym.rs"]
mod verify_w10_mergepiece_dynsym;

#[path = "verify_w10_mips_callother.rs"]
mod verify_w10_mips_callother;

#[path = "verify_w10_partial_types.rs"]
mod verify_w10_partial_types;

#[path = "verify_w10_printc_decl_render.rs"]
mod verify_w10_printc_decl_render;

#[path = "verify_w10_proto_unlock.rs"]
mod verify_w10_proto_unlock;

#[path = "verify_w10_pushpartialsymbol.rs"]
mod verify_w10_pushpartialsymbol;

#[path = "verify_w10_rel_pointer.rs"]
mod verify_w10_rel_pointer;

#[path = "verify_w10_spacebase_ptrsub_cast.rs"]
mod verify_w10_spacebase_ptrsub_cast;

#[path = "verify_w10_spacebase_render.rs"]
mod verify_w10_spacebase_render;

#[path = "verify_w10_stackslot_ssa.rs"]
mod verify_w10_stackslot_ssa;

#[path = "verify_w10_struct_corpus.rs"]
mod verify_w10_struct_corpus;

#[path = "verify_w10_struct_return.rs"]
mod verify_w10_struct_return;

#[path = "verify_w10_structreturn_v2.rs"]
mod verify_w10_structreturn_v2;

#[path = "verify_w10_symbol_consolidate.rs"]
mod verify_w10_symbol_consolidate;

#[path = "verify_w10_symbol_consolidate_adversarial.rs"]
mod verify_w10_symbol_consolidate_adversarial;

#[path = "verify_w10_symbol_consolidate_verifier2.rs"]
mod verify_w10_symbol_consolidate_verifier2;

#[path = "verify_w10_typed_access.rs"]
mod verify_w10_typed_access;

#[path = "verify_w10_typeseed_constptr.rs"]
mod verify_w10_typeseed_constptr;

#[path = "verify_w10_union_render_ptradd_up.rs"]
mod verify_w10_union_render_ptradd_up;

#[path = "verify_w10_union_render_r2.rs"]
mod verify_w10_union_render_r2;

#[path = "verify_w10_union_scoring.rs"]
mod verify_w10_union_scoring;

#[path = "verify_w10_union_value.rs"]
mod verify_w10_union_value;

#[path = "verify_w10_usepoint_register_symbol.rs"]
mod verify_w10_usepoint_register_symbol;

#[path = "verify_w3_ir_block.rs"]
mod verify_w3_ir_block;

#[path = "verify_w3_ir_flow.rs"]
mod verify_w3_ir_flow;

#[path = "verify_w3_ir_funcdata.rs"]
mod verify_w3_ir_funcdata;

#[path = "verify_w3_ir_funcdata_op.rs"]
mod verify_w3_ir_funcdata_op;

#[path = "verify_w3_ir_funcdata_varnode.rs"]
mod verify_w3_ir_funcdata_varnode;

#[path = "verify_w3_ir_jumptable.rs"]
mod verify_w3_ir_jumptable;

#[path = "verify_w3_ir_op.rs"]
mod verify_w3_ir_op;

#[path = "verify_w3_ir_userop_inject.rs"]
mod verify_w3_ir_userop_inject;

#[path = "verify_w3_ir_varnode.rs"]
mod verify_w3_ir_varnode;

#[path = "verify_w3_kuna_flow_pack.rs"]
mod verify_w3_kuna_flow_pack;

#[path = "verify_w4_fw_action.rs"]
mod verify_w4_fw_action;

#[path = "verify_w4_fw_arch_frontends.rs"]
mod verify_w4_fw_arch_frontends;

#[path = "verify_w4_fw_architecture.rs"]
mod verify_w4_fw_architecture;

#[path = "verify_w4_fw_architecture_r2.rs"]
mod verify_w4_fw_architecture_r2;

#[path = "verify_w4_fw_cpool_graph.rs"]
mod verify_w4_fw_cpool_graph;

#[path = "verify_w4_fw_options.rs"]
mod verify_w4_fw_options;

#[path = "verify_w4_kuna_p0_pack.rs"]
mod verify_w4_kuna_p0_pack;

#[path = "verify_w4_p0_database.rs"]
mod verify_w4_p0_database;

#[path = "verify_w4_p0_override_comment.rs"]
mod verify_w4_p0_override_comment;

#[path = "verify_w4x_flow_linkage.rs"]
mod verify_w4x_flow_linkage;

#[path = "verify_w5_dtype_expand.rs"]
mod verify_w5_dtype_expand;

#[path = "verify_w5_kuna_rule_pack.rs"]
mod verify_w5_kuna_rule_pack;

#[path = "verify_w5_s3_condexe_expression.rs"]
mod verify_w5_s3_condexe_expression;

#[path = "verify_w5_s3_coreaction_early.rs"]
mod verify_w5_s3_coreaction_early;

#[path = "verify_w5_s3_coreaction_early_r2.rs"]
mod verify_w5_s3_coreaction_early_r2;

#[path = "verify_w5_s3_heritage.rs"]
mod verify_w5_s3_heritage;

#[path = "verify_w5_s3_rules_1.rs"]
mod verify_w5_s3_rules_1;

#[path = "verify_w5_s3_rules_2.rs"]
mod verify_w5_s3_rules_2;

#[path = "verify_w5_s3_rules_4.rs"]
mod verify_w5_s3_rules_4;

#[path = "verify_w5_s3_rules_5.rs"]
mod verify_w5_s3_rules_5;

#[path = "verify_w5_s3_rules_6.rs"]
mod verify_w5_s3_rules_6;

#[path = "verify_w5_s3_rules_7.rs"]
mod verify_w5_s3_rules_7;

#[path = "verify_w5_s3_rules_8.rs"]
mod verify_w5_s3_rules_8;

#[path = "verify_w5_s3_subflow.rs"]
mod verify_w5_s3_subflow;

#[path = "verify_w5_s3_transform.rs"]
mod verify_w5_s3_transform;

#[path = "verify_w5x_helpers_completion.rs"]
mod verify_w5x_helpers_completion;

#[path = "verify_w6_harness_unittests.rs"]
mod verify_w6_harness_unittests;

#[path = "verify_w6_kuna_s4s5_pack.rs"]
mod verify_w6_kuna_s4s5_pack;

#[path = "verify_w6_s4_coreaction_protos.rs"]
mod verify_w6_s4_coreaction_protos;

#[path = "verify_w6_s4_fspec_1.rs"]
mod verify_w6_s4_fspec_1;

#[path = "verify_w6_s4_fspec_2.rs"]
mod verify_w6_s4_fspec_2;

#[path = "verify_w6_s4_fspec_3.rs"]
mod verify_w6_s4_fspec_3;

#[path = "verify_w6_s4_modelrules.rs"]
mod verify_w6_s4_modelrules;

#[path = "verify_w6_s5_bitfield.rs"]
mod verify_w6_s5_bitfield;

#[path = "verify_w6_s5_constseq_prefersplit.rs"]
mod verify_w6_s5_constseq_prefersplit;

#[path = "verify_w6_s5_double.rs"]
mod verify_w6_s5_double;

#[path = "verify_w6_s5_rangeutil.rs"]
mod verify_w6_s5_rangeutil;

#[path = "verify_w6_s5_rangeutil_r3.rs"]
mod verify_w6_s5_rangeutil_r3;

#[path = "verify_w6_s5_type_1.rs"]
mod verify_w6_s5_type_1;

#[path = "verify_w6_s5_type_2.rs"]
mod verify_w6_s5_type_2;

#[path = "verify_w6_s5_type_2_adversarial.rs"]
mod verify_w6_s5_type_2_adversarial;

#[path = "verify_w6_s5_type_3.rs"]
mod verify_w6_s5_type_3;

#[path = "verify_w6_s5_typeop.rs"]
mod verify_w6_s5_typeop;

#[path = "verify_w6_s5_unionresolve.rs"]
mod verify_w6_s5_unionresolve;

#[path = "verify_w7_harness_kunaregion.rs"]
mod verify_w7_harness_kunaregion;

#[path = "verify_w7_m1_closure.rs"]
mod verify_w7_m1_closure;

#[path = "verify_w7_s37_coreaction_cleanup.rs"]
mod verify_w7_s37_coreaction_cleanup;

#[path = "verify_w7_s6_dynamic_pack.rs"]
mod verify_w7_s6_dynamic_pack;

#[path = "verify_w7_s6_merge.rs"]
mod verify_w7_s6_merge;

#[path = "verify_w7_s6_variable_cover.rs"]
mod verify_w7_s6_variable_cover;

#[path = "verify_w7_s6_varmap.rs"]
mod verify_w7_s6_varmap;

#[path = "verify_w7_s6_varmap_r2.rs"]
mod verify_w7_s6_varmap_r2;

#[path = "verify_w7_s6_varmap_r3.rs"]
mod verify_w7_s6_varmap_r3;

#[path = "verify_w7_s7_blockaction.rs"]
mod verify_w7_s7_blockaction;

#[path = "verify_w7_s7_kuna_loweredswitch.rs"]
mod verify_w7_s7_kuna_loweredswitch;

#[path = "verify_w7_s7_kuna_regiongraph.rs"]
mod verify_w7_s7_kuna_regiongraph;

#[path = "verify_w7_s7_kuna_regionid.rs"]
mod verify_w7_s7_kuna_regionid;

#[path = "verify_w8_fw_universalaction.rs"]
mod verify_w8_fw_universalaction;

#[path = "verify_w8_s9_coreaction_render.rs"]
mod verify_w8_s9_coreaction_render;

#[path = "verify_w8_s9_prettyprint.rs"]
mod verify_w8_s9_prettyprint;

#[path = "verify_w8_s9_printc.rs"]
mod verify_w8_s9_printc;

#[path = "verify_w8_s9_printlanguage_cast.rs"]
mod verify_w8_s9_printlanguage_cast;

#[path = "verify_w8_s9_stringmanage_pack.rs"]
mod verify_w8_s9_stringmanage_pack;

#[path = "verify_w8x_allowlist.rs"]
mod verify_w8x_allowlist;

#[path = "verify_w9x_arch_engine_glue.rs"]
mod verify_w9x_arch_engine_glue;
