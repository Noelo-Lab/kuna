//! Port of `org/jetbrains/java/decompiler/main/rels/MethodWrapper.java`.

use std::cell::RefCell;
use std::fmt;

use crate::counter_container::CounterContainer;
use crate::direct_graph::DirectGraph;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::invocation_exprent::InvocationExprent;
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
    /// The empty superclass call `InitializerProcessor.hideEmptySuper` removed
    /// (kuna fix `kt_hidden_empty_super_call`).
    pub hidden_super: Option<InvocationExprent>,
}

arena_object!(pub MethodWrapper => MethodWrapperData, Class);
arena_identity!(MethodWrapper);

thread_local! {
    static CACHED_GRAPHS: RefCell<Vec<DirectGraph>> = const { RefCell::new(Vec::new()) };
}

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
            hidden_super: None,
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
                CACHED_GRAPHS.with(|c| {
                    let mut c = c.borrow_mut();
                    if c.last().is_some_and(|g| !g.is_live()) {
                        c.clear();
                    }
                    c.push(graph);
                });
            }
        }

        self.with(|d| d.graph)
    }

    /// The graphs `get_or_build_graph` cached on some wrapper whose index is
    /// at least `mark`: the only direct graphs that outlive the pass that built them.
    pub(crate) fn cached_graphs_since(mark: u32) -> Vec<DirectGraph> {
        CACHED_GRAPHS.with(|c| {
            c.borrow()
                .iter()
                .rev()
                .take_while(|g| g.is_live() && g.index() >= mark)
                .copied()
                .collect()
        })
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
        let qualified_name = class_struct.qualified_name();
        write!(f, "{{{}: {}}}", qualified_name, method_struct.get_name())
    }
}
