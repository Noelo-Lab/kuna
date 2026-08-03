## What

A Windows PE reaches its imports through an **Import Address Table slot**, which is
*data*. `call dword ptr [slot]` therefore lifts to a `CALLIND` whose target is the
contents of a global — unlike an ELF PLT stub or a Mach-O `__stubs` entry, which are
code and resolve as ordinary direct calls. The only pass that resolves such a target,
`ActionDeindirect` (`coreaction.cc:1235`), gates its `queryExternalRefFunction` arm on
`isPersist() && isExternalRef()`, and **`Varnode::externref` was set nowhere in the
workspace**: Ghidra sets it from an `ExternRefSymbol` (`Scope::addExternalRef`) that
kuna's port never carried.

So every Windows API call in every PE stayed an unnamed `(*dat_4112c4)(0)` — no name,
no prototype and, the damaging part, **no no-return flow effect**, so a function
ending in `ExitThread`/`ExitProcess` kept its fall-through and swallowed the whole of
the next function.

Closes decbench case `O0-mydoom-mydoom-mmsender_th` (IDA, Ghidra and angr all GED 0;
kuna 37) and its un-triaged ida-pool sibling `O0-mydoom-mydoom-sync_visual_th`.

## Before / after — `mmsender_th` (mydoom.exe @ `0x4041e2`)

```c
// BEFORE (option peimportcall off) — 50 lines, 4 ifs, and a `while` loop the
// function does not have: everything from `v7 = ...0x4102e0;` down belongs to the
// NEXT function, which kuna separately emits as sub_40424c, statement for statement.
void sub_4041e2(int4 a0)
{
  ...
  if (dat_4102e4) { LOCK(); UNLOCK(); }
  (*dat_4112c4)(0);                         // <- this is ExitThread(0)
  v7 = (unsigned int *)0x4102e0;
  v3 = dat_4102e0;
  while (v7 = v3, v8 = v7 != NULL, v8) {
    v1 = (char *)&v7[2];
    if (*v1 != '\x02') v3 = (unsigned int *)*v7;
    else {
      v4 = (*dat_411324)();                 // GetTickCount
      ...
      v6 = (*dat_411308)();                 // GetProcessHeap
      (*dat_41133c)(v6,0,v7);               // HeapFree
    }
  }
  return;
}
```

```c
// AFTER (option peimportcall on, the shipped default) — exactly IDA's and Ghidra's
// shape: 2 ifs, no loop, ends at the no-return import.
void sub_4041e2(int4 a0)
{
  LOCK();
  UNLOCK();
  if (a0) {
    *(char *)(a0 + 8) = 1;
    sub_4040eb(a0);
    *(char *)(a0 + 8) = 2;
  }
  if (dat_4102e4) {
    LOCK();
    UNLOCK();
  }
  ExitThread(0); // no-return
}
```

The stored IDA pane for reference (GED 0): `_InterlockedExchangeAdd(&dword_4102E4,
1u);` … `if (lpThreadParameter) { … sub_4040EB(…); … }` … `if (dword_4102E4)
_InterlockedExchangeAdd(…);` … `ExitThread(0);`. The full captured side-by-side,
including the `sub_40424c` body kuna had been absorbing, is
`docs/features/peimportcall/before-after.txt`.

## Mechanism (three parts, one gate)

1. **`externref` over the IAT slots.**
   `kuna-analysis/src/loader/kuna_peimportcall.rs (PeImportCallPass)` reports one
   `[slot, slot+ptr)` range per import-descriptor entry (`pe_iat::resolve_pe_import_slots`,
   reusing the existing INT/IAT lockstep walk), and the commit ORs
   `Varnode::externref` over each in the symbol-table property map — the same
   `Database::set_property_range` the loader's read-only section ranges already use.
   `Scope::queryProperties` folds the property map into every global Varnode covering
   the range, so the slot read now carries `persist|externref` and `ActionDeindirect`
   resolves it against the `FunctionSymbol` `pe_iat` already registered at that same
   slot VA. No second symbol is mapped: kuna's `query_function` keys on the Varnode's
   own address where upstream indirects through `ExternRefSymbol::refaddr`, and the
   import FunctionSymbol's map entry is 1 byte (`min_funcsymbol_size`), so a
   pointer-width slot read never finds it as a container and always takes the property
   branch. `externref` is read in exactly one place in the whole workspace
   (`p9_emit/coreaction_render.rs:1177`), so the blast radius is bounded by construction.

