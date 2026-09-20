# ptrfromuse — analysis

## 1. The defect

A function parameter that the body only ever dereferences is declared as an
integer, and whether it gets a pointer type at all depends on nothing more than
whether the first field the compiler reads happens to sit at offset 0. Three
lines of C, stock `gcc -O2`, nothing kuna-specific:

```
$ printf 'struct S { long pad; int a; int b; };\nint sum(struct S *s) { return s->a + s->b; }\nint main(void){ struct S s={0,1,2}; return sum(&s); }\n' > st.c && gcc -O2 -o st st.c
$ kuna decompile ./st sum
int sum(long a0)
{
  return *(int *)(a0 + 8) + *(int *)(a0 + 0xc);
}
```

Move `a` to offset 0 and the same parameter is declared `int *a0`. The same
split shows on two neighbouring functions of coreutils `fmt` -O2 (stripped),
with `--option libctypes off` so the callee tables do not answer first:

```
$ kuna decompile-all fmt --addr 0x3000 --option libctypes off | sed -n 2p
void sub_3000(unsigned long *a0)          # dereferenced at offset 0
$ kuna decompile-all fmt --addr 0x3420 --option libctypes off | sed -n 2p
void sub_3420(long a0,unsigned int a1)    # dereferenced at +8 and +0x10
```

Ground truth for both is `FILE *`.

## 2. The bisect (instrumented throwaway build, env-gated)

Two hypotheses were on the table for `sub_3420` — the up-edge refusal at
`propagate_int_add` (`coreaction_infertypes.rs`, the `inslot == -1` arm), or a
competing integer vote from the untyped calls `__uflow(a0)` / `sub_2ea0(a0, ...)`.
An instrumented build printing every vote in `get_local_type`, every
`propagate_to_pointer` result and every `inslot == -1` refusal:

```
[PFU] input vn off=56 sz=8            # off=56 = RDI = a0
[PFU]   vote INT_ADD  -> int8      x3 # wins
[PFU]   vote CALL     -> xunknown8 x3 # type_order +1, loses

[PFU] propagate_to_pointer sz=8 -> ptrto=Some("xunknown8:8") x30 (117 results in all)
[PFU] :1105 REFUSE out->in alttype=metaTYPE_PTR outvn_is_input=true off=56 x12
```

So the competing-call-vote hypothesis is **refuted** — the calls vote
`xunknown8`, which is *less* specific than `int8` and loses; the `int8` comes
from `TypeOpIntAdd::getInputLocal` itself. The LOAD/STORE addresses **are**
typed pointer, and the `inslot == -1` arm refuses to hand that pointer back to
`a0` twelve times per run.

**Both halves fail, and they fail independently.** That is what decides the
design: a candidate supplied *in the fold* is a single, order-independent vote,
while flipping the `:1105` arm fires on every local, not just parameters, and
its pointee would be the access width (`long *`, `int *`) rather than the
GT-dominant `char *`. The up-edge is recorded here as a possible v2, not shipped.

## 3. The rule

`kuna_ptrfromuse::pointer_from_use(data, vn, mode)`, called from
`build_localtypes`, returns a candidate only when:

* `vn` is a function **input** whose storage `FuncProto::possible_input_param`
  accepts, whose width is the default data space's address size, and which is
  not type-locked;
* a bounded **breadth-first** walk of its transitive descendants (visited set,
  ten hops, so a loop-carried MULTIEQUAL is entered once) through
  `COPY`/`MULTIEQUAL`/`INDIRECT` identity, the base slot of `PTRADD`/`PTRSUB`,
  and `INT_ADD`-with-a-literal reaches at least one terminal use that is a
  `LOAD`/`STORE` address. Breadth first makes the cap a property of the graph —
  everything within ten hops of the input is seen — rather than of the traversal
  order: depth-first recorded whichever path reached a shared Varnode first, so
  whether a refusing use just past the boundary was classified at all depended
  on pop order. The switch types one more function in the 16-binary corpus
  (O2 tar `sub_3c8f0`) and leaves the typesweep bit-identical in both arms;
* and no use on the way refuses: `INT_MULT`, `INT_DIV`/`SDIV`/`REM`/`SREM`,
  `INT_2COMP`/`INT_NEGATE`, any shift, `PIECE`, any float opcode, a comparison
  against a non-zero literal, a `CALL` whose callee prototype has already
  committed that argument to a non-pointer, or an `INT_ADD` whose constant
  addend is itself a global object address (§4).

The candidate is **folded** by `Datatype::type_order` against `get_local_type`'s
result, never installed as a replacement seed — `build_localtypes` treats a seed
as a replacement, which would clobber `libctypes`' `FILE *` (`SUB_PTR_STRUCT`
outranks this rule's `SUB_PTR` only *inside* the fold). It also never speaks
when the fold already holds a pointer.

## 4. The constant addend: three grades

`p->field` and `table[i]` lower to the **same** `INT_ADD` of a value and a
constant. Only the constant says which operand is the base, so the walk asks
`ActionConstantPtr::isPointer`'s own question of it — the default data space's
pointer bounds, `resolve_constant` into that space, then the global scope — and
grades the answer:

