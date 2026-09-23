# Round G arms — one build or option arm per landed PR (444 slices, pinned metric)

Per-commit ladder (each commit built from origin/main history, own decomp_dbg beside it):

| arm | binary | perfect | mean | aggregate | O0 | O2 | O2-noinline |
|---|---|---:|---:|---:|---:|---:|---:|
| round F | `_final-f/kuna` (810b7dc86) | 1,522 | .3553 | 3,818.77 | 1,065 | 74 | 383 |
| + #711 `calleevote` | `_final-g/b711/kuna` (9e07ab931) | 1,567 | .3597 | 3,866.26 | 1,090 | 80 | 397 |
| + #714 `libcwiden` | `_final-g/b714/kuna` (1c57f06b6) | 1,582 | .3637 | 3,909.56 | 1,104 | 80 | 398 |
| + #716 `passthrough` on | `_final-g/kuna` (10a0db235) | 1,609 | .3688 | 3,963.45 | 1,104 | 89 | 416 |

Option arms on the round-G build:

| arm | perfect | mean | aggregate | O0 | O2 | O2-noinline |
|---|---:|---:|---:|---:|---:|---:|
| `--option calleevote off` | 1,575 | .3659 | 3,932.87 | 1,083 | 82 | 410 |
| `--option passthrough off` | 1,582 | .3637 | 3,909.56 | 1,104 | 80 | 398 |
| default | 1,609 | .3688 | 3,963.45 | 1,104 | 89 | 416 |

`passthrough off` reproduces the `1c57f06b6` step value for value (same perfect, mean and aggregate).

Moves:

| from | to | improved | worse | onto perfect | off perfect |
|---|---|---:|---:|---:|---:|
| round F | +#711 | 144 | 3 | 46 | 1 |
| +#711 | +#714 | 202 | 4 | 15 | 0 |
| +#714 | +#716 (round G) | 111 | 0 | 27 | 0 |
| `calleevote off` | round G | 101 | 3 | 35 | 1 |
| `passthrough off` | round G | 111 | 0 | 27 | 0 |
| round F | round G | 457 | 7 | 88 | 1 |
| baseline | round G | 3,341 | 17 | 763 | 2 |

Every worse row, attributed:

| function | GT | before | after | step |
|---|---|---|---|---|
| `gzip::xmemdup` -O0 (1.0 → 0.5), -O2 and -O2-noinline (0.4 → 0.2) | `void *p` | `void *` | `char *` | #711 |
| `ls::gobble_file`, `dir::gobble_file`, `vdir::gobble_file` -O0 (0.697 → 0.606) | three `_Bool` slots | `bool` | `char` | #714 |
| `cp::make_dir_parents_private` -O0 (0.72 → 0.68) | `_Bool missing_dir` | `bool` | `char` | #714 |
