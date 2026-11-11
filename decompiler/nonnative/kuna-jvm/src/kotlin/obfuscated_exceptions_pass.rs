//! Port of `org/vineflower/kotlin/pass/ObfuscatedExceptionsPass.java`.
//!
//! kuna fix `overlapping_ranges` (spec 10.16): ranges that overlap without
//! nesting are split before the multiple-entry check.

use crate::decompiler_context::DecompilerContext;
use crate::exception_deobfuscator::ExceptionDeobfuscator;
use crate::i_fernflower_logger::Severity;
use crate::kuna_fixes::{self, KunaFix};
use crate::p2_lift::kuna_fix_overlapping_ranges::OverlappingRanges;
use crate::pass::Pass;
use crate::pass_context::PassContext;

pub struct ObfuscatedExceptionsPass;

impl Pass for ObfuscatedExceptionsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let graph = ctx.get_graph().expect("NullPointerException: graph");
        let mt = ctx.get_method();
        if kuna_fixes::enabled(KunaFix::OverlappingRanges) {
            OverlappingRanges::split(graph);
        }

        if ExceptionDeobfuscator::has_obfuscated_exceptions(graph) {
            if !ExceptionDeobfuscator::handle_multiple_entry_exception_ranges(graph) {
                DecompilerContext::get_logger().write_message(
                    "Found multiple entry exception ranges which could not be splitted",
                    Severity::Warn,
                );
                graph.add_comment("$VF: Could not handle exception ranges with multiple entries");
                graph.with_mut(|d| d.add_error_comment = true);
            }

            ExceptionDeobfuscator::insert_dummy_exception_handler_blocks(
                graph,
                mt.get_bytecode_version(),
            );
            return true;
        }

        false
    }
}
