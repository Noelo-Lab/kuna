//! kuna fix `kt_constructor_body_after_first_block` (`kuna_fixes`): a
//! Kotlin constructor is written without a body only when its body writes
//! nothing.
//!
//! `KConstructor.stringify` removes the `super(..)`/`this(..)` call from the
//! first node of the method's graph and, when that node is then empty, writes
//! no `init` block (primary constructor) or no body (secondary constructor).
//! The first node is only the first basic block: when the constructor goes on
//! with an `if` (`if (initParentJob) initParentJob(..)` after `super(active)`),
//! the condition and everything after it are in later nodes, and Vineflower
//! drops them. kuna renders the body and treats it as empty only when the
//! rendering is blank.

use crate::text_buffer::TextBuffer;

/// The constructor's rendered body, or `None` when it writes nothing.
pub fn body(render: impl FnOnce() -> TextBuffer) -> Option<TextBuffer> {
    let body = render();
    let blank = body
        .convert_to_string_and_allow_data_discard()
        .chars()
        .all(char::is_whitespace);
    (!blank).then_some(body)
}
