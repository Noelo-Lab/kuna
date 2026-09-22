# structsynth `locals` -- the default-flip procedure

Measured on origin/main 2888a2a94 (the build is byte-identical to the one on
2da619852, #706; the only commit between is documentation), both arms of one
build unless noted: new default `locals` against the old default `param`.

| step | criterion | result |
|---|---|---|
| (a) | `make test`, moved datatest assertions read | 675/675, PARITY OK; none moves |
| (b) | `make test-stages`, stage tests of the option pin their pass 1 | 1303/1303, PARITY OK; only `STRUCTSYNTH-LOCALS #1-#7` are new (pass 1 `option structsynth param`); no assertion of another feature moves; `docs/baseline-stages.json` re-recorded for those 7 keys only |
| (c) | `make test-cli`, probes pinning the old output | 219/219; none moves |
| (d) | 444-slice typesweep new vs old default, improved >= worse | 1,353 -> 1,353 perfect, aggregate 3,669.96 both, 0 improved / 0 worse; all 65,377 scored decisions the same type up to `struct_N` numbering; no function's variable count changes |
| (e) | interleaved min-of-15, fmt/ls/sort O2 + bash O2, worst delta <= +5% | quiet run, locals vs param: fmt -1.74%, ls -0.32%, sort +1.72%, bash -0.37% (against the tree before this change, worst +0.75%). Rebased build: fmt -0.26%, ls -5.04%, bash -0.32%, sort +7.33% while this worktree's own rust-test lane ran (load 47-63); sort re-run three times: +4.61% (min-of-15), +0.24% (min-of-21), +2.54% (min-of-15, load 14). Worst stable delta +2.54% (sort: its 10 s `main` gains a record and repeats the main loop once) |
| (f) | whole-corpus decompile-all hunks, every hunk classified | 16 binaries, 8,682 functions: 693 change (621 numbering, 61 mechanically verified respellings, 11 read), 0 control-flow deltas; against origin/main one more (the `PTRSUB` printer fix, tar O0 `0x2c4f3`) |
| (g) | `p0_knowledge/modes.rs` coherent | no preset names a `structsynth` value; every mode inherits the default |

`--jobs`: tar O2 `decompile-all --jobs 8`, interleaved min-of-11: 15.90 s under `locals` against 15.58 s under `param` (+2.05%); 115 functions with synthesized structures, 109 renamed in place and 10 decompiled again; the output is byte-identical to the serial `--option protoorder off` run in both arms. Before the widest-first order, the same run fell back to one ordered worker (26.0 s).

Decision: flip `param` -> `locals`. `nest`/`all` stay opt-in for the reason
`docs/features/structnest/default-on-evaluation.md` gives.
