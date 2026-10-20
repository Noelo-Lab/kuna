import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock
import xml.etree.ElementTree as ET

from scripts.repipe import counters, mergecheck


class CatalogCountRenameTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-mergecheck-test-")
        self.addCleanup(temporary.cleanup)
        self.repo = Path(temporary.name)
        self.git("init", "-q")
        self.snapshot(321)
        self.write(mergecheck.REL_BASELINE, self.read(mergecheck.REL_BASELINE_STAGES))
        self.write("docs/other-baseline.json", self.read(mergecheck.REL_BASELINE_STAGES))
        self.commit()
        self.snapshot(322)

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

    def replace(self, rel, old, new):
        self.write(rel, self.read(rel).replace(old, new))

    def baseline(self, change):
        data = json.loads(self.read(mergecheck.REL_BASELINE_STAGES))
        change(data)
        self.write(mergecheck.REL_BASELINE_STAGES, json.dumps(data, indent=2) + "\n")

    def snapshot(self, count):
        options = ["returnpair", "fast_funcdisc", "armframes", "bytearrayc"]
        options += ["option-{}".format(i) for i in range(count - len(options))]
        self.write(counters.REL_PHASES_TOML, "\n".join(
            '[[settable]]\noption = "{}"'.format(option) for option in options) + "\n")
        self.write(counters.REL_CATALOG_FIXTURE, json.dumps(
            [{"option": option} for option in options], indent=2) + "\n")
        name = "KUNA-CATALOG #15: {} registry rows plus six individual queries".format(count)
        queries = ["returnpair", "fast_funcdisc", "armframes", "armframes",
                   "bytearrayc", "bytearrayc"]
        self.write(counters.REL_CATALOG_XML,
                   '<decompilertest><script><com>phase catalog</com>'
                   + ''.join('<com>phase catalog {}</com>'.format(option) for option in queries)
                   + '</script><stringmatch name="{}" min="{}" max="{}">'.format(
                       name, count + 6, count + 6)
                   + '"option": </stringmatch></decompilertest>\n')
        self.write(mergecheck.REL_BASELINE_STAGES, json.dumps({
            "data_footer": [2, 2], "passing": ["data:" + name, "data:OTHER #1: keep this key"],
            "returncode": 0, "unit_footer": None,
        }, indent=2) + "\n")

    def findings(self, rel=mergecheck.REL_BASELINE_STAGES):
        return mergecheck.assert_keepboth(rel, "HEAD", repo=self.repo)

    def assert_rejected(self, finding="line-removed"):
        findings = self.findings()
        self.assertIn(finding, [item.id for item in mergecheck.rejects(findings)], findings)
        return findings

    def test_valid_321_to_322_rename_is_reported_without_rejecting(self):
        with mock.patch.object(mergecheck, "_catalog_count_rename", return_value=None):
            previous = self.assert_rejected()
            self.assertIn('"data:KUNA-CATALOG #15: 321 registry rows plus six individual queries"',
                          next(item.detail for item in previous if item.id == "line-removed"))
        findings = self.findings()
        self.assertEqual(mergecheck.rejects(findings), [])
        self.assertEqual([item.id for item in findings], ["catalog-count-renamed"])
        self.assertIn("321 registry rows", findings[0].detail)
        self.assertIn("322 registry rows", findings[0].detail)

    def test_additional_keys_are_preserved_alongside_the_rename(self):
        self.baseline(lambda data: (data["passing"].append("data:NEW #1: a new assertion"),
                                    data.update(data_footer=[3, 3])))
        self.assertEqual(mergecheck.rejects(self.findings()), [])

    def test_unrelated_removed_key_still_rejects(self):
        self.baseline(lambda data: (data["passing"].pop(), data.update(data_footer=[1, 1])))
        findings = self.assert_rejected()
        self.assertIn("data:OTHER #1", next(item.detail for item in findings if item.id == "line-removed"))

    def test_duplicate_passing_keys_still_reject(self):
        for slot in (0, 1):
            with self.subTest(slot=slot):
                self.snapshot(322)
                self.baseline(lambda data: (data["passing"].append(data["passing"][slot]),
                                            data.update(data_footer=[3, 3])))
                self.assert_rejected("key-added-twice")
                self.assert_rejected("line-added-twice")

    def test_stale_and_impossible_footers_still_reject(self):
        for footer, finding in (([1, 1], "stale-footer"), ([2, 1], "impossible-footer")):
            with self.subTest(footer=footer):
                self.snapshot(322)
                self.baseline(lambda data: data.update(data_footer=footer))
                self.assert_rejected(finding)

    def test_incorrect_head_label_source_or_capture_rejects(self):
        mutations = [
            lambda: self.replace(mergecheck.REL_BASELINE_STAGES, "322 registry", "323 registry"),
            lambda: self.replace(counters.REL_PHASES_TOML, "[[settable]]", "[[group]]"),
            lambda: self.write(counters.REL_CATALOG_FIXTURE,
                               json.dumps(json.loads(self.read(counters.REL_CATALOG_FIXTURE))[:-1])),
            lambda: self.replace(counters.REL_CATALOG_XML, "322 registry", "323 registry"),
        ]
        for index, mutation in enumerate(mutations):
            with self.subTest(index=index):
                self.snapshot(322)
                mutation()
                self.assert_rejected()

    def test_incorrect_base_evidence_also_rejects(self):
        for rel, old, new in (
            (mergecheck.REL_BASELINE_STAGES, "321 registry", "320 registry"),
            (counters.REL_PHASES_TOML, "[[settable]]", "[[group]]"),
            (counters.REL_CATALOG_FIXTURE, '"option"', '"not-option"'),
            (counters.REL_CATALOG_XML, 'min="327"', 'min="326"'),
        ):
            with self.subTest(rel=rel):
                self.git("reset", "--hard", "-q", "HEAD")
                self.snapshot(321)
                self.replace(rel, old, new)
                self.commit()
                self.snapshot(322)
                self.assert_rejected()

    def test_stale_option_tally_rejects_even_when_bucket_checker_skips_it(self):
        with mock.patch.object(counters, "catalog_buckets", return_value=[]):
            for attribute in ("min", "max"):
                with self.subTest(attribute=attribute):
                    self.snapshot(322)
                    self.replace(counters.REL_CATALOG_XML, '{}="328"'.format(attribute),
                                 '{}="327"'.format(attribute))
                    self.assert_rejected()

    def test_changed_pattern_or_query_transcript_rejects(self):
        mutations = [
            ('"option": ', '"tier": '),
            ('<com>phase catalog</com>', '<com>phase catalog</com><com>phase catalog</com>'),
            ('<com>phase catalog returnpair</com>', ''),
            ('phase catalog returnpair', 'phase catalog missing-option'),
            ('phase catalog returnpair', 'phase catalog bytearrayc'),
            ('phase catalog returnpair', 'phase catalog returnpair extra'),
            ('phase catalog returnpair', 'phase cataloggarbage returnpair'),
            ('six individual queries', 'seven individual queries'),
        ]
        for old, new in mutations:
            with self.subTest(old=old, new=new):
                self.snapshot(322)
                self.replace(counters.REL_CATALOG_XML, old, new)
                self.assert_rejected()

    def test_duplicate_catalog_rows_or_xml_assertions_reject(self):
        rows = json.loads(self.read(counters.REL_CATALOG_FIXTURE))
        rows[-1] = rows[0]
        self.write(counters.REL_CATALOG_FIXTURE, json.dumps(rows))
        self.assert_rejected()
        self.snapshot(322)
        tree = ET.fromstring(self.read(counters.REL_CATALOG_XML))
        tree.append(ET.fromstring(ET.tostring(tree.find("stringmatch"))))
        self.write(counters.REL_CATALOG_XML, ET.tostring(tree, encoding="unicode"))
        self.assert_rejected()

    def test_missing_or_malformed_evidence_rejects(self):
        for rel in (counters.REL_PHASES_TOML, counters.REL_CATALOG_FIXTURE,
                    counters.REL_CATALOG_XML):
            with self.subTest(rel=rel):
                self.snapshot(322)
                (self.repo / rel).unlink()
                self.assert_rejected()
        for rel, text in ((counters.REL_CATALOG_FIXTURE, "[]"),
                          (counters.REL_CATALOG_FIXTURE, "[null]"),
                          (counters.REL_CATALOG_FIXTURE, '{}'),
                          (counters.REL_CATALOG_FIXTURE, '[{"option": 322}]'),
                          (counters.REL_CATALOG_FIXTURE, "{broken"),
                          (counters.REL_CATALOG_XML, "<broken"),
                          (counters.REL_CATALOG_XML, "<other/>"),
                          (mergecheck.REL_BASELINE_STAGES, "null"),
                          (mergecheck.REL_BASELINE_STAGES, "[]"),
                          (mergecheck.REL_BASELINE_STAGES, '{"passing": null}'),
                          (mergecheck.REL_BASELINE_STAGES, '{"passing": "not a list"}'),
                          (mergecheck.REL_BASELINE_STAGES, "{broken")):
            with self.subTest(rel=rel, text=text):
                self.snapshot(322)
                self.write(rel, text)
                self.assert_rejected()

    def test_empty_or_invalid_catalog_option_identity_rejects(self):
        for option in ("", " ", " option", "option ", None, 7):
            with self.subTest(option=option):
                self.snapshot(322)
                rows = json.loads(self.read(counters.REL_CATALOG_FIXTURE))
                rows[-1]["option"] = option
                self.write(counters.REL_CATALOG_FIXTURE, json.dumps(rows))
                self.assert_rejected()

    def test_same_count_does_not_authorize_a_label_change(self):
        self.snapshot(321)
        self.replace(mergecheck.REL_BASELINE_STAGES, "321 registry", "322 registry")
        self.replace(counters.REL_CATALOG_XML, "321 registry", "322 registry")
        self.replace(counters.REL_CATALOG_XML, '="327"', '="328"')
        self.assert_rejected()

    def test_nonstring_passing_entries_deny_the_rename(self):
        for invalid in (None, 322, {"key": "not a string"}, []):
            with self.subTest(invalid=invalid):
                self.snapshot(322)
                self.baseline(lambda data: (data["passing"].append(invalid),
                                            data.update(data_footer=[3, 3])))
                self.assert_rejected()

    def test_catalog_count_decrease_is_not_a_rename_exception(self):
        self.snapshot(320)
        self.assert_rejected()

    def test_other_baselines_and_upstream_baseline_remain_protected(self):
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
