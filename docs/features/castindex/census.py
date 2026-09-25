#!/usr/bin/env python3
"""Census of 64-bit integer casts of a bare variable ('(long)v') over the castbench shared set.

  census.py ARM_DIR [--ida] [--examples N] [--json OUT]
Classifies every (i64|u64)<var> cast by the operand's declared type (pointer or integer) and its
context. With --ida, counts IDA's (i64|u64)<var> casts on the same functions by the same classes.
"""
import collections, json, re, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB
import castcount as CC


def casts_with_index(toks, vocab):
    n = len(toks)
    match = CC.matching(toks)
    out, cast_close = [], set()
    for i in range(n):
        if toks[i].text != "(":
            continue
        j = match.get(i)
        if j is None or j == i + 1:
            continue
        if not CC.cast_allowed_before(toks, i, cast_close):
            continue
        k = j + 1
        if k >= n:
            continue
        nt = toks[k]
        if not (nt.kind in ("id", "num", "str", "chr") or nt.text in CC.UNARY_START_PUNCT):
            continue
        ty = CC.parse_type_name(toks, i + 1, j, vocab)
        if ty is None:
            continue
        cast_close.add(j)
        out.append((ty, CC.operand_shape(toks, k, n), i, j))
    return out


DECL = re.compile(r"^\s*(?:const\s+)?([A-Za-z_][\w ]*?)\s*(\*+)?\s*([A-Za-z_]\w*)\s*(?:\[[^\]]*\])?\s*;")


def declared_types(text):
    """{name: 'ptr'|'int'|'other'} from the signature and local declarations."""
    d = {}
    lines = text.splitlines()
    sig = next((L for L in lines if "(" in L and not L.strip().startswith("//")), "")
    m = re.search(r"\((.*)\)", sig)
    if m:
        for p in m.group(1).split(","):
            p = p.strip()
            mm = re.match(r"(.*?)([A-Za-z_]\w*)\s*$", p)
            if mm and mm.group(1).strip():
                d[mm.group(2)] = "ptr" if "*" in mm.group(1) else "int"
    for L in lines[1:]:
        mm = DECL.match(L.split("//")[0])
        if mm and mm.group(1).split()[0] not in ("return", "goto", "break", "continue"):
            ty = mm.group(1)
            if mm.group(2):
                d[mm.group(3)] = "ptr"
            elif re.search(r"\b(char|short|int|long|bool|uint\d*|int\d+)\b", ty) and "struct" not in ty:
                d[mm.group(3)] = "int"
            else:
                d[mm.group(3)] = "other"
    return d


I64 = {"i64", "u64"}


def ida_kind(ty, text):
    return ty in I64


def classify(toks, i, j, decl, casts_at):
    """Context class of the cast (toks[i..j]) of the variable toks[j+1]."""
    var = toks[j + 1].text
    vk = decl.get(var, "unk")
    after = toks[j + 2].text if j + 2 < len(toks) else ""
    after2 = toks[j + 3] if j + 3 < len(toks) else None
    before = toks[i - 1].text if i > 0 else ""
    prev_close = i - 1
    # is the other operand of +/- after us a cast var / const / scaled
    def term_after(k):
        """shape of the term starting at k: 'const','castvar','var','scaled','addr','other'"""
        if k >= len(toks):
            return "none"
        t = toks[k]
        if t.kind == "num":
            nx = toks[k + 1].text if k + 1 < len(toks) else ""
            return "scaled" if nx == "*" else "const"
        if t.text == "(" and k in casts_at:
            jj = casts_at[k]
            nx = toks[jj + 2].text if jj + 2 < len(toks) else ""
            if toks[jj + 1].text == "&":
                return "addr"
            return "scaledcast" if nx in ("*", "<<") else "castvar"
        if t.kind == "id":
            nx = toks[k + 1].text if k + 1 < len(toks) else ""
            if nx in ("*", "<<"):
                return "scaled"
            if nx in ("(",):
                return "call"
            return "var"
        if t.text == "&":
            return "addr"
        return "other"
    if vk == "ptr":
        if after in ("+", "-"):
            ta = term_after(j + 3)
            if after == "-" and ta in ("castvar",):
                return "P_diff"
            if ta == "const":
                return "P_plus_const"
            if ta in ("scaled", "scaledcast"):
                return "P_plus_scaled"
            if ta in ("castvar", "var", "call"):
                return "P_plus_var"
            if ta == "addr":
                return "P_plus_addr"
            return "P_plus_other"
        if before == "-" and i >= 2:
            # (long)q - (long)p : we are the subtrahend
            return "P_diff_rhs"
        if before == "+":
            return "P_rhs_of_plus"
        if after in ("<", ">", "<=", ">=", "==", "!="):
            return "P_cmp"
        if before in ("<", ">", "<=", ">=", "==", "!="):
            return "P_cmp"
        if after in ("&", "|", "^", ">>", "<<", "%", "/", "*"):
            return "P_bitop"
        return "P_other"
    if vk == "int":
        if after in ("*", "<<"):
            return "I_scaled"
        if before in ("+",) or after in ("+", "-"):
            return "I_addsub"
        return "I_other"
    return "U_" + vk


def walk(arm, ida_side=False, want_examples=0):
    cnt = collections.Counter()
    ex = collections.defaultdict(list)
    ida_cnt = 0
    for opt, proj, b in CB.corpus("full"):
        kp = Path(arm) / opt / proj / f"{b}.c"
        ip = CB.RES / opt / proj / "decompiled" / f"ida_{b}.c"
        try:
            ksrc = kp.read_text(errors="replace")
            isrc = ip.read_text(errors="replace")
        except OSError:
            continue
        kvoc = CC.harvest_types(CC.tokenize(ksrc), ksrc)
        ivoc = CC.harvest_types(CC.tokenize(isrc), isrc)
        kf = {a: t for _, a, t in CC.split_functions(ksrc)}
        if_ = {a: t for _, a, t in CC.split_functions(isrc)}
        for a in sorted(set(kf) & set(if_)):
            text = kf[a]
            toks = CC.tokenize(text)
            cl = casts_with_index(toks, kvoc)
            casts_at = {i: j for _, _, i, j in cl}
            decl = declared_types(text)
            for ty, sh, i, j in cl:
                if ty not in I64 or sh != "<var>":
                    continue
                c = classify(toks, i, j, decl, casts_at)
                cnt[c] += 1
                if len(ex[c]) < want_examples:
                    line = toks[i].line
                    ex[c].append(f"{opt}/{b}@{hex(a)}: {text.splitlines()[line - 1].strip()[:160]}")
            if ida_side:
                it = CC.tokenize(if_[a])
                for ty, sh, *_ in CC.find_casts(it, ivoc):
                    if ty in I64 and sh == "<var>":
                        ida_cnt += 1
    return cnt, ex, ida_cnt


if __name__ == "__main__":
    arm = sys.argv[1]
    nex = int(next((a.split("=")[1] for a in sys.argv if a.startswith("--examples=")), "3"))
    cnt, ex, ida = walk(arm, "--ida" in sys.argv, nex)
    tot = sum(cnt.values())
    print(f"(i64|u64)<var> casts on shared functions: {tot}   IDA: {ida}")
    for c, n in cnt.most_common():
        print(f"{n:6}  {c}")
        for e in ex[c]:
            print("        " + e)
    js = next((a.split("=", 1)[1] for a in sys.argv if a.startswith("--json=")), None)
    if js:
        Path(js).write_text(json.dumps({"total": tot, "ida": ida, "classes": dict(cnt), "examples": ex}, indent=1))
