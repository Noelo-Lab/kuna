## What

An identifier containing `$$` was reaching emitted C. Worse than cosmetic: the
placeholder was a **second** identifier for a stack Symbol already declared under its
`vN` name, so the body **read a variable that is never written** while the update landed
on the other name.

`tar` `xstrcat` (`O2`, `0x611a0`) is the sharpest witness -- the x86-64 `va_arg`
expansion, testing and incrementing an uninitialised variable:

```c
  uint4 $$undef00000000; // stack - 0x58     <-- declared, READ TWICE, NEVER WRITTEN
  uint8 v12;             // stack - 0x58     <-- SAME SLOT, written twice, read once
  ...
      v4 = v12 & 0xffffffff;
      if (0x30 <= (uint4)$$undef00000000) { ... }
      else v12 = (uint8)((uint4)$$undef00000000 + 8);
```

After:

```c
  uint8 v12; // stack - 0x58
  ...
      v4 = v12 & 0xffffffff;
      if (0x30 <= (uint4)v12) { ... }
      else v12 = (uint8)((uint4)v12 + 8);
```

`ip` `set_qlen` (`O2-noinline`, `0x2b420`) is the second mechanism -- there the
placeholder is in the **Symbol table itself**, so it leaked out of the C into the
structured surfaces too:

```
before:  char $$undef00000001 [16]; // stack - 0x38     |  {"name": "$$undef00000001", ... "stack_offset": -56}
         $$undef00000001 = CONCAT124(v4[0],a1);         |
after:   char v7 [16]; // stack - 0x38                  |  {"name": "v7", ... "stack_offset": -56}
         v7 = CONCAT124(v4[0],a1);                      |
```

Full write-up: [`docs/features/undefname/analysis.md`](docs/features/undefname/analysis.md).

## Why it happened

`Scope::buildUndefinedName` (`database.cc:2854`) names an empty-named Symbol `$$undef`
plus 8 hex digits. Upstream that string is unobservable for two reasons, and kuna
inherits neither:

* upstream `ActionNameVars::apply` renames **every** still-undefined Symbol before it
  returns -- its last statement is `localmap->assignDefaultNames(base)`
  (`coreaction.cc:3079`);
* upstream `PrintC` reads `sym->getDisplayName()` **live** at emit time.

kuna caches the name per HighVariable (`HighVariable::kuna_name`), and its port of
`ScopeInternal::assignDefaultNames` had **zero call sites in the tree**.

Instrumented (a temporary backtrace on `set_kuna_name`; note `kuna decompile` shells out
to `decomp_dbg` with **stderr piped**, so the in-process `decompile-all` path is the one
to instrument):

```
[undef] high=HighVariableId(103) addr=-88 size=4 entry_size=8 sym_off=0 -> $$undef00000000
[undef-set] $$undef00000000   <- frozen onto the partial-cover high
[undef] high=HighVariableId(104) addr=-88 size=8 entry_size=8 sym_off=0 -> v12
```

One Symbol, two highs. `ScopeLocal::resolve_default_name_override` renames the covering
Symbol only for a **whole-symbol** cover (the C++ `namerec` gate
`sym->isNameUndefined() && high->getSymbolOffset() < 0`) and otherwise returns the
Symbol's *current* display name; the 4-byte partial sorts first in location order, so it
froze the placeholder and the 8-byte sibling then renamed the shared Symbol. Nothing
back-filled the first cache. The `--json` `variables` array confirms the diagnosis
non-perturbingly: exactly **one** Symbol at that offset, and no `$$undef` Symbol at all
-- so the printed placeholder corresponded to no live Symbol, which rules out the
"second Symbol" reading the case was originally filed under.

**The gate is a faithful port and is left alone.** Relaxing it would move *when* a Symbol
consumes `base` and renumber every later local. Instead `ActionNameVars` grows the tail
it was missing (`p6_variables/kuna_undefname.rs`): wire the catch-all `assignDefaultNames`,
then re-resolve every high still holding a placeholder through the *same*
`queryProperties(addr,1,usepoint)` containment query that bound it -- kuna's stand-in for
the printer's live read. Both steps run after every `vN` is handed out and after the
spacebase attach pass, so neither renumbers an existing local and every `&symbol` render
stays byte-identical.

## No option

