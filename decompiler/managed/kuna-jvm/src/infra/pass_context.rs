//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/PassContext.java`.

use crate::control_flow_graph::ControlFlowGraph;
use crate::decompile_record::DecompileRecord;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_processor::VarProcessor;

/// What a pass sees: the method being decompiled and its current statement tree.
pub struct PassContext {
    root: Option<RootStatement>,
    graph: Option<ControlFlowGraph>,
    mt: StructMethod,
    cl: StructClass,
    var_proc: VarProcessor,
    rec: DecompileRecord,
    md: MethodDescriptor,
}

impl PassContext {
    pub fn new(
        root: Option<RootStatement>,
        graph: Option<ControlFlowGraph>,
        mt: StructMethod,
        cl: StructClass,
        var_proc: VarProcessor,
        rec: DecompileRecord,
    ) -> PassContext {
        let md = MethodDescriptor::parse_descriptor_struct(mt, None);
        PassContext {
            root,
            graph,
            mt,
            cl,
            var_proc,
            rec,
            md,
        }
    }

    pub fn get_root(&self) -> Option<RootStatement> {
        self.root
    }

    pub fn set_root(&mut self, root: Option<RootStatement>) {
        self.root = root;
    }

    pub fn get_graph(&self) -> Option<ControlFlowGraph> {
        self.graph
    }

    pub fn get_method(&self) -> StructMethod {
        self.mt
    }

    pub fn get_enclosing_class(&self) -> StructClass {
        self.cl
    }

    pub fn get_var_proc(&self) -> VarProcessor {
        self.var_proc
    }

    pub fn get_rec(&self) -> DecompileRecord {
        self.rec
    }

    pub fn get_method_descriptor(&self) -> MethodDescriptor {
        self.md.clone()
    }
}
