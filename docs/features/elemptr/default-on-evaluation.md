# elemptr: default-on evaluation

`elemptr` ships on. Both arms are one build of this branch on main `850e8c692`
(`--option elemptr off` against the default); the `off` arm is byte-identical to a
build of main on all 45 castbench binaries. Measured on the final engine (sign on
evidence, the table ledger, the index-of-other refusals).

| step | criterion | result |
|---|---|---|
| (a) `make test` | no datatest assertion moves | 675/675, PARITY OK, no per-test opt-out |
| (b) `make test-stages` | only the option's own effect moves | PARITY OK (1,405/1,405); 8 assertions of other options read the subscript for the same access (listed in `record.json`), `kuna-castindex.xml` turns `elemptr` off in both passes |
| (c) `make test-cli` | a moved probe has a reason | 2 probes of other options read the subscript for the same access; the two `elemptr` probes pin the rebuilt fixture |
| (d) 444-slice typesweep | improved >= worse, no perfect function lost | 1,615 -> 1,645 perfect, 171 better, 0 worse; 223 scored variables gained, 0 lost; no function's variable count moved |
| (e) speed | worst delta <= +5% | interleaved min-of-15 whole-binary: fmt +0.35%, ls -0.10%, sort -1.58%, bash +2.14% |
| (f) whole-corpus hunks | every hunk in the documented effect | 1,077 functions over 45 binaries, the 25 `structural.py` flags read; a disjoint 11-binary sweep read (analysis.md section 4) |
| (g) `modes.rs` | coherent | nothing to do: a default-on option is outside the `aggressive` preset's default-off scope |
| (h) castbench full | casts removed, every function with more read | 35,588 -> 33,553 (0.941x -> 0.887x IDA), 339 fewer (-2,054), 16 more (+19), all read (analysis.md section 3) |

Value preservation (analysis.md section 7): the compiled round trip in
`decompile_all_cli.rs` and two further fixtures of returned and shared tables print, with the
option on, exactly what the binary (or, where main already differs, the option
off) prints, compiled with gcc and clang.
