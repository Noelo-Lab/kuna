| binary | functions | baseline min | round E min | round F min | **round G min** | Δ G vs F (min / median) | Δ G vs baseline (min) |
|---|---:|---:|---:|---:|---:|---:|---:|
| coreutils fmt | 151 | 4,074.6 ms | 4,124.0 ms | 4,108.8 ms | **4,170.9 ms** | +1.51% / +1.49% | +2.36% |
| coreutils ls | 404 | 13,241.5 ms | 13,441.0 ms | 13,416.7 ms | **13,670.5 ms** | +1.89% / +1.66% | +3.24% |
| coreutils sort | 343 | 13,895.0 ms | 14,129.3 ms | 14,113.8 ms | **14,407.7 ms** | +2.08% / +2.19% | +3.69% |
| bash | 2538 | 83,623.9 ms | 85,939.1 ms | 85,628.4 ms | **87,630.5 ms** | +2.34% / +2.79% | +4.79% |

# fmt: first run G/F +1.51 min +1.49 med, G/base +2.36
# ls: first run G/F +1.89 min +1.66 med, G/base +3.24
# sort: first run G/F +2.08 min +2.19 med, G/base +3.69
# bash: first run G/F +2.34 min +2.79 med, G/base +4.79
