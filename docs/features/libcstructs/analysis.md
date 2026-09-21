# `libctypes`, second round — the libc structs the corpus actually holds

The first round of `libctypes` was designed from the platform headers: the aggregate
slots of the two built-in prototype tables, named. This round was designed from the
other end — from the ground truth of the benchmark corpus — and asks one question per
variable: **of the pointer-to-named-struct variables the debug twins hold, which ones
could a libc declaration have named, and which declaration would it have been?**

## How the pool was mined

`mine_pool.py` beside this file walks the DWARF of the unstripped twin of every one of
the 444 corpus slices (`coreutils grep gzip diffutils bzip2 findutils tar shadow x`,
`O0`/`O2`/`O2-noinline`), over exactly the functions decbench scores and with exactly
`decbench.metrics.type_match`'s DIE walk (it reuses `final/gtclass.py`), and records the
pointee STRUCT TAG of every variable whose class is `ptr_struct`. That is 11,687
variables over 196 distinct tags.

Three further inputs decide reachability, none of them a guess:

* **which names a STRIPPED image carries** — `readelf --dyn-syms` over the stripped
  copy of each slice, split into undefined (imported) and defined-and-exported;
* **which function each variable's own function calls** — `objdump -d` over the twin,
  per function, restricted to names the platform declares;
* **which aggregate each libc function traffics in** — `gcc -aux-info` over the
  installed headers with `_GNU_SOURCE` + `_FORTIFY_SOURCE=2` (3,428 declarations), the
  same reduction `kuna_libcsigs` documents. No signature in this round was written from
  memory.

A variable is *reachable* when its own function calls, directly, a libc function whose
declared signature mentions that tag. That is a floor, not a ceiling: `protoorder`
carries a callee's recovered parameter type back to its callers, so a variable one hop
away can still be named. It is the floor that is worth ranking on.

## The ranked pool

`ranked-pool.md` beside this file is the full table. The head of it:

| GT struct tag | GT vars | reachable | a slot the table already had | NEW | the new slots |
|---|---:|---:|---:|---:|---|
| `obstack` | 431 | 371 | 0 | **371** | `_obstack_newchunk`, `_obstack_begin` |
| `timespec` | 47 | 14 | 2 | **12** | ~~`utimensat`, `futimens`~~ dropped, see below |
| `passwd` | 204 | 107 | 98 | **9** | `getpwent` |
| `re_pattern_buffer` | 9 | 9 | 0 | **9** | `re_compile_pattern`, `re_compile_fastmap` |
| `lconv` | 8 | 8 | 0 | **8** | `localeconv` |
| `_IO_FILE` | 577 | 431 | 425 | **6** | `fread_unlocked`, `feof_unlocked`, `__getdelim`, `fputc_unlocked` |
| `termios` | 22 | 6 | 0 | **6** | `cfgetispeed`, `cfgetospeed`, `cfsetispeed`, `cfsetospeed` |
| `spwd` | 44 | 6 | 0 | **6** | `getspnam` |
| `timeval` | 6 | 6 | 0 | **6** | ~~`utimes`, `futimesat`~~ dropped, see below |
| `utmp` | 12 | 3 | 0 | **3** | `getutent` |
| `group` | 161 | 63 | 61 | **2** | `getgrent` |

Two findings decided the shape of the change.

**`obstack` is the whole story, and in these slices it is not an import.** 431 of the
pool's variables are `struct obstack *`, and 371 of them sit inside a function that
calls `_obstack_newchunk` or `_obstack_begin` directly — but the census of imports finds
those names undefined in **0** of the 444 slices. gnulib links its copy of `obstack.c`
into the program, and the linker exports the symbols from the program itself, so a
stripped `grep`, `tar` or `coreutils` binary carries `_obstack_newchunk` in `.dynsym` as
a DEFINED function. The shipped named tables match imported names only, on purpose, so
nothing in the table could ever have reached it.

