"""Normalize DiffVerify's verification artifacts without changing their verdicts."""

import argparse
from dataclasses import asdict, dataclass, field, replace
import json
from pathlib import Path
import re
import sys

from proof_certificate import obligations
from sail_rv64 import digest


@dataclass(frozen=True)
class VerificationEvidence:
    function: str
    verdict: str
    classification: str
    assurance: str
    backends: tuple
    source_sha256: str
    assumptions: dict = field(default_factory=dict)
    coverage: dict = field(default_factory=dict)
    bounds: dict = field(default_factory=dict)
    proof: dict | None = None
    counterexample: dict | None = None
    limitations: tuple = ()
    version: int = 1


SHARED_PCODE = "raw-and-final-ir-share-sleigh-and-pcode-semantics"
KUNA = ("raw-pcode", "final-ir")


def sampled(row, source):
    verdict, label = row["verdict"], row["label"]
    coverage, witness = {}, None
    match verdict:
        case "AGREES":
            checked = re.fullmatch(r"AGREES\(([1-9][0-9]*)\)", label)
            if checked is None:
                raise ValueError("sampled agreement needs at least one compared run")
            coverage["compared_runs"] = int(checked[1])
            classification = "SAMPLED_AGREEMENT"
        case "DIFFERS":
            run = row["run"]
            if not 1 <= run["run"] <= run["of"] or not row["lines"]:
                raise ValueError("sampled mismatch needs its failing run and observation")
            witness = {"run": run, "observations": row["lines"], "replay": "not-recorded"}
            classification = "COUNTEREXAMPLE"
        case "DIVERGES":
            declared = re.fullmatch(r"DIVERGES\(([a-z0-9-]+)\)", label)
            if declared is None:
                raise ValueError("declared divergence needs its registered identifier")
            coverage["divergence"] = declared[1]
            classification = "DECLARED_DIVERGENCE"
        case "UNKNOWN" | "SKIPPED":
            classification = "INCONCLUSIVE"
        case _:
            raise ValueError("unsupported Tier C verdict")
    return VerificationEvidence(row["function"], verdict, classification, "sampled", KUNA, source,
                                assumptions={"stage": row.get("test")}, coverage=coverage,
                                counterexample=witness, limitations=(SHARED_PCODE,))


def external(report, source):
    if report["version"] != 1:
        raise ValueError("unsupported cross-backend report version")
    coverage = report["coverage"]
    findings = report["findings"]
    limitations = report["limitations"]
    # A confirmed external mismatch survives unrelated limits in the case.
    if any("unicorn" in finding["values"] for finding in findings):
        label, verdict = "COUNTEREXAMPLE", "DIFFERS"
    elif findings or limitations or not coverage.get("full_return_comparison"):
        label, verdict = "INCONCLUSIVE", "UNKNOWN"
    else:
        label, verdict = "CROSS_BACKEND_AGREEMENT", "AGREES"
    if report["classification"] != label:
        raise ValueError("cross-backend verdict disagrees with its observations")
    return VerificationEvidence(
        report["function"], verdict, label, "independent-case", tuple(report["backends"]), source,
        assumptions={"case_sha256": report["case_sha256"], "provenance": report["provenance"]},
        coverage=coverage, counterexample={"findings": findings} if verdict == "DIFFERS" else None,
        limitations=tuple(dict.fromkeys((*limitations, SHARED_PCODE))),
    )


