# structscore, `--option calleevote off` vs the round-G default (same eight builds)

| opt | binary | TRex mean (off -> default) | `c_primitive` passed | params typed as a struct | layout F1 (filler) |
|---|---|---|---|---|---|
| O0 | fmt | 4.8571 -> 4.8571 | 218/295 -> 218/295 | 4 -> 4 | 0.0524 -> 0.0524 |
| O0 | ls | 4.4997 -> 4.539 | 756/1026 -> 779/1047 | 44 -> 63 | 0.2099 -> 0.2179 |
| O0 | sort | 4.3212 -> 4.3623 | 616/802 -> 632/818 | 36 -> 45 | 0.1731 -> 0.1768 |
| O0 | du | 4.3375 -> 4.3996 | 546/730 -> 573/757 | 39 -> 61 | 0.153 -> 0.1786 |
| O2 | fmt | 2.1896 -> 2.2014 | 100/137 -> 101/138 | 2 -> 2 | 0.039 -> 0.039 |
| O2 | ls | 1.7913 -> 1.8019 | 250/376 -> 257/382 | 25 -> 31 | 0.2536 -> 0.2594 |
| O2 | sort | 2.0321 -> 2.0371 | 258/337 -> 260/337 | 21 -> 22 | 0.1799 -> 0.1812 |
| O2 | du | 1.949 -> 1.9788 | 219/304 -> 230/315 | 29 -> 40 | 0.2094 -> 0.2291 |

pooled off O0: TRex 4.4317 over 5216 GT vars; struct params 123/538
pooled off O2: TRex 1.9377 over 4413 GT vars; struct params 77/297
pooled default O0: TRex 4.4747 over 5216 GT vars; struct params 173/538
pooled default O2: TRex 1.9520 over 4413 GT vars; struct params 95/297
