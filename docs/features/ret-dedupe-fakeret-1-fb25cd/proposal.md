# [PROPOSAL] ET_REL undefined-external tail-call recovery (`etrel_extern_tailcall`)

**Opportunity:** angr `test_ret_dedupe_fakeret_1::sort_found_occurs`
**Binary:** `binaries/tests/x86_64/decompiler/ptx.o` (ET_REL, x86-64)
**Status:** proposal — needs human go/no-go before implementation.

## The problem

`sort_found_occurs` is a guarded **tail call**:

```asm
  mov  rsi, [number_of_occurs]
  test rsi, rsi
  jne  .L
  ret
.L:
  mov  rdi, [occurs_table]
  lea  rcx, [compare_occurs]
  mov  edx, 0x30
  jmp  qsort          ; R_X86_64_PLT32, qsort is an UNDEFINED external
```

angr decompiles it to (and the test asserts) `if (number_of_occurs) qsort(...); return;`.

**kuna emits NO C at all on `ptx.o`:**

```
[decomp]> load function sort_found_occurs
Execution error: Unable to load 512 bytes at r0x00407210
[decomp]> decompile
Execution error: No function selected
```

Reproduced independently with a byte-equivalent object built from
`gcc -O2 -c` (identical `Unable to load 512 bytes` abort). See
[`analysis.md`](./analysis.md) and [`angr-vs-kuna.txt`](./angr-vs-kuna.txt).

## Root cause (cross-tier)

`qsort` is an **undefined external** in a relocatable object — a `.o` has no PLT
thunk. kuna's ET_REL loader maps undefined externals to an **empty synthetic
region**, and the **S1 decoder follows the direct `jmp qsort` as intraprocedural
flow** into that region, then fails to fetch 512 code bytes. The function never
reaches any Action/Rule pass.

On a properly **linked** ELF (real PLT stub present) kuna already does the right
thing — it classifies the `jmp qsort@plt` as a call and emits the
already-deduplicated guard-clause form `if (!c) return; qsort_equiv(); return;`,
which is semantically identical to angr's `if (c) { qsort(); } return;`.

## Why angr's nominal feature is *already covered* (negative result)

angr closes this with `ReturnDeduplicator` (`optimization_passes/ret_deduplicator.py`),
which folds `if (c) { …; return x; } return x;` → `if (c) { … } return x;`.
**kuna never produces that pattern here** — on a loadable binary it already emits
the deduplicated guard-clause form. Porting a return-deduplicator would be a
verified **no-op** on `sort_found_occurs`. So the *structuring* feature the test
name suggests is not the real gap; the real gap is the ET_REL undefined-external
tail-call decode failure above.

## Why this is large (proposal, not a one-shot Action)

The function aborts in **S1 decode**, before any Action/Rule runs — a structuring
Rule (the `kuna_loweredswitch.rs` template) has nothing to operate on. Closing it
is cross-tier and not modelable as a single gated Action:

1. **Loader tier (`kuna-analysis`):** when an undefined external is the target of
   a **direct branch** (call *or* jmp) in an ET_REL object, synthesize a real
   call/return **thunk** (a 1-instruction `RETURN`/external stub with code bytes)
   instead of an empty region, and resolve the symbol name (`qsort`). This is the
   `relocobjects`/DIV-7 ET_REL undefined-symbol track.
2. **Flow tier (S1/S2):** classify a direct `jmp <undefined-external>` as a
   **tail call** (CALL + RETURN terminator) at decode time, so the decoder stops
   instead of falling into the external region. Mirrors the `gh6882-sparcstructret`
   "post-call terminator" family but needs loader knowledge of "target is an
   undefined external".

Either path touches the loader **and** the S1/S2 decode/flow layer (>1 module,
cross-tier), exceeding the one-Action/one-option budget.

## Proposed plan (if approved)

1. Loader: detect undefined-external symbols that are direct-branch targets in
   ET_REL objects; allocate them a minimal code thunk (`RET`/external marker) in a
   mapped region and attach the symbol name. Gate behind `option etrel_extern_tailcall on`.
2. Flow: ensure the direct `jmp <thunk>` is classified as a tail call so the
   structurer emits `extern_fn(args); return;`. Reuse the existing
   "Treating indirect jump as call" path that already works for linked PLT.
3. Symbol naming: surface `qsort` (and `compare_occurs`, `number_of_occurs`,
   `occurs_table`) from the `.o` symbol table so the call renders `qsort(...)`.
4. Stage test `tests/stages/ghangr-ret-dedupe-fakeret-1-fb25cd.xml`: pass 1
   (option off) asserts the load failure / empty body; pass 2 (option on) asserts
   `qsort(` + the `if (...) ... return` structure. (May need a bytechunk that
   models the undefined-external thunk, since the failure is loader-level.)

## Speed / risk

- **Risk:** medium. Touches the ET_REL loader's region layout (undefined-symbol
  handling) and S1 decode flow — both are shared, so the option must be
  default-OFF and byte-identical when off. Synthesizing thunks risks perturbing
  unrelated ET_REL decompilations if not tightly scoped to direct-branch targets.
- **Speed:** expected negligible (a per-undefined-symbol thunk allocation at load;
  no extra per-op work). To be measured in the implementation worker.
- **Scope guard:** keep the loader change minimal and behind the option; do not
  generalize beyond direct-branch undefined-external targets in ET_REL objects.

## Proposed option

`etrel_extern_tailcall` (default-OFF), `change_kind = structure-recovery`,
`source_decompiler = angr`,
`inspiration = "test_ret_dedupe_fakeret_1; ReturnDeduplicator / CFGFast tail-call + extern resolution; sort_found_occurs"`.
