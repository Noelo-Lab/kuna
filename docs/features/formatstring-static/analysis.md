# `formatstring static` — resolving the format constant at load

## The opportunity

A `printf`/`scanf`-family call states the types of its variadic arguments in its
format string. kuna could already read that — `formatstring` has shipped the
ported Ghidra `FormatStringParser` and its per-call-site prototype override since
the analysis tier landed — but only through Ghidra's own mechanism, which is
`DecompilerDependent`: the format pointer is an input varnode of a lifted `CALL`,
so the caller has to be decompiled once to find it, given the override, and
decompiled again.

That second decompile is the whole cost. Measured on `decompile-all`: +77.5%
(cronie `crontab`), +55.1% (gnutls `psktool`), +43.7% (`gzip`). Only ~5–15% of
functions re-decompile — they are simply the expensive ones. So the option was
held to a per-run opt-in and taken out of the `aggressive` preset, even though
the recovery it bought was real (decbench C `type_match` perfects 80 → 88 of
1,133).

The constant, though, is in the image; so is the call site. Nothing about reading
it requires a decompiler.

## What the program-prep tier already knows

Four facts, each already held by an existing sub-model:

1. **Which callees take a format, and in which slot.** The built-in libc
   signature tables (`analyzers/protos`) carry `vararg`, the index of the first
   variadic slot. A signature is a format function when `vararg >= 1` and the
   fixed parameter immediately before it is a `char *` — which is Ghidra's
   `usesVariadicFormatString`, read off the *declaration* instead of off a
   recovered prototype. This is strictly better than the `printf`/`scanf`
   substring test it replaces: it admits `err`/`errx`/`warn`/`warnx`/`error`/
   `syslog`/`__syslog_chk` (all format-taking, none containing either
   substring), and it excludes `open`/`openat`/`fcntl`/`ioctl` structurally.
   `execlp`, whose trailing `char *` is `argv[0]` rather than a format, is the
   one name that has to be excluded by hand.

2. **Where it is called.** `Listing::refs_to(callee)` — the edges of the
   Listing/xref tier into the callee — with `function_containing` naming the
   caller. Two kinds count. A `Call` edge is the ordinary call site. A `Code`
   edge that is not a fall-through is a **tail jump**, `jmp printf@plt`, which
   gcc emits at `-O2` for any function whose last act is the format call; it is
   a format call site too, since the jump hands the callee the caller's own
   argument registers and the lifter turns it into a `CALL` at the jump's own
   address, so the override keys the same way. Tail jumps are 3-15% of the
   direct format sites in the corpus binaries below (`objdump`, PLT-target
   exact: `fmt` 4 of 26, `ls` 7 of 63, `sort` 7 of 84, `grep` 8 of 77, `tar` 12
   of 349). An indirect call through a function pointer produces no edge at all
   and is invisible here.

3. **Which register holds it.** `ProtoModel::assign_parameter_storage` over the
   callee's signature. The answer comes from the compiler spec, so it is right on
   every architecture without a per-ABI table.

4. **What it points at.** A bounded backward walk of the call site's own basic
   block, re-lifted to p-code and constant-folded forward.

## The `gettext` hop

GNU programs do not pass the literal. They pass `_(...)`, which is
`dcgettext(NULL, "…%s…", 5)`. Without a rule for that, the resolver finds almost
nothing in coreutils, grep, tar or findutils — an intervening call clears every
register the fold tracks, which is the correct conservative answer for a call in
general.

The rule: a format argument produced by a `gettext`/`dgettext`/`dcgettext` call
resolves through that call's msgid parameter. It is sound because the translation
a `gettext` returns has the same conversions as its msgid — that is exactly what
`xgettext`'s `c-format` check enforces, and a translation that violated it would
be a crash in the original program too.

## Why the parameters are anonymous

`build_override_pieces` named the override's parameters positionally
(`param0`…`paramN`), following Ghidra's `ParameterDefinitionImpl("param" + i, …)`.
A call-site override's parameter names reach the **caller's** locals, and a
positional name is the same at every site, so coreutils `fmt` `sub_4100`
rendered:

```c
param3 = dcgettext(NULL,"write error",5);
if (dat_17140) {
  param3 = (char *)sub_6290(dat_17140);
  error(0,*v2,"%s: %s",param3,param3);   /* two different values, one name */
}
```

Two variables took the name `param3` — one from the two-vararg site, one from the
one-vararg site in the branch beside it — and only one of them was declared. The
override now leaves `innames` empty, exactly as the built-in prototype tables do
(`protos::build_pieces`), and the caller keeps its own naming.

## Where the window is wrong, and who overrules it

The window is only as sound as the Listing's edges, and review found two shapes
where that is not enough. Both are now caught after the first drive, by the
drive itself.

**A join reached from a jump table.** The Listing's walk does not read jump
tables, so the case bodies of a `switch` are decoded by nobody. In

```c
if (__builtin_expect(k > 7, 1)) fmt = "default %ld\n";
else switch (k) { case 0: g(0); fmt = "zero %s %ld\n"; break; /* ... */ }
printf(fmt, a, b);
```

gcc -O2 loads `"default %ld\n"` before the range check, and the case bodies
reload the format and `jmp` back to the instruction after the check. To the
Listing that join is straight-line code, so the window reads the default path's
string, the closed prototype keeps one vararg, and both the `%s` argument and
the caller's parameter it came from disappear
(`void sub_11b0(unsigned int a0,long a1)` with a caller still passing three).
Stopping the walk at every case target would not help: the join is not a case
target, it is where the case bodies *go*, and seeing that needs the case bodies
decoded.

**An `alloca` frame.** When the drive cannot track the stack pointer, the slot
the call pushes its return address into is readable as a stack argument, and a
closed prototype picks it up as one more argument; the open varargs prototype
does not. coreutils `df` rendered `error(0,v4,v8,v6,v20[-8])` for the
one-conversion `"cannot change to directory %s"`, and `cp`, `mv`, `ginstall`,
`stat` and `ip` did the same at 51 sites. The same frame already costs every
closed-prototype call in those functions a phantom (`free(v6,v16[v2 + -8])` is
rendered with `formatstring off` too); the override only made the format calls
closed.

**The audit.** After the first drive, `decompile_step.rs
(audit_parked_format_sites)` looks at each parked site's `CALL` and keeps the
override only when the call passes exactly the arguments it declares and every
value that can reach the format argument, through copies and phi-nodes, is the
resolved string: directly, as a load from read-only memory, or as the msgid of a
`gettext`-family call. Anything else withdraws the override, and the function is
driven once more without it, which renders that call exactly as `off` does. The
parked site carries the format's slot and address for this.

Measured `off` against `static` over every coreutils binary at `O0`, `O2` and
`O2-noinline` and 31 others (325 binaries, 74,707 functions; re-run on the tree
rebased onto `d03560b2`, both arms from one build), checking every format call
whose format the rendered C names for its vararg count against the format's
conversion count and each vararg's class (floating or not) against its
conversion's, and every function's arity (`argcount.py`, output in
`argcount.txt`):

- **0** calls newly render a different number of varargs than their format
  consumes; **313** that did under `off` now agree.
- **0** varargs newly render in the wrong class; **4** that did under `off` now
  agree (`uptime` `O2`/`O2-noinline`, `%.2f` load averages that were
  `unsigned long` locals and are now `double`). Of the 18,057 varargs checked
  under `static`, 90.5% have a known class (81 floating conversions). The 2 class mismatches left
  under `static` are also under `off`: `uptime`'s third `%.2f` keeps its format
  in callee-saved `rbp` across the previous call, which the fold conservatively
  clears, so the site is declined.
- **8** functions change arity, all toward what their callers pass: `ls`/`dir`/
  `vdir` `0xc590` 3 → 2 and `ip`/`rtmon` `rtnl_rtscope_n2a` and `rtnl_rtrealm_n2a`
  5 → 3 lose a phantom parameter, and `kmod` `0xd650` 5 → 6 gains the one its
  `"%c! /dev/%s 0600 - - - %u:%u\n"` call reads.
