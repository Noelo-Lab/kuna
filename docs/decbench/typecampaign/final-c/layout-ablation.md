# Layout precision: default vs `--option protoorder off` (round-C binary)

`structscore --layout`, filler counted, the eight goal-3 builds.

| opt | binary | default P (tp/pred) | protoorder off P (tp/pred) | params typed as a struct |
|---|---|---|---|---|
| O0 | fmt | 0.882 (15/17) | 0.882 (15/17) | 4 / 4 |
| O0 | ls | 0.429 (109/254) | 0.824 (136/165) | 40 / 32 |
| O0 | sort | 0.493 (143/290) | 0.776 (173/223) | 36 / 35 |
| O0 | du | 0.443 (94/212) | 0.798 (146/183) | 38 / 34 |
| O2 | fmt | 1.000 (8/8) | 1.000 (8/8) | 2 / 2 |
| O2 | ls | 0.417 (93/223) | 0.783 (130/166) | 24 / 24 |
| O2 | sort | 0.628 (86/137) | 0.793 (123/155) | 21 / 22 |
| O2 | du | 0.401 (61/152) | 0.811 (133/164) | 28 / 29 |

- **default**: precision 609/1293 = 0.4710, recall 0.0645, F1 0.1135
- **protoorder off**: precision 864/1081 = 0.7993, recall 0.0915, F1 0.1642
- round B, same instrument: precision 798/1021 = 0.7816, recall 0.0845, F1 0.1525
