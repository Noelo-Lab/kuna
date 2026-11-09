//! Port of `org/vineflower/kotlin/pass/ObfuscatedExceptionsPass.java`.

use crate::decompiler_context::DecompilerContext;
use crate::exception_deobfuscator::ExceptionDeobfuscator;
use crate::i_fernflower_logger::Severity;
use crate::pass::Pass;
use crate::pass_context::PassContext;

pub struct ObfuscatedExceptionsPass;

impl Pass for ObfuscatedExceptionsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let graph = ctx.get_graph().expect("NullPointerException: graph");
        let mt = ctx.get_method();

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
