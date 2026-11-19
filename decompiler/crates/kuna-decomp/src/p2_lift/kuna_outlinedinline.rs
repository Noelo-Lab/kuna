//! **outlinedinline** — inline clang's MachineOutliner fragments into their
//! callers.
//!
//! At `-Oz`, and in most Swift and Apple system code, clang's MachineOutliner
//! moves instruction sequences that repeat across functions into shared
//! `OUTLINED_FUNCTION_<n>` fragments and calls them with `bl` (or ends the
//! caller with a `b`). A fragment is not a function of the source: it has no
//! frame of its own, often reads and writes the caller's registers and stack,
//! and a caller split across several of them reads as a string of opaque calls
//! that hide its data flow. The function symbol of each fragment is flagged
//! inline (what `option inline <name>` does), so flow splices the fragment's
//! body into each caller.
//!
//! Name based: a stripped image keeps its fragments as calls.

use kuna_base::address::Address;

use crate::architecture::Architecture;

/// Is `name` a MachineOutliner fragment (`OUTLINED_FUNCTION_<n>`, with any
/// leading underscores)?
pub fn is_outlined_name(name: &str) -> bool {
    name.trim_start_matches('_')
        .strip_prefix("OUTLINED_FUNCTION_")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// Flag the function symbol at every address in `entries` inline. Returns how
/// many were flagged.
pub fn mark_inline(arch: &mut Architecture, entries: &[Address]) -> usize {
    let mut flagged = 0;
    for entry in entries {
        if let Some((sid, _)) = arch.symboltab.find_function_across_scopes(entry) {
            arch.symboltab.set_function_inline(sid, true);
            flagged += 1;
        }
    }
    flagged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outliner_names() {
        for name in ["_OUTLINED_FUNCTION_0", "OUTLINED_FUNCTION_17", "__OUTLINED_FUNCTION_3"] {
            assert!(is_outlined_name(name), "{name}");
        }
        for name in ["_OUTLINED_FUNCTION_", "_OUTLINED_FUNCTION_x", "outlined_function_1", "_main"] {
            assert!(!is_outlined_name(name), "{name}");
        }
    }
}
