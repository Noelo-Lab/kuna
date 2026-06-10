# Golden vectors (C++ oracle -> Rust port)

Deterministic low-level semantics vectors emitted by the C++ decompiler's
`golden <kind>` console commands (`decompiler/cpp/kuna_goldengen.cc`).  The
Rust port must reproduce these byte-for-byte (same CSV rows from its own
OpBehavior / FloatFormat / Address implementations).

Generated at C++ tree rev `87016be` against the pinned architecture datatest
`decompiler/datatests/floatprint.xml` (x86:LE:64:default:gcc).

## Regeneration

```bash
make binaries        # rebuild decomp_dbg after any C++ change
~/.virtualenvs/kuna/bin/python tools/rust-port/gen_vectors.py
```

(Equivalently at the console: `load test file decompiler/datatests/floatprint.xml`,
then `golden opbehavior` / `golden float` / `golden addrsort` under
`openfile write <path>` / `closefile`.)

## Files

### opbehavior.csv

One row per `OpBehavior` evaluation cell, in opcode enum order, then size
combination, then input index.

- binary rows: `op,sizein,sizeout,in0,in1,result` over sizein in {1,2,4,8}
  with sizeout in {sizein,1}, inputs the fixed 8-value edge list
  (0, 1, 2, signbit-1, signbit, all-ones, 0xdeadbeef..., 0x12345678...
  masked to size) crossed with itself.
- unary rows: `op,sizein,sizeout,in0,result` over all 16 (sizein,sizeout)
  pairs and the same edge list.
- `result` values: hex output, `ERR` (the evaluation throws -- EvaluationError
  or the base-class "emulation unimplemented" LowlevelError), or `TRAP`
  (the INT64_MIN / -1 cell of INT_SDIV / INT_SREM, which would raise SIGFPE
  in the C++ host's signed division and is never evaluated).
- Special (non-evaluatable) opcodes -- branches, calls, LOAD/STORE,
  MULTIEQUAL, INDIRECT, CAST, SEGMENTOP, CPOOLREF, NEW -- are skipped.
- Row names are the canonical enum names: `ZPULL`/`SPULL` are named directly
  because upstream's `get_opname` table is stale there (index 71 still reads
  "EXTRACT"; index 74 is past the end of the table).
- Requires a loaded architecture: the CPUI_FLOAT_* behaviors look up
  FloatFormat by operand size through the program's Translate.  Sizes
  without a registered format (1 and 2 here) fall back to ERR.

### float.csv

`FloatFormat(4)` and `FloatFormat(8)` (default IEEE 754 layouts; no program
needed) over a fixed 15-encoding list (+-0, +-1, 0.5, pi, 1e-10, 1e10 via
getEncoding; subnormal min/max, min normal, max finite, +-inf, quiet NaN as
raw bit patterns).

- `dec,size,enc,class,decimal`: getClass + the iostream default-precision
  rendering of getHostFloat (printc relies on iostream formatting).
- `<unop>,size,a,res` for neg, abs, sqrt, ceil, floor, round, nan.
- `<binop>,size,a,b,res` for add, sub, mult, div, equal, less, lessequal
  over the full encoding cross product.
- `trunc,size,sizeout,a,res` for sizeout in {1,2,4,8}.
- `int2float,sizein,size,a,res` for sizein in {1,2,4,8} over the opbehavior
  edge-input list.
- `float2float,sizein,sizeout,a,res` for 4->8 and 8->4.

### addrsort.csv

`spaceA,offA,spaceB,offB,less,equal,lessequal` for every ordered pair of
Address(space, offset) over all address spaces of the pinned architecture
(space-index order) x offsets {0, 1, 0x1000, 0xffffffff}.  The space set is
a property of the loaded program -- regeneration must keep the pinned
datatest above.

## Determinism caveats

The vectors are deterministic for a fixed binary on a fixed host (the gate:
two consecutive generator runs diff empty).  Cross-platform reproduction
notes for the Rust port:

- `trunc` of NaN/infinity/out-of-range values goes through a host
  `(int64_t)double` cast in C++ (x86: 0x8000000000000000); Rust's saturating
  `as` casts differ and must special-case these cells.
- NaN-producing arithmetic re-encodes through the host's double pipeline;
  the C++ side canonicalizes NaNs via getNaNEncoding, so payloads are stable,
  but the sign of a NaN *result* follows host semantics.
- The decimal column of `dec` rows is C++ iostream `operator<<(double)` with
  default (6 significant digit) precision, e.g. `3.14159`, `1e+10`, `-inf`,
  `nan`.
