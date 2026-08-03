# `peimportcall` -- PE import calls are never bound, so function boundaries overrun no-return Windows APIs

Route B (decbench). Case `O0-mydoom-mydoom-mmsender_th`; triage record
`docs/decbench/triage/O0-mydoom-mydoom-mmsender_th.md`. IDA, Ghidra and angr all
score GED 0 on this function; kuna scored 37.

## The symptom

`mmsender_th` (kuna `sub_4041e2`) is a thread entry point: bump a refcount, run the
worker, drop the refcount, `ExitThread(0)`. IDA emits 14 lines, two `if`s and no
loop. kuna emitted 50 lines, four `if`s and **one loop the function does not have** --
because flow did not stop at `call dword ptr [ExitThread]`, so the body absorbed the
whole of the next function. kuna's own function list contains an entry at `0x40424c`
and `decompile-all` emits that body a second time as `sub_40424c`, statement for
statement: the same code twice in one project export.

The call itself rendered `(*dat_4112c4)(0)` -- an unnamed pointer -- even though
`kuna functions mydoom.exe` already lists `0x4112c4 ExitThread`.

## Root cause (instrumented, not read)

A `KUNA_DEBUG_DEINDIRECT` probe inside `ActionDeindirect::apply` on this function:

```
[deindirect] apply ncalls=5
[deindirect] callind target vn addr=0x4112c4 persist=true externref=false const=false qf=Some(("ExitThread", false))
[deindirect] callind target vn addr=0x411324 persist=true externref=false const=false qf=Some(("GetTickCount", false))
[deindirect] callind target vn addr=0x411308 persist=true externref=false const=false qf=Some(("GetProcessHeap", false))
[deindirect] callind target vn addr=0x41133c persist=true externref=false const=false qf=Some(("HeapFree", false))
```

Every piece was already in place except two bits.

**1. `externref = false`.** `ActionDeindirect` (`coreaction.cc:1235`) gates its
external-reference arm on `v.is_persist() && v.is_external_ref()`. `persist` is
painted; `varnode_flags::externref` is set **nowhere in the workspace**, because
Ghidra sets it from an `ExternRefSymbol` (`Scope::addExternalRef` / the console
`map externalref`), which is an `engine_unavailable` stub in kuna. So the arm was
dead on every PE, and the `is_constant()` arm never applies -- the target is a global
read, not a folded constant (`--option readonly on` changes nothing, and folding
would be wrong anyway: the on-disk IAT slot holds the RVA of the
`IMAGE_IMPORT_BY_NAME` entry, not a function address).

**2. `qf=(..., false)`.** `query_function` already resolves the *name* (the loader
registers the import `FunctionSymbol` at the slot VA), but the `no_return` half is
`false` for every callee: `Database::build_global_query` never copied
`SymbolKind::Function { no_return }` into `GlobalEntry`. Upstream `queryFunction`
returns the callee's live `Funcdata`, so its `getFuncProto().isNoReturn()` comes for
free; kuna's snapshot dropped it. Without it `FuncCallSpecs::deindirect` takes its
`lateRestriction` success path and returns **without scheduling a restart**, so the
CALLIND->CALL rewrite happens in place and the *flow* -- which had already walked past
the call -- is never recomputed. `decomp_dbg`'s `restarts` reports "No restart events
recorded" even after a successful deindirect. This was confirmed by an A/B hack: with
only `externref` forced on, the call renders `ExitThread(0)` but the overrun body is
still there, and even the explicit `option noreturn ExitThread` override changes
nothing.

Finding (2) was **not** in the brief and is a general port gap: `build_global_query`
copies a FunctionSymbol's `inject_id` but not its `no_return`, so `query_function` --
whose only caller is `ActionDeindirect` -- could never report a no-return callee on any
target, not just a PE. It is the half that actually fixes the boundary; naming the call
is necessary but not sufficient.

A subordinate third gap: kuna's shipped `PeMacFunctionsThatDoNotReturn` carries only
eight CRT names. The Windows API exits (`ExitProcess`, `ExitThread`,
`FreeLibraryAndExitThread`, `KeBugCheck`, `longjmp`, ...) live in upstream's separate
`PEFunctionsThatDoNotReturn`, which kuna never vendored.

## Why the filed mechanism was refuted

The triage record proposed a new P2 pass in `p2_lift/flow.rs` matching a CALLIND whose
target is a global read. A pre-pipeline IR dump (`load addr 0x4041e2` + `print raw`,
no `decompile`) shows that at the P2 flow seam the CALLIND target is a **free unique
temp** and every varnode is `(free)` -- raw flow IR carries no def-use links at all, so
the backward walk that mechanism needs is not merely unimplemented, it is unavailable.
The decision does not live in P2. It lives in the P0/P1 knowledge plane (which
addresses are external references) and at the deindirect seam (what the resolved
callee's prototype says).

## Owning phase

**P1 program-prep / external-refinement** -- `docs/spec/01-program-prep.md`. The new
pass is `decompiler/crates/kuna-analysis/src/loader/kuna_peimportcall.rs`; the two
ported-core anchors it feeds are `substrate/context.rs` (`GlobalEntry` /
`query_function`) and `p0_knowledge/database.rs` (`build_global_query`).
`p9_emit/coreaction_render.rs` (`ActionDeindirect`) and `p4_calls/fspec.rs`
(`FuncCallSpecs::deindirect`) are **unchanged consumers**, verified against.

## Breadth

Both PE projects in the decbench corpus, whole-binary:

| binary | functions | unnamed `(*dat_...)()` before -> after | functions overrunning a no-return API |
|---|---|---|---|
| mydoom `mydoom.exe` | 156 | 501 -> 4 | 5 |
| dexter `dexter.dll`  | 109 | 426 -> 4 | 3 |

927 unnamed Windows API call sites across two binaries, and zero Win32 API names in
either pane before the fix.
