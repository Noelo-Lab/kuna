//! Port of `org/jetbrains/java/decompiler/api/plugin/GraphParser.java`, with
//! `DomHelper implements GraphParser`.

use crate::control_flow_graph::ControlFlowGraph;
use crate::dom_helper::DomHelper;
use crate::root_statement::RootStatement;
use crate::struct_method::StructMethod;

/// Turns a control flow graph into a structured statement.
pub trait GraphParser {
    fn create_statement(&self, graph: ControlFlowGraph, mt: StructMethod) -> RootStatement;
}

impl<F: Fn(ControlFlowGraph, StructMethod) -> RootStatement> GraphParser for F {
    fn create_statement(&self, graph: ControlFlowGraph, mt: StructMethod) -> RootStatement {
        self(graph, mt)
    }
}

impl GraphParser for DomHelper {
    fn create_statement(&self, graph: ControlFlowGraph, mt: StructMethod) -> RootStatement {
        DomHelper::create_statement(self, graph, mt)
    }
}
