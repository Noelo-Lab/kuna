# fieldtype — a synthesized field some access holds as a pointer

The lane was given the address-of cast family, `(char **)&x`, `(int *)&x`,
`(long *)&x`, as "synthesized-record fields read at a type other than the
field's declaration". The census below says where that family actually is, and
which part of it a better field type removes. The option shipped here is the
part that is structsynth's to fix; the rest is measured and handed on.

## 1. Census

### 1.1 The address-of family, by what `x` is

`census2.py` over the castbench arm of main 632437155, the 4,815 functions kuna
and IDA both emit (the castbench counter's own cast grammar):

| what the cast's operand is | kuna | example |
|---|---:|---|
| an element of a pointer to a scalar, `&p[k]` | 1,843 | `*(int *)&v36[0x15]` over `unsigned long *v36` (ls `struct fileinfo`) |
| a member of an opaque libc shell (`FILE`, `tm`, `obstack`, `stat`, `lconv`, ...) | 763 | `**(char **)&v26->field_0x8` over `lconv *v26` (sort `main`) |
| a member of a synthesized `struct_N` | 65 | `fstat(a0,(stat *)&a2->field_0x4[4])` (a nested record) |
| a stack local | 58 | `waitpid(v11,(int *)&v9,0)` |
| **total** | **2,783** (IDA 1,864) | |

The task's witness, sort -O2 `v13 = **(char **)&v26->field_0x8`, is `lconv`'s
`decimal_point`: `v26` is `localeconv()`'s `lconv *`, an opaque shell with no
members, so every access names the pseudo-member `field_0x8` behind a cast.
IDA prints `v7->decimal_point`. That class is `libctypes glibc`'s (the published
layouts; opt-in, docs/features/libctypes/glibc.md; castobject's analysis
measured `FILE` alone at 67 functions fewer, 0 more). The element class is a
record kuna types as an array of words; IDA prints the same accesses as
`*((_DWORD *)v36 + 42)`, one cast each, so it is not a gap against IDA.

### 1.2 Every cast on a synthesized record's member

`census4.py` over `decompile-project` exports of the 45 binaries (so the
declared field type is read from the exported header), shared functions: **362
casts**, of which the address-of form is 51. The value form is the larger:

| declared field | the cast reads it as | casts |
|---|---|---:|
| an integer or undefined word | a pointer (`(char *)a0->field_0x0`, `free((void *)a0->field_0xb0)`, `(*(void *)a0->field_0x40)(...)`) | ~135 |
| an undefined word spelled `unsigned long` | `(unsigned long)`, an unsigned compare | 44 |
| a pointer | another pointer (`(struct_2 *)a2->field_0x8` over `unsigned long *`) | ~36 |
| a filler array or a sign-contested word | anything | the rest |

### 1.3 Why structsynth declared the field so

`KUNA_FIELDTYPE_TRACE=1` prints every synthesized field with every access of
its width and what the program does with the value (`ftanalyze.py`). Over the
45 binaries, 7,650 distinct fields:

| | fields |
|---|---:|
| no access carries a pointer | 5,340 |
| declared an integer or undefined word, some access carries a pointer | 358 |
| declared a pointer, every pointer access agrees | 1,844 |
| declared a pointer, two pointers disagree | 105 |

The rule was "the first access of the widest width decides". At -O0 a field is
reloaded for every use, so the first load is often the one an integer add or a
compare with a constant reads: sort's `fillbuf` declares `buffer.buf` `long`
beside a `memmove (void *)` and two compares with a `char *`; ls's `free_ent`
declares the security context `long` beside `free (void *)` and
`freecon (char *)`; a function pointer compared with zero before the call is
declared `unsigned long`, and the call prints as `(*(void *)a0->field_0x40)(*v3)`,
which is not C.

### 1.4 What IDA prints at the same addresses

IDA types these parameters as integer arrays and casts at the use:

```c
/* IDA, sort -O0 fillbuf */
memmove((void *)*a1, (const void *)(*a1 + a1[1] - a1[4]), a1[4]);
ptr = (void *)(*a1 + a1[1]);
/* IDA, ls -O0 free_ent: void **a1 */
if ( a1[22] != &unk_2B020 ) ... free(a1[22]); ... freecon(a1[22]);
```

Castbench per function (kuna off -> on, IDA): fillbuf 22 -> 19 (28), free_ent
2 -> 0 (1), find -O2 0xa040 20 -> 12 (34), tar -O2-noinline 0x28d20 24 -> 11
(38), tar -O0 0x2faf0 19 -> 11 (29). The tar pair is a `struct tar_stat_info *`
field kuna reads by the word: `*(int8 *)(a0->field_0x18 + 0x140)` becomes
`a0->field_0x18[0x28]`, the same address.

## 2. The rule (`p5_types/kuna_fieldtype.rs`)

- structsynth keeps every access of a field's widest width with its value
  (`Evidence::record_access`).
- The evidence is what the program does with a value: an access counts when its
  value, or a copy or cast of it, is a load or store address (at most after a
  constant offset or a scaled index), an indirect-call target, an argument a
  type-locked pointer parameter takes, or what a function declared to return a
  pointer returned (`held_as_pointer`). The type the value carries does not
  count: structsynth reads it from the variable the value was merged into, and a
  register merged across a number and a string is typed `char *`.
