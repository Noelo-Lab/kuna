# structheadless -- the default-on evaluation

The option ships `off`. Everything below was measured on build e2071591f (main
dbe854ba3 plus this branch), both arms of the same build.

## The criteria that hold it

| criterion | result |
|---|---|
| layout precision (the gate: fields-only >= .8713 on the published instrument) | **fails**: .8713 -> .8298 (880/1010 -> 1068/1287). With named embedded records, arrays of records and unions flattened in the ground truth (`layoutdepth.py`) it is .9594 -> .9526, and 262 of the 282 added claims are real members; the published instrument keeps `struct stat stat` as one 0x90-byte member, so a correct `f->stat.st_mode` claim at 0x30 is a miss there |
| (d) 444-slice typesweep, improved >= worse | **fails by one**: 1,609 -> 1,609 perfect, 1 improved (grep `bmexec_trans`), 2 worse (bzip2 `BZ2_bzReadClose` at -O0 and -O2-noinline: `BZFILE *` is `typedef void`, the record is the truer type and DWARF scores it a miss), 0 perfect functions lost |

## The other criteria

| criterion | result |
|---|---|
| (a) `make test` with the rule on | inert by construction: the console harness never marks a function closed, so no datatest can reach the rule (675/675 either way) |
| (b) `make test-stages` | inert for the same reason; the new stage test pins both passes |
| (c) `make test-cli` | not run with the rule as the default: the flip already fails above. With the option off the suite passes, and the two new probes pin both values |
| (e) speed, interleaved min-of-15, `decompile-all --json`, -O2 | fmt -0.18%, ls -2.60%, sort -1.37%, bash +2.98%: worst +2.98% against +5% |
| (f) whole-corpus hunks, 45 binaries | 3,450 of 20,230 functions change; 0 skeleton deltas; every other hunk read and classified (`hunks.md`) |
| (g) `p0_knowledge/modes.rs` | nothing to do: the option is `off|closed`, not in any preset, as `structmerge` is |
| (h) castbench full, both directions | 45,126 -> 42,008 casts (1.193 -> 1.111 times IDA); 221 functions fewer (-3,150), 20 more (+32), every one read (`analysis.md`) |

## What flipping it would buy and cost

It is the largest single cast reduction measured in the cast campaign that does
not touch printing: -6.9% of all casts on the shared set, and the O2-noinline
ratio to IDA falls to 1.053. Under decbench#93's crediting it is +124 perfect
functions. The costs are the ones listed: 12 declarations move from a pointer to
an integer (against 43 the other way), one program object can carry several
record names, bzip2's opaque handle, and the published precision instrument's
blindness to embedded records. A flip needs the precision gate restated at
depth, or the member-at-depth claims to be modelled as embedded records.
