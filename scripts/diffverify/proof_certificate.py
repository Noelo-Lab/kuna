"""Produce and independently check certificates for DiffVerify's archived SMT queries."""

import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time

from sail_rv64 import capture, digest, file_digest, sexps


CERTIFICATE_COMMANDS = {"declare-const", "define", "assume", "assume-push", "step", "step-pop"}
SCRIPT_COMMANDS = {"set-logic", "declare-const", "define-fun", "assert", "check-sat", "get-info"}
INCLUDE = re.compile(r'^\s*\(include "([^"\n]+)"\)', re.MULTILINE)


def obligation(query, kind, expected):
    smt = query["smt2"]
    if digest(smt.encode()) != query["sha256"]:
        raise ValueError(f"archived {kind} query digest differs")
    forms = sexps(smt)
    checks = [i for i, form in enumerate(forms) if form == ["check-sat"]]
    if len(checks) != 1 or forms[0] != ["set-logic", "QF_BV"]:
        raise ValueError(f"{kind} does not contain an exact QF_BV check")
    index, = checks
    if forms[index + 1:] not in ([], [["get-info", ":reason-unknown"]]):
        raise ValueError(f"{kind} contains statements after the satisfiability check")
    if any(not isinstance(form, list) or not form or form[0] not in SCRIPT_COMMANDS for form in forms):
        raise ValueError(f"{kind} contains an unsupported SMT command")
    return dict(kind=kind, expected=expected, sha256=query["sha256"], smt2=smt)


def obligations(evidence):
    label = evidence["classification"]
    if evidence["version"] != 1 or label not in ("MODEL_EQUIVALENCE", "BOUNDED_EQUIVALENCE"):
        raise ValueError("certificate requires a version 1 symbolic equivalence report")
    if "induction" in evidence or "queries" in evidence:
        if label != "MODEL_EQUIVALENCE":
            raise ValueError("inductive and Sail reports must claim model equivalence")
        if "induction" in evidence:
            induction = evidence["induction"]
            queries = induction["queries"]
            expected = [("initial-domain", "sat"), ("loop-initiation-and-prefix-exits", "unsat"),
                        ("reachable-loop-cutpoint", "sat"), ("loop-preservation-and-suffix-exits", "unsat")]
            rank = induction["ranking"]
            if not isinstance(rank, dict) or len(queries) <= len(expected):
                raise ValueError("inductive result is missing its ranking proof")
            selected = [q for q in queries[len(expected):] if q.get("kind") == "rank-decrease"
                        and q.get("rank") == rank and q.get("result") == "unsat"]
            if len(selected) != 1:
                raise ValueError("inductive result does not identify one validated ranking proof")
            queries = queries[:len(expected)] + selected
            expected.append(("rank-decrease", "unsat"))
        else:
            queries = evidence["queries"]
            expected = [("input-domain", "sat"), ("sail-trace-coverage", "unsat"),
                        ("sail-vs-raw", "unsat"), ("raw-vs-final-ir", "unsat")]
        if [(q.get("kind"), q.get("result")) for q in queries] != expected:
            raise ValueError("symbolic equivalence report omits required query results")
        return [obligation(q, kind, result) for q, (kind, result) in zip(queries, expected, strict=True)]

    archived = evidence["obligation"]
    scope = evidence["scope"]
    expected = [("domain", "sat"), ("difference", "unsat")]
    if "coverage_smt2" in archived or scope.get("memory") or scope.get("control_flow") in ("acyclic-function", "bounded-function"):
        expected.append(("coverage", "unsat"))
    if scope.get("max_visits_per_raw_instruction_and_ir_block") is not None:
        expected.append(("completion", "sat" if label == "BOUNDED_EQUIVALENCE" else "unsat"))
    if label == "BOUNDED_EQUIVALENCE":
        if "bounded_domain_smt2" not in archived or scope.get("max_visits_per_raw_instruction_and_ir_block") is None:
            raise ValueError("bounded agreement is missing a jointly completed domain")
        expected.append(("bounded_domain", "sat"))
    return [obligation({"smt2": archived[f"{name}_smt2"], "sha256": archived[f"{name}_sha256"]}, name, result)
            for name, result in expected]


def signature_fingerprint(path):
    path = Path(path).resolve(strict=True)
    root = path.parent
    pending = [path]
    seen = {}
    while pending:
        included = pending.pop().resolve(strict=True)
        relative = included.relative_to(root).as_posix()
        if relative in seen:
            continue
        data = included.read_bytes()
        seen[relative] = digest(data)
        pending.extend(included.parent / name for name in INCLUDE.findall(data.decode("utf-8")))
        if len(seen) > 128:
            raise ValueError("CPC signature exceeds its include limit")
    return digest(json.dumps(seen, sort_keys=True).encode())


