//! Port of `org/jetbrains/java/decompiler/main/rels/MethodProcessor.java`.
//!
//! Only the per-method schedule (`codeToJava`) is ported. The Java class also
//! runs that schedule on a watchdog thread (`run`, `lock`, `getResult`,
//! `isFinished`) to enforce `MAX_PROCESSING_METHOD`; kuna calls
//! [`MethodProcessor::code_to_java`] directly, as Vineflower itself does when
//! the limit is 0 or in unit-test mode.

#![allow(unused_variables)]

use std::cell::Cell;

use crate::control_flow_graph::ControlFlowGraph;
use crate::decompile_record::DecompileRecord;
use crate::method_descriptor::MethodDescriptor;
use crate::plugin_context::LanguageSpec;
use crate::root_statement::RootStatement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::unported;
use crate::var_processor::VarProcessor;

thread_local! {
    static DEBUG_CURRENTLY_DECOMPILING: Cell<Option<RootStatement>> = const { Cell::new(None) };
    static DEBUG_CURRENT_CFG: Cell<Option<ControlFlowGraph>> = const { Cell::new(None) };
    static DEBUG_CURRENT_DECOMPILE_RECORD: Cell<Option<DecompileRecord>> = const { Cell::new(None) };
}

pub struct MethodProcessor;

impl MethodProcessor {
    /// `debugCurrentlyDecompiling.get()`.
    pub fn debug_currently_decompiling() -> Option<RootStatement> {
        DEBUG_CURRENTLY_DECOMPILING.with(Cell::get)
    }

    /// `debugCurrentlyDecompiling.set(root)`.
    pub fn set_debug_currently_decompiling(root: Option<RootStatement>) {
        DEBUG_CURRENTLY_DECOMPILING.with(|c| c.set(root));
    }

    /// `debugCurrentCFG.get()`.
    pub fn debug_current_cfg() -> Option<ControlFlowGraph> {
        DEBUG_CURRENT_CFG.with(Cell::get)
    }

    /// `debugCurrentCFG.set(graph)`.
    pub fn set_debug_current_cfg(graph: Option<ControlFlowGraph>) {
        DEBUG_CURRENT_CFG.with(|c| c.set(graph));
    }

    /// `debugCurrentDecompileRecord.get()`.
    pub fn debug_current_decompile_record() -> Option<DecompileRecord> {
        DEBUG_CURRENT_DECOMPILE_RECORD.with(Cell::get)
    }

    /// `debugCurrentDecompileRecord.set(record)`.
    pub fn set_debug_current_decompile_record(record: Option<DecompileRecord>) {
        DEBUG_CURRENT_DECOMPILE_RECORD.with(|c| c.set(record));
    }

    /// The per-method pass schedule: bytecode to a structured statement tree.
    pub fn code_to_java(
        cl: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        spec: Option<&LanguageSpec>,
    ) -> RootStatement {
        unported!("MethodProcessor.codeToJava")
    }
}
