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
casts and unsettled stores occur 0 times here; both have unit tests.

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
otherwise convert a value:

- a bare base `p[k]` only when the printer's declaration of `p` points at `T` or at an
  integer of `T`'s width (`printc::declared_variable_type`); a float element never
  counts, since converting it changes the bits;
- a STORE of a value whose defining op the pass has not cast yet: that op may retype it
  (a float op's result becomes `float`), and an integer element would then convert it.
  Allowed only when the opcode fixes the kind (float arithmetic for a float element,
  integer arithmetic for a non-bool integer element);
- a sum shared by several LOADs/STOREs; a sum read as an integer (integer arithmetic, a
  store into an integer slot), where `(long)&((T *)p)[k]` would cost one cast more than
  the integer form; a `void *` sum passed to a call or returned, whose `char *` would
  reach calleevote's whole-program vote on the callee (measured: before this refusal,
  whole-binary runs retyped one callee's `void *` parameter to `char *`);
- a constant that names a global, and an address-like constant beside an integer cast to
  a pointer (`table[i]`; sort -O0 `*(char *)((long)v6 + 0x23260)` stays).

Compiled round trips (`decompile_all_cli.rs
a_pointer_plus_whole_elements_round_trips_through_the_printed_c`, fixture
`castarith_x86_64.c` built with gcc -O0, clang -O0 and gcc -O2, option on and off):
loads and stores of 1, 2, 4 and 8 bytes signed and unsigned, float and double loads and
stores, a negative offset, a non-whole offset (kept), a pointer passed on, compared and
stepped in a loop, a base typed as another pointer, a record base (keeps `->field`), and
a subtracted pointer (kuna types it an integer, kept). In all six arms the printed C,
compiled and run, prints exactly what the binary prints.

## 5. Whole-corpus hunks (castbench full, 45 binaries, both arms of one build)

The off arm is byte-identical to main's castbench arm. Every changed line of the on arm
is classified by `hunks.py` (beside this file). It reverse-applies the rewrite to the new
line (`((T *)X)[k] -> *(T *)((long)X + k*sizeof T)`, `&((T *)X)[k] -> (T *)((long)X +
K)`, a bare `X[k]` checked against `X`'s printed declaration) and requires the old line
back:

On the current base (`5458b7ab5`): 2,338 functions changed; 13,246 changed lines.

| class | lines | reading |
|---|---:|---|
| exact reverse | 12,986 | the rewrite, undone, gives the old line |
| assignment, outer cast | 130 | `v = (A *)((long)p + K)` vs `v = &((B *)p)[k]`: same address, only the assigned expression's pointer type |
| element signedness | 63 | `*(short *)(...) = x` vs `((unsigned short *)p)[k] = x`: a store of the same bytes |
| bare base, checked against its declaration | 12 | `v27[4]` for `unsigned long *v27`, `&v7[2]` for `char *v7` |
| void, in bytes | 5 | `(void *)((long)v + 1)` vs `&((char *)v)[1]` |
| parentheses only | 2 | `!(*(T *)(...))` vs `!((T *)p)[k]` |
| read by hand | 48 | 12 stack-probe loop steps whose old outer cast was `(char *)` (same address), 19 stores whose element spelling differs in sign only (`int4` vs `unsigned int`) or whose old offset carried a `U` suffix, 5 assignments inside an expression (outer cast only), 4 pointer-to-array elements (`((char (**)[16])v3)[10]` is `+ 0x50`), 2 pointer compares, 1 nested subscript (`&(*(char **)(a1 + 8))[1] == &((char **)a1)[1][1]`), and 6 `(unsigned long)(long)((int *)a0)[2] % a1` where castimplied left out the inner `(long)` of the integer form (`(unsigned long)*(int *)((long)a0 + 8)`): the same conversion of an `int`, the same cast count |

Nothing outside the documented effect.

No function signature, declaration, parameter or JSON variable type changes (12
binaries, 6,355 functions: 39,262 variables and 11,219 parameters in both arms, 0
signature differences). Three functions' JSON `variables[].addresses` lose one
instruction: a load printed as a subscript carries no token of its own (upstream
`PrintC::opLoad` under array notation), so an instruction that performs only that load
drops out of the variable's use-address list. Types and names are unchanged.

## 6. Measurements

**castbench full** (45 binaries x O0/O2/O2-noinline, the 4,815 functions kuna and IDA
both emit; both arms of this build, `--option castarith off` vs default; the off arm is
byte-identical to a build of the base):

| | casts | /kloc | vs IDA |
|---|---:|---:|---:|
| IDA | 37,821 | 155.4 | 1.000 |
| main `5458b7ab5` (castimplied landed) | 43,673 | 229.2 | 1.155 |
| **castarith on `5458b7ab5`** | **38,709** | **203.1** | **1.023** |
| main `dbe854ba3` (measured before the rebase) | 45,126 | 236.8 | 1.193 |
| castarith on `dbe854ba3` | 40,138 | 210.6 | 1.061 |

On the current base: -4,964 casts (-11.4%), lines unchanged (190,585). 946 functions
have fewer casts, **0 have more**, 3,869 unchanged. Per level against IDA: O0 1.198 ->
1.039, O2 1.168 -> 1.057, O2-noinline 1.096 -> 0.971 (below IDA). The shape moved is
exactly the census's: `(i64) <var>` about -4,900, and each `(T*) <paren>` becomes a
`(T*) <var>`.

**444-slice typesweep** (coreutils grep gzip diffutils bzip2 findutils tar shadow x
O0/O2/O2-noinline, metric pinned to decbench `625e892` via `final-c/pindb.py`, both arms
the same binary, `--option castarith off` vs default): on `5458b7ab5`, 10,748/10,748
functions, **1,615 -> 1,615 perfect (15.03%), mean .3697 -> .3697, 0 improved, 0 worse, 0
on or off perfect** (before the rebase, on `dbe854ba3`: 1,609 -> 1,609, the same zeros).
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
