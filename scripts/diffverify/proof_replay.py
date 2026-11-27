"""Replay a symbolic return counterexample in Kuna and Unicorn."""

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

from oracle_compare import Return, compare
from oracle_protocol import load_request
from oracle_replay import ROOT, RUNNER


def replay_proof(proof, binary, specs, verify):
    if proof["version"] != 1 or proof["classification"] != "COUNTEREXAMPLE":
        raise ValueError("expected a version 1 symbolic counterexample")
    obligation = proof["obligation"]
    for query in ("domain", "difference"):
        encoded = obligation[f"{query}_smt2"].encode("utf-8")
        if hashlib.sha256(encoded).hexdigest() != obligation[f"{query}_sha256"]:
            raise ValueError(f"archived {query} query digest differs")
    scope = proof["scope"]
    observation = scope["observation"]
    expected = Return.parse(f'{observation["register"]}:{observation["bytes"]}')
    registers = proof["witness"]["registers"]
    expected_inputs = set(scope["symbolic_inputs"])
    if scope.get("symbolic_flags"):
        expected_inputs.add("RFLAGS")
    if set(registers) != expected_inputs:
        raise ValueError("witness does not fill the declared symbolic inputs")
    if any(int(registers[reg], 16) != int(value, 16) for reg, value in scope["bindings"]):
        raise ValueError("witness violates the archived register bindings")
    command = [
        verify, "oracle-case", binary, scope["raw_interval"]["start"], "--sleighpath", specs,
    ]
    for name, value in proof["decompile_options"]:
        command.extend(("--option", name, value))
    if mutation := proof.get("mutation"):
        command.extend(("--opcode-mutant", str(mutation["op"])))
    for register, value in registers.items():
        command.extend(("--input-reg", register, value))
    capture = json.loads(subprocess.check_output(command, timeout=60))
    for field in ("elf_sha256", "sla_sha256", "language"):
        if capture["provenance"][field] != proof["provenance"][field]:
            raise ValueError(f"proof and replay differ in {field}")
    if capture.get("mutation") != proof.get("mutation"):
        raise ValueError("proof and replay use different mutants")
    invocation = (sys.executable, RUNNER, "-")
    case = json.dumps(capture["case"]).encode("utf-8")
    oracle = json.loads(subprocess.check_output(invocation, input=case, timeout=15))
    report = compare(capture, oracle, expected)
    confirmed = any(
        f["observable"] in ("return", "termination")
        and set(f["values"]) == {"unicorn", "raw-pcode", "final-ir"}
        and f["values"]["unicorn"] == f["values"]["raw-pcode"] != f["values"]["final-ir"]
        for f in report["findings"]
    )
    return {
        "version": 1,
        "independent_replay": "confirmed" if confirmed else "not-confirmed",
        "difference_sha256": obligation["difference_sha256"],
        "observation_report": report,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("proof", type=Path)
    parser.add_argument("binary", type=Path)
    parser.add_argument("--sleighpath", default=ROOT / "specs")
    parser.add_argument("--verify", default=ROOT / "decompiler/target/release/kuna-verify")
    args = parser.parse_args()
    try:
        with args.proof.open("rb") as file:
            proof = load_request(file)
        report = replay_proof(proof, args.binary, args.sleighpath, args.verify)
    except (AttributeError, KeyError, TypeError, ValueError, OSError, subprocess.SubprocessError) as error:
        parser.exit(2, f"proof replay: {error}\n")
    print(json.dumps(report, sort_keys=True))
    if report["independent_replay"] != "confirmed":
        parser.exit(3)


if __name__ == "__main__":
    main()
