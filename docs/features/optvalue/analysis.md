# optvalue — analysis

## 1. The measured symptom

`kuna decompile` accepted a `--option NAME VALUE` pair whose VALUE the engine
refuses, exited 0, and printed the default C.

```console
$ kuna decompile fmt 0x26a0 --addr --option realtypes zzz > bad.c ; echo $?
0
$ kuna decompile fmt 0x26a0 --addr                          > default.c ; echo $?
0
$ cmp bad.c default.c && echo identical
identical

$ kuna decompile-all fmt --addr 0x26a0 --option realtypes zzz ; echo $?
error: option realtypes: Must specify toggle value, on/off
1
```

(`fmt` = `decbench .../O2/coreutils/stripped/fmt`, `main` at `0x26a0`.)

This is the failure the option-NAME check (`optname.rs`, `docs/spec/00-overview.md`,
"the option-name contract") exists to prevent, one field over: an agent flips a
decision, sees byte-identical output and a zero status, and records that the
decision point is innocent. Here the decision never flipped. It is worse than a
bad name, because a bad name at least produces no plausible story — `zzz` looks
like a considered ablation that changed nothing.

Every whole-binary surface already refuses the same pair: `decompile-all`,
`functions` and `xrefs` all print `error: option realtypes: Must specify toggle
value, on/off` and exit 1. Only the subprocess surface disagreed.

## 2. Root cause

`kuna decompile` does not hold an error object: it builds a `decomp_dbg` script
and pipes it on **stdin** (`kuna-cli/src/decompile.rs (build_script_for_input)`,
the spawn at `decompile.rs`), then recovers what failed by reading the transcript
(`check_errors`).

The console does report the refusal — `IfcOption::execute`
(`kuna-console/src/ifacedecomp.rs`) routes the pair to
`Architecture::set_kuna_option` or `OptionDatabase::set` and maps either one's
error to `IfaceError::execution`, which prints

```
[decomp]> option realtypes zzz
Execution error: Must specify toggle value, on/off
```

— but `errorisdone` is set only for a **pushed** script
(`kuna-console/src/interface.rs (push_script_state)`). A script arriving on
stdin is not pushed, so the session survives the raise, runs `read symbols`,
`load addr`, `decompile` and `print C`, and exits 0.

`check_errors` had arms for the architecture (`load file`), the analysis commit
(`read symbols`) and the selection (`load function` / `load addr`). It had none
for `option`, so nothing read that line.

## 3. Why the VALUE cannot be checked in the parser

The NAME is checkable up front because there are exactly two tables to look in
(`KUNA_OPTION_NAMES`, `UPSTREAM_OPTION_ELEMENTS`). The VALUE is not: each option
owns its grammar, and only the setter knows it. Measured over every catalogued
option (205 rows, `kuna catalog --json`, each fed `zzzbadvalue` through the
console):

| refusal wording | options |
|---|---:|
| `Must specify toggle value, on/off` | 189 |
| 16 option-specific wordings (`namestyle must be "angr" or "ghidra"`, `Unknown int3pad value: … (expected off\|warn\|halt)`, `symbolnamebound wants a positive scope-component count or \`off\``, `outline: expected <fn_entry>:<head>-<exit>`, …) | 16 |

plus the 37 upstream `OptionDatabase` names, whose grammars include a prototype
model name (`defaultprototype`), a function name (`inline`, `noreturn`), a
rule specifier (`warning`, `togglerule`) and an integer (`jumptablemax`).
Reimplementing that in the CLI parser would be a second, drifting copy of the
engine's own judgement. Reading what the engine actually said does not drift.

## 4. The fix

`option_failure(out)` in `decompile.rs`, modelled on the existing
`read_symbols_failure`: walk the transcript tracking the last `[decomp]> ` echo,
and when a `CONSOLE_DIAGNOSTICS`-prefixed line follows an `option …` echo,
answer `option <name>: <reason>` — the exact wording and exit code
`decompile_all.rs (apply_one_option)` already produces. It is called from
`check_errors` **after** the architecture arm and **before** the analysis-commit
arm.

The order is load-bearing in both directions:

- after the architecture arm, because a failed `load file` leaves no image and
  every later `option` line answers `Execution error: No load image present` —
  a consequence, not a bad value (the `EMPTY_SCOPE` transcript fixture is
  exactly this shape, and its existing assertion still reports the load
  failure);
