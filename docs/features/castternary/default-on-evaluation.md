# castternary: default-on evaluation

Base: origin/main `c960fb18d`. Both arms are the same build of this branch:
the default (on) and `--option castternary off`. The off arm prints exactly what
origin/main prints: its castbench output is byte-identical to main's on all 45
binaries, so the refactor of the printer's integer-literal path is inert.

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675, PARITY OK. No datatest assertion moved (the datatest harness applies no mode, so `iteexpr` is off and only `COPY`-arm conditionals print). |
| (b) | `make test-stages` | 1,380/1,380. The only new passing keys are the 10 of `kuna-castternary.xml` (pass 1 sets `option castternary off`); no existing stage assertion moved. `docs/baseline-stages.json` re-recorded for those 10 keys only. |
| (c) | `make test-cli` | 237/237; no probe pins an arm cast. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | off 1,615 perfect, mean .3697; on 1,615, mean .3697. All 10,748 functions score identically, and every function's tp/fp/fn/gt_vars/decomp_vars row is identical, so no variable or argument is added or removed (the option changes C text only; `variables[]` is untouched). |
| (e) | speed, interleaved min-of-15, `decompile-all` fmt/ls/sort/bash O2 | min-of-15 deltas fmt -2.09%, ls -1.90%, sort +2.98%, bash +1.51% (medians -2.04%, +1.47%, -0.96%, -1.57%); worst +2.98%, within the +5% budget. The rule runs once per printed conditional. Measured under load from other runs (load average 12-36), both arms interleaved. `speed.json`. |
| (f) | whole-corpus `decompile-all` before/after | 418 binaries (all 256 O0 binaries of the decbench tree, the 152 O2 binaries outside coreutils and bash, and 10 coreutils O2), 144,159 functions (128 of the binaries print no conditional at all): 23 functions change, in 13 binaries (coreutils tail, diffutils diff, openssh scp/sftp/sftp-server/ssh-pkcs11-helper/ssh-sk-helper, cleanflight O0 and O2, crazyflie cf2/firmware O0 and O2). Every changed line is a conditional whose only difference is one arm's integer widening left out; line counts are unchanged and no other line moves (`corpus-hunks.json`). |
| (g) | `p0_knowledge/modes.rs` | nothing to add: the option is on by default, so every preset has it; the `aggressive` preset's `iteexpr` is what prints computed arms as conditionals, and the rule covers them. |
| (h) | castbench full, both directions | 37,477 -> 37,474 casts on the 4,815 functions kuna and IDA both emit (0.991x IDA either way, 196.6 per kloc, 31.2 per 100 statements): 2 functions fewer (diff O0 0xe2f8 -2, tail O0 0x5a69 -1), **0 functions with more casts**. The shape the option targets, a promoted byte against an `int` arm, does not occur in that corpus; see `analysis.md` for the census of the 176 arm casts there and why the rest stay. |

All criteria pass; the option ships default on.

## Landing base

Re-measured on origin/main `04f693d82`, where `castindex` (#727) prints table
lookups by a variable index as subscripts. Every criterion above holds. castbench:
36,617 -> 36,614 casts (0.968x IDA either way), 2 functions fewer (the same
diff and tail sites), 0 more; `castternary off` is byte-identical to main on
all 45 binaries. The 444-slice typesweep is 1,615 perfect in both arms with
every value and variables[] row identical. The 418-binary corpus changes the
same 23 functions with the same lines as `corpus-hunks.json`, each only by an
arm cast.
