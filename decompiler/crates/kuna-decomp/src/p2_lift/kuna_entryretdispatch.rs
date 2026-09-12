//! (kuna) `entryretdispatch` — option identity for entry-point RET-dispatch
//! recovery (P2 flow classification).
//!
//! Some packers replace a direct call chain with repeated
//! `push <continuation>; push <callee>; ret` links. The console-side flow seed
//! recognizer proves each link from bounded raw p-code before the ordinary flow
//! follower runs. This module owns the catalog/marshal identity for the
//! default-on decision point; the implementation lives at the decompile-drive
//! boundary because it must merge derived sites with caller-supplied flow
//! assertions before the engine follows the function.
//!
//! The gate is deliberately independent from explicit assertions. Turning the
//! option off disables only automatic recognition, while an explicit
//! `flow <address> call` continues to request the existing chain propagation.

use kuna_base::marshal::ElementId;

/// Marshaling element `<entryretdispatch>` (kuna). ElementIds live in the
/// 4000+ range; 4164 was the next free id when this option was added.
pub const ELEM_ENTRYRETDISPATCH: ElementId = ElementId::new("entryretdispatch", 4164);

#[cfg(test)]
mod tests {
    use super::ELEM_ENTRYRETDISPATCH;

    #[test]
    fn element_id_is_stable() {
        assert_eq!(ELEM_ENTRYRETDISPATCH.get_id(), 4164);
    }
}
