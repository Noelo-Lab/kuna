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

1. `op_is_barrier` also stops at an op whose **output varnode is persistent or
   address-tied** (`op_writes_tied_storage`, the helper #654 wrote for the
   discounted fold, moved here and shared rather than duplicated). A global's
   output is persistent; a frame slot's is tied to its stack address, and the
   callee reaches one whenever the frame address escaped into it.
2. A call output is re-examined in `check_implied_cover`, where
   `ActionMarkImplied`'s descendants-first walk has already classified the chain
   below the use, so the landing statement is derivable (`print_chain`). The
   barrier test then runs over the whole distance the call travels
   (`fold_print_point_is_order_safe`).

Two exemptions keep the guard from declining over things that are not writes or
not reorderings:

* **A self-copy is inert.** `RuleIndirectCollapse` rewrites the INDIRECT a call
  carried over a global it turns out not to write as `glob = COPY glob`, and that
  op's output is persistent. It stores the value already there, so it is not a
  barrier (`op_is_self_copy`; a volatile location is excluded, since there the
  access itself is the effect). Without this, `gh275-spillargtrial` and
  `kuna-libctypes` lose a fold each to an artifact.
* **The ops the expression travels through are exempt.** Each consumes the
  previous one's value, so the call is evaluated before them in the folded text
  exactly as it is in the binary — a `LOAD` of the pointer a call has just
  returned is not a load the call was moved past. Without this exemption the
  guard declines 233 further functions over the 16-binary sweep (all consumers),
  and `ghidra_sim_faillog_pins`' flattened-C line counts move by +4/+5.

The INDIRECT half of the predicate is **not** widened for the default fold. It
keeps the span it has always had, from the call to its single use, for a measured
reason: asking it past the use changes 31 functions across `grep` O0, `tar` O0
and `ssh` O2 and de-folds 27 of them, correcting none. The shape that dominates
is `dat_33798 = *__errno_location();` (`grep` O0 `sub_6c53`), and the binary
there is `call 4890 / mov (%rax),%eax / mov %eax,0x33798` — one call, one load,
one store, in exactly the order the folded text prints. Exempting the self-copy
from it as well makes `kuna-elfmain`'s `sub_1357(a1[1])` fold where it did not
before — a *new* fold, which a correctness fix has no business adding.

## What this got wrong first: `foldcallretphi`

That span *is* `foldcallretphi`'s, and moving #654's machinery into
`kuna_callretfold.rs` narrowed it to the landing statement alone. That widened
the option rather than leaving it alone: with `--option foldcallretphi on` it
then folded two calls it used to decline and recovered none.

* `ssh` O2 `sub_4fd30` printed `sub_3fa80(v2,v3)` after `v5 = v2`, a copy of the
  escaped stack slot at `0x8(%rsp)` whose address went to `sub_4f830`. The
  binary is `4fe85 mov 0x8(%rsp),%rdi / 4fe93 call 3fa80 / 4fe98 mov
  0x8(%rsp),%rdi / 4fe9f je 4fe4e`: the value reaching the taken branch is the
  **post**-call load at `0x4fe98`, which is GH-181's shape.
* `tar` O0 `sub_67494` folded three `tolower` calls into a store whose base is an
  address-tied slot the call carries an INDIRECT over.

`landing_span_reads_call_effect` asks the question over the whole span again,
landing statement included, as #654 did. The phi-delta set — the functions where
`foldcallretphi on` differs from `off` — is then identical to `origin/main`'s:
68 on `ssh` O2 and 120 on `tar` O0, the same addresses (69 and 121 without the
clause, the extras being exactly those two). Default output is byte-identical
with the clause and without it on `ssh` O2, `tar` O0, `grep` O0 and `fmt` O2;
the clause is reachable only from `conflict_is_self_call_effect`, which
`check_implied_cover` calls only when the option is on.

## Witnesses

| shape | what fixes it |
|---|---|
| `repro.c` (the issue verbatim) | clause 1 |
| `int u = t * 3;` between call and write | clause 1 (gcc schedules the store first) |
| `travel.s` (write after the use) | clause 2 |
| `libselinux.so.1` `sub_16170` | clause 1, in a real binary |

The last one, from the 16-binary sweep:

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

## Corpus sweep

16 binaries (`decompile-all`, O0 and O2 of coreutils `fmt`, grep, gzip,
diffutils `diff`, bzip2, findutils `find`, plus the four ELFs in
`tests/bug-repro/`), classified by `foldclassify.py`:

| | |
|---|---|
| functions | 6207 |
| functions changed | 57 (0.92%) |
| `defold` | 46 |
| `defold-dedup` | 7 |
| `OTHER-same-calls` | 4 — read by hand, all de-folds the counter missed because the old text already bound the call to something (`v7[2] = sub_1ef10(0x40) + 0x40;`) |
| `FLAG-call-gained-or-vanished` | **0** |
| functions that *gain* a fold | 1 (`grep` O2 `0x6a70`, counted twice because `grep` appears as both a decbench and an in-tree ELF) |
| call tokens | 29228 → 29221 (−7, exactly the seven duplicated emissions) |
| emitted lines | 187182 → 187277 (+95) |

Attribution, from a third build with clause 2 disabled: clause 1 alone changes
**14** functions and costs 11 folds; clause 2 accounts for the remaining 43.

The seven `defold-dedup` functions are the more interesting half. In `find` at
`0x1da6f` the folded call was printed at *both* of its sinks:

```c
/* before */                              /* after */
v1 = v2[sub_33502(v2)];                   v2 = sub_33502(v3);
v2[sub_33502(v2)] = '\0';                 v1 = v3[v2];
                                          v3[v2] = '\0';
```

## What this does not fix

A call output whose landing statement is in **another block** still folds if the
call-to-use span is clear — the second clause declines it (`print_chain`'s block
test), so this is covered, but only while the chain is derivable. `print_chain`
gives up after 8 hops, on a marker, and on a fan-out; each of those declines the
fold, which is the conservative direction.