Outside the mined 444, the other channel does occur, and it matters: 15 slices of the
same results tree (the five `dpkg` programs at each optimization level) carry
`UND _obstack_begin@GLIBC_2.2.5` and `UND _obstack_newchunk@GLIBC_2.2.5`. glibc's
installed header and gnulib's copy disagree about the size parameters — `int` against
`size_t` — so the two channels take two tables and the twins confirm each: the `dpkg`
twins type those parameters `int` at `/usr/include/obstack.h:184`, the `tar` and `grep`
twins `size_t`.

**The already-named types are nearly saturated.** `stat` has 469 GT variables and 99
reachable ones, and the table already covers all 99: there is no unused `stat` slot in
the platform headers. `_IO_FILE` is 425 of 431 covered. The remaining 146 `FILE`
variables and 370 `stat` ones are not a table problem — they are in functions that never
call a stdio or stat function, and only propagation can reach them.

What is left over is what the metric cannot reach at all: 2,222 `hash_entry`, 1,639
`hash_table`, 453 `predicate`, 369 `tar_stat_info` — program-defined names a stripped
binary does not carry — and three libc-named families that are also linked in rather
than imported and are NOT taken here, because unlike `_obstack_*` their names are
ordinary ones a program may define itself: gnulib's `fts` (`_ftsent`, 100 variables),
shadow's own `gshadow` (`sgrp`, 61) and gnulib's `argp` (`argp_state`, 6).

## What ships

Seven new aggregates, sized from `sizeof`/`_Alignof` on glibc x86-64 and cross-checked
against the corpus's own DWARF, which agrees on every one:

```
obstack 88/8   spwd 72/8   utmpx 384/4   utmp 384/4
re_pattern_buffer 64/8     lconv 96/8    statfs 120/8
```

Sixty-six new slots (61 imported names and the five obstack entry points), every one
of them new to BOTH shipped tables (so `libctypes off` is byte-identical to what it
was), and the obstack pair of tables. Round 4 had ten more; why they were dropped is under
*Measured*. Every kept row either supplies a NEW slot in `ranked-pool.md` or carries a
one-line off-pool note there saying what it costs.

### `LIBC_DEFINED_NAMED`, and why it is five names long

The obstack entry points are matched against a name the image DEFINES as well as one it
imports. The rule that makes the other tables imports-only — a coincidental `fopen` in
an image's own symbol table is that image's function — cannot apply to `_obstack_*`:
that is the implementation-reserved half of `obstack.h`, written only by glibc or by the
gnulib copy of the same file, and both publish the same `struct obstack`.

They publish different size slots for it, though, so the five names live in two tables
of the same shape: `LIBC_DEFINED_NAMED` (`size_t`, seeded only from the names the image
DEFINES and does not also import) and `LIBC_IMPORTED_OBSTACK` (`int`, seeded from the
import channel, by name and by resolver address). Getting that backwards is visible at
the call site rather than in the declaration alone — on a two-line glibc-obstack program
built with `gcc -O2`, a `size_t` slot turns the caller's `void f(int n)` into
`void f(unsigned int n)` and inserts two casts.

The split touches the import channel and nothing else. `decompile-all` over five whole
binaries, the two builds differing only by it: `-O2` grep, `-O2` tar, `-O0` ls and `-O2`
gzip are byte-identical, and `-O2` dpkg-query moves 7 lines — the two declarations, and
one caller that gets its own `int` parameter back:

```
-void _obstack_newchunk(obstack *a0,unsigned long a1)        -long sub_e080(unsigned int a0)
+void _obstack_newchunk(obstack *a0,int a1)                  +long sub_e080(int a0)
-    _obstack_newchunk((obstack *)0x22c560,(unsigned long)a0);
+    _obstack_newchunk((obstack *)0x22c560,a0);
```

`_obstack_allocated_p` is left out: the installed header does not declare it, so there
is nothing to reduce — the same rule that rejected `__underflow` in the first round.
`_obstack_free`'s declaration is the one that needed a step of reasoning: the header
declares it as `__obstack_free`, a macro gnulib re-points at `_obstack_free`
(`/usr/include/obstack.h:194`). Same declaration, same line, one documented renaming.

