# angr `test_decompiling_missing_function_call` :: adams/main — analysis

## The gap

angr recovers `main`'s getopt dispatch as a proper `switch` (6 cases:
`strdup`/`fprintf`/`atoi`/… inside a `while` getopt loop). kuna fails to recover the
jump table and emits:

```
                    /* WARNING: Treating indirect jump as call */
  v2 = (*(code *)((int8)*(int4 *)((uint8)(uint4)(m - 0x3fU) * 4 + 0x96ac) + 0x96ac))();
  return v2;
```

The entire `switch` body (every case, every `strdup`/`fprintf`) **and** the surrounding
`while (getopt(...) != -1)` loop collapse into a single bogus indirect call. The angr test
asserts exactly this is wrong: `code.count("fileno") == 1`, every `break` is followed by
`case`/`default`/`}`, and `emaillist = strdup(...)` is present.

## The construct

`main`'s switch is a textbook GCC PIC jump table (x86-64, `adams` is a PIE; binary vaddrs,
main @ 0x8e40, table @ 0x96ac in `.rodata`):

```
8eef: add $0xffffffc1,%eax    ; eax = m + (-0x3f) = idx        (m = getopt() result)
8ef2: mov %eax,%ecx           ; ecx = idx
8ef4: sub $0x36,%eax          ; eax = idx - 0x36
8ef7: mov %rcx,-0x48(%rbp)    ; SPILL idx to stack slot -0x48
8efe: ja  9106                ; if (idx >u 0x36) goto default     <- the range guard
8f04: lea 0x7a1(%rip),%rax    ; rax = &table (0x96ac)
8f0b: mov -0x48(%rbp),%rcx    ; RELOAD idx from stack slot -0x48
8f0f: movslq (%rax,%rcx,4),%rdx ; rdx = sext(table[idx])  (self-relative 4-byte offset)
8f13: add %rax,%rdx           ; rdx = table_base + table[idx]
8f16: jmp *%rdx
```

The bound is a single clean range guard `idx <=u 0x36` ⇒ **table size 0x37 (55 entries)**;
most entries point at the default, leaving 6 distinct case targets.

## Owning stage / why kuna fails (instrumented)

Owning stage: **S2 / switch-model** (jump-table recovery), in
`JumpBasicModel::recover_model_basic` (`s2_lift/jumptable.rs`), invoked from
`JumpTable::recover_addresses` during `FlowInfo::recover_jump_tables`.

Instrumentation (`KUNA_JT_DEBUG=1`) at the failure point shows:

```
basic model2=false jrange.size=2147483648 maxtablesize=1024 num_common_vn=8 vidx=2
  meldvn[2] u+0x23d00 sz=4 def=CPUI_LOAD          (table load result — the chosen vn, unbounded)
  meldvn[6] %+0x0    sz=4 def=CPUI_INT_ADD         (eax = idx = switchvar-0x3f — the real index)
  guard[2]  vn=%+0x0 sz=4 rangesize=4294967295 branch=true   (the range guard on idx — FULL range)
```

So:

1. `find_normalized` → `analyze_guards` DID collect the range guard (`guard[2]`, on `eax`),
   but its **range came back full** (2³²−1). The guard is realized as `sub 0x36; ja` — a
   *flag-form* unsigned comparison. Jump-table recovery runs **early, on a partial/truncated
   function**, before the flag idiom is simplified to `INT_LESS(0x36, idx)`, so
   `circlerange_pull_back` cannot extract the `<= 0x36` bound.
2. With no bounded guard, `calc_range`/`find_smallest_normal` leave the index range at the
   full 2³¹, which exceeds `maxtablesize` (1024).
3. `recover_model_basic` therefore declines the basic model; model-2 also declines; so
   `recover_model` yields no model and `recover_addresses` throws
   *"Could not recover jumptable … Too many branches"*.
4. `FlowInfo::recover_jump_tables` → `truncate_indirect_jump(FailNormal)` rewrites the
   `BRANCHIND` to a `CALLIND` (`/* WARNING: Treating indirect jump as call */`), which is why
   the whole switch + loop disappear.

The stack spill (`mov %rcx,-0x48(%rbp)` / `mov -0x48(%rbp),%rcx`) additionally means the
guarded value and the table-index value are distinct varnodes connected only through memory,
so even a non-flag guard would not `value_match` the normalized index — `analyze_guards`'
`usenzmask = !is_partial` conservatism on partial functions is by design.

This is the **same failure symptom** that kuna's existing `switchmodbound` option (GH-9191)
addresses for an *in-band modulo/and-mask* bound — here the bound is instead an
*out-of-band CBRANCH range guard* in unsimplified flag form.

## Hypothesis / fix

A new **default-OFF, option-gated** `JumpBasic` model-extension, exactly parallel to
`kuna_try_modulo_bound_table`: when the basic model fails to bound the table
(`jrange.size > maxtablesize`) and `option switchguardbound on`, look at the collected
`selectguards`; for a guard whose storage matches a meld index varnode, extract the
unsigned comparison constant `K` from the (possibly flag-form) guard CBRANCH and re-bound the
table index to `[0, K+1)`, then let the normal `buildAddresses` emulation enumerate the
targets. Gate off ⇒ byte-identical to today (the method is only reached in the
already-failed branch). See `plan.md`.

## Performance investigation (post-merge, 2026-06-25)

The reviewer flagged the feature as "incredibly slow." Measured on the target
(`adams`/`main`, x86-64) with a **slope** method that cancels the fixed per-run load cost
(load the binary once in a single `decomp_dbg` session, decompile `main` N times, take the
per-decompile slope `(T(N₂)−T(N₁))/(N₂−N₁)`; 5-rep median):

