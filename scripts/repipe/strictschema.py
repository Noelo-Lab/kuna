"""Make a JSON Schema acceptable to OpenAI's strict structured-output mode, and check it.

`codex exec --output-schema` sends the schema straight to the API's `response_format`, which
enforces rules ordinary JSON Schema does not. Every one of these was learned from a 400 on a
live call, because nothing local rejects the schema:

  * every object must carry `required` listing EVERY key in `properties`, and
    `additionalProperties: false`. Optionality is expressed by admitting `null` in the type,
    never by leaving a key out of `required`;
  * every node needs an explicit `type` -- a bare `enum`, `const` or `{}` is rejected;
  * a `$ref` to a sibling FILE does not resolve, so nothing may be split across files;
  * `"type": ["object", "null"]` is still an object and still needs all of the above --
    matching only the string `"object"` is the easy way to miss one.

`check()` is wired into tools/repipe/smoke.sh so a schema edit cannot silently break every
tester in the fleet with a 400 that only appears once real money is being spent.
"""
from __future__ import annotations

import argparse
import json
import sys

_JSON_TYPE = {str: "string", bool: "boolean", int: "integer",
              float: "number", type(None): "null"}


def _is_object(node):
    t = node.get("type")
    return t == "object" or (isinstance(t, list) and "object" in t)


def _has_shape(node):
    return any(k in node for k in ("properties", "items", "enum", "const"))


def strictify(node):
    """Return a copy of ``node`` that satisfies the rules above."""
    if isinstance(node, list):
        return [strictify(v) for v in node]
    if not isinstance(node, dict):
        return node
    out = {k: strictify(v) for k, v in node.items()}
    if "properties" in out:
        out["required"] = list(out["properties"].keys())
        out["additionalProperties"] = False
        if "type" not in out:
            out["type"] = "object"
    if "enum" in out and "type" not in out:
        kinds = sorted({_JSON_TYPE[type(v)] for v in out["enum"]})
        out["type"] = kinds[0] if len(kinds) == 1 else kinds
    if "const" in out and "type" not in out:
        out["type"] = _JSON_TYPE[type(out["const"])]
    return out


def check(schema):
    """[] if the schema is strict-mode clean, else a list of (path, problem)."""
    bad = []

    def walk(n, path="$"):
        if isinstance(n, dict):
            if "properties" in n or _is_object(n):
                props = set(n.get("properties") or {})
                missing = props - set(n.get("required") or [])
                if missing:
                    bad.append((path, "required is missing %s" % sorted(missing)))
                if props and n.get("additionalProperties") is not False:
                    bad.append((path, "additionalProperties must be false"))
            if str(n.get("$ref", "")).endswith(".json"):
                bad.append((path, "$ref to a file does not resolve"))
            if _has_shape(n) and "type" not in n and "$ref" not in n:
                bad.append((path, "no explicit type"))
            for k, v in n.items():
                walk(v, "%s.%s" % (path, k))
        elif isinstance(n, list):
            for i, v in enumerate(n):
                walk(v, "%s[%d]" % (path, i))

    walk(schema)
    return bad


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.strictschema")
    ap.add_argument("schema")
    ap.add_argument("--fix", action="store_true", help="rewrite the file in place")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)

    with open(args.schema) as fh:
        doc = json.load(fh)
    if args.fix:
        doc = strictify(doc)
        tmp = args.schema + ".tmp"
        with open(tmp, "w") as fh:
            json.dump(doc, fh, indent=2)
            fh.write("\n")
        import os
        os.replace(tmp, args.schema)
    bad = check(doc)
    if args.json:
        print(json.dumps({"ok": not bad, "problems": [{"path": p, "problem": w} for p, w in bad]},
                         indent=2))
    elif bad:
        for p, w in bad:
            print("%s  %s" % (p, w), file=sys.stderr)
    else:
        print("strict-mode OK: %s" % args.schema)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
