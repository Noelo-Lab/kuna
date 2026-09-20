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
| `timespec` | 47 | 14 | 2 | **12** | `utimensat`, `futimens` |
| `passwd` | 204 | 107 | 98 | **9** | `getpwent` |
| `re_pattern_buffer` | 9 | 9 | 0 | **9** | `re_compile_pattern`, `re_compile_fastmap` |
| `lconv` | 8 | 8 | 0 | **8** | `localeconv` |
| `_IO_FILE` | 577 | 431 | 425 | **6** | `fread_unlocked`, `feof_unlocked`, `__getdelim`, `fputc_unlocked` |
| `termios` | 22 | 6 | 0 | **6** | `cfgetispeed`, `cfgetospeed`, `cfsetispeed`, `cfsetospeed` |
| `spwd` | 44 | 6 | 0 | **6** | `getspnam` |
| `timeval` | 6 | 6 | 0 | **6** | `utimes`, `futimesat` |
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

About seventy-five new slots, every one of them new to BOTH shipped tables (so
`libctypes off` is byte-identical to what it was), and one new table.

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
`625e892`, `DECBENCH_NO_CACHE=1`, the two arms being two builds of this tree over the
same slices. The baseline arm reproduces the campaign's published round-D numbers
exactly (1,349 perfect, mean 0.3403), which is the control.

