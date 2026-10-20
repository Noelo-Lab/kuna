// Shared integration-test executable. Process-sensitive tests stay separate.
mod common;

#[path = "aif_lea_padding.rs"]
mod aif_lea_padding;

#[path = "ambiguous_selector_cli.rs"]
mod ambiguous_selector_cli;

#[path = "argument_errors_cli.rs"]
mod argument_errors_cli;

#[path = "conditional_register_lifetimes.rs"]
mod conditional_register_lifetimes;

#[path = "conditional_stack_bytes.rs"]
mod conditional_stack_bytes;
#[path = "arm_conditional_switch.rs"]
mod arm_conditional_switch;
#[path = "arm_constant_shift.rs"]
mod arm_constant_shift;

#[path = "arm_float_arguments.rs"]
mod arm_float_arguments;

#[path = "arm_float_returns.rs"]
mod arm_float_returns;

#[path = "narrow_args.rs"]
mod narrow_args;

#[path = "narrow_ret.rs"]
mod narrow_ret;

#[path = "arm_frame_retain.rs"]
mod arm_frame_retain;
#[path = "arm_frame_pool.rs"]
mod arm_frame_pool;
#[path = "arm_frame_switch.rs"]
mod arm_frame_switch;

#[path = "arm_interwork_cli.rs"]
mod arm_interwork_cli;

#[path = "arm_isa_cli.rs"]
mod arm_isa_cli;

#[path = "arm_variadic_returns.rs"]
mod arm_variadic_returns;

#[path = "arm_veneer_tailcall.rs"]
mod arm_veneer_tailcall;

#[path = "arm_wrapper_returns.rs"]
mod arm_wrapper_returns;

#[path = "arm_xref_roots.rs"]
mod arm_xref_roots;

#[path = "thumbswitch_guards.rs"]
mod thumbswitch_guards;

#[path = "asserted_enum_layout.rs"]
mod asserted_enum_layout;

#[path = "baseline_cli.rs"]
mod baseline_cli;

#[path = "broken_pipe.rs"]
mod broken_pipe;

#[path = "forloop_visible_read.rs"]
mod forloop_visible_read;

#[path = "buildstamp_cli.rs"]
mod buildstamp_cli;

#[path = "call_array_extent.rs"]
mod call_array_extent;

#[path = "call_wide.rs"]
mod call_wide;

#[path = "call_result_high_word_reads.rs"]
mod call_result_high_word_reads;

#[path = "call_result_pair_returns.rs"]
mod call_result_pair_returns;

#[path = "callee_memory_lifetimes.rs"]
mod callee_memory_lifetimes;

#[path = "caller_stack_cli.rs"]
mod caller_stack_cli;

#[path = "castnarrow_round_trip.rs"]
mod castnarrow_round_trip;
#[path = "strlenscan_round_trip.rs"]
mod strlenscan_round_trip;

#[path = "coff_strings.rs"]
mod coff_strings;

#[path = "coldentry_source_boundary.rs"]
mod coldentry_source_boundary;

#[path = "compact_switch_cli.rs"]
mod compact_switch_cli;

#[path = "contiguous_byte_frame.rs"]
mod contiguous_byte_frame;

#[path = "cookie_pointer_return_271.rs"]
mod cookie_pointer_return_271;

#[path = "crypto_cli.rs"]
mod crypto_cli;

#[path = "ctypes_per_arch.rs"]
mod ctypes_per_arch;

#[path = "decompile_budget_cli.rs"]
mod decompile_budget_cli;

#[path = "decompile_cli.rs"]
mod decompile_cli;

#[path = "decompile_graph_cli.rs"]
mod decompile_graph_cli;

#[path = "decompile_json_cli.rs"]
mod decompile_json_cli;

#[path = "decompile_project_cli.rs"]
mod decompile_project_cli;

#[path = "direct_call_return.rs"]
mod direct_call_return;

#[path = "disassemble_cli.rs"]
mod disassemble_cli;

#[path = "divopt_modulo_cli.rs"]
mod divopt_modulo_cli;

