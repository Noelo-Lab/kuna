# dallparity -- `decompile-all` and `decompile` do not decompile the same way

Filed against every whole-binary surface: `kuna decompile-all`, `kuna
decompile-project`, the WASM/browser front-end, and therefore decbench and every
agent-facing whole-binary workflow. The witness suite is the vendored
`fmt_{x86_64,aarch64,arm,riscv64}` fixtures plus 13 real decbench binaries (C and
C++, stripped and unstripped, O0 and O2, x86-64 ELF / shared library / ARM
Cortex-M firmware) across four modes.

## The symptom

For the same binary, the same address and the same mode, the two surfaces
produce different C -- and the whole-binary one is the weaker:

```
$ kuna decompile      O0/zlib/stripped/minigzip --addr 0x14c9
void error(char *a0)                       # correct
$ kuna decompile-all  O0/zlib/stripped/minigzip --addr 0x14c9
void error(unsigned long a0)               # weaker
$ kuna decompile-all  ... --mode aggressive
void error(unsigned long a0)               # unchanged: the mode is ignored
$ kuna decompile-all  ... --option formatstring on
void error(unsigned long a0)               # unchanged: the option is a NO-OP
```

`kuna decompile --option formatstring off` reproduces the weak form exactly, so
the missing ingredient is the `formatstring` feature and `decompile-all` silently
ignores it -- including when a **mode** turns it on. `--mode aggressive` names
`formatstring on` (`p0_knowledge/modes.rs (AGGRESSIVE_OVERRIDES)`), and `auto`
resolves to `aggressive` for anything under 500 KiB, so the option looks enabled
by default on most binaries and does nothing. IDA Pro 9.2 renders
`void error(const char *a1)` here.

## The mechanism (verified, not assumed)

The two surfaces each called the engine drive themselves:

| surface | entry point |
|---|---|
| console `decompile` (what `kuna decompile` drives in its `decomp_dbg` subprocess) | `kuna-console/src/ifacedecomp.rs`, `IfcDecompile` |
| whole-binary loop (`decompile-all` / `decompile-project` / wasm) | `kuna-console/src/project.rs (decompile_targets)` |

`IfcDecompile` wrapped `decompile_func_full_with_override_dyn` in Ghidra's
`FormatStringAnalyzer` half-B loop -- decompile, walk the `CALL` ops, read the
constant format string at each printf/scanf-family call's format-argument slot,
build the derived per-call-site `PrototypePieces` override, and decompile a
**second** time with them installed -- and enabled `readonlypropagate` for the
duration, because reading that constant on ARM requires folding a PC-relative
literal-pool LOAD (`Funcdata::fillin_read_only`). `decompile_targets` called the
drive directly and did neither. Nothing about the loop is console-specific: it
consumes only the architecture and the first decompile's `Funcdata`.

## The second effect, which is the larger one

The scoped `readonlypropagate` flip is not confined to format strings. With it
on, **every** read-only load in the function constant-folds. On x86-64 that turns
`s_15405[0]` into `'1'` and a bare `0x525e0` into a string literal; on ARM
Cortex-M, where essentially every global address arrives through a literal pool,
it rewrites most of the function body:

```c
-  *(unsigned int *)(dat_14001c5c + 0x204) = 0x200;      // decompile-all
+  v1 = dat_10003af8;                                    // decompile
-  sub_140053aa(a1,dat_140013fc,6);
+  sub_140053aa(a1,"1.2.0",6);
```

On the sampled ARM firmware `CMSIS_DAP.axf` this hit **8 of 14** sampled
functions, versus 1-2 of 14 on the x86-64 C binaries. Every decbench ARM number
was therefore measured on the weaker rendering.

## Scope of the divergence (the enumeration)

Both surfaces were diffed by code and by output. The output diff is 182
`(binary, mode, address)` records over 13 binaries, comparing
`kuna decompile --addr X --mode M` against `kuna decompile-all --addr X --mode M`
and normalizing only the function name.