| grade | test | walk |
|---|---|---|
| **Object** | a global symbol at the resolved address | **refuse the candidate** |
| **Maybe** | address-like, names nothing | **neutral**, like a variable addend |
| **No** | below the space's pointer lower bound | carry on; a field offset |

Without the Object refusal, `char table[256]; int lookup(long i){ return
table[i]; }` under stock `gcc -O2` is declared `int lookup(char *a0)` and prints
`(char)a0[0x4040]` — a wrong declaration, *and* the named global `table` replaced
in the body by its raw address. Reproduced on four builds (PIE and `-no-pie`,
stripped and not) before the fix and after.

The Maybe grade earns its keep by ablation. A flat "at or above the bound ⇒
refuse" rule costs nine correct `bzFile *` declarations in bzip2, whose struct is
0x13f0 bytes and whose fields are read at `+0x1394` and `+0x13e8`; a stripped
`.bss` table carries no symbol either, so requiring one would miss the very case
the refusal exists for. Treating it as ambiguous does the right thing in both:
a parameter with no other use is declined, one that is *also* dereferenced at an
ordinary offset keeps its candidate from that use (one of the nine, O0 bzip2
`sub_b9c1`, comes back this way).

**Residual exposure, stated plainly.** A parameter used only to subscript an
*unnamed* byte-element table is declined rather than typed — the safe direction,
but by accident rather than by knowing. A parameter used both as such a subscript
and as a real memory base is typed on the strength of the second use, and its
subscripted global loses its name in the body. `tests/stages/kuna-ptrfromuse.xml`
pins both grades (`globalindex`, `blindindex`); each was proven two-pass with a
throwaway build that short-circuits its own arm.

## 5. Three more fixes the measurements forced

**`possible_input_param`.** The first corpus sweep showed the canary check
reappearing on coreutils `ls` `sub_d820`: `FS_OFFSET`, the x86-64 segment base
the canary is read through, is a function *input* used only as a LOAD base, so
the rule typed it `char *` and `stackguard` stopped recognising the pattern.

**Follow `PTRADD`/`PTRSUB`.** Once the parameter is a pointer, `oppool2` rewrites
`INT_ADD(p, k)`. A walk that stopped at `INT_ADD` found no base on the *next*
pass, withdrew the candidate, let the rewrite be undone, and oscillated into the
seven-pass settle ceiling (`byte`, before the fix: `fmt` 1→6, `ls` 6→42,
`grep` 10→71, `tar` 23→248).

**Never override a vote that is already a pointer.** The first `typesweep`'s
entire `worse` column was `BZ2_bzReadClose`, `src_to_dest_free` and `entry_free`,
all ground-truth `void *` that `free`'s own prototype already supplied;
`Datatype::compare` ranks `undefined1` above `void`, so the candidate outranked a
correct answer.

## 6. Measurements

### Whole-corpus `decompile-all`, 16 stripped binaries, 7,523 functions

8 projects at **O2 and O0** (the O0 half was added in review — the
statement-fold class below only shows there).

| | `byte` | `void` |
|---|---|---|
| functions changed | 1,038 (13.8%) | 1,034 (13.7%) |
| index rewrite (`(p+k)` → `&p[k]`) | 3,049 + 442 + 104 + 20 + 2 hunks | 20 |
| cast added/removed | 27 + 442 + 104 + 2 | 3,169 + 475 + 16 + 20 |
| declaration only | 1,111 | 1,094 |
| add-tree operand reorder | 118 + 20 + 2 | 114 + 16 + 1 |
| declaration set changed | 60 | 32 |
| hand-reviewed (`STATEMENT`/`stmt-fold`/`OTHER`) | 159 | 89 |
| **statements folded or deleted** | **29 hunks, 22 functions** | **2 hunks, 2 functions** |

(The classifier's type regex does not know `bool`, so a lone `bool vN;`
declaration lands in its `STATEMENT` bucket; one byte hunk and two void hunks are
that, and are excluded from the row above.)

Every one of those 31 hunks is a hoisted temp folded into its uses, or a common
subexpression recomputed at each use: the value the vanished statement carried
reappears in the statements that stay. Worked examples, each hand-checked:

```
O0 coreutils fmt sub_406b (byte)           # a statement is deleted and a block collapses
-  if (*(long *)(a0 + 0x20) != dat_18120) {
-    a1 -= *(int *)(a0 + 0x14);
-    v1 += ((long)(a1 * 10) * (long)(a1 * 10)) / 2;
-  }
+  if (*(char **)&a0[0x20] != dat_18120)
+    v1 += ((long)((a1 - *(int *)&a0[0x14]) * 10) * (long)((a1 - *(int *)&a0[0x14]) * 10)) / 2;

O2 gzip sub_a870 (byte)                    # one address temp becomes two recomputations
-  v1 = (unsigned short *)(a0 + (unsigned long)(v3 + 0x101) * 4);
-  sub_43b0(*v1,v1[1]);
+  sub_43b0(*(unsigned short *)&a0[(unsigned long)(v3 + 0x101) * 4],
+           *(unsigned short *)((long)&a0[(unsigned long)(v3 + 0x101) * 4] + 2));
```

