## What was broken

> `map address <addr> <typedeclaration>`, `parse line`, `type varnode`, `readonly <addr+size>`
> and `volatile <addr+size>` all work in the console and none is reachable from `kuna`.
> `--option readonly on|off` is a GLOBAL toggle, not a range. `global add`/`global remove` —
> the commands `phases.toml` names as the exposure for the `code-data-partition` subphase —
> are `engine_unavailable` stubs.
>
> — `docs/re-needs/no-cli-data-code-override.md` (severity major, instances 1, rounds [2])

Stage A (#389) shipped `--assert` with nine directives and closed the naming/prototype half
of the family. This is **Stage B**: the two range directives, the console stubs the need
names, and — the deliverable the captain called load-bearing — **the acceptance probe this
need has never had**. `acceptance_id` was `null`, so no `B_DONE` could have closed it
whatever shipped.

## Why nobody could find a fixture where `readonly` did anything

Section 6 of the `--assert` proposal reported `readonly`/`volatile` as "accepted; no fold on
the fixture tried" and left the probe unfiled for exactly that reason. **The effect is real;
the order was wrong.** `Scope::addMap` folds a range property into each `SymbolEntry` as it
maps it (`database.rs:1979`, C++ `database.cc:1156-1158`) and never consults the range again,
and the per-function global snapshot then reads the *symbol's* flags. So a `readonly` painted
after `read symbols` is silently inert over every address the loader named — which is every
address a caller is likely to name. Painted before, the same command on the same fixture
folds.

That forced a fourth script slot (`Slot::Image`, between the `option` lines and
`read symbols`) and, for the in-process surface — where `bootstrap_from_object` has read the
loader's symbols before a caller can say anything — a re-application of the property to the
symbols the range covers. Both surfaces were then measured emitting byte-identical C.

## The mechanism

```bash
# .data is writable, so the loader never calls it read-only — but nothing in this
# program writes these eight bytes, and the agent has checked.
kuna decompile ./fw.elf sample --assert 'readonly 0x404028+8'

# 0x50000000 is a device register. Two reads of it are two reads.
kuna decompile ./fw.elf sample --assert 'volatile 0x50000000+4'
```

| | emitted C |
|---|---|
| baseline | `return scale * a0 + bias + dat_50000000 * 2;` |
| `readonly 0x404028+8` | `return a0 * 7 + 100 + dat_50000000 * 2;` |
| `volatile 0x50000000+4` | `v1 = dat_50000000; return scale * a0 + bias + v1 + dat_50000000;` |
| both | `v1 = dat_50000000; return a0 * 7 + 100 + v1 + dat_50000000;` |

Asserting a `readonly` range also turns read-only propagation on for the run:
`Funcdata::fillin_read_only` is gated on the program-wide `readonly` option, which is
default-off, so painting the property and then declining to act on it would be a directive
that is accepted and does nothing — this family's own failure mode. It is applied *ahead of*
the caller's own `--option`s, so an explicit `--option readonly off` still wins. The reverse
composition is not equivalent, which is the control: `--option readonly on` with no range
folds nothing here, because the loader marks read-only from section flags.

`global add`/`global remove` are wired onto `Database::add_range`/`remove_range` — the second
half of the need's Hypothesis overturned again, since both have been ported and public in
`p0_knowledge/database.rs` the whole time (~15 lines each). But **no `global` directive
ships**, and that is a measurement rather than a scope cut: every stock cspec's `<global>`
claims the whole default data space (`<range space="ram"/>`), so on any ordinary image the
range is global before the caller speaks and `global add` alone is measurably a no-op. Only
the removal direction moves the C — it kills a store that survives a call — and both
directions are covered by a test that asserts the C changed.

## The acceptance probe

`a-567d3207ce62`, filed on the need and promoted verbatim to
`tests/cli/no-cli-data-code-override.json`:

```
kuna decompile .../assertranges_x86_64 sample --json \
  --assert 'readonly 0x404028+8' --assert 'volatile 0x50000000+4'
```

asserting two `applied` rows at `P1 code-data-partition` and that the C contains `* 7` and
`+ 100`, no longer names `scale` or `bias`, and no longer merges the device reads into
`dat_50000000 * 2`. **FAILS at `ed353fb2`** (exit 2, `unknown directive "readonly"`).

```
verify --need no-cli-data-code-override → pass 1 / fail 0, transition: closed
```

Its fixture (`assertranges_x86_64`, 15,800 bytes, source vendored beside it) is new and
purpose-built, because nothing in the tree had a `.data` global that is read and never
written next to an out-of-section device word. CI has no dataset, so the probe promotes
verbatim.

## Tests

`kuna-console/tests/verify_assertranges.rs` — 11 tests, every one a diff of the emitted C
against a measured baseline, because reviewing on "did the command return Ok" is how
`override prototype` got shipped accepted-and-inert. Including the two controls above, the
explicit-size parse (a 4-byte range stops short of a word four bytes past its end; an 8-byte
one reaches it), the rejection of a zero-length range, and the `global remove` → `global add`
round trip. Plus parser and script-shape tests in `kuna-cli` (`--assert` lowering, the
`Slot::Image` line landing before `read symbols`, `option readonly on` landing before the
caller's own options, and no `option readonly` line appearing when no range is asserted).

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK 675/675** |
| `make test-stages` | **PARITY OK 603/603** |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `verify --need no-cli-data-code-override` | acceptance **PASS** |

No option row, no `tests/stages/` case, no catalog counters and no DIV row: a CLI surface
cannot change emitted C on its own, which is the tooling track's rule and what Stage A
shipped under. The one console-command change — an optional explicit size after the address
on `readonly`/`volatile` — is additive; the corpus's only two uses are the bracketed
`volatile [ram,...]` form, which carries no trailing token.

**Speed:** zero on every existing invocation. A range directive adds one console line and one
symbol-table walk over the range (one lookup per covered symbol, not per byte) before any
decompile. No pipeline pass is added, and these are image-scoped, so unlike Stage A's
symbol-scoped directives they never force a second decompile.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
