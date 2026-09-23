# castcensus lane (MEASURE) — DONE
branch campaign/castcensus. READ-ONLY lane: no engine change, no PR (as briefed).
REPORT: /home/mahaloz/kwt/castcensus/.scratch/measure.md
Counter: .scratch/castcount.py (self-test 16/16; hand-check 696 spans, 0 FP / 0 FN)
Headline: on 4,815 address-matched functions kuna 44,821 casts vs ida 37,821 (+18.5%).
36.2% of kuna's casts are the *(T *)((long)P + K) laundering idiom; ida 1.1%.
99.8% of 3,429 constant-offset sites convert to ida's one-cast *((T *)P + K/w).
Campaign regression base->main +10.6%; ptrfromuse explains 85% of it.
No-op casts are only 0.2% (ida the same) => no suppression shortcut exists.
