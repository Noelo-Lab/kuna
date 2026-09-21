# Why the `char *` gap to binja stays large, and what closes it

Measured on main `ce008ce2b`, which includes #695 `charptr` (default off), #704
and #692. The set is the campaign's 444 slices (coreutils grep gzip diffutils
bzip2 findutils tar shadow x, at -O0, -O2 and -O2-noinline), with the metric
pinned to decbench `625e892`. Kuna's decisions come from a typesweep
`--dump-decisions` run. The rivals' decisions come from the round-A replay of
their stored outputs. `gapcensus.py` in this folder does the join.

## The short answer

Kuna gets 4,233 of its 14,587 ground-truth `char *` variables right (29.0%).
Binja gets 5,323 of 14,469 (36.8%). On the 14,425 variables both tools are
scored on, binja alone is right on 1,625 and kuna alone on 496, a net gap of
1,129. The byte-evidence walk in `charptr` can reach very little of that gap: on
this base it adds 2 true positives. Most of the gap comes from two places where
kuna already has the type:

1. **805 of the 1,625 are stack slots that kuna exports as a width-only
   `undefined8`.** At -O0, each C local has its own spill slot. Kuna
   copy-propagates the value out of the slot, and `framelayout` then reports
   the empty slot without a type. The value itself is printed as `char *` in
   kuna's own C body. Binja keeps each slot as a variable and types it from
   the value stored into it. The evidence is the same; kuna loses it at export.
2. **Most of the 411 arguments that kuna spells as an integer are only passed
   on to another function in the program.** Binja carries a callee's inferred
   parameter type up through every caller. Kuna does the same with
   `protoorder`, but `protoorder` states nothing for a recursive function. The
   char-level work in coreutils and gnulib bottoms out in recursive functions
   (`quotearg_buffer_restyled` calls itself, and `copy_internal` and
   `copy_dir` call each other), so the chain stops at the first recursion.
   Letting recursive functions state their recovered types is measured below:
   +36 to +46 perfect functions and +163 to +188 `char *` true positives, with
   one function worse.

## Where the missed variables are

Kuna's own 14,587 ground-truth `char *` variables:

| storage | GT | kuna right | missed | GT spelled `void` (decbench#94 restrict artifact) |
|---|---|---|---|---|
| argument | 4,131 | 2,726 | 1,405 | 10 |
| stack local | 3,316 | 1,507 | 1,809 | 35 |
| register only | 7,140 | 0 | 7,140 | 2,904 |
| **total** | 14,587 | 4,233 | 10,354 | 2,949 |

For the 7,405 misses that anyone could match (GT not spelled `void`), here is
what kuna spells instead:

| kuna spelling | argument | stack | register | total |
|---|---|---|---|---|
| no kuna variable there | 68 | 163 | 4,236 | 4,467 |
| an integer (`unsigned long`, `long`, `int8`, ...) | 1,014 | 376 | 0 | 1,390 |
| a width-only placeholder (`undefinedN`) | 25 | 1,043 | 0 | 1,068 |
| a pointer to something else | 286 | 186 | 0 | 472 |
| (`void *`) | 242 | 157 | | 399 |
| (`int *`, a named libc record `*`, `struct_N *`, `T **`) | 44 | 29 | | 73 |
| other (array, float) | 2 | 6 | 0 | 8 |

Register-only ground truth is not a gap to binja. Kuna exports no register
locals, and binja is right on 51 of the 7,091 it is scored on. 2,883 of those
7,091 are the `void`-spelled restrict artifact, which no tool can match.

## The gap to binja, by bucket

On the 14,425 shared variables, the net gap of 1,129 splits as stack 859
(1,037 binja-only against 178 kuna-only), arguments 219 (537 against 318) and
register-only 51. By optimisation level it is -O0 809, -O2-noinline 184 and
-O2 136. The gap is mostly an -O0 problem.

For the 1,625 variables only binja gets right, kuna's spelling is:

| kuna spelling | stack | argument | register | total |
|---|---|---|---|---|
| width-only filler slot (`local_NN undefined8`) | 805 | 0 | 0 | 805 |
| an integer | 131 | 411 | 0 | 542 |
| a pointer to something else (`void *` 115, named libc record 21, `int *` 19, `struct_N *` 7, `T **` 3) | 78 | 87 | 0 | 165 |
| no kuna variable there | 22 | 38 | 51 | 111 |
| other | 1 | 1 | 0 | 2 |

### Filler slots (805): kuna has the evidence, and export drops it

In 20 filler cases drawn at random from the 805, 19 were traced back to the
stored value, and kuna's own C body types it `char *` (or `unsigned char *`,
which the metric treats the same) in every one. The 20th, tar -O2-noinline
`xheader_string_end`'s `np`, has a matching `char *v8` in rbp that was not
traced. The value lives in one of two places:

