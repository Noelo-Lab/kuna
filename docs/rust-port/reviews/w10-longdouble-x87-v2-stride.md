# longdouble array stride (w10-longdouble-x87-v2) — ACCEPT, +1

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-longdouble-x87-v2` @ `1506038`,
base c2245d3/435). Verdict: **ACCEPT**. (Distinct from the dormant-shelf verifier doc
`w10-longdouble-x87.md`, which covered the CALL-arg/x87-reassembly approach @ b8a26af, +0.)

## Root + fix

`PrintC::pushSymbolDetail`'s symbol-mapped array branch (`printc.rs:4347`) strided by the
element's RAW size (`elem.get_size()` = 10 for `float10`) instead of its ALIGNED size. A
`float10` occupies 16 bytes per array element (alignMap[10]→align 8→`primitive_align_size`
rounds 10→16), so `ldarr[1]` lives at byte offset 0x10; with stride 10 the offset-0x10 store
failed `16 % 10 == 0` and fell through to the bare `ldarr` render. Fix = `elem.get_align_size()`,
the faithful transcription of C++ `TypeArray::getSubEntry` (`type.cc:1430-1433`). Float-format
printing is NOT the root — C++ `FloatFormat` has no 80-bit case (`float.cc:36-61` handles sz 4/8
only). The fix is float10-agnostic (any over-aligned element type now strides correctly).

## Effect

435 → 436 on its base; +1 on the integrated 441 tree → **442**. Gained: Long double #2.

## Gate (re-run at integration onto 441)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 442]`; passing-set diff vs base 441: **regressed-set EMPTY** (gained Long
  double #2). printc.rs auto-merged (stride fix disjoint from the string-literal render).
- switch cluster unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining longdouble failures (separate roots, out of scope)

#3-#11: float10 VALUE reassembly from x87/stack register-pieces (`(float10)CONCAT(...)` in
`coreaction_protos.rs` CALL-arg recovery), and a `"%d\n"` string-literal-render/naming root.
