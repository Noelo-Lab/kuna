//! Integration tests for the JVM engine.

#[path = "rt_kernel.rs"]
mod rt_kernel;
#[path = "rt_jchar.rs"]
mod rt_jchar;
#[path = "rt_jnum.rs"]
mod rt_jnum;
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
#[path = "common/mod.rs"]
pub mod common;
#[path = "kuna_corpus.rs"]
mod kuna_corpus;
#[path = "kuna_support.rs"]
mod kuna_support;
#[path = "kuna_input.rs"]
mod kuna_input;
#[path = "kuna_options.rs"]
mod kuna_options;
#[path = "kuna_fixes.rs"]
mod kuna_fixes;
#[path = "kuna_inventory.rs"]
mod kuna_inventory;
#[path = "kuna_spans.rs"]
mod kuna_spans;
#[path = "kuna_deadline.rs"]
mod kuna_deadline;
#[path = "kuna_driver.rs"]
mod kuna_driver;
#[path = "trace_sweep.rs"]
mod trace_sweep;
#[path = "kuna_stackguard.rs"]
mod kuna_stackguard;
#[path = "kuna_driver_determinism.rs"]
mod kuna_driver_determinism;
#[path = "kuna_driver_failures.rs"]
mod kuna_driver_failures;
#[path = "kuna_engine_state.rs"]
mod kuna_engine_state;
#[path = "class_file_parser.rs"]
mod class_file_parser;
#[path = "class_tree.rs"]
mod class_tree;
#[path = "fernflower_driver.rs"]
mod fernflower_driver;
#[path = "fixes_outside_the_corpus.rs"]
mod fixes_outside_the_corpus;
#[path = "fix_helpers.rs"]
mod fix_helpers;
#[path = "kotlin_metadata.rs"]
mod kotlin_metadata;
#[path = "logger.rs"]
mod logger;
#[path = "naming_plugins.rs"]
mod naming_plugins;
#[path = "switch_fixes.rs"]
mod switch_fixes;
