# abnormal-switch-case-case3 — analysis

## Opportunity

- angr testcase: `test_decompiling_abnormal_switch_case_case3 :: 0x18003c330`
- Binary: `binaries/tests/x86_64/windows/msvcr120.dll` (PE32+ / x86-64), function `memmove`
- angr 9.2.213

## What angr does better

angr recovers a clean, fully-structured `switch (ptr6)` with 17 cases (0..16),
each case a small tail-copy body returning `addr`:

```c
switch (ptr6) {
case 0:  return addr;
case 1:  *((char *)ptr) = *((char *)p); return addr;
...
case 16: *((int128_t *)ptr) = *((int128_t *)p); return addr;
}
```

kuna **loses the switch entirely** and renders the indirect dispatch as an
indirect *call*:

```c
                    /* WARNING: Treating indirect jump as call */
v7 = (void *)(*(code *)((uint8)*(uint4 *)(a2 * 4 + 0x18003c410) + 0x180000000))(a0,a1);
return v7;
```

(Both decompilations are saved verbatim in `angr-vs-kuna.txt`.)

## The exact construct

`memmove` dispatches the small-count (`count <= 16`) tail through an
**image-base-relative jump table**:

```
18003c3f9:  mov  %r11,%r10
18003c3fc:  lea  -0x3c403(%rip),%r9      # r9 = 0x180000000  (image base, a pure computed constant)
18003c403:  mov  0x3c410(%r9,%r8,4),%eax  # eax = *(uint32*)(0x18003c410 + r8*4)  ; 17-entry RVA table
18003c40b:  add  %r9,%rax                 # rax = 0x180000000 + table[r8]
18003c40e:  jmp  *%rax                     # BRANCHIND  -> the switch
```

The index `r8` (`count`) is bounded by `cmp $0x10,%r8 ; jbe 0x18003c3f9` at the
**function entry** — far from the `jmp`. Crucially the dispatch block at
`0x18003c3fc` is *also* entered from `0x18003c77a` (`jmp 0x18003c3fc`) on the
backward-copy path, where `r8` is bounded by a **different** guard
(`and $0x7,%r8 ; test %r8,%r8 ; jne ...`). So the `BRANCHIND` has a
**multi-predecessor dispatch with no single dominating bound guard**. This is
precisely what angr names the "abnormal switch case."

## The owning stage and why kuna fails

Stage: **S2 flow/switch model** — jump-table recovery
(`decompiler/crates/kuna-decomp/src/s2_lift/jumptable.rs`,
`.../flow.rs`). kuna IS Ghidra's decompiler; the `BRANCHIND` already exists and
the basic jump-table model is invoked, but it **declines** and the fallback in
`flow.rs:2719` (`self.data.warning("Treating indirect jump as call", ...)`)
fires.

Root cause (traced in `jumptable.rs`):

1. `JumpBasicModel::analyze_guards` (`jumptable.rs:1896`) bounds the table by
   walking **backward from the BRANCHIND's parent through single-predecessor
   blocks** to find the dominating `cmp/jbe` guard.
2. Here the dispatch block has **two** predecessors, so the
   single-predecessor walk short-circuits and calls
   `check_unrolled_guard` (`jumptable.rs:1927`).
3. `check_unrolled_guard` (`jumptable.rs:2080`) is an **unimplemented SEAM**:

   ```rust
   fn check_unrolled_guard(&mut self, _fd, _bl, _maxpullback, _usenzmask) {
       // SEAM(structuring): checkCommonCbranch + findMultiequal + liftVerifyUnroll
   }
   ```

4. With no bound found, `find_normalized` leaves `jrange.size > maxtablesize`,
   `recover_model_basic` (`jumptable.rs:2113`) returns `false` (the
   `switchmodbound` modulo path also presupposes a melded guard and does not
   fire), and the indirect jump is truncated to a call.

So even kuna's existing image-base-relative readonly-LOAD table machinery is
never reached — recovery dies at the **guard-bounding** step that the
multi-entry CFG defeats.

## Hypothesis for the kuna change

Closing this requires implementing the missing **multi-predecessor unrolled-guard
analysis** — porting the upstream `checkCommonCbranch` + `findMultiequal` +
`liftVerifyUnroll` machinery into the `check_unrolled_guard` SEAM so the table is
bounded across the two entry guards, after which the existing LOAD-table model
recovers the 17-entry RVA table and the structurer emits the `switch`.

This is **not** modelable as one option-gated Action/Rule (the
`kuna_loweredswitch.rs` template manufactures a table from a *comparison cascade*
and finds nothing here; `kuna_switchmodbound` presupposes the guard walk already
succeeded). It is a genuine S2 jump-table-recovery infrastructure addition
touching the core `JumpBasicModel` guard analysis. → **PROPOSAL** (Hard Rule 7).

## Secondary blocker (loader)

kuna's binary loader is ELF-only; `load file` on this PE/DLL returns
"Could not create architecture". The gap is only reproducible via a raw
**bytechunk** (used here: function bytes `0x18003c330..0x18003c895` loaded at
their real addresses, so the `lea`-computed image base and the in-chunk RVA
table resolve correctly). The standard pipeline before/after demo
(`kuna decompile <pe>`) cannot run on this target; an end-to-end testcase needs
a bespoke bytechunk harness. This is flagged as a human go/no-go item.
