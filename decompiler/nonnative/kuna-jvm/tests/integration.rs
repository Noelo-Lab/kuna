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
