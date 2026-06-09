# angr-vs-kuna recovery matrix

**Generated** by `python -m kuna.pipeline.sweep` -- do not edit by hand.

Each row is one angr testcase function. `Score` ranks where angr's decompilation looks structurally better than kuna's (higher = bigger gap = better feature candidate). `Hint` notes whether an existing kuna option may already address it. The worker turns a top row into one option-gated kuna feature + PR.

Reference: angr 9.2.213. Comparison is structural (gotos / labels / switch / loops / loc), never raw-text.

**185 testcases swept; 121 show an angr advantage.**

| # | Score | Testcase :: function | Kinds | Why angr is better | Coverage hint |
|---|---|---|---|---|---|
| 1 | 14 | `test_decompiling_x8664_cvs :: main` | goto-reduction, label-reduction, switch-recovery, recovery-failure | ref has fewer gotos (10 vs 29); ref has fewer labels (9 vs 22); ref recovered a switch kuna did not (1 vs 0); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; check loweredswitch / switchmodbound (S2 switch-model); kuna aborted/failed structuring here |
| 2 | 14 | `test_decompiling_tr_O2_parse_str :: parse_str` | goto-reduction, label-reduction, switch-recovery, recovery-failure | ref has fewer gotos (11 vs 22); ref has fewer labels (8 vs 17); ref recovered a switch kuna did not (1 vs 0); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; check loweredswitch / switchmodbound (S2 switch-model); kuna aborted/failed structuring here |
| 3 | 14 | `test_decompiling_abnormal_switch_case_case3 :: memmove` | goto-reduction, label-reduction, switch-recovery, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref recovered a switch kuna did not (1 vs 0); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; check loweredswitch / switchmodbound (S2 switch-model); kuna aborted/failed structuring here |
| 4 | 10 | `test_decompiling_prototype_recovery_two_blocks :: screen_columns` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 66% shorter (14 vs 41 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 5 | 10 | `test_decompiling_ptx_fix_output_parameters :: fix_output_parameters` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (2 vs 29); ref has fewer labels (2 vs 24); ref is 85% shorter (135 vs 880 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 6 | 10 | `test_decompiling_base32_basenc_do_decode :: do_decode` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 9); ref has fewer labels (0 vs 9); ref is 73% shorter (101 vs 379 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 7 | 10 | `test_decompiling_sort_specify_nmerge :: specify_nmerge` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 70% shorter (47 vs 159 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 8 | 10 | `test_decompiling_tr_build_spec_list :: build_spec_list` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (4 vs 11); ref has fewer labels (3 vs 7); ref is 33% shorter (122 vs 181 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 9 | 10 | `test_eliminating_stack_canary_reused_stack_chk_fail_call :: split_3` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (4 vs 10); ref has fewer labels (4 vs 5); ref is 23% shorter (197 vs 256 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 10 | 10 | `test_decompiling_printenv_main :: main` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 43% shorter (35 vs 61 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 11 | 10 | `test_od_else_simplification :: skip` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (2 vs 3); ref has fewer labels (2 vs 3); ref is 44% shorter (129 vs 230 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 12 | 10 | `test_ternary_propagation_2 :: print_only_size` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 76% shorter (12 vs 50 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 13 | 10 | `test_ret_dedupe_fakeret_1 :: sort_found_occurs` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 38); ref has fewer labels (0 vs 29); ref is 99% shorter (7 vs 969 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 14 | 10 | `test_ifelseflatten_iprule :: flush_rule` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 39); ref has fewer labels (0 vs 15); ref is 92% shorter (41 vs 501 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 15 | 10 | `test_ifelseflatten_clientloop :: client_request_tun_fwd` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 57% shorter (69 vs 160 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 16 | 10 | `test_tail_tail_bytes_ret_dup :: tail_bytes` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (1 vs 8); ref has fewer labels (1 vs 8); ref is 66% shorter (96 vs 281 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 17 | 10 | `test_stty_recover_mode_ret_dup_region :: recover_mode` | goto-reduction, label-reduction, conciseness, recovery-failure | ref has fewer gotos (0 vs 3); ref has fewer labels (0 vs 2); ref is 91% shorter (26 vs 300 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 18 | 9 | `test_decompiling_phoenix_natural_loop_region_head_in_body :: sub_442300` | goto-reduction, label-reduction, loop-recovery, conciseness | ref has fewer gotos (1 vs 18); ref has fewer labels (1 vs 13); ref recovered more loops (2 vs 1); ref is 16% shorter (208 vs 248 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: loop structuring; uncovered: structuring / expression folding |
| 19 | 9 | `test_decompiling_fmt0_main :: main` | goto-reduction, label-reduction, recovery-failure | ref has fewer gotos (3 vs 8); ref has fewer labels (3 vs 5); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; kuna aborted/failed structuring here |
| 20 | 9 | `test_decompiling_x8664_mv_O2 :: main` | goto-reduction, label-reduction, recovery-failure | ref has fewer gotos (9 vs 17); ref has fewer labels (8 vs 15); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; kuna aborted/failed structuring here |
| 21 | 9 | `test_decompiling_mv0_main :: main` | goto-reduction, label-reduction, recovery-failure | ref has fewer gotos (10 vs 29); ref has fewer labels (9 vs 16); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; kuna aborted/failed structuring here |
| 22 | 9 | `test_reverting_switch_clustering_and_lowering_fmt_main :: main` | goto-reduction, label-reduction, recovery-failure | ref has fewer gotos (2 vs 5); ref has fewer labels (2 vs 4); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; kuna aborted/failed structuring here |
| 23 | 9 | `test_decompiling_missing_function_call :: main` | switch-recovery, recovery-failure | ref recovered a switch kuna did not (1 vs 0); kuna emitted a recovery-failure marker | check loweredswitch / switchmodbound (S2 switch-model); kuna aborted/failed structuring here |
| 24 | 8 | `test_decompiling_optimized_memcpy :: sub_42cca0` | goto-reduction, conciseness, recovery-failure | ref has fewer gotos (16 vs 32); ref is 30% shorter (243 vs 349 loc); kuna emitted a recovery-failure marker | uncovered: control-flow structuring (no kuna option yet); uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 25 | 8 | `test_reverting_switch_clustering_and_lowering_cat_main :: main` | switch-recovery, loop-recovery | ref recovered a switch kuna did not (1 vs 0); ref recovered more loops (2 vs 0) | check loweredswitch / switchmodbound (S2 switch-model); uncovered: loop structuring |
| 26 | 8 | `test_switch_case_shared_case_nodes_b2sum_digest :: main` | switch-recovery, loop-recovery | ref recovered a switch kuna did not (1 vs 0); ref recovered more loops (2 vs 0) | check loweredswitch / switchmodbound (S2 switch-model); uncovered: loop structuring |
| 27 | 8 | `test_decompiling_incorrect_duplication_chcon_main :: main` | switch-recovery, loop-recovery | ref recovered a switch kuna did not (1 vs 0); ref recovered more loops (1 vs 0) | check loweredswitch / switchmodbound (S2 switch-model); uncovered: loop structuring |
| 28 | 6 | `test_decompilation_call_expr_folding_into_if_conditions :: find_bind_mount` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 4); ref has fewer labels (0 vs 3); ref is 53% shorter (63 vs 134 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 29 | 6 | `test_decompiling_nl_i386_pie :: usage` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 62% shorter (80 vs 210 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 30 | 6 | `test_decompiling_tee_O2_x2nrealloc :: x2nrealloc` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 33% shorter (24 vs 36 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 31 | 6 | `test_who_condensing_opt_reversion :: scan_entries` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (1 vs 2); ref has fewer labels (1 vs 2); ref is 27% shorter (98 vs 134 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 32 | 6 | `test_reverting_switch_lowering_cksum_digest_print_filename :: print_filename` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 6); ref has fewer labels (0 vs 6); ref is 67% shorter (41 vs 126 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 33 | 6 | `test_numfmt_process_field :: process_field` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 64% shorter (36 vs 100 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 34 | 6 | `test_ifelseflatten_certtool_common :: cipher_to_flags` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 3); ref has fewer labels (0 vs 3); ref is 42% shorter (75 vs 130 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 35 | 6 | `test_ite_region_converter_missing_break_statement :: authenticate` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 48% shorter (21 vs 40 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 36 | 6 | `test_decompiling_cancel_sys_incorrect_memory_write_removal :: sub_140005234` | goto-reduction, label-reduction, conciseness | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1); ref is 25% shorter (68 vs 91 loc) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring; uncovered: structuring / expression folding |
| 37 | 5 | `test_decompiling_true_1804_x86_64 :: usage` | goto-reduction, label-reduction | ref has fewer gotos (2 vs 3); ref has fewer labels (2 vs 3) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 38 | 5 | `test_decompiling_dir_gcc_O0_main :: main` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 2); ref has fewer labels (0 vs 2) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 39 | 5 | `test_decompiling_1after909_doit :: doit` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 40 | 5 | `test_decompiling_morton_my_message_callback :: my_message_callback` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 41 | 5 | `test_decompiling_newburry_main :: main` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 42 | 5 | `test_decompiling_tee_O2_tail_jumps :: setlocale_null_androidfix` | conciseness, recovery-failure | ref is 29% shorter (5 vs 7 loc); kuna emitted a recovery-failure marker | uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 43 | 5 | `test_decompiling_dd_argmatch_to_argument_noeagerreturns :: argmatch_to_argument` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 44 | 5 | `test_call_return_variable_folding :: print_long_format` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 3); ref has fewer labels (0 vs 2) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 45 | 5 | `test_decompiling_du_humblock_missing_conditions :: humblock` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 2); ref has fewer labels (0 vs 2) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 46 | 5 | `test_comma_separated_statement_expression_whoami :: main` | conciseness, recovery-failure | ref is 51% shorter (38 vs 77 loc); kuna emitted a recovery-failure marker | uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 47 | 5 | `test_no_switch_case_touch_touch :: touch` | conciseness, recovery-failure | ref is 32% shorter (117 vs 172 loc); kuna emitted a recovery-failure marker | uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 48 | 5 | `test_sensitive_eager_returns :: bar` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 1); ref has fewer labels (0 vs 1) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 49 | 5 | `test_stop_iteration_in_canary_init_stmt :: sub_4010b1` | conciseness, recovery-failure | ref is 33% shorter (4 vs 6 loc); kuna emitted a recovery-failure marker | uncovered: structuring / expression folding; kuna aborted/failed structuring here |
| 50 | 5 | `test_phoenix_last_resort_refinement_on_region_with_multiple_successors :: sub_140005980` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 4); ref has fewer labels (0 vs 3) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 51 | 5 | `test_decompiling_abnormal_switch_case_within_a_loop_with_redundant_jump :: sub_1400040c0` | goto-reduction, label-reduction | ref has fewer gotos (34 vs 37); ref has fewer labels (15 vs 20) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 52 | 5 | `test_decompiling_abnormal_switch_case_within_a_loop_case_1 :: sub_41d560` | goto-reduction, label-reduction | ref has fewer gotos (19 vs 21); ref has fewer labels (11 vs 12) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 53 | 5 | `test_decompiling_rep_stosq :: sub_403670` | goto-reduction, label-reduction | ref has fewer gotos (0 vs 2); ref has fewer labels (0 vs 2) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 54 | 5 | `test_decompiling_reused_entries_between_switch_cases :: sub_40d760` | goto-reduction, label-reduction | ref has fewer gotos (88 vs 96); ref has fewer labels (35 vs 44) | uncovered: control-flow structuring (no kuna option yet); uncovered: control-flow structuring |
| 55 | 5 | `test_reverting_switch_lowering_filename_unescape :: filename_unescape` | switch-recovery | ref recovered a switch kuna did not (2 vs 0) | check loweredswitch / switchmodbound (S2 switch-model) |
| 56 | 4 | `test_decompiling_true_x86_64_1 :: sub_404dc0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x404dc0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004; decompiler said:
[decomp | kuna could not decompile this function at all |
| 57 | 4 | `test_decompiling_true_x86_64_0 :: sub_4048c0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4048c0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004; decompiler said:
[decomp | kuna could not decompile this function at all |
| 58 | 4 | `test_decompiling_true_a_x86_64_1 :: sub_404410` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x404410' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a; decompiler said:
[decomp]> load fi | kuna could not decompile this function at all |
| 59 | 4 | `test_decompilation_excessive_condition_removal :: _compile_bf` | kuna-failed | kuna produced no output (addr-mode: could not build an architecture for /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/bf (unsupported/!recogniz | kuna could not decompile this function at all |
| 60 | 4 | `test_stack_canary_removal_x8664_extra_exits :: main` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 61 | 4 | `test_decompilation_stat_human_fstype :: human_fstype.isra.0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401a70' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stat.o; decompiler said:
[decom | kuna could not decompile this function at all |
| 62 | 4 | `test_decompiling_fmt_get_space :: get_space` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4020f0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt; decompiler said:
[decomp]> | kuna could not decompile this function at all |
| 63 | 4 | `test_decompiling_fmt_paragraph_dowhile :: sub_403c78` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x403c78' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_O0; decompiler said:
[decom | kuna could not decompile this function at all |
| 64 | 4 | `test_decompiling_fmt_main :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401900' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt; decompiler said:
[decomp]> | kuna could not decompile this function at all |
| 65 | 4 | `test_cascading_boolean_and :: foo` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 66 | 4 | `test_decompiling_true_a_x86_64_0 :: sub_401e60` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401e60' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a; decompiler said:
[decomp]> load fi | kuna could not decompile this function at all |
| 67 | 4 | `test_decompiling_split_lines_split :: lines_split` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401290' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o; decompiler said:
[deco | kuna could not decompile this function at all |
| 68 | 4 | `test_decompiling_uname_main :: main` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 69 | 4 | `test_decompiling_dd_iwrite :: iwrite.constprop.0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401820' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o; decompiler said:
[decomp] | kuna could not decompile this function at all |
| 70 | 4 | `test_decompiling_rewrite_negated_cascading_logical_conjunction_expressions :: strtoul_cc_t.constprop.0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4013e0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stty.o; decompiler said:
[decom | kuna could not decompile this function at all |
| 71 | 4 | `test_complex_stack_offset_calculation :: main` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 72 | 4 | `test_df_add_uint_with_neg_flag_ite_expressions :: add_uint_with_neg_flag.constprop.0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400ea0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/df.o; decompiler said:
[decomp] | kuna could not decompile this function at all |
| 73 | 4 | `test_ifelsesimplifier_insert_node_into_while_body :: sub_4030d0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4030d0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/angr_4082_cache; decompiler sai | kuna could not decompile this function at all |
| 74 | 4 | `test_ternary_propagation_1 :: display_speed` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 75 | 4 | `test_automatic_ternary_creation_2 :: head` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4014e0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o; decompiler said:
[decom | kuna could not decompile this function at all |
| 76 | 4 | `test_ifelseflatten_gzip :: treat_file` | recovery-failure | kuna emitted a recovery-failure marker | kuna aborted/failed structuring here |
| 77 | 4 | `test_eager_returns_simplifier_no_duplication_of_default_case :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x404df0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/ls_ubuntu_2004; decompiler said:
[decomp]> | kuna could not decompile this function at all |
| 78 | 4 | `test_dd_iread_ret_dup_region :: iread` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x401640' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o; decompiler said:
[decomp] | kuna could not decompile this function at all |
| 79 | 4 | `test_hostname_bad_mem_read :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x402540' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/hostname; decompiler said:
[dec | kuna could not decompile this function at all |
| 80 | 4 | `test_incorrect_function_argument_unification :: sub_40d450` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x40d450' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/liblzma.so.5.6.1; decompiler said:
[decomp | kuna could not decompile this function at all |
| 81 | 4 | `test_decompiling_rust_binary_rust_probestack :: _ZN15linux_encryptor5files11create_note17hd3c91fc90c0b0684E` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x40b720' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878 | kuna could not decompile this function at all |
| 82 | 4 | `test_decompiling_lighttpd_expression_over_folding :: connection_read_header_more` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x422e80' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/lighttpd; decompiler said:
[dec | kuna could not decompile this function at all |
| 83 | 4 | `test_call_expr_folding_call_order :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_order.o; | kuna could not decompile this function at all |
| 84 | 4 | `test_call_expr_folding_load_order :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_load_order.o; | kuna could not decompile this function at all |
| 85 | 4 | `test_call_expr_folding_store_order :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_store_order.o | kuna could not decompile this function at all |
| 86 | 4 | `test_call_expr_folding_call_loop :: main` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_loop.o;  | kuna could not decompile this function at all |
| 87 | 4 | `test_call_expr_folding_call_before_cond :: f1` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_before_c | kuna could not decompile this function at all |
| 88 | 4 | `test_call_expr_folding_cond_call :: f1` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_cond_call.o;  | kuna could not decompile this function at all |
| 89 | 4 | `test_decompiling_livectf_dc30_open_to_interpretation :: sub_4012f0` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4012f0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/livectf-dc30-open-to-interpreta | kuna could not decompile this function at all |
| 90 | 4 | `test_decompiling_amd64_single_block_jumptable :: _ZN15linux_encryptor5files8walk_dir17h3bbd2a8a81e3b38eE` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x40bcf0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878 | kuna could not decompile this function at all |
| 91 | 4 | `test_decompiling_budgit_cgc_recvline :: cgc_recvline` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x402360' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/BudgIT; decompiler said:
[decom | kuna could not decompile this function at all |
| 92 | 4 | `test_tail_calls :: test_tailcall` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 93 | 4 | `test_tail_calls :: test_noreturn_tailcall` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400014' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 94 | 4 | `test_tail_calls :: test_cond_tailcall_jmp` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400044' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 95 | 4 | `test_tail_calls :: test_cond_noreturn_tailcall_jmp` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400060' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 96 | 4 | `test_tail_calls :: test_cond_tailcall_cjmp` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400078' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 97 | 4 | `test_tail_calls :: test_cond_noreturn_tailcall_cjmp` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x400092' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
 | kuna could not decompile this function at all |
| 98 | 4 | `test_decompiling_rust_fmt_build_best_path_no_ref_using_args :: _ZN6uu_fmt9linebreak15build_best_path17h4613d5d21e8bb56eE` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x4bc130' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust; decompiler said:
[dec | kuna could not decompile this function at all |
| 99 | 4 | `test_decompiling_rust_fmt_main :: _ZN6uu_fmt6uumain6uumain17h4d597dedc19545e7E` | kuna-failed | kuna produced no output (addr-mode: no C output for '0x469200' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust; decompiler said:
[dec | kuna could not decompile this function at all |
| 100 | 3 | `test_sailr_motivating_example :: schedule_job` | goto-reduction | ref has fewer gotos (1 vs 2) | uncovered: control-flow structuring (no kuna option yet) |
| 101 | 1 | `test_decompiling_switch0_x86_64 :: main` | conciseness | ref is 18% shorter (32 vs 39 loc) | uncovered: structuring / expression folding |
| 102 | 1 | `test_decompiling_switch1_x86_64 :: main` | conciseness | ref is 15% shorter (33 vs 39 loc) | uncovered: structuring / expression folding |
| 103 | 1 | `test_decompiling_switch2_x86_64 :: main` | conciseness | ref is 20% shorter (28 vs 35 loc) | uncovered: structuring / expression folding |
| 104 | 1 | `test_decompiling_1after909_verify_password :: verify_password` | conciseness | ref is 21% shorter (23 vs 29 loc) | uncovered: structuring / expression folding |
| 105 | 1 | `test_decompiling_dir_gcc_O0_emit_ancillary_info :: emit_ancillary_info` | conciseness | ref is 25% shorter (44 vs 59 loc) | uncovered: structuring / expression folding |
| 106 | 1 | `test_decompiling_dir_gcc_O0_free_ent :: free_ent` | conciseness | ref is 50% shorter (8 vs 16 loc) | uncovered: structuring / expression folding |
| 107 | 1 | `test_decompiling_strings_local_strlen :: local_strlen` | conciseness | ref is 38% shorter (8 vs 13 loc) | uncovered: structuring / expression folding |
| 108 | 1 | `test_decompilation_call_expr_folding :: strlen_should_not_fold` | conciseness | ref is 21% shorter (15 vs 19 loc) | uncovered: structuring / expression folding |
| 109 | 1 | `test_decompiling_dirname_x2nrealloc :: x2nrealloc` | conciseness | ref is 23% shorter (27 vs 35 loc) | uncovered: structuring / expression folding |
| 110 | 1 | `test_decompiling_tee_O2_tail_jumps :: __argmatch_die` | conciseness | ref is 87% shorter (4 vs 30 loc) | uncovered: structuring / expression folding |
| 111 | 1 | `test_decompiling_tac_base_len :: base_len` | conciseness | ref is 21% shorter (11 vs 14 loc) | uncovered: structuring / expression folding |
| 112 | 1 | `test_decompiling_functions_with_unknown_simprocedures :: cgc_recv_haiku` | conciseness | ref is 46% shorter (28 vs 52 loc) | uncovered: structuring / expression folding |
| 113 | 1 | `test_fauxware_read_packet_call_folding_into_store_stmt :: main` | conciseness | ref is 36% shorter (21 vs 33 loc) | uncovered: structuring / expression folding |
| 114 | 1 | `test_argument_cvars_in_map_pos_to_node :: authenticate` | conciseness | ref is 36% shorter (16 vs 25 loc) | uncovered: structuring / expression folding |
| 115 | 1 | `test_ifelseflatten_iplink_bridge :: bridge_print_opt` | conciseness | ref is 80% shorter (177 vs 881 loc) | uncovered: structuring / expression folding |
| 116 | 1 | `test_sort_zaptemp_if_choices :: zaptemp` | conciseness | ref is 22% shorter (46 vs 59 loc) | uncovered: structuring / expression folding |
| 117 | 1 | `test_decompiling_sp_altering_function :: sub_4011de` | conciseness | ref is 25% shorter (80 vs 107 loc) | uncovered: structuring / expression folding |
| 118 | 1 | `test_decompiling_fprintf_multiple_format_string_args :: sub_402ee0` | conciseness | ref is 16% shorter (32 vs 38 loc) | uncovered: structuring / expression folding |
| 119 | 1 | `test_fastfail_intrinsic :: fastfail_with_code_if_lt_10` | conciseness | ref is 33% shorter (6 vs 9 loc) | uncovered: structuring / expression folding |
| 120 | 1 | `test_decompiling_many_consecutive_regions :: main` | conciseness | ref is 33% shorter (806 vs 1204 loc) | uncovered: structuring / expression folding |
| 121 | 1 | `test_decompiling_48460c9633d06cad3e3b41c87de04177d129906610c5bbdebc7507a211100e98_sub_4025F0 :: xcsdxgffhhASzxsdweFzc` | conciseness | ref is 80% shorter (343 vs 1754 loc) | uncovered: structuring / expression folding |

## kuna produced no output (reference succeeded)

- `test_decompiling_true_x86_64_1 :: sub_404dc0` -- addr-mode: no C output for '0x404dc0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x404dc0
Low-level ERROR: Unable to load 512 bytes at r0x00404dc0
Unable to proceed with function: sub_404dc0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_6fp4p4wb.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_true_x86_64_0 :: sub_4048c0` -- addr-mode: no C output for '0x4048c0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_ubuntu_2004 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x4048c0
Low-level ERROR: Unable to load 512 bytes at r0x004048c0
Unable to proceed with function: sub_4048c0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c__di9mod_.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_true_a_x86_64_1 :: sub_404410` -- addr-mode: no C output for '0x404410' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x404410
Low-level ERROR: Unable to load 512 bytes at r0x00404410
Unable to proceed with function: sub_404410
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_2ofs028y.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompilation_excessive_condition_removal :: _compile_bf` -- addr-mode: could not build an architecture for /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/bf (unsupported/!recognized binary)
- `test_decompilation_stat_human_fstype :: human_fstype.isra.0` -- addr-mode: no C output for '0x401a70' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stat.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stat.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stat.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load addr 0x401a70
Low-level ERROR: Unable to load 512 bytes at r0x00401a70
Unable to proceed with function: sub_401a70
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_klbxjblj.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_fmt_get_space :: get_space` -- addr-mode: no C output for '0x4020f0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: __uflow@@GLIBC_2.2.5
WARNING: Function  overlaps object: free@@GLIBC_2.2.5
WARNING: Function  overlaps object: abort@@GLIBC_2.2.5
WARNING: Function  overlaps object: __errno_location@@GLIBC_2.2.5
WARNING: Function  overlaps object: strncmp@@GLIBC_2.2.5
WARNING: Function  overlaps object: _exit@@GLIBC_2.2.5
WARNING: Function  overlaps object: __fpending@@GLIBC_2.2.5
WARNING: Function  overlaps object: reallocarray@@GLIBC_2.26
WARNING: Function  overlaps object: clearerr_unlocked@@GLIBC_2.2.5
WARNING: Function  overlaps object: textdomain@@GLIBC_2.2.5
WARNING: Function  overlaps object: fclose@@GLIBC_2.2.5
WARNING: Function  overlaps object: bindtextdomain@@GLIBC_2.2.5
WARNING: Function  overlaps object: dcgettext@@GLIBC_2.2.5
WARNING: Function  overlaps object: __ctype_get_mb_cur_max@@GLIBC_2.2.5
WARNING: Function  overlaps object: strlen@@GLIBC_2.2.5
WARNING: Function  overlaps object: __stack_chk_fail@@GLIBC_2.4
WARNING: Function  overlaps object: getopt_long@@GLIBC_2.2.5
WARNING: Function  overlaps object: mbrtowc@@GLIBC_2.2.5
WARNING: Function  overlaps object: strchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: __overflow@@GLIBC_2.2.5
WARNING: Function  overlaps object: strrchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: lseek@@GLIBC_2.2.5
WARNING: Function  overlaps object: __assert_fail@@GLIBC_2.2.5
WARNING: Function  overlaps object: __strtoul_internal@@GLIBC_2.2.5
WARNING: Function  overlaps object: memset@@GLIBC_2.2.5
WARNING: Function  overlaps object: posix_fadvise@@GLIBC_2.2.5
WARNING: Function  overlaps object: __libc_start_main@@GLIBC_2.2.5
WARNING: Function  overlaps object: memcmp@@GLIBC_2.2.5
WARNING: Function  overlaps object: fputs_unlocked@@GLIBC_2.2.5
WARNING: Function  overlaps object:
- `test_decompiling_fmt_paragraph_dowhile :: sub_403c78` -- addr-mode: no C output for '0x403c78' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_O0; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_O0
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_O0 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x403c78
Low-level ERROR: Unable to load 512 bytes at r0x00403c78
Unable to proceed with function: sub_403c78
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_snslp3n_.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_fmt_main :: main` -- addr-mode: no C output for '0x401900' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: __uflow@@GLIBC_2.2.5
WARNING: Function  overlaps object: free@@GLIBC_2.2.5
WARNING: Function  overlaps object: abort@@GLIBC_2.2.5
WARNING: Function  overlaps object: __errno_location@@GLIBC_2.2.5
WARNING: Function  overlaps object: strncmp@@GLIBC_2.2.5
WARNING: Function  overlaps object: _exit@@GLIBC_2.2.5
WARNING: Function  overlaps object: __fpending@@GLIBC_2.2.5
WARNING: Function  overlaps object: reallocarray@@GLIBC_2.26
WARNING: Function  overlaps object: clearerr_unlocked@@GLIBC_2.2.5
WARNING: Function  overlaps object: textdomain@@GLIBC_2.2.5
WARNING: Function  overlaps object: fclose@@GLIBC_2.2.5
WARNING: Function  overlaps object: bindtextdomain@@GLIBC_2.2.5
WARNING: Function  overlaps object: dcgettext@@GLIBC_2.2.5
WARNING: Function  overlaps object: __ctype_get_mb_cur_max@@GLIBC_2.2.5
WARNING: Function  overlaps object: strlen@@GLIBC_2.2.5
WARNING: Function  overlaps object: __stack_chk_fail@@GLIBC_2.4
WARNING: Function  overlaps object: getopt_long@@GLIBC_2.2.5
WARNING: Function  overlaps object: mbrtowc@@GLIBC_2.2.5
WARNING: Function  overlaps object: strchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: __overflow@@GLIBC_2.2.5
WARNING: Function  overlaps object: strrchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: lseek@@GLIBC_2.2.5
WARNING: Function  overlaps object: __assert_fail@@GLIBC_2.2.5
WARNING: Function  overlaps object: __strtoul_internal@@GLIBC_2.2.5
WARNING: Function  overlaps object: memset@@GLIBC_2.2.5
WARNING: Function  overlaps object: posix_fadvise@@GLIBC_2.2.5
WARNING: Function  overlaps object: __libc_start_main@@GLIBC_2.2.5
WARNING: Function  overlaps object: memcmp@@GLIBC_2.2.5
WARNING: Function  overlaps object: fputs_unlocked@@GLIBC_2.2.5
WARNING: Function  overlaps object:
- `test_decompiling_true_a_x86_64_0 :: sub_401e60` -- addr-mode: no C output for '0x401e60' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/true_a successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x401e60
Low-level ERROR: Unable to load 512 bytes at r0x00401e60
Unable to proceed with function: sub_401e60
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_6kddrgnd.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_split_lines_split :: lines_split` -- addr-mode: no C output for '0x401290' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load addr 0x401290
Low-level ERROR: Unable to load 512 bytes at r0x00401290
Unable to proceed with function: sub_401290
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_t6ts342u.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no C output for 'lines_split' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/split.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load function lines_split
Function lines_split: 0x00001290
[decomp]> decompile
Decompiling lines_split
Low-level ERROR: Symbol dat_ffffffffffffffff extends beyond the end of the address space
Unable to proceed with function: lines_split
[decomp]> openfile write /tmp/kuna_c_avsehttj.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit)
- `test_decompiling_dd_iwrite :: iwrite.constprop.0` -- addr-mode: no C output for '0x401820' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load addr 0x401820
Low-level ERROR: Unable to load 512 bytes at r0x00401820
Unable to proceed with function: sub_401820
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_xtyk52hg.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_rewrite_negated_cascading_logical_conjunction_expressions :: strtoul_cc_t.constprop.0` -- addr-mode: no C output for '0x4013e0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stty.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stty.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/stty.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
WARNING: Function  overlaps object: mode_type_flag.cold
[decomp]> load addr 0x4013e0
Low-level ERROR: Unable to load 512 bytes at r0x004013e0
Unable to proceed with function: sub_4013e0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_x0_pns0l.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_df_add_uint_with_neg_flag_ite_expressions :: add_uint_with_neg_flag.constprop.0` -- addr-mode: no C output for '0x400ea0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/df.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/df.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/df.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load addr 0x400ea0
Low-level ERROR: Unable to load 512 bytes at r0x00400ea0
Unable to proceed with function: sub_400ea0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_3nvhs2yo.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_ifelsesimplifier_insert_node_into_while_body :: sub_4030d0` -- addr-mode: no C output for '0x4030d0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/angr_4082_cache; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/angr_4082_cache
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/angr_4082_cache successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x4030d0
Low-level ERROR: Unable to load 512 bytes at r0x004030d0
Unable to proceed with function: sub_4030d0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_bz07g_8v.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_automatic_ternary_creation_2 :: head` -- addr-mode: no C output for '0x4014e0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: emit_stdin_note
[decomp]> load addr 0x4014e0
Low-level ERROR: Unable to load 512 bytes at r0x004014e0
Unable to proceed with function: sub_4014e0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_wglxwfcd.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no C output for 'head' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/head.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: emit_stdin_note
[decomp]> load function head
Function head: 0x000014e0
[decomp]> decompile
Decompiling head
Low-level ERROR: Symbol dat_ffffffffffffffff extends beyond the end of the address space
Unable to proceed with function: head
[decomp]> openfile write /tmp/kuna_c_16gvnkum.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit)
- `test_eager_returns_simplifier_no_duplication_of_default_case :: main` -- addr-mode: no C output for '0x404df0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/ls_ubuntu_2004; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/ls_ubuntu_2004
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/ls_ubuntu_2004 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x404df0
Low-level ERROR: Unable to load 512 bytes at r0x00404df0
Unable to proceed with function: sub_404df0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_wly9xaqe.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'main' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/ls_ubuntu_2004; for a stripped binary pass an address with --addr)
- `test_dd_iread_ret_dup_region :: iread` -- addr-mode: no C output for '0x401640' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load addr 0x401640
Low-level ERROR: Unable to load 512 bytes at r0x00401640
Unable to proceed with function: sub_401640
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_wnr8zwe4.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no C output for 'iread' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/dd.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: to_uchar
[decomp]> load function iread
Function iread: 0x00001640
[decomp]> decompile
Decompiling iread
Low-level ERROR: Symbol dat_ffffffffffffffff extends beyond the end of the address space
Unable to proceed with function: iread
[decomp]> openfile write /tmp/kuna_c_zmp5dpk3.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit)
- `test_hostname_bad_mem_read :: main` -- addr-mode: no C output for '0x402540' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/hostname; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/hostname
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/hostname successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x402540
Low-level ERROR: Unable to load 512 bytes at r0x00402540
Unable to proceed with function: sub_402540
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_3o7byvq6.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'main' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/hostname; for a stripped binary pass an address with --addr)
- `test_incorrect_function_argument_unification :: sub_40d450` -- addr-mode: no C output for '0x40d450' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/liblzma.so.5.6.1; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/liblzma.so.5.6.1
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/liblzma.so.5.6.1 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x40d450
Low-level ERROR: Unable to load 512 bytes at r0x0040d450
Unable to proceed with function: sub_40d450
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_3ml1s05g.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_rust_binary_rust_probestack :: _ZN15linux_encryptor5files11create_note17hd3c91fc90c0b0684E` -- addr-mode: no C output for '0x40b720' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: pthread_getattr_np@@GLIBC_2.2.5
WARNING: Function  overlaps object: mprotect@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_getspecific@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_GetRegionStart@@GCC_3.0
WARNING: Function  overlaps object: _ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hf281f0aecdbc2e94E.llvm.2107281703195253163
WARNING: Function  overlaps object: memset@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_SetGR@@GCC_3.0
WARNING: Function  overlaps object: posix_memalign@@GLIBC_2.2.5
WARNING: Function  overlaps object: close@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_GetDataRelBase@@GCC_3.0
WARNING: Function  overlaps object: ioctl@@GLIBC_2.2.5
WARNING: Function  overlaps object: abort@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_setspecific@@GLIBC_2.2.5
WARNING: Function  overlaps object: memchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_mutexattr_destroy@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN47_$LT$std..fs..File$u20$as$u20$std..io..Read$GT$4read17hedc7a42d7dfc7d2bE
WARNING: Function  overlaps object: read@@GLIBC_2.2.5
WARNING: Function  overlaps object: malloc@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h582b96954f7404b3E
WARNING: Function  overlaps object: __libc_start_main@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_DeleteException@@GCC_3.0
WARNING: Function  overlaps object: sysconf@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$isize$GT$3fmt17h
- `test_decompiling_lighttpd_expression_over_folding :: connection_read_header_more` -- addr-mode: no C output for '0x422e80' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/lighttpd; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/lighttpd
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/lighttpd successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: inet_ntop@GLIBC_2.2.5
WARNING: Function  overlaps object: getenv@GLIBC_2.2.5
WARNING: Function  overlaps object: dlerror@GLIBC_2.34
WARNING: Function  overlaps object: free@GLIBC_2.2.5
WARNING: Function  overlaps object: recv@GLIBC_2.2.5
WARNING: Function  overlaps object: closelog@GLIBC_2.2.5
WARNING: Function  overlaps object: __libc_start_main@GLIBC_2.34
WARNING: Function  overlaps object: abort@GLIBC_2.2.5
WARNING: Function  overlaps object: __errno_location@GLIBC_2.2.5
WARNING: Function  overlaps object: srandom@GLIBC_2.2.5
WARNING: Function  overlaps object: unlink@GLIBC_2.2.5
WARNING: Function  overlaps object: strncmp@GLIBC_2.2.5
WARNING: Function  overlaps object: localtime_r@GLIBC_2.2.5
WARNING: Function  overlaps object: splice@GLIBC_2.5
WARNING: Function  overlaps object: _exit@GLIBC_2.2.5
WARNING: Function  overlaps object: chroot@GLIBC_2.2.5
WARNING: Function  overlaps object: __read_chk@GLIBC_2.4
WARNING: Function  overlaps object: writev@GLIBC_2.2.5
WARNING: Function  overlaps object: puts@GLIBC_2.2.5
WARNING: Function  overlaps object: sigaction@GLIBC_2.2.5
WARNING: Function  overlaps object: setsockopt@GLIBC_2.2.5
WARNING: Function  overlaps object: clock_gettime@GLIBC_2.17
WARNING: Function  overlaps object: setenv@GLIBC_2.2.5
WARNING: Function  overlaps object: write@GLIBC_2.2.5
WARNING: Function  overlaps object: getpid@GLIBC_2.2.5
WARNING: Function  overlaps object: getentropy@GLIBC_2.25
WARNING: Function  overlaps object: lstat64@GLIBC_2.33
WARNING: Function  overlaps object: getpeername@GLIBC_2.2.5
WARNING: Function  overlaps object: getloadavg@GLIBC_2.2.5
WARNING: Function  overlaps object: shutdown@GLIBC_2.2.5
WARNING: Function  overlaps object: globfree64@GLIBC_2.2.5
WARNING: Fun
- `test_call_expr_folding_call_order :: main` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_order.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_order.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_order.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_3_1g0zod.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_call_expr_folding_load_order :: main` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_load_order.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_load_order.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_load_order.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_ge1d0266.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_call_expr_folding_store_order :: main` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_store_order.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_store_order.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_store_order.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_4mnj_q7c.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_call_expr_folding_call_loop :: main` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_loop.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_loop.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_loop.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_eqss5sm2.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_call_expr_folding_call_before_cond :: f1` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_before_cond.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_before_cond.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_call_before_cond.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_nf76s2_j.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_call_expr_folding_cond_call :: f1` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_cond_call.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_cond_call.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/call_expr_folding_cond_call.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_fvcb40zr.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_livectf_dc30_open_to_interpretation :: sub_4012f0` -- addr-mode: no C output for '0x4012f0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/livectf-dc30-open-to-interpretation; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/livectf-dc30-open-to-interpretation
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/livectf-dc30-open-to-interpretation successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x4012f0
Low-level ERROR: Unable to load 512 bytes at r0x004012f0
Unable to proceed with function: sub_4012f0
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_d46o355a.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_decompiling_amd64_single_block_jumptable :: _ZN15linux_encryptor5files8walk_dir17h3bbd2a8a81e3b38eE` -- addr-mode: no C output for '0x40bcf0' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/1cbbf108f44c8f4babde546d26425ca5340dccf878d306b90eb0fbec2f83ab51 successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: pthread_getattr_np@@GLIBC_2.2.5
WARNING: Function  overlaps object: mprotect@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_getspecific@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_GetRegionStart@@GCC_3.0
WARNING: Function  overlaps object: _ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hf281f0aecdbc2e94E.llvm.2107281703195253163
WARNING: Function  overlaps object: memset@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_SetGR@@GCC_3.0
WARNING: Function  overlaps object: posix_memalign@@GLIBC_2.2.5
WARNING: Function  overlaps object: close@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_GetDataRelBase@@GCC_3.0
WARNING: Function  overlaps object: ioctl@@GLIBC_2.2.5
WARNING: Function  overlaps object: abort@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_setspecific@@GLIBC_2.2.5
WARNING: Function  overlaps object: memchr@@GLIBC_2.2.5
WARNING: Function  overlaps object: pthread_mutexattr_destroy@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN47_$LT$std..fs..File$u20$as$u20$std..io..Read$GT$4read17hedc7a42d7dfc7d2bE
WARNING: Function  overlaps object: read@@GLIBC_2.2.5
WARNING: Function  overlaps object: malloc@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h582b96954f7404b3E
WARNING: Function  overlaps object: __libc_start_main@@GLIBC_2.2.5
WARNING: Function  overlaps object: _Unwind_DeleteException@@GCC_3.0
WARNING: Function  overlaps object: sysconf@@GLIBC_2.2.5
WARNING: Function  overlaps object: _ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$isize$GT$3fmt17h
- `test_decompiling_budgit_cgc_recvline :: cgc_recvline` -- addr-mode: no C output for '0x402360' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/BudgIT; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/BudgIT
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/BudgIT successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: __libc_start_main@GLIBC_2.34
WARNING: Function  overlaps object: cgc_fdwait
WARNING: Function  overlaps object: cgc__terminate
WARNING: Function  overlaps object: memset@GLIBC_2.2.5
WARNING: Function  overlaps object: cgc_transmit
WARNING: Function  overlaps object: cgc_random
WARNING: Function  overlaps object: cgc_allocate
WARNING: Function  overlaps object: __cxa_finalize@GLIBC_2.2.5
[decomp]> load addr 0x402360
Low-level ERROR: Unable to load 512 bytes at r0x00402360
Unable to proceed with function: sub_402360
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_0igpqlyf.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit
- `test_tail_calls :: test_tailcall` -- addr-mode: no C output for '0x400000' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400000
Low-level ERROR: Unable to load 512 bytes at r0x00400000
Unable to proceed with function: sub_400000
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_m4_5um6j.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_tailcall' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_tail_calls :: test_noreturn_tailcall` -- addr-mode: no C output for '0x400014' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400014
Low-level ERROR: Unable to load 512 bytes at r0x00400014
Unable to proceed with function: sub_400014
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_s_4gfbi5.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_noreturn_tailcall' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_tail_calls :: test_cond_tailcall_jmp` -- addr-mode: no C output for '0x400044' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400044
Low-level ERROR: Unable to load 512 bytes at r0x00400044
Unable to proceed with function: sub_400044
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_4n157skr.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_cond_tailcall_jmp' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_tail_calls :: test_cond_noreturn_tailcall_jmp` -- addr-mode: no C output for '0x400060' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400060
Low-level ERROR: Unable to load 512 bytes at r0x00400060
Unable to proceed with function: sub_400060
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_dng2pqzh.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_cond_noreturn_tailcall_jmp' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_tail_calls :: test_cond_tailcall_cjmp` -- addr-mode: no C output for '0x400078' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400078
Low-level ERROR: Unable to load 512 bytes at r0x00400078
Unable to proceed with function: sub_400078
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_jdkom8t3.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_cond_tailcall_cjmp' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_tail_calls :: test_cond_noreturn_tailcall_cjmp` -- addr-mode: no C output for '0x400092' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
[decomp]> load addr 0x400092
Low-level ERROR: Unable to load 512 bytes at r0x00400092
Unable to proceed with function: sub_400092
[decomp]> decompile
Execution error: No function selected
[decomp]> openfile write /tmp/kuna_c_85uy8vhv.c
[decomp]> print C
Execution error: No function selected
[decomp]> closefile
[decomp]> quit (name-mode: no function 'test_cond_noreturn_tailcall_cjmp' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tail_calls.o; for a stripped binary pass an address with --addr)
- `test_decompiling_rust_fmt_build_best_path_no_ref_using_args :: _ZN6uu_fmt9linebreak15build_best_path17h4613d5d21e8bb56eE` -- addr-mode: no C output for '0x4bc130' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: _ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8b2039c1e45b47daE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h87218e32e0de75d8E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h60236c8ad89dcee5E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0ef3be04b388d902E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he4217271bc7d275dE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he7b716b3615efef1E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h77005f5ea33d0e40E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f9c2efdae37fb28E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbcf0ac28cb325edbE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h49dc5451ba84d574E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h431808cbd3a5bc62E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7ed05f009902dd5cE
WARNING: Function  overlaps object: __libc_start_m
- `test_decompiling_rust_fmt_main :: _ZN6uu_fmt6uumain6uumain17h4d597dedc19545e7E` -- addr-mode: no C output for '0x469200' in /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust; decompiler said:
[decomp]> load file /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust
/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/fmt_rust successfully loaded: Intel/AMD 64-bit x86
[decomp]> read symbols
WARNING: Function  overlaps object: _ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8b2039c1e45b47daE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h87218e32e0de75d8E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h60236c8ad89dcee5E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h0ef3be04b388d902E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he4217271bc7d275dE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17he7b716b3615efef1E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h77005f5ea33d0e40E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3f9c2efdae37fb28E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbcf0ac28cb325edbE
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h49dc5451ba84d574E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h431808cbd3a5bc62E
WARNING: Function  overlaps object: _ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h7ed05f009902dd5cE
WARNING: Function  overlaps object: __libc_start_m
