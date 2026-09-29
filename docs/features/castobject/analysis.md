# castobject — an object read or passed at a type other than its declaration

The cast family this lane was given is `(T *)&x`: an object whose address is
taken at a type other than the one it is declared at. The census below says
where those casts are, which of them a better declaration removes, and which
belong to another lever. The option shipped here is the one that is in reach
and sound; the census is the other half of the result.

## 1. Census

`addrcensus.py` (beside this file) walks the castbench arm of `origin/main`
0096e984d (45 decbench binaries at -O0/-O2/-O2-noinline) with the castbench
counter's own cast grammar, keeps every pointer cast whose operand starts with
`&`, and records what follows the `&`, the context the cast sits in, and the
declared type of the base variable.

| what `x` is | all 45 binaries | shared with IDA | example |
|---|---:|---:|---|
| an element of a pointer at another width, `&p[k]` | 4,093 | 1,611 | `*(long *)&v50[8]` over `int *v50` |
| a member of an opaque libc aggregate (`FILE`, `obstack`, `stat`, `tm`, ...) | 2,310 | 763 | `*(char **)&a1->field_0x28` over `FILE *a1` |
| a member of a synthesized record | 427 | 57 | `mbsinit((mbstate_t *)&a0->field_0x1[3])` |
| a stack local | 210 | 64 | `waitpid(v2,(int *)&v5,0)` over `unsigned int v5` |
| a global | 14 | 0 | |
| **total** | **7,054** | **2,495** | |

At the function level against IDA on the 4,815 shared functions, only one class
is a real gap:

| functions holding the class | functions | kuna casts | IDA casts | ratio |
|---|---:|---:|---:|---:|
| pointer element | 326 | 11,288 | 10,894 | 1.036 |
| libc aggregate member | 117 | 4,051 | 3,322 | **1.219** |
| stack local | 38 | 2,031 | 1,667 | 1.218 (the 64 sites are 3% of it) |
| synthesized-record member | 23 | 703 | 868 | 0.810 |

### 1.1 Pointer elements (65% of the family): a record, not a declaration

`*(long *)&v50[8]` is a pointer whose pointee was typed by one access
(`*v50 = 0`) and read at other widths everywhere else. 1,331 of the 1,611 shared
sites read a width different from the declared pointee (794 wider, 537
narrower); 87 differ in sign alone. IDA prints the same accesses as
`*((_QWORD *)v50 + 4)`, one cast each, which is why these functions sit at
1.036x IDA. The type that removes these casts is a record for the pointer
(`structsynth`), not a re-declared pointee: re-pointing `int *` to `long *`
trades the casts on one width for casts on the other.

### 1.2 libc aggregate members (31%): the published layout `libctypes glibc` already has

The sites the task statement quotes as synthesized-record fields,
`**(char **)&v26->field_0x8` in sort's `main` and `*(char **)&v4->field_0x28` in
every inlined `putc`, are members of the opaque libc shells `libctypes opaque`
interns: `lconv`, `FILE`, `obstack`, `stat`, `tm`, `group`, `passwd`,
`re_pattern_buffer`, `dirent`. A shell has its width and no members, so every
access prints the pseudo-member `field_0xN` behind a cast. IDA prints
`a1->_IO_write_ptr`. The declaration that removes the cast is the published
member, and `libctypes glibc` installs exactly that for nine of the shells. It
stays opt-in for defect classes that belong to `stat`, `timeval` and `tm`
(docs/features/libctypes/glibc.md). Measured here with castbench, full set:

| arm | shared casts | vs IDA | O2 casts | functions fewer / more |
|---|---:|---:|---:|---|
| main 0096e984d | 35,588 | 0.941 | 13,135 | |
| `libctypes glibc` | 35,041 | 0.926 | 12,724 | |
| `libctypes glibc` with `FILE` alone (experiment, not shipped) | **35,253** | 0.932 | **12,818** | **67 / 0** |

`FILE` alone is 61% of what `glibc` removes, 317 of it at -O2 (where the
per-statement density gap is), with no function gaining a cast. It is the
largest lever in this family and it is a `libctypes` decision, so it is recorded
here and not taken.

### 1.3 Synthesized-record members (2% of shared): mostly genuine disagreement

