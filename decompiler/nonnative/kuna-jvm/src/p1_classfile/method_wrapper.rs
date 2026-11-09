//! Port of `org/jetbrains/java/decompiler/main/rels/MethodWrapper.java`.

use std::fmt;

use crate::counter_container::CounterContainer;
use crate::direct_graph::DirectGraph;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::rt::{Caught, JHashSet, JLinkedHashSet};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object};

pub struct MethodWrapperData {
    pub root: Option<RootStatement>,
    pub varproc: VarProcessor,
    pub method_struct: StructMethod,
    pub class_struct: StructClass,
    pub counter: CounterContainer,
    pub set_outer_var_names: JHashSet<String>,
    pub graph: Option<DirectGraph>,
    /// One entry per constructor parameter; `None` where the parameter is not synthetic.
    pub synth_parameters: Option<Vec<Option<VarVersionPair>>>,
    pub decompile_error: Option<Caught>,
    pub comment_lines: Option<JLinkedHashSet<String>>,
    pub add_error_comment: bool,
    pub is_compact_record_constructor: bool,
}

arena_object!(pub MethodWrapper => MethodWrapperData, Class);
arena_identity!(MethodWrapper);

impl MethodWrapper {
    pub fn new(
        root: Option<RootStatement>,
        varproc: VarProcessor,
        method_struct: StructMethod,
        class_struct: StructClass,
        counter: CounterContainer,
    ) -> MethodWrapper {
        let wrapper = MethodWrapper::alloc(MethodWrapperData {
            root,
            varproc,
            method_struct,
            class_struct,
            counter,
            set_outer_var_names: JHashSet::new(),
            graph: None,
            synth_parameters: None,
            decompile_error: None,
            comment_lines: None,
            add_error_comment: false,
            is_compact_record_constructor: false,
        });

        if let Some(root) = root {
            let (comment_lines, add_error_comment) = root.d(|d| {
                (
                    d.comment_lines.as_ref().map(|c| c.to_vec()),
                    d.add_error_comment,
                )
            });
            if let Some(comment_lines) = comment_lines {
                for s in comment_lines {
                    wrapper.add_comment(&s);
                }
                wrapper.with_mut(|d| d.add_error_comment |= add_error_comment);
            }
        }

        wrapper
    }

    pub fn get_or_build_graph(self) -> Option<DirectGraph> {
        let (graph, root) = self.with(|d| (d.graph, d.root));
        if graph.is_none() {
            if let Some(root) = root {
                let graph = FlattenStatementsHelper::new().build_direct_graph(root);
                self.with_mut(|d| d.graph = Some(graph));
            }
        }

        self.with(|d| d.graph)
    }

    pub fn add_comment(self, comment: &str) {
        self.with_mut(|d| {
            d.comment_lines
                .get_or_insert_with(JLinkedHashSet::new)
                .add(comment.to_string());
        });
    }

    pub fn desc(self) -> MethodDescriptor {
        MethodDescriptor::parse_descriptor_struct(self.with(|d| d.method_struct), None)
    }
}

impl fmt::Display for MethodWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (class_struct, method_struct) = self.with(|d| (d.class_struct, d.method_struct));
        let qualified_name = class_struct.with(|d| d.qualified_name.clone());
        write!(f, "{{{}: {}}}", qualified_name, method_struct.get_name())
    }
}
