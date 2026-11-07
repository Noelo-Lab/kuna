# Rust zero byte-array stores

Preserve the recovered byte-array destination and its alignment. For a constant-zero STORE, require identical value/destination byte width, a positive array length, and one-byte integer elements of alignment one. Emit a contextually typed `[0; N]` only for that bounded case. Keep the existing STORE address, evaluation count and cast-suppression behavior.

Ship `rustarrayzero` off in every preset after the initial default-on timing failure. Compare complete native bodies with the source C at both optimization levels, including unaligned byte arrays, all guard bytes and pointer callback counts. Retain the wider six-function bundle with carry and alias cases. Compare fresh parent/off/on captures across the pinned corpus, then classify every enabled change using recursive AST comparison. Run the four repository checks and the fixed speed plan on the integrated main tree before publication.

Nonzero constants, arbitrary element types, scalar STORE-width recovery and source-wide Rust parity are separate work. `analysis.md` and `record.json` contain the measured outcome; failed and superseded evidence remains archived.
