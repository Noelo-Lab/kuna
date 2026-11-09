//! P8, structure: refinement of the statement tree into idiomatic Java constructs.

pub mod assert_processor;
pub mod eliminate_loops_helper;
pub mod enum_processor;
pub mod exit_helper;
pub mod if_helper;
pub mod if_node;
pub mod if_pattern_match_processor;
pub mod initializer_processor;
pub mod kuna_fix_type_switch_enum_label;
pub mod kuna_fix_type_switch_label_lookup;
pub mod inline_single_block_helper;
pub mod label_helper;
pub mod loop_extract_helper;
pub mod low_break_helper;
pub mod merge_helper;
pub mod nested_class_processor;
pub mod record_helper;
pub mod switch_expression_helper;
pub mod switch_helper;
pub mod switch_pattern_match_processor;
pub mod synchronized_helper;
pub mod try_helper;
pub mod try_with_resources_processor;
