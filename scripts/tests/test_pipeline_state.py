import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

from scripts.pipeline import state


class InventoryTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="kuna-inventory-test-")
        self.addCleanup(temporary.cleanup)
        self.path = Path(temporary.name) / "inventory.json"
        environment = mock.patch.dict(os.environ, {
            "KUNA_PIPELINE_STATE_DIR": temporary.name,
        })
        environment.start()
        self.addCleanup(environment.stop)

    def test_missing_inventory_can_be_initialized(self):
        self.assertEqual(state.snapshot(), state._empty())
        state.register("worker", "slug", "branch", "opportunity")
        self.assertEqual(state.snapshot()["workers"]["worker"]["slug"], "slug")

    def test_corrupt_inventory_is_not_overwritten_by_registration(self):
        for raw in [b'{"workers":', b'\xff']:
            with self.subTest(raw=raw):
                self.path.write_bytes(raw)
                with self.assertRaises(state.StateError):
                    state.register("worker", "slug", "branch", "opportunity")
                self.assertEqual(self.path.read_bytes(), raw)
                self.assertFalse(self.path.with_suffix(".json.tmp").exists())

    def test_invalid_container_types_are_rejected_without_writes(self):
        documents = [None, [], {"workers": []}, {"leases": "invalid"}]
        for document in documents:
            with self.subTest(document=document):
                text = json.dumps(document)
                self.path.write_text(text)
                with self.assertRaises(state.StateError):
                    state.claim("worker", "opportunity")
                self.assertEqual(self.path.read_text(), text)

    def test_unreadable_inventory_is_not_treated_as_empty(self):
        self.path.write_text("{}")
        with mock.patch("builtins.open", side_effect=PermissionError("denied")):
            with self.assertRaisesRegex(state.StateError, "cannot read inventory"):
                state._load()

    def test_legacy_missing_sections_and_extra_metadata_are_preserved(self):
        self.path.write_text(json.dumps({"workers": {}, "metadata": {"version": 1}}))
        state.register("worker", "slug", "branch", "opportunity")
        actual = state.snapshot()
        self.assertEqual(actual["metadata"], {"version": 1})
        for key in state._empty():
            self.assertIsInstance(actual[key], dict)

    def test_cli_reports_corruption_and_preserves_the_file(self):
        self.path.write_text("not JSON")
        result = subprocess.run(
            [sys.executable, "-m", "scripts.pipeline.state", "register",
             "--worker", "worker", "--slug", "slug", "--branch", "branch",
             "--opportunity", "opportunity"],
            cwd=Path(__file__).resolve().parents[2],
            capture_output=True, text=True, check=False,
        )
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn("cannot read inventory", result.stderr)
        self.assertNotIn("Traceback", result.stderr)
        self.assertEqual(result.stdout, "")
        self.assertEqual(self.path.read_text(), "not JSON")


if __name__ == "__main__":
    unittest.main()