The size slots are `size_t`, not the `int` the installed glibc header spells at
`obstack.h:184`. Two published declarations of the same symbol exist — glibc's, and the
gnulib copy every obstack in this corpus is compiled from, whose `_OBSTACK_SIZE_T` is
`size_t` — and the corpus's own debug info settles which applies (`size_t` in every
`tar` and `grep` twin). Both pass the value in a register, so only the rendering moves;
`int` would put a truncating cast on every call.

## Measured

### type_match, the 444-slice campaign corpus

`scripts.decbench.typesweep` through `final-c/finalsweep.py`, decbench pinned to
`625e892`, `DECBENCH_NO_CACHE=1`, over the same slices. The base arm is `main` at
`f39f67b6e` built from a clean export of that commit; the other arm is the same commit
plus this branch. (`2e8e407be`, the base the branch now sits on, adds only the opt-in
`stackaddrargtrial`, off by default.) The base arm reproduces the campaign's published
round-D perfect count and mean exactly (1,349, 0.3405), which is the control.

| | off | on |
|---|---:|---:|
| functions scored | 10,748 | 10,748 |
| perfect | 1,349 | **1,353** |
| mean | 0.3405 | **0.3415** |
| true positives | 20,314 | **20,419** |
| false positives | 17,870 | **17,765** |
| false negatives | 27,531 | 27,531 |
| improved / worse functions | — | **71 / 1** |

The rows dropped in round 5 (the four two-element time slots and six `sigset_t` rows,
below) cost 20 improved functions and 0.0004 of mean against the round-4 build, and no
perfect function: 1,353 either way.

The `ptr_struct` ground-truth class, which is what this was aimed at:

| GT class | GT vars | off | on |
|---|---:|---:|---:|
| `ptr_struct` | 14,252 | 422 (2.96%) | **487 (3.42%)** |
| `ptr_char` | 14,645 | 4,204 | **4,233** |
| `ptr_void` | 3,481 | 612 | **615** |
| `struct_val` | 1,769 | 497 | **505** |

The 110 newly-correct variables, by the ground-truth type they match:

```
 47  obstack *      6  termios *   5  FILE *              3  __mbstate_t, void *
 31  char *         5  statfs      4  re_pattern_buffer *  2  int, group *
                                                           1  utmp *, lconv *
```

against 5 newly-wrong, all of them listed:

| slice | function | GT | off | on |
|---|---|---|---|---|
| `coreutils::O0::pinky` | `print_long_entry` | `passwd *` | `passwd *` | `undefined8` |
| `coreutils::O0::pinky` | `print_long_entry` | `char *` | `char *` | `undefined8` |
| `coreutils::O0::pinky` | `print_long_entry` | `char *` | `char *` | `undefined8` |
| `grep::O0::grep` | `EGexecute` | `idx_t` | `long` | `char *` |
| `grep::O0::grep` | `EGexecute` | `idx_t` | `long` | `char *` |

Not one is a named libc struct standing where a correct primitive pointer used
to. The three `pinky` rows are the register-resident case below; the two
`EGexecute` rows are a pointee guess spreading into two index variables.

`spwd` and `utmpx` win nothing on this corpus: their pools sit behind gnulib
wrappers that `protoorder` does not reach. They are kept because the declaration
is derived and correct, not because they scored.

### Arity and fabricated variables

The user's rule for anything that can add or remove a variable. Over the same 444
slices, every one of these is measured from the same runs:

```
exported variables (all functions)   97,350 -> 97,350
variables the metric saw              96,671 -> 96,671
functions with MORE exported vars     2 (+2)
functions with FEWER                  1 (-2)
```

Nothing is fabricated: the change is a retyping, and the three functions whose variable
count moves are the whole of it. At call sites the change REMOVES missing arguments
rather than adding phantom ones: over all 663 x86-64 binaries of the results tree,
`main` prints 196 `fseeko` calls with one argument where the callee takes three
(`return fseeko(a0); // tail-call` in `-O2` gzip), and this branch prints all 348
`fseeko` calls with three; `re_match` goes from 2 one-argument calls to 0. The only
calls that gain an argument are the alloca-frame artifact below.

### The one worse function

