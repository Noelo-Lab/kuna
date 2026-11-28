"""Check that evidence classification retains scope and concrete disagreements."""

import json
import unittest

from test_proof_certificate import DOMAIN, archive, evidence
from verification_evidence import normalize, with_certificate


def encoded(value):
    return json.dumps(value).encode()


class EvidenceTests(unittest.TestCase):
    def test_sampled_verdicts_do_not_claim_equivalence(self):
        row = {"function": "sample", "verdict": "AGREES", "label": "AGREES(7)"}
        result, = normalize("tier-c", encoded(row))
        self.assertEqual(result.classification, "SAMPLED_AGREEMENT")
        self.assertEqual(result.coverage, {"compared_runs": 7})
        self.assertIsNone(result.proof)
        row.update(verdict="DIVERGES", label="DIVERGES(copy-overlap)")
        result, = normalize("tier-c", encoded(row))
        self.assertEqual(result.classification, "DECLARED_DIVERGENCE")
        self.assertEqual(result.coverage, {"divergence": "copy-overlap"})

    def test_external_counterexample_survives_incomplete_execution(self):
        capture = {"version": 1, "function": "sample", "classification": "COUNTEREXAMPLE",
                   "coverage": {"full_return_comparison": False}, "case_sha256": "a" * 64,
                   "provenance": {}, "backends": {"unicorn": "2", "raw-pcode": "kuna"},
                   "limitations": ["one-or-more-backends-did-not-complete"],
                   "findings": [{"values": {"unicorn": "0x3", "raw-pcode": "0x2"}}]}
        result, = normalize("cross-backend", encoded(capture))
        self.assertEqual(result.classification, "COUNTEREXAMPLE")
        capture["findings"][0]["values"].pop("unicorn")
        with self.assertRaisesRegex(ValueError, "disagrees with its observations"):
            normalize("cross-backend", encoded(capture))
        capture["classification"] = "INCONCLUSIVE"
        result, = normalize("cross-backend", encoded(capture))
        self.assertEqual(result.verdict, "UNKNOWN")

    def test_certification_preserves_bounds_and_binds_the_original_obligations(self):
        proof = evidence()
        proof.update(function="count_down", classification="BOUNDED_EQUIVALENCE")
        proof["scope"]["max_visits_per_raw_instruction_and_ir_block"] = 4
        proof["obligation"] |= {"completion_smt2": DOMAIN, "completion_sha256": archive(DOMAIN),
                                "bounded_domain_smt2": DOMAIN, "bounded_domain_sha256": archive(DOMAIN)}
        result, = normalize("symbolic", encoded(proof))
        checked = {"version": 1, "classification": "CERTIFIED_EQUIVALENCE",
                   "evidence_sha256": result.source_sha256, "base_classification": "BOUNDED_EQUIVALENCE",
                   "checked": ["domain", "difference", "completion", "bounded_domain"],
                   "checker_sha256": "d" * 64, "signature_sha256": "e" * 64, "cvc5_sha256": "f" * 64}
        certified = with_certificate(result, checked, proof)
        self.assertEqual(certified.classification, "CERTIFIED_EQUIVALENCE")
        self.assertEqual(certified.proof["base_classification"], "BOUNDED_EQUIVALENCE")
        self.assertEqual(certified.bounds["max_visits_per_raw_instruction_and_ir_block"], 4)
        checked["checked"].pop()
        with self.assertRaisesRegex(ValueError, "omits a required obligation"):
            with_certificate(result, checked, proof)


if __name__ == "__main__":
    unittest.main()