- The audit withdrew **130** of the 11,743 sites resolved at load, all on the
  argument count, in `alloca` frames: `cp` 41, `ginstall` 34, `mv` 34, `ip` 7,
  `df` 4, `stat` 4, and one each in `ls`/`dir`/`vdir` at `O0` and `O2-noinline`.
  Those calls now render exactly as under `off`.

A first run of the `O2` part of that sweep also withdrew six sites on the format, and all six
were the same fold bug: x86 `cmovcc` lifts to `if (!cc) goto inst_next; dst =
src;`, and the fold set the destination unconditionally, so a format picked by a
conditional move resolved to one of its two strings (`nohup`, `od`, `mkdir`,
`ginstall`, and `last` twice). The fold now kills whatever an instruction writes
after a branch inside it, and the second run withdrew none on the format.

The jump-table shape does occur in the corpus, in `tar` O2 (not in the sweep
above): the window before the `__fprintf_chk` at `0x23875` reads the msgid at
`0x65108`, but a body at `0x23a38` that the Listing never decoded loads its own
msgid (`0x650fb`) and jumps to `0x2385a`, inside the window. The drive renders
`dcgettext(NULL,v6,5)` there and the audit withdraws the site. The stage test
pins the shape with `fmtjoin_x86_64`.

## Where the closed prototype is a different call

The override closes the prototype, and the compiler spec then places each
conversion where it would place a **named** argument of that type. That is the
same call only where the target passes a variadic argument exactly like a named
one, and the evidence behind the default was x86-64 alone. Review found three
targets where it is not:

- **Apple AArch64** passes every vararg on the stack. The in-tree
  `macho_imports_arm64` fixture's `_main` is `printf("%d\n", compute(argc))`; the
  closed prototype read the `%d` from `w1`, so `_main` grew a phantom second
  parameter and the call printed it instead of `a0 * 3 + 7`.
- **ARM32 hard-float** passes a floating vararg in `r2:r3` and a named `double`
  in `d0`. `f_conv(int x, double a) { printf("x=%f\n", (double)x); }` lost its
  `int` parameter and printed an uninitialized `d0`; `f_sum(a, b)` printed
  `(a, b)` for `(a + b, a)`.
- **RISC-V lp64d** passes a floating vararg in integer registers; the same
  `f_conv` printed its unrelated `double` parameter.

The post-drive audit cannot see any of these, because the call's argument count
matches. So the rule is stated per target instead (`kuna_formatstring.rs
(vararg_abi)`), checked for every conversion in `build_override_pieces`, and
therefore binding on `full` as well as `static`:

| target | conversions typed |
|---|---|
| x86 (32/64-bit, SysV and Win64), AArch64 AAPCS64 (ELF) | all |
| ARM32, RISC-V, MIPS, PowerPC, AArch64 in a PE | integer and pointer, no wider than a pointer |
| Apple AArch64, every other processor | none |

Win64 is in the first row because the caller also copies a floating vararg into
the `XMM` register the named-argument rule assigns, so the override reads the
right value. RISC-V also aligns a double-width vararg to an even register pair
where a named one is not, which is why the middle row stops at pointer width
(`%lld` on ARM32 is passed the same both ways, but is declined with the rest).
Whether an AArch64 image is Apple's, Windows' or the standard one comes from the
container, so the console records it at `load file` beside the other one-bit
image facts; the XML path has no container and types no AArch64 site.

