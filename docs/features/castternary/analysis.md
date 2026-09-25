# castternary: analysis

## The gap

A textbook base64 decoder looks every input character up in a decode table
unless it is the `=` pad. gcc -O0 keeps each conditional's result in a register,
`iteregion` prints the diamond as a conditional, and under `iteexpr` (on in the
`aggressive` preset `auto` picks below 500 KiB, so the default rendering of a
small binary) the arm is the sign-extending load itself:

```c
v2 = (*(char *)(v7 + a1) != '=') ? (int)*(char *)(a0 + (unsigned long)*(unsigned char *)(v7 + a1)) : 0;
v3 = (*(char *)(a1 + v7 + 1) != '=') ? (int)*(char *)(a0 + (unsigned long)*(unsigned char *)(a1 + v7 + 1)) : 0;
v4 = (*(char *)(a1 + v7 + 2) != '=') ? (int)*(char *)(a0 + (unsigned long)*(unsigned char *)(a1 + v7 + 2)) : 0;
v5 = (*(char *)(a1 + v7 + 3) != '=') ? (int)*(char *)(a0 + (unsigned long)*(unsigned char *)(a1 + v7 + 3)) : 0;
```

(`castternary_gcc_O0_x86_64`, `b64_decode`.) The same diamond without
`iteexpr`, or from clang -O0, prints as if/else, and there `castimplied` already
leaves the `(int)` out: `v5 = *(char *)(a0 + ...);`. Only the conditional form
keeps it, because `castimplied` refuses the arms of a conditional.

## Why castimplied refuses the arms, and why the refusal is too broad

The arms of `?:` are not converted to the destination's type; they are converted
to their common type, the usual arithmetic conversions of the two (C11 6.5.15p5,
6.3.1.8, integer promotion first). Leaving one arm's cast out can therefore change
the type of the whole conditional:

```c
long v = c ? (long)i : u;   /* long: i = -5 stores -5 */
long v = c ? i : u;         /* unsigned int: i = -5 stores 4294967291 */
```

But when the conditional's type is the cast's target type both with and without
the cast, the conditional performs the conversion itself: `c ? (int)ch : 0` and
`c ? ch : 0` are both `int`, with the same value, for any `char ch`. Nothing that
reads the conditional can tell the two apart.

## The census

`census.py` over the castbench main arm (`c960fb18d`, 45 binaries x 3 levels):
176 casts sit at the top of a conditional's arm and cover the whole arm. 64 are
pointer casts (`(char *)sub_x(...)` against `NULL` or a string, the unknown-return
residue), 76 are the float-to-unsigned-long idiom (`(long)(d - K) ^ 1 << 63 :
(unsigned long)d`, a float conversion), and 36 integer:

| shape | casts | outcome |
|---|---:|---|
| `(unsigned long)uc` against `(unsigned long)tolower(...)` (diff O0, 2 lines) | 2 + 2 | the widening goes, the sign-changing `(unsigned long)tolower` stays |
| `0xffffffff : (unsigned int)a1` with `unsigned char a1` (tail O0) | 1 | dropped |
| `(unsigned long)v2 : (unsigned long)call()` into a pointer-declared merged variable (ls O0, 5 lines) | 10 | kept: the destination is not an integer |
| `(unsigned int)(a != b) : 0xffffffff`, `... : (unsigned int)(v != 0)` | 8 | kept: a comparison (`bool` source) |
| `(unsigned long)a6 : 0` with `long a6`, `2 : (unsigned int)v21` with `int v21` | 5 | kept: sign change |
| `(char)v : (char)toupper(...)` (2 lines) | 4 | kept: narrowing |
| `(unsigned long)*v + 0x8000 : (long)*v` | 1 | kept: the conditional is `unsigned long`, not the cast's `long` |
| `(long)(int expression) : dat % 0x15180` (find O2, O2-noinline) | 2 | kept: an expression whose type the text does not state may be `unsigned` |
| `(long)(v33 - v15) : NULL` | 1 | kept: a pointer arm |

The user-visible shape, a promoted byte against an `int` constant, does not occur
in that corpus at all; gcc -O0 over byte tables is where it lives (the fixture,
ARM firmware, base64 decoders).

## What IDA prints

IDA splits these diamonds into if/else and relies on C's conversion of the
assignment, with no cast on the byte:

```c
// cleanflight O0 0x803533c mavlinkSendSystemStatus
if ( sub_8015ED4() ) v2 = (short)sub_8015EE4(); else v2 = -1;
if ( sub_8015E88() ) v1 = (char)sub_8015D60(); else v1 = 100;
// kuna, option off -> on
v4 = (sub_8015ed4()) ? (int)(short)sub_8015ee4() : -1;   ->   (short)sub_8015ee4() : -1
v1 = (sub_8015e88()) ? (int)(char)sub_8015d60() : 100;   ->   (char)sub_8015d60() : 100

// crazyflie O0 0x804edf0 saturateSignedInt16
return (short)(int)v0;
v1 = (...) ? (int)(short)(int)a0 : -0x7fff;               ->   (short)(int)a0 : -0x7fff

// diffutils diff O0 0xe2f8
if ( v17 ) v1 = tolower(v14); else v1 = v14;
v11 = (v8) ? (unsigned long)tolower((unsigned int)v1) : (unsigned long)v1;   ->   ... : v1
```

kuna keeps the conditional; with the option it keeps exactly IDA's casts there
(the truncations `(short)`/`(char)` stay, they change the value) and drops the
widening IDA never prints.

## Why the rule is safe

For an arm `(T)x` with `T` the conditional's type `R` both as printed and with
the cast left out:

- the conditional converts `x` to `R = T` (6.5.15p5), which is the conversion
  `(T)x` spelled, so the taken arm's value is unchanged;
- the other arm is converted to the same `R` as before, so its value is unchanged;
- the conditional's type is unchanged, so its reader sees the same type and value.
  The reader is always an assignment here (`iteregion` prints only `dest = ...`),
  and an integer conversion depends only on the value (6.3.1.3), so an integer
  destination of any type stores what it stored before.

The only risk is computing `R` wrong, which would happen if an arm's C type were
assumed. The rule never assumes: an arm's C type is the set of promoted types it
may have, and `R` must be the same single type for every combination. Literals
are typed from the exact token the printer writes (base, suffix, magnitude,
C11 6.4.4.1); an expression the text does not type may be any promotion of its
width. A float, pointer, enum, `bool` source, or a destination not declared as
an integer stops the rule.

## Evidence

- Compiled round trips: `castternary_x86_64.c` (base64 decoder, `char`,
  `unsigned char`, `short`, `int`, `unsigned int` arms against `int`, negative,
  `0xffffffff`, `0xffffffffffffffff`, `3000000000` constants and a second cast, fed
  bytes 0x80..0xff and negative values), gcc and clang builds, the printed
  functions compiled by gcc and clang at -O0 and -O2, option off and on: every
  build prints the binary's output. Dropping both casts in `arm_both`
  (`c ? a0 : a1` over `int` and `unsigned int`) is caught: it prints `4294967291`
  where the binary prints `-5`.
- `tests/stages/kuna-castternary.xml`: 10 assertions; with the option off in both
  passes 8 fail.
- castbench, typesweep, speed and the corpus sweep: `record.json` and
  `default-on-evaluation.md`.
