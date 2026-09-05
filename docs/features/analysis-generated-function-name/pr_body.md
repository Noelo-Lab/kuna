## What was broken

RE-need `analysis-generated-function-name` (major, 2 instances, 2 testers, 2 formats, 2 modes):

> **Analysis-generated function name cannot be used by decompile**
> kuna strings assigned the relevant strings to `sub_4045b0`, but `kuna decompile` rejected
> `sub_4045b0` with exit 1. The address plus `--addr` succeeded.

and its twin, filed independently by a second tester on an ELF x86:

> a function listed in auto mode cannot be selected by its displayed name in reliable mode

kuna printed a name it would not then accept. That is worse than not finding the function:
an agent has no way to tell a name it mistyped from a name the tool minted.

## The real mechanism (both filed diagnoses are overturned)

The hypothesis was "a narrower symbol table"; `T_REFUTE` corrected it to "`sub_<hex>` is a
synthesized display name, never registered as a symbol anywhere, so fold the name back to an
address in `kuna-cli`". Neither holds.

`sub_4045b0` **is** a `FunctionEntry` name in `ConsoleProgram::function_entries_canonical`,
and `resolve_entry(Name(..))` resolves it — on main today, `kuna decompile --json`,
`kuna disassemble` and `kuna decompile-all` all select the witness by that exact name. Only
the **text** surface fails, and it fails because its `decomp_dbg` session never *discovered*
the entry:

| surface | how it loads | discovery options |
|---|---|---|
| `functions`, `decompile-all`, `decompile-project`, `decompile --json` | in-process `load_program` | `listing on` + **`funcstart_patterns on` + `aif on`** on non-x86-64 (DIV-20/DIV-68) |
| `kuna decompile` (text) | `decomp_dbg` script from `build_script` | `listing on` **only** |

`KeyVal2.exe` is PE32 **i386** — non-x86-64 — so the enumerating surfaces ran the prologue
matcher and the entry existed; the script path did not and it did not. Bisected on the
witness: `listing` alone still answers `no function matches`; `listing` +
`funcstart_patterns` selects it. There is no name to fold and no table to widen — there is
an entry set to agree on.

The mode policy is why two testers filed what looked like two bugs. `auto` resolves to
`aggressive` under 500 KiB and that preset names all three options itself, so the gap is
reachable only above the size threshold (`KeyVal2.exe` is 558,592 bytes → `reliable`) or
under an explicit `--mode reliable` — which is precisely the twin sighting.

## Why this is a retry and not plain alignment

The obvious fix — inject the same bundle up front so both surfaces share one policy — was
built first, swept, and **rejected by its own sweep**. The bundle changes the entry set, and
not every entry it adds is real: on i386 and PPC64 the prologue matcher seeds a start a few
bytes inside a function it already knew (PPC64 ELFv2's local entry point sits 8 bytes past
the global one), and `funcboundflow` then truncates the outer function's flow at that seed.

```
 plt_ppc64le __do_global_ctors_aux, bundle injected up front
-void __do_global_ctors_aux(void)          void __do_global_ctors_aux(void)
-{ ... do { (*v2)(); ... } while (...) }   { // warn: funcboundflow: fall-through reached
                                           //       the next function entry; truncating
```

8 such truncations across the 33 non-x86-64 fixtures, and no other difference. Trading a
selection bug for an output bug on the witness's own architecture is not a fix.

So the wider inventory is paid for **only where it is the answer**. `decompile()` splits the
shared `driver_default_options` table into the Listing (injected as always) and the discovery
bundle (held back); a by-name selection the console answers with `no function matches` is
retried once with the bundle appended. The retry is gated on a **miss** — not an ambiguity,
not a load failure, not a pipeline abort, never an `--addr` selector — so everything that
already resolved runs the byte-identical script it always ran, and the husk trade stays the
whole-binary surfaces' to make, where a complete inventory is the point.

The husk itself is a live analyzer-tier defect, visible on main today in `kuna functions`
(`__do_global_ctors_aux` reports size 8 beside a `sub_948` of 112). It is **not** closed here
and is recorded in `record.json` as worth a need of its own.

No `phases.toml` row, no `options.rs` registration, no catalog counter, no stages XML, no new
DIV: a retry that only fires where the command used to exit 1 cannot change emitted C for any
run that worked, and DIV-15/DIV-20/DIV-68 are already recorded.

## Collateral sweep

`kuna decompile --mode reliable <name>` for **every** name `kuna functions` reports, over all
33 non-x86-64 fixtures plus the witness (250-name cap). The before arm is
`--option funcstart_patterns off --option aif off`, which makes the discovery list empty —
main's script byte for byte, with no retry to make.

| | |
|---|---|
| targets / names swept | 34 / 723 |
| resolved before, **unchanged** | 318 |
| resolved before, **changed** | **0** |
| **unlocked by the retry** | **403** |
| still failing | 2 |

The two are `macho_arm64e` / `macho_imports_arm64` `printf`, which names both the stub and
the import slot; the selector model reports the ambiguity and its candidates rather than
guessing, identically before and after. x86-64 is unchanged by construction (the discovery
list is empty there) and was confirmed empirically at 0 differences over 40 fixtures during
the alignment sweep.

## Acceptance

```
python -m scripts.repipe.verify --need analysis-generated-function-name --json
→ {"total": 1, "pass": 1, "fail": 0, "closed": 1, "regressed": 0}
```

`kuna decompile KeyVal2.exe sub_4045b0` now exits 0 and renders `void sub_4045b0(void)`.

Promoted to `tests/cli/analysis-generated-function-name.json`. `verify --promote` refuses the
recorded acceptance because its target is `binary_source: dataset` and CI has no dataset, so
the vendored probe asserts the identical contract on the in-repo `entrymain_arm` fixture — a
stripped ARM PIE whose `sub_410` is discovered only by the bundle — under `--mode reliable`,
which is load-bearing at that size. `make test-cli`: **14/14**.

Negative control both ways: with only the `decompile.rs` hunk reverted, the promoted probe
FAILS and both new cargo tests FAIL; restored, all three pass.

Cost on the witness (3 interleaved runs): a resolved name or `--addr` is unchanged; a
generated name is 1.84–1.96 s against exit 1 in 0.73–1.03 s. The second attempt is a
whole-program decode — the DIV-68 price `kuna functions` already pays on the same image.

## Gates

| gate | result |
|---|---|
| `make test` | PARITY OK, 675/675 assertions |
| `make test-stages` | PARITY OK, 597/597 assertions |
| `make rust-test` | green, 5,283 tests |
| `make check-spec` | OK (lenient mode) |
| `kuna catalog --check` | catalog OK |
| `make test-cli` | 14/14 |
| acceptance probe | PASS |

🤖 Generated with [Claude Code](https://claude.com/claude-code)
