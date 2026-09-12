//! (kuna) `pushimmediateret` — option identity for one-store RET tail
//! transfer recovery (P2 flow classification).
//!
//! A packer can encode a terminal jump as `push <immediate>; ret`. The shared
//! decompile drive asks the bounded console-side raw-p-code provenance scanner
//! to prove the RETURN pops exactly that one in-run constant store, then seeds
//! the existing BRANCH flow override. An adjacent in-run stack store declines,
//! leaving the two-push call-emulation shape to `entryretdispatch`. Explicit
//! flow assertions retain precedence and no target function is synthesized.

use kuna_base::marshal::ElementId;

/// Marshaling element `<pushimmediateret>` (kuna). ElementIds live in the
/// 4000+ range; 4165 was the next free id when this option was added.
pub const ELEM_PUSHIMMEDIATERET: ElementId = ElementId::new("pushimmediateret", 4165);

#[cfg(test)]
mod tests {
    use super::ELEM_PUSHIMMEDIATERET;

    #[test]
    fn element_id_is_stable() {
        assert_eq!(ELEM_PUSHIMMEDIATERET.get_id(), 4165);
    }
}