`AGENTS.md`: "a strict bug fix that only corrects wrong output needs no flag." An
identifier containing `$$` is not valid C under any naming policy (`namestyle ghidra`
reaches the same tail and produces upstream's `xStack<offset>` form). The ablation
decided it rather than the argument: **0 of 675** upstream assertions move. DIV-58 (DIV-55/56/57 were claimed by #252/#253/#254 while this branch was in flight; re-check the number at merge).

## Breadth (re-measured on this rebase, post-#249 through #254)

One binary per project per optimisation level: **116 stripped binaries / 90,808
functions**, x86-64 and ARM Cortex-M, `O0`/`O2`/`O2-noinline`, watchdog disabled so a
timeout cannot blank a function in either arm.

| measure | before | after |
|---|---|---|
| binaries emitting a placeholder | 73 | **0** |
| functions emitting one | 371 | **0** |
| total occurrences | 2,142 | **0** |
| of those, reads / writes | 1,503 / 639 | **0 / 0** |
| functions whose Symbol table carried one | 93 | **0** |
| placeholder Symbols in those tables | 125 | **0** |
| functions produced | 90,808 | 90,808 |
| per-function error records | 22 | 22 |

## Blast radius

Twelve binaries (the eight leakiest plus four that never leaked, as controls) decompiled
whole in both arms and diffed function by function:

| binary | functions | changed | changed **without** a placeholder in the before text |
|---|---|---|---|
| `tar` (leaky) | 1125 | 13 | 0 |
| `ip` (leaky) | 1008 | 38 | 0 |
| `e2fsck` (leaky) | 1421 | 21 | 0 |
| `bash` (leaky) | 2538 | 13 | 0 |
| `tar` (leaky) | 1585 | 9 | 0 |
| `ip` (leaky) | 1912 | 33 | 0 |
| `e2fsck` (leaky) | 1909 | 21 | 0 |
| `betaflight_STM32F405.elf` (leaky) | 5797 | 14 | 0 |
| `update-passwd` (control) | 98 | 0 | 0 |
| `bzip2` (control) | 160 | 0 | 0 |
| `ch.elf` (control) | 770 | 0 | 0 |
| `CMSIS_DAP.axf` (control) | 267 | 0 | 0 |
| **total** | **18590** | **162** | **0** |

Every changed function had a `$$undef` identifier before; the function *set* is identical
in both arms (asserted), error-record counts match, and all four controls are byte-identical.
Stack-local declaration lines go 16,828 -> 16,612: once the two names agree, the printer's existing
whole-sibling guard folds the duplicate declaration.

> Method note: a first pass with the watchdog **enabled** reported 3 "unexplained"
> changes. All three were timeouts in the loaded before-arm (an aborted function has no
> `code`, so it diffed against a real body with no placeholder to match). Re-run serially
> with `--max-fn-seconds 0` they are ordinary placeholder fixes, and one is byte-identical.

## Benchmark

GED scores CFG shape, so an identifier rename cannot move it. `scripts.decbench.rescore`
on `O2-noinline-iproute2-ip-netns_add` (a case whose stored pane carries the placeholder):
before 8.0, after 8.0, **delta 0.0**, no metric drift. This item came from the NOVEL pool
precisely because it does not rank by margin.

## Speed

`scripts.pipeline.timeit` flips an **option** on one binary, so it does not apply to a
default-path fix; this is the interleaved two-binary A/B that DIV-52 and DIV-54 used for
the same reason. `decompile-all --mode reliable`, whole binary, medians:

| binary | before (ms) | after (ms) | delta | pairs |
|---|---|---|---|---|
| `tar-O2` | 52,998.0 | 50,085.7 | -5.50% | 7 |
| `ip-O2-noinline` | 30,959.3 | 31,025.3 | +0.21% | 7 |
| `gzip-O2` | 4,733.7 | 4,891.3 | +3.33% | 7 |

Worst case +3.33%, inside the +5% budget. The added work is one bounded `nametree` range
query plus one high walk that exits immediately on any function with no placeholder.

## Gates (verbatim, on this rebase onto `208bb155`)

```
make test          -> datatests: 675/675 assertions passed   === baseline parity ===  PARITY OK
make test-stages   -> datatests: 394/394 assertions passed   === baseline parity ===  PARITY OK
make rust-test     -> 4427 passed, 0 failed
make check-spec    -> check-spec OK (lenient mode)
kuna catalog --check -> catalog OK: documents exactly the registered kuna options
```

`docs/baseline.json` is untouched. `docs/baseline-stages.json` was re-recorded, not
hand-merged: 386 -> 394, and the diff is exactly this test's 8 new keys plus the footer.

## Test

`tests/stages/ghdec-undefname.xml` carries both mechanisms as real bytes with their
callees stubbed. Against the pre-fix build **3 of 8 assertions pass**; with the fix,
8/8. Pass 2 sets `option dedupvardecls off` to pin that neither witness depends on the
P9 declaration collapse -- this is a P6 naming fix, so the off path renders identically.

## Findings that correct the record (for the next agent)

**1. DIV-52's `collapse_symbol_decls` is NOT what folds the duplicate declaration.** The
triage predicted it would, once the two names agreed. It cannot: it is gated on
`option dedupvardecls`. What actually folds the declaration is the older, **ungated**
`is_scalar_partial_piece` guard (`p9_emit/printc.rs`), which skips a high that is a
strict partial of a scalar mapped Symbol when a whole-cover sibling **of the same name**
exists -- before this fix the names differed (`$$undef00000000` vs `v12`), so the guard
could never match. Proven, not reasoned: pass 2 of the stage test runs with
`option dedupvardecls off` and the fold still happens.

**2. `kuna decompile` shells out to `decomp_dbg` with stderr PIPED, so engine-level
instrumentation is invisible on that path.** An `eprintln!`/backtrace added inside the
engine produces *nothing* when you drive it with `kuna decompile ...`, which reads as
"my hypothesis is wrong" when it only means "you cannot see it from here". The
in-process path is `kuna decompile-all <binary> --json --addr 0x...` -- instrument
against that. This has now cost two agents in this campaign real time.

A third, cheaper one: the non-perturbing discriminator for symbol-vs-cache questions is
the `--json` `variables` array (`extract_variables` reads the ScopeLocal Symbol table
directly). The console `rename` command, which the case was originally filed with,
**perturbs** -- it also deletes the stack-canary locals.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