#[path = "elf_definition_selectors.rs"]
mod elf_definition_selectors;

#[path = "elf_symbol_aliases.rs"]
mod elf_symbol_aliases;

#[path = "elfv1_entries.rs"]
mod elfv1_entries;

#[path = "elfv1_imports.rs"]
mod elfv1_imports;

#[path = "emitted_bitcasts.rs"]
mod emitted_bitcasts;

#[path = "emitted_label_statements.rs"]
mod emitted_label_statements;

#[path = "emitted_object_types.rs"]
mod emitted_object_types;

#[path = "emitted_pointer_arguments.rs"]
mod emitted_pointer_arguments;

#[path = "entry_ret_dispatch_cli.rs"]
mod entry_ret_dispatch_cli;

#[path = "entry_selectors.rs"]
mod entry_selectors;

#[path = "explicit_branch_assertion_cli.rs"]
mod explicit_branch_assertion_cli;

#[path = "fid_cli.rs"]
mod fid_cli;

#[path = "float_unordered_compare.rs"]
mod float_unordered_compare;

#[path = "float_precision.rs"]
mod float_precision;

#[path = "function_boundary_return_cli.rs"]
mod function_boundary_return_cli;

#[path = "global_load_guard.rs"]
mod global_load_guard;

#[path = "global_pointee.rs"]
mod global_pointee;

#[path = "globalorder_cli.rs"]
mod globalorder_cli;

#[path = "indexed_stack_stores.rs"]
mod indexed_stack_stores;

#[path = "install_skill_cli.rs"]
mod install_skill_cli;

#[path = "limits_cli.rs"]
mod limits_cli;

#[path = "local_name_batch.rs"]
mod local_name_batch;

#[path = "local_stack_views.rs"]
mod local_stack_views;

#[path = "library_names.rs"]
mod library_names;

#[path = "rettriangle_round_trip.rs"]
mod rettriangle_round_trip;

#[path = "stable_local_names.rs"]
mod stable_local_names;

#[path = "loop_cursor_initializer.rs"]
mod loop_cursor_initializer;
#[path = "mapped_array_c.rs"]
mod mapped_array_c;

#[path = "mapped_decode_failure.rs"]
mod mapped_decode_failure;

#[path = "mapped_flow_boundary.rs"]
mod mapped_flow_boundary;

#[path = "mixed_tail_returns.rs"]
mod mixed_tail_returns;

#[path = "name_miss_cli.rs"]
mod name_miss_cli;

#[path = "narrow_extension.rs"]
mod narrow_extension;

#[path = "narrow_switch_labels.rs"]
mod narrow_switch_labels;

#[path = "narrow_zext_returns.rs"]
mod narrow_zext_returns;

#[path = "noreturn_wrapper.rs"]
mod noreturn_wrapper;

#[path = "option_name_cli.rs"]
mod option_name_cli;

#[path = "overlapbranch_cli.rs"]
mod overlapbranch_cli;

#[path = "own_register_returns.rs"]
mod own_register_returns;

#[path = "pair_return_halves.rs"]
mod pair_return_halves;

#[path = "parameter_name_replay.rs"]
mod parameter_name_replay;

#[path = "partial_global_load_guard.rs"]
mod partial_global_load_guard;

#[path = "passthrough_own_return.rs"]
mod passthrough_own_return;

#[path = "pointer_table_strings.rs"]
mod pointer_table_strings;

#[path = "powerpc_isa_selection.rs"]
mod powerpc_isa_selection;

#[path = "push_immediate_ret_cli.rs"]
mod push_immediate_ret_cli;

#[path = "pushed_stack_bytes.rs"]
mod pushed_stack_bytes;

#[path = "recursive_records_cli.rs"]
mod recursive_records_cli;

#[path = "register_lifetime_assertions.rs"]
mod register_lifetime_assertions;

#[path = "register_pair_params.rs"]
mod register_pair_params;

#[path = "register_parameter_replay.rs"]
mod register_parameter_replay;

#[path = "relocation_diagnostics.rs"]
mod relocation_diagnostics;

