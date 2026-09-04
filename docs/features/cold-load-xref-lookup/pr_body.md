## What was broken

RE-need `cold-load-xref-lookup` (1 instance, round 2, challenge `5bd1d1bb33c5d4110a29b31e`):

> **Cold-load xref lookup takes about four seconds on a 466 KB ELF** — the actual
> `kuna xrefs ./target/Obfuscation1 --to 0x80ba3d2 --json` invocation took 4.1303 seconds.
> Every independent query reloads analysis state.

The tester's diagnosis was "no persistent session". Measured, that names the workflow cost
but not the per-query one: **one cold query decoded the program three times.**

| stage | cost | what it is |
|---|---|---|
| bootstrap (loader + `.sla` + load-time oracles) | 0.10 s | |
| analysis-tier Listing walk | 1.08 s | decode pass #1, 154,608 instructions |
| `operand_refs` | 0.58 s | decode pass #2, its own linear decode |
| `xrefs::build` recursive descent | 1.26 s | decode pass #3, the same 154,608 instructions |

Both extra passes arrived through `--mode auto`, which selects `aggressive` under 500 KiB —
a preset for the quality of emitted *C*, on a command that emits none.

## Mechanism

Five changes. No new option (a reference query cannot change emitted C), no `phases.toml`
row, no catalog counters.

1. **`kuna xrefs` takes its own driver bundle** (`DriverDefaults::Query`) and resolves no
   automatic mode. The Listing's only contribution to a *reference* answer was seeds, and
   `xrefs::build` already runs its own two-worklist descent — so the `<patternpairs>`
   prologue starts go straight into that walk (`listing::xrefs::discovery_seeds`, gated to
   the same non-x86-64 architectures DIV-20/DIV-68 injects for, so x86-64's seed set is
   unchanged). `kuna xrefs --mode aggressive` still asks for the full analysis tier.
2. **The assembly render is lazy.** `print_assembly` is a second full SLEIGH parse of the
   address, and the walk paid it for every instruction to fill rows only a few carry.
3. **The PIC-base pass stops buffering p-code.** On any 32-bit PIC image the deferred
   base-relative pass cloned every instruction's whole op list *and* rendered every
   instruction — 154,608 renders to file 151 references. Both halves it needs
   (`writes_base`, the base-relative refs) are pure functions of the ops, so they are
   computed in the walk and only the answers carried; the render happens for the admitted
   instructions that actually form a reference. Its live-varnode map is a small vector
   rather than a `HashMap`, which was paying a bucket-wide `retain` per written op.
4. **`PcodeCacher::emit` borrows its input run** out of the pool instead of rebuilding it —
   one heap allocation per emitted p-code op, on **every decode in the program**, gone.
   The C++ passes a pointer into the same array.
5. **The p-code build arena and the walk's capture are pooled.** `one_instruction` allocated
   four fresh `Vec`s per decode (the C++ clears one long-lived instance); the xref walk
   allocated a vector per emitted op.

Refuted en route, and recorded in `record.json`: the per-node `ConstructTpl` deep clone
(fixed anyway, ~2%), the `WalkCursor` breadcrumb copy (`MAX_DEPTH` 32 → 12 is byte-identical
and no faster), and the SLEIGH context-DB commits (9 ms of 3.4 s).

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need cold-load-xref-lookup --json
```

Promoted verbatim into `tests/cli/cold-load-xref-lookup.json`.

Interleaved base/new, warmup + 7 reps, same shell, same binary:

| | median | min |
|---|---|---|
| base | 3418 ms | 3367 ms |
| this PR | 1039 ms | 727 ms |

**−69.6 % median**, and the JSON answer is byte-identical.

## Gates

- `make test` — **PARITY OK**, 675/675
- `make test-stages` — **PARITY OK**
- `make rust-test` — green
- `make check-spec` — green
- `kuna catalog --check` — catalog OK

Spec prose: `docs/spec/01-program-prep.md` (the query's own analysis bundle). CLI surface:
`docs/cli.md` (`kuna xrefs` does not resolve `--mode auto`).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
