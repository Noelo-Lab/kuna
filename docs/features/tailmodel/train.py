"""Fit the `tailform model` weights from two decbench runs with the decision forced each way.

usage: train.py --harness DIR --dump-wrap DIR --dump-guard DIR --tag-wrap TAG --tag-guard TAG
                [--opts O2-noinline,O0] [--out-rs FILE] [--out-json FILE] [--max-lines N]

Each run decompiles the corpus with `--option tailform wrap` (or `guard`) and `KUNA_TAILFORM_DUMP=<dir>`, which
makes kuna append one JSON line per decided function-tail site: the binary, the function address, the site and
its features.  A function is a training example when both runs decided exactly one site, the same one, and its
control flow matches the source under exactly one of the two forms; that form is the label.  The script reports,
held out by project (5 folds grouped by project, and the even/odd project-hash FIT/CHECK halves the harness uses),
how often the logistic model and the shipped 4-line threshold pick the matching form, and how many functions each
gets right, per optimisation level; then it trains on every example and writes the Rust weight table.

The model decides only in functions the unoptimized-code detector (`kuna_unoptimized`) passes: in unoptimized
functions the threshold is the better rule, so those sites are neither training examples nor model decisions.
"""
import argparse
import collections
import glob
import hashlib
import json
import os
import pickle
import random
import sys
from pathlib import Path

import numpy as np
from sklearn.linear_model import LogisticRegression
from sklearn.model_selection import GroupKFold
from sklearn.preprocessing import StandardScaler

# `switches` and `gotos` are dumped but not model inputs: a handful of examples have any, so their raw weights grow
# large and decide the few very long tails that have many.
NUMERIC = ["void", "wrapped", "lines", "top", "stmts", "calls", "assigns", "rets", "ifs", "loops", "tail_call",
           "only_calls", "cond_ops", "cond_call", "null_param", "p_top", "p_stmts", "p_rets", "p_loops", "p_guards"]
GATED = ["unopt", "unopt_wrapped"]
ONEHOT = {
    "last": ["call", "assign", "ret", "if", "loop", "switch", "other"],
    "cond": ["truthy", "eqz", "eq", "ne", "rel", "and", "or", "bit", "not_bit", "call", "not_call", "other",
             "not_other"],
    "k": ["void", "const", "var"],
}
NAMES = NUMERIC + [f"{k}={v}" for k, vs in ONEHOT.items() for v in vs]
C = 0.01


def vector(feat, numeric=NUMERIC):
    """The model input, exactly as `TailFeatures::value` computes it."""
    x = [np.sign(feat[n]) * np.log1p(abs(feat[n])) for n in numeric]
    x += [1.0 if feat[k] == v else 0.0 for k, vs in ONEHOT.items() for v in vs]
    return x


def fit_half(project):
    return int(hashlib.md5(project.encode()).hexdigest(), 16) % 2 == 0


def load_dump(d):
    out = collections.defaultdict(dict)
    for p in glob.glob(os.path.join(d, "*.jsonl")):
        for line in open(p):
            r = json.loads(line)
            out[(r["file"], r["addr"])][(r["site"], r["feat"]["wrapped"])] = r
    return out


def load_scores(harness, tag, opts):
    out = {}
    for opt in opts:
        for p in glob.glob(str(Path(harness) / "fresh" / tag / opt / "*" / "*.score.pkl")):
            r = pickle.loads(Path(p).read_bytes())
            for fn, v in r["fn"].items():
                out[(opt, r["project"], r["stem"], fn)] = v
    return out


def function_names(harness, opts):
    """(binary path, kuna function address) -> (opt, project, stem, source function name), as the harness maps them."""
    sys.path.insert(0, str(harness))
    import fresh
    out = {}
    for opt, project, stem, binary in fresh.plan(opts):
        base = fresh.image_base(binary)
        for fn, addr in fresh.markers_for(opt, project, stem).items():
            for a in (addr, addr + base, addr & ~1, addr | 1, (addr + base) & ~1):
                out.setdefault((binary, a), (opt, project, stem, fn))
    return out


