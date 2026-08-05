# cppsig — analysis

## 1. The measured symptom

A full cross-decompiler run on google/leveldb (stripped x86-64 shared library, decbench
`projects/cpp/leveldb.toml`) put kuna at or near the top on every metric **except**
`type_match`, where it was last:

| leveldb O0, `type_match` perfects (n=525) | before |
|---|---|
| ghidra | 44 |
| ida | 35 |
| binja | 11 |
| **kuna** | **9** |
| angr | 8 |

kuna is 1st on `byte_match` and ties for 1st on the union metric at O2, so the deficit was
never structural recovery. It was one construct:

```c
/* kuna */   void WriteBlock(int8 *a0,unsigned long a1,unsigned long a2)
/* ghidra */ void __thiscall leveldb::TableBuilder::WriteBlock
                 (TableBuilder *this,BlockBuilder *param_1,BlockHandle *param_2)
```

## 2. The root cause — a demangler that keeps only the name

`analyzers/demangle/mod.rs` calls `.no_params().no_return_type()` and then
`strip_bracket_groups`, which deletes `(...)` and `<...>`. The full c++filt form was
already available from `demangle_raw` — with **no production caller**. The module header
said so outright: *"Scope (PARTIAL - names + namespaces only) ... It does NOT apply the
demangled signature (parameter / return types) ... a deferred follow-up."*

That the mechanism is the **demangler** and not type inference was measured, not assumed:

- named-pointer first parameter across 527 functions: ghidra 188, ida 150, binja 60,
  angr 0, **kuna 0**.
- where ghidra wins, **37 of 42 are `.dynsym`-exported**; on the 112 non-exported
  `this`-shaped functions ghidra recovers **1**.
- on the C control corpus kuna's type recovery is ordinary, so the gap is C++-specific.

leveldb is a shared library, so `.dynsym` survives `strip` with 1397 defined mangled
`FUNC` symbols. That is the channel every rival types C++ from, and the one kuna ignored.

## 3. The hard part — deciding which functions take `this`

Itanium mangles a **static** member exactly like a non-static one, and like a namespaced
free function. Guessing wrong does not merely miss a type; it shifts every parameter
position by one, which scores **worse than doing nothing** (this is binja's bug — it
applies declared params while omitting `this`).

So the rule was measured before it was written, over 1329 mangled `.dynsym` symbols with
DWARF ground truth (915 of which truly take `this`):

| rule | precision | recall |
|---|---|---|
| **`proven`** — ctor/dtor, cv-qualified, unqualified-global, MSVC encoding | **1.0000** | 0.7093 |
| `inferred` — + class evidence from `_ZTV`/`_ZTI`/ctor/dtor symbols | 0.9278 | 1.0000 |
| naive "every nested name takes `this`" | 0.6942 | 1.0000 |

For reference, **Ghidra 12.1's own `this` decision on the same binary is precision 0.85**.
`proven` ships as the default; `inferred` is available and scores higher on this corpus
(38) at a real precision cost.

## 4. What the whole-binary sweep found that the witness could not

Three defects, all invisible to the witness function and all fixed before shipping:

1. **`outtype: None` handled at the wrong seam.** `ActionDefaultParams` rebuilds the
   *caller*-side callee prototype and hit `expect("outtype null")`, killing every `main`
   that called such a function.
2. **`cpp_demangle` silently drops a parameter** on explicit function-template
   specializations (`_ZNSt4pairI...EOT_OT0_` renders 2 params as 1). Refusing those cut
   measured harm from 8 functions to 2, and incidentally sidesteps the long-standing
   `maxof<int>`/`maxof<double>` name collision.
3. **Overloaded operators** break the depth-tracking parse and are the densest source of
   false positives. Refused.

## 5. Results

`9 -> 35` perfects at the default, **+26 gained, 0 lost**; mean 0.1277 -> 0.1784; 28
functions improved, 0 worse. kuna moves from last to second, tied with ida. The 26 gained
are precisely decbench's own mined margin-1.0 set (17 destructors + 9 accessors/ctors).

Reference parameters render as pointers (`const Slice&` -> `Slice *`), which is what the
ground truth expects now that decbench scores `DW_TAG_reference_type` (decbench #60), so
that headroom is captured rather than left on the table.

**Plain C is completely untouched**: 11 binaries / 1,351 functions, 0 changed.

## 6. The honest cost

The wrongness axis is a **new read-before-write of an argument register**, not raw arity
change (an arity *decrease* is often kuna over-recovering, not the signature being wrong).
On leveldb -O0 stripped, 3,962 functions: `proven` damages **2 (0.05%)**, `inferred` 29.

Both `proven` cases are the SysV **hidden return pointer** — a class returned by value
takes argument slot 0, which the mangling never mentions. Ghidra gets these wrong too, and
worse (it drops `this` entirely). Fixing it needs the recovered parameter count, i.e. a
decompile pass rather than a load-time one; recorded as a follow-up.

## 7. Deliberately out of scope

- **`this->field` rendering.** Measured: *no* decompiler recovers it on stripped binaries
  — all five emit `*(long *)this + 0x1d0`. Not a competitive gap.
- **`__thiscall` model selection.** #265 registered the model, but `PrototypePieces` has
  no model back-pointer and `ArchContext` exposes only `defaultfp`. It is not needed here:
  every ABI in this corpus passes `this` as argument 0. The one that does not — 32-bit
  MSVC, `this` in ECX — is **refused** rather than mis-placed.
- **Devirtualization** and non-exported class-type propagation: nobody solves these
  (ghidra recovers 1 of 112 non-exported `this`-shaped functions).
