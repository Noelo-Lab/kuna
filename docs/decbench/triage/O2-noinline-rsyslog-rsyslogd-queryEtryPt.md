---
case_id: O2-noinline-rsyslog-rsyslogd-queryEtryPt
pool: novel
group_id: rsyslog::queryEtryPt (12 module-entry-point siblings in one binary)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: reproduces on e38ffc31. The `(code *)` is NOT a type-propagation or cast-insertion defect — it is a P6 FORCED merge. kuna's own `mark_output_storage_addr_tied` (a "W4 ScopeLocal stand-in" in ActionMergeRequired) paints the whole return register `mapped|addrtied` whenever ANY same-register SSA version is a phi/indirect join, and `Merge::mergeAddrTied` then force-merges all 27 RAX versions into ONE HighVariable whose type representative is the most specific member. In queryEtryPt the join is the function-pointer table (`*pEtryPoint = modExit`), which never reaches a RETURN, so `code *` wins over the `rsRetVal` error codes and the function is declared `code *` returning `(code *)0xfffffc18`. Gating the paint on "the join must actually reach a RETURN" reproduces ghidra's shape exactly, keeps 675/675 datatests and 394/394 stages green, and is 9% FASTER.
option_closing: null
feature_slug: retjointie
scope: small
confidence: high
---

## Side-by-side

### fresh kuna — today's build (`e38ffc31`), `--mode auto` (= aggressive, 916 KiB binary)

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ ./decompiler/target/release/kuna decompile \
      ~/github/decbench/results/full_run/O2-noinline/rsyslog/stripped/rsyslogd --addr 0x20f60
