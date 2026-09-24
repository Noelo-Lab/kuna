# castarith — pointer arithmetic stays in pointer terms

## 1. The problem

`coreutils ls -O0` `file_or_link_mode` (`sub_53e7`) on main `dbe854ba3`:

```c
unsigned int sub_53e7(void *a0)
{
  if ((dat_2b358) && (*(char *)((long)a0 + 0xb9)))
    v1 = *(unsigned int *)((long)a0 + 0xac);
  else
    v1 = *(unsigned int *)((long)a0 + 0x30);
```

Six casts where IDA prints three. The cast census (`castcensus/.scratch/{measure,
engine,ida}.md`) measured this `*(T *)((long)p + K)` round trip as the largest single
source of kuna's casts: 3,771 casts on the 4,815 functions kuna and IDA share, a shape
IDA prints 0 times. The ops are `INT_ADD` from the start (`PTRSUB -> INT_ADD` demotion
fires 0 times): a `void *` base has no field at `K` and no element size, so P5 builds
neither a `PTRSUB` nor a `PTRADD`, and the cast pass prices the `INT_ADD` as integer
arithmetic: `(long)` on the pointer input (`get_input_cast` default arm,
`cast_standard`) and `(T *)` on the sum (`cast_output`).

## 2. Census (STEP 1), `castbench --set small` (fmt/ls/sort/du x O0,O2)

Every printed `INT_ADD` the cast pass reaches, by the final rule's verdict:

| verdict | ops | |
|---|---:|---|
| converted, cast base | 1,547 | `*(T *)((long)p + K)` -> `((T *)p)[k]` |
| converted, bare base | 2 | `p[k]`: `p`'s declaration already points at `T` |
| kept: no pointer operand | 3,925 | integer arithmetic |
| kept: variable index | 282 | `(long)p + i * S` |
| kept: `void *` sum leaves the function | 29 | a call argument or return value |
| kept: address shared by several accesses | 24 | |
| kept: aggregate element | 20 | `(struct_N *)((long)p + K)`, `char (*)[16]` |
| kept: sum not a pointer | 16 | |
| kept: offset not whole elements | 14 | `*(unsigned int *)((long)a0 + 0x6a)` |
| kept: `table[i]` | 3 | sort -O0 `*(char *)((long)v6 + 0x23260)` |
| kept: sum read as an integer | 1 | |

Converted element sizes: 8 bytes 835, 1 byte 310, 4 bytes 308, 2 bytes 96. Retargeted
casts and unsettled stores occur 0 times here; both have unit tests. Two cases section 4
describes came later: a base that is a shared implied cast (0 here, 21 lines on the
disjoint set of section 5) and a sum assigned to an integer-declared variable (2 lines
over all 201 binaries).

