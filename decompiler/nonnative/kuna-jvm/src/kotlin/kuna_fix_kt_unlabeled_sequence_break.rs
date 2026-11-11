//! kuna fix `kt_unlabeled_sequence_break`: an unlabelled `break` whose
//! closure is a sequence is written `break`, as Java writes it.
//!
//! `LabelHelper` leaves a `break` unlabelled when the innermost loop around
//! it ends where the jump goes (a `return@synchronized` out of a `while`
//! that is the last statement of the lambda), even when its closure is the
//! sequence that ends there rather than the loop. Java writes `break`.
//! `KExprProcessor.jmpWrapper` and `KIfStatement` write `return@run` for any
//! `break` whose closure is a `KSequenceStatement`, but a sequence is only
//! written `run labelN@{ .. }` when it is labelled, so an unlabelled one has
//! no `run` to return from and kotlinc rejects the `return`. With the fix
//! such a `break` is written `break`: in Kotlin as in Java it leaves the
//! innermost loop.

use crate::stat_edge::StatEdge;

/// Whether the `break` along `edge` is written `return@run`/`return@labelN`
/// from the sequence it closes.
pub fn returns_from_sequence(edge: StatEdge) -> bool {
    edge.closure().is_some_and(|c| c.is_k_sequence()) && edge.labeled()
}
