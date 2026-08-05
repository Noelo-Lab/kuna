## What

kuna printed `sub_<addr>` at every call to a demangled C++ function on an
**unstripped** binary — for a name it already had. `kuna functions` listed
`Account::deposit`; `kuna decompile <bin> 'Account::deposit'` resolved and
decompiled by that name; only the call site lost it. IDA Pro 9.2 and Ghidra 12.1
both render the qualified name on the same bytes.

Captured on the vendored fixture `cpp_noreturn_x86_64` (no probe needed — this
reproduces in-tree):

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/cpp_noreturn_x86_64 fail
```
```diff
  void fail(void)
  {
-   sub_401070(); // no-return
+   std::terminate(); // no-return
  }
```

A second, independent defect closed here — `--addr` never applied the known symbol
name, so the *same function* printed two different headers depending on how you
selected it:

```console
$ kuna decompile ...cpp_noreturn_x86_64 --addr 0x401070
```
```diff
- void sub_401070(void)
+ void std::terminate(void)
  {
    (*dat_404018)(); // jump-as-call
```

And on a purpose-built probe (`g++ -O0`), the shape the report was filed on:

```diff
  int4 probe_member_fields(void *a,int4 amount)
  {
    ...
-   d = sub_271c(a,amount);
-   v = sub_2776(a);
+   d = Account::deposit(a,amount);
+   v = Account::available(a);
    o = 5;
-   sub_279c(a,&o);
+   Account::bump(a,&o);
    return o + d + v;
  }
```

IDA 9.2 on the same binary: `d = Account::deposit(a, amount);` — identical modulo
variable naming.

## Why (the real mechanism, not the obvious one)

Three plausible causes are all **wrong**, and were checked: the name is not stored
only as an alias, there is no identifier filter rejecting `::`, and the call-site
path reads the same symbol table `kuna functions` does. It is also not a demangler
defect — a demangled name with *no namespace* (`_Z8topleveli` → `toplevel`) rendered
fine all along.

It is **scope shadowing in the gated-commit seam**, and it is mode-dependent:

| surface | `Account::deposit` call site |
|---|---|
| `--mode reliable` | `Account::deposit(a,amount)` ✅ |
| default (`auto` → `aggressive` under 500 KiB) | `sub_271c(a,amount)` ❌ |
| `--option fast_funcdisc off --option funcstart_patterns off` | `Account::deposit(a,amount)` ✅ |

1. `read_loader_symbols` installs a demangled funcsym in its **namespace** scope —
   `std::terminate` is base `terminate` in scope `std`.
2. A discovery pass (`fast_funcdisc` / `funcstart_patterns`, both in
   `AGGRESSIVE_OVERRIDES`) re-reports the same VMA.
3. `commit_analysis_output`'s discovered-entry arm named it `sub_<addr>`, resolved
   that synthetic name to a scope — no `::`, so **GLOBAL** — and probed
   `find_function(scope, addr)` **in that scope only**. It missed the real symbol
   and installed a *duplicate* generic FunctionSymbol in GLOBAL at the same address.
4. `FlowInfo::queryCall` resolves through `find_function_across_scopes`, which
   searches **global first**. It found the duplicate.

Turning off *either* discovery pass alone did not restore the names — both report
the VMA, so either one suffices to install the duplicate. The in-source comment
claiming the probe "still lets a real `.symtab`/`.dynsym` name win on a non-stripped
binary" was false for every `::`-qualified symbol, and `find_function_across_scopes`
already documented the hazard verbatim; the two facts had simply never been
connected.

**The fix**: resolve the idempotence probe across scopes — the port of C++
`Scope::queryFunction`, which spans the scope tree. Discovery is unchanged: the
passes still run, still find the same VMAs, a genuinely unnamed find is still
`sub_<addr>`, and `register_symbol` is untouched so `sub_<addr>` remains a working
selection alias.

## Why nothing caught it

* Every `tests/datatests/` and `tests/stages/` file is an XML `<binaryimage>`, and
  that bootstrap never produces an `AnalysisOutput` — `commit_analysis_output` is a
  structural no-op there, so **neither corpus can reach the defect**.
* `verify_noreturn_demangle` drives the **console** with shipped defaults (no
  discovery pass), where the duplicate is never created. It printed
  `std::terminate()` and passed — while the CLI on that same fixture printed
  `sub_401070()`. No test asserted the CLI render. Now one does.

## No option

Deliberate, for three reasons:

1. **There is no driver-tier option axis.** All 76 `phases.toml` settables carry an
   engine phase P1–P9; `kuna-console`/`kuna-cli` only ever *write* options. Gating
   this would mean inventing an axis, not filling in a row (precedent:
   `--max-fn-seconds`, "Driver policy, not a stage-model settable").
2. **The "after" behavior already ships** under `--mode reliable`. This removes an
   inconsistency in which two of four modes silently lost names the other two kept.
3. **There is no judgment in it.** A synthetic placeholder shadowing a real symbol
   at the same address is wrong under every naming policy, `namestyle ghidra`
   included.

`docs/history.md` DIV-59 records it.

## Sweep — `decompile-all` before vs after, 13 binaries, every line classified

```
  8 C binaries   : byte-identical  (incl. /bin/grep 35,070 lines, /usr/bin/ls 14,788)
  5 C++ binaries : 184 changed lines
                     181 CALL-RENAME        <- the fix
                       3 ADDR-TAKEN-RENAME  <- pre-existing path, see below
                       0 unexplained
```

Every C++ diff is exactly `-N/+N`: no line added or removed, no structural or
signature change anywhere. Plain C is untouched **structurally** — a C symbol has no
`::`, so the global-only probe always found it and no duplicate was ever created.

Discovery itself is provably unchanged: `kuna functions --json` before vs after over
**15 discovery fixtures** (ARM/Thumb, Cortex-M, AArch64, RISC-V, Mach-O, i386-PIE,
`funcstart_patterns`, `aif_gap`, stripped ELF) — **zero entries lost, gained, or
renamed** on any of them. The fix only changes naming when a function already exists
at the exact same address in a non-global scope.

The 3 address-taken lines (`sub_2ca0` → `perimeter`, `sub_2d20` → `emit`,
`sub_401120` → `~runtime_error`) render the symbol's *base* name, because
`Funcdata::link_spacebase_symbol` binds the **unscoped** `name_for_global_varnode`
where the sibling `ActionNameVars` path uses `name_for_global_varnode_scoped` +
`kuna_qualify_global_name`. That gap is pre-existing and unrelated: on `main` the
same fixture already emits `{virtual override thunk}` in exactly that position and
`void Account::~Account(void)` as a function header. Recorded as a follow-up.

## Recompilability (the `::`-in-a-C-header question)

Measured, not assumed. **The header is bit-identical before and after**, and C
output is untouched on every axis:

| artifact | `::` | `~ident` | `{…}` | `gcc -fsyntax-only` errors |
|---|---|---|---|---|
| `fmt_x86_64` (C) `.c` + `.h` — before | 0 | 0 | 0 | 8 |
| `fmt_x86_64` (C) `.c` + `.h` — after | 0 | 0 | 0 | 8 |
| `probe_O0` (C++) `.h` — before | 122 | 22 | 3 | — |
| `probe_O0` (C++) `.h` — **after** | **122** | **22** | **3** | — |
| `probe_O0` (C++) `.c` — before | 246 | 44 | 6 | 316 |
| `probe_O0` (C++) `.c` — after | 410 | 93 | 6 | 407 |

A C++ project export already failed to compile as C in 316 places; nothing
recompilable regresses, and the header this PR was warned about is produced from the
canonical function names, which this change does not alter. The `.c` bodies gain
`::` at call sites — the point of the fix, and what IDA and Ghidra print. Since C
output is byte-identical, no C-binary recompile metric can move.

A sanitized identifier spelling for the export path is a genuinely separate feature
(it must rewrite `.h` *and* `.c` consistently, including the pre-existing
`Account::~Account` headers); filed as follow-up 1 in
`docs/features/cppcallnames/analysis.md`.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK — 675/675**, no re-pin |
| `make test-stages` | **PARITY OK — 394/394**, no re-pin |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |

## Speed

No option to flip, so `scripts.pipeline.timeit` (which times one binary off vs on)
does not apply — same as DIV-58. Arms are the pre-change `kuna` and this build,
interleaved, **minimum of 11 pairs** (the box is shared with concurrent agents, so
medians are contaminated: a first 5-pair pass reported +40.96% on `probe_O0` that the
11-pair minimum resolves to −0.46%).

| surface | binary | before | after | Δ |
|---|---|---|---|---|
| `decompile-all` | probe_O0 (C++, ~150 namespace scopes) | 358.1 ms | 356.5 ms | −0.46% |
| `decompile-all` | cpp_noreturn (C++) | 111.7 ms | 110.4 ms | −1.14% |
| `decompile-all` | eh_lsda (C++) | 118.9 ms | 117.6 ms | −1.12% |
| `functions` | probe_O0 (load + commit only) | 149.2 ms | 142.8 ms | −4.29% |
| `decompile-all` | /usr/bin/ls (C, stripped) | 31,445 ms | 31,030 ms | −1.32% |
| `functions` | /bin/grep (C, stripped) | 617.2 ms | 616.8 ms | −0.06% |

Worst delta **−0.46%**, budget 5%. The added work is one probe per discovered entry;
`find_function_across_scopes` still hits the global scope first and walks the rest
only on a miss.

## Tests

`decompiler/crates/kuna-console/tests/verify_cppcallnames.rs` — real engine over the
already-vendored `cpp_noreturn_x86_64`, discovery passes enabled in the live-CLI
order (`option` before `read symbols`):

1. the namespaced callee renders qualified, and no `sub_401070` shadows it;
2. `load addr 0x401070` names the function from the symbol table;
3. control — discovery still names genuinely-unknown finds `sub_<addr>`, and an
   explicit `load addr <vma> <name>` still wins.

Ablated against the pre-fix sources (files copied aside — never `git stash`, it is
shared across worktrees): **1 passed / 2 failed**, both defect gates failing and the
control passing. A gate that passes either way proves nothing.

No `tests/stages/*.xml`: structurally impossible for the primary defect (see "Why
nothing caught it"), so no corpus-count bump and no stages-baseline re-record.

### One existing test moved, and it is worth naming

`verify_multiformat_dwarf` used `load addr` — with the DWARF pass still **on** — as
its "no-DWARF-name baseline", asserting `sub_140001550` / `sub_0`. That only held
because `load addr` ignored the symbol table: the test was pinning the bug. Its
baseline arm now turns the pass off (`option dwarf off`), which is what it always
meant, and the claim gets stronger:

* **PE** — nothing else names `0x140001550` (the COFF FUNC entry is
  `--strip-symbol`-removed), so `sub_140001550` is unchanged and now actually tests
  the absence of DWARF rather than the absence of a lookup.
* **Mach-O** — with DWARF off the `--redefine-sym`ed symtab entry `_l0` remains, so
  the baseline asserts `_l0`. "The symtab says `_l0` here, not `first_byte`" is a
  sharper statement of *DWARF-only* than a placeholder was.

Both DWARF-recovery assertions are untouched. This was the only test in the
workspace that moved.

## Files

| file | change |
|---|---|
| `kuna-console/src/engine.rs` | discovered-entry commit arm: cross-scope idempotence probe |
| `kuna-console/src/ifacedecomp.rs` | `IfcAddrrangeLoad`: prefer the installed qualified name |
| `kuna-console/tests/verify_cppcallnames.rs` | **new** — 3 tests |
| `docs/spec/00-overview.md` | §0.1 commit idempotence is cross-scope; §0.2 both selection forms name identically |
| `docs/history.md` | DIV-59 |
| `docs/features/cppcallnames/` | `analysis.md`, `plan.md`, `record.json` |