| # | Class | Stronger side | Verdict |
|---|---|---|---|
| 1 | format-string varargs typing loop never runs on the whole-binary surface | `decompile` | **fixed here** |
| 2 | the loop's scoped `readonlypropagate` never applies (dominant effect; ARM literal pools) | `decompile` | **fixed here** |
| 3 | `funcstart_patterns on` + `aif on` injected for non-x86-64 objects | `decompile-all` | intentional and documented (DIV-20); discovery-tier, changes *which* functions exist, not a given function's body |
| 4 | per-function watchdog (`--max-fn-seconds`, 10 s in resolved `fast`, else 120 s) | `decompile` (unbudgeted) | intentional driver policy; a whole-binary run must not hang on one function |
| 5 | flow-extent bound: the console re-drives with `fd.get_size()` from the prior unbounded follow, the loop passes `0` (unbounded) | -- | equivalent; 0 residual differences in 182 records |
| 6 | function naming under `--addr` (`function_display_name_across_scopes` vs the canonical `find_entry_at` record) | -- | 0 name differences in 182 records |
| 7 | console-seeded facts (`map addr`, `parse line extern`, `map hash`, `type varnode`, `override flow`/`prototype`, `map param`) | `decompile` | not reachable from either CLI; not a divergence for the same input |
| 8 | `print raw` / `--regions` exist only on `kuna decompile` | -- | output-format surface, not a decompile difference |

Classes 1 and 2 are one root cause and one fix. Classes 3 and 4 are deliberate
whole-binary policy and are left alone -- both are already recorded (DIV-20 and
`docs/spec/00-overview.md` respectively). Classes 5-8 were checked and are not
divergences.

Before: 13 of 182 records diverged, all in mode `auto`; `reliable` and `fast`,
where `formatstring` is off, were already 0/14 each -- which is itself the
confirmation that the whole divergence is this one feature. After: **0 of 182**.

## Ruled out

- *A `--mode` plumbing bug.* No: both surfaces resolve the mode through the same
  `decompile_all::mode_options_for_binary`, and an explicit
  `--option formatstring on` on `decompile-all` was equally inert.
- *A load/analysis-tier difference (`listing`, symbol recovery).* No: both
  surfaces inject `option listing on` by default, and the divergent functions
  carried identical names, identical callee names and identical call graphs.
- *`decompile-all` never sets the option.* No: it reaches the architecture
  (`analysis_formatstring` is true); nothing read it.
- *The loop needs console state.* No: it consumes only `&Architecture` and the
  first decompile's `Funcdata`.

## The fix

One shared per-function decompile step,
`kuna-console/src/decompile_step.rs (decompile_one)`, holding the drive call, the
format-string loop and the scoped `readonlypropagate` flip. Both `IfcDecompile`
and `decompile_targets` call it; a caller passes the facts it has through one
`DecompileSeed` (the console its `map addr` / `parse line` / `override` state,
the whole-binary loop the function's DWARF locals and the `error(nonzero,...)`
no-return prunes). `extract_format_string_overrides` and its helpers moved with
it. This makes the shared pipeline the single source of truth rather than copying
the loop into a second place -- the next per-function policy lands once.

The discovered call-site overrides are **returned**, not stashed globally. The
console persists them into its own `pending_proto_overrides` so a later
re-decompile of the same function keeps the typing; the whole-binary loop drops
them, because each function is decompiled exactly once and persisting them across
functions would make a whole-binary run order-dependent and, again, different
from `kuna decompile`.

## What it costs, and why the feature is now an opt-in

Honouring the option exposed what it costs. Whole-binary `decompile-all`,
interleaved, minimum of 13 pairs, `--option formatstring off` vs `on`:

