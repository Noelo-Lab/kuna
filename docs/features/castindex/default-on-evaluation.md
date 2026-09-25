# castindex: default-on evaluation

Both arms of one build (`--option castindex off` against the default), base origin/main
`c960fb18d` (`castimplied`, `castarith`, `castsign` and `globalref` on).

| criterion | result |
|---|---|
| (a) `make test` with the new default | 675/675, PARITY OK; 0 datatest assertions moved |
| (b) `make test-stages` | PARITY OK, 1377/1377; 0 existing assertions moved (1370 before this test); `kuna-castindex.xml` adds 7, its pass 1 says `option castindex off` |
| (c) `make test-cli` | TESTCLI_ROW |
| (d) 444-slice typesweep | 1,615 -> 1,615 perfect, aggregate 3,973.97 -> 3,973.97, 0 improved, 0 worse, 0 moved on or off; `variables[]` byte-identical for all 10,748 functions, so arity and phantom counts cannot move |
| (e) speed, interleaved min-of-15 | SPEED_ROW |
| (f) whole corpus, 45 binaries | 776 functions changed, 0 structural; 1,487 hunks: 998 subscript, 481 difference, 8 both, 0 outside the documented effect (`hunkclass.py`, `structcheck.py`); option off byte-identical to main on 45/45 |
| (g) `p0_knowledge/modes.rs` | coherent: default-on and outside every preset, like `castarith`, `castimplied`, `castsign` and `globalref` |
| (h) castbench full, both directions | 37,477 -> 36,617 casts (0.991x -> 0.968x IDA), 220 functions fewer (860 casts), 0 more |

All pass: the default is `on`.
