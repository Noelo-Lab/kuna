---
case_id: O0-mydoom-mydoom-mmsender_th
pool: ida
group_id: mydoom::mmsender_th
status: feature-candidate
tier: M
margin: 37
fresh_verdict: reproduces on today's build in BOTH modes — the function body runs straight past `ExitThread(0)` and swallows the whole of the next function (which kuna separately emits as `sub_40424c`, byte-identical); the call renders as `(*dat_4112c4)(0)` because kuna never binds a `call [IAT slot]` to the import symbol it already resolved
option_closing: null
feature_slug: peimportcall
scope: small
confidence: high
---

## Side-by-side

**IDA (stored)** — 14 LOC, 2 `if`s, no loop:

```c
void mmsender_th(char *lpThreadParameter)
{
  _InterlockedExchangeAdd(&dword_4102E4, 1u);
  if ( lpThreadParameter ) {
    lpThreadParameter[8] = 1;
    sub_4040EB((int)lpThreadParameter);
    lpThreadParameter[8] = 2;
  }
  if ( dword_4102E4 )
    _InterlockedExchangeAdd(&dword_4102E4, 0xFFFFFFFF);
  ExitThread(0);
}
```

Ghidra's pane is the same 14 statements and ends with
`/* WARNING: Subroutine does not return */ ExitThread(0);`. angr, ghidra and phoenix
all score GED 0 here; kuna scores 37. Bucket: **kuna-specific**.