`NoPointer` is integer arithmetic: neither operand is pointer-typed (IDA's own
`*(T *)(x + K)` with `x` an integer, 7,542 sites on IDA's side). `NonConstant` is a
pointer plus a variable index (`(long)a0 + (long)v1 * 4`), kept in integer form; a
follow-up could fold `i * sizeof T` into the index.

## 3. The change and its rendering

`kuna_castarith::rewrite`, called from `action_set_casts` after the upstream
PTRADD/PTRSUB repairs and before the op is cast, turns `INT_ADD(p, #K)` into
`PTRADD(base, #K/sizeof T, #sizeof T)`. The printer renders it as it renders every
PTRADD: `((unsigned int *)a0)[0x2b]` under a load or store, `&((T *)p)[k]` as a value.
One rendering, `((T *)p)[k]`, chosen for three reasons. It is the spelling the printer
already uses for native typed-base arithmetic (`a0[0x18a]`, `&v7[1]`), so a converted
site reads like a native one. `arraynotation` (default on) already chose subscripts over
`base + index` for kuna's pointer arithmetic, so `*((T *)p + k)` would be a second
spelling of the same op. And it needs no printer change, so `arraynotation off` still
gives upstream's `(T *)p + k` for a value.

`T` is the value a lone LOAD reads or a lone STORE writes through the sum (the type the
upstream LOAD/STORE input casts would have recast the pointer to), unless the sum's own
pointee is the same type or an integer of the same width; otherwise the sum's own
pointee, with a `void` pointee stepping in bytes for a value that stays in the function.

## 4. Value preservation

The address is unchanged by construction: `k * sizeof(T) == K` exactly (the rule
requires divisibility, `K` read as signed), the base is converted pointer to pointer with
no integer in between, and the element size is the access width, so every load and store
moves the same bytes. Alignment is unchanged too: `K` is a multiple of `sizeof T`, so
`p` is `T`-aligned exactly when `p + K` is. The refusals exist where the printed C could
otherwise convert a value or cost a cast:

- a bare base `p[k]` only when the printer's declaration of `p` points at `T` or at an
  integer of `T`'s width (`printc::declared_variable_type`); a float element never
  counts, since converting it changes the bits;
- a STORE of a value whose defining op the pass has not cast yet: that op may retype it
  (a float op's result becomes `float`), and an integer element would then convert it.
  Allowed only when the opcode fixes the kind (float arithmetic for a float element,
  integer arithmetic for a non-bool integer element);
- a sum shared by several LOADs/STOREs; a sum read as an integer (integer arithmetic, a
  store into an integer slot) or assigned to a variable declared as an integer (tar -O0
  `sub_64e58` keeps `v2 = v2 + 1` for its `int8 v2` stack slot, where the pointer form
  would print `v2 = &((uint1 *)v2)[1]`); a `void *` sum passed to a call or returned,
  whose `char *` would reach calleevote's whole-program vote on the callee (measured:
  before this refusal, whole-binary runs retyped one callee's `void *` parameter to
  `char *`);
- a constant that names a global, and an address-like constant beside an integer cast to
  a pointer (`table[i]`; sort -O0 `*(char *)((long)v6 + 0x23260)` stays).

Two base forms need no new conversion. An implied cast read only by the add is
retargeted to `T *`. An implied cast that other ops read too is left alone and the new
`(T *)` cast reads that cast's input: bash -O2 `sub_48380` prints `((uint1 *)a0[1])[10]`
for the old `*(uint1 *)(a0[1] + 10)`, not `((uint1 *)(unsigned long *)a0[1])[10]`. Both
casts only reinterpret the same pointer-sized bits, and the integer form converted that
same integer.

castimplied (#721) leaves out a widening whose operand's C type the text states, and a
converted load is a subscript: its operand is `((T *)p)[k]` where it was `*(T *)(...)`.
castimplied now reads that type from the subscript's cast base, so `(int)*(unsigned char
*)((long)a0 + 0x11)` becomes `(int)((unsigned char *)a0)[0x11]` and not `(int)(unsigned
int)((unsigned char *)a0)[0x11]`, and rsyslogd -O0 `sub_5770a`'s `(int8)*(uint2
*)((int8)a0 + 0x48)` becomes `(int8)((uint2 *)a0)[0x24]` and not `(int8)(int4)(uint4)((uint2
*)a0)[0x24]`. The type it reads is the one the printed cast states, exactly as for
`*(T *)p`. With `castarith off` the output is byte-identical to the base on all 201
binaries below, so the new reading changes nothing there.

Compiled round trips (`decompile_all_cli.rs
a_pointer_plus_whole_elements_round_trips_through_the_printed_c`, fixture
`castarith_x86_64.c` built with gcc -O0, clang -O0 and gcc -O2, option on and off), 23
functions: loads and stores of 1, 2, 4 and 8 bytes signed and unsigned, float and double
loads and stores, a negative offset, a non-whole offset (kept), a pointer passed on,
compared and stepped in a loop, a base typed as another pointer, a record base (keeps
`->field`), a subtracted pointer (kuna types it an integer, kept), a byte widened for a
signed compare and a word widened for an index (castimplied through the subscript), and
an integer base (kept). In all six arms the printed C, compiled and run, prints exactly
what the binary prints. The shared-cast base has 1 unit test (a whole-program type
context produces it; a small program does not).

## 5. Whole-corpus hunks, both arms of one build

Every changed line is classified by `hunks.py` (beside this file). It reverse-applies the
rewrite to the new line (`((T *)X)[k] -> *(T *)((long)X + k*sizeof T)`, `&((T *)X)[k] ->
(T *)((long)X + K)`, a bare `X[k]` checked against `X`'s printed declaration, the pointer
width and the console spellings taken from the file) and requires the old line back. Two
sets: castbench full (45 binaries: coreutils fmt ls sort du cp tail wc, grep, gzip,
diffutils x4, tar, find, x O0/O2/O2-noinline), and 156 binaries disjoint from it (kmod
dash bzip2 cronie dpkg libedit sysvinit zlib libbsd libacl libselinux nuttx freertos
minipig mydoom x0r-usb libexpat base-passwd e2fsprogs rsyslog iproute2 bash, x
O0/O2/O2-noinline; x86-64, ARM32 and PE32). The off arm is byte-identical to the base on
all 201.

| class | castbench full | disjoint | reading |
|---|---:|---:|---|
| functions changed | 2,337 | 10,021 | |
| exact reverse | 12,993 | 57,312 | the rewrite, undone, gives the old line |
| store element sign | 79 | 1,307 | `*(int *)X = e` vs `((unsigned int *)p)[k] = e`: the same bytes stored |
| assignment, outer cast | 130 | 167 | `v = (A *)((long)p + K)` vs `v = &((B *)p)[k]`: same address, only the assigned expression's pointer type |
| parentheses only | 2 | 40 | |
| void, in bytes | 5 | 12 | `(void *)((long)v + 1)` vs `&((char *)v)[1]` |
| bare base, checked against its declaration | 12 | 11 | `v27[4]` for `unsigned long *v27` |
| read by hand | 24 | 114 | below |
| control flow changed (taildup) | 0 | 4 functions | below |

Read by hand, all the same address: 41 nested subscripts (`&(*(uint2 **)((int8)a0 +
8))[2]` -> `&((uint2 **)a0)[1][2]`), 21 integer bases under a shared cast (`*(uint1
*)(a0[1] + 10)` -> `((uint1 *)a0[1])[10]`, one cast each), 13 `wchar_t` elements (a size
the classifier does not know), 24 stack-probe loop steps whose old outer cast was
`(char *)` (`&((unsigned long *)v4)[-0x200]` is `+ -0x1000`), 16 pointer-to-array
elements (`((char (**)[16])v3)[10]` is `+ 0x50`), 10 assigned pointers or stores inside
an expression whose element differs in sign only, 11 assignments whose outer cast
differs, and 2 offsets with a `U` suffix.

**One structuring decision moves.** The rewrite only removes ops (the integer form's two
casts become one or none), and three P8 passes run after the cast pass and bound the tail
they duplicate by its op count, `CAST`s included: `gotoreduce` (8 ops, all counted),
`taildup` (12 printed ops, `kuna_taildup.rs:310`) and `crossjumprevert` (8 printed ops).
A return tail just over the budget can fit once its casts go, and its `goto` becomes a
duplicated `return` (`// return-dupe`). taildup preserves the value by design: the copy
runs the same ops on the one path that reached the goto. `structural.py` (beside this
file) compares each function's control-flow signature (gotos, labels, return-dupe tails,
returns, `if`/`else`/loops/`switch`/`break`/`continue`) between the arms: 0 of 20,230
functions on castbench full, 4 of 55,066 on the disjoint set, all four this one shape
and all four taildup's (with `--option taildup off` both arms print the same gotos):