2. **The callee's no-return flag reaches the deindirect** — a *separate* port gap,
   see the section below. `Database::build_global_query` now carries
   `SymbolKind::Function { no_return }` into `GlobalEntry::func_no_return`, and
   `Architecture::query_function` puts it on the prototype it hands `ActionDeindirect`.

3. **Upstream's PE-only no-return API list.**
   `Ghidra/Features/Base/data/PEFunctionsThatDoNotReturn` is vendored verbatim at
   `decompiler/crates/kuna-analysis/data/PEFunctionsThatDoNotReturn` and matched against
   a **PE/COFF object only**. kuna merges the PE and Mach-O name sets into one shipped
   file (`PeMacFunctionsThatDoNotReturn`, 8 shared CRT names) where upstream keeps them
   separate; rather than widen that merged file with `ExitProcess`/`ExitThread`/
   `KeBugCheck`/the MSVC-mangled `?_Xbad_alloc@std@@YAXXZ` family — Windows names a
   Mach-O would then be matched against — the Windows list stays its own file on its own
   format arm. (kuna's Mach-O list is still missing upstream's `MachOFunctionsThatDoNotReturn`
   names: a separate, out-of-scope gap, recorded in the bundle.)

Each part alone can change emitted C on *some* target, so all three sit behind the one
option. `p9_emit/coreaction_render.rs` (`ActionDeindirect`) and `p4_calls/fspec.rs`
(`FuncCallSpecs::deindirect`) are **unchanged consumers**, verified against.

**The filed mechanism was refuted before any code was written.** The triage record
proposed a new P2 pass in `p2_lift/flow.rs` matching a CALLIND through a global; a
pre-pipeline IR dump (`load addr` + `print raw`, no `decompile`) shows the target is a
free unique temp and every varnode is `(free)` at that seam — raw flow IR carries no
def-use links, so the backward walk it needs is unavailable. Nothing new is added in
`p2_lift`.

## Second, separate finding: `query_function` never reported a callee's no-return

This one was **not** in the brief and is worth reading on its own, because it is a
general port gap that happens to be load-bearing here.

`Database::build_global_query` flattens the global scope into a read-only `GlobalQuery`
snapshot for the per-function `ArchHandle`. It copied `SymbolKind::Function`'s
`inject_id` but **not** its `no_return`, so `Architecture::query_function` — the only
caller of which is `ActionDeindirect` — returned a prototype with `is_no_return() ==
false` for *every* callee, including one the no-return analysis pass had explicitly
flagged. Upstream never had to carry the bit: `Scope::queryFunction` returns the
callee's live `Funcdata`, whose `getFuncProto().isNoReturn()` is simply true.

The consequence is not "the call is missing a comment". `FuncCallSpecs::deindirect`
branches on it:

```
if (!newproto_no_return && !newproto_inline) {   // fspec.cc:5448
    ...lateRestriction... -> commit the merged prototype and RETURN (no restart)
}
data.set_restart_pending(true);                  // only reached for no-return/inline
```

With the flag lost, deindirect always took the `lateRestriction` success path, rewrote
the `CALLIND` to a `CALL` **in place**, and returned without scheduling a restart — so
the flow, which had already walked past the call when the function was first followed,
was never recomputed. `decomp_dbg`'s `restarts` reported *"No restart events recorded
for sub_4041e2"* even after a successful deindirect.

