# varargstackargs — a variadic call's stack arguments are not evidence about its register file

Route: RE-friction loop (`docs/re-pipeline.md`), round 1, challenge
`68149b8a8f555589f353117c` — Mach-O arm64 `practice_2_rengr`.
Track: `quality`.

## The observation, as filed

> **Call argument recovery drops scanf destination and printf varargs**
> (`wrong-output` / `major`)
> kuna emitted `scanf("%d")` while subsequently comparing an apparently
> uninitialized `v3`, and emitted three `printf` calls containing `%s` with no
> corresponding argument.
> probe: `kuna decompile {{BIN}} _main` contains `v2 = scanf("%d");`
> acceptance: same command contains `scanf("%d",&v3)`

Reproduced on a fresh build of `fca7a971`. Filed hypothesis: *"AArch64 variadic-call
argument recovery or stack-address propagation is losing live arguments even though
libc prototypes and format strings are known."* That is right about the symptom's
family and wrong about where the loss happens: **the arguments are recovered.** They
are thrown away one pass later, by a rule about a *different* resource.

## What the code actually does

`_main` at `0x100003d94`. Apple's arm64 ABI passes variadic arguments on the stack,
so clang -O0 sets up each call as

```
  0x100003e44: mov  x9, sp
  0x100003e48: add  x8, sp, #0x14        ; &passcode
  0x100003e4c: str  x8, [x9]             ; the variadic argument, at [sp+0]
  0x100003e50: adrp x0, "%d"             ; the one fixed parameter
  0x100003e58: bl   scanf
```

kuna gets the first three steps right. `libproto` gives `scanf` the prototype
`int scanf(char *, ...)`, `Heritage::guard_calls` registers a stack-parameter trial
at callee-relative `s:0x0` and appends the argument Varnode, and
`FuncCallSpecs::check_input_trial_use` scores that trial **ACTIVE** — it passes the
local-alias test, the caller-local-range test and `ancestor_op_use`:

```
TRIAL call=0x100003dec slot=5 trial=s:0 spacebase=true vnspace=stack
      vnoff=0xffffffffffffffa0 vnsz=8 => checked=true active=true ...
```

The trial then arrives at `build_input_from_trials` **inactive**. The pass in between
is `FuncProto::derive_input_map` -> `ParamListStandard::fillin_map`, and instrumenting
its four steps says exactly which rule fires:

```
FILLIN sections [0, 1, 9, 28] resource_start [0, 1, 9, 18]
FILLIN after force_no_use            #17 s:0x0 ... act=true    <- printf call
FILLIN after force_inactive_chain    #17 s:0x0 ... act=false
```

for the three `printf` sites, and

```
FILLIN after force_exclusion_group   #17 s:0x0 ... act=true    <- scanf call
FILLIN after force_no_use            #17 s:0x0 ... act=false
```

for the `scanf` site.

Both rules are section-scoped and positional:

* `force_no_use` — once a whole exclusion group in the section is
  definitely-not-used, everything after it in the section is demoted.
* `force_inactive_chain` (`maxchain = 2`) — an active trial past a run of more than
  two unused slots is demoted.

Both encode "parameters fill the resource in order, so a hole means the list ended".
On AArch64 the general registers `x0`-`x7` *and* the outgoing stack area are **one
section** (`AARCH64.cspec` / `AARCH64_apple.cspec` open a new resource only for the
hidden-return and floating-point pentries): section 2 spans trials 9..28, groups 9
(`x0`) through 17 (the stack pentry). At a variadic call on this ABI, `x1`-`x7` are
structurally empty. Seven slots. Both rules read that as "the argument list ended at
`x0`" and kill the stack trial:

* printf: `x2`'s group makes `chainlength` reach 3 > `maxchain`, `seenchain` latches,
  and every later trial including `s:0x0` is marked inactive.
* scanf: `x1` is definitely-not-used (killed by the call, no realistic ancestor), so
  `force_no_use` latches `seendefnouse` at the next group and demotes the rest.

The consequence is not cosmetic. A dropped argument leaves the `str x8,[x9]` store
with no consumer, so dead-code elimination reaps it and the address computation
behind it. That is why `_main` also *reads a local it never writes*: `v3` in the
filed output is the `scanf` destination, and nothing assigns it because the only
thing that ever did was the argument that was discarded.

## Why this is kuna's problem and not the ABI's

`fillin_map`'s positional inference is sound while arguments really do fill the
resource in order. At a **variadic** call on an ABI that passes varargs on the stack
they do not, and the gap is a property of the calling convention, not of the
recovery. kuna already knows which calls those are: the trace shows
`dotdotdot=true inputlocked=true` on every one of the five sites. The information
needed to stop guessing is present and unused.

Note what is *not* the fix. Reactivating the stack trial where it sits would also
trip `force_inactive_chain`'s trailing promotion loop — "every inactive slot before
the last surviving active trial is promoted, because the list must be contiguous" —
and fabricate `x1`-`x7` as six invented register arguments. The register hole and the
stack argument have to stop being in the same section at all.

## The owning decision

P4 (calls & prototypes), sub-stage **active-input-trial-scoring**, in
`ParamListStandard::fillin_map_standard`
(`decompiler/crates/kuna-decomp/src/p4_calls/fspec.rs`), between
`separate_sections` and the `force_no_use` / `force_inactive_chain` loops. Spec
chapter: `docs/spec/04-calls-and-prototypes.md` 4.2.

## Scope: the other three observations in this cluster are NOT this bug

The brief grouped four observations as one possible root cause. They are not.
Reproduced and root-caused separately:

| Observation | Root cause | Same bug? |
|---|---|---|
| `68149b8a` scanf/printf varargs | `fillin_map` section rules vs. a variadic stack tail | **this one** |
| `69a54bd7` allocator args (`sub_140008160()`) | `Funcdata::only_op_use` rejects the trial because the size is also branched on (`ONLYUSE reject match=0x1400026b0 slot=2 at op 0x1400026aa CPUI_CBRANCH`) | no — see `docs/features/calleearity/` |
| `68149b8a` `_secret_function(v2)` | x0 (scanf's return) is genuinely live and unused at the call; only the *callee's* body proves it takes no argument. Needs interprocedural prototype propagation, which kuna has for library functions (`libproto`) and not for internal ones | no |
| `5ab77f54` `sub_40c81e` 2 params vs 5 | callee-side: the function is itself variadic (`lea eax,[ebp+0x10]; push eax` forwards a `va_list` to `sub_40df87`), so its own `ActionInputPrototype` correctly recovers only the two parameters it *reads*. Materialising three more needs the 5-argument call site pushed into the callee | no |

Only the first two are "an argument dropped at a call site", and even those are two
different gates in the same pipeline. Fixed separately, each behind its own option.