| | per-decompile (slope-isolated) |
|---|---|
| `switchguardbound off` | **≈ 23.9 ms** |
| `switchguardbound on`  | **≈ 63.2 ms** (**+164 %**, ~2.6×) |

(The PR body's wall-clock figure — off ≈ 192.9 ms, on ≈ 259.1 ms, +34 % — is the same
effect *diluted* by the ~190 ms fixed process-spawn + `load file` + SLEIGH-mmap cost that
OFF and ON share; the slope number above is the honest *marginal* decompile cost.)

### Root cause: the cost is downstream and **inherent**, not the heuristic

Instrumented (`std::time::Instant` around `kuna_try_guard_bound_table`, the only new
hot-path code): over a full `main` decompile the heuristic is invoked **3 times, ~7 µs
each (~22 µs total)** — i.e. **< 0.1 %** of the ~40 ms ON↔OFF gap. With the gate OFF it is
invoked **0 times** (confirmed). So the new model-extension itself is essentially free; the
slow-down is **entirely downstream**.

The reason is structural, and is the whole point of the feature: with the gate OFF the
basic model fails, `recoverAddresses` throws *"Too many branches"*, and
`truncate_indirect_jump` rewrites the `BRANCHIND` into a single `CALLIND` — collapsing the
**entire 55-entry switch and the surrounding getopt `while` loop** (six fully-bodied cases:
`strdup`/`fprintf`/`atoi`/`usage`/…) into one bogus computed `(*(code*)…)()` call. With the
gate ON the table is bounded and the switch survives, so the rest of the pipeline (CFG
build, dataflow simplification, **structuring**, and **type inference**) now processes a
~2.6× larger, real function instead of a truncated stub. That is the cost of *correctness*,
not an algorithmic defect.

We specifically ruled out the failure modes the reviewer worried about (cf. the prior
`ActionPool` O(N²) regression): the heuristic is **not** called per-op or per-CFG-node (3
calls total), there is no O(n²)/O(n³) scan, no redundant per-op recomputation, and no
repeated dominator/region rebuild. `scan_guard_tree`'s inner work is already triple-bounded
(64-step tree walk, 4096-step `eval_guard_expr` budget, and the `v`-loop capped at
`maxtablesize`=1024), and empirically resolves in microseconds.

### Disposition

There is **no localized algorithmic fix** — the extra time is the decompiler correctly
doing ~2.6× the work to recover the real switch+loop. Per the speed gate (budget +5 %), the
option therefore stays **DEFAULT-OFF, opt-in** (`speed_forced_off = true`), which also
matches its risk profile: the guard-to-index correspondence is asserted across a stack spill
it cannot prove in dataflow, so a loose match could over-bound an unrelated indirect jump.
Flip it per program (`option switchguardbound on`) when a switch renders as a computed call
with *"Could not recover jumptable … Too many branches"*. Output is byte-identical with the
gate off (675/675 datatests unchanged), so there is no `docs/divergences.md` entry.

## "Too many declared variables on top" — duplicated declarations (context for a follow-up)

The reviewer noted the PR's *before* snapshot has a wall of identical declarations at the
top of `main` (e.g. `int4 c; // stack - 0xc` ×16, `int4 fd; // stack - 0x10` ×15,
`int4 t; // stack - 0x28` ×30). This is **not** caused by `switchguardbound`; it is a
pre-existing rendering artifact, and is now **already mitigated on `main`** by the
`dedupvardecls` option (DIV-7, **default-on**), which landed *after* this branch was cut.
The PR body's *before*/*after* blocks were captured pre-merge, before `dedupvardecls`
existed.

Measured on the post-merge tree (`adams`/`main`):

- With `option dedupvardecls off` (i.e. the old behavior the PR body shows), `switchguardbound on`
  renders **67× `int4 t`, 34× `int4 m`, 34× `int4 c`, 33× `int4 fd`** — even worse than the
  PR snapshot, because the recovered switch+loop has more SSA merge points for the same
  stack slots.
- With `dedupvardecls` **on** (the current default), the same decompile's declaration block
  collapses to a clean ~12 lines; the only residual repeat is `int4 t; // stack - 0x28`
  **twice**.

### Why it is over-defined (hypothesis)

Each named stack slot is mapped onto **many distinct same-named scalar `HighVariable`s** —
one per SSA-merge point that the HighVariable-merging pass did not unify. The getopt `while`
loop and the 6-case switch create many back-edges / case-join `MULTIEQUAL` phis writing the
*same* stack slot; the merge heuristic leaves them as separate `HighVariable`s (separate
`Symbol`-less locals that happen to print the same `type name; // stack - off` line). The
printer emits one declaration per `HighVariable`, so N un-merged copies ⇒ N identical
declaration lines. `dedupvardecls` is a **render-side** collapse (it de-dups *identical
fully-rendered lines*), not a fix of the underlying over-fragmentation — so it hides the
symptom but the engine still carries N HighVariables.

The residual `int4 t; // stack - 0x28` **×2** is a *different*, legitimate case: the 8-byte
slot at `-0x28` is split into two genuine 4-byte sub-field HighVariables, used in the body as
`t._0_4_` and `t._4_4_`. They share the printed name `t` but are distinct sub-pieces, so
`dedupvardecls` correctly does *not* collapse them — this one is not a bug.

Follow-up (separate agent, out of scope here): investigate the HighVariable-merge pass
(`Merge::mergeAdjacent*` / the intersection test in `s6_merge`) for why same-slot
loop/switch-phi locals are not unified into one HighVariable, which would fix the
over-definition at the source rather than at the printer. The `int4 t` sub-piece split is
expected and should be left alone.