| function | castarith off | castarith on | taildup off, either arm |
|---|---|---|---|
| kmod -O2-noinline `sub_b200` | 1 goto | 0 gotos, 1 return-dupe | 1 goto |
| rsyslogd -O2 `sub_69b50` | 1 goto, 2 return-dupe | 0 gotos, 3 return-dupe | 3 gotos |
| rsyslogd -O2-noinline `sub_69570` | 1 goto, 2 return-dupe | 0 gotos, 3 return-dupe | 3 gotos |
| libedit -O0 `sub_1fe22` | 4 gotos, 1 return-dupe | 3 gotos, 2 return-dupe | 4 gotos, 1 return-dupe |

kmod `sub_b200`, off: `if (v1 <= v3) goto label_b287;` ... `label_b287: ...`; on: `if
(v1 <= v3) { v2 = sub_e920(((unsigned int **)a0)[8]); // return-dupe ... return 0; }`.
Making the P8 budgets ignore `CAST` ops would remove this coupling, but that is a change
to taildup's own policy, for its own PR.

**JSON surface.** `surface.py` (beside this file) compares `decompile-all --json` between
the arms on 13 binaries (fmt ls sort du O0+O2, bash O2, kmod O2-noinline, bzip2 O0, gzip
O2, dash O2), 6,690 functions: 0 signatures, 0 variables (name, kind, type) differ, 41,302
variables in both arms. The one loss is provenance: a subscript is a surround token and
carries no op (upstream `PrintLanguage::emitOp`, the same for every native subscript), so
an instruction whose only printed ops were the add and the access through it maps to no C
line. Of 175,526 mapped instructions, 4 in 3 functions drop out of `line_mappings` (dash
-O2 `sub_12b50` `0x12e16 mov 0x10(%rdi),%r15`, line 66 `[0x12e0f, 0x12e16, 0x12e1a]` ->
`[0x12e0f, 0x12e1a]`), and 7 addresses in 3 functions drop out of `variables[].addresses`.
Nothing is gained or moved.

