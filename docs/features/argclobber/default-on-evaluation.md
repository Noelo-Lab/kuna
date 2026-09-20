> **Superseded.** This is the evaluation of the rule as it shipped in #652, whose
> callee clause was a bounded walk of the callee's entry. Its recommendation --
> redesign that clause around the callee's own recovered prototype once
> `protoorder` lands -- is what `analysis.md` describes and what the option now
> does. The four `ce-*` programs below are still the negative controls; under the
> current rule each keeps its argument, and `ce-forward-thunk.s` /
> `ce-forward-thunk-2param.c`, then `ce-import-forward.c` and
> `ce-forward-thunk-2frame.c`, were added afterwards as the further classes the
> redesign had to answer -- each of them is declined by the callee's BODY and by
> what it forwards to (`resolve_forward_transfer`), not by its recovered
> prototype. `sweep-2026-09-19.txt` is the OLD rule's corpus sweep;
> `sweep-2026-09-20-forwarding.txt` is the current one.

# `argclobber` default-on evaluation

**Decision: stays off.** A deleted argument that the callee reads counts as a
bug under the flip rule, and four small, ordinary C/asm programs produce one.
The datatest, stage and CLI gates all stay green with the default flipped, so
they cannot catch it. Nothing in this directory changes the engine.

Base: `origin/main` d96e3408. Both arms come from the same build, with
`--option argclobber off|on`. The `on-default` gate runs used a throwaway
build that set the two constructor defaults (`Architecture`, `ArchContext`)
to `true`. That build is not committed.

## 1. The counterexamples

Built with gcc 11.4.0 on x86-64. The caller address given is the one each
build gets:

```
gcc -O2 -fno-inline -o a ce-remainder-or-pair.c && strip a        # caller 0x1200
gcc -O2 -fno-inline -o b ce-probe-budget.c && strip b             # caller 0x1770
gcc -nostdlib -static -o c ce-probe-budget.s                      # caller 0x401000
gcc -O2 -shared -fPIC -o libce.so ce-import-lib.c
gcc -O2 -fno-inline -o d ce-import.c -L. -lce && strip d          # caller 0x1180
kuna decompile-all <x> --addr <caller> --option argclobber off|on
```

Every shape clause of the rule passes at all four. The only clause that could decline is the
callee-body clause (`CalleeEntryDead::proves_input`), and that clause can only
veto when its bounded entry walk *sees* a read. When the walk cannot see the
read, the drop goes through.

| file | how the callee's read is hidden from the entry probe | callee's own recovered prototype |
|---|---|---|
| `ce-remainder-or-pair.c` | read only past a jump table (`notrack jmp *%rax` is a cut) | `sub_1170(uint a0, ulong a1, ulong a2)`, 3 |
| `ce-probe-budget.c` | read in the **first** instruction (`mov %rdx,%r8`), but the call-free body is over the 192-instruction budget, so the walk is abandoned and its reads cleared | `sub_1160(int a0, long a1, long a2)`, 3 |
| `ce-probe-budget.s` | `tests/stages/kuna-argclobber-guards.xml`'s caller unchanged; its callee reads `rdx` first, then runs 200 `add`s | `sub_401031(long a0, ulong a1, long a2)`, 3 |
| `ce-import.c` + `ce-import-lib.c` | the callee is a PLT import in no signature table; the walk decodes the stub's indirect jump and stops | (no body) |

In the three C callers, `rdx` at the call is a join of `a % d` (a real
remainder, passed on purpose) and the high half of a 16-byte struct returned in
`rax:rdx`. That covers both of the #652 review's residual holes: the `rax:rdx`
forward, and a callee clause that has nothing to say when the probe is
incomplete. The `.s` file uses the exact fixture shape, a real clobber on one
path and an `idivl` remainder on the other.

```
$ gcc -O2 -fno-inline -o ce ce-remainder-or-pair.c && strip ce
$ kuna decompile-all ce --addr 0x1200 --option argclobber off
  if (a0) { v1 = a1 / a2;  a1 %= a2; }
  else    { v3 = sub_1160(a1);  a1 = SUB168(v3,8);  v1 = SUB168(v3,0); }
  return sub_1170(v2 & 0xffffffff,v1,a1) + 1;
$ kuna decompile-all ce --addr 0x1200 --option argclobber on
  if (a0) a1 /= a2;
  else    { a1 = sub_1160(a1); }
  return sub_1170(v1 & 0xffffffff,a1) + 1;
```

With the option on, the output loses both the argument and the `a % d` that
computed it, and on the other path it loses the struct's second half. The same
diff appears at all four callers: `sub_1160(...,v1,a1)` in `b`, and
`ext3(...,v1,a1)` in `d`, each lose their third argument. In `c`,
`sub_401031(a0,5,v3)` becomes `sub_401031(a0,5)` and `v3 = v1 % 200` is
deleted. The #652 review built the `rax:rdx` shape with a *direct* forward
(`p = mk(x); use3(op, p.a, p.b)`) and could not get a phantom out of kuna. That
matches what we see here: at a direct forward, kuna already renders two
arguments with the option off, so the option changes nothing there. That is a
separate baseline defect. What the option adds is the joined shape above.

**protoorder does not change this.** PR #669's build (types mode, default on)
renders the same wrong drop on `ce-remainder-or-pair.c` and
`ce-probe-budget.c` with `argclobber on`. `types` locks no call spec, so the
rule still runs. On the fmt witness it gives the expected 2/2/2.