#[path = "retcallchain_cli.rs"]
mod retcallchain_cli;

#[path = "runtime_hints_cli.rs"]
mod runtime_hints_cli;

#[path = "saved_xmm_msabi.rs"]
mod saved_xmm_msabi;

#[path = "signed_stack_index.rs"]
mod signed_stack_index;

#[path = "soft_float_typed_calls.rs"]
mod soft_float_typed_calls;

#[path = "sourcelang_macho_cli.rs"]
mod sourcelang_macho_cli;

#[path = "spilled_stack_stores.rs"]
mod spilled_stack_stores;

#[path = "split_struct_params.rs"]
mod split_struct_params;

#[path = "sse_minss_cli.rs"]
mod sse_minss_cli;

#[path = "stack_alias_preservation.rs"]
mod stack_alias_preservation;

#[path = "stack_float_snapshot.rs"]
mod stack_float_snapshot;

#[path = "register_locals_json.rs"]
mod register_locals_json;

#[path = "stack_object_discovery.rs"]
mod stack_object_discovery;

#[path = "stack_object_lifetimes.rs"]
mod stack_object_lifetimes;

#[path = "stack_parameter_lifetimes.rs"]
mod stack_parameter_lifetimes;

#[path = "stack_home_aliases.rs"]
mod stack_home_aliases;

#[path = "stack_struct.rs"]
mod stack_struct;

#[path = "store_alias.rs"]
mod store_alias;

#[path = "store_copy_effects.rs"]
mod store_copy_effects;

#[path = "stored_return_values.rs"]
mod stored_return_values;

#[path = "strings_cli.rs"]
mod strings_cli;

#[path = "subcommand_help.rs"]
mod subcommand_help;

#[path = "switch_default_pointer.rs"]
mod switch_default_pointer;

#[path = "switch_return_paths.rs"]
mod switch_return_paths;

#[path = "syscall_memory_cli.rs"]
mod syscall_memory_cli;

#[path = "syscall_regs_cli.rs"]
mod syscall_regs_cli;

#[path = "sysreg_return_halves.rs"]
mod sysreg_return_halves;

#[path = "tail_call_returns.rs"]
mod tail_call_returns;

#[path = "target_selection_cli.rs"]
mod target_selection_cli;

#[path = "thunk_entries_cli.rs"]
mod thunk_entries_cli;

#[path = "triage_cli.rs"]
mod triage_cli;

#[path = "typed_macho_tail_cli.rs"]
mod typed_macho_tail_cli;

#[path = "typed_vtable_cli.rs"]
mod typed_vtable_cli;

#[path = "unpack_cli.rs"]
mod unpack_cli;

#[path = "unstructured_goto_cli.rs"]
mod unstructured_goto_cli;

#[path = "variadic_return_register.rs"]
mod variadic_return_register;

#[path = "variadic_secondary_reader.rs"]
mod variadic_secondary_reader;

#[path = "variadic_xmm_whole.rs"]
mod variadic_xmm_whole;

#[path = "volatile_loads_cli.rs"]
mod volatile_loads_cli;

#[path = "volatile_qualifiers_cli.rs"]
mod volatile_qualifiers_cli;

#[path = "wide_element_strings.rs"]
mod wide_element_strings;

#[path = "wide_string32_literals.rs"]
mod wide_string32_literals;

#[path = "wrapped_stack_aggregate.rs"]
mod wrapped_stack_aggregate;

#[path = "x64_syscall_memory.rs"]
mod x64_syscall_memory;

#[path = "xref_body_ownership.rs"]
mod xref_body_ownership;

#[path = "xrefs_cli.rs"]
mod xrefs_cli;

#[path = "xrefs_own_export_plt.rs"]
mod xrefs_own_export_plt;

#[path = "xrefs_switch_case_calls.rs"]
mod xrefs_switch_case_calls;

#[path = "zero_extended_returns.rs"]
mod zero_extended_returns;

#[path = "zext_pair_returns.rs"]
mod zext_pair_returns;

#[path = "common_allocation_cli.rs"]
mod common_allocation_cli;

