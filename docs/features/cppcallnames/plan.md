# cppcallnames — implementation plan and measurements

Root cause and design rationale: `analysis.md`. This file is the plan as executed
plus the numbers behind it.

## What changed

| file | change |
|---|---|
| `decompiler/crates/kuna-console/src/engine.rs` | discovered-entry commit arm: the idempotence probe resolves **across scopes** (`find_function_across_scopes`) instead of global-only, so no generic placeholder is installed over a namespaced C++ funcsym; scope creation moves inside the branch |
| `decompiler/crates/kuna-console/src/ifacedecomp.rs` | `IfcAddrrangeLoad`: with no explicit name token, prefer the qualified `function_display_name_across_scopes(offset)`; `name_function` stays the fallback for an unknown address |
| `decompiler/crates/kuna-console/tests/verify_cppcallnames.rs` | **new** — 3 tests: the call-site render, the `load addr` header, and the control that discovery still names its genuinely-unknown finds `sub_<addr>` |
| `decompiler/crates/kuna-console/tests/verify_multiformat_dwarf.rs` | baseline arm switched from "`load addr` with the pass on" to `option dwarf off` — the existing test was pinning the `--addr` bug (see Decision 5) |
| `docs/spec/00-overview.md` | 0.1 (the gated-commit idempotence contract is cross-scope) and 0.2 (`kuna decompile`'s two selection forms name identically) |
| `docs/history.md` | DIV-59 row |
| `docs/features/cppcallnames/` | this bundle |

No option, no `phases.toml` row, no catalog-count bump, no baseline re-pin.

## Decisions

### 1. No `--option` gate — and why that is not the "when in doubt" case

Three independent reasons, in order of weight:

1. **There is no driver-tier option machinery to gate it with.** Every settable in
   `phases.toml` carries an engine phase P1–P9; `kuna-console`/`kuna-cli` only ever
   *write* options. Gating this would mean inventing a new axis, not filling in a
   row. The precedent for driver policy deliberately kept out of the catalog is
   `--max-fn-seconds` ("Driver policy, not a stage-model settable", `docs/cli.md`);
   the closest recent behavior change of this shape is issue #197.
2. **The "after" behavior already ships.** `--mode reliable` renders
   `Account::deposit(a,amount)` today. This does not introduce a new rendering
   policy — it removes an inconsistency in which two of kuna's four modes silently
   lost names the other two kept. Anyone who wants the old text can still get it
   with `--option fast_funcdisc off --option funcstart_patterns off`.
3. **There is no judgment call in it.** A synthetic `sub_<addr>` shadowing a real
   symbol at the same address is wrong under every naming policy, including
   `namestyle ghidra`. The option catalog is for decisions; this has none.

### 2. Fix the source, not the resolver

The alternative was to make `find_function_across_scopes` *prefer* a non-generic
name over a global placeholder. That would paper over the duplicate rather than stop
it, and the resolver is shared with the no-return, inline, inject-id and FID arms —
changing its preference order changes all of them. Not installing the duplicate is
the narrower change and leaves every other consumer's semantics alone.

### 3. Only the arm the witnesses implicate

`commit_analysis_output` has a second arm (`out.symbols`) and `read_loader_symbols` a
third with the same global-only probe. Neither was implicated: the passes that emit
`out.symbols` (dwarf/pdb/pclntab/rtti/objc) supply real names, which resolve to the
matching scope, and `read_loader_symbols` runs first against an empty table. Changing
them on suspicion would alter alias sets on binaries no witness covers. Left alone,
recorded in `analysis.md`.

### 4. Testing: Rust integration tests, not a `tests/stages/*.xml`

Not a preference — a structural fact, verified before choosing. Every file in
`tests/datatests/` and `tests/stages/` is an XML `<binaryimage>` document, and that
bootstrap never produces an `AnalysisOutput`, so `commit_analysis_output` is a no-op
there and the primary defect is **unreachable** from either corpus. The gate is
`decompiler/crates/kuna-console/tests/verify_cppcallnames.rs`, which drives the real
engine over the already-vendored `cpp_noreturn_x86_64` with the discovery passes
enabled in the live-CLI order (`option` before `read symbols`) — the exact
configuration `--mode aggressive` produces.

The two-pass "option off = the bug, default = the fix" shape has no analogue without
an option; the equivalent evidence is the before/after binary pair in the sweep
below, plus the third test, which pins that suppressing the placeholder did not
suppress discovery.

### 5. One existing test was pinning the `--addr` bug

`verify_multiformat_dwarf` proves the DWARF pass recovers `first_byte` on PE and
Mach-O, and used `load addr <vma>` — *with the pass still on* — as its
"no-DWARF-name baseline", asserting `sub_140001550` / `sub_0`. That baseline held
only because `load addr` ignored the symbol table, so the `--addr` fix broke it. It
was the only test in the workspace that moved.

The baseline arm now turns the pass off (`option dwarf off`), which is what it always
meant, and the claim comes out stronger:

* **PE** — nothing else names `0x140001550` (the COFF symtab FUNC entry is
  `--strip-symbol`-removed), so the `sub_140001550` assertion is unchanged and is now
  actually testing the absence of DWARF rather than the absence of a lookup.
* **Mach-O** — with DWARF off the `--redefine-sym`ed symtab entry `_l0` remains, so
  the baseline asserts `_l0`, not `sub_0`. That `first_byte` is *not* what the symtab
  says at that address is a sharper statement of "DWARF-only" than a placeholder was.

Both DWARF-recovery assertions (`after` contains `first_byte` and `char *`) and the
third test are untouched.

## Verification

### The new gate fails without the fix

Ablation (files copied aside, `git show HEAD:<path>` restored, no `git stash` —
`refs/stash` is shared across worktrees): against the pre-fix sources,
`verify_cppcallnames` is **1 passed / 2 failed** — both defect gates fail
(`sub_401070()` at the call site, `void sub_401070(void)` from `load addr`) and the
control passes, which is exactly the intended split.

### Two-arm sweep

`kuna decompile-all` before (main build) vs after (this build), 13 binaries, every
changed line classified by script:

```
  8 C binaries   : byte-identical  (incl. /bin/grep 35,070 lines, /usr/bin/ls 14,788)
  5 C++ binaries : 184 changed lines
                   181 CALL-RENAME       (the fix)
                     3 ADDR-TAKEN-RENAME (pre-existing unqualified path, see analysis.md)
                     0 anything else
```

Every C++ diff is exactly `-N/+N` — no line added or removed, no structural or
signature change anywhere.

### Repo fixture, before vs after

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/cpp_noreturn_x86_64 fail
- void fail(void) {  sub_401070();      // no-return }
+ void fail(void) {  std::terminate();  // no-return }

$ kuna decompile ... --addr 0x401070
- void sub_401070(void)
+ void std::terminate(void)
```

### Gates

| gate | result |
|---|---|
| `make test` | PARITY OK 675/675, no re-pin |
| `make test-stages` | PARITY OK, no re-pin |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |

### Speed

There is no option to flip, so `scripts.pipeline.timeit` (which times one binary
OFF vs ON) does not apply — the same reason DIV-58 recorded its own interleaved
measurement. Arms are the pre-change `kuna` and this build, runs interleaved to
cancel drift, minimum of 5 pairs reported. Numbers in `record.json`.