## 2. Positive evidence would lose the witness

A rule that drops the argument only when the callee's walk proves the register
is never consumed was measured (throwaway patch, not committed). The test: the
walk is complete, there is no live read, and the register is written before
every non-`RETURN` terminator. It declines all four counterexamples. It also
declines three of the four drops #652 shipped as correct:

| site | walk | positive proof |
|---|---|---|
| fmt O2 `0x2ac1` -> `sub_3700` | complete, 1 terminator (a call, `rdx` unwritten) | no |
| fmt O2-noinline `0x2a91` -> `sub_3a30` | same | no |
| bash O2 `0xe3194` -> `xrealloc` | complete, 3 terminators, `rdx` unwritten | no |
| e2fsck O2 `0x282de` -> `ext2fs_dblist_sort2` | complete, `rdx` written before its call | **yes** |

So the fmt drop is correct because of what `fmt` is. Nothing the rule reads
shows it. The evidence that does separate the two sets is the **callee's own
recovered prototype**. All four shipped drops land on a callee kuna itself
recovers with two parameters (`sub_3700(FILE *, ulong)`, `sub_3a30(FILE *,
ulong)`, `ext2fs_dblist_sort2(int8, code *)`, `xrealloc(void *, uint8)`).
Every counterexample callee recovers three, or has no body. A sound version
needs callee-first ordering to read that prototype, which is protoorder's
machinery (PR #669). It would decline whenever the callee has no recovered
prototype: an import, `kuna decompile` of a single function, or a callee not
yet decompiled.

## 3. Flip procedure results (default-on build, same base)

| step | result |
|---|---|
| (a) `make test` | 675/675, PARITY OK. No datatest assertion moves |
| (b) `make test-stages` | 1177/1177, PARITY OK. The three argclobber stage tests already pin both passes |
| (c) `make test-cli` | 189/189 |
| arity lens / (f) | **fails** on the counterexamples in section 1. The corpus sweep in section 4 finds no wrong drop |
| (d) typesweep, 444 slices | 10,748 functions. 2 have different `variables[]` and none scores differently: perfect 986 -> 986, aggregate 3111.18 -> 3111.18, improved 0 / worse 0 (control: 0 byte-identical functions scored differently) |
| (e) speed | not run: the flip failed on correctness first. #652 measured -0.61% on fmt main |
| (g) modes.rs | `argclobber` is in `EXCLUDED_ON_PURPOSE`; unchanged |

(a) to (c) are green because none of the corpora contains the shape, not
because the rule is sound.

## 4. Corpus exposure

`decompile-all` ran off and on over every stripped ELF in decbench
`full_run_address_2026-09-11`: 758 binaries and 255,115 functions, with no run
exiting non-zero. The full table and the hunk classes are in
`sweep-2026-09-19.txt`.

* 22 functions change, one per binary. Each loses exactly one trailing
  argument (21 go 3 -> 2, u-boot's goes 2 -> 1), and the two fmt `main`s also
  gain the second argument at their under-recovered sibling site.
* The arity lens finds no wrong drop. All 22 land on the true arity: DWARF on
  the unstripped twin, or the libgcc/e2fsprogs header where the callee has no
  DWARF (`__addvsi3`, `__mulvsi3`, `ext2fs_badblocks_list_create`). 21 also land
  on the arity kuna recovers for the callee. The exception is
  `ext2fs_badblocks_list_create`, a one-`jmp` tail-call thunk that kuna renders
  `(void)`.
* **19 of the 22 are right for a reason the rule never reads.** At those 19 the callee
  walk is complete but "silent": it neither sees a read nor sees the register
  written before a call. That is the same state the walk is in at
  `ce-remainder-or-pair.c`. Only e2fsck O2, e2fsck O2-noinline and u-boot
  O2-noinline have positive evidence.
* The hunks outside the argument itself are the dead remainder and its copies,
  dropped braces, a `while` that re-renders as a `for` (the 11 O0 openssh
  `permute_args`), and one inlined single-use divisor. One further effect is
  missing from the option's documentation. At bash `expand_prompt` (O2 and
  O2-noinline) the **preceding** call's output shrinks from 16 bytes to 8,
  because the `rdx` half has lost its only reader: `v = xmalloc(n); a =
  SUB168(v,8); ...` becomes `v = (char *)xmalloc(n);`. That is right for
  `xmalloc`, and it is the same mechanism that deletes the struct's real second
  half in the counterexample.

So the corpora the gates run cannot tell the rule's correct drops from its
wrong ones. The corpus holds no instance of the unsound shape, and the one
signal that would separate the two, the callee's recovered prototype, is one
the rule does not read.

## 5. Recommendations

* Keep `argclobber` off.
* Correct its `phases.toml` row to `destructive = true`. `kuna_phases.rs`
  defines the field as "NOT safe as a global default", and the evidence above
  is exactly that. This was the #652 review's nit.
* Redesign the callee clause around the callee's recovered prototype once
  protoorder lands: drop only when that prototype exists and has no parameter
  overlapping the register. On the sweep above that keeps 21 of the 22 drops.
  It loses the thunk, whose recovered prototype is `(void)` and so proves
  nothing, and it declines all four counterexamples. The four files here are
  that design's negative stage tests (pass 1 option off, pass 2 option on, both
  keep three arguments).
* Name the output resize in the option's row and in spec chapter 04. Dropping
  the argument also narrows the preceding call's return value, which is how
  the struct half goes missing in the counterexample.