`fmtabi_armhf` (the stage test's passes 10 to 12) pins ARM: `f_conv` and
`f_sum` render as under `off`, and the integer site `f_is(int, char *)` is still
typed. Passes 7 to 9 pin `macho_imports_arm64`.

## A format the program can rewrite

`read_cstring` read whatever bytes were at the resolved address, while the fold's
`LOAD` already insisted on a read-only section. A format in `.data` is whatever
the program last stored there:

```c
char fmtbuf[16] = "v=%d\n";
void setmode(int s) { if (s) strcpy(fmtbuf, "v=%s\n"); }
void show(const char *p) { printf(fmtbuf, p); }
```

typed `show`'s `char *` parameter `int` from the image's `"v=%d"`, and the
caller then rendered `show(*argv)`. The resolved string, NUL included, must now
lie in one section the image initializes and does not write, the same predicate
the fold uses; the `full` loop checks the loader's read-only ranges.

## Wide characters

Ghidra's parser returns before the length modifier for `c` and `s`, so it types
`%lc` as a `char` and `%ls` as a `char *` (and `%C`, a wide character, as a
pointer). On by default, that put a truncating `(char)a0` on a `wint_t`
argument. `%lc`/`%C` are now an `int`-sized unsigned value (what a `wint_t` is
passed as), and `%ls`/`%S` a `wchar_t *`; `scanf` wraps both in a pointer as
before.

## `l` on a floating conversion

Ghidra's `longLengthModification` answers `unsigned long` for every conversion
it does not list, so `%lf`, `%le`, `%lg` and `%la` became integers, and the
`scanf` forms `long *`. C11 7.21.6.1p7 says `l` has no effect on a floating
`printf` conversion; in `scanf` it selects a `double` destination over a
`float` one. On by default the old answer was not a mistyped argument but a
moved one. `printf("value=%lf n=%d\n", d, n)` at x86-64 `-O2` rendered
`show(unsigned long a0,unsigned long a1,unsigned long a2,int a3)` and printed
`a0 & 0xffffffff`: the `double` in `xmm0` was replaced by an integer register
read and the int shifted two slots, adding two phantom parameters. On ARM
hard-float `unsigned long` fits in a pointer, so the ABI rule did not catch
it either, and a `sscanf("%lf", &d)` local became `int v1[3]`. In the corpus,
e2fsck `O2` `0x736b0` printed `%16.4lf` with the bit counter; it now prints
`v9 / (float8)(uint8)(v2 - v1)`, which is the source's
`eff = (double)((count * sizeof(struct bmap_rb_extent)) << 3) / (real_end - start)`
(`40 << 3 == 0x140`).

`%l` plus a floating conversion is now `double` for `printf` and a new
`double *` for `scanf` (a plain `%f` there stays `void *`, as before). `%lp` is
`%p`. The ARM rule then declines the `printf` sites, which render as under
`off`, and still types the `scanf` pointer: the local is a `float8`, and off's
phantom trailing argument goes. Passes 15-20 of the stage test pin both targets
(`fmtlf_x86_64`, `fmtlf_armhf`); the round-3 build fails 8 of the 9 new
assertions.

`argcount.py` compared only counts, which is why its 325-binary check could not
see this. It now also classes each vararg (floating or not, from the declared
type, a cast, a literal, or an arithmetic expression whose known operands
agree; an unknown class never counts) against its conversion, and lists every
`%l`-float site with the class its argument received. Over the 325 binaries
there is one such site, e2fsck `0x736b0` above: under `off` its call passes no
argument at all, and under `static` it passes a floating one.

## Delay slots

The window walks backward from the call and never lifted the call instruction
itself. A MIPS or SPARC call runs its delay slot before it transfers, so a
format register set there (`jal printf` with `addiu a0,a0,%lo(fmt)` in the
slot) would fold to the `%hi` half alone. The call instruction is now lifted
too, and a site (or a `gettext` hop) whose call instruction writes the format
register before it transfers is declined. MIPS is inert today regardless: a
non-PIC `jal printf@plt` stub is not resolved to an import name, and PIC code
calls through `$t9`, which leaves no Listing edge.

## A `%s` of a struct's first field

A `%s` whose argument is the address of a structure's leading `char[]` field is
the structure's own address, so the `char *` the conversion asserts reaches the
caller's structure pointer. Wider stores through that pointer then render as runs
of byte stores: `ssh-keygen` O2 `0x52d30` renders `*(unsigned int *)(a1 + 0xd8)
= 1;` as four byte stores, and `useradd` O0 `0x1ca0d` splits an 8-byte store
into eight (+32 lines in that binary), and `chfn` O2-noinline `0x8e70` zeroes
a field with 40 `a0[0x40N] = '\0'` stores where `off` has none (`gpasswd` and
`rsyslogd` are unchanged by it). The value stored is the same on a
little-endian target, so this is a readability cost, not wrong output. It is the
same thing any `char *` libc prototype does to a structure pointer passed as its
first field (`strlen(&s->name)`), which makes it a type-propagation question
rather than a format-string one, and it is left as it is.

## What `static` declines, and what `full` is for

The resolver answers only what it can prove: no Listing; no edge into the callee
(an indirect call through a function pointer); a format written outside the
call's own block, or not constant there; an intervening call that is not a
`gettext`; an address with no readable NUL-terminated string; a string outside a
read-only section; a conversion the target does not pass as a named argument; a
format with no conversions; a format carrying `%Lf`, whose x86-64 argument class
the override pieces cannot spell. Each is a decline, and a decline costs only
the typing.

One decline is not the resolver's at all: a site the Listing's walk never
decoded is a site with no edge, so the pass never sees it. That ceiling is
measured in *Coverage* below.

`full` (the old `on`) keeps the loop for the sites declined for want of an edge
or of a constant in the window, at the old price; the read-only, ABI and `%Lf`
rules bind it too.
On coreutils `ls` it recovers five call sites `static` declines, including
`__sprintf_chk(…,"%%%02x",(unsigned int)v1)` where `static` leaves
`(unsigned long)v1` — plus the unrelated body changes its read-only propagation
brings.

## Coverage

Two denominators, because they answer different questions.

**In the image** (`objdump`, every direct `call`/`jmp` whose target is the PLT
stub of a format callee the resolver accepts) is the honest total. **Seen** is
what the Listing/xref tier actually hands the pass — fewer, for a reason below.
`KUNA_FMTSTATIC_DEBUG=1 kuna decompile-all <binary> --option formatstring full`
traces each site the resolver considered and what it decided; the `loop-only`
column is what the second decompile then found on top.

| binary (decbench O2) | in the image (call + jmp) | seen | of which tail jumps | resolved at load | unresolved | withdrawn by the audit | loop-only |
|---|---|---|---|---|---|---|---|
| coreutils `fmt`  |  26 (22+4) |  21 | 1 |  18 |  3 | 0 |  0 |
| coreutils `ls`   |  63 (56+7) |  57 | 4 |  48 |  9 | 0 |  5 |
| coreutils `sort` |  84 (77+7) |  72 | 4 |  62 | 10 | 0 |  1 |
| `grep`           |  77 (69+8) |  60 | 5 |  55 |  5 | 0 |  1 |
| `gzip`           |    6 (6+0) |   0 | 0 |   0 |  0 | 0 |  0 |
| `tar`            | 349 (337+12) | 310 | 9 | 237 | 73 | 3 | 10 |
| **total**        | **605** | **520** | **23** | **420** | **100** | **3** | **17** |

420 of 520 seen (81%), 420 of 605 in the image (69%).

The 100 declines the pass does see are dominated by a format that is not
constant in the call's own block — a `%s` string built at run time, a format
selected by a branch, a call between the load and the use that is not a
`gettext`. The loop recovers 17 of them with its second decompile, which is what
`full` is for.

The 85-site gap between the image and what the pass sees is **not** this pass: it
is the Listing's own walk coverage. On x86-64 the recursive descent is seeded
from `listing_seeds` only — the `committed_entries` inventory is added as walk
roots under the `funcstart_patterns` gate, which is off for x86-64 — so a
function that only a load-time entry oracle found contributes no instructions and
no edges. `gzip` is the pure case: `kuna xrefs --to 0x3520` reports all six
`__snprintf_chk` call sites, and `Listing::refs_to(0x3520)` reports none, so
`gzip`'s whole-binary output is byte-identical on all three values. Widening the
walk's seed set would lift this ceiling for every Listing consumer at once
(`noreturn_disc`, `fid`, this pass) and is its own change.

## The cost of the pass itself

Isolated by decompiling a single function out of `bash` (so the whole load,
including the Listing build and this pass, is measured against a decompile that
is not): paired median over 9 interleaved rounds, **+31 ms** on a ~1.7 s load —
0.04% of `bash`'s ~85 s whole-binary run.