code * sub_20f60(char *a0,unsigned long *a1) // return-dupe
{
  int4 v1; // eax
  code *v2; // rax

  if (!a0) {
    v2 = (code *)0xfffffc18;
    return v2;
  }
  if (a1) {
    *a1 = 0;
    v1 = strcmp(a0,"modExit");
    if (v1) { // branch-flip
      ...                                     (22 nested strcmp arms)
                                                r_dbgprintf("omfile.c","entry point '%s' not present in module\n",a0);
                                                v2 = (code *)0xfffffc14;
                                                return v2;
      ...
    else { v2 = sub_20d40; }
    *a1 = v2;
    v2 = NULL;
    return v2;
  }
  v2 = (code *)0xfffffc18;
  return v2;
}
```

151 lines, 24 `if`s, 0 gotos. One `code *v2; // rax` carries **both** the handler
address (`v2 = sub_20d40; *a1 = v2;`) and the integer return code
(`v2 = (code *)0xfffffc18; return v2;`).

### ghidra (stored) — note: a DIFFERENT instance, see *Metric-artifact check*

```c
undefined8 queryEtryPt(char *param_1,undefined8 *param_2)
{
  int iVar1;
  code *pcVar2;          // <- the handler
  undefined8 uVar3;      // <- the return value; SEPARATE variable

  if (param_1 == (char *)0x0) { return 0xfffffc18; }
  if (param_2 == (undefined8 *)0x0) { uVar3 = 0xfffffc18; }
  else {
    *param_2 = 0;
    iVar1 = strcmp(param_1,"modExit");
    if (iVar1 == 0) { pcVar2 = FUN_00127760; } else { ... }
    *param_2 = pcVar2;
    uVar3 = 0;
  }
  return uVar3;
}
```

### ida (stored)

```c
long long queryEtryPt(char *s1, long long (**a2)())
{
  long long (*v2)(); // rax
  if ( !s1 )  return 4294966296LL;      /* 0xfffffc18, printed as an integer */
  ...
}
```

### kuna with the proposed gate (measured, worktree build)

```c
unsigned long sub_20f60(char *a0,unsigned long *a1) // return-dupe
{
  code *v1; // rax

  if (!a0)
    return 0xfffffc18;
  if (a1) {
    *a1 = 0;
    if (strcmp(a0,"modExit")) { // branch-flip
      ...
                                                r_dbgprintf("omfile.c","entry point '%s' not present in module\n",a0);
                                                return 0xfffffc14;
      ...
    else { v1 = sub_20d40; }
    *a1 = v1;
    return 0;
  }
  return 0xfffffc18;
}
```

123 lines (was 151). Return type `unsigned long` (ghidra: `undefined8`; ida: `long long`),
handler still `code *v1` (ghidra: `code *pcVar2`), constants no longer cast.
Structurally identical to ghidra's pane and 13 lines shorter than it on the
address-matched instance (0x27790: kuna 42 lines vs ghidra 55).

## Source

`~/github/decbench/results/full_run/O2-noinline/rsyslog/compiled/rsyslogd-omfile.i:14175`

```c
static rsRetVal queryEtryPt(uchar *name, rsRetVal (**pEtryPoint)())
{
  rsRetVal iRet = RS_RET_OK;
  if ((name == NULL) || (pEtryPoint == NULL)) { return RS_RET_PARAM_ERROR; }
  *pEtryPoint = NULL;
  if     (!strcmp((char*)name,"modExit"))   { *pEtryPoint = modExit; }
  else if(!strcmp((char*)name,"modGetID"))  { *pEtryPoint = modGetID; }
  ...                                        /* 22 arms */
  if (iRet == RS_RET_OK)
    if (*pEtryPoint == NULL) {
      r_dbgprintf("omfile.c","entry point '%s' not present in module\n", name);
      iRet = RS_RET_MODULE_ENTRY_POINT_NOT_FOUND;
    }
  return iRet;
}
```

`rsRetVal` is an **enum/int**; `pEtryPoint` is a function-pointer out-parameter.
`0xfffffc18` = `RS_RET_PARAM_ERROR` (-1000), `0xfffffc14` = -1004.  kuna's
`code * queryEtryPt(...)` returning `(code *)0xfffffc18` is therefore wrong against
the source, not merely a rendering-convention difference.

## Analysis

### Symptom (one)

The function's **return register is force-merged with an unrelated same-register phi**,
so a single HighVariable has to carry both a `code *` and an integer, and the pointer
type wins — for the local declaration *and* for the function's declared return type.

### Root cause — instrumented, not read

`kuna decompile` shells out to `decomp_dbg`, and `break action` / `print high` /
`print C xml` are all `engine_unavailable` stubs, so the trace was taken by building an
instrumented `decomp_dbg` in a throwaway detached worktree (no branch, no main-tree edit).

**Step 1 — the SSA is clean.** `print raw` on `sub_20f60` shows the two roles as
disjoint varnodes:

```
0x00020fa1:2ed:  RAX(2ed) = RAX(0x20f9a:36) ? RAX(0x2118f:193) ? ... (22-way MULTIEQUAL of function entries)
0x00020fa1:38:   *(ram,RSI(i)) = RAX(2ed)            <- the join feeds the STORE, and only the STORE
0x00020fa6:2ee:  RAX(2ee) = #0x0                     -+
0x00020fa6:44f:  RAX(44f) = #0xfffffc14               |  four independent return values
0x00020fa6:451:  RAX(451) = #0xfffffc18               |
0x00021260:1d1:  RAX(1d1) = #0xfffffc18              -+
```

The join (`2ed`) has **no path to any `return`**; the returned varnodes are four
separate constants.

**Step 2 — which pass merges them.** Instrumenting `Merge::merge` with a per-pass label
(`p6_variables/merge.rs`, driven from `coreaction_cleanup.rs`) attributes every one of
these merges to **`mergeAddrTied`** — the *required*, non-speculative pass — not to
`mergeByDatatype` / `mergeAdjacent`:

```
[MERGE mergeAddrTied spec=false] H173[1]{vn#141@0x20f9a} ty=/m9/sz8  <=  H174[1]{vn#1961@0x20fa1} ty=/m9/sz8
[MERGE mergeAddrTied spec=false] H173[2]{...}            ty=/m9/sz8  <=  H175[1]{vn#1984@0x20fa6} ty=xunknown8/m15/sz8
[MERGE mergeAddrTied spec=false] H173[3]{...}            ty=/m9/sz8  <=  H176[1]{vn#3470@0x20fa6} ty=xunknown8/m15/sz8
... 27 members in total
```

The existing `KUNA_DBG_MERGE` hook confirms the range:

```
[mergeAddrTied] sub n=22 addr=0x0 size=4 -> OK     (EAX)
[mergeAddrTied] sub n=27 addr=0x0 size=8 -> OK     (RAX)
[mergeAddrTied] groupWith off=0 h2=HighVariableId(173) h1=HighVariableId(151)
```

**Step 3 — why a *register* is address-tied at all.** Instrumenting
`Funcdata::addr_tied_ranges` (`p6_variables/funcdata_merge.rs:886`) to print the window
flags:

```
[ATIED] space=register off=0x0 winflags=0x1288130 addrtied=true members=49
   [ATIED-MEMBER] vn#141  off=0x0 sz=8 flags=0x1288130 ...   (0x8000 = addrtied, 0x200000 = mapped)
   [ATIED-MEMBER] vn#1961 off=0x0 sz=8 flags=0x1288030 ...
   ... every RAX version carries addrtied
```

The painter is kuna's own, not the port:
`p6_variables/coreaction_cleanup.rs` → `ActionMergeRequired::apply` calls
**`mark_output_storage_addr_tied(data)`** immediately before `merge_addr_tied`, described
in its own comment as a *"(kuna W4-ScopeLocal stand-in)"*. Its whole-function comment
block states the upstream fact it is standing in for:

> C++ `syncVarnodesWithSymbols` (funcdata_varnode.cc:993) ties an un-symboled processor
> register ONLY via `lm->inScope`, ALWAYS false for a register, so C++ never restructures
> this loop-carried register into a whole-function local.

It fires when **any** same-address SSA version is a `marker` write (MULTIEQUAL/INDIRECT),
subject to five existing escape hatches (all-marker-inputs-persist, all-writes-const-COPY,
forwarding-register-alias, transient-self-chain, loop-carried-marker — LOSS-206/229/231/
234/241). None of them asks the one question that matters here: **is the join the value
the function returns?**

**Step 4 — the type follows the merge.** `HighVariable::updateType` picks the most
specific member type, so `code *` (from the 22-way function-entry phi) beats
`xunknown8` (the return constants); the printed return type follows the returned
HighVariable, so the *function signature* becomes `code *` too.

**Step 5 — ablation.** Building the same tree with `mark_output_storage_addr_tied`
skipped reproduces ghidra's shape exactly:

```
$ KUNA_NO_OUTTIE=1 decomp_dbg  ... load addr 0x20f60 ; decompile ; print C
unsigned long sub_20f60(char *a0,unsigned long *a1) // return-dupe
{
  code *v1; // rax
  if (!a0)
    return 0xfffffc18;
  ...
```

### Owning phase

**P6 — variables / merge** (`decompiler/crates/kuna-decomp/src/p6_variables/`). The
`(code *)` is a *symptom* of a forced-merge decision; nothing in P3 (cast insertion) or
P5 (type inference) is wrong here. This is the fourth distinct cause hiding under
`novel.md`'s `code_ptr` column (see `novel-code-ptr-cluster.md` for causes 1–3).

## Option sweep — nothing covers it

All 88 catalog options at every non-default value, plus `--mode reliable|aggressive|fast`,
run on `--addr 0x20f60`, counting lines containing `code *` (baseline 5):

| setting | count |
|---|---|
| DEFAULT / `--mode reliable` / `aggressive` / `fast` | 5 |
| `inferfuncentry off`, `returndup off`, `earlyreturn off`, `realtypes off`, `dedupvardecls off`, `foldcallret off`, `condfold on`, `condfold wide`, `paramcopyhoist on`, `ptrentry on`, `namestyle ghidra`, … (all 88 rows) | 5 |
| `entry_disc off` | 0 — **not a fix**: function discovery is off, so the callee entries are never resolved and the whole binary degrades |

`condfold on|wide` swept explicitly, as required. No option closes it.

## Metric-artifact check

- `approximated: false`, `degenerate_source: false`, `artifact_suspect: false`,
  source CFG 54 nodes / 79 edges — the score is a real edit distance.
- **`source_ambiguous: true`, and it bites here.** rsyslog compiles 12 modules that each
  define a `static rsRetVal queryEtryPt(...)`; decbench's `_relabel_to_dwarf` is
  name-keyed, so the tools' panes are for *different* instances — kuna's artifact is
  `@ 0x20f60` (omfile.c, 22 entry points) while ghidra's and ida's are `@ 0x27790`
  (smtradfwd.c, 6 entry points). The group's GED spread (kuna 22 vs ghidra 145 / ida 150)
  is therefore **not** a like-for-like comparison and must not be quoted as one.
  It does not weaken the case: the defect reproduces at every one of the 12 addresses,
  *including* 0x27790, which is where ghidra's pane comes from — so the ghidra contrast
  above is address-matched after all.

## Breadth

**Within rsyslogd (O2-noinline, 1,907 functions).** All 12 module-entry-point siblings
carry it, 3 casts each:

| addr | `(code *)` now | with gate | return type now → gated | lines now/gated |
|---|---|---|---|---|
| 0x20f60 (the case) | 3 | 0 | `code *` → `unsigned long` | 150 / 122 |
| 0x1bbc0 0x1c220 0x1d290 0x241c0 0x25090 0x255c0 0x25d80 0x26a60 0x26eb0 0x272f0 0x27790 | 3 each | 0 | `code *` → `unsigned long` | −12…−28 each |

36 occurrences → 0, matching the census (24× `0xfffffc18` + 12× `0xfffffc14`).

**Corpus-wide A/B** — `kuna decompile-all --json`, 13 binaries across 3 arches / 2 opt
levels / ELF+PE, every changed function diffed:

| binary | fns | changed | `(code *)` before→after | Δlines |
|---|---:|---:|---:|---:|
| O0 bash | 3,278 | 50 | 157 → 156 | −904 |
| O0 iproute2 ip | 1,962 | 8 | 19 → 19 | −276 |
| O2-noinline betaflight (ARM) | 6,388 | 10 | 634 → 634 | −81 |
| O2-noinline coreutils ls | 639 | 9 | 17 → 17 | −141 |
| O2-noinline crazyflie (ARM) | 2,914 | 13 | 393 → 393 | −79 |
| O2-noinline e2fsck | 1,909 | 29 | 391 → 391 | −254 |
| O2-noinline certtool | 833 | 2 | 18 → 18 | −14 |
| O2-noinline mydoom.exe (PE) | 161 | 1 | 39 → 32 | −8 |
| O2-noinline nuttx (ARM) | 890 | 7 | 461 → 460 | −26 |
| O2-noinline sshd | 2,042 | 63 | 100 → 99 | −585 |
| O2-noinline tar | 1,585 | 24 | 71 → 70 | −154 |
| O2-noinline x0r-usb.exe (PE) | 15 | 0 | 261 → 261 | 0 |
| O2-noinline rsyslogd | 1,907 | 48 | 433 → 397 | −419 |
| **total** | **24,523** | **264 (1.1%)** | **2,994 → 2,947 (−47)** | **−2,941** |

The `code_ptr` counter under-states it. The real class is *"the return register absorbs
an unrelated pointer type"*, whatever that type is:

- **pointer-cast-on-integer-constant** occurrences (`(code *)0x…`, `(char *)0xffffffff`,
  `(int4 *)0xd5a5a`, …): **911 → 784, −127**.
- functions whose **declared return type** changes: **173 of the 264**; 15 lose a bogus
  `code *`, **0 gain one**; 22 lose `char *`, 4 lose `void *`.
- functions declaring a `code *` **return type**: **16 → 1**.

A second witness, unrelated to `code_ptr`, is `bash::unbind_array_element` @ 0xa3738
(source returns `int`): today `char * unbind_array_element(...)` with
`v4 = (char *)0xfffffffe; return v4;` five times; gated, `unsigned long` with
`return 0xfffffffe;`.

**Speed.** Faster, because 27-member forced merges stop happening:
`decompile-all` on sshd, 3 runs each — 51.24 / 48.83 / 49.52 s → 45.28 / 44.58 / 45.75 s
(**−9%**). Well inside the ≤5% *regression* budget.

## Proposed fix

**One module.** `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs`,
inside `mark_output_storage_addr_tied` — add a sixth escape hatch next to the five
existing LOSS-* ones:

> The `marker` join that justifies the tie must be the value the function **returns**.
> If no `marker`-written same-address Varnode has a `CPUI_RETURN` among its descendants,
> the register is carrying somebody else's value through a join, not a whole-function
> return local, and C++ `inScope` would leave it un-tied.

This is the same class of IR-shape test as the four already there (transient-self-chain,
loop-carried-marker, forwarding-alias, all-persist), and it is the one that distinguishes
`multiret`'s `getval` (the phi **is** the return value → keep tying) from `queryEtryPt`
(the phi feeds a `STORE` → do not tie).

**Measured, not proposed.** Implemented and run in the throwaway worktree behind
`KUNA_GATE_RET`:

```
$ KUNA_GATE_RET=1 kuna test --datatests                              -> 675/675  exit 0
$ KUNA_GATE_RET=1 kuna test --datatests --datatests-dir tests/stages -> 394/394  exit 0
```

(The crude whole-ablation, for contrast, fails exactly 3 of 675 —
`multiret.xml` *Multi-size return #1/#2/#3* — and 0 of 394 stages. The narrow gate
recovers all three.)

**Gating.** Output-changing on 1.1% of functions and a judgment call about kuna's own
stand-in heuristic ⇒ it ships as a named option per `docs/agents.md` (`retjointie`,
P6 / variable-merge, `transform` tier), with a DIV row if it ever goes default-on.
Stage test: `tests/stages/ghdec-retjointie.xml`, two-pass — `off` reproduces
`code *` + `(code *)0x…`, default emits the integer. Remember the catalog-count bumps
(`kuna_phases/tests.rs`, `tests/catalog_bytecompat.rs`, `tests/stages/kuna-catalog.xml`,
`kuna-base/src/xml.rs` corpus count) and the stages-baseline re-record.

### Risks — every one of the 264 changed functions was diffed

No function loses a call. Three need a companion fix or an explicit accept:

1. **A folded call can be rendered twice.** `rsyslogd sub_6a8e0` and
   `sshd sub_6daa0` gain an extra `__ctype_b_loc()` / `__errno_location()` occurrence
   against **one** `CALL` op in the p-code (`print raw` confirms a single
   `0x0006a911:3f: u… = call f__ctype_b_loc`, and `objdump` a single `call` site).
   Root-caused: `foldcallret` (kuna's angr-derived call-return folding, **default-ON**)
   inlines the single-use call output into a `LOAD` that is itself implied at *two*
   references under `max_implied_ref = 2`, so the whole folded expression prints twice.
   Proved by knob: `KUNA_GATE_RET=1 --option foldcallret off` → 1 occurrence,
   `KUNA_GATE_RET=1` alone → 2, default → 1. This is a **pre-existing `foldcallret`
   bug that the un-tie merely exposes** (the tie was forcing the call output explicit),
   and its fix belongs in `p6_variables/kuna_callretfold.rs::call_output_foldable`:
   refuse to fold when the consumer chain is implied at more than one reference.
   File it as its own case; ship it with, or before, `retjointie`.
2. **A dead partial-register `CONCAT44` can appear.** `sshd sub_97bc0` gains
   `v3 = CONCAT44(dat_4,v1);`, immediately overwritten — the same shape #251 accepted in
   2 of 14,720 functions. 1 occurrence in 24,523 here.
3. **Some pointer return types flatten to `unsigned long`** (22 `char *`, 4 `void *`).
   Spot-checked both bash witnesses: in `unbind_array_element` (source returns `int`) the
   flatten is a strict fix; in `parse_string_to_command` (source returns `COMMAND *`) it
   trades a wrong-but-pointer type for a right-but-untyped one — the same trade ghidra
   makes (`undefined8`). Net −127 bogus pointer casts, 0 new `code *` return types.

`make rust-test` was **not** run for this triage (the workspace suite is the CI long
pole and the record touches no `.rs`); the implementing PR owns it, along with
`kuna catalog --check` and `make check-spec`.

### Spec

`docs/spec/06-variables-and-merge.md` (the chapter owning `p6_variables/`) — the
return-register address-tie stand-in and its escape hatches are described there; the
new gate is prose in the same paragraph.

## Siblings

- **11 in-binary siblings** (0x1bbc0 0x1c220 0x1d290 0x241c0 0x25090 0x255c0 0x25d80
  0x26a60 0x26eb0 0x272f0 0x27790) — identical shape, identical fix, table above.
- **`O0-rsyslog-rsyslogd-queryEtryPt`** (the pool's other case, @ 0x2734d) —
  **already clean on today's build**: at `-O0` the handler lives in a stack slot, not in
  RAX, so nothing joins the return register. `unsigned int sub_2734d(...)` with
  `return 0xfffffc18;`. The defect is optimized-build-only.
- Cross-project: 264 functions in the 13-binary sample, above.

## Related records

- `novel-code-ptr-cluster.md` — causes 1 (ARM Thumb `funcptr_align`, closed by #249),
  2 (`inferfuncentry` collisions), 3 (untyped callbacks, not a defect). This is
  **cause 4**, and it is the only one that is a *merge* defect.
- The census that opened this track also closed `raw_reg` (dead since #226/DIV-46) and
  recommended scoring `code_ptr` relatively (`max(0, kuna − min_rival)`); this record
  independently supports that — kuna emits 2,994 `(code *)` on the sample and the change
  removes 47 of them, while 173 functions improve with no `code_ptr` movement at all.
  **The wart column is measuring the wrong thing; the return-type flip is the signal.**
