"""structscore B -> C -> D comparison table."""
import json, sys
from pathlib import Path
B = Path("/home/mahaloz/kwt/_final/ss"); C = Path("/home/mahaloz/kwt/_final-c/ss"); D = Path("/home/mahaloz/kwt/_final-d/ss")
rows = []
pool = {"B": [0, 0], "C": [0, 0], "D": [0, 0]}   # points_possible, points_lost per opt handled below
acc = {}
print("| opt | binary | TRex mean 0-6 C -> **D** | mean 0-5 | `is_c_struct` passed | `c_primitive` passed | params typed as a struct / GT | layout F1 (filler counted) |")
print("|---|---|---|---|---|---|---|---|")
tot = {k: {"pp": 0, "pl": 0, "pp5": 0, "pl5": 0} for k in ("C", "D")}
for opt in ("O0", "O2"):
    for b in ("fmt", "ls", "sort", "du"):
        try:
            c = json.loads((C / f"c-{opt}-{b}.json").read_text())[0]
            d = json.loads((D / f"d-{opt}-{b}.json").read_text())[0]
        except Exception as e:
            print(f"MISSING {opt} {b}: {e}", file=sys.stderr); continue
        def g(x, *k):
            for kk in k: x = x[kk]
            return x
        print(f"| {opt} | {b} | {g(c,'trex','mean')} -> **{g(d,'trex','mean')}** | "
              f"{g(c,'trex','mean_0_5')} -> {g(d,'trex','mean_0_5')} | "
              f"{g(c,'trex','steps','is_c_struct','passed')}/{g(c,'trex','steps','is_c_struct','reached')} -> "
              f"{g(d,'trex','steps','is_c_struct','passed')}/{g(d,'trex','steps','is_c_struct','reached')} | "
              f"{g(c,'trex','steps','c_primitive','passed')}/{g(c,'trex','steps','c_primitive','reached')} -> "
              f"{g(d,'trex','steps','c_primitive','passed')}/{g(d,'trex','steps','c_primitive','reached')} | "
              f"{g(c,'layout','params_kuna_typed_as_a_struct')} -> {g(d,'layout','params_kuna_typed_as_a_struct')}"
              f" / {g(d,'layout','ptr_to_struct_params')} | "
              f"{g(c,'layout','layout','f1')} -> {g(d,'layout','layout','f1')} |")
        for lab, x in (("C", c), ("D", d)):
            acc.setdefault((lab, opt), [0, 0.0, 0, 0.0])
            a = acc[(lab, opt)]
            a[0] += g(x, 'trex', 'gt_variables')
            a[1] += g(x, 'trex', 'mean') * g(x, 'trex', 'gt_variables')
            a[2] += g(x, 'layout', 'params_kuna_typed_as_a_struct')
            a[3] += g(x, 'layout', 'ptr_to_struct_params')
for lab in ("C", "D"):
    for opt in ("O0", "O2"):
        a = acc[(lab, opt)]
        print(f"pooled {lab} {opt}: TRex mean {a[1]/a[0]:.4f} over {a[0]} GT vars; struct params {a[2]}/{int(a[3])}")
