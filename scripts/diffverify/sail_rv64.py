"""Compose an Isla/Sail RV64 trace with Kuna's raw and final-IR obligations."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time


ROOT = Path(__file__).resolve().parents[2]
OUTPUT_LIMIT = 4 * 1024 * 1024
EXPRESSION_LIMIT = 65536
VARIABLE = re.compile(r"v[0-9]+")
WORD = re.compile(r"#(?:x[0-9a-fA-F]+|b[01]+)")
TOKEN = re.compile(r'\s+|;[^\r\n]*|\(|\)|\|[^|\r\n]*\||"(?:[^"\\]|\\.)*"|[^\s();|"]+')
OPERATORS = frozenset("""
    = distinct not and or xor ite concat bvnot bvneg bvadd bvsub bvmul
    bvand bvor bvxor bvshl bvlshr bvashr bvult bvule bvugt bvuge
    bvslt bvsle bvsgt bvsge
""".split())


def sexps(source):
    outer = []
    stack = [outer]
    end = 0
    for match in TOKEN.finditer(source):
        if end != match.start():
            raise ValueError("invalid Isla or SMT token")
        end = match.end()
        token = match[0]
        if token[0].isspace() or token.startswith(";"):
            continue
        if token == "(":
            nested = []
            stack[-1].append(nested)
            stack.append(nested)
            if len(stack) > 128:
                raise ValueError("symbolic trace nesting limit")
        elif token == ")":
            if len(stack) == 1:
                raise ValueError("unexpected end of symbolic form")
            stack.pop()
        else:
            stack[-1].append(token)
    if len(stack) != 1 or end != len(source):
        raise ValueError("truncated symbolic form")
    return outer


def term(value, symbols):
    if isinstance(value, str):
        if value in symbols:
            return symbols[value]
        if value in ("true", "false") or WORD.fullmatch(value):
            return value
    elif value:
        op, *args = value
        if isinstance(op, str) and op in OPERATORS:
            parts = [op]
            size = len(op) + 2
            for arg in args:
                part = term(arg, symbols)
                size += len(part) + 1
                if size > EXPRESSION_LIMIT:
                    raise ValueError("Isla expression expansion exceeds its limit")
                parts.append(part)
            return f"({' '.join(parts)})"
        if isinstance(op, list) and op[:1] == ["_"] and len(args) == 1:
            spec, *numbers = op[1:]
            width = {"extract": 2, "zero_extend": 1, "sign_extend": 1}.get(spec)
            if width is not None and len(numbers) == width and all(n.isascii() and n.isdecimal() for n in numbers):
                return f"((_ {spec} {' '.join(numbers)}) {term(args[0], symbols)})"
        if op == "_" and len(args) == 2:
            number, width = args
            if re.fullmatch(r"bv[0-9]+", number) and width in ("1", "8", "16", "32", "64", "128"):
                return f"(_ {number} {width})"
    raise ValueError(f"unsupported Isla expression {value!r}")


def disjunction(terms):
    terms = list(terms)
    if not terms:
        return "false"
    return terms[0] if len(terms) == 1 else f"(or {' '.join(terms)})"


def conjunction(terms):
    if not terms:
        return "true"
    return terms[0] if len(terms) == 1 else f"(and {' '.join(terms)})"


def import_trace(forms, inputs, output):
    if not forms or len(forms) > 16:
        raise ValueError("expected one to sixteen complete Isla traces")
    definitions = []
    paths = []
    for index, form in enumerate(forms):
        if not isinstance(form, list) or form[:1] != ["trace"]:
            raise ValueError("expected an Isla trace form")
        declarations = set()
        aliases = {}
        symbols = {}
        constraints = []
        seen_inputs = set()
        value = None
        cycles = 0
        branches = 0
        expanded = 0
        for event in form[1:]:
            match event:
                case ["assume-reg", register, "nil", _] if cycles == 0 and (
                    isinstance(register, str) and not re.fullmatch(r"\|x[0-9]+\|", register)
                ):
                    pass
                case ["cycle"]:
                    cycles += 1
                case ["define-enum", _, _, list()]:
                    pass
                case ["branch", str(number), str(location)] if number.isdecimal() and location.startswith('"'):
                    branches += 1
                case ["read-reg", "|cur_privilege|", "nil", "|Machine|"]:
                    pass
                case ["read-reg", "|mseccfg|", "nil", ["_", "struct", ["|bits|", "#x0000000000000000"]]]:
                    pass
                case ["declare-const", name, ["_", "BitVec", "64"]] if VARIABLE.fullmatch(name):
                    if name in declarations or name in symbols:
                        raise ValueError("duplicate Isla declaration")
                    declarations.add(name)
                case ["read-reg", register, "nil", name] if register in inputs and name in declarations and value is None:
                    alias = inputs[register]
                    if name in aliases and aliases[name] != alias:
                        raise ValueError("Sail trace merged distinct input registers")
                    aliases[name] = alias
                    symbols[name] = alias
                    seen_inputs.add(register)
                case ["define-const", name, rhs] if VARIABLE.fullmatch(name):
                    if name in declarations or name in symbols:
                        raise ValueError("duplicate Isla definition")
                    # Inlining keeps the bit-vector widths from the original SMT terms.
                    # These short, instruction-level expressions have an explicit output cap.
                    rendered = term(rhs, symbols)
                    expanded += len(rendered)
                    symbols[name] = rendered
                case ["assert", cond]:
                    condition = term(cond, symbols)
                    expanded += len(condition)
                    constraints.append(condition)
                case ["write-reg", register, "nil", rhs] if register == output and value is None:
                    value = term(rhs, symbols)
                case _:
                    head = event[0] if isinstance(event, list) and event else event
                    raise ValueError(f"unmodeled Isla trace event {head!r}")
            if expanded > OUTPUT_LIMIT // len(forms):
                raise ValueError("Isla trace expansion exceeds its limit")
        if cycles != 1 or value is None or declarations != aliases.keys() or seen_inputs != inputs.keys():
            raise ValueError("Isla trace does not cover exactly the declared input and output registers")
        if branches > len(constraints):
            raise ValueError("Isla trace omits a branch condition")
        guard = f"sail_path_{index}"
        result = f"sail_value_{index}"
        definitions.append(f"(define-fun {guard} () Bool {conjunction(constraints)})")
        definitions.append(f"(define-fun {result} () (_ BitVec 64) {value})")
        paths.append((guard, result))
    return "\n".join(definitions), paths


def digest(data):
    return hashlib.sha256(data).hexdigest()


def file_digest(path):
    with Path(path).open("rb") as read:
        return hashlib.file_digest(read, "sha256").hexdigest()


def capture(command, deadline, text=None):
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise TimeoutError("verification wall-clock deadline")
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        result = subprocess.run(command, input=text, text=True, stdout=stdout, stderr=stderr, timeout=remaining)
        if result.returncode:
            stderr.seek(0)
            reason = stderr.read(2048).decode("utf-8", "replace").strip()
            raise ValueError(f"{Path(command[0]).name} failed ({result.returncode}): {reason}")
        stdout.seek(0)
        data = stdout.read(OUTPUT_LIMIT + 1)
        if len(data) > OUTPUT_LIMIT:
            raise ValueError("symbolic verifier output exceeded its limit")
        return data.decode("utf-8")


def word_value(data):
    if isinstance(data, str) and WORD.fullmatch(data):
        return int(data[2:], 16 if data[1] == "x" else 2)
    if isinstance(data, list) and len(data) == 3 and data[0] == "_":
        return int(data[1].removeprefix("bv"))
    raise ValueError("solver did not return a concrete bit-vector")


def replay_counterexample(kuna, paths, model):
    forms = sexps(model)
    if len(forms) != 2 or forms[0] != "sat":
        raise ValueError("solver did not reproduce the formal counterexample")
    values = dict(forms[1])
    regs = {i: word_value(values[f"kuna_x{i}"]) for i in kuna["input_registers"]}
    sail = {word_value(values[value]) for guard, value in paths if values[guard] == "true"}
    if len(sail) != 1:
        raise ValueError("formal witness does not select one observable Sail result")
    raw = word_value(values[kuna["raw_value"]])
    final = word_value(values[kuna["ir_value"]])
    observation = {"registers": {f"x{i}": hex(bits) for i, bits in regs.items()},
                   "sail": hex(sail.pop()), "raw": hex(raw), "final_ir": hex(final)}
    try:
        from unicorn import UC_ARCH_RISCV, UC_MODE_RISCV64, Uc, UcError
        from unicorn import riscv_const
    except ImportError:
        return {**observation, "replay": "unicorn-unavailable"}
    try:
        address = int(kuna["pc"], 16)
        machine = Uc(UC_ARCH_RISCV, UC_MODE_RISCV64)
        machine.mem_map(address & ~0xfff, 4096)
        machine.mem_write(address, int(kuna["opcode"], 16).to_bytes(4, "little"))
        for i, bits in regs.items():
            machine.reg_write(getattr(riscv_const, f"UC_RISCV_REG_X{i}"), bits)
        machine.emu_start(address, address + 4, timeout=100000, count=1)
        if machine.reg_read(riscv_const.UC_RISCV_REG_PC) != address + 4:
            return {**observation, "replay": "incomplete"}
        obtained = hex(machine.reg_read(riscv_const.UC_RISCV_REG_X10))
    except UcError as error:
        return {**observation, "replay": "incomplete", "reason": str(error)}
    return {**observation, "unicorn": obtained,
            "replay": "confirmed" if observation["sail"] == obtained else "disagreed-with-sail"}


def validate(args, report):
    deadline = time.monotonic() + args.timeout_ms / 1000
    command = [args.verify, "rv64-terms", args.elf, args.function, "--sleighpath", args.sleighpath]
    if args.opcode_mutant is not None:
        command.extend(("--opcode-mutant", str(args.opcode_mutant)))
    report["phase"] = "kuna-encoding"
    kuna = json.loads(capture(command, deadline))
    opcode = int(kuna["opcode"], 16).to_bytes(4, "little").hex()
    command = [
        args.isla, "-A", args.architecture, "-C", args.config, "-T", "1",
        "-x", "-i", opcode, "-s", "--timeout", str(max(1, args.timeout_ms // 1000)),
    ]
    report["scope"] = {"projection": "rv64-a0-before-return", "backend_opcode_bytes": opcode, "privilege": "Machine"}
    report["kuna"] = kuna
    report["provenance"] = {
        "isla_sha256": file_digest(shutil.which(args.isla) or args.isla),
        "configuration_sha256": file_digest(args.config),
        "sail_ir_sha256": file_digest(args.architecture),
    }
    report["phase"] = "sail-execution"
    trace = capture(command, deadline)
    report["trace"] = {"text": trace, "sha256": digest(trace.encode())}
    inputs = {f"|x{i}|": f"kuna_x{i}" for i in kuna["input_registers"]}
    definitions, paths = import_trace(sexps(trace), inputs, f'|x{kuna["output_register"]}|')
    base = f'{kuna["symbols_smt2"]}\n{definitions}\n'
    raw, ir = kuna["raw_value"], kuna["ir_value"]
    raw_defined, ir_defined = kuna["raw_defined"], kuna["ir_defined"]
    coverage_gap = f"(not {disjunction(guard for guard, _ in paths)})"
    sail_raw = disjunction(f"(and {guard} (or (= {raw_defined} #x00) (distinct {value} {raw})))" for guard, value in paths)
    raw_ir = f"(or (distinct {raw_defined} {ir_defined}) (and (distinct {raw_defined} #x00) (distinct {raw} {ir})))"
    checks = (
        ("input-domain", "true", "sat"),
        ("sail-trace-coverage", coverage_gap, "unsat"),
        ("sail-vs-raw", sail_raw, "unsat"),
        ("raw-vs-final-ir", raw_ir, "unsat"),
    )
    report["provenance"]["solver_version"] = capture([args.solver, "-version"], deadline).strip()
    solver = [args.solver, "-in", "-smt2"]
    for name, predicate, expected in checks:
        formula = f"{base}(assert {predicate})\n(check-sat)\n"
        query = {"kind": name, "smt2": formula, "sha256": digest(formula.encode()), "expected": expected}
        report["queries"].append(query)
        report["phase"] = name
        response = capture(solver, deadline, formula + "(get-info :reason-unknown)\n")
        answer = response.splitlines()[0]
        query["result"] = answer
        if answer == expected:
            continue
        if answer == "sat" and name in ("sail-vs-raw", "raw-vs-final-ir"):
            values = " ".join(dict.fromkeys((*inputs.values(), raw, ir, *(symbol for path in paths for symbol in path))))
            model = capture(solver, deadline, formula + f"(get-value ({values}))\n")
            report["witness"] = replay_counterexample(kuna, paths, model)
            report["classification"] = "COUNTEREXAMPLE"
        else:
            report["reason"] = response.strip() if answer == "unknown" else f"{answer} in {name}"
        return
    report.pop("phase")
    report["classification"] = "MODEL_EQUIVALENCE"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("elf", type=Path)
    parser.add_argument("function")
    parser.add_argument("--isla", default="isla-footprint")
    parser.add_argument("--architecture", type=Path, required=True)
    parser.add_argument("--config", type=Path, required=True)
    parser.add_argument("--verify", type=Path, default=ROOT / "decompiler/target/release/kuna-verify")
    parser.add_argument("--sleighpath", type=Path, default=ROOT / "specs")
    parser.add_argument("--solver", default="z3")
    parser.add_argument("--timeout-ms", type=int, default=5000)
    parser.add_argument("--opcode-mutant", type=int)
    args = parser.parse_args()
    if not 1 <= args.timeout_ms <= 60000:
        parser.error("--timeout-ms must be 1..60000")
    report = {"version": 1, "function": args.function, "classification": "INCONCLUSIVE", "queries": []}
    try:
        validate(args, report)
    except (OSError, ValueError, TypeError, IndexError, KeyError, subprocess.SubprocessError) as error:
        report["reason"] = str(error)
    print(json.dumps(report, sort_keys=True))
    return {"MODEL_EQUIVALENCE": 0, "COUNTEREXAMPLE": 1}.get(report["classification"], 3)


if __name__ == "__main__":
    sys.exit(main())