## 6. Measurements

**castbench full** (45 binaries x O0/O2/O2-noinline, the 4,815 functions kuna and IDA
both emit; both arms of this build, `--option castarith off` vs default; the off arm is
byte-identical to a build of the base):

| | casts | /kloc | vs IDA |
|---|---:|---:|---:|
| IDA | 37,821 | 155.4 | 1.000 |
| main `5458b7ab5` (castimplied landed) | 43,673 | 229.2 | 1.155 |
| **castarith on `5458b7ab5`** | **38,703** | **203.1** | **1.023** |
| main `dbe854ba3` (measured before the rebase) | 45,126 | 236.8 | 1.193 |
| castarith on `dbe854ba3` | 40,138 | 210.6 | 1.061 |

On the current base: -4,970 casts (-11.4%), lines unchanged (190,585). 952 functions
have fewer casts, **0 have more**, 3,863 unchanged. Per level against IDA: O0 1.198 ->
1.039, O2 1.168 -> 1.057, O2-noinline 1.096 -> 0.971 (below IDA). The shape moved is
exactly the census's: `(i64) <var>` about -4,900, and each `(T*) <paren>` becomes a
`(T*) <var>`. On the 156 disjoint binaries of section 5 (every function, not only those
IDA shares): 400,905 -> 332,946 casts (-17.0%), 10,011 functions fewer, **0 more**.

**444-slice typesweep** (coreutils grep gzip diffutils bzip2 findutils tar shadow x
O0/O2/O2-noinline, metric pinned to decbench `625e892` via `final-c/pindb.py`, both arms
the same binary, `--option castarith off` vs default): on `5458b7ab5`, 10,748/10,748
functions, **1,615 -> 1,615 perfect (15.03%), mean .3697 -> .3697, 0 improved, 0 worse, 0
on or off perfect**, every level and project unchanged (before the rebase, on
`dbe854ba3`: 1,609 -> 1,609, the same zeros). No variable or argument is added or
removed (section 5, JSON surface), so the arity and phantom counters cannot move.
The rewrite runs after every type and variable is final and changes no declaration, so
this is the expected null result; the cast count is the target.

**Speed** (one binary, `--option castarith off` vs default, `decompile-all --json`, arms
interleaved and order-alternated, one warm-up discarded, min-of-15; `speed.py` beside this
file; other lanes and a workspace test run shared the box, load 6-41):

| binary | off min ms | on min ms | delta (min) | median of ratios |
|---|---:|---:|---:|---:|
| fmt -O2 | 4,218.8 | 4,208.7 | -0.24% | -0.57% |
| ls -O2 | 14,026.3 | 13,976.9 | -0.35% | -2.61% |
| sort -O2 | 14,624.5 | 14,711.3 | +0.59% | +1.05% |
| bash -O2 | 89,838.4 | 92,482.3 | +2.94% | +5.65% |

The bash -O2 run shared the box with a workspace test run (load 20-41); re-measured alone
at load 4-9 (`speed-rerun.json`) it is **-0.16%** (min) and -0.17% (median of ratios). Worst
min delta over the four binaries: +0.59% (sort -O2); budget +5%. Measured on the build
before the rebase onto `5458b7ab5`; the rebase changed no line of the rewrite.
