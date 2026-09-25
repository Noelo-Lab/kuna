# `callbacktype` — a callback takes the prototype of the slot it is passed to

## The gap

kuna resolves a declared prototype at a call site. On `coreutils/cut -O0` the sort call
already renders with the library's name and the comparator's address:

```
qsort(dat_d148,dat_d150,0x10,sub_3a72);
```

Nothing has ever been said about `sub_3a72`. A function reached only through a function
pointer has no call site of its own, so `protoorder` (callee to callers) has nobody to tell
and `calleevote` (callers to callee) has nobody to ask — `CallGraph::direct_call_sites`
returns `None` for any entry with a `Data` cross-reference, and taking a function's address
IS one. Its own body is all it has:

```
unsigned long sub_3a72(unsigned long *a0,unsigned long *a1)
```

where the program declares `int (const void *, const void *)`. The other half is worse: a
signal handler that ignores the signal number recovers no parameter at all
(`void sub_3b72(void)` for bzip2's `void mySignalCatcher(IntNative)`), so there is nothing
for a type vote to retype.

## What the corpus says

`docs/features/callbacktype/census.md` has the measurement. Over the campaign's 444 slices,
294 callback arguments name 114 distinct functions, 50 of which decbench scores. **All 50
DWARF declarations are exactly the slot's signature** — none contradicted, none reached
through two slots that disagree. 67 ground-truth parameters would move, 16 of them
parameters kuna recovers no trace of.

So the claim is sound where it fires, and the question is only whether it fires where the
address is not really a declaration of the function. That is what the refusals are for.

## The rule

At every call to one of 23 library entry points that declare a function-pointer parameter,
the constant in the callback slot is recorded (per caller, so a redo replaces rather than
doubles). The slot's argument register is the prototype model's own assignment for the
entry point's declared parameter list, so it comes from the program's convention. The
driver then parks the slot's declared prototype on the function it names and decompiles it,
and every function that calls it, again.

Refused when: a declared prototype already covers the function (DWARF, a user `--assert`, the
library tables, or, under `protoorder lock`, its own recovered prototype); two slots disagree; the
address reaches somewhere the recorded arguments do not explain (it is open in the image,
or an address-taking cross-reference sits in a function that handed it to no slot, or there
are more such references than there were arguments, or a registering body used the same
address somewhere no slot accounts for); the function's own recovery found more inputs than
the declaration passes; the body reads the argument register one past the declared list
before writing it; the body hands back a value it computed where the slot says `void`; or the
model cannot place the parameters.

The last two are not the same refusal, and the review round is why both are here. The
register walk sees READS: a body that forwards its arguments in a tail call
(`int cmp3(const void *a, const void *b, size_t n) { return memcmp(a, b, n); }`, cast into
`qsort`'s two-argument slot) never reads the third register and states nothing, so only the
arity its own recovery found refuses the declaration — without which the park drops a live
argument at every direct call site and leaves an uninitialized read in the body. Likewise the
cross-reference count is a count of INSTRUCTIONS: `signal(SIGINT, f); atexit(f);` materializes
`f` once and registers it twice, so only the body's second USE of the address refuses a
`void (int)` on a routine that takes nothing. That use is followed past whatever only carries
the value: a handler registered and then called through a phi that may also hold another
function, or whose register is also written into a global the body never reads back, is the
same second use one carrier further on. None of these shapes occurs anywhere in the 444-slice
corpus — all are pinned by `tests/cli` probes over
`decompiler/crates/kuna-analysis/tests/fixtures/callbacktype_refused_x86_64`.

Two names are deliberately out of the table. `__cxa_atexit` is how glibc's `<stdlib.h>`
compiles `atexit(f)` — `f` is cast from `void (*)(void)` — so its declared
`void (*)(void *)` would give every handler a parameter its source never wrote.
`sigaction`'s handler is a structure member, not an argument.

## Why it moves arity where `calleevote` does not

`calleevote` states types against the storage a callee already recovered, and refuses to
move an argument list — a fabricated parameter is invisible to `type_match` and shows up
only as wrongness. Here the arity claim is the point, and it rests on a declaration the
program itself carries rather than on an inference: a `signal` handler takes an `int`
whether or not the body reads it. The 16 handlers in the census are the case, and DWARF
confirms every one.