| binary | off | on | delta |
|---|---|---|---|
| crontab-O0 | 1,632.1 ms | 2,863.7 ms | **+75.46%** |
| psktool-O0 | 331.5 ms | 505.7 ms | **+52.54%** |
| gzip-O0 | 2,713.6 ms | 3,884.1 ms | **+43.13%** |
| tar-O0 (1,570 fns) | 24,352.2 ms | 28,067.0 ms | **+15.25%** |
| CMSIS_DAP.axf (ARM) | 2,513.9 ms | 2,653.6 ms | +5.56% |
| b2sum-O0 | 5,433.7 ms | 5,633.7 ms | +3.68% |
| minigzip-O0 | 4,986.9 ms | 5,081.2 ms | +1.89% |

That is an order of magnitude over the 5% budget, so standing requirement 4
applies: **`formatstring` is removed from `AGGRESSIVE_OVERRIDES`** and joins
`v850indirectbranch` / `dwarf_lines` in the `EXCLUDED_ON_PURPOSE` list that
`modes.rs`'s own invariant test enforces. Its shipped option default was already
`off`; only the preset made it effectively on, and (before this PR) only on the
one surface that ran the loop. So the net effect is that **no surface pays unless
asked, and both deliver when asked** -- which also stops `kuna decompile` being
charged for a feature `decompile-all` never delivered.

The default path is free, measured rather than assumed -- default vs an explicit
`--option formatstring off`, both of which disable the loop, minimum of 13 pairs:
crontab -0.11%, psktool +0.16%, gzip -0.02%, minigzip +0.16%, CMSIS_DAP.axf
+0.02%, **tar (1,570 functions, 24 s) +0.04%**, b2sum +0.02%. Worst **+0.16%**.

### The optimization was implemented, measured, and rejected

The obvious fix -- decline an override whose format-derived types already match
the call's argument types, so the caller need not be decompiled twice -- was
built and instrumented. It does not work, for a reason the instrumentation makes
plain:

| binary | functions | re-decompiled | with the skip | redundant overrides dropped |
|---|---|---|---|---|
| crontab | 160 | 25 | **24** | 12 |
| psktool | 57 | 5 | **3** | 9 |
| gzip | 248 | 25 | **25** | 8 |

The redundant overrides sit in the *same functions* as real ones, so the caller
re-decompiles anyway -- and the skip changed emitted C on **12 of 15,990**
functions, breaking the very parity this PR exists for. Reverted.

The instrumented rates also explain the size of the cost: only **15% / 5% / 10%**
of functions re-decompile on crontab / psktool / gzip. They simply carry most of
the decompile time, because a printf-heavy function is a big function. A third
timing arm confirms where the cost is: `formatstring off` + upstream
`readonly on` is free (crontab +1.15%, psktool -0.14%, gzip +0.08%), so **100% of
the cost is the second decompile**, not the read-only propagation.

Making the second decompile cheap is real work on the `formatstring` option --
recorded as a follow-up, and the precondition for ever putting it back in the
preset.

## What it buys (measured, under the opt-in)

decbench `origin/main` @`325046f`, both arms re-decompiled and re-scored against the same
source CFGs, three O0 C projects: `type_match` perfects **80 -> 88** of 1,133 scored functions
(29 improved, 10 regressed, every project's mean up); GED **4 improved, 0 regressed** of 1,168,
with cronie's GED mean 9.8844 -> 7.8241. The ten `type_match` regressions are the
`formatstring` feature's own known cost -- a `%c` argument typed `uint1` where the C variadic
promotion makes the parameter an `int`, and a format-derived width narrowing a recovered
`uLong` -- which `kuna decompile` has always produced; this PR only makes the whole-binary
surface see them too.

## Not fixed here, and why

**`kuna decompile-all` cannot load an unstripped C++ binary that uses an
anonymous namespace.** A separate, pre-existing, load-tier root cause, reported
in full in the PR body: the loader's name-only demangler reduces
`leveldb::(anonymous namespace)::HandleDumpCommand` to
`leveldb::::HandleDumpCommand` (`strip_bracket_groups` deletes the whole
parenthesized group), and the `::` scope splitter rejects the empty component
with `Non-global scope has empty name` -- aborting `load file` for the entire
binary. It is not a divergence (both surfaces fail identically, at load) and it
lives in a different crate, so it belongs in its own PR.
