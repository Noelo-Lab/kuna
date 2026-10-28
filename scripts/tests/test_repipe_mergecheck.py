import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from scripts.repipe import mergecheck


class MergeSafetyTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-mergecheck-test-")
        self.addCleanup(temporary.cleanup)
        self.repo = Path(temporary.name)
        self.git("init", "-q")
        self.snapshot()
        self.write(mergecheck.REL_BASELINE, self.read(mergecheck.REL_BASELINE_STAGES))
        self.write("docs/other-baseline.json", self.read(mergecheck.REL_BASELINE_STAGES))
        self.commit()

    def git(self, *args):
        return subprocess.run(
            ["git", "-c", "user.name=mergecheck-test", "-c",
             "user.email=mergecheck@localhost", "-c", "commit.gpgsign=false",
             "-C", str(self.repo), *args],
            capture_output=True, text=True, check=True,
        )

    def commit(self):
        self.git("add", "-A")
        self.git("commit", "-qm", "fixture")

    def write(self, rel, text):
        path = self.repo / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")

    def read(self, rel):
        return (self.repo / rel).read_text(encoding="utf-8")

    def baseline(self, change):
        data = json.loads(self.read(mergecheck.REL_BASELINE_STAGES))
        change(data)
        self.write(mergecheck.REL_BASELINE_STAGES, json.dumps(data, indent=2) + "\n")

    def snapshot(self):
        self.write(mergecheck.REL_BASELINE_STAGES, json.dumps({
            "data_footer": [2, 2],
            "passing": ["data:CATALOG: registry schema", "data:OTHER: retained assertion"],
            "returncode": 0, "unit_footer": None,
        }, indent=2) + "\n")

    def findings(self, rel=mergecheck.REL_BASELINE_STAGES):
        return mergecheck.assert_keepboth(rel, "HEAD", repo=self.repo)

    def assert_rejected(self, finding="line-removed"):
        findings = self.findings()
        self.assertIn(finding, [item.id for item in mergecheck.rejects(findings)], findings)
        return findings

    def test_unchanged_semantic_assertions_need_no_catalog_snapshot(self):
        self.assertEqual(self.findings(), [])

    def test_additional_assertion_preserves_existing_keys(self):
        self.baseline(lambda data: (data["passing"].append("data:NEW: a new assertion"),
                                    data.update(data_footer=[3, 3])))
        self.assertEqual(self.findings(), [])

    def test_removed_assertion_rejects_even_with_correct_footer(self):
        self.baseline(lambda data: (data["passing"].pop(), data.update(data_footer=[1, 1])))
        findings = self.assert_rejected()
        self.assertIn("data:OTHER", next(item.detail for item in findings if item.id == "line-removed"))

    def test_renaming_an_assertion_does_not_bypass_removal_guard(self):
        self.baseline(lambda data: data["passing"].__setitem__(0, "data:CATALOG: different claim"))
        self.assert_rejected()

    def test_historical_count_label_has_no_rename_exception(self):
        self.baseline(lambda data: data["passing"].__setitem__(
            0, "data:KUNA-CATALOG #15: 321 registry rows plus six individual queries"))
        self.commit()
        self.baseline(lambda data: data["passing"].__setitem__(
            0, "data:KUNA-CATALOG #15: 322 registry rows plus six individual queries"))
        self.assert_rejected()

    def test_duplicate_passing_keys_reject(self):
        for slot in (0, 1):
            with self.subTest(slot=slot):
                self.snapshot()
                self.baseline(lambda data: (data["passing"].append(data["passing"][slot]),
                                            data.update(data_footer=[3, 3])))
                self.assert_rejected("key-added-twice")
                self.assert_rejected("line-added-twice")

    def test_stale_and_impossible_footers_reject(self):
        for footer, finding in (([1, 1], "stale-footer"), ([2, 1], "impossible-footer")):
            with self.subTest(footer=footer):
                self.snapshot()
                self.baseline(lambda data: data.update(data_footer=footer))
                self.assert_rejected(finding)

    def test_moving_assertions_does_not_count_as_removal(self):
        self.baseline(lambda data: data["passing"].reverse())
        self.assertEqual(self.findings(), [])

    def test_other_baselines_and_upstream_baseline_remain_protected(self):
        self.baseline(lambda data: data["passing"].__setitem__(0, "data:NEW: replacement claim"))
        for rel in ("docs/other-baseline.json", mergecheck.REL_BASELINE):
            with self.subTest(rel=rel):
                self.write(rel, self.read(mergecheck.REL_BASELINE_STAGES))
                self.assertTrue(mergecheck.rejects(self.findings(rel)))
        findings = mergecheck.assert_baseline_untouched("HEAD", repo=self.repo)
        self.assertEqual([item.id for item in findings], ["baseline-repinned"])

    def test_original_merge_hazards_remain_detected(self):
        result = mergecheck.self_test()
        self.assertTrue(result["ok"], result)
        self.assertEqual([case["case"] for case in result["cases"]], ["B", "C", "C2"])


if __name__ == "__main__":
    unittest.main()
