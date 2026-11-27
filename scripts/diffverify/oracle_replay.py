"""Rerun all three backends for an archived Kuna function case."""

import argparse
import json
import subprocess
import sys
from hashlib import file_digest
from pathlib import Path

from oracle_compare import Return, compare
from oracle_protocol import load_request

ROOT = Path(__file__).resolve().parents[2]
RUNNER = Path(__file__).with_name("function_oracle.py")
GPR_INPUTS = ("RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", *(f"R{i}" for i in range(8, 16)))
INPUTS = (*GPR_INPUTS, *(f"XMM{i}" for i in range(16)), "RFLAGS")


def replay(capture, binary, specs, verify, expected):
    case = capture["case"]
    recorded = capture["provenance"]
    with open(binary, "rb") as file:
        if file_digest(file, "sha256").hexdigest() != recorded["elf_sha256"]:
            raise ValueError("ELF does not match the archived case")
    command = [
        verify, "oracle-case", binary, case["entry"], "--sleighpath", specs,
        "--seed", capture["seed"], "--budget", str(capture["pcode_budget"]),
    ]
    for name, value in capture["decompile_options"]:
        command.extend(("--option", name, value))
    for address, arity in case.get("external_calls", {}).items():
        command.extend(("--external-entry", address, str(arity)))
    if mutation := capture.get("mutation"):
        command.extend(("--opcode-mutant", str(mutation["op"])))
    for register in INPUTS:
        command.extend(("--input-reg", register, case["registers"][register]))
    reproduced = json.loads(subprocess.check_output(command, timeout=60))
    if reproduced["case"] != case or reproduced["provenance"] != recorded:
        raise ValueError("ELF, SLEIGH configuration or regenerated input differs")
    if reproduced["observations"] != capture["observations"]:
        raise ValueError("regenerated Kuna observations differ from the archive")
    if reproduced.get("mutation") != capture.get("mutation"):
        raise ValueError("regenerated mutant differs from the archive")
    invocation = (sys.executable, RUNNER, "-")
    oracle = json.loads(subprocess.check_output(invocation, input=json.dumps(case).encode("utf-8"), timeout=15))
    return compare(reproduced, oracle, expected)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", help="archived JSON envelope from kuna-verify oracle-case")
    parser.add_argument("binary", help="the ELF used to produce the capture")
    parser.add_argument("--sleighpath", default=ROOT / "specs")
    parser.add_argument("--verify", default=ROOT / "decompiler/target/release/kuna-verify")
    parser.add_argument("--return", dest="return_spec", required=True, help="void or REGISTER:BYTES")
    args = parser.parse_args()
    try:
        with open(args.capture, "rb") as file:
            capture = load_request(file)
        result = replay(capture, args.binary, args.sleighpath, args.verify, Return.parse(args.return_spec))
    except (AttributeError, KeyError, TypeError, ValueError, OSError, subprocess.SubprocessError) as error:
        parser.exit(2, f"oracle replay: {error}\n")
    print(json.dumps(result, sort_keys=True))
    if result["classification"] == "COUNTEREXAMPLE":
        parser.exit(1)


if __name__ == "__main__":
    main()
