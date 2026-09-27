"""survey.py - how often the mode or the language changes what a symbol means.

For each binary, every function is decompiled by the native CLI in Automatic
and in Fast (and in C and in Rust).  A C-level local `vN` whose declared
storage (`// stack - 0x18`, `// eax`) differs between the modes is a key that
would name two different variables on two peers; one declared in one mode
only is a key the other peer's engine rejects.  The language comparison
checks the whole variable table.

    make binaries && python3 docs/features/decompile2-collab/spike/survey.py [binary ...]
"""
import json
import os
import re
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))
KUNA = os.path.join(ROOT, 'decompiler/target/release/kuna')
FIXTURES = os.path.join(ROOT, 'decompiler/crates/kuna-analysis/tests/fixtures')
DEFAULT = ['i386_pie_nl', 'regglobal_fmt_x86_64', 'calleevote_budget_x86_64', 'fauxware', 'katavm_level1_x86_64']
DECL = re.compile(r'^\s+[^;=()]+?\s+\**(v\d+)(\[[^\]]*\])?;\s*(?://\s*(.*))?$')
ENV = dict(os.environ, SLEIGHHOME=os.path.join(ROOT, 'specs'), KUNA_SPECS=os.path.join(ROOT, 'specs'))


def decompile_all(binary, mode, language='auto'):
    out = subprocess.run([KUNA, 'decompile-all', binary, '--json', '--mode', mode, '--language', language],
                         capture_output=True, text=True, env=ENV, timeout=900, check=True).stdout
    return {f['address_hex']: f for f in json.loads(out).get('functions', [])}


def decls(fn):
    found = {}
    for line in (fn.get('code') or '').split('\n'):
        m = DECL.match(line)
        if m:
            found[m.group(1)] = (m.group(3) or '').strip()
    return found


def table(fn):
    return [(v['name'], v.get('kind'), v.get('stack_offset'), v.get('size'), v.get('arg_index'))
            for v in fn.get('variables', [])]


totals = dict(functions=0, moved=0, one_mode=0, lang_differ=0)
for name in sys.argv[1:] or DEFAULT:
    binary = name if os.path.exists(name) else os.path.join(FIXTURES, name)
    auto, fast = decompile_all(binary, 'auto'), decompile_all(binary, 'fast')
    moved, one_mode, example = 0, 0, None
    for addr, fn in auto.items():
        if addr not in fast:
            continue
        a, f = decls(fn), decls(fast[addr])
        for v in set(a) | set(f):
            if v in a and v in f:
                if a[v] and f[v] and a[v] != f[v]:
                    moved += 1
                    example = example or f"{fn['name']}: {v} is '{a[v]}' in Automatic, '{f[v]}' in Fast"
            else:
                one_mode += 1
    c, rust = decompile_all(binary, 'auto', 'c'), decompile_all(binary, 'auto', 'rust')
    lang_differ = sum(1 for addr in c if table(c[addr]) != table(rust.get(addr, {})))
    print(f"{os.path.basename(binary)}: {len(auto)} functions ({len(fast)} in Fast); "
          f"same vN, different storage: {moved}; vN in one mode only: {one_mode}; "
          f"variable tables differ C vs Rust: {lang_differ}" + (f"\n    e.g. {example}" if example else ''))
    totals['functions'] += len(auto)
    totals['moved'] += moved
    totals['one_mode'] += one_mode
    totals['lang_differ'] += lang_differ
print(f"SURVEY — {totals['functions']} functions: {totals['moved']} vN name a different variable in Fast, "
      f"{totals['one_mode']} exist in one mode only; {totals['lang_differ']} variable tables differ between C and Rust")
