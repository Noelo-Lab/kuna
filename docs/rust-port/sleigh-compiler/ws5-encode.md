# WS5 — the top-level `.sla` encode

Port of the `.sla` writer orchestration: C++ `SleighBase::encode` +
`SleighBase::encodeSlaSpace` (`decompiler/cpp/sleighbase.cc:197-255`), the
top-level document emit that `sleigh_opt` runs at the tail of
`run_compilation` (`FormatEncode encoder(s,-1); encode(encoder);
encoder.flush();`, slgh_compile.cc:3805-3807).

## What landed

The architect's map was right: every *sub*-encode already existed in
`kuna-sleigh` (built/exercised by the decoder round-trip) — `SymbolTable::encode`,
every per-symbol `encode_header`/`encode`, the pattern / patexpress / semantics
`encode`s, `SourceFileIndexer::encode`, the `FormatEncode` packed+deflate writer.
WS5 only had to add the single top-level orchestrator and wire the compiler-side
byte-buffer assembly.

- **`SleighBase::encode`** + **`SleighBase::encode_sla_space`** landed *in*
  `kuna-sleigh/src/sleighbase.rs`, next to `SleighBase::decode`. They need
  `&self` access to the private `symtab` / `manager` / `templates` / `indexer`
  state and to the private `SlaTrans` `ConstructTpl`-encode seam, so putting them
  in `kuna-sleigh` (which the plan/map explicitly permit, "If WS5 finds it
  cleaner to land `SleighBase::encode` *in* `kuna-sleigh`… record it as a freeze
  interface") is far cleaner than promoting ~8 fields to `pub`. They mirror the
  C++ attribute order exactly (sleigh: version/bigendian/align/uniqbase +
  optional maxdelay/uniqmask/numsections; then indexer; then `<spaces>` with
  `defaultspace` and the internal-space skip list constant/fspec/iop/join; then
  `symtab.encode`. space: name/index/bigendian/delay/size/optional-wordsize/
  physical, with the `<space>`/`<space_other>`/`<space_unique>` element pick).

- **`kuna-slacomp/src/encode.rs`** (the WS5-owned module) is the compiler-side
  wiring: `encode_sleigh(base, encoder)` delegates to `SleighBase::encode`, and
  `encode_to_sla_writer` / `encode_to_sla_bytes` drive it through a
  `FormatEncode` (header + packed stream + deflate) to produce the final `.sla`
  bytes — exactly the C++ `run_compilation` tail. `slgh_compile.rs::run_compilation`
  (WS4, still `todo!()`) will call `encode::encode_to_sla_writer` once it owns a
  populated `SleighBase`; nothing in WS5 is `todo!()`.

### One latent decode bug fixed: `SourceFileIndexer::decode`

The re-encode round-trip (below) immediately exposed a latent bug: C++ (and the
kuna port) `SourceFileIndexer::decode` populates `fileToIndex`/`indexToFile` but
never updates `leastUnusedIndex`, while `encode` iterates `0..leastUnusedIndex`.
Upstream this is harmless because `encode` only ever runs on a
compiler-*populated* indexer (built via `index()`, which increments
`leastUnusedIndex`); the decode→re-encode path is never exercised in C++. kuna's
WS5 self-test *does* re-encode a decoded indexer, so the fix re-establishes the
`leastUnusedIndex = max(index)+1` invariant in `decode`. This changes no
compiler-path behavior (the compiler never decodes before encoding); it only
makes a decoded indexer faithfully re-encodable. Marked `(kuna)` in
`sleighbase.rs`.

## Verification — the decode→re-encode round-trip (no driver needed)

WS1–WS4 are not yet wired, so there is no compiler to produce a `SleighBase`
from a `.slaspec`. The self-contained WS5 gate instead **decodes an existing
C++-built `.sla` with the ported `kuna-sleigh` reader into a `SleighBase`, then
re-encodes it with WS5's `encode`, and asserts byte-identity** — in
`rust/crates/kuna-slacomp/tests/encode_roundtrip.rs`.

The `.sla` file is `sla\x04` + a deflate-compressed packed *element stream*. Two
levels of identity:

1. **Element-stream identity (the WS5 contract).** The *uncompressed* packed
   element stream WS5 re-encodes must equal the one decoded out of the original
   `.sla`. This is exactly what `SleighBase::encode` controls and is the **hard
   assertion** in every round-trip test. It is byte-identical for all specs
   covered: `data-le-64`, `data-be-64`, `toy_builder_{le,be}`, `toy_{le,be}`,
   `mips32be`, `x86`, `6502` (the simplest DATA spec through full x86/MIPS — i.e.
   every per-symbol/pattern/semantics sub-encode, with-blocks, macros, attaches,
   deep constructor tables).

2. **Whole-file byte identity — blocked by LOSS-010, non-fatal.** The compressed
   bytes additionally depend on the deflate backend. kuna's shared
   `FormatEncode`/`CompressBuffer` uses `flate2`'s `miniz_oxide`, which makes
   different (equally valid) encoder choices than C zlib at compressed levels.
   This is the **pre-existing, documented LOSS-010** (`docs/rust-port/losses.md`):
   at level -1 the two emit different deflate streams (e.g. data-le-64: 417 vs
   418 bytes, diverging at byte 8 of the compressed payload — the zlib header
   `78 9c` and the inflated content are identical). LOSS-010's stated restoration
   criterion is explicit: byte-identity against C++-written `.sla` "cannot hold"
   without binding C zlib by FFI, and the correct gate is **content equality
   after decompression** — precisely the element-stream assertion above. The
   round-trip therefore records the whole-file divergence non-fatally (an
   `eprintln!`) rather than failing on it.

### Consequence for the WS7 byte-parity gate

The plan's final WS7 gate (`kuna.slacomp --all` → 148/148 *byte*-identical `.sla`
vs `sleigh_opt`) is, as written, unreachable while the deflate backend is
miniz_oxide — for the same LOSS-010 reason, independent of WS5. The achievable
and equivalent gate is **content equality**: `decompress(Rust .sla) ==
decompress(C++ .sla)` for every spec (which is what WS5 already demonstrates for
the element stream). Whole-file byte-identity would require a C-zlib-FFI deflate
backend in `kuna-base` (superseding LOSS-004/010); that is a kuna-base decision
orthogonal to the compiler port. The end-to-end backstop (rebuild specs with
Rust → re-decode → 675/675) is unaffected: inflate is exact, so a
miniz_oxide-compressed `.sla` decodes identically.

## Freeze interface added

| `kuna-sleigh` item made `pub` | For | Wave |
|---|---|---|
| `SleighBase::encode(&self, &mut dyn Encoder) -> KunaResult<()>` (new) | top-level `.sla` emit; called by `kuna-slacomp::encode` | WS5 |

(`SleighBase::encode_sla_space` is a private helper of `encode`; no fields were
promoted.)
