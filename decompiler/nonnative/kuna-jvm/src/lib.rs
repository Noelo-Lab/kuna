//! kuna's JVM engine: a class-file decompiler that emits Java.
//!
//! This is a **non-native** engine. Nothing here is lifted to p-code or passes
//! through `kuna-decomp`: JVM bytecode is typed, stack-based and carries its own
//! class, exception and generic metadata, so it is decompiled over a Java-level
//! IR. The engine is a Rust port of Vineflower 1.12.0 (Apache-2.0); each file
//! cites the Java class it ports.
//!
//! Source folders follow kuna's phase taxonomy; the real pass order is the
//! schedule in `infra/method_processor.rs`. Module names are flat
//! (`kuna_jvm::if_helper`) through the re-exports below.

pub mod infra;
pub mod kotlin;
pub mod p0_knowledge;
pub mod p1_classfile;
pub mod p2_lift;
pub mod p3_dataflow;
pub mod p4_calls;
pub mod p5_types;
pub mod p6_variables;
pub mod p7_regions;
pub mod p8_structure;
pub mod p9_emit;
pub mod rt;
pub mod substrate;

pub use infra::{
    cancelation_manager, class_pass, class_pass_location, decompile_record, java_pass_location,
    java_pass_registrar, kuna_corpus, kuna_driver, kuna_partition, kuna_spans, kuna_trace, looping_pass_builder, main_pass_builder, method_processor, named_pass,
    pass, pass_context, plugin_context, validation_helper, wrapped_pass,
};
pub use kotlin::{bit_encoding, kt_flags, kt_metadata, kt_proto, metadata_name_resolver};
pub use infra::{
    graph_parser, kuna_plugins, language_chooser, language_spec, plugin, plugin_options,
    plugin_source, statement_writer,
};
pub use kotlin::{
    k_annotation_exprent, k_catch_statement, k_class, k_const_exprent, k_constructor, k_dispatch,
    k_do_statement, k_element, k_exit_exprent, k_field_exprent, k_file, k_function,
    k_function_exprent, k_function_reference, k_hidden_element, k_if_statement,
    k_invocation_exprent, k_multifile_facade, k_new_exprent, k_property, k_sequence_statement,
    k_switch_head_exprent, k_switch_statement, k_type, k_var_exprent, kotlin_import_collector,
    kt_attrs,
};
pub use kotlin::{
    collapse_string_concat_pass, default_args_map, eliminate_dead_vars_pass, java_finally_pass,
    k_contract, k_enum_processor, k_expr_processor, k_merge_pass, k_parameter,
    k_property_accessor, k_type_parameter, k_types, k_utils, kotlin_chooser,
    kotlin_object_processors, kotlin_options, kotlin_plugin, kotlin_writer,
    obfuscated_exceptions_pass, replace_exprents_pass, replace_stats_pass,
    resugar_kotlin_methods_pass, stack_var_initial_pass,
};
pub use p0_knowledge::{
    class_wrapper_node, context_source, context_unit, converter_helper, counter_container,
    decompiler_context, fernflower, i_decompiled_data, i_fernflower_logger,
    i_fernflower_preferences, i_identifier_renamer, i_variable_name_provider,
    i_variable_naming_factory, identifier_converter, identity_renamer_factory, jdk_runtime, kuna_input, kuna_inventory, kuna_options,
    pool_interceptor, struct_context,
};
pub use p1_classfile::{
    bytecode_version, class_wrapper, classes_processor, code_constants, constant_pool, lazy_loader,
    link_constant, method_wrapper, pooled_constant, primitive_constant,
    struct_ann_default_attribute, struct_annotation_attribute,
    struct_annotation_parameter_attribute, struct_bootstrap_methods_attribute, struct_class,
    struct_code_attribute, struct_constant_value_attribute, struct_enclosing_method_attribute,
    struct_exceptions_attribute, struct_field, struct_general_attribute,
    struct_generic_signature_attribute, struct_inner_classes_attribute,
    struct_line_number_table_attribute, struct_local_variable_table_attribute,
    struct_local_variable_type_table_attribute, struct_member, struct_method,
    struct_method_parameters_attribute, struct_module_attribute, struct_nest_host_attribute,
    struct_permitted_subclasses_attribute, struct_record_attribute, struct_record_component,
    struct_source_file_attribute, struct_type_annotation_attribute,
};
pub use p2_lift::{
    basic_block, control_flow_graph, dead_code_helper, exception_deobfuscator, exception_handler,
    exception_range_cfg, exception_table, expr_processor, full_instruction_sequence, instruction,
    instruction_impact, instruction_sequence, jump_instruction, method_properties,
    method_properties_processor, primitive_exprs_list, switch_instruction,
};
pub use p3_dataflow::{
    concatenation_helper, direct_edge, direct_edge_type, direct_graph, direct_node,
    direct_node_type, flatten_statements_helper, p_pand_mm_helper, s_forms_constructor,
    secondary_functions_helper, simple_ssa_reassign, simplify_exprents_helper,
    ssa_constructor_sparse_ex, ssau_constructor_sparse_ex, stack_vars_processor, var_map_holder,
};
pub use p4_calls::{
    class_reference14_processor, classpath_helper, condy_helper, lambda_processor,
    nested_member_access,
};
pub use p5_types::{
    check_types_result, generic_class_descriptor, generic_field_descriptor, generic_main,
    generic_method_descriptor, generic_type, generics_checker, generics_processor,
    intersection_cast_processor, var_type_processor,
};
pub use p6_variables::{
    var_definition_helper, var_names_collector, var_processor, var_version_node, var_version_pair,
    var_versions_graph, var_versions_processor,
};
pub use p7_regions::{
    clear_struct_helper, dec_helper, dom_helper, dom_tracer, dominator_engine,
    dominator_tree_exception_filter, fast_extended_postdominance_helper, finally_processor,
    generic_dominator_engine, i_graph, i_graph_node, irreducible_cfg_deobfuscator, sequence_helper,
    strong_connectivity_helper, support_component,
};
pub use p8_structure::{
    assert_processor, eliminate_loops_helper, enum_processor, exit_helper, if_helper, if_node,
    if_pattern_match_processor, initializer_processor, inline_single_block_helper, label_helper,
    loop_extract_helper, low_break_helper, merge_helper, nested_class_processor, record_helper,
    switch_expression_helper, switch_helper, switch_pattern_match_processor, synchronized_helper,
    try_helper, try_with_resources_processor,
};
pub use p9_emit::{
    bytecode_mapping_tracer, bytecode_source_mapper, class_text_token, class_writer,
    field_text_token, import_collector, method_text_token, text_buffer, text_range, text_token,
    text_token_dump_visitor, text_token_visitor, variable_text_token,
};
pub use substrate::{
    annotation_exprent, array_exprent, array_helper, assert_exprent, assignment_exprent,
    basic_block_statement, byte_cursor, catch_all_statement, catch_statement, code_type,
    const_exprent, data_point, do_statement, dummy_exit_statement, either, exit_exprent, expr_util,
    exprent, fast_fixed_set, fast_fixed_set_factory, fast_sparse_set_factory, field_descriptor,
    field_exprent, function_exprent, general_statement, i_matchable, if_exprent, if_statement,
    int_helper, interpreter_util, invocation_exprent, key, list_stack, long_fast_fixed_set_factory,
    match_engine, match_node, method_descriptor, monitor_exprent, new_class_name_builder,
    new_exprent, packed_map, pair, pattern, pattern_exprent, root_statement,
    s_forms_fast_map_direct, sequence_statement, short_fast_fixed_set_factory, start_end_pair,
    stat_edge, statement, statement_iterator, statements, switch_exprent, switch_head_exprent,
    switch_statement, synchronized_statement, text_util, type_annotation, type_family, typed,
    var_exprent, var_type, vb_style_collection, yield_exprent, zip_reader,
};