**kuna (fresh, today's build, no flags — and identical under `--mode reliable`)**:

```c
void sub_4041e2(int4 a0)
{
  ...
  if (dat_4102e4) { LOCK(); UNLOCK(); }
  (*dat_4112c4)(0);                       // <-- this is ExitThread(0)
  v7 = (unsigned int *)0x4102e0;          // <-- everything below belongs to
  v3 = dat_4102e0;                        //     the NEXT function
  while (v7 = v3, v8 = v7 != NULL, v8) {
    v1 = (char *)&v7[2];
    if (*v1 != '\x02')
      v3 = (unsigned int *)*v7;
    else {
      v4 = (*dat_411324)();
      v2 = &v7[1];
      v5 = (uint4)(v4 - *v2) / 1000;
      if ((-5 <= (int4)v5) && (v5 <= 0x1c20)) v3 = (unsigned int *)*v7;
      else { ... (*dat_411308)(); (*dat_41133c)(v6,0,v7); }
    }
  }
  return;
}
```

50 LOC, 4 `if`s and **1 loop that does not exist in this function**.

**The proof that this is an overrun, not a discovery difference**: kuna's *own*
function list contains an entry at `0x40424c`, and `kuna decompile-all` emits it
separately as `sub_40424c` — whose body is the trailing `while` loop above,
statement for statement. kuna emits the same code twice.

```c
void sub_40424c(void)
{
  v7 = (unsigned int *)0x4102e0;
  v3 = dat_4102e0;
  while (v7 = v3, v8 = v7 != NULL, v8) { ... }      // identical text
  return;
}
```

**Option sweep**: `readonly on`, `noreturn_disc on`, `listing on` — no change.
`--mode reliable` — no change. And the decisive counterfactual:
`--option noreturn ExitThread` (the explicit per-name override) **also changes
nothing**, because no name is ever queried for this call site.

## Source

mydoom is a leaked-source worm; the corpus ships no `.i` for it
(`~/github/decbench/results/full_run/O0/mydoom/compiled/` has the PE and the rival
artifacts only, and `mydoom.exe` is a PE with `do-not-execute` labels). The
source-side ground truth used here is therefore the decbench-recorded source CFG for
`mmsender_th`: **5 nodes / 6 edges, non-degenerate** — a body with two `if`s and no
loop, which is exactly IDA's and Ghidra's shape and is not compatible with kuna's
extra `while`. The function is a thread entry point: bump a refcount, run the worker,
drop the refcount, `ExitThread(0)`. The trailing loop kuna appends walks a linked
list at `0x4102e0` calling `GetTickCount`/`GetProcessHeap`/`HeapFree` — a different
routine's body.

## Analysis

### Symptom (one, named)

**Function-boundary overrun past a no-return Windows import.** Flow does not stop at
`call dword ptr [ExitThread]`, so the decompiled function absorbs the following
function's entire body.

### Root cause (instrumented, not read)

kuna's PE loader **does** resolve the import table. `kuna functions mydoom.exe --json`
lists 149 named symbols, including exactly the right one:

```
0x4112c0 ExitProcess
0x4112c4 ExitThread          <-- the slot the call reads
0x411308 GetProcessHeap
0x411324 GetTickCount
0x41133c HeapFree
```

So the name is in the symbol table at the slot VA. The decompiler never asks for it.
`ArchFlowEnv::query_call` / `query_call_no_return`
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs:92`) are keyed on a
**direct** call's entry address. The post-decompile IR for this call site
(`decomp_dbg` → `print raw`) is:

```
0x0040424a:80:  callind r0x004112c4(0x00404220:22b)(#0x0)
```

— a `CALLIND` whose target is the **contents** of the global at `0x4112c4` (an
unmodified read of the IAT slot, carried as an input Varnode). There is no entry
address to query, so no name, no prototype, and no no-return flow effect. The call
keeps its fall-through and flow walks into the next function.

`p2_lift`'s module doc for the PE import path (`kuna-analysis/src/loader/pe_iat.rs`)
says the slot-VA naming works because "the decompiler constant-folds the `[slot]`
load (the IAT lives in a read-only `.idata` page)". On this corpus **that primary path
is inert**: `--option readonly on` changes nothing, and folding would be wrong anyway
— the on-disk IAT slot holds the RVA of the `IMAGE_IMPORT_BY_NAME` entry, not a
function address. Only the module's *fallback* (case 2: naming `FF 25` thunk veneers
so a **direct** `call thunk` resolves) actually fires.

That split is visible inside a single function. `sub_4013e0` in the same binary:

```c
  v4 = (unsigned int *)malloc(dat_41001c * 4 + 4);   // msvcrt, via named thunk
  memcpy(v5,*v8,v1);                                 // named
  _cexit();      // no-return                        // named AND no-return honoured
  exit(dat_410010); // no-return
  ...
  dat_4102a0 = (*dat_411360)(sub_40a420);            // kernel32, via `call [IAT]`
  if (dat_40d464) (*dat_40d464)(0,2,0);              // kernel32, via `call [IAT]`
```

Same function, same binary: the thunked imports get names, prototypes and no-return;
the slot-called imports get `(*dat_XXXXXX)()`. kuna is internally inconsistent, which
is the same shape of evidence that carried `jumptable-callother-inject` in round 1.

**Second, subordinate gap**: even once the call is bound,
`decompiler/crates/kuna-analysis/data/PeMacFunctionsThatDoNotReturn` lists only
`exit / quick_exit / abort / fastfail / invoke_watson /
invalid_parameter_noinfo_noreturn / terminate / abort_handler4`. Ghidra additionally
ships the Win32 API names (`ExitProcess`, `ExitThread`, …). Both halves are needed;
the binding is the blocker (proved by the `--option noreturn ExitThread`
counterfactual above).

### Owning phase

**P2** — Flow & Op-Graph Recovery (`decompiler/crates/kuna-decomp/src/p2_lift/`).
The flow decision "does this call fall through" is made there, at the same seam
`kuna_tailcalljump` / `kuna_noreturn_externmatch` already hook. The no-return name
list is a P1 (`kuna-analysis`) data file.

### Correctness value, beyond GED

- **Wrong function extent** — the emitted C for `mmsender_th` contains a loop the
  function does not have, and the same code is emitted twice in one project export.
- **Unnamed API calls with guessed argument lists** — `(*dat_4112c4)(0)` has no
  prototype, so arguments are recovered by trial rather than from the API signature.
  In `sub_4041e2` alone, four call sites lose their names.

### Metric-artifact check

Not an artifact. Source CFG 5 nodes / 6 edges, `degenerate_source: false`,
`approximated: false`, `artifact_suspect: false`. Three other decompilers score 0.

### Breadth

`kuna decompile-all --json` on the two PE projects in the corpus:

| binary | fns emitted | `(*dat_…)()` IAT-slot calls | fns overrunning a Win32 exit |
|---|---|---|---|
| mydoom `mydoom.exe` | 156 | **501** | **5** |
| dexter `dexter.dll` | 110 | **426** | (not measured) |

**927 unnamed Windows API call sites across two binaries**, and zero Win32 API names
anywhere in either pane (`ExitThread`, `ExitProcess`, `CreateThread`, `GetTickCount`,
`HeapFree` — all 0 occurrences in the emitted C, all present in the symbol table).

The five mydoom functions whose body continues past a no-return exit:

```
sub_402a62 calls ExitThread  : 21 residual statement-lines after
sub_402eda calls ExitProcess : 53 residual statement-lines after
sub_4041e2 calls ExitThread  : 18 residual statement-lines after   <-- this case
sub_40451c calls ExitThread  : 18 residual statement-lines after
sub_407107 calls ExitThread  : 16 residual statement-lines after
```

`sub_402a62` is `mydoom::sync_visual_th` — **backlog-ida row 34, margin 34, its own
un-triaged group** — so one fix closes at least two ida-pool rows, and `sub_402eda`
(53 residual lines) is the worst of the five.

### Siblings

`O2-noinline-mydoom-mydoom-mmsender_th` (margin 8, `artifact_suspect`) is the same
function at -O2; the small margin there means the overrun costs less once the
following function is inlined away. The real siblings are the four other mydoom
functions listed above.

## Proposed fix

**Slug `peimportcall`, P2, one module** — a new
`decompiler/crates/kuna-decomp/src/p2_lift/kuna_peimportcall.rs`, built on the
`kuna_tailcalljump` template (which already rewrites one flow-level call shape into
another at this seam).

Mechanism: match a `CALLIND` whose target Varnode is an **unmodified read of a global
at a constant address `A`** (the shape in the `print raw` dump above — in kuna's IR
the IAT slot read is a global Varnode, *not* a `LOAD` op, which is why an
`INT_ZEXT`/`LOAD`-shaped matcher would miss it), and for which
`FlowEnvironment::query_call(A)` resolves a function symbol (i.e. `A` is a registered
import slot). Rewrite the op to a direct `CALL` to `A`, so the existing machinery —
`query_call` (name), the callee prototype, and `query_call_no_return` (flow effect,
the artificial-halt path) — all apply unchanged. This is the PE analog of what
already works for an ELF PLT stub, where the slot happens to be code and the call
happens to be direct.

Owning files:
- `p2_lift/kuna_peimportcall.rs` (new): the matcher + rewrite.
- `p2_lift/flow.rs`: one hook at the CALLIND classification point.
- `p0_knowledge/options.rs` + `phases.toml`: the `peimportcall on|off` row
  (plus the hard-coded catalog counts listed in `kuna-adding-option-count-tests`).
- `kuna-analysis/data/PeMacFunctionsThatDoNotReturn`: add the Win32 no-return names
  Ghidra ships (`ExitProcess`, `ExitThread`, `TerminateProcess`, `FatalExit`, …).
  Data-only; no code change.

Risks to settle at build time:
- The callee address is in a **data** space with no decodable body. Direct calls to
  known functions are not decoded (only `inline` decodes), so this should be inert —
  but it must be checked, and the rewrite should decline if the address is inside the
  current function's flow range.
- A `call [reg]` that happens to load a slot value into a register first (two-step)
  is *not* matched by the constant-LOAD pattern; that is deliberate for a first cut.
- Guard on "the address has an import-origin FunctionSymbol", not merely "any
  symbol", so a genuine indirect call through a data pointer is untouched.

Stage test: `tests/stages/ghdec-peimportcall.xml` (no angr analog), two-pass — option
off = the overrun body, default = the function ends at the no-return call. Speed via
`scripts.pipeline.timeit`; benchmark delta via
`scripts.decbench.rescore --case O0-mydoom-mydoom-mmsender_th --siblings`, and it
should also be rescored against `O0-mydoom-mydoom-sync_visual_th`.

### One loose thread found while triaging (not this feature)

kuna prints the `lock`-prefixed read-modify-write as **`LOCK(); UNLOCK();` with the
body missing**:

```c
  LOCK();
  UNLOCK();                       // kuna
```
```c
  LOCK();
  DAT_004102e4 = DAT_004102e4 + 1;
  UNLOCK();                       // ghidra
```

The atomic increment is simply gone — twice in this one function — and it is gone in
the IR too, not just in the printer: block 0 of the post-decompile `print raw` dump is

```
0x004041fd:1a:  LOCK
0x004041fd:2b:  UNLOCK
```

with nothing between, and the later `ZF = r0x004102e4(i) != #0x0` reads the
**unincremented input** value. I did not determine whether the RMW is never lifted or
is lifted and then dropped, so this needs its own case rather than a guess. It is a
wrong-value defect independent of the boundary overrun, and it is the sort of item
round 1 ranked above metric wins.
