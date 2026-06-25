# Analysis — `test_decompiling_x8664_mv_O2::main`

angr testcase: `test_decompiling_x8664_mv_O2` · binary
`/home/mahaloz/github/angr-dev/binaries/tests/x86_64/mv_-O2` · function `main` @ `0x402b40`
(arch x86_64). angr 9.2.213. Side-by-side: `angr-vs-kuna.txt`.

## Metrics (reference angr | kuna)

```
loc          300 | 278
gotos          9 | 24     <-- +15 extra gotos in kuna
labels         8 | 14     <-- +6 extra labels
switches       1 | 1
cases         13 | 10     <-- kuna recovers 3 fewer switch cases
ifs           23 | 25
loops          3 | 4
```

Pipeline signals: *ref has fewer gotos*, *ref has fewer labels*, *kuna emitted a
recovery-failure marker*.

## What angr does better

`main` is GNU `mv`'s option-parsing entry: a `while (true) { v = getopt_long(...); if
(v == -1) break; switch (v) { ... } }` getopt loop, compiled at `-O2`. angr renders
exactly that shape:

```c
while (true) {
    v30 = getopt_long(a0, a1, "bfint:uvS:TZ", &long_options.name, NULL);
    if (v30 == -1) break;
    switch (v30) {
    case 102: ... break;
    case 116: ... break;
    ...
    case 4294967165: version_etc(...); exit(0);   // --version  (return val 0x...7D)
    case 4294967166: usage(0);                     // --help     (0x...7E)
    case 128:        v29 = 1; break;
    default:         usage(1);
    }
}
```

kuna produces the **same switch** but structures everything around it with gotos:

1. **Switch breaks become `goto`.** Nine of kuna's case bodies end with
   `goto label_2c88` (the post-switch loop-latch block) instead of `break`. angr
   renders these as `break`. This alone is ~9 of the 15 extra gotos.
2. **The loop is not recovered as a loop.** kuna emits the function *prologue*
   (`set_program_name`, `setlocale`, all the field initialisers — lines 436-470 of
   `angr-vs-kuna.txt`) **after** the switch, jumping back over it with
   `goto label_2c88`; the getopt call sits at `label_2c88` reached only by back-goto.
   angr recovers the clean `while (true)` with the prologue before it.
3. **3 switch cases are missing.** kuna recovers 10 cases; the three sparse
   long-option return values (`0x80`/128, `0xFFFFFF7D`/-0x83 `--version`,
   `0xFFFFFF7E`/-0x82 `--help`) are not in kuna's jump table. They survive as an
   equality cascade folded **inside** kuna's `default:` arm
   (`do { usage(1); if (v5==-0x83) {version_etc; exit;} } while (v5 != -0x82);`).
4. **The post-getopt tail** (`target_directory_operand` / `do_move` region, lines
   478-590) is structured with ~6 more gotos (`label_2f9c`, `label_2e6a`,
   `label_2e73`, `label_3012`, `label_3150`, `label_319b`) where angr nests clean
   `if/else`.

## Owning stage

This is **structuring quality**, not a lifting/dataflow defect. The switch jump table
*is* recovered (S2/`ActionSwitchNorm`); the failure is in **S8 structure recovery**
(block structuring / loop + switch-break emission) and **S7 region** ordering — the
Ghidra `BlockGraph` structurer emitting gotos where angr's SAILR-style structurer
(loop refinement, switch-break recovery, condensing irreducible regions) emits
`while`/`break`/`if-else`. See `docs/stage-mapping.md` S7/S8 and
`s8_structure/`/`s7_regions/`.

## Hypothesis / scope concern

There is **no single Action/Rule** (the `kuna_loweredswitch.rs` template shape) that
turns this goto-soup into the clean `while/switch` form. The improvements angr shows
here are the *aggregate* of several distinct structuring transforms:

- switch-break recovery (case-exit-to-follow → `break`),
- loop head/latch recovery so the getopt loop becomes `while (true)` with the prologue
  ordered before it,
- jump-table augmentation to absorb the 3 sparse equality cases,
- goto→if/else condensing in the tail.

Each is its own pass touching S7/S8 region/structuring code — exactly the
"new pass *type* / touches S7 structuring / >1 module" profile that Hard rule 7 routes
to a **`[PROPOSAL]` draft PR** for human go/no-go rather than a single option-gated
implementation. A decider subagent confirmed the scope call (recorded verbatim in
`record.json`). See `proposal.md`.
