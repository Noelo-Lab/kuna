//! Port of `org/jetbrains/java/decompiler/modules/decompiler/StatEdge.java`.

use crate::statement::{EdgeDirection, Statement};
use crate::validation_helper::ValidationHelper;
use crate::{arena_identity, arena_object};

pub struct StatEdgeData {
    pub type_: i32,
    pub source: Statement,
    pub destination: Statement,
    pub exceptions: Option<Vec<String>>,
    pub closure: Option<Statement>,
    pub labeled: bool,
    pub explicit: bool,
    pub can_inline: bool,
    pub phantom_continue: bool,
}

arena_object!(pub StatEdge => StatEdgeData, Class);
arena_identity!(StatEdge);

impl StatEdge {
    pub const TYPE_REGULAR: i32 = 1;
    pub const TYPE_EXCEPTION: i32 = 2;
    pub const TYPE_BREAK: i32 = 4;
    pub const TYPE_CONTINUE: i32 = 8;
    pub const TYPE_FINALLYEXIT: i32 = 32;

    pub const TYPES: [i32; 5] = [
        StatEdge::TYPE_REGULAR,
        StatEdge::TYPE_EXCEPTION,
        StatEdge::TYPE_BREAK,
        StatEdge::TYPE_CONTINUE,
        StatEdge::TYPE_FINALLYEXIT,
    ];

    /// Java: StatEdge(int, Statement, Statement, Statement)
    pub fn new_closure(
        type_: i32,
        source: Statement,
        destination: Statement,
        closure: Option<Statement>,
    ) -> StatEdge {
        let edge = StatEdge::new(type_, source, destination);
        edge.with_mut(|d| d.closure = closure);
        edge
    }

    /// Java: StatEdge(int, Statement, Statement)
    pub fn new(type_: i32, source: Statement, destination: Statement) -> StatEdge {
        let edge = StatEdge::alloc(StatEdgeData {
            type_,
            source,
            destination,
            exceptions: None,
            closure: None,
            labeled: true,
            explicit: true,
            can_inline: true,
            phantom_continue: false,
        });
        ValidationHelper::not_null(Some(&source));
        ValidationHelper::not_null(Some(&destination));
        edge
    }

    /// Java: StatEdge(Statement, Statement, List<String>)
    pub fn new_exceptions(
        source: Statement,
        destination: Statement,
        exceptions: Option<Vec<String>>,
    ) -> StatEdge {
        let edge = StatEdge::new(StatEdge::TYPE_EXCEPTION, source, destination);
        if let Some(exceptions) = exceptions {
            edge.with_mut(|d| d.exceptions = Some(exceptions));
        }
        edge
    }

    pub fn get_type(self) -> i32 {
        self.with(|d| d.type_)
    }

    pub fn set_type(self, type_: i32) {
        self.with_mut(|d| d.type_ = type_)
    }

    pub fn get_source(self) -> Statement {
        self.with(|d| d.source)
    }

    pub fn set_source(self, source: Statement) {
        ValidationHelper::assert_true(true, "Should not be null");
        self.with_mut(|d| d.source = source)
    }

    pub fn change_source(self, new_source: Statement) {
        ValidationHelper::assert_true(true, "Should not be null");
        let old_source = self.get_source();
        old_source.remove_edge_internal(EdgeDirection::Forward, self);
        new_source.add_edge_internal(EdgeDirection::Forward, self);
        self.with_mut(|d| d.source = new_source);
    }

    pub fn get_destination(self) -> Statement {
        self.with(|d| d.destination)
    }

    pub fn set_destination(self, destination: Statement) {
        ValidationHelper::assert_true(true, "Should not be null");
        self.with_mut(|d| d.destination = destination)
    }

    pub fn change_destination(self, new_destination: Statement) {
        ValidationHelper::assert_true(true, "Should not be null");
        let old_destination = self.get_destination();
        old_destination.remove_edge_internal(EdgeDirection::Backward, self);
        new_destination.add_edge_internal(EdgeDirection::Backward, self);
        self.with_mut(|d| d.destination = new_destination);
    }

    pub fn change_type(self, type_: i32) {
        self.get_source()
            .change_edge_type(EdgeDirection::Forward, self, type_);
    }

    pub fn remove(self) {
        let (source, destination, closure) = self.with(|d| (d.source, d.destination, d.closure));
        source.remove_edge_internal(EdgeDirection::Forward, self);
        destination.remove_edge_internal(EdgeDirection::Backward, self);
        if let Some(closure) = closure {
            closure.get_label_edges().borrow_mut().remove(&self);
        }
    }

    pub fn remove_closure(self) {
        let closure = self.closure();
        ValidationHelper::assert_true(closure.is_some(), "closure shouldn't be null here");
        closure
            .expect("closure")
            .get_label_edges()
            .borrow_mut()
            .remove(&self);
        self.with_mut(|d| {
            d.labeled = false;
            d.closure = None;
        });
    }

    pub fn get_exceptions(self) -> Option<Vec<String>> {
        self.with(|d| d.exceptions.clone())
    }

    pub fn change_closure(self, stat: Option<Statement>) {
        if let Some(closure) = self.closure() {
            closure.get_label_edges().borrow_mut().remove(&self);
        }
        self.with_mut(|d| d.closure = stat);
        if let Some(closure) = stat {
            closure.get_label_edges().borrow_mut().add(self);
        }
    }

    /// Field read of the public Java field `closure`.
    pub fn closure(self) -> Option<Statement> {
        self.with(|d| d.closure)
    }

    /// Field read of the public Java field `labeled`.
    pub fn labeled(self) -> bool {
        self.with(|d| d.labeled)
    }

    /// Field read of the public Java field `explicit`.
    pub fn explicit(self) -> bool {
        self.with(|d| d.explicit)
    }

    /// Field read of the public Java field `canInline`.
    pub fn can_inline(self) -> bool {
        self.with(|d| d.can_inline)
    }

    /// Field read of the public Java field `phantomContinue`.
    pub fn phantom_continue(self) -> bool {
        self.with(|d| d.phantom_continue)
    }
}

impl std::fmt::Display for StatEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (type_, source, destination, closure, exceptions) = self.with(|d| {
            (
                d.type_,
                d.source,
                d.destination,
                d.closure,
                d.exceptions.clone(),
            )
        });
        write!(f, "{type_}: {source} -> {destination}")?;
        if let Some(closure) = closure {
            write!(f, " ({closure})")?;
        }
        if let Some(exceptions) = exceptions {
            write!(f, " Exceptions: [{}]", exceptions.join(", "))?;
        }
        Ok(())
    }
}
