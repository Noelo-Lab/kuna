# castindex: default-on evaluation

Both arms of one build (`--option castindex off` against the default), base origin/main
`003db2dd8` (code `b3878d32e`).

| criterion | result |
|---|---|
| (a) `make test` with the new default | 675/675, PARITY OK; 0 datatest assertions moved |
| (b) `make test-stages` | PARITY OK; 0 existing assertions moved (1362/1362 before this test was added); `kuna-castindex.xml` adds 7, its pass 1 says `option castindex off` |
| (c) `make test-cli` | 237/237; no probe moved |
| (d) 444-slice typesweep | 1,615 -> 1,615 perfect, mean .3697 -> .3697, 0 improved, 0 worse, 0 moved on or off; `variables[]` byte-identical for all 10,748 functions, so arity and phantom counts cannot move |
| (e) speed, interleaved min-of-15 | SPEED_TABLE |
| (f) whole corpus, 45 binaries | 776 functions changed, 0 structural; 1,487 hunks: 998 subscript, 481 difference, 8 both, 0 outside the documented effect (`hunkclass.py`, `structcheck.py`) |
| (g) `p0_knowledge/modes.rs` | coherent: default-on and outside every preset, like `castarith`, `castimplied` and `castsign` |
| (h) castbench full, both directions | 38,602 -> 37,742 casts (1.021x -> 0.998x IDA), 220 functions fewer (860 casts), 0 more |

All pass: the default is `on`.
