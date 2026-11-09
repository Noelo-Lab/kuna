//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/RootStatement.java`.

use crate::control_flow_graph::ControlFlowGraph;
use crate::dummy_exit_statement::DummyExitStatement;
use crate::rt::JLinkedHashSet;
use crate::start_end_pair::StartEndPair;
use crate::statement::{Statement, StatementKind, StatementType};
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::{java_subclass, unported};

/// `RootStatement.ContentFlags`.
#[derive(Clone, Copy, Default, Debug)]
pub struct ContentFlags {
    pub has_try_catch: bool,
    pub has_loops: bool,
    pub has_switch: bool,
}

pub struct RootStatementData {
    pub dummy_exit: DummyExitStatement,
    pub mt: StructMethod,
    pub comment_lines: Option<JLinkedHashSet<String>>,
    pub add_error_comment: bool,
    pub flags: ContentFlags,
}

java_subclass!(pub RootStatement: Statement, StatementKind::Root(RootStatementData), is_root, as_root, try_as_root);

impl RootStatement {
    pub fn new(head: Statement, dummy_exit: DummyExitStatement, mt: StructMethod) -> RootStatement {
        let this = RootStatement(Statement::new(
            StatementType::Root,
            StatementKind::Root(RootStatementData {
                dummy_exit,
                mt,
                comment_lines: None,
                add_error_comment: false,
                flags: ContentFlags::default(),
            }),
        ));

        this.set_first(head);

        this.get_stats().borrow_mut().add_with_key(head, head.id());
        head.set_parent(Some(this.0));
        this
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("RootStatement.toJava")
    }

    pub fn get_dummy_exit(self) -> DummyExitStatement {
        self.d(|d| d.dummy_exit)
    }

    /// Field read of the public Java field `mt`.
    pub fn mt(self) -> StructMethod {
        self.d(|d| d.mt)
    }

    /// Java: addComment(String)
    pub fn add_comment(self, comment: &str) {
        self.add_comment_error(comment, false);
    }

    /// Java: addComment(String, boolean)
    pub fn add_comment_error(self, comment: &str, error: bool) {
        self.d_mut(|d| {
            d.comment_lines
                .get_or_insert_with(JLinkedHashSet::new)
                .add(comment.to_string());
            d.add_error_comment |= error;
        });
    }

    /// Java: addComments(RootStatement)
    pub fn add_comments(self, root: RootStatement) {
        let (lines, error) = root.d(|d| {
            (
                d.comment_lines.as_ref().map(|c| c.to_vec()),
                d.add_error_comment,
            )
        });
        if let Some(lines) = lines {
            for s in lines {
                self.add_comment(&s);
            }
        }

        self.d_mut(|d| d.add_error_comment |= error);
    }

    /// Java: addComments(ControlFlowGraph)
    pub fn add_comments_graph(self, graph: ControlFlowGraph) {
        let (lines, error) = graph.with(|d| {
            (
                d.comment_lines.as_ref().map(|c| c.to_vec()),
                d.add_error_comment,
            )
        });
        if let Some(lines) = lines {
            for s in lines {
                self.add_comment(&s);
            }
        }

        self.d_mut(|d| d.add_error_comment |= error);
    }

    pub fn build_content_flags(self) {
        self.build_content_flags_stat(self.0);
    }

    fn build_content_flags_stat(self, stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            self.build_content_flags_stat(st);
        }

        if stat.is_catch() || stat.is_catch_all() {
            self.d_mut(|d| d.flags.has_try_catch = true);
        } else if stat.is_do() {
            self.d_mut(|d| d.flags.has_loops = true);
        } else if stat.is_switch() {
            self.d_mut(|d| d.flags.has_switch = true);
        }
    }

    pub fn has_try_catch(self) -> bool {
        self.d(|d| d.flags.has_try_catch)
    }

    pub fn has_loops(self) -> bool {
        self.d(|d| d.flags.has_loops)
    }

    pub fn has_switch(self) -> bool {
        self.d(|d| d.flags.has_switch)
    }

    pub fn get_start_end_range(self) -> StartEndPair {
        let first = self.get_first().expect("first");
        StartEndPair::join(&[
            Some(first.get_start_end_range()),
            Some(self.get_dummy_exit().get_start_end_range()),
        ])
    }
}