The 427 sites split into: an address into a filler array handed to a libc call
that declares an aggregate (`mbsinit((mbstate_t *)&a0->field_0x1[3])`, 77;
`getdelim`, `_obstack_newchunk`, `regexec`, `fstat` another 35), a nested
aggregate member structsynth does not mint because its evidence is 1/2/4/8-byte
accesses; two one-byte flags merged into an `unsigned short` field by one
2-byte access (`*(char *)&a0->field_0x34 = 1`); one `struct_N` reused for
pointers into different objects (gzip's `printf_parse`); and 2 `waitpid`
statuses held in a record. None of these is a field every access reads at one
type.

### 1.4 Stack locals (3% of shared): the lever shipped here

Of the 64 shared sites, 49 are call arguments. Two kinds disagree for real and
keep their cast: a character buffer the body also writes as words
(`v7 = 0x3f3f; strcmp((char *)&v7,v12)`, `strlen((char *)&v54)` after
`v54 = 0x206b2d`), and a slot the compiler reuses for two objects (sort's `main`
passes one slot as a `sigaction *` and as a `stat *`). The rest is the
out-parameter the callee declares, and the frame model's ordering is what
declares it at the other sign:

```c
/* kuna (sort -O0 0x48cf) */                  /* IDA, same address */
unsigned int v5; // stack - 0x28              int stat_loc; // [rsp+10h] [rbp-20h] BYREF
v6 = waitpid(v2,(int *)&v5,(unsigned int)(a0 == 0));
                                              v9 = waitpid(v1, &stat_loc, a1 == 0);
if ((v5 & 0x7f) || ((int)v5 >> 8 & 0xffU))    if ( (stat_loc & 0x7F) != 0 || BYTE1(stat_loc) )
```

`RangeHint::preferred` keeps the more specific of two same-width hints, and the
ported ordering ranks `unsigned int` ahead of `int`, so the bit tests' reading
wins over the callee's declaration. IDA declares `int stat_loc` at all six
addresses this option changes.

## 2. The mechanism

`p6_variables/kuna_castobject.rs` re-types the frame hints of one slot to the
pointee the callee declares, before the layout decision, when:

- every alias-checker base landing on the slot is a plain `&local` read only
  by direct calls at locked `T *` parameters naming one plain 4- or 8-byte
  integer `T` (never a format-string position, never an override);
- every hint inside the slot starts at it, covers it whole, and is an integer
  or unknown of `T`'s width, and the layout would not stretch the slot into an
  array (a hint starts where it ends);
- no reader wants the other sign: every sign-sensitive reader (`<`, `/`, `%`,
  `>>`, a widening, a piece above the lowest byte), reached through copies,
  phis and the operators whose result keeps the operand's type, computes with
  `T`'s sign; the walk stops at a value stored into another stack slot, which
  is read at its own declaration. A signed re-declaration also needs no
  `+ - * <<` reader and no top-bit constant in `== != & | ^`.

Only the declaration moves. The cast pass computes with the merged variable's
type, and a slot stored from a register (`int st = init;`) keeps a member typed
like the stored value, so the variable can stay `unsigned int` under an `int`
declaration. With every sign-sensitive reader agreeing, each cast the cast pass
puts in against `unsigned int` is an `(int)` the declaration makes an identity,
and the assignment, the argument and the return convert by themselves. The
first version counted votes instead (a net gain moved the slot), which printed
the unsigned readers of such a slot with no cast: `int v2; ... v2 >> 0x14` for a
logical shift, `0x100 <= v2` for an unsigned compare, and `ZEXT48(v1)` for a
zero-extension. A merge can also bring in readers the walk never saw: a value
stored into the slot joins the slot's variable through the COPY. So after every
merge, at the head of `ActionSetCasts`, `reconcile` repeats the walk over every
member of the variable of each slot whose variable's type is not `T`, and when a
reader disagrees it puts the slot and its Symbol back at the variable's type.

The -O2 `WEXITSTATUS` is a logical shift (`v >> 8 & 0xff`, `(unsigned char)(v
>> 8)`), which reads the slot unsigned, so those slots stay as they are.

## 3. Two wrong outputs the round trips found (both arms, not this option)

The round-trip fixture covers a callee writing through the out-parameter. It
found two defects that print the same with the option off and on; both are
left out of the round trip and reported:

1. A value saved from an out-parameter before a second call that rewrites it
   is read after that call. gcc -O2:

   ```c
   int was_state(void) { int old; pthread_setcancelstate(1, &old);
                         int was = old; pthread_setcancelstate(old, &old); return was; }
   ```
   ```
   pthread_setcancelstate(1,&v1);
   pthread_setcancelstate(v1,&v1);
   return v1;          /* the binary returns the value saved before the second call */
   ```
   The raw p-code carries an INDIRECT for the slot at the first call and none
   at the second.

2. A slot clang -O2 allocates with `push %rax` reads back the pushed register.
   In `castobject_x86_64.c` (`exit_code`, `status_order`, `reaped` at clang
   -O2), `push %rax; lea 4(%rsp),%rsi; call waitpid; mov 4(%rsp),%ecx` prints
   `v3 = (int)((unsigned long)v1 >> 0x20);`, the high half of the pushed `rax`,
   after the call that wrote the slot.
