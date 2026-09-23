# `libcwiden` -- the rest of the libc signature table

## 1. The opportunity

`libcsigs` grew kuna's built-in libc prototype table from 27 entries to 206. Which
names it took was decided by one question: *which callee, if typed, would move
`type_match` on a case a rival decompiler already gets right?* That question is a
good one and it is also narrow. It never asks which callees the corpus actually
**calls**, so a whole class of imports was left out -- the `*at` family, the
SELinux surface GNU coreutils links against, the account/xattr/socket/pthread
names -- and a caller that only ever hands its argument to one of those keeps the
inferred integer, because nothing in the program says what the pointer addresses
and the callee has no declaration to say it either.

The round-F ptr_char census (`gap.md`) measured the cost on the campaign's 444
slices: of the 878 ground-truth `char *` variables binja gets right and kuna does
not, **158 sit in a function that calls a libc import kuna cannot type**, across
35 distinct names. 33 of the 35 are reachable under the existing rejection rule.

A runnable witness, against an unpatched tree:

```
$ kuna decompile-all <O0>/coreutils/stripped/ginstall --addr 0xb57a
unsigned int sub_b57a(unsigned int a0,unsigned long a1,unsigned int a2)
{
  if ((((a2 & 0xf000) != 0xa000) && (!sub_1db12())) && (faccessat(a0,a1,2,0x200)))
    return 0;
  return 1;
}
```

`a1` is `writable_destination`'s `char const *file`. `faccessat` is the only
thing in the function that touches it.

## 2. Which names -- the same corpus, a broader question

The selection rule is mechanical and re-runnable (`corpus_imports.py`,
`cands.py`):

> a name is admitted when it is an **undefined** `FUNC` symbol (and not also
> defined) in **at least three** of the 665 dynamically linked binaries of the
> frozen decbench corpus, kuna's shipped tables do not already carry it, and its
> platform declaration reduces **whole** to the width-stable `Ty` vocabulary.

1,775 distinct undefined names appear across those binaries. 264 of the ones the
444-slice subset imports are already carried. Of the remainder, 338 clear the
three-binary bar and reduce cleanly; five are then rejected for a 64-bit return
(below), leaving **333**.

The head of the admitted set, by binaries importing it: `fdatasync` 93, `execvp`
93, `chroot` 90, `utime` 84, `setreuid` 79, `lckpwdf`/`ulckpwdf` 78, `setregid`
73, `getopt` 72, `__open_2` 70, `__fgets_chk` 68, `getppid` 63, `chmod` 60,
`iswcntrl` 60, `gai_strerror` 60, `execv` 57, `strsignal` 57, `initgroups` 57,
`faccessat` 54, `dlopen` 54, `freecon` 51.

## 3. Which signatures -- the same provenance, unchanged

Every entry is reduced from a machine-readable declaration, never written from
memory. `tu.c` is a translation unit including the platform headers with
`_GNU_SOURCE` and `_FORTIFY_SOURCE=2` plus `<selinux/selinux.h>`;
`gcc -aux-info` emits 3,182 prototypes from it, and `reduce.py` maps each `NC`
line to the `Ty` vocabulary by a fixed rule, rejecting the declaration **whole**
when any slot has no honest spelling. The reducer is in this folder and its
output (`selected.json`) is what `gentable.py` turns into the Rust table, so the
table is regenerable rather than hand-kept.

### Rejected rather than approximated

The width rule is unchanged: a slot must be `void`, exactly 4 bytes, or exactly
pointer-width on every ILP32/LP64 target. What it costs here, by corpus binaries:

| name | binaries | rejected because |
|---|---:|---|
| `lseek` | 321 | returns `__off_t` |
| `qsort` | 141 | takes `__compar_fn_t` |
| `strtoll` | 130 | returns `long long int` |
| `strtoumax` | 108 | returns `uintmax_t` |
| `signal` | 84 | returns `__sighandler_t` |
| `posix_fadvise` | 69 | takes `off_t` |
| `time` | 57 | returns `time_t` |
| `strtoimax` | 45 | returns `intmax_t` |
| `strtoull` | 39 | returns `uintmax_t` |
| `iconv` / `iconv_open` | 24 | `iconv_t` |
| `context_*` (libselinux) | 24 | `context_t` |
| `llabs` | 18 | returns `long long int` |

`strtoll` and `strtoumax` are the two most widely imported names kuna still
cannot type at all, and both are rejected only for their **return**: their
parameters (`const char *`, `char **`, `int`) are exactly the `char *` evidence
this lane is chasing. Admitting them needs a fixed-width 64-bit member of `Ty`,
which is a separate decision with its own risk surface (is `long long` exactly 8
bytes on every target kuna has a compiler spec for?) and is deliberately not
taken here. `a_sixty_four_bit_return_is_still_rejected` pins all five as absent
so a later widening pass cannot let them drift in silently.

`__libc_start_main`, `__cxa_atexit`, `__cxa_finalize` and `getrandom` are in
every binary and in no header the TU reaches; they are absent for want of a
declaration, not by the width rule.

## 4. The wrongness axis is unchanged

The pass already applies an entry only to a name the image **imports** and does
not itself define. Measured over the same 665 binaries, 20 of the 333 new names
are also *defined* somewhere in the corpus, all by the same three images: 17 by
libselinux itself (which implements the genuine API, so the prototype would be
right anyway) and `putenv`, `posix_memalign` and `wcsdup` by its gnulib half.
The guard makes all 20 inert on the image that defines them.

## 5. Why no new option

`libcsigs` is the option. It exists to ablate exactly this table, its `off` arm
declares exactly the names the 28-entry base table carries, and that is still
true with 333 more rows in it. It does NOT ablate the placeholder strip below,
which is ungated and runs in both arms, so `off` is not `origin/main`. Adding a
second flag would make the two halves of one measured table separately ablatable
for no reason a reader would use. The stage
test `tests/stages/kuna-libcwiden.xml` pins both arms.

## 6. What it costs

The first measurement of this branch said a declared prototype "can still pick up
one recovered argument past its declared arity" -- **24 of the 445 call sites** of
a new name, against a pre-existing **575 of 13,036** for the names the table
already carried. The second number was the tell: the rate did not belong to these
rows, or to the table, but to a bug in the engine that only a declared arity can
make visible. `Heritage::clearStackPlaceholders` was an unported stub, so the
stack-pointer placeholder `ActionFuncLink` hangs on every CALL was never taken
off a call whose input list is locked. It printed as one argument past the
declared arity, reading the slot the `call` pushed its return address into, and
on `origin/main` ginstall's function at 0xbdda alone carries 45 of them --
every one in that binary (`phantom_args.py` over its whole-binary output).

So the cost is now zero and something on main is fixed with it: over 12 whole
binaries the trailing argument goes **149 to 0** (100 of them on main's own
already-declared names), counted by the committed `phantom_args.py`, with
function count, gotos and labels identical. Its own price is ten ground-truth `_Bool` variables of 65,715
that spelled `bool` and now spell `char`, in two functions whose frame is
alloca-shaped -- the same shape that kept the placeholder. Against all of it, 220
import thunks go from `(void)` to their real declared arity and 554 parameter
slots gain a type. See `record.json`.