also O2 gzip `sub_9f80` (a loop-invariant bound deleted and recomputed inside the
loop), O2 tar `sub_3b890` (`*(uint4 *)(a0+0x18) | *(uint4 *)(a1+0x18)` split into
two temps with the `|` recomputed at both uses) and O2 diffutils `sub_9e40`
(`a0 += v2;` deleted, `v2` folded into every subsequent index). The pre-review
bundle claimed **0** statements moved or deleted; that was false — the
classifier's own `STATEMENT` bucket was non-zero, and the O2-only sweep could not
see the O0 cases at all.

**One consequence worth naming on its own.** Folding a temp that holds a
*call-bearing* load duplicates the call in the text: O0 grep `sub_eae1` gains
three textual occurrences of `sub_c845(` in the `byte` arm (1,537 → 1,540 over
the binary). The p-code still calls once; read as C it looks like two calls. The
`off` arm already shows the same shape elsewhere in that function, so this is the
existing explicit-varnode heuristic reacting to a type change rather than
something new — but it is a readability regression and it is the only structural
counter in the whole sweep that moves.

**Structural control.** Counts of `// Function:` headers, `goto`, `label_*:`,
`return`, `while`, `if` and `sub_xxxx(` call sites are byte-identical in all
three arms on all 16 binaries, with that one exception. Line and semicolon counts
are *not* — the previous bundle's control was the set of counters that happens
not to move, so both are now recorded per binary in `record.json`
(`structural_control.line_and_semicolon_deltas`; totals −17 lines `byte`,
+1 `void`, over 7,523 functions).

**Seven-pass ceiling.** Re-measured after the guard with a throwaway instrumented
build: identical in all three arms on all 16 binaries.

### Bidirectional `typesweep` — the ship gate

8 projects × O0 / O2 / O2-noinline, 444 slices, 10,748 functions,
`DECBENCH_NO_CACHE=1`, both arms the same binary, base = `origin/main`
at 12c31465 (which carries `libctypes` default-**on**, hence a perfect base of
959 rather than the 889 an earlier pre-#658 run reported).

| arm | PERFECT | aggregate | onto perfect | off perfect | improved | **worsened** |
|---|---|---|---|---|---|---|
| `byte` | 959 → 964 (+5) | 3037.05 → 3045.57 (+8.51) | 5 | 0 | 15 | **0** |
| `void` | 959 → 959 (+0) | 3037.05 → 3069.92 (+32.87) | 0 | 0 | 100 | **0** |

Controls: baseline vs published `type_match` 88.44% agreement; every function
whose two arms hand the metric identical variables scored identically (8,802 /
8,803, 0 discrepancies); and per-**variable** from `--dump-decisions`, over
**55,932** GT-matched comparisons, **0 regressions** in either arm (32 / 165
improvements).

### What `worse 0` does and does not mean

Every binary in that corpus is gcc-built ELF from the coreutils family, and the
shape the rule has to refuse is nearly absent from it. Over the 16-binary sweep,
exactly **four** sites spell a named global subscripted by a parameter
(`dat_24580[a2]` in O0 dash, `dat_9fcd0[a2]` twice in O0 tar, `dat_1f548[a2]` in
O2 dash) and all four are byte-identical in all three arms; every other `x[aN]`
in the corpus is a pointer that was already typed as one. The guard ablation
moves 9 of 7,523 functions, none of them scored differently.

So `worse 0` is a statement about gcc-built ELF coreutils-family code, not about
the rule. The class this corpus cannot see is pinned by a stage control instead,
and the three-line `gcc -O2` repro in §4 is what a reader should run against
their own corpus.

### Speed

Interleaved min-of-N `decompile-all` over all three arms on a quiet box
(`scripts.pipeline.timeit`'s off/on pair cannot express a three-valued option):
`fmt` -O2, n=13 — `byte` −0.21%, `void` −0.04%; `ls` -O2, n=11 — `byte` −0.10%,
`void` +0.27%. Flat.

## 7. Default flip

**Flipped to `void`** after `protoorder` (#669) landed; the evidence is
`default-on-evaluation.md`. The earlier recommendation below was made on a tree
without `protoorder`, where `void` moved no function onto perfect; with the
callee's `void *` reaching its call sites, `void` moves 204 of the 444-slice
corpus's 10,748 functions onto perfect and 85 more up, `byte` 7 and 21, neither
any down.

The withdrawn opt-in verdict, kept for the record: an earlier draft of this
bundle recommended flipping `void` on; that was withdrawn because the evidence
was one corpus, gcc-built ELF of the coreutils family, containing four instances
of the class the rule has to get right. The flip sweep adds the ARM32 firmwares
`chibios` and `freertos` (a second target, bare-metal images) to the
whole-corpus diff, and the four sites that spell a named global subscripted by a
parameter (O0 and O2 `dash`, O0 `tar`) were re-checked with the default flipped:
all four print the same in both arms. The grade is still pinned by
`tests/stages/kuna-ptrfromuse.xml` (`globalindex`, `blindindex`).