#[path = "call_outparam_snapshot.rs"]
mod call_outparam_snapshot;

#[path = "global_load_call_order.rs"]
mod global_load_call_order;
#[path = "integer_float_rounding.rs"]
mod integer_float_rounding;
#[path = "guarded_local_read.rs"]
mod guarded_local_read;

#[path = "indirect_value_arguments.rs"]
mod indirect_value_arguments;

#[path = "global_aggregate_store.rs"]
mod global_aggregate_store;

#[path = "multireg_call_inputs.rs"]
mod multireg_call_inputs;

#[path = "narrow_register_inputs.rs"]
mod narrow_register_inputs;
#[path = "float_field_slices.rs"]
mod float_field_slices;

#[path = "float_storage_truncation.rs"]
mod float_storage_truncation;

#[path = "dwarf_reference_arguments.rs"]
mod dwarf_reference_arguments;

#[path = "fortran_fatal_diagnostics.rs"]
mod fortran_fatal_diagnostics;

#[path = "dwarf_array_bounds.rs"]
mod dwarf_array_bounds;

#[path = "wide_right_shift.rs"]
mod wide_right_shift;

#[path = "float_subword_bits.rs"]
mod float_subword_bits;

#[path = "x87_custom_storage.rs"]
mod x87_custom_storage;

#[path = "dwarf_complex_values.rs"]
mod dwarf_complex_values;
#[path = "project_direct_globals.rs"]
mod project_direct_globals;

#[path = "x87_empty_return.rs"]
mod x87_empty_return;

#[path = "windows_stack_argument_alignment.rs"]
mod windows_stack_argument_alignment;

#[path = "imported_data_identifiers.rs"]
mod imported_data_identifiers;
#[path = "dwarf_linkage_calls.rs"]
mod dwarf_linkage_calls;
#[path = "project_named_globals.rs"]
mod project_named_globals;
#[path = "x87_register_payload.rs"]
mod x87_register_payload;

#[path = "x87_free_operands.rs"]
mod x87_free_operands;

#[path = "prototype_call_mechanisms.rs"]
mod prototype_call_mechanisms;
#[path = "qualified_storage_addresses.rs"]
mod qualified_storage_addresses;
#[path = "qualified_import_storage.rs"]
mod qualified_import_storage;

#[path = "readonly_leaf_stack.rs"]
mod readonly_leaf_stack;

#[path = "global_array_bit_transfers.rs"]
mod global_array_bit_transfers;

#[path = "project_assertion_status.rs"]
mod project_assertion_status;

#[path = "x87_integer_conversions_native.rs"]
mod x87_integer_conversions_native;

#[path = "float_pointer_storage.rs"]
mod float_pointer_storage;

#[path = "stack_union_components.rs"]
mod stack_union_components;

#[path = "dwarf_overlapping_locals.rs"]
mod dwarf_overlapping_locals;

#[path = "float_literal_width.rs"]
mod float_literal_width;

#[path = "project_call_declarations.rs"]
mod project_call_declarations;

#[path = "parameter_spill_reuse.rs"]
mod parameter_spill_reuse;

#[path = "declared_callee_pop.rs"]
mod declared_callee_pop;

#[path = "declared_data_range.rs"]
mod declared_data_range;

#[path = "declared_pointer_frame_alias.rs"]
mod declared_pointer_frame_alias;
#[path = "project_function_linkage.rs"]
mod project_function_linkage;

#[path = "aggregate_storage_cast.rs"]
mod aggregate_storage_cast;

#[path = "x87_native_returns.rs"]
mod x87_native_returns;

#[path = "x86_bound_checks.rs"]
mod x86_bound_checks;

#[path = "floating_stack_bytes.rs"]
mod floating_stack_bytes;

#[path = "aggregate_interior_address.rs"]
mod aggregate_interior_address;

#[path = "aggregate_scalar_bitcasts.rs"]
mod aggregate_scalar_bitcasts;

#[path = "float_integer_truncation.rs"]
mod float_integer_truncation;

#[path = "intermediate_float_rounding.rs"]
mod intermediate_float_rounding;
