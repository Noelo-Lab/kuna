# O2 census: why kuna is 3.09% perfect at -O2

Build: main 97315551d (release, this worktree). Metric: decbench 625e892 via `final-c/pindb.py`
(`DECBENCH_PIN=/home/mahaloz/kwt/_final-d/db625`). Slices: the 296 O2 + O2-noinline slices of the
444 set (coreutils grep gzip diffutils bzip2 findutils tar shadow). Instrument: `gtclass` extended
with GT provenance (own param / own local / lexical block / inlined-callee param|local), DWARF
location kind, restrict flag, GT index, and kuna's exported `variables[]` per function
(`o2gt.py` in this folder, driver `o2sweep.py`, analysis `census.py`, `ceiling.py`, `levers.py`,
`rivals.py`). Control: 6,462 functions, per-function values identical to round D's rows (0 diffs);
O2 74/2,394 perfect, O2-noinline 380/4,068.

Tiers used below. U = structurally unreachable under the metric: register-only GT (not an arg, no
DW_OP_fbreg: matched only by exact name, and kuna exports no register locals) or restrict-void GT.
S = program-defined struct pointer (arg/stack): no stripped decompiler can spell `Hash_table *`;
reachable only under decbench#93 crediting. P = reachable under the pinned metric.

## 1. Missed GT variables in non-perfect functions

| opt | fns not perfect | non-TP GT | reg-only miss | arg wrong / miss | stack wrong / miss |
|---|---:|---:|---:|---:|---:|
| O2 | 2,320 | 17,724 | 13,143 (74%) | 2,411 / 336 | 1,345 / 489 |
| O2-noinline | 3,688 | 17,960 | 12,168 (68%) | 3,633 / 422 | 1,405 / 332 |

Class x storage at O2 (arg / stack / reg-only): ptr_struct 1,172/177/3,314; ptr_char 322/309/3,558;
int_s4 67/119/1,902; int_u8 45/94/1,645; ptr_void 592/36/387; bool 72/167/533; ptr_ptr 236/19/194;
struct_val 15/314/77; array_char 0/248/104; funcptr 104/2/117.

Top kuna spellings of wrong arg/stack vars. O2: ptr_struct as `struct_N *` 420, as `void *` 397;
ptr_void as `struct_N *` 220, as `long` 165; ptr_char as `unsigned long` 151; stack array as `char *`
95; ptr_struct as `unsigned long *` 80; funcptr as `void *` 76. O2-noinline: ptr_struct `void *` 803,
`struct_N *` 531; ptr_char `unsigned long` 266; ptr_void `unsigned long *` 183, `long` 166.
Arg misses: 160 of O2's 336 are in 71 functions whose `variables[]` is empty (regex fallback).

## 2. One-variable-short functions: the lever list

One-short: O2 186 (108 with a reachable blocker, 78 blocked by a single register-only var);
O2-noinline 666 (392 / 274). "solo" = functions where every blocker is in this bucket, so fixing
the bucket alone makes them perfect; (one-short in parentheses).

