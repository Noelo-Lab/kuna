//! P7, regions: dominator-based recovery of the statement tree, including `finally` reconstruction.

pub mod clear_struct_helper;
pub mod dec_helper;
pub mod dom_helper;
pub mod dom_tracer;
pub mod dominator_engine;
pub mod dominator_tree_exception_filter;
pub mod fast_extended_postdominance_helper;
pub mod finally_processor;
pub mod generic_dominator_engine;
pub mod i_graph;
pub mod i_graph_node;
pub mod irreducible_cfg_deobfuscator;
pub mod kuna_fix_inline_marker_finally_copy;
pub mod kuna_fix_irreducible_with_exception_edges;
pub mod kuna_fix_synchronized_break_exit;
pub mod sequence_helper;
pub mod strong_connectivity_helper;
pub mod support_component;
pub mod kuna_fix_protected_back_edge;