* **A register variable.** Eleven of the twenty are shadow's `SYSLOG` macro. At
  -O0, every expansion's `char *old_locale = setlocale(LC_ALL, NULL)` gets its
  own slot. Kuna merges all of them into one `char *` register variable
  (`v3 = setlocale(6,NULL);`), and reports each slot as `undefined8`. The others follow
  the same pattern: `comma = strchr(...)` (pinky `print_entry`),
  `resolved = canonicalize_file_name(...)` (stat `out_mount_point`),
  `dstbase = last_component(...)` (ginstall `source_is_dst_backup`), and the
  `salt` that chpasswd `main` prints in `failed to crypt password with salt '%s'`.
* **Another source local's variable.** In sort -O0 `write_line`, `buf` and `c`
  share `char *v4` at `c`'s slot, and `buf`'s own slot becomes `local_20
  undefined8`.

The same mechanism costs other classes too. Across all classes, 3,018 stack
ground-truth variables land on a filler slot and miss (ptr_struct 1,205,
ptr_char 1,056, ptr_void 134, and others), and binja gets 1,097 of those right.
Another 3,004 land on a filler slot and are *credited* on width alone, because
they are scalars. Typing a filler slot as a scalar would put that credit at
risk. Typing it only with a pointer cannot, because a width-only spelling never
matches a pointer.

### Arguments spelled as an integer (411): the chain stops at recursion

A random sample of 24 from the 411 (two could not be resolved, gzip's
`__printf__` has no symbol by that name):

| what the argument does in the function | samples |
|---|---|
| only passed on to another function of the program (cp `copy` to `copy_internal`, stty/chown/tac/head to the `quotearg` family, od to its format printer, find to `print_quoted`) | 16 |
| numbered wrong: 3 variadics whose recovered list starts with eight `unsigned long` slots from the register-save prologue, so the ground truth's `fmt` is compared with one of those, and 1 with a float first (find `get_relative_timestamp`, where kuna has `char *a5` but the ground truth is argument 4) | 4 |
| passed to a libc import that kuna's signature table lacks (ginstall `writable_destination` to `faccessat`) | 1 |
| read one byte at a time through a computed index (dd `operand_matches`) | 1 |

Here is the forwarding case on one chain. In cp -O0, kuna recovers
`quotearg_buffer_restyled` correctly as `sub_17254(..., char *a2, ...)`. Its
caller, `quotearg_n_options`, still prints `sub_185de(int a0, unsigned long a1,
...)`, even though it hands `a1` straight to that `char *a2`. `KUNA_PROTOORDER_TRACE`
shows why: `sub_17254` never states anything. `callee_first_plan`
(`kuna-cli/src/decompile_all.rs`) gives every member of a recursion cycle, and
every function that calls itself, `park = false`, and
`quotearg_buffer_restyled` calls itself. In cp -O0, every function that
passes a file name to `quotearg_style` therefore keeps it an integer, and so
does every caller above it.

**The measurement.** `recursion-park-experiment.diff` adds an env-gated switch
to `callee_first_plan`. `self` lets a self-recursive function state its types.
`all` lets every member of a cycle state them. Everything else is the same
`types`-mode path, which never locks a prototype, so no call's arity can change.
Each arm was swept against this base's default on all 444 slices:

