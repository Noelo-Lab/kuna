# Analysis — `test_ret_dedupe_fakeret_1::sort_found_occurs`

angr testcase: `test_ret_dedupe_fakeret_1` (angr 9.2.213)
Binary: `binaries/tests/x86_64/decompiler/ptx.o` (ET_REL, x86-64)
Function: `sort_found_occurs` (`.text+0x440`, 37 bytes)

## What the function is

```asm
sort_found_occurs:
  mov  rsi, [rip+number_of_occurs]   ; .bss global, R_X86_64_PC32
  test rsi, rsi
  jne  .L
  ret                                ; early exit
.L:
  mov  rdi, [rip+occurs_table]       ; .bss global
  lea  rcx, [rip+compare_occurs]     ; local fn
  mov  edx, 0x30
  jmp  qsort                         ; TAIL CALL — R_X86_64_PLT32, qsort UNDEFINED extern
```

It is a guarded **tail call**: `if (n) jmp qsort` else `ret`.

## What angr produces (the win the test asserts)

```c
void sort_found_occurs(void) {
    if (number_of_occurs)
        qsort(occurs_table, number_of_occurs, 48, compare_occurs);
    return;
}
```

Assertion: `re.search(r"if\(.+?\).*qsort\(.*\);.*return", text)`.

angr's `ReturnDeduplicator` (`optimization_passes/ret_deduplicator.py`,
`STAGE = DURING_REGION_IDENTIFICATION`) folds `if (c) { …; return x; } return x;`
into `if (c) { … } return x;`. The tail-call `jmp qsort` is structured into
`qsort(); return;`, creating a second `return` that the deduplicator then merges.

## What kuna produces

**On the real `ptx.o` (and a byte-equivalent `.o` I built): kuna emits NO C at all.**

```
[decomp]> load function sort_found_occurs
Execution error: Unable to load 512 bytes at r0x00402000   (resp. r0x00407210 in ptx.o)
[decomp]> decompile
Execution error: No function selected
```

Root cause (reproduced independently with my own `gcc -O2 -c` object — identical
failure): `qsort` is an **undefined external** in an ET_REL `.o` (there is no PLT
thunk in a relocatable object). kuna's ET_REL loader maps undefined externals to
an empty synthetic region, and the S1 decoder **follows the direct tail-call
`jmp qsort` as intraprocedural flow** into that region, then fails to fetch 512
code bytes there. The function never decompiles.

**On a properly *linked* ELF (PLT present), kuna already does the right thing:**

```c
void sub_401170(void) {
  if (dat_404040 == 0) {
    return;
  }
                    /* WARNING: Treating indirect jump as call */
  (*dat_404018)(dat_404038,dat_404040,0x30,compare_occurs);
  return;
}
```

Here the `jmp qsort@plt` lands on a real PLT stub (`jmp *GOT`), kuna's flow
analysis classifies it as a call ("Treating indirect jump as call"), and the
structurer emits the **already-deduplicated** guard-clause form:
`if (!c) return; qsort_equiv(); return;` — semantically identical to angr's
`if (c) { qsort(); } return;`.

## Conclusion: the nominal feature does not reproduce as a kuna defect

1. **There is no return-duplication defect in kuna to fix.** angr's
   `ReturnDeduplicator` targets `if(c){…return x;} return x;`. kuna never emits
   that shape here — on a loadable binary it already produces the deduplicated
   guard-clause form (one guard `return` + one tail `return`, which is the
   *output* of deduplication, not the bug). Porting a return-deduplicator would
   be a no-op on this function.

2. **The actual reason kuna loses to angr on `ptx.o` is a loader/flow gap, not a
   structuring gap:** kuna cannot decode a direct `jmp` to an **undefined external
   symbol** in an ET_REL object (it decodes into the empty external region and
   aborts). Closing it requires either:
   - **(loader)** synthesizing a real call/return thunk for undefined externals
     referenced by direct branches in ET_REL objects (the known ET_REL
     undefined-symbol gap — `relocobjects`/`docs/divergences.md` DIV-7 track), or
   - **(flow)** classifying a direct `jmp <undefined-external>` as a tail-call
     (CALL + RETURN terminator) at decode time so the decoder stops instead of
     falling into the external region.

   Both are **cross-tier** (loader + S1/S2 decode/flow), multi-step, and not
   modelable as a single Action/Rule like `kuna_loweredswitch.rs`. Per Hard
   rule 7 this is **large / proposal-gate**, not a one-option implementation.

Stage ownership (for the proposal): S1 decode-table / S2 flow classification
(the "post-call terminator" family, cf. `gh6882-sparcstructret`) coupled with the
ET_REL loader's undefined-symbol resolution (`kuna-analysis` loader tier).