def verify_proof(args, proof, smt, deadline):
    forms = sexps(proof)
    if not forms or any(not isinstance(form, list) or not form or form[0] not in CERTIFICATE_COMMANDS for form in forms):
        raise ValueError("CPC certificate contains a command outside the proof language")
    with tempfile.TemporaryDirectory(prefix="kuna-cpc-") as directory:
        reference = Path(directory) / "query.smt2"
        certificate = Path(directory) / "proof.cpc"
        reference.write_text(smt, encoding="utf-8")
        certificate.write_text(proof, encoding="utf-8")
        answer = capture([args.ethos, f"--include={args.signature.resolve()}", f"--reference={reference}",
                          "--require-proof-of-false", certificate], deadline)
    if answer.strip() != "correct":
        raise ValueError(f"Ethos did not validate a complete contradiction ({answer.strip()})")


def cvc5(args, query, deadline, prove=False):
    text = query["smt2"].rstrip()
    text = text.removesuffix("(get-info :reason-unknown)").rstrip() + "\n"
    command = [args.cvc5, "--lang=smt2"]
    if prove:
        command.extend(("--produce-proofs", "--proof-format-mode=cpc", "--proof-granularity=dsl-rewrite"))
        # The appended command only requests a proof of the original assertion.
        text += "\n(get-proof)\n"
    answer = capture(command, deadline, text)
    result, _, payload = answer.partition("\n")
    if result != query["expected"]:
        raise ValueError(f'{query["kind"]}: cvc5 gave {result!r}, expected {query["expected"]!r}')
    if not prove:
        return None
    payload = payload.strip()
    forms = sexps(payload)
    if not payload.startswith("(") or not payload.endswith(")") or len(forms) != 1:
        raise ValueError(f'{query["kind"]}: cvc5 did not return one CPC proof')
    return payload[1:-1].strip() + "\n"


def certify(args, evidence, archived, bundle):
    deadline = time.monotonic() + args.timeout_ms / 1000
    checks = obligations(evidence)
    if bundle is not None:
        if bundle.get("version") != 1 or bundle.get("evidence_sha256") != archived:
            raise ValueError("certificate bundle references different verification evidence")
        if bundle.get("signature_sha256") != signature_fingerprint(args.signature):
            raise ValueError("CPC signature differs from the producer signature")
        certificates = bundle["certificates"]
        if not isinstance(certificates, list) or [item.get("kind") for item in certificates] != [
            q["kind"] for q in checks if q["expected"] == "unsat"
        ]:
            raise ValueError("certificate bundle does not cover each required UNSAT query")
    else:
        certificates = []
    prior = iter(certificates)
    for query in checks:
        if query["expected"] == "sat":
            cvc5(args, query, deadline)
            continue
        if bundle is None:
            proof = cvc5(args, query, deadline, prove=True)
            certificates.append({
                "kind": query["kind"], "query_sha256": query["sha256"],
                "proof_sha256": digest(proof.encode()), "proof": proof,
            })
        else:
            record = next(prior)
            proof = record["proof"]
            if record["query_sha256"] != query["sha256"] or record["proof_sha256"] != digest(proof.encode()):
                raise ValueError(f'{query["kind"]}: query or certificate digest differs')
        verify_proof(args, proof, query["smt2"], deadline)
    result = {
        "version": 1, "classification": "CERTIFIED_EQUIVALENCE",
        "base_classification": evidence["classification"], "evidence_sha256": archived,
        "signature_sha256": signature_fingerprint(args.signature),
        "checker_sha256": file_digest(shutil.which(args.ethos) or args.ethos),
        "cvc5_sha256": file_digest(shutil.which(args.cvc5) or args.cvc5),
        "checked": [q["kind"] for q in checks],
    }
    if bundle is None:
        result["certificates"] = certificates
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("--check-bundle", type=Path)
    parser.add_argument("--signature", type=Path, required=True)
    parser.add_argument("--ethos", default="ethos")
    parser.add_argument("--cvc5", default="cvc5")
    parser.add_argument("--timeout-ms", type=int, default=30000)
    args = parser.parse_args()
    if not 1 <= args.timeout_ms <= 120000:
        parser.error("--timeout-ms must be 1..120000")
    try:
        raw = args.evidence.read_bytes()
        bundle = json.loads(args.check_bundle.read_text()) if args.check_bundle else None
        result = certify(args, json.loads(raw), digest(raw), bundle)
    except (OSError, ValueError, TypeError, KeyError, IndexError, subprocess.SubprocessError) as error:
        result = {"classification": "INCONCLUSIVE", "reason": str(error)}
    print(json.dumps(result, sort_keys=True))
    return 0 if result["classification"] == "CERTIFIED_EQUIVALENCE" else 3


if __name__ == "__main__":
    sys.exit(main())
