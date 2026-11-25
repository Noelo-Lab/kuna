//! WBCQ experiment (Fission RFC #1093, read for its idea only): the minimal
//! state machine of a step function `result step(ctx *, input)`, learned
//! against its machine code and certified by path closure. Development only;
//! `docs/features/wbcq/README.md` is the guide.
//!
//! - [`machine`]: one step on the machine code, concolic over the input, with
//!   the context bytes every value depends on.
//! - [`learn`]: L#, an observation tree with a basis of pairwise apart states.
//! - [`engine`]: alphabet refinement by path enumeration (z3), the path-closure
//!   certificate, random conformance, counterexamples back to the learner.
//! - [`guard`]: path conditions as transition guards.
//! - [`mutate`]: mutation testing of the certificate.
//!
//! Built on kuna-verify's lifting, op semantics, expressions and z3 driver.

pub mod engine;
pub mod guard;
pub mod learn;
pub mod machine;
pub mod mutate;
