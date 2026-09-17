# GH-657 — `foldcallret` sank a call past a write to storage the callee reads

## The defect

`foldcallret` (default **on**) lets a single-use call output fall through
`ActionMarkExplicit::base_explicit`'s forced-explicit call arm, which moves the
call's *evaluation* to wherever the expression is printed. `call_output_foldable`
is what keeps that move honest, and its barrier set was stated in opcodes: no
call, `CPUI_LOAD`, `CPUI_STORE` or `CPUI_CALLOTHER` between the call and its use.

Heritage promotes a write to a fixed address into a plain `CPUI_COPY`, so a write
to a global is none of those:

```c
int k = 1;
__attribute__((noinline)) int f(int x) { return x + k; }
int main(void) { int t = f(7); k = 42; return t + k; }
```

```console
$ gcc -O2 -o /tmp/p1 p1.c && kuna decompile /tmp/p1 main
int main(void)
{
  k = 0x2a;
  return f(7) + 0x2a;
}
```

The binary returns `(7 + 1) + 42 = 50`; that C reads `(7 + 42) + 42 = 91`.

The same hole reaches further than the call-to-use span. `call_output_foldable`
guards up to the single use, but the use is the call's textual home only when the
use op is itself a statement. When the use op's own output is *implied* the
expression keeps travelling, and crosses whatever sits between:

```asm
target2:  mov edi,7 / call helper / lea edx,[rax+rax*2]   # the use, implied
          / mov [k],42                                    # the write
          / lea eax,[rdx+42] / ret                        # where it lands
```

`docs/features/gh657/travel.s` is that shape as an assembler input, because gcc
schedules the global store before the arithmetic in every C form of it.

## The fix

Two clauses, in `p6_variables/kuna_callretfold.rs`:

1. `op_is_barrier` — the call-to-use span — also stops at an op whose **output
   varnode is persistent or address-tied** (`op_writes_tied_storage`, the helper
   #654 wrote for the discounted fold, moved here and shared rather than
   duplicated). A global's output is persistent; a frame slot's is tied to its
   stack address, and the callee reaches one whenever the frame address escaped
   into it.
2. A call output is re-examined in `check_implied_cover`, where
   `ActionMarkImplied`'s descendants-first walk has already classified the chain
   below the use, so the landing statement is derivable (`print_chain`). The
   guard then runs over the whole distance the call travels
   (`fold_print_point_is_order_safe`).

**The second span asks a narrower question than the first**
(`op_is_write_barrier`): a call, a `CPUI_STORE`, a `CPUI_CALLOTHER`, or a write to
persistent or address-tied storage. `CPUI_LOAD` is deliberately not in it. Up to
the use a `LOAD` is a hazard this pass can decide — the call may `STORE` what it
reads, and the use is where the folded text lands. Past the use, the reads left in
the span are the ones the folded expression is printed *beside*, inside the single
statement the whole implied chain collapses into, so de-folding over them
repositions nothing. Counting them costs **117 further functions** on the sweep
below, the dominant shape being `__ctype_b_loc()` inlined into the same
conditional as the loads it now sits next to (`bash` O0 `0xfa64d`: call, then four
loads, then the branch, all one expression either way).

Two exemptions keep the guard from declining over things that are not writes or
not reorderings:

* **A self-copy is inert.** `RuleIndirectCollapse` rewrites the INDIRECT a call
  carried over a global it turns out not to write as `glob = COPY glob`, and that
  op's output is persistent. It stores the value already there, so it is not a
  barrier (`op_is_self_copy`; a volatile location is excluded, since there the
  access itself is the effect). Without this, `gh275-spillargtrial` and
  `kuna-libctypes` lose a fold each to an artifact. Nine unit tests in
  `kuna_callretfold/tests.rs` pin the predicate: a constant or an arithmetic
  result written into a global is a barrier, `glob = COPY glob` is not, a copy
  between two globals or at a different width or of a volatile location is, a
  write to an untied temporary is not, and a `LOAD` is a barrier on the first span
  only.
* **The ops the expression travels through are exempt.** Each consumes the
  previous one's value, so the call is evaluated before them in the folded text
  exactly as it is in the binary. With the narrowed set this exemption is no
  longer load-bearing: dropping it changes **0** of the 23,851 functions below. It
  is kept for the case it is written for — a chain op that is itself a call with an
  implied output, where the folded expression is that call's own argument.

The INDIRECT half of the predicate is **not** widened for the default fold. It
keeps the span it has always had, from the call to its single use, for a measured
reason: asking it past the use changes 31 functions across `grep` O0, `tar` O0
and `ssh` O2 and de-folds 27 of them, correcting none. The shape that dominates
is `dat_33798 = *__errno_location();` (`grep` O0 `sub_6c53`), and the binary
there is `call 4890 / mov (%rax),%eax / mov %eax,0x33798` — one call, one load,
one store, in exactly the order the folded text prints.

## `foldcallretphi` is left alone

