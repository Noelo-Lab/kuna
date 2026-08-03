---
case_id: O2-noinline-dexter-dexter-GetProcList
pool: novel
group_id: dexter::GetProcList (track record for `code_ptr` + `raw_reg`; dominant witness dexter::_entryPoint @ 0x69943550; 42 functions / 4 PE projects)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: The `code_ptr` survivors on i386 PE are not a cast, type or naming defect at all — they are the visible residue of a **stack-pointer model that drifts**. kuna guesses `extrapop = 4` for every `__stdcall` callee (upstream `StackSolver::build`'s fallback, `coreaction_stackptr.rs:317`), so at each Win32 import that pops N argument bytes the modeled ESP sinks N bytes below the true frame and never recovers. Two independent binary probes prove the chain end to end — zeroing only the compensating `sub $N,%esp` displacements turns `_entryPoint` from 265 lines / 46 locals / 24 `code *` / 31 argument-less API calls into 125 lines / 10 locals / 0 `code *` / every argument recovered, byte-identical frame to ghidra and ida. `raw_reg` is confirmed dead: 0 bare register identifiers in 1,031 PE functions.
option_closing: null
feature_slug: stdcallpop
scope: small
confidence: high
---

## What this record covers

The triage brief pointed at the `(code *)` / `(code **)` + `raw_reg` family, whose only
surviving witnesses after round 2 are i386 PE: `dexter.dll::_entryPoint` and its eight named
siblings, plus `minipig.exe::Infects`. The census called it "a P4/P6 stack-frame +
call-argument recovery failure … kuna's local frame extends BELOW the outgoing-argument
area, so the push cannot be dead-coded and the argument trials are rejected."

That description is the **effect**, stated as the cause. This record instruments the cause
one level up and finds a single, measurable defect that explains the frame, the dropped
arguments, the surviving return-address pushes and the `code *` casts at once.

---

## Side-by-side

### fresh kuna — today's build (`e38ffc31`), default `--mode auto` → `aggressive`

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ kuna decompile ~/github/decbench/results/full_run/O2-noinline/dexter/stripped/dexter.dll \
      --addr 0x69943550
```

```c
unsigned int sub_69943550(void) // ternary
{
  ...                                     // 46 declared locals, 24 of them `code *`
  unsigned int v24; // stack - 0x100      // <-- 0x44 bytes BELOW the real frame bottom
  ...
  code *v40; // stack - 0xc0
  code *v42; // stack - 0xb8

  v40 = 0x69943560;
  v18 = GetCommandLineA();
  if (v18) {
    v42 = (char *)s_69948160;
    v40 = 0x6994357b;
    if (StrStrA()) {                      // <-- both arguments dropped
      v42 = NULL;
      v41 = NULL;
      v40 = 0x699435a1;
      v19 = CreateMutexA();               // <-- three arguments dropped
      ...
      v35 = "urlmon.dll";
      v34 = (char *)0x69943770;
      LoadLibraryA();
      v34 = "wininet.dll";
      v33 = (code *)0x6994377c;           // <-- the return address of the NEXT call,
      LoadLibraryA();                     //     typed `code *` because the same slot
```

265 lines · 46 locals · 24 `code *` · **31 argument-less calls to Win32 imports** ·
**31 surviving return-address pushes** (52 in-range constants total).
Runtime 1.05 s.

### ghidra (stored pane, same stripped binary)

```c
void __cdecl _entryPoint(void)
{
  _SECURITY_ATTRIBUTES local_84;
  _MEMORY_BASIC_INFORMATION local_78;
  CHAR local_5c [76];

  lpFirst = GetCommandLineA();
  if ((lpFirst != (LPSTR)0x0) &&
     (pCVar2 = StrStrA(lpFirst,s_UpdateMutex__69948160), pCVar2 != (LPSTR)0x0)) {
    pvVar3 = CreateMutexA((LPSECURITY_ATTRIBUTES)0x0,0,lpFirst);
    ...
    LoadLibraryA("urlmon.dll");
    LoadLibraryA("wininet.dll");
```

### ida (stored pane)

```c
  struct _SECURITY_ATTRIBUTES FileMappingAttributes; // [esp+38h] [ebp-84h] BYREF
  struct _MEMORY_BASIC_INFORMATION Buffer;           // [esp+44h] [ebp-78h] BYREF
  CHAR Name[92];                                     // [esp+60h] [ebp-5Ch] BYREF
  ...
      hObject = CreateMutexA(0, 0, lpName);
```

Both rivals recover every argument **and** Windows types (`LPSECURITY_ATTRIBUTES`,
`_MEMORY_BASIC_INFORMATION`) — i.e. both applied a Win32 type archive
(Ghidra `windows_vs12_32.gdt`, IDA `.til`). Neither shows an algorithm kuna is missing;
they show *knowledge* kuna is missing. That is why this stayed in the NOVEL pool.

### stored kuna (2026-08-03 pinned build 9623dc27)

Identical frame and identical drift, pre-`peimportcall`, so the calls are still unnamed:

```c
  code *v24; // stack - 0x100   ...   code *v40; // stack - 0xc0
  v40 = (code *)0x69943560;
  v17 = (code *)(*dat_6994e350)();
```

The staleness is cosmetic (#254 named the imports). The defect is unchanged.

### PROBE — the same binary with the stack drift removed

```
$ # zero the immediate of every `sub $N,%esp` that follows a call in 0x69943550..0x69943cd8
$ #   (47 sites; `83 ec NN` -> `83 ec 00`)
$ kuna decompile dexter_sub0.dll --addr 0x69943550
```

```c
unsigned int sub_69943550(void) // ternary
{
  unsigned int v1;
  unsigned int v10; // stack - 0x74
  int4 v2; // eax
  unsigned int v3;
  unsigned int v4;
  char v5 [76];
  char v6 [4];
  unsigned int v7; // stack - 0x84
  unsigned int v8; // stack - 0x80
  unsigned int v9; // stack - 0x7c

  v2 = GetCommandLineA();
  if ((v2) && (StrStrA(v2,s_69948160))) {
    v3 = CreateMutexA(0,0,v2);
    _memset(v5,0,0x40);
    wsprintfA(v5,"%s%d",v2,GetCurrentProcessId());
    while( true ) {
      SetLastError(0);
      v4 = CreateMutexA(0,0,v5);
      if (GetLastError() == 0xb7) break;
      CloseHandle(v4);
      Sleep(1000);
    }
    ...
    LoadLibraryA("urlmon.dll");
    LoadLibraryA("wininet.dll");
```

**125 lines · 10 locals · 0 `code *` · 0 leaked return addresses · every argument
recovered.** The recovered frame is `-0x84` / `-0x78` / `[76]` — byte-for-byte ghidra's
`local_84` / `local_78` / `local_5c[76]` and ida's `ebp-84h` / `ebp-78h` / `ebp-5Ch`.
Ghidra's pane is 123 lines; kuna's probe pane is 125.

---

## Source

`~/github/decbench/results/full_run/O2-noinline/dexter/compiled/dexter.dll-POSGrabber.i:78965`
(`#line` noise stripped):

```c
void _entryPoint() {
  BYTE *pCommandLine;  DWORD CSIDL;  MEMORY_BASIC_INFORMATION MBI;
  char UpdateMutexString[64];  HANDLE hUpdateMutexOne,hUpdateMutexTwo,hProcess;
  SECURITY_ATTRIBUTES sa;

  pCommandLine = GetCommandLineA();
  if (pCommandLine != NULL) {
    if (StrStrA(pCommandLine, UpdateMutexMark) != NULL) {
      hUpdateMutexOne = CreateMutexA(NULL, 0, pCommandLine);
      _memset(UpdateMutexString, 0x00, sizeof(UpdateMutexString));
      wsprintfA(UpdateMutexString, "%s%d", pCommandLine, GetCurrentProcessId());
      while (1) {
        SetLastError(0l);
        hUpdateMutexTwo = CreateMutexA(NULL, 0, UpdateMutexString);
        if (GetLastError() == 183l) { ... }
        CloseHandle(hUpdateMutexTwo);
        Sleep(1000);
      }
```

Four source locals (`sa` 12 B, `MBI` 0x1c, `UpdateMutexString[64]`, plus scalars) — a frame
of 0x84 bytes below the outgoing-argument area, exactly what the probe recovers and exactly
what kuna's default run does **not**: it declares 46 locals reaching 0x100, i.e. 0x44 bytes
of pure fiction below the real frame. Every call in the source has arguments.

---

## Analysis

### The one symptom

> **The modeled stack pointer drifts N bytes below the true frame at every `__stdcall`
> call whose callee pops N bytes of arguments, and never recovers.**

Measured directly from the return-address push offsets in `print raw`
(`decomp_dbg`, `load addr 0x69943550; decompile; print raw`):

| call | true ESP | kuna's ESP | drift |
|---|---:|---:|---:|
| `GetCommandLineA` @0x6994355a (0 args) | -0xbc | -0xbc | 0 |
| `StrStrA` @0x69943575 (2 args) | -0xbc | -0xbc | 0 |
| `CreateMutexA` @0x6994359f (3 args) | -0xbc | -0xbc | 0 |
| `_memset` @0x699435bf (internal, cdecl) | -0xbc | **-0xc8** | **-0xc** |
| `SetLastError` @0x6994360f (1 arg) | -0xbc | -0xc8 | -0xc |
| `CreateMutexA` @0x6994362b | -0xbc | **-0xd0** | **-0x10** |
| … | | | |
| `HeapAlloc` @0x69943828 | -0xbc | **-0xfc** | **-0x40** |

Each step down equals exactly the immediate of the preceding `sub $N,%esp` — GCC's
`-maccumulate-outgoing-args` compensation that undoes the `__stdcall` callee's pop:

```
69943568: movl $0x69948160,0x4(%esp)     ; arg2
69943572: mov  %eax,(%esp)               ; arg1
69943575: call *0x6994e478               ; StrStrA — __stdcall, `ret 8`
6994357b: sub  $0x8,%esp                 ; put ESP back where GCC assumes it is
```

True: `esp_after = esp_before + N` (the callee popped `4 + N`), then `sub $N` restores
`esp_before`. kuna's model: `esp_after = esp_before` (extrapop 4), then `sub $N` →
`esp_before - N`. The residual **is** the drift.

I verified the correspondence against the source for all 18 stdcall call sites in
`_entryPoint`: `StrStrA` 2 args ↔ `sub $8`, `CreateMutexA` 3 ↔ `sub $0xc`,
`SetLastError`/`CloseHandle`/`Sleep`/`LoadLibraryA` 1 ↔ `sub $4`, `GetProcAddress` 2 ↔
`sub $8`, `HeapAlloc` 3 ↔ `sub $0xc`, `CreateEventA`/`VirtualAlloc` 4 ↔ `sub $0x10`,
`SHGetFolderPathW`/`MapViewOfFile` 5 ↔ `sub $0x14`, `CreateThread`/`CreateFileMappingA` 6 ↔
`sub $0x18`. The two genuinely `__cdecl` callees in the same function — `wsprintfA`
(varargs) and the internal `_memset` — carry **no** post-call `sub`, and kuna's ESP is
correct across both. 18/18, 0 false positives.

Corpus-wide the idiom is regular: **1,727** post-call `sub $N,%esp` sites across the eight
O2/O2-noinline PE binaries, `N` always a multiple of 4, `N ≤ 40` (10 arguments), zero
exceptions.

### Why the first three calls are right

`StackSolver::solve` propagates **hard** equations from the entry Varnode first, and only
then fills unknowns with the `rhs = 4` guesses. At a CFG merge whose two paths must agree
(here the `je 0x69943668` join that bypasses `StrStrA`/`CreateMutexA`), the MULTIEQUAL is a
hard equation, so the pop is *derived* rather than guessed. Everywhere no merge pins both
sides, the guess wins and the drift starts. This is not a mis-port — it is the algorithm
working as upstream specifies with the knowledge it has.

### The chain from drift to `code_ptr`

1. No frame pointer at O2/O2-noinline ⇒ **both** the locals and the outgoing-argument area
   are ESP-relative, so the drift moves every call's argument slots to a fresh offset.
2. `ScopeLocal::restructure` therefore carves the wandering argument area into ~35 named
   frame locals reaching `stack - 0x100` (P6 §6.2).
3. `check_input_trial_use` (`p4_calls/funcdata_callsite.rs:47`) can no longer accept the
   trials — `callee_pop` is false because `get_extra_pop()` is still 4, and ancestor
   realism fails on slots that now look like ordinary caller locals — so it zeroes each
   trial's CALL input and the arguments are dead-coded. 31 Win32 calls in this one function
   lose every argument.
4. The `call`'s own return-address push is not dead-codable (its slot is now a live named
   local), so it prints as `v40 = 0x6994357b;`.
5. Some of those slots also carry a real function pointer elsewhere in the merged
   HighVariable, so the slot's type is `code *` — and the return address, an address inside
   `.text`, is a perfectly plausible member of it. **The `(code *)` is honest; the frame
   underneath it is not.**

This is why no cast/type/naming option touches it, and why #249 (`funcptralign`) was inert:
i386 declares no `<funcptr align>`, and the defect is three phases upstream of casting.

### Why O0 is clean and O2 is not

At `-O0` GCC keeps `%ebp` as a frame pointer. The drift still happens on ESP, but every
real local is EBP-relative, so `ScopeLocal` is unaffected and the per-call argument slots
stay self-consistent relative to each call's own (drifted) ESP. Measured: `O0/dexter`
`_entryPoint` decompiles *perfectly today* — `CreateMutexA(0,0,v13)`,
`LoadLibraryA("urlmon.dll")`, `GetProcAddress(GetModuleHandleA("kernel32.dll"),…)`. The
whole defect lives in frame-pointer-less builds.

### Metric-artifact check

- Mined row `O2-noinline-dexter-dexter-GetProcList`: `approximated: false`,
  `degenerate_source: false`, `source_nodes: 7`, and **all six** decompilers tie at
  GED 8.0. Not an artifact — a genuine, universal tie.
- The dominant witness `_entryPoint` @O2-noinline is **in no pool at all**, at any base.
  That is the finding, not an omission: dropping a call's arguments does not change the
  CFG, so GED is structurally blind to it. 140 lines of fabricated stack traffic and 31
  calls missing every argument cost ~0 GED. Only the wart column (`code_ptr`) surfaced it,
  which is exactly what the NOVEL pool is for. Any future re-weighting of `code_ptr` (the
  prior record proposed `max(0, kuna − min_rival)`) must keep this class visible — under
  that formula dexter still scores 52-8 = 44 and stays at the top, so the proposal is safe.

### `raw_reg`, the second half of the track

Dead, and confirmed independently of the census: **0** bare/undeclared register identifiers
(`eax…esp`, `sp/lr/pc`, `r0…r15`) across all **1,031** functions of the twelve PE binaries,
on today's build. Nothing to fix; the pool column reads zero because it is zero.

---

## Breadth

Twelve PE binaries (`dexter.dll`, `minipig.exe`, `mydoom.exe`, `x0r-usb.exe` × O0/O2/
O2-noinline), whole-binary `kuna decompile-all --json` on today's build, then the same run
against a drift-neutralised copy of each (every `sub $N,%esp` that *immediately* follows a
call rewritten to `sub $0,%esp` — a conservative filter that never touches a prologue and
misses the sites where scheduling put a `mov` between the call and the `sub`):

| measure | today | drift removed | ghidra |
|---|---:|---:|---:|
| functions leaking a return address | **42** / 1031 | 7 | — |
| leaked return-address pushes | **448** | 74 | — |
| argument-less Win32 import call sites | **646** | 404 | 162 |
| `(code *)` / `(code **)` occurrences | **1189** | 1066 | — |
| total emitted lines | **34,103** | 32,472 | — |

Per level: at O0 nothing moves (0/0/0 leaks in dexter/minipig/x0r-usb, 6 in mydoom); the
whole delta is O2 and O2-noinline. `dexter` 100 → 0 leaks and 108 → 34 argument-less calls;
`minipig` 21 → 0 and 58 → 37; `mydoom` 60 → 9; `x0r-usb` 62 → 34 (its residual is the
scheduling case the conservative probe cannot reach, plus COM-vtable `call *0x44(%eax)`
sites, so **the true impact is larger than the probe's delta**).

Reach beyond this corpus is bounded by the compiler spec: the defect requires
`default_proto` with `extrapop="unknown"`, which only `x86win.cspec`, `x86-16.cspec`,
`x86borland.cspec`, `x86delphi.cspec`, `M16C_60.cspec`, `avr32a.cspec` and the Toy specs
declare. Every ELF and every x86-64 target has a fixed extrapop and cannot drift.

---

## Option sweep — nothing closes it

All **89** non-default settings of the 88-row catalog, run on
`dexter.dll --addr 0x69943550`, counting argument-less named calls:

- 87 settings → **31**, the baseline. `condfold on` and `condfold wide` swept explicitly:
  31 and 31. `--mode reliable` 31, `--mode aggressive` 31, `--mode fast` 31 (the binary is
  50,190 bytes, so `auto` = `aggressive`; mode is not the variable here).
- `peimportcall off` → 7, purely because the calls re-render as unnamed
  `(*dat_6994e478)()`; the leaked return-address pushes go **up**, 31 → 44. Not a fix.
- `callsitestackargs off` → 48 argument-less calls / 0 pushes — the documented pre-#229
  ablation: every stack trial is scored no-use, so the whole argument computation is
  dead-coded. Strictly worse, and the reason the census mistook it for a fix.
- The non-catalog console knob the spec references (`option extrapop`, `OptionExtraPop`) was
  swept too: `unknown` → 31, `4` → 41, `8` → 40, `12` → 40. It sets one global value; the
  problem needs a per-call-site one.

---

## Owning phase

**P6 — Variable & Storage Model.**
`decompiler/crates/kuna-decomp/src/p6_variables/coreaction_stackptr.rs`
(`ActionStackPtrFlow` / `StackSolver`, scheduled in `stackstall`), spec chapter
`docs/spec/06-variables-and-merge.md` §"The exception that proves the rule", which already
names the exact line: *"an underdetermined call contributes the guessed equation
`extrapop = 4`"*. The prose then describes the failure mode as *"an unsolvable system
leaves the INDIRECTs in place … the affected frame offsets never promote"*. **That is not
the failure mode observed here.** Here the solve *succeeds* — consistently, silently, and
wrongly. The chapter needs that second failure mode written down whatever the fix.

The consumers are P4 (`p4_calls/funcdata_callsite.rs::check_input_trial_use`, the
`callee_pop` branch that would have accepted the trials) and P6 §6.2
(`varmap.rs::ScopeLocal::restructure`, which carves the fictitious locals). Neither is where
the decision is made.

---

## Proposed fix

`feat/decbench-stdcallpop` — option `stdcallpop {on|off}`, P6 /
`stack-pointer-flow`, tier `transform`.

**Mechanism (one module).** New `p6_variables/kuna_stdcallpop.rs`, consulted from the single
site in `StackSolver::build` that currently pushes the fallback guess
(`coreaction_stackptr.rs:317`, `self.guess.push(StackEqn { var1: i, var2: idx, rhs: 4 })`):

> When a call's spacebase INDIRECT has `extrapop == EXTRAPOP_UNKNOWN` and its output
> Varnode's sole spacebase descendant is `INT_ADD(sp, -N)` with `N > 0` and `N % 4 == 0`,
> the caller is compensating a callee pop of `N` bytes. Emit the **hard** equation
> `rhs = stackshift + N` instead of the `rhs = 4` guess.

`analyze_extra_pop`'s existing L1 keystone then latches the solved value back onto the call
spec via `set_effective_extra_pop`, which is exactly what `check_input_trial_use`'s
`callee_pop` branch already reads — so no P4 change is needed. Everything downstream is
existing, exercised machinery.

**The probe is a positive control, not just a negative one.** Crediting the call with
`extrapop = 4 + N` while leaving `sub $N,%esp` in place is arithmetically identical to
leaving extrapop at 4 and making the `sub` displacement zero. That is precisely the
`sub $0x0` probe, and its output is byte-identical to the `nop` probe and matches ghidra's
frame exactly. So the fix's *output* is already measured — 265 → 125 lines, 46 → 10 locals,
24 → 0 `code *`, 31 → 0 leaked return addresses on the witness; the corpus table above for
the rest.

**Owning files.**
`decompiler/crates/kuna-decomp/src/p6_variables/kuna_stdcallpop.rs` (new),
`p6_variables/coreaction_stackptr.rs` (one call site),
`p0_knowledge/options.rs` + `phases.toml` (option row, `tier` + `symptoms`),
`docs/spec/06-variables-and-merge.md` (the second failure mode + the new equation),
`tests/stages/ghdec-stdcallpop.xml`, `docs/options.md` regen, plus the hard-coded catalog
counts (`kuna_phases/tests.rs`, `tests/catalog_bytecompat.rs`,
`tests/stages/kuna-catalog.xml`, `kuna-base/src/xml.rs` corpus count,
`docs/baseline-stages.json` re-record).

**Risks — and the "would this produce WRONG output?" axis.**

1. **A post-call `sub $N,%esp` that is a genuine frame extension** (an `alloca`, or a
   dynamically sized outgoing area) would be mis-credited to the callee and shift the frame
   the *other* way. The sole-descendant requirement and the `N % 4` filter narrow it, but
   this is the failure this feature can actually cause and the audit must target it. The
   existing `check_clog` path in the same file handles `SP = SP + *(SP+k)` alloca and should
   be given precedence.
2. **Conflict with an already-derived solve.** `propagate` refuses to overwrite a set
   solution, so a contradictory hard equation is dropped silently rather than loudly. On
   this witness the merge-derived values (`StrStrA` 12, `CreateMutexA` 16) agree with the
   new equation, but an assertion/diagnostic on disagreement is worth having.
3. **Baseline movement.** Four corpus testcases run on `x86:LE:32:default:win*` —
   `tests/datatests/retstruct.xml`, `tests/datatests/statuscmp.xml`,
   `tests/stages/ghangr-optimized-memcpy-6301a9.xml`,
   `tests/stages/ghdec-spacebase-unnamed.xml`. Inspect each; do **not** re-pin.
   Everything else in both corpora is x86-64/gcc or ARM, where extrapop is fixed and the
   rule is provably inert.
4. **Audit set (rule 8).** All 1,031 PE functions before/after, plus the x86-64 and ARM
   control set to prove byte-identity off i386-Windows. Classify each hunk; the expected
   shapes are "arguments appear", "frame shrinks", "stack local disappears". Anything else
   is the alloca case.

**Alternative mechanism, deliberately not recommended as the first PR.** Extend
`kuna-analysis/src/analyzers/protos/mod.rs` (`libproto`, P1/external-refinement — already
the "kuna analog of Ghidra's ApplyDataArchiveAnalyzer") with a Win32 `__stdcall` signature
table. This is literally what ghidra and ida do, and it fixes types as well as arity. But it
is a data-curation task, incomplete by construction, and a single wrong arity corrupts a
frame; it also cannot reach `x0r-usb`'s COM-vtable `call *0x44(%eax)` sites, which no
archive covers and which the drift also breaks. Worth doing as its own PR *after*
`stdcallpop`, for the types.

**Speed.** The new test is a two-op def-use walk per unknown-extrapop call inside an
existing loop; expected well inside the 5% budget, but measure with
`scripts.pipeline.timeit` as usual. Whole-function runtime on the witness today is 1.05 s.

**GED.** Expect ~0. This is a readability/correctness fix that the campaign metric cannot
see (see *Metric-artifact check*); `rescore --case O2-noinline-dexter-dexter-GetProcList`
will report a flat delta and that is the correct outcome, not a failure. The evidence for
the PR is the argument-recovery and frame table above.

---

## Siblings

Same mechanism, confirmed on today's build: `dexter::Update` 0x699465a0,
`dexter::AddItem` 0x69943de0, `dexter::Infect` 0x69942940, `dexter::ExecCommands` 0x69946a30,
`dexter::BeginInjection` 0x69943320, `dexter::GetParentProcessId`, `dexter::GetOSVersion`,
`minipig::Infects` 0x4014e0 (`v29 = 0x4014ff; v33 = FindFirstFileA();`), plus 34 more
functions across `mydoom.exe` and `x0r-usb.exe` — 42 in total by the leaked-return-address
signature, 4.1% of the PE corpus and 6.2% of its O2 + O2-noinline half (41 of 665).

One sibling in the pool, `dexter::GetProcList` @0x69941910, additionally shows the
*mis-attributed* form rather than the *dropped* form — `v4 = (*v1)(dat_6994c948,8,v10)`
where the source says `HeapAlloc(hHeap, 8, size)` and `v1` is the
`CreateToolhelp32Snapshot` handle, plus `_memset(v5,0)` (2 args) and
`_memset(&v7,0,0x104,v11,v12,v13)` (6 args) against a 3-argument callee. It sits outside the
probed range so it is not covered by the measurement above; it should be re-checked against
the implemented fix before being filed separately.

---

## Artifacts

Read-only; no `.rs` file, branch or PR was touched and `make specs` was not run.
Scratchpad `…/scratchpad/censustrack/`: `ep.c` (fresh witness), `probe_ep.c` /
`nosub_ep.c` (the two probes, byte-identical), `ep_raw.txt` (`print raw`),
`sweep/` (89 option settings), `ex_*.c` (`option extrapop`), `pe/*.json` (12 whole-binary
runs, today's build), `probejson/*.json` (the same 12 drift-neutralised),
`dexter_sub0.dll` / `dexter_nosub.dll` / `dexter_keepfirst.dll` / `probe/*` (patched
copies), `r1s.so` (the i386-ELF control, which recovers every stack argument correctly).