- Among the accesses that count, a field none of whose accesses carries a float
  is declared as the most specific pointer they carry (record/named aggregate/
  code > scalar > undefined > void), when that pointer is the only one of its
  rank or the only one of its rank a declared parameter the value is handed to
  names, and the first access does not already carry a pointer whose pointee
  outranks it.
- A field some access divides, takes a remainder of, shifts, multiplies (not by
  -1), compares with sign, sign-extends or converts to a float keeps its type
  (`used_as_number`), and is not promoted by the next rule.
- A field compared, for equality or unsigned order, with a field the rule
  declared `T *` is `T *` too (`mergelines_node`'s end pointers).
- An access whose value is an address formed from the record's own base
  (`&rec[1]`) is not evidence: its type is the base's pre-synthesis guess
  (gnulib's scratch buffer keeps `void *data` only with this rule).
- Integer accesses are unchanged; the printer casts where an operation needs a
  number.

### 2.1 Why the evidence is a use, not a type

The rule first took any access's type, merged ones included. Over a slice
disjoint from the one it was built on (e2fsprogs, dash, kmod, zlib, shadow, dpkg)
that declared real integers `char *`: e2fsck -O2-noinline `expand_percent_expression` hands
`ctx->num` (`__u64`) and `ctx->str` to one `fprintf`, the register is one
variable typed `char *`, and `problem_context.num` became `char *`, which reached
`handle_nomem(..., char *a2)`; at -O2 `check_dir_block`'s `db->blk` took a merged
local's `char *` the same way; typesweep 4 better, 2 worse. Constructed cases
added an index once passed to `write`'s `void *`, a hash of a pointer kept in an
`unsigned long`, and a signed sentinel compared with a pointer field. With the
evidence restricted to uses and the number test, all of these keep their integer
types, the disjoint typesweep is 454 -> 454 perfect with 0 better and 0 worse,
and the 444-slice typesweep 1,674 -> 1,674 with 0 and 0. The price is the wins
that rested on a merged type: castbench 32,073 -> 31,974 (53 functions fewer by
134, 14 more by 35) against 31,900 for the first rule.

Variants measured and not taken:

- without the compare rule: the loop tests of `mergelines_node` cast the end
  pointers back to `long *`;
- with `void *` evidence ignored (first rule): 243 fewer casts over all 20,230
  functions against 297; `void *` is still the truer declaration of a freed
  field than `long`;
- with proofs of an undefined pointee ignored (this rule): 32,014 instead of
  31,974; it drops `mergelines_node`'s +20 but also tar's word-read records
  (`a0->field_0x18[0x28]`), which are the same addresses in the pointer's own
  terms;
- replacing a first-access pointer with a proven one of lower rank: sort -O2
  `fillbuf`'s `char *` buffer became memmove's `void *` and gained casts; the
  first access now wins when its pointee outranks the proof.

## 3. The convergence sweep fix (`p4_calls/kuna_calleevote.rs`)

Measuring the option exposed a pre-existing defect: the batch's convergence
sweep forgot a callee's whole caller statement when one input named a
superseded record, so tar's `exclude_add_pattern_buffer (struct exclude *,
char *buf)`, redone for its record, printed `buf` as `unsigned long`. The sweep
now forgets only the inputs that name the record. With `fieldtype off` the fix is
byte-identical to main on all 45 castbench binaries.

## 4. What is left

- libc shell members (763 shared sites): `libctypes glibc` (the published
  layouts), measured in docs/features/castobject/analysis.md.
- Records typed as an array of words (1,843 `&p[k]` sites): a record for the
  pointer (structsynth's base selection), not a field type.
- A pointer difference scaled by a record size the field's pointee does not
  have prints both sides cast (`(long)a - (long)b >> 5` over `long *`); and a
  field proven only as a pointer to undefined words is read into locals that
  stay `long` (sort -O0 `mergelines_node`, +20), the largest function moving
  the other way.
- An integer that is only ever added and handed to a declared `void *` parameter
  cannot be told from an address and is declared `void *`; the option never
  makes a field an integer, so an index whose one -O2 load feeds both a table read
  and `write` stays `void *` as the first-access rule already declares it.
- kuna spells a code pointer `void *`, so a call through a field so typed is
  not compilable C in either arm (`(*a0->field_0x8)(...)`); pre-existing.
- A pointer stored through an `unsigned long *` prints without the cast C
  requires (`*a1 = v2[0]`, e2fsck -O0 `ext2fs_init_dblist` once the option moves
  its callee's types); main prints about 251 such stores on the castbench set,
  so it is the printer's, not this option's.

## 5. The default

Off. Every flip criterion passes except the lane's layout gate: fields-only
precision .8713 -> .8693 (880 -> 878 of 1,010 claimed fields), because the ledger
shares a record between readers only on exact field-type agreement and the
option re-types a field in one reader and not in another
(default-on-evaluation.md). With the option off the output is byte-identical to
main on all 45 castbench binaries; the convergence-sweep fix of section 3 changes
nothing there either.