- before the analysis-commit arm, because an option line runs before
  `read symbols` and a refused option is the earliest thing that can make a
  later command fail.

One more clause at the call site: the retry that widens the discovery bundle
fires on a name MISS and nothing else, and `option noreturn <not a function>`
puts `Unknown function name:` in the transcript. The retry now also requires
`option_failure(&combined).is_none()`.

No option row: this only makes a refusal that already existed observable on one
more surface. Emitted C never changes — a run that succeeds produces exactly the
bytes it produced before, and a run that now fails produced C that did not
reflect what was asked for.

## 5. Verification

**No false positives.** All 205 catalogued options set to their catalogued
DEFAULT value in one console session: **0** diagnostic lines. The three
driver-injected bundles — `RELIABLE_OVERRIDES` / `AGGRESSIVE_OVERRIDES` /
`FAST_OVERRIDES` (70 pairs), `driver_default_options`'
`errortoomanyinstructions off` / `listing on` / `funcstart_patterns on` /
`aif on`, and the `option readonly on` an `--assert` range implies: **0**
diagnostics. So no `--mode` and no `--assert` can synthesize a fatal line.

End to end: one `kuna decompile fauxware main` carrying all 205 options at
their default value exits 0, as does each of `--mode auto|reliable|aggressive|fast`,
alone and with an `--assert` range.

And one `kuna decompile` per pair, over every catalogued option crossed with
every catalogued value — 420 invocations — exits 0 on 418. The two that do not
are the catalog's placeholder tokens, `symbolnamebound <n>` and
`outline <fn>:<head>-<exit>[;...]`, which are not values.

**Full prefix coverage.** All 205 catalogued options fed `zzzbadvalue`: 205/205
answered with a single-line `Execution error: ` — the first entry of
`CONSOLE_DIAGNOSTICS` — 0 accepted it, and none printed a non-diagnostic line
after the echo. The 37 upstream names, swept one per session: every refusal is
likewise a single-line `Execution error: `. Eight of them
(`extrapop`, `maxlinewidth`, `indentincrement`, `commentindent`, `commentstyle`,
`integerformat`, `setaction`, `maxinstruction`) **tolerate** garbage in
upstream's own parser and print a success line; those are unchanged here, and
`decompile-all` tolerates them identically.

**Nothing relied on the old behaviour.** The 51 `--option` pairs across the 170
`tests/cli/*.json` probes are all valid values (plus the deliberate
`zzzznotanoption off` NAME probe, which already expects a non-zero exit). The
only `docs/` occurrences of a refused pair are in
`docs/decbench/triage/audit-uncatalogued-options.md`, which **documents** these
errors as `decompile-all` output. The `tests/stages` uses
(`option togglerule <rule> off`, `option braceformat function skip`) are valid
three-token forms and run through the datatest driver, not this path.

**Multi-parameter options still work.** `--option togglerule "subright off"`
lowers to the three tokens the console wants and is accepted, before and after.

**Read statically, not just swept.** `IfcOption` maps both setters' errors
unconditionally to `IfaceError::execution`, so the prefix is always
`Execution error: ` whatever the `KunaError` kind, and no option setter's
message contains a newline (the only multi-line `KunaError` in `kuna-decomp` is
`p2_lift/flow.rs`'s `Basic blocks already calculated`). The two parse errors the
command itself can raise (`Missing option name`, `Too many option parameters`)
arrive under `Command parsing error:`, also in `CONSOLE_DIAGNOSTICS`.

**Output is unchanged.** `decompile-all` over fauxware, `decompile` over its
first 20 functions, and `decompile` over `fmt` O2's first 40 functions plus the
six named witnesses: 0 byte differences in 69 comparisons, old binary vs new.

One existing test moved deliberately.
`decompile_cli.rs (an_unrelated_console_diagnostic_is_not_a_commit_failure)`
used an `option` diagnostic as its "unrelated" witness — which the new arm now
claims by design. It is re-witnessed on `rename v2 buf`, a command no arm
claims, and the option case gets its own end-to-end test beside it.

## 6. Speed

No block is required (no option to ablate) and none is meaningful: the change
adds one extra pass over the same in-memory transcript `check_errors` already
scans three times — a few KB, once per invocation, against a ~300 ms
subprocess decompile. Interleaved min-of-15 on `fmt` `0x26a0`, patched vs
`origin/main` at the same commit: 282.4 ms → 270.6 ms on the min, i.e. inside
this box's contention noise and in the wrong direction to be caused by added
work.
