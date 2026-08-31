"""The spoiler filter: decide which of a challenge's extras/ may enter a tester arena.

`extras/` is the one part of the dataset that is *both* the only human-written task statement
("find the serial for user `foo`") *and* a spoiler channel (author source, keygens, hint files
holding valid serials). It cannot be copied wholesale and it cannot be dropped wholesale, so
every file goes through `classify()`.

Five layers, applied in this order; the first four are cheap and name-based, the last two read
the bytes:

  1. allowlist   only README*, *.txt, *.nfo, *.md survive at all       -> not-allowlisted
  2. name        hint*/serial*/key*/solution*/flag*/answer*/crack*      -> spoiler-name
  3. extension   .c .h .cpp .cc .py .rb .asm, makefile                  -> source
  4. flag bytes  the file contains meta.json's ground_truth.flag        -> contains-flag
  5. shape       >= 5 lines that look like license keys                 -> serial-shape
  6. url         the file names crackmes.one, where the writeup lives    -> spoiler-url

Layer 6 is not about the answer but about the *pointer* to it: an author readme that says
"posted on crackmes.one" hands a tester the one search term that finds the public writeup, and
it also poisons `scan_for_leak` -- an arena must never contain a string the tripwire treats as
proof of contamination, or every run in it is falsely marked contaminated.

Layer 5 exists because layers 2 and 4 both miss the worst real case in the dataset:
`challenges/5ab77f5533c5d40ad448c1ea/extras/GiveMeMoney.zip.__x/hints.txt` is 19 valid serials
for a challenge whose `ground_truth.flag` is null and whose `ships_source_code` is false, so
there is no flag to match; only the *shape* of its contents gives it away. Content is proof
and a filename is a guess, so when several layers fire the reported reason is the
content-derived one (see `_REASON_RANK`) -- that is what an auditor needs to see, and a
renamed spoiler file must not be recorded as if the name rule had caught it.

`scan_for_leak()` is the other half of containment: the post-hoc tripwire that greps a finished
tester transcript for the dataset path, the literal flag, the solutions-zip password
`crackmes.one`, or `solutions/`. A hit marks the run contaminated (observations are kept --
friction is friction -- but the solve outcome is voided).

CLI:
    python -m scripts.repipe.redact classify PATH... [--hexid H] [--json]
    python -m scripts.repipe.redact scan TRANSCRIPT [--hexid H] [--json]
"""
from __future__ import annotations

import argparse
import fnmatch
import json
import os
import re
import sys

from . import config

# Layer 1. Nothing outside this list is ever copied, whatever its contents.
ALLOW_NAME_GLOBS = ("readme*", "*.txt", "*.nfo", "*.md")

# Layer 2. Filenames authors use for the answer.
SPOILER_NAME_GLOBS = ("hint*", "serial*", "key*", "solution*", "flag*", "answer*", "crack*")

# Layer 6. The solutions archives' ZipCrypto password, and the site that hosts the writeups.
SPOILER_URL = "crackmes.one"

# Layer 3. Author source. Six challenges ship it; they stay testable from the binary.
SOURCE_EXTS = (".c", ".h", ".cpp", ".cc", ".py", ".rb", ".asm")
SOURCE_NAMES = ("makefile",)

# Layer 5 thresholds. 16 alnum chars is below the shortest serial in the dataset (26) and above
# any ordinary all-caps word; 5 lines keeps a task statement that quotes one example serial.
MIN_SERIAL_CHARS = 16
MIN_SERIAL_GROUP = 3
MIN_SERIAL_LINES = 5
MAX_SCAN_LINES = 4096

# A flag shorter than this is not searched for: two-character "flags" would match everything.
MIN_FLAG_LEN = 4

# Lower rank wins when several layers fire. Bytes outrank names on purpose.
_REASON_RANK = {"contains-flag": 0, "serial-shape": 1, "spoiler-url": 2,
                "spoiler-name": 3, "source": 4}

_SERIAL_TOKEN = re.compile(r"[A-Z0-9]+(?:-[A-Z0-9]+)*")

LEAK_TAGS = ("dataset-path", "literal-flag", "crackmes-one", "solutions-dir")


# --- helpers ----------------------------------------------------------------

def _name_of(path) -> str:
    return os.path.basename(str(path)).lower()


def _matches_any(name: str, globs) -> bool:
    return any(fnmatch.fnmatchcase(name, g) for g in globs)


def _as_text(data: bytes) -> str:
    """Decode for line scanning: UTF-8 where possible, latin-1 otherwise, NULs stripped.

    UTF-16 text files decode to NUL-interleaved latin-1; stripping NULs makes their serials
    look like every other serial to the shape rule instead of hiding them.
    """
    if isinstance(data, str):
        text = data
    else:
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            text = data.decode("latin-1", "replace")
    return text.replace("\x00", "")


def _flag_of(meta) -> str:
    if not meta:
        return ""
    flag = (meta.get("ground_truth") or {}).get("flag")
    return flag if isinstance(flag, str) else ""


# --- layer 5: serial shape --------------------------------------------------

def _token_is_serial_like(token: str) -> bool:
    groups = token.split("-")
    alnum = sum(len(g) for g in groups)
    if alnum < MIN_SERIAL_CHARS:
        return False
    if not any(c.isdigit() for c in token) or not any(c.isalpha() for c in token):
        return False
    if len(groups) == 1:
        return True
    return all(len(g) >= MIN_SERIAL_GROUP for g in groups)