def examples(args, opts):
    wrap, guard = load_dump(args.dump_wrap), load_dump(args.dump_guard)
    names = function_names(args.harness, opts)
    sw, sg = load_scores(args.harness, args.tag_wrap, opts), load_scores(args.harness, args.tag_guard, opts)
    rows, stats = [], collections.Counter()
    for key, sites in wrap.items():
        stats["functions with a site"] += 1
        other = guard.get(key, {})
        if len(sites) != 1 or set(sites) != set(other):
            stats["more than one site, or not the same site in both runs"] += 1
            continue
        name = names.get(key)
        if name is None:
            stats["not a scored corpus function"] += 1
            continue
        r = next(iter(sites.values()))
        iw, ig = sw.get(name) is True, sg.get(name) is True
        stats["usable"] += 1
        rows.append({"key": name, "opt": name[0], "project": name[1], "feat": r["feat"], "iw": iw, "ig": ig})
    return rows, stats


def threshold_choice(rows, n):
    return np.array([r["feat"]["lines"] <= n for r in rows])


def report(rows, choose, base_choose, label):
    """Matching-form share on decidable sites and functions right, against the threshold, per level and half."""
    iw = np.array([r["iw"] for r in rows])
    ig = np.array([r["ig"] for r in rows])
    dec = iw ^ ig
    out = []
    for opt in sorted({r["opt"] for r in rows}):
        for half, name in ((True, "FIT"), (False, "CHECK")):
            sel = np.array([r["opt"] == opt and fit_half(r["project"]) == half for r in rows])
            d = sel & dec
            got = np.where(choose, iw, ig)[sel].sum()
            base = np.where(base_choose, iw, ig)[sel].sum()
            acc = ((choose == iw) & d).sum() / max(1, d.sum())
            bacc = ((base_choose == iw) & d).sum() / max(1, d.sum())
            out.append(f"  {label:28s} {opt:12s} {name:5s} decidable {d.sum():5d}  matching form {acc:6.1%} "
                       f"(threshold {bacc:6.1%})  functions right {got:5d} vs {base:5d}  net {got - base:+4d}")
    return "\n".join(out)


def fit(X, y, c=None):
    sc = StandardScaler().fit(X)
    lr = LogisticRegression(max_iter=5000, C=C if c is None else c).fit(sc.transform(X), y)
    return sc, lr


def predict(model, X):
    sc, lr = model
    return lr.predict(sc.transform(X)) == 1


def cross(rows, X, use, train_frac=1.0, seed=0, c=None):
    """Leave-projects-out choices (the model where `use`, else the threshold): 5 project folds, FIT/CHECK halves."""
    iw = np.array([r["iw"] for r in rows])
    ig = np.array([r["ig"] for r in rows])
    dec = (iw ^ ig) & use
    proj = np.array([r["project"] for r in rows])
    base = threshold_choice(rows, 4)
    rng = random.Random(seed)
    folds = np.zeros(len(rows), bool)
    for tr, te in GroupKFold(n_splits=5).split(X, iw, proj):
        tr = tr[dec[tr]]
        if train_frac < 1.0:
            tr = np.array(sorted(rng.sample(list(tr), max(2, int(len(tr) * train_frac)))))
        folds[te] = predict(fit(X[tr], iw[tr], c), X[te])
    halves = np.zeros(len(rows), bool)
    isfit = np.array([fit_half(p) for p in proj])
    for h in (True, False):
        tr = np.where((isfit != h) & dec)[0]
        if train_frac < 1.0:
            tr = np.array(sorted(rng.sample(list(tr), max(2, int(len(tr) * train_frac)))))
        halves[isfit == h] = predict(fit(X[tr], iw[tr], c), X[isfit == h])
    return np.where(use, folds, base), np.where(use, halves, base)