`coreutils::O0::pinky` `print_long_entry`, 0.4286 -> 0.2143. Three variables
(`pw`, `project`, `plan`) move from a stack slot to a register, and decbench never
exports a register local, so they stop being scored at all — they are not mistyped, they
are invisible. The trigger is `fread_unlocked` gaining a declaration, which changes what
the O0 frame copies merge with. The same slot wins four `FILE *` elsewhere.

### Dropped: the two-element time slots

`utimensat`, `futimens`, `utimes` and `futimesat` were in round 4 and are not in this
one. Their aggregate slot is `const struct timespec[2]` / `const struct timeval[2]`,
and the vocabulary can only name one element, so the caller's frame object shrinks to
16 bytes of a 32-byte array. gzip `-O2` `lutimens` (`0xee90`) was the visible case: the
neighbouring 144-byte `stat` slot split at 80, and the body read `st_atim.tv_nsec` and
`st_mtim` from locals nothing wrote:

```
main, and this branch:  stat v5;          // stack - 0xc8      lstat(a0,&v5)      v8 = v5._88_8_;
round 4:                char v5 [80];     // stack - 0xc8      lstat(a0,(stat *)v5)
                        long v9;          // stack - 0x78      v4._8_8_ = v9;     <- nothing writes v9
```

The same slot shrank openssh's `struct timeval tv[2]` before `utimes` in 6 `scp`/`sftp`
functions (`unsigned long v28 [3]` -> `timeval v28` plus two detached words), and
lost a whole `stat` declaration in shadow `useradd`/`usermod`. No spelling of the slot
avoids it without an array type in the vocabulary, so the four rows are gone; the
`timespec` pool entries they reached (12 variables, `ranked-pool.md`) are unclaimed
again. `lutimens` is byte-identical to `main` on this branch.

### Dropped: `sigfillset` and the five other `sigset_t` rows

`sigfillset(&sa.sa_mask)` split a `struct sigaction` local at the mask in openssh's
`ssh_signal` (every `-O2` openssh program), so the `sa_flags` store landed outside the
object `sigaction()` is handed. With the row gone, `-O2` `ssh-keygen` `0x4ce20` is
byte-identical to `main`, the object whole and the store inside it:

```
  sigaction v4; // stack - 0x158
  sigfillset(&v4.field_0x8);
    v4._136_4_ = 0x10000000;
  if (sigaction(a0,&v4,&v5) != -1)
```

The five other `sigset_t` rows were checked for the same split before keeping them.
Over the 87 corpus binaries that import any of them, none splits anything. But an
8-line program that hands `&sa.sa_mask` of a `struct sigaction` local to each one
(`gcc -O0`/`-O2`, both arms of `decompile-all`) splits the object under every one of
them: `sigdelset`, `sigismember`, `sigsuspend` and `sigwait` at `-O0`
(`char v1 [136]` plus a detached `v2 = 0x10000000;`), `pthread_sigmask` at both levels
(`sigaction(a0,NULL,(sigaction *)v1); ... if (v2 & 4)` — `sa_flags` read from a local
that `sigaction()` never writes). They buy nothing in the mined pool, so all five are
dropped rather than kept on the corpus's silence. The shipped `sigemptyset`,
`sigaddset` and `sigprocmask` rows split the same way on `main` (sysvinit `wall` prints
`sigemptyset((sigset_t *)&v13[8])` beside a detached `v26 = 0;`); that is a frame-merge
fix, not this PR's.

### Whole-corpus frame check

Both builds over every x86-64 ELF in the results tree — 663 binaries, all 35 projects,
three optimization levels, 199,963 functions — with a checker that flags a function
when a stack object of 16 bytes or more on `main` gains a declaration starting inside
it (a split), or loses a named libc aggregate local (`stat vN;` becoming `char vN [K]`):

```
functions whose text changes           5,338
functions with a split frame object        0
functions that lose a named local          0
```

