import contextlib
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

from scripts.pipeline import select


ROOT = Path(__file__).resolve().parents[2]


class BacklogTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-backlog-test-")
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.path = self.directory / "opportunities.json"
        environment = mock.patch.dict(os.environ, {
            "KUNA_PIPELINE_DOCS_DIR": temporary.name,
            "KUNA_PIPELINE_STATE_DIR": str(self.directory / "state"),
        })
        environment.start()
        self.addCleanup(environment.stop)

    def write(self, rows, name="opportunities.json"):
        (self.directory / name).write_text(json.dumps({"ranked": rows}))

    def test_missing_and_explicitly_empty_backlogs_are_empty(self):
        self.assertEqual(select.load_backlog(), [])
        self.write([])
        self.write([], "units.json")
        self.assertEqual(select.load_backlog(), [])
        for argument in ["--shell", "--json"]:
            with contextlib.redirect_stdout(io.StringIO()) as output:
                self.assertEqual(select.main([argument]), 1)
            self.assertEqual(output.getvalue(), "")

    def test_corrupt_backlogs_are_not_treated_as_empty(self):
        for raw in [b'{"ranked":', b'\xff']:
            with self.subTest(raw=raw):
                self.path.write_bytes(raw)
                with self.assertRaisesRegex(RuntimeError, "cannot read backlog"):
                    select.load_backlog()
                self.assertEqual(self.path.read_bytes(), raw)

    def test_invalid_container_types_are_rejected(self):
        for document in [None, [], {}, {"ranked": None}, {"ranked": {}},
                         {"ranked": "invalid"}, {"ranked": [None]},
                         {"ranked": [[]]}, {"ranked": [1]}]:
            with self.subTest(document=document):
                raw = json.dumps(document)
                self.path.write_text(raw)
                with self.assertRaisesRegex(RuntimeError, "invalid backlog"):
                    select.load_backlog()
                self.assertEqual(self.path.read_text(), raw)

    def test_unreadable_backlog_is_not_treated_as_empty(self):
        self.write([])
        with mock.patch("builtins.open", side_effect=PermissionError("denied")):
            with self.assertRaisesRegex(RuntimeError, "cannot read backlog"):
                select.load_backlog()

    def test_consumed_fields_have_usable_types(self):
        invalid = [
            {"score": None}, {"score": "high"}, {"score": True},
            {"score": float("nan")}, {"score": float("inf")},
            {"test_name": None}, {"test_name": []}, {"func_name": 3},
            {"kinds": "structural"}, {"kinds": [1]}, {"kinds": None},
            {"reasons": {}}, {"reasons": [False]},
        ]
        for row in invalid:
            with self.subTest(row=row):
                self.write([row])
                with self.assertRaisesRegex(RuntimeError, "invalid backlog"):
                    select.load_backlog()

    def test_invalid_secondary_backlog_does_not_hide_behind_a_valid_primary(self):
        self.write([{"score": 5, "comparable": True, "test_name": "valid"}])
        (self.directory / "units.json").write_text("not JSON")
        with self.assertRaisesRegex(RuntimeError, "units.json"):
            select.pick()

    def test_combining_backlogs_preserves_stable_ranking_and_extra_fields(self):
        high = {"score": 9, "extra": {"nested": True}}
        first_tie = {"score": 4, "test_name": "first"}
        second_tie = {"score": 4, "test_name": "second"}
        unscored = {"test_name": "unscored"}
        self.write([first_tie, unscored])
        self.write([second_tie, high], "units.json")
        self.assertEqual(select.load_backlog(), [high, first_tie, second_tie, unscored])

    def test_pick_preserves_filters_claims_and_legacy_skip_custom_behavior(self):
        taken = {"score": 9, "comparable": True, "test_name": "taken", "selector": "main"}
        unavailable = {"score": 8, "comparable": False}
        unit = {"score": 6, "comparable": True, "test_name": "unit", "kinds": ["small-unit"]}
        structural = {"score": 4, "comparable": True, "test_name": "structural",
                      "custom_options": {"option": True}}
        low = {"score": 0, "comparable": True}
        self.write([low, structural, taken, unavailable, unit])
        with mock.patch.object(select.state, "taken", return_value={"taken::main"}):
            self.assertEqual(select.pick()["test_name"], "unit")
            self.assertEqual(select.pick(kind="small-unit")["test_name"], "unit")
            selected = select.pick(kind="structural")
            self.assertEqual(selected["test_name"], "structural")
            self.assertEqual(select.pick(kind="structural", skip_custom=True), selected)
            self.assertIsNone(select.pick(min_score=7))

    def test_cli_reports_bad_backlog_without_emitting_a_selection(self):
        self.path.write_text("not JSON")
        for argument in ["--shell", "--json"]:
            with self.subTest(argument=argument):
                result = subprocess.run(
                    [sys.executable, "-m", "scripts.pipeline.select", argument],
                    cwd=ROOT, capture_output=True, text=True, check=False,
                )
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("cannot read backlog", result.stderr)
                self.assertIn(str(self.path), result.stderr)
                self.assertNotIn("Traceback", result.stderr)
                self.assertNotIn("no remaining opportunities", result.stderr)
                self.assertEqual(result.stdout, "")
                self.assertEqual(self.path.read_text(), "not JSON")

    def test_cli_reports_bad_inventory_without_a_traceback(self):
        self.write([])
        inventory = self.directory / "state" / "inventory.json"
        inventory.parent.mkdir()
        inventory.write_text("not JSON")
        result = subprocess.run(
            [sys.executable, "-m", "scripts.pipeline.select", "--json"],
            cwd=ROOT, capture_output=True, text=True, check=False,
        )
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn("cannot read inventory", result.stderr)
        self.assertNotIn("Traceback", result.stderr)
        self.assertEqual(result.stdout, "")


if __name__ == "__main__":
    unittest.main()
