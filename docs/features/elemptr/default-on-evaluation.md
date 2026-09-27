# elemptr: default-on evaluation

`elemptr` ships on. Both arms are one build of this branch on main `f96e80805`
(`--option elemptr off` against the default); the `off` arm is byte-identical to a
build of main `f96e80805` on all 45 castbench binaries and all 32 of the disjoint
sweep. Measured on the final engine (sign on evidence, the table ledger, the
index-of-other refusals, the pointer balance, the batch agreement over every
function that holds a global or names a table, the narrow load).

| step | criterion | result |
|---|---|---|
| (a) `make test` | no datatest assertion moves | 675/675, PARITY OK, no per-test opt-out |
| (b) `make test-stages` | only the option's own effect moves | PARITY OK (1,463/1,463 on `f96e80805`; the baseline gains the 22 `kuna-elemptr.xml` keys and loses none); 8 assertions of other options read the subscript for the same access and 3 read the narrow load of a 4-byte field (listed in `record.json`), `kuna-castindex.xml` turns `elemptr` off in both passes |
| (c) `make test-cli` | a moved probe has a reason | 2 probes of other options read the subscript for the same access; the three `elemptr` probes pin the rebuilt fixture (250/250) |
| (d) 444-slice typesweep | improved >= worse, no perfect function lost | 1,631 -> 1,674 perfect, 189 better, 0 worse; 243 scored variables gained, 0 lost; no function's variable or argument count moved |
| (e) speed | worst delta <= +5% | interleaved min-of-15 whole-binary on `f96e80805`: fmt +0.14%, ls +0.17%, sort +0.21%, bash +0.39% (on `b273c2259`: +0.05%, +0.11%, +0.32%, +1.65%) |
| (f) whole-corpus hunks | every hunk in the documented effect | 1,255 functions over 45 binaries, the 19 `structural.py` flags read; a disjoint 32-binary sweep read (analysis.md section 4) |
| (g) `modes.rs` | coherent | nothing to do: a default-on option is outside the `aggressive` preset's default-off scope |
| (h) castbench full | casts removed, every function with more read | 33,289 -> 32,073 (0.880x -> 0.848x IDA), 301 fewer (-1,242), 18 more (+26), all read (analysis.md section 3) |

Value preservation (analysis.md section 7): the compiled round trip in
`decompile_all_cli.rs` and further fixtures of returned and shared tables, globals
stepped by bytes, tables named by their first element and narrow fields at a page
end print, with the option on, exactly what the binary (or, where main already
differs, the option off) prints, compiled with gcc and clang; a global the header
declines is compiled at the pointer type its comment lists.
