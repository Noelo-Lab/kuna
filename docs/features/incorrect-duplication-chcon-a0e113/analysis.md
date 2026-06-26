# incorrect-duplication-chcon-a0e113 — analysis

- **angr testcase**: `test_decompiling_incorrect_duplication_chcon_main`
- **binary**: `binaries/tests/x86_64/decompiler/chcon.o` (ET_REL `.o`)
- **function**: `main` @ `0x401ca0`  (arch `x86_64`)
- **angr**: 9.2.213

## The gap (angr is better)

kuna emits **~90 lines of garbage code** after the noreturn `__stack_chk_fail()`
call in `main`. The tail of kuna's output is:

```c
    __stack_chk_fail();
    v3 = (uint1)v11;
    *v11 = *v11 + v3;        // <-- decoded padding bytes
    *v11 = *v11 + v3;
    ... (40+ such lines) ...
    return;
```

The block is the inter-function alignment padding (`00 00 …` decoded as
`add byte ptr [rax], al`) past the end of `main`. angr does not emit any of it —
its `main` ends cleanly at `return (char)process_files(...) ^ 1;`. Metrics:
**313 loc / 33 gotos (kuna)** vs **223 loc / 7 gotos (angr)**; the garbage block
is the bulk of the difference.

## Root cause

`__stack_chk_fail` is **no-return** — once recognized, the engine plants an
`artificialHalt(noreturn)` after the call (`flow.rs` / `checkForFlowModification`)
and never decodes the trailing padding. kuna already ships a known-no-return name
list (`option noreturn_known`, default-on; vendored
`ElfFunctionsThatDoNotReturn`, which contains `stack_chk_fail`), so on a normal
dynamically-linked ELF this gap does not appear.

But this target is an **ET_REL `.o`**. In a `.o`, `__stack_chk_fail` is an
*undefined extern*: its `.symtab` entry has `address()==0` and there is no PLT
stub. The `noreturn_known` scan (`s1_loader/noreturn.rs::scan_noreturn`) keys its
`NoReturnFact`s by **address** — it skips `address()==0` symbols and finds no PLT
stub — so it emits no fact for `__stack_chk_fail`. The `relocobjects` loader
(default-on, DIV-7) binds the undefined extern to a *synthetic* call target and
installs a `FunctionSymbol` named `__stack_chk_fail` there (the call **prints**
`__stack_chk_fail()`), but no no-return flag is ever set on that synthetic
target. So flow runs past the call into the padding.

This is the ET_REL face of the known [[kuna-etrel-noreturn-merge]] /
[[kuna-noreturn-propagation-gap]] family: the no-return *fact* exists in the
name list but is never *applied* to the reloc-resolved synthetic extern.

### Proof

The engine seam that decides this is `FlowEnvironment::query_call_no_return`
(`infra/decompile_drive.rs`), which returns
`symboltab.function_is_no_return_across_scopes(entry)`. Forcing the proto flag by
hand reproduces angr's clean output exactly:

```
$ decomp_dbg: load file chcon.o; option noreturn __stack_chk_fail; read symbols;
              load function main; decompile; print C
  -> 0 garbage `add`-padding lines (block fully gone)
```

(default, and `listing`+`noreturn_propagate`/`noreturn_disc` on, all leave the 40
garbage lines — no shipped option closes this.)

## Owning stage / sub-stage

- **S2 (lift / flow follow)** — `s2_lift/flow.rs` plants the artificial halt; the
  per-call no-return decision is the `FlowEnvironment::query_call_no_return` seam
  implemented in `infra/decompile_drive.rs::ArchFlowEnv`.
- `docs/stage-mapping.md`: flow follow / `FlowInfo::queryCall` is S2.

## Hypothesis / planned fix

Add an option (`noreturn_externmatch`, default-OFF while developing) that makes
`query_call_no_return` *also* return true when the callee name at `entry`
matches the same vendored ELF known-no-return list (after stripping leading
`_`, with the upstream namespace guard). On a normal ELF the proto flag is
already set by `noreturn_known`, so the OR is a no-op (byte-identical); on an
ET_REL `.o` it supplies the missing fact for `__stack_chk_fail`/`abort`/`exit`/…
so the dead padding after the noreturn call disappears — matching angr.

Single gated check at exactly the flow decision seam (no proto mutation, no
timing concern). Name-matching logic lives in the new
`kuna_noreturn_externmatch.rs`; the list is `include_str!`'d from the canonical
`kuna-analysis/data/ElfFunctionsThatDoNotReturn` so it can never drift from
`noreturn_known`.