| arm | perfect | aggregate | improved / worse | onto / off perfect | ptr_char TP |
|---|---|---|---|---|---|
| base (`ce008ce2b`) | 1,353 | 3670.31 | | | 4,233 (29.0%) |
| `self` | 1,389 (+36) | 3708.49 (+38.18) | 136 / 1 | 36 / 0 | 4,396 (30.1%) |
| `all` | 1,399 (+46) | 3714.03 (+43.72) | 160 / 1 | 46 / 0 | 4,421 (30.3%) |

`ptr_char` is the only class that moves (189 gains and 1 loss under `all`). The
binja-only argument bucket shrinks from 537 to 368. The gains are at -O0
(coreutils 95, findutils 29, tar 29) plus 7 functions at -O2-noinline. Why the
-O2 chains stay broken was not traced. The single worse row is coreutils -O2
ginstall `install_file_in_file`, where `to_relname` goes from `char *` to
`stat *`: a recovered guess from a callee, which is the known cost of any
recovered vote. In cp -O0, `emit_verbose` goes from
`void sub_a6db(unsigned long a0,unsigned long a1,long a2)` to
`void sub_a6db(char *a0,char *a1,char *a2)`.

### What else binja has

* **A complete libc type library.** Across the 444 slices, 238 distinct libc
  imports take a character-pointer parameter, and 133 of them are missing
  from kuna's built-in table. By number of slices that import them:
  `strtoumax` 96, `chroot` 81, `utime` 78, `strtoll` 78, `__open_2` 64,
  `__fgets_chk` 50, `faccessat` 48, `getfilecon`/`lgetfilecon`/`freecon` 42,
  `strtoimax` 39, `execvp` 33, `error_at_line` 18. Some are kept out on
  purpose, because the table only admits slots that have the same width on
  every target (`strtoll` returns `long long`). How much of the gap this is
  worth was not measured; it was 1 of the 22 samples.
* **Correct numbering for variadic arguments.** 52 of the 537 binja-only
  arguments belong to variadic functions (DWARF `DW_TAG_unspecified_parameters`).
  In the sampled cases the variable sits at a different index in kuna's list.
  In cp -O0 `copy_attr_error` it is even typed right: `fmt` is kuna's
  `char *a9`, behind eight slots from the register-save prologue.

## Ranked next steps

1. **Let `protoorder` state recovered types for recursive functions.** Measured
   at +36 perfect (`self`) to +46 (`all`), +163 to +188 `char *` true
   positives, and 1 function worse. That is about ten times the best `charptr`
   ever measured (+4 perfect, before #704). It is a few lines in
   `callee_first_plan` behind an option value. Before it ships, it still needs
   the wrong-output corpus diff, a speed measurement, and a two-pass stage
   test. For `all`, one design question also needs an answer: a cycle member
   decompiled first states types it recovered without any of its partners'.
2. **Type a `framelayout` filler slot from the value stored into it, with
   pointer types only.** The ceiling is 805 `char *` variables that binja gets
   right (1,056 filler misses in the class, 3,018 across all classes). This is
   a chapter-06 change. An earlier attempt read the slot's *symbol* from a
   later pass and changed nothing, because the symbol is gone by then. The
   stack-space Varnodes themselves still exist during type inference (the
   `charptr` census prints them, `stack@-152 cur=int8`), so the type has to be
   captured there and handed to `framelayout`. Together with step 1, this would
   bring the class from 4,421 to at most about 5,230 true positives, against
   binja's 5,323.
3. **Number variadic and float-first parameters the way the ABI does.** At
   least 52 binja-only arguments. The index is wrong, and sometimes the type is
   already right.
4. **Widen the libc signature table.** 133 missing imports that take a
   character pointer. Worth unknown.
5. **Fix `pointee_refuses`' character exemption for non-string constants**
   before `charptr` is proposed for the default again (see
   `default-on-evaluation.md`). This is about correctness, not the metric.

`charptr` itself stays off. On this base it moves the class by 2 true
positives and the aggregate by -0.57.
