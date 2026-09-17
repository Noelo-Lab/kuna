# optsweep: `--option elfmain off`

kuna `0147ccae` -- baseline arm = shipped defaults, variant arm = the flip. 18 slices, 1367 functions scored in both arms.

## Controls

- unchanged bodies: **1349**; scoring differently: **0** (OK)
- slug-only diffs: **0**; scoring differently: **0** (OK)
- baseline arm agrees with the tree's published verdict on **1364/1367** (99.8%)

## Headline

| scope | n | changed | GED base | GED var | perfect base | perfect var | dPerfect | dGED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| O0 | 860 | 9 | 9630 | 9629 | 384 | 384 | +0 | -1 |
| O2 | 507 | 9 | 19142 | 19141 | 143 | 143 | +0 | -1 |
| ALL | 1367 | 18 | 28772 | 28770 | 527 | 527 | +0 | -2 |

## Both directions

- **to perfect: 0**, **off perfect: 0** (ratio 0.00 : 1, McNemar z = +0.00)
- improved 2 / worsened 0

### per opt level

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| O0 | 860 | -1 | 0 | 0 | +0 |
| O2 | 507 | -1 | 0 | 0 | +0 |

### per architecture

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| x86-64 | 1367 | -2 | 0 | 0 | +0 |

### per project

| cell | n | dGED | to perfect | off perfect | net |
|---|---:|---:|---:|---:|---:|
| bzip2 | 176 | +0 | 0 | 0 | +0 |
| diffutils | 269 | +0 | 0 | 0 | +0 |
| findutils | 509 | +0 | 0 | 0 | +0 |
| grep | 159 | -2 | 0 | 0 | +0 |
| gzip | 254 | +0 | 0 | 0 | +0 |

### Biggest single movements

| direction | function | base | var |
|---|---|---:|---:|