| bucket | O2 solo (1-short) | O2-noinline solo (1-short) |
|---|---:|---:|
| S struct ptr -> `void *` | 20 (19) | 143 (141) |
| P `void *` -> `long` / `unsigned long *` | 24 (2) | 104 (4) |
| S struct ptr -> `struct_N *` (credited by #93) | 29 (29) | 66 (65) |
| P arity short (no kuna arg) | 24 (17) | 51 (40) |
| P `char *` -> `unsigned long` / `void *` / `long` | 17 (13) | 36 (28) |
| S struct ptr -> integer / primitive pointer | 11 (11) | 41 (38) |
| P libc struct ptr (`FILE *`, `stat *`) -> other | 9 (6) | 20 (16) |
| P 32-bit int / uid_t / gid_t / pid_t -> `unsigned long` | 1 (1) | 21 (21) |
| P `T **` -> `long` / `unsigned long *` | 3 (3) | 10 (9) |
| P `_Bool` -> `unsigned char` / `unsigned int` | 1 (1) | 7 (7) |

## 3. Reachable vs structurally unreachable

GT variables:

| opt | GT vars | U | of which | S | P | kuna TP in P |
|---|---:|---:|---|---:|---:|---:|
| O2 | 21,038 | 13,172 (62.6%) | own register locals 5,285; inlined-callee params 4,656 + locals 2,492; constant/value-only 403; static locals 309; restrict arg/stack 27 | 1,199 (5.7%) | 6,667 (31.7%) | 3,312 (49.7%) |
| O2-noinline | 23,060 | 12,209 (52.9%) | own 7,988; inlined 3,075; const 562; static 545; restrict 39 | 1,838 (8.0%) | 9,013 (39.1%) | 5,097 (56.6%) |

Functions (a function is in the worst tier any of its GT vars is in):

| opt | fns | U | S | P | kuna perfect | perfect / P |
|---|---:|---:|---:|---:|---:|---:|
| O0 (round-D rows) | 4,286 | 360 (8.4%) | 1,870 (43.6%) | 2,056 (48.0%) | 894 (+1 in S) | 43.5% |
| O2 | 2,394 | 2,055 (85.8%) | 170 (7.1%) | 169 (7.1%) | 74 | **43.8%** |
| O2-noinline | 4,068 | 2,865 (70.4%) | 480 (11.8%) | 723 (17.8%) | 379 (+1 in S) | 52.4% |

**The O2 gap is the ceiling, not the engine.** On the functions the pinned metric can score as
perfect, kuna converts 43.8% at O2 against 43.5% at O0. The pinned-metric ceiling is 169
functions (7.06%) at O2; with #93 crediting it is 339 (14.2%).

* **Inlined-away callers.** decbench counts `DW_TAG_inlined_subroutine` children as the outer
  function's locals. 416 O2 functions (17.4%) are blocked by inlined-callee variables only; in 79
  of them every own variable is already TP, and 26 have no own GT at all. O2-noinline: 216 / 118 / 47.
  The names are glibc FORTIFY wrappers: `__fmt` 404 (O2) / 505, `__stream` 123 / 167, `__s`, `__dest`.
  Excluding them would take O2 74 -> 153 and O2-noinline 380 -> 498 with no engine change.
* **Restrict.** 1,798 O2 GT vars are restrict-qualified; 1,771 (98.5%) are register-only anyway.
* **Split / clone instances.** GT DIE low_pc differs from kuna's function in 1 O2 and 4
  O2-noinline functions (2 / 9 args). Functions kuna did not discover are outside the scored set by
  construction (typesweep scores the checkpoint's kuna function set), so they cannot move perfect%.
* **Rivals.** binja beats kuna at O2 (79 vs 68 on its functions); its 23 wins are mostly `dir_name`
  (kuna drops the pass-through `file` arg) plus register-only GT named `result`/`i` that binja's
  own variable names happen to match.

## 4. Top reachable blocker shapes: witnesses and hypotheses

**A. Program struct pointer -> `void *` (163 solo, #93-reachable only).** chcon -O2-noinline
`hash_get_n_buckets` @0x68f0: kuna `unsigned long sub_68f0(void *a0) { return *(unsigned long *)((long)a0 + 0x10); }`;
DWARF `size_t hash_get_n_buckets (const Hash_table *table)`. 66 of the 141 O2-noinline one-shorts
are gnulib `Hash_table *` getters, 8 are bzip2 `EState *` helpers. Hypothesis: structsynth needs
two distinct offsets *and* offset 0 (`p5_types/kuna_structsynth.rs:748-757`), so a one-field
accessor declines and `ptrfromuse void` leaves `void *`. Nothing flows caller -> callee (protoorder
is callee-first), so the caller's struct never reaches the getter.

**B. Arity short (75 solo; 57 one-short).** gzip -O2 `gzip_base_name` @0xd290: kuna
`void sub_d290(void) { sub_dfd0(); // tail-call }`; DWARF `char *gzip_base_name (char const *fname)`.
The same run recovers the callee as `char * sub_dfd0(char *a0)`. Not tail-call specific: df -O2
`dir_name` @0x7550 is `void sub_7550(void) { if (sub_75b0()) return; ... }`. tar -O2
`open_diag`/`stat_diag`/`readlink_diag`/`savedir_diag` have the same shape. 17 of the 57
are unused `int sig` params of signal handlers (dd `siginfo_handler` -> `void sub_4960(void)`),
5 are x87 `long double` stack params (numfmt `absld` shows `long double v1; // stack + 0x8`
as a local), 10 are params the body never reads (`BZ2_bzflush`, `count_items (item *unused)`,
grep `private_malloc`), and the other 25 are a register forwarded to a call. Hypothesis: `AncestorRealistic` pops fail on an
unwritten, non-direct-write input (`substrate/funcdata_varnode.rs:2755-2764`), so a register
forwarded untouched is not a parameter. Observed: it survives when a later argument register is
written (ls `xstrcoll_ctime` -> `sub_5a80(a0,a1,sub_72a0)` keeps a0/a1). `protoorder types`
never moves arity (`p4_calls/kuna_protoorder.rs:7-11`). Measured: `--option protoorder lock`
also leaves `sub_d290` at `(void)`. The unused-handler case needs caller-side evidence
(a `signal`/`sigaction` sink's `void (*)(int)`).

**C. Program struct pointer -> integer / primitive pointer (52 solo).** du -O2-noinline
`duinfo_init` @0x4a40: kuna `void sub_4a40(unsigned long *a0) { *a0 = 0; a0[1] = 0; a0[2] = 0x8000000000000000; a0[3] = 0xffffffffffffffff; }`;
DWARF `duinfo_init (struct duinfo *a)`. Hypothesis: the Howard rule reads three or more contiguous
pointer-width fields as an array (`kuna_structsynth.rs` `is_array_shaped`, ~543-575).
diff `print_*_script (struct change *script)` becomes `long`: the pointer is only forwarded (see B).

**D. `char *` -> `unsigned long` / `void *` (53 solo).** gzip -O2 `gzip_error` @0xd3f0: kuna
`void sub_d3f0(unsigned long a0) { sub_f6b0(stderr,"\n%s: %s: %s\n",dat_1a850,0x1a420,a0); ...`;
DWARF `void gzip_error (char const *m)`. `sub_f6b0` is gnulib `rpl_fprintf`. Hypothesis: format
typing recognises only named libc families
(`kuna-analysis/src/analyzers/protos/mod.rs:275` `variadic_format_prototype`: a callee
qualifies only when its name is in the libc signature tables), so a
local printf wrapper types nothing. Second sub-shape: the pointer is only stored to a global
(sort `add_temp_dir`, diff `set_color_palette`), which leaves no evidence in the function.

**E. `void *` -> `long` / `unsigned long *` (128 solo, nearly all two-short).** ls -O2-noinline
`xstrcoll_ctime` @0x5c60: kuna `(unsigned long *a0, unsigned long *a1)` in `decompile-all` and
`(unsigned long, unsigned long)` under `--addr`; DWARF `(V a, V b)`, where `V` is `void const *`.
The 78 ls/dir/vdir comparators follow. chcon -O2 `raw_comparator` @0x63e0 is
`bool sub_63e0(long a0,long a1) { return a1 == a0; }` (22 + 22). Hypothesis: a pass-through
wrapper takes its callee's recovered pointer type by the protoorder vote (`kuna_protoorder.rs`
`call_argument_vote`, ~150-184), and a pointer that is only compared gets no pointer evidence.
The GT type comes from the callback contract (mpsort / hash_initialize comparators), which only
a sink-to-callback prototype rule would see. This is the same mechanism B's signal handlers need.

**Next: 32-bit int -> `unsigned long` (22 solo, all one-short).** ls -O2-noinline
`format_user_width` @0x7260: kuna `void sub_7260(unsigned long a0) ... sub_6ed0(0,a0 & 0xffffffff)`;
DWARF `int format_user_width (uid_t u)`. gzip `dup_safer`: `void sub_10c00(unsigned long a0) { sub_e0f0(a0,0,3); }`,
while the callee is recovered as `int sub_e0f0(int a0,int a1,unsigned long a2)`. Hypothesis: an
unlocked call site's trial takes the whole 8-byte register, and the callee's `int` vote cannot
narrow it (`kuna_protoorder.rs` ~172-184, size / storage refusal).

Default decision: none. This lane is investigation only.
