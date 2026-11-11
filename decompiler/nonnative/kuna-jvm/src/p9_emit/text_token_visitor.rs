//! Port of `org/jetbrains/java/decompiler/main/extern/TextTokenVisitor.java`.
//!
//! The abstract class is a trait: every method forwards to the `next` visitor
//! of the chain by default, and the `*_base` methods are those default bodies,
//! for overrides that call `super`. The class's statics live on
//! `dyn TextTokenVisitor` (`<dyn TextTokenVisitor>::create_visitor()`). The
//! registered factories are kept per thread rather than in the context's
//! properties, which hold only strings in the port.

use std::cell::RefCell;
use std::rc::Rc;

use crate::field_descriptor::FieldDescriptor;
use crate::method_descriptor::MethodDescriptor;
use crate::text_range::TextRange;

pub trait TextTokenVisitor {
    /// The next visitor of the chain (Java's `next` field).
    fn get_next(&mut self) -> Option<&mut dyn TextTokenVisitor>;

    fn start(&mut self, content: &str) {
        self.start_base(content);
    }

    fn visit_class(&mut self, range: TextRange, declaration: bool, name: &str) {
        self.visit_class_base(range, declaration, name);
    }

    fn visit_field(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &FieldDescriptor,
    ) {
        self.visit_field_base(range, declaration, class_name, name, descriptor);
    }

    fn visit_method(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &MethodDescriptor,
    ) {
        self.visit_method_base(range, declaration, class_name, name, descriptor);
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_parameter(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        self.visit_parameter_base(
            range,
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_local(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        self.visit_local_base(
            range,
            declaration,
            class_name,
            method_name,
            method_descriptor,
            index,
            name,
        );
    }

    fn end(&mut self) {
        self.end_base();
    }

    fn start_base(&mut self, content: &str) {
        if let Some(next) = self.get_next() {
            next.start(content);
        }
    }

    fn visit_class_base(&mut self, range: TextRange, declaration: bool, name: &str) {
        if let Some(next) = self.get_next() {
            next.visit_class(range, declaration, name);
        }
    }

    fn visit_field_base(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &FieldDescriptor,
    ) {
        if let Some(next) = self.get_next() {
            next.visit_field(range, declaration, class_name, name, descriptor);
        }
    }

    fn visit_method_base(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &MethodDescriptor,
    ) {
        if let Some(next) = self.get_next() {
            next.visit_method(range, declaration, class_name, name, descriptor);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_parameter_base(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        if let Some(next) = self.get_next() {
            next.visit_parameter(
                range,
                declaration,
                class_name,
                method_name,
                method_descriptor,
                index,
                name,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_local_base(
        &mut self,
        range: TextRange,
        declaration: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) {
        if let Some(next) = self.get_next() {
            next.visit_local(
                range,
                declaration,
                class_name,
                method_name,
                method_descriptor,
                index,
                name,
            );
        }
    }

    fn end_base(&mut self) {
        if let Some(next) = self.get_next() {
            next.end();
        }
    }
}

/// `TextTokenVisitor.Factory`: wraps the next visitor of a chain.
pub trait Factory {
    fn create(&self, next: Box<dyn TextTokenVisitor>) -> Box<dyn TextTokenVisitor>;
}

impl<F: Fn(Box<dyn TextTokenVisitor>) -> Box<dyn TextTokenVisitor>> Factory for F {
    fn create(&self, next: Box<dyn TextTokenVisitor>) -> Box<dyn TextTokenVisitor> {
        self(next)
    }
}

/// `Factory.andThen(after)`: `after` wraps what `first` creates.
pub fn and_then(first: Rc<dyn Factory>, after: Rc<dyn Factory>) -> Rc<dyn Factory> {
    Rc::new(move |next: Box<dyn TextTokenVisitor>| after.create(first.create(next)))
}

/// The visitor that ends every chain (Java's `EMPTY`).
pub struct EmptyTextTokenVisitor;

impl TextTokenVisitor for EmptyTextTokenVisitor {
    fn get_next(&mut self) -> Option<&mut dyn TextTokenVisitor> {
        None
    }
}

thread_local! {
    static FACTORIES: RefCell<Vec<Rc<dyn Factory>>> = const { RefCell::new(Vec::new()) };
}

impl dyn TextTokenVisitor {
    #[allow(non_snake_case)]
    pub fn EMPTY() -> Box<dyn TextTokenVisitor> {
        Box::new(EmptyTextTokenVisitor)
    }

    pub fn add_visitor(factory: Rc<dyn Factory>) {
        FACTORIES.with(|f| f.borrow_mut().push(factory));
    }

    fn chain_factories() -> Rc<dyn Factory> {
        let factories = FACTORIES.with(|f| f.borrow().clone());
        factories
            .into_iter()
            .reduce(and_then)
            .unwrap_or_else(|| Rc::new(|v: Box<dyn TextTokenVisitor>| v))
    }

    pub fn create_visitor() -> Box<dyn TextTokenVisitor> {
        <dyn TextTokenVisitor>::chain_factories().create(<dyn TextTokenVisitor>::EMPTY())
    }

    /// Java: `createVisitor(Factory)`.
    pub fn create_visitor_with(factory: Rc<dyn Factory>) -> Box<dyn TextTokenVisitor> {
        and_then(<dyn TextTokenVisitor>::chain_factories(), factory)
            .create(<dyn TextTokenVisitor>::EMPTY())
    }
}
