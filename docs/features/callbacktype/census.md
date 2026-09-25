# `callbacktype` census — a function whose address is handed to a libc callback slot

Measured on the campaign's 444 slices (projects coreutils grep gzip diffutils bzip2
findutils tar shadow x, `--opt O0 --opt O2 --opt O2-noinline`, stripped images), against
the unstripped DWARF twins in the same tree. Scripts: `.scratch/cbscan.py` (the objdump
walk) and `.scratch/cbstep2.py` (kuna + DWARF), kept in the PR as `census-scan.py` /
`census-types.py` beside this file.

## 1. How many

A stripped binary is disassembled, and for every call (or tail jump) to an import that
declares a function-pointer parameter, the ABI register of that slot is read back through
a bounded window of `lea`/`mov` in the same block.

| | hits | distinct targets | targets decbench scores |
|---|---|---|---|
| all | 294 | 114 | **50** |
| `signal` | 247 | — | 36 |
| `qsort` | 37 | — | 27 |
| `bsearch` | 4 | — | 4 |
| `pthread_create` | 3 | — | 3 |
| `scandir` | 2 | — | 0 |
| `glob` | 1 | — | 0 |

76 of the 444 slices have at least one. By level, the scored targets are O0 19, O2 14,
O2-noinline 17; by project coreutils 20, shadow 14, bzip2 6, tar 5, diffutils 3,
findutils 2. `atexit`/`__cxa_atexit` produce no scored target: every handler they register
in this corpus is a gnulib `close_stdout`, which decbench does not narrow to.

## 2. Is the declared signature right?

**50 of 50.** Every scored target's DWARF declaration is exactly the slot's declared
signature — `int (void *, void *)` for the comparators, `void (int)` for the signal
handlers (`bzip2` spells it `IntNative`, a `typedef` for `int`), `void *(void *)` for
`sortlines_thread`. Not one target is reached through two slots that disagree, and not one
has a DWARF declaration the slot contradicts.

## 3. What does kuna recover today?

Parameters of the 50, from `kuna decompile-all --json` on 796939d54:

| rows | declared | kuna today | |
|---|---|---|---|
| 16 | `(int)` | `()` | the handler ignores its argument, so nothing is recovered |
| 15 | `(void *, void *)` | `(unsigned long *, unsigned long *)` | right shape, wrong pointee |
| 6 | `(int)` | `(unsigned int)` | signedness only — worth zero on `type_match` |
| 5 | `(void *, void *)` | `(struct_N *, struct_N *)` | `structsynth` minted a record for the pointee |
| 4 | `(void *, void *)` | `(int *, int *)` | |
| 3 | `(void *)` | `(unsigned long *)` | |
| 1 | `(int)` | `(int)` | already right |

**67 GT parameters** would move, of which 16 are a parameter kuna does not recover at all
(the signal handlers that never read `edi`) and the remaining 51 are a pointee. Seven rows
are signedness-only and move no metric.

## 4. What the measurement says about the design

* The arity half is not optional: 16 of the 50 need the parameter to *appear*, so the
  statement has to reach `ActionDefaultParams` as a declared prototype, not only
  `Varnode::getLocalType`'s vote. DWARF confirms every one of those 16, so the parameter
  is recovered, not fabricated.
* `calleevote` cannot reach any of them. `CallGraph::direct_call_sites` returns `None` for
  an entry with a `Data` cross-reference, and taking a function's address *is* one, so a
  callback is never a closed callee.
* Five rows overwrite a synthesized `struct_N *` with the declared `void *`. The program
  declares `void *` and so does the ground truth, so the declared type wins; it is the one
  place where this option and `structsynth` disagree, and it is called out in the option's
  documentation.

## 5. What fired

Over the 37 binaries that hold a scored target, the option parks **46 of the 50** and refuses
four as escapes: `findutils/find -O0` `cost_table_comparison` (three references taking the
address, two recorded arguments) and `shadow/su` `catch_signals` at all three levels (two
references, one recorded argument). Both are the intended reading — the run did not see
every use of the address, so it cannot say the callback slot is what declares the function.
