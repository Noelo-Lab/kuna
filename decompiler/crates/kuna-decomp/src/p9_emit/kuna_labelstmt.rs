//! (kuna) Every label in the printed C labels a statement.
//!
//! In C99/C11/C17 a label is part of a labeled statement, so a label directly
//! before a closing brace (`case 2: }`, `default: }`, `label_1234: }`) is a
//! syntax error, "label at end of compound statement"; only C23 accepts it.  The
//! shape arises whenever the labeled code prints nothing: a switch arm that only
//! leaves the switch, or a goto target that is just the jump back to a loop head
//! or the join before a closing brace.
//!
//! The emitter records when a label was the last thing printed
//! ([`EmitBase::dangling_label`](crate::prettyprint::EmitBase)); starting a
//! statement or opening a brace clears the record, a comment does not.  The last
//! arm of a switch that still ends on a label gets `break;`, and every other
//! closing brace reached with a label pending first prints the null statement
//! `;`.  Only the C label forms set the record; Rust prints labels as comments.

use crate::prettyprint::{Emit, MarkupRef, SyntaxHighlight};
use crate::printc::PrintEmit;

impl PrintEmit {
    /// Record that a label (`case N:`, `default:`, `name:`) was just printed.
    pub(crate) fn note_label(&mut self) {
        self.state_mut().dangling_label = true;
    }

    /// Whether the last thing printed was a label with no statement after it.
    pub(crate) fn label_dangling(&self) -> bool {
        self.state().dangling_label
    }

    /// Give a pending label the null statement `;` on its own line.
    pub(crate) fn settle_dangling_label(&mut self) {
        if !std::mem::take(&mut self.state_mut().dangling_label) {
            return;
        }
        self.tag_line();
        let id = self.begin_statement(&MarkupRef::none());
        self.print(";", SyntaxHighlight::NoColor);
        self.end_statement(id);
    }
}

#[cfg(test)]
#[path = "kuna_labelstmt/tests.rs"]
mod tests;