Proved by A/B rather than inferred: with `externref` forced on and this bit still
missing, the call renders `ExitThread(0)` and **the entire absorbed body is still
there** — and so it is even under the explicit `option noreturn ExitThread` override
(the same counterfactual the triage record reported as evidence that "no name is ever
queried", which turns out to have had a second cause). Naming was necessary but not
sufficient; this is the half that fixes the boundary.

The carry is gated by the same option, because `query_function` is format-neutral: a
constant-folded indirect call on any target (e.g. the MIPS `$gp`/GOT const-range path)
could now resolve to a known no-return callee. Measured as inert on every ELF tested.

## Follow-up filed (campaign-wide, not this PR): decbench cannot rescore PE cases

`scripts.decbench.rescore` invokes `kuna decompile-all --addr 0x<address_hex>` using the
address stored in the mined pools. For a PE those pools store an **RVA** (`0x41e2`)
while kuna — like IDA and Ghidra — uses the image-based **VA** (`0x4041e2`), so every PE
case fails with `no fresh code: Unable to load 512 bytes at r0x000041e2` before any
comparison happens. This silently blocks GED rescoring for the whole PE half of the
corpus (mydoom and dexter), which is exactly the half this PR moves.

**Still present after the results-tree refresh and pool re-mine** — I re-checked on the
current pools and `address_hex` is still the RVA `0x41e2`, so this is not something the
refresh fixed. The two mydoom entries were corrected in a throwaway local edit to take
the numbers above and then reverted — **`docs/decbench/triage-queue*.json` are unmodified
by this PR** (`git status` on `docs/decbench/` is clean). The real fix (normalise the
pool address to the loader's base, or teach `rescore` to add the PE `ImageBase`) belongs
in the decbench tooling, not here.

## The option

`--option peimportcall on|off`, P1 / external-refinement, **default-ON (DIV-57)**,
`tier = transform`, `change_kind = correctness-fix`. PE/COFF only — the pass is
registered only for those formats *and* self-gates in `run`, so every other object
format's pass set is byte-identical (pinned by
`passes::tests::peimportcall_pass_is_pe_coff_gated`).

## Ablation and the default decision

| Ablation (option ON) | Speed | Ship as |
|---|---|---|
| **0 / 675** upstream assertions change | within budget (negative whole-binary) | **default-ON**, DIV-57 |

`make test` is **PARITY OK 675/675 with the flip and no re-pin and no per-test
opt-out**. `make test-stages` is **PARITY OK 386/386** (the delta is purely the 5 new
keys).

## Regression sweep — both PE binaries, whole-binary

`kuna decompile-all`, option OFF vs ON:

| | mydoom.exe | dexter.dll |
|---|---|---|
| functions | 156 → 156 | 109 → 109 |
| emitted lines | 5,247 → 4,768 | 3,422 → 3,115 |
| unnamed `(*dat_…)()` calls | **501 → 4** | **426 → 4** |
| distinct Win32 API names in the C | 0 → **71** | 0 → **90** |
| `// no-return` marks | 9 → 14 | 7 → 10 |
| `while` loops | 65 → 60 | 24 → 25 |
| `goto` | 23 → 23 | 20 → 19 |
| bodies: shrank / grew / renamed-same-length | 48 / 2 / 64 | 31 / 0 / 28 |

**927 unnamed Windows API call sites → 8.** No function is gained or lost in either
binary and gotos never increase. Beyond the five mydoom functions the triage named,
three dexter functions also stop overrunning: the largest single win is dexter
`Uninstall` **152 → 19 lines**, now 16 statements ending `ExitProcess(0); // no-return`.
Others: mydoom `sub_402a62` 169 → 64, `sub_402eda` 96 → 28, `sub_407107` 87 → 50,
`sub_40451c` 40 → 7, `sub_4024c5` 53 → 30; dexter `HttpMain` 118 → 91, `TrackSearch`
299 → 284, `ScanMemory` 88 → 72 (and it loses its one `goto`).

**Exactly two functions grow, both by one line**, and both for the same benign reason —
a named import now takes a typed argument through a temporary:
`a1 = (*dat_411464)(*(unsigned short *)(a0+a1)); a1 &= 0x3fff;` becomes
`v2 = *(unsigned short *)(a0+a1); a1 = ntohs(v2) & 0x3fff;`.

Shipped PE fixtures: `pe_imports_stripped.exe` 79 → 3 unnamed slot calls,
`pe_dwarf.exe` 50 → 0; `msvc_rtti_x64/x86.exe`, `pdb_min.exe`, `pdb_prog.exe` unchanged.

**Controls.** Option-ON output is byte-identical to option-OFF on every non-PE target
measured (coreutils `b2sum`, `mirai`, `tar` -O2 — same md5), and option-OFF is
byte-identical to the pre-change binary on both PEs (mydoom `49a1873b…`, dexter
`8dc4bd5b…`).

All four PE md5s were **re-measured on the final base `65543461`** and are bit-for-bit
the numbers above: neither #252 (which does change emitted C — it drops phantom
`.rodata` stores) nor #253 (ternary rendering) touches these witnesses, and the ELF
control (`b2sum`, ON == OFF, `06df920f…`) is likewise unchanged. Confirmed by
re-measurement, not from memory.

## Benchmark (GED)

| case | before | after | |
|---|---|---|---|
| `O0-mydoom-mydoom-mmsender_th` | 37 | **0** | perfect — matches IDA / Ghidra / angr |
| `O0-mydoom-mydoom-sync_visual_th` | 39 (recomputed; 34 recorded) | **0** | perfect — a second mined ida-pool row, closed by the same fix |

