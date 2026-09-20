# `argclobber` — the callee's recovered prototype as the admitting evidence

## The symptom, still

coreutils `fmt` -O2 (stripped, decbench `full_run_address_2026-09-11`) calls one
static function three times from `main` and kuna renders three different arities,
the longest of which reads a local nothing assigns:

```
$ kuna decompile-all O2/coreutils/stripped/fmt --option argclobber off
  unsigned long v9;  // rdx        <- never assigned
  unsigned long v11; // rdx        <- never assigned
      v12 &= sub_3700(v6);
    v3 = sub_3700(stdin,v7);
  v12 = sub_3700(stdin,"-",v10);
unsigned long sub_3700(FILE *a0,char *a1)
```

The callee's own recovered prototype takes **two** parameters, and DWARF on the
unstripped twin says `fmt(FILE *f, char const *name)`. The third argument is
`rdx`, which at that call is a join of a previous call's clobber and a dead
`idiv` remainder. Nothing on the caller's side put a value there for this call.

## Why the shipped rule could not be a default

`argclobber` shipped off. Its eight clauses admit a drop when the caller's side
shows a clobber and nothing else, and the only clause that could speak for the
callee was a **bounded walk of the callee's entry**: decline if that walk sees
the register read before it is written.

That walk can only ever *see*, and the evaluation on `feat/argclobber-on`
(`default-on-evaluation.md`, kept here with its four programs) found four
ordinary ways for a real read to be invisible to it:

| program | how the read hides | callee's recovered prototype |
|---|---|---|
| `ce-remainder-or-pair.c` | read only past a jump table | 3 parameters |
| `ce-probe-budget.c` | read in the first instruction, but the body is over the walk's 192-instruction budget | 3 parameters |
| `ce-probe-budget.s` | same shape, hand written | 3 parameters |
| `ce-import.c` | the callee is a PLT import | no body |

In each, `argclobber on` deleted an argument the callee reads, and with it the
expression that computed it — including the second half of a 16-byte `rax:rdx`
struct return. Every gate stayed green, because no corpus the gates run contains
the shape.

The evaluation also measured the opposite rule (drop only on *positive* proof
that the register is dead through the callee) and it declined three of the four
drops the option shipped as correct. The one signal that separates the two sets
is the one the rule never read: **the callee's own recovered prototype**. All
four shipped drops land on a callee kuna itself recovers with two parameters;
every counterexample callee recovers three, or has no body.

## What changed

Reading that prototype needs callees decompiled before their callers, which is
`protoorder`'s machinery (#669, merged). `protoorder types` parks each callee's
recovered parameter list — storage and type — as it decompiles the call graph
callee-first, and the caller reads it back through
`Funcdata::kuna_protoorder_types`.

The callee clause is now:

* the callee's recovered prototype must **exist** for this entry,
* none of its parameters may overlap the argument's register bytes, and
* its parameters must **account for every argument the drop leaves behind** —
  each recovered parameter covered by a surviving argument and each surviving
  argument covered by a recovered parameter.

Requiring it to exist carries part of the clause, because `protoorder` states
nothing for a recursive component (`Decline::Scc`), an import with no recovered
body, a callee that recovered no parameters, and a callee whose prototype is
already *declared* (which input-locks the call spec, declined at the rule's first
line). A callee that was never decompiled first states nothing, so the rule is
inert on a single-function `kuna decompile`, a narrowed `decompile-all`, a
`--jobs N` run, and under `--option protoorder off`.

The accounting clause is what carries the rest, and it exists because of a
counterexample the free-bytes test alone gets wrong. A recovery *short* of what
the call passes is not a statement that the tail is unwanted; it is a statement
that the recovery did not reach the tail. `ce-forward-thunk.s` is that shape:

```
fwd:    movq (%rdi), %rax
        jmp  *%rax
```

The thunk never names `rsi` or `rdx`, so kuna recovers **one** parameter for it
while the function it tail calls consumes three, and `protoorder` states that
short list in both its modes (`KUNA_PROTOORDER_TRACE=1` prints
`state sub_11cd params=1`). With only the free-bytes test the drop is admitted
and the forwarded `rdx` argument is deleted. The accounting clause declines it:
one recovered parameter cannot be the two arguments that would remain.

### `protoorder` does not decline a variadic callee

This must not be claimed, and an earlier draft of these docs claimed it. The
variadic guards in `protoorder` — `Decline::UnderRecovered` and
`Decline::RegisterFileFull` — live in the branch that parks a prototype in the
symbol table (`lock`). `park_recovered` returns through `state_recovered_types`
**before** that branch in the default `types` mode, which is the mode this rule
reads. `Decline::Variadic` itself is only ever set from a *declared* `...`, and a
declared prototype is rejected one branch earlier by `Decline::Declared`, so on a
stripped image it cannot fire at all.

A stripped SysV variadic is therefore *stated*. `long vlog(int op, long a, ...)`
with a `va_arg` read of the third argument, built `gcc -O2` and stripped, states
three parameters:

```
$ KUNA_PROTOORDER_TRACE=1 kuna decompile-all ./vprobe 2>&1 >/dev/null | grep 11d0
[protoorder] state sub_11d0 @0x11d0 params=3 trimmed=0
```

What declines the drop there is the **recovered list itself**: a register-save
prologue reads every argument register the convention has, so the list carries
the register the drop would take and the free-bytes test refuses. Where a
variadic recovers short instead, the accounting clause and the body veto answer
for it.

The bounded entry walk stays, as a **veto only**: it can refuse a drop the
prototype admitted, never admit one (u-boot's `printf`, 1,924 sites, its
`va_list` prologue spilling `r1`–`r3`). It is also now seeded only when something
is stated, so the option costs nothing on a surface where it cannot fire.

## The clause a prototype cannot carry

The accounting clause above declines `ce-forward-thunk.s` because one recovered
parameter cannot be two surviving arguments. Add one argument-register read to
the thunk and the accounting is satisfied again — and the drop is wrong anyway.
`ce-forward-thunk-2param.c` is that program, in plain C with no hand assembly:

```c
long fwd(struct box *o, long a, long b)
{ if (!a) return 0; return o->fn(o, a, b); }
```

`gcc -O2` emits `test %rsi,%rsi; je; jmp *(%rdi)`. kuna recovers `(rdi, rsi)` —
two parameters, an honest report of the two registers the body touches — while
the function it jumps to consumes three. Two recovered parameters are exactly the
two arguments the drop would leave behind, so `stated.len() == surviving.len()`,
no parameter lies in `rdx`, and the third argument is deleted together with
`mk()`'s 16-byte `rax:rdx` return half that feeds it:

```
$ gcc -O2 -o t3 ce-forward-thunk-2param.c && strip t3
$ kuna decompile-all ./t3 --option argclobber off
    v2 = sub_1160(a1); a2 = SUB168(v2,8); v1 = SUB168(v2,0);
    return sub_1190((unsigned long *)0x4018,v1,a2) + 1;
$ kuna decompile-all ./t3                            # the drop, before this change
    a2 = sub_1160(a1);
    return sub_1190((unsigned long *)0x4018,a2) + 1;
```

A prototype states what a callee **names**, and this one names nothing about
`rdx`. The clause that answers it is therefore not about the prototype at all:
wherever the callee's control leaves it, the register must already be **written**
by the callee, unless the target can be answered for
(`resolve_forward_transfer`). At the `jmp *(%rdi)` it is not and an indirect
target answers for nothing, so the caller's value is still travelling into code
no recovery read, and the drop is declined.

A **direct** call has a target to ask, and asking it is the whole of the second
round of this work. Naming it is not enough: `long wrap(void *o,long a,long b)
{ if (!a) return 0; return ext3(o,a,b); }` forwards `rdx` to an import nothing
states a signature for, recovers the same two parameters, and lost the same
argument (`ce-import-forward.c`, and `ce-forward-thunk-2frame.c` for the same
thing one ordinary call frame deeper). So a direct call lets the register through
only when its target carries a **declared, non-variadic** prototype, or when the
target's own body answers the same question recursively. That is what keeps the
`fmt` witness alive: `fmt(FILE *, char const *)` reaches its first call with
`rdx` unwritten, that call is direct, and its target's body reaches only `ret`
and a call to the declared `fileno(FILE *)`.

An incomplete summary — the probe budget, a runaway written set — now declines
too, because the clause asks for positive evidence. That subsumes
`ce-probe-budget` as a second, independent reason.

## What it still cannot know

The evidence is a recovery, not a fact: a callee whose own parameter list kuna
under-recovers states a prototype that admits the drop. That is the residual
hole, and it is the hole every callee-derived statement has. Two clauses bound
it. The accounting clause requires the under-recovery to be exactly one slot
deep, at exactly the register the clobber wrote, with every other argument still
accounted for. The opaque-transfer clause requires the register to be dead at
every control transfer the walk could not follow, which is what closes the case
where the recovery is short *because* it never saw the code that reads the
register.

Neither covers the other. The prototype clause removes the class a bounded body
walk cannot see at all — a read past a jump table, a read beyond the probe's
budget, a read inside an import, the `rax:rdx` struct-return forward — by
carrying the parameter in the recovered list, which only works where the recovery
reached far enough to name it. Where it did not, the thunk above is the whole
class, and the body walk is what answers. The six programs in this directory are
those classes.

One effect belongs in the option's own description rather than in a footnote:
dropping the argument also **narrows the preceding call's return value** where
that call was the register's only writer. `bash`'s `expand_prompt` renders
`v = xmalloc(n); a = SUB168(v,8);` off and `v = (char *)xmalloc(n);` on. That is
right for `xmalloc`, and it is the same mechanism that would delete a real struct
half at an under-recovered callee.

## Why `destructive = true` even as a default

`kuna_phases.rs` defines `destructive` as "not safe as a global default". What
this option drops is an *argument*: a wrong drop is a deleted expression, which
is the kind of wrong output a reader cannot see. Seven other options ship
`destructive = true` with a non-`off` default for the same reason — the field
records the failure mode, not the shipped value.
