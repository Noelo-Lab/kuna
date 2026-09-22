# structsynth `locals` -- the default-flip procedure

Measured after the review round's fixes on origin/main 2888a2a94, and the
typesweep, the hunk sweep and every gate re-run after the rebase onto 656e6c701
(#710 `slotptr`, which types -O0 filler slots from the stored pointer)
(`copies_alone`/`storage_is_tied` by overlap on every RETURN,
`returned_beside_others`, `stored_text` for read-only literals, `points_past`),
both arms of one build unless noted: new default `locals` against the old
default `param`.

| step | criterion | result |
|---|---|---|
| (a) | `make test`, moved datatest assertions read | 675/675, PARITY OK; none moves |
| (b) | `make test-stages`, stage tests of the option pin their pass 1 | 1306/1306, PARITY OK; only `STRUCTSYNTH-LOCALS #1-#10` are new (pass 1 `option structsynth param`); no assertion of another feature moves; `docs/baseline-stages.json` re-recorded for those 10 keys only |
| (c) | `make test-cli`, probes pinning the old output | 221/221 (rebased; #710 added two); none moves |
| (d) | 444-slice typesweep new vs old default, improved >= worse | rebased: 1,472 -> 1,472 perfect, aggregate 3,773.19 both, 0 improved / 0 worse; of 65,377 decisions 708 differ only in `struct_N` numbering and 95 change type, every one an -O0 slot (typed by `slotptr` from the stored record) going from `undefined8`/`long *`/`int8 *` to `struct_N *` where DWARF names a struct pointer (44 `hash_entry *`, 9 `predicate *`, 8 `group *`, 7 `dev_ino *`, 6 `passwd *`, ...); none replaces a correct pointer; per-function scored variable count, tp, fp and fn identical. Before the rebase (2888a2a94): 1,353 both, 676 numbering-only differences, 0 type changes |
| (e) | interleaved min-of-15, fmt/ls/sort O2 + bash O2, worst delta <= +5% | fmt -8.19% (median +0.03%), ls -1.28%, sort +3.43%, bash +1.20%; sort's output is identical in both arms; re-run min-of-21 +5.63% (median +1.92%), and an A/A/B CPU-time run (a second `param` arm interleaved) reads param_b +6.66% over param with locals +4.60% between them: the sort delta is noise on a shared machine |
| (f) | whole-corpus decompile-all hunks, every hunk classified | 29 binaries, 12,939 functions: 872 change (769 numbering, 83 clean respellings with declarations and casts checked by `declcast.py`, 20 read: see `hunks.md`); 0 control-flow, call or arity deltas; against origin/main one more line (the `PTRSUB` printer fix, tar O0 `0x2c4f3`); identical classification after the rebase |
| (g) | `p0_knowledge/modes.rs` coherent | no preset names a `structsynth` value; every mode inherits the default |

`--jobs`: tar O2 `decompile-all --jobs 8`, interleaved min-of-11: 15.41 s under
`locals` against 14.96 s under `param` (+3.01%); 109 functions renamed in place
and 10 decompiled again (100 and 9 under `param`); the output is byte-identical
to the serial `--option protoorder off` run in both arms.

Layout against DWARF (14 builds, `localscore.py`): call-defined record locals
40 -> 116, 103 struct pointers, none anything else, 13 unjoined; the new
records' claimed fields 173/203 exact (0.852); parameter layouts unchanged.

Decision: flip `param` -> `locals`. `nest`/`all` stay opt-in for the reason
`docs/features/structnest/default-on-evaluation.md` gives.
