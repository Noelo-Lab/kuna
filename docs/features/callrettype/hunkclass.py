"""Classify every function callrettype changes: types and conversions only, or more.

hunkclass.py OFF.json ON.json   (kuna decompile-all --json of one binary, option off / on)

A function whose code differs is normalized on both sides: casts dropped, struct_N and
vN/aN numbering erased, the literal spellings a type picks (0 / NULL / '\\0') unified,
declarations and the signature's types erased.  Equal after that = the change is types,
conversions and names only; anything else is a STATEMENT change and is listed for reading.
"""
import json, re, sys

CAST = re.compile(r"\((?:const )?(?:unsigned |signed )?(?:struct )?[A-Za-z_]\w*(?: \w+)?\s*\**\)(?=[\w(*&\"'-])")
DECL = re.compile(r"^\s+[A-Za-z_][\w ]*?(?:[\s*]+|\s*\(\*)[va]\d+\)?(?:\s*\[\d+\])?;.*$")


def norm(code):
    out = []
    for i, l in enumerate(code.splitlines()):
        if i == 0 or DECL.match(l):
            l = re.sub(r"[A-Za-z_][\w ]*?[\s*]+(?=[va]\d+\b)", "", l) if i else re.sub(r"^.*?(\w+\()", r"\1", l)
            l = re.sub(r"\([^)]*\)", "()", l) if i == 0 else "DECL"
        l = CAST.sub("", l)
        l = re.sub(r"struct_\d+", "struct_N", l)
        l = re.sub(r"\b[va]\d+\b", "v", l)
        l = re.sub(r"\bNULL\b|'\\0'", "0", l)
        l = re.sub(r"\s+//.*$", "", l)
        out.append(l.strip())
    out = [l for l in out if l != "DECL"]
    return out


def deep(lines):
    """norm, then also the spellings a type picks for the same expression: a literal's
    suffix and base, -1 at any width, an address as &dat_X / a string / a number, a member
    or element access as ->field_0xN / [k] / *(v + k), and parentheses."""
    out = []
    for l in lines:
        l = re.sub(r"\b(0x[0-9a-f]+|\d+)U\b", r"\1", l)
        l = re.sub(r"\b0xf{8}(?:f{8})?\b|-1\b", "M1", l)
        l = re.sub(r"&dat_([0-9a-f]+)", r"0x\1", l)
        l = re.sub(r'"(?:[^"\\]|\\.)*"', "ADDR", l)
        l = re.sub(r"\b0x[0-9a-f]{4,}\b", "ADDR", l)
        l = re.sub(r"->field_0x[0-9a-f]+|\[[^\]]*\]|\*\(v \+ [^)]*\)|&v|\*v\b", "v", l)
        l = re.sub(r"'(?:[^'\\]|\\.)+'", "C", l)
        l = re.sub(r"\b(?:0x[0-9a-f]+|\d+)\b", "C", l)
        l = re.sub(r"[()\s*&]", "", l)
        out.append(l)
    return out


def main(off, on):
    f = {k: {x["address"]: x for x in json.load(open(p))["functions"]} for k, p in (("off", off), ("on", on))}
    changed = typeonly = spelling = 0
    stmt = []
    for a in sorted(set(f["off"]) & set(f["on"])):
        co, cn = f["off"][a].get("code") or "", f["on"][a].get("code") or ""
        if co == cn:
            continue
        changed += 1
        no, nn = norm(co), norm(cn)
        if no == nn:
            typeonly += 1
        elif deep(no) == deep(nn):
            spelling += 1
        else:
            stmt.append(hex(a))
    return {"changed": changed, "types_casts_names_only": typeonly, "expression_spelling_only": spelling,
            "statement_changes": stmt}


if __name__ == "__main__":
    print(json.dumps(main(sys.argv[1], sys.argv[2])))
