# [PROPOSAL] Valid-C rendering of 128-bit register-pair / __int128 returns

Status: **proposal** (entangled across three subsystems — no implementation in this PR).
Origin: the interp-bee/jit-bee IDA-vs-kuna study (`docs/features/funcboundflow/analysis.md`, finding #4).

## Symptom

`eval_result_noun @ 0xa3e0` is `mov %rdi,%rdx; xor %eax,%eax; ret` — a 16-byte
struct/`__int128` return where `RAX=0`, `RDX=a0`, i.e. the value `a0 << 64`:

```c
undefined16 eval_result_noun(uint8 a0) {
  char v1 [16];
  v1[8] = 0;
  v1[0] = a0;
  return v1 << 0x40;   // INVALID C: array shifted; v1[0]=a0 stores 8 bytes into a char
}
```

`array << int` does not compile, and `v1[0] = a0` silently truncates an 8-byte store
to one `char`. ~20 `undefined16`-returning functions in interp-bee (the Bee value
constructors, which return a 16-byte `{tag,payload}` in `RAX:RDX`) render this way.
IDA emits `return 0;` (it drops the second return register — valid but lossy).

## Root cause (entangled across three subsystems)

The raw p-code is clean:

```
u16 = CONCAT88(#0x0, RDI)      // 16-byte value: high=0, low=a0
j:16 = u16 << #0x40:4          // << 64
return j:16
```

1. **Variable merge (P6).** The three overlapping unique-space varnodes
   (`u0x10000008:8` = RDI, `u0x10000010:8` = 0, `u0x10000008:16` = the CONCAT) share
   a base in the unique space and are merged into ONE 16-byte local `v1`, whose two
   8-byte halves are written separately (`v1[0]=…`, `v1[8]=…`).
2. **Type factory (P5).** `v1` is typed `undefined1[16]` because
   `TypeFactory::get_base(16, …)` returns an array of unknown bytes for any size
   `> max_basetype_size` (10) — **exactly Ghidra's behavior** (there is no 16-byte
   integer primitive).
3. **Printer (P9).** Given an array-typed `v1`, the half-writes render as element
   assignments (`v1[0]=a0`, losing the 8-byte size — Ghidra instead uses the sized
   `v1._0_8_ = a0` sub-field syntax kuna does not emit) and the shift renders as
   `v1 << 0x40` (Ghidra emits this too — it is Ghidra-faithful, not a kuna bug).

So kuna == Ghidra on the shift; kuna is a *regression from Ghidra* only on the
`v1[0]` (vs `._0_8_`) sized-write notation. Neither compiles as C; only IDA folds
the whole thing.

## Proposed fixes (pick one; all gated, default-off opt-in)

- **Option A — sized sub-field writes (printer-only, restores Ghidra fidelity).**
  Emit `v1._0_8_ = a0` / `v1._8_8_ = 0` (Ghidra's `PrintC::opInsertOp` notation) for
  an 8-byte write into an array-typed local, instead of `v1[0] = a0`. Bounded to P9;
  fixes the size-losing writes but leaves `v1 << 0x40` (Ghidra-faithful).
- **Option B — scalar-type wide integers used arithmetically (P5, divergent).**
  When the option is on, `get_base(s, TYPE_INT)` for `8 < s ≤ 16` returns a scalar
  `Datatype(s, TYPE_INT)` (a `uint16`), so the shift operand is a scalar and the cast
  pass yields `(uint16)v1 << 0x40`. Must NOT touch `TYPE_UNKNOWN` (16-byte SIMD stays
  `undefined1[16]`). Diverges from Ghidra; blast radius limited to integer arithmetic
  on wide values.
- **Option C — fold `CONCAT88(0,x) << 64` → the __int128 form (P3, hardest).** A
  mid-end rule recovering `(unsigned __int128)a0 << 64`, matching IDA. Largest, but
  the only one that produces the ideal rendering.

## Test strategy

Minimal reproducer exists: `unsigned __int128 f(unsigned long x){ return (unsigned __int128)x << 64; }`
compiles to the exact `mov %rdi,%rdx; xor %eax,%eax; ret` of `eval_result_noun`.
Two-pass stage test: option off = the array-shift; option on = valid C.

## Effort / risk

Entangled (var-merge + type + printer). Option A is the smallest and safest (printer
fidelity, no semantic change); B/C are larger and (B) Ghidra-divergent. Gated
default-off keeps all gates byte-identical until validated.
