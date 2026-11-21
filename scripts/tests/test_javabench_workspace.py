"""javabench's workspace convention: every path under one directory, none without it.

The metric tests need tree-sitter and a JDK and live in scripts/javabench/tests; this part needs
neither, so it runs everywhere.
"""
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from scripts.javabench import fetch, projects

ROOT = Path(__file__).resolve().parents[2]
ENV_KEYS = ("JAVABENCH_WORKSPACE", "JAVABENCH_DATASET", "JAVABENCH_RESULTS", "JAVABENCH_JDK25")


class WorkspaceTests(unittest.TestCase):
    def setUp(self):
        self.saved = {k: os.environ.get(k) for k in ENV_KEYS}
        for k in ENV_KEYS:
            os.environ.pop(k, None)
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)

    def tearDown(self):
        for k, v in self.saved.items():
            if v is None:
                os.environ.pop(k, None)
            else:
                os.environ[k] = v

    def test_paths_resolve_under_the_workspace(self):
        root = projects.configure(self.tmp.name)
        self.assertEqual(projects.DATASET_DIR, root / "dataset")
        self.assertEqual(projects.RESULTS_DIR, root / "results")
        self.assertEqual(projects.TOOLS_DIR, root / "tools")
        self.assertEqual(projects.LOCK_DIR, root / "locks")
        self.assertEqual(projects.get("guava").dir, root / "dataset" / "guava")
        self.assertTrue(projects.JDKS[25].startswith(str(root / "tools")))
        self.assertEqual(os.environ["JAVABENCH_WORKSPACE"], str(root))

    def test_per_directory_overrides_win(self):
        os.environ["JAVABENCH_DATASET"] = os.path.join(self.tmp.name, "elsewhere")
        projects.configure(self.tmp.name)
        self.assertEqual(projects.DATASET_DIR, Path(self.tmp.name) / "elsewhere")

    def test_pins_are_the_lock_until_the_workspace_has_one(self):
        projects.configure(self.tmp.name)
        pin = json.loads((projects.PINS_DIR / "guava.json").read_text())
        self.assertEqual(fetch._load_lock("guava"), pin)
        self.assertTrue(all(len(d["sha256"]) == 64 for d in pin["deps"]))
        own = projects.LOCK_DIR / "guava.json"
        own.parent.mkdir(parents=True)
        own.write_text('{"artifacts": {}, "deps": []}')
        self.assertEqual(fetch._load_lock("guava"), {"artifacts": {}, "deps": []})
        self.assertEqual(sorted(p.stem for p in projects.PINS_DIR.glob("*.json")), sorted(projects.PROJECTS))

    def test_without_a_workspace_the_cli_stops_with_one_line(self):
        env = {k: v for k, v in os.environ.items() if k not in ENV_KEYS}
        env["PYTHONPATH"] = str(ROOT)
        r = subprocess.run([sys.executable, "-m", "scripts.javabench", "report", "some-run"], cwd=self.tmp.name,
                           env=env, capture_output=True, text=True, timeout=60)
        self.assertEqual(r.returncode, 1)
        self.assertEqual(r.stderr.strip(), projects.NO_WORKSPACE)
        self.assertIn("JAVABENCH_WORKSPACE", r.stderr)
        self.assertIn("scripts/javabench/README.md", r.stderr)

    def test_the_workspace_option_names_the_directory(self):
        env = {k: v for k, v in os.environ.items() if k not in ENV_KEYS}
        env["PYTHONPATH"] = str(ROOT)
        r = subprocess.run([sys.executable, "-m", "scripts.javabench", "--workspace", "ws", "report", "no-such-run"],
                           cwd=self.tmp.name, env=env, capture_output=True, text=True, timeout=60)
        self.assertEqual(r.returncode, 1)
        self.assertIn("not a javabench run: no-such-run", r.stderr)


if __name__ == "__main__":
    unittest.main()