Both **re-measured on the refreshed decbench results tree and the re-mined pools**, at
the final rebase (`ged_kuna_commit 6d160156`), not quoted from the pre-refresh run --
the numbers are unchanged. See the follow-up section above: the RVA/VA bug is **still
present after the pool re-mine** (`address_hex` is still `0x41e2`), so the measurement
still needs a local address fix-up first.

## Speed

`scripts.pipeline.timeit`, target function, 15 repeats: **387.96 → 380.58 ms (-1.9%)**,
within the +5% budget. An earlier 7-repeat run on this (shared, loaded) box reported
+49% and did **not** reproduce — three re-runs at repeat=11 gave +0.67%, -0.99%,
-3.74% — so the interleaved form DIV-54 uses on this box is quoted alongside:

| measurement | off | on | delta |
|---|---|---|---|
| per-function interleaved, 11 pairs | 338.0 ms | 340.7 ms | +0.79% |
| whole-binary `decompile-all` mydoom, 7 pairs | 1,820 ms | 1,689 ms | **-7.23%** |
| whole-binary `decompile-all` dexter, 7 pairs | 1,552 ms | 1,520 ms | **-2.09%** |
| whole-binary ELF control `b2sum`, 5 pairs | 5,663 ms | 5,575 ms | -1.55% |

Whole-binary is negative because the truncated functions are less work.

## Test

`tests/stages/ghdec-peimportcall.xml` — a two-pass `decompilertest` over the real
`load file → read symbols → decompile` path on a new 6 KB PE fixture
(`pe_noreturn_import.exe`, built with MinGW-w64 in the `kuna-dev` container; source and
recipe committed beside it and documented in the fixtures README). It is purpose-built
for the one call shape `pe_imports.exe` lacks: a `__declspec(dllimport)`
`call [__imp_ExitProcess]` (the direct IAT-slot shape MSVC emits for every Win32 call),
with the next `.text` function deliberately containing a loop. Pass 1
(`option peimportcall off`) asserts the bug — `(*dat_140005038)(0xc)` and
`(*dat_140005038)()`, no `// no-return`. Pass 2 (`option peimportcall on`) asserts the
fix — `ExitProcess(0xc); // no-return`, `ExitProcess(); // no-return`, and exactly two
`// no-return` marks. 2 of the 5 assertions fail with the option off; 5/5 with it on.

Plus `passes::tests::peimportcall_pass_is_pe_coff_gated` (format gating) and the two
vendored-list guards in `kuna_peimportcall.rs` (the Win32 exits are present; the
returning APIs — `CreateThread`, `GetTickCount`, `TerminateProcess`, … — are not).

## Gates

```
make test        -> datatests: 675/675 assertions passed   |  PARITY OK
make test-stages -> datatests: 386/386 assertions passed   |  PARITY OK
make rust-test   -> green (workspace suite)
make check-spec  -> check-spec OK (lenient mode) / check-spec OK (strict mode)
kuna catalog --check -> catalog OK: documents exactly the registered kuna options
```

All five re-run on the final base `65543461` (after #252/DIV-55 and #253/DIV-56).
Shared counters this PR moves: `kuna-base/src/xml.rs` corpus count 189 -> **190**,
settables 86 -> **87**, transform tier 40 -> 41, live-value 28 -> 29 (`itecondlist`
declares no `live_field`, so only this PR moves that last one).
`tests/fixtures/phase_catalog.json` was **re-captured** on the rebased tree rather than
carried forward, and `docs/baseline-stages.json` **re-recorded**, never hand-merged
(386 keys, `data_footer [386, 386]`).

Two counters git resolved *without* a conflict were nevertheless wrong and were fixed by
hand — worth flagging for the next PR in this queue: `catalog_bytecompat.rs` silently
kept `86` because both sides had made the identical `85 -> 86` edit, and my own
pre-emptive `with_live` bump to `30` was one too many. Both were caught only by running
the suite, not by the merge.

## Docs

- `docs/spec/01-program-prep.md` — new prose in §import resolution: why naming a
  pointer slot is not enough to bind a call *through* it, the property-map carrier, and
  the `query_function` no-return carry.
- `docs/history.md` — **DIV-57**.
- `docs/options.md` regenerated; `tests/stages/README.md` and the fixtures README gain
  their rows; `docs/features/peimportcall/` bundle (`analysis.md`, `plan.md`,
  `before-after.txt`, `record.json`, `pr_body.md`).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