def symbolic(report, source):
    if report["version"] != 1:
        raise ValueError("unsupported symbolic report version")
    label = report["classification"]
    verdicts = {"MODEL_EQUIVALENCE": "AGREES", "BOUNDED_EQUIVALENCE": "AGREES",
                "COUNTEREXAMPLE": "DIFFERS", "INCONCLUSIVE": "UNKNOWN"}
    if label not in verdicts:
        raise ValueError("unsupported symbolic verdict")
    scope = report.get("scope", {})
    formal = report.get("kuna")
    bounds = {key: value for key, value in scope.items()
              if key.startswith(("max_", "solver_")) and value is not None}
    queries = report.get("queries", report.get("induction", {}).get("queries", []))
    obligation = report.get("obligation", {})
    hashes = {q["kind"]: q["sha256"] for q in queries}
    hashes.update({key.removesuffix("_sha256"): value for key, value in obligation.items()
                   if key.endswith("_sha256")})
    witness = report.get("witness")
    if label == "COUNTEREXAMPLE" and not witness:
        raise ValueError("symbolic counterexample needs a witness")
    if label.endswith("EQUIVALENCE") and not hashes:
        raise ValueError("symbolic equivalence needs its verification conditions")
    if label == "BOUNDED_EQUIVALENCE" and not bounds.get("max_visits_per_raw_instruction_and_ir_block"):
        raise ValueError("bounded equivalence needs an explicit loop bound")
    assumptions = {"provenance": report.get("provenance", {}), "scope": scope}
    if formal:
        assumptions["kuna_provenance"] = formal["provenance"]
    return VerificationEvidence(
        formal["function"] if formal else report["function"], verdicts[label], label,
        "bounded-model" if label == "BOUNDED_EQUIVALENCE" else "symbolic-model",
        ("sail-rv64", *KUNA) if formal else KUNA, source,
        assumptions=assumptions, bounds=bounds, coverage={
            "phase": report.get("phase"), "reason": report.get("reason"),
            "queries": [{"kind": q["kind"], "result": q.get("result")} for q in queries],
        },
        proof={"query_sha256": hashes} if hashes else None, counterexample=witness,
        limitations=tuple(dict.fromkeys((*report.get("limitations", ()), SHARED_PCODE))),
    )


def normalize(kind, raw):
    data = json.loads(raw)
    if kind == "tier-c":
        rows = data if isinstance(data, list) else [data]
        return [sampled(row, digest(raw)) for row in rows if "verdict" in row]
    if kind == "cross-backend":
        return [external(data, digest(raw))]
    return [symbolic(data, digest(raw))]


def with_certificate(evidence, checked, original):
    if (checked.get("version") != 1 or checked.get("classification") != "CERTIFIED_EQUIVALENCE"
            or checked.get("evidence_sha256") != evidence.source_sha256
            or checked.get("base_classification") != evidence.classification
            or evidence.verdict != "AGREES" or evidence.proof is None):
        raise ValueError("certificate validation does not identify this equivalence")
    fingerprints = ("checker_sha256", "signature_sha256", "cvc5_sha256")
    if not all(re.fullmatch(r"[a-f0-9]{64}", checked.get(key, "")) for key in fingerprints):
        raise ValueError("certificate validation is missing its tool identities")
    if checked["checked"] != [query["kind"] for query in obligations(original)]:
        raise ValueError("certificate validation omits a required obligation")
    certificate = {key: checked[key] for key in fingerprints}
    certificate["checked"] = checked["checked"]
    return replace(evidence, assurance="checked-certificate", classification="CERTIFIED_EQUIVALENCE",
                   proof={**evidence.proof, "certificate": certificate, "base_classification": evidence.classification})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", type=Path)
    parser.add_argument("--kind", required=True, choices=("tier-c", "cross-backend", "symbolic"))
    parser.add_argument("--verified-certificate", type=Path,
                        help="certificate validation emitted by proof_certificate.py for this report")
    args = parser.parse_args()
    try:
        raw = args.report.read_bytes() if args.report != Path("-") else sys.stdin.buffer.read()
        if args.kind == "tier-c":
            documents = (line for line in raw.splitlines() if line.strip())
        else:
            documents = (raw,)
        if args.verified_certificate and args.kind != "symbolic":
            raise ValueError("a certificate must reference a symbolic equivalence report")
        checked = json.loads(args.verified_certificate.read_bytes()) if args.verified_certificate else None
        for document in documents:
            for evidence in normalize(args.kind, document):
                if checked is not None:
                    evidence = with_certificate(evidence, checked, json.loads(document))
                print(json.dumps(asdict(evidence), sort_keys=True))
    except (OSError, TypeError, KeyError, ValueError) as error:
        parser.exit(2, f"verification evidence: {error}\n")


if __name__ == "__main__":
    main()