`foldcallretphi` (#654, opt-in) shipped with its own span guard, wider than either
of `foldcallret`'s: every opcode, the storage test with no self-copy exemption, no
chain exemption, and the INDIRECT question over the whole distance including the
landing statement. An earlier revision of this branch moved that guard into
`kuna_callretfold.rs` and shared it, which silently relaxed it — with
`--option foldcallretphi on`, `ssh` O2 `0xcdd0` and `dpkg` O2 `0x2e170` each
*gained* a fold that `origin/main` declines.

`kuna_foldcallretphi.rs` now composes its guard exactly as it shipped; only
`print_point` and the raw storage test delegate to `kuna_callretfold`, so the two
options share the helpers without sharing the policy. Since that guard is strictly
wider than the second `foldcallret` span, a phi-discounted fold is decided exactly
as it was on `origin/main`.

Measured per function on the `--option foldcallretphi on` dumps, main build
(`origin/main` 520443d7) against this branch, split on the `// Function:` banner:

| binary | functions | phi delta (on≠off) main / branch | ON arm main≠branch | of those, also changed in the DEFAULT arm |
|---|---|---|---|---|
| `ssh` O2 | 1510 | 68 / 68, same set | 15 | 15 |
| `dpkg` O2 | 800 | 45 / 45, same set | 6 | 6 |
| `bash` O2 | 2538 | 95 / 95, same set | 43 | 43 |
| `tar` O0 | 1570 | 120 / 120, same set | 3 | 3 |
| `grep` O0 | 642 | 26 / 26, same set | 1 | 1 |

The last column is the acceptance test, and it is content equality, not set
equality: **no function renders differently under `--option foldcallretphi on`
unless the default rendering changed too.** `ssh` O2 `0xcdd0` and `dpkg` O2
`0x2e170` are byte-identical between the two ON dumps.

## Witnesses

| shape | what fixes it |
|---|---|
| `repro.c` (the issue verbatim) | clause 1 |
| `int u = t * 3;` between call and write | clause 1 (gcc schedules the store first) |
| `travel.s` (write after the use) | clause 2 |
| `libselinux.so.1` `sub_16170` | clause 1, in a real binary |
| `dash` O0 `0xab64` | clause 2, a `STORE` statement in a real binary |

`libselinux`:

```c
/* before */                              /* after */
v23 = 1;                                  v9 = strdup(v14);
v22 = 0x6666666;                          v24 = 1;
*(char **)(a0 + 0x50) = strdup(v12);      v23 = 0x6666666;
                                          *(char **)(a0 + 0x50) = v9;
```

```asm
162d0: call 8e30 <strdup@plt>
162d5: movl $0x1,0x20(%rsp)
162dd: movl $0x6666666,0x14(%rsp)
162ef: mov  %rax,0x50(%rbx)
```

`dash` O0 `0xab64` — the call was printed after a store that puts back the byte
the call is supposed to read as a terminator:

```c
/* before */                              /* after */
*v3 = v1;                                 v2 = fnmatch(a3,v5,0);
if (!fnmatch(a3,v4,0))                    *v4 = v1;
  return v2;                              if (!v2)
                                            return v3;
```

## Corpus sweep

37 binaries, `decompile-all`, before = `origin/main` 520443d7, after = this
branch: O0 and O2 of coreutils `fmt` and `ls`, `grep`, `gzip`, `diff`, `bzip2`,
`find`, `tar`, `dash`, `su`, `e2fsck`, `kmod`, `ip`, `libedit`, `ssh`, plus `libz`
O0, `libselinux` O2, `libacl` O2 and the four ELFs in `tests/bug-repro/`.
Classified by `foldclassify.py`:

| | |
|---|---|
| functions | 23,851 |
| functions changed | 126 (0.53%) |
| `defold` | 88 |
| `defold-dedup` | 19 |
| `OTHER-same-calls` | 19 — read by hand, all de-folds the counter missed because the old text already bound the call to something (`v7[2] = sub_1ef10(0x40) + 0x40;`), plus `vN` renumbering |
| `FLAG-call-gained-or-vanished` | **0** |
| call tokens | 150,151 → 150,130 (−21, exactly the 21 duplicated emissions) |
| emitted lines | 801,176 → 801,368 (+192) |

A second, disjoint set — O0 of `bash`, `dpkg` and `certtool`, O2 of `rsyslogd`,
O0 of `crond` (6,610 functions, none in the table above) — changes 39: 28
`defold`, 2 `defold-dedup`, 9 read by hand, **0 FLAG**.

### Attribution by cause

Five builds of the second span were swept over the same 37 binaries — structural
conditions only, then plus the storage test, then plus the opcodes — so every
changed function is attributed to the clause that produced it. A function can have
two causes.

| clause | functions |
|---|---|
| **(i)** a write to persistent or address-tied storage in the span — the GH-657 class | **54** (49 between the call and its use, 6 between the use and the landing statement, 1 both) |
| **(ii)** a call, `STORE` or `CALLOTHER` between the use and the landing statement | **10** |
| **(iii)** the landing statement is in another block (47) or the print point is not derivable (21) | **67** |

(iii) is the largest bucket and the least ambiguous: a landing statement in
another block means the folded call would be evaluated behind a branch or after a
merge, so it can run at a different time or not at all. The 21 underivable ones
are conservative declines — a marker, a fan-out, or a chain over eight hops.

All ten of (ii) were read: each has a `STORE` emitted as its own statement between
the call and the landing statement, and in four of them that store writes memory
the callee reads (`dash` O0 `0xab64`/`0xac29` restore the byte `fnmatch` sees;
`ssh` O2 `0x57170` folds `close(fd)` past `*a2 = -1`; `tar` O2 `0x614a0` folds
`strlen(v11)` past `*a6 = *a6 + 1`). None is a de-fold over an op that shares the
landing statement.

## What it costs to read

A de-folded call that sat inside a condition comes back as a comma expression
rather than a preceding statement, because the condition is where its value is
consumed: `while (v1 = __ctype_b_loc(), ...)`. Declarations rise by a handful per
binary (`varcensus`: `fmt` O2 +2, `grep` O0 +1, `dash` O2 +2, `libselinux` +4).
That is the price of keeping the call ahead of the write, and it is the order the
binary already takes.

## What this does not fix

`print_chain` gives up after 8 hops, on a marker, and on a fan-out; each of those
declines the fold, which is the conservative direction. A call whose landing
statement is in another block always declines, even where the block is
unconditional.
