# SLEIGH compiler port — COMPLETE (148/148)

**The Rust SLEIGH compiler (`kuna-slacomp`, binary `slacomp`) is functionally complete and
proven end-to-end.** It compiles `.slaspec → .sla` with output that is byte-identical (in the
decompressed element stream) to C++ `sleigh_opt`.

## Results

- **148 / 148** (ALL) vendored `.slaspec` compile to a `.sla` whose decompressed element stream is
  **byte-identical** to C++ `sleigh_opt` (`python -m kuna.slacomp --all`). The full ISA range:
  x86/x86-64, all ARM/AARCH64, MIPS, RISC-V, SPARC, SuperH, Dalvik/JVM, 6502/Z80, AVR, m68k,
  MSP430, PIC/dsPIC, BPF, PA-RISC, TriCore, V850, LoongArch, … — every vendored spec, including `hexagon` (its named-p-code-section grammar landed in WS7).
- **`hexagon`** (the last spec) landed in WS7: the named-p-code-section body form
  (`{ … <<SECTION>> … }` with `crossbuild`, slghparse.y:268/347) the hand parser didn't yet accept.
- **End-to-end backstop (the decisive proof):** rebuilding all 148 specs with the Rust
  compiler and re-running the entire decompiler datatest suite gives **675/675** on the Rust engine,
  and the C++ `decomp_test_dbg` reading the same Rust-built `.sla` is **PARITY OK**. The Rust-built
  specs decode identically to C++-built specs across all 675 assertions, for both engines.

## The one residual on raw byte-identity (LOSS-010)

The `.sla` is a 4-byte magic + a zlib stream. The *decompressed element stream* is byte-identical;
the *compressed bytes* differ because kuna's deflate backend (`flate2`/miniz_oxide) emits a different
(equally valid) DEFLATE encoding than C zlib (e.g. data-le-64: 417 vs 418 compressed bytes). This is
the pre-existing LOSS-010 dependency substitution (zlib → flate2) and is purely cosmetic — both `.sla`
decode to the same content. True whole-file raw byte-identity would require a zlib-bit-compatible
deflate backend (e.g. `zlib-rs`), a small, separable follow-up if ever needed.

## Build with the Rust compiler

`slacomp` matches `sleigh_opt`'s CLI (`slacomp <file.slaspec>` → `<file>.sla`; `-a <dir>` recurses),
so the Makefile's `SLEIGH` override drives it:

    make specs SLEIGH="$(pwd)/rust/target/release/slacomp"     # or: make specs-rust

or the convenience target `make specs-rust`. `python -m kuna.slacomp --all` is the per-spec
content-parity gate.

## Methodology (same as the decompiler port)

Differential, oracle-driven, fan-out + serial-integrate through a rigorous gate. The oracle is C++
`sleigh_opt` (byte-deterministic); the gate is decompressed-`.sla` content-identity + the 675/675
backstop, plus per-stage golden dumps from instrumented /tmp C++ copies (vendored tree byte-untouched).
Waves WS0 (harness+skeleton) → WS1 lexer → WS2 parser → WS3 SleighPcode/MacroBuilder → WS4a pattern
build → WS4b driver/first-compile → WS4c p-code RTL + ConsistencyChecker → WS5 encode. Each `ws*.md`
in this directory records that wave's port + any divergence diagnosis.

## Status: COMPLETE

The Rust SLEIGH compiler is done — 148/148 specs content-identical, 675/675 decompiler backstop on
Rust-built specs (both engines). Combined with the M3 decompiler port, the entire Ghidra C++ decompiler
+ SLEIGH compiler is now ported to Rust.

The one cosmetic residual is **LOSS-010** (raw whole-file `.sla` bytes differ by the deflate backend
only; decompressed content is identical) — a zlib-bit-compatible deflate backend (e.g. `zlib-rs`) would
close it if true byte-identity is ever wanted. Not required for correctness or for dropping the C++ tree.