def serial_like_lines(text) -> int:
    """Count lines carrying at least one license-key-shaped token.

    Tokens rather than whole lines, so `Serial: ABCD-1234-...` counts the same as a bare key.
    """
    hits = 0
    for i, line in enumerate(_as_text(text).splitlines()):
        if i >= MAX_SCAN_LINES:
            break
        line = line.strip()
        if not line:
            continue
        for token in _SERIAL_TOKEN.findall(line):
            if _token_is_serial_like(token):
                hits += 1
                break
    return hits


def looks_like_serial_list(text) -> bool:
    return serial_like_lines(text) >= MIN_SERIAL_LINES


# --- the classifier ---------------------------------------------------------

def explain(path, data: bytes, meta=None) -> dict:
    """Full audit record for one extras/ file: decision, reported reason, every reason that fired."""
    name = _name_of(path)
    if not _matches_any(name, ALLOW_NAME_GLOBS):
        return {"path": str(path), "decision": "drop", "reason": "not-allowlisted",
                "reasons": ["not-allowlisted"], "serial_lines": 0}

    reasons = []
    if _matches_any(name, SPOILER_NAME_GLOBS):
        reasons.append("spoiler-name")

    stem, ext = os.path.splitext(name)
    if ext in SOURCE_EXTS or name in SOURCE_NAMES or stem in SOURCE_NAMES:
        reasons.append("source")

    flag = _flag_of(meta)
    if len(flag) >= MIN_FLAG_LEN:
        text = _as_text(data)
        if flag in text or flag.lower() in text.lower():
            reasons.append("contains-flag")

    n_serial = serial_like_lines(data)
    if n_serial >= MIN_SERIAL_LINES:
        reasons.append("serial-shape")

    if SPOILER_URL in _as_text(data).lower():
        reasons.append("spoiler-url")

    if not reasons:
        return {"path": str(path), "decision": "copy", "reason": "allowlisted",
                "reasons": [], "serial_lines": n_serial}
    reasons.sort(key=lambda r: _REASON_RANK.get(r, 99))
    return {"path": str(path), "decision": "drop", "reason": reasons[0],
            "reasons": reasons, "serial_lines": n_serial}


def classify(path, data: bytes, meta=None):
    """("copy"|"drop", reason) for one file under a challenge's extras/."""
    d = explain(path, data, meta)
    return d["decision"], d["reason"]


def classify_file(path, meta=None):
    with open(path, "rb") as fh:
        return classify(path, fh.read(), meta)


def sanitize(text, meta=None) -> str:
    """Blank the flag and the writeup site out of a string before it is shown to a tester.

    For metadata that must be quoted verbatim-ish (a challenge whose *name* is
    "Crackmes.one RE CTF 2026 - FlipVM"), not for author prose -- prose is dropped whole by
    `classify`, because silently editing a task statement changes the task.
    """
    out = _as_text(text)
    flag = _flag_of(meta)
    if len(flag) >= MIN_FLAG_LEN:
        out = re.sub(re.escape(flag), "[redacted]", out, flags=re.IGNORECASE)
    return re.sub(re.escape(SPOILER_URL), "[redacted]", out, flags=re.IGNORECASE)


# --- the post-hoc tripwire --------------------------------------------------

def scan_for_leak(text, meta=None) -> list:
    """Tags for every spoiler channel visible in a tester transcript. Empty list = clean.

    Deliberately blunt: `solutions/` and `crackmes.one` are flagged wherever they appear,
    because a tester that merely *names* the writeup archive has already been somewhere it
    should not have been.
    """
    blob = _as_text(text)
    lower = blob.lower()
    hits = []

    dataset = str(config.dataset_root())
    for candidate in {dataset, config.DEFAULT_DATASET, os.path.basename(dataset)}:
        if candidate and candidate in blob:
            hits.append("dataset-path")
            break

    flag = _flag_of(meta)
    if len(flag) >= MIN_FLAG_LEN and (flag in blob or flag.lower() in lower):
        hits.append("literal-flag")

    if "crackmes.one" in lower:
        hits.append("crackmes-one")
    if "solutions/" in lower:
        hits.append("solutions-dir")

    return sorted(set(hits), key=LEAK_TAGS.index)


# --- CLI --------------------------------------------------------------------

def _load_meta(hexid):
    if not hexid:
        return None
    p = config.dataset_root() / "challenges" / hexid / "meta.json"
    with open(p) as fh:
        return json.load(fh)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.redact",
                                 description="Spoiler filter for arena extras/ and transcripts.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    c = sub.add_parser("classify", help="copy/drop verdict for one or more extras/ files")
    c.add_argument("paths", nargs="+")
    c.add_argument("--hexid", help="challenge whose ground_truth.flag to match against")
    c.add_argument("--json", action="store_true")

    s = sub.add_parser("scan", help="tripwire a tester transcript for spoiler leakage")
    s.add_argument("path")
    s.add_argument("--hexid")
    s.add_argument("--json", action="store_true")

    args = ap.parse_args(argv)
    meta = _load_meta(getattr(args, "hexid", None))

    if args.cmd == "classify":
        out = []
        for p in args.paths:
            with open(p, "rb") as fh:
                out.append(explain(p, fh.read(), meta))
        if args.json:
            print(json.dumps({"results": out}, indent=2))
        else:
            for r in out:
                print("%-6s %-16s %s" % (r["decision"], r["reason"], r["path"]))
        return 0

    with open(args.path, "rb") as fh:
        hits = scan_for_leak(fh.read(), meta)
    if args.json:
        print(json.dumps({"path": args.path, "contaminated": bool(hits), "hits": hits}, indent=2))
    else:
        print("contaminated: %s" % (", ".join(hits) if hits else "no"))
    return 1 if hits else 0


if __name__ == "__main__":
    sys.exit(main())
