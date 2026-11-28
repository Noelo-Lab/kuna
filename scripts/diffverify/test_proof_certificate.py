"""Verify that certificates bind to complete, original DiffVerify obligations."""

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from proof_certificate import obligations


SCRIPT = Path(__file__).with_name("proof_certificate.py")
DOMAIN = "(set-logic QF_BV)\n(declare-const x (_ BitVec 8))\n(check-sat)\n"
DIFF = DOMAIN.replace("(check-sat)", "(assert (distinct x x))\n(check-sat)")


def archive(smt):
    return hashlib.sha256(smt.encode()).hexdigest()


def evidence():
    return {"version": 1, "classification": "MODEL_EQUIVALENCE", "scope": {},
            "obligation": {"domain_smt2": DOMAIN, "domain_sha256": archive(DOMAIN),
                           "difference_smt2": DIFF, "difference_sha256": archive(DIFF)}}


class ProofSelectionTests(unittest.TestCase):
    def test_changed_query_and_extra_smt_command_are_refused(self):
        proof = evidence()
        proof["obligation"]["difference_smt2"] += "(assert false)\n"
        with self.assertRaisesRegex(ValueError, "digest differs"):
            obligations(proof)
        proof["obligation"]["difference_sha256"] = archive(proof["obligation"]["difference_smt2"])
        with self.assertRaisesRegex(ValueError, "after the satisfiability check"):
            obligations(proof)

    def test_bounded_proof_requires_a_nonempty_joint_domain(self):
        proof = evidence()
        proof["classification"] = "BOUNDED_EQUIVALENCE"
        proof["scope"]["max_visits_per_raw_instruction_and_ir_block"] = 4
        proof["obligation"].update(completion_smt2=DOMAIN, completion_sha256=archive(DOMAIN))
        with self.assertRaisesRegex(ValueError, "jointly completed domain"):
            obligations(proof)
        proof["obligation"].update(bounded_domain_smt2=DOMAIN, bounded_domain_sha256=archive(DOMAIN))
        self.assertEqual([(q["kind"], q["expected"]) for q in obligations(proof)],
                         [("domain", "sat"), ("difference", "unsat"), ("completion", "sat"), ("bounded_domain", "sat")])


class CertificateIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        env = ("KUNA_CVC5", "KUNA_ETHOS", "KUNA_CPC_SIGNATURE")
        if not all(key in os.environ for key in env):
            raise unittest.SkipTest("configure cvc5, Ethos and a matching CPC signature")
        cls.cvc5, cls.ethos, cls.signature = (os.environ[key] for key in env)

    def run_checker(self, source, *extra):
        result = subprocess.run([sys.executable, SCRIPT, source, "--cvc5", self.cvc5,
                                 "--ethos", self.ethos, "--signature", self.signature, *extra],
                                capture_output=True, timeout=30)
        return result.returncode, json.loads(result.stdout)

    def test_requires_checked_refutation_of_the_original_query(self):
        with tempfile.TemporaryDirectory(prefix="kuna-proof-certificate-") as directory:
            original = Path(directory) / "check.json"
            original.write_text(json.dumps(evidence()))
            code, produced = self.run_checker(original)
            self.assertEqual((code, produced["classification"]), (0, "CERTIFIED_EQUIVALENCE"), produced.get("reason"))
            bundle = Path(directory) / "bundle.json"
            bundle.write_text(json.dumps(produced))
            code, checked = self.run_checker(original, "--check-bundle", bundle)
            self.assertEqual((code, checked["classification"]), (0, "CERTIFIED_EQUIVALENCE"), checked.get("reason"))

            record, = produced["certificates"]
            record["proof"] = "(assume forged false)\n"
            record["proof_sha256"] = archive(record["proof"])
            bundle.write_text(json.dumps(produced))
            code, checked = self.run_checker(original, "--check-bundle", bundle)
            self.assertEqual(code, 3)
            self.assertIn("ethos", checked["reason"].lower())

            record["proof"] = '(include "different-rules.eo")\n' + record["proof"]
            record["proof_sha256"] = archive(record["proof"])
            bundle.write_text(json.dumps(produced))
            code, checked = self.run_checker(original, "--check-bundle", bundle)
            self.assertEqual(code, 3)
            self.assertIn("outside the proof language", checked["reason"])


if __name__ == "__main__":
    unittest.main()
