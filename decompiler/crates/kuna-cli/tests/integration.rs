// Shared integration-test executable. Process-sensitive tests stay separate.
mod common;

#[path = "ambiguous_selector_cli.rs"]
mod ambiguous_selector_cli;

#[path = "argument_errors_cli.rs"]
mod argument_errors_cli;

#[path = "arm_float_arguments.rs"]
mod arm_float_arguments;

#[path = "arm_float_returns.rs"]
mod arm_float_returns;

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

#[path = "asserted_enum_layout.rs"]
mod asserted_enum_layout;

#[path = "baseline_cli.rs"]
mod baseline_cli;

#[path = "broken_pipe.rs"]
mod broken_pipe;

#[path = "buildstamp_cli.rs"]
mod buildstamp_cli;

#[path = "call_array_extent.rs"]
mod call_array_extent;

#[path = "call_result_high_word_reads.rs"]
mod call_result_high_word_reads;

#[path = "call_result_pair_returns.rs"]
mod call_result_pair_returns;

#[path = "caller_stack_cli.rs"]
mod caller_stack_cli;

#[path = "coff_strings.rs"]
mod coff_strings;

#[path = "compact_switch_cli.rs"]
mod compact_switch_cli;

#[path = "crypto_cli.rs"]
mod crypto_cli;

#[path = "ctypes_per_arch.rs"]
mod ctypes_per_arch;

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

#[path = "recursive_records_cli.rs"]
mod recursive_records_cli;

#[path = "register_pair_params.rs"]
mod register_pair_params;

#[path = "relocation_diagnostics.rs"]
mod relocation_diagnostics;

#[path = "retcallchain_cli.rs"]
mod retcallchain_cli;

#[path = "runtime_hints_cli.rs"]
mod runtime_hints_cli;

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

#[path = "zero_extended_returns.rs"]
mod zero_extended_returns;

#[path = "zext_pair_returns.rs"]
mod zext_pair_returns;