def rust_table(names, w, b, n, path):
    lines = [
        "//! (kuna `tailform model`) The saved logistic weights of the function-tail",
        "//! form decision.  Generated by `docs/features/tailmodel/train.py` from",
        f"//! {n} labelled decbench sites; do not edit.",
        "",
        "/// Model inputs: `sign(x)*ln(1+|x|)` of a numeric feature, or a one-hot",
        "/// `kind=value` indicator (see `kuna_tailform::TailFeatures::value`).",
        "pub const NAMES: &[&str] = &[",
    ]
    lines += [f'    "{nm}",' for nm in names]
    lines += ["];", "", "/// One weight per name; a positive score means wrap.", "pub const WEIGHTS: &[f64] = &["]
    lines += [f"    {x!r}," for x in w]
    lines += ["];", "", f"pub const BIAS: f64 = {b!r};", ""]
    Path(path).write_text("\n".join(lines))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--harness", required=True)
    ap.add_argument("--dump-wrap", required=True)
    ap.add_argument("--dump-guard", required=True)
    ap.add_argument("--tag-wrap", required=True)
    ap.add_argument("--tag-guard", required=True)
    ap.add_argument("--opts", default="O2-noinline,O0")
    ap.add_argument("--max-lines", type=int, default=4)
    ap.add_argument("--out-rs")
    ap.add_argument("--out-json")
    ap.add_argument("--drop", default="", help="comma-separated feature names to leave out (ablation)")
    ap.add_argument("--ungated", action="store_true", help="decide unoptimized functions too, with the detector as features")
    ap.add_argument("--fit-only", action="store_true", help="train the written table on the FIT half of the projects only "
                    "(an end-to-end run of it then measures the CHECK half held out)")
    ap.add_argument("--check-only", action="store_true", help="train the written table on the CHECK half only")
    args = ap.parse_args()
    opts = args.opts.split(",")
    rows, stats = examples(args, opts)
    for k, v in stats.items():
        print(f"{k}: {v}")
    numeric = NUMERIC + (GATED if args.ungated else [])
    all_names = numeric + [f"{k}={v}" for k, vs in ONEHOT.items() for v in vs]
    keep = [i for i, n in enumerate(all_names) if n not in set(filter(None, args.drop.split(",")))]
    names = [all_names[i] for i in keep]
    X = np.array([vector(r["feat"], numeric) for r in rows], dtype=float)[:, keep]
    iw = np.array([r["iw"] for r in rows])
    ig = np.array([r["ig"] for r in rows])
    use = np.ones(len(rows), bool) if args.ungated else np.array([not r["feat"]["unopt"] for r in rows])
    dec = iw ^ ig
    print(f"model sites (not unoptimized): {use.sum()} of {len(rows)}, decidable {(use & dec).sum()}")
    for opt in opts:
        sel = np.array([r["opt"] == opt for r in rows])
        print(f"{opt}: {sel.sum()} sites, {(sel & dec).sum()} decidable ({(sel & dec & iw).sum()} wrap), "
              f"both forms match {(sel & iw & ig).sum()}, neither {(sel & ~iw & ~ig).sum()}")
    base = threshold_choice(rows, args.max_lines)
    folds, halves = cross(rows, X, use)
    print("held out, 5 project folds:")
    print(report(rows, folds, base, "logistic model"))
    print("held out, FIT/CHECK halves (train on one half, test on the other):")
    print(report(rows, halves, base, "logistic model"))
    acc = lambda c: ((c == iw) & dec & use).sum() / (dec & use).sum()
    print(f"model sites, matching form: threshold {acc(base):.1%}, folds {acc(folds):.1%}, halves {acc(halves):.1%}")
    for frac in (0.25, 0.5, 0.75):
        f2, h2 = cross(rows, X, use, train_frac=frac, seed=1)
        print(f"training on {frac:.0%} of each fold's examples: matching form folds {acc(f2):.1%}, halves {acc(h2):.1%}")
    for c in (0.003, 0.03, 0.1):
        f2, h2 = cross(rows, X, use, c=c)
        print(f"C={c}: matching form folds {acc(f2):.1%}, halves {acc(h2):.1%}")
    train = dec & use
    if args.fit_only:
        train &= np.array([fit_half(r["project"]) for r in rows])
    if args.check_only:
        train &= np.array([not fit_half(r["project"]) for r in rows])
    sc, lr = fit(X[train], iw[train])
    w = lr.coef_[0] / sc.scale_
    b = lr.intercept_[0] - (lr.coef_[0] * sc.mean_ / sc.scale_).sum()
    print(f"final model on {train.sum()} decidable sites; largest standardized weights:")
    for i in np.argsort(-np.abs(lr.coef_[0]))[:12]:
        print(f"   {names[i]:16s} {lr.coef_[0][i]:+.3f}")
    full = np.where(use, X @ w + b > 0, base)
    print("final model on all sites (in-sample):")
    print(report(rows, full, base, "logistic model"))
    if args.out_rs:
        rust_table(names, [float(x) for x in w], float(b), int(train.sum()), args.out_rs)
        print("wrote", args.out_rs)
    if args.out_json:
        json.dump({"names": names, "w": [float(x) for x in w], "b": float(b),
                   "standardized": {names[i]: float(lr.coef_[0][i]) for i in range(len(names))}},
                  open(args.out_json, "w"), indent=1)


if __name__ == "__main__":
    main()