It also counts named casts onto a char array smaller than the type (`(stat *)v5` on a
`char v5 [80]`): 82 on `main`, 131 here. None of the 49 new ones is a shrink. 34 sit on
an array this branch GREW toward the type's real size by merging words `main` declared
separately (openssh's `struct tm` before `mktime`/`timegm`, 16 -> 32 bytes of 56; grep's
and expr's `regex_t`, one 8-byte word -> 32 bytes of 64), 13 are tar's `struct obstack`
locals now declared as one 16-24 byte array where `main` declared loose words or
nothing, and 2 are arrays of the same size on both arms (bash `sh_regmatch` hands
`char v7 [16]` to `regcomp` either way; this branch only spells the cast).

The checker was calibrated on round 4's rows: over the 108 binaries that import a
signal or utime name, the round-4 build (without `sigfillset`) is flagged in exactly the
functions described above — gzip `lutimens`, the six `scp`/`sftp` `utimes` callers, and
the lost `stat` in shadow `useradd`/`usermod`.

### The trailing argument in alloca functions (`O0` coreutils `cp`)

Round 4 printed `utimensat(v18,v26,&v12,0,v3)` against a four-parameter declaration in
`-O0` `cp` `0x4793`. That is not the declaration being appended to: it is an artifact
`main` already prints for every declared callee in a function that grows its stack
with `alloca`. The call's own pushed return address is read back as one extra trailing
argument, and `main` shows it on the same lines of the same function:

```
v10 = gettext("failed to preserve times for %s",v13[v4 + -8]);
v3 = v13[v4 + -8];
v6 = *__errno_location(v3);
```

Over the 663 binaries, 251 functions carry an alloca stack probe; `main` prints 3,249
declared calls with exactly one extra argument in 179 of them (`free` 768, `memcpy`
416, `__errno_location` 384, `dcgettext` 243, ...). `utimensat` added 12 more in
round 4; with it dropped, this branch adds 9 (`getutent` 6 in sysvinit `wall`/`shutdown`,
`pthread_mutex_init` 2 and `__getdelim` 1 in libselinux), all in functions that
already print the artifact. `argclobber` (on by default since #689) does not remove it,
because the extra value is a stack read, not a register. `-O0` `cp` is in the hunk
classification set below.

### The undefined high half (`dat_4`)

A declared `int` return feeding a caller kuna recovered as returning 8 bytes renders
the upper half of `rax` as `dat_4`: `return CONCAT44(dat_4,fclose(a0));` appears 1,408
times on `main`. This branch adds 121, in 109 functions: 98 `-O0` gnulib `rpl_fseeko`
wrappers (`return CONCAT44(dat_4,fseeko(a0,a1,a2));`, where `main` printed a `void`
`fseeko` in return position), 21 in gnulib's `nanosleep` wrappers in coreutils
`sleep`/`sort`/`tail` at all three levels, and 2 in find's `re_match` caller.
It is the caller's return width that is wrong, and the value really is undefined
there. What the same rows buy: every `fseeko` call gets its three arguments (196 were
printed with one), and `nanosleep`'s `timespec` is one object in coreutils `sleep`
where `main` split it into `v2` and `v4`.

### Whole-corpus output diff

`decompile-all` before and after over 15 whole binaries (7,987 functions; round 4's
14 plus `-O0` coreutils `cp`): 763 functions change, 2,337 hunks, classified by
`classify.py` beside this file into `corpus-hunk-classification.txt`:

```
 626  local renumbering only
 586  synthesized struct_N renumbered (one fewer slot in the ledger)
 443  other, downstream of a changed pointee
 154  signature gains a named pointee
 153  a named cast appears
 110  cast at an offset becomes a field
  93  declaration gains a named pointee
  70  a PLT thunk gains the return the declaration states
  44  other, mentions a named type
  33  a field access gains or loses its width cast
  21  a local's spelling or width moves under the new pointee
   4  field becomes a cast at an offset
```

`-O0` `cp` is 5 functions and 11 hunks: three PLT thunks, `rpl_fseeko`'s signature and
tail call, and `fstatfs` filling a whole `statfs v5` where `main` had
`unsigned long v5 [15]`. Its `utimensat` call is `main`'s four-argument one.

The two largest buckets are renumbering: a synthesized structure that is now a named
libc type leaves the ledger, so every later `struct_N` shifts down one ordinal. That is
the interaction the round was designed around — a named libc type outranks a synthesized
`struct_N` — and it is visible in the four `field becomes a cast at an offset` hunks
too, which are the cost side of it: the named shell is opaque under `opaque`, so a field
the synthesized structure had a member for renders as a width cast at an offset instead.
`i386_pie_nl`'s `build_type_arg` is the clearest single case:

```
-unsigned int build_type_arg(unsigned int *a0,struct_0 *a1,unsigned int a2)
+unsigned int build_type_arg(unsigned int *a0,re_pattern_buffer *a1,unsigned int a2)
-        a1->field_0x0 = 0;
+        *(unsigned int *)a1 = 0;
```

The name is right — the O0 coreutils `nl` twin's own debug info types that
parameter `re_pattern_buffer *` — and four field writes render one level less
directly.

### The `utmp`/`utmpx` rows, measured

The ranked table puts `utmpx` in the "not reachable by a direct call in the 444
slices" column, so the four `getutx*`/`pututxline` slots ship on a pool count
rather than on a sweep delta. They are not unpaid: 24 corpus binaries import
`getutent`, `getutxent` or `pututline`, and the type reaches a caller's own
local, not only the PLT thunk. `-O2` coreutils `who`:

```
-  short *v4;                              +  utmpx *v4;
-  v4 = (short *)getutxent();              +  v4 = getutxent();
-  v3 = *(int *)&v4[2];                    +  v3 = *(int *)&v4->field_0x4;
-  if (((char)v4[0x16]) && (*v4 == 7))     +  if ((v4->field_0x2c) && (*(short *)v4 == 7))
-  v5[0x2f] = *(unsigned long *)&v4[0xbc]; +  v5[0x2f] = *(unsigned long *)&v4->field_0x178;
```

The GT for that variable is `struct utmpx *`, and the `short *` it replaces was
an artefact of the 2-byte `ut_type` read. `-O2` shadow `logoutd` and sysvinit
`runlevel` do the same for `utmp *` (189 and 17 changed lines).

### Pointee sizes are glibc x86-64

Every size in the table is the x86-64 one. As merged, this subsection said that
only pointers take the name, "no stack object is ever re-sized by one", so a
wrong width could only move where a field access is attributed. That was false.
A named pointer handed the address of a frame object gives that object the
aggregate's width. On x86-64 against glibc the growth is right (`char v1 [12]`
becomes `termios v1` where the program touched twelve bytes of the struct
`tcgetattr` fills), but on a 32-bit image it runs past the real object: an i386
`timespec` handed to `clock_getres` as a 16-byte one swallowed two `int` locals,
and an obstack grown to 88 bytes swallowed an `int[12]`. The shipped `stat` row
already did this on i386. #706 names nothing unless the image is an x86-64 ELF
not linked against another C library; see "After merge" below.

### The export fixture swap, re-derived on #693

`libctypes` names `i386_pie_nl`'s only synthesized structure, so the two places
that used that binary as their "a structure is synthesized here" fixture had to
move. #693 then gave `decompile-project` the callee-first schedule, which
decides what a function mints, so the swap was re-derived under it rather than
carried across:

```
i386_pie_nl                        default 0   protoorder off 0   (libctypes off: 1)
structsynthchain_x86_64            default 2   protoorder off 2
itaniumrtti_x86_64.so              default 5   protoorder off 5
explicit_branch_assertion_pe_i386.exe  default 5   protoorder off 5
structsynth_teb_pe_x86_64.exe      default 5   protoorder off 5
```

(`struct struct_N {` blocks in the exported `.h`.) The four fixtures the rust
test iterates all still declare `struct_0` in the arm it runs, and the 32-bit
slot keeps a 32-bit image.

Since #706 `libctypes` names nothing on i386, so `i386_pie_nl` has its
synthesized structure back by default (the `libctypes off: 1` column). The
swapped fixtures still witness the same thing and stay.

### The export's compile-error cost

The option's documented cost is that `decompile-project`'s exported `.c` reads
fields out of types its own `.h` declares incomplete. On the `-O2` `ls` export
that was +58 `cc -fsyntax-only` errors; with this round it is **+106**
(835 off, 941 on, re-measured on the round-5 build with `cc -fsyntax-only -w`), and
the `.h` is still 0 errors in both arms. Seven more
opaque shells is seven more types a body can read a field out of; the header,
which is what the rest of the export depends on, is unmoved.

One of the seven adds an instance of a collision the option already had:
`statfs` is both an aggregate name and a libc function name, so an export that
declares the type and calls the function draws
`'statfs' redeclared as different kind of symbol` — exactly what `main` already
does for `stat` on a five-line `stat(2)` program. The `.h` emitter already
guards its side (`\`statfs\` is a type name above; prototype omitted`); the
body's declaration is the unguarded half, and it is one line per colliding name.
Plain `decompile-all` shows the collision too, without an export in sight: `O0`
shadow `usermod` `sub_111a1` declares `statfs v1; // stack - 0x88` and calls
`statfs(a0,&v1)` two lines later. The typing is right — 120 bytes, the glibc
size — and the two spellings are the same word, so that body does not compile
as printed.

### Speed

Interleaved min-of-15, `decompile-all --max-fn-seconds 120` over five whole binaries,
the two builds (`main` at `f39f67b6e`, and the same commit plus this branch)
alternating on every repetition (`speed.py` / `speed.json` beside this file), measured
while this lane's own workspace suite was running:

| binary | off | on | delta |
|---|---:|---:|---:|
| `O2` tar | 49.753s | 49.020s | -1.47% |
| `O2` grep | 12.405s | 12.527s | +0.98% |
| `O2` ls | 14.614s | 14.632s | +0.12% |
| `O0` grep | 7.141s | 6.983s | -2.21% |
| `O2` sort | 15.025s | 15.743s | +4.78% |

`sort`, the one delta near the budget, re-measured once the suite had finished:
14.655s -> 14.659s, +0.02% (`speed-confirm.json`). Round 5 only removes rows, and
every binary is inside the +5% budget; round 4's worst was tar at +3.49%, +2.68% on
its confirmation run.

Splitting the obstack channel added one more symbol-table walk to the load
(the defined-name set), which is below the noise floor of the path it is on: min-of-9
interleaved `kuna functions` over `-O2` tar, 0.144s before against 0.142s after.

## After merge: the width gate and the fit rule (#706)

A review after this merged found two ways the table put the wrong struct into a
function. #706 fixes both; this section is its measurement. Base is `main`
`eaa19ebbb` (this PR, merged), which reproduces this PR's own published sweep
exactly (1,353 perfect, mean .3415), which is the control. The sweep was repeated
after rebasing onto `a26b99db1` (#704, #705): that `main` scores the same, and
the branch moves it by exactly the same rows.

### The widths only hold on x86-64 against glibc

The section above explains why a wrong width grows a frame object. The i386
witness (`tests/fixtures/libctypes_widths_i386`, stage pass 14) shows both
shapes. On `main`, an 8-byte i386 `timespec` declared as 16 gives
`v1._12_4_ = a0 + 2`, and an `int k[12]` next to an obstack becomes a field of
it. #706 names nothing unless the image is an x86-64 ELF that is not linked
against another C library. Eight non-x86-64 fixtures (i386 `i386_pie_nl`,
`libctypes_widths_i386`, `declaredlibcproto_i386`; ARM `armlibcmain_le32`,
`armlibcmain_got_le32`; PE `pe_iatincode_i386.exe`; MIPS32; Mach-O
`macho_imports`) and the x86-64 PE `pe_imports.exe` now decompile byte-identically
to `main --option libctypes off`. That also returns `pe_imports.exe`'s
`__report_error` va_list spill stores (`v4 = v1; v5 = v2; v6 = v3;`), which the
`vfprintf` row had dropped. The stream slots are refused with the rest, because
they mint the same 216-byte `FILE` and `structdefs` reports that size. Two
`tests/cli` probes pinned that size on `i386_pie_nl` and were retargeted to
`datasyms_faillog_x86_64`.

### A struct that starts with a libc aggregate is not that aggregate

`kuna_libcfit` declines a table's struct vote when the caller reaches the
argument outside the struct, and propagation refuses the same type on the same
evidence (spec 05). Over the 663 x86-64 binaries:

```
functions indexing a named libc pointer past element 0   71 -> 8
  (obstack 9 -> 0, pthread_mutex_t 31 -> 0, sigaction 8 -> 8, stat 3 -> 0,
   group 3 -> 0, spwd 3 -> 0, re_pattern_buffer 3 -> 0, FILE 4 -> 0, ...)
lines with an element-indexed field `[k].field_0x`      1,141 -> 584
```

The eight left are sdiff's and dpkg's `struct sigaction` tables, which really are
arrays of sigaction and are stepped by exactly 152. grep's `kwsincr` prints
`sub_ed20(void *a0,...)` and `kwsprep` `sub_f210(unsigned long *a0)`, which is
what `libctypes off` gives. rsyslog's instance records and libselinux's 120-byte
records holding a mutex at +0x38 lose their `pthread_mutex_t *`.

The first cut of the rule counted any step that was not a whole number of
aggregates as an overrun. That wrongly declined four real uses (stty's `termios
*mode`, logoutd's stack `termios`, grep `main`'s stack `stat` at `-O2` and
`-O2 -fno-inline`), because a word-at-a-time struct copy or a phi between two
fields also has a step smaller than the struct. A step smaller than the aggregate
now proves nothing, and all four are back.

### Measured

Same instrument, pin and slices as above:

| | main | #706 |
|---|---:|---:|
| perfect | 1,353 | 1,353 |
| mean | 0.34149 | 0.34145 |
| true positives | 20,419 | 20,405 |
| false positives | 17,765 | 17,779 |
| improved / worse functions | — | 0 / 1 |
| exported variables | 97,350 | 97,350 (no function gains or loses one) |
| `ptr_struct` correct | 487 | 474 |

The one worse function is grep `-O0` `kwsprep`, 0.6667 -> 0.3077. gnulib's obstack
macros copy `&kwset->obstack` into locals of their own (`__h`, `__o`, `__o1`), and
after copy propagation every call reads `kwset` itself. Those locals took `obstack
*` only by propagation from `kwset`, and with `kwset` declined nothing gives it to
them. Their ground truth is `obstack *`, and nothing in the function tells them
apart from the kwset pointer they copy. That is the whole 14-variable loss.
`O2` `kwsprep`/`kwsincr` score the same as before: their `obstack *a0` was never
credited against the `kwset *` ground truth.

Whole-corpus diff: 136 of 199,963 functions change
(`followup-hunk-classification.txt`). 81 render the same statements and calls
with byte offsets instead of element indexes, 35 differ only in types and casts,
12 are the `pthread_mutex_destroy` PLT thunks, and 8 fold or keep a register
temporary with the same stores and calls in the same order. No call is gained or
lost anywhere. `libctypes off` is byte-identical to `main` (checked on O2 grep,
wc, sort, logoutd and O0 grep). `undefinedN` locals stay at 1,269. The +69 the
review counted for this PR is from its first round and unchanged here.

### The mutex rows

`pthread_mutex_destroy` is dropped. It scored nothing and its single argument is
recovered either way (110 calls with one argument in both arms).
`pthread_mutex_init` is kept for its arity: without it, 12 corpus calls print
`pthread_mutex_init(x)` without the `NULL` attribute and 2 gain a fourth
argument. Its container case, a record whose first member is the mutex, is what
the fit rule declines.

### Speed

Interleaved min-of-15, `decompile-all --max-fn-seconds 120`, `main` against #706
alternating on every repetition (`followup-speed.py`), on the binaries the rule
fires in: O2 grep 11.83 -> 11.71 s (-0.97%), O2 sort 14.11 -> 13.99 s (-0.88%),
O2 wc 4.45 -> 4.44 s (-0.43%), O0 grep 6.80 -> 6.71 s (-1.34%), O2 rsyslogd
25.93 -> 25.79 s (-0.55%). The walk runs only for a vote or a propagation step
that would put a libc aggregate pointer on a Varnode.
