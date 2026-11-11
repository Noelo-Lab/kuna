//! Port of `org/jetbrains/java/decompiler/util/token/TextToken.java`.
//!
//! A token is a plain value: the base fields plus a `kind` holding the data of
//! the concrete subclass. `copy()` is `clone()`.

use crate::class_text_token::ClassTextToken;
use crate::field_text_token::FieldTextToken;
use crate::method_text_token::MethodTextToken;
use crate::text_token_visitor::TextTokenVisitor;
use crate::variable_text_token::VariableTextToken;

#[derive(Clone)]
pub struct TextToken {
    pub start: i32,
    pub length: i32,
    pub declaration: bool,
    pub kind: TextTokenKind,
}

#[derive(Clone)]
pub enum TextTokenKind {
    Class(ClassTextToken),
    Field(FieldTextToken),
    Method(MethodTextToken),
    Variable(VariableTextToken),
    /// A zero-width position kuna records (see `kuna_spans`); never visited.
    KunaMark(u32),
}

impl TextToken {
    pub fn new(start: i32, length: i32, declaration: bool, kind: TextTokenKind) -> TextToken {
        TextToken {
            start,
            length,
            declaration,
            kind,
        }
    }

    pub fn shift(&mut self, amount: i32) {
        self.start += amount;
    }

    pub fn copy(&self) -> TextToken {
        self.clone()
    }

    pub fn visit(&self, visitor: &mut dyn TextTokenVisitor) {
        match &self.kind {
            TextTokenKind::Class(t) => ClassTextToken::visit(t, self, visitor),
            TextTokenKind::Field(t) => FieldTextToken::visit(t, self, visitor),
            TextTokenKind::Method(t) => MethodTextToken::visit(t, self, visitor),
            TextTokenKind::Variable(t) => VariableTextToken::visit(t, self, visitor),
            TextTokenKind::KunaMark(_) => {}
        }
    }

    pub fn get_start(&self) -> i32 {
        self.start
    }

    pub fn get_length(&self) -> i32 {
        self.length
    }

    pub fn is_declaration(&self) -> bool {
        self.declaration
    }
}