Measured twice, at two bases: first against `2e28ece4a`, then re-run in full after the
rebase onto `724381149` (`argclobber` default-on, #689), whose base arm is the
origin/main sources of `kuna_libctypes.rs` and `protos/mod.rs` built in this tree. Every
number below, the per-variable tables included, came out identical on both.

| | off | on |
|---|---:|---:|
| functions scored | 10,748 | 10,748 |
| perfect | 1,349 | **1,353** |
| mean | 0.3403 | **0.3417** |
| true positives | 20,295 | **20,436** |
| false positives | 17,889 | **17,748** |
| false negatives | 27,531 | 27,531 |
| improved / worse functions | — | **91 / 1** |

The `ptr_struct` ground-truth class, which is what this was aimed at:

| GT class | GT vars | off | on |
|---|---:|---:|---:|
| `ptr_struct` | 14,252 | 422 (2.96%) | **496 (3.48%)** |
| `ptr_char` | 14,645 | 4,204 | **4,239** |
| `ptr_void` | 3,481 | 612 | **615** |
| `struct_val` | 1,769 | 497 | **506** |
| `array` | 750 | 145 | **162** |

The 147 newly-correct variables, by the ground-truth type they match:

```
 47  obstack *      11  timespec (by value)   6  timeval (by value)   4  re_pattern_buffer *
 37  char *          7  timespec *            5  FILE *               3  __mbstate_t, void *
                     6  termios *             5  statfs (by value)    2  __sigset_t, group *,
                                                5  int                     timeval *
                                                                      1  utmp *, lconv *
```

against 6 newly-wrong, all of them listed:

| slice | function | GT | off | on |
|---|---|---|---|---|
| `coreutils::O0::pinky` | `print_long_entry` | `passwd *` | `passwd *` | `undefined8` |
| `coreutils::O0::pinky` | `print_long_entry` | `char *` | `char *` | `undefined8` |
| `coreutils::O0::pinky` | `print_long_entry` | `char *` | `char *` | `undefined8` |
| `grep::O0::grep` | `EGexecute` | `idx_t` | `long` | `char *` |
| `grep::O0::grep` | `EGexecute` | `idx_t` | `long` | `char *` |
| `gzip::O2::gzip` | `lutimens` | `stat` | `stat` | `char[80]` |

Not one is a named libc struct standing where a correct primitive pointer used
to. The three `pinky` rows are the register-resident case below; the two
`EGexecute` rows are a pointee guess spreading into two index variables; the
`gzip` one is the frame re-split described in full below. A second re-split of
the same kind, and the one place this round makes a store escape its object,
falls outside the scored corpus entirely: the `sigfillset` slot, two sections
down.

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
count moves are the whole of it.

### The one worse function

`coreutils::O0::pinky` `print_long_entry`, 0.4286 -> 0.2143. Three variables
(`pw`, `project`, `plan`) move from a stack slot to a register, and decbench never
exports a register local, so they stop being scored at all — they are not mistyped, they
are invisible. The trigger is `fread_unlocked` gaining a declaration, which changes what
the O0 frame copies merge with. The same slot wins four `FILE *` elsewhere.

### The gzip `lutimens` row, in full

The `stat` row in that table is not just a name lost. `gzip::O2::lutimens`
(`0xee90`) on `main` declares the whole 144-byte frame slot as one `stat v5` and
reads its tail as members; on this branch the slot is `char v5 [80]` handed to
an `lstat` that writes 144 bytes, and the three words past 80 —
`st_atim.tv_nsec`, `st_mtim` — detach into `v9`/`v10`/`v11`, which the emitted
body reads with nothing writing them:

```
main:    stat v5;           // stack - 0xc8      lstat(a0,&v5)      v8 = v5._88_8_;
branch:  char v5 [80];      // stack - 0xc8      lstat(a0,(stat *)v5)
         long v9;           // stack - 0x78      v4._8_8_ = v9;     <- nothing writes v9
         undefined8 v10;    // stack - 0x70      v7 = v10;
         long v11;          // stack - 0x68      v8 = v11;
```

The cause is the `timespec *` slot on `utimensat`/`futimens`: with those two
slots reverted to `void *` and nothing else changed, `stat v5` comes back whole.

It is kept, and this is the measurement it is kept on. Over all 30 corpus slices
that import `utimensat` or `futimens` (coreutils `cp`/`ginstall`/`mv`/`touch`,
gzip, tar, shadow `useradd`/`usermod`, openssh `sshd`/`sftp-server`, three
optimization levels), the two builds differ in 123 functions:

```
122  gain a `timespec` rendering (gnulib's own `struct timespec ts[2]` among them)
  3  lose a whole `stat` declaration
  1  of those 3 is under-declared: gzip O2 `lutimens`
```

The other two (`shadow::O2::useradd` `0xc4a0`, `usermod` `0xbd90`) trade a
`stat v17` for a 152-byte `char v17 [152]` — wider than the struct, not
narrower, so no write lands outside it — while the adjacent
`unsigned long v18 [4]` becomes exactly the `timespec v18 [2]` the source
declares.

The under-declared shape itself is not new. Scanning the same 30 slices for a
`char vN [K < 144]` that the body casts to `stat *`: `main` already prints 7 of
them, and this branch prints 8. `coreutils::O2::cp` `0x76f0` is main's, with the
same detached tail — `char v24 [80]` cast to `(stat *)`, and `v25`, `v89`,
`v90`, `v91` read with nothing writing them. So the row is one more instance of
a rendering the option already ships, bought with 122 functions that gain the
type the source actually declares; it is not a defect this round introduces, and
fixing the class belongs to the frame-merge seam rather than to a prototype
table.

### The `sigfillset` slot and the `sigaction` frame

The same re-split one binary family further out, and this one detaches a write
rather than a read. `sigfillset(sigset_t *)` gives the 128-byte `sa_mask`
sub-object of a `struct sigaction` local an identity of its own, so the 152-byte
frame slot the caller hands to `sigaction()` splits at 136 and the `sa_flags`
store lands in a local outside the object. `-O2` openssh `ssh-keygen`
`sub_4ce20` (`ssh_signal`, `misc.c:0xa24`) is the shape:

```
base:    sigaction v4;    // stack - 0x158   sigfillset(&v4.field_0x8);
                                             v4._136_4_ = 0x10000000;
                                             sigaction(a0,&v4,&v5)
branch:  char v4 [136];   // stack - 0x158   sigfillset((sigset_t *)&v4[8]);
         undefined4 v7;   // stack - 0xd0    v7 = 0x10000000;   <- nothing reads v7
                                             sigaction(a0,(sigaction *)v4,&v5)
```

`v7` is dead: the flag never reaches the object, and `sigaction()` is handed 136
bytes of a 152-byte structure. That is worse than the gzip row above — there a
body read words nothing wrote; here a store escapes the object, which a
recompile would observe.

Measured by ablation, this branch built twice with nothing but the `sigfillset`
row removed from the table in the second build:

```
444-slice corpus       6 slices import sigfillset (coreutils env, shadow su, three -O levels)
                       0 `sigaction` declarations lost; the slot only adds sigset_t
                       renderings (+2/+3 in env, +1 in su)
disjoint openssh -O2   7 binaries, exactly 1 function each, all of them ssh_signal:
                       ssh 11 -> 10 `sigaction vN` declarations, ssh-keygen 11 -> 10,
                       ssh-agent 11 -> 10, ssh-add 11 -> 10, sshd 3 -> 2, scp 2 -> 1,
                       sftp 2 -> 1
```

Nothing in the scored corpus moves; the visible cost is one function per openssh
binary.

The shape itself is not new, and the mitigation is the same as the gzip row's:
`sigemptyset(sigset_t *)` ships on `main` and splits the frame the same way. On
the ablated build, the five-line `sigaction(2)` program with `sigfillset`
replaced by `sigemptyset` prints the identical detached `undefined4 v6;
// stack - 0x30` next to `sigaction(a0,(sigaction *)&v3,a1)`; and ssh-keygen's
base arm already prints 18 `(sigaction *)` cast-at-use sites against this
branch's 19, eight of them in one function immediately after a
`sigemptyset(&v23)`.

The row is kept: typing `sigset_t` where the program passes one is right, the
scored corpus loses nothing, and dropping `sigfillset` would leave
`sigemptyset`/`sigaddset`/`sigprocmask` triggering the same split on `main`
anyway. Fixing the class belongs to the frame-merge seam, not to a prototype
table.

### Whole-corpus output diff

`decompile-all` before and after over 14 whole binaries (7,457 functions): 796 functions
change, 2,450 hunks, classified by `classify.py` beside this file into
`corpus-hunk-classification.txt`:

```
 638  local renumbering only
 587  synthesized struct_N renumbered (one fewer slot in the ledger)
 485  other, downstream of a changed pointee
 173  signature gains a named pointee
 165  a named cast appears
 115  cast at an offset becomes a field
  99  declaration gains a named pointee
  84  a PLT thunk gains the return the declaration states
  45  other, mentions a named type
  33  a field access gains or loses its width cast
  22  a local's spelling or width moves under the new pointee
   4  field becomes a cast at an offset
```

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

### The export's compile-error cost

The option's documented cost is that `decompile-project`'s exported `.c` reads
fields out of types its own `.h` declares incomplete. On the `-O2` `ls` export
that was +58 `cc -fsyntax-only` errors; with this round it is **+102**
(830 off, 932 on), and the `.h` is still 0 errors in both arms. Seven more
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
the two builds alternating on every repetition (`speed.py` / `speed.json` beside this
file). Under campaign load — eight other lanes plus this one's own workspace suite —
which is what the interleaving controls for.

| binary | off | on | delta |
|---|---:|---:|---:|
| `O2` tar | 48.241s | 49.924s | +3.49% |
| `O2` grep | 11.980s | 12.085s | +0.87% |
| `O2` ls | 13.635s | 14.091s | +3.34% |
| `O0` grep | 6.936s | 7.083s | +2.11% |
| `O2` sort | 14.799s | 14.842s | +0.30% |

Re-measured once this lane's own workspace suite had finished, the two largest
deltas come down: tar +2.68% (45.712s -> 46.938s) and ls +1.26% (13.808s ->
13.982s), `speed-confirm.json`.

Worst +3.49%, inside the +5% budget. The cost is where the names land: `tar` and `ls`
are the two binaries with the most newly-typed pointers, and a named pointee is more
type-propagation work than a `void *` one. `grep -O2` and `sort`, whose pointers were
already typed, are noise.

Splitting the obstack channel afterwards added one more symbol-table walk to the load
(the defined-name set), which is below the noise floor of the path it is on: min-of-9
interleaved `kuna functions` over `-O2` tar, 0.144s before against 0.142s after.
