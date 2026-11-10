"""Static self-check of kuna_fix_diff.py, the guard against parallel branches merged textually.

`problems(source, fix_ids)` reads the tool's source without running it and lists what is wrong:
a module-level name bound twice (two functions of one name: the later silently replaces the earlier),
a table entry (RECOGNISERS, CONTEXT_RECOGNISERS) whose recogniser is not a module-level function or
does not take the table's arguments, and a table entry that names no fix: its key must be a `KunaFix`
id (the strings `KunaFix::id` returns in kuna_fixes.rs), the name of a kuna-only option of options.toml,
or a key of FIX_GROUPS, whose ids must all be such ids. Standard library only: CI runs it with a bare Python.
"""
import ast
import os
import re

TABLES = {'RECOGNISERS': 2, 'CONTEXT_RECOGNISERS': 7}
FIXES_RS = os.path.join(os.path.dirname(os.path.abspath(__file__)), os.pardir, os.pardir, 'decompiler',
                        'nonnative', 'kuna-jvm', 'src', 'p0_knowledge', 'kuna_fixes.rs')
FIX_ID = re.compile(r'KunaFix::\w+\s*=>\s*"([a-z0-9_]+)"')
OPTIONS_TOML = os.path.join(os.path.dirname(os.path.abspath(__file__)), os.pardir, os.pardir, 'decompiler',
                            'nonnative', 'kuna-jvm', 'options.toml')
KUNA_OPTION = re.compile(r'^option = "(jvm_[a-z0-9_]+)"\nvineflower = ""$', re.M)


def fix_ids(path=FIXES_RS):
    """The strings `KunaFix::id` returns, and the names of kuna's own options (`kuna_option_ids`)."""
    with open(path, encoding='utf-8') as f:
        text = f.read()
    start = text.find('fn id(self)')
    return set(FIX_ID.findall(text[start:] if start >= 0 else '')) | kuna_option_ids()


def kuna_option_ids(path=OPTIONS_TOML):
    """The kuna-only rows of options.toml (`vineflower = ""`, e.g. jvm_javac_layout): a change such an
    option makes is recognised, and its cases named, by the option's name."""
    if not os.path.isfile(path):
        return set()
    with open(path, encoding='utf-8') as f:
        return set(KUNA_OPTION.findall(f.read()))


def bound_names(node):
    """The names a module-level statement binds (not the tables it changes in place)."""
    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
        return [node.name]
    targets = list(node.targets) if isinstance(node, ast.Assign) else \
        [node.target] if isinstance(node, (ast.AnnAssign, ast.AugAssign)) else []
    out = []
    while targets:
        t = targets.pop()
        if isinstance(t, ast.Name):
            out.append(t.id)
        elif isinstance(t, (ast.Tuple, ast.List)):
            targets += t.elts
        elif isinstance(t, ast.Starred):
            targets.append(t.value)
    return out


def literal(node, constants):
    """The value of a string, a tuple or list of strings, a module-level name of one, or a slice of it."""
    if isinstance(node, ast.Name) and node.id in constants:
        return literal(constants[node.id], constants)
    if isinstance(node, ast.Subscript) and isinstance(node.slice, ast.Slice):
        seq, s = literal(node.value, constants), node.slice
        bounds = [None if b is None else ast.literal_eval(b) for b in (s.lower, s.upper, s.step)]
        return None if seq is None else seq[slice(*bounds)]
    try:
        return ast.literal_eval(node)
    except ValueError:
        return None


def problems(source, ids):
    tree = ast.parse(source)
    out, seen, functions, constants = [], {}, {}, {}
    for node in tree.body:
        for name in bound_names(node):
            if name in seen:
                out.append(f'line {node.lineno}: `{name}` is bound again (first at line {seen[name]})')
            seen.setdefault(name, node.lineno)
        if isinstance(node, ast.FunctionDef):
            functions[node.name] = node
        elif isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            constants[node.targets[0].id] = node.value
    groups = literal(constants['FIX_GROUPS'], constants) if 'FIX_GROUPS' in constants else {}
    if not isinstance(groups, dict):
        out.append('FIX_GROUPS is not a literal dict of fix ids')
        groups = {}
    for group, members in groups.items():
        if not members:
            out.append(f'FIX_GROUPS[{group!r}] names no fix')
        for fix in members:
            if fix not in ids:
                out.append(f'FIX_GROUPS[{group!r}] names {fix!r}, not a KunaFix id')

    def entry(table, line, key, value, factory_arg=None):
        if not isinstance(key, str):
            out.append(f'line {line}: a {table} key is not a string literal')
            return
        if key not in ids and key not in groups:
            out.append(f'line {line}: {table}[{key!r}] names no KunaFix id (nor a FIX_GROUPS entry)')
        name = value.id if isinstance(value, ast.Name) else \
            value.func.id if isinstance(value, ast.Call) and isinstance(value.func, ast.Name) else None
        if name is None:
            out.append(f'line {line}: {table}[{key!r}] is not a function or a call of one')
        elif name not in functions:
            out.append(f'line {line}: {table}[{key!r}] names `{name}`, which is not a module-level function')
        elif isinstance(value, ast.Name):
            args = functions[name].args
            if args.vararg is None and len(args.posonlyargs + args.args) != TABLES[table]:
                out.append(f'line {line}: {table}[{key!r}] is `{name}`, which does not take {TABLES[table]} '
                           f'arguments')

    for node in tree.body:
        stmt = node.value if isinstance(node, ast.Expr) else node
        if isinstance(stmt, ast.Assign) and len(stmt.targets) == 1:
            target = stmt.targets[0]
            if isinstance(target, ast.Name) and target.id in TABLES:
                if not isinstance(stmt.value, ast.Dict) or None in stmt.value.keys:
                    out.append(f'line {node.lineno}: {target.id} is not a dict literal')
                    continue
                for k, v in zip(stmt.value.keys, stmt.value.values):
                    entry(target.id, k.lineno, literal(k, constants), v)
            elif (isinstance(target, ast.Subscript) and isinstance(target.value, ast.Name)
                  and target.value.id in TABLES):
                entry(target.value.id, node.lineno, literal(target.slice, constants), stmt.value)
        elif (isinstance(stmt, ast.Call) and isinstance(stmt.func, ast.Attribute)
              and isinstance(stmt.func.value, ast.Name) and stmt.func.value.id in TABLES):
            table = stmt.func.value.id
            gen = stmt.args[0] if stmt.func.attr == 'update' and len(stmt.args) == 1 else None
            if not (isinstance(gen, ast.GeneratorExp) and len(gen.generators) == 1
                    and isinstance(gen.elt, ast.Tuple) and len(gen.elt.elts) == 2
                    and isinstance(gen.generators[0].target, ast.Name)
                    and isinstance(gen.elt.elts[0], ast.Name)
                    and gen.elt.elts[0].id == gen.generators[0].target.id):
                out.append(f'line {node.lineno}: {table} changed in a form the self-check cannot read '
                           f'(use `{table}[id] = f` or `{table}.update((fix, f(fix)) for fix in LIST)`)')
                continue
            keys = literal(gen.generators[0].iter, constants)
            if not isinstance(keys, (list, tuple)):
                out.append(f'line {node.lineno}: {table}.update iterates over something not a literal list')
                continue
            for key in keys:
                entry(table, node.lineno, key, gen.elt.elts[1])
    for table in TABLES:
        if table not in constants:
            out.append(f'{table} is not defined')
    return out
